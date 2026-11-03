//! The AFG on a person's record (docs/adr/0029), for a member it is open to:
//! the face restored into the band of reworked portraits, a photograph
//! restored beside itself as `_IA` and never the face, a document read into a
//! note, and the summary written above Identity in the language asked. The
//! notes are kept up on the Médias tab and read on the Fiche; the picture
//! service is the administrator's, its key never shown.
//!
//! A fake stands in for every service: a local HTTP server for OpenAI's and
//! Gemini's pictures, and for an Ollama that reads and writes. Nothing leaves
//! the machine, and every picture and word below is the fake's.

mod common;

use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpListener;
use std::sync::{Arc, Mutex};

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use base64::Engine as _;
use common::*;
use serde_json::{json, Value};
use tower::ServiceExt;

const PASSWORD: &str = "correct-horse-battery-staple";
const LUC: &str = "c1000000-0000-4000-8000-000000000001";
const ALI: &str = "c1000000-0000-4000-8000-000000000002";
const IMAGE_KEY: &str = "sk-fake-images-0123456789wxyz";

const LETTER: &str = "Chère Marie,\n\nNous sommes bien arrivés à Lyon.\n\nTon frère, Lucien";
const SUMMARY_PL: &str = "Lucien Fabre urodził się około 1924 roku i był stolarzem.";
const SUMMARY_FR: &str = "Lucien Fabre, né vers 1924, était menuisier.";

// ---------------------------------------------------------------------------
// the fakes
// ---------------------------------------------------------------------------

/// What a fake service was sent: method, path and body of each request.
type Seen = Arc<Mutex<Vec<(String, String, String)>>>;

/// A local HTTP server answering each request with `respond`, one connection
/// at a time, keeping what it was sent.
fn fake(respond: fn(&str, &str, &str) -> (u16, String)) -> (String, Seen) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
    let url = format!("http://{}", listener.local_addr().unwrap());
    let seen: Seen = Arc::default();
    let log = seen.clone();
    std::thread::spawn(move || {
        for stream in listener.incoming() {
            let Ok(mut stream) = stream else { continue };
            let mut reader = BufReader::new(stream.try_clone().expect("clone"));
            let mut first = String::new();
            if reader.read_line(&mut first).is_err() {
                continue;
            }
            let mut words = first.split_whitespace();
            let method = words.next().unwrap_or_default().to_string();
            let path = words.next().unwrap_or_default().to_string();
            let mut length = 0;
            loop {
                let mut line = String::new();
                if reader.read_line(&mut line).unwrap_or(0) == 0 || line.trim().is_empty() {
                    break;
                }
                if let Some((k, v)) = line.split_once(':') {
                    if k.trim().eq_ignore_ascii_case("content-length") {
                        length = v.trim().parse().unwrap_or(0);
                    }
                }
            }
            let mut body = vec![0; length];
            let _ = reader.read_exact(&mut body);
            let body = String::from_utf8_lossy(&body).into_owned();
            let (status, reply) = respond(&method, &path, &body);
            log.lock().unwrap().push((method, path, body));
            let _ = write!(
                stream,
                "HTTP/1.1 {status} Fake\r\nContent-Type: application/json\r\n\
                 Content-Length: {}\r\nConnection: close\r\n\r\n{reply}",
                reply.len()
            );
        }
    });
    (url, seen)
}

fn png(w: u32, h: u32, shade: u8) -> Vec<u8> {
    let img = image::RgbImage::from_pixel(w, h, image::Rgb([shade, 120, 80]));
    let mut out = std::io::Cursor::new(Vec::new());
    image::DynamicImage::ImageRgb8(img)
        .write_to(&mut out, image::ImageFormat::Png)
        .unwrap();
    out.into_inner()
}

fn b64(bytes: &[u8]) -> String {
    base64::engine::general_purpose::STANDARD.encode(bytes)
}

