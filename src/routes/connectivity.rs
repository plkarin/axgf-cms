//! Connectivity (v0.5.0, docs/adr/0012): the site's public address, the
//! sign-in services and the rules every sign-in follows — and, from the
//! Accounts page, the registration policy.
//!
//! Administrators only. A provider's client secret is written here and never
//! shown again: the page says whether one is set, and a blank field keeps it.
//!
//! Two rules keep an administrator from locking every administrator out:
//! a policy that takes passwords away from administrators, or a change to a
//! provider, is refused while no active administrator could still sign in
//! some other way. The emergency token remains, but it is the way back from a
//! mistake, not a reason to allow one.

use axum::extract::{Form, Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Redirect, Response};
use minijinja::context;
use serde::Deserialize;
use serde_json::json;

use super::admin::{require, Need};
use crate::acl::{Acl, Role, Status};
use crate::render;
use crate::routes::Shared;
use crate::site::{PasswordPolicy, RegistrationMode, Site};
use crate::sso::{self, Kind};

macro_rules! admin_only {
    ($state:expr, $headers:expr) => {
        match require(&$state, &$headers, Need::Admin) {
            Ok(v) => v,
            Err(r) => return r,
        }
    };
}

/// Could an administrator still sign in under `site`? With a password while
/// the policy lets administrators use one; otherwise only through a service
/// that is ready and linked to an active administrator.
pub(crate) fn admins_can_sign_in(site: &Site, acl: &Acl) -> bool {
    if site.sign_in.password.allows(Role::Admin) {
        return true;
    }
    let ready: Vec<&str> = site.providers.ready().into_iter().map(Kind::slug).collect();
    acl.users.iter().any(|u| {
        u.status == Status::Active
            && u.role == Role::Admin
            && u.identities
                .iter()
                .any(|i| ready.contains(&i.provider.as_str()))
    })
}

/// `GET /admin/connectivity`
pub async fn connectivity(State(state): State<Shared>, headers: HeaderMap) -> Response {
    let (_viewer, chrome) = admin_only!(state, headers);
    render_connectivity(&state, &chrome, StatusCode::OK, None, None)
}

fn render_connectivity(
    state: &Shared,
    chrome: &render::Chrome,
    status: StatusCode,
    error: Option<String>,
    notice: Option<String>,
) -> Response {
    let site = state.site();
    let base = site.public_base();
    let providers = Kind::ALL
        .into_iter()
        .map(|k| {
            let p = site.providers.get(k);
            json!({
                "slug": k.slug(),
                "brand": k.brand(),
                "name": k.label(p),
                "enabled": p.enabled,
                "ready": p.is_ready(k),
                "client_id": p.client_id,
                // Whether one is set, never what it is.
                "secret_set": !p.client_secret.trim().is_empty(),
                "tenant": p.tenant,
                "issuer": p.issuer,
                "label": p.label,
                "needs_tenant": k == Kind::Microsoft,
                "needs_issuer": k == Kind::Oidc,
                "redirect_uri": base.as_deref().map(|b| sso::callback_url(b, k)),
                "console": k.console_url(),
            })
        })
        .collect::<Vec<_>>();
    let (linked_admins, without_way) = state.acl_read(|acl| {
        let active = acl.users.iter().filter(|u| u.status == Status::Active);
        let linked_admins = active
            .clone()
            .filter(|u| u.role == Role::Admin && !u.identities.is_empty())
            .count();
        // Accounts that a "no passwords" policy would leave with no way in:
        // nothing linked, and no address for a first sign-in to link by.
        let without_way = active
            .filter(|u| {
                u.identities.is_empty() && u.email.as_deref().is_none_or(|e| e.trim().is_empty())
            })
            .count();
        (linked_admins, without_way)
    });
    let ready = site.providers.ready().len();
    let mut r = render::page_with(
        chrome,
        "admin_connectivity.html",
        context! {
            nav => "admin",
            admin_page => "connectivity",
            public_url => site.public_url.clone(),
            public_base => base.clone(),
            public_https => site.public_is_https(),
            providers,
            ready,
            password_policy => site.sign_in.password.as_str(),
            policies => PasswordPolicy::ALL.iter().map(|p| p.as_str()).collect::<Vec<_>>(),
            link_by_email => site.sign_in.link_by_email,
            domains => site.sign_in.allowed_domains.join("\n"),
            registration => site.registration.mode.as_str(),
            linked_admins,
            without_way,
            outbound_note => state.bind().is_some(),
            // Who may open each part of the site (docs/adr/0046).
            door_rows => crate::site::Part::ALL
                .into_iter()
                .map(|p| json!({"part": p.slug(), "level": site.doors.level(p).slug()}))
                .collect::<Vec<_>>(),
            levels => crate::site::Level::ALL.iter().map(|l| l.slug()).collect::<Vec<_>>(),
            // The mail relay (docs/adr/0048).
            mail => mail_view(state, chrome),
            error,
            notice,
        },
    );
    *r.status_mut() = status;
    r
}

