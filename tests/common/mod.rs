//! Shared test harness: an app instance over a throwaway bundle.
//!
//! `axgf-cms` is a binary crate, so integration tests drive it the way a
//! browser would — through the router, with axum's `oneshot`.

#![allow(dead_code)]

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use axum::body::Body;
use axum::http::{header, Request, Response, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

pub const TOKEN: &str = "test-token-abc123";

static COUNTER: AtomicUsize = AtomicUsize::new(0);

/// A unique scratch directory for one test.
pub fn scratch(tag: &str) -> PathBuf {
    let base = option_env!("CARGO_TARGET_TMPDIR")
        .map(PathBuf::from)
        .unwrap_or_else(std::env::temp_dir);
    let n = COUNTER.fetch_add(1, Ordering::SeqCst);
    let dir = base.join(format!("axgf-cms-it-{}-{}-{}", std::process::id(), tag, n));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("create scratch dir");
    dir
}

/// Build an app over a fresh empty bundle, returning the router and the path.
pub fn app_with_empty_bundle(tag: &str) -> (axum::Router, PathBuf) {
    let dir = scratch(tag);
    let path = dir.join("family.axgf");
    let app = axgf_cms::app(&path, TOKEN).expect("build app");
    (app, path)
}

/// Build an app over a copy of an existing bundle file.
pub fn app_with_bundle(tag: &str, source: &Path) -> (axum::Router, PathBuf) {
    let dir = scratch(tag);
    let path = dir.join("family.axgf");
    std::fs::copy(source, &path).expect("copy source bundle");
    let app = axgf_cms::app(&path, TOKEN).expect("build app");
    (app, path)
}

/// Issue a GET and return the response.
pub async fn get(app: &axum::Router, uri: &str) -> Response<Body> {
    app.clone()
        .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
        .await
        .expect("request")
}

/// Issue a GET carrying the admin cookie.
pub async fn get_admin(app: &axum::Router, uri: &str) -> Response<Body> {
    app.clone()
        .oneshot(
            Request::builder()
                .uri(uri)
                .header(header::COOKIE, format!("axgf_admin={TOKEN}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("request")
}

/// Issue a form POST, optionally authenticated.
pub async fn post_form(app: &axum::Router, uri: &str, body: &str, admin: bool) -> Response<Body> {
    let mut b = Request::builder()
        .uri(uri)
        .method("POST")
        .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded");
    if admin {
        b = b.header(header::COOKIE, format!("axgf_admin={TOKEN}"));
    }
    app.clone()
        .oneshot(b.body(Body::from(body.to_string())).unwrap())
        .await
        .expect("request")
}

/// Collect a response body into a string.
pub async fn body_string(resp: Response<Body>) -> String {
    let bytes = resp.into_body().collect().await.expect("body").to_bytes();
    String::from_utf8_lossy(&bytes).into_owned()
}

/// Collect a response body into bytes.
pub async fn body_bytes(resp: Response<Body>) -> Vec<u8> {
    resp.into_body()
        .collect()
        .await
        .expect("body")
        .to_bytes()
        .to_vec()
}

/// Assert a status, printing the body when it does not match.
pub async fn expect_status(resp: Response<Body>, want: StatusCode, what: &str) -> String {
    let got = resp.status();
    let body = body_string(resp).await;
    assert_eq!(
        got,
        want,
        "{what}: expected {want}, got {got}\n{}",
        body.chars().take(600).collect::<String>()
    );
    body
}
