//! What the site writes (docs/adr/0048): every message is one shape — a
//! title, a greeting, a few paragraphs, at most one button, sections for a
//! digest, small print, a footer — laid out by `templates/mail.html` for the
//! HTML part and written out in Rust for the text part, in the recipient's
//! language and the colours of their theme.
//!
//! Top level, not a submodule: the catalogue's reachability test reads the
//! top of `src/` only, and every key a message uses is named here.

use std::collections::BTreeMap;

use serde::Serialize;

use crate::i18n::{self, Locale};

/// The colours a message takes from a theme of `tokens.css`, the only file
/// that may name one.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Palette {
    pub page: String,
    pub raised: String,
    pub border: String,
    pub text: String,
    pub muted: String,
    pub accent: String,
    pub on_accent: String,
}

/// Top-level blocks of a stylesheet with their custom properties, comments
/// out, nested blocks (media queries) left aside.
fn blocks(css: &str) -> Vec<(String, BTreeMap<String, String>)> {
    let mut src = String::new();
    let mut rest = css;
    while let Some(i) = rest.find("/*") {
        src.push_str(&rest[..i]);
        match rest[i..].find("*/") {
            Some(j) => rest = &rest[i + j + 2..],
            None => {
                rest = "";
                break;
            }
        }
    }
    src.push_str(rest);
    let mut out = Vec::new();
    let mut depth = 0usize;
    let mut start = 0usize;
    let mut open = 0usize;
    for (i, c) in src.char_indices() {
        match c {
            '{' => {
                if depth == 0 {
                    open = i;
                }
                depth += 1;
            }
            '}' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    let selector = src[start..open]
                        .split_whitespace()
                        .collect::<Vec<_>>()
                        .join(" ");
                    let body = &src[open + 1..i];
                    if !body.contains('{') {
                        let vars = body
                            .split(';')
                            .filter_map(|d| d.trim().split_once(':'))
                            .filter(|(k, _)| k.trim().starts_with("--"))
                            .map(|(k, v)| (k.trim().to_string(), v.trim().to_string()))
                            .collect();
                        out.push((selector, vars));
                    }
                    start = i + 1;
                }
            }
            _ => {}
        }
    }
    out
}

/// Whether `v` is a colour a mail client can take as it is.
fn is_colour(v: &str) -> bool {
    let v = v.trim();
    (v.starts_with('#')
        && v[1..].chars().all(|c| c.is_ascii_hexdigit())
        && matches!(v.len(), 4 | 7 | 9))
        || v.starts_with("rgb(")
        || v.starts_with("rgba(")
}

impl Palette {
    /// The colours of `theme`, falling back on `:root`'s for anything the
    /// theme leaves out, `var()` followed.
    pub fn of(theme: &str) -> Palette {
        let all = blocks(crate::render::TOKENS_CSS);
        let wanted = format!("[data-theme=\"{theme}\"]");
        let empty = BTreeMap::new();
        let root = all
            .iter()
            .find(|(s, _)| s == ":root")
            .map(|(_, v)| v)
            .unwrap_or(&empty);
        let block = all
            .iter()
            .find(|(s, _)| *s == wanted)
            .map(|(_, v)| v)
            .unwrap_or(root);
        let get = |name: &str| -> String {
            let mut value = block.get(name).or_else(|| root.get(name)).cloned();
            for _ in 0..4 {
                match value.as_deref() {
                    Some(v) if v.starts_with("var(") => {
                        let inner = v.trim_start_matches("var(").trim_end_matches(')');
                        let inner = inner.split(',').next().unwrap_or("").trim();
                        value = block.get(inner).or_else(|| root.get(inner)).cloned();
                    }
                    _ => break,
                }
            }
            match value {
                Some(v) if is_colour(&v) => v,
                // `:root`'s own, which is a plain colour.
                _ => root
                    .get(name)
                    .cloned()
                    .filter(|v| is_colour(v))
                    .unwrap_or_default(),
            }
        };
        Palette {
            page: get("--surface-page"),
            raised: get("--surface-raised"),
            border: get("--border-hairline"),
            text: get("--text-primary"),
            muted: get("--text-secondary"),
            accent: get("--accent"),
            on_accent: get("--on-accent"),
        }
    }
}

/// A button: what it says and where it goes.
#[derive(Debug, Clone, Serialize)]
pub struct Action {
    pub label: String,
    pub url: String,
}

