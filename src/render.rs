//! Template environment and HTML response plumbing.
//!
//! Templates and the stylesheet live in the repository as ordinary files but
//! are embedded into the binary with `include_str!`. That keeps the deliverable
//! a single executable — `bootstrap.sh` copies one file and is done — while
//! still letting the templates be edited and diffed like normal source.

use std::sync::OnceLock;

use axum::http::{header, StatusCode};
use axum::response::{Html, IntoResponse, Response};
use minijinja::{context, Environment, Value as MjValue};
use serde::Serialize;

/// The design tokens (Study 06), served at `/static/css/tokens.css` and
/// loaded before the stylesheet: the one file that names colours by value.
pub const TOKENS_CSS: &str = include_str!("../static/css/tokens.css");

/// The stylesheet, served at `/static/app.css`.
pub const APP_CSS: &str = include_str!("../static/app.css");

/// The tree page's client-side name filter, served at `/static/tree.js`.
pub const TREE_JS: &str = include_str!("../static/tree.js");

/// The river's travel, hover and keyboard, served at `/static/river.js`. The
/// river is complete without it.
pub const RIVER_JS: &str = include_str!("../static/river.js");

/// The river's living ground, served at `/static/river-scene.js`: the WebGL
/// forest under the drawing and the water's light over it (docs/adr/0004
/// §11). Decoration only; without it, or without WebGL, the photograph is
/// the ground.
pub const RIVER_SCENE_JS: &str = include_str!("../static/river-scene.js");

/// Fusion's living tree, served at `/static/fusion.js`, Galaxy's solar
/// system, at `/static/galaxy.js` (docs/adr/0007), and Summit's ascent, at
/// `/static/summit.js` (docs/adr/0010): each a view of the river for river.js
/// to paint with, and the scene that keeps it alive. Loaded only under their
/// theme; the server's river is the page without them.
pub const FUSION_JS: &str = include_str!("../static/fusion.js");
pub const GALAXY_JS: &str = include_str!("../static/galaxy.js");
pub const SUMMIT_JS: &str = include_str!("../static/summit.js");
/// Clarity's funnel, at `/static/clarity.js` (docs/adr/0013).
pub const CLARITY_JS: &str = include_str!("../static/clarity.js");
/// Genealogy's scroll, at `/static/genealogy.js` (docs/adr/0016).
pub const GENEALOGY_JS: &str = include_str!("../static/genealogy.js");

/// The place-editor map. Enhancement only: the coordinate fields work without
/// it, and it does nothing at all unless the operator configured a tile URL.
pub const MAP_JS: &str = include_str!("../static/map.js");

/// The avatar picker's focal-point control. Enhancement only.
pub const AVATAR_JS: &str = include_str!("../static/avatar.js");

/// The profile editor's "add another entry", enhancement only.
pub const PROFILE_JS: &str = include_str!("../static/profile.js");

/// The record's editor: its tabs in place, the marks on a tab with unsaved
/// changes, the question before leaving them. Enhancement only.
pub const EDITOR_JS: &str = include_str!("../static/editor.js");
/// Notes and contributions (docs/adr/0014): writing one (the toolbar, pasted
/// pictures, the preview) and reading them (the filter, the letters, the
/// window). Enhancement only.
pub const NOTES_JS: &str = include_str!("../static/notes.js");

/// The six behaviours that were inline `onclick`/`oninput`/`onsubmit`
/// attributes until the Content-Security-Policy stopped allowing inline
/// script. Loaded on every page; enhancement only.
pub const UI_JS: &str = include_str!("../static/ui.js");

/// Leaflet, vendored. See `static/vendor/leaflet.css` for the one deliberate
/// modification and why it is there.
pub const LEAFLET_JS: &str = include_str!("../static/vendor/leaflet.js");
pub const LEAFLET_CSS: &str = include_str!("../static/vendor/leaflet.css");