/// What the mail section shows: where the relay comes from, whether links
/// can be made, the day's counts, the last success and the last failure apart
/// — and never the password.
fn mail_view(state: &Shared, chrome: &render::Chrome) -> serde_json::Value {
    let site = state.site();
    let m = &site.mail;
    let mailer = state.mailer();
    let ledger = mailer.ledger();
    let today = crate::view::now_iso8601()
        .get(..10)
        .unwrap_or_default()
        .to_string();
    let accepted = ledger.recent.iter().filter(|(_, ok)| *ok).count();
    let refused_count = ledger.recent.len() - accepted;
    let admin_refused = if mailer.set_by_env() || m.host.trim().is_empty() {
        None
    } else {
        crate::mail::Relay::from_settings(m).err().map(|r| r.key())
    };
    let gmail_sender = m.host.trim().eq_ignore_ascii_case("smtp.gmail.com")
        && !m.from_address.trim().is_empty()
        && !m
            .from_address
            .trim()
            .eq_ignore_ascii_case(m.username.trim());
    let test_to = chrome.viewer_email.clone().unwrap_or_default();
    json!({
        "env": mailer.set_by_env(),
        "env_refused": mailer.env_refused().map(|r| r.key()),
        "env_host": mailer.env_relay().map(|r| r.host.clone()),
        "env_from": mailer.env_relay().map(|r| r.from_address.clone()),
        "from_without_url": mailer.from_without_url(),
        "host": m.host,
        "port": if m.port == 0 { String::new() } else { m.port.to_string() },
        "security": m.security.as_str(),
        "securities": ["starttls", "tls", "none"],
        "username": m.username,
        "password_set": !m.password.is_empty(),
        "from_address": m.from_address,
        "from_name": m.from_name,
        "reply_to": m.reply_to,
        "language": m.language,
        "daily_limit": m.ceiling(),
        "ready": mailer.relay(&site).is_some(),
        "admin_refused": admin_refused,
        "gmail_sender_warning": gmail_sender,
        "links": site.public_base().is_some(),
        "paused": ledger.paused,
        "today": if ledger.day == today { ledger.today } else { 0 },
        "last_ok": ledger.last_ok,
        "last_failure": ledger.last_failure,
        "accepted": accepted,
        "refused": refused_count,
        "test_to": test_to,
    })
}

fn refused(state: &Shared, chrome: &render::Chrome, msg: String) -> Response {
    render_connectivity(
        state,
        chrome,
        StatusCode::UNPROCESSABLE_ENTITY,
        Some(msg),
        None,
    )
}

fn saved(state: &Shared, chrome: &render::Chrome, key: &str) -> Response {
    render_connectivity(state, chrome, StatusCode::OK, None, Some(chrome.t(key)))
}

#[derive(Debug, Default, Deserialize)]
pub struct AddressForm {
    #[serde(default)]
    public_url: String,
}

/// `POST /admin/connectivity/address`
pub async fn save_address(
    State(state): State<Shared>,
    headers: HeaderMap,
    Form(f): Form<AddressForm>,
) -> Response {
    let (viewer, chrome) = admin_only!(state, headers);
    let raw = f.public_url.trim();
    let value = if raw.is_empty() {
        String::new()
    } else {
        match crate::site::normalise_base(raw) {
            Some(b) => b,
            None => return refused(&state, &chrome, chrome.t("connectivity-address-invalid")),
        }
    };
    if let Err(e) = state.site_mutate(|s| s.public_url = value.clone()) {
        return refused(
            &state,
            &chrome,
            chrome.t_args("connectivity-not-saved", &[("error", e.to_string().into())]),
        );
    }
    // Where the providers send readers back has changed with it.
    state.flows().forget_discovery();
    tracing::info!(by = viewer.name(), public_url = %value, "public address changed");
    saved(&state, &chrome, "connectivity-address-saved")
}

