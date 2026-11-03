//! My personal assistant (docs/adr/0026): each account the AFG is open to
//! has its own projects, each with a code of conduct and a second brain,
//! and in each, conversations with the AFG that carry files and go on where
//! they were left. Nobody reads another account's.
//!
//! A local HTTP server stands in for Ollama, which streams its answers a
//! line at a time, and a shell script named `claude` for Claude Code, which
//! streams its own. Nothing here leaves the machine.

mod common;

use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use common::*;
use serde_json::Value;
use tower::ServiceExt;

const PASSWORD: &str = "correct-horse-battery-staple";

fn sample() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("deploy/sample.axgf")
}

/// What a fake service was sent: method, path and body of each request.
type Seen = Arc<Mutex<Vec<(String, String, String)>>>;

/// A local HTTP server answering each request with `respond`.
fn fake_service(respond: fn(&str, &str, &str) -> (u16, String)) -> (String, Seen) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
    let url = format!("http://{}", listener.local_addr().unwrap());
    let seen: Seen = Arc::default();
    let log = seen.clone();
    std::thread::spawn(move || {
        for stream in listener.incoming() {
            let Ok(mut stream) = stream else { continue };
            let log = log.clone();
            std::thread::spawn(move || {
                let mut reader = BufReader::new(stream.try_clone().expect("clone"));
                let mut first = String::new();
                if reader.read_line(&mut first).is_err() {
                    return;
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
                    "HTTP/1.1 {status} Fake\r\nContent-Type: application/x-ndjson\r\n\
                     Content-Length: {}\r\nConnection: close\r\n\r\n{reply}",
                    reply.len()
                );
            });
        }
    });
    (url, seen)
}

