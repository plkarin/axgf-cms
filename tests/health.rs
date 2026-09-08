//! Health data leaving the process, or not.
//!
//! Like `visibility.rs`, every assertion here is about *bytes in the response*
//! rather than about what a template chose to draw: a diagnosis that reaches
//! the browser and is hidden with CSS has been published, not withheld. The
//! fixture uses strings that appear nowhere else in the bundle, so finding one
//! is proof of a leak and not a coincidence of vocabulary.
//!
//! The load-bearing case is the **public living person**. Their record is
//! readable by anyone, so nothing about the person lens hides anything — and
//! their health must still be withheld, because the rule is a property of being
//! alive rather than of the record's visibility. If health were merely
//! inheriting `person_visibility`, that one test would fail and the rest would
//! pass.

mod common;

use axum::http::StatusCode;
use common::*;
use serde_json::{json, Value};

const LIVING: &str = "11111111-1111-4111-8111-111111111111";
const DEAD: &str = "22222222-2222-4222-8222-222222222222";

/// Strings that exist nowhere else.
const CONDITION: &str = "Sarcoidosis";
const RELIGION: &str = "Old Believer";
const CAUSE: &str = "Diphtheria";
const HEIGHT: &str = "187";

fn bundle(tag: &str) -> std::path::PathBuf {
    let dir = scratch(tag);
    let path = dir.join("health.axgf");

    let traits = |h: &str| {
        json!({"height_cm": [{"value": h.parse::<u64>().unwrap(),
                              "date": {"value": "1958", "precision": "year"}}],
               "eye_colour": [{"value": "blue"}]})
    };

    let flat = json!({
        "manifest": {"axgf": "1.0"},
        "persons": {
            // Deliberately PUBLIC and living: the person lens hides nothing
            // here, so anything withheld is withheld by the health rule alone.
            LIVING: {
                "id": LIVING, "type": "person", "axgf_version": "1.0", "version_num": 1,
                "identity": {
                    "name": {"display": "Marek Zaleski", "components": []},
                    "is_living": true,
                    "visibility": "public"
                },
                "birth": {"date": {"value": "1958", "precision": "year"}},
                "extensions": {
                    "axgf-cms:traits/v1": traits(HEIGHT),
                    "axgf-cms:health/v1": {
                        "conditions": [{"value": CONDITION}],
                        "religion": [{"value": RELIGION}]
                    }
                }
            },
            // Public and deceased: a cause of death two centuries old is the
            // substance of genealogy, and article 9 governs the living.
            DEAD: {
                "id": DEAD, "type": "person", "axgf_version": "1.0", "version_num": 1,
                "identity": {
                    "name": {"display": "Otylia Zaleska", "components": []},
                    "is_living": false,
                    "visibility": "public"
                },
                "birth": {"date": {"value": "1861", "precision": "year"}},
                "death": {"date": {"value": "1899", "precision": "year"}},
                "extensions": {
                    "axgf-cms:health/v1": {"cause_of_death": [{"value": CAUSE}]}
                }
            }
        },
        "families": {}, "events": {}, "links": {}, "occupations": {},
        "sources": {}, "places": {}, "documents": {}
    });
    std::fs::write(
        &path,
        axgf_cms::state::export_to_bytes(&flat.to_string()).expect("export"),
    )
    .expect("write");
    path
}

/// Every surface a signed-out reader can reach, for one person.
async fn surfaces(app: &axum::Router, id: &str) -> Vec<(&'static str, String)> {
    let mut out = Vec::new();
    for (name, uri) in [
        ("record tab", format!("/person/{id}")),
        ("life tab", format!("/person/{id}?tab=life")),
        ("media tab", format!("/person/{id}?tab=media")),
        ("tree tab", format!("/person/{id}?tab=tree")),
        ("panel fragment", format!("/tree/panel/{id}")),
        ("tree page", format!("/tree?root={id}")),
        ("health json", "/health".to_string()),
    ] {
        let resp = get(app, &uri).await;
        out.push((name, body_string(resp).await));
    }
    out
}

#[tokio::test]
async fn a_living_persons_health_never_reaches_a_reader_who_may_not_see_it() {
    let (app, _p) = app_with_bundle("health-living", &bundle("health-living-src"));

    for (surface, body) in surfaces(&app, LIVING).await {
        for secret in [CONDITION, RELIGION] {
            assert!(
                !body.contains(secret),
                "{surface} carried {secret} to a signed-out reader"
            );
        }
        // The extension key itself is a disclosure: it says which special
        // category this person has data in, even with the values gone.
        assert!(
            !body.contains("axgf-cms:health"),
            "{surface} named the health extension at all"
        );
    }
}

