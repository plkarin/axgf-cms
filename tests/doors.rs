//! Who may open each part of the site (docs/adr/0046): an installation shows
//! visitors the family's page and the sign-in, and nothing else, until an
//! administrator opens more in the Connectivity room — or keeps a part for
//! fewer roles. The rule is the server's: a page not offered is not served
//! either.

mod common;

use std::path::{Path, PathBuf};

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use common::*;
use tower::ServiceExt;

const PASSWORD: &str = "correct-horse-battery-staple";
/// A person of the demonstration family, which is nobody's.
const CAMILLE: &str = "45ff9914-54f9-4d21-a816-554759749450";

fn sample() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("deploy/sample.axgf")
}

/// The demonstration family, installed as an installation starts, with a
/// reader (vera) and a contributor (cleo); the administrator's cookie, and
/// theirs.
async fn installed(tag: &str) -> (axum::Router, Scratch, String, String, String) {
    installed_over(tag, true).await
}

/// As installed, over the sample family or over an empty archive.
async fn installed_over(
    tag: &str,
    with_family: bool,
) -> (axum::Router, Scratch, String, String, String) {
    use axgf_cms::acl::{new_user, Acl, Role};
    let dir = scratch(tag);
    let path = dir.join("family.axgf");
    if with_family {
        std::fs::copy(sample(), &path).unwrap();
    }
    let mut acl = Acl::default();
    acl.users
        .push(new_user("vera", PASSWORD, Role::Viewer).unwrap());
    acl.users
        .push(new_user("cleo", PASSWORD, Role::Contributor).unwrap());
    acl.save(&Acl::path_for(&path)).unwrap();
    let app = build_app_as_installed(&path).unwrap();
    let admin = admin_cookie(&app).await;
    let mut cookies = Vec::new();
    for name in ["vera", "cleo"] {
        let login = post_form(
            &app,
            "/admin/login",
            &format!("username={name}&password={PASSWORD}"),
            false,
        )
        .await;
        assert!(login.status().is_redirection(), "{name} signs in");
        cookies.push(
            login.headers()[header::SET_COOKIE]
                .to_str()
                .unwrap()
                .split(';')
                .next()
                .unwrap()
                .to_string(),
        );
    }
    let cleo = cookies.pop().unwrap();
    let vera = cookies.pop().unwrap();
    (app, dir.pointing_at(path), admin, vera, cleo)
}

fn location(resp: &axum::response::Response) -> String {
    resp.headers()
        .get(header::LOCATION)
        .and_then(|v| v.to_str().ok())
        .unwrap_or_default()
        .to_string()
}

