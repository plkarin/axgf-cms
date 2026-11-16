//! What search engines see (docs/adr/0049): a page a reader reads names its
//! canonical address on the public address, whatever query brought it there;
//! every other page, and every error, says `noindex` — the sign-in above all,
//! which every address of a closed site leads to under a `next` of its own.

mod common;

use axum::body::Body;
use axum::http::{header, Response, StatusCode};
use common::*;
use serde_json::{json, Value};

const PUBLIC: &str = "https://family.example";

fn person(id: &str, given: &str, born: i32) -> Value {
    json!({"id": id, "type": "person", "axgf_version": "1.1", "version_num": 1,
        "identity": {"name": {"display": format!("{given} Nowak"),
            "components": [{"type": "given_name", "value": given},
                           {"type": "family_name", "value": "Nowak"}]},
            "gender": {"value": "M"}, "is_living": false, "visibility": "public"},
        "birth": {"date": {"value": born.to_string(), "precision": "year"}}})
}

/// A family of two, written where `dir` says.
fn bundle_in(dir: &Scratch) -> std::path::PathBuf {
    let path = dir.join("family.axgf");
    let flat = json!({
        "manifest": {"axgf": "1.1", "created_at": "2026-10-10T00:00:00Z"},
        "persons": {"p01": person("p01", "Jan", 1800), "p02": person("p02", "Antoni", 1830)},
        "families": {"f1": {"id": "f1", "type": "family", "axgf_version": "1.1", "version_num": 1,
            "union": {"type": "marriage", "persons": [{"person_id": "p01", "role": "spouse"}]},
            "children": [{"person_id": "p02", "confidence": 0.9}]}},
    });
    std::fs::write(
        &path,
        axgf_cms::state::export_to_bytes(&flat.to_string()).expect("export"),
    )
    .expect("write");
    path
}

fn robots(r: &Response<Body>) -> Option<String> {
    r.headers()
        .get("x-robots-tag")
        .map(|v| v.to_str().unwrap().to_string())
}

/// The canonical address the `Link` header names, if any.
fn canonical(r: &Response<Body>) -> Option<String> {
    r.headers()
        .get_all(header::LINK)
        .iter()
        .filter_map(|v| v.to_str().ok())
        .find(|v| v.ends_with("rel=\"canonical\""))
        .map(|v| {
            v.trim_start_matches('<')
                .split('>')
                .next()
                .unwrap()
                .to_string()
        })
}

fn is_html(r: &Response<Body>) -> bool {
    r.headers()
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .is_some_and(|t| t.starts_with("text/html"))
}

async fn set_public(app: &axum::Router, admin: &str) {
    let r = post_form_as(
        app,
        admin,
        "/admin/connectivity/address",
        "public_url=https%3A%2F%2Ffamily.example",
    )
    .await;
    assert_eq!(r.status(), StatusCode::OK, "the public address is saved");
}

#[tokio::test]
async fn a_closed_sites_sign_in_says_noindex_whatever_page_led_there() {
    let dir = scratch("seo-closed");
    let path = bundle_in(&dir);
    let app = build_app_as_installed(&path).expect("build app");
    let admin = admin_cookie(&app).await;
    set_public(&app, &admin).await;

    // Every closed address sends a visitor to the sign-in with a `next` of
    // its own; none of those copies may be kept.
    for page in ["/tree", "/person/p01", "/impact", "/search?q=Jan"] {
        let r = get(&app, page).await;
        assert!(r.status().is_redirection(), "{page}: {}", r.status());
        let to = r.headers()[header::LOCATION].to_str().unwrap().to_string();
        assert!(to.starts_with("/admin/login"), "{page} → {to}");
        let r = get(&app, &to).await;
        assert_eq!(r.status(), StatusCode::OK, "{to}");
        assert_eq!(robots(&r).as_deref(), Some("noindex"), "{to}");
        assert_eq!(canonical(&r), None, "{to}");
    }
    for page in [
        "/admin/login",
        "/register",
        "/mail/forgot",
        "/mail/set/abc.def.ghi",
    ] {
        let r = get(&app, page).await;
        assert!(is_html(&r), "{page}");
        assert_eq!(robots(&r).as_deref(), Some("noindex"), "{page}");
        assert_eq!(canonical(&r), None, "{page}");
    }

    // The family's page, open to visitors, names its own address.
    let r = get(&app, "/about").await;
    assert_eq!(r.status(), StatusCode::OK);
    assert_eq!(canonical(&r), Some(format!("{PUBLIC}/about")));
    assert_eq!(robots(&r), None);
}

#[tokio::test]
async fn an_open_sites_reading_pages_name_their_address_without_the_query() {
    let dir = scratch("seo-open");
    let path = bundle_in(&dir);
    let app = build_app(&path).expect("build app");
    let admin = admin_cookie(&app).await;
    set_public(&app, &admin).await;

    for (page, want) in [
        ("/", "/"),
        ("/?p=p01&n=5", "/"),
        ("/about", "/about"),
        ("/tree", "/tree"),
        ("/tree?root=p01&depth=2", "/tree"),
        ("/person/p01", "/person/p01"),
        ("/person/p01?tab=kinship&with=p02", "/person/p01"),
        ("/impact", "/impact"),
        ("/impact?span=family", "/impact"),
        ("/impact/p01", "/impact/p01"),
    ] {
        let r = get(&app, page).await;
        assert_eq!(r.status(), StatusCode::OK, "{page}");
        assert_eq!(canonical(&r), Some(format!("{PUBLIC}{want}")), "{page}");
        assert_eq!(robots(&r), None, "{page}");
    }

    // A search's results, the tools, a fragment and every error are not
    // kept.
    for page in [
        "/search?q=Jan",
        "/convert",
        "/settings",
        "/tree/panel/p01",
        "/admin/login",
        "/no-such-page",
        "/person/nobody",
    ] {
        let r = get(&app, page).await;
        assert!(is_html(&r), "{page}: {}", r.status());
        assert_eq!(robots(&r).as_deref(), Some("noindex"), "{page}");
        assert_eq!(canonical(&r), None, "{page}");
    }

    // What is not a page is left alone.
    for page in ["/health", "/static/app.css", "/favicon.svg"] {
        let r = get(&app, page).await;
        assert!(!is_html(&r), "{page}");
        assert_eq!(robots(&r), None, "{page}");
        assert_eq!(canonical(&r), None, "{page}");
    }
}

