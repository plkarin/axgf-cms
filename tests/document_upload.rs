//! Uploading files into the bundle, and getting them back out.
//!
//! The point of these tests is the round trip: an upload is only real if the
//! bytes survive `export_bundle` writing the `.axgf` and `import_bundle`
//! reading it back, because that is what happens between one request and the
//! next restart.

mod common;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use common::*;
use serde_json::json;
use tower::ServiceExt;

const PERSON: &str = "11111111-1111-4111-8111-111111111111";

/// A bundle with one person and nothing else.
fn one_person_bundle(tag: &str) -> std::path::PathBuf {
    let dir = scratch(tag);
    let path = dir.join("upload.axgf");
    let flat = json!({
        "manifest": {"axgf": "1.0"},
        "persons": {
            PERSON: {
                "id": PERSON, "type": "person", "axgf_version": "1.0",
                "identity": {"name": {"display": "Jules Meunier", "components": []},
                             "gender": {"value": "M"}, "is_living": false}}
        },
        "families": {}, "links": {}, "occupations": {}, "sources": {},
        "places": {}, "events": {}, "documents": {}
    });
    let bytes = axgf_cms::state::export_to_bytes(&flat.to_string()).expect("export");
    std::fs::write(&path, bytes).expect("write");
    path
}

/// A real PNG, so the image decoder has something genuine to work on.
fn png(w: u32, h: u32) -> Vec<u8> {
    let img = image::RgbImage::from_fn(w, h, |x, y| {
        image::Rgb([(x % 256) as u8, (y % 256) as u8, 128])
    });
    let mut out = std::io::Cursor::new(Vec::new());
    image::DynamicImage::ImageRgb8(img)
        .write_to(&mut out, image::ImageFormat::Png)
        .expect("encode");
    out.into_inner()
}

/// Build a multipart body with one file field and optional text fields.
fn multipart_body(
    filename: &str,
    content_type: &str,
    bytes: &[u8],
    caption: &str,
) -> (String, Vec<u8>) {
    let boundary = "----axgfcmstestboundary";
    let mut body: Vec<u8> = Vec::new();
    body.extend_from_slice(
        format!(
            "--{boundary}\r\nContent-Disposition: form-data; name=\"file\"; \
             filename=\"{filename}\"\r\nContent-Type: {content_type}\r\n\r\n"
        )
        .as_bytes(),
    );
    body.extend_from_slice(bytes);
    body.extend_from_slice(
        format!(
            "\r\n--{boundary}\r\nContent-Disposition: form-data; \
             name=\"caption\"\r\n\r\n{caption}\r\n--{boundary}--\r\n"
        )
        .as_bytes(),
    );
    (format!("multipart/form-data; boundary={boundary}"), body)
}

async fn upload(
    app: &axum::Router,
    person: &str,
    filename: &str,
    content_type: &str,
    bytes: &[u8],
    admin: bool,
) -> axum::http::Response<Body> {
    let (ct, body) = multipart_body(filename, content_type, bytes, "A caption");
    let mut b = Request::builder()
        .uri(format!("/admin/person/{person}/document"))
        .method("POST")
        .header(header::CONTENT_TYPE, ct);
    if admin {
        b = b.header(header::COOKIE, format!("axgf_admin={TOKEN}"));
    }
    app.clone()
        .oneshot(b.body(Body::from(body)).unwrap())
        .await
        .expect("request")
}

/// Pull the one document id out of a bundle on disk.
fn document_ids(path: &std::path::Path) -> Vec<String> {
    let bytes = std::fs::read(path).expect("read bundle");
    let env = axgf_rs::import_bundle(&bytes);
    env.data
        .get("documents")
        .and_then(|d| d.as_object())
        .map(|m| m.keys().cloned().collect())
        .unwrap_or_default()
}

#[tokio::test]
async fn an_upload_round_trips_through_export_and_reimport_with_the_bytes_intact() {
    let src = one_person_bundle("upload-rt-src");
    let (app, path) = app_with_bundle("upload-rt", &src);
    let original = png(120, 80);

    let resp = upload(&app, PERSON, "holiday.png", "image/png", &original, true).await;
    assert_eq!(
        resp.status(),
        StatusCode::SEE_OTHER,
        "a stored upload redirects back to the person"
    );

    // The bundle on disk must now carry the file. Reading it with a fresh
    // import is the only proof that matters: it is what a restart does.
    let ids = document_ids(&path);
    assert_eq!(ids.len(), 1, "one document entity was created");
    let doc_id = &ids[0];

    let reloaded = axgf_cms::app(&path, TOKEN).expect("reopen the written bundle");
    let resp = get(&reloaded, &format!("/document/{doc_id}/raw")).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let served = body_bytes(resp).await;
    assert_eq!(
        served, original,
        "the bytes must survive export, the ZIP, and re-import unchanged"
    );

    // And the metadata the page needs came back with them.
    let bundle = axgf_rs::import_bundle(&std::fs::read(&path).unwrap()).data;
    let doc = &bundle["documents"][doc_id];
    assert_eq!(doc["mime_type"], "image/png");
    assert_eq!(doc["status"], "present");
    assert_eq!(doc["filename"], "holiday.png");
    assert_eq!(doc["caption"], "A caption");
    assert_eq!(doc["file"]["size_bytes"], original.len());
    assert_eq!(
        doc["file"]["sha256"],
        axgf_cms::documents::sha256_hex(&original)
    );
    assert_eq!(
        doc["file"]["path"],
        format!("documents/files/{doc_id}.png"),
        "stored where import_bundle looks for attachments"
    );
    assert_eq!(doc["linked_to"][0]["entity_id"], PERSON);
}

