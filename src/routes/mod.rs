//! HTTP routing.

mod account;
mod admin;
mod ai_portraits;
mod connectivity;
mod console;
mod convert;
mod editors;
mod entry;
mod notes;
mod person_editor;
mod prefs;
mod profile;
mod public;
mod setup;

use std::sync::Arc;

use axum::extract::DefaultBodyLimit;
use axum::http::{header, HeaderMap, HeaderValue};
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
    let guarded = routes(state.clone())
        .layer(axum::middleware::from_fn_with_state(
            state,
            same_site_writes,
        ))
        .layer(axum::middleware::from_fn(cache_policy));
    security_headers(guarded, &csp)
}

/// Refuse a form posted from another site (v0.5.0, docs/adr/0011 §2).
///
/// The session cookie is `SameSite=Strict`, and for a long time that was the
/// whole defence. It is not enough here: a *site* is a registrable domain, so
/// every application served from the same host — another port, another
/// virtual host on the same address — is the same site, and a page served by
/// any of them could post this application's forms with an administrator's
/// cookie attached.
///
/// A browser says where a post comes from in `Origin`, on every cross-origin
/// post and, in every current browser, on same-origin ones too. A post whose
/// `Origin` (or, failing that, `Referer`) names another origin is refused. One
/// carrying neither is let through: no browser sends a cross-site form post
/// without them, and a script or a test calling this directly has no cookie
/// worth forging.
async fn same_site_writes(
    axum::extract::State(state): axum::extract::State<Shared>,
    req: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    use axum::http::Method;
    let writes = matches!(
        *req.method(),
        Method::POST | Method::PUT | Method::PATCH | Method::DELETE
    );
    if writes {
        let headers = req.headers();
        let claimed = headers
            .get(header::ORIGIN)
            .and_then(|v| v.to_str().ok())
            .map(str::to_string)
            .or_else(|| {
                headers
                    .get(header::REFERER)
                    .and_then(|v| v.to_str().ok())
                    .and_then(origin_of_url)
            });
        if let Some(origin) = claimed {
            let public = state.site().public_base();
            if !is_own_origin(&origin, headers, public.as_deref()) {
                state.events().record(crate::events::Kind::CrossSiteRefused);
                tracing::warn!(
                    origin = %origin,
                    path = %req.uri().path(),
                    "refused a form posted from another site"
                );
                let viewer = crate::auth::viewer(&state, headers);
                let chrome = crate::render::Chrome::resolve(&viewer, headers, "/");
                return crate::render::error_page_in(
                    &chrome,
                    axum::http::StatusCode::FORBIDDEN,
                    "error-cross-site-title",
                    "error-cross-site-detail",
                );
            }
        }
    }
    next.run(req).await
}

/// `https://a.example:8443/x?y` → `https://a.example:8443`.
fn origin_of_url(url: &str) -> Option<String> {
    let (scheme, rest) = url.split_once("://")?;
    let authority = rest.split(['/', '?', '#']).next()?;
    (!authority.is_empty()).then(|| format!("{scheme}://{authority}"))
}

/// The `host[:port]` an origin names, lowercased, the default port dropped.
fn authority_of(origin: &str) -> Option<String> {
    let (scheme, rest) = origin.trim().split_once("://")?;
    let authority = rest.split(['/', '?', '#']).next()?.to_ascii_lowercase();
    if authority.is_empty() || authority.contains('@') {
        return None;
    }
    let default = match scheme.to_ascii_lowercase().as_str() {
        "https" => ":443",
        "http" => ":80",
        _ => return None,
    };
    Some(
        authority
            .strip_suffix(default)
            .map(str::to_string)
            .unwrap_or(authority),
    )
}

/// Whether `origin` is this site: the host the request was addressed to, the
/// host a reverse proxy says it was addressed to, or the public address the
/// operator set (docs/adr/0012).
fn is_own_origin(origin: &str, headers: &HeaderMap, public: Option<&str>) -> bool {
    let Some(claimed) = authority_of(origin) else {
        // `null` — a sandboxed frame, a `data:` page — is nobody's origin.
        return false;
    };
    let host_of = |name: &str| {
        headers
            .get(name)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.split(',').next())
            .map(|v| v.trim().to_ascii_lowercase())
            .filter(|v| !v.is_empty())
    };
    let strip_default = |h: String| {
        h.strip_suffix(":80")
            .or_else(|| h.strip_suffix(":443"))
            .map(str::to_string)
            .unwrap_or(h)
    };
    let mut ours: Vec<String> = Vec::new();
    if let Some(h) = host_of("host") {
        ours.push(strip_default(h));
    }
    if let Some(h) = host_of("x-forwarded-host") {
        ours.push(strip_default(h));
    }
    if let Some(p) = public.and_then(authority_of) {
        ours.push(p);
    }
    ours.contains(&claimed)
}

