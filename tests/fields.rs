//! Fields, as Study 06 draws them (B3).
//!
//! A field sits on the sunken surface; its label is mono, uppercase and
//! always visible (the walk in `tests/interactive_walk.rs` holds every field
//! on every page to that, and to no placeholder); a date carries its
//! qualifier inside its own box, `vers ▾ 1890`, rather than a checkbox beside
//! it; an approximate date reads "vers 1890" with a dotted underline.

mod common;

use common::*;
use serde_json::json;

fn css() -> &'static str {
    include_str!("../static/app.css")
}

/// The body of the first rule whose selector list is exactly `selector`.
fn rule(selector: &str) -> &'static str {
    let css = css();
    let at = css
        .find(&format!("\n{selector} {{"))
        .unwrap_or_else(|| panic!("no rule for {selector}"));
    let open = at + css[at..].find('{').unwrap();
    &css[open..open + css[open..].find('}').unwrap()]
}

#[test]
fn a_field_sits_below_the_page_and_its_label_is_mono_and_uppercase() {
    let field = rule(
        "input[type=\"text\"], input[type=\"password\"], input[type=\"email\"],\ninput[type=\"search\"], input[type=\"number\"], input[type=\"url\"],\ninput[type=\"file\"], select, textarea",
    );
    assert!(
        field.contains("background: var(--surface-sunken)"),
        "{field}"
    );
    // Study 06's 2px, through the token every theme but Canopée leaves alone
    // (docs/adr/0004 §2): the reference theme softens its fields to 12px.
    assert!(
        field.contains("block-size: 36px") && field.contains("border-radius: var(--field-radius)")
    );
    assert!(
        css().contains("--field-radius: 2px;"),
        "Study 06's field radius on :root"
    );
    // A field inside a label does not take the label's type.
    assert!(field.contains("font-family: var(--face)") && field.contains("text-transform: none"));
    let label = rule("label, .field-label");
    assert!(label.contains("font-family: var(--mono)"), "{label}");
    assert!(label.contains("text-transform: uppercase"), "{label}");
    let inline = rule("label.inline");
    assert!(
        inline.contains("text-transform: none"),
        "a checkbox's sentence stays one"
    );
    let approx = rule(".date[data-kind=\"approximate\"]");
    assert!(approx.contains("underline dotted"), "{approx}");
    assert!(!approx.contains("italic"), "{approx}");
}

const P: &str = "p01";

fn app() -> (axum::Router, Scratch) {
    let flat = json!({
        "manifest": {"axgf": "1.1", "created_at": "2026-10-11T00:00:00Z"},
        "persons": {P: {"id": P, "type": "person", "axgf_version": "1.1", "version_num": 1,
            "identity": {"name": {"display": "Józef Wierzbięta",
                "components": [{"type": "given_name", "value": "Józef"},
                               {"type": "family_name", "value": "Wierzbięta"}]},
                "gender": {"value": "M"}, "is_living": false, "visibility": "public"},
            "birth": {"date": {"value": "1888", "precision": "year", "circa": true}},
            "death": {"date": {"value": "1951", "precision": "year"}}}},
        "families": {}, "events": {}, "places": {}, "occupations": {}, "links": {},
        "sources": {}, "documents": {}
    });
    let src = scratch("fields-src");
    let path = src.join("fields.axgf");
    std::fs::write(
        &path,
        axgf_cms::state::export_to_bytes(&flat.to_string()).expect("export"),
    )
    .expect("write");
    let out = app_with_bundle("fields", &path);
    drop(src);
    out
}

/// The `<select>` named `name`, as rendered.
fn select<'a>(html: &'a str, name: &str) -> &'a str {
    let at = html
        .find(&format!(r#"<select name="{name}""#))
        .unwrap_or_else(|| panic!("no select {name}"));
    &html[at..at + html[at..].find("</select>").unwrap()]
}

#[tokio::test]
async fn a_date_carries_its_qualifier_inside_its_own_box() {
    let (app, _s) = app();
    let cookie = format!("{}; axgf_lang=fr", admin_cookie(&app).await);
    let html =
        body_string(get_with_cookie(&app, &format!("/admin/person/{P}/edit"), &cookie).await).await;

    // Birth is circa: the qualifier says so. Death is not.
    let birth = select(&html, "birth.date.circa");
    assert!(
        birth.contains(r#"<option value="on" selected>vers</option>"#),
        "{birth}"
    );
    let death = select(&html, "death.date.circa");
    assert!(!death.contains("selected"), "{death}");
    // And it sits in the date's box, before the year.
    let boxed = &html[html
        .find(r#"<span class="date-input">"#)
        .expect("the date's box")..];
    let boxed = &boxed[..boxed.find("</span>").unwrap()];
    assert!(
        boxed.find("birth.date.circa").unwrap() < boxed.find("birth.date.value").unwrap(),
        "{boxed}"
    );
    // No checkbox for it any more.
    assert!(!html.contains(r#"type="checkbox" name="birth.date.circa""#));
}

#[tokio::test]
async fn an_approximate_date_reads_as_one() {
    let (app, _s) = app();
    let html =
        body_string(get_with_cookie(&app, &format!("/person/{P}"), "axgf_lang=fr").await).await;
    assert!(
        html.contains(r#"<bdi class="date" data-kind="approximate">vers 1888</bdi>"#),
        "{}",
        &html[html.find("1888").map_or(0, |i| i.saturating_sub(200))..][..400.min(html.len())]
    );
}
