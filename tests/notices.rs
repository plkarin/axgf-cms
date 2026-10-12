//! Notices, errors and page states, as Study 06 draws them (B7).
//!
//! Each kind has its own shape — a ring for information, a disc for done, a
//! diamond for caution, a square for an error — so none of it needs colour;
//! no tinted ground except a faint one under an error; what happened, then
//! what to do, with the actions as buttons underneath. And the study's page
//! state: a person's old address, after they were merged into another
//! record, says so and offers the way on.

mod common;

use axum::http::StatusCode;
use common::*;
use serde_json::{json, Value};

fn css() -> &'static str {
    include_str!("../static/app.css")
}

/// The body of the LAST rule for `selector`: `.notice` also closes the
/// shared surface list near the top of the file.
fn rule(selector: &str) -> &'static str {
    let css = css();
    let at = css
        .rfind(&format!("\n{selector} {{"))
        .unwrap_or_else(|| panic!("no rule for {selector}"));
    let open = at + css[at..].find('{').unwrap();
    &css[open..open + css[open..].find('}').unwrap()]
}

#[test]
fn each_kind_of_notice_has_its_own_shape_and_only_an_error_is_tinted() {
    let ring = rule(".notice::before");
    assert!(
        ring.contains("border-radius: 50%") && ring.contains("border: 1.5px solid"),
        "{ring}"
    );
    let disc = rule(".notice.ok::before");
    assert!(
        disc.contains("background:") && disc.contains("border: 0"),
        "{disc}"
    );
    let diamond = rule(".notice.warn::before, .notice-defect::before");
    assert!(
        diamond.contains("rotate(45deg)") && diamond.contains("border-radius: 0"),
        "{diamond}"
    );
    let square = rule(".notice.error::before");
    assert!(
        square.contains("border-radius: 0") && !square.contains("rotate"),
        "{square}"
    );
    assert!(rule(".notice").contains("background: transparent"));
    assert!(rule(".notice.error").contains("color-mix(in srgb, var(--danger) 5%, transparent)"));
}

fn person(id: &str, name: &str, visibility: &str) -> Value {
    json!({"id": id, "type": "person", "axgf_version": "1.1", "version_num": 1,
        "identity": {"name": {"display": name, "components": []},
            "gender": {"value": "M"}, "is_living": false, "visibility": visibility},
        "birth": {"date": {"value": "1850", "precision": "year"}},
        "death": {"date": {"value": "1900", "precision": "year"}}})
}

/// Two records of one man — same name, same years — that `dedup` merges into
/// the first; and a third man who only shares the name.
fn app(tag: &str, visibility: &str) -> (axum::Router, Scratch) {
    let mut other = person("m3", "Jan Kowalski", visibility);
    other["birth"]["date"]["value"] = json!("1851");
    let flat = json!({
        "manifest": {"axgf": "1.1", "created_at": "2026-10-12T00:00:00Z"},
        "persons": {"m1": person("m1", "Jan Kowalski", visibility),
                    "m2": person("m2", "Jan Kowalski", visibility),
                    "m3": other},
        "families": {}, "events": {}, "places": {}, "occupations": {}, "links": {},
        "sources": {}, "documents": {}
    });
    let src = scratch(&format!("{tag}-src"));
    let path = src.join("notices.axgf");
    std::fs::write(
        &path,
        axgf_cms::state::export_to_bytes(&flat.to_string()).expect("export"),
    )
    .expect("write");
    let out = app_with_bundle(tag, &path);
    drop(src);
    out
}

async fn deduplicate(app: &axum::Router, cookie: &str) {
    let resp = post_form_as(app, cookie, "/admin/dedup", "families=&back=").await;
    assert!(resp.status().is_success(), "dedup: {}", resp.status());
}

#[tokio::test]
async fn a_merged_persons_old_address_says_where_they_went() {
    let (app, _s) = app("merged", "public");
    let cookie = admin_cookie(&app).await;
    deduplicate(&app, &cookie).await;

    let resp = get_with_cookie(&app, "/person/m2", "axgf_lang=fr").await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    let html = body_string(resp).await;
    assert!(html.contains("Cette fiche n’existe plus."), "{html}");
    assert!(
        html.contains("Elle a été fusionnée avec Jan Kowalski le "),
        "{html}"
    );
    assert!(html
        .contains("Les liens vers l’ancienne adresse mènent désormais vers la fiche fusionnée."));
    // The two ways on, as buttons, the merged record first.
    let open =
        html.find(r#"<a class="btn primary" href="/person/m1">Ouvrir la fiche fusionnée</a>"#);
    let river = html.find(r#"<a class="btn" href="/">Retour à la rivière</a>"#);
    assert!(open.is_some() && river.is_some() && open < river, "{html}");
    // The journal says what happened, not "no field changed".
    let dash =
        body_string(get_with_cookie(&app, "/admin", &format!("{cookie}; axgf_lang=fr")).await)
            .await;
    assert!(
        dash.contains("a fusionné dans une autre fiche"),
        "the dashboard's recent edits"
    );
    assert!(!dash.contains("aucun champ modifié"));
    // The survivor is there, and the man who only shares a name is untouched.
    assert_eq!(get(&app, "/person/m1").await.status(), StatusCode::OK);
    assert_eq!(get(&app, "/person/m3").await.status(), StatusCode::OK);
}

#[tokio::test]
async fn an_old_address_names_no_one_its_reader_may_not_see() {
    let (app, _s) = app("merged-private", "private");
    let cookie = admin_cookie(&app).await;
    deduplicate(&app, &cookie).await;

    // A visitor: the survivor is private, so this is simply not found.
    let resp = get(&app, "/person/m2").await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    let html = body_string(resp).await;
    assert!(
        !html.contains("/person/m1") && !html.contains("Kowalski"),
        "{html}"
    );
    let nobody = body_string(get(&app, "/person/never-was").await).await;
    assert_eq!(
        html.replace("m2", ""),
        nobody.replace("never-was", ""),
        "the same page as an id that never existed"
    );
    // Signed in, the same address says where they went.
    let html = body_string(get_with_cookie(&app, "/person/m2", &cookie).await).await;
    assert!(html.contains(r#"href="/person/m1""#), "{html}");
}

#[tokio::test]
async fn an_error_page_says_what_happened_then_offers_the_way_on() {
    let (app, _s) = app_with_empty_bundle("notice-error");
    let html = body_string(get(&app, "/person/nobody").await).await;
    // Not here: information, a ring — the plain notice.
    let notice = &html[html.find(r#"<div class="notice">"#).expect("a notice")..];
    let p = notice.find("<p>").unwrap();
    let actions = notice.find(r#"<div class="notice-actions">"#).unwrap();
    assert!(p < actions, "the sentence, then the buttons");
    assert!(notice[actions..].contains(r#"<a class="btn" href="/">"#));
}
