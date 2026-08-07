//! The identity view, driven through the real router.

mod common;

use axum::http::StatusCode;
use common::*;
use serde_json::json;

/// A bundle exercising every showcase feature at once.
fn showcase_bundle(tag: &str) -> std::path::PathBuf {
    let dir = scratch(tag);
    let path = dir.join("showcase.axgf");
    let flat = json!({
        "manifest": {"axgf": "1.0"},
        "persons": {
            "11111111-1111-4111-8111-111111111111": {
                "id": "11111111-1111-4111-8111-111111111111", "type": "person",
                "axgf_version": "1.0",
                "identity": {
                    "name": {"display": "Jules Meunier", "components": []},
                    "gender": {"value": "M"}, "is_living": false,
                    "names": [{"type": "birth", "display": "Jules Meunier",
                               "components": [], "confidence": 0.9}]},
                "birth": {"date": {"value": "1500", "circa": true, "precision": "year"},
                          "confidence": 0.35},
                "death": {"date": {"precision": "unknown",
                                   "note": "Michaelmas term, 3 Edw. III"},
                          "confidence": 0.2}
            },
            "22222222-2222-4222-8222-222222222222": {
                "id": "22222222-2222-4222-8222-222222222222", "type": "person",
                "axgf_version": "1.0",
                "identity": {"name": {"display": "Jean Boucher", "components": []}}
            }
        },
        "families": {},
        "links": {
            "33333333-3333-4333-8333-333333333333": {
                "id": "33333333-3333-4333-8333-333333333333", "type": "link",
                "axgf_version": "1.0",
                "from": {"entity_type": "person",
                         "entity_id": "22222222-2222-4222-8222-222222222222"},
                "to": {"entity_type": "person",
                       "entity_id": "11111111-1111-4111-8111-111111111111"},
                "label": "godfather", "label_reverse": "godson",
                "category": "spiritual",
                "valid_from": {"date": {"value": "1950", "precision": "year"}},
                "confidence": 0.85,
                "source_id": "44444444-4444-4444-8444-444444444444",
                "note": "per a family letter"}
        },
        "occupations": {
            "55555555-5555-4555-8555-555555555555": {
                "id": "55555555-5555-4555-8555-555555555555", "type": "occupation",
                "axgf_version": "1.0",
                "person_id": "11111111-1111-4111-8111-111111111111",
                "title": "Schoolteacher",
                "employer": {"name": "École Normale"},
                "valid_from": {"date": {"value": "1948", "precision": "year"}},
                "valid_until": {"date": {"value": "1978", "precision": "year"}},
                "confidence": 0.9}
        },
        "sources": {
            "44444444-4444-4444-8444-444444444444": {
                "id": "44444444-4444-4444-8444-444444444444", "type": "source",
                "axgf_version": "1.0", "title": "Family letter, 1954",
                "source_type": "letter", "reliability": "oral", "confidence": 0.6}
        },
        "places": {}, "events": {}, "documents": {}
    });
    let bytes = axgf_cms::state::export_to_bytes(&flat.to_string()).expect("export");
    std::fs::write(&path, bytes).expect("write");
    path
}

const JULES: &str = "11111111-1111-4111-8111-111111111111";

#[tokio::test]
async fn person_page_surfaces_every_gedcom_gap() {
    let src = showcase_bundle("person-src");
    let (app, _p) = app_with_bundle("person", &src);
    let body = expect_status(
        get(&app, &format!("/person/{JULES}")).await,
        StatusCode::OK,
        "person page",
    )
    .await;

    // Date honesty: circa and preserved-unparseable both stated as such.
    assert!(body.contains("circa 1500"), "circa must be stated");
    assert!(
        body.contains("Michaelmas term, 3 Edw. III"),
        "unparseable text must survive, not be dropped"
    );
    assert!(
        body.contains(r#"data-kind="preserved""#),
        "preserved text needs its own styling hook"
    );
    assert!(
        body.contains(r#"data-kind="approximate""#),
        "an approximate date must be marked approximate"
    );

    // Confidence is rendered visually, with bands.
    assert!(body.contains("conf-bar"), "confidence needs a visual form");
    assert!(
        body.contains(r#"data-band="low""#),
        "a 0.35 fact must land in the low band"
    );

    // First-class links, in their own section.
    assert!(body.contains("links-section"));
    assert!(
        body.contains("godson"),
        "the link reads from this person's side"
    );
    assert!(body.contains("per a family letter"));

    // Occupations as spans.
    assert!(body.contains("Schoolteacher"));
    assert!(
        body.contains("tl-bar"),
        "an occupation is a bar, not a bullet"
    );
    assert!(body.contains("1948–1978"));

    // Sources graded by reliability.
    assert!(body.contains("src-chip"));
    assert!(body.contains("oral"));
}

#[tokio::test]
async fn a_missing_person_is_a_404_page_not_a_panic() {
    let (app, _p) = app_with_empty_bundle("person-404");
    let body = expect_status(
        get(&app, "/person/99999999-9999-4999-8999-999999999999").await,
        StatusCode::NOT_FOUND,
        "missing person",
    )
    .await;
    assert!(body.contains("No such person"));
}

#[tokio::test]
async fn the_edit_link_appears_only_for_an_admin() {
    let src = showcase_bundle("person-edit-src");
    let (app, _p) = app_with_bundle("person-edit", &src);

    let anon = body_string(get(&app, &format!("/person/{JULES}")).await).await;
    assert!(
        !anon.contains("/admin/person/"),
        "an anonymous visitor must not see an edit link"
    );

    let admin = body_string(get_admin(&app, &format!("/person/{JULES}")).await).await;
    assert!(
        admin.contains(&format!("/admin/person/{JULES}/edit")),
        "an admin should see the edit link"
    );
}

#[tokio::test]
async fn a_referenced_but_absent_person_renders_unlinked() {
    let dir = scratch("ghost-src");
    let path = dir.join("ghost.axgf");
    let flat = json!({
        "manifest": {"axgf": "1.0"},
        "persons": {
            "aaaaaaaa-1111-4111-8111-111111111111": {
                "id": "aaaaaaaa-1111-4111-8111-111111111111", "type": "person",
                "axgf_version": "1.0",
                "identity": {"name": {"display": "Present Person", "components": []}}}
        },
        "families": {
            "bbbbbbbb-1111-4111-8111-111111111111": {
                "id": "bbbbbbbb-1111-4111-8111-111111111111", "type": "family",
                "axgf_version": "1.0",
                "union": {"type": "marriage", "persons": [
                    {"person_id": "aaaaaaaa-1111-4111-8111-111111111111", "role": "spouse"}]},
                "children": [
                    {"person_id": "cccccccc-9999-4999-8999-999999999999", "confidence": 0.5}]}
        },
        "links": {}, "occupations": {}, "sources": {}, "places": {},
        "events": {}, "documents": {}
    });
    let bytes = axgf_cms::state::export_to_bytes(&flat.to_string()).expect("export");
    std::fs::write(&path, bytes).expect("write");

    let (app, _p) = app_with_bundle("ghost", &path);
    let body = expect_status(
        get(&app, "/person/aaaaaaaa-1111-4111-8111-111111111111").await,
        StatusCode::OK,
        "person with a ghost child",
    )
    .await;

    assert!(
        body.contains("[Unknown]"),
        "an absent person is named as unknown"
    );
    assert!(
        !body.contains("/person/cccccccc-9999-4999-8999-999999999999"),
        "an absent person must not be linked into a 404"
    );
}
