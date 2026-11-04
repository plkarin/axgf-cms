//! The Artificial family genealogist's room (docs/adr/0023 to 0025): only an
//! administrator reaches it; its key is kept and never shown; it suggests the
//! models of the moment and their efforts, and keeps any other; its test sends
//! the archive and the asker's brain to a service or an agent, and shows what
//! came back in a window; each account's brain is its own, on disk; and the
//! accounts page says who may talk to the AFG.
//!
//! A fake stands in for each: a local HTTP server for Ollama, a shell script
//! named `claude` for Claude Code. Nothing here leaves the machine, and every
//! answer below is the fake's.

mod common;

use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, Once};
use std::time::Duration;

use axum::http::{header, StatusCode};
use common::*;
use serde_json::Value;

const PASSWORD: &str = "correct-horse-battery-staple";
const KEY: &str = "sk-fake-0123456789abcdef";

/// A setting of this site's own in the environment, which no agent may
/// inherit. Set once, before any test of this file does anything else.
const SITE_SECRET: &str = "AXGF_CMS_AFG_TEST_SECRET";
static ENV: Once = Once::new();

fn site_secret_in_env() {
    ENV.call_once(|| std::env::set_var(SITE_SECRET, "not-for-agents"));
}

fn sample() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("deploy/sample.axgf")
}

fn form(pairs: &[(&str, &str)]) -> String {
    pairs
        .iter()
        .map(|(k, v)| {
            let enc: String = v
                .bytes()
                .map(|b| match b {
                    b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                        (b as char).to_string()
                    }
                    _ => format!("%{b:02X}"),
                })
                .collect();
            format!("{k}={enc}")
        })
        .collect::<Vec<_>>()
        .join("&")
}

/// Configure the AFG as an administrator and return the page.
async fn configure(
    app: &axum::Router,
    cookie: &str,
    pairs: &[(&str, &str)],
) -> (StatusCode, String) {
    let resp = post_form_as(app, cookie, "/admin/afg", &form(pairs)).await;
    let status = resp.status();
    (status, body_string(resp).await)
}