/// OpenAI's picture edits: the photograph comes back restored, in colour.
fn openai_images(method: &str, path: &str, _: &str) -> (u16, String) {
    match (method, path) {
        ("POST", "/v1/images/edits") => (
            200,
            json!({"created": 1, "data": [{"b64_json": b64(&png(64, 64, 210))}]}).to_string(),
        ),
        _ => (404, r#"{"error":{"message":"no such path"}}"#.into()),
    }
}

/// Gemini, asked for a picture: it answers with one inline.
fn gemini_images(method: &str, path: &str, _: &str) -> (u16, String) {
    if method == "POST" && path.ends_with(":generateContent") {
        return (
            200,
            json!({"candidates": [{"content": {"parts": [
                {"inlineData": {"mimeType": "image/png", "data": b64(&png(48, 48, 90))}}
            ]}, "finishReason": "STOP"}]})
            .to_string(),
        );
    }
    (404, r#"{"error":{"message":"no such path"}}"#.into())
}

/// Gemini refusing the picture, for its own reason.
fn gemini_refuses(_: &str, _: &str, _: &str) -> (u16, String) {
    (
        200,
        json!({"candidates": [{"content": {"parts": []}, "finishReason": "IMAGE_SAFETY"}]})
            .to_string(),
    )
}

/// An Ollama that reads letters and writes summaries, in the language asked.
fn ollama(method: &str, path: &str, body: &str) -> (u16, String) {
    if (method, path) != ("POST", "/api/chat") {
        return (404, r#"{"error":"no such path"}"#.into());
    }
    let words = if body.contains("Transcribe every word") {
        LETTER
    } else if body.contains("Write it in Polish") {
        SUMMARY_PL
    } else if body.contains("short summary") {
        SUMMARY_FR
    } else {
        "?"
    };
    (
        200,
        json!({"model": "fausse-famille:1b", "message": {"role": "assistant", "content": words},
               "done": true})
        .to_string(),
    )
}

/// An Ollama that finds nothing to read.
fn ollama_reads_nothing(_: &str, _: &str, _: &str) -> (u16, String) {
    (
        200,
        json!({"model": "fausse-famille:1b", "message": {"content": "NO_TEXT"}, "done": true})
            .to_string(),
    )
}

// ---------------------------------------------------------------------------
// the family, its accounts, its files
// ---------------------------------------------------------------------------

/// Lucien and Alice, with no file yet; ana (an administrator the AFG is open
/// to), dora (a contributor it is open to) and carl (a contributor it is
/// not). Their sessions, in that order, reading French.
async fn family(tag: &str) -> (axum::Router, Scratch, Vec<String>) {
    use axgf_cms::acl::{new_user, Acl, Role};
    let dir = scratch(tag);
    let path = dir.join("family.axgf");
    let person = |id: &str, name: &str| {
        json!({"id": id, "type": "person", "axgf_version": "1.1",
               "identity": {"name": {"display": name, "components": []},
                            "is_living": false, "visibility": "public"},
               "birth": {"date": {"value": "1924", "precision": "year"}}})
    };
    let flat = json!({
        "manifest": {"axgf": "1.1"},
        "persons": {LUC: person(LUC, "Lucien Fabre"), ALI: person(ALI, "Alice Fabre")},
        "families": {}, "events": {}, "links": {}, "occupations": {}, "places": {},
        "sources": {}, "documents": {}
    });
    std::fs::write(
        &path,
        axgf_cms::state::export_to_bytes(&flat.to_string()).expect("export"),
    )
    .expect("write");
    let mut acl = Acl::default();
    for (name, role, afg) in [
        ("ana", Role::Admin, true),
        ("dora", Role::Contributor, true),
        ("carl", Role::Contributor, false),
    ] {
        let mut u = new_user(name, PASSWORD, role).unwrap();
        u.afg = afg;
        acl.users.push(u);
    }
    acl.save(&Acl::path_for(&path)).unwrap();
    let app = build_app(&path).unwrap();
    let mut cookies = Vec::new();
    for name in ["ana", "dora", "carl"] {
        cookies.push(format!("{}; axgf_lang=fr", sign_in(&app, name).await));
    }
    (app, dir.pointing_at(path), cookies)
}

async fn sign_in(app: &axum::Router, name: &str) -> String {
    let login = post_form(
        app,
        "/admin/login",
        &format!("username={name}&password={PASSWORD}"),
        false,
    )
    .await;
    assert!(login.status().is_redirection(), "{name} signs in");
    login.headers()[header::SET_COOKIE]
        .to_str()
        .unwrap()
        .split(';')
        .next()
        .unwrap()
        .to_string()
}

fn enc(v: &str) -> String {
    v.bytes()
        .map(|b| match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                (b as char).to_string()
            }
            _ => format!("%{b:02X}"),
        })
        .collect()
}

/// A path as the templates write it in an attribute: its slashes escaped.
fn attr(path: &str) -> String {
    path.replace('/', "&#x2f;")
}

fn form(pairs: &[(&str, &str)]) -> String {
    pairs
        .iter()
        .map(|(k, v)| format!("{k}={}", enc(v)))
        .collect::<Vec<_>>()
        .join("&")
}

/// The picture service, set by an administrator.
async fn set_images(
    app: &axum::Router,
    admin: &str,
    pairs: &[(&str, &str)],
) -> (StatusCode, String) {
    let resp = post_form_as(app, admin, "/admin/afg/images", &form(pairs)).await;
    let status = resp.status();
    (status, body_string(resp).await)
}

/// The AFG's computing: the fake Ollama at `url`.
async fn set_ollama(app: &axum::Router, admin: &str, url: &str) {
    let resp = post_form_as(
        app,
        admin,
        "/admin/afg",
        &form(&[
            ("kind", "ollama"),
            ("url", url),
            ("model", "fausse-famille:1b"),
            ("timeout", "60"),
        ]),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::OK, "the AFG is set");
}

/// Attach a file to Lucien, as the record's upload does, and give its id.
async fn attach(
    app: &axum::Router,
    cookie: &str,
    filename: &str,
    bytes: Vec<u8>,
    caption: &str,
) -> String {
    let before: Vec<String> = documents(app, cookie)
        .await
        .as_object()
        .map(|m| m.keys().cloned().collect())
        .unwrap_or_default();
    let boundary = "----axgfrecordboundary";
    let mut body = Vec::new();
    body.extend_from_slice(
        format!(
            "--{boundary}\r\nContent-Disposition: form-data; name=\"caption\"\r\n\r\n{caption}\r\n"
        )
        .as_bytes(),
    );
    body.extend_from_slice(
        format!(
            "--{boundary}\r\nContent-Disposition: form-data; name=\"file\"; \
             filename=\"{filename}\"\r\nContent-Type: application/octet-stream\r\n\r\n"
        )
        .as_bytes(),
    );
    body.extend_from_slice(&bytes);
    body.extend_from_slice(format!("\r\n--{boundary}--\r\n").as_bytes());
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/admin/person/{LUC}/document"))
                .method("POST")
                .header(
                    header::CONTENT_TYPE,
                    format!("multipart/form-data; boundary={boundary}"),
                )
                .header(header::COOKIE, cookie)
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();
    assert!(resp.status().is_redirection(), "the file is attached");
    documents(app, cookie)
        .await
        .as_object()
        .unwrap()
        .keys()
        .find(|k| !before.contains(k))
        .cloned()
        .expect("a new document")
}

/// The archive's documents, as the server's own export holds them.
async fn documents(app: &axum::Router, cookie: &str) -> Value {
    let resp = get_with_cookie(app, "/admin/export", cookie).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let bytes = body_bytes(resp).await;
    axgf_rs::import_bundle(&bytes).data["documents"].clone()
}

fn tagged<'a>(docs: &'a Value, tag: &str) -> Vec<(&'a String, &'a Value)> {
    docs.as_object()
        .into_iter()
        .flatten()
        .filter(|(_, d)| {
            d["tags"]
                .as_array()
                .is_some_and(|t| t.iter().any(|x| x == tag))
        })
        .collect()
}