/// An Ollama that streams what it was given, a line at a time, and asks to
/// remember a fact when it is told to.
fn ollama(method: &str, path: &str, body: &str) -> (u16, String) {
    if (method, path) == ("GET", "/api/tags") {
        return (200, r#"{"models":[{"name":"fausse-famille:1b"}]}"#.into());
    }
    let v: Value = serde_json::from_str(body).unwrap_or_default();
    let msgs = v["messages"].as_array().cloned().unwrap_or_default();
    let last = msgs.last().cloned().unwrap_or_default();
    let words = format!(
        "Bien reçu : {} tours, {} images.",
        msgs.len() - 1,
        last["images"].as_array().map_or(0, Vec::len)
    );
    let mut lines: Vec<String> = words
        .split(' ')
        .enumerate()
        .map(|(i, w)| {
            let piece = if i == 0 {
                w.to_string()
            } else {
                format!(" {w}")
            };
            serde_json::json!({"model": "fausse-famille:1b",
                               "message": {"role": "assistant", "content": piece}, "done": false})
            .to_string()
        })
        .collect();
    if last["content"].as_str().unwrap_or("").contains("Souviens") {
        lines.push(
            serde_json::json!({"message": {"content": "\n\n<memory>\n- Mamie adore les roses\n</memory>"},
                               "done": false})
            .to_string(),
        );
    }
    lines.push(
        r#"{"model":"fausse-famille:1b","message":{"content":""},"done":true,"prompt_eval_count":120,"eval_count":9}"#
            .into(),
    );
    (200, lines.join("\n") + "\n")
}

/// An Ollama that takes its time, for an answer still being written.
fn slow_ollama(method: &str, path: &str, body: &str) -> (u16, String) {
    if method == "POST" {
        std::thread::sleep(Duration::from_millis(1500));
    }
    ollama(method, path, body)
}

/// An app over the sample, its AFG an Ollama at `url`, with an account the
/// AFG is open to (vera), another (marie), and one it is not (paul).
async fn app_with_afg(tag: &str, url: &str) -> (axum::Router, Scratch, Vec<String>) {
    use axgf_cms::acl::{new_user, Acl, Role};
    let dir = scratch(tag);
    let path = dir.join("family.axgf");
    std::fs::copy(sample(), &path).unwrap();
    let mut acl = Acl::default();
    for (name, afg) in [("vera", true), ("marie", true), ("paul", false)] {
        let mut u = new_user(name, PASSWORD, Role::Viewer).unwrap();
        u.afg = afg;
        acl.users.push(u);
    }
    acl.save(&Acl::path_for(&path)).unwrap();
    let app = build_app(&path).unwrap();
    let admin = admin_cookie(&app).await;
    let resp = post_form_as(
        &app,
        &admin,
        "/admin/afg",
        &format!(
            "kind=ollama&url={}&timeout=60",
            url.replace(':', "%3A").replace('/', "%2F")
        ),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::OK, "the AFG is set");
    let mut cookies = Vec::new();
    for name in ["vera", "marie", "paul"] {
        cookies.push(sign_in(&app, name).await);
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

fn png() -> Vec<u8> {
    let img = image::RgbImage::from_pixel(64, 48, image::Rgb([180, 140, 90]));
    let mut out = std::io::Cursor::new(Vec::new());
    image::DynamicImage::ImageRgb8(img)
        .write_to(&mut out, image::ImageFormat::Png)
        .unwrap();
    out.into_inner()
}

/// A multipart form: words, and files as (name, bytes).
async fn post_multipart(
    app: &axum::Router,
    cookie: &str,
    uri: &str,
    text: Option<&str>,
    files: &[(&str, &[u8])],
    field: &str,
    json: bool,
) -> axum::http::Response<Body> {
    let b = "----axgfassistant";
    let mut body = Vec::new();
    if let Some(t) = text {
        body.extend(
            format!("--{b}\r\nContent-Disposition: form-data; name=\"text\"\r\n\r\n{t}\r\n")
                .bytes(),
        );
    }
    for (name, bytes) in files {
        body.extend(
            format!("--{b}\r\nContent-Disposition: form-data; name=\"{field}\"; filename=\"{name}\"\r\nContent-Type: application/octet-stream\r\n\r\n")
                .bytes(),
        );
        body.extend_from_slice(bytes);
        body.extend(b"\r\n");
    }
    body.extend(format!("--{b}--\r\n").bytes());
    let mut req = Request::builder()
        .uri(uri)
        .method("POST")
        .header(header::COOKIE, cookie)
        .header(
            header::CONTENT_TYPE,
            format!("multipart/form-data; boundary={b}"),
        );
    if json {
        req = req.header(header::ACCEPT, "application/json");
    }
    app.clone()
        .oneshot(req.body(Body::from(body)).unwrap())
        .await
        .unwrap()
}

/// Make a project, and answer its id.
async fn project(app: &axum::Router, cookie: &str, name: &str, avatar: &str) -> String {
    let resp = post_form_as(
        app,
        cookie,
        "/account/assistant",
        &format!("name={name}&avatar={avatar}"),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::SEE_OTHER, "a project is made");
    let at = resp.headers()[header::LOCATION].to_str().unwrap();
    at.strip_prefix("/account/assistant/").unwrap().to_string()
}

/// Wait until the conversation is answered; its messages' roles.
async fn answered(app: &axum::Router, cookie: &str, pid: &str, cid: &str) -> Value {
    for _ in 0..200 {
        let resp = get_with_cookie(
            app,
            &format!("/account/assistant/{pid}/c/{cid}/state?after=0"),
            cookie,
        )
        .await;
        assert_eq!(resp.status(), StatusCode::OK);
        let v: Value = serde_json::from_str(&body_string(resp).await).unwrap();
        if v["answering"] == false {
            return v;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    panic!("never answered");
}

/// A page, its attributes' slashes as a browser reads them: the templates
/// write `/` as `&#x2f;` in an attribute.
async fn page_of(app: &axum::Router, uri: &str, cookie: &str) -> String {
    body_string(get_with_cookie(app, uri, cookie).await)
        .await
        .replace("&#x2f;", "/")
}

fn roles(state: &Value) -> Vec<String> {
    state["messages"]
        .as_array()
        .unwrap()
        .iter()
        .map(|m| m["role"].as_str().unwrap().to_string())
        .collect()
}

#[tokio::test]
async fn only_an_account_the_afg_is_open_to_has_an_assistant() {
    let (url, _) = fake_service(ollama);
    let (app, _dir, c) = app_with_afg("assistant-door", &url).await;
    let resp = get(&app, "/account/assistant").await;
    assert!(resp.status().is_redirection(), "signed out: to the sign-in");
    let page = body_string(get_admin(&app, "/account/assistant").await).await;
    assert!(
        page.contains("This session has no account"),
        "the emergency token has none"
    );
    let resp = get_with_cookie(&app, "/account/assistant", &c[2]).await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
    assert!(body_string(resp)
        .await
        .contains("The AFG is not open to your account"));
    let page = body_string(get_with_cookie(&app, "/account/assistant", &c[0]).await).await;
    assert!(page.contains("No project yet") && page.contains(r#"id="new-project""#));
    // Its door: on the account page, and in the account's menu.
    let page = body_string(get_with_cookie(&app, "/account", &c[0]).await).await;
    assert!(page.contains(r#"class="as-entry" href="/account/assistant""#));
    let page = body_string(get_with_cookie(&app, "/account", &c[2]).await).await;
    assert!(!page.contains("/account/assistant"), "not for paul");
}

#[tokio::test]
async fn a_conversation_carries_files_follows_the_conduct_and_feeds_the_projects_memory() {
    let (url, seen) = fake_service(ollama);
    let (app, dir, c) = app_with_afg("assistant-talk", &url).await;
    let vera = &c[0];
    let pid = project(&app, vera, "Sant%C3%A9+de+mamie", "medicine-heart").await;
    let page = page_of(&app, &format!("/account/assistant/{pid}"), vera).await;
    assert!(
        page.contains("Santé de mamie") && page.contains("/static/avatars/medicine-heart.webp")
    );
    let resp = post_form_as(
        &app,
        vera,
        &format!("/account/assistant/{pid}"),
        "part=conduct&conduct=R%C3%A9ponds+en+deux+phrases.",
    )
    .await;
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);

    // The first message opens the conversation, its picture with it.
    let resp = post_multipart(
        &app,
        vera,
        &format!("/account/assistant/{pid}/c"),
        Some("Voici une photo de mamie."),
        &[("mamie.png", &png())],
        "files",
        true,
    )
    .await;
    assert_eq!(resp.status(), StatusCode::OK);
    let v: Value = serde_json::from_str(&body_string(resp).await).unwrap();
    let to = v["redirect"].as_str().unwrap().to_string();
    let cid = to.rsplit('/').next().unwrap().to_string();
    let state = answered(&app, vera, &pid, &cid).await;
    assert_eq!(roles(&state), ["user", "assistant"]);
    let html = state["messages"][1]["html"].as_str().unwrap();
    assert!(html.contains("Bien reçu : 1 tours, 1 images."), "{html}");
    let sent: Vec<Value> = seen
        .lock()
        .unwrap()
        .iter()
        .filter(|(m, _, _)| m == "POST")
        .map(|(_, _, b)| serde_json::from_str(b).unwrap())
        .collect();
    assert_eq!(sent[0]["stream"], true, "the answer is streamed");
    let system = sent[0]["messages"][0]["content"].as_str().unwrap();
    assert!(system.contains("<code-of-conduct>\nRéponds en deux phrases.\n</code-of-conduct>"));
    assert!(system.contains("You are talking to vera"));
    assert!(
        system.contains("does not go with this project"),
        "no archive unless asked"
    );
    assert_eq!(
        sent[0]["messages"][1]["images"].as_array().unwrap().len(),
        1
    );

    // The page shows the picture, served to vera alone.
    let page = page_of(&app, &to, vera).await;
    let f = page.split("/f/").nth(1).unwrap()[..16].to_string();
    let resp = get_with_cookie(&app, &format!("/account/assistant/{pid}/f/{f}"), vera).await;
    assert_eq!(resp.status(), StatusCode::OK);
    assert_eq!(resp.headers()[header::CONTENT_TYPE], "image/png");
    assert_eq!(body_bytes(resp).await, png());

    // Asked to remember: kept in the project's second brain, not shown.
    let resp = post_multipart(
        &app,
        vera,
        &format!("/account/assistant/{pid}/c/{cid}"),
        Some("Souviens-toi : elle adore les roses."),
        &[],
        "files",
        true,
    )
    .await;
    assert_eq!(resp.status(), StatusCode::OK);
    let state = answered(&app, vera, &pid, &cid).await;
    assert_eq!(roles(&state), ["user", "assistant", "user", "assistant"]);
    let last = state["messages"][3]["html"].as_str().unwrap();
    assert!(
        !last.contains("memory>") && last.contains("as-kept"),
        "{last}"
    );
    let sent = seen.lock().unwrap().last().unwrap().2.clone();
    let sent: Value = serde_json::from_str(&sent).unwrap();
    assert_eq!(
        sent["messages"].as_array().unwrap().len(),
        4,
        "the conversation goes back with it"
    );
    let page = page_of(&app, &format!("/account/assistant/{pid}"), vera).await;
    assert!(page.contains("Mamie adore les roses"));
    let fact = page.split(r#"name="fact" value=""#).nth(1).unwrap()[..16].to_string();
    // The next message takes the memory with it.
    post_multipart(
        &app,
        vera,
        &format!("/account/assistant/{pid}/c/{cid}"),
        Some("Et maintenant ?"),
        &[],
        "files",
        true,
    )
    .await;
    answered(&app, vera, &pid, &cid).await;
    let sent = seen.lock().unwrap().last().unwrap().2.clone();
    assert!(
        sent.contains("Mamie adore les roses"),
        "the memory goes back to the AFG"
    );
    // Forgotten on request.
    let resp = post_form_as(
        &app,
        vera,
        &format!("/account/assistant/{pid}/memory/forget"),
        &format!("fact={fact}"),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    let page = page_of(&app, &format!("/account/assistant/{pid}"), vera).await;
    assert!(!page.contains("Mamie adore les roses"), "forgotten");
    // Files on disk are the account's alone.
    let mode = |p: &Path| {
        use std::os::unix::fs::PermissionsExt;
        std::fs::metadata(p).unwrap().permissions().mode() & 0o777
    };
    let space = dir.with_extension("assistant").join("vera");
    assert_eq!(mode(&space), 0o700);
    let files = space.join("projects").join(&pid).join("files");
    for e in std::fs::read_dir(files).unwrap().flatten() {
        assert_eq!(mode(&e.path()), 0o600, "{}", e.path().display());
    }
}

#[tokio::test]
async fn nobody_reads_another_accounts_space() {
    let (url, _) = fake_service(ollama);
    let (app, _dir, c) = app_with_afg("assistant-walls", &url).await;
    let pid = project(&app, &c[0], "Secret", "").await;
    let resp = post_multipart(
        &app,
        &c[0],
        &format!("/account/assistant/{pid}/c"),
        Some("Un secret"),
        &[("a.png", &png())],
        "files",
        false,
    )
    .await;
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    let to = resp.headers()[header::LOCATION]
        .to_str()
        .unwrap()
        .to_string();
    let page = page_of(&app, &to, &c[0]).await;
    let f = page.split("/f/").nth(1).unwrap()[..16].to_string();
    for uri in [
        format!("/account/assistant/{pid}"),
        to.clone(),
        format!("/account/assistant/{pid}/f/{f}"),
    ] {
        let resp = get_with_cookie(&app, &uri, &c[1]).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND, "marie, {uri}");
    }
    let resp = get_with_cookie(&app, &format!("{to}/state"), &c[1]).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    let resp = post_multipart(&app, &c[1], &to, Some("Intrusion"), &[], "files", true).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    // Nor does the emergency session, which has no account, and an address
    // that is not an id is none.
    let page = body_string(
        get_with_cookie(
            &app,
            &format!("/account/assistant/{pid}"),
            &admin_cookie(&app).await,
        )
        .await,
    )
    .await;
    assert!(!page.contains("Secret") && page.contains("This session has no account"));
    let resp = get_with_cookie(&app, "/account/assistant/..%2F..%2Ffamily.acl", &c[0]).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn a_message_is_refused_when_empty_or_too_full_or_while_the_afg_writes() {
    let (url, _) = fake_service(slow_ollama);
    let (app, _dir, c) = app_with_afg("assistant-refusals", &url).await;
    let vera = &c[0];
    let pid = project(&app, vera, "Essais", "").await;
    let resp = post_multipart(
        &app,
        vera,
        &format!("/account/assistant/{pid}/c"),
        Some("  "),
        &[],
        "files",
        true,
    )
    .await;
    assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);
    assert!(body_string(resp)
        .await
        .contains("Write a message or attach a file"));
    let p = png();
    let many: Vec<(&str, &[u8])> = (0..7).map(|_| ("p.png", p.as_slice())).collect();
    let resp = post_multipart(
        &app,
        vera,
        &format!("/account/assistant/{pid}/c"),
        Some("Trop"),
        &many,
        "files",
        true,
    )
    .await;
    assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);
    assert!(body_string(resp).await.contains("Too many files"));
    let resp = post_multipart(
        &app,
        vera,
        &format!("/account/assistant/{pid}/c"),
        Some("Bonjour"),
        &[],
        "files",
        true,
    )
    .await;
    let v: Value = serde_json::from_str(&body_string(resp).await).unwrap();
    let to = v["redirect"].as_str().unwrap().to_string();
    // While the AFG writes, the conversation takes nothing more.
    let resp = post_multipart(&app, vera, &to, Some("Encore"), &[], "files", true).await;
    assert_eq!(resp.status(), StatusCode::CONFLICT);
    let page = page_of(&app, &to, vera).await;
    assert!(
        page.contains("data-live") && page.contains(r#"http-equiv="refresh""#),
        "without scripting, asked again"
    );
    let cid = to.rsplit('/').next().unwrap();
    let state = answered(&app, vera, &pid, cid).await;
    assert_eq!(roles(&state), ["user", "assistant"]);
}

#[tokio::test]
async fn a_failed_answer_is_said_in_the_thread_and_asked_again() {
    // Nothing listens there.
    let (app, _dir, c) = app_with_afg("assistant-failure", "http://127.0.0.1:9").await;
    let vera = &c[0];
    let pid = project(&app, vera, "Panne", "").await;
    let resp = post_multipart(
        &app,
        vera,
        &format!("/account/assistant/{pid}/c"),
        Some("Allô ?"),
        &[],
        "files",
        true,
    )
    .await;
    let v: Value = serde_json::from_str(&body_string(resp).await).unwrap();
    let to = v["redirect"].as_str().unwrap().to_string();
    let cid = to.rsplit('/').next().unwrap().to_string();
    let state = answered(&app, vera, &pid, &cid).await;
    assert_eq!(roles(&state), ["user", "error"]);
    let html = state["messages"][1]["html"].as_str().unwrap();
    assert!(
        html.contains("The AFG could not answer") && html.contains("/retry"),
        "{html}"
    );
    let resp = post_form_as(&app, vera, &format!("{to}/retry"), "").await;
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    let state = answered(&app, vera, &pid, &cid).await;
    assert_eq!(
        roles(&state),
        ["user", "error", "error"],
        "asked again, failed again"
    );
}

#[tokio::test]
async fn a_message_left_without_an_answer_is_offered_again_and_an_answered_one_is_not() {
    let (url, _) = fake_service(ollama);
    let (app, dir, c) = app_with_afg("assistant-orphan", &url).await;
    let vera = &c[0];
    let pid = project(&app, vera, "Reprise", "").await;
    let resp = post_multipart(
        &app,
        vera,
        &format!("/account/assistant/{pid}/c"),
        Some("Première question"),
        &[],
        "files",
        true,
    )
    .await;
    let v: Value = serde_json::from_str(&body_string(resp).await).unwrap();
    let to = v["redirect"].as_str().unwrap().to_string();
    let cid = to.rsplit('/').next().unwrap().to_string();
    let state = answered(&app, vera, &pid, &cid).await;
    assert_eq!(roles(&state), ["user", "assistant"]);

    // An answered message is not asked again.
    let resp = post_form_as(&app, vera, &format!("{to}/retry"), "").await;
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    let state = answered(&app, vera, &pid, &cid).await;
    assert_eq!(roles(&state), ["user", "assistant"], "nothing asked again");
    assert!(!page_of(&app, &to, vera).await.contains("as-orphan"));

    // The site stopped while the AFG wrote: a message with no answer.
    let thread = dir
        .with_extension("assistant")
        .join("vera")
        .join("projects")
        .join(&pid)
        .join("conversations")
        .join(format!("{cid}.jsonl"));
    let first = std::fs::read_to_string(&thread).unwrap();
    let mut orphan: Value = serde_json::from_str(first.lines().next().unwrap()).unwrap();
    orphan["id"] = "00000000000000ff".into();
    orphan["text"] = "Seconde question".into();
    let mut f = std::fs::OpenOptions::new()
        .append(true)
        .open(&thread)
        .unwrap();
    writeln!(f, "{orphan}").unwrap();
    drop(f);
    let page = page_of(&app, &to, vera).await;
    assert!(
        page.contains("as-orphan") && page.contains(&format!("{to}/retry")),
        "{page}"
    );
    let resp = post_form_as(&app, vera, &format!("{to}/retry"), "").await;
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    let state = answered(&app, vera, &pid, &cid).await;
    assert_eq!(roles(&state), ["user", "assistant", "user", "assistant"]);
    assert!(!page_of(&app, &to, vera).await.contains("as-orphan"));
}

#[tokio::test]
async fn avatars_are_the_packs_or_a_picture_of_ones_own_and_a_project_goes_whole() {
    let (url, _) = fake_service(ollama);
    let (app, dir, c) = app_with_afg("assistant-avatars", &url).await;
    let vera = &c[0];
    let resp = get(&app, "/static/avatars/genealogy-tree.webp").await;
    assert_eq!(resp.status(), StatusCode::OK);
    assert_eq!(resp.headers()[header::CONTENT_TYPE], "image/webp");
    assert_eq!(
        get(&app, "/static/avatars/nope.webp").await.status(),
        StatusCode::NOT_FOUND
    );
    let pid = project(&app, vera, "Album", "not-an-avatar").await;
    let page = page_of(&app, &format!("/account/assistant/{pid}"), vera).await;
    assert!(page.contains("/static/avatars/"), "the pack's, by its id");
    let resp = post_multipart(
        &app,
        vera,
        &format!("/account/assistant/{pid}/avatar"),
        None,
        &[("moi.png", &png())],
        "picture",
        false,
    )
    .await;
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    let resp = get_with_cookie(&app, &format!("/account/assistant/{pid}/avatar"), vera).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let img = image::load_from_memory(&body_bytes(resp).await).unwrap();
    assert_eq!((img.width(), img.height()), (256, 256));
    let resp = post_multipart(
        &app,
        vera,
        &format!("/account/assistant/{pid}/avatar"),
        None,
        &[("x.png", b"not a picture")],
        "picture",
        false,
    )
    .await;
    assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);
    // Deleted only once confirmed, and then all of it.
    let resp = post_form_as(&app, vera, &format!("/account/assistant/{pid}/delete"), "").await;
    assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);
    let resp = post_form_as(
        &app,
        vera,
        &format!("/account/assistant/{pid}/delete"),
        "confirm=on",
    )
    .await;
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    assert_eq!(
        get_with_cookie(&app, &format!("/account/assistant/{pid}"), vera)
            .await
            .status(),
        StatusCode::NOT_FOUND
    );
    let projects = dir
        .with_extension("assistant")
        .join("vera")
        .join("projects");
    assert_eq!(std::fs::read_dir(projects).unwrap().count(), 0);
}

/// Claude Code in conversation: the picture goes with the words as content
/// blocks of one message on its input, and its answer is read as it streams.
#[tokio::test]
async fn claude_code_streams_its_answer_and_reads_the_picture() {
    let dir = scratch("assistant-claude");
    let bin = dir.join("bin");
    std::fs::create_dir_all(&bin).unwrap();
    let program = bin.join("claude");
    let d = dir.display();
    std::fs::write(
        &program,
        format!(
            r#"#!/bin/sh
if [ "$1" = "--help" ]; then
  printf '  --include-partial-messages  Include\n  --input-format <format>\n  --verbose\n'
  exit 0
fi
printf '%s\n' "$@" > "{d}/args"
cat > "{d}/stdin"
printf '%s\n' '{{"type":"system","subtype":"init"}}'
printf '%s\n' '{{"type":"stream_event","event":{{"type":"content_block_delta","index":0,"delta":{{"type":"text_delta","text":"Une "}}}}}}'
printf '%s\n' '{{"type":"stream_event","event":{{"type":"content_block_delta","index":0,"delta":{{"type":"text_delta","text":"photo."}}}}}}'
printf '%s\n' '{{"type":"result","subtype":"success","is_error":false,"result":"Une photo.","total_cost_usd":0.002,"usage":{{"input_tokens":50,"output_tokens":3}}}}'
"#
        ),
    )
    .unwrap();
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&program, std::fs::Permissions::from_mode(0o755)).unwrap();
    }
    let path = dir.join("family.axgf");
    std::fs::copy(sample(), &path).unwrap();
    let mut acl = axgf_cms::acl::Acl::default();
    let mut u = axgf_cms::acl::new_user("vera", PASSWORD, axgf_cms::acl::Role::Viewer).unwrap();
    u.afg = true;
    acl.users.push(u);
    acl.save(&axgf_cms::acl::Acl::path_for(&path)).unwrap();
    let app = build_app(&path).unwrap();
    let admin = admin_cookie(&app).await;
    let form = format!(
        "kind=claude-code&model=claude-opus-5-5&effort=low&program={}",
        program.display().to_string().replace('/', "%2F")
    );
    assert_eq!(
        post_form_as(&app, &admin, "/admin/afg", &form)
            .await
            .status(),
        StatusCode::OK
    );
    let vera = sign_in(&app, "vera").await;
    let pid = project(&app, &vera, "Photos", "").await;
    let resp = post_multipart(
        &app,
        &vera,
        &format!("/account/assistant/{pid}/c"),
        Some("Qu'est-ce ?"),
        &[("p.png", &png())],
        "files",
        true,
    )
    .await;
    let v: Value = serde_json::from_str(&body_string(resp).await).unwrap();
    let cid = v["redirect"]
        .as_str()
        .unwrap()
        .rsplit('/')
        .next()
        .unwrap()
        .to_string();
    let state = answered(&app, &vera, &pid, &cid).await;
    assert_eq!(roles(&state), ["user", "assistant"]);
    assert!(state["messages"][1]["html"]
        .as_str()
        .unwrap()
        .contains("Une photo."));
    let args = std::fs::read_to_string(dir.join("args")).unwrap();
    for want in [
        "--output-format\nstream-json",
        "--include-partial-messages",
        "--input-format\nstream-json",
        "--model\nclaude-opus-5-5",
        "--effort\nlow",
    ] {
        assert!(args.contains(want), "{want}: {args}");
    }
    let stdin: Value =
        serde_json::from_str(std::fs::read_to_string(dir.join("stdin")).unwrap().trim()).unwrap();
    let content = &stdin["message"]["content"];
    assert!(content[0]["text"].as_str().unwrap().contains("Qu'est-ce ?"));
    assert_eq!(content[2]["type"], "image");
}
