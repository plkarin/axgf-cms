//! The ways in (v0.5.0, docs/adr/0012): a visitor creating an account, and
//! anyone signing in through a sign-in service.
//!
//! * `GET|POST /register` — the services first, then a small form, as the
//!   site's registration policy allows;
//! * `POST /auth/:provider/start` and `GET /auth/:provider/callback` — the
//!   authorization-code flow with PKCE (`crate::sso`);
//! * `GET|POST /register/finish` — the registration secret, asked after a
//!   service vouched for somebody the site does not know yet.
//!
//! # What links to what
//!
//! A service's answer finds its account by the identity it carries first
//! (provider, issuer, subject), then — when the policy allows it — by an
//! address the service says it *verified*, and only an account whose address
//! an administrator typed or a service verified: a visitor registering with
//! the form gives no address at all, so nobody can claim another person's
//! address and wait for their Google account to be linked to theirs.

use axum::extract::{Form, Path, Query, State};
use axum::http::{header, HeaderMap, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Redirect, Response};
use minijinja::context;
use serde::Deserialize;
use serde_json::json;

use crate::acl::{Identity, Role, Status};
use crate::events::Kind as Event;
use crate::routes::Shared;
use crate::site::RegistrationMode;
use crate::sso::{self, Intent, Kind};
use crate::{auth, render};

/// How a visitor reaches this page, for the throttle: every attempt to create
/// an account counts, made or refused.
fn register_key(client: &str) -> String {
    format!("register:{client}")
}

fn chrome_for(state: &Shared, headers: &HeaderMap, path: &str) -> render::Chrome {
    render::Chrome::resolve(&auth::viewer(state, headers), headers, path)
}

/// The services a visitor can use here, ready ones only, in the page's order.
pub(crate) fn ready_providers(site: &crate::site::Site) -> Vec<serde_json::Value> {
    site.providers
        .ready()
        .into_iter()
        .map(|k| {
            json!({
                "slug": k.slug(),
                "brand": k.brand(),
                "label": k.label(site.providers.get(k)),
            })
        })
        .collect()
}

/// What the registration page may offer, from the policy alone.
struct Offer {
    mode: RegistrationMode,
    /// The small form: a password account. Not under a password policy that
    /// keeps members off passwords, and not when only some domains may
    /// register — an address typed into a form proves nothing about it.
    form: bool,
    providers: Vec<serde_json::Value>,
}

fn offer(site: &crate::site::Site) -> Offer {
    Offer {
        mode: site.registration.mode,
        form: site.registration.is_open_to_visitors()
            && site.sign_in.password.allows(Role::Viewer)
            && site.sign_in.allowed_domains.is_empty(),
        providers: if site.registration.is_open_to_visitors() {
            ready_providers(site)
        } else {
            Vec::new()
        },
    }
}

fn render_register(
    state: &Shared,
    chrome: &render::Chrome,
    status: StatusCode,
    error: Option<String>,
    username: &str,
) -> Response {
    let site = state.site();
    let o = offer(&site);
    let mut r = render::page_with(
        chrome,
        "register.html",
        context! {
            nav => "entry",
            mode => o.mode.as_str(),
            form => o.form,
            providers => o.providers,
            domains => site.sign_in.allowed_domains.clone(),
            min_password => crate::acl::MIN_PASSWORD,
            error,
            username,
        },
    );
    *r.status_mut() = status;
    r
}

/// `GET /register`
pub async fn register_form(State(state): State<Shared>, headers: HeaderMap) -> Response {
    let viewer = auth::viewer(&state, &headers);
    if viewer.user.is_some() || viewer.emergency {
        return Redirect::to("/").into_response();
    }
    let chrome = chrome_for(&state, &headers, "/register");
    let closed = !state.site().registration.is_open_to_visitors();
    // A closed site says so — the address is printed nowhere, but somebody
    // will type it — rather than offering a form that cannot succeed.
    let status = if closed {
        StatusCode::FORBIDDEN
    } else {
        StatusCode::OK
    };
    render_register(&state, &chrome, status, None, "")
}

