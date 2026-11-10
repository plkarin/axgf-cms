//! The AFG on a person's page (docs/adr/0031): a member the AFG is open to
//! asks about the person on the record's AFG tab, the AFG reads what that
//! member may read of the person and of the family, and the member finds
//! their conversations there again, to carry on, rename or delete. Nobody
//! reads another account's.
//!
//! A local HTTP server stands in for Ollama, streaming its answer a line at
//! a time and keeping what it was sent. Nothing here leaves the machine.

mod common;

use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use common::*;
use serde_json::{json, Value};
use tower::ServiceExt;

const PASSWORD: &str = "correct-horse-battery-staple";
const JEAN: &str = "a1000000-0000-4000-8000-000000000001";
const PAUL: &str = "a1000000-0000-4000-8000-000000000002";
const MARIE: &str = "a1000000-0000-4000-8000-000000000003";
const LUCIE: &str = "a1000000-0000-4000-8000-000000000004";
const ALBERT: &str = "a1000000-0000-4000-8000-000000000005";

/// What the fake was sent: the body of each request.
type Seen = Arc<Mutex<Vec<String>>>;

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
                if method == "POST" {
                    log.lock().unwrap().push(body);
                }
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

/// An Ollama that reads the dossier: it names the regiment when the notes
/// it was given name one, and says how many messages it was given.
fn ollama(method: &str, path: &str, body: &str) -> (u16, String) {
    if (method, path) == ("GET", "/api/tags") {
        return (200, r#"{"models":[{"name":"fausse-famille:1b"}]}"#.into());
    }
    let v: Value = serde_json::from_str(body).unwrap_or_default();
    let turns = v["messages"]
        .as_array()
        .map_or(0, |m| m.iter().filter(|m| m["role"] != "system").count());
    let words = if body.contains("152e régiment") {
        format!("D'après la note « Verdun », le 152e régiment d'infanterie ({turns} tours).")
    } else {
        format!("La note ne le dit pas ({turns} tours).")
    };
    let mut lines: Vec<String> = words
        .split(' ')
        .enumerate()
        .map(|(i, w)| {
            let piece = if i == 0 {
                w.to_string()
            } else {
                format!(" {w}")
            };
            json!({"model": "fausse-famille:1b",
                   "message": {"role": "assistant", "content": piece}, "done": false})
            .to_string()
        })
        .collect();
    lines.push(r#"{"model":"fausse-famille:1b","message":{"content":""},"done":true}"#.into());
    (200, lines.join("\n") + "\n")
}

fn person(id: &str, name: &str, born: &str, living: bool, visibility: &str) -> Value {
    json!({"id": id, "type": "person", "axgf_version": "1.1", "version_num": 1,
           "identity": {"name": {"display": name, "components": []},
                        "is_living": living, "visibility": visibility},
           "birth": {"date": {"value": born, "precision": "year"}}})
}

fn family(id: &str, parents: &[&str], kids: &[&str]) -> Value {
    json!({"id": id, "type": "family", "axgf_version": "1.1", "version_num": 1,
           "union": {"type": "marriage",
                     "persons": parents.iter().map(|p| json!({"person_id": p, "role": "spouse"})).collect::<Vec<_>>()},
           "children": kids.iter().map(|k| json!({"person_id": k})).collect::<Vec<_>>()})
}

/// Jean, his father Paul, his wife Marie, their daughter Lucie (living and
/// private), and Albert, of the family but not close to Jean. Accounts: ana
/// (an administrator, who writes the notes), vera and nora (members the AFG
/// is open to) and otto (one it is not). The AFG is the fake Ollama at
/// `url`, or none. Their sessions, reading French: ana, vera, nora, otto.
async fn family_app(tag: &str, url: Option<&str>) -> (axum::Router, Scratch, Vec<String>) {
    use axgf_cms::acl::{new_user, Acl, Role};
    let dir = scratch(tag);
    let path = dir.join("family.axgf");
    let flat = json!({
        "manifest": {"axgf": "1.1"},
        "persons": {
            JEAN: person(JEAN, "Jean Martin", "1890", false, "public"),
            PAUL: person(PAUL, "Paul Martin", "1860", false, "public"),
            MARIE: person(MARIE, "Marie Dupont", "1893", false, "public"),
            LUCIE: person(LUCIE, "Lucie Martin", "1950", true, "private"),
            ALBERT: person(ALBERT, "Albert Durand", "1901", false, "public"),
        },
        "families": {
            "f1": family("f1", &[PAUL], &[JEAN]),
            "f2": family("f2", &[JEAN, MARIE], &[LUCIE]),
        },
        "events": {}, "links": {}, "occupations": {}, "places": {},
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
        ("vera", Role::Viewer, true),
        ("nora", Role::Viewer, true),
        ("otto", Role::Viewer, false),
    ] {
        let mut u = new_user(name, PASSWORD, role).unwrap();
        u.afg = afg;
        acl.users.push(u);
    }
    acl.save(&Acl::path_for(&path)).unwrap();
    let app = build_app(&path).unwrap();
    let mut cookies = Vec::new();
    for name in ["ana", "vera", "nora", "otto"] {
        cookies.push(format!("{}; axgf_lang=fr", sign_in(&app, name).await));
    }
    if let Some(url) = url {
        let resp = post_form_as(
            &app,
            &cookies[0],
            "/admin/afg",
            &format!(
                "kind=ollama&url={}&model=fausse-famille%3A1b&timeout=60",
                url.replace(':', "%3A").replace('/', "%2F")
            ),
        )
        .await;
        assert_eq!(resp.status(), StatusCode::OK, "the AFG is set");
    }
    // The notes the AFG reads: one about Jean, one about his father.
    for (who, title, text) in [
        (
            JEAN,
            "Verdun",
            "Il servit au 152e régiment d'infanterie lors de la bataille de Verdun, en 1916.",
        ),
        (
            PAUL,
            "Carnet",
            "Paul tenait un carnet de route pendant ses voyages.",
        ),
    ] {
        let resp = post_form_as(
            &app,
            &cookies[0],
            "/admin/notes",
            &format!("title={title}&person={who}&markdown={}", enc(text)),
        )
        .await;
        assert!(resp.status().is_redirection(), "a note about {who}");
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

/// A question, sent as the composer sends it.
async fn ask(
    app: &axum::Router,
    cookie: &str,
    uri: &str,
    text: &str,
    json: bool,
) -> axum::http::Response<Body> {
    let b = "----axgfpersonchat";
    let body = format!(
        "--{b}\r\nContent-Disposition: form-data; name=\"text\"\r\n\r\n{text}\r\n--{b}--\r\n"
    );
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

/// A page, its attributes' slashes as a browser reads them.
async fn page(app: &axum::Router, uri: &str, cookie: &str) -> (StatusCode, String) {
    let resp = get_with_cookie(app, uri, cookie).await;
    let status = resp.status();
    (
        status,
        body_string(resp)
            .await
            .replace("&#x2f;", "/")
            .replace("&amp;", "&"),
    )
}

/// Wait until the conversation is answered: its state.
async fn answered(app: &axum::Router, cookie: &str, cid: &str) -> Value {
    for _ in 0..200 {
        let resp = get_with_cookie(
            app,
            &format!("/person/{JEAN}/afg/{cid}/state?after=0"),
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

fn roles(state: &Value) -> Vec<String> {
    state["messages"]
        .as_array()
        .unwrap()
        .iter()
        .map(|m| m["role"].as_str().unwrap().to_string())
        .collect()
}

/// Begin a conversation about Jean: its id.
async fn begin(app: &axum::Router, cookie: &str, text: &str) -> String {
    let resp = ask(app, cookie, &format!("/person/{JEAN}/afg"), text, true).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let v: Value = serde_json::from_str(&body_string(resp).await).unwrap();
    let to = v["redirect"].as_str().unwrap().to_string();
    let prefix = format!("/person/{JEAN}?tab=afg&c=");
    assert!(to.starts_with(&prefix), "{to}");
    to[prefix.len()..].to_string()
}

#[tokio::test]
async fn the_afg_tab_is_for_the_accounts_it_is_open_to() {
    let (url, _seen) = fake_service(ollama);
    let (app, dir, c) = family_app("person-chat-door", Some(&url)).await;
    let (vera, otto) = (&c[1], &c[3]);

    // Signed out, or the AFG not open to the account: no tab, no way in.
    let signed_out = body_string(get(&app, &format!("/person/{JEAN}?tab=afg")).await).await;
    assert!(!signed_out.contains("tab=afg"));
    assert!(!signed_out.contains("data-composer"));
    let (_, page_otto) = page(&app, &format!("/person/{JEAN}?tab=afg"), otto).await;
    assert!(!page_otto.contains("tab=afg") && !page_otto.contains("data-composer"));
    let resp = ask(&app, otto, &format!("/person/{JEAN}/afg"), "Bonjour", false).await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);

    // Open to vera: the tab, what the AFG reads, and the composer.
    let (status, record) = page(&app, &format!("/person/{JEAN}"), vera).await;
    assert_eq!(status, StatusCode::OK);
    assert!(
        record.contains(&format!("href=\"/person/{JEAN}?tab=afg\"")),
        "the tab"
    );
    let (status, tab) = page(&app, &format!("/person/{JEAN}?tab=afg"), vera).await;
    assert_eq!(status, StatusCode::OK);
    assert!(
        tab.contains("Demander à l’AFG à propos de Jean Martin"),
        "{tab}"
    );
    assert!(tab.contains(&format!("action=\"/person/{JEAN}/afg\"")));
    assert!(
        tab.contains("les notes (1), les proches (2)"),
        "the notes and the relatives vera may read"
    );
    assert!(tab.contains("Aucune conversation pour l’instant"));
    assert!(!tab.contains("as-attach"), "words only");
    assert!(tab.contains("/static/assistant.js"));
    // Looking makes nothing.
    assert!(!dir.with_extension("assistant").join("vera").exists());
}

#[tokio::test]
async fn a_question_reads_the_dossier_vera_may_read_and_is_answered_in_the_thread() {
    let (url, seen) = fake_service(ollama);
    let (app, dir, c) = family_app("person-chat-ask", Some(&url)).await;
    let vera = &c[1];

    let cid = begin(&app, vera, "Dans quel régiment était-il à Verdun ?").await;
    let state = answered(&app, vera, &cid).await;
    assert_eq!(roles(&state), ["user", "assistant"]);
    assert!(state["messages"][1]["html"]
        .as_str()
        .unwrap()
        .contains("152e régiment"));
    {
        let sent = seen.lock().unwrap();
        let body = sent.last().unwrap();
        // Jean's record and note, his close relatives and their notes, and
        // the family vera may read; never Lucie, whom vera may not read.
        for word in [
            "Jean Martin",
            "152e régiment",
            "Paul Martin",
            "Marie Dupont",
            "carnet de route",
            "Albert Durand",
            "Dans quel régiment",
        ] {
            assert!(body.contains(word), "{word} goes to the AFG");
        }
        assert!(!body.contains("Lucie"), "nobody vera may not read");
    }

    // The conversation, on the tab; its count beside the tab's name.
    let (_, tab) = page(&app, &format!("/person/{JEAN}?tab=afg&c={cid}"), vera).await;
    assert!(tab.contains("Dans quel régiment était-il à Verdun ?"));
    assert!(tab.contains("152e régiment"));
    assert!(tab.contains(&format!("data-state=\"/person/{JEAN}/afg/{cid}/state\"")));
    assert!(tab.contains(&format!("action=\"/person/{JEAN}/afg/{cid}\"")));
    assert!(tab.contains("tab-count\">1<"), "one conversation");

    // Carried on: the AFG is given the conversation so far.
    let resp = ask(
        &app,
        vera,
        &format!("/person/{JEAN}/afg/{cid}"),
        "Et son père ?",
        true,
    )
    .await;
    assert_eq!(resp.status(), StatusCode::OK);
    let state = answered(&app, vera, &cid).await;
    assert_eq!(roles(&state), ["user", "assistant", "user", "assistant"]);
    assert!(state["messages"][3]["html"]
        .as_str()
        .unwrap()
        .contains("(3 tours)"));

    // Kept in vera's own space, in the person's project, which the
    // assistant's page does not list among its projects.
    let space = dir.with_extension("assistant").join("vera");
    assert!(space.join("owner.json").is_file());
    let (_, home) = page(&app, "/account/assistant", vera).await;
    // What the page lists, before the guide and the names its examples use.
    let listed = home.split(r#"id="guide""#).next().unwrap_or_default();
    assert!(home.contains(r#"id="guide""#), "the guide is there");
    assert!(
        !listed.contains("Jean Martin"),
        "not one of the assistant's projects"
    );
}

#[tokio::test]
async fn nobody_reads_another_accounts_conversations() {
    let (url, _seen) = fake_service(ollama);
    let (app, _dir, c) = family_app("person-chat-others", Some(&url)).await;
    let (ana, vera, nora) = (&c[0], &c[1], &c[2]);
    let cid = begin(&app, vera, "Où est-il né ?").await;
    answered(&app, vera, &cid).await;

    for other in [nora, ana] {
        let (status, tab) = page(&app, &format!("/person/{JEAN}?tab=afg&c={cid}"), other).await;
        assert_eq!(status, StatusCode::OK);
        assert!(!tab.contains("Où est-il né ?"), "not theirs to read");
        assert!(tab.contains("Aucune conversation pour l’instant"));
        let resp = get_with_cookie(
            &app,
            &format!("/person/{JEAN}/afg/{cid}/state?after=0"),
            other,
        )
        .await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
        let resp = ask(
            &app,
            other,
            &format!("/person/{JEAN}/afg/{cid}"),
            "Et ?",
            false,
        )
        .await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
        let resp = post_form_as(
            &app,
            other,
            &format!("/person/{JEAN}/afg/{cid}/delete"),
            "confirm=on",
        )
        .await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }
    let state = answered(&app, vera, &cid).await;
    assert_eq!(roles(&state), ["user", "assistant"], "still vera's, whole");
}

#[tokio::test]
async fn a_conversation_is_renamed_and_deleted_and_so_are_all_of_them() {
    let (url, _seen) = fake_service(ollama);
    let (app, _dir, c) = family_app("person-chat-manage", Some(&url)).await;
    let vera = &c[1];
    let first = begin(&app, vera, "Première question").await;
    answered(&app, vera, &first).await;
    let second = begin(&app, vera, "Seconde question").await;
    answered(&app, vera, &second).await;

    let resp = post_form_as(
        &app,
        vera,
        &format!("/person/{JEAN}/afg/{first}/edit"),
        "title=Le+r%C3%A9giment",
    )
    .await;
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    let (_, tab) = page(&app, &format!("/person/{JEAN}?tab=afg"), vera).await;
    assert!(tab.contains("Le régiment") && tab.contains("Seconde question"));
    assert!(tab.contains("tab-count\">2<"));

    // A deletion is confirmed first.
    let resp = post_form_as(
        &app,
        vera,
        &format!("/person/{JEAN}/afg/{first}/delete"),
        "",
    )
    .await;
    assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);
    let resp = post_form_as(
        &app,
        vera,
        &format!("/person/{JEAN}/afg/{first}/delete"),
        "confirm=on",
    )
    .await;
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    let (_, tab) = page(&app, &format!("/person/{JEAN}?tab=afg&done=deleted"), vera).await;
    assert!(!tab.contains("Le régiment") && tab.contains("Seconde question"));

    // And all of them at once.
    let resp = post_form_as(&app, vera, &format!("/person/{JEAN}/afg/delete"), "").await;
    assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);
    let resp = post_form_as(
        &app,
        vera,
        &format!("/person/{JEAN}/afg/delete"),
        "confirm=on",
    )
    .await;
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    let (_, tab) = page(
        &app,
        &format!("/person/{JEAN}?tab=afg&done=deleted-all"),
        vera,
    )
    .await;
    assert!(!tab.contains("Seconde question"));
    assert!(tab.contains("Vos conversations sur cette personne sont supprimées."));
}

#[tokio::test]
async fn a_person_the_account_may_not_read_has_no_afg() {
    let (url, seen) = fake_service(ollama);
    let (app, _dir, c) = family_app("person-chat-private", Some(&url)).await;
    let vera = &c[1];
    let (_, tab) = page(&app, &format!("/person/{LUCIE}?tab=afg"), vera).await;
    assert!(!tab.contains("data-composer"));
    let resp = ask(
        &app,
        vera,
        &format!("/person/{LUCIE}/afg"),
        "Qui est-elle ?",
        false,
    )
    .await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    assert!(seen.lock().unwrap().is_empty(), "nothing sent");
}

#[tokio::test]
async fn without_the_afgs_computing_nothing_is_sent() {
    let (app, dir, c) = family_app("person-chat-off", None).await;
    let vera = &c[1];
    let (_, tab) = page(&app, &format!("/person/{JEAN}?tab=afg"), vera).await;
    assert!(tab.contains("Le calcul de l’AFG n’est pas encore réglé"));
    let resp = ask(&app, vera, &format!("/person/{JEAN}/afg"), "Bonjour", true).await;
    assert_eq!(resp.status(), StatusCode::CONFLICT);
    assert!(
        !dir.with_extension("assistant")
            .join("vera")
            .join("projects")
            .exists()
            || std::fs::read_dir(
                dir.with_extension("assistant")
                    .join("vera")
                    .join("projects")
            )
            .unwrap()
            .next()
            .is_none()
    );
}

/// A picture made here, for the test: never a photograph of the family.
fn png(w: u32, h: u32) -> Vec<u8> {
    let img = image::RgbImage::from_fn(w, h, |x, y| {
        image::Rgb([(x % 256) as u8, (y % 256) as u8, 90])
    });
    let mut out = std::io::Cursor::new(Vec::new());
    image::DynamicImage::ImageRgb8(img)
        .write_to(&mut out, image::ImageFormat::Png)
        .unwrap();
    out.into_inner()
}

/// A file attached to a person's record by an administrator, as the
/// record's editor sends it.
async fn attach(
    app: &axum::Router,
    cookie: &str,
    person: &str,
    name: &str,
    mime: &str,
    bytes: &[u8],
    caption: &str,
) {
    let b = "----axgfpersonchatfile";
    let mut body = format!(
        "--{b}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"{name}\"\r\n\
         Content-Type: {mime}\r\n\r\n"
    )
    .into_bytes();
    body.extend_from_slice(bytes);
    body.extend_from_slice(
        format!(
            "\r\n--{b}\r\nContent-Disposition: form-data; name=\"caption\"\r\n\r\n{caption}\r\n--{b}--\r\n"
        )
        .as_bytes(),
    );
    let req = Request::builder()
        .uri(format!("/admin/person/{person}/document"))
        .method("POST")
        .header(header::COOKIE, cookie)
        .header(
            header::CONTENT_TYPE,
            format!("multipart/form-data; boundary={b}"),
        )
        .body(Body::from(body))
        .unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    assert!(
        resp.status().is_success() || resp.status().is_redirection(),
        "{name} attached: {}",
        resp.status()
    );
}

/// The AFG reads the photographs of the person's record with each message
/// (the operator's request of 6 November 2026, docs/adr/0033): given as
/// pictures with the message being answered, sized for a model, named in
/// the dossier; a film is not given, nor a relative's photograph.
#[tokio::test]
async fn the_afg_reads_the_photographs_of_the_record_and_not_its_films() {
    let (url, seen) = fake_service(ollama);
    let (app, _dir, c) = family_app("person-chat-photos", Some(&url)).await;
    let (ana, vera) = (&c[0], &c[1]);
    attach(
        &app,
        ana,
        JEAN,
        "uniforme.png",
        "image/png",
        &png(2000, 1000),
        "Jean en uniforme, 1916",
    )
    .await;
    let mut film = vec![0, 0, 0, 0x20];
    film.extend_from_slice(b"ftypisom");
    film.extend_from_slice(&[0; 64]);
    attach(
        &app,
        ana,
        JEAN,
        "mariage.mp4",
        "video/mp4",
        &film,
        "Le mariage, 1919",
    )
    .await;
    attach(
        &app,
        ana,
        PAUL,
        "paul.png",
        "image/png",
        &png(300, 200),
        "Paul à Lyon",
    )
    .await;

    // The tab says how many go with each message.
    let (_, tab) = page(&app, &format!("/person/{JEAN}?tab=afg"), vera).await;
    assert!(
        tab.contains("la fiche, les photos (1), les PDF (0), les notes (1)"),
        "{tab}"
    );

    let cid = begin(&app, vera, "Que montre la photo ?").await;
    answered(&app, vera, &cid).await;
    let images = |body: &str| -> Vec<Vec<Value>> {
        let v: Value = serde_json::from_str(body).unwrap();
        v["messages"]
            .as_array()
            .unwrap()
            .iter()
            .map(|m| m["images"].as_array().cloned().unwrap_or_default())
            .collect()
    };
    let first = seen.lock().unwrap().last().unwrap().clone();
    let given = images(&first);
    // [system, the question]: the picture with the question.
    assert_eq!(given.len(), 2);
    assert!(given[0].is_empty());
    assert_eq!(
        given[1].len(),
        1,
        "one photograph: not the film, not Paul's"
    );
    use base64::Engine;
    let jpeg = base64::engine::general_purpose::STANDARD
        .decode(given[1][0].as_str().unwrap())
        .unwrap();
    assert!(jpeg.starts_with(&[0xff, 0xd8]), "a JPEG");
    let img = image::load_from_memory(&jpeg).unwrap();
    assert_eq!(
        (img.width(), img.height()),
        (1568, 784),
        "sized for a model"
    );
    // Named in the dossier, with its caption; the film is not given.
    assert!(first.contains("## The photographs of Jean Martin"));
    assert!(first.contains("1. \\\"uniforme.png\\\""), "{first}");
    assert!(first.contains("Jean en uniforme, 1916"));
    assert!(!first.contains("1. \\\"mariage.mp4\\\""));

    // Carried on: the photograph goes with the new message only.
    let resp = ask(
        &app,
        vera,
        &format!("/person/{JEAN}/afg/{cid}"),
        "Et l'uniforme ?",
        true,
    )
    .await;
    assert_eq!(resp.status(), StatusCode::OK);
    answered(&app, vera, &cid).await;
    let second = seen.lock().unwrap().last().unwrap().clone();
    let given = images(&second);
    assert_eq!(given.len(), 4, "[system, question, answer, question]");
    assert!(given[1].is_empty() && given[2].is_empty());
    assert_eq!(given[3].len(), 1);
}

/// A PDF made here, for the test: two empty pages.
const TWO_PAGES: &[u8] = b"%PDF-1.4\n1 0 obj << /Type /Catalog /Pages 2 0 R >> endobj\n\
2 0 obj << /Type /Pages /Kids [3 0 R 4 0 R] /Count 2 >> endobj\n\
3 0 obj << /Type /Page /Parent 2 0 R >> endobj\n4 0 obj << /Type /Page /Parent 2 0 R >> endobj\n\
trailer << /Root 1 0 R >>\n%%EOF\n";

/// The AFG reads the PDF documents of the person's record with each message
/// (the operator's answer of 6 November 2026, docs/adr/0040): counted on the
/// tab, listed in the dossier with their pages, and joined to the question,
/// which a way of computing that reads no PDF says it could not read.
#[tokio::test]
async fn the_afg_is_given_the_pdf_documents_of_the_record() {
    let (url, seen) = fake_service(ollama);
    let (app, _dir, c) = family_app("person-chat-pdfs", Some(&url)).await;
    let (ana, vera) = (&c[0], &c[1]);
    attach(
        &app,
        ana,
        JEAN,
        "acte.pdf",
        "application/pdf",
        TWO_PAGES,
        "Acte de naissance",
    )
    .await;
    attach(
        &app,
        ana,
        PAUL,
        "paul.pdf",
        "application/pdf",
        TWO_PAGES,
        "Livret",
    )
    .await;

    let (_, tab) = page(&app, &format!("/person/{JEAN}?tab=afg"), vera).await;
    assert!(tab.contains("les photos (0), les PDF (1)"), "{tab}");

    let cid = begin(&app, vera, "Que dit l'acte ?").await;
    answered(&app, vera, &cid).await;
    let sent = seen.lock().unwrap().last().unwrap().clone();
    assert!(
        sent.contains("## The PDF documents of Jean Martin"),
        "{sent}"
    );
    assert!(sent.contains("P1. \\\"acte.pdf\\\""), "{sent}");
    assert!(sent.contains("2 pages) — Acte de naissance"), "{sent}");
    assert!(!sent.contains("paul.pdf"), "not a relative's");
    // Ollama reads no PDF: the question says one was sent.
    assert!(
        sent.contains("A PDF document \\\"P1. acte.pdf\\\" was sent here"),
        "{sent}"
    );
}

/// An Ollama whose model takes no picture: a request with one is refused,
/// as such a model's is.
fn ollama_without_pictures(method: &str, path: &str, body: &str) -> (u16, String) {
    if method == "POST" && body.contains("\"images\"") {
        return (
            400,
            r#"{"error":"this model does not support images"}"#.into(),
        );
    }
    ollama(method, path, body)
}

/// A model that takes no picture still answers: the question is asked again
/// without the photographs, and the AFG is told they could not be given.
#[tokio::test]
async fn a_model_that_takes_no_picture_is_asked_again_without_them() {
    let (url, seen) = fake_service(ollama_without_pictures);
    let (app, _dir, c) = family_app("person-chat-no-pictures", Some(&url)).await;
    let (ana, vera) = (&c[0], &c[1]);
    attach(
        &app,
        ana,
        JEAN,
        "uniforme.png",
        "image/png",
        &png(64, 48),
        "Jean en uniforme",
    )
    .await;

    let cid = begin(&app, vera, "Dans quel régiment était-il ?").await;
    let state = answered(&app, vera, &cid).await;
    assert_eq!(
        roles(&state),
        ["user", "assistant"],
        "answered all the same"
    );
    assert!(state["messages"][1]["html"]
        .as_str()
        .unwrap()
        .contains("152e régiment"));
    let sent = seen.lock().unwrap().clone();
    let (refused, again) = (&sent[sent.len() - 2], &sent[sent.len() - 1]);
    assert!(refused.contains("\"images\""));
    assert!(!again.contains("\"images\""));
    assert!(again.contains("The photographs of the record could not be given"));
}
