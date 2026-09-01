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
    // Committed to this repository, not read out of a sibling checkout. It used
    // to be loaded from an absolute path into axgf-rs, with a one-person inline
    // GEDCOM as a silent fallback when that path was absent. On this machine the
    // path resolved and the realistic file was used; on CI it never did, so the
    // fallback ran and the assertions about plural counts failed there and only
    // there. A fixture a test depends on belongs beside the test.
    let p = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name);
    std::fs::read(&p).unwrap_or_else(|e| panic!("read fixture {}: {e}", p.display()))
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

    assert!(body.contains("Imported small.ged"));
    assert!(body.contains("What came across"));
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
    // A real webtrees export contains entries with nothing importable in them:
    // FAM stubs left behind by deletions, empty tags, records private to the
    // exporter. This fixture carries one of each. It used to be the operator's
    // own 866-person export, read from outside the repository, so on CI the
    // file was absent, the fallback GEDCOM had nothing unimportable in it, and
    // the assertions below — guarded by an `if` on the diagnostic being
    // present — asserted nothing at all. The guard is gone: the fixture is
    // chosen so the diagnostic must appear, and the test fails if it does not.
    let ged = fixture("stray-records.ged");
    let resp = post_convert(&app, multipart("stray-records.ged", &ged, "0.8", "pl")).await;
    let body = expect_status(resp, StatusCode::OK, "convert stray-records.ged").await;

    assert!(
        body.contains("GEDCOM_UNRECOGNIZED_TAG"),
        "the fixture must produce something the importer cannot read, or this \
         test proves nothing"
    );
    // They must be listed, with the framing that nothing was silently dropped
    // — an import report that hides what it could not read is worse than no
    // report.
    assert!(
        body.contains("could not be read"),
        "skipped entries need the explanatory framing"
    );
    // What is *not* there any more is the import arguing for itself — "they
    // are listed rather than swallowed: knowing exactly what was left behind
    // is the difference between an import you can trust and one you cannot".
    // The list is the argument. The sentence was the software talking about
    // the software.
    assert!(
        !body.contains("listed rather than swallowed"),
        "the report shows what was left behind; it does not praise itself for it"
    );
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

    assert!(body.contains("The import did not go through"));
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

#[tokio::test]
async fn the_result_page_reports_what_the_import_brought_over() {
    let (app, _p) = app_with_empty_bundle("conv-complete");
    let ged = fixture("small.ged");
    let resp = post_convert(&app, multipart("small.ged", &ged, "0.8", "pl")).await;
    let body = expect_status(resp, StatusCode::OK, "convert for completeness").await;

    assert!(body.contains("completeness"), "the panel must be present");
    assert!(body.contains("What the import brought over"));

    // Each of the six things the report must account for.
    for expected in [
        "How sure each fact is",
        "How sure each parent–child link is",
        "Relationships beyond blood and marriage",
        "Work recorded with a start and an end",
        "Sources graded for how reliable they are",
        "Dates, by the shape they actually have",
    ] {
        assert!(body.contains(expected), "report is missing: {expected}");
    }

    // A blank row is a fact about the file that was imported, and the page
    // says so without blaming the import or arguing about file formats.
    assert!(
        body.contains("the original file did not record"),
        "a blank row must be explained as absent from the source"
    );
    assert!(
        body.contains("not something the import lost"),
        "the framing must not imply data was dropped"
    );
    for absent in ["SPEC_1.0.md", "GEDCOM cannot express", "AXGF field"] {
        assert!(
            !body.contains(absent),
            "the import report must not argue about the format: {absent}"
        );
    }

    // The date breakdown is real: this file has all four interesting shapes.
    for shape in ["exact", "approximate", "ranged", "preserved"] {
        assert!(body.contains(shape), "date shape missing: {shape}");
    }
}

#[tokio::test]
async fn a_rich_bundle_is_not_told_its_data_is_missing() {
    // The honest framing has to cut both ways. Feeding the demonstration
    // bundle through the same analysis must produce the opposite verdict.
    let sample = std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/deploy/sample.axgf"));
    let bytes = std::fs::read(sample).expect("sample bundle");
    let env = axgf_rs::import_bundle(&bytes);
    let report = axgf_cms::completeness::analyse(&env.data, "en");

    assert_eq!(
        report.empty, 0,
        "the sample populates every kind of detail; report was: {}",
        report.headline
    );
    assert!(
        report.headline.contains("recorded somewhere"),
        "a rich bundle needs a different sentence: {}",
        report.headline
    );
    assert!(report.dates.preserved > 0, "and it keeps unparsable text");
}
