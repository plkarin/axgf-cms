//! Route-level integration tests, driving the real router.

mod common;

use axum::http::StatusCode;
use common::*;

#[tokio::test]
async fn home_renders_and_pitches_the_format() {
    let (app, _p) = app_with_empty_bundle("home");
    let body = expect_status(get(&app, "/").await, StatusCode::OK, "GET /").await;

    assert!(body.contains("Why AXGF"), "home must carry the pitch panel");
    assert!(
        body.contains("github.com/plkarin/axgf-spec"),
        "the pitch must link to the specification"
    );
    assert!(
        body.contains("confidence"),
        "the pitch must mention confidence"
    );
}

#[tokio::test]
async fn health_reports_entity_counts() {
    let (app, _p) = app_with_empty_bundle("health");
    let body = expect_status(get(&app, "/health").await, StatusCode::OK, "GET /health").await;

    let v: serde_json::Value = serde_json::from_str(&body).expect("health returns JSON");
    assert_eq!(v["status"], "ok");
    assert_eq!(v["total_entities"], 0);
    // Every collection is reported, including the empty ones.
    for k in [
        "persons",
        "families",
        "events",
        "links",
        "occupations",
        "sources",
        "places",
        "documents",
    ] {
        assert!(v["entities"][k].is_number(), "health omitted {k}");
    }
}