#[derive(Debug, Default, Deserialize)]
pub struct PolicyForm {
    #[serde(default)]
    password: String,
    #[serde(default)]
    link_by_email: Option<String>,
    #[serde(default)]
    domains: String,
}

/// `POST /admin/connectivity/policy`
pub async fn save_policy(
    State(state): State<Shared>,
    headers: HeaderMap,
    Form(f): Form<PolicyForm>,
) -> Response {
    let (viewer, chrome) = admin_only!(state, headers);
    let Some(policy) = PasswordPolicy::parse(&f.password) else {
        return refused(&state, &chrome, chrome.t("connectivity-policy-unknown"));
    };
    let domains: Vec<String> = f
        .domains
        .split(['\n', ',', ' '])
        .map(|d| d.trim().trim_start_matches('@').to_ascii_lowercase())
        .filter(|d| !d.is_empty())
        .collect();
    if let Some(bad) = domains.iter().find(|d| {
        !d.contains('.')
            || !d
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '-')
    }) {
        return refused(
            &state,
            &chrome,
            chrome.t_args(
                "connectivity-domain-invalid",
                &[("domain", bad.clone().into())],
            ),
        );
    }
    let mut proposed = state.site();
    proposed.sign_in.password = policy;
    proposed.sign_in.link_by_email = f.link_by_email.is_some();
    proposed.sign_in.allowed_domains = domains;
    if !state.acl_read(|acl| admins_can_sign_in(&proposed, acl)) {
        return refused(&state, &chrome, chrome.t("connectivity-policy-lockout"));
    }
    let sign_in = proposed.sign_in.clone();
    if let Err(e) = state.site_mutate(|s| s.sign_in = sign_in) {
        return refused(
            &state,
            &chrome,
            chrome.t_args("connectivity-not-saved", &[("error", e.to_string().into())]),
        );
    }
    tracing::warn!(
        by = viewer.name(),
        password = policy.as_str(),
        "sign-in policy changed"
    );
    saved(&state, &chrome, "connectivity-policy-saved")
}

#[derive(Debug, Default, Deserialize)]
pub struct ProviderForm {
    #[serde(default)]
    enabled: Option<String>,
    #[serde(default)]
    client_id: String,
    /// Blank keeps the one set.
    #[serde(default)]
    client_secret: String,
    #[serde(default)]
    clear_secret: Option<String>,
    #[serde(default)]
    tenant: String,
    #[serde(default)]
    issuer: String,
    #[serde(default)]
    label: String,
}

/// `POST /admin/connectivity/provider/:slug`
pub async fn save_provider(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(slug): Path<String>,
    Form(f): Form<ProviderForm>,
) -> Response {
    let (viewer, chrome) = admin_only!(state, headers);
    let Some(kind) = Kind::parse(&slug) else {
        return refused(&state, &chrome, chrome.t("sso-error-not-configured"));
    };
    let mut proposed = state.site();
    {
        let p = proposed.providers.get_mut(kind);
        p.enabled = f.enabled.is_some();
        p.client_id = f.client_id.trim().to_string();
        if f.clear_secret.is_some() {
            p.client_secret.clear();
        } else if !f.client_secret.trim().is_empty() {
            p.client_secret = f.client_secret.trim().to_string();
        }
        p.tenant = f.tenant.trim().to_string();
        p.issuer = f.issuer.trim().trim_end_matches('/').to_string();
        p.label = f.label.trim().chars().take(60).collect();
    }
    let p = proposed.providers.get(kind).clone();
    if kind == Kind::Oidc && !p.issuer.is_empty() && kind.discovery_url(&p).is_none() {
        return refused(&state, &chrome, chrome.t("connectivity-issuer-invalid"));
    }
    if kind == Kind::Microsoft && !p.tenant.is_empty() && kind.discovery_url(&p).is_none() {
        return refused(&state, &chrome, chrome.t("connectivity-tenant-invalid"));
    }
    if p.enabled && !p.is_ready(kind) {
        return refused(
            &state,
            &chrome,
            chrome.t_args(
                "connectivity-provider-incomplete",
                &[("name", kind.label(&p).into())],
            ),
        );
    }
    if !state.acl_read(|acl| admins_can_sign_in(&proposed, acl)) {
        return refused(&state, &chrome, chrome.t("connectivity-policy-lockout"));
    }
    if let Err(e) = state.site_mutate(|s| *s.providers.get_mut(kind) = p.clone()) {
        return refused(
            &state,
            &chrome,
            chrome.t_args("connectivity-not-saved", &[("error", e.to_string().into())]),
        );
    }
    state.flows().forget_discovery();
    tracing::warn!(
        by = viewer.name(),
        provider = kind.slug(),
        enabled = p.enabled,
        "sign-in service changed"
    );
    render_connectivity(
        &state,
        &chrome,
        StatusCode::OK,
        None,
        Some(chrome.t_args(
            "connectivity-provider-saved",
            &[("name", kind.label(&p).into())],
        )),
    )
}