/// Press the test button and wait for the answer as the room's window does:
/// the room opens on `?run=<id>` with the window over it, and the window is
/// asked for until it no longer waits. Returns the window, then the room.
async fn run_test(app: &axum::Router, cookie: &str) -> (String, String) {
    let resp = post_form_as(app, cookie, "/admin/afg/test", "").await;
    assert_eq!(
        resp.status(),
        StatusCode::SEE_OTHER,
        "the test opens the room with its window"
    );
    let at = resp.headers()[header::LOCATION]
        .to_str()
        .unwrap()
        .to_string();
    let id = at
        .strip_prefix("/admin/afg?run=")
        .unwrap_or_else(|| panic!("{at}"))
        .to_string();
    let first = body_string(get_with_cookie(app, &at, cookie).await).await;
    assert!(
        first.contains("data-afg-dialog"),
        "the window, over the room"
    );
    for _ in 0..200 {
        let resp = get_with_cookie(app, &format!("/admin/afg/run/{id}"), cookie).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let v: Value = serde_json::from_str(&body_string(resp).await).expect("JSON");
        if v["state"] != "running" {
            let room = body_string(get_with_cookie(app, &at, cookie).await).await;
            assert!(
                !room.contains(r#"http-equiv="refresh""#),
                "an answered test is not asked for again"
            );
            return (v["html"].as_str().unwrap_or_default().to_string(), room);
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    panic!("the test {id} never finished");
}

/// An app over the sample whose accounts are these, and a session for each.
async fn app_with(
    tag: &str,
    users: &[(&str, axgf_cms::acl::Role)],
) -> (axum::Router, Scratch, Vec<(String, String)>) {
    use axgf_cms::acl::{new_user, Acl};
    let dir = scratch(tag);
    let path = dir.join("family.axgf");
    std::fs::copy(sample(), &path).unwrap();
    let mut acl = Acl::default();
    for (name, role) in users {
        acl.users.push(new_user(name, PASSWORD, *role).unwrap());
    }
    let ids: Vec<String> = acl.users.iter().map(|u| u.id.clone()).collect();
    acl.save(&Acl::path_for(&path)).unwrap();
    let app = build_app(&path).unwrap();
    let mut sessions = Vec::new();
    for ((name, _), id) in users.iter().zip(ids) {
        let login = post_form(
            &app,
            "/admin/login",
            &format!("username={name}&password={PASSWORD}"),
            false,
        )
        .await;
        assert!(login.status().is_redirection(), "{name} signs in");
        let cookie = login.headers()[header::SET_COOKIE]
            .to_str()
            .unwrap()
            .split(';')
            .next()
            .unwrap()
            .to_string();
        sessions.push((id, cookie));
    }
    (app, dir.pointing_at(path), sessions)
}

/// What a fake service was sent: method, path and body of each request.
type Seen = Arc<Mutex<Vec<(String, String, String)>>>;

/// A local HTTP server answering each request with `respond`, one connection
/// at a time, keeping what it was sent.
fn fake_service(respond: fn(&str, &str) -> (u16, String)) -> (String, Seen) {
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
            let (status, reply) = respond(&method, &path);
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

/// An Ollama with one model installed, which answers every question alike.
fn ollama(method: &str, path: &str) -> (u16, String) {
    match (method, path) {
        ("GET", "/api/tags") => (200, r#"{"models":[{"name":"fausse-famille:1b"}]}"#.into()),
        ("POST", "/api/chat") => (
            200,
            r#"{"model":"fausse-famille:1b","message":{"role":"assistant",
               "content":"1. La famille Exemple — 3 personnes"},
               "prompt_eval_count":1500,"eval_count":42,"done":true}"#
                .into(),
        ),
        _ => (404, r#"{"error":"no such path"}"#.into()),
    }
}

/// An Ollama that answers in Markdown and asks to remember something.
fn ollama_that_remembers(method: &str, path: &str) -> (u16, String) {
    match (method, path) {
        ("POST", "/api/chat") => (
            200,
            serde_json::json!({
                "model": "fausse-famille:1b",
                "message": {"role": "assistant", "content":
                    "**1.** La famille *Exemple*\n\n<memory>\n- Cherche la branche Exemple\n</memory>"},
                "done": true
            })
            .to_string(),
        ),
        _ => (404, r#"{"error":"no such path"}"#.into()),
    }
}

/// An Ollama without the model asked for.
fn ollama_without_the_model(_: &str, _: &str) -> (u16, String) {
    (
        404,
        r#"{"error":"model \"absent:7b\" not found, try pulling it first"}"#.into(),
    )
}

/// A program named `claude` that answers as `claude -p --output-format json`
/// does, after writing down its arguments, its input and its environment in
/// `dir`. Its help names the options of a recent version. Given an empty archive, it answers the words it was told to answer
/// then, as a model following its instructions would.
fn fake_claude(dir: &Path) -> PathBuf {
    let bin = dir.join("bin");
    std::fs::create_dir_all(&bin).unwrap();
    let program = bin.join("claude");
    let d = dir.display();
    std::fs::write(
        &program,
        format!(
            r#"#!/bin/sh
if [ "$1" = "--help" ]; then
  printf '  --no-session-persistence  Disable\n  --safe-mode  Start with\n'
  printf '  --strict-mcp-config  Only use\n  --permission-prompts <target>  Who\n'
  exit 0
fi
printf '%s\n' "$@" > "{d}/args"
cat > "{d}/stdin"
env > "{d}/env"
if head -n 1 "{d}/stdin" | grep -q ' 0 persons, 0 families'; then
  answer=$(printf '%s\n' "$@" | sed -n 's/.*answer exactly: //p')
else
  answer='1. La famille Exemple'
fi
printf '{{"type":"result","subtype":"success","is_error":false,"result":"%s","total_cost_usd":0.0123,"usage":{{"input_tokens":4321,"output_tokens":21}}}}' "$answer"
"#
        ),
    )
    .unwrap();
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(&program, std::fs::Permissions::from_mode(0o755)).unwrap();
    program
}

fn acl_text(bundle: &Path) -> String {
    std::fs::read_to_string(axgf_cms::acl::Acl::path_for(bundle)).unwrap_or_default()
}

#[tokio::test]
async fn only_an_administrator_reaches_the_room() {
    site_secret_in_env();
    use axgf_cms::acl::{new_user, Acl, Role};
    let dir = scratch("afg-roles");
    let path = dir.join("family.axgf");
    std::fs::copy(sample(), &path).unwrap();
    let mut acl = Acl::default();
    for (name, role) in [("vera", Role::Viewer), ("carl", Role::Contributor)] {
        acl.users.push(new_user(name, PASSWORD, role).unwrap());
    }
    acl.save(&Acl::path_for(&path)).unwrap();
    let app = build_app(&path).unwrap();

    assert_eq!(
        get(&app, "/admin/afg").await.status(),
        StatusCode::UNAUTHORIZED
    );
    for name in ["vera", "carl"] {
        let login = post_form(
            &app,
            "/admin/login",
            &format!("username={name}&password={PASSWORD}"),
            false,
        )
        .await;
        assert!(login.status().is_redirection(), "{name} signs in");
        let cookie = login.headers()[header::SET_COOKIE]
            .to_str()
            .unwrap()
            .split(';')
            .next()
            .unwrap()
            .to_string();
        assert_eq!(
            get_with_cookie(&app, "/admin/afg", &cookie).await.status(),
            StatusCode::FORBIDDEN,
            "{name} may not see where the archive is sent"
        );
        for (uri, body) in [
            (
                "/admin/afg",
                form(&[("kind", "ollama"), ("url", "http://127.0.0.1:9")]),
            ),
            ("/admin/afg/test", String::new()),
        ] {
            assert_eq!(
                post_form_as(&app, &cookie, uri, &body).await.status(),
                StatusCode::FORBIDDEN,
                "{name} may not post {uri}"
            );
        }
        assert_eq!(
            get_with_cookie(&app, "/admin/afg/run/x", &cookie)
                .await
                .status(),
            StatusCode::FORBIDDEN
        );
    }
    assert!(
        !acl_text(&path).contains("ollama"),
        "nothing was saved by anyone else"
    );
}

#[tokio::test]
async fn the_key_is_kept_never_shown_and_forgotten_on_request() {
    site_secret_in_env();
    let (app, bundle) = app_with_bundle("afg-key", &sample());
    let admin = format!("{}; axgf_lang=en", admin_cookie(&app).await);

    let page = body_string(get_with_cookie(&app, "/admin/afg", &admin).await).await;
    assert!(page.contains("No key is kept."), "nothing configured yet");
    assert!(
        page.contains(r#"type="submit" data-busy="Working…" disabled>"#),
        "no test yet"
    );

    let (status, page) = configure(
        &app,
        &admin,
        &[
            ("kind", "openai"),
            ("url", ""),
            ("api_key", KEY),
            ("model", "gpt-fake"),
            ("timeout", "120"),
        ],
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{page}");
    assert!(page.contains("Saved."));
    assert!(page.contains("…cdef"), "the key is said by its end");
    assert!(
        !page.contains(KEY) && !page.contains("0123456789"),
        "never the key itself"
    );
    assert!(
        page.replace("&#x2f;", "/")
            .contains("https://api.openai.com/v1/chat/completions"),
        "the address called is said"
    );
    assert!(acl_text(&bundle).contains(KEY), "kept with the accounts");
    let again = body_string(get_with_cookie(&app, "/admin/afg", &admin).await).await;
    assert!(!again.contains(KEY));

    // Left empty, the kept key stays.
    let (status, page) = configure(
        &app,
        &admin,
        &[
            ("kind", "openai"),
            ("api_key", ""),
            ("model", "gpt-fake-2"),
            ("timeout", "120"),
        ],
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert!(page.contains("…cdef"));
    assert!(acl_text(&bundle).contains(KEY) && acl_text(&bundle).contains("gpt-fake-2"));

    // Forgetting the key a service needs is refused, and nothing changes.
    let (status, page) = configure(
        &app,
        &admin,
        &[
            ("kind", "openai"),
            ("clear_key", "on"),
            ("model", "gpt-fake-3"),
        ],
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    assert!(page.contains("This service needs an API key."));
    assert!(acl_text(&bundle).contains(KEY) && !acl_text(&bundle).contains("gpt-fake-3"));

    // vLLM may go without one.
    let (status, page) = configure(
        &app,
        &admin,
        &[
            ("kind", "vllm"),
            ("url", "http://127.0.0.1:9/v1"),
            ("clear_key", "on"),
            ("model", "m"),
        ],
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert!(page.contains("No key is kept."));
    assert!(!acl_text(&bundle).contains(KEY), "forgotten");

    for (pairs, says) in [
        (
            vec![("kind", "claude-code"), ("program", "/bin/sh")],
            "The program must be the agent's own",
        ),
        (
            vec![("kind", "claude-code"), ("program", "claude")],
            "The program must be the agent's own",
        ),
        (
            vec![("kind", "claude-code"), ("effort", "--turbo")],
            "An effort is written in letters, digits and",
        ),
        (
            vec![("kind", "ollama"), ("url", "ftp://127.0.0.1/")],
            "The address must begin with http:// or https://.",
        ),
        (
            vec![("kind", "mistral"), ("api_key", KEY)],
            "A model is needed here",
        ),
    ] {
        let (status, page) = configure(&app, &admin, &pairs).await;
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{pairs:?}");
        let page = page.replace("&#x27;", "'").replace("&#x2f;", "/");
        assert!(page.contains(says), "{pairs:?}: {says}");
    }
}

#[tokio::test]
async fn a_test_sends_the_archive_to_ollama_and_shows_its_answer() {
    site_secret_in_env();
    let (url, seen) = fake_service(ollama);
    let (app, _bundle) = app_with_bundle("afg-ollama", &sample());
    let admin = format!("{}; axgf_lang=fr", admin_cookie(&app).await);
    let (status, page) = configure(
        &app,
        &admin,
        &[("kind", "ollama"), ("url", &url), ("timeout", "60")],
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{page}");

    let (page, room) = run_test(&app, &admin).await;
    assert!(page.contains("Succès"), "{page}");
    assert!(page.contains("La famille Exemple — 3 personnes"));
    assert!(
        page.contains("Cette session n’a pas de compte, donc pas de cerveau."),
        "the emergency token has no brain"
    );
    assert!(
        !_bundle.with_extension("brains").exists(),
        "and none is made"
    );
    assert!(
        page.contains("fausse-famille:1b"),
        "the model that answered"
    );
    assert!(
        page.contains("Donne-moi le top 3 des familles"),
        "the question, in French"
    );

    let seen = seen.lock().unwrap().clone();
    assert_eq!(
        seen.iter()
            .map(|(m, p, _)| format!("{m} {p}"))
            .collect::<Vec<_>>(),
        ["GET /api/tags", "POST /api/chat"],
        "no model named: the first installed one"
    );
    let body: Value = serde_json::from_str(&seen[1].2).expect("JSON");
    assert_eq!(body["model"], "fausse-famille:1b");
    assert_eq!(body["stream"], false);
    assert!(body["options"]["num_ctx"].as_u64().unwrap() >= 8192);
    let system = body["messages"][0]["content"].as_str().unwrap();
    assert!(system.ends_with("answer exactly: Bundle vide"), "{system}");
    let user = body["messages"][1]["content"].as_str().unwrap();
    assert!(
        user.contains("\n=== manifest.json ===\n"),
        "the manifest first"
    );
    let names: Vec<&str> = user
        .lines()
        .filter_map(|l| l.strip_prefix("=== ")?.strip_suffix(" ==="))
        .collect();
    assert!(names.len() > 3, "{names:?}");
    assert!(
        names.iter().all(|n| n.ends_with(".json")),
        "only JSON files: {names:?}"
    );
    assert!(user.ends_with(
        "Question: Donne-moi le top 3 des familles disposant du plus grand nombre d’individus déclarés."
    ));

    // The room says how the last test went, and opens it again.
    assert!(room.contains("data-afg-dialog") && room.contains("Succès"));
    let room = body_string(get_with_cookie(&app, "/admin/afg", &admin).await).await;
    assert!(room.contains("Répondu") && room.contains("/admin/afg?run="));
    assert!(
        !room.contains("data-afg-dialog"),
        "no window unless asked for"
    );
}

/// The values of a list the page suggests, in order.
fn suggested(page: &str, id: &str) -> Vec<String> {
    let start = page
        .find(&format!(r#"<datalist id="{id}""#))
        .unwrap_or_else(|| panic!("no list {id}"));
    let list = &page[start..start + page[start..].find("</datalist>").unwrap()];
    list.split(r#"<option value=""#)
        .skip(1)
        .map(|o| o[..o.find('"').unwrap()].replace("&#x2f;", "/"))
        .collect()
}

#[tokio::test]
async fn the_room_suggests_the_models_of_the_moment_and_keeps_any_other() {
    site_secret_in_env();
    let (url, seen) = fake_service(ollama);
    let (app, _bundle) = app_with_bundle("afg-suggestions", &sample());
    let admin = admin_cookie(&app).await;

    // Each way of computing has its list of models, each with its efforts;
    // the field points at the chosen way's, and the efforts offered are its
    // model's, its usual one marked.
    let (status, page) = configure(
        &app,
        &admin,
        &[
            ("kind", "claude-code"),
            ("model", "claude-opus-5-5"),
            ("effort", "high"),
        ],
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{page}");
    for slug in [
        "ollama",
        "openai",
        "mistral",
        "vllm",
        "anthropic",
        "gemini",
        "claude-code",
        "codex",
        "opencode",
    ] {
        assert!(!suggested(&page, &format!("afg-models-{slug}")).is_empty());
    }
    assert!(page.contains(r#"list="afg-models-claude-code""#));
    let models = suggested(&page, "afg-models-claude-code");
    assert_eq!(models[0], "claude-opus-5-5", "the one to start with first");
    assert!(models.contains(&"opus".to_string()), "and the aliases");
    assert!(suggested(&page, "afg-models-opencode").contains(&"anthropic/claude-opus-5-5".into()));
    assert_eq!(
        suggested(&page, "afg-efforts"),
        ["low", "medium", "high", "xhigh", "max"]
    );
    assert!(
        page.contains(r#"<option value="medium" label="default">"#),
        "Opus 5.5 thinks at medium unless told otherwise"
    );
    for field in ["model", "effort"] {
        assert!(
            page.contains(&format!(r#"data-afg-unlisted="{field}" hidden"#)),
            "{field}: suggested, nothing to say"
        );
    }

    // A model and an effort that are not suggested are kept as written,
    // and the room says they are not suggestions.
    let (status, page) = configure(
        &app,
        &admin,
        &[
            ("kind", "claude-code"),
            ("model", "opus-5"),
            ("effort", "turbo"),
        ],
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{page}");
    assert!(page.contains(r#"value="opus-5""#) && page.contains(r#"value="turbo""#));
    for field in ["model", "effort"] {
        assert!(
            !page.contains(&format!(r#"data-afg-unlisted="{field}" hidden"#)),
            "{field}: said not to be a suggestion"
        );
    }
    assert!(page.contains("Not one of the suggestions: sent as written."));
    // But never an effort a program could read as an option.
    let (status, page) = configure(
        &app,
        &admin,
        &[("kind", "claude-code"), ("effort", "--max")],
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    assert!(page.contains("An effort is written in letters"), "{page}");

    // A service is given its effort too: Ollama's `think`.
    let (status, page) = configure(
        &app,
        &admin,
        &[
            ("kind", "ollama"),
            ("url", &url),
            ("model", "qwen3.8"),
            ("effort", "false"),
            ("timeout", "60"),
        ],
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{page}");
    assert_eq!(
        suggested(&page, "afg-efforts"),
        ["false", "true"],
        "Qwen thinks or does not"
    );
    let (window, _) = run_test(&app, &admin).await;
    assert!(window.contains("Success"), "{window}");
    let seen = seen.lock().unwrap().clone();
    let (_, path, body) = seen.last().unwrap();
    assert_eq!(path, "/api/chat");
    let body: Value = serde_json::from_str(body).unwrap();
    assert_eq!(
        (&body["model"], &body["think"]),
        (&"qwen3.8".into(), &false.into())
    );
}

#[tokio::test]
async fn what_a_service_says_when_it_fails_is_shown() {
    site_secret_in_env();
    let (url, _seen) = fake_service(ollama_without_the_model);
    let (app, _bundle) = app_with_bundle("afg-fails", &sample());
    let admin = format!("{}; axgf_lang=en", admin_cookie(&app).await);
    configure(
        &app,
        &admin,
        &[("kind", "ollama"), ("url", &url), ("model", "absent:7b")],
    )
    .await;
    let (page, _room) = run_test(&app, &admin).await;
    assert!(page.contains("Error"), "{page}");
    assert!(
        page.replace("&quot;", "\"")
            .contains(r#"HTTP 404: model "absent:7b" not found"#),
        "{page}"
    );
}

/// A test that fails leaves no brain behind (1.5.1): on 3 November 2026 a
/// program not yet signed in left an account an empty one.
#[tokio::test]
async fn a_test_that_fails_makes_no_brain() {
    site_secret_in_env();
    use axgf_cms::acl::Role;
    let (url, _seen) = fake_service(ollama_without_the_model);
    let (app, bundle, sessions) = app_with("afg-fails-no-brain", &[("marie", Role::Admin)]).await;
    let cookie = format!("{}; axgf_lang=en", sessions[0].1);
    configure(
        &app,
        &cookie,
        &[("kind", "ollama"), ("url", &url), ("model", "absent:7b")],
    )
    .await;
    let (page, _room) = run_test(&app, &cookie).await;
    assert!(page.contains("Error"), "{page}");
    assert!(
        !bundle.with_extension("brains").join("marie").exists(),
        "no brain for an exchange that did not happen"
    );
}

#[tokio::test]
async fn claude_code_reads_the_archive_on_its_input_and_an_empty_one_is_said_so() {
    site_secret_in_env();
    let (app, _bundle) = app_with_empty_bundle("afg-claude");
    let work = scratch("afg-claude-bin");
    let program = fake_claude(&work);
    let admin = format!("{}; axgf_lang=fr", admin_cookie(&app).await);
    let (status, page) = configure(
        &app,
        &admin,
        &[
            ("kind", "claude-code"),
            ("program", program.to_str().unwrap()),
            ("model", "sonnet"),
            ("effort", "high"),
        ],
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{page}");
    assert!(
        page.contains("Trouvé"),
        "the program is found where it was said"
    );

    let (page, _room) = run_test(&app, &admin).await;
    assert!(page.contains("Succès"), "{page}");
    assert!(
        page.contains(r#"<div class="afg-answer note-text"><p>Bundle vide</p>"#),
        "{page}"
    );

    let args = std::fs::read_to_string(work.join("args")).unwrap();
    let args: Vec<&str> = args.lines().collect();
    assert_eq!(&args[..3], ["-p", "--output-format", "json"]);
    assert!(
        args.windows(2).any(|w| w == ["--tools", ""]),
        "no tool: {args:?}"
    );
    for flag in [
        "--safe-mode",
        "--strict-mcp-config",
        "--no-session-persistence",
    ] {
        assert!(args.contains(&flag), "{flag}: {args:?}");
    }
    assert!(args.windows(2).any(|w| w == ["--model", "sonnet"]));
    assert!(args.windows(2).any(|w| w == ["--effort", "high"]));
    let stdin = std::fs::read_to_string(work.join("stdin")).unwrap();
    assert!(
        stdin.starts_with("The AXGF bundle: "),
        "the archive on its input"
    );
    assert!(stdin.contains(" 0 persons, 0 families."));
    assert!(
        !args.iter().any(|a| a.contains("=== manifest.json ===")),
        "never on its command line"
    );
    let env = std::fs::read_to_string(work.join("env")).unwrap();
    assert!(
        std::env::var_os(SITE_SECRET).is_some() && !env.lines().any(|l| l.starts_with("AXGF_")),
        "this site's own settings stay its own"
    );
    // Its private working directory is gone.
    let left: Vec<_> = std::fs::read_dir(_bundle.parent().unwrap())
        .unwrap()
        .flatten()
        .filter(|e| e.file_name().to_string_lossy().starts_with(".afg-"))
        .collect();
    assert!(left.is_empty(), "{left:?}");
}

#[tokio::test]
async fn each_account_has_a_brain_that_the_afg_reads_and_writes() {
    site_secret_in_env();
    use axgf_cms::acl::Role;
    let (url, seen) = fake_service(ollama_that_remembers);
    let (app, bundle, sessions) = app_with("afg-brain", &[("marie", Role::Admin)]).await;
    let (marie_id, cookie) = sessions[0].clone();
    let cookie = format!("{cookie}; axgf_lang=fr");
    let (status, page) = configure(
        &app,
        &cookie,
        &[
            ("kind", "ollama"),
            ("url", &url),
            ("model", "fausse-famille:1b"),
        ],
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{page}");
    assert!(page.contains("0 échange"), "a brain not made yet");

    // The first exchange makes the brain, and the answer's Markdown is drawn.
    let (window, _room) = run_test(&app, &cookie).await;
    assert!(window.contains("Succès"), "{window}");
    assert!(window.contains("<strong>1.</strong>") && window.contains("<em>Exemple</em>"));
    assert!(
        !window.contains("memory&gt;") && !window.contains("<memory>"),
        "the block is not shown"
    );
    assert!(window.contains("Retenu dans le cerveau de marie"));
    assert!(window.contains("<li>Cherche la branche Exemple</li>"));

    let brain = bundle.with_extension("brains").join("marie");
    let card = std::fs::read_to_string(brain.join("brain.json")).unwrap();
    assert!(card.contains(&marie_id), "her account's, by its id");
    let memory = std::fs::read_to_string(brain.join("memory.md")).unwrap();
    assert!(memory.contains("Cherche la branche Exemple"), "{memory}");
    let log = std::fs::read_to_string(brain.join("conversations.jsonl")).unwrap();
    assert_eq!(log.lines().count(), 1);
    let first: Value = serde_json::from_str(log.lines().next().unwrap()).unwrap();
    assert_eq!(first["context"], "test");
    assert_eq!(
        first["answer"], "**1.** La famille *Exemple*",
        "kept as answered"
    );
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = |p: &Path| std::fs::metadata(p).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode(&brain), 0o700);
        assert_eq!(mode(&brain.join("memory.md")), 0o600);
    }

    // The next question carries the brain back, after the archive.
    let (window, room) = run_test(&app, &cookie).await;
    assert!(
        window.contains("Rien de nouveau retenu dans le cerveau de marie."),
        "a fact is kept once: {window}"
    );
    let bodies: Vec<String> = seen
        .lock()
        .unwrap()
        .iter()
        .map(|(_, _, b)| b.clone())
        .collect();
    assert_eq!(bodies.len(), 2);
    let second: Value = serde_json::from_str(&bodies[1]).unwrap();
    let system = second["messages"][0]["content"].as_str().unwrap();
    assert!(system.contains("their brain") && system.contains("<memory> block"));
    let user = second["messages"][1]["content"].as_str().unwrap();
    let brain_at = user.find("=== brain: marie ===").expect("the brain");
    assert!(
        user.find("=== manifest.json ===").unwrap() < brain_at,
        "after the archive"
    );
    assert!(user[brain_at..].contains("Cherche la branche Exemple"));
    assert!(user[brain_at..].contains("Your last exchanges with them"));
    assert!(user.ends_with("individus déclarés."));
    assert!(room.contains("2 échanges"), "the room counts them");
    assert!(room.contains("1 fait retenu · 1 cerveau sur le disque"));
}

#[tokio::test]
async fn the_accounts_page_says_who_may_talk_to_the_afg() {
    site_secret_in_env();
    use axgf_cms::acl::Role;
    let (app, bundle, sessions) = app_with(
        "afg-access",
        &[("marie", Role::Admin), ("vera", Role::Viewer)],
    )
    .await;
    let admin = format!("{}; axgf_lang=en", sessions[0].1);
    let vera = sessions[1].0.clone();
    let flag = |name: &str| {
        let acl = axgf_cms::acl::Acl::load(&axgf_cms::acl::Acl::path_for(&bundle)).unwrap();
        acl.users.iter().find(|u| u.username == name).unwrap().afg
    };
    let page = body_string(get_with_cookie(&app, "/admin/users", &admin).await).await;
    assert!(page.contains("May talk to the AFG"));
    assert!(page.contains("0 accounts may talk to the AFG."));
    assert!(!flag("vera"), "off until an administrator says so");

    let change = |afg: bool| {
        let mut f = "role=viewer&status=active&email=&family_scope=&password=".to_string();
        if afg {
            f.push_str("&afg=on");
        }
        f
    };
    let resp = post_form_as(&app, &admin, &format!("/admin/users/{vera}"), &change(true)).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let page = body_string(resp).await;
    assert!(flag("vera"));
    assert!(page.contains("1 account may talk to the AFG."), "{page}");
    post_form_as(
        &app,
        &admin,
        &format!("/admin/users/{vera}"),
        &change(false),
    )
    .await;
    assert!(!flag("vera"), "and off again");

    let made = post_form_as(
        &app,
        &admin,
        "/admin/users",
        "username=lea&password=correct-horse-battery&role=viewer&email=&family_scope=&afg=on",
    )
    .await;
    assert!(made.status().is_success() || made.status().is_redirection());
    assert!(flag("lea"), "set as the account is made");

    // A test that is gone says so, to the room and to its script.
    let resp = get_with_cookie(&app, "/admin/afg/run/no-such-test", &admin).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    let v: Value = serde_json::from_str(&body_string(resp).await).unwrap();
    assert_eq!(v["state"], "gone");
    let room =
        body_string(get_with_cookie(&app, "/admin/afg?run=no-such-test", &admin).await).await;
    assert!(room.contains("Tests are kept for the life of the process"));
}

/// The marks a page names for one kind of thing to forget, in order.
fn marks(page: &str, what: &str) -> Vec<String> {
    let needle = format!(r#"name="{what}" value=""#);
    page.match_indices(&needle)
        .map(|(i, _)| {
            page[i + needle.len()..]
                .chars()
                .take_while(|c| *c != '"')
                .collect()
        })
        .collect()
}

#[tokio::test]
async fn each_account_sees_and_forgets_its_own_brain() {
    site_secret_in_env();
    use axgf_cms::acl::Role;
    let (url, seen) = fake_service(ollama_that_remembers);
    let (app, bundle, sessions) = app_with(
        "afg-forget",
        &[("marie", Role::Admin), ("vera", Role::Viewer)],
    )
    .await;
    let marie = format!("{}; axgf_lang=fr", sessions[0].1);
    let vera = format!("{}; axgf_lang=fr", sessions[1].1);
    let vera_id = sessions[1].0.clone();

    // No access and no brain: nothing to show.
    let page = body_string(get_with_cookie(&app, "/account", &vera).await).await;
    assert!(
        !page.contains(r#"id="account-brain""#),
        "vera may not talk to it yet"
    );
    let allow = format!("/admin/users/{vera_id}");
    post_form_as(
        &app,
        &marie,
        &allow,
        "role=viewer&status=active&email=&family_scope=&password=&afg=on",
    )
    .await;
    let vera = format!("{}; axgf_lang=fr", sign_in(&app, "vera").await);
    let page = body_string(get_with_cookie(&app, "/account", &vera).await).await;
    assert!(
        page.contains(r#"id="account-brain""#)
            && page.contains("L’AFG ne retient encore rien de vous.")
    );

    // Marie's first exchange makes her brain; a second fact is added beside it.
    configure(
        &app,
        &marie,
        &[
            ("kind", "ollama"),
            ("url", &url),
            ("model", "fausse-famille:1b"),
        ],
    )
    .await;
    run_test(&app, &marie).await;
    let owner = axgf_cms::brain::Owner {
        id: sessions[0].0.clone(),
        name: "marie".into(),
        person_id: None,
    };
    let brain = axgf_cms::brain::Brains::beside(&bundle).of(&owner).unwrap();
    let extra = axgf_cms::brain::Exchange {
        at: "2026-11-02T12:00:00Z".into(),
        context: "test".into(),
        question: "Qui est l’aînée ?".into(),
        answer: "Ama.".into(),
        ..Default::default()
    };
    brain
        .remember(&extra, &["Préfère les réponses courtes".into()])
        .unwrap();

    let page = body_string(get_with_cookie(&app, "/account", &marie).await).await;
    assert!(page.contains("Le cerveau de votre AFG"));
    assert!(
        page.contains("Cherche la branche Exemple")
            && page.contains("Préfère les réponses courtes")
    );
    assert!(page.contains("Donne-moi le top 3") && page.contains("Qui est l’aînée ?"));
    let facts = marks(&page, "fact");
    let exchanges = marks(&page, "exchange");
    assert_eq!((facts.len(), exchanges.len()), (2, 2), "{page}");
    assert!(
        page.find("Qui est l’aînée ?").unwrap() < page.find("Donne-moi le top 3").unwrap(),
        "the most recent exchange first"
    );

    // Vera cannot reach Marie's brain, whatever she posts.
    let resp = post_form_as(
        &app,
        &vera,
        "/account/brain/forget",
        &format!("fact={}", facts[0]),
    )
    .await;
    assert_eq!(
        resp.headers()[header::LOCATION],
        "/account?done=already#account-brain"
    );
    assert_eq!(brain.listed_facts().len(), 2, "untouched");

    // Marie forgets a fact: it is no longer sent with her next question.
    let resp = post_form_as(
        &app,
        &marie,
        "/account/brain/forget",
        &format!("fact={}", facts[0]),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    assert_eq!(
        resp.headers()[header::LOCATION],
        "/account?done=forgotten#account-brain"
    );
    let page = body_string(get_with_cookie(&app, "/account?done=forgotten", &marie).await).await;
    assert!(page.contains("Oublié : l’AFG ne le saura plus."));
    assert!(!page.contains("Cherche la branche Exemple"));
    let again = post_form_as(
        &app,
        &marie,
        "/account/brain/forget",
        &format!("fact={}", facts[0]),
    )
    .await;
    assert_eq!(
        again.headers()[header::LOCATION],
        "/account?done=already#account-brain"
    );
    run_test(&app, &marie).await;
    let sent = seen.lock().unwrap().last().unwrap().2.clone();
    let body: Value = serde_json::from_str(&sent).unwrap();
    let user = body["messages"][1]["content"].as_str().unwrap();
    let brain_part = &user[user.find("=== brain: marie ===").unwrap()..];
    assert!(
        !brain_part.contains("Cherche la branche Exemple"),
        "forgotten, not told"
    );
    assert!(
        brain_part.contains("Préfère les réponses courtes"),
        "the rest still is"
    );

    // An exchange, then everything, which asks to be sure.
    let page = body_string(get_with_cookie(&app, "/account", &marie).await).await;
    let before = brain.listed_exchanges(100).1;
    let first = marks(&page, "exchange")[0].clone();
    post_form_as(
        &app,
        &marie,
        "/account/brain/forget",
        &format!("exchange={first}"),
    )
    .await;
    assert_eq!(brain.listed_exchanges(100).1, before - 1);
    let unsure = post_form_as(&app, &marie, "/account/brain/forget", "all=1").await;
    assert_eq!(unsure.status(), StatusCode::BAD_REQUEST);
    assert!(body_string(unsure)
        .await
        .contains("Cochez la case pour tout oublier."));
    assert!(brain.dir().exists());
    let resp = post_form_as(&app, &marie, "/account/brain/forget", "all=1&confirm=on").await;
    assert_eq!(
        resp.headers()[header::LOCATION],
        "/account?done=forgotten-all#account-brain"
    );
    assert!(!brain.dir().exists(), "the folder is gone");
    let page =
        body_string(get_with_cookie(&app, "/account?done=forgotten-all", &marie).await).await;
    assert!(page.contains("L’AFG ne retient plus rien de vous."));
    assert!(
        !page.contains(r#"id="account-brain""#),
        "marie may not talk to it herself: with no brain left, nothing to show"
    );

    // The emergency token is no one, and has no brain to forget.
    let token = format!("{}; axgf_lang=fr", admin_cookie(&app).await);
    let resp = post_form_as(&app, &token, "/account/brain/forget", "all=1&confirm=on").await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

/// Sign an account of `app_with` in again, for a fresh session.
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