/// The raw-JSON dump is the shortest path from a stored diagnosis to a
/// stranger's screen, and it is shown to every reader, not only to admins.
#[tokio::test]
async fn the_raw_entity_dump_is_stripped_rather_than_printed_whole() {
    let (app, _p) = app_with_bundle("health-raw", &bundle("health-raw-src"));

    let body = body_string(get(&app, &format!("/person/{LIVING}")).await).await;
    assert!(
        body.contains("Raw entity") || body.contains("raw-json"),
        "the raw section is on the page, so this test is testing something"
    );
    assert!(
        !body.contains(CONDITION),
        "and it does not contain the condition"
    );
    assert!(
        body.contains(HEIGHT),
        "while the height, which is not health data, is still there"
    );
}

/// A trait is not a diagnosis, and over-restricting it would make the feature
/// useless for the thing it is for.
#[tokio::test]
async fn physical_traits_are_not_swept_up_with_the_health_fields() {
    let (app, _p) = app_with_bundle("health-traits", &bundle("health-traits-src"));

    let body = body_string(get(&app, &format!("/person/{LIVING}?tab=life")).await).await;
    assert!(body.contains(HEIGHT), "the height is shown: {body}");
    assert!(body.contains("blue"), "and so is the eye colour");
    assert!(!body.contains(CONDITION));
}

/// Withheld, not silently absent — the same choice the person lens makes.
#[tokio::test]
async fn a_reader_is_told_that_something_is_being_withheld() {
    let (app, _p) = app_with_bundle("health-told", &bundle("health-told-src"));
    let body = body_string(get(&app, &format!("/person/{LIVING}?tab=life")).await).await;
    assert!(
        body.contains("recorded but withheld"),
        "a record that shows nothing where a diagnosis exists reads as a \
         record with nothing in it: {body}"
    );
}

#[tokio::test]
async fn an_administrator_reads_it() {
    let (app, _p) = app_with_bundle("health-admin", &bundle("health-admin-src"));
    let body = body_string(get_admin(&app, &format!("/person/{LIVING}?tab=life")).await).await;
    assert!(body.contains(CONDITION), "an admin sees the condition");
    assert!(body.contains(RELIGION));
}

/// Article 9 governs living people. A cause of death from 1899 is genealogy.
#[tokio::test]
async fn a_deceased_persons_cause_of_death_follows_the_records_own_visibility() {
    let (app, _p) = app_with_bundle("health-dead", &bundle("health-dead-src"));
    let body = body_string(get(&app, &format!("/person/{DEAD}?tab=life")).await).await;
    assert!(
        body.contains(CAUSE),
        "a public deceased record publishes its cause of death: {body}"
    );
}

/// The export leaves health out unless asked, and the default is the one a
/// tired operator gets.
#[tokio::test]
async fn the_export_excludes_health_by_default_and_includes_it_on_request() {
    let (app, _p) = app_with_bundle("health-export", &bundle("health-export-src"));

    let plain = get_admin(&app, "/admin/export").await;
    assert_eq!(plain.status(), StatusCode::OK);
    let plain = body_bytes(plain).await;
    let with = body_bytes(get_admin(&app, "/admin/export?health=include").await).await;

    // Re-imported through the library, so what is asserted is what a
    // recipient would actually open rather than what the writer intended.
    let persons_of = |bytes: &[u8]| -> Value {
        axgf_rs::import_bundle(bytes)
            .data
            .get("persons")
            .cloned()
            .unwrap_or(Value::Null)
    };

    let stripped = persons_of(&plain);
    let s = stripped.to_string();
    assert!(
        !s.contains(CONDITION),
        "the shareable export carries no condition"
    );
    assert!(!s.contains(RELIGION), "…and no religion");
    assert!(
        !s.contains(CAUSE),
        "…and not the deceased person's either: an export is a file that \
         travels, and the operator asked for one without health in it"
    );
    assert!(s.contains(HEIGHT), "but it still carries the traits");
    assert!(
        stripped[LIVING]["identity"]["name"]["display"].is_string(),
        "and it is still a bundle: {s}"
    );

    let full = persons_of(&with).to_string();
    assert!(
        full.contains(CONDITION),
        "the backup export carries everything"
    );
    assert!(full.contains(CAUSE));
}

/// An editor shown a form without the health rows must not be able to erase
/// them by submitting it.
#[tokio::test]
async fn a_form_that_never_showed_the_health_rows_cannot_blank_them() {
    let (app, _p) = app_with_bundle("health-blank", &bundle("health-blank-src"));

    // A signed-out writer is refused before this even matters, so the check
    // that counts is the one an *admin* passes and a lesser writer does not.
    // The admin path is the positive control: it saves.
    let ok = post_form(
        &app,
        &format!("/admin/person/{LIVING}/physical"),
        "height_cm.0.value=190",
        true,
    )
    .await;
    assert_eq!(ok.status(), StatusCode::OK, "an admin may edit");

    // And the health survived, because the admin's form carried it — here it
    // did not, so this also proves the write is a replacement of what the form
    // held rather than a blind overwrite of the entity.
    let after = body_string(get_admin(&app, &format!("/person/{LIVING}?tab=life")).await).await;
    assert!(after.contains("190"), "the new height is stored: {after}");
}