/// The pictures of v0.5.0 (docs/adr/0004): Canopée's two photographs, made
/// for it in the operator's Canva, the river's ground (a forest from above,
/// also made there) and the river's contour lines (generated, a mask that
/// app.css colours). Embedded like every other asset, so a release is still
/// one file; 0.3 MB in all.
pub const CANOPEE_HERO_JPG: &[u8] = include_bytes!("../static/img/canopee-hero.jpg");
pub const CANOPEE_CTA_JPG: &[u8] = include_bytes!("../static/img/canopee-cta.jpg");
/// Fusion's and Galaxy's two photographs each, made for them in the
/// operator's Canva (docs/adr/0007): behind the overview's hero and the
/// sign-in, and beside the overview's last band.
pub const FUSION_HERO_JPG: &[u8] = include_bytes!("../static/img/fusion-hero.jpg");
pub const FUSION_CTA_JPG: &[u8] = include_bytes!("../static/img/fusion-cta.jpg");
pub const GALAXY_HERO_JPG: &[u8] = include_bytes!("../static/img/galaxy-hero.jpg");
pub const GALAXY_CTA_JPG: &[u8] = include_bytes!("../static/img/galaxy-cta.jpg");
/// Summit's two, made the same way (docs/adr/0010): a summit at sunrise above
/// a misty valley, and footprints climbing a snowy ridge above the clouds.
pub const SUMMIT_HERO_JPG: &[u8] = include_bytes!("../static/img/summit-hero.jpg");
pub const SUMMIT_CTA_JPG: &[u8] = include_bytes!("../static/img/summit-cta.jpg");
/// Clarity's two (docs/adr/0013), made in Canva from the operator's
/// references: a glass funnel of light on white, and a bright, airy band.
pub const CLARITY_HERO_JPG: &[u8] = include_bytes!("../static/img/clarity-hero.jpg");
pub const CLARITY_CTA_JPG: &[u8] = include_bytes!("../static/img/clarity-cta.jpg");
/// Genealogy's (docs/adr/0016), made in the operator's Canva after their
/// frame pack: a golden tree beside a scroll and empty oval frames, a desk
/// of old letters and photographs, and the vine along each edge of its pages (a tall strip
/// and its mirror, drawn at a third of their size so they stay sharp on a
/// dense screen; 2026-10-23).
pub const GENEALOGY_HERO_JPG: &[u8] = include_bytes!("../static/img/genealogy-hero.jpg");
pub const GENEALOGY_CTA_JPG: &[u8] = include_bytes!("../static/img/genealogy-cta.jpg");
pub const GENEALOGY_VINE_L_JPG: &[u8] = include_bytes!("../static/img/genealogy-vine-l.jpg");
pub const GENEALOGY_VINE_R_JPG: &[u8] = include_bytes!("../static/img/genealogy-vine-r.jpg");
/// Genealogy's tree (docs/adr/0016 §1, 2026-10-23): an old tree with a
/// twisted trunk and a crown of gold leaves growing out of a parchment scroll,
/// set on the stage's green; and its depth map, which `genealogy.js` moves it
/// in relief by.
pub const GENEALOGY_TREE_JPG: &[u8] = include_bytes!("../static/img/genealogy-tree.jpg");
pub const GENEALOGY_TREE_DEPTH_JPG: &[u8] =
    include_bytes!("../static/img/genealogy-tree-depth.jpg");
