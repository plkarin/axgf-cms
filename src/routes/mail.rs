//! The visitor pages of mail (docs/adr/0048 §5): the forgotten password, and
//! the page a mailed link opens to choose a password. Under `/mail/`, outside
//! every part of the site, so a closed site still serves them; `no-store`,
//! since a link's token is in their address.

use std::time::Duration;

use axum::extract::{Form, Path, State};
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::{IntoResponse, Redirect, Response};
use minijinja::context;
use serde::Deserialize;

use crate::jail::Way;
use crate::mail_tokens::{self, Purpose, Verdict};
use crate::routes::Shared;
use crate::{auth, render};

const QUARTER: Duration = Duration::from_secs(15 * 60);
const HOUR: Duration = Duration::from_secs(60 * 60);
const DAY: Duration = Duration::from_secs(24 * 60 * 60);

/// "Forgot your password?" asked from one client, at most, a quarter of an
/// hour.
const ASKS_PER_CLIENT: usize = 8;

/// The limits of one message with a link to `address`: one a quarter of an
/// hour, five a day.
fn link_limits(address: &str) -> [crate::mail::Limit; 2] {
    let to = address.to_lowercase();
    [
        (format!("link-quarter:{to}"), 1, QUARTER),
        (format!("link-day:{to}"), 5, DAY),
    ]
}

fn chrome_for(state: &Shared, headers: &HeaderMap, path: &str) -> render::Chrome {
    render::Chrome::resolve(&auth::viewer(state, headers), headers, path).with_archive(state)
}

/// The client's bucket, and its address as the journal writes it.
fn client_of(
    headers: &HeaderMap,
    peer: Option<axum::extract::ConnectInfo<std::net::SocketAddr>>,
) -> (String, String) {
    let peer = peer.map(|c| c.0);
    let addr = crate::session::client_addr(headers, peer)
        .map_or_else(|| "unknown".to_string(), |ip| ip.to_string());
    (crate::session::client_key(headers, peer), addr)
}

/// Whether the site can send a link: a relay, and a public address to build
/// it on — never the request's `Host`, which anyone can forge.
pub(super) fn can_send_links(state: &Shared) -> bool {
    let site = state.site();
    state.mailer().relay(&site).is_some() && site.public_base().is_some()
}

/// The services `user` signs in with, by the names the sign-in page shows.
pub(super) fn services_of(site: &crate::site::Site, user: &crate::acl::User) -> String {
    let mut names: Vec<String> = user
        .identities
        .iter()
        .filter_map(|i| crate::sso::Kind::parse(&i.provider))
        .map(|k| k.label(site.providers.get(k)))
        .collect();
    names.dedup();
    names.join(", ")
}

/// What every answer under `/mail/` says besides itself: kept by no cache,
/// and its address — a link's token — sent on by no request it leads to.
/// The site's `same-origin` would hand the whole address to the page's own
/// stylesheets and scripts, and so to the proxy's access log; `strict-origin`
/// gives them the site's origin alone, and still names it on the form's post,
/// which the check against posts from other sites needs (`no-referrer` would
/// send `Origin: null`).
const PRIVATE: [(header::HeaderName, &str); 2] = [
    (header::CACHE_CONTROL, "no-store"),
    (header::REFERRER_POLICY, "strict-origin"),
];

fn page(
    status: StatusCode,
    chrome: &render::Chrome,
    name: &str,
    ctx: minijinja::Value,
) -> Response {
    (status, PRIVATE, render::page_with(chrome, name, ctx)).into_response()
}

#[derive(Deserialize)]
pub struct ForgotForm {
    #[serde(default)]
    who: String,
}

/// `GET /mail/forgot`
pub async fn forgot_form(State(state): State<Shared>, headers: HeaderMap) -> Response {
    let chrome = chrome_for(&state, &headers, "/mail/forgot");
    page(
        StatusCode::OK,
        &chrome,
        "mail_forgot.html",
        context! { nav => "entry", available => can_send_links(&state), sent => false },
    )
}

