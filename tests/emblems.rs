//! The picture that stands for a family, and for the archive's family
//! (docs/adr/0022): added, chosen, shown on the record, on the tree, at the
//! head of every page, and read by whoever may read the family's people.

mod common;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use common::*;
use serde_json::{json, Value};
use tower::ServiceExt;

const HUSBAND: &str = "b0000000-0000-4000-8000-000000000001";
const WIFE: &str = "b0000000-0000-4000-8000-000000000002";
const SON: &str = "b0000000-0000-4000-8000-000000000003";
const FAMILY: &str = "fb000000-0000-4000-8000-000000000001";

fn person(id: &str, name: &str, sex: &str) -> Value {
    json!({"id": id, "type": "person", "axgf_version": "1.1",
           "identity": {"name": {"display": name, "components": []},
                        "gender": {"value": sex},
                        "is_living": false, "visibility": "public"}})
}

/// A fictional couple and their son.
fn bundle(tag: &str) -> common::Scratch {
    let dir = scratch(tag);
    let path = dir.join("emblems.axgf");
    let flat = json!({
        "manifest": {"axgf": "1.1", "family": {"name": "Mensah"}},
        "persons": {
            HUSBAND: person(HUSBAND, "Kofi Mensah", "M"),
            WIFE: person(WIFE, "Ama Boateng", "F"),
            SON: person(SON, "Kwame Mensah", "M"),
        },
        "families": {FAMILY: {"id": FAMILY, "type": "family", "axgf_version": "1.0",
            "union": {"type": "marriage", "persons": [
                {"person_id": HUSBAND, "role": "spouse"}, {"person_id": WIFE, "role": "spouse"}]},
            "children": [{"person_id": SON}]}},
        "events": {}, "links": {}, "occupations": {}, "sources": {}, "places": {}, "documents": {}
    });
    std::fs::write(
        &path,
        axgf_cms::state::export_to_bytes(&flat.to_string()).expect("export"),
    )
    .expect("write");
    dir.pointing_at(path)
}

fn png() -> Vec<u8> {
    let img = image::RgbImage::from_pixel(8, 10, image::Rgb([150, 40, 40]));
    let mut out = std::io::Cursor::new(Vec::new());
    img.write_to(&mut out, image::ImageFormat::Png)
        .expect("png");
    out.into_inner()
}