#[tokio::test]
async fn magic_byte_detection_rejects_a_renamed_executable() {
    let src = one_person_bundle("upload-elf-src");
    let (app, path) = app_with_bundle("upload-elf", &src);

    // The name says .jpg and the browser says image/jpeg. Both are lying.
    let elf = b"\x7fELF\x02\x01\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x02\x00\x3e\x00";
    let resp = upload(&app, PERSON, "portrait.jpg", "image/jpeg", elf, true).await;
    assert_eq!(resp.status(), StatusCode::UNSUPPORTED_MEDIA_TYPE);
    let body = body_string(resp).await;
    assert!(
        body.contains("read from the file"),
        "the refusal should say why: {body}"
    );

    assert!(
        document_ids(&path).is_empty(),
        "a refused upload must leave no document behind"
    );
}

#[tokio::test]
async fn an_over_limit_upload_is_refused_cleanly() {
    let src = one_person_bundle("upload-big-src");
    let (app, path) = app_with_bundle("upload-big", &src);
    let before = std::fs::read(&path).expect("read before");

    // A valid PNG header followed by more than the limit allows, so the
    // refusal is about size and nothing else.
    let mut huge = b"\x89PNG\r\n\x1a\n".to_vec();
    huge.resize(axgf_cms::documents::MAX_UPLOAD + 4096, 0x41);

    let resp = upload(&app, PERSON, "huge.png", "image/png", &huge, true).await;
    assert_eq!(resp.status(), StatusCode::PAYLOAD_TOO_LARGE);
    let body = body_string(resp).await;
    assert!(
        body.contains("limit"),
        "the refusal names the limit: {body}"
    );

    assert!(document_ids(&path).is_empty());
    assert_eq!(
        std::fs::read(&path).expect("read after"),
        before,
        "a refused upload must not touch the bundle at all"
    );
}

#[tokio::test]
async fn an_anonymous_visitor_cannot_upload() {
    let src = one_person_bundle("upload-anon-src");
    let (app, path) = app_with_bundle("upload-anon", &src);
    let resp = upload(&app, PERSON, "x.png", "image/png", &png(8, 8), false).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    assert!(document_ids(&path).is_empty());
}

#[tokio::test]
async fn uploading_to_a_person_who_does_not_exist_is_a_404() {
    let src = one_person_bundle("upload-nobody-src");
    let (app, path) = app_with_bundle("upload-nobody", &src);
    let resp = upload(
        &app,
        "99999999-9999-4999-8999-999999999999",
        "x.png",
        "image/png",
        &png(8, 8),
        true,
    )
    .await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    assert!(document_ids(&path).is_empty());
}

#[tokio::test]
async fn an_image_is_served_inline_and_a_document_as_an_attachment() {
    let src = one_person_bundle("upload-disp-src");
    let (app, path) = app_with_bundle("upload-disp", &src);

    upload(&app, PERSON, "photo.png", "image/png", &png(40, 40), true).await;
    let mut pdf = b"%PDF-1.7\n".to_vec();
    pdf.extend_from_slice(b"1 0 obj\n<<>>\nendobj\ntrailer\n%%EOF\n");
    upload(&app, PERSON, "record.pdf", "application/pdf", &pdf, true).await;

    let bundle = axgf_rs::import_bundle(&std::fs::read(&path).unwrap()).data;
    let docs = bundle["documents"].as_object().expect("documents");
    assert_eq!(docs.len(), 2);

    for (id, doc) in docs {
        let resp = get(&app, &format!("/document/{id}/raw")).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let headers = resp.headers().clone();
        // Every response says "do not guess the type", whatever it is.
        assert_eq!(
            headers.get("x-content-type-options").unwrap(),
            "nosniff",
            "a stored file must never be content-sniffed by the browser"
        );
        let disposition = headers
            .get(header::CONTENT_DISPOSITION)
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        if doc["mime_type"] == "image/png" {
            assert!(
                disposition.starts_with("inline"),
                "a raster image is shown in the page: {disposition}"
            );
        } else {
            assert!(
                disposition.starts_with("attachment"),
                "anything that could carry script downloads instead: {disposition}"
            );
        }
    }
}