/// A link in the footer.
#[derive(Debug, Clone, Serialize)]
pub struct Link {
    pub label: String,
    pub url: String,
}

/// One line of a digest: what it is about, where it is, and what happened.
#[derive(Debug, Clone, Serialize)]
pub struct Item {
    pub title: String,
    pub url: String,
    pub meta: String,
    pub lines: Vec<String>,
}

/// A part of a digest.
#[derive(Debug, Clone, Serialize)]
pub struct Section {
    pub heading: String,
    pub items: Vec<Item>,
}

/// A message, before it is laid out.
#[derive(Debug, Clone, Default)]
pub struct Shape {
    pub subject: String,
    /// What a mail client shows beside the subject.
    pub preheader: String,
    pub title: String,
    pub greeting: Option<String>,
    pub paragraphs: Vec<String>,
    pub action: Option<Action>,
    pub sections: Vec<Section>,
    pub small: Vec<String>,
    pub links: Vec<Link>,
    /// The footer's key, when it is not `mail-footer` — which says an
    /// account there has this address, false for a test sent anywhere.
    pub footer: Option<&'static str>,
}

/// Where a message is written from: the recipient's language and theme, the
/// family, the site's public address.
#[derive(Debug, Clone)]
pub struct Letterhead {
    pub lang: &'static str,
    pub theme: String,
    pub family: String,
    pub site_url: Option<String>,
}

/// A system font stack: what every client has, quoted nowhere.
const FONT: &str = "system-ui, -apple-system, Segoe UI, Roboto, Helvetica, Arial, sans-serif";

impl Letterhead {
    /// `key` in the recipient's language.
    pub fn t(&self, key: &str) -> String {
        i18n::translate(self.lang, key, None)
    }

    /// `key` with its arguments, names isolated so that a Latin name keeps
    /// its place in Arabic or Hebrew text.
    pub fn t_args(&self, key: &str, pairs: &[(&str, &str)]) -> String {
        let rtl = Locale::get(self.lang).dir.as_str() == "rtl";
        let mut args = fluent::FluentArgs::new();
        for (k, v) in pairs {
            let v = if rtl {
                format!("\u{2068}{v}\u{2069}")
            } else {
                (*v).to_string()
            };
            args.set(*k, fluent::FluentValue::from(v));
        }
        i18n::translate(self.lang, key, Some(&args))
    }

    /// The family's name, or the product's when the archive has none yet.
    fn family_or_app(&self) -> String {
        if self.family.trim().is_empty() {
            self.t("app-name")
        } else {
            self.family.clone()
        }
    }
}

/// The two parts of a message: HTML and text.
pub fn lay_out(shape: &Shape, head: &Letterhead) -> (String, String) {
    let locale = Locale::get(head.lang);
    let rtl = locale.dir.as_str() == "rtl";
    let palette = Palette::of(&head.theme);
    let family = head.family_or_app();
    let footer = head.t_args(
        shape.footer.unwrap_or("mail-footer"),
        &[("family", &family)],
    );
    let fallback = head.t("mail-action-fallback");
    let site_host = head.site_url.as_deref().map(|u| {
        u.trim_start_matches("https://")
            .trim_start_matches("http://")
            .to_string()
    });
    let html = crate::render::env()
        .get_template("mail.html")
        .and_then(|t| {
            t.render(minijinja::context! {
                lang => head.lang,
                dir => if rtl { "rtl" } else { "ltr" },
                start => if rtl { "right" } else { "left" },
                c => palette,
                font => FONT,
                subject => shape.subject,
                preheader => shape.preheader,
                family => family,
                title => shape.title,
                greeting => shape.greeting,
                paragraphs => shape.paragraphs,
                action => shape.action,
                action_fallback => fallback,
                sections => shape.sections,
                small => shape.small,
                footer => footer,
                site_url => head.site_url,
                site_host => site_host,
                links => shape.links,
            })
        })
        .unwrap_or_else(|e| {
            tracing::error!(error = %e, "the mail layout did not render");
            String::new()
        });

    let mut text = String::new();
    text.push_str(&shape.title);
    text.push_str("\n\n");
    if let Some(g) = &shape.greeting {
        text.push_str(g);
        text.push_str("\n\n");
    }
    for p in &shape.paragraphs {
        text.push_str(p);
        text.push_str("\n\n");
    }
    if let Some(a) = &shape.action {
        text.push_str(&a.label);
        text.push_str(":\n");
        text.push_str(&a.url);
        text.push_str("\n\n");
    }
    for sec in &shape.sections {
        text.push_str(&sec.heading);
        text.push('\n');
        for it in &sec.items {
            text.push_str("- ");
            text.push_str(&it.title);
            if !it.meta.is_empty() {
                text.push_str(" (");
                text.push_str(&it.meta);
                text.push(')');
            }
            text.push('\n');
            for line in &it.lines {
                text.push_str("  ");
                text.push_str(line);
                text.push('\n');
            }
            text.push_str("  ");
            text.push_str(&it.url);
            text.push('\n');
        }
        text.push('\n');
    }
    for s in &shape.small {
        text.push_str(s);
        text.push_str("\n\n");
    }
    text.push_str("-- \n");
    text.push_str(&footer);
    text.push('\n');
    if let Some(u) = &head.site_url {
        text.push_str(u);
        text.push('\n');
    }
    for l in &shape.links {
        text.push_str(&l.label);
        text.push_str(": ");
        text.push_str(&l.url);
        text.push('\n');
    }
    (html, text)
}

