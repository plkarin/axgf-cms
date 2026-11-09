//! A record's films and sounds, played on its page (docs/adr/0041): served
//! inline, a piece at a time when the player asks for one.

mod common;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use common::*;
use serde_json::json;
use tower::ServiceExt;

const JEAN: &str = "a0000000-0000-4000-8000-0000000000aa";

fn bundle(tag: &str) -> common::Scratch {
    let dir = scratch(tag);
    let path = dir.join("media.axgf");
    let flat = json!({
        "manifest": {"axgf": "1.1"},
        "persons": {JEAN: {"id": JEAN, "type": "person", "axgf_version": "1.1",
            "identity": {"name": {"display": "Jean Martin", "components": []},
                         "is_living": false, "visibility": "public"}}},
        "families": {}, "events": {}, "links": {}, "occupations": {}, "places": {},
        "sources": {}, "documents": {}
    });
    std::fs::write(
        &path,
        axgf_cms::state::export_to_bytes(&flat.to_string()).expect("export"),
    )
    .expect("write");
    dir.pointing_at(path)
}

/// A film made here: an ISO media header and bytes that count.
fn film() -> Vec<u8> {
    let mut v = vec![0, 0, 0, 0x20];
    v.extend_from_slice(b"ftypisom");
    v.extend((0..1000u32).map(|i| (i % 251) as u8));
    v
}

async fn attach(app: &axum::Router, cookie: &str, name: &str, bytes: &[u8]) {
    let b = "----axgfmediafile";
    let mut body = format!(
        "--{b}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"{name}\"\r\n\
         Content-Type: application/octet-stream\r\n\r\n"
    )
    .into_bytes();
    body.extend_from_slice(bytes);
    body.extend_from_slice(
        format!("\r\n--{b}\r\nContent-Disposition: form-data; name=\"caption\"\r\n\r\nLe mariage\r\n--{b}--\r\n")
            .as_bytes(),
    );
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/admin/person/{JEAN}/document"))
                .method("POST")
                .header(header::COOKIE, cookie)
                .header(
                    header::CONTENT_TYPE,
                    format!("multipart/form-data; boundary={b}"),
                )
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();
    assert!(
        resp.status().is_success() || resp.status().is_redirection(),
        "attached: {}",
        resp.status()
    );
}

async fn ranged(
    app: &axum::Router,
    cookie: &str,
    uri: &str,
    range: Option<&str>,
) -> axum::response::Response {
    let mut req = Request::builder().uri(uri).header(header::COOKIE, cookie);
    if let Some(r) = range {
        req = req.header(header::RANGE, r);
    }
    app.clone()
        .oneshot(req.body(Body::empty()).unwrap())
        .await
        .unwrap()
}

#[tokio::test]
async fn a_film_plays_on_the_record_a_piece_at_a_time() {
    let src = bundle("media-src");
    let (app, _p) = app_with_bundle("media", &src);
    let cookie = admin_cookie(&app).await;
    let bytes = film();
    attach(&app, &cookie, "mariage.mp4", &bytes).await;

    // The Media tab plays it.
    let page =
        body_string(get_with_cookie(&app, &format!("/person/{JEAN}?tab=media"), &cookie).await)
            .await;
    let id = page
        .split("<video controls preload=\"metadata\" playsinline src=\"/document/")
        .nth(1)
        .and_then(|s| s.split('/').next())
        .expect("a player on the Media tab")
        .to_string();
    assert!(page.contains("aria-label=\"Le mariage\""));
    let raw = format!("/document/{id}/raw");

    // Whole, inline, saying it gives pieces.
    let resp = ranged(&app, &cookie, &raw, None).await;
    assert_eq!(resp.status(), StatusCode::OK);
    assert_eq!(resp.headers()[header::ACCEPT_RANGES], "bytes");
    assert_eq!(resp.headers()[header::CONTENT_TYPE], "video/mp4");
    assert!(resp.headers()[header::CONTENT_DISPOSITION]
        .to_str()
        .unwrap()
        .starts_with("inline"));
    assert_eq!(body_bytes(resp).await, bytes);

    // A piece.
    let resp = ranged(&app, &cookie, &raw, Some("bytes=10-19")).await;
    assert_eq!(resp.status(), StatusCode::PARTIAL_CONTENT);
    let len = bytes.len();
    assert_eq!(
        resp.headers()[header::CONTENT_RANGE],
        format!("bytes 10-19/{len}").as_str()
    );
    assert_eq!(resp.headers()[header::CONTENT_LENGTH], "10");
    assert_eq!(body_bytes(resp).await, bytes[10..20]);

    // From somewhere to the end, and the last bytes.
    let resp = ranged(&app, &cookie, &raw, Some("bytes=1000-")).await;
    assert_eq!(resp.status(), StatusCode::PARTIAL_CONTENT);
    assert_eq!(body_bytes(resp).await, bytes[1000..]);
    let resp = ranged(&app, &cookie, &raw, Some("bytes=-4")).await;
    assert_eq!(body_bytes(resp).await, bytes[len - 4..]);

    // Past the end: nothing to give.
    let resp = ranged(&app, &cookie, &raw, Some("bytes=999999-")).await;
    assert_eq!(resp.status(), StatusCode::RANGE_NOT_SATISFIABLE);
    assert_eq!(
        resp.headers()[header::CONTENT_RANGE],
        format!("bytes */{len}").as_str()
    );
}

