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

/// The stylesheet, served at `/static/app.css`.
pub const APP_CSS: &str = include_str!("../static/app.css");

/// The tree page's client-side name filter, served at `/static/tree.js`.
pub const TREE_JS: &str = include_str!("../static/tree.js");

macro_rules! templates {
    ($($name:literal),* $(,)?) => {
        &[ $( ($name, include_str!(concat!("../templates/", $name))) ),* ]
    };
}

/// Every template, embedded at compile time.
const TEMPLATES: &[(&str, &str)] = templates![
    "base.html",
    "_prefs.html",
    "home.html",
    "error.html",
    "tree.html",
    "person.html",
    "_person_detail.html",
    "_panel.html",
    "convert.html",
    "convert_result.html",
    "admin_login.html",
    "admin_users.html",
    "admin_conflict.html",
    "admin_dashboard.html",
    "admin_list.html",
    "admin_form.html",
    "admin_result.html",
    "_macros.html",
    "_completeness.html"
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
    /// Where a preference form should return the reader to.
    pub back: String,
    pub locales: Vec<serde_json::Value>,
    pub themes: Vec<serde_json::Value>,
    pub current_locale: serde_json::Value,
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
        let wash_cookie = crate::session::named_cookie(headers, crate::theme::WASH_COOKIE_NAME);
        let wash_choice =
            crate::theme::reader_wants_wash(viewer.backgrounds(), wash_cookie.as_deref());
        Self {
            lang: locale.tag,
            dir: locale.dir.as_str(),
            theme: theme.attribute().unwrap_or(""),
            theme_choice: theme.id,
            wash: crate::theme::wash_enabled(theme, viewer.backgrounds(), wash_cookie.as_deref()),
            wash_choice,
            signed_in: viewer.signed_in(),
            may_write: viewer.may_write(),
            is_admin: viewer.is_admin(),
            back: safe_back(back),
            locales: crate::i18n::selector_entries(),
            themes: crate::theme::selector_entries(),
            current_locale: serde_json::json!({
                "tag": locale.tag,
                "native_name": locale.native_name,
                "english_name": locale.english_name,
                "reviewed": locale.reviewed,
                "coverage": locale.coverage_percent(),
            }),
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

/// Sanitise the `back` value a preference form will post.
///
/// It is reflected into a `Location` header after the form submits, so an
/// unchecked value is an open redirect: `?back=https://elsewhere.example` would
/// turn the language selector into a way of bouncing a reader off the site.
/// Only a same-site absolute path is accepted, and a protocol-relative `//host`
/// is rejected along with everything else.
fn safe_back(raw: &str) -> String {
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
        header::HeaderValue::from_static("application/vnd.axgf+zip"),
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
