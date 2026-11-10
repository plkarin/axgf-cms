//! The AFG's commands (docs/adr/0045 §2): in a member's personal assistant,
//! the AFG proposes changes to the archive in an `axgf-ops` block; the
//! thread draws them as a card, checked against the archive as the member
//! would write them; the member applies them, once, or sets them aside; and
//! what became of them is said in the thread, for them and for the AFG.
//!
//! A local HTTP server stands in for Ollama. Nothing here leaves the machine,
//! and the family is the sample's, which is nobody's.

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

type Seen = Arc<Mutex<Vec<String>>>;

/// A local Ollama that proposes, to every question, to add Testa Fictive;
/// and keeps what it was sent.
fn fake_ollama() -> (String, Seen) {
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
                let get = first.starts_with("GET");
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
                let reply = if get {
                    r#"{"models":[{"name":"fausse-famille:1b"}]}"#.to_string()
                } else {
                    log.lock()
                        .unwrap()
                        .push(String::from_utf8_lossy(&body).into_owned());
                    let answer = "Je propose d'ajouter Testa Fictive.\n\n```axgf-ops\n\
                        [{\"op\":\"create_person\",\"ref\":\"new:1\",\"given\":\"Testa\",\"family\":\"Fictive\",\
                        \"gender\":\"F\",\"living\":false,\"birth\":{\"date\":\"1931\",\"place\":\"Villefictive\"}}]\n```\n";
                    [
                        serde_json::json!({"model": "fausse-famille:1b",
                            "message": {"role": "assistant", "content": answer}, "done": false})
                        .to_string(),
                        r#"{"model":"fausse-famille:1b","message":{"content":""},"done":true}"#
                            .to_string(),
                    ]
                    .join("\n")
                        + "\n"
                };
                let _ = write!(
                    stream,
                    "HTTP/1.1 200 Fake\r\nContent-Type: application/x-ndjson\r\n\
                     Content-Length: {}\r\nConnection: close\r\n\r\n{reply}",
                    reply.len()
                );
            });
        }
    });
    (url, seen)
}