#[tokio::test]
async fn a_visitor_sees_the_familys_page_and_the_sign_in_and_nothing_else() {
    let (app, _dir, _, _, _) = installed("doors-visitor").await;

    // The family's page: its masthead, the wordmark and the sign-in.
    let resp = get(&app, "/about").await;
    assert_eq!(resp.status(), StatusCode::OK);
    let html = body_string(resp).await.replace("&#x2f;", "/");
    assert!(html.contains(r#"href="/admin/login""#), "the sign-in");
    assert!(
        html.contains(r#"class="brand-link" href="/about""#),
        "the way back"
    );
    for gone in [
        "masthead-search",
        "reading-prefs",
        "prefs-more",
        r#"href="/tree""#,
        r#"href="/impact""#,
        r#"href="/convert""#,
        r#"href="/person/"#,
    ] {
        assert!(!html.contains(gone), "{gone} offered to a visitor");
    }

    // The river: the family's page; every other page: the sign-in.
    let resp = get(&app, "/").await;
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    assert_eq!(location(&resp), "/about");
    for page in [
        "/tree".to_string(),
        "/impact".into(),
        "/convert".into(),
        "/search?q=Meunier".into(),
        format!("/person/{CAMILLE}"),
        format!("/impact/{CAMILLE}"),
        "/quality".into(),
    ] {
        let resp = get(&app, &page).await;
        assert_eq!(resp.status(), StatusCode::SEE_OTHER, "{page}");
        // The sign-in, and back to the page once signed in (docs/adr/0048 §7).
        assert_eq!(
            location(&resp),
            axgf_cms::render::sign_in_to(&page),
            "{page}"
        );
        assert!(location(&resp).starts_with("/admin/login?next=/"), "{page}");
    }
    // What a script or a file asks for: refused, and nothing else.
    for data in [
        "/search.json?q=Meunier".to_string(),
        "/river/data?n=2".into(),
        format!("/tree/panel/{CAMILLE}"),
        "/document/00000000-0000-4000-8000-000000000000/thumb".into(),
    ] {
        let resp = get(&app, &data).await;
        assert_eq!(resp.status(), StatusCode::FORBIDDEN, "{data}");
        assert!(body_string(resp).await.is_empty(), "{data}");
    }
    // What every page needs still is.
    for open in ["/health", "/static/app.css", "/admin/login", "/settings"] {
        assert_eq!(get(&app, open).await.status(), StatusCode::OK, "{open}");
    }
}

#[tokio::test]
async fn an_account_signed_in_reads_the_site_as_before() {
    let (app, _dir, _, vera, _) = installed("doors-member").await;
    for page in [
        "/".to_string(),
        "/tree".into(),
        format!("/person/{CAMILLE}"),
        "/impact".into(),
    ] {
        let resp = get_with_cookie(&app, &page, &vera).await;
        assert_eq!(resp.status(), StatusCode::OK, "{page}");
    }
    let html = body_string(get_with_cookie(&app, "/about", &vera).await).await;
    assert!(html.contains("masthead-search") && html.contains("reading-prefs"));
    assert!(html.contains(r#"href="/tree""#));
    assert!(
        !html.contains(r#"href="/convert""#),
        "the import, once the family is there, is no longer offered"
    );
    assert_eq!(
        get_with_cookie(&app, "/convert", &vera).await.status(),
        StatusCode::OK,
        "though its page is still there"
    );
    assert!(
        !html.contains("brand-link"),
        "the sections' row is the way back"
    );
}

#[tokio::test]
async fn an_administrator_opens_a_part_to_everyone_or_keeps_it_for_fewer() {
    let (app, _dir, admin, vera, cleo) = installed("doors-admin").await;

    // The Connectivity room says who may open what, and changes it.
    let html = body_string(get_with_cookie(&app, "/admin/connectivity", &admin).await).await;
    assert!(html.contains(r#"id="pages""#) && html.contains(r#"name="river""#));
    let resp = post_form_as(&app, &vera, "/admin/connectivity/pages", "river=visitors").await;
    assert_ne!(resp.status(), StatusCode::OK, "an administrator's alone");
    let resp = post_form_as(&app, &admin, "/admin/connectivity/pages", "river=everyone").await;
    assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);
    let resp = post_form_as(
        &app,
        &admin,
        "/admin/connectivity/pages",
        "river=visitors&import=contributors",
    )
    .await;
    assert_eq!(resp.status(), StatusCode::OK);
    assert!(body_string(resp)
        .await
        .contains("Saved: who may open each part"));

    // The river open to anyone: drawn, and offered with the family's page.
    let resp = get(&app, "/").await;
    assert_eq!(resp.status(), StatusCode::OK);
    let html = body_string(get(&app, "/about").await).await;
    assert!(
        !html.contains(r#"href="/tree""#),
        "the tree is still closed"
    );
    assert!(
        html.contains("reading-prefs"),
        "more than the family's page to read"
    );
    assert!(
        !html.contains("masthead-search"),
        "the records are still closed"
    );

    // The import kept for contributors: a reader is told so; cleo is in.
    let resp = get_with_cookie(&app, "/convert", &vera).await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
    assert!(body_string(resp).await.contains("Not open to your account"));
    let html = body_string(get_with_cookie(&app, "/about", &vera).await).await;
    assert!(!html.contains(r#"href="/convert""#), "not offered to vera");
    assert_eq!(
        get_with_cookie(&app, "/convert", &cleo).await.status(),
        StatusCode::OK
    );
}

#[tokio::test]
async fn on_an_empty_archive_the_import_link_follows_its_door() {
    // Where no family hides the link, the door alone decides who is offered it.
    let (app, _dir, admin, vera, cleo) = installed_over("doors-import-door", false).await;
    let resp = post_form_as(
        &app,
        &admin,
        "/admin/connectivity/pages",
        "import=contributors",
    )
    .await;
    assert_eq!(resp.status(), StatusCode::OK);
    let offered = |html: String| html.replace("&#x2f;", "/").contains(r#"href="/convert""#);
    assert!(!offered(
        body_string(get_with_cookie(&app, "/about", &vera).await).await
    ));
    assert!(offered(
        body_string(get_with_cookie(&app, "/about", &cleo).await).await
    ));
}

#[tokio::test]
async fn the_import_is_offered_until_a_family_is_there() {
    let (app, _dir) = app_with_empty_bundle("doors-import-empty");
    let html = body_string(get(&app, "/about").await)
        .await
        .replace("&#x2f;", "/");
    assert!(
        html.contains(r#"href="/convert""#),
        "an empty site: the first step is offered"
    );
}

/// A multipart POST: text fields, then one file.
async fn upload(
    app: &axum::Router,
    cookie: &str,
    uri: &str,
    fields: &[(&str, &str)],
    file: Vec<u8>,
) -> axum::response::Response {
    let boundary = "----axgfdoorsboundary";
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

#[tokio::test]
async fn the_archives_emblem_is_served_with_the_familys_page() {
    let (app, _dir, admin, _, _) = installed("doors-emblem").await;
    let picker = body_string(get_with_cookie(&app, "/admin/bundle/emblem", &admin).await).await;
    let at = picker.find("name=\"token\" value=\"").expect("a token");
    let rest = &picker[at + 20..];
    let token = rest[..rest.find('"').unwrap()].to_string();
    let png = {
        let img = image::RgbImage::from_pixel(8, 10, image::Rgb([150, 40, 40]));
        let mut out = std::io::Cursor::new(Vec::new());
        img.write_to(&mut out, image::ImageFormat::Png).unwrap();
        out.into_inner()
    };
    let resp = upload(
        &app,
        &admin,
        "/admin/bundle/emblem/upload",
        &[("token", &token)],
        png,
    )
    .await;
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    let page = body_string(get(&app, "/about").await).await;
    let needle = "class=\"masthead-emblem\" src=\"/document/";
    let at = page
        .find(needle)
        .expect("the emblem heads the family's page");
    let rest = &page[at + needle.len()..];
    let doc = rest[..rest.find('/').unwrap()].to_string();
    let resp = get(&app, &format!("/document/{doc}/thumb")).await;
    assert_eq!(
        resp.status(),
        StatusCode::OK,
        "served with the family's page"
    );
}
