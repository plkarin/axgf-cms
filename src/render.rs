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

macro_rules! templates {
    ($($name:literal),* $(,)?) => {
        &[ $( ($name, include_str!(concat!("../templates/", $name))) ),* ]
    };
}

/// Every template, embedded at compile time.
const TEMPLATES: &[(&str, &str)] = templates!["base.html", "home.html", "error.html"];

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
    let body = env()
        .get_template("error.html")
        .and_then(|t| {
            t.render(context! {
                title => title,
                detail => detail,
                status => status.as_u16(),
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
