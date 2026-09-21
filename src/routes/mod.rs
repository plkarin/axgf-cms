//! HTTP routing.

mod admin;
mod convert;
mod editors;
mod prefs;
mod profile;
mod public;

use std::sync::Arc;

use axum::extract::DefaultBodyLimit;
use axum::http::{header, HeaderValue};
use axum::routing::{get, post};
use axum::Router;
use tower_http::compression::predicate::{NotForContentType, Predicate as _, SizeAbove};
use tower_http::compression::CompressionLayer;
use tower_http::set_header::SetResponseHeaderLayer;

use crate::state::AppState;

/// Shared handler state.
pub type Shared = Arc<AppState>;

/// The Content-Security-Policy every response carries.
///
/// Read directive by directive, because each one is a decision:
///
/// * `default-src 'self'` — nothing loads from anywhere else by default.
/// * `script-src 'self'` — **no `unsafe-inline`**. The six inline `onclick`
///   and `oninput` handlers the templates used to carry are in
///   `/static/ui.js` now precisely so this directive could be written without
///   it. Script injection is the attack this whole header is for; allowing
///   inline script back would give most of it away.
/// * `style-src 'self' 'unsafe-inline'` — inline *styles* stay. Dozens of
///   places compute a width, an offset or a colour from the data and write it
///   into a `style` attribute, and the worst an injected style can do here is
///   make a page ugly. Buying `style-src 'self'` would cost a rewrite of the
///   tree canvas and the radar charts for no meaningful gain.
/// * `img-src 'self' data:` — avatars and the silhouette are data URIs. A
///   configured basemap adds its tile host and nothing else.
/// * `frame-ancestors 'none'` — this application is never framed, which is
///   what stops a clickjacked admin form. `X-Frame-Options: DENY` says the
///   same to browsers too old to read CSP.
/// * `form-action 'self'` — a form cannot be made to post somewhere else.
/// * `base-uri 'none'` — an injected `<base>` cannot re-point every relative
///   URL on the page, including the ones the scripts fetch.
/// * `object-src 'none'` — no plugins, ever.
fn content_security_policy(state: &AppState) -> String {
    // A basemap is fetched by the reader's browser from whatever host the
    // operator configured, so that host — and only that host — is named.
    let tiles = state
        .map()
        .and_then(|m| tile_origin(&m.url))
        .map(|o| format!(" {o}"))
        .unwrap_or_default();
    format!(
        "default-src 'self';          script-src 'self';          style-src 'self' 'unsafe-inline';          img-src 'self' data:{tiles};          font-src 'self';          connect-src 'self'{tiles};          form-action 'self';          frame-ancestors 'none';          base-uri 'none';          object-src 'none'"
    )
}

/// `https://tile.example/{z}/{x}/{y}.png` → `https://tile.example`.
///
/// Only the origin, because that is all CSP wants, and a template with `{z}`
/// in it is not a URL any parser will take whole.
fn tile_origin(template: &str) -> Option<String> {
    let (scheme, rest) = template.split_once("://")?;
    let host = rest.split('/').next()?;
    if host.is_empty() {
        return None;
    }
    Some(format!("{scheme}://{host}"))
}

/// Build the application router.
pub fn router(state: Shared) -> Router {
    let csp = content_security_policy(&state);
    security_headers(routes(state), &csp)
}

/// The security headers, and why each one is here.
fn security_headers(router: Router, csp: &str) -> Router {
    let fixed = |name: header::HeaderName, value: &'static str| {
        SetResponseHeaderLayer::overriding(name, HeaderValue::from_static(value))
    };
    router
        .layer(SetResponseHeaderLayer::overriding(
            header::CONTENT_SECURITY_POLICY,
            HeaderValue::from_str(csp)
                .unwrap_or_else(|_| HeaderValue::from_static("default-src 'self'")),
        ))
        // A browser that guesses a response is HTML when the server said it
        // was text/plain is how an uploaded "note" becomes a script. The
        // document routes set this themselves as well; belt and braces.
        .layer(fixed(header::X_CONTENT_TYPE_OPTIONS, "nosniff"))
        // `frame-ancestors` above is the modern form. This is the one every
        // browser has understood for fifteen years.
        .layer(fixed(header::X_FRAME_OPTIONS, "DENY"))
        // A family tree's URLs name people — `/person/<id>` and the search
        // query alongside it. `same-origin` keeps them off every outbound
        // link, while leaving them on internal navigation where they are
        // needed for the back button and the active-nav highlight.
        .layer(fixed(header::REFERRER_POLICY, "same-origin"))
        // Nothing here uses a camera, a microphone or a location, and saying
        // so costs one header.
        .layer(fixed(
            header::HeaderName::from_static("permissions-policy"),
            "geolocation=(), camera=(), microphone=(), payment=(), usb=()",
        ))
        // Text responses are mostly HTML built from one large stylesheet and
        // repetitive markup, and gzip takes roughly nine tenths off both.
        // Already-compressed payloads — photographs, PDFs, a downloaded
        // bundle — are excluded: re-compressing them spends CPU to make them
        // very slightly larger.
        .layer(
            CompressionLayer::new().gzip(true).compress_when(
                SizeAbove::new(512)
                    .and(NotForContentType::IMAGES)
                    .and(NotForContentType::GRPC)
                    .and(NotForContentType::const_new("application/zip"))
                    .and(NotForContentType::const_new("application/pdf"))
                    .and(NotForContentType::const_new("audio/"))
                    .and(NotForContentType::const_new("video/")),
            ),
        )
}