/// `POST /mail/forgot` — the same page whatever was typed, and before
/// anything is looked up: the lookup, the link and the message happen after
/// the answer, so neither it nor its timing says whether an account exists.
pub async fn forgot(
    State(state): State<Shared>,
    headers: HeaderMap,
    peer: Option<axum::extract::ConnectInfo<std::net::SocketAddr>>,
    Form(f): Form<ForgotForm>,
) -> Response {
    let chrome = chrome_for(&state, &headers, "/mail/forgot");
    let available = can_send_links(&state);
    if available {
        let (client, _) = client_of(&headers, peer);
        let asks = [(format!("forgot:{client}"), ASKS_PER_CLIENT, QUARTER)];
        if !state.mailer().take_all(&asks) {
            tracing::debug!("a forgotten password asked too often from one client; nothing sent");
        } else {
            let who = f.who.trim().to_string();
            if !who.is_empty() && who.len() <= 254 {
                let lang = chrome.lang;
                let state = state.clone();
                tokio::spawn(async move { send_reset(&state, &who, lang) });
            }
        }
    }
    page(
        StatusCode::OK,
        &chrome,
        "mail_forgot.html",
        context! { nav => "entry", available, sent => available },
    )
}

/// Look `who` up — a user name, else an address only one active account
/// holds — and send what fits: a link to an account with a password its role
/// may use, the name of its services to one that signs in through them,
/// nothing to anyone else. Counted per account and per address first.
fn send_reset(state: &Shared, who: &str, request_lang: &'static str) {
    let site = state.site();
    let (Some(relay), Some(base)) = (state.mailer().relay(&site), site.public_base()) else {
        return;
    };
    let Some(user) = state.acl_read(|acl| {
        acl.active(who)
            .cloned()
            .or_else(|| acl.active_by_email(who).cloned())
    }) else {
        return;
    };
    let Some(address) = user.email.clone().filter(|e| crate::mail::valid_address(e)) else {
        return;
    };
    let with_password = crate::acl::has_password(&user) && site.sign_in.password.allows(user.role);
    if !with_password && user.identities.is_empty() {
        return;
    }
    let [quarter, day] = link_limits(&address);
    let limits = [
        (format!("reset-hour:{}", user.id), 3usize, HOUR),
        (format!("reset-day:{}", user.id), 6, DAY),
        quarter,
        day,
    ];
    if !state.mailer().take_all(&limits) {
        tracing::info!(account = %user.id, "forgotten password: a limit is reached; nothing sent");
        return;
    }
    let head = crate::mail_messages::letterhead(state, Some(&user), Some(request_lang));
    let sign_in = format!("{base}/admin/login");
    let (shape, kind) = if with_password {
        let token = mail_tokens::mint(
            state.mail_key(),
            &user,
            Purpose::Reset,
            time::OffsetDateTime::now_utc(),
        );
        let link = format!("{base}/mail/set/{token}");
        (
            crate::mail_messages::reset(&head, &user.username, &link),
            "reset",
        )
    } else {
        let services = services_of(&site, &user);
        (
            crate::mail_messages::service_only(&head, &user.username, &services, &sign_in),
            "service",
        )
    };
    let message = crate::mail_messages::outgoing(&shape, &head, &address, kind);
    // Not handed over — paused, or past the day's ceiling: the limits are
    // given back, so that asking again once mail flows is not refused.
    if state
        .mailer()
        .send_later(relay, message, Some(user.id.clone()), site.mail.ceiling())
        .is_err()
    {
        state.mailer().refund(&limits);
    }
}

/// Tell the account `id` its password changed — by itself through a link, or
/// by an administrator. Nothing when there is no relay, no public address or
/// no address to write to.
pub(crate) fn tell_password_changed(state: &Shared, id: &str, by_admin: bool) {
    let site = state.site();
    let (Some(relay), Some(base)) = (state.mailer().relay(&site), site.public_base()) else {
        return;
    };
    let Some(user) = state.acl_read(|acl| acl.by_id(id).cloned()) else {
        return;
    };
    let Some(address) = user.email.clone().filter(|e| crate::mail::valid_address(e)) else {
        return;
    };
    let head = crate::mail_messages::letterhead(state, Some(&user), None);
    let shape = crate::mail_messages::password_changed(
        &head,
        &user.username,
        &crate::mail_messages::when_now(),
        by_admin,
        &format!("{base}/admin/login"),
    );
    let message = crate::mail_messages::outgoing(&shape, &head, &address, "changed");
    let _ = state
        .mailer()
        .send_later(relay, message, Some(user.id.clone()), site.mail.ceiling());
}

