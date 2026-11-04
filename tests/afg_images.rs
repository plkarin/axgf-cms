//! Where the AFG's pictures are made (docs/adr/0030), beyond OpenAI and
//! Gemini: Canva's AI through the account an administrator connects once,
//! and the family's own servers (Ollama, any OpenAI-compatible server,
//! ComfyUI running the administrator's workflow).
//!
//! A fake stands in for every one: a local HTTP server that answers as
//! Canva's authorization server and MCP server do, one that answers as
//! ComfyUI does, one that answers as an OpenAI-compatible server does.
//! Nothing leaves the machine.

mod common;

use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpListener;
use std::sync::{Arc, Mutex};

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use common::*;
use serde_json::{json, Value};
use tower::ServiceExt;

const PASSWORD: &str = "correct-horse-battery-staple";
const LUC: &str = "c1000000-0000-4000-8000-000000000001";

// ---------------------------------------------------------------------------
// a fake server
// ---------------------------------------------------------------------------

/// One request as the fake received it.
#[derive(Clone, Debug)]
struct Req {
    method: String,
    path: String,
    headers: Vec<(String, String)>,
    body: Vec<u8>,
    base: String,
}

impl Req {
    fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(name))
            .map(|(_, v)| v.as_str())
    }

    fn text(&self) -> String {
        String::from_utf8_lossy(&self.body).into_owned()
    }

    fn json(&self) -> Value {
        serde_json::from_slice(&self.body).unwrap_or(Value::Null)
    }
}

type Reply = (u16, &'static str, Vec<u8>);
type Handler = Arc<dyn Fn(&Req) -> Reply + Send + Sync>;
type Log = Arc<Mutex<Vec<Req>>>;

/// A local HTTP server answering each request with `handler`, keeping what
/// it was sent.
fn serve(handler: Handler) -> (String, Log) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
    let base = format!("http://{}", listener.local_addr().unwrap());
    let log: Log = Arc::default();
    let seen = log.clone();
    let at = base.clone();
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
            let mut headers = Vec::new();
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
                    headers.push((k.trim().to_string(), v.trim().to_string()));
                }
            }
            let mut body = vec![0; length];
            let _ = reader.read_exact(&mut body);
            let req = Req {
                method,
                path,
                headers,
                body,
                base: at.clone(),
            };
            let (status, ctype, reply) = handler(&req);
            seen.lock().unwrap().push(req);
            let _ = write!(
                stream,
                "HTTP/1.1 {status} Fake\r\nContent-Type: {ctype}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                reply.len()
            );
            let _ = stream.write_all(&reply);
        }
    });
    (base, log)
}

fn reply_json(status: u16, v: Value) -> Reply {
    (status, "application/json", v.to_string().into_bytes())
}

/// A picture `w` × `h`: red where `x < rw && y < rh`, white elsewhere.
fn png_with(w: u32, h: u32, rw: u32, rh: u32) -> Vec<u8> {
    let img = image::RgbImage::from_fn(w, h, |x, y| {
        if x < rw && y < rh {
            image::Rgb([200, 30, 30])
        } else {
            image::Rgb([255, 255, 255])
        }
    });
    let mut out = std::io::Cursor::new(Vec::new());
    image::DynamicImage::ImageRgb8(img)
        .write_to(&mut out, image::ImageFormat::Png)
        .unwrap();
    out.into_inner()
}

fn png(w: u32, h: u32) -> Vec<u8> {
    png_with(w, h, w, h)
}

/// The form fields of an urlencoded body.
fn form_of(body: &str) -> Vec<(String, String)> {
    body.split('&')
        .filter_map(|p| p.split_once('='))
        .map(|(k, v)| (k.to_string(), v.replace("%3A", ":").replace("%2F", "/")))
        .collect()
}

// ---------------------------------------------------------------------------
// the fake Canva
// ---------------------------------------------------------------------------

/// What the fake Canva remembers between requests.
#[derive(Default)]
struct CanvaState {
    polled: usize,
    /// Folders made: `search-folders` finds them.
    folders: usize,
    /// Canva holding the account's AI credits back: `create-design` refused.
    cool: bool,
}

/// A tool's answer: structured, or as JSON text only (Canva does both).
fn tool(id: &Value, out: Value, structured: bool) -> Reply {
    let mut result = json!({"content": [{"type": "text", "text": out.to_string()}]});
    if structured {
        result["structuredContent"] = out;
        result["isError"] = json!(false);
    }
    reply_json(200, json!({"jsonrpc": "2.0", "id": id, "result": result}))
}

fn canva_fake() -> (String, Log) {
    canva_fake_with(Arc::default())
}

