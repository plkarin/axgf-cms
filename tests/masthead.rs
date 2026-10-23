//! The masthead, as Study 06 draws it (B5).
//!
//! 58px on the page's own ground with one hairline under it; the wordmark in
//! ink and a serif; the current section underlined in ink, not a pill and not
//! a colour; the language as a real select showing a two-letter code; signed
//! in, an initials chip and a name opening a menu, rather than a filled
//! sign-out button; signed out, a secondary "sign in".

mod common;

use axum::http::StatusCode;
use common::*;

fn css() -> &'static str {
    include_str!("../static/app.css")
}

/// The body of the first rule whose selector is exactly `selector`.
fn rule(selector: &str) -> &'static str {
    let css = css();
    let at = css
        .find(&format!("\n{selector} {{"))
        .unwrap_or_else(|| panic!("no rule for {selector}"));
    let open = at + css[at..].find('{').unwrap();
    &css[open..open + css[open..].find('}').unwrap()]
}

/// The masthead's markup: everything from `<header class="masthead">` to its
/// close.
fn masthead(html: &str) -> &str {
    let start = html
        .find(r#"<header class="masthead">"#)
        .expect("a masthead");
    &html[start..start + html[start..].find("</header>").expect("its end")]
}

#[test]
fn the_bar_is_58px_on_the_page_ground_with_one_hairline() {
    let bar = rule(".masthead");
    assert!(bar.contains("background: var(--surface-page)"), "{bar}");
    assert!(
        bar.contains("border-bottom: 1px solid var(--border-hairline)"),
        "{bar}"
    );
    assert!(rule(".masthead-inner").contains("min-block-size: 58px"));
}

/// Every rule as (selector, body), comments stripped and `@media` wrappers
/// looked through — the same reading `tests/controls.rs` makes.
fn rules() -> Vec<(String, String)> {
    let src = css();
    let mut css = String::new();
    let mut rest = src;
    while let Some(i) = rest.find("/*") {
        css.push_str(&rest[..i]);
        rest = &rest[i + rest[i..].find("*/").expect("closed comment") + 2..];
    }
    css.push_str(rest);
    let mut out = Vec::new();
    let mut start = 0;
    for (i, c) in css.char_indices() {
        match c {
            '}' => start = i + 1,
            '{' => {
                let end = i + css[i..].find('}').expect("a closing brace");
                let body = &css[i + 1..end];
                if !body.contains('{') {
                    out.push((css[start..i].trim().to_string(), body.trim().to_string()));
                }
                start = i + 1;
            }
            _ => {}
        }
    }
    out
}

#[test]
fn nothing_in_the_bar_is_coloured() {
    // The purple wordmark and its underline were in no palette. Amber is the
    // primary action's and the focus ring's; the masthead is ink.
    let brand = rule(".brand");
    assert!(brand.contains("color: var(--text-primary)"), "{brand}");
    assert!(brand.contains("font-family: var(--serif)"), "{brand}");
    let current = rule(".masthead nav a[aria-current=\"page\"]");
    assert!(
        current.contains("border-bottom-color: var(--text-primary)")
            && !current.contains("background"),
        "an underline in ink, not a pill: {current}"
    );
    // Read rule by rule. It used to be read in chunks split at "\n.", which
    // would have let any rule that does not begin with a class through
    // unread. Canopée, the reference theme, writes the section you are in
    // and the sign-in in its accent (docs/adr/0004 §2), and so do Fusion and
    // Galaxy, which carry its kit (docs/adr/0007): that is the one
    // exception, and it holds only under those themes' own selectors, so
    // every other theme keeps Study 06's ink masthead.
    let mut canopee = 0;
    for (sel, body) in rules() {
        let reaches = [
            ".masthead",
            ".brand",
            ".chrome-nav",
            ".lang-form",
            ".prefs-more",
        ]
        .iter()
        .any(|m| sel.contains(m));
        if !reaches || !body.contains("--accent") {
            continue;
        }
        const KIT: &str =
            ":is([data-theme=\"canopee\"], [data-theme=\"fusion\"], [data-theme=\"galaxy\"], [data-theme=\"summit\"], [data-theme=\"clarity\"], [data-theme=\"genealogy\"])";
        let own = |part: &str| {
            let part = part.trim();
            part.starts_with(KIT)
                || [
                    "canopee",
                    "fusion",
                    "galaxy",
                    "summit",
                    "clarity",
                    "genealogy",
                ]
                .iter()
                .any(|t| part.starts_with(&format!("[data-theme=\"{t}\"]")))
        };
        // A selector list, split at its own commas only: the kit's `:is()`
        // carries commas of its own.
        let mut parts = Vec::new();
        let (mut depth, mut start) = (0i32, 0usize);
        for (i, c) in sel.char_indices() {
            match c {
                '(' => depth += 1,
                ')' => depth -= 1,
                ',' if depth == 0 => {
                    parts.push(&sel[start..i]);
                    start = i + 1;
                }
                _ => {}
            }
        }
        parts.push(&sel[start..]);
        if parts.into_iter().all(own) {
            canopee += 1;
            continue;
        }
        panic!("{sel} puts the accent in the masthead outside the kit's themes: {body}");
    }
    assert!(
        canopee > 0,
        "the kit draws its own masthead, under its themes' selectors"
    );
}

#[tokio::test]
async fn signed_out_the_language_is_a_select_and_sign_in_is_secondary() {
    let (app, _s) = app_with_empty_bundle("masthead-out");
    let html = expect_status(get(&app, "/").await, StatusCode::OK, "GET /").await;
    let bar = masthead(&html);
    // The wordmark is text: "Rivière" beside it is the link to the same place.
    assert!(
        bar.contains(r#"<span class="brand">ax·genealogy</span>"#),
        "the wordmark: {bar}"
    );
    assert!(
        bar.contains(r#"<select name="lang""#),
        "a real select: {bar}"
    );
    for code in ["EN", "FR", "PL", "ZH", "AR", "JA", "RU"] {
        assert!(
            bar.contains(&format!(">{code}</option>")),
            "{code} as a code"
        );
    }
    assert!(
        bar.contains(r#"<a class="btn" href="/admin/login">"#),
        "sign in is a secondary button: {bar}"
    );
    assert!(!bar.contains("/admin/logout"));
    assert!(!bar.contains(r#"class="account""#));
}

#[tokio::test]
async fn signed_in_a_chip_and_a_name_open_the_menu() {
    let (app, _s) = app_with_empty_bundle("masthead-in");
    let cookie = admin_cookie(&app).await;
    let html = expect_status(
        get_with_cookie(&app, "/", &cookie).await,
        StatusCode::OK,
        "GET / signed in",
    )
    .await;
    let bar = masthead(&html);
    let account = &bar[bar
        .find(r#"<details class="account">"#)
        .expect("the account menu")..];
    assert!(account.contains(r#"<span class="initials" aria-hidden="true">"#));
    assert!(account.contains(r#"class="account-name""#));
    // Signing out lives in the menu, as a ghost, and nowhere else.
    assert_eq!(bar.matches("/admin/logout").count(), 1);
    assert!(account.contains(r#"action="/admin/logout""#));
    assert!(account.contains(r#"<button class="btn ghost""#));
    assert!(!bar.contains(r#"href="/admin/login""#));
}

#[tokio::test]
async fn the_language_select_shows_the_current_language() {
    let (app, _s) = app_with_empty_bundle("masthead-lang");
    let html = body_string(get_with_cookie(&app, "/about", "axgf_lang=pl").await).await;
    let bar = masthead(&html);
    assert!(
        bar.contains(r#"<option value="pl" lang="pl" selected>PL</option>"#),
        "{bar}"
    );
    // A select of its own (v0.4.3), not hidden in a menu with the theme.
    assert!(!bar.contains("display-menu"), "{bar}");
}

/// Pinned at a desktop width, where the bar is one 58px row. At phone width
/// it wraps to five rows, a third of the screen: pinned there, it hid a third
/// of every page. It scrolls away with the page instead.
#[test]
fn on_a_phone_the_bar_scrolls_away_with_the_page() {
    assert!(rule(".masthead").contains("position: sticky"));
    let css = css();
    let phone = css
        .match_indices("@media (max-width: 640px) {")
        .map(|(i, _)| &css[i..i + css[i..].find("\n}").expect("the block's end")])
        .find(|block| block.contains(".masthead { position: static; }"));
    assert!(phone.is_some(), "no phone-width rule unpins the masthead");
}