#[tokio::test]
async fn without_a_public_address_no_canonical_is_named_whatever_the_host() {
    let dir = scratch("seo-no-address");
    let path = bundle_in(&dir);
    let app = build_app(&path).expect("build app");
    let r = {
        use tower::ServiceExt as _;
        app.clone()
            .oneshot(
                axum::http::Request::builder()
                    .uri("/about")
                    .header(header::HOST, "evil.example")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap()
    };
    assert_eq!(r.status(), StatusCode::OK);
    assert_eq!(canonical(&r), None, "never built on the request's Host");
    assert_eq!(robots(&r), None, "a reading page all the same");
    // The rest is kept out all the same.
    let r = get(&app, "/admin/login").await;
    assert_eq!(robots(&r).as_deref(), Some("noindex"));
}

#[tokio::test]
async fn a_door_that_refuses_a_member_says_noindex_too() {
    let dir = scratch("seo-door");
    let path = bundle_in(&dir);
    let app = build_app_as_installed(&path).expect("build app");
    let admin = admin_cookie(&app).await;
    let r = post_form_as(
        &app,
        &admin,
        "/admin/connectivity/pages",
        "records=contributors",
    )
    .await;
    assert_eq!(
        r.status(),
        StatusCode::OK,
        "the records kept for contributors"
    );
    let r = post_form_as(
        &app,
        &admin,
        "/admin/users",
        "username=vera&password=correct-horse-battery-staple&role=viewer",
    )
    .await;
    assert!(r.status().is_success());
    let r = post_form_as(
        &app,
        "",
        "/admin/login",
        "username=vera&password=correct-horse-battery-staple",
    )
    .await;
    let member = r.headers()[header::SET_COOKIE]
        .to_str()
        .unwrap()
        .split(';')
        .next()
        .unwrap()
        .to_string();
    let r = get_with_cookie(&app, "/person/p01", &member).await;
    assert_eq!(r.status(), StatusCode::FORBIDDEN, "the door's own page");
    assert!(is_html(&r));
    assert_eq!(robots(&r).as_deref(), Some("noindex"));
}

#[tokio::test]
async fn an_empty_river_names_the_familys_page_as_its_address() {
    let (app, _dir) = app_with_empty_bundle("seo-empty");
    let admin = admin_cookie(&app).await;
    set_public(&app, &admin).await;
    // Nobody to draw: `/` shows the family's page, and says where it lives.
    let r = get(&app, "/").await;
    assert_eq!(r.status(), StatusCode::OK);
    let links = r.headers().get_all(header::LINK).iter().count();
    assert_eq!(links, 1, "one canonical, not two");
    assert_eq!(canonical(&r), Some(format!("{PUBLIC}/about")));
}

#[tokio::test]
async fn a_file_seen_through_its_viewer_names_its_own_address() {
    use tower::ServiceExt as _;
    let dir = scratch("seo-file");
    let path = bundle_in(&dir);
    let app = build_app(&path).expect("build app");
    let admin = admin_cookie(&app).await;
    set_public(&app, &admin).await;
    let pdf: &[u8] = b"%PDF-1.4\n1 0 obj<<>>endobj\ntrailer<<>>\n%%EOF\n";
    let boundary = "----axgfseoboundary";
    let mut body = format!(
        "--{boundary}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"acte.pdf\"\r\nContent-Type: application/pdf\r\n\r\n"
    )
    .into_bytes();
    body.extend_from_slice(pdf);
    body.extend_from_slice(format!("\r\n--{boundary}--\r\n").as_bytes());
    let r = app
        .clone()
        .oneshot(
            axum::http::Request::builder()
                .uri("/admin/person/p01/document")
                .method("POST")
                .header(header::COOKIE, &admin)
                .header(
                    header::CONTENT_TYPE,
                    format!("multipart/form-data; boundary={boundary}"),
                )
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();
    assert!(
        r.status().is_success() || r.status().is_redirection(),
        "{}",
        r.status()
    );
    let env = axgf_rs::import_bundle(&std::fs::read(&path).unwrap());
    let id = env.data["documents"]
        .as_object()
        .and_then(|m| m.keys().next().cloned())
        .expect("a document");

    let r = get(&app, &format!("/document/{id}/view")).await;
    assert_eq!(r.status(), StatusCode::OK);
    assert_eq!(canonical(&r), Some(format!("{PUBLIC}/document/{id}/raw")));
    let r = get(&app, &format!("/document/{id}/raw")).await;
    assert_eq!(r.status(), StatusCode::OK);
    assert_eq!(canonical(&r), None, "the address itself");
}
