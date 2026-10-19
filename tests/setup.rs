//! Setting an installation up (docs/adr/0005): three doors into a family —
//! an `.axgf` archive, nothing, a GEDCOM of any version — open to an
//! administrator while the installation has none, closed once it has one,
//! and none of them ever writes over a family.

mod common;

use std::io::Read as _;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use common::*;
use serde_json::Value;
use tower::ServiceExt;

const BOUNDARY: &str = "----axgfcmssetupboundary";

fn multipart(filename: &str, content: &[u8], fields: &[(&str, &str)]) -> Vec<u8> {
    let mut body = Vec::new();
    body.extend_from_slice(format!("--{BOUNDARY}\r\n").as_bytes());
    body.extend_from_slice(
        format!("Content-Disposition: form-data; name=\"file\"; filename=\"{filename}\"\r\n")
            .as_bytes(),
    );
    body.extend_from_slice(b"Content-Type: application/octet-stream\r\n\r\n");
    body.extend_from_slice(content);
    body.extend_from_slice(b"\r\n");
    for (name, value) in fields {
        body.extend_from_slice(format!("--{BOUNDARY}\r\n").as_bytes());
        body.extend_from_slice(
            format!("Content-Disposition: form-data; name=\"{name}\"\r\n\r\n").as_bytes(),
        );
        body.extend_from_slice(value.as_bytes());
        body.extend_from_slice(b"\r\n");
    }
    body.extend_from_slice(format!("--{BOUNDARY}--\r\n").as_bytes());
    body
}

