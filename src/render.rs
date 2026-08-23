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
        env
    })
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

/// Render the shared error page with a status code.
pub fn error_page(status: StatusCode, title: &str, detail: &str) -> Response {
    error_page_back(status, title, detail, None)
}

/// The error page with an extra link back to where the reader came from.
///
/// A refusal that makes the reader navigate back by hand is a worse refusal:
/// they were on a person's page, and that is where they want to be returned
/// to, with the reason in front of them.
pub fn error_page_back(
    status: StatusCode,
    title: &str,
    detail: &str,
    back: Option<(&str, &str)>,
) -> Response {
    let body = env()
        .get_template("error.html")
        .and_then(|t| {
            t.render(context! {
                title => title,
                detail => detail,
                status => status.as_u16(),
                back => back.map(|(href, _)| href),
                back_label => back.map(|(_, label)| label),
                nav => MjValue::from(""),
                is_admin => false,
            })
        })
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
pub async fn bundle_download_from(filename: &str, path: std::path::PathBuf) -> Response {
    let file = match tokio::fs::File::open(&path).await {
        Ok(f) => f,
        Err(e) => {
            let _ = tokio::fs::remove_file(&path).await;
            return error_page(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Could not read the exported bundle",
                &e.to_string(),
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
