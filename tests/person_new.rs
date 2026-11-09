//! A person created with what surrounds them (docs/adr/0039): the page the
//! search offers when nobody bears the name, and its one save — the person,
//! the places and the people it names that the archive does not hold yet,
//! the families that tie them — in one commit, or nothing at all.

mod common;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use common::*;
use serde_json::{json, Value};
use tower::ServiceExt;

const ANNA: &str = "a0000000-0000-4000-8000-000000000001";
const JAN: &str = "a0000000-0000-4000-8000-000000000002";
const KID: &str = "a0000000-0000-4000-8000-000000000003";
const LONE: &str = "a0000000-0000-4000-8000-000000000004";
const MARIA1: &str = "a0000000-0000-4000-8000-000000000005";
const MARIA2: &str = "a0000000-0000-4000-8000-000000000006";
const FAM: &str = "f0000000-0000-4000-8000-000000000001";
const KRAKOW: &str = "c0000000-0000-4000-8000-000000000001";

fn person(id: &str, name: &str) -> Value {
    json!({"id": id, "type": "person", "axgf_version": "1.1",
           "identity": {"name": {"display": name, "components": []},
                        "is_living": false, "visibility": "public"}})
}

/// Anna and Jan married, with Zofia; somebody with no family; two people
/// called Maria Kowal; one place.
fn bundle(tag: &str) -> common::Scratch {
    let dir = scratch(tag);
    let path = dir.join("new.axgf");
    let flat = json!({
        "manifest": {"axgf": "1.1"},
        "persons": {
            ANNA: person(ANNA, "Anna Lis"), JAN: person(JAN, "Jan Nowak"),
            KID: person(KID, "Zofia Nowak"), LONE: person(LONE, "Nobody Else"),
            MARIA1: person(MARIA1, "Maria Kowal"), MARIA2: person(MARIA2, "Maria Kowal"),
        },
        "families": {FAM: {"id": FAM, "type": "family", "axgf_version": "1.0",
            "union": {"type": "marriage", "persons": [
                {"person_id": ANNA, "role": "spouse"}, {"person_id": JAN, "role": "spouse"}]},
            "children": [{"person_id": KID}]}},
        "places": {KRAKOW: {"id": KRAKOW, "type": "place", "axgf_version": "1.0",
            "names": [{"lang": "und", "value": "Kraków", "is_primary": true}]}},
        "events": {}, "links": {}, "occupations": {}, "sources": {}, "documents": {}
    });
    std::fs::write(
        &path,
        axgf_cms::state::export_to_bytes(&flat.to_string()).expect("export"),
    )
    .expect("write");
    dir.pointing_at(path)
}

/// The archive as the bundle on disk now holds it.
fn archive(path: &std::path::Path) -> Value {
    let state = axgf_cms::AppState::load_or_create(path, TOKEN.to_string()).expect("load");
    state.read(Value::clone)
}

fn encode(pairs: &[(&str, &str)]) -> String {
    fn enc(s: &str) -> String {
        s.bytes()
            .map(|b| match b {
                b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                    (b as char).to_string()
                }
                b' ' => "+".into(),
                _ => format!("%{b:02X}"),
            })
            .collect()
    }
    pairs
        .iter()
        .map(|(k, v)| format!("{}={}", enc(k), enc(v)))
        .collect::<Vec<_>>()
        .join("&")
}

fn by_name<'a>(flat: &'a Value, name: &str) -> Vec<(&'a String, &'a Value)> {
    flat["persons"]
        .as_object()
        .unwrap()
        .iter()
        .filter(|(_, p)| p["identity"]["name"]["display"] == name)
        .collect()
}

fn location(resp: &axum::response::Response) -> String {
    resp.headers()
        .get(header::LOCATION)
        .and_then(|v| v.to_str().ok())
        .unwrap_or_default()
        .to_string()
}