async fn post_multipart(
    app: &axum::Router,
    uri: &str,
    cookie: &str,
    body: Vec<u8>,
) -> axum::http::Response<Body> {
    app.clone()
        .oneshot(
            Request::builder()
                .uri(uri)
                .method("POST")
                .header(
                    header::CONTENT_TYPE,
                    format!("multipart/form-data; boundary={BOUNDARY}"),
                )
                .header(header::COOKIE, cookie)
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .expect("request")
}

fn fixture(name: &str) -> Vec<u8> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name);
    std::fs::read(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

/// The family the installation now holds, read back from its own export.
async fn exported(app: &axum::Router, cookie: &str) -> Vec<(String, Value)> {
    let bytes = body_bytes(get_with_cookie(app, "/admin/export", cookie).await).await;
    let mut zip = zip::ZipArchive::new(std::io::Cursor::new(bytes)).expect("the export is a zip");
    let names: Vec<String> = zip.file_names().map(str::to_string).collect();
    names
        .into_iter()
        .filter(|n| {
            n.ends_with(".json")
                && !n.starts_with("schema/")
                && n != "manifest.json"
                && !n.ends_with("index.json")
        })
        .map(|n| {
            let mut s = String::new();
            zip.by_name(&n).unwrap().read_to_string(&mut s).unwrap();
            (n, serde_json::from_str(&s).unwrap())
        })
        .collect()
}

fn people(entities: &[(String, Value)]) -> Vec<String> {
    let mut out: Vec<String> = entities
        .iter()
        .filter(|(n, _)| n.starts_with("persons/"))
        .map(|(_, v)| {
            v["identity"]["name"]["display"]
                .as_str()
                .unwrap_or_default()
                .to_string()
        })
        .collect();
    out.sort();
    out
}

/// Read a GEDCOM through the third door, and keep what it showed.
async fn set_up_from_gedcom(app: &axum::Router, cookie: &str, file: &str) -> String {
    let resp = post_multipart(
        app,
        "/admin/setup/gedcom",
        cookie,
        multipart(
            file,
            &fixture(&format!("gedcom/{file}")),
            &[("confidence", "0.8"), ("lang", "pl")],
        ),
    )
    .await;
    let page = expect_status(resp, StatusCode::OK, file).await;
    let at = page
        .find("action=\"/admin/setup/gedcom/")
        .unwrap_or_else(|| panic!("{file}: no button to keep it"));
    let action: String = page[at + 8..].chars().take_while(|c| *c != '"').collect();
    let done = expect_status(
        post_form_as(app, cookie, &action, "").await,
        StatusCode::OK,
        file,
    )
    .await;
    assert!(
        done.contains("The archive is set up"),
        "{file}: the done page"
    );
    page
}

#[tokio::test]
async fn an_installation_without_a_family_offers_its_administrator_three_doors() {
    let (app, _s) = app_with_empty_bundle("setup-doors");
    let cookie = admin_cookie(&app).await;
    let page = expect_status(
        get_with_cookie(&app, "/admin/setup", &cookie).await,
        StatusCode::OK,
        "setup",
    )
    .await;
    for door in [
        r#"action="/admin/setup/archive""#,
        r#"href="/admin/person/new""#,
        r#"action="/admin/setup/gedcom""#,
    ] {
        assert!(page.contains(door), "{door}");
    }
    // A visitor is offered nothing to bring in: not the page, not a link.
    assert_eq!(
        get(&app, "/admin/setup").await.status(),
        StatusCode::UNAUTHORIZED
    );
    let visitor = body_string(get(&app, "/about").await).await;
    assert!(!visitor.contains("/admin/setup"));
    let admin = body_string(get_with_cookie(&app, "/about", &cookie).await).await;
    assert!(
        admin.contains(r#"href="/admin/setup""#),
        "the overview shows the administrator the way in"
    );
    let dashboard = body_string(get_with_cookie(&app, "/admin", &cookie).await).await;
    assert!(
        dashboard.contains(r#"href="/admin/setup""#),
        "and so does the dashboard"
    );
}

/// The operator's question, answered for each case a family's file can be:
/// GEDCOM 5.5 in ANSEL, 5.5.1 in UTF-8 and in Windows-1252, 5.5.5 in UTF-16
/// without a byte-order mark, old Macintosh line endings, GEDCOM 7.0 bare and
/// in a `.gdz`. Every one sets a family up with its names whole.
#[tokio::test]
async fn every_gedcom_version_and_encoding_sets_a_family_up_with_its_names_whole() {
    let whole = ["Helena Nowak", "Józef Gęsiorek", "Éléonore Gęsiorek"];
    // Windows-1252 has no ę: the sample was written with a question mark.
    let ansi = ["Helena Nowak", "Józef G?siorek", "Éléonore G?siorek"];
    for (file, read_as, names) in [
        ("g551-utf8.ged", "GEDCOM 5.5.1, in UTF-8", &whole),
        ("g55-ansel.ged", "GEDCOM 5.5, in ANSEL", &whole),
        ("g555-utf16le.ged", "GEDCOM 5.5.5, in UTF-16 LE", &whole),
        ("g551-ansi.ged", "GEDCOM 5.5.1, in Windows-1252", &ansi),
        ("g551-cr.ged", "GEDCOM 5.5.1, in UTF-8", &whole),
        ("g70-utf8.ged", "GEDCOM 7.0, in UTF-8", &whole),
        ("g70.gdz", "GEDCOM 7.0, in UTF-8", &whole),
    ] {
        let (app, _s) = app_with_empty_bundle(&format!("setup-{file}"));
        let cookie = admin_cookie(&app).await;
        let page = set_up_from_gedcom(&app, &cookie, file).await;
        assert!(page.contains(read_as), "{file}: how it was read");
        let family = exported(&app, &cookie).await;
        assert_eq!(people(&family), names.to_vec(), "{file}");
        // GEDCOM 7: the @VOID@ child is not a child, and the shared note is
        // Józef's note.
        if file.starts_with("g70") {
            assert!(
                page.contains("@VOID@") && page.contains("SNOTE"),
                "{file}: the repairs are said"
            );
            let fam = family
                .iter()
                .find(|(n, _)| n.starts_with("families/"))
                .expect("a family");
            assert_eq!(
                fam.1["children"].as_array().map(Vec::len),
                Some(1),
                "{file}: one child, not two"
            );
            let jozef = family
                .iter()
                .find(|(n, v)| {
                    n.starts_with("persons/")
                        && v["identity"]["name"]["display"] == "Józef Gęsiorek"
                })
                .expect("Józef");
            assert!(
                jozef.1.to_string().contains("Une note longue"),
                "{file}: the shared note reached him"
            );
        }
        // The doors close behind the family.
        assert_eq!(
            get_with_cookie(&app, "/admin/setup", &cookie)
                .await
                .status(),
            StatusCode::CONFLICT
        );
    }
}

#[tokio::test]
async fn an_archive_sets_a_family_up_photographs_and_all() {
    let (app, _s) = app_with_empty_bundle("setup-archive");
    let cookie = admin_cookie(&app).await;
    let archive = fixture("with-photo.axgf");
    let resp = post_multipart(
        &app,
        "/admin/setup/archive",
        &cookie,
        multipart("with-photo.axgf", &archive, &[]),
    )
    .await;
    expect_status(resp, StatusCode::OK, "archive").await;
    let family = exported(&app, &cookie).await;
    assert_eq!(people(&family), ["Józef Gęsiorek"]);
    // The photograph is served, byte for byte the archive's own.
    let mut zip = zip::ZipArchive::new(std::io::Cursor::new(archive)).unwrap();
    let name = zip
        .file_names()
        .find(|n| n.ends_with(".jpg"))
        .unwrap()
        .to_string();
    let mut original = Vec::new();
    zip.by_name(&name)
        .unwrap()
        .read_to_end(&mut original)
        .unwrap();
    let id = name
        .trim_start_matches("documents/files/")
        .trim_end_matches(".jpg");
    let served =
        body_bytes(get_with_cookie(&app, &format!("/document/{id}/raw"), &cookie).await).await;
    assert_eq!(served, original);
    assert_eq!(
        get_with_cookie(&app, "/admin/setup", &cookie)
            .await
            .status(),
        StatusCode::CONFLICT
    );
}

#[tokio::test]
async fn a_file_that_is_not_an_archive_changes_nothing() {
    let (app, _s) = app_with_empty_bundle("setup-not-archive");
    let cookie = admin_cookie(&app).await;
    let resp = post_multipart(
        &app,
        "/admin/setup/archive",
        &cookie,
        multipart("x.axgf", b"not a zip", &[]),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    // Still unset: the doors are still open.
    assert_eq!(
        get_with_cookie(&app, "/admin/setup", &cookie)
            .await
            .status(),
        StatusCode::OK
    );
}

/// Nothing is ever written over a family: each door, tried again once one is
/// there, is refused and leaves it as it was. Replacing a family stays
/// `axgf-cms restore`.
#[tokio::test]
async fn no_door_writes_over_a_family() {
    let (app, _s) = app_with_empty_bundle("setup-occupied");
    let cookie = admin_cookie(&app).await;
    let first = set_up_from_gedcom(&app, &cookie, "g551-utf8.ged").await;
    let before = people(&exported(&app, &cookie).await);
    let at = first.find("action=\"/admin/setup/gedcom/").unwrap();
    let again: String = first[at + 8..].chars().take_while(|c| *c != '"').collect();
    let gedcom = multipart("g70-utf8.ged", &fixture("gedcom/g70-utf8.ged"), &[]);
    let archive = multipart("with-photo.axgf", &fixture("with-photo.axgf"), &[]);
    for (what, resp) in [
        (
            "the page",
            get_with_cookie(&app, "/admin/setup", &cookie).await,
        ),
        (
            "the same conversion again",
            post_form_as(&app, &cookie, &again, "").await,
        ),
        (
            "another GEDCOM",
            post_multipart(&app, "/admin/setup/gedcom", &cookie, gedcom).await,
        ),
        (
            "an archive",
            post_multipart(&app, "/admin/setup/archive", &cookie, archive).await,
        ),
    ] {
        assert_eq!(resp.status(), StatusCode::CONFLICT, "{what}");
    }
    assert_eq!(people(&exported(&app, &cookie).await), before);
}