/// The fake Canva, with its state in the test's hands.
fn canva_fake_with(state: Arc<Mutex<CanvaState>>) -> (String, Log) {
    serve(Arc::new(move |r: &Req| {
        let base = &r.base;
        match (r.method.as_str(), r.path.as_str()) {
            ("GET", "/.well-known/oauth-authorization-server") => reply_json(
                200,
                json!({
                    "authorization_endpoint": format!("{base}/authorize"),
                    "token_endpoint": format!("{base}/token"),
                    "registration_endpoint": format!("{base}/register"),
                }),
            ),
            ("POST", "/register") => reply_json(
                201,
                json!({"client_id": "fake-client", "redirect_uris": r.json()["redirect_uris"]}),
            ),
            ("POST", "/token") => {
                let f = form_of(&r.text());
                let get = |k: &str| {
                    f.iter()
                        .find(|(a, _)| a == k)
                        .map(|(_, v)| v.as_str())
                        .unwrap_or("")
                };
                match get("grant_type") {
                    "authorization_code"
                        if get("code") == "fake:code"
                            && get("client_id") == "fake-client"
                            && !get("code_verifier").is_empty() =>
                    {
                        // A token that is about to expire: the first picture
                        // renews it.
                        reply_json(
                            200,
                            json!({"access_token": "at-1", "refresh_token": "rt-1", "expires_in": 1}),
                        )
                    }
                    "refresh_token" if get("refresh_token") == "rt-1" => reply_json(
                        200,
                        json!({"access_token": "at-2", "refresh_token": "rt-2", "expires_in": 3600}),
                    ),
                    _ => reply_json(400, json!({"error": "invalid_grant"})),
                }
            }
            ("POST", "/upload/once") => reply_json(201, json!({"mediaId": "M-up"})),
            // The page exported 60 wide: its top 60 × 60 is the picture.
            ("GET", "/export/page.png") => (200, "image/png", png_with(60, 80, 60, 60)),
            ("POST", "/mcp") => {
                let auth = r.header("Authorization").unwrap_or("");
                if auth != "Bearer at-1" && auth != "Bearer at-2" {
                    return (401, "application/json", b"{}".to_vec());
                }
                let msg = r.json();
                let id = msg["id"].clone();
                match msg["method"].as_str().unwrap_or("") {
                    "initialize" => reply_json(
                        200,
                        json!({"jsonrpc": "2.0", "id": id, "result": {
                            "protocolVersion": "2025-06-18",
                            "capabilities": {"tools": {}},
                            "serverInfo": {"name": "Fake Canva", "version": "0"}}}),
                    ),
                    "notifications/initialized" => (202, "application/json", Vec::new()),
                    "tools/call" => {
                        let name = msg["params"]["name"].as_str().unwrap_or("");
                        match name {
                            "get-logged-in-account-details" => tool(
                                &id,
                                json!({"id": "u1", "name": "Famille Test", "team_id": "t1"}),
                                true,
                            ),
                            "create-upload-url" => tool(
                                &id,
                                json!({"uploadUrl": format!("{base}/upload/once"), "httpMethod": "POST"}),
                                true,
                            ),
                            "generate-image" => tool(&id, json!({"jobId": "job-1"}), true),
                            "get-generate-image-job" => {
                                let mut st = state.lock().unwrap();
                                st.polled += 1;
                                if st.polled % 2 == 1 {
                                    tool(&id, json!({"status": "PENDING"}), false)
                                } else {
                                    tool(
                                        &id,
                                        json!({"status": "SUCCESS", "result": {"results": [
                                            {"type": "MEDIA", "id": "M-restored", "version": "1"}]}}),
                                        false,
                                    )
                                }
                            }
                            "get-assets" => tool(
                                &id,
                                json!({"asset": {"id": "M-restored", "metadata": {"width": 60, "height": 60}}}),
                                false,
                            ),
                            "create-design" if state.lock().unwrap().cool => tool(
                                &id,
                                json!({"status": "failed", "failure": {"code": "quota_cooldown",
                                                  "message": "Credit quota cooldown is active"}}),
                                true,
                            ),
                            "create-design" => tool(
                                &id,
                                json!({"job_id": "dj1", "status": "pending", "continuation_token": "c1",
                                       "polling_policy": {"wait_seconds": 1}}),
                                true,
                            ),
                            "get-create-design-async-job" => tool(
                                &id,
                                json!({"job_id": "dj1", "status": "completed", "design": {"id": "DBASE"}}),
                                true,
                            ),
                            "start-editing-transaction" => tool(
                                &id,
                                json!({
                                    "transaction": {"status": "open", "transaction_id": "tx1"},
                                    "pages": [{"page_id": "PG", "page_number": 1,
                                               "dimension": {"width": 90, "height": 120},
                                               "is_editable": true}],
                                    "richtexts": [{"element_id": "PG-T1"}],
                                    "fills": [{"element_id": "PG", "type": "image"},
                                              {"element_id": "PG-OLD", "type": "image", "containerElement": {}}],
                                }),
                                true,
                            ),
                            "perform-editing-operations" => {
                                tool(&id, json!({"richtexts": [], "fills": []}), true)
                            }
                            "commit-editing-transaction" => tool(
                                &id,
                                json!({"transaction": {"id": "tx1", "status": "committed"}}),
                                true,
                            ),
                            "export-design" => tool(
                                &id,
                                json!({"job": {"id": "e1", "status": "success",
                                               "urls": [format!("{base}/export/page.png")]}}),
                                true,
                            ),
                            "create-folder" => {
                                state.lock().unwrap().folders += 1;
                                tool(
                                    &id,
                                    json!({"folder": {"id": "F1", "name": "ax-genealogy"}}),
                                    true,
                                )
                            }
                            // Another folder whose name only begins the same.
                            "search-folders" => {
                                let mut items = vec![
                                    json!({"id": "FX", "name": "ax-genealogy old", "created_at": 9}),
                                ];
                                if state.lock().unwrap().folders > 0 {
                                    items.push(json!({"id": "F1", "name": "ax-genealogy", "created_at": 1}));
                                }
                                tool(&id, json!({"items": items}), false)
                            }
                            "move-item-to-folder" => tool(&id, json!({"moved": true}), true),
                            _ => reply_json(
                                200,
                                json!({"jsonrpc": "2.0", "id": id,
                                       "error": {"code": -32602, "message": "no such tool"}}),
                            ),
                        }
                    }
                    _ => reply_json(
                        200,
                        json!({"jsonrpc": "2.0", "id": id,
                               "error": {"code": -32601, "message": "Method not found"}}),
                    ),
                }
            }
            _ => reply_json(404, json!({"error": "no such path"})),
        }
    }))
}