/// And the mountain its ascent climbs (docs/adr/0010 §1): a wall of peaks
/// above a forest and a misty valley at dawn, under the scene of `summit.js`.
pub const SUMMIT_PANORAMA_JPG: &[u8] = include_bytes!("../static/img/summit-panorama.jpg");
/// Fusion's laboratory (2026-10-23), made in the operator's Canva: a dark
/// room, blue panels at its sides and a projector glowing red on the floor,
/// under the tree's glass (fusion.js stands the glass on the projector).
pub const FUSION_LAB_JPG: &[u8] = include_bytes!("../static/img/fusion-lab.jpg");
pub const RIVER_GROUND_JPG: &[u8] = include_bytes!("../static/img/river-ground.jpg");
/// Its map: how near each pixel is (red), by which river-scene.js draws the
/// ground in relief.
pub const RIVER_GROUND_MAPS_JPG: &[u8] = include_bytes!("../static/img/river-ground-maps.jpg");
/// The water of the family's streams and rivers under Canopée (docs/adr/0004
/// §15): a mountain stream and a calm river seen from above, each repeating
/// down its length without a seam.
pub const RIVER_WATER_STREAM_JPG: &[u8] = include_bytes!("../static/img/river-water-stream.jpg");
pub const RIVER_WATER_RIVER_JPG: &[u8] = include_bytes!("../static/img/river-water-river.jpg");
/// Behind the family's notes and contributions (docs/adr/0014), one per
/// theme, made in the operator's Canva: soft and nearly plain, under a veil of
/// the theme's surface, so text over them reads as it does anywhere else.
pub const NOTES_CANOPEE_JPG: &[u8] = include_bytes!("../static/img/notes-canopee.jpg");
pub const NOTES_FUSION_JPG: &[u8] = include_bytes!("../static/img/notes-fusion.jpg");
pub const NOTES_GALAXY_JPG: &[u8] = include_bytes!("../static/img/notes-galaxy.jpg");
pub const NOTES_SUMMIT_JPG: &[u8] = include_bytes!("../static/img/notes-summit.jpg");
pub const NOTES_CLARITY_JPG: &[u8] = include_bytes!("../static/img/notes-clarity.jpg");
pub const NOTES_GENEALOGY_JPG: &[u8] = include_bytes!("../static/img/notes-genealogy.jpg");
pub const RIVER_CONTOURS_SVG: &str = include_str!("../static/img/river-contours.svg");

/// ax·genealogy's logo (v0.5.0, docs/adr/0006), and what a browser or a phone
/// asks for to show it: the favicon (the tile and the X, without the wreath,
/// which is noise under 48 pixels), the home-screen icons (the whole logo,
/// full bleed, for the platform to round) and the manifest that names them.
/// The masthead and the sign-in page draw the logo inline (`_logo.html`), so
/// a theme can colour it.
/// Who made ax·genealogy: credited in every page's footer (v0.5.0).
pub const AUTHOR: &str = "Pierre-Léonard Karin";

pub const LOGO_SVG: &str = include_str!("../static/img/logo-mark.svg");
pub const FAVICON_SVG: &str = include_str!("../static/img/favicon.svg");
pub const FAVICON_ICO: &[u8] = include_bytes!("../static/img/favicon.ico");
pub const APPLE_TOUCH_ICON_PNG: &[u8] = include_bytes!("../static/img/apple-touch-icon.png");
pub const ICON_192_PNG: &[u8] = include_bytes!("../static/img/icon-192.png");
pub const ICON_512_PNG: &[u8] = include_bytes!("../static/img/icon-512.png");
pub const WEB_MANIFEST: &str = include_str!("../static/site.webmanifest");

macro_rules! templates {
    ($($name:literal),* $(,)?) => {
        &[ $( ($name, include_str!(concat!("../templates/", $name))) ),* ]
    };
}

/// Every template, embedded at compile time.
const TEMPLATES: &[(&str, &str)] = templates![
    "base.html",
    "_prefs.html",
    "settings.html",
    "home.html",
    "error.html",
    "tree.html",
    "river.html",
    "search.html",
    "quality.html",
    "person.html",
    "_person_detail.html",
    "_profile.html",
    "_profile_read.html",
    "_radar.html",
    "_tabs.html",
    "_tree_canvas.html",
    "_panel.html",
    "convert.html",
    "convert_result.html",
    "_gedcom_reading.html",
    "admin_setup.html",
    "admin_setup_result.html",
    "admin_login.html",
    "admin_users.html",
    "admin_bulk.html",
    "merged.html",
    "admin_conflict.html",
    "admin_dashboard.html",
    "admin_bundle.html",
    "admin_infrastructure.html",
    "admin_connectivity.html",
    "register.html",
    "register_finish.html",
    "admin_list.html",
    "admin_form.html",
    "admin_place.html",
    "admin_profile.html",
    "admin_person.html",
    "_profile_edit.html",
    "admin_identity.html",
    "admin_family.html",
    "admin_links.html",
    "admin_occupations.html",
    "admin_events.html",
    "admin_documents.html",
    "admin_avatar.html",
    "admin_ai_portraits.html",
    "admin_result.html",
    "_macros.html",
    "_icons.html",
    "_admin_nav.html",
    "_checks.html",
    "_logo.html",
    "_completeness.html",
    "_notes.html",
    "_note_frame.html",
    "note.html",
    "admin_notes.html",
    "admin_note.html",
    "account.html"
];