#[tokio::test]
async fn stylesheet_is_served_from_the_binary() {
    let (app, _p) = app_with_empty_bundle("css");
    let resp = get(&app, "/static/app.css").await;
    assert_eq!(resp.status(), StatusCode::OK);
    let ct = resp
        .headers()
        .get(axum::http::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();
    assert!(ct.starts_with("text/css"), "content-type was {ct}");
    let body = body_string(resp).await;
    assert!(
        body.contains(".conf-bar"),
        "the confidence styles must ship"
    );
}

#[tokio::test]
async fn unknown_paths_render_a_404_page_not_a_panic() {
    let (app, _p) = app_with_empty_bundle("404");
    let body = expect_status(
        get(&app, "/no/such/page").await,
        StatusCode::NOT_FOUND,
        "unknown path",
    )
    .await;
    assert!(body.contains("Not found"));
}

#[tokio::test]
async fn tree_renders_for_an_empty_bundle() {
    let (app, _p) = app_with_empty_bundle("tree-empty");
    let body = expect_status(get(&app, "/tree").await, StatusCode::OK, "GET /tree").await;
    assert!(
        body.contains("no people yet"),
        "an empty tree should say so, not render a blank page"
    );
}

#[tokio::test]
async fn tree_shows_confidence_as_line_opacity_and_a_legend() {
    // Two children of one parent with very different parentage confidence.
    let (app, path) = app_with_empty_bundle("tree-conf");
    drop(app);

    let flat = serde_json::json!({
        "manifest": {"axgf": "1.0"},
        "persons": {
            "aaaaaaaa-0000-4000-8000-000000000000": {
                "id": "aaaaaaaa-0000-4000-8000-000000000000", "type": "person",
                "axgf_version": "1.0", "identity": {"name": {"display": "Parent"}}},
            "bbbbbbbb-0000-4000-8000-000000000000": {
                "id": "bbbbbbbb-0000-4000-8000-000000000000", "type": "person",
                "axgf_version": "1.0", "identity": {"name": {"display": "Sure Child"}}},
            "cccccccc-0000-4000-8000-000000000000": {
                "id": "cccccccc-0000-4000-8000-000000000000", "type": "person",
                "axgf_version": "1.0", "identity": {"name": {"display": "Doubtful Child"}}}
        },
        "families": {
            "dddddddd-0000-4000-8000-000000000000": {
                "id": "dddddddd-0000-4000-8000-000000000000", "type": "family",
                "axgf_version": "1.0",
                "union": {"type": "marriage", "persons": [
                    {"person_id": "aaaaaaaa-0000-4000-8000-000000000000", "role": "spouse"}]},
                "children": [
                    {"person_id": "bbbbbbbb-0000-4000-8000-000000000000", "confidence": 0.99},
                    {"person_id": "cccccccc-0000-4000-8000-000000000000", "confidence": 0.35}]
            }
        },
        "events": {}, "links": {}, "occupations": {},
        "sources": {}, "places": {}, "documents": {}
    });
    let bytes = axgf_cms::state::export_to_bytes(&flat.to_string()).expect("export");
    std::fs::write(&path, bytes).expect("write");

    let app = axgf_cms::app(&path, TOKEN).expect("rebuild app");
    let body = expect_status(get(&app, "/tree").await, StatusCode::OK, "GET /tree").await;

    // The two parentage lines must carry visibly different opacity.
    let ops: Vec<f64> = body
        .match_indices("opacity:")
        .filter_map(|(i, _)| {
            body[i + 8..]
                .split(['"', ';'])
                .next()
                .and_then(|s| s.parse::<f64>().ok())
        })
        .collect();
    assert!(ops.len() >= 2, "expected parentage lines, found {ops:?}");
    let (lo, hi) = (
        ops.iter().cloned().fold(f64::MAX, f64::min),
        ops.iter().cloned().fold(0.0, f64::max),
    );
    assert!(
        hi - lo > 0.3,
        "a 0.35 parentage must look clearly fainter than a 0.99 one ({lo} vs {hi})"
    );
    assert!(lo > 0.0, "a speculative link is faint, never invisible");
    assert!(
        body.contains("speculative"),
        "the legend explains the scale"
    );
}

#[tokio::test]
async fn an_empty_bundle_is_created_on_first_start() {
    let (_app, path) = app_with_empty_bundle("create");
    assert!(
        path.exists(),
        "startup must write a bundle when none exists"
    );
    let bytes = std::fs::read(&path).expect("read bundle");
    assert!(!bytes.is_empty());
    // A .axgf bundle is a ZIP archive; check the magic rather than trusting
    // the extension.
    assert_eq!(&bytes[..2], b"PK", "bundle should be a ZIP archive");
}

#[tokio::test]
async fn binary_attachments_survive_a_mutation() {
    // A bundle's document payloads are not entities; they ride along in the
    // flat JSON's `attachments` map. Every write re-exports the whole bundle,
    // so a mutation must not quietly drop them.
    let (app, path) = app_with_empty_bundle("attach");
    drop(app);

    let flat = serde_json::json!({
        "manifest": {"axgf": "1.0"},
        "persons": {}, "families": {}, "events": {}, "links": {},
        "occupations": {}, "sources": {}, "places": {},
        "documents": {
            "dddddddd-0000-4000-8000-000000000000": {
                "id": "dddddddd-0000-4000-8000-000000000000", "type": "document",
                "axgf_version": "1.0", "filename": "scan.bin",
                "mime_type": "application/octet-stream",
                "document_type": "other", "status": "present",
                "file": {"path": "documents/files/scan.bin"}}
        },
        // "hello payload" in base64.
        "attachments": {"documents/files/scan.bin": "aGVsbG8gcGF5bG9hZA=="}
    });
    let bytes = axgf_cms::state::export_to_bytes(&flat.to_string()).expect("export");
    std::fs::write(&path, bytes).expect("write");

    let app = axgf_cms::app(&path, TOKEN).expect("reopen");

    // Any mutation rewrites the whole file.
    let resp = post_form(
        &app,
        "/admin/person",
        "identity.name.display=Someone&raw_json=",
        true,
    )
    .await;
    assert_eq!(resp.status(), StatusCode::OK);

    // The payload must still be in the file afterwards.
    let written = std::fs::read(&path).expect("read back");
    let env = axgf_rs::import_bundle(&written);
    let payload = env.data["attachments"]["documents/files/scan.bin"].as_str();
    assert_eq!(
        payload,
        Some("aGVsbG8gcGF5bG9hZA=="),
        "a mutation must not drop document payloads"
    );
}