/// The tools the fake Canva was asked for, in order, with their arguments.
fn tools_called(log: &Log) -> Vec<(String, Value)> {
    log.lock()
        .unwrap()
        .iter()
        .filter(|r| r.path == "/mcp")
        .filter_map(|r| {
            let m = r.json();
            (m["method"] == "tools/call").then(|| {
                (
                    m["params"]["name"].as_str().unwrap_or("").to_string(),
                    m["params"]["arguments"].clone(),
                )
            })
        })
        .collect()
}

// ---------------------------------------------------------------------------
// the family
// ---------------------------------------------------------------------------

/// Lucien, with no file yet; ana (an administrator the AFG is open to) and
/// carl (a contributor). Their sessions, reading French.
async fn family(tag: &str) -> (axum::Router, Scratch, Vec<String>) {
    use axgf_cms::acl::{new_user, Acl, Role};
    let dir = scratch(tag);
    let path = dir.join("family.axgf");
    let flat = json!({
        "manifest": {"axgf": "1.1"},
        "persons": {LUC: {"id": LUC, "type": "person", "axgf_version": "1.1",
            "identity": {"name": {"display": "Lucien Fabre", "components": []},
                         "is_living": false, "visibility": "public"}}},
        "families": {}, "events": {}, "links": {}, "occupations": {}, "places": {},
        "sources": {}, "documents": {}
    });
    std::fs::write(
        &path,
        axgf_cms::state::export_to_bytes(&flat.to_string()).expect("export"),
    )
    .expect("write");
    let mut acl = Acl::default();
    for (name, role) in [("ana", Role::Admin), ("carl", Role::Contributor)] {
        let mut u = new_user(name, PASSWORD, role).unwrap();
        u.afg = true;
        acl.users.push(u);
    }
    acl.save(&Acl::path_for(&path)).unwrap();
    let app = build_app(&path).unwrap();
    let mut cookies = Vec::new();
    for name in ["ana", "carl"] {
        let login = post_form(
            &app,
            "/admin/login",
            &format!("username={name}&password={PASSWORD}"),
            false,
        )
        .await;
        assert!(login.status().is_redirection(), "{name} signs in");
        let c = login.headers()[header::SET_COOKIE]
            .to_str()
            .unwrap()
            .split(';')
            .next()
            .unwrap()
            .to_string();
        cookies.push(format!("{c}; axgf_lang=fr"));
    }
    (app, dir.pointing_at(path), cookies)
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

fn form(pairs: &[(&str, &str)]) -> String {
    pairs
        .iter()
        .map(|(k, v)| format!("{k}={}", enc(v)))
        .collect::<Vec<_>>()
        .join("&")
}

async fn post(
    app: &axum::Router,
    cookie: &str,
    uri: &str,
    pairs: &[(&str, &str)],
) -> (StatusCode, String) {
    let resp = post_form_as(app, cookie, uri, &form(pairs)).await;
    let status = resp.status();
    (status, body_string(resp).await)
}

/// A page as a reader sees it: what the templates escape in an attribute,
/// put back.
fn unescape(html: &str) -> String {
    html.replace("&#x2f;", "/").replace("&amp;", "&")
}

fn acl_text(dir: &Scratch) -> String {
    std::fs::read_to_string(axgf_cms::acl::Acl::path_for(dir)).unwrap()
}

/// Attach a photograph to Lucien and give its id.
async fn attach(app: &axum::Router, cookie: &str, bytes: Vec<u8>) -> String {
    let boundary = "----axgfimagesboundary";
    let mut body = Vec::new();
    body.extend_from_slice(
        format!(
            "--{boundary}\r\nContent-Disposition: form-data; name=\"file\"; \
             filename=\"portrait-1931.png\"\r\nContent-Type: application/octet-stream\r\n\r\n"
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
    assert!(resp.status().is_redirection(), "the photograph is attached");
    let docs = documents(app, cookie).await;
    docs.as_object()
        .unwrap()
        .iter()
        .find(|(_, d)| d["filename"] == "portrait-1931.png")
        .map(|(k, _)| k.clone())
        .expect("the photograph")
}

async fn documents(app: &axum::Router, cookie: &str) -> Value {
    let resp = get_with_cookie(app, "/admin/export", cookie).await;
    assert_eq!(resp.status(), StatusCode::OK);
    axgf_rs::import_bundle(&body_bytes(resp).await).data["documents"].clone()
}

/// The restored copies of Lucien's photographs: id, record.
async fn copies(app: &axum::Router, cookie: &str) -> Vec<(String, Value)> {
    let docs = documents(app, cookie).await;
    docs.as_object()
        .unwrap()
        .iter()
        .filter(|(_, d)| axgf_cms::afg_record::is_ai_retouch(d))
        .map(|(k, d)| (k.clone(), d.clone()))
        .collect()
}

/// A stored picture's size.
async fn size_of(app: &axum::Router, cookie: &str, id: &str) -> (u32, u32) {
    let resp = get_with_cookie(app, &format!("/document/{id}/raw"), cookie).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let img = image::load_from_memory(&body_bytes(resp).await).unwrap();
    (img.width(), img.height())
}

// ---------------------------------------------------------------------------
// Canva
// ---------------------------------------------------------------------------

#[tokio::test]
async fn canva_is_connected_once_and_restores_through_one_page_reused() {
    let (app, dir, cookies) = family("afg-images-canva").await;
    let (ana, carl) = (&cookies[0], &cookies[1]);
    let (canva, log) = canva_fake();

    // Chosen first, connected after.
    let (status, room) = post(
        &app,
        ana,
        "/admin/afg/images",
        &[("kind", "canva"), ("url", &canva)],
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert!(room.contains("action=\"/admin/afg/canva/connect\""));
    let (status, _) = post(&app, carl, "/admin/afg/canva/connect", &[]).await;
    assert_eq!(status, StatusCode::FORBIDDEN, "an administrator's");

    // The way to Canva, this site registered with it on the way.
    let (status, room) = post(&app, ana, "/admin/afg/canva/connect", &[]).await;
    assert_eq!(status, StatusCode::OK);
    let room = unescape(&room);
    let at = room
        .find(&format!("{canva}/authorize?"))
        .expect("the way to Canva");
    let link = &room[at..room[at..].find('"').unwrap() + at];
    assert!(link.contains("client_id=fake-client") && link.contains("code_challenge_method=S256"));
    let state = link
        .split("state=")
        .nth(1)
        .unwrap()
        .split(['&', '"'])
        .next()
        .unwrap()
        .to_string();
    {
        let reg = log
            .lock()
            .unwrap()
            .iter()
            .find(|r| r.path == "/register")
            .cloned()
            .unwrap();
        assert_eq!(
            reg.json()["redirect_uris"][0],
            "http://localhost:80/admin/afg/canva/callback"
        );
    }
    // A return from another connection is refused.
    let (status, _) = post(
        &app,
        ana,
        "/admin/afg/canva/finish",
        &[(
            "returned",
            "http://localhost/admin/afg/canva/callback?code=fake%3Acode&state=forged",
        )],
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    let (status, room) = post(
        &app,
        ana,
        "/admin/afg/canva/finish",
        &[(
            "returned",
            &format!("http://localhost/admin/afg/canva/callback?code=fake%3Acode&state={state}"),
        )],
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{}", &room[..room.len().min(2000)]);
    assert!(room.contains("Famille Test"), "the account named");
    assert!(
        !room.contains("at-1") && !room.contains("rt-1"),
        "no token on a page"
    );
    // The page every picture is laid on, made and filed on connecting (1.5.1).
    assert!(room.contains("sa page de travail est prête"), "{room}");
    let kept = acl_text(&dir);
    assert!(kept.contains("DBASE") && kept.contains("\"F1\""));
    // Given a token about to expire, the connection renewed it at once.
    let kept = acl_text(&dir);
    assert!(
        kept.contains("rt-2") && !kept.contains("rt-1"),
        "the renewed token kept"
    );

    // A photograph restored: one page made once, the picture cut out of it.
    let did = attach(&app, ana, png(50, 50)).await;
    let resp = post_form_as(
        &app,
        ana,
        &format!("/admin/person/{LUC}/document/{did}/ai-retouch"),
        "",
    )
    .await;
    assert_eq!(
        resp.status(),
        StatusCode::SEE_OTHER,
        "{}",
        body_string(resp).await
    );
    let made = copies(&app, ana).await;
    assert_eq!(made.len(), 1);
    assert_eq!(
        made[0].1["extensions"]["axgf-cms:ai-retouch/v1"]["generator"],
        "Canva"
    );
    assert_eq!(made[0].1["filename"], "portrait-1931_IA.jpg");
    assert_eq!(
        size_of(&app, ana, &made[0].0).await,
        (60, 60),
        "the picture at its own size"
    );

    let called = tools_called(&log);
    let names: Vec<&str> = called.iter().map(|(n, _)| n.as_str()).collect();
    let at = |n: &str| {
        names
            .iter()
            .position(|x| *x == n)
            .unwrap_or_else(|| panic!("{n} in {names:?}"))
    };
    assert!(
        at("create-design") < at("generate-image"),
        "the page made before the credit is spent"
    );
    let prompt = &called[at("generate-image")].1;
    assert!(prompt["prompt"]
        .as_str()
        .unwrap()
        .contains("Restore this old photograph"));
    assert_eq!(prompt["imageReferences"][0]["id"], "M-up");
    let ops = called[at("perform-editing-operations")].1["operations"]
        .as_array()
        .unwrap()
        .clone();
    let deleted: Vec<&str> = ops
        .iter()
        .filter(|o| o["type"] == "delete_element")
        .map(|o| o["element_id"].as_str().unwrap())
        .collect();
    assert_eq!(
        deleted,
        ["PG-T1", "PG-OLD"],
        "the page emptied, its background kept"
    );
    let laid = ops.iter().find(|o| o["type"] == "insert_fill").unwrap();
    assert_eq!(
        (
            laid["asset_id"].as_str(),
            laid["width"].as_f64(),
            laid["height"].as_f64()
        ),
        (Some("M-restored"), Some(90.0), Some(90.0))
    );
    assert_eq!(called[at("export-design")].1["format"]["width"], 60);
    assert!(called
        .iter()
        .any(|(n, a)| n == "move-item-to-folder" && a["item_id"] == "M-restored"));
    let kept = acl_text(&dir);
    assert!(kept.contains("rt-2"), "the token still kept");
    assert!(kept.contains("DBASE") && kept.contains("\"F1\""));

    // Again: the same page, not another.
    let resp = post_form_as(
        &app,
        ana,
        &format!("/admin/person/{LUC}/document/{did}/ai-retouch"),
        "",
    )
    .await;
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    assert_eq!(
        tools_called(&log)
            .iter()
            .filter(|(n, _)| n == "create-design")
            .count(),
        1
    );

    // Let go: the tokens forgotten, the registration kept, the buttons greyed.
    let (status, room) = post(&app, ana, "/admin/afg/canva/disconnect", &[]).await;
    assert_eq!(status, StatusCode::OK);
    assert!(room.contains("action=\"/admin/afg/canva/connect\""));
    let kept = acl_text(&dir);
    assert!(!kept.contains("rt-2") && kept.contains("fake-client"));
    let resp = post_form_as(
        &app,
        ana,
        &format!("/admin/person/{LUC}/document/{did}/ai-retouch"),
        "",
    )
    .await;
    assert_eq!(resp.status(), StatusCode::CONFLICT);

    // None forgets everything.
    post(&app, ana, "/admin/afg/images", &[("kind", "off")]).await;
    assert!(!acl_text(&dir).contains("fake-client"));
}

#[tokio::test]
async fn canva_holding_its_credits_back_is_said_in_the_readers_language() {
    let (app, dir, cookies) = family("afg-images-canva-cool").await;
    let ana = &cookies[0];
    // A Canva cooling down: every AI call refused in the answer.
    let (canva, log) = serve(Arc::new(|r: &Req| {
        if r.path == "/mcp" {
            let msg = r.json();
            let id = msg["id"].clone();
            return match msg["method"].as_str().unwrap_or("") {
                "initialize" => reply_json(
                    200,
                    json!({"jsonrpc": "2.0", "id": id, "result": {"protocolVersion": "2025-06-18"}}),
                ),
                "notifications/initialized" => (202, "application/json", Vec::new()),
                _ => tool(
                    &id,
                    json!({"status": "failed", "failure": {"code": "quota_cooldown",
                                      "message": "Credit quota cooldown is active"}}),
                    true,
                ),
            };
        }
        reply_json(404, json!({}))
    }));
    post(
        &app,
        ana,
        "/admin/afg/images",
        &[("kind", "canva"), ("url", &canva)],
    )
    .await;
    // Connected by hand, as a finished connection leaves it.
    {
        let path = axgf_cms::acl::Acl::path_for(&dir);
        let mut acl = axgf_cms::acl::Acl::load(&path).unwrap();
        acl.site.afg.images.canva.client_id = "c".into();
        acl.site.afg.images.canva.access_token = "a".into();
        acl.site.afg.images.canva.refresh_token = "r".into();
        acl.site.afg.images.canva.expires_at = u64::MAX / 2;
        acl.save(&path).unwrap();
    }
    drop(app);
    let app = build_app(&dir).unwrap();
    let did = attach(&app, ana, png(20, 20)).await;
    let resp = post_form_as(
        &app,
        ana,
        &format!("/admin/person/{LUC}/document/{did}/ai-retouch"),
        "",
    )
    .await;
    assert_eq!(resp.status(), StatusCode::BAD_GATEWAY);
    let page = body_string(resp).await;
    assert!(
        page.contains("Canva a suspendu les crédits IA de ce compte pour quelques minutes."),
        "{page}"
    );
    assert!(
        !page.contains("Credit quota cooldown"),
        "not in Canva's English"
    );
    let names: Vec<String> = tools_called(&log).into_iter().map(|(n, _)| n).collect();
    assert_eq!(
        names,
        ["create-design"],
        "refused at the page: no picture asked for"
    );
}

/// Connect Canva as an administrator does: the way to it, then its return
/// pasted in the room. The room it ends on.
async fn connect(app: &axum::Router, cookie: &str, canva: &str) -> (StatusCode, String) {
    let (status, room) = post(app, cookie, "/admin/afg/canva/connect", &[]).await;
    assert_eq!(status, StatusCode::OK);
    let room = unescape(&room);
    let at = room
        .find(&format!("{canva}/authorize?"))
        .expect("the way to Canva");
    let state = room[at..]
        .split("state=")
        .nth(1)
        .unwrap()
        .split(['&', '"'])
        .next()
        .unwrap()
        .to_string();
    post(
        app,
        cookie,
        "/admin/afg/canva/finish",
        &[(
            "returned",
            &format!("http://localhost/admin/afg/canva/callback?code=fake%3Acode&state={state}"),
        )],
    )
    .await
}

/// 1.5.1: connected again after "Disconnect Canva", the site files into the
/// folder it had, found by its name, rather than making another.
#[tokio::test]
async fn canva_connected_again_files_into_the_folder_it_had() {
    let (app, dir, cookies) = family("afg-images-canva-again").await;
    let ana = &cookies[0];
    let (canva, log) = canva_fake();
    post(
        &app,
        ana,
        "/admin/afg/images",
        &[("kind", "canva"), ("url", &canva)],
    )
    .await;
    let (status, _) = connect(&app, ana, &canva).await;
    assert_eq!(status, StatusCode::OK);
    let (status, _) = post(&app, ana, "/admin/afg/canva/disconnect", &[]).await;
    assert_eq!(status, StatusCode::OK);
    assert!(
        !acl_text(&dir).contains("\"F1\""),
        "forgotten with the account"
    );
    let (status, room) = connect(&app, ana, &canva).await;
    assert_eq!(status, StatusCode::OK, "{room}");
    let kept = acl_text(&dir);
    assert!(
        kept.contains("\"F1\"") && !kept.contains("\"FX\""),
        "the folder of that name"
    );
    let count = |tool: &str| tools_called(&log).iter().filter(|(n, _)| n == tool).count();
    assert_eq!(
        count("create-folder"),
        1,
        "no second folder of the same name"
    );
    assert_eq!(count("create-design"), 2, "a page for each connection");
}

/// 1.5.1: a Canva that cannot make the page yet still connects, and the
/// first retouch makes it.
#[tokio::test]
async fn canva_holding_its_credits_back_at_connection_still_connects() {
    let (app, dir, cookies) = family("afg-images-canva-cool-connect").await;
    let ana = &cookies[0];
    let state = Arc::new(Mutex::new(CanvaState {
        cool: true,
        ..CanvaState::default()
    }));
    let (canva, log) = canva_fake_with(state.clone());
    post(
        &app,
        ana,
        "/admin/afg/images",
        &[("kind", "canva"), ("url", &canva)],
    )
    .await;
    let (status, room) = connect(&app, ana, &canva).await;
    assert_eq!(status, StatusCode::OK, "{room}");
    assert!(room.contains("Canva est connecté."), "{room}");
    assert!(!room.contains("sa page de travail est prête"));
    let kept = acl_text(&dir);
    assert!(
        kept.contains("rt-2") && !kept.contains("DBASE"),
        "connected, no page yet"
    );

    state.lock().unwrap().cool = false;
    let did = attach(&app, ana, png(50, 50)).await;
    let resp = post_form_as(
        &app,
        ana,
        &format!("/admin/person/{LUC}/document/{did}/ai-retouch"),
        "",
    )
    .await;
    assert_eq!(
        resp.status(),
        StatusCode::SEE_OTHER,
        "{}",
        body_string(resp).await
    );
    assert_eq!(copies(&app, ana).await.len(), 1);
    let names: Vec<String> = tools_called(&log).into_iter().map(|(n, _)| n).collect();
    let made: Vec<usize> = names
        .iter()
        .enumerate()
        .filter(|(_, n)| *n == "create-design")
        .map(|(i, _)| i)
        .collect();
    let asked = names.iter().position(|n| n == "generate-image").unwrap();
    assert_eq!(made.len(), 2, "refused on connecting, made by the retouch");
    assert!(made[1] < asked, "and before the picture is asked for");
    assert!(acl_text(&dir).contains("DBASE"));
}

// ---------------------------------------------------------------------------
// the family's own servers
// ---------------------------------------------------------------------------

const WORKFLOW: &str = r#"{
  "3": {"class_type": "LoadImage", "inputs": {"image": "example.png"}},
  "6": {"class_type": "CLIPTextEncode", "inputs": {"text": "{{restore}}"}},
  "9": {"class_type": "SaveImage", "inputs": {"images": ["8", 0], "filename_prefix": "axgf"}}
}"#;

#[tokio::test]
async fn comfyui_runs_the_administrators_workflow_on_the_photograph() {
    let (app, _dir, cookies) = family("afg-images-comfyui").await;
    let ana = &cookies[0];
    let polls = Arc::new(Mutex::new(0usize));
    let (comfy, log) = serve(Arc::new(move |r: &Req| {
        match (r.method.as_str(), r.path.split('?').next().unwrap_or("")) {
            ("POST", "/upload/image") => {
                let text = r.text();
                let name = text
                    .split("filename=\"")
                    .nth(1)
                    .and_then(|s| s.split('"').next())
                    .unwrap_or("x")
                    .to_string();
                reply_json(200, json!({"name": name, "subfolder": "", "type": "input"}))
            }
            ("POST", "/prompt") => reply_json(
                200,
                json!({"prompt_id": "p1", "number": 1, "node_errors": {}}),
            ),
            ("GET", "/history/p1") => {
                let mut n = polls.lock().unwrap();
                *n += 1;
                if *n == 1 {
                    reply_json(200, json!({}))
                } else {
                    reply_json(
                        200,
                        json!({"p1": {
                        "status": {"status_str": "success", "completed": true},
                        "outputs": {"9": {"images": [{"filename": "axgf_00001_.png", "subfolder": "", "type": "output"}]}}}}),
                    )
                }
            }
            ("GET", "/view") => (200, "image/png", png(40, 50)),
            _ => reply_json(404, json!({})),
        }
    }));
    // A workflow that loads nothing is refused.
    let (status, room) = post(
        &app,
        ana,
        "/admin/afg/images",
        &[
            ("kind", "comfyui"),
            ("url", &comfy),
            (
                "workflow",
                r#"{"1": {"class_type": "SaveImage", "inputs": {}}}"#,
            ),
        ],
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    assert!(room.contains("LoadImage"));
    let (status, _) = post(
        &app,
        ana,
        "/admin/afg/images",
        &[("kind", "comfyui"), ("url", &comfy), ("workflow", WORKFLOW)],
    )
    .await;
    assert_eq!(status, StatusCode::OK);

    let did = attach(&app, ana, png(30, 30)).await;
    let resp = post_form_as(
        &app,
        ana,
        &format!("/admin/person/{LUC}/document/{did}/ai-retouch"),
        "",
    )
    .await;
    assert_eq!(
        resp.status(),
        StatusCode::SEE_OTHER,
        "{}",
        body_string(resp).await
    );
    let made = copies(&app, ana).await;
    assert_eq!(made.len(), 1);
    assert_eq!(
        made[0].1["extensions"]["axgf-cms:ai-retouch/v1"]["generator"],
        "ComfyUI"
    );
    assert_eq!(size_of(&app, ana, &made[0].0).await, (40, 50));

    let seen = log.lock().unwrap().clone();
    let upload = seen
        .iter()
        .find(|r| r.path == "/upload/image")
        .unwrap()
        .text();
    let uploaded = upload
        .split("filename=\"")
        .nth(1)
        .unwrap()
        .split('"')
        .next()
        .unwrap()
        .to_string();
    assert!(uploaded.starts_with("axgf-") && upload.contains("name=\"overwrite\""));
    let queued = seen.iter().find(|r| r.path == "/prompt").unwrap().json();
    assert_eq!(
        queued["prompt"]["3"]["inputs"]["image"],
        uploaded.as_str(),
        "the photograph loaded"
    );
    assert!(queued["prompt"]["6"]["inputs"]["text"]
        .as_str()
        .unwrap()
        .starts_with("Restore this old photograph"));
    let view = seen.iter().find(|r| r.path.starts_with("/view")).unwrap();
    assert!(view.path.contains("filename=axgf_00001_.png") && view.path.contains("type=output"));
}

#[tokio::test]
async fn a_server_of_the_familys_answers_as_openai_does_without_a_key() {
    let (app, dir, cookies) = family("afg-images-compatible").await;
    let ana = &cookies[0];
    let (server, log) = serve(Arc::new(|r: &Req| {
        match (r.method.as_str(), r.path.as_str()) {
            // An address rather than base64, as some servers answer.
            ("POST", "/v1/images/edits") => reply_json(
                200,
                json!({"data": [{"url": format!("{}/out/1.png", r.base)}]}),
            ),
            ("GET", "/out/1.png") => (200, "image/png", png(30, 30)),
            _ => reply_json(404, json!({})),
        }
    }));
    // Its address is its own: there is no usual one.
    let (status, _) = post(
        &app,
        ana,
        "/admin/afg/images",
        &[("kind", "openai-compatible")],
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    let (status, _) = post(
        &app,
        ana,
        "/admin/afg/images",
        &[
            ("kind", "openai-compatible"),
            ("url", &format!("{server}/v1")),
            ("model", "Qwen/Qwen-Image-Edit"),
        ],
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert!(!acl_text(&dir).contains("api_key"), "no key needed");

    let did = attach(&app, ana, png(30, 30)).await;
    let resp = post_form_as(
        &app,
        ana,
        &format!("/admin/person/{LUC}/document/{did}/ai-retouch"),
        "",
    )
    .await;
    assert_eq!(
        resp.status(),
        StatusCode::SEE_OTHER,
        "{}",
        body_string(resp).await
    );
    let made = copies(&app, ana).await;
    assert_eq!(
        made[0].1["extensions"]["axgf-cms:ai-retouch/v1"]["generator"],
        "OpenAI-compatible Qwen/Qwen-Image-Edit"
    );

    let seen = log.lock().unwrap().clone();
    let edit = seen.iter().find(|r| r.path == "/v1/images/edits").unwrap();
    let body = edit.text();
    assert!(
        body.contains("name=\"image\"") && !body.contains("name=\"image[]\""),
        "one picture, as the libraries send it"
    );
    assert!(body.contains("name=\"response_format\"") && body.contains("b64_json"));
    assert!(edit.header("Authorization").is_none(), "no key, no header");

    // Ollama: its usual address and its suggestion, no key.
    let (status, room) = post(&app, ana, "/admin/afg/images", &[("kind", "ollama")]).await;
    assert_eq!(status, StatusCode::OK);
    let room = unescape(&room);
    assert!(room.contains("x/flux2-klein") && room.contains("http://127.0.0.1:11434/v1"));
}