static ENV: OnceLock<Environment<'static>> = OnceLock::new();

/// The shared template environment.
pub fn env() -> &'static Environment<'static> {
    ENV.get_or_init(|| {
        let mut env = Environment::new();
        // Autoescaping is on for .html, which is every template here. Any
        // value rendered into a page is escaped unless explicitly marked safe.
        env.set_auto_escape_callback(|_| minijinja::AutoEscape::Html);
        for (name, body) in TEMPLATES {
            env.add_template(name, body)
                .unwrap_or_else(|e| panic!("template {name} failed to compile: {e}"));
        }
        crate::view::register_filters(&mut env);
        register_translate(&mut env);
        env
    })
}

/// Compile every template now rather than on the first page.
///
/// The environment compiles all of them on first use, and that first use was
/// a visitor: after the catalogue moved to boot, the first river request in a
/// process still cost ~4 ms more than the ones after it. `main` calls this
/// next to [`crate::i18n::warm`], before the listener opens.
pub fn warm() {
    let _ = env();
}

/// Add `t(key, ...)` to the environment.
///
/// The locale is read from the render context rather than being bound into the
/// function, because the environment is built once and shared by every request
/// while the language changes per request. `t` therefore looks up `lang` in
/// the template state, which every page carries from [`Chrome`].
///
/// Keyword arguments become Fluent variables, so `t("tree-hidden", n=17)`
/// selects the right plural form for Polish or Arabic without this crate
/// knowing anything about either.
fn register_translate(env: &mut Environment<'static>) {
    use minijinja::value::{Kwargs, Value};
    use minijinja::{Error, ErrorKind, State};

    env.add_function(
        "t",
        |state: &State, key: &str, kwargs: Kwargs| -> Result<Value, Error> {
            let lang = state
                .lookup("lang")
                .as_ref()
                .and_then(|v| v.as_str().map(str::to_string))
                .unwrap_or_else(|| crate::i18n::DEFAULT.to_string());

            let mut args = fluent::FluentArgs::new();
            for name in kwargs.args() {
                let v: Value = kwargs.get(name)?;
                args.set(name.to_string(), to_fluent(&v));
            }
            kwargs.assert_all_used()?;

            let has_args = args.iter().next().is_some();
            let out = crate::i18n::translate(&lang, key, has_args.then_some(&args));
            if out == key {
                // Not fatal — the key renders as itself, which is visible on
                // the page — but it is always a bug, so it should be findable
                // in the log rather than only by looking at the page.
                tracing::warn!(lang = %lang, key, "no message for this key in any locale");
            }
            let _ = ErrorKind::InvalidOperation;
            Ok(Value::from(out))
        },
    );
}

/// Convert a template value to a Fluent argument.
///
/// Numbers stay numbers, because that is what the CLDR plural rules select on;
/// passing "3" as a string would make every language take the `other` branch.
fn to_fluent(v: &minijinja::value::Value) -> fluent::FluentValue<'static> {
    if let Ok(n) = i64::try_from(v.clone()) {
        return fluent::FluentValue::from(n);
    }
    if let Ok(n) = f64::try_from(v.clone()) {
        return fluent::FluentValue::from(n);
    }
    fluent::FluentValue::from(v.to_string())
}