/// `POST /admin/connectivity/test/:slug` — can this machine reach it?
pub async fn test_provider(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(slug): Path<String>,
) -> Response {
    let (_viewer, chrome) = admin_only!(state, headers);
    let Some(kind) = Kind::parse(&slug) else {
        return refused(&state, &chrome, chrome.t("sso-error-not-configured"));
    };
    let site = state.site();
    let p = site.providers.get(kind).clone();
    match sso::probe(state.flows(), kind, &p).await {
        Ok(found) => render_connectivity(
            &state,
            &chrome,
            StatusCode::OK,
            None,
            Some(chrome.t_args(
                "connectivity-test-ok",
                &[("name", kind.label(&p).into()), ("answer", found.into())],
            )),
        ),
        Err(e) => {
            tracing::info!(provider = kind.slug(), error = %e.detail(), "a sign-in service could not be reached");
            render_connectivity(
                &state,
                &chrome,
                StatusCode::OK,
                Some(chrome.t_args(
                    "connectivity-test-failed",
                    &[
                        ("name", kind.label(&p).into()),
                        ("detail", e.detail().into()),
                    ],
                )),
                None,
            )
        }
    }
}

// ---------------------------------------------------------------------------
// Who may open each part of the site
// ---------------------------------------------------------------------------

/// `POST /admin/connectivity/pages` — who may open each part of the site
/// (docs/adr/0046): each part's level by the part's name (`family`, `river`,
/// `tree`, `records`, `impact`, `import`), anyone, the accounts signed in, the
/// contributors or the administrators. A part the form does not name keeps
/// its level.
pub async fn save_pages(
    State(state): State<Shared>,
    headers: HeaderMap,
    Form(levels): Form<std::collections::BTreeMap<String, String>>,
) -> Response {
    let (viewer, chrome) = admin_only!(state, headers);
    let mut doors = state.site().doors;
    for part in crate::site::Part::ALL {
        let Some(typed) = levels.get(part.slug()) else {
            continue;
        };
        match crate::site::Level::parse(typed) {
            Some(level) => doors.set(part, level),
            None => return refused(&state, &chrome, chrome.t("doors-unknown")),
        }
    }
    let saved = doors.clone();
    if let Err(e) = state.site_mutate(|s| s.doors = saved) {
        return refused(
            &state,
            &chrome,
            chrome.t_args("connectivity-not-saved", &[("error", e.to_string().into())]),
        );
    }
    // Who opened or closed the site, and how; nothing else.
    tracing::warn!(
        by = viewer.name(),
        family = doors.family.slug(),
        river = doors.river.slug(),
        tree = doors.tree.slug(),
        records = doors.records.slug(),
        impact = doors.impact.slug(),
        import = doors.import.slug(),
        "who may open each part of the site changed"
    );
    render_connectivity(
        &state,
        &chrome,
        StatusCode::OK,
        None,
        Some(chrome.t("doors-saved")),
    )
}

// ---------------------------------------------------------------------------
// The registration policy, from the Accounts page
// ---------------------------------------------------------------------------

#[derive(Debug, Default, Deserialize)]
pub struct RegistrationForm {
    #[serde(default)]
    mode: String,
    /// Draw a new secret, which shuts the old one out.
    #[serde(default)]
    rotate: Option<String>,
}