#[derive(Debug, Default, Deserialize)]
pub struct RegisterForm {
    #[serde(default)]
    username: String,
    #[serde(default)]
    password: String,
    #[serde(default)]
    secret: String,
}

/// `POST /register` — the small form.
pub async fn register(
    State(state): State<Shared>,
    headers: HeaderMap,
    peer: Option<axum::extract::ConnectInfo<std::net::SocketAddr>>,
    Form(f): Form<RegisterForm>,
) -> Response {
    let chrome = chrome_for(&state, &headers, "/register");
    let client = crate::session::client_key(&headers, peer.map(|c| c.0));
    let key = register_key(&client);
    let site = state.site();
    let refuse = |status: StatusCode, msg: String| {
        render_register(&state, &chrome, status, Some(msg), f.username.trim())
    };
    if !offer(&site).form {
        return refuse(StatusCode::FORBIDDEN, chrome.t("register-form-closed"));
    }
    if state.sessions().is_throttled(&key) {
        return refuse(
            StatusCode::TOO_MANY_REQUESTS,
            chrome.t("register-throttled"),
        );
    }
    state.sessions().record_failure(&key);
    if site.registration.mode == RegistrationMode::Secret
        && !site.registration.secret_matches(&f.secret)
    {
        tracing::warn!(client = %client, "a wrong registration secret was offered");
        return refuse(
            StatusCode::UNPROCESSABLE_ENTITY,
            chrome.t("register-secret-wrong"),
        );
    }
    let username = f.username.trim().to_ascii_lowercase();
    if crate::acl::validate_username(&username).is_err() {
        return refuse(
            StatusCode::UNPROCESSABLE_ENTITY,
            chrome.t("register-username-invalid"),
        );
    }
    if crate::acl::validate_password(&f.password).is_err() {
        return refuse(
            StatusCode::UNPROCESSABLE_ENTITY,
            chrome.t_args(
                "register-password-short",
                &[("min", (crate::acl::MIN_PASSWORD as i64).into())],
            ),
        );
    }
    let mut user = match crate::acl::new_user(&username, &f.password, Role::Viewer) {
        Ok(u) => u,
        Err(e) => {
            tracing::warn!(error = %e, "could not create a registered account");
            return refuse(
                StatusCode::INTERNAL_SERVER_ERROR,
                chrome.t("register-not-saved"),
            );
        }
    };
    user.created_via = Some("register".into());
    user.last_login = Some(crate::view::now_iso8601());
    let id = user.id.clone();
    let mut taken = false;
    if let Err(e) = state.acl_mutate(|acl| {
        if acl.has_username(&user.username) {
            taken = true;
        } else {
            acl.users.push(user);
        }
    }) {
        tracing::warn!(error = %e, "could not save a registered account");
        return refuse(
            StatusCode::INTERNAL_SERVER_ERROR,
            chrome.t("register-not-saved"),
        );
    }
    if taken {
        return refuse(
            StatusCode::UNPROCESSABLE_ENTITY,
            chrome.t("register-username-taken"),
        );
    }
    state.events().record(Event::Registered);
    state.events().record(Event::SignedIn);
    tracing::info!(username = %username, via = "register", "a visitor created an account");
    let cookie = state.sessions().open(Some(id), false);
    (
        StatusCode::SEE_OTHER,
        [
            (
                header::SET_COOKIE,
                crate::session::set_cookie(&cookie, crate::session::is_tls(&headers)),
            ),
            (header::LOCATION, "/".to_string()),
        ],
    )
        .into_response()
}

// ---------------------------------------------------------------------------
// Sign-in services
// ---------------------------------------------------------------------------