/// The per-request context every page needs: who is reading, in what language,
/// in what direction, under what theme.
///
/// Assembled once per request and merged into each page's own context, so no
/// handler has to remember to pass `dir` and none can forget to. That is the
/// point of it existing rather than each handler passing its own: the day one
/// page forgets `dir`, Arabic breaks on that page only, and nobody notices
/// until a reader of Arabic does.
#[derive(Debug, Clone, Serialize)]
pub struct Chrome {
    pub lang: &'static str,
    pub dir: &'static str,
    /// The `data-theme` attribute value, empty for `system` so the
    /// stylesheet's `prefers-color-scheme` queries decide.
    pub theme: &'static str,
    /// What the reader chose, which may be `system`. Distinct from `theme`
    /// because the selector has to show `system` as ticked.
    pub theme_choice: &'static str,
    /// The landing page's drawing under this theme: `river`, or a theme's own
    /// view, `tree` (Fusion) or `galaxy` (Galaxy). See
    /// [`crate::theme::view_of`].
    pub view: &'static str,
    /// The presentation style: how dense the page is, never what colour. A
    /// second axis from the theme, and orthogonal to it — see [`crate::style`].
    pub style: &'static str,
    pub styles: Vec<serde_json::Value>,
    /// Whether to draw the soft page background: the theme's own flag and the
    /// reader's preference, already resolved. The template writes it as
    /// `data-wash`, so the stylesheet needs no exception of its own.
    pub wash: bool,
    /// Whether *this reader* asked for it, ignoring the theme's veto — the
    /// selector has to show the switch as they left it even under a theme
    /// that will not honour it.
    pub wash_choice: bool,
    pub signed_in: bool,
    pub may_write: bool,
    pub is_admin: bool,
    /// Who is signed in, for the masthead's account menu: the username, or
    /// the emergency session's name in the interface language. Empty when
    /// nobody is.
    pub viewer_name: String,
    /// One or two letters for the masthead's chip, from `viewer_name`.
    pub viewer_initials: String,
    /// Where the format is published, for the footer's acknowledgement.
    pub spec_url: &'static str,
    /// This build's version, shown in the footer on every page.
    pub version: &'static str,
    /// Who made ax·genealogy, credited in the footer.
    pub author: &'static str,
    /// Where a preference form should return the reader to.
    pub back: String,
    /// The masthead's link to the settings, carrying `back` as the way out.
    /// Built here rather than in the template because percent-encoding a path
    /// is not a thing to do in Jinja.
    pub settings_url: String,
    pub locales: Vec<serde_json::Value>,
    pub themes: Vec<serde_json::Value>,
    pub current_locale: serde_json::Value,
    /// The operator's plausibility limit, for the sentence that explains a
    /// presumed death. On the chrome rather than passed per page because four
    /// templates need it and one of them is a tree card drawn a thousand times
    /// a page. See [`crate::living`].
    pub presume_after: i64,
}

impl Chrome {
    /// Resolve the chrome for one request.
    pub fn resolve(
        viewer: &crate::access::Viewer,
        headers: &axum::http::HeaderMap,
        back: &str,
    ) -> Self {
        let locale = crate::i18n::negotiate(
            viewer.language(),
            crate::session::named_cookie(headers, crate::i18n::COOKIE_NAME).as_deref(),
            headers
                .get(axum::http::header::ACCEPT_LANGUAGE)
                .and_then(|v| v.to_str().ok()),
        );
        let theme = crate::theme::negotiate(
            viewer.theme(),
            crate::session::named_cookie(headers, crate::theme::COOKIE_NAME).as_deref(),
        );
        let style = crate::style::negotiate(
            crate::session::named_cookie(headers, crate::style::COOKIE_NAME).as_deref(),
        );
        let wash_cookie = crate::session::named_cookie(headers, crate::theme::WASH_COOKIE_NAME);
        let wash_choice =
            crate::theme::reader_wants_wash(viewer.backgrounds(), wash_cookie.as_deref());
        let viewer_name = if viewer.emergency {
            crate::i18n::translate(locale.tag, "account-emergency", None)
        } else {
            viewer.name().to_string()
        };
        Self {
            spec_url: "github.com/plkarin/axgf-spec",
            version: env!("CARGO_PKG_VERSION"),
            author: AUTHOR,
            viewer_initials: initials(&viewer_name),
            viewer_name,
            lang: locale.tag,
            dir: locale.dir.as_str(),
            theme: theme.attribute(),
            theme_choice: theme.id,
            view: crate::theme::view_of(theme.id),
            wash: crate::theme::wash_enabled(theme, viewer.backgrounds(), wash_cookie.as_deref()),
            wash_choice,
            signed_in: viewer.signed_in(),
            may_write: viewer.may_write(),
            is_admin: viewer.is_admin(),
            settings_url: crate::settings::entry_url(&safe_back(back)),
            back: safe_back(back),
            locales: crate::i18n::selector_entries(),
            themes: crate::theme::selector_entries(),
            current_locale: serde_json::json!({
                "tag": locale.tag,
                // As the language select prints it: "ZH", not "ZH-HANS".
                "code": locale.tag.split('-').next().unwrap_or(locale.tag).to_uppercase(),
                "native_name": locale.native_name,
                "english_name": locale.english_name,
                "reviewed": locale.reviewed,
                "coverage": locale.coverage_percent(),
            }),
            style: style.id,
            styles: crate::style::selector_entries(),
            presume_after: crate::living::max_age_years(),
        }
    }