/// The services the site offers for signing in, by their names.
fn ready_services(site: &crate::site::Site) -> String {
    site.providers
        .ready()
        .into_iter()
        .map(|k| k.label(site.providers.get(k)))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Whether one more message with a link may go to `address`; if so the
/// limits counted, to give back should it not be handed over.
fn may_write_to(state: &Shared, address: &str) -> Option<[crate::mail::Limit; 2]> {
    let limits = link_limits(address);
    state.mailer().take_all(&limits).then_some(limits)
}

/// Hand `message` over for `id`, giving the limits back if it is not.
fn hand_over(
    state: &Shared,
    relay: crate::mail::Relay,
    message: crate::mail::Outgoing,
    id: &str,
    limits: &[crate::mail::Limit],
) -> bool {
    let ceiling = state.site().mail.ceiling();
    let handed = state
        .mailer()
        .send_later(relay, message, Some(id.to_string()), ceiling)
        .is_ok();
    if !handed {
        state.mailer().refund(limits);
    }
    handed
}

/// What a welcome carried, if one was handed over.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Welcome {
    /// A link to choose the account's password.
    Link,
    /// The services to sign in with: its role may not use a password.
    Services,
    /// Nothing: no relay, no public address, no address, a message to it a
    /// moment ago, no way in to offer, or mail held back.
    NotSent,
}

/// Welcome the account `id`, made by an administrator (docs/adr/0048 §6): a
/// link to choose its password when its role may use one, else the services
/// to sign in with.
pub(crate) fn send_welcome(state: &Shared, id: &str) -> Welcome {
    let site = state.site();
    let (Some(relay), Some(base)) = (state.mailer().relay(&site), site.public_base()) else {
        return Welcome::NotSent;
    };
    let Some(user) = state.acl_read(|acl| acl.by_id(id).cloned()) else {
        return Welcome::NotSent;
    };
    let Some(address) = user.email.clone().filter(|e| crate::mail::valid_address(e)) else {
        return Welcome::NotSent;
    };
    let head = crate::mail_messages::letterhead(state, Some(&user), None);
    let sign_in = format!("{base}/admin/login");
    let with_link = site.sign_in.password.allows(user.role);
    let shape = if with_link {
        let token = mail_tokens::mint(
            state.mail_key(),
            &user,
            Purpose::Welcome,
            time::OffsetDateTime::now_utc(),
        );
        let days = Purpose::Welcome.lifetime(&user).whole_days();
        crate::mail_messages::welcome_with_link(
            &head,
            &user.username,
            &format!("{base}/mail/set/{token}"),
            days,
        )
    } else {
        let services = ready_services(&site);
        if services.is_empty() {
            return Welcome::NotSent;
        }
        crate::mail_messages::welcome_with_services(&head, &user.username, &services, &sign_in)
    };
    let Some(limits) = may_write_to(state, &address) else {
        tracing::info!(account = %user.id, "welcome not sent: the address had one a moment ago");
        return Welcome::NotSent;
    };
    let message = crate::mail_messages::outgoing(&shape, &head, &address, "welcome");
    match (
        hand_over(state, relay, message, &user.id, &limits),
        with_link,
    ) {
        (false, _) => Welcome::NotSent,
        (true, true) => Welcome::Link,
        (true, false) => Welcome::Services,
    }
}

