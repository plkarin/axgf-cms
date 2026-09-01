//! Setting the interface language and the theme.
//!
//! Both are plain form POSTs that set a cookie and redirect back to where the
//! reader was. No JavaScript: the whole application works with scripting off,
//! and a preference control that did not would be the first thing to break
//! that promise — and it would break it for the reader most likely to have
//! scripting disabled or a text browser, which is not a coincidence.
//!
//! A signed-in account gets the choice written to the ACL as well, so it
//! follows them to another browser. The cookie is still set in that case: it
//! costs nothing and it keeps the choice working for the rest of the session
//! if the ACL write fails.

use axum::extract::{Form, State};
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::{IntoResponse, Redirect, Response};
use serde::Deserialize;

use crate::routes::Shared;
use crate::{auth, render};

#[derive(Deserialize)]
pub struct LanguageForm {
    #[serde(default)]
    lang: String,
    #[serde(default)]
    back: String,
}

#[derive(Deserialize)]
pub struct WashForm {
    /// Absent when the checkbox is unticked, which is how a checkbox reports
    /// "off" in a form post. That is the whole of the parsing.
    #[serde(default)]
    wash: Option<String>,
    #[serde(default)]
    back: String,
}

#[derive(Deserialize)]
pub struct ThemeForm {
    #[serde(default)]
    theme: String,
    #[serde(default)]
    back: String,
}

/// `POST /prefs/language`
pub async fn language(
    State(state): State<Shared>,
    headers: HeaderMap,
    Form(f): Form<LanguageForm>,
) -> Response {
    let Some(locale) = crate::i18n::Locale::find(&f.lang) else {
        return bad_choice(&state, &headers);
    };
    let viewer = auth::viewer(&state, &headers);
    store_preference(&state, &viewer, |u| {
        u.language = Some(locale.tag.to_string())
    });
    respond(&headers, crate::i18n::COOKIE_NAME, locale.tag, &f.back)
}

/// `POST /prefs/theme`
pub async fn theme(
    State(state): State<Shared>,
    headers: HeaderMap,
    Form(f): Form<ThemeForm>,
) -> Response {
    let Some(theme) = crate::theme::Theme::find(&f.theme) else {
        return bad_choice(&state, &headers);
    };
    let viewer = auth::viewer(&state, &headers);
    store_preference(&state, &viewer, |u| u.theme = Some(theme.id.to_string()));
    respond(&headers, crate::theme::COOKIE_NAME, theme.id, &f.back)
}

/// `POST /prefs/background`
///
/// The wash is a preference like the theme and the language, stored the same
/// way and in the same place, so a reader who turns it off has turned it off
/// on every machine they sign in from.
pub async fn background(
    State(state): State<Shared>,
    headers: HeaderMap,
    Form(f): Form<WashForm>,
) -> Response {
    let on = f.wash.is_some();
    let viewer = auth::viewer(&state, &headers);
    store_preference(&state, &viewer, move |u| u.backgrounds = Some(on));
    let value = if on { "on" } else { crate::theme::WASH_OFF };
    respond(&headers, crate::theme::WASH_COOKIE_NAME, value, &f.back)
}

/// Persist the choice on the account, when there is one behind this request.
///
/// A failure is logged and not surfaced: the cookie has already been set by
/// the time this matters, so the reader's choice works either way, and telling
/// them their language change failed when the page in front of them just
/// changed language would be a worse lie than saying nothing.
fn store_preference(
    state: &Shared,
    viewer: &crate::access::Viewer,
    apply: impl FnOnce(&mut crate::acl::User),
) {
    let Some(id) = viewer.user.as_ref().map(|u| u.id.clone()) else {
        return;
    };
    if let Err(e) = state.acl_mutate(move |acl| {
        if let Some(u) = acl.users.iter_mut().find(|u| u.id == id) {
            apply(u);
        }
    }) {
        tracing::warn!(error = %e, "could not store a preference on the account");
    }
}

/// Set the cookie and send the reader back where they were.
fn respond(headers: &HeaderMap, name: &str, value: &str, back: &str) -> Response {
    // A year: this is a preference, not a session, and a reader who picks
    // Arabic should not have to pick it again next month.
    let mut cookie = format!("{name}={value}; Path=/; SameSite=Lax; Max-Age=31536000");
    if crate::session::is_tls(headers) {
        cookie.push_str("; Secure");
    }
    // Deliberately *not* HttpOnly. Nothing here is a credential — it is which
    // language to render in — and leaving it readable lets a future
    // progressive enhancement see the choice without a round trip.
    //
    // `SameSite=Lax` rather than `Strict`, unlike the session cookie: a reader
    // following a link to this site from elsewhere should arrive in the
    // language they chose, and there is nothing to protect by refusing.
    (
        [(header::SET_COOKIE, cookie)],
        Redirect::to(&safe_back(back)),
    )
        .into_response()
}

/// A choice that is not on the menu.
///
/// Rendered rather than silently ignored: a selector that appears to accept an
/// unknown value and then does nothing is harder to diagnose than one that
/// says no.
fn bad_choice(state: &Shared, headers: &HeaderMap) -> Response {
    let viewer = auth::viewer(state, headers);
    let chrome = render::Chrome::resolve(&viewer, headers, "/");
    render::error_page_in(
        &chrome,
        StatusCode::BAD_REQUEST,
        "error-bad-preference-title",
        "error-bad-preference-detail",
    )
}

/// The same-site check as [`render::Chrome::resolve`] applies, repeated here
/// because this is the path where the value actually reaches a `Location`
/// header and an open redirect would be live.
fn safe_back(raw: &str) -> String {
    let candidate = raw.trim();
    if candidate.starts_with('/') && !candidate.starts_with("//") && !candidate.contains('\\') {
        candidate.to_string()
    } else {
        "/".to_string()
    }
}