    /// The interface language, for code that needs to translate outside a
    /// template — an error page's title, a flash message.
    pub fn t(&self, key: &str) -> String {
        crate::i18n::translate(self.lang, key, None)
    }

    /// Translate with arguments.
    pub fn t_args(&self, key: &str, pairs: &[(&str, fluent::FluentValue<'_>)]) -> String {
        let mut args = fluent::FluentArgs::new();
        for (k, v) in pairs {
            args.set(*k, v.clone());
        }
        crate::i18n::translate(self.lang, key, Some(&args))
    }
}

/// The masthead chip's letters: the first letter of each of the first two
/// words of a name (`marta.w` → `MW`), or the first two of a single word
/// (`verification` → `VE`). Letters, not bytes: a name may be in any script.
///
/// ```
/// use axgf_cms::render::initials;
/// assert_eq!(initials("marta.w"), "MW");
/// assert_eq!(initials("verification"), "VE");
/// assert_eq!(initials("Jeton d’urgence"), "JU");
/// assert_eq!(initials("Łucja Żak"), "ŁŻ");
/// assert_eq!(initials("紧急令牌"), "紧急");
/// assert_eq!(initials(""), "");
/// ```
pub fn initials(name: &str) -> String {
    let mut words: Vec<&str> = name
        .split(|c: char| !c.is_alphanumeric())
        .filter(|w| !w.is_empty())
        .collect();
    // An elision is not a word: `Jeton d’urgence` is JU, not JD.
    if words.iter().filter(|w| w.chars().count() > 1).count() >= 2 {
        words.retain(|w| w.chars().count() > 1);
    }
    let letters: String = match words.as_slice() {
        [] => String::new(),
        [one] => one.chars().take(2).collect(),
        [a, b, ..] => a.chars().take(1).chain(b.chars().take(1)).collect(),
    };
    letters.to_uppercase()
}

/// Sanitise the `back` value a preference form will post.
///
/// It is reflected into a `Location` header after the form submits, so an
/// unchecked value is an open redirect: `?back=https://elsewhere.example` would
/// turn the language selector into a way of bouncing a reader off the site.
/// Only a same-site absolute path is accepted, and a protocol-relative `//host`
/// is rejected along with everything else.
pub fn safe_back(raw: &str) -> String {
    let candidate = raw.trim();
    if candidate.starts_with('/') && !candidate.starts_with("//") && !candidate.contains('\\') {
        candidate.to_string()
    } else {
        "/".to_string()
    }
}

/// Render a page with the shared chrome merged into its own context.
pub fn page_with(chrome: &Chrome, name: &str, ctx: impl Serialize) -> Response {
    let merged = context! { ..MjValue::from_serialize(chrome), ..MjValue::from_serialize(&ctx) };
    page(name, merged)
}

/// Render a template to an HTML response.
///
/// A template failure is a programming error, not a user error, so it renders
/// a 500 with the message rather than panicking a worker thread.
pub fn page(name: &str, ctx: impl Serialize) -> Response {
    match env().get_template(name).and_then(|t| t.render(ctx)) {
        Ok(body) => Html(body).into_response(),
        Err(e) => {
            tracing::error!(template = name, error = %e, "template render failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Html(format!(
                    "<h1>Template error</h1><pre>{}</pre>",
                    html_escape(&e.to_string())
                )),
            )
                .into_response()
        }
    }
}