/// An app over the sample, its AFG the fake Ollama, with a contributor the
/// AFG is open to (vera) and a viewer it is open to (paul).
async fn app(tag: &str, url: &str) -> (axum::Router, Scratch, String, String) {
    use axgf_cms::acl::{new_user, Acl, Role};
    let dir = scratch(tag);
    let path = dir.join("family.axgf");
    std::fs::copy(sample(), &path).unwrap();
    let mut acl = Acl::default();
    for (name, role) in [("vera", Role::Contributor), ("paul", Role::Viewer)] {
        let mut u = new_user(name, PASSWORD, role).unwrap();
        u.afg = true;
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
    let vera = sign_in(&app, "vera").await;
    let paul = sign_in(&app, "paul").await;
    (app, dir.pointing_at(path), vera, paul)
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

/// A project that takes the archive, and a conversation in it, answered:
/// (project, conversation).
async fn asked(app: &axum::Router, cookie: &str) -> (String, String) {
    let resp = post_form_as(
        app,
        cookie,
        "/account/assistant",
        "name=Arbre&avatar=genealogy-tree",
    )
    .await;
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    let pid = resp.headers()[header::LOCATION]
        .to_str()
        .unwrap()
        .rsplit('/')
        .next()
        .unwrap()
        .to_string();
    let resp = post_form_as(
        app,
        cookie,
        &format!("/account/assistant/{pid}"),
        "part=settings&name=Arbre&avatar=genealogy-tree&archive=on",
    )
    .await;
    assert_eq!(
        resp.status(),
        StatusCode::SEE_OTHER,
        "the archive goes with it"
    );
    let b = "----axgfops";
    let body = format!(
        "--{b}\r\nContent-Disposition: form-data; name=\"text\"\r\n\r\nAjoute Testa Fictive\r\n--{b}--\r\n"
    );
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/account/assistant/{pid}/c"))
                .method("POST")
                .header(header::COOKIE, cookie)
                .header(header::ACCEPT, "application/json")
                .header(
                    header::CONTENT_TYPE,
                    format!("multipart/form-data; boundary={b}"),
                )
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let v: Value = serde_json::from_str(&body_string(resp).await).unwrap();
    let cid = v["redirect"]
        .as_str()
        .unwrap()
        .rsplit('/')
        .next()
        .unwrap()
        .to_string();
    for _ in 0..200 {
        let resp = get_with_cookie(
            app,
            &format!("/account/assistant/{pid}/c/{cid}/state?after=0"),
            cookie,
        )
        .await;
        let v: Value = serde_json::from_str(&body_string(resp).await).unwrap();
        if v["answering"] == false {
            return (pid, cid);
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    panic!("never answered");
}

async fn page(app: &axum::Router, uri: &str, cookie: &str) -> String {
    body_string(get_with_cookie(app, uri, cookie).await)
        .await
        .replace("&#x2f;", "/")
}

/// The answer's id, as the thread names it.
fn answer_id(page: &str) -> String {
    page.split(r#"data-role="assistant""#)
        .next()
        .and_then(|before| before.rsplit("data-msg=\"").next())
        .map(|s| s[..s.find('"').unwrap()].to_string())
        .expect("an answer")
}

fn persons_named(dir: &Scratch, name: &str) -> usize {
    let state =
        axgf_cms::AppState::load_or_create(&dir.dir().join("family.axgf"), "t".into()).unwrap();
    state.read(|flat| {
        flat["persons"]
            .as_object()
            .unwrap()
            .values()
            .filter(|p| axgf_cms::view::person_display_name(p) == name)
            .count()
    })
}

#[tokio::test]
async fn a_contributor_applies_what_the_afg_proposes_once() {
    let (url, seen) = fake_ollama();
    let (app, dir, vera, _) = app("afg-commands-apply", &url).await;
    let (pid, cid) = asked(&app, &vera).await;

    // The AFG was told how to propose, and what it may propose.
    let system = seen.lock().unwrap()[0].clone();
    let v: Value = serde_json::from_str(&system).unwrap();
    let system = v["messages"][0]["content"].as_str().unwrap();
    assert!(
        system.contains("```axgf-ops"),
        "the AFG is told how to propose"
    );
    assert!(
        system.contains("\"op\":\"create_person\""),
        "and the operations"
    );

    // The card: what it would do, in the reader's language, and its buttons.
    let conv = format!("/account/assistant/{pid}/c/{cid}");
    let html = page(&app, &conv, &vera).await;
    assert!(html.contains(r#"class="as-ops as-ops-open""#), "{html}");
    assert!(html.contains("Create Testa Fictive"), "{html}");
    assert!(html.contains("Create the place Villefictive"), "{html}");
    assert!(html.contains(r#"value="apply""#) && html.contains(r#"value="aside""#));
    assert!(
        !html.contains("```axgf-ops"),
        "the block is a card, not text"
    );
    let mid = answer_id(&html);

    // Applied: the person is in the archive, journalled as the AFG's.
    let resp = post_form_as(
        &app,
        &vera,
        &format!("{conv}/ops"),
        &format!("msg={mid}&block=1&do=apply"),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::SEE_OTHER, "applied");
    let html = page(&app, &conv, &vera).await;
    assert!(html.contains(r#"class="as-ops as-ops-applied""#), "{html}");
    assert!(
        html.contains("Applied: 2 records written."),
        "the person and the place: {html}"
    );
    assert!(html.contains(r#"data-role="done""#), "said in the thread");
    let journal = axgf_cms::journal::Journal::new(axgf_cms::journal::Journal::path_for(
        &dir.dir().join("family.axgf"),
    ));
    assert!(
        journal
            .read_all()
            .iter()
            .any(|e| e.who == "vera · AFG" && e.action == "create" && e.kind == "person"),
        "journalled as the AFG's"
    );

    assert!(
        !html.contains("new:1 ="),
        "the ids are the AFG's, not the thread's"
    );

    // The AFG hears of it, with the ids its refs took.
    let b = "----axgfops3";
    let body = format!(
        "--{b}\r\nContent-Disposition: form-data; name=\"text\"\r\n\r\nMerci\r\n--{b}--\r\n"
    );
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(conv.clone())
                .method("POST")
                .header(header::COOKIE, &vera)
                .header(header::ACCEPT, "application/json")
                .header(
                    header::CONTENT_TYPE,
                    format!("multipart/form-data; boundary={b}"),
                )
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    for _ in 0..200 {
        if seen.lock().unwrap().len() >= 2 {
            break;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    let second: Value = serde_json::from_str(&seen.lock().unwrap()[1]).unwrap();
    let said = second["messages"].to_string();
    assert!(
        said.contains("I applied the proposed changes: 2 records written."),
        "{said}"
    );
    assert!(said.contains("Ids: new:1 = "), "{said}");
    for _ in 0..200 {
        let resp = get_with_cookie(&app, &format!("{conv}/state?after=0"), &vera).await;
        let v: Value = serde_json::from_str(&body_string(resp).await).unwrap();
        if v["answering"] == false {
            break;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }

    // Once is all: asked again, nothing more is written.
    let resp = post_form_as(
        &app,
        &vera,
        &format!("{conv}/ops"),
        &format!("msg={mid}&block=1&do=apply"),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    drop(app);
    assert_eq!(persons_named(&dir, "Testa Fictive"), 1);
}

#[tokio::test]
async fn what_was_set_aside_changes_nothing_and_the_afg_hears_of_it() {
    let (url, seen) = fake_ollama();
    let (app, dir, vera, _) = app("afg-commands-aside", &url).await;
    let (pid, cid) = asked(&app, &vera).await;
    let conv = format!("/account/assistant/{pid}/c/{cid}");
    let mid = answer_id(&page(&app, &conv, &vera).await);
    let resp = post_form_as(
        &app,
        &vera,
        &format!("{conv}/ops"),
        &format!("msg={mid}&block=1&do=aside"),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    let html = page(&app, &conv, &vera).await;
    assert!(html.contains(r#"class="as-ops as-ops-aside""#), "{html}");
    // The next question carries what became of them.
    let b = "----axgfops2";
    let body = format!(
        "--{b}\r\nContent-Disposition: form-data; name=\"text\"\r\n\r\nEt maintenant ?\r\n--{b}--\r\n"
    );
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(conv.clone())
                .method("POST")
                .header(header::COOKIE, &vera)
                .header(header::ACCEPT, "application/json")
                .header(
                    header::CONTENT_TYPE,
                    format!("multipart/form-data; boundary={b}"),
                )
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    for _ in 0..200 {
        if seen.lock().unwrap().len() >= 2 {
            break;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    let second: Value = serde_json::from_str(&seen.lock().unwrap()[1]).unwrap();
    let said = second["messages"].to_string();
    assert!(
        said.contains("[I set the proposed changes aside: nothing was changed.]"),
        "{said}"
    );
    drop(app);
    assert_eq!(persons_named(&dir, "Testa Fictive"), 0);
}

#[tokio::test]
async fn a_viewer_is_told_why_and_cannot_apply() {
    let (url, seen) = fake_ollama();
    let (app, dir, _, paul) = app("afg-commands-viewer", &url).await;
    let (pid, cid) = asked(&app, &paul).await;
    let v: Value = serde_json::from_str(&seen.lock().unwrap()[0]).unwrap();
    let system = v["messages"][0]["content"].as_str().unwrap();
    assert!(system.contains("may read the archive, not change it"));
    assert!(!system.contains("\"op\":\"create_person\""));
    let conv = format!("/account/assistant/{pid}/c/{cid}");
    let html = page(&app, &conv, &paul).await;
    assert!(html.contains(r#"class="as-ops as-ops-refused""#), "{html}");
    assert!(html.contains("This account may read the archive, not change it."));
    assert!(!html.contains(r#"value="apply""#));
    // Posted anyway, it is refused, and nothing is written.
    let mid = answer_id(&html);
    let resp = post_form_as(
        &app,
        &paul,
        &format!("{conv}/ops"),
        &format!("msg={mid}&block=1&do=apply"),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::CONFLICT);
    drop(app);
    assert_eq!(persons_named(&dir, "Testa Fictive"), 0);
}

#[tokio::test]
async fn the_personal_assistant_carries_the_guide() {
    let (url, _) = fake_ollama();
    let (app, _dir, vera, paul) = app("afg-commands-guide", &url).await;
    let html = page(&app, "/account/assistant", &vera).await;
    assert!(html.contains(r#"id="guide""#));
    assert!(html.contains("genealogy through the AFG"));
    assert!(html.contains(r#"id="guide-11""#), "the MCP's chapter");
    assert!(html.contains("<blockquote>"), "requests to make");
    let html = page(&app, "/account/assistant", &paul).await;
    assert!(html.contains("Your account reads the archive and does not change it"));
}