fn location(resp: &axum::response::Response) -> String {
    resp.headers()[header::LOCATION]
        .to_str()
        .unwrap()
        .to_string()
}

async fn page(app: &axum::Router, uri: &str, cookie: &str) -> String {
    let resp = get_with_cookie(app, uri, cookie).await;
    let status = resp.status();
    let body = body_string(resp).await;
    assert_eq!(
        status,
        StatusCode::OK,
        "{uri}: {}",
        &body[..body.len().min(3000)]
    );
    body
}

// ---------------------------------------------------------------------------
// who may
// ---------------------------------------------------------------------------

#[tokio::test]
async fn a_member_the_afg_is_not_open_to_has_no_button_and_is_refused() {
    let (app, _dir, cookies) = family("afg-record-who").await;
    let (ana, carl) = (&cookies[0], &cookies[2]);
    let (images, _) = fake(openai_images);
    let (text, seen) = fake(ollama);
    set_images(
        &app,
        ana,
        &[
            ("kind", "openai"),
            ("url", &format!("{images}/v1")),
            ("api_key", IMAGE_KEY),
        ],
    )
    .await;
    set_ollama(&app, ana, &text).await;
    let did = attach(
        &app,
        ana,
        "lettre-1932.png",
        png(32, 24, 40),
        "Lettre de Lucien",
    )
    .await;

    let media = page(&app, &format!("/person/{LUC}?tab=media"), carl).await;
    assert!(
        !media.contains("/ai-retouch") && !media.contains("/ai-read"),
        "no AI button for carl"
    );
    let record = page(&app, &format!("/person/{LUC}"), carl).await;
    assert!(!record.contains("/summary/ai") && !record.contains("/ai-rework"));
    assert!(
        record.contains("kind=summary"),
        "carl may still write the summary by hand"
    );

    for uri in [
        format!("/admin/person/{LUC}/document/{did}/ai-retouch"),
        format!("/admin/person/{LUC}/document/{did}/ai-read"),
        format!("/admin/person/{LUC}/ai-rework"),
        format!("/admin/person/{LUC}/summary/ai"),
    ] {
        let resp = post_form_as(&app, carl, &uri, "lang=fr").await;
        assert_eq!(resp.status(), StatusCode::FORBIDDEN, "{uri}");
        let resp = post_form(&app, &uri, "lang=fr", false).await;
        assert!(!resp.status().is_success(), "{uri}, signed out");
    }
    assert!(seen.lock().unwrap().is_empty(), "nothing was sent anywhere");

    // ana sees them all.
    let media = page(&app, &format!("/person/{LUC}?tab=media"), ana).await;
    assert!(media.contains(&format!("/document/{did}/ai-retouch")));
    assert!(media.contains(&format!("/document/{did}/ai-read")));
}

// ---------------------------------------------------------------------------
// pictures
// ---------------------------------------------------------------------------