/// `POST /admin/users/registration`
pub async fn save_registration(
    State(state): State<Shared>,
    headers: HeaderMap,
    Form(f): Form<RegistrationForm>,
) -> Response {
    let (viewer, chrome) = admin_only!(state, headers);
    let Some(mode) = RegistrationMode::parse(&f.mode) else {
        let mut r = super::admin::render_users(
            &state,
            &chrome,
            &viewer,
            Some(&chrome.t("registration-unknown")),
            None,
        );
        *r.status_mut() = StatusCode::UNPROCESSABLE_ENTITY;
        return r;
    };
    let by = viewer.name().to_string();
    let rotate = f.rotate.is_some();
    let r = state.site_mutate(|s| {
        let reg = &mut s.registration;
        if mode == RegistrationMode::Secret && (reg.secret.is_empty() || rotate) {
            reg.secret = crate::site::generate_secret();
        }
        reg.mode = mode;
        reg.changed_at = Some(crate::view::now_iso8601());
        reg.changed_by = Some(by.clone());
    });
    if let Err(e) = r {
        let msg = chrome.t_args("accounts-not-saved", &[("error", e.to_string().into())]);
        return super::admin::render_users(&state, &chrome, &viewer, Some(&msg), None);
    }
    tracing::warn!(by = %by, mode = mode.as_str(), rotated = rotate, "registration policy changed");
    // Back to the page, at the section, so a reload does not post it again.
    let _ = &chrome;
    Redirect::to("/admin/users#registration").into_response()
}

// ---------------------------------------------------------------------------
// The mail relay (docs/adr/0048 §1)
// ---------------------------------------------------------------------------

#[derive(Debug, Default, Deserialize)]
pub struct MailForm {
    #[serde(default)]
    host: String,
    #[serde(default)]
    port: String,
    #[serde(default)]
    security: String,
    #[serde(default)]
    username: String,
    #[serde(default)]
    password: String,
    #[serde(default)]
    clear_password: Option<String>,
    #[serde(default)]
    from_address: String,
    #[serde(default)]
    from_name: String,
    #[serde(default)]
    reply_to: String,
    #[serde(default)]
    language: String,
    #[serde(default)]
    daily_limit: String,
}

/// `POST /admin/connectivity/mail` — the relay, checked before it is kept. A
/// blank password keeps the stored one; saving again lifts a pause the relay's
/// refusal of the credentials put on.
pub async fn save_mail(
    State(state): State<Shared>,
    headers: HeaderMap,
    Form(f): Form<MailForm>,
) -> Response {
    use crate::mail::{valid_address, Security};
    let (viewer, chrome) = admin_only!(state, headers);
    // What the section adds to any relay: where replies go, the language for
    // an account without one, the day's ceiling.
    let reply_to = f.reply_to.trim().to_string();
    if !reply_to.is_empty() && !valid_address(&reply_to) {
        return refused(&state, &chrome, chrome.t("mail-reply-to-invalid"));
    }
    let language = match f.language.trim() {
        "" => String::new(),
        l => match crate::i18n::Locale::find(l) {
            Some(locale) => locale.tag.to_string(),
            None => return refused(&state, &chrome, chrome.t("mail-language-invalid")),
        },
    };
    let daily_limit: u32 = match f.daily_limit.trim() {
        "" => 0,
        n => match n.parse::<u32>() {
            Ok(n) if (1..=10_000).contains(&n) => n,
            _ => return refused(&state, &chrome, chrome.t("mail-limit-invalid")),
        },
    };
    // The configuration's relay: its own fields are for show, and only what
    // the section adds is kept.
    if state.mailer().set_by_env() {
        if let Err(e) = state.site_mutate(|s| {
            s.mail.reply_to = reply_to.clone();
            s.mail.language = language.clone();
            s.mail.daily_limit = daily_limit;
        }) {
            return refused(
                &state,
                &chrome,
                chrome.t_args("connectivity-not-saved", &[("error", e.to_string().into())]),
            );
        }
        // Not a pause lifted: the relay's credentials are the configuration's,
        // and only a restart with new ones, or a test that gets through,
        // changes anything for them.
        tracing::info!(
            by = viewer.name(),
            "mail settings beside the configuration's relay changed"
        );
        return saved(&state, &chrome, "mail-saved");
    }
    // A choice of three; anything else is STARTTLS, which never sends a
    // password in the clear.
    let security = Security::parse(&f.security).unwrap_or(Security::Starttls);
    let port: u16 = match f.port.trim() {
        "" => 0,
        p => match p.parse::<u16>() {
            Ok(n) if n > 0 => n,
            _ => return refused(&state, &chrome, chrome.t("mail-port-invalid")),
        },
    };
    let effective = if port == 0 {
        security.default_port()
    } else {
        port
    };
    if (effective == 465 && security == Security::Starttls)
        || (effective == 587 && security == Security::Tls)
    {
        return refused(&state, &chrome, chrome.t("mail-port-mismatch"));
    }
    let host = f.host.trim().to_string();
    let from_address = f.from_address.trim().to_string();
    if !host.is_empty() && !valid_address(&from_address) {
        return refused(&state, &chrome, chrome.t("mail-refused-sender"));
    }
    let old = state.site().mail;
    let mut new = old.clone();
    new.host = host.clone();
    new.port = port;
    new.security = security;
    new.username = f.username.trim().to_string();
    let password_changed = f.clear_password.is_some() || !f.password.trim().is_empty();
    // The stored password goes only to the relay it was given for: a new
    // host, port, security or user name needs it typed again, or a stolen
    // session could point the relay at its own server and press the test.
    let port_of = |m: &crate::mail::Settings| {
        if m.port == 0 {
            m.security.default_port()
        } else {
            m.port
        }
    };
    let elsewhere = new.host != old.host
        || port_of(&new) != port_of(&old)
        || new.security != old.security
        || new.username != old.username;
    if elsewhere && !old.password.is_empty() && !password_changed && !host.is_empty() {
        return refused(&state, &chrome, chrome.t("mail-password-again"));
    }
    if f.clear_password.is_some() {
        new.password.clear();
    } else if !f.password.trim().is_empty() {
        // Gmail prints its app passwords in groups of four.
        new.password = f.password.split_whitespace().collect();
    }
    new.from_address = from_address;
    new.from_name = f.from_name.trim().to_string();
    new.reply_to = reply_to;
    new.language = language;
    new.daily_limit = daily_limit;
    if !host.is_empty() {
        if let Err(r) = crate::mail::Relay::from_settings(&new) {
            return refused(&state, &chrome, chrome.t(r.key()));
        }
    }
    if let Err(e) = state.site_mutate(|s| s.mail = new.clone()) {
        return refused(
            &state,
            &chrome,
            chrome.t_args("connectivity-not-saved", &[("error", e.to_string().into())]),
        );
    }
    // Saving lifts a pause for refused credentials; a pause for the relay's
    // own daily limit lifts only with another relay or account, or a test
    // that gets through.
    if elsewhere {
        state.mailer().resume_all();
    } else {
        state.mailer().resume();
    }
    tracing::info!(by = viewer.name(), host = %host, password_changed, "mail relay changed");
    saved(&state, &chrome, "mail-saved")
}

