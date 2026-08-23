//! What a reader who may not read a record actually receives.
//!
//! Every assertion here is about *bytes leaving the process*, not about what a
//! template chose to draw. That is the distinction the whole feature turns on:
//! a name that reaches the browser and is hidden with CSS has not been
//! withheld, it has been published with a note asking politely. So these tests
//! search the raw response body for the secret string, on every surface that
//! can carry it — the tree page, the panel fragment, the standalone page, the
//! JSON endpoint and the document bytes.
//!
//! The fixture is built so that a leak is unambiguous: the hidden people have
//! names that appear nowhere else, so `body.contains("Ludwika")` is proof and
//! not a coincidence.

mod common;

use axum::http::StatusCode;
use common::*;
use serde_json::json;
use tower::ServiceExt as _;

const OPEN: &str = "11111111-1111-4111-8111-111111111111";
const MEMBERS: &str = "22222222-2222-4222-8222-222222222222";
const PRIVATE: &str = "33333333-3333-4333-8333-333333333333";
const CHILD: &str = "44444444-4444-4444-8444-444444444444";
const FAMILY: &str = "55555555-5555-4555-8555-555555555555";
const QUIET_LINK: &str = "88888888-8888-4888-8888-888888888888";

/// Names that exist nowhere else in the bundle, so finding one in a response
/// body is proof of a leak rather than an accident of shared vocabulary.
const MEMBERS_NAME: &str = "Ludwika Zawadzka";
const PRIVATE_NAME: &str = "Bronisława Wierzbięta";
const OPEN_NAME: &str = "Kazimierz Dąbrowski";

fn person(id: &str, display: &str, visibility: Option<&str>, living: bool) -> serde_json::Value {
    let mut identity = json!({
        "name": {"display": display, "components": [
            {"type": "given_name", "value": display.split(' ').next().unwrap(), "order": 1}]},
        "gender": {"value": "F"},
        "is_living": living
    });
    if let Some(v) = visibility {
        identity["visibility"] = json!(v);
    }
    json!({
        "id": id, "type": "person", "axgf_version": "1.0",
        "identity": identity,
        "birth": {"date": {"value": "1899-04-02", "precision": "exact"}, "confidence": 0.9},
        "documents": []
    })
}

/// A family where a public child has one public and one hidden parent, plus a
/// private person off to the side, a private link and two documents.
fn bundle(tag: &str) -> std::path::PathBuf {
    let dir = scratch(tag);
    let path = dir.join("vis.axgf");

    // No documents are declared here. The one test that needs them uploads
    // real files through the admin route, because a document whose bytes are
    // absent 404s for every reader — which would make a permission test pass
    // for the wrong reason.
    let open = person(OPEN, OPEN_NAME, Some("public"), false);
    let members = person(MEMBERS, MEMBERS_NAME, Some("members"), true);

    let flat = json!({
        "manifest": {"axgf": "1.0", "family": {"name": "Test"}},
        "persons": {
            OPEN: open,
            MEMBERS: members,
            PRIVATE: person(PRIVATE, PRIVATE_NAME, Some("private"), false),
            CHILD: person(CHILD, "Jan Dąbrowski", Some("public"), false)
        },
        "families": {
            FAMILY: {
                "id": FAMILY, "type": "family", "axgf_version": "1.0",
                "union": {"persons": [
                    {"person_id": OPEN, "role": "wife"},
                    {"person_id": MEMBERS, "role": "husband"}],
                    "union_type": "marriage", "confidence": 0.9},
                "children": [{"person_id": CHILD, "confidence": 0.95}]
            }
        },
        "events": {},
        "links": {
            QUIET_LINK: {
                "id": QUIET_LINK, "type": "link", "axgf_version": "1.0",
                "from": {"entity_type": "person", "entity_id": CHILD},
                "to": {"entity_type": "person", "entity_id": OPEN},
                "label": "acknowledged natural parent of",
                "label_reverse": "acknowledged natural child of",
                "visibility": "private", "confidence": 0.6
            }
        },
        "occupations": {}, "sources": {}, "places": {},
        "documents": {}
    });
    let bytes = axgf_cms::state::export_to_bytes(&flat.to_string()).expect("export");
    std::fs::write(&path, bytes).expect("write");
    path
}

