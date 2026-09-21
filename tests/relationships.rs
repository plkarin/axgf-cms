//! The relationships a record holds outside itself, and the writes that keep
//! them honest.
//!
//! AXGF 1.1 puts three facts about a relationship on the entity that holds it
//! rather than on the person: how a child is a child of a family (`lineage`),
//! what kind of tie a link is (`relation`), and the post somebody held within
//! an occupation (`position`). Each is edited on the page that edits that
//! entity, and each obliges the entity to declare `"1.1"` — which the update
//! path used to leave at `"1.0"`, so every such save came back with a
//! `SPEC_VERSION_MISMATCH`.
//!
//! Alongside them, the write paths those pages share: a delete that is not a
//! save in disguise, an avatar choice that checks the version it was made on,
//! and uploads that write a Document the schema accepts.

mod common;

use axum::http::StatusCode;
use common::*;
use serde_json::{json, Value};

const MOTHER: &str = "11111111-1111-4111-8111-111111111111";
const FATHER: &str = "22222222-2222-4222-8222-222222222222";
const CHILD: &str = "33333333-3333-4333-8333-333333333333";
const GODFATHER: &str = "44444444-4444-4444-8444-444444444444";
const FAMILY: &str = "55555555-5555-4555-8555-555555555555";

fn person(id: &str, name: &str) -> Value {
    json!({
        "id": id, "type": "person", "axgf_version": "1.0", "version_num": 1,
        "identity": {
            "name": {"display": name, "components": []},
            "gender": {"value": "U"}, "is_living": false, "visibility": "public"
        }
    })
}

fn app(tag: &str) -> (axum::Router, common::Scratch) {
    let dir = scratch(&format!("{tag}-src"));
    let path = dir.join("rel.axgf");
    let flat = json!({
        "manifest": {"axgf": "1.0", "family": {"name": "Relationships"}},
        "persons": {
            MOTHER: person(MOTHER, "Zofia Brandt"),
            FATHER: person(FATHER, "Karol Brandt"),
            CHILD: person(CHILD, "Ada Brandt"),
            GODFATHER: person(GODFATHER, "Ignacy Wolski"),
        },
        "families": {
            FAMILY: {
                "id": FAMILY, "type": "family", "axgf_version": "1.0", "version_num": 1,
                "union": {"type": "marriage", "persons": [
                    {"person_id": MOTHER, "role": "spouse"},
                    {"person_id": FATHER, "role": "spouse"}]},
                "children": [{"person_id": CHILD}]
            }
        },
        "events": {}, "links": {}, "occupations": {}, "sources": {}, "places": {},
        "documents": {}
    });
    std::fs::write(
        &path,
        axgf_cms::state::export_to_bytes(&flat.to_string()).expect("export"),
    )
    .expect("write");
    app_with_bundle(tag, &path)
}

/// The entity as stored, read back out of the generic editor's raw textarea.
async fn stored(app: &axum::Router, kind: &str, id: &str) -> Value {
    let page = body_string(get_admin(app, &format!("/admin/{kind}/{id}/edit")).await).await;
    let start = page.find("name=\"raw_json\"").expect("a raw textarea");
    let open = page[start..].find('>').expect("textarea opens") + start + 1;
    let close = page[open..].find("</textarea>").expect("textarea closes") + open;
    let raw = page[open..close]
        .replace("&quot;", "\"")
        .replace("&#x27;", "'")
        .replace("&#39;", "'")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&");
    serde_json::from_str(&raw).expect("the textarea holds the entity")
}

/// The one entity of `kind` in the bundle, as `(id, entity)`, found through
/// the admin listing's edit links.
async fn only_entity_of(app: &axum::Router, kind: &str) -> (String, Value) {
    let page = body_string(get_admin(app, &format!("/admin/{kind}")).await).await;
    let marker = format!("/admin/{kind}/");
    let id = page
        .match_indices(&marker)
        .filter_map(|(i, _)| {
            let rest = &page[i + marker.len()..];
            let id: String = rest.chars().take_while(|c| *c != '/').collect();
            (rest[id.len()..].starts_with("/edit") && id.len() == 36).then_some(id)
        })
        .next()
        .unwrap_or_else(|| panic!("one {kind} is listed"));
    let entity = stored(app, kind, &id).await;
    (id, entity)
}