/// Welcome an account made through a sign-in service, at the address the
/// service verified: which service, and where to sign in.
pub(crate) fn send_service_welcome(state: &Shared, id: &str) -> bool {
    let site = state.site();
    let (Some(relay), Some(base)) = (state.mailer().relay(&site), site.public_base()) else {
        return false;
    };
    let Some(user) = state.acl_read(|acl| acl.by_id(id).cloned()) else {
        return false;
    };
    let Some(address) = user.email.clone().filter(|e| crate::mail::valid_address(e)) else {
        return false;
    };
    let services = services_of(&site, &user);
    if services.is_empty() {
        return false;
    }
    let Some(limits) = may_write_to(state, &address) else {
        return false;
    };
    let head = crate::mail_messages::letterhead(state, Some(&user), None);
    let shape = crate::mail_messages::welcome_with_services(
        &head,
        &user.username,
        &services,
        &format!("{base}/admin/login"),
    );
    let message = crate::mail_messages::outgoing(&shape, &head, &address, "welcome");
    hand_over(state, relay, message, &user.id, &limits)
}

/// A reset link an administrator sends from the accounts page, to an account
/// that has signed in before. Whether a message was handed over.
pub(crate) fn send_reset_to(state: &Shared, id: &str) -> bool {
    let site = state.site();
    let (Some(relay), Some(base)) = (state.mailer().relay(&site), site.public_base()) else {
        return false;
    };
    let Some(user) = state.acl_read(|acl| acl.by_id(id).cloned()) else {
        return false;
    };
    let Some(address) = user.email.clone().filter(|e| crate::mail::valid_address(e)) else {
        return false;
    };
    if !(crate::acl::has_password(&user) && site.sign_in.password.allows(user.role)) {
        return false;
    }
    let Some(limits) = may_write_to(state, &address) else {
        return false;
    };
    let head = crate::mail_messages::letterhead(state, Some(&user), None);
    let token = mail_tokens::mint(
        state.mail_key(),
        &user,
        Purpose::Reset,
        time::OffsetDateTime::now_utc(),
    );
    let shape =
        crate::mail_messages::reset(&head, &user.username, &format!("{base}/mail/set/{token}"));
    let message = crate::mail_messages::outgoing(&shape, &head, &address, "reset");
    hand_over(state, relay, message, &user.id, &limits)
}

#[derive(Deserialize)]
pub struct SetForm {
    #[serde(default)]
    password: String,
    #[serde(default)]
    again: String,
}

/// The page a link opens, in each of its states.
fn render_set(
    state: &Shared,
    chrome: &render::Chrome,
    token: &str,
    verdict: &Verdict,
    error: Option<String>,
    status: StatusCode,
) -> Response {
    let min = chrome.t_args(
        "register-password-short",
        &[("min", (crate::acl::MIN_PASSWORD as i64).into())],
    );
    let (usable, heading, lede, message) = match verdict {
        Verdict::Good { id, purpose } => {
            let username = state
                .acl_read(|acl| acl.by_id(id).map(|u| u.username.clone()))
                .unwrap_or_default();
            let (title, lede) = match purpose {
                Purpose::Reset => ("set-title-reset", "set-lede-reset"),
                Purpose::Welcome => ("set-title-welcome", "set-lede-welcome"),
            };
            (
                true,
                chrome.t(title),
                chrome.t_args(lede, &[("username", username.into())]),
                String::new(),
            )
        }
        Verdict::Expired => (
            false,
            chrome.t("set-title-reset"),
            String::new(),
            chrome.t("set-expired"),
        ),
        Verdict::Forged | Verdict::Stale => (
            false,
            chrome.t("set-title-reset"),
            String::new(),
            chrome.t("set-stale"),
        ),
    };
    page(
        status,
        chrome,
        "mail_set.html",
        context! { nav => "entry", usable, heading, lede, message, token, error, min_note => min, min => crate::acl::MIN_PASSWORD },
    )
}

/// `GET /mail/set/:token` — shows the form; spends nothing, charges nothing.
pub async fn set_form(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(token): Path<String>,
) -> Response {
    let chrome = chrome_for(&state, &headers, "/mail/set");
    let now = time::OffsetDateTime::now_utc();
    let verdict = state.acl_read(|acl| mail_tokens::verify(state.mail_key(), &token, acl, now));
    render_set(&state, &chrome, &token, &verdict, None, StatusCode::OK)
}