fn app(tag: &str) -> axum::Router {
    let src = bundle(&format!("{tag}-src"));
    app_with_bundle(tag, &src).0
}

/// Assert that none of `secrets` appears anywhere in `body`.
fn no_leak(surface: &str, body: &str, secrets: &[&str]) {
    for s in secrets {
        assert!(
            !body.contains(s),
            "{surface} leaked {s:?} to a reader who may not read it.\n\
             This is the one bug this feature exists to prevent: the string \
             reached the browser. Whether it was styled away does not matter."
        );
    }
}

#[tokio::test]
async fn a_signed_out_reader_gets_no_hidden_name_on_any_surface() {
    let app = app("vis-surfaces");
    let secrets = [MEMBERS_NAME, PRIVATE_NAME];

    // The tree page draws every card, including the hidden ones.
    let tree = body_string(get(&app, "/tree?all=1").await).await;
    no_leak("/tree?all=1", &tree, &secrets);
    assert!(tree.contains(OPEN_NAME), "a public person still renders");

    // The focused default, which also renders a panel server-side.
    let focused = body_string(get(&app, "/tree").await).await;
    no_leak("/tree", &focused, &secrets);

    // The panel fragment: a separate route that returns no page and would
    // never have been covered by a check inside a page template.
    for id in [MEMBERS, PRIVATE] {
        let resp = get(&app, &format!("/tree/panel/{id}")).await;
        assert_eq!(
            resp.status(),
            StatusCode::FORBIDDEN,
            "the panel fetch must refuse a record it may not serve"
        );
        no_leak("/tree/panel", &body_string(resp).await, &secrets);
    }

    // The standalone page.
    for id in [MEMBERS, PRIVATE] {
        let resp = get(&app, &format!("/person/{id}")).await;
        assert_eq!(resp.status(), StatusCode::FORBIDDEN);
        no_leak("/person", &body_string(resp).await, &secrets);
    }

    // The child's own page, which names both parents — one of whom is hidden.
    let child = body_string(get(&app, &format!("/person/{CHILD}")).await).await;
    no_leak("/person (as a relative)", &child, &secrets);
    assert!(
        child.contains(OPEN_NAME),
        "the parent who *is* readable must still be named"
    );

    // The home page's showcase, which counts and links examples.
    no_leak("/", &body_string(get(&app, "/").await).await, &secrets);
}

#[tokio::test]
async fn an_admin_reads_everything_the_bundle_holds() {
    let app = app("vis-admin");
    for id in [MEMBERS, PRIVATE] {
        let body = expect_status(
            get_admin(&app, &format!("/person/{id}")).await,
            StatusCode::OK,
            "admin reads a hidden record",
        )
        .await;
        assert!(body.contains(MEMBERS_NAME) || body.contains(PRIVATE_NAME));
    }
    let tree = body_string(get_admin(&app, "/tree?all=1").await).await;
    assert!(tree.contains(MEMBERS_NAME) && tree.contains(PRIVATE_NAME));
    assert!(
        !tree.contains("is-restricted"),
        "nothing is redacted for a reader with no ceiling"
    );
}

#[tokio::test]
async fn a_hidden_parent_keeps_their_place_in_the_family() {
    // Redaction, not omission. A record that showed one parent where the
    // bundle holds two would be a false statement about the genealogy — and
    // for this application that is the one unacceptable failure mode.
    let app = app("vis-shape");
    let child = body_string(get(&app, &format!("/person/{CHILD}")).await).await;
    assert!(
        child.contains("Private"),
        "the hidden parent is shown as withheld, not dropped: {child}"
    );
    assert_eq!(
        child.matches("class=\"restricted\"").count(),
        1,
        "exactly one of the two parents is withheld"
    );
    assert!(
        !child.contains(&format!("/person/{MEMBERS}")),
        "a withheld person carries no link to their record"
    );
}