#[tokio::test]
async fn a_thumbnail_is_produced_for_an_image_and_refused_for_anything_else() {
    let src = one_person_bundle("upload-thumb-src");
    let (app, path) = app_with_bundle("upload-thumb", &src);

    upload(&app, PERSON, "big.png", "image/png", &png(1200, 600), true).await;
    upload(
        &app,
        PERSON,
        "notes.txt",
        "text/plain",
        b"Just some notes about the family.\n",
        true,
    )
    .await;

    let bundle = axgf_rs::import_bundle(&std::fs::read(&path).unwrap()).data;
    for (id, doc) in bundle["documents"].as_object().unwrap() {
        let resp = get(&app, &format!("/document/{id}/thumb")).await;
        if doc["mime_type"] == "image/png" {
            assert_eq!(resp.status(), StatusCode::OK);
            assert_eq!(
                resp.headers().get(header::CONTENT_TYPE).unwrap(),
                "image/png"
            );
            let thumb = body_bytes(resp).await;
            let decoded = image::load_from_memory(&thumb).expect("a real png");
            assert!(decoded.width() <= 320 && decoded.height() <= 320);
            assert!(
                thumb.len() < 200_000,
                "a thumbnail should be much smaller than the original"
            );
        } else {
            assert_eq!(
                resp.status(),
                StatusCode::NOT_FOUND,
                "a text file has no thumbnail"
            );
        }
    }
}

#[tokio::test]
async fn a_document_with_no_payload_renders_without_breaking() {
    // 400 of the operator's documents are `referenced`: the record names the
    // file, the bundle does not carry it. The page must say so and offer
    // nothing to download rather than linking into an error.
    let dir = scratch("referenced-src");
    let path = dir.join("referenced.axgf");
    let doc_id = "22222222-2222-4222-8222-222222222222";
    let flat = json!({
        "manifest": {"axgf": "1.0"},
        "persons": {
            PERSON: {
                "id": PERSON, "type": "person", "axgf_version": "1.0",
                "identity": {"name": {"display": "Jules Meunier", "components": []}},
                "documents": [{"document_id": doc_id, "role": "portrait"}]}
        },
        "documents": {
            doc_id: {"id": doc_id, "type": "document", "axgf_version": "1.0",
                     "filename": "teofila german.png", "mime_type": "image/png",
                     "document_type": "photo", "status": "referenced",
                     "file": {"path": "teofila german.png"}}
        },
        "families": {}, "links": {}, "occupations": {}, "sources": {},
        "places": {}, "events": {}
    });
    let bytes = axgf_cms::state::export_to_bytes(&flat.to_string()).expect("export");
    std::fs::write(&path, bytes).expect("write");

    let (app, _p) = app_with_bundle("referenced", &path);
    let body = expect_status(
        get(&app, &format!("/person/{PERSON}")).await,
        StatusCode::OK,
        "person with a referenced document",
    )
    .await;

    assert!(body.contains("teofila german.png"));
    assert!(body.contains("referenced"));
    assert!(body.contains("no file"), "it must say the file is not here");
    assert!(
        !body.contains(&format!("/document/{doc_id}/raw")),
        "nothing to download, so nothing may be linked"
    );
    assert!(
        !body.contains(&format!("/document/{doc_id}/thumb")),
        "and no gallery entry either"
    );

    // Asking for it directly is a clean 404, not a panic.
    let resp = get(&app, &format!("/document/{doc_id}/raw")).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    let resp = get(&app, &format!("/document/{doc_id}/thumb")).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn the_upload_form_is_only_offered_to_an_admin() {
    let src = one_person_bundle("upload-form-src");
    let (app, _p) = app_with_bundle("upload-form", &src);

    let anon = body_string(get(&app, &format!("/person/{PERSON}")).await).await;
    assert!(!anon.contains("Attach a document"));

    let admin = body_string(get_admin(&app, &format!("/person/{PERSON}")).await).await;
    assert!(admin.contains("Attach a document"));
    assert!(admin.contains(&format!("/admin/person/{PERSON}/document")));
}

#[tokio::test]
async fn an_uploaded_photograph_appears_in_the_gallery() {
    let src = one_person_bundle("upload-gallery-src");
    let (app, path) = app_with_bundle("upload-gallery", &src);
    upload(
        &app,
        PERSON,
        "wedding.png",
        "image/png",
        &png(200, 150),
        true,
    )
    .await;

    let doc_id = document_ids(&path).into_iter().next().expect("a document");
    let body = body_string(get(&app, &format!("/person/{PERSON}")).await).await;
    assert!(body.contains("Photographs"));
    assert!(body.contains(&format!("/document/{doc_id}/thumb")));
    assert!(body.contains(&format!("/document/{doc_id}/raw")));
    assert!(body.contains("A caption"));
}

#[tokio::test]
async fn the_admin_dashboard_reports_the_bundle_size() {
    let src = one_person_bundle("upload-size-src");
    let (app, _p) = app_with_bundle("upload-size", &src);
    upload(&app, PERSON, "a.png", "image/png", &png(300, 300), true).await;

    let body = body_string(get_admin(&app, "/admin").await).await;
    assert!(body.contains("attached file"), "{body}");
    assert!(body.contains("on disk"));
    // Well under the threshold, so no warning.
    assert!(!body.contains("starts costing real memory"));
}

#[tokio::test]
async fn an_svg_is_refused_at_the_door() {
    // The documented decision is refusal, not sanitisation and not
    // store-but-download: an SVG is a program a browser runs, and the rule an
    // operator reads in the README has to be the rule the code follows.
    let src = one_person_bundle("upload-svg-src");
    let (app, path) = app_with_bundle("upload-svg", &src);
    let svg = br#"<?xml version="1.0"?>
<svg xmlns="http://www.w3.org/2000/svg"><script>fetch('/admin/export')</script></svg>"#;

    let resp = upload(&app, PERSON, "family-crest.svg", "image/svg+xml", svg, true).await;
    assert_eq!(resp.status(), StatusCode::UNSUPPORTED_MEDIA_TYPE);
    let body = body_string(resp).await;
    assert!(
        body.contains("SVG is refused"),
        "the rule is stated: {body}"
    );
    assert!(document_ids(&path).is_empty());
}

#[tokio::test]
async fn a_plain_text_note_is_stored_and_downloads_rather_than_rendering() {
    let src = one_person_bundle("upload-text-src");
    let (app, path) = app_with_bundle("upload-text", &src);
    let note = b"Notes from the parish visit, March 2019.\nThe register starts in 1782.\n";

    let resp = upload(&app, PERSON, "notes.txt", "text/plain", note, true).await;
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);

    let doc_id = document_ids(&path).into_iter().next().expect("a document");
    let resp = get(&app, &format!("/document/{doc_id}/raw")).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let disposition = resp
        .headers()
        .get(header::CONTENT_DISPOSITION)
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    assert!(disposition.starts_with("attachment"), "{disposition}");
    assert_eq!(body_bytes(resp).await, note.to_vec());
}