#[tokio::test]
async fn the_search_offers_the_page_that_starts_from_the_name() {
    let src = bundle("pn-search-src");
    let (app, _p) = app_with_bundle("pn-search", &src);
    let cookie = admin_cookie(&app).await;

    let page = body_string(get_with_cookie(&app, "/search?q=Ewa+Brzoza", &cookie).await).await;
    assert!(
        page.contains(r#"href="/admin/person/new?name=Ewa%20Brzoza""#),
        "a writer is offered the person"
    );
    let reader = body_string(get(&app, "/search?q=Ewa+Brzoza").await).await;
    assert!(!reader.contains("/admin/person/new"), "a reader is not");

    let json: Value = serde_json::from_str(
        &body_string(get_with_cookie(&app, "/search.json?q=Ewa+Brzoza", &cookie).await).await,
    )
    .unwrap();
    assert_eq!(
        json["create"]["href"],
        "/admin/person/new?name=Ewa%20Brzoza"
    );
    assert!(json["create"]["label"]
        .as_str()
        .unwrap()
        .contains("Ewa Brzoza"));
    // Somebody bears that very name: nothing to create.
    let json: Value = serde_json::from_str(
        &body_string(get_with_cookie(&app, "/search.json?q=anna+lis", &cookie).await).await,
    )
    .unwrap();
    assert!(json["create"].is_null(), "{json}");

    let form =
        body_string(get_with_cookie(&app, "/admin/person/new?name=Ewa%20Brzoza", &cookie).await)
            .await;
    assert!(
        form.contains(r#"name="given" value="Ewa""#),
        "the first name"
    );
    assert!(
        form.contains(r#"name="family" value="Brzoza""#),
        "the family name"
    );
    for tab in ["pn-name", "pn-life", "pn-texts", "pn-family"] {
        assert!(
            form.contains(&format!(r##"href="#{tab}""##)),
            "the tab {tab}"
        );
    }
}

#[tokio::test]
async fn one_save_creates_the_person_and_everyone_named_around_them() {
    let src = bundle("pn-save-src");
    let (app, path) = app_with_bundle("pn-save", &src);
    let cookie = admin_cookie(&app).await;
    let before = archive(&path);

    let resp = post_form_as(
        &app,
        &cookie,
        "/admin/person/new",
        &encode(&[
            ("tab", "pn-family"),
            ("given", "Ewa"),
            ("nick", "Ewka"),
            ("family", "Nowak"),
            ("gender", "F"),
            ("living", "on"),
            ("birth_date", "1950"),
            ("birth_place", "Gdańsk"),
            ("death_place", "kraków"),
            ("bio", "**Strong** line\nsecond line"),
            ("father", "Jan Nowak"),
            ("mother", "Anna Lis"),
            ("sibling", "Piotr Nowak"),
            ("sibling", ""),
            ("u.0.partner", "Adam Brzoza"),
            ("u.0.type", "marriage"),
            ("u.0.child", "Ola Brzoza"),
            ("u.0.child", ""),
        ]),
    )
    .await;
    assert_eq!(
        resp.status(),
        StatusCode::SEE_OTHER,
        "{}",
        body_string(resp).await
    );
    let to = location(&resp);
    assert!(
        to.starts_with("/person/") && to.contains("?created="),
        "{to}"
    );
    let id = to["/person/".len()..]
        .split('?')
        .next()
        .unwrap()
        .to_string();

    let flat = archive(&path);
    let ewa = &flat["persons"][&id];
    assert_eq!(ewa["identity"]["name"]["display"], "Ewa “Ewka” Nowak");
    let parts: Vec<(&str, &str)> = ewa["identity"]["name"]["components"]
        .as_array()
        .unwrap()
        .iter()
        .map(|c| (c["type"].as_str().unwrap(), c["value"].as_str().unwrap()))
        .collect();
    assert_eq!(
        parts,
        [
            ("given_name", "Ewa"),
            ("nickname", "Ewka"),
            ("family_name", "Nowak")
        ]
    );
    assert_eq!(ewa["identity"]["gender"]["value"], "F");
    // A death place says the person died, whatever the box said.
    assert_eq!(ewa["identity"]["is_living"], false);

    // A new place, and the one the archive holds, found whatever its case.
    let birth = ewa["birth"]["place_id"].as_str().unwrap();
    assert_eq!(flat["places"][birth]["names"][0]["value"], "Gdańsk");
    assert_eq!(ewa["death"]["place_id"], KRAKOW);
    assert_eq!(
        flat["places"].as_object().unwrap().len(),
        before["places"].as_object().unwrap().len() + 1
    );

    // The parents' family takes her and her new brother, Zofia kept.
    let kids: Vec<&str> = flat["families"][FAM]["children"]
        .as_array()
        .unwrap()
        .iter()
        .map(|c| c["person_id"].as_str().unwrap())
        .collect();
    let piotr = by_name(&flat, "Piotr Nowak");
    assert_eq!(piotr.len(), 1);
    assert!(
        kids.contains(&KID) && kids.contains(&id.as_str()) && kids.contains(&piotr[0].0.as_str()),
        "{kids:?}"
    );

    // Her union: a new husband, a new child.
    let adam = by_name(&flat, "Adam Brzoza");
    let ola = by_name(&flat, "Ola Brzoza");
    assert_eq!((adam.len(), ola.len()), (1, 1));
    let union = flat["families"]
        .as_object()
        .unwrap()
        .values()
        .find(|f| {
            f["union"]["persons"]
                .as_array()
                .is_some_and(|ps| ps.iter().any(|p| p["person_id"] == *adam[0].0))
        })
        .expect("the union");
    assert_eq!(union["union"]["type"], "marriage");
    assert!(union["union"]["persons"]
        .as_array()
        .unwrap()
        .iter()
        .all(|p| p["role"] == "spouse"));
    assert_eq!(union["children"][0]["person_id"], *ola[0].0);
    assert_eq!(
        flat["persons"].as_object().unwrap().len(),
        before["persons"].as_object().unwrap().len() + 4
    );

    // The biography reads as it was written, line by line.
    assert_eq!(ewa["bio"], "**Strong** line\nsecond line");
    let record = body_string(get_with_cookie(&app, &to, &cookie).await).await;
    assert!(
        record.contains("<strong>Strong</strong> line<br />"),
        "Markdown, lines kept"
    );
    assert!(record.contains("Record created, with"), "the notice");
}

#[tokio::test]
async fn a_name_two_people_answer_to_is_refused_and_nothing_is_written() {
    let src = bundle("pn-amb-src");
    let (app, path) = app_with_bundle("pn-amb", &src);
    let cookie = admin_cookie(&app).await;
    let before = archive(&path);
    let resp = post_form_as(
        &app,
        &cookie,
        "/admin/person/new",
        &encode(&[
            ("given", "Tomasz"),
            ("father", "Maria Kowal"),
            ("birth_place", "Gdańsk"),
        ]),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    let body = body_string(resp).await;
    assert!(
        body.contains("More than one person answers to that"),
        "{body}"
    );
    assert!(
        body.contains(r#"value="Tomasz""#),
        "what was typed is shown again"
    );
    assert!(
        body.contains(r#"data-tab="pn-family""#),
        "on the tab of the field"
    );
    let after = archive(&path);
    assert_eq!(after["persons"], before["persons"]);
    assert_eq!(after["places"], before["places"]);
}

#[tokio::test]
async fn a_brother_or_sister_alone_names_the_family() {
    let src = bundle("pn-sib-src");
    let (app, path) = app_with_bundle("pn-sib", &src);
    let cookie = admin_cookie(&app).await;

    // Somebody in a family: theirs.
    let resp = post_form_as(
        &app,
        &cookie,
        "/admin/person/new",
        &encode(&[
            ("given", "Halina"),
            ("family", "Nowak"),
            ("sibling", "Zofia Nowak"),
        ]),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    let to = location(&resp);
    let id = to["/person/".len()..]
        .split('?')
        .next()
        .unwrap()
        .to_string();
    let flat = archive(&path);
    assert!(flat["families"][FAM]["children"]
        .as_array()
        .unwrap()
        .iter()
        .any(|c| c["person_id"] == id.as_str()));

    // Somebody in none: no family to join, and nothing written.
    let before = archive(&path);
    let resp = post_form_as(
        &app,
        &cookie,
        "/admin/person/new",
        &encode(&[("given", "Lech"), ("sibling", "Nobody Else")]),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    assert!(body_string(resp).await.contains("only through a parent"));
    assert_eq!(archive(&path)["persons"], before["persons"]);
}

/// The canonical 1×1 PNG.
const PNG: [u8; 67] = [
    0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52,
    0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x06, 0x00, 0x00, 0x00, 0x1F, 0x15, 0xC4,
    0x89, 0x00, 0x00, 0x00, 0x0A, 0x49, 0x44, 0x41, 0x54, 0x78, 0x9C, 0x63, 0x00, 0x01, 0x00, 0x00,
    0x05, 0x00, 0x01, 0x0D, 0x0A, 0x2D, 0xB4, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE,
    0x42, 0x60, 0x82,
];

#[tokio::test]
async fn the_portrait_comes_with_the_person() {
    const B: &str = "----axgfcmspersonnewboundary";
    let src = bundle("pn-pic-src");
    let (app, path) = app_with_bundle("pn-pic", &src);
    let cookie = admin_cookie(&app).await;
    let mut body = Vec::new();
    for (name, value) in [("given", "Irena"), ("family", "Lis")] {
        body.extend_from_slice(
            format!("--{B}\r\nContent-Disposition: form-data; name=\"{name}\"\r\n\r\n{value}\r\n")
                .as_bytes(),
        );
    }
    body.extend_from_slice(
        format!(
            "--{B}\r\nContent-Disposition: form-data; name=\"portrait\"; filename=\"irena.png\"\r\nContent-Type: image/png\r\n\r\n"
        )
        .as_bytes(),
    );
    body.extend_from_slice(&PNG);
    body.extend_from_slice(format!("\r\n--{B}--\r\n").as_bytes());
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/admin/person/new")
                .method("POST")
                .header(
                    header::CONTENT_TYPE,
                    format!("multipart/form-data; boundary={B}"),
                )
                .header(header::COOKIE, &cookie)
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        resp.status(),
        StatusCode::SEE_OTHER,
        "{}",
        body_string(resp).await
    );
    let to = location(&resp);
    let id = to["/person/".len()..]
        .split('?')
        .next()
        .unwrap()
        .to_string();
    let flat = archive(&path);
    let docs: Vec<&Value> = flat["documents"]
        .as_object()
        .unwrap()
        .values()
        .filter(|d| d["linked_to"][0]["entity_id"] == id.as_str())
        .collect();
    assert_eq!(docs.len(), 1, "one picture, linked to her");
    assert_eq!(docs[0]["mime_type"], "image/png");
    let record = body_string(get_with_cookie(&app, &to, &cookie).await).await;
    let doc_id = docs[0]["id"].as_str().unwrap();
    assert!(
        record.contains(&format!("/document/{doc_id}/thumb")),
        "the record shows it as her portrait"
    );
}

#[tokio::test]
async fn a_place_the_archive_lacks_is_said_to_be_a_place() {
    let src = bundle("pn-place-src");
    let (app, _p) = app_with_bundle("pn-place", &src);
    let cookie = admin_cookie(&app).await;
    let resp = post_form_as(
        &app,
        &cookie,
        "/admin/person",
        &encode(&[
            ("identity.name.display", "Somebody"),
            ("birth.place_id", "Nowhere"),
        ]),
    )
    .await;
    let body = body_string(resp).await;
    assert!(
        body.contains("No place of that name is in this archive"),
        "{body}"
    );
    assert!(!body.contains("No person of that name"));
}

#[tokio::test]
async fn a_biography_previews_with_its_lines() {
    let src = bundle("pn-preview-src");
    let (app, _p) = app_with_bundle("pn-preview", &src);
    let cookie = admin_cookie(&app).await;
    let html = body_string(
        post_form_as(
            &app,
            &cookie,
            "/admin/notes/preview",
            &encode(&[("markdown", "one\ntwo"), ("lines", "1")]),
        )
        .await,
    )
    .await;
    assert!(html.contains("one<br />"), "{html}");
    let note = body_string(
        post_form_as(
            &app,
            &cookie,
            "/admin/notes/preview",
            &encode(&[("markdown", "one\ntwo")]),
        )
        .await,
    )
    .await;
    assert!(
        !note.contains("<br"),
        "a note's soft breaks stay soft: {note}"
    );
}