/// Render the shared error page, in the reader's language.
///
/// `title_key` and `detail_key` are locale message ids, not sentences. An
/// error page is the one screen a reader is *most* likely to need in a
/// language they read fluently, so it is the last place to leave English
/// hardcoded.
pub fn error_page_in(
    chrome: &Chrome,
    status: StatusCode,
    title_key: &str,
    detail_key: &str,
) -> Response {
    error_page_full(chrome, status, title_key, detail_key, &[], None)
}

/// The error page with arguments for the detail message.
pub fn error_page_args(
    chrome: &Chrome,
    status: StatusCode,
    title_key: &str,
    detail_key: &str,
    args: &[(&str, fluent::FluentValue<'_>)],
) -> Response {
    error_page_full(chrome, status, title_key, detail_key, args, None)
}

/// The error page with an extra link back to where the reader came from.
///
/// A refusal that makes the reader navigate back by hand is a worse refusal:
/// they were on a person's page, and that is where they want to be returned
/// to, with the reason in front of them.
pub fn error_page_back_in(
    chrome: &Chrome,
    status: StatusCode,
    title_key: &str,
    detail_key: &str,
    args: &[(&str, fluent::FluentValue<'_>)],
    back: Option<(&str, &str)>,
) -> Response {
    error_page_full(chrome, status, title_key, detail_key, args, back)
}

fn error_page_full(
    chrome: &Chrome,
    status: StatusCode,
    title_key: &str,
    detail_key: &str,
    args: &[(&str, fluent::FluentValue<'_>)],
    back: Option<(&str, &str)>,
) -> Response {
    let title = chrome.t(title_key);
    let detail = chrome.t_args(detail_key, args);
    let ctx = context! {
        title => title.clone(),
        detail => detail.clone(),
        status => status.as_u16(),
        // Deliberately not `back`: `Chrome` already carries a field of that
        // name — where a preference form should return the reader to, which is
        // the page they are on. Merging the two contexts let the chrome's
        // value win, so every error page grew a second button pointing at the
        // page that had just refused them, labelled with a rendered `None`.
        error_back => back.map(|(href, _)| href),
        error_back_label => back.map(|(_, label)| chrome.t(label)),
        nav => MjValue::from(""),
    };
    let merged = context! { ..MjValue::from_serialize(chrome), ..MjValue::from_serialize(&ctx) };
    let body = env()
        .get_template("error.html")
        .and_then(|t| t.render(merged))
        // The fallback is deliberately plain: if the template itself is
        // broken, the reader still gets the sentence explaining what went
        // wrong rather than a second error about the first one.
        .unwrap_or_else(|_| format!("<h1>{title}</h1><p>{detail}</p>"));
    (status, Html(body)).into_response()
}

/// A `.axgf` download response.
pub fn bundle_download(filename: &str, bytes: Vec<u8>) -> Response {
    (
        [
            (header::CONTENT_TYPE, "application/vnd.axgf+zip".to_string()),
            (
                header::CONTENT_DISPOSITION,
                format!("attachment; filename=\"{}\"", sanitize_filename(filename)),
            ),
        ],
        bytes,
    )
        .into_response()
}

/// A `.axgf` download streamed from a file the response takes ownership of.
///
/// The file is unlinked as soon as it is open: on a POSIX filesystem the bytes
/// survive until the handle closes, so the temp file cannot outlive the
/// response even if the client disconnects halfway through or the process is
/// killed.
pub async fn bundle_download_from(
    chrome: &Chrome,
    filename: &str,
    path: std::path::PathBuf,
) -> Response {
    download_from(chrome, filename, path, "application/vnd.axgf+zip").await
}

/// The cookie that tells the page a download it asked for has started
/// (`data-download` forms, static/ui.js): a file arriving does not unload the
/// page, so without it the button would say "Working…" for ever. It names a
/// token the page made up, readable by the page's script, and dies in two
/// minutes.
pub const DOWNLOAD_COOKIE: &str = "axgf_download";

/// Mark a download response with the page's token, when it sent one that is
/// plainly a token.
pub fn mark_download(resp: &mut Response, headers: &axum::http::HeaderMap, token: Option<&str>) {
    let Some(token) = token
        .filter(|t| (8..=64).contains(&t.len()) && t.bytes().all(|b| b.is_ascii_alphanumeric()))
    else {
        return;
    };
    if !resp.status().is_success() {
        return;
    }
    let mut cookie =
        format!("{DOWNLOAD_COOKIE}={token}; Path=/admin; SameSite=Strict; Max-Age=120");
    if crate::session::is_tls(headers) {
        cookie.push_str("; Secure");
    }
    if let Ok(v) = cookie.parse() {
        resp.headers_mut().append(header::SET_COOKIE, v);
    }
}

/// Stream a temp file as a download of `content_type`, removing it from the
/// disk as soon as it is open — the bundle's own download, and the encrypted
/// one (docs/adr/0011 §4).
pub async fn download_from(
    chrome: &Chrome,
    filename: &str,
    path: std::path::PathBuf,
    content_type: &'static str,
) -> Response {
    let file = match tokio::fs::File::open(&path).await {
        Ok(f) => f,
        Err(e) => {
            let _ = tokio::fs::remove_file(&path).await;
            return error_page_args(
                chrome,
                StatusCode::INTERNAL_SERVER_ERROR,
                "error-export-unreadable-title",
                "error-export-unreadable-detail",
                &[("error", e.to_string().into())],
            );
        }
    };
    let _ = tokio::fs::remove_file(&path).await;
    let len = file.metadata().await.map(|m| m.len()).ok();

    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static(content_type),
    );
    if let Ok(v) = header::HeaderValue::from_str(&format!(
        "attachment; filename=\"{}\"",
        sanitize_filename(filename)
    )) {
        headers.insert(header::CONTENT_DISPOSITION, v);
    }
    if let Some(len) = len {
        if let Ok(v) = header::HeaderValue::from_str(&len.to_string()) {
            headers.insert(header::CONTENT_LENGTH, v);
        }
    }
    let body = axum::body::Body::from_stream(tokio_util::io::ReaderStream::new(file));
    (headers, body).into_response()
}

/// Strip characters that would let a filename break out of the
/// `Content-Disposition` quoting.
fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            '"' | '\\' | '\r' | '\n' | '/' => '_',
            c => c,
        })
        .collect()
}

/// Minimal HTML escaping for the few places that build markup by hand.
pub fn html_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            c => out.push(c),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_template_compiles() {
        // Building the environment panics on a malformed template, so simply
        // touching it proves all of them parse.
        let e = env();
        for (name, _) in TEMPLATES {
            assert!(e.get_template(name).is_ok(), "{name} did not load");
        }
    }

    #[test]
    fn filename_cannot_break_content_disposition() {
        assert_eq!(sanitize_filename("a\"b.axgf"), "a_b.axgf");
        // Path separators are neutralised so a crafted name cannot suggest a
        // directory to the browser.
        assert_eq!(sanitize_filename("../../etc/passwd"), ".._.._etc_passwd");
        assert_eq!(sanitize_filename("a\r\nX: y"), "a__X: y");
    }

    #[test]
    fn html_escape_covers_the_dangerous_five() {
        assert_eq!(
            html_escape(r#"<a href="x">&'"#),
            "&lt;a href=&quot;x&quot;&gt;&amp;&#39;"
        );
    }
}