/// Every route, before the middleware is wrapped around it.
fn routes(state: Shared) -> Router {
    Router::new()
        .route("/", get(public::home))
        .route("/tree", get(public::tree))
        .route("/tree/panel/:id", get(public::tree_panel))
        .route("/person/:id", get(public::person))
        .route("/document/:id/raw", get(public::document_raw))
        .route("/document/:id/view", get(public::document_view))
        .route("/document/:id/thumb", get(public::document_thumb))
        .route("/convert", get(convert::form))
        .route(
            "/convert/gedcom",
            post(convert::gedcom).layer(DefaultBodyLimit::max(
                crate::convert::MAX_UPLOAD + 64 * 1024,
            )),
        )
        .route("/convert/download/:id", get(convert::download))
        .route("/health", get(public::health))
        // Preferences are POSTs so that choosing a language is not something a
        // link or a crawler can do to a reader.
        .route("/settings", get(prefs::settings))
        .route("/prefs/language", post(prefs::language))
        .route("/prefs/theme", post(prefs::theme))
        .route("/prefs/style", post(prefs::style))
        .route("/prefs/background", post(prefs::background))
        // Static admin segments are declared alongside the ":kind" routes;
        // the router matches literal segments in preference to a parameter.
        .route("/admin", get(admin::dashboard))
        .route("/admin/login", get(admin::login_form).post(admin::login))
        .route("/admin/logout", post(admin::logout))
        // Declared before the ":kind" routes so the literal "users" segment
        // wins over the parameter form.
        .route("/admin/users", get(admin::users).post(admin::create_user))
        .route("/admin/users/:id", post(admin::update_user))
        .route("/admin/validate", post(admin::validate))
        .route("/admin/dedup", post(admin::dedup))
        .route("/admin/export", get(admin::export))
        // Declared before the ":kind" routes so the literal "person" segment
        // and the trailing "document" segment win over the parameter forms.
        .route(
            "/admin/person/:id/document",
            post(admin::upload_document).layer(DefaultBodyLimit::max(
                crate::documents::MAX_UPLOAD + 64 * 1024,
            )),
        )
        // The structured Place editor. Declared before the ":kind" forms so
        // the literal "place" segment wins: a place is mostly lists — several
        // names, a border history — and the generic one-input-per-path form
        // cannot express either.
        // Identity: several names with type, script, transliteration, period
        // of use and source, none of which a one-input-per-path form can hold.
        .route(
            "/admin/person/:id/identity",
            get(editors::identity_edit).post(editors::identity_update),
        )
        // Relationships. Every write here lands on a Family entity, never on
        // the person whose page it hangs off.
        .route(
            "/admin/person/:id/family",
            get(editors::family_edit).post(editors::family_create),
        )
        .route(
            "/admin/person/:id/family/:fid",
            post(editors::family_update),
        )
        .route("/admin/person/:id/parents", post(editors::parents_attach))
        // Links and occupations: entities of their own that name a person.
        .route(
            "/admin/person/:id/links",
            get(editors::links_edit).post(editors::links_create),
        )
        .route("/admin/person/:id/links/:lid", post(editors::links_update))
        .route(
            "/admin/person/:id/occupations",
            get(editors::occupations_edit).post(editors::occupations_create),
        )
        .route(
            "/admin/person/:id/occupations/:oid",
            post(editors::occupations_update),
        )
        // Events and attachments.
        .route(
            "/admin/person/:id/events",
            get(editors::events_edit).post(editors::events_create),
        )
        .route(
            "/admin/person/:id/events/:eid",
            post(editors::events_update),
        )
        .route(
            "/admin/person/:id/documents",
            get(editors::documents_edit).post(editors::documents_update),
        )
        // The AXGF 1.1 profile, one form per group. Every save is one
        // `update_entity` on the person, through the same tail as every other
        // editor.
        .route("/admin/person/:id/profile", get(profile::edit_first))
        .route(
            "/admin/person/:id/profile/:group",
            get(profile::edit).post(profile::update),
        )
        // The editor this one replaced, for a bookmark or a link in an old
        // page: its fields are the profile's Morphology and Health now.
        .route("/admin/person/:id/physical", get(profile::edit_first))
        // Choosing which picture stands for a person.
        .route(
            "/admin/person/:id/avatar",
            get(admin::avatar_picker).post(admin::avatar_set),
        )
        .route("/admin/place/:id/edit", get(admin::place_edit))
        .route("/admin/place/:id", post(admin::place_update))
        .route("/admin/place/:id/geocode", post(admin::place_geocode))
        .route("/admin/:kind", get(admin::list).post(admin::create))
        .route("/admin/:kind/new", get(admin::new_form))
        .route("/admin/:kind/:id/edit", get(admin::edit_form))
        .route("/admin/:kind/:id", post(admin::update))
        .route("/admin/:kind/:id/delete", post(admin::delete))
        .route("/static/app.css", get(public::css))
        .route("/static/tree.js", get(public::tree_js))
        .route("/static/map.js", get(public::map_js))
        .route("/static/avatar.js", get(public::avatar_js))
        .route("/static/profile.js", get(public::profile_js))
        .route("/static/ui.js", get(public::ui_js))
        .route("/static/vendor/leaflet.js", get(public::leaflet_js))
        .route("/static/vendor/leaflet.css", get(public::leaflet_css))
        .fallback(public::not_found)
        .with_state(state)
}