#[tokio::test]
async fn the_extension_follows_the_bytes_not_the_name() {
    // A PDF uploaded as "holiday.jpg" is stored as a PDF called "holiday.pdf".
    let src = one_person_bundle("upload-ext-src");
    let (app, path) = app_with_bundle("upload-ext", &src);
    let mut pdf = b"%PDF-1.7\n".to_vec();
    pdf.extend_from_slice(b"trailer\n%%EOF\n");

    upload(&app, PERSON, "holiday.jpg", "image/jpeg", &pdf, true).await;

    let bundle = axgf_rs::import_bundle(&std::fs::read(&path).unwrap()).data;
    let (id, doc) = bundle["documents"]
        .as_object()
        .unwrap()
        .iter()
        .next()
        .expect("a document");
    assert_eq!(doc["mime_type"], "application/pdf");
    assert_eq!(doc["filename"], "holiday.pdf");
    assert_eq!(doc["file"]["path"], format!("documents/files/{id}.pdf"));
}

#[tokio::test]
async fn a_filename_cannot_escape_its_directory_or_the_header_quoting() {
    let src = one_person_bundle("upload-name-src");
    let (app, path) = app_with_bundle("upload-name", &src);
    upload(
        &app,
        PERSON,
        "../../etc/pass\"wd.png",
        "image/png",
        &png(8, 8),
        true,
    )
    .await;

    let doc_id = document_ids(&path).into_iter().next().expect("a document");
    let bundle = axgf_rs::import_bundle(&std::fs::read(&path).unwrap()).data;
    let stored = bundle["documents"][&doc_id]["filename"].as_str().unwrap();
    assert!(!stored.contains('/'), "no path separators: {stored}");
    assert!(!stored.contains('"'), "no quotes: {stored}");
    assert_eq!(
        bundle["documents"][&doc_id]["file"]["path"],
        format!("documents/files/{doc_id}.png"),
        "the stored path is derived from the id, never from the name"
    );

    // And the header it lands in is still one well-formed header.
    let resp = get(&app, &format!("/document/{doc_id}/raw")).await;
    let disposition = resp
        .headers()
        .get(header::CONTENT_DISPOSITION)
        .unwrap()
        .to_str()
        .unwrap();
    assert_eq!(disposition.matches('"').count(), 2, "{disposition}");
}
