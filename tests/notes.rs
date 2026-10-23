//! Notes and contributions (docs/adr/0014): written by administrators and
//! contributors, read by the family's members on the Family page and on the
//! records they concern, each opened in a window and offered as a PDF.

mod common;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use common::*;
use scraper::{Html, Selector};
use serde_json::{json, Value};
use tower::ServiceExt;

const LUC: &str = "d1000000-0000-4000-8000-000000000001";
const ALI: &str = "d1000000-0000-4000-8000-000000000002";
/// Private: no member reads this record, and so no member reads a note
/// about it.
const HID: &str = "d1000000-0000-4000-8000-000000000003";

fn bundle(tag: &str) -> common::Scratch {
    let dir = scratch(tag);
    let path = dir.join("notes.axgf");
    let person = |id: &str, name: &str, visibility: &str| {
        json!({"id": id, "type": "person", "axgf_version": "1.1",
               "identity": {"name": {"display": name, "components": []},
                            "is_living": false, "visibility": visibility},
               "birth": {"date": {"value": "1924", "precision": "year"}}})
    };
    let flat = json!({
        "manifest": {"axgf": "1.1", "family": {"name": "Fabre"}},
        "persons": {
            LUC: person(LUC, "Lucien Fabre", "public"),
            ALI: person(ALI, "Alice Fabre", "public"),
            HID: person(HID, "Hector Fabre", "private"),
        },
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

fn png(shade: u8) -> Vec<u8> {
    let img = image::RgbImage::from_pixel(8, 6, image::Rgb([shade, 90, 60]));
    let mut out = std::io::Cursor::new(Vec::new());
    img.write_to(&mut out, image::ImageFormat::Png)
        .expect("png");
    out.into_inner()
}

fn sel(s: &str) -> Selector {
    Selector::parse(s).unwrap()
}

/// A form body, each value percent-encoded.
fn form(fields: &[(&str, &str)]) -> String {
    let encode = |v: &str| {
        v.bytes()
            .map(|b| match b {
                b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                    (b as char).to_string()
                }
                _ => format!("%{b:02X}"),
            })
            .collect::<String>()
    };
    fields
        .iter()
        .map(|(k, v)| format!("{k}={}", encode(v)))
        .collect::<Vec<_>>()
        .join("&")
}

async fn account(app: &axum::Router, name: &str, role: &str) -> String {
    let made = post_form(
        app,
        "/admin/users",
        &format!("username={name}&password=correct-horse-battery&role={role}&email=&family_scope="),
        true,
    )
    .await;
    assert!(made.status().is_success() || made.status().is_redirection());
    let login = post_form(
        app,
        "/admin/login",
        &format!("username={name}&password=correct-horse-battery"),
        false,
    )
    .await;
    login
        .headers()
        .get(header::SET_COOKIE)
        .and_then(|v| v.to_str().ok())
        .and_then(|c| c.split(';').next())
        .unwrap_or_else(|| panic!("{name} signs in"))
        .to_string()
}

/// Writes a note and answers its id, from where the save sends the writer.
async fn write(app: &axum::Router, cookie: &str, fields: &[(&str, &str)]) -> String {
    let resp = post_form_as(app, cookie, "/admin/notes", &form(fields)).await;
    let status = resp.status();
    let location = resp
        .headers()
        .get(header::LOCATION)
        .and_then(|v| v.to_str().ok())
        .map(str::to_string);
    let body = body_string(resp).await;
    assert_eq!(status, StatusCode::SEE_OTHER, "the note is saved: {body}");
    location
        .and_then(|l| l.strip_prefix("/notes/").map(str::to_string))
        .expect("the save opens the note")
}

/// The archive as the server holds it, from its own export.
async fn documents(app: &axum::Router, cookie: &str) -> Value {
    let resp = get_with_cookie(app, "/admin/export", cookie).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let env = axgf_rs::import_bundle(&body_bytes(resp).await);
    env.data["documents"].clone()
}

async fn upload(
    app: &axum::Router,
    cookie: &str,
    filename: &str,
    bytes: Vec<u8>,
) -> axum::response::Response {
    let boundary = "----axgfnotesboundary";
    let mut body = format!(
        "--{boundary}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"{filename}\"\r\n\
         Content-Type: application/octet-stream\r\n\r\n"
    )
    .into_bytes();
    body.extend_from_slice(&bytes);
    body.extend_from_slice(format!("\r\n--{boundary}--\r\n").as_bytes());
    app.clone()
        .oneshot(
            Request::builder()
                .uri("/admin/notes/images")
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
        .unwrap()
}

#[tokio::test]
async fn a_member_reads_what_the_family_wrote_and_a_visitor_is_not_told_it_exists() {
    let src = bundle("notes-read-src");
    let (app, _p) = app_with_bundle("notes-read", &src);
    let admin = admin_cookie(&app).await;
    let vera = account(&app, "vera", "viewer").await;

    let id = write(
        &app,
        &admin,
        &[
            ("title", "Le moulin de Lucien"),
            ("category", "Souvenir"),
            ("date", "1952-06"),
            ("person", LUC),
            ("markdown", "# Le moulin\n\nIl y avait **un moulin**.\n\n<script>alert(1)</script>\n\n[piège](javascript:alert(1))"),
        ],
    )
    .await;

    // A visitor: not the note, not its PDF, and the Family page says only
    // that there is something for members.
    for uri in [
        format!("/notes/{id}"),
        format!("/notes/{id}?frame=1"),
        format!("/notes/{id}/pdf"),
    ] {
        let resp = get(&app, &uri).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND, "{uri}");
        assert!(!body_string(resp).await.contains("moulin"), "{uri}");
    }
    let about = body_string(get(&app, "/about").await).await;
    assert!(
        about.contains("notes-locked"),
        "the visitor is told to sign in"
    );
    assert!(!about.contains("Le moulin de Lucien"));
    let record = body_string(get(&app, &format!("/person/{LUC}")).await).await;
    assert!(
        !record.contains("person-notes"),
        "no notes section for a visitor"
    );

    // A member: the card on the Family page, under the page's head.
    let about = body_string(get_with_cookie(&app, "/about", &vera).await).await;
    let doc = Html::parse_document(&about);
    let cards: Vec<String> = doc
        .select(&sel("#notes .note-card .note-title"))
        .map(|e| e.text().collect())
        .collect();
    assert_eq!(cards, ["Le moulin de Lucien"]);
    assert!(about.contains("Souvenir"));
    assert!(
        doc.select(&sel(r#"#notes a[href="/about?letter=L#notes"]"#))
            .next()
            .is_some(),
        "the letter the note's title starts with is a link"
    );
    assert!(
        doc.select(&sel(r#"#notes a[href^="/admin/"]"#))
            .next()
            .is_none(),
        "a viewer is offered no way to write"
    );

    // The note in its window: the markup a reader may have, and nothing else.
    let frame =
        body_string(get_with_cookie(&app, &format!("/notes/{id}?frame=1"), &vera).await).await;
    assert!(
        !frame.contains("<html"),
        "the window's content is a fragment"
    );
    assert!(frame.contains("<strong>un moulin</strong>"));
    assert!(
        frame.contains("<h3>Le moulin</h3>"),
        "headings sit under the note's own: {frame}"
    );
    assert!(
        !frame.contains("<script>alert"),
        "raw HTML is shown as text: {frame}"
    );
    assert!(
        !frame.contains("javascript:"),
        "a script link is not a link: {frame}"
    );
    assert!(frame.contains(&format!(r#"href="/notes/{id}/pdf""#)));
    assert!(frame.contains(&format!(r#"href="/person/{LUC}""#)));
    assert!(!frame.contains("/edit"), "a viewer is not offered to edit");

    // Its own page, and its record's section.
    let page = get_with_cookie(&app, &format!("/notes/{id}"), &vera).await;
    assert_eq!(page.headers()[header::CACHE_CONTROL], "private, no-store");
    let page = body_string(page).await;
    assert!(page.contains("<html"));
    assert!(
        page.contains(env!("CARGO_PKG_VERSION")),
        "the footer still names the application's version"
    );
    let record = body_string(get_with_cookie(&app, &format!("/person/{LUC}"), &vera).await).await;
    let doc = Html::parse_document(&record);
    let mine: Vec<String> = doc
        .select(&sel("#person-notes .note-title"))
        .map(|e| e.text().collect())
        .collect();
    assert_eq!(mine, ["Le moulin de Lucien"]);
    let other = body_string(get_with_cookie(&app, &format!("/person/{ALI}"), &vera).await).await;
    assert!(
        !other.contains("person-notes"),
        "nothing about Alice, and a viewer cannot write"
    );

    // The PDF.
    let pdf = get_with_cookie(&app, &format!("/notes/{id}/pdf"), &vera).await;
    assert_eq!(pdf.status(), StatusCode::OK);
    assert_eq!(pdf.headers()[header::CONTENT_TYPE], "application/pdf");
    let disposition = pdf.headers()[header::CONTENT_DISPOSITION]
        .to_str()
        .unwrap()
        .to_string();
    assert!(disposition.contains(".pdf"), "{disposition}");
    let bytes = body_bytes(pdf).await;
    assert!(bytes.starts_with(b"%PDF"), "a PDF");
    assert!(bytes.len() > 2_000);

    // A viewer cannot write.
    let resp = get_with_cookie(&app, "/admin/notes/new", &vera).await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
    let resp = post_form_as(
        &app,
        &vera,
        "/admin/notes",
        &form(&[("title", "x"), ("markdown", "y")]),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);

    // Stored as the format's own document: text, a payload, the person linked.
    let docs = documents(&app, &admin).await;
    let d = &docs[&id];
    assert_eq!(d["mime_type"], "text/markdown");
    assert_eq!(d["caption"], "Le moulin de Lucien");
    assert_eq!(d["date"]["value"], "1952-06");
    assert_eq!(d["linked_to"][0]["entity_id"], LUC);
    assert_eq!(d["extensions"]["axgf-cms:note/v1"]["category"], "Souvenir");
}

#[tokio::test]
async fn a_note_about_a_hidden_person_is_hidden_with_them() {
    let src = bundle("notes-hidden-src");
    let (app, _p) = app_with_bundle("notes-hidden", &src);
    let admin = admin_cookie(&app).await;
    let vera = account(&app, "vera", "viewer").await;

    let hidden = write(
        &app,
        &admin,
        &[
            ("title", "Hector au front"),
            ("person", HID),
            ("markdown", "Secret."),
        ],
    )
    .await;
    let both = write(
        &app,
        &admin,
        &[
            ("title", "Hector et Lucien"),
            ("person", HID),
            ("person", LUC),
            ("markdown", "Ensemble."),
        ],
    )
    .await;
    let family = write(
        &app,
        &admin,
        &[("title", "La maison"), ("markdown", "Toute la famille.")],
    )
    .await;

    for id in [&hidden, &both] {
        assert_eq!(
            get_with_cookie(&app, &format!("/notes/{id}"), &vera)
                .await
                .status(),
            StatusCode::NOT_FOUND
        );
        assert_eq!(
            get_with_cookie(&app, &format!("/notes/{id}/pdf"), &vera)
                .await
                .status(),
            StatusCode::NOT_FOUND
        );
    }
    let about = body_string(get_with_cookie(&app, "/about", &vera).await).await;
    assert!(
        !about.contains("Hector"),
        "neither note, nor the name, reaches a viewer"
    );
    assert!(
        about.contains("La maison"),
        "a note about no one belongs to the family"
    );
    assert!(
        about.contains("Toute la famille")
            || about.contains("notes-family-wide")
            || about.contains("The whole family")
    );
    let record = body_string(get_with_cookie(&app, &format!("/person/{LUC}"), &vera).await).await;
    assert!(!record.contains("Hector et Lucien"));

    // An administrator reads them all.
    let about = body_string(get_with_cookie(&app, "/about", &admin).await).await;
    for t in ["Hector au front", "Hector et Lucien", "La maison"] {
        assert!(about.contains(t), "{t}");
    }
    assert_eq!(
        get_with_cookie(&app, &format!("/notes/{family}"), &vera)
            .await
            .status(),
        StatusCode::OK
    );
}

#[tokio::test]
async fn a_contributor_changes_only_the_notes_they_wrote() {
    let src = bundle("notes-own-src");
    let (app, _p) = app_with_bundle("notes-own", &src);
    let admin = admin_cookie(&app).await;
    let cora = account(&app, "cora", "contributor").await;

    let theirs = write(
        &app,
        &admin,
        &[("title", "Par l'administration"), ("markdown", "A.")],
    )
    .await;
    let mine = write(
        &app,
        &cora,
        &[("title", "Par Cora"), ("person", ALI), ("markdown", "B.")],
    )
    .await;

    // Not someone else's.
    assert_eq!(
        get_with_cookie(&app, &format!("/admin/notes/{theirs}/edit"), &cora)
            .await
            .status(),
        StatusCode::FORBIDDEN
    );
    let resp = post_form_as(
        &app,
        &cora,
        &format!("/admin/notes/{theirs}"),
        &form(&[
            ("base_version", "1"),
            ("title", "Repris"),
            ("markdown", "C."),
        ]),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
    let resp = post_form_as(
        &app,
        &cora,
        &format!("/admin/notes/{theirs}/delete"),
        "base_version=1",
    )
    .await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
    let frame =
        body_string(get_with_cookie(&app, &format!("/notes/{theirs}?frame=1"), &cora).await).await;
    assert!(!frame.contains("/edit"));
    assert!(frame.contains("Par l&#x27;administration") || frame.contains("Par l'administration"));

    // Their own: changed, then deleted. The version the edit is made from is
    // the one its form carries, as a browser sends it.
    let frame =
        body_string(get_with_cookie(&app, &format!("/notes/{mine}?frame=1"), &cora).await).await;
    assert!(frame.contains(&format!("/admin/notes/{mine}/edit")));
    let edit =
        body_string(get_with_cookie(&app, &format!("/admin/notes/{mine}/edit"), &cora).await).await;
    let base: String = Html::parse_document(&edit)
        .select(&sel(r#"form[data-note-form] input[name="base_version"]"#))
        .next()
        .and_then(|i| i.value().attr("value").map(str::to_string))
        .expect("the form carries its version");
    assert_eq!(base, "1", "the note's version, not the application's");
    let resp = post_form_as(
        &app,
        &cora,
        &format!("/admin/notes/{mine}"),
        &form(&[
            ("base_version", &base),
            ("title", "Par Cora, revu"),
            ("person", ALI),
            ("markdown", "B2."),
        ]),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    let docs = documents(&app, &admin).await;
    assert_eq!(docs[&mine]["caption"], "Par Cora, revu");
    assert_eq!(docs[&mine]["version_num"], 2);
    assert_eq!(docs[&mine]["created_by"], "cora");

    // The list says which are theirs to change.
    let list = body_string(get_with_cookie(&app, "/admin/notes", &cora).await).await;
    assert!(list.contains(&format!("/admin/notes/{mine}/edit")));
    assert!(!list.contains(&format!("/admin/notes/{theirs}/edit")));

    let resp = post_form_as(
        &app,
        &cora,
        &format!("/admin/notes/{mine}/delete"),
        "base_version=2",
    )
    .await;
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    let docs = documents(&app, &admin).await;
    assert!(docs.get(&mine).is_none(), "deleted");
    assert!(docs.get(&theirs).is_some());

    // An administrator changes anyone's.
    let resp = post_form_as(
        &app,
        &admin,
        &format!("/admin/notes/{theirs}"),
        &form(&[
            ("base_version", "1"),
            ("title", "Revu"),
            ("markdown", "A2."),
        ]),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
}

#[tokio::test]
async fn a_pasted_picture_waits_for_its_note_and_is_shown_to_members_only() {
    let src = bundle("notes-pics-src");
    let (app, _p) = app_with_bundle("notes-pics", &src);
    let admin = admin_cookie(&app).await;
    let vera = account(&app, "vera", "viewer").await;

    // Not a picture: refused.
    let resp = upload(&app, &admin, "x.txt", b"not a picture at all".to_vec()).await;
    assert_eq!(resp.status(), StatusCode::UNSUPPORTED_MEDIA_TYPE);
    // A viewer may not paste.
    assert_eq!(
        upload(&app, &vera, "a.png", png(10)).await.status(),
        StatusCode::FORBIDDEN
    );

    let resp = upload(&app, &admin, "moulin.png", png(10)).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let answer: Value = serde_json::from_slice(&body_bytes(resp).await).unwrap();
    let pending = answer[0]["ref"].as_str().unwrap().to_string();
    let url = answer[0]["url"].as_str().unwrap().to_string();
    assert!(pending.starts_with("axgf:pending/"));
    // Shown to the one who pasted it, and to nobody else.
    let mine = get_with_cookie(&app, &url, &admin).await;
    assert_eq!(mine.status(), StatusCode::OK);
    assert_eq!(mine.headers()[header::CONTENT_TYPE], "image/png");
    assert_ne!(
        get_with_cookie(&app, &url, &vera).await.status(),
        StatusCode::OK
    );

    // The note takes it: a document of its own, named by the text.
    let id = write(
        &app,
        &admin,
        &[
            ("title", "Le moulin"),
            ("person", LUC),
            ("markdown", &format!("Voici.\n\n![Le moulin]({pending})")),
        ],
    )
    .await;
    assert_eq!(
        get_with_cookie(&app, &url, &admin).await.status(),
        StatusCode::NOT_FOUND,
        "no longer waiting"
    );
    let docs = documents(&app, &admin).await;
    let pictures: Vec<&String> = docs
        .as_object()
        .unwrap()
        .iter()
        .filter(|(_, d)| d["extensions"]["axgf-cms:note-image/v1"]["note"] == id.as_str())
        .map(|(k, _)| k)
        .collect();
    assert_eq!(pictures.len(), 1);
    let pic = pictures[0].clone();
    assert_eq!(docs[&pic]["mime_type"], "image/png");
    assert_eq!(
        docs[&id]["extensions"]["axgf-cms:note/v1"]["images"],
        json!([pic])
    );

    // The reader sees it in the note; a visitor cannot fetch it.
    let frame =
        body_string(get_with_cookie(&app, &format!("/notes/{id}?frame=1"), &vera).await).await;
    assert!(
        frame.contains(&format!(r#"src="/document/{pic}/view""#)),
        "{frame}"
    );
    assert_eq!(
        get_with_cookie(&app, &format!("/document/{pic}/view"), &vera)
            .await
            .status(),
        StatusCode::OK
    );
    assert_ne!(
        get(&app, &format!("/document/{pic}/view")).await.status(),
        StatusCode::OK
    );
    assert_ne!(
        get(&app, &format!("/document/{pic}/raw")).await.status(),
        StatusCode::OK
    );
    // It is the note's, not a photograph of the person.
    let record =
        body_string(get_with_cookie(&app, &format!("/person/{LUC}?tab=files"), &admin).await).await;
    assert!(
        !record.contains(&format!("/document/{pic}/raw")),
        "not among the record's files"
    );
    // And it is in the PDF.
    let pdf = body_bytes(get_with_cookie(&app, &format!("/notes/{id}/pdf"), &vera).await).await;
    assert!(pdf.starts_with(b"%PDF"));
    assert!(
        pdf.windows(b"/Image".len()).any(|w| w == b"/Image"),
        "the picture is drawn"
    );

    // Taken out of the text, the picture goes with it.
    let resp = post_form_as(
        &app,
        &admin,
        &format!("/admin/notes/{id}"),
        &form(&[
            ("base_version", "1"),
            ("title", "Le moulin"),
            ("person", LUC),
            ("markdown", "Plus d'image."),
        ]),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    let docs = documents(&app, &admin).await;
    assert!(
        docs.get(&pic).is_none(),
        "the picture left with its mention"
    );
}

#[tokio::test]
async fn a_failed_save_loses_neither_the_text_nor_its_pasted_pictures() {
    let src = bundle("notes-fail-src");
    let (app, _p) = app_with_bundle("notes-fail", &src);
    let admin = admin_cookie(&app).await;

    // A date the form does not read, a person the archive does not hold.
    let resp = post_form_as(
        &app,
        &admin,
        "/admin/notes",
        &form(&[
            ("title", "Gardé"),
            ("date", "14/06/1952"),
            ("markdown", "Mon texte"),
        ]),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    let body = body_string(resp).await;
    assert!(
        body.contains("Gardé") && body.contains("Mon texte"),
        "what was written is still there"
    );
    assert!(
        body.contains("1952-06-14"),
        "the error says how to write it"
    );
    let resp = post_form_as(
        &app,
        &admin,
        "/admin/notes",
        &form(&[
            ("title", "Gardé"),
            ("person_add", "Personne Inconnue"),
            ("markdown", "Mon texte"),
        ]),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    let body = body_string(resp).await;
    assert!(body.contains("Personne Inconnue"));
    assert!(
        !body.contains("pick-error-"),
        "the reason is a sentence, not a key: {body}"
    );

    // Two writers on one note: the second's save fails, and keeps the picture
    // it pasted waiting for the next try.
    let id = write(&app, &admin, &[("title", "Deux mains"), ("markdown", "v1")]).await;
    let resp = post_form_as(
        &app,
        &admin,
        &format!("/admin/notes/{id}"),
        &form(&[
            ("base_version", "1"),
            ("title", "Deux mains"),
            ("markdown", "v2"),
        ]),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    let answer: Value =
        serde_json::from_slice(&body_bytes(upload(&app, &admin, "p.png", png(30)).await).await)
            .unwrap();
    let pending = answer[0]["ref"].as_str().unwrap().to_string();
    let url = answer[0]["url"].as_str().unwrap().to_string();
    let resp = post_form_as(
        &app,
        &admin,
        &format!("/admin/notes/{id}"),
        &form(&[
            ("base_version", "1"),
            ("title", "Deux mains"),
            ("markdown", &format!("v3 ![p]({pending})")),
        ]),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::CONFLICT);
    let body = body_string(resp).await;
    let token = pending.rsplit('/').next().unwrap();
    let doc = Html::parse_document(&body);
    let text: String = doc
        .select(&sel(r#"textarea[name="markdown"]"#))
        .next()
        .expect("the text")
        .text()
        .collect();
    assert_eq!(
        text,
        format!("v3 ![p](axgf:pending/{token})"),
        "the form shows what was written"
    );
    assert_eq!(
        get_with_cookie(&app, &url, &admin).await.status(),
        StatusCode::OK,
        "still waiting"
    );
    let docs = documents(&app, &admin).await;
    assert_eq!(docs[&id]["version_num"], 2);
    assert_eq!(
        docs.as_object().unwrap().len(),
        1,
        "no picture was stored for a save that failed"
    );
}

#[tokio::test]
async fn the_family_page_searches_and_reads_by_letter_without_script() {
    let src = bundle("notes-find-src");
    let (app, _p) = app_with_bundle("notes-find", &src);
    let admin = admin_cookie(&app).await;
    write(
        &app,
        &admin,
        &[
            ("title", "Arbre de mai"),
            ("category", "Récit"),
            ("markdown", "Le printemps."),
        ],
    )
    .await;
    write(
        &app,
        &admin,
        &[
            ("title", "Bateau du dimanche"),
            ("markdown", "La rivière, en été."),
        ],
    )
    .await;
    write(
        &app,
        &admin,
        &[
            ("title", "Écluse"),
            ("person", LUC),
            ("markdown", "Un soir."),
        ],
    )
    .await;

    let titles = |html: &str| -> Vec<String> {
        let doc = Html::parse_document(html);
        let mut v: Vec<String> = doc
            .select(&sel("#notes .note-card .note-title"))
            .map(|e| e.text().collect())
            .collect();
        v.sort();
        v
    };
    let all = body_string(get_with_cookie(&app, "/about", &admin).await).await;
    assert_eq!(
        titles(&all),
        ["Arbre de mai", "Bateau du dimanche", "Écluse"]
    );
    let doc = Html::parse_document(&all);
    let letters: Vec<String> = doc
        .select(&sel("#notes a.notes-letter[data-notes-letter]"))
        .filter_map(|a| a.value().attr("data-notes-letter").map(str::to_string))
        .collect();
    assert_eq!(
        letters,
        ["", "A", "B", "E"],
        "an accented initial files under its letter"
    );

    let b = body_string(get_with_cookie(&app, "/about?letter=B", &admin).await).await;
    assert_eq!(titles(&b), ["Bateau du dimanche"]);
    let e = body_string(get_with_cookie(&app, "/about?letter=E", &admin).await).await;
    assert_eq!(titles(&e), ["Écluse"]);
    // Search reads the text too, without accents or case.
    let q = body_string(get_with_cookie(&app, "/about?q=RIVIERE", &admin).await).await;
    assert_eq!(titles(&q), ["Bateau du dimanche"]);
    let q = body_string(get_with_cookie(&app, "/about?q=lucien", &admin).await).await;
    assert_eq!(
        titles(&q),
        ["Écluse"],
        "and the names of the people it concerns"
    );
    let none = body_string(get_with_cookie(&app, "/about?q=zzz", &admin).await).await;
    assert!(titles(&none).is_empty());
    let doc = Html::parse_document(&none);
    let empty = doc
        .select(&sel("[data-notes-empty]"))
        .next()
        .expect("the empty message");
    assert!(
        empty.value().attr("hidden").is_none(),
        "shown when nothing is found"
    );
}

#[tokio::test]
async fn the_dashboard_and_the_record_lead_a_writer_to_the_notes() {
    let src = bundle("notes-doors-src");
    let (app, _p) = app_with_bundle("notes-doors", &src);
    let cora = account(&app, "cora", "contributor").await;
    let dash = body_string(get_with_cookie(&app, "/admin", &cora).await).await;
    assert!(
        dash.contains(r#"href="/admin/notes""#),
        "a contributor finds the notes on the dashboard"
    );
    let record = body_string(get_with_cookie(&app, &format!("/person/{LUC}"), &cora).await).await;
    assert!(
        record.contains(&format!(r#"href="/admin/notes/new?person={LUC}""#)),
        "and on a record, the way to write about that person"
    );
    let form =
        body_string(get_with_cookie(&app, &format!("/admin/notes/new?person={LUC}"), &cora).await)
            .await;
    let doc = Html::parse_document(&form);
    let chosen: Vec<String> = doc
        .select(&sel(r#"[data-note-chosen] input[name="person"]"#))
        .filter_map(|i| i.value().attr("value").map(str::to_string))
        .collect();
    assert_eq!(chosen, [LUC], "the person is already chosen");
}
