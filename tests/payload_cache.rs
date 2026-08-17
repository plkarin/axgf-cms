//! The binary-payload disk cache (Change 4): payloads leave the in-memory
//! bundle at load time and are streamed from disk, and the `.axgf` still
//! round-trips byte-for-byte.

mod common;

use std::path::{Path, PathBuf};

use axum::http::StatusCode;
use base64::Engine as _;
use common::*;
use serde_json::json;

/// A small real PNG, and the flat bundle that carries it as an attachment.
fn png_bytes() -> Vec<u8> {
    let img = image::RgbImage::from_pixel(8, 6, image::Rgb([12, 200, 90]));
    let mut out = std::io::Cursor::new(Vec::new());
    image::DynamicImage::ImageRgb8(img)
        .write_to(&mut out, image::ImageFormat::Png)
        .expect("encode png");
    out.into_inner()
}

/// Write a `.axgf` carrying one present document and its payload. Returns the
/// path, the payload bytes, the document id and its ZIP path.
fn bundle_with_image(tag: &str) -> (PathBuf, Vec<u8>, String, String) {
    let dir = scratch(tag);
    let path = dir.join("family.axgf");
    let png = png_bytes();
    let sha = axgf_cms::documents::sha256_hex(&png);
    let doc_id = "11111111-1111-4111-8111-111111111111".to_string();
    let zip_path = format!("documents/files/{doc_id}.png");
    let b64 = base64::engine::general_purpose::STANDARD.encode(&png);

    let flat = json!({
        "manifest": {"axgf": "1.0", "created_at": "2026-01-01T00:00:00Z",
                     "updated_at": "2026-01-01T00:00:00Z"},
        "persons": {}, "families": {}, "events": {}, "links": {},
        "occupations": {}, "sources": {}, "places": {},
        "documents": {
            doc_id.clone(): {
                "id": doc_id, "type": "document", "axgf_version": "1.0",
                "filename": "portrait.png", "mime_type": "image/png",
                "document_type": "photo", "status": "present",
                "file": {"path": zip_path, "size_bytes": png.len(), "sha256": sha}
            }
        },
        "attachments": { zip_path.clone(): b64 }
    });

    let bytes = axgf_cms::state::export_to_bytes(&flat.to_string()).expect("export bundle");
    std::fs::write(&path, bytes).expect("write bundle");
    (path, png, doc_id, zip_path)
}

/// Load the bundle with an explicit cache directory under the same scratch dir.
fn load(path: &Path, cache: &Path) -> (axgf_cms::AppState, axgf_cms::payloads::PopulateReport) {
    axgf_cms::AppState::load(path, "t".into(), None, Some(cache)).expect("load")
}

#[test]
fn payloads_are_absent_from_the_in_memory_bundle_after_load() {
    let (path, _png, _id, _zip) = bundle_with_image("pc-absent");
    let cache = path.parent().unwrap().join("cache");
    let (state, report) = load(&path, &cache);

    assert_eq!(report.extracted, 1, "the one payload was extracted to disk");
    // The resident bundle keeps the metadata but not the bytes.
    state.read(|flat| {
        assert!(
            flat.get("attachments").is_none(),
            "the attachments map must be gone from memory"
        );
        assert!(
            flat.get("documents")
                .and_then(|d| d.as_object())
                .map(|m| m.len())
                == Some(1),
            "document metadata stays resident"
        );
    });
}

#[tokio::test]
async fn a_served_document_matches_the_bytes_in_the_original_bundle() {
    let (path, png, id, _zip) = bundle_with_image("pc-serve");
    let app = axgf_cms::app(&path, TOKEN).expect("app");

    let resp = get(&app, &format!("/document/{id}/raw")).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let served = body_bytes(resp).await;
    assert_eq!(
        served, png,
        "the streamed bytes must equal the stored payload"
    );
}

#[test]
fn a_second_startup_reuses_the_cache_without_re_extracting() {
    let (path, _png, _id, _zip) = bundle_with_image("pc-reuse");
    let cache = path.parent().unwrap().join("cache");

    let (_s1, first) = load(&path, &cache);
    assert_eq!(first.extracted, 1);
    assert_eq!(first.reused, 0);

    // A restart on the unchanged bundle must not rewrite the payload.
    let (_s2, second) = load(&path, &cache);
    assert_eq!(second.extracted, 0, "nothing re-extracted on a warm start");
    assert_eq!(second.reused, 1, "the cached payload was reused");
}

#[test]
fn a_corrupted_cache_file_is_detected_by_sha256() {
    let (path, _png, _id, _zip) = bundle_with_image("pc-corrupt");
    let cache = path.parent().unwrap().join("cache");

    let (_s1, first) = load(&path, &cache);
    let cache_dir = first.cache_dir.clone();

    // Corrupt the one cached payload (leaving the index.json alone).
    let payload_file = std::fs::read_dir(&cache_dir)
        .expect("read cache dir")
        .filter_map(Result::ok)
        .map(|e| e.path())
        .find(|p| p.file_name().and_then(|n| n.to_str()) != Some("index.json"))
        .expect("a cached payload file");
    std::fs::write(&payload_file, b"corrupted not-a-png").expect("corrupt");

    // A fresh load must notice the mismatch and re-extract from the bundle.
    let (_s2, second) = load(&path, &cache);
    assert!(
        second.mismatches >= 1,
        "a corrupted cache file must be detected, got report {second:?}"
    );
    assert!(
        second.extracted >= 1,
        "the good bytes must be recovered from the bundle"
    );
}

#[test]
fn export_after_a_load_reproduces_the_bundle_including_payloads() {
    let (path, _png, _id, zip) = bundle_with_image("pc-roundtrip");
    let original = std::fs::read(&path).expect("read original");
    let cache = path.parent().unwrap().join("cache");

    let (state, _report) = load(&path, &cache);
    // Payloads are folded back in from the cache just for this export.
    let exported = state.export_bytes().expect("export");

    // `axgf-rs`'s export_bundle stamps `manifest.updated_at` with the current
    // instant on every call, so two exports are never byte-identical — a
    // library behaviour, not a payload one. Everything else, the media
    // payloads included, must round-trip exactly. Comparing the re-imported
    // flat bundles (which carry the attachments back as base64) proves the
    // payload survived, and normalising the one volatile field isolates the
    // library's timestamp from the comparison.
    let reimport = |bytes: &[u8]| {
        let env = axgf_rs::import_bundle(bytes);
        axgf_cms::state::envelope_into_data(env).expect("reimport")
    };
    let mut a = reimport(&original);
    let mut b = reimport(&exported);
    for v in [&mut a, &mut b] {
        if let Some(m) = v.get_mut("manifest").and_then(|m| m.as_object_mut()) {
            m.remove("updated_at");
        }
    }
    assert_eq!(
        a, b,
        "load then export must reproduce every entry (payloads included) apart \
         from the manifest timestamp the library rewrites"
    );
    // And the payload bytes themselves are identical, not merely present.
    assert_eq!(
        a.get("attachments").and_then(|x| x.get(&zip)),
        b.get("attachments").and_then(|x| x.get(&zip)),
        "the media payload must be byte-identical across the round-trip"
    );
}