#[tokio::test]
async fn a_photograph_restored_is_kept_beside_it_marked_and_never_the_face() {
    let (app, _dir, cookies) = family("afg-record-retouch").await;
    let ana = &cookies[0];
    let (images, seen) = fake(openai_images);
    let (status, _) = set_images(
        &app,
        ana,
        &[
            ("kind", "openai"),
            ("url", &format!("{images}/v1")),
            ("api_key", IMAGE_KEY),
        ],
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    let did = attach(&app, ana, "mariage-1949.png", png(40, 30, 30), "Le mariage").await;

    let resp = post_form_as(
        &app,
        ana,
        &format!("/admin/person/{LUC}/document/{did}/ai-retouch"),
        "",
    )
    .await;
    assert_eq!(resp.status(), StatusCode::SEE_OTHER, "restored");
    assert_eq!(location(&resp), format!("/person/{LUC}?tab=media#evidence"));

    let sent = seen.lock().unwrap().clone();
    assert_eq!(sent.len(), 1);
    let (method, path, body) = &sent[0];
    assert_eq!(
        (method.as_str(), path.as_str()),
        ("POST", "/v1/images/edits")
    );
    assert!(
        body.contains("name=\"model\"") && body.contains("gpt-image-2.5-sunburst"),
        "the first suggested model"
    );
    assert!(body.contains("name=\"image[]\"") && body.contains("Restore this old photograph"));

    let docs = documents(&app, ana).await;
    let copies = tagged(&docs, "ai-retouched");
    assert_eq!(copies.len(), 1);
    let (cid, copy) = copies[0];
    assert_eq!(copy["filename"], "mariage-1949_IA.png");
    assert_eq!(copy["caption"], "Le mariage");
    assert_eq!(copy["document_type"], "other", "nobody took it");
    assert_eq!(copy["linked_to"][0]["entity_id"], LUC);
    assert_eq!(
        copy["extensions"]["axgf-cms:ai-retouch/v1"]["derived_from_id"],
        did.as_str()
    );
    assert_eq!(
        copy["extensions"]["axgf-cms:ai-retouch/v1"]["generator"],
        "OpenAI gpt-image-2.5-sunburst"
    );
    assert!(
        !axgf_cms::ai_portrait::is_ai_portrait(copy),
        "a copy in the media, not a portrait"
    );

    // In the Media tab, marked; offered no second retouch; never the face.
    let media = page(&app, &format!("/person/{LUC}?tab=media"), ana).await;
    assert!(media.contains("mariage-1949_IA.png") && media.contains("shot is-ai"));
    assert!(
        !media.contains(&format!("/document/{cid}/ai-retouch")),
        "no retouch of a retouch"
    );
    assert!(media.contains(&format!("/document/{did}/ai-retouch")));
    let record = page(&app, &format!("/person/{LUC}"), ana).await;
    assert!(
        record.contains(&format!(
            "person-avatar\" data-content href=\"/document/{did}/view"
        )),
        "the face is the photograph"
    );
    // Chosen as the face, it is refused.
    let resp = post_form_as(
        &app,
        ana,
        &format!("/admin/person/{LUC}/avatar"),
        &form(&[("choice", cid), ("base_version", "1")]),
    )
    .await;
    assert!(
        !resp.status().is_redirection(),
        "a copy an AI made is never the face"
    );
}

#[tokio::test]
async fn the_face_reworked_joins_the_band_as_an_upload_would() {
    let (app, _dir, cookies) = family("afg-record-rework").await;
    let ana = &cookies[0];
    let did = attach(&app, ana, "lucien.png", png(40, 40, 60), "").await;

    // No picture service yet: the button is greyed, and the door refuses.
    let record = page(&app, &format!("/person/{LUC}"), ana).await;
    assert!(record.contains("/ai-rework") && record.contains("/admin/afg#images"));
    let resp = post_form_as(&app, ana, &format!("/admin/person/{LUC}/ai-rework"), "").await;
    assert_eq!(resp.status(), StatusCode::CONFLICT);

    let (images, seen) = fake(gemini_images);
    let (status, _) = set_images(
        &app,
        ana,
        &[
            ("kind", "gemini"),
            ("url", &format!("{images}/v1beta")),
            ("api_key", IMAGE_KEY),
        ],
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    let resp = post_form_as(&app, ana, &format!("/admin/person/{LUC}/ai-rework"), "").await;
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    assert_eq!(location(&resp), format!("/person/{LUC}#ai-portraits"));
    {
        let sent = seen.lock().unwrap();
        let (_, path, body) = &sent[0];
        assert_eq!(
            path,
            "/v1beta/models/gemini-3.1-flash-image:generateContent"
        );
        assert!(
            body.contains("\"responseModalities\":[\"IMAGE\"]") && body.contains("inline_data")
        );
    }
    let docs = documents(&app, ana).await;
    let portraits: Vec<&Value> = docs
        .as_object()
        .unwrap()
        .values()
        .filter(|d| axgf_cms::ai_portrait::is_ai_portrait(d))
        .collect();
    assert_eq!(portraits.len(), 1);
    let p = portraits[0];
    assert_eq!(p["filename"], "lucien_IA.png");
    assert_eq!(
        p["extensions"]["axgf-cms:ai-portrait/v1"]["derived_from_id"],
        did.as_str()
    );
    assert_eq!(
        p["extensions"]["axgf-cms:ai-portrait/v1"]["generator"],
        "Google gemini-3.1-flash-image"
    );
    let record = page(&app, &format!("/person/{LUC}"), ana).await;
    assert!(record.contains("person-ai-shot"), "in the band");

    // Three at most: past them the button is greyed and the door refuses.
    for _ in 0..2 {
        let resp = post_form_as(&app, ana, &format!("/admin/person/{LUC}/ai-rework"), "").await;
        assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    }
    let resp = post_form_as(&app, ana, &format!("/admin/person/{LUC}/ai-rework"), "").await;
    assert_eq!(resp.status(), StatusCode::CONFLICT);
    assert_eq!(seen.lock().unwrap().len(), 3, "the fourth was never sent");

    // A refusal of the service's own is shown in its words, and nothing kept.
    let (refusing, _) = fake(gemini_refuses);
    set_images(
        &app,
        ana,
        &[
            ("kind", "gemini"),
            ("url", &format!("{refusing}/v1beta")),
            ("api_key", IMAGE_KEY),
        ],
    )
    .await;
    let did2 = attach(&app, ana, "noces.png", png(20, 20, 10), "").await;
    let resp = post_form_as(
        &app,
        ana,
        &format!("/admin/person/{LUC}/document/{did2}/ai-retouch"),
        "",
    )
    .await;
    assert_eq!(resp.status(), StatusCode::BAD_GATEWAY);
    assert!(body_string(resp).await.contains("IMAGE_SAFETY"));
    assert!(tagged(&documents(&app, ana).await, "ai-retouched").is_empty());
}

#[tokio::test]
async fn the_picture_service_is_the_administrators_and_its_key_is_never_shown() {
    let (app, dir, cookies) = family("afg-record-service").await;
    let (ana, dora) = (&cookies[0], &cookies[1]);
    let resp = post_form_as(&app, dora, "/admin/afg/images", "kind=openai&api_key=sk-x").await;
    assert_eq!(
        resp.status(),
        StatusCode::FORBIDDEN,
        "a contributor does not choose it"
    );

    let (status, room) = set_images(&app, ana, &[("kind", "openai"), ("api_key", IMAGE_KEY)]).await;
    assert_eq!(status, StatusCode::OK);
    assert!(!room.contains(IMAGE_KEY), "the key is never on a page");
    assert!(room.contains("…wxyz"), "its end says which one is kept");
    let acl = || std::fs::read_to_string(axgf_cms::acl::Acl::path_for(&dir)).unwrap();
    assert!(acl().contains("\"openai\""));

    // Another service forgets the key unless a new one comes with it.
    let (status, room) = set_images(&app, ana, &[("kind", "gemini")]).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    assert!(!room.contains(IMAGE_KEY));
    assert!(
        acl().contains(IMAGE_KEY),
        "a refused change changes nothing"
    );
    let (status, _) = set_images(
        &app,
        ana,
        &[("kind", "gemini"), ("api_key", "AIza-fake-gemini-key-42")],
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert!(!acl().contains(IMAGE_KEY) && acl().contains("AIza-fake-gemini-key-42"));

    // None forgets it all.
    let (status, room) = set_images(&app, ana, &[("kind", "off")]).await;
    assert_eq!(status, StatusCode::OK);
    assert!(!acl().contains("AIza-fake-gemini-key-42"));
    assert!(room.contains("id=\"images\""));
}

// ---------------------------------------------------------------------------
// reading
// ---------------------------------------------------------------------------

#[tokio::test]
async fn a_document_read_is_a_note_kept_up_in_media_and_read_on_the_record() {
    let (app, _dir, cookies) = family("afg-record-read").await;
    let ana = &cookies[0];
    let (text, seen) = fake(ollama);
    // Without the AFG's computing, the door says so.
    let did = attach(
        &app,
        ana,
        "lettre-1932.png",
        png(32, 24, 40),
        "Lettre de Lucien",
    )
    .await;
    let uri = format!("/admin/person/{LUC}/document/{did}/ai-read");
    let resp = post_form_as(&app, ana, &uri, "").await;
    assert_eq!(resp.status(), StatusCode::CONFLICT);

    set_ollama(&app, ana, &text).await;
    let resp = post_form_as(&app, ana, &uri, "").await;
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    assert_eq!(
        location(&resp),
        format!("/person/{LUC}?tab=media#media-notes")
    );
    {
        let sent = seen.lock().unwrap();
        let (_, path, body) = sent.last().unwrap();
        assert_eq!(path, "/api/chat");
        assert!(
            body.contains("Transcribe every word") && body.contains("\"images\":["),
            "the picture goes with the words"
        );
        assert!(
            !body.contains("Lucien Fabre"),
            "the document alone: no record, no archive"
        );
    }

    let docs = documents(&app, ana).await;
    let notes = tagged(&docs, "transcription");
    assert_eq!(notes.len(), 1);
    let (nid, note) = notes[0];
    assert!(note["tags"].as_array().unwrap().iter().any(|t| t == "note"));
    assert_eq!(note["caption"], "Transcription — Lettre de Lucien");
    assert_eq!(note["linked_to"][0]["entity_id"], LUC);
    assert_eq!(
        note["extensions"]["axgf-cms:transcription/v1"]["document"],
        did.as_str()
    );
    assert_eq!(
        note["extensions"]["axgf-cms:transcription/v1"]["generator"],
        "AFG ollama fausse-famille:1b"
    );
    let shown = page(&app, &format!("/notes/{nid}"), ana).await;
    assert!(shown.contains("Nous sommes bien arrivés à Lyon."));
    assert!(
        shown.contains(&format!("href=\"/document/{did}/view\"")),
        "the way to the document read"
    );

    // Kept up on the Médias tab, coming back there.
    let media = page(&app, &format!("/person/{LUC}?tab=media"), ana).await;
    assert!(media.contains("id=\"media-notes\""));
    let back = enc(&format!("/person/{LUC}?tab=media#media-notes"));
    assert!(media.contains(&format!("/admin/notes/{nid}/edit?back={back}")));
    assert!(media.contains(&format!("action=\"/admin/notes/{nid}/delete\"")));
    // Read on the Fiche, nothing to change there.
    let record = page(&app, &format!("/person/{LUC}"), ana).await;
    assert!(
        record.contains("id=\"person-notes\"")
            && record.contains("Transcription — Lettre de Lucien")
    );
    assert!(!record.contains(&format!("/admin/notes/{nid}/edit")) && !record.contains("/delete\""));
    assert!(
        record.contains("?tab=media#media-notes"),
        "the way to keep them up"
    );
    assert!(record.contains("data-notes-read"), "its window says so");
    // Its window, opened from the Fiche, only reads; opened from the Médias
    // tab, its edit comes back there.
    let window = page(&app, &format!("/notes/{nid}?frame=1&read=1"), ana).await;
    assert!(window.contains("Nous sommes bien arrivés") && !window.contains("/edit"));
    let window = page(&app, &format!("/notes/{nid}?frame=1&back={back}"), ana).await;
    assert!(
        window.contains(&format!("/admin/notes/{nid}/edit?back={back}")),
        "{window}"
    );
    let window = page(
        &app,
        &format!(
            "/notes/{nid}?frame=1&back={}",
            enc("https://ailleurs.example/")
        ),
        ana,
    )
    .await;
    assert!(
        window.contains(&format!("/admin/notes/{nid}/edit\"")),
        "a way out is not followed"
    );

    // Deleted from the Médias tab, back there; a way out of the site is not.
    let resp = post_form_as(
        &app,
        ana,
        &format!("/admin/notes/{nid}/delete"),
        &form(&[("base_version", "1"), ("back", "https://ailleurs.example/")]),
    )
    .await;
    assert_eq!(location(&resp), "/admin/notes?deleted=1");
    // Read once more, and deleted the Médias tab's way.
    let resp = post_form_as(&app, ana, &uri, "").await;
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    let docs = documents(&app, ana).await;
    let (nid, _) = tagged(&docs, "transcription")[0];
    let resp = post_form_as(
        &app,
        ana,
        &format!("/admin/notes/{nid}/delete"),
        &form(&[
            ("base_version", "1"),
            ("back", &format!("/person/{LUC}?tab=media#media-notes")),
        ]),
    )
    .await;
    assert_eq!(
        location(&resp),
        format!("/person/{LUC}?tab=media#media-notes")
    );
    assert!(tagged(&documents(&app, ana).await, "transcription").is_empty());
}

#[tokio::test]
async fn a_document_without_text_writes_nothing_and_a_pdf_needs_computing_that_reads_one() {
    let (app, _dir, cookies) = family("afg-record-nothing").await;
    let ana = &cookies[0];
    let (text, _) = fake(ollama_reads_nothing);
    set_ollama(&app, ana, &text).await;
    let did = attach(&app, ana, "paysage.png", png(16, 16, 200), "").await;
    let resp = post_form_as(
        &app,
        ana,
        &format!("/admin/person/{LUC}/document/{did}/ai-read"),
        "",
    )
    .await;
    assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);
    assert!(tagged(&documents(&app, ana).await, "note").is_empty());

    // Ollama is given a PDF's name only: it is not sent one to read.
    let pdf = b"%PDF-1.4\n1 0 obj << /Type /Catalog >> endobj\ntrailer << /Root 1 0 R >>\n%%EOF\n"
        .to_vec();
    let pid = attach(&app, ana, "acte.pdf", pdf, "Acte de naissance").await;
    let media = page(&app, &format!("/person/{LUC}?tab=media"), ana).await;
    assert!(
        media.contains(&format!("/document/{pid}/ai-read")),
        "a PDF is read from its row"
    );
    assert!(
        !media.contains(&format!("/document/{pid}/ai-retouch")),
        "and never retouched"
    );
    let resp = post_form_as(
        &app,
        ana,
        &format!("/admin/person/{LUC}/document/{pid}/ai-read"),
        "",
    )
    .await;
    assert_eq!(resp.status(), StatusCode::UNSUPPORTED_MEDIA_TYPE);
    let resp = post_form_as(
        &app,
        ana,
        &format!("/admin/person/{LUC}/document/{pid}/ai-retouch"),
        "",
    )
    .await;
    assert!(!resp.status().is_redirection());
}

// ---------------------------------------------------------------------------
// the summary
// ---------------------------------------------------------------------------

#[tokio::test]
async fn the_summary_is_written_in_the_language_asked_shown_first_and_rewritten_in_place() {
    let (app, _dir, cookies) = family("afg-record-summary").await;
    let (ana, dora) = (&cookies[0], &cookies[1]);
    let (text, seen) = fake(ollama);
    set_ollama(&app, ana, &text).await;
    // A note about Lucien, which the summary reads.
    let resp = post_form_as(
        &app,
        dora,
        "/admin/notes",
        &form(&[
            ("title", "Souvenir"),
            ("person", LUC),
            ("markdown", "Il était menuisier à Lyon."),
        ]),
    )
    .await;
    assert!(resp.status().is_redirection());
    let did = attach(&app, ana, "photo-secrete.png", png(8, 8, 1), "Une photo").await;

    let resp = post_form_as(
        &app,
        dora,
        &format!("/admin/person/{LUC}/summary/ai"),
        "lang=pl",
    )
    .await;
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    assert_eq!(location(&resp), format!("/person/{LUC}#summary"));
    {
        let sent = seen.lock().unwrap();
        let (_, _, body) = sent.last().unwrap();
        assert!(body.contains("Write it in Polish"));
        assert!(body.contains("Lucien Fabre") && body.contains("Il était menuisier à Lyon."));
        assert!(
            !body.contains(&did) && !body.contains("photo-secrete"),
            "the files are left out"
        );
    }
    let docs = documents(&app, ana).await;
    let summaries = tagged(&docs, "person-summary");
    assert_eq!(summaries.len(), 1);
    let (sid, s) = summaries[0];
    let sid = sid.clone();
    assert_eq!(
        s["caption"], "Podsumowanie — Lucien Fabre",
        "titled in its own language"
    );
    assert_eq!(s["extensions"]["axgf-cms:summary/v1"]["person"], LUC);
    assert_eq!(s["extensions"]["axgf-cms:summary/v1"]["language"], "pl");
    assert_eq!(
        s["extensions"]["axgf-cms:summary/v1"]["generator"],
        "AFG ollama fausse-famille:1b"
    );

    // First on the record, above Identity, in its language; not among the notes.
    let record = page(&app, &format!("/person/{LUC}"), ana).await;
    let (at_summary, at_identity) = (
        record.find("id=\"summary\"").unwrap(),
        record.find("id=\"identity\"").unwrap(),
    );
    assert!(at_summary < at_identity, "above Identity");
    assert!(record.contains(SUMMARY_PL) && record.contains("lang=\"pl\""));
    assert_eq!(
        record.matches("Podsumowanie — Lucien Fabre").count(),
        0,
        "not listed as a note"
    );
    let media = page(&app, &format!("/person/{LUC}?tab=media"), ana).await;
    assert!(!media.contains(&sid), "nor kept up among them");
    // A visitor reads no note, and so no summary.
    let visitor = body_string(get(&app, &format!("/person/{LUC}")).await).await;
    assert!(!visitor.contains(SUMMARY_PL) && !visitor.contains("id=\"summary\""));

    // Asked again, in French: the same note, rewritten.
    let resp = post_form_as(
        &app,
        dora,
        &format!("/admin/person/{LUC}/summary/ai"),
        "lang=fr",
    )
    .await;
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    let docs = documents(&app, ana).await;
    let summaries = tagged(&docs, "person-summary");
    assert_eq!(summaries.len(), 1, "one summary");
    assert_eq!(summaries[0].0, &sid);
    assert_eq!(
        summaries[0].1["extensions"]["axgf-cms:summary/v1"]["language"],
        "fr"
    );
    assert!(page(&app, &format!("/person/{LUC}"), ana)
        .await
        .contains(SUMMARY_FR));

    // Rewritten by hand, it is the writer's: no longer the AI's.
    let version = summaries[0].1["version_num"]
        .as_u64()
        .unwrap_or(2)
        .to_string();
    let edit = page(
        &app,
        &format!(
            "/admin/notes/{sid}/edit?back={}",
            enc(&format!("/person/{LUC}#summary"))
        ),
        dora,
    )
    .await;
    assert!(
        edit.contains(&format!(
            "name=\"back\" value=\"{}\"",
            attr(&format!("/person/{LUC}#summary"))
        )),
        "the editor goes back to the record"
    );
    let resp = post_form_as(
        &app,
        dora,
        &format!("/admin/notes/{sid}"),
        &form(&[
            ("base_version", &version),
            ("title", "Résumé — Lucien Fabre"),
            ("person", LUC),
            ("markdown", "Lucien, menuisier, aimait la Saône."),
            ("back", &format!("/person/{LUC}#summary")),
        ]),
    )
    .await;
    assert_eq!(location(&resp), format!("/person/{LUC}#summary"));
    let docs = documents(&app, ana).await;
    let s = &docs[&sid];
    assert!(s["extensions"]["axgf-cms:summary/v1"]
        .get("generator")
        .is_none());
    assert_eq!(s["extensions"]["axgf-cms:summary/v1"]["language"], "fr");
    let record = page(&app, &format!("/person/{LUC}"), ana).await;
    assert!(record.contains("aimait la Saône") && !record.contains("summary-by"));

    // carl wrote none of it, and may not have the AFG replace it.
    let resp = post_form_as(
        &app,
        &cookies[2],
        &format!("/admin/person/{LUC}/summary/ai"),
        "lang=fr",
    )
    .await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn a_summary_is_written_by_hand_from_the_record() {
    let (app, _dir, cookies) = family("afg-record-by-hand").await;
    let carl = &cookies[2];
    let back = format!("/person/{ALI}#summary");
    let new = page(
        &app,
        &format!(
            "/admin/notes/new?person={ALI}&kind=summary&back={}",
            enc(&back)
        ),
        carl,
    )
    .await;
    assert!(
        new.contains("value=\"Résumé — Alice Fabre\""),
        "titled for her"
    );
    assert!(new.contains("name=\"kind\" value=\"summary\""));
    assert!(new.contains(&format!("name=\"back\" value=\"{}\"", attr(&back))));
    let resp = post_form_as(
        &app,
        carl,
        "/admin/notes",
        &form(&[
            ("title", "Résumé — Alice Fabre"),
            ("category", "Résumé"),
            ("person", ALI),
            ("markdown", "Alice tenait l'épicerie du village."),
            ("kind", "summary"),
            ("back", &back),
        ]),
    )
    .await;
    assert_eq!(location(&resp), back);
    let docs = documents(&app, &cookies[0]).await;
    let summaries = tagged(&docs, "person-summary");
    assert_eq!(summaries.len(), 1);
    assert_eq!(
        summaries[0].1["extensions"]["axgf-cms:summary/v1"]["person"],
        ALI
    );
    assert!(summaries[0].1["extensions"]["axgf-cms:summary/v1"]
        .get("generator")
        .is_none());
    let record = page(&app, &format!("/person/{ALI}"), carl).await;
    assert!(
        record.contains("Alice tenait l'épicerie du village.")
            || record.contains("Alice tenait l&#x27;épicerie du village.")
    );
    // A back that is not one of the site's records is not followed.
    let resp = post_form_as(
        &app,
        carl,
        "/admin/notes",
        &form(&[
            ("title", "Ailleurs"),
            ("person", ALI),
            ("markdown", "x"),
            ("back", "//ailleurs.example/"),
        ]),
    )
    .await;
    assert!(location(&resp).starts_with("/notes/"));
}

#[tokio::test]
async fn the_media_tab_is_there_for_notes_alone() {
    let (app, _dir, cookies) = family("afg-record-tab").await;
    let ana = &cookies[0];
    let record = page(&app, &format!("/person/{ALI}"), ana).await;
    assert!(
        !tab_hrefs(&record).iter().any(|h| h.contains("tab=media")),
        "nothing in it yet"
    );
    let resp = post_form_as(
        &app,
        ana,
        "/admin/notes",
        &form(&[
            ("title", "Souvenir d'Alice"),
            ("person", ALI),
            ("markdown", "Elle chantait."),
        ]),
    )
    .await;
    assert!(resp.status().is_redirection());
    let record = page(&app, &format!("/person/{ALI}"), ana).await;
    assert!(
        tab_hrefs(&record).iter().any(|h| h.contains("tab=media")),
        "a note is kept up there"
    );
    let media = page(&app, &format!("/person/{ALI}?tab=media"), ana).await;
    assert!(media.contains("Souvenir d") && media.contains("id=\"media-notes\""));
    // A visitor reads no note: no tab for them.
    let visitor = body_string(get(&app, &format!("/person/{ALI}")).await).await;
    assert!(!tab_hrefs(&visitor).iter().any(|h| h.contains("tab=media")));
}