#[derive(Debug, Default, Deserialize)]
pub struct MailTestForm {
    #[serde(default)]
    to: String,
}

/// `POST /admin/connectivity/mail/test` — the one page that waits on the
/// relay: fifteen seconds at most, outside the queue, and a plain account of
/// what came back.
pub async fn test_mail(
    State(state): State<Shared>,
    headers: HeaderMap,
    Form(f): Form<MailTestForm>,
) -> Response {
    let (viewer, chrome) = admin_only!(state, headers);
    let to = f.to.trim().to_string();
    if !crate::mail::valid_address(&to) {
        return refused(&state, &chrome, chrome.t("mail-test-to-invalid"));
    }
    let site = state.site();
    let Some(relay) = state.mailer().relay(&site) else {
        return refused(&state, &chrome, chrome.t("mail-not-ready"));
    };
    let user = viewer.user.as_ref();
    let head = crate::mail_messages::letterhead(&state, user, Some(chrome.lang));
    let shape = crate::mail_messages::test(&head);
    let message = crate::mail_messages::outgoing(&shape, &head, &to, "test");
    let host = relay.host.clone();
    match state.mailer().send_now(relay, message).await {
        Ok(reply) => render_connectivity(
            &state,
            &chrome,
            StatusCode::OK,
            None,
            Some(chrome.t_args(
                "mail-test-ok",
                &[
                    ("host", host.into()),
                    ("reply", crate::mail::scrub(&reply).into()),
                ],
            )),
        ),
        Err(failure) => {
            tracing::info!(host = %host, failure = failure.kind(), detail = %failure.detail(), "the mail relay test did not get through");
            render_connectivity(
                &state,
                &chrome,
                StatusCode::OK,
                Some(chrome.t_args(
                    &format!("mail-test-failed-{}", failure.kind()),
                    &[("host", host.into()), ("detail", failure.detail().into())],
                )),
                None,
            )
        }
    }
}