#[derive(Debug, Default, Deserialize)]
pub struct StartForm {
    /// `signin` or `register`.
    #[serde(default)]
    intent: String,
    #[serde(default)]
    next: String,
    #[serde(default)]
    secret: String,
}

fn flow_cookie(value: &str, secure: bool, max_age: u64) -> String {
    // Lax, unlike the session's Strict: the provider sends the reader back
    // with a top-level navigation from its own site, and this cookie has to
    // arrive with it. It names a flow, not a session, and dies in minutes.
    let mut s = format!(
        "{}={value}; Path=/auth; HttpOnly; SameSite=Lax; Max-Age={max_age}",
        sso::FLOW_COOKIE
    );
    if secure {
        s.push_str("; Secure");
    }
    s
}

fn pending_cookie(value: &str, secure: bool, max_age: u64) -> String {
    let mut s = format!(
        "{}={value}; Path=/register; HttpOnly; SameSite=Lax; Max-Age={max_age}",
        sso::PENDING_COOKIE
    );
    if secure {
        s.push_str("; Secure");
    }
    s
}

fn sso_refused(chrome: &render::Chrome, status: StatusCode, key: &str) -> Response {
    render::error_page_in(chrome, status, "sso-error-title", key)
}

/// `POST /auth/:provider/start`
pub async fn start(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(slug): Path<String>,
    peer: Option<axum::extract::ConnectInfo<std::net::SocketAddr>>,
    Form(f): Form<StartForm>,
) -> Response {
    let intent = if f.intent == "register" {
        Intent::Register
    } else {
        Intent::SignIn
    };
    let path = if intent == Intent::Register {
        "/register"
    } else {
        "/admin/login"
    };
    let chrome = chrome_for(&state, &headers, path);
    let Some(kind) = Kind::parse(&slug) else {
        return sso_refused(&chrome, StatusCode::NOT_FOUND, "sso-error-not-configured");
    };
    let site = state.site();
    let mut secret_ok = false;
    if intent == Intent::Register {
        if !site.registration.is_open_to_visitors() {
            return render_register(&state, &chrome, StatusCode::FORBIDDEN, None, "");
        }
        if site.registration.mode == RegistrationMode::Secret && !f.secret.trim().is_empty() {
            let client = crate::session::client_key(&headers, peer.map(|c| c.0));
            let key = register_key(&client);
            if state.sessions().is_throttled(&key) {
                return render_register(
                    &state,
                    &chrome,
                    StatusCode::TOO_MANY_REQUESTS,
                    Some(chrome.t("register-throttled")),
                    "",
                );
            }
            secret_ok = site.registration.secret_matches(&f.secret);
            if !secret_ok {
                state.sessions().record_failure(&key);
                return render_register(
                    &state,
                    &chrome,
                    StatusCode::UNPROCESSABLE_ENTITY,
                    Some(chrome.t("register-secret-wrong")),
                    "",
                );
            }
        }
    }
    match sso::begin(state.flows(), &site, kind, intent, &f.next, secret_ok).await {
        Ok((url, flow_state)) => {
            let secure = crate::session::is_tls(&headers) || site.public_is_https();
            let mut r = Redirect::to(&url).into_response();
            if let Ok(v) =
                HeaderValue::from_str(&flow_cookie(&flow_state, secure, sso::FLOW_TTL.as_secs()))
            {
                r.headers_mut().insert(header::SET_COOKIE, v);
            }
            r
        }
        Err(e) => {
            tracing::warn!(provider = kind.slug(), error = %e.detail(), "a sign-in could not start");
            sso_refused(&chrome, StatusCode::BAD_GATEWAY, e.key())
        }
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct CallbackQuery {
    #[serde(default)]
    code: String,
    #[serde(default)]
    state: String,
    #[serde(default)]
    error: String,
}

/// `GET /auth/:provider/callback`
pub async fn callback(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(slug): Path<String>,
    Query(q): Query<CallbackQuery>,
) -> Response {
    let chrome = chrome_for(&state, &headers, "/admin/login");
    let Some(kind) = Kind::parse(&slug) else {
        return sso_refused(&chrome, StatusCode::NOT_FOUND, "sso-error-not-configured");
    };
    let site = state.site();
    let secure = crate::session::is_tls(&headers) || site.public_is_https();
    let cookie_state = crate::session::named_cookie(&headers, sso::FLOW_COOKIE);
    let failed = |status: StatusCode, key: &str| {
        state.events().record(Event::SignInFailed);
        let mut r = sso_refused(&chrome, status, key);
        if let Ok(v) = HeaderValue::from_str(&flow_cookie("", secure, 0)) {
            r.headers_mut().append(header::SET_COOKIE, v);
        }
        r
    };
    if !q.error.is_empty() {
        tracing::info!(provider = kind.slug(), error = %q.error, "the provider sent the reader back without signing in");
        return failed(StatusCode::UNAUTHORIZED, "sso-error-refused");
    }
    let (claims, flow) = match sso::finish(
        state.flows(),
        &site,
        kind,
        &q.state,
        cookie_state.as_deref(),
        &q.code,
    )
    .await
    {
        Ok(v) => v,
        Err(e) => {
            tracing::warn!(provider = kind.slug(), error = %e.detail(), "a sign-in failed");
            let status = match e {
                sso::SsoError::Unreachable(_) => StatusCode::BAD_GATEWAY,
                sso::SsoError::Stale => StatusCode::BAD_REQUEST,
                _ => StatusCode::UNAUTHORIZED,
            };
            return failed(status, e.key());
        }
    };

    // 1. The identity, linked before.
    let linked = state.acl_read(|acl| {
        acl.by_identity(kind.slug(), &claims.issuer, &claims.subject)
            .cloned()
    });
    if let Some(user) = linked {
        if user.status != Status::Active {
            return failed(StatusCode::FORBIDDEN, "sso-account-disabled");
        }
        return signed_in(&state, &chrome, &user.id, &flow.next, secure, None);
    }

    // 2. An address the service verified, on an account an administrator or a
    //    service gave it.
    let verified_email = claims.email.clone().filter(|_| claims.email_verified);
    if site.sign_in.link_by_email {
        if let Some(email) = verified_email.as_deref() {
            if let Some(user) = state.acl_read(|acl| acl.active_by_email(email).cloned()) {
                let identity = identity_of(&claims);
                let id = user.id.clone();
                if let Err(e) = state.acl_mutate(|acl| {
                    if let Some(u) = acl.users.iter_mut().find(|u| u.id == id) {
                        u.identities.push(identity);
                    }
                }) {
                    tracing::warn!(error = %e, "could not link an identity");
                    return failed(StatusCode::INTERNAL_SERVER_ERROR, "sso-error-invalid");
                }
                tracing::info!(username = %user.username, provider = kind.slug(), "linked a sign-in service by its verified address");
                return signed_in(&state, &chrome, &user.id, &flow.next, secure, None);
            }
        }
    }

    // 3. Somebody new.
    if !site.registration.is_open_to_visitors() {
        return failed(StatusCode::FORBIDDEN, "sso-no-account");
    }
    if !site.sign_in.allowed_domains.is_empty()
        && !verified_email
            .as_deref()
            .is_some_and(|e| site.sign_in.domain_allowed(e))
    {
        return failed(StatusCode::FORBIDDEN, "sso-domain-refused");
    }
    if site.registration.mode == RegistrationMode::Secret && !flow.secret_ok {
        // Held, not created: the secret is asked first.
        let token = state.flows().hold(claims, flow.next.clone());
        let mut r = continue_to(&chrome, "/register/finish");
        let h = r.headers_mut();
        if let Ok(v) =
            HeaderValue::from_str(&pending_cookie(&token, secure, sso::FLOW_TTL.as_secs()))
        {
            h.append(header::SET_COOKIE, v);
        }
        if let Ok(v) = HeaderValue::from_str(&flow_cookie("", secure, 0)) {
            h.append(header::SET_COOKIE, v);
        }
        return r;
    }
    match create_from(&state, &claims) {
        Ok(id) => signed_in(
            &state,
            &chrome,
            &id,
            &flow.next,
            secure,
            Some(Event::Registered),
        ),
        Err(()) => failed(StatusCode::INTERNAL_SERVER_ERROR, "sso-error-invalid"),
    }
}

fn identity_of(claims: &sso::Claims) -> Identity {
    Identity {
        provider: claims.kind.slug().to_string(),
        issuer: claims.issuer.clone(),
        subject: claims.subject.clone(),
        email: claims.email.clone().filter(|_| claims.email_verified),
        linked_at: crate::view::now_iso8601(),
    }
}

/// A member's account for a service's user. Its address is kept only when
/// the service verified it.
fn create_from(state: &Shared, claims: &sso::Claims) -> Result<String, ()> {
    let mut created = None;
    let identity = identity_of(claims);
    let email = identity.email.clone();
    let r = state.acl_mutate(|acl| {
        // Raced by a second callback for the same person: link nothing twice.
        if let Some(u) = acl.by_identity(&identity.provider, &identity.issuer, &identity.subject) {
            created = Some(u.id.clone());
            return;
        }
        let username = sso::username_for(claims, |n| acl.has_username(n));
        let user = crate::acl::new_provider_user(&username, identity.clone(), email.clone());
        created = Some(user.id.clone());
        acl.users.push(user);
    });
    match (r, created) {
        (Ok(()), Some(id)) => {
            tracing::info!(
                provider = claims.kind.slug(),
                "a visitor created an account through a sign-in service"
            );
            Ok(id)
        }
        (Err(e), _) => {
            tracing::warn!(error = %e, "could not save an account from a sign-in service");
            Err(())
        }
        _ => Err(()),
    }
}

/// Open the session and continue to `next`.
///
/// Answered with a page that moves on by itself rather than a redirect: the
/// provider's redirect is a navigation from another site, and a browser does
/// not send a SameSite=Strict cookie along a chain that started there. The
/// page's own refresh is a navigation from this site, so the session cookie
/// set here arrives with it.
fn signed_in(
    state: &Shared,
    chrome: &render::Chrome,
    id: &str,
    next: &str,
    secure: bool,
    also: Option<Event>,
) -> Response {
    let id2 = id.to_string();
    if let Err(e) = state.acl_mutate(|acl| {
        if let Some(u) = acl.users.iter_mut().find(|u| u.id == id2) {
            u.last_login = Some(crate::view::now_iso8601());
        }
    }) {
        tracing::warn!(error = %e, "could not record last_login");
    }
    if let Some(ev) = also {
        state.events().record(ev);
    }
    state.events().record(Event::SignedIn);
    let cookie = state.sessions().open(Some(id.to_string()), false);
    let next = render::safe_back(next);
    let next = if next == "/" { "/".to_string() } else { next };
    let mut r = continue_to(chrome, &next);
    let h = r.headers_mut();
    if let Ok(v) = HeaderValue::from_str(&crate::session::set_cookie(&cookie, secure)) {
        h.append(header::SET_COOKIE, v);
    }
    if let Ok(v) = HeaderValue::from_str(&flow_cookie("", secure, 0)) {
        h.append(header::SET_COOKIE, v);
    }
    r
}

/// A page that moves on to `to`, same-site.
fn continue_to(chrome: &render::Chrome, to: &str) -> Response {
    let to = html_escape(&render::safe_back(to));
    let word = html_escape(&chrome.t("sso-continue"));
    let body = format!(
        "<!doctype html><html lang=\"{lang}\"><head><meta charset=\"utf-8\">\
         <meta http-equiv=\"refresh\" content=\"0;url={to}\">\
         <meta name=\"robots\" content=\"noindex\"><title>{word}</title></head>\
         <body><p><a href=\"{to}\">{word}</a></p></body></html>",
        lang = html_escape(chrome.lang),
    );
    (
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, "text/html; charset=utf-8"),
            (header::CACHE_CONTROL, "no-store"),
        ],
        body,
    )
        .into_response()
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

// ---------------------------------------------------------------------------
// The secret, after a service
// ---------------------------------------------------------------------------

fn held(state: &Shared, headers: &HeaderMap) -> Option<(String, sso::PendingAccount)> {
    let token = crate::session::named_cookie(headers, sso::PENDING_COOKIE)?;
    let p = state.flows().held(&token)?;
    Some((token, p))
}

fn render_finish(
    chrome: &render::Chrome,
    status: StatusCode,
    p: &sso::PendingAccount,
    error: Option<String>,
) -> Response {
    let mut r = render::page_with(
        chrome,
        "register_finish.html",
        context! {
            nav => "entry",
            provider => p.claims.kind.slug(),
            email => p.claims.email.clone().filter(|_| p.claims.email_verified),
            name => p.claims.name.clone(),
            error,
        },
    );
    *r.status_mut() = status;
    r
}

/// `GET /register/finish`
pub async fn finish_form(State(state): State<Shared>, headers: HeaderMap) -> Response {
    let chrome = chrome_for(&state, &headers, "/register");
    match held(&state, &headers) {
        Some((_, p)) => render_finish(&chrome, StatusCode::OK, &p, None),
        None => sso_refused(&chrome, StatusCode::BAD_REQUEST, "sso-error-stale"),
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct FinishForm {
    #[serde(default)]
    secret: String,
}

/// `POST /register/finish`
pub async fn finish(
    State(state): State<Shared>,
    headers: HeaderMap,
    peer: Option<axum::extract::ConnectInfo<std::net::SocketAddr>>,
    Form(f): Form<FinishForm>,
) -> Response {
    let chrome = chrome_for(&state, &headers, "/register");
    let Some((token, p)) = held(&state, &headers) else {
        return sso_refused(&chrome, StatusCode::BAD_REQUEST, "sso-error-stale");
    };
    let site = state.site();
    if !site.registration.is_open_to_visitors() {
        state.flows().release(&token);
        return sso_refused(&chrome, StatusCode::FORBIDDEN, "sso-no-account");
    }
    let client = crate::session::client_key(&headers, peer.map(|c| c.0));
    let key = register_key(&client);
    if state.sessions().is_throttled(&key) {
        return render_finish(
            &chrome,
            StatusCode::TOO_MANY_REQUESTS,
            &p,
            Some(chrome.t("register-throttled")),
        );
    }
    if site.registration.mode == RegistrationMode::Secret
        && !site.registration.secret_matches(&f.secret)
    {
        state.sessions().record_failure(&key);
        return render_finish(
            &chrome,
            StatusCode::UNPROCESSABLE_ENTITY,
            &p,
            Some(chrome.t("register-secret-wrong")),
        );
    }
    state.sessions().record_failure(&key);
    state.flows().release(&token);
    let secure = crate::session::is_tls(&headers) || site.public_is_https();
    match create_from(&state, &p.claims) {
        Ok(id) => {
            let mut r = signed_in(
                &state,
                &chrome,
                &id,
                &p.next,
                secure,
                Some(Event::Registered),
            );
            if let Ok(v) = HeaderValue::from_str(&pending_cookie("", secure, 0)) {
                r.headers_mut().append(header::SET_COOKIE, v);
            }
            r
        }
        Err(()) => sso_refused(
            &chrome,
            StatusCode::INTERNAL_SERVER_ERROR,
            "sso-error-invalid",
        ),
    }
}
