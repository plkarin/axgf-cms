//! Portraits an AI reworked (v0.5.0, docs/adr/0009): up to three pictures
//! beside the face, for the family's members only, and never one of the
//! record's photographs — not its picture, not in its gallery or its files.

mod common;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use common::*;
use scraper::{Html, Selector};
use serde_json::{json, Value};
use tower::ServiceExt;

const LUC: &str = "c1000000-0000-4000-8000-000000000001";
const ALL: &str = "c1000000-0000-4000-8000-000000000002";

fn bundle(tag: &str) -> common::Scratch {
    let dir = scratch(tag);
    let path = dir.join("ai.axgf");
    let person = |id: &str, name: &str| {
        json!({"id": id, "type": "person", "axgf_version": "1.1",
               "identity": {"name": {"display": name, "components": []},
                            "is_living": false, "visibility": "public"},
               "birth": {"date": {"value": "1924", "precision": "year"}}})
    };
    let flat = json!({
        "manifest": {"axgf": "1.1"},
        "persons": {LUC: person(LUC, "Lucien Fabre"), ALL: person(ALL, "Alice Fabre")},
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
    let img = image::RgbImage::from_pixel(6, 6, image::Rgb([shade, 90, 60]));
    let mut out = std::io::Cursor::new(Vec::new());
    img.write_to(&mut out, image::ImageFormat::Png)
        .expect("png");
    out.into_inner()
}

fn sel(s: &str) -> Selector {
    Selector::parse(s).unwrap()
}

/// A multipart POST: text fields, then files under their field names.
async fn multipart(
    app: &axum::Router,
    cookie: &str,
    uri: &str,
    fields: &[(&str, &str)],
    files: &[(&str, &str, Vec<u8>)],
) -> axum::response::Response {
    let boundary = "----axgfaiboundary";
    let mut body = Vec::new();
    for (k, v) in fields {
        body.extend_from_slice(
            format!("--{boundary}\r\nContent-Disposition: form-data; name=\"{k}\"\r\n\r\n{v}\r\n")
                .as_bytes(),
        );
    }
    for (name, filename, bytes) in files {
        body.extend_from_slice(
            format!(
                "--{boundary}\r\nContent-Disposition: form-data; name=\"{name}\"; \
                 filename=\"{filename}\"\r\nContent-Type: application/octet-stream\r\n\r\n"
            )
            .as_bytes(),
        );
        body.extend_from_slice(bytes);
        body.extend_from_slice(b"\r\n");
    }
    body.extend_from_slice(format!("--{boundary}--\r\n").as_bytes());
    app.clone()
        .oneshot(
            Request::builder()
                .uri(uri)
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

/// The archive as the server holds it, from its own export.
async fn documents(app: &axum::Router, cookie: &str) -> Value {
    let resp = get_with_cookie(app, "/admin/export", cookie).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let bytes = body_bytes(resp).await;
    let env = axgf_rs::import_bundle(&bytes);
    env.data["documents"].clone()
}

fn ai_ids(docs: &Value, person: &str) -> Vec<String> {
    let mut out: Vec<String> = docs
        .as_object()
        .into_iter()
        .flatten()
        .filter(|(_, d)| {
            axgf_cms::ai_portrait::is_ai_portrait(d)
                && d["linked_to"][0]["entity_id"].as_str() == Some(person)
        })
        .map(|(id, _)| id.clone())
        .collect();
    out.sort();
    out
}

async fn viewer_cookie(app: &axum::Router) -> String {
    let made = post_form(
        app,
        "/admin/users",
        "username=vera&password=correct-horse-battery&role=viewer&email=&family_scope=",
        true,
    )
    .await;
    assert!(made.status().is_success() || made.status().is_redirection());
    let login = post_form(
        app,
        "/admin/login",
        "username=vera&password=correct-horse-battery",
        false,
    )
    .await;
    login
        .headers()
        .get(header::SET_COOKIE)
        .and_then(|v| v.to_str().ok())
        .and_then(|c| c.split(';').next())
        .expect("vera signs in")
        .to_string()
}

#[tokio::test]
async fn an_editor_adds_three_at_most_marked_as_the_ais_and_never_a_photograph() {
    let src = bundle("ai-src");
    let (app, _p) = app_with_bundle("ai", &src);
    let admin = admin_cookie(&app).await;

    // Nothing yet: an editor is offered the way to add them, in the
    // masthead, and the page that does it.
    let page = body_string(get_with_cookie(&app, &format!("/person/{LUC}"), &admin).await).await;
    assert!(page.contains(r#"<header class="person-head has-ai">"#));
    assert!(page.contains(&format!(r#"href="/admin/person/{LUC}/ai-portraits""#)));
    assert!(page.contains("Upload AI-reworked pictures"));
    let form = body_string(
        get_with_cookie(&app, &format!("/admin/person/{LUC}/ai-portraits"), &admin).await,
    )
    .await;
    let main = &form[form.find("<main").unwrap_or(0)..];
    assert!(
        form.contains(r#"name="file""#) && form.contains(" multiple"),
        "the page offers several files at once: {}",
        main.chars().take(1500).collect::<String>()
    );

    // A real photograph first, the kind a portrait is made from.
    let photo = multipart(
        &app,
        &admin,
        &format!("/admin/person/{LUC}/document"),
        &[("caption", "Lucien, 1949")],
        &[("file", "lucien.png", png(40))],
    )
    .await;
    assert_eq!(photo.status(), StatusCode::SEE_OTHER);
    let photo_id = documents(&app, &admin)
        .await
        .as_object()
        .unwrap()
        .keys()
        .next()
        .unwrap()
        .clone();

    // Two at once, made from that photograph, with the tool named.
    let resp = multipart(
        &app,
        &admin,
        &format!("/admin/person/{LUC}/ai-portraits"),
        &[("generator", "Restorer 2"), ("derived_from", &photo_id)],
        &[("file", "a.png", png(10)), ("file", "b.png", png(20))],
    )
    .await;
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    assert_eq!(
        resp.headers()["location"],
        format!("/person/{LUC}#ai-portraits").as_str()
    );
    let docs = documents(&app, &admin).await;
    let ai = ai_ids(&docs, LUC);
    assert_eq!(ai.len(), 2);
    for id in &ai {
        let d = &docs[id];
        // Marked three ways, none of them this application's alone, and no
        // claim of being a photograph.
        assert_eq!(d["document_type"], "other");
        assert_eq!(d["linked_to"][0]["role"], "ai_portrait");
        assert_eq!(d["tags"], json!(["ai-generated"]));
        let ext = &d["extensions"]["axgf-cms:ai-portrait/v1"];
        assert_eq!(ext["generator"], "Restorer 2");
        assert_eq!(ext["derived_from_id"], photo_id.as_str());
    }

    // On the record: a band of its own, each picture marked; the photograph
    // is still the face, alone in the gallery.
    let page = body_string(get_with_cookie(&app, &format!("/person/{LUC}"), &admin).await).await;
    let doc = Html::parse_document(&page);
    let shots: Vec<String> = doc
        .select(&sel(".person-ai-list a.person-ai-shot"))
        .map(|a| a.value().attr("href").unwrap().to_string())
        .collect();
    assert_eq!(shots.len(), 2);
    assert_eq!(doc.select(&sel(".person-ai-list .ai-badge")).count(), 2);
    assert!(page.contains("not pictures of the time"));
    assert!(page.contains("Manage the AI pictures"));
    let face = doc
        .select(&sel("a.person-avatar"))
        .next()
        .expect("the photograph is the face")
        .value()
        .attr("href")
        .unwrap()
        .to_string();
    assert_eq!(face, format!("/document/{photo_id}/view"));
    let media =
        body_string(get_with_cookie(&app, &format!("/person/{LUC}?tab=media"), &admin).await).await;
    for id in &ai {
        assert!(
            !media.contains(&format!("/document/{id}/view\" class")),
            "not in the gallery"
        );
        let body = media.split("record-body").nth(1).unwrap_or_default();
        assert!(
            !body.contains(id.as_str()),
            "{id} is not one of the record's files"
        );
    }
    // …nor in the editor's Media tab, nor on the picker of the face.
    let editor =
        body_string(get_with_cookie(&app, &format!("/admin/person/{LUC}/edit"), &admin).await)
            .await;
    let picker =
        body_string(get_with_cookie(&app, &format!("/admin/person/{LUC}/avatar"), &admin).await)
            .await;
    for id in &ai {
        assert!(
            !editor.contains(&format!("doc.{id}")),
            "{id} on the Media tab"
        );
        assert!(!picker.contains(id.as_str()), "{id} offered as the face");
    }

    // Two more would make four: refused, and nothing is written.
    let resp = multipart(
        &app,
        &admin,
        &format!("/admin/person/{LUC}/ai-portraits"),
        &[],
        &[("file", "c.png", png(30)), ("file", "d.png", png(50))],
    )
    .await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    assert_eq!(ai_ids(&documents(&app, &admin).await, LUC).len(), 2);

    // Only a picture: a PDF is refused.
    let resp = multipart(
        &app,
        &admin,
        &format!("/admin/person/{LUC}/ai-portraits"),
        &[],
        &[("file", "notes.pdf", b"%PDF-1.4\n%fake\n".to_vec())],
    )
    .await;
    assert_eq!(resp.status(), StatusCode::UNSUPPORTED_MEDIA_TYPE);

    // One out and two in, in one save: three, the most a record holds; and
    // the one removed is gone from the archive, not left behind unlinked.
    let gone = ai[0].clone();
    let resp = multipart(
        &app,
        &admin,
        &format!("/admin/person/{LUC}/ai-portraits"),
        &[("remove", &gone), ("derived_from", "no-such-photo")],
        &[("file", "e.png", png(60)), ("file", "f.png", png(70))],
    )
    .await;
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    let docs = documents(&app, &admin).await;
    let now = ai_ids(&docs, LUC);
    assert_eq!(now.len(), 3);
    assert!(docs.get(&gone).is_none(), "removed, not detached");
    let fresh: Vec<&String> = now.iter().filter(|id| !ai.contains(id)).collect();
    for id in fresh {
        assert!(
            docs[id]["extensions"]["axgf-cms:ai-portrait/v1"]
                .get("derived_from_id")
                .is_none(),
            "an id the page did not offer is not recorded"
        );
    }
    let form = body_string(
        get_with_cookie(&app, &format!("/admin/person/{LUC}/ai-portraits"), &admin).await,
    )
    .await;
    assert!(!form.contains(r#"name="file""#), "no room left, no slot");
    assert!(form.contains("three AI pictures"));

    // Someone else's portrait cannot be removed from here.
    let resp = multipart(
        &app,
        &admin,
        &format!("/admin/person/{ALL}/ai-portraits"),
        &[("remove", &now[0])],
        &[],
    )
    .await;
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    assert_eq!(ai_ids(&documents(&app, &admin).await, LUC).len(), 3);
}

#[tokio::test]
async fn only_a_signed_in_member_sees_them_and_only_an_editor_adds_them() {
    let src = bundle("ai-members-src");
    let (app, _p) = app_with_bundle("ai-members", &src);
    let admin = admin_cookie(&app).await;
    let resp = multipart(
        &app,
        &admin,
        &format!("/admin/person/{LUC}/ai-portraits"),
        &[],
        &[("file", "a.png", png(10))],
    )
    .await;
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    let id = ai_ids(&documents(&app, &admin).await, LUC)
        .pop()
        .expect("one portrait");

    // A visitor: no band, and the bytes are refused at every door.
    let page = body_string(get(&app, &format!("/person/{LUC}")).await).await;
    assert!(!page.contains("person-ai"), "no band for a visitor");
    for door in ["raw", "view", "thumb"] {
        assert_eq!(
            get(&app, &format!("/document/{id}/{door}")).await.status(),
            StatusCode::NOT_FOUND,
            "/{door}"
        );
    }

    // A member who may only read: the band, the pictures, no way to add.
    let vera = viewer_cookie(&app).await;
    let page = body_string(get_with_cookie(&app, &format!("/person/{LUC}"), &vera).await).await;
    assert!(page.contains("person-ai-shot"));
    assert!(
        !page.contains("/ai-portraits\""),
        "a reader is offered no edit control"
    );
    assert_eq!(
        get_with_cookie(&app, &format!("/document/{id}/thumb"), &vera)
            .await
            .status(),
        StatusCode::OK
    );
    let refused = multipart(
        &app,
        &vera,
        &format!("/admin/person/{LUC}/ai-portraits"),
        &[],
        &[("file", "b.png", png(20))],
    )
    .await;
    assert_ne!(
        refused.status(),
        StatusCode::SEE_OTHER,
        "a reader cannot add one"
    );
    assert_eq!(ai_ids(&documents(&app, &admin).await, LUC).len(), 1);

    // A member with nothing to see and nothing to add: no empty band.
    let page = body_string(get_with_cookie(&app, &format!("/person/{ALL}"), &vera).await).await;
    assert!(!page.contains("person-ai"));
}