#[tokio::test]
async fn a_redacted_card_carries_nothing_for_the_filter_to_match() {
    // The client-side filter matches on data-search. A redacted card that
    // still carried the real name there would hand back every hidden name in
    // the bundle, one keystroke at a time — a leak through a feature nobody
    // would think to check.
    let app = app("vis-filter");
    let tree = body_string(get(&app, "/tree?all=1").await).await;
    assert_eq!(
        tree.matches("class=\"tcard is-restricted\"").count(),
        2,
        "the fixture must actually produce redacted cards, or the loop below \
         asserts nothing at all"
    );
    for chunk in tree.split("class=\"tcard is-restricted\"").skip(1) {
        let card = &chunk[..chunk.len().min(400)];
        assert!(
            card.contains("data-search=\"\""),
            "a redacted card must be unsearchable: {card}"
        );
        assert!(
            !card.contains("data-name="),
            "a redacted card must carry no name attribute: {card}"
        );
        assert!(
            card.contains("data-sex=\"u\""),
            "a redacted card must not disclose a recorded gender: {card}"
        );
    }
}

#[tokio::test]
async fn a_link_can_be_private_when_both_its_endpoints_are_not() {
    // The one case that is not reducible to person visibility: an
    // acknowledged natural parentage between two public people. The
    // relationship is the sensitive fact, so the link carries its own
    // visibility and it has to be honoured on its own.
    let app = app("vis-link");
    let anon = body_string(get(&app, &format!("/person/{CHILD}")).await).await;
    assert!(
        !anon.contains("acknowledged natural"),
        "a private link must not render, even between two public people"
    );
    let admin = body_string(get_admin(&app, &format!("/person/{CHILD}")).await).await;
    assert!(
        admin.contains("acknowledged natural"),
        "…and must still render for a reader entitled to it"
    );
}

#[tokio::test]
async fn a_document_is_governed_by_the_person_who_attaches_it() {
    // The payloads are uploaded rather than declared, because a document whose
    // bytes are not in the cache 404s for everybody — which would make this
    // test pass whether or not the permission check existed.
    let app = app("vis-doc");
    let png = tiny_png();
    for person in [OPEN, MEMBERS] {
        let resp = upload_as_admin(&app, person, &png).await;
        assert!(
            resp.status().is_success() || resp.status().is_redirection(),
            "upload for {person} failed with {}",
            resp.status()
        );
    }

    let open_doc = attached_document(&app, OPEN).await;
    let hidden_doc = attached_document(&app, MEMBERS).await;
    assert_ne!(open_doc, hidden_doc, "two distinct uploads");

    // The readable person's file is served on all three doors, so the test
    // below is about permission and not about a broken pipeline.
    for route in ["raw", "view", "thumb"] {
        let resp = get(&app, &format!("/document/{open_doc}/{route}")).await;
        assert_eq!(
            resp.status(),
            StatusCode::OK,
            "/document/{open_doc}/{route} should serve a public person's file"
        );
    }

    // The hidden person's file is served on none of them. /raw, /view and
    // /thumb are three doors into the same bytes, and a check on two of them
    // is a check on none.
    for route in ["raw", "view", "thumb"] {
        let resp = get(&app, &format!("/document/{hidden_doc}/{route}")).await;
        assert_eq!(
            resp.status(),
            StatusCode::NOT_FOUND,
            "/document/{hidden_doc}/{route} must not serve a hidden person's file"
        );
        let body = body_bytes(resp).await;
        assert_ne!(body, png, "the payload itself must not reach the reader");
    }

    // …and an admin still gets it.
    assert_eq!(
        get_admin(&app, &format!("/document/{hidden_doc}/raw"))
            .await
            .status(),
        StatusCode::OK
    );
}