fn encode(pairs: &[(&str, &str)]) -> String {
    pairs
        .iter()
        .map(|(k, v)| format!("{k}={}", v.replace('/', "%2F")))
        .collect::<Vec<_>>()
        .join("&")
}

/// Every kind the archive recognises, ticked, but `refused`.
fn kinds_but(refused: &[&str]) -> Vec<(&'static str, &'static str)> {
    axgf_cms::limits::KINDS
        .iter()
        .flat_map(|(_, k)| k.iter())
        .filter(|(m, _)| !refused.contains(m))
        .map(|(m, _)| ("kind", *m))
        .collect()
}

/// The limits and the kinds are set in the administration and hold at once,
/// without a restart (docs/adr/0041).
#[tokio::test]
async fn the_administration_sets_what_a_file_may_weigh_and_be() {
    let src = bundle("limits-src");
    let (app, _p) = app_with_bundle("limits", &src);
    let cookie = admin_cookie(&app).await;

    let page = body_string(get_with_cookie(&app, "/admin/uploads", &cookie).await).await;
    assert!(
        page.contains(r#"name="document" value="50""#),
        "the default"
    );
    assert!(page.contains("16 kinds accepted out of 16"));
    let room = body_string(get_with_cookie(&app, "/admin/bundle", &cookie).await).await;
    assert!(
        room.contains(r#"href="/admin/uploads""#),
        "the archive's room links to it"
    );

    // Out of bounds: refused, nothing kept.
    let mut form = vec![
        ("document", "0"),
        ("note-image", "10"),
        ("assistant-file", "20"),
        ("assistant-message", "40"),
        ("gedcom", "10"),
        ("archive", "4096"),
    ];
    form.extend(kinds_but(&[]));
    let resp = post_form_as(&app, &cookie, "/admin/uploads", &encode(&form)).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    assert!(body_string(resp)
        .await
        .contains("a whole number from 1 to 2048"));

    // One megabyte for a record's files, and no films in AVI.
    form[0] = ("document", "1");
    let mut form: Vec<(&str, &str)> = form.into_iter().filter(|(k, _)| *k != "kind").collect();
    form.extend(kinds_but(&["video/x-msvideo"]));
    let resp = post_form_as(&app, &cookie, "/admin/uploads", &encode(&form)).await;
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    let page = body_string(get_with_cookie(&app, "/admin/uploads", &cookie).await).await;
    assert!(page.contains(r#"name="document" value="1""#));
    assert!(page.contains("15 kinds accepted out of 16"));
    // The attribute's slash comes escaped; what follows it is the point.
    let after_avi = page.split("x-msvideo\"").nth(1).expect("the AVI box");
    assert!(
        after_avi.starts_with('>'),
        "AVI unticked: {}",
        &after_avi[..20]
    );
    let after_mp4 = page.split("video&#x2f;mp4\"").nth(1).expect("the MP4 box");
    assert!(after_mp4.starts_with(" checked"), "MP4 still ticked");

    // At once: a file of two megabytes is refused, and so is an AVI.
    let big = {
        let mut v = film();
        v.resize(2 * 1024 * 1024, 7);
        v
    };
    let refused = upload_status(&app, &cookie, "big.mp4", &big).await;
    assert_eq!(refused.0, StatusCode::PAYLOAD_TOO_LARGE, "{}", refused.1);
    assert!(refused.1.contains("1 MB"), "{}", refused.1);
    let mut avi = b"RIFF\0\0\0\0AVI LIST".to_vec();
    avi.extend_from_slice(&[0; 64]);
    let refused = upload_status(&app, &cookie, "old.avi", &avi).await;
    assert_eq!(refused.0, StatusCode::UNSUPPORTED_MEDIA_TYPE);
    assert!(
        refused.1.contains("(AVI) is refused by this site"),
        "{}",
        refused.1
    );
    // A film in MP4 within the limit still goes.
    let ok = upload_status(&app, &cookie, "ok.mp4", &film()).await;
    assert!(ok.0.is_success() || ok.0.is_redirection(), "{:?}", ok.0);
}

async fn upload_status(
    app: &axum::Router,
    cookie: &str,
    name: &str,
    bytes: &[u8],
) -> (StatusCode, String) {
    let b = "----axgfmedialimits";
    let mut body = format!(
        "--{b}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"{name}\"\r\n\
         Content-Type: application/octet-stream\r\n\r\n"
    )
    .into_bytes();
    body.extend_from_slice(bytes);
    body.extend_from_slice(format!("\r\n--{b}--\r\n").as_bytes());
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/admin/person/{JEAN}/document"))
                .method("POST")
                .header(header::COOKIE, cookie)
                .header(
                    header::CONTENT_TYPE,
                    format!("multipart/form-data; boundary={b}"),
                )
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();
    let status = resp.status();
    (status, body_string(resp).await)
}