/// A multipart POST: text fields, then one file.
async fn upload(
    app: &axum::Router,
    cookie: &str,
    uri: &str,
    fields: &[(&str, &str)],
    file: Vec<u8>,
) -> axum::response::Response {
    let boundary = "----axgfemblemboundary";
    let mut body = Vec::new();
    for (k, v) in fields {
        body.extend_from_slice(
            format!("--{boundary}\r\nContent-Disposition: form-data; name=\"{k}\"\r\n\r\n{v}\r\n")
                .as_bytes(),
        );
    }
    body.extend_from_slice(
        format!(
            "--{boundary}\r\nContent-Disposition: form-data; name=\"file\"; \
             filename=\"arms.png\"\r\nContent-Type: application/octet-stream\r\n\r\n"
        )
        .as_bytes(),
    );
    body.extend_from_slice(&file);
    body.extend_from_slice(format!("\r\n--{boundary}--\r\n").as_bytes());
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

/// The value of the first `<input name="...">` on a page.
fn input(html: &str, name: &str) -> String {
    let at = html
        .find(&format!("name=\"{name}\" value=\""))
        .unwrap_or_else(|| panic!("no {name} on the page"));
    let rest = &html[at + name.len() + 15..];
    rest[..rest.find('"').unwrap()].to_string()
}

/// The id in the first `/document/<id>/thumb` on a page.
fn thumb_id(html: &str, class: &str) -> String {
    let at = html
        .find(&format!("class=\"{class}\" src=\"/document/"))
        .unwrap_or_else(|| panic!("no {class} on the page"));
    let rest = &html[at + class.len() + 24..];
    rest[..rest.find('/').unwrap()].to_string()
}

#[tokio::test]
async fn a_familys_emblem_is_added_shown_and_read_as_its_people_are() {
    let src = bundle("emblem-family-src");
    let (app, _p) = app_with_bundle("emblem-family", &src);
    let cookie = admin_cookie(&app).await;

    let picker = body_string(
        get_with_cookie(
            &app,
            &format!("/admin/family/{FAMILY}/avatar?back={HUSBAND}"),
            &cookie,
        )
        .await,
    )
    .await;
    let version = input(&picker, "base_version");
    let resp = upload(
        &app,
        &cookie,
        &format!("/admin/family/{FAMILY}/document"),
        &[
            ("set_avatar", "1"),
            ("base_version", &version),
            ("back", HUSBAND),
        ],
        png(),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);

    // On the record, beside the union, to a visitor who is not signed in.
    let record = body_string(get(&app, &format!("/person/{HUSBAND}?tab=family")).await).await;
    let doc = thumb_id(&record, "union-emblem");
    let resp = get(&app, &format!("/document/{doc}/thumb")).await;
    assert_eq!(
        resp.status(),
        StatusCode::OK,
        "the family's people are public"
    );
    // On the couple's knot in the tree.
    let tree = body_string(get(&app, &format!("/tree?root={HUSBAND}")).await).await;
    assert!(tree.contains("knot-emblem-disc"), "{tree}");
    assert!(tree.contains(&format!("/document/{doc}/thumb")));
    // In the record's editor, beside the union, with its own page.
    let editor =
        body_string(get_with_cookie(&app, &format!("/admin/person/{HUSBAND}/edit"), &cookie).await)
            .await;
    // Attribute values are escaped, a slash included.
    let editor = editor.replace("&#x2f;", "/");
    assert!(editor.contains(&format!("/admin/family/{FAMILY}/avatar?back={HUSBAND}")));

    // Nobody chose: no emblem, and the record says nothing of one.
    let picker = body_string(
        get_with_cookie(&app, &format!("/admin/family/{FAMILY}/avatar"), &cookie).await,
    )
    .await;
    let version = input(&picker, "base_version");
    let resp = post_form_as(
        &app,
        &cookie,
        &format!("/admin/family/{FAMILY}/avatar"),
        &format!("choice=none&base_version={version}"),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    let record = body_string(get(&app, &format!("/person/{HUSBAND}?tab=family")).await).await;
    assert!(!record.contains("union-emblem"), "{record}");
}

#[tokio::test]
async fn the_archives_emblem_heads_every_page() {
    let src = bundle("emblem-archive-src");
    let (app, _p) = app_with_bundle("emblem-archive", &src);
    let cookie = admin_cookie(&app).await;

    let picker = body_string(get_with_cookie(&app, "/admin/bundle/emblem", &cookie).await).await;
    let token = input(&picker, "token");
    let resp = upload(
        &app,
        &cookie,
        "/admin/bundle/emblem/upload",
        &[("token", &token)],
        png(),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    assert_eq!(
        resp.headers()["location"].to_str().unwrap(),
        "/admin/bundle/emblem?saved=1"
    );

    // At the head of every page and on the overview, for every visitor.
    let overview = body_string(get(&app, "/about").await).await;
    let doc = thumb_id(&overview, "masthead-emblem");
    assert!(overview.contains("class=\"hero-emblem\""));
    let record = body_string(get(&app, &format!("/person/{SON}")).await).await;
    assert!(record.contains("class=\"masthead-emblem\""));
    let resp = get(&app, &format!("/document/{doc}/thumb")).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Taken down.
    let picker = body_string(get_with_cookie(&app, "/admin/bundle/emblem", &cookie).await).await;
    let token = input(&picker, "token");
    let resp = post_form_as(
        &app,
        &cookie,
        "/admin/bundle/emblem",
        &format!("choice=none&token={token}"),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    let overview = body_string(get(&app, "/about").await).await;
    assert!(!overview.contains("masthead-emblem"));
}