/// The smallest PNG the image decoder accepts: 1x1, opaque.
fn tiny_png() -> Vec<u8> {
    use base64::Engine as _;
    base64::engine::general_purpose::STANDARD
        .decode(
            "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8BQDwAEhQGAhKmMIQAAAABJRU5ErkJggg==",
        )
        .expect("decode fixture png")
}

async fn upload_as_admin(
    app: &axum::Router,
    person: &str,
    bytes: &[u8],
) -> axum::http::Response<axum::body::Body> {
    let boundary = "----axgfcmsvisboundary";
    let mut body: Vec<u8> = Vec::new();
    body.extend_from_slice(
        format!(
            "--{boundary}\r\nContent-Disposition: form-data; name=\"file\"; \
             filename=\"scan.png\"\r\nContent-Type: image/png\r\n\r\n"
        )
        .as_bytes(),
    );
    body.extend_from_slice(bytes);
    body.extend_from_slice(
        format!(
            "\r\n--{boundary}\r\nContent-Disposition: form-data; \
             name=\"caption\"\r\n\r\nA scan\r\n--{boundary}--\r\n"
        )
        .as_bytes(),
    );
    app.clone()
        .oneshot(
            axum::http::Request::builder()
                .uri(format!("/admin/person/{person}/document"))
                .method("POST")
                .header(
                    axum::http::header::CONTENT_TYPE,
                    format!("multipart/form-data; boundary={boundary}"),
                )
                .header(axum::http::header::COOKIE, format!("axgf_admin={TOKEN}"))
                .body(axum::body::Body::from(body))
                .unwrap(),
        )
        .await
        .expect("request")
}

/// The document id this person attaches, read back as an admin.
async fn attached_document(app: &axum::Router, person: &str) -> String {
    let page = body_string(get_admin(app, &format!("/person/{person}")).await).await;
    let marker = "/document/";
    let at = page.find(marker).expect("an attached document");
    page[at + marker.len()..]
        .split(['/', '"'])
        .next()
        .expect("document id")
        .to_string()
}

#[tokio::test]
async fn the_root_picker_lists_only_reachable_people() {
    let app = app("vis-roster");
    let tree = body_string(get(&app, "/tree").await).await;
    let options: Vec<&str> = tree
        .split("<option value=\"")
        .skip(1)
        .map(|c| c.split('"').next().unwrap_or(""))
        .collect();
    assert!(options.contains(&OPEN), "a public person is a destination");
    assert!(
        !options.contains(&MEMBERS) && !options.contains(&PRIVATE),
        "a picker entry that leads to “Private” is not a destination: {options:?}"
    );
}

#[tokio::test]
async fn health_counts_what_the_reader_may_read() {
    let app = app("vis-health");
    let anon: serde_json::Value =
        serde_json::from_str(&body_string(get(&app, "/health").await).await).unwrap();
    assert_eq!(
        anon["entities"]["persons"], 2,
        "two of the four persons are public"
    );
    let admin: serde_json::Value =
        serde_json::from_str(&body_string(get_admin(&app, "/health").await).await).unwrap();
    assert_eq!(admin["entities"]["persons"], 4);
}

#[tokio::test]
async fn an_absent_record_and_a_withheld_one_are_different_answers() {
    // Collapsing both into 404 is the reflex, and it is wrong here: the tree
    // already shows that a hidden person exists, so a 404 would protect
    // nothing while telling a family member who is merely signed out that the
    // record they were sent has been deleted.
    let app = app("vis-404");
    assert_eq!(
        get(&app, "/person/00000000-0000-4000-8000-000000000000")
            .await
            .status(),
        StatusCode::NOT_FOUND
    );
    let resp = get(&app, &format!("/person/{PRIVATE}")).await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
    let body = body_string(resp).await;
    assert!(
        body.contains("Sign in"),
        "a signed-out reader is told what would change the answer"
    );
}

