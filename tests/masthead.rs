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
    let all: String = css()
        .split("\n.")
        .filter(|r| r.starts_with("masthead") || r.starts_with("brand"))
        .collect();
    assert!(
        !all.contains("--accent"),
        "no accent anywhere in the masthead's rules"
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
}