/// `POST /mail/set/:token` — sets the password. The token is checked before
/// the password is hashed, and again in the write that stores the hash, so two
/// posts of one link cannot both win. A token never made here counts in the
/// jail; one that was good once does not.
pub async fn set(
    State(state): State<Shared>,
    headers: HeaderMap,
    peer: Option<axum::extract::ConnectInfo<std::net::SocketAddr>>,
    Path(token): Path<String>,
    Form(f): Form<SetForm>,
) -> Response {
    let chrome = chrome_for(&state, &headers, "/mail/set");
    let now = time::OffsetDateTime::now_utc();
    let verdict = state.acl_read(|acl| mail_tokens::verify(state.mail_key(), &token, acl, now));
    let Verdict::Good { id, purpose } = verdict.clone() else {
        if verdict == Verdict::Forged {
            let (client, addr) = client_of(&headers, peer);
            state
                .sessions()
                .jail()
                .record_failure(&client, &addr, Way::Password);
        }
        return render_set(
            &state,
            &chrome,
            &token,
            &verdict,
            None,
            StatusCode::BAD_REQUEST,
        );
    };
    if f.password != f.again {
        let error = Some(chrome.t("set-mismatch"));
        return render_set(
            &state,
            &chrome,
            &token,
            &verdict,
            error,
            StatusCode::UNPROCESSABLE_ENTITY,
        );
    }
    if crate::acl::validate_password(&f.password).is_err() {
        let error = Some(chrome.t_args(
            "register-password-short",
            &[("min", (crate::acl::MIN_PASSWORD as i64).into())],
        ));
        return render_set(
            &state,
            &chrome,
            &token,
            &verdict,
            error,
            StatusCode::UNPROCESSABLE_ENTITY,
        );
    }
    let Ok(hash) = crate::acl::hash_password_bounded(f.password.clone()).await else {
        return render_set(
            &state,
            &chrome,
            &token,
            &Verdict::Stale,
            None,
            StatusCode::INTERNAL_SERVER_ERROR,
        );
    };
    let key = state.mail_key();
    let lang = chrome.lang.to_string();
    let written = state.acl_mutate(|acl| {
        let again = mail_tokens::verify(key, &token, acl, time::OffsetDateTime::now_utc());
        if again != verdict {
            return Err(again);
        }
        let policy = acl.site.sign_in.password;
        let Some(user) = acl.users.iter_mut().find(|u| u.id == id) else {
            return Err(Verdict::Stale);
        };
        if !policy.allows(user.role) {
            return Err(Verdict::Stale);
        }
        user.password_hash = hash;
        user.last_login = Some(crate::view::now_iso8601());
        user.seen_language = Some(lang);
        Ok((user.username.clone(), user.role))
    });
    match written {
        Ok(Ok((username, role))) => {
            state.sessions().close_all_for(&id);
            // Under the name the sign-in counts failures under.
            state
                .sessions()
                .clear_account(&username.to_ascii_lowercase());
            let cookie = state.sessions().open(Some(id.clone()), false);
            state.events().record(crate::events::Kind::SignedIn);
            tracing::info!(account = %id, purpose = ?purpose, "a password was set through a mailed link");
            if purpose == Purpose::Reset {
                tell_password_changed(&state, &id, false);
            }
            let secure = crate::session::is_tls(&headers) || state.site().public_is_https();
            let to = if role.may_write() { "/admin" } else { "/" };
            (
                PRIVATE,
                [(
                    header::SET_COOKIE,
                    crate::session::set_cookie(&cookie, secure),
                )],
                Redirect::to(to),
            )
                .into_response()
        }
        Ok(Err(now)) => render_set(&state, &chrome, &token, &now, None, StatusCode::BAD_REQUEST),
        Err(e) => {
            tracing::error!(error = %e, "could not store a password set through a mailed link");
            render_set(
                &state,
                &chrome,
                &token,
                &Verdict::Stale,
                None,
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        }
    }
}