/// Say how a page may be kept (v0.5.0, docs/adr/0011 §2).
///
/// A page a signed-in reader saw can hold anything their account may read —
/// a living relative's address, the accounts list — and without a word on the
/// matter a browser keeps it, and shows it again from its history on a shared
/// computer after the reader signed out. Every HTML page that does not say
/// otherwise is `no-store` for a signed-in reader and `no-cache` for anyone
/// else. Stylesheets, scripts and pictures say their own.
async fn cache_policy(
    req: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    let signed_in = crate::session::cookie_value(req.headers()).is_some();
    let mut resp = next.run(req).await;
    let html = resp
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .is_some_and(|v| v.starts_with("text/html"));
    if html && !resp.headers().contains_key(header::CACHE_CONTROL) {
        resp.headers_mut().insert(
            header::CACHE_CONTROL,
            HeaderValue::from_static(if signed_in { "no-store" } else { "no-cache" }),
        );
    }
    resp
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
        // The river is the landing page. The overview that was `/` is at
        // `/about`, and is still what `/` shows for an archive with nobody in
        // it, which has no river to draw.
        .route("/", get(public::river))
        .route("/river/data", get(public::river_data))
        .route("/about", get(public::home))
        .route("/tree", get(public::tree))
        .route("/quality", get(public::quality))
        .route("/quality.csv", get(public::quality_csv))
        .route("/quality.json", get(public::quality_json))
        .route("/quality/rule/:action", post(admin::quality_mark_rule))
        .route("/quality/:action", post(admin::quality_mark))
        .route("/search", get(public::search))
        .route("/search.json", get(public::search_json))
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
        // Notes and contributions (docs/adr/0014): read by the family's
        // members, written from the administration.
        .route("/notes/:id", get(notes::show))
        .route("/notes/:id/pdf", get(notes::pdf))
        .route("/admin/notes", get(notes::list).post(notes::create))
        .route("/admin/notes/new", get(notes::new_form))
        .route(
            "/admin/notes/images",
            post(notes::images).layer(DefaultBodyLimit::max(
                crate::notes::MAX_IMAGE * 4 + 64 * 1024,
            )),
        )
        .route("/admin/notes/preview", post(notes::preview))
        .route("/admin/notes/pending/:token", get(notes::pending))
        .route("/admin/notes/:id/edit", get(notes::edit_form))
        .route(
            "/admin/notes/:id",
            post(notes::update).layer(DefaultBodyLimit::max(
                crate::notes::MAX_TEXT * 4 + 64 * 1024,
            )),
        )
        .route("/admin/notes/:id/delete", post(notes::delete))
        .route("/health", get(public::health))
        // Preferences are POSTs so that choosing a language is not something a
        // link or a crawler can do to a reader.
        .route("/settings", get(prefs::settings))
        .route("/account", get(account::page))
        .route("/account/person", post(account::person))
        .route("/prefs/language", post(prefs::language))
        .route("/prefs/theme", post(prefs::theme))
        .route("/prefs/style", post(prefs::style))
        .route("/prefs/background", post(prefs::background))
        .route("/prefs", post(prefs::all))
        // Static admin segments are declared alongside the ":kind" routes;
        // the router matches literal segments in preference to a parameter.
        .route("/admin", get(admin::dashboard))
        // Setting an installation up (docs/adr/0005): three doors, open while
        // it has no family. Declared before the ":kind" forms so the literal
        // "setup" segment wins.
        .route("/admin/setup", get(setup::page))
        .route(
            "/admin/setup/gedcom",
            post(setup::gedcom).layer(DefaultBodyLimit::max(
                crate::convert::MAX_UPLOAD + 64 * 1024,
            )),
        )
        .route("/admin/setup/gedcom/:id", post(setup::adopt_gedcom))
        .route(
            "/admin/setup/archive",
            post(setup::archive).layer(DefaultBodyLimit::max(setup::MAX_ARCHIVE)),
        )
        .route("/admin/login", get(admin::login_form).post(admin::login))
        .route("/admin/logout", post(admin::logout))
        // Declared before the ":kind" routes so the literal "users" segment
        // wins over the parameter form.
        .route("/admin/users", get(admin::users).post(admin::create_user))
        // The console's own rooms (v0.5.0, docs/adr/0011).
        .route("/admin/bundle", get(console::bundle))
        .route("/admin/bundle/sealed", post(console::sealed))
        .route("/admin/bundle/family", post(console::family))
        .route(
            "/admin/bundle/forget-recipient",
            post(console::forget_recipient),
        )
        .route("/admin/infrastructure", get(console::infrastructure))
        .route(
            "/admin/infrastructure.json",
            get(console::infrastructure_json),
        )
        .route(
            "/admin/infrastructure/releases",
            post(console::refresh_releases),
        )
        .route("/admin/infrastructure/upgrade", post(console::upgrade))
        .route("/admin/connectivity", get(connectivity::connectivity))
        .route(
            "/admin/connectivity/address",
            post(connectivity::save_address),
        )
        .route(
            "/admin/connectivity/policy",
            post(connectivity::save_policy),
        )
        .route(
            "/admin/connectivity/provider/:slug",
            post(connectivity::save_provider),
        )
        .route(
            "/admin/connectivity/test/:slug",
            post(connectivity::test_provider),
        )
        .route(
            "/admin/users/registration",
            post(connectivity::save_registration),
        )
        .route("/register", get(entry::register_form).post(entry::register))
        .route(
            "/register/finish",
            get(entry::finish_form).post(entry::finish),
        )
        .route("/auth/:provider/start", post(entry::start))
        .route("/auth/:provider/callback", get(entry::callback))
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
        // The portraits an AI reworked (docs/adr/0009): up to three pictures
        // in one save, so the body may be three uploads and the fields
        // around them.
        .route(
            "/admin/person/:id/ai-portraits",
            get(ai_portraits::page)
                .post(ai_portraits::save)
                .layer(DefaultBodyLimit::max(
                    crate::ai_portrait::MAX * crate::documents::MAX_UPLOAD + 256 * 1024,
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
        // Before `/admin/place/:id`, which takes POST only and otherwise
        // matched "new" as an id: the place list's "New place" answered 405.
        .route("/admin/place/new", get(admin::new_place_form))
        .route("/admin/place/:id/edit", get(admin::place_edit))
        .route("/admin/place/:id", post(admin::place_update))
        .route("/admin/place/:id/geocode", post(admin::place_geocode))
        // The record's editor (v0.4.2): six tabs over one form, one save —
        // which may carry a file from the Médias tab, so its body may be as
        // large as an upload and the fields around it.
        .route("/admin/person/new", get(person_editor::new_form))
        .route("/admin/person/bulk", post(person_editor::bulk))
        .route("/admin/person/:id/edit", get(person_editor::edit))
        .route(
            "/admin/person/:id",
            post(person_editor::save).layer(DefaultBodyLimit::max(
                crate::documents::MAX_UPLOAD + 4 * 1024 * 1024,
            )),
        )
        .route("/admin/:kind", get(admin::list).post(admin::create))
        .route("/admin/:kind/new", get(admin::new_form))
        .route("/admin/:kind/:id/edit", get(admin::edit_form))
        .route("/admin/:kind/:id", post(admin::update))
        .route("/admin/:kind/:id/delete", post(admin::delete))
        .route("/admin/:kind/bulk", post(admin::bulk))
        .route("/static/css/tokens.css", get(public::tokens_css))
        .route("/static/app.css", get(public::css))
        .route(
            "/static/img/canopee-hero.jpg",
            get(public::canopee_hero_jpg),
        )
        .route("/static/img/canopee-cta.jpg", get(public::canopee_cta_jpg))
        .route("/static/img/fusion-hero.jpg", get(public::fusion_hero_jpg))
        .route("/static/img/fusion-cta.jpg", get(public::fusion_cta_jpg))
        .route("/static/img/fusion-lab.jpg", get(public::fusion_lab_jpg))
        .route("/static/img/galaxy-hero.jpg", get(public::galaxy_hero_jpg))
        .route("/static/img/galaxy-cta.jpg", get(public::galaxy_cta_jpg))
        .route("/static/img/summit-hero.jpg", get(public::summit_hero_jpg))
        .route("/static/img/summit-cta.jpg", get(public::summit_cta_jpg))
        .route(
            "/static/img/clarity-hero.jpg",
            get(public::clarity_hero_jpg),
        )
        .route("/static/img/clarity-cta.jpg", get(public::clarity_cta_jpg))
        .route(
            "/static/img/genealogy-hero.jpg",
            get(public::genealogy_hero_jpg),
        )
        .route(
            "/static/img/genealogy-cta.jpg",
            get(public::genealogy_cta_jpg),
        )
        .route(
            "/static/img/genealogy-vine-l.jpg",
            get(public::genealogy_vine_l_jpg),
        )
        .route(
            "/static/img/genealogy-vine-r.jpg",
            get(public::genealogy_vine_r_jpg),
        )
        .route(
            "/static/img/genealogy-tree.jpg",
            get(public::genealogy_tree_jpg),
        )
        .route(
            "/static/img/genealogy-tree-depth.jpg",
            get(public::genealogy_tree_depth_jpg),
        )
        .route(
            "/static/img/summit-panorama.jpg",
            get(public::summit_panorama_jpg),
        )
        .route(
            "/static/img/river-ground.jpg",
            get(public::river_ground_jpg),
        )
        .route(
            "/static/img/river-ground-maps.jpg",
            get(public::river_ground_maps_jpg),
        )
        .route(
            "/static/img/river-water-stream.jpg",
            get(public::river_water_stream_jpg),
        )
        .route(
            "/static/img/river-water-river.jpg",
            get(public::river_water_river_jpg),
        )
        .route(
            "/static/img/river-contours.svg",
            get(public::river_contours_svg),
        )
        .route(
            "/static/img/notes-canopee.jpg",
            get(public::notes_canopee_jpg),
        )
        .route(
            "/static/img/notes-fusion.jpg",
            get(public::notes_fusion_jpg),
        )
        .route(
            "/static/img/notes-galaxy.jpg",
            get(public::notes_galaxy_jpg),
        )
        .route(
            "/static/img/notes-summit.jpg",
            get(public::notes_summit_jpg),
        )
        .route(
            "/static/img/notes-clarity.jpg",
            get(public::notes_clarity_jpg),
        )
        .route(
            "/static/img/notes-genealogy.jpg",
            get(public::notes_genealogy_jpg),
        )
        .route("/static/img/logo-mark.svg", get(public::logo_svg))
        .route("/static/img/icon-192.png", get(public::icon_192_png))
        .route("/static/img/icon-512.png", get(public::icon_512_png))
        .route("/favicon.svg", get(public::favicon_svg))
        .route("/favicon.ico", get(public::favicon_ico))
        .route("/apple-touch-icon.png", get(public::apple_touch_icon_png))
        .route("/site.webmanifest", get(public::web_manifest))
        .route("/static/tree.js", get(public::tree_js))
        .route("/static/river.js", get(public::river_js))
        .route("/static/river-scene.js", get(public::river_scene_js))
        .route("/static/fusion.js", get(public::fusion_js))
        .route("/static/galaxy.js", get(public::galaxy_js))
        .route("/static/summit.js", get(public::summit_js))
        .route("/static/clarity.js", get(public::clarity_js))
        .route("/static/genealogy.js", get(public::genealogy_js))
        .route("/static/map.js", get(public::map_js))
        .route("/static/avatar.js", get(public::avatar_js))
        .route("/static/profile.js", get(public::profile_js))
        .route("/static/editor.js", get(public::editor_js))
        .route("/static/notes.js", get(public::notes_js))
        .route("/static/ui.js", get(public::ui_js))
        .route("/static/vendor/leaflet.js", get(public::leaflet_js))
        .route("/static/vendor/leaflet.css", get(public::leaflet_css))
        .fallback(public::not_found)
        .with_state(state)
}

#[cfg(test)]
mod origin_tests {
    use super::*;

    fn headers(pairs: &[(&'static str, &str)]) -> HeaderMap {
        let mut h = HeaderMap::new();
        for (k, v) in pairs {
            h.insert(*k, HeaderValue::from_str(v).unwrap());
        }
        h
    }

    #[test]
    fn a_post_from_the_page_itself_is_ours() {
        let h = headers(&[("host", "192.168.1.105:8088")]);
        assert!(is_own_origin("http://192.168.1.105:8088", &h, None));
        let h = headers(&[("host", "family.example")]);
        assert!(is_own_origin("https://family.example", &h, None));
        assert!(is_own_origin("https://FAMILY.example:443", &h, None));
    }

    #[test]
    fn another_port_on_the_same_host_is_another_origin() {
        let h = headers(&[("host", "192.168.1.105:8088")]);
        assert!(!is_own_origin("http://192.168.1.105", &h, None));
        assert!(!is_own_origin("http://192.168.1.105:8080", &h, None));
        assert!(!is_own_origin("null", &h, None));
        assert!(!is_own_origin("https://evil.example", &h, None));
    }

    #[test]
    fn behind_a_proxy_the_public_address_is_ours() {
        let h = headers(&[
            ("host", "127.0.0.1:8088"),
            ("x-forwarded-host", "family.example"),
        ]);
        assert!(is_own_origin("https://family.example", &h, None));
        let h = headers(&[("host", "127.0.0.1:8088")]);
        assert!(is_own_origin(
            "https://family.example",
            &h,
            Some("https://family.example")
        ));
        assert!(!is_own_origin(
            "https://other.example",
            &h,
            Some("https://family.example")
        ));
    }

    #[test]
    fn a_referer_gives_its_origin() {
        assert_eq!(
            origin_of_url("https://family.example:8443/admin/users?x=1").as_deref(),
            Some("https://family.example:8443")
        );
        assert_eq!(origin_of_url("not a url"), None);
    }
}
