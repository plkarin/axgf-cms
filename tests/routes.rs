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
