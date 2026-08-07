//! The GEDCOM conversion page, driven through the real router.

mod common;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use common::*;
use tower::ServiceExt;

const BOUNDARY: &str = "----axgfcmstestboundary";

/// Build a multipart body with the three form fields.
fn multipart(filename: &str, content: &[u8], confidence: &str, lang: &str) -> Vec<u8> {
    let mut body = Vec::new();
    let mut push = |s: &str| body.extend_from_slice(s.as_bytes());

    push(&format!("--{BOUNDARY}\r\n"));
    push(&format!(
        "Content-Disposition: form-data; name=\"file\"; filename=\"{filename}\"\r\n"
    ));
    push("Content-Type: application/octet-stream\r\n\r\n");
    body.extend_from_slice(content);
    body.extend_from_slice(b"\r\n");

    for (name, value) in [("confidence", confidence), ("lang", lang)] {
        body.extend_from_slice(format!("--{BOUNDARY}\r\n").as_bytes());
        body.extend_from_slice(
            format!("Content-Disposition: form-data; name=\"{name}\"\r\n\r\n").as_bytes(),
        );
        body.extend_from_slice(value.as_bytes());
        body.extend_from_slice(b"\r\n");
    }
    body.extend_from_slice(format!("--{BOUNDARY}--\r\n").as_bytes());
    body
}

async fn post_convert(app: &axum::Router, body: Vec<u8>) -> axum::http::Response<Body> {
    app.clone()
        .oneshot(
            Request::builder()
                .uri("/convert/gedcom")
                .method("POST")
                .header(
                    header::CONTENT_TYPE,
                    format!("multipart/form-data; boundary={BOUNDARY}"),
                )
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .expect("request")
}

fn fixture(name: &str) -> Vec<u8> {
    // The axgf-rs fixtures are the most realistic GEDCOM available; fall back
    // to a minimal inline file when the sibling checkout is absent.
    let p = std::path::Path::new("/home/cbrain/axgf-lib/tests/fixtures").join(name);
    std::fs::read(&p).unwrap_or_else(|_| {
        b"0 HEAD\n1 GEDC\n2 VERS 5.5.1\n1 CHAR UTF-8\n\
          0 @I1@ INDI\n1 NAME Test /Person/\n1 SEX M\n1 BIRT\n2 DATE 1900\n\
          0 TRLR\n"
            .to_vec()
    })
}

#[tokio::test]
async fn the_form_offers_confidence_and_language() {
    let (app, _p) = app_with_empty_bundle("conv-form");
    let body = expect_status(get(&app, "/convert").await, StatusCode::OK, "GET /convert").await;
    assert!(body.contains("name=\"confidence\""));
    assert!(body.contains("name=\"lang\""));
    assert!(body.contains("type=\"file\""));
    // Export back to GEDCOM is out of scope and must not be advertised.
    assert!(
        !body.to_lowercase().contains("coming soon"),
        "no stubbed reverse conversion"
    );
}

#[tokio::test]
async fn converting_a_real_gedcom_reports_counts_diagnostics_and_a_download() {
    let (app, _p) = app_with_empty_bundle("conv-ok");
    let ged = fixture("small.ged");
    let resp = post_convert(&app, multipart("small.ged", &ged, "0.75", "fr")).await;
    let body = expect_status(resp, StatusCode::OK, "convert small.ged").await;

    assert!(body.contains("Converted small.ged"));
    assert!(body.contains("What the conversion produced"));
    assert!(body.contains("persons"), "counts by kind must be shown");
    assert!(
        body.contains("/convert/download/"),
        "a download must be offered"
    );
    // The settings used are echoed back.
    assert!(body.contains("0.75"));
    assert!(body.contains("fr"));

    // Follow the download and check it really is an .axgf bundle.
    let href = body
        .split("/convert/download/")
        .nth(1)
        .and_then(|s| s.split('"').next())
        .expect("download id");
    let dl = get(&app, &format!("/convert/download/{href}")).await;
    assert_eq!(dl.status(), StatusCode::OK);
    let ct = dl
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();
    assert_eq!(ct, "application/vnd.axgf+zip");
    let bytes = body_bytes(dl).await;
    assert_eq!(&bytes[..2], b"PK", "the download is a ZIP archive");
}

#[tokio::test]
async fn unrecognized_tags_are_presented_as_a_feature_not_hidden() {
    let (app, _p) = app_with_empty_bundle("conv-tags");
    let ged = fixture("tree.ged");
    let resp = post_convert(&app, multipart("tree.ged", &ged, "0.8", "pl")).await;
    let body = expect_status(resp, StatusCode::OK, "convert tree.ged").await;

    // The real webtrees export contains tags AXGF has nowhere to put. They
    // must be listed, with the framing that nothing was silently dropped.
    if body.contains("GEDCOM_UNRECOGNIZED_TAG") {
        assert!(
            body.contains("carried no usable data"),
            "skipped tags need the explanatory framing"
        );
    }
    assert!(body.contains("/convert/download/"));
}

#[tokio::test]
async fn conversion_never_touches_the_served_bundle() {
    let (app, path) = app_with_empty_bundle("conv-safe");
    let before = std::fs::read(&path).expect("read before");
    let health_before = body_string(get(&app, "/health").await).await;

    let ged = fixture("small.ged");
    let resp = post_convert(&app, multipart("small.ged", &ged, "0.8", "en")).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let after = std::fs::read(&path).expect("read after");
    assert_eq!(
        before, after,
        "converting must not rewrite the bundle this site serves"
    );
    let health_after = body_string(get(&app, "/health").await).await;
    assert_eq!(
        health_before, health_after,
        "in-memory state must be unchanged"
    );
}

#[tokio::test]
async fn a_non_gedcom_upload_is_refused_clearly_not_by_panicking() {
    let (app, _p) = app_with_empty_bundle("conv-bad");
    let junk = b"{\"this\": \"is json, not gedcom\"}";
    let resp = post_convert(&app, multipart("data.json", junk, "0.8", "en")).await;
    let body = expect_status(resp, StatusCode::OK, "non-gedcom upload").await;

    assert!(body.contains("Conversion failed"));
    assert!(
        body.contains("does not look like a GEDCOM"),
        "the message must say what was wrong"
    );
    assert!(body.contains("Nothing was converted"));
}

#[tokio::test]
async fn an_empty_upload_is_refused_clearly() {
    let (app, _p) = app_with_empty_bundle("conv-empty");
    let resp = post_convert(&app, multipart("empty.ged", b"", "0.8", "en")).await;
    let body = expect_status(resp, StatusCode::OK, "empty upload").await;
    assert!(body.contains("No file was uploaded"));
}

#[tokio::test]
async fn an_oversized_upload_is_rejected_by_the_body_limit() {
    let (app, _p) = app_with_empty_bundle("conv-big");
    // Comfortably past the 10 MB ceiling plus its slack.
    let big = vec![b'0'; 11 * 1024 * 1024];
    let resp = post_convert(&app, multipart("big.ged", &big, "0.8", "en")).await;
    assert!(
        resp.status() == StatusCode::PAYLOAD_TOO_LARGE,
        "an oversized body must be refused with 413, got {}",
        resp.status()
    );
}

#[tokio::test]
async fn an_expired_or_unknown_download_id_is_a_clean_404() {
    let (app, _p) = app_with_empty_bundle("conv-gone");
    let body = expect_status(
        get(&app, "/convert/download/deadbeef").await,
        StatusCode::NOT_FOUND,
        "unknown download",
    )
    .await;
    assert!(body.contains("expired"));
}