#[tokio::test]
async fn a_childs_lineage_is_saved_on_the_family_and_the_family_declares_1_1() {
    let (app, _p) = app("lineage");
    let resp = post_form(
        &app,
        &format!("/admin/person/{MOTHER}/family/{FAMILY}"),
        &format!(
            "base_version=1&type=marriage&partner.0.person={MOTHER}&partner.0.role=spouse\
             &partner.1.person={FATHER}&partner.1.role=spouse\
             &child.0.person={CHILD}&child.0.lineage=adoptive&child.0.birth_order=1"
        ),
        true,
    )
    .await;
    let page = expect_status(resp, StatusCode::OK, "family save").await;
    assert!(
        !page.contains("SPEC_VERSION_MISMATCH"),
        "the family must declare the version its lineage needs: {page}"
    );

    let fam = stored(&app, "family", FAMILY).await;
    assert_eq!(fam["children"][0]["lineage"], "adoptive");
    assert_eq!(fam["axgf_version"], "1.1");

    // A term the vocabulary does not have is no claim at all.
    post_form(
        &app,
        &format!("/admin/person/{MOTHER}/family/{FAMILY}"),
        &format!(
            "base_version=2&type=marriage&partner.0.person={MOTHER}&partner.0.role=spouse\
             &child.0.person={CHILD}&child.0.lineage=changeling"
        ),
        true,
    )
    .await;
    let fam = stored(&app, "family", FAMILY).await;
    assert!(fam["children"][0].get("lineage").is_none());
}

#[tokio::test]
async fn the_family_editor_offers_every_lineage_by_name() {
    let (app, _p) = app("lineage-offer");
    let page = body_string(get_admin(&app, &format!("/admin/person/{MOTHER}/family")).await).await;
    for term in [
        "biological",
        "adoptive",
        "foster",
        "step",
        "guardianship",
        "unknown",
    ] {
        assert!(
            page.contains(&format!("value=\"{term}\"")),
            "{term} is offered"
        );
    }
    assert!(page.contains("Adoptive"), "and named, not spelt as a code");
}

#[tokio::test]
async fn a_links_relation_is_saved_and_declares_1_1() {
    let (app, _p) = app("relation");
    let resp = post_form(
        &app,
        &format!("/admin/person/{CHILD}/links"),
        &format!("to={GODFATHER}&label=chrzestny&relation=godparent&confidence=0.9"),
        true,
    )
    .await;
    let page = expect_status(resp, StatusCode::OK, "link create").await;
    assert!(!page.contains("SPEC_VERSION_MISMATCH"), "{page}");

    let (_id, link) = only_entity_of(&app, "link").await;
    assert_eq!(link["relation"], "godparent");
    assert_eq!(link["label"], "chrzestny", "the record's own word is kept");
    assert_eq!(link["axgf_version"], "1.1");
}

#[tokio::test]
async fn an_occupations_position_is_saved_and_declares_1_1() {
    let (app, _p) = app("position");
    let resp = post_form(
        &app,
        &format!("/admin/person/{MOTHER}/occupations"),
        "title=Teacher&position=Headmistress",
        true,
    )
    .await;
    let page = expect_status(resp, StatusCode::OK, "occupation create").await;
    assert!(!page.contains("SPEC_VERSION_MISMATCH"), "{page}");

    let (_id, occ) = only_entity_of(&app, "occupation").await;
    assert_eq!(occ["position"], "Headmistress");
    assert_eq!(occ["axgf_version"], "1.1");
}