/// The language a message to `user` is written in (docs/adr/0048 §3): the
/// interface language the member chose; the one the account was made in;
/// for a forgotten password, the request's; the last sign-in's; the
/// section's; English.
pub fn language_for(
    user: Option<&crate::acl::User>,
    request: Option<&str>,
    site: &crate::site::Site,
) -> &'static str {
    [
        user.and_then(|u| u.language.as_deref()),
        user.and_then(|u| u.mail_language.as_deref()),
        request,
        user.and_then(|u| u.seen_language.as_deref()),
        Some(site.mail.language.as_str()).filter(|l| !l.is_empty()),
    ]
    .into_iter()
    .flatten()
    .find_map(Locale::find)
    .map_or("en", |l| l.tag)
}

/// Where a message to `user` is written from.
pub fn letterhead(
    state: &crate::state::AppState,
    user: Option<&crate::acl::User>,
    request: Option<&str>,
) -> Letterhead {
    let site = state.site();
    let family = state.read(|flat| {
        flat.pointer("/manifest/family/name")
            .and_then(serde_json::Value::as_str)
            .map(str::trim)
            .unwrap_or_default()
            .to_string()
    });
    Letterhead {
        lang: language_for(user, request, &site),
        theme: user
            .and_then(|u| u.theme.clone())
            .unwrap_or_else(|| crate::theme::DEFAULT.to_string()),
        family,
        site_url: site.public_base(),
    }
}

/// Now, as a message says it: `2026-11-11 21:40 UTC`.
pub fn when_now() -> String {
    let now = time::OffsetDateTime::now_utc();
    format!(
        "{:04}-{:02}-{:02} {:02}:{:02} UTC",
        now.year(),
        u8::from(now.month()),
        now.day(),
        now.hour(),
        now.minute()
    )
}

fn greeting(head: &Letterhead, name: &str) -> Option<String> {
    Some(head.t_args("mail-greeting", &[("name", name)]))
}

/// A forgotten password's link.
pub fn reset(head: &Letterhead, username: &str, link: &str) -> Shape {
    let family = head.family_or_app();
    Shape {
        subject: head.t_args("mail-reset-subject", &[("family", &family)]),
        preheader: head.t("mail-reset-ignore"),
        title: head.t("mail-reset-title"),
        greeting: greeting(head, username),
        paragraphs: vec![head.t_args("mail-reset-body", &[("username", username)])],
        action: Some(Action {
            label: head.t("mail-reset-action"),
            url: link.to_string(),
        }),
        small: vec![head.t("mail-reset-ignore")],
        ..Shape::default()
    }
}

/// An account an administrator made, with the link to choose its password,
/// good for `days`.
pub fn welcome_with_link(head: &Letterhead, username: &str, link: &str, days: i64) -> Shape {
    let family = head.family_or_app();
    let mut args = fluent::FluentArgs::new();
    args.set("days", fluent::FluentValue::from(days));
    Shape {
        subject: head.t_args("mail-welcome-subject", &[("family", &family)]),
        preheader: head.t_args(
            "mail-welcome-body",
            &[("family", &family), ("username", username)],
        ),
        title: head.t("mail-welcome-title"),
        greeting: greeting(head, username),
        paragraphs: vec![
            head.t_args(
                "mail-welcome-body",
                &[("family", &family), ("username", username)],
            ),
            i18n::translate(head.lang, "mail-welcome-choose", Some(&args)),
        ],
        action: Some(Action {
            label: head.t("mail-welcome-action"),
            url: link.to_string(),
        }),
        ..Shape::default()
    }
}