#[tokio::test]
async fn an_unmarked_living_person_is_not_published() {
    // The default that matters, because every converted GEDCOM hits it: AXGF
    // makes `visibility` optional, so a bundle imported from GEDCOM carries
    // none at all.
    let dir = scratch("vis-default-src");
    let path = dir.join("d.axgf");
    let living = "aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa";
    let dead = "bbbbbbbb-bbbb-4bbb-8bbb-bbbbbbbbbbbb";
    let flat = json!({
        "manifest": {"axgf": "1.0"},
        "persons": {
            living: person(living, "Zofia Nowakowska", None, true),
            dead: person(dead, "Ignacy Nowakowski", None, false)
        },
        "families": {}, "events": {}, "links": {}, "occupations": {},
        "sources": {}, "places": {}, "documents": {}
    });
    std::fs::write(
        &path,
        axgf_cms::state::export_to_bytes(&flat.to_string()).expect("export"),
    )
    .expect("write");
    let (app, _p) = app_with_bundle("vis-default", &path);

    let tree = body_string(get(&app, "/tree?all=1").await).await;
    assert!(
        !tree.contains("Zofia"),
        "a living person with no stated visibility is not published"
    );
    assert!(
        tree.contains("Ignacy"),
        "publishing the dead is what genealogy is for"
    );
}

#[tokio::test]
async fn tightening_a_visibility_takes_effect_on_the_very_next_read() {
    // The resolved visible-person set is memoised per bundle version, because
    // rebuilding it on every request cost a quarter of the render budget. A
    // memoised permission check is exactly the kind of optimisation that turns
    // into a vulnerability when its invalidation is wrong: a stale set would
    // keep publishing a record for as long as the process lived after somebody
    // marked it private. So the invalidation is pinned here, not assumed.
    let app = app("vis-invalidate");

    // The public person is visible to a signed-out reader.
    assert_eq!(
        get(&app, &format!("/person/{OPEN}")).await.status(),
        StatusCode::OK
    );

    // Save the same person back with `visibility` flipped to private, whole,
    // through the raw-JSON editor the admin forms carry.
    let entity = json!({
        "id": OPEN, "type": "person", "axgf_version": "1.0",
        "identity": {
            "name": {"display": OPEN_NAME, "components": [
                {"type": "given_name", "value": "Kazimierz", "order": 1}]},
            "gender": {"value": "F"}, "is_living": false,
            "visibility": "private"
        }
    });
    // `base_version` is what the edit form carries; a save fails closed
    // without it. The fixture's people carry no `version_num`, so 0.
    let body = format!(
        "base_version=0&identity.name.display={}&raw_json={}",
        urlencode(OPEN_NAME),
        urlencode(&serde_json::to_string(&entity).unwrap())
    );
    let resp = post_form(&app, &format!("/admin/person/{OPEN}"), &body, true).await;
    let out = expect_status(resp, StatusCode::OK, "tighten visibility").await;
    assert!(out.contains("Updated") || out.contains("Saved"), "{out}");

    // The very next signed-out read must already be refused.
    assert_eq!(
        get(&app, &format!("/person/{OPEN}")).await.status(),
        StatusCode::FORBIDDEN,
        "a memoised visible set must not outlive the bundle it was built from"
    );
    let tree = body_string(get(&app, "/tree?all=1").await).await;
    assert!(
        !tree.contains(OPEN_NAME),
        "…on every surface, not just the one that was asked first"
    );
}

/// Minimal form encoding for the raw-JSON field.
fn urlencode(s: &str) -> String {
    let mut out = String::with_capacity(s.len() * 3);
    for b in s.as_bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(*b as char)
            }
            b' ' => out.push('+'),
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}
