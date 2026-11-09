//! Canopée's world (docs/adr/0043): built once for each shape of the family,
//! kept by the browser under its fingerprint, built again when a person is
//! added — and its words, the names the reader may read.

mod common;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use common::*;
use serde_json::{json, Value};
use tower::ServiceExt;

fn person(id: &str, name: &str, visibility: &str) -> Value {
    json!({"id": id, "type": "person", "axgf_version": "1.1",
           "identity": {"name": {"display": name, "components": []},
                        "is_living": false, "visibility": visibility}})
}

fn bundle(tag: &str) -> common::Scratch {
    let dir = scratch(tag);
    let path = dir.join("world.axgf");
    let flat = json!({
        "manifest": {"axgf": "1.1"},
        "persons": {
            "a0000000-0000-4000-8000-000000000001": person("a0000000-0000-4000-8000-000000000001", "Adam Brzoza", "public"),
            "a0000000-0000-4000-8000-000000000002": person("a0000000-0000-4000-8000-000000000002", "Ewa Lipa", "public"),
            "a0000000-0000-4000-8000-000000000003": person("a0000000-0000-4000-8000-000000000003", "Hidden Child", "private")
        },
        "families": {"f0000000-0000-4000-8000-000000000001": {
            "id": "f0000000-0000-4000-8000-000000000001", "type": "family", "axgf_version": "1.0",
            "union": {"type": "marriage", "persons": [
                {"person_id": "a0000000-0000-4000-8000-000000000001", "role": "spouse"},
                {"person_id": "a0000000-0000-4000-8000-000000000002", "role": "spouse"}]},
            "children": [{"person_id": "a0000000-0000-4000-8000-000000000003"}]}},
        "events": {}, "links": {}, "occupations": {}, "places": {}, "sources": {}, "documents": {}
    });
    std::fs::write(
        &path,
        axgf_cms::state::export_to_bytes(&flat.to_string()).expect("export"),
    )
    .expect("write");
    dir.pointing_at(path)
}

async fn world(app: &axum::Router, query: &str, etag: Option<&str>) -> axum::response::Response {
    let mut req = Request::builder().uri(format!("/canopee/world.bin{query}"));
    if let Some(e) = etag {
        req = req.header(header::IF_NONE_MATCH, e);
    }
    app.clone()
        .oneshot(req.body(Body::empty()).unwrap())
        .await
        .unwrap()
}

#[tokio::test]
async fn the_world_is_built_once_for_a_shape_and_again_for_another() {
    let src = bundle("world-src");
    let (app, _p) = app_with_bundle("world", &src);

    let resp = world(&app, "", None).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let etag = resp.headers()[header::ETAG].to_str().unwrap().to_string();
    let fp = etag.trim_matches('"').to_string();
    assert_eq!(fp.len(), 32);
    assert!(resp.headers()[header::CACHE_CONTROL]
        .to_str()
        .unwrap()
        .contains("no-cache"));
    let bytes = body_bytes(resp).await;
    assert_eq!(&bytes[..4], b"AXW2");

    // Asked for by its fingerprint: kept a year; unchanged: not sent again.
    let resp = world(&app, &format!("?v={fp}"), None).await;
    assert!(resp.headers()[header::CACHE_CONTROL]
        .to_str()
        .unwrap()
        .contains("immutable"));
    assert_eq!(
        body_bytes(resp).await,
        bytes,
        "the same world, not built again"
    );
    let resp = world(&app, &format!("?v={fp}"), Some(&etag)).await;
    assert_eq!(resp.status(), StatusCode::NOT_MODIFIED);

    // The river page names it.
    let river = body_string(get_with_cookie(&app, "/", "axgf_theme=canopee").await).await;
    assert!(
        river.contains(&format!("data-world=\"/canopee/world.bin?v={fp}\"")),
        "the page asks for this world"
    );
    assert!(river.contains("/static/canopee-world.js"));

    // A person added: another shape, another world.
    let cookie = admin_cookie(&app).await;
    let resp = post_form_as(
        &app,
        &cookie,
        "/admin/person/new",
        "given=Nowy&family=Brzoza",
    )
    .await;
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    let resp = world(&app, "", None).await;
    let etag2 = resp.headers()[header::ETAG].to_str().unwrap().to_string();
    assert_ne!(etag2, etag, "a person added is a new shape");
    assert_ne!(body_bytes(resp).await, bytes);
}

#[tokio::test]
async fn the_words_are_only_those_the_reader_may_read() {
    let src = bundle("world-words-src");
    let (app, _p) = app_with_bundle("world-words", &src);
    let words: Value =
        serde_json::from_str(&body_string(get(&app, "/canopee/words.json").await).await).unwrap();
    let people = words["people"].as_object().unwrap();
    assert_eq!(
        people["a0000000-0000-4000-8000-000000000001"][0], "Adam Brzoza",
        "a public person's name"
    );
    assert!(
        !people.contains_key("a0000000-0000-4000-8000-000000000003"),
        "a private person has no words for a visitor"
    );
    assert_eq!(words["fingerprint"].as_str().unwrap().len(), 32);
}