/// An account that signs in through services — made through one, or of a
/// role that may not use a password: which, and where.
pub fn welcome_with_services(
    head: &Letterhead,
    username: &str,
    services: &str,
    sign_in: &str,
) -> Shape {
    let family = head.family_or_app();
    Shape {
        subject: head.t_args("mail-welcome-subject", &[("family", &family)]),
        preheader: head.t_args(
            "mail-welcome-body",
            &[("family", &family), ("username", username)],
        ),
        title: head.t("mail-welcome-title"),
        greeting: greeting(head, username),
        paragraphs: vec![
            head.t_args(
                "mail-welcome-body",
                &[("family", &family), ("username", username)],
            ),
            head.t_args("mail-welcome-services", &[("services", services)]),
        ],
        action: Some(Action {
            label: head.t("mail-sign-in"),
            url: sign_in.to_string(),
        }),
        ..Shape::default()
    }
}

/// The account's password changed, by its member or by an administrator.
pub fn password_changed(
    head: &Letterhead,
    username: &str,
    when: &str,
    by_admin: bool,
    sign_in: &str,
) -> Shape {
    let family = head.family_or_app();
    let mut paragraphs = vec![head.t_args(
        "mail-changed-body",
        &[("username", username), ("when", when)],
    )];
    if by_admin {
        paragraphs.push(head.t("mail-changed-by-admin"));
    }
    Shape {
        subject: head.t_args("mail-changed-subject", &[("family", &family)]),
        preheader: head.t("mail-changed-warning"),
        title: head.t("mail-changed-title"),
        greeting: greeting(head, username),
        paragraphs,
        action: Some(Action {
            label: head.t("mail-sign-in"),
            url: sign_in.to_string(),
        }),
        small: vec![head.t("mail-changed-warning")],
        ..Shape::default()
    }
}

/// A forgotten password asked for an account that has none: it signs in
/// through `services`.
pub fn service_only(head: &Letterhead, username: &str, services: &str, sign_in: &str) -> Shape {
    let family = head.family_or_app();
    Shape {
        subject: head.t_args("mail-service-subject", &[("family", &family)]),
        preheader: head.t_args(
            "mail-service-body",
            &[("username", username), ("services", services)],
        ),
        title: head.t("mail-service-title"),
        greeting: greeting(head, username),
        paragraphs: vec![head.t_args(
            "mail-service-body",
            &[("username", username), ("services", services)],
        )],
        action: Some(Action {
            label: head.t("mail-sign-in"),
            url: sign_in.to_string(),
        }),
        small: vec![head.t("mail-reset-ignore")],
        ..Shape::default()
    }
}

/// The administration's test.
pub fn test(head: &Letterhead) -> Shape {
    let family = head.family_or_app();
    Shape {
        subject: head.t_args("mail-test-subject", &[("family", &family)]),
        preheader: head.t_args("mail-test-body", &[("family", &family)]),
        title: head.t("mail-test-title"),
        paragraphs: vec![head.t_args("mail-test-body", &[("family", &family)])],
        footer: Some("mail-footer-test"),
        ..Shape::default()
    }
}

/// `shape`, laid out, as a message to `to`.
pub fn outgoing(
    shape: &Shape,
    head: &Letterhead,
    to: &str,
    kind: &'static str,
) -> crate::mail::Outgoing {
    let (html, text) = lay_out(shape, head);
    crate::mail::Outgoing {
        to: to.to_string(),
        subject: shape.subject.clone(),
        text,
        html,
        kind,
        unsubscribe: None,
        lang: head.lang.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_theme_gives_a_message_plain_colours() {
        for theme in [
            "canopee",
            "fusion",
            "galaxy",
            "summit",
            "clarity",
            "genealogy",
            "nope",
        ] {
            let p = Palette::of(theme);
            for (name, v) in [
                ("page", &p.page),
                ("raised", &p.raised),
                ("border", &p.border),
                ("text", &p.text),
                ("muted", &p.muted),
                ("accent", &p.accent),
                ("on_accent", &p.on_accent),
            ] {
                assert!(is_colour(v), "{theme} {name}: {v:?}");
            }
        }
        assert_ne!(
            Palette::of("canopee"),
            Palette::of("clarity"),
            "a theme's own"
        );
    }
}
