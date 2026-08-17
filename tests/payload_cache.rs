//! The binary-payload disk cache: payloads are streamed out of the archive
//! into the cache at load time and streamed back into a new archive on save,
//! never passing through the in-memory bundle in either direction — and the
//! `.axgf` still round-trips byte-for-byte.

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
fn a_corrupted_cache_file_is_detected_by_its_crc32() {
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
    // The archive's central directory carries the CRC-32 of every entry, so
    // recomputing it over the cached file proves whether it is still the bytes
    // this bundle holds — without decompressing the entry to find out.
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

#[test]
fn the_bundle_declares_its_payloads_so_a_streaming_export_asks_for_them() {
    let (path, png, _id, zip) = bundle_with_image("pc-declare");
    let cache = path.parent().unwrap().join("cache");
    let (state, _report) = load(&path, &cache);

    state.read(|flat| {
        let declared = flat
            .get("external_payloads")
            .and_then(|v| v.as_object())
            .expect("a streamed import records what it streamed out");
        let entry = declared.get(&zip).expect("this payload must be declared");
        assert_eq!(
            entry.get("size_bytes").and_then(|v| v.as_u64()),
            Some(png.len() as u64),
            "the declaration carries the size the archive recorded"
        );
    });
}

/// A streaming export builds the archive somewhere else entirely. Nothing it
/// does can reach the live bundle, which is the first half of the atomic-write
/// property; [`a_failed_streaming_export_leaves_the_previous_bundle_intact`] is
/// the second half.
#[test]
fn a_streaming_export_never_touches_the_live_bundle() {
    let (path, png, _id, zip) = bundle_with_image("pc-elsewhere");
    let cache = path.parent().unwrap().join("cache");
    let before = std::fs::read(&path).expect("read before");

    let (state, _report) = load(&path, &cache);
    let dest = path.parent().unwrap().join("copy.axgf");
    state.export_to_file(&dest).expect("streaming export");

    assert_eq!(
        std::fs::read(&path).expect("read after"),
        before,
        "exporting must not write a single byte to the bundle being served"
    );

    // And what it wrote elsewhere is a complete bundle, media included.
    let env = axgf_rs::import_bundle(&std::fs::read(&dest).expect("read copy"));
    let flat = axgf_cms::state::envelope_into_data(env).expect("the copy must import");
    let b64 = base64::engine::general_purpose::STANDARD.encode(&png);
    assert_eq!(
        flat.get("attachments")
            .and_then(|x| x.get(&zip))
            .and_then(|v| v.as_str()),
        Some(b64.as_str()),
        "the streamed archive must carry the payload byte-for-byte"
    );
}

/// The property the whole DRP story rests on: the new archive becomes the live
/// bundle only at the rename, so a failure at any earlier point leaves the
/// previous bundle exactly as it was.
///
/// The failure is injected where the streaming export can genuinely fail
/// mid-archive: the bundle is made to declare a payload that neither the cache
/// nor the `.axgf` holds, so `axgf-rs` refuses with `PAYLOAD_SOURCE_FAILED`
/// after it has already written the manifest, the entities and the real
/// payload into the temp file.
#[test]
fn a_failed_streaming_export_leaves_the_previous_bundle_intact() {
    let (path, _png, _id, _zip) = bundle_with_image("pc-atomic");
    let cache = path.parent().unwrap().join("cache");
    let (state, _report) = load(&path, &cache);

    let before = std::fs::read(&path).expect("read before");
    let person = r#"{"identity":{"name":{"display":"Ada Lovelace"}}}"#;
    let result = state.mutate_and_adjust(
        |flat| axgf_rs::add_entity(flat, axgf_rs::EntityKind::Person, person),
        |bundle, _| {
            // A file the bundle claims to carry and nothing can supply.
            bundle["external_payloads"]["documents/files/ghost.bin"] =
                json!({"size_bytes": 4096, "crc32": 1});
        },
    );

    assert!(
        result.is_err(),
        "a bundle whose media cannot be supplied must not be written"
    );
    assert_eq!(
        std::fs::read(&path).expect("read after"),
        before,
        "a failed export must leave the previous bundle byte-identical"
    );
    let mut tmp = path.file_name().unwrap().to_os_string();
    tmp.push(".tmp");
    assert!(
        !path.with_file_name(&tmp).exists(),
        "a failed export must not leave its temp file behind"
    );
    // The write happens before the in-memory swap, so a failed save must also
    // leave memory matching the file rather than stranding an unsaved edit.
    state.read(|flat| {
        assert_eq!(
            flat.get("persons")
                .and_then(|p| p.as_object())
                .map(|m| m.len()),
            Some(0),
            "a failed save must not leave the person resident"
        );
    });
}

/// A cache entry deleted behind the application's back used to be invisible:
/// the export would write a bundle with that photograph silently missing. 0.3
/// refuses instead, and the refusal is recoverable — the `.axgf` on disk is the
/// authoritative copy, so the entry is rebuilt from it and the save goes
/// through.
#[test]
fn a_cache_file_deleted_behind_our_back_is_rebuilt_from_the_bundle() {
    let (path, png, _id, zip) = bundle_with_image("pc-recover");
    let cache = path.parent().unwrap().join("cache");
    let (state, report) = load(&path, &cache);

    // Delete the cached payload, leaving the index still naming it — exactly
    // what an operator clearing disk space would produce.
    let payload_file = std::fs::read_dir(&report.cache_dir)
        .expect("read cache dir")
        .filter_map(Result::ok)
        .map(|e| e.path())
        .find(|p| p.file_name().and_then(|n| n.to_str()) != Some("index.json"))
        .expect("a cached payload file");
    std::fs::remove_file(&payload_file).expect("delete the cached payload");

    let person = r#"{"identity":{"name":{"display":"Ada Lovelace"}}}"#;
    let out = state
        .mutate(|flat| axgf_rs::add_entity(flat, axgf_rs::EntityKind::Person, person))
        .expect("the save must recover rather than fail permanently");
    assert!(out.applied, "diagnostics: {:?}", out.diagnostics);

    // The rebuilt entry is the real thing, so the written bundle still carries
    // the photograph.
    let env = axgf_rs::import_bundle(&std::fs::read(&path).expect("read saved"));
    let flat = axgf_cms::state::envelope_into_data(env).expect("the saved bundle must import");
    let b64 = base64::engine::general_purpose::STANDARD.encode(&png);
    assert_eq!(
        flat.get("attachments")
            .and_then(|x| x.get(&zip))
            .and_then(|v| v.as_str()),
        Some(b64.as_str()),
        "the recovered payload must be the original bytes, not a hole"
    );
    assert!(
        payload_file.exists(),
        "the cache entry must be back on disk"
    );
}