#[tokio::test]
async fn every_write_declares_the_version_its_content_needs() {
    // The profile editor updates a person declared 1.0. The library warns
    // when content and declaration disagree, so a clean result page is the
    // library's own verdict that the declaration was raised.
    let (app, _p) = app("declared");
    let resp = post_form(
        &app,
        &format!("/admin/person/{FATHER}/profile/health"),
        "base_version=1&health.blood_group.0.v=AB",
        true,
    )
    .await;
    let page = expect_status(resp, StatusCode::OK, "profile save").await;
    assert!(!page.contains("SPEC_VERSION_MISMATCH"), "{page}");
    assert_eq!(stored(&app, "person", FATHER).await["axgf_version"], "1.1");

    // Content that needs nothing new changes nothing: a 1.0 save stays 1.0.
    post_form(
        &app,
        &format!("/admin/person/{MOTHER}/identity"),
        "base_version=1&name.display=Zofia+Brandt&gender.value=F",
        true,
    )
    .await;
    assert_eq!(stored(&app, "person", MOTHER).await["axgf_version"], "1.0");
}

#[tokio::test]
async fn a_delete_control_is_never_inside_the_form_it_sits_beside() {
    // A form nested in a form is dropped by the HTML parser, which made every
    // "delete" on these pages submit the save around it instead.
    let (app, _p) = app("nesting");
    post_form(
        &app,
        &format!("/admin/person/{CHILD}/links"),
        &format!("to={GODFATHER}&label=godfather"),
        true,
    )
    .await;
    post_form(
        &app,
        &format!("/admin/person/{CHILD}/occupations"),
        "title=Scribe",
        true,
    )
    .await;
    for page in ["links", "occupations", "events", "documents", "family"] {
        let html =
            body_string(get_admin(&app, &format!("/admin/person/{CHILD}/{page}")).await).await;
        let mut depth = 0i32;
        let mut deepest = 0i32;
        let mut rest = html.as_str();
        while let Some(i) = rest.find("form") {
            let before = &rest[..i];
            if before.ends_with('<') {
                depth += 1;
                deepest = deepest.max(depth);
            } else if before.ends_with("</") {
                depth -= 1;
            }
            rest = &rest[i + 4..];
        }
        assert!(deepest <= 1, "{page}: a form is nested inside another");
    }
}

#[tokio::test]
async fn an_avatar_chosen_on_an_older_version_meets_the_conflict_page() {
    let (app, _p) = app("avatar-stale");
    // Somebody else saves the record first.
    post_form(
        &app,
        &format!("/admin/person/{MOTHER}/identity"),
        "base_version=1&name.display=Zofia+Brandt-Wolska&gender.value=F",
        true,
    )
    .await;
    let resp = post_form(
        &app,
        &format!("/admin/person/{MOTHER}/avatar"),
        "choice=none&base_version=1",
        true,
    )
    .await;
    let page = expect_status(
        resp,
        StatusCode::CONFLICT,
        "the conflict page, not a redirect as if it had saved",
    )
    .await;
    assert!(page.contains("Someone else changed this first"), "{page}");
    assert!(
        stored(&app, "person", MOTHER)
            .await
            .get("extensions")
            .is_none(),
        "no avatar choice was written"
    );

    // Made on the version that is there, it saves.
    let resp = post_form(
        &app,
        &format!("/admin/person/{MOTHER}/avatar"),
        "choice=none&base_version=2",
        true,
    )
    .await;
    assert!(resp.status().is_redirection());
}

#[tokio::test]
async fn upload_forms_offer_only_the_schemas_document_types() {
    let (app, _p) = app("doc-types");
    let page =
        body_string(get_admin(&app, &format!("/admin/person/{MOTHER}/documents")).await).await;
    for bad in [
        "value=\"certificate\"",
        "value=\"record\"",
        "value=\"newspaper\"",
        "value=\"portrait\"",
    ] {
        assert!(!page.contains(bad), "{bad} is not a type AXGF has");
    }
    for good in [
        "value=\"birth_certificate\"",
        "value=\"newspaper_clipping\"",
        "value=\"photo\"",
    ] {
        assert!(page.contains(good), "{good} is offered");
    }
    assert!(
        page.contains("name=\"set_avatar\""),
        "the checkbox is the field the handler reads"
    );
    let avatar =
        body_string(get_admin(&app, &format!("/admin/person/{MOTHER}/avatar")).await).await;
    assert!(!avatar.contains("value=\"portrait\""));
}
