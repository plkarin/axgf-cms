//! The administration console (v0.5.0, docs/adr/0011): the AXGF bundle page,
//! the encrypted export, the same-site rule for posts and the cache policy.

mod common;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use common::*;
use tower::ServiceExt;

fn sample() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("deploy/sample.axgf")
}

fn fixture(name: &str) -> String {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    std::fs::read_to_string(root.join("tests/fixtures/pgp").join(name)).unwrap()
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

#[tokio::test]
async fn the_bundle_page_is_an_administrators_and_says_what_conforms() {
    let (app, _s) = app_with_bundle("console-bundle", &sample());
    assert_eq!(
        get(&app, "/admin/bundle").await.status(),
        StatusCode::UNAUTHORIZED
    );
    let cookie = admin_cookie(&app).await;
    let resp = get_with_cookie(&app, "/admin/bundle", &format!("{cookie}; axgf_lang=en")).await;
    assert_eq!(resp.status(), StatusCode::OK);
    assert_eq!(
        resp.headers()[header::CACHE_CONTROL],
        "no-store",
        "a page an administrator saw is not kept by the browser"
    );
    let html = body_string(resp).await;
    for expected in [
        r#"class="admin-nav""#,
        r#"<a class="admin-nav-item" href="/admin/bundle" aria-current="page">"#,
        "Format version",
        // The sample bundle predates AXGF 1.1, and the page says so: a
        // warning with nothing to do but let the next save rewrite it.
        "Written as AXGF 1.0; the next save writes it as AXGF 1.1.",
        "The manifest&#x27;s counts",
        "Validation",
        "Accounts and bundle",
        "The living, in public",
        r#"action="/admin/bundle/sealed""#,
        r#"action="/admin/export""#,
    ] {
        assert!(html.contains(expected), "missing {expected}");
    }
}

#[tokio::test]
async fn an_empty_installation_says_there_is_nothing_to_check() {
    let (app, _s) = app_with_empty_bundle("console-empty");
    let cookie = admin_cookie(&app).await;
    let html = body_string(
        get_with_cookie(&app, "/admin/bundle", &format!("{cookie}; axgf_lang=en")).await,
    )
    .await;
    assert!(html.contains("No family yet."));
    assert!(html.contains(r#"href="/admin/setup""#));
    assert!(!html.contains("Format version"), "no checks over nothing");
}

#[tokio::test]
async fn the_encrypted_export_opens_with_the_private_key_and_nothing_else() {
    use pgp::composed::{Deserializable, Message, SignedSecretKey};
    let (app, _s) = app_with_bundle("console-seal", &sample());
    let cookie = admin_cookie(&app).await;
    let body = form(&[("recipient", &fixture("p384.pub.asc")), ("remember", "on")]);
    let resp = post_form_as(&app, &cookie, "/admin/bundle/sealed", &body).await;
    assert_eq!(resp.status(), StatusCode::OK);
    assert_eq!(
        resp.headers()[header::CONTENT_TYPE],
        "application/pgp-encrypted"
    );
    let disposition = resp.headers()[header::CONTENT_DISPOSITION]
        .to_str()
        .unwrap()
        .to_string();
    assert!(disposition.contains("family.axgf.gpg"), "{disposition}");
    let sealed = body_bytes(resp).await;
    assert!(!sealed.starts_with(b"PK"), "never the plain archive");
    let (secret, _) = SignedSecretKey::from_string(&fixture("p384.sec.asc")).unwrap();
    let mut msg = Message::from_bytes(&sealed[..])
        .unwrap()
        .decrypt(&"".into(), &secret)
        .unwrap();
    let plain = msg.as_data_vec().unwrap();
    assert!(plain.starts_with(b"PK"), "the archive inside is a ZIP");
    // And it is a bundle this application reads back.
    let env = axgf_cms_reads(&plain);
    assert!(env, "the decrypted archive imports");

    // The key is kept: the page now describes it.
    let html = body_string(
        get_with_cookie(&app, "/admin/bundle", &format!("{cookie}; axgf_lang=en")).await,
    )
    .await;
    assert!(html.contains("ECDH P-384"), "the kept key is described");
    assert!(html.contains("in the FIPS-approved set"));
}

fn axgf_cms_reads(bytes: &[u8]) -> bool {
    let env = axgf_rs::import_bundle(bytes);
    env.data.get("manifest").is_some()
}

#[tokio::test]
async fn a_weak_key_is_refused_before_anything_is_exported() {
    let (app, _s) = app_with_bundle("console-weak", &sample());
    let cookie = admin_cookie(&app).await;
    let body = form(&[("recipient", &fixture("rsa1024.pub.asc"))]);
    let resp = post_form_as(
        &app,
        &format!("{cookie}; axgf_lang=en"),
        "/admin/bundle/sealed",
        &body,
    )
    .await;
    assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);
    let html = body_string(resp).await;
    assert!(
        html.contains("That RSA key has 1024 bits; an export needs at least 2048."),
        "{}",
        &html[..html.len().min(400)]
    );
    let resp = post_form_as(
        &app,
        &cookie,
        "/admin/bundle/sealed",
        &form(&[("recipient", "garbage")]),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn a_form_posted_from_another_site_is_refused() {
    let (app, _s) = app_with_bundle("console-origin", &sample());
    let cookie = admin_cookie(&app).await;
    let post = |origin: &'static str| {
        Request::builder()
            .uri("/admin/bundle/forget-recipient")
            .method("POST")
            .header(header::HOST, "family.example")
            .header(header::ORIGIN, origin)
            .header(header::COOKIE, cookie.clone())
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(Body::empty())
            .unwrap()
    };
    let refused = app
        .clone()
        .oneshot(post("http://family.example:8080"))
        .await
        .unwrap();
    assert_eq!(
        refused.status(),
        StatusCode::FORBIDDEN,
        "another port is another site's page"
    );
    let refused = app
        .clone()
        .oneshot(post("https://evil.example"))
        .await
        .unwrap();
    assert_eq!(refused.status(), StatusCode::FORBIDDEN);
    let ours = app
        .clone()
        .oneshot(post("http://family.example"))
        .await
        .unwrap();
    assert!(ours.status().is_redirection(), "{}", ours.status());
}

#[tokio::test]
async fn a_contributor_does_not_see_the_administrators_rooms() {
    let (app, _s) = app_with_bundle("console-contrib", &sample());
    let admin = admin_cookie(&app).await;
    let resp = post_form_as(
        &app,
        &admin,
        "/admin/users",
        "username=cora&password=correct-horse-battery&role=contributor",
    )
    .await;
    assert_eq!(resp.status(), StatusCode::OK);
    let login = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/admin/login")
                .method("POST")
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .body(Body::from("username=cora&password=correct-horse-battery"))
                .unwrap(),
        )
        .await
        .unwrap();
    let cookie = login.headers()[header::SET_COOKIE]
        .to_str()
        .unwrap()
        .split(';')
        .next()
        .unwrap()
        .to_string();
    let dash = body_string(get_with_cookie(&app, "/admin", &cookie).await).await;
    assert!(
        !dash.contains(r#"class="admin-nav""#),
        "no strip of doors that answer 403"
    );
    assert_eq!(
        get_with_cookie(&app, "/admin/bundle", &cookie)
            .await
            .status(),
        StatusCode::FORBIDDEN
    );
}

// ---------------------------------------------------------------------------
// The Infrastructure page
// ---------------------------------------------------------------------------

/// A list of releases as the helper writes it, with one newer and one older
/// than any build of this project.
fn write_channel(dir: &std::path::Path) {
    let body = serde_json::json!({
        "checked_at": "2026-10-21T08:00:00Z",
        "source": "helper",
        "releases": [
            { "tag": "v99.0.0", "name": "v99.0.0", "published_at": "2026-10-20T10:00:00Z", "prerelease": false, "url": "https://github.com/plkarin/axgf-cms/releases/tag/v99.0.0" },
            { "tag": "v99.1.0-rc1", "name": "v99.1.0-rc1", "published_at": "2026-10-21T07:00:00Z", "prerelease": true, "url": "" },
            { "tag": "v0.0.1", "name": "v0.0.1", "published_at": "2025-01-01T10:00:00Z", "prerelease": false, "url": "" }
        ]
    });
    std::fs::write(dir.join("releases.json"), body.to_string()).unwrap();
}

#[tokio::test]
async fn the_infrastructure_page_says_green_warning_or_critical_and_why() {
    let (app, _s) = app_with_bundle("machine-page", &sample());
    assert_eq!(
        get(&app, "/admin/infrastructure").await.status(),
        StatusCode::UNAUTHORIZED
    );
    let cookie = admin_cookie(&app).await;
    let resp = get_with_cookie(
        &app,
        "/admin/infrastructure",
        &format!("{cookie}; axgf_lang=en"),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::OK);
    assert_eq!(resp.headers()[header::CACHE_CONTROL], "no-store");
    let html = body_string(resp).await;
    for expected in [
        r#"<a class="admin-nav-item" href="/admin/infrastructure" aria-current="page">"#,
        "How the site is reached",
        "Administrators",
        "Emergency token",
        "Disk space",
        "Whether a newer version exists is not known",
        // No helper on a test machine: the button is replaced by the command.
        "sudo bash bootstrap.sh --upgrade --version",
        r#"id="volumes""#,
        "Family bundle",
        r#"id="compute""#,
        "Architecture",
        r#"href="/admin/infrastructure.json""#,
    ] {
        assert!(html.contains(expected), "missing {expected}");
    }
    let at = html
        .find("infra-")
        .map(|i| &html[i.saturating_sub(120)..(i + 80).min(html.len())]);
    assert!(at.is_none(), "a key rendered as itself: {at:?}");
}

#[tokio::test]
async fn the_infrastructure_json_is_for_a_monitor_and_an_administrator() {
    let (app, _s) = app_with_bundle("infra-json", &sample());
    assert_eq!(
        get(&app, "/admin/infrastructure.json").await.status(),
        StatusCode::UNAUTHORIZED
    );
    let cookie = admin_cookie(&app).await;
    let resp = get_with_cookie(&app, "/admin/infrastructure.json", &cookie).await;
    assert_eq!(resp.status(), StatusCode::OK);
    assert_eq!(resp.headers()[header::CONTENT_TYPE], "application/json");
    let v: serde_json::Value = serde_json::from_str(&body_string(resp).await).unwrap();
    assert!(["ok", "warn", "fail"].contains(&v["status"].as_str().unwrap()));
    let checks = v["checks"].as_array().unwrap();
    for id in [
        "transport",
        "admins",
        "emergency-token",
        "bundle",
        "disk",
        "version",
        "sandbox",
    ] {
        assert!(
            checks.iter().any(|c| c["id"] == id),
            "no {id} check in {checks:?}"
        );
    }
    assert_eq!(v["version"]["current"], env!("CARGO_PKG_VERSION"));
    assert!(v["volumes"]["bundle"].as_u64().unwrap() > 0);
    assert!(v["compute"]["cpus"].as_u64().unwrap() >= 1);
}

#[tokio::test]
async fn an_upgrade_is_only_asked_of_an_installed_helper_for_a_published_tag() {
    let (app, s) = app_with_bundle("infra-upgrade", &sample());
    let dir = s.dir().to_path_buf();
    let cookie = format!("{}; axgf_lang=en", admin_cookie(&app).await);
    let post = |body: &'static str| {
        let cookie = cookie.clone();
        let app = app.clone();
        async move { post_form_as(&app, &cookie, "/admin/infrastructure/upgrade", body).await }
    };

    // No helper: refused, and nothing written.
    let resp = post("version=v99.0.0").await;
    assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);
    assert!(body_string(resp)
        .await
        .contains("The upgrade helper is not installed"));
    assert!(!dir.join("upgrade.request").exists());

    // The helper, and a list: the page offers the newer release.
    std::fs::write(dir.join("helper.json"), "{}").unwrap();
    write_channel(&dir);
    let html = body_string(get_with_cookie(&app, "/admin/infrastructure", &cookie).await).await;
    assert!(
        html.contains(r#"<option value="v99.0.0" selected>"#),
        "the newer stable release is preselected"
    );
    assert!(html.contains("v99.0.0 available"));
    assert!(html.contains(r#"action="/admin/infrastructure/upgrade""#));

    // Not a published tag; not a tag at all.
    assert_eq!(
        post("version=v98.0.0").await.status(),
        StatusCode::UNPROCESSABLE_ENTITY
    );
    assert_eq!(
        post("version=main%3Brm").await.status(),
        StatusCode::UNPROCESSABLE_ENTITY
    );
    // Older, without the tag typed again.
    let resp = post("version=v0.0.1").await;
    assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);
    assert!(body_string(resp)
        .await
        .contains("Type its tag in the confirmation field"));
    assert!(!dir.join("upgrade.request").exists());

    // The newer one: asked for, once.
    let resp = post("version=v99.0.0").await;
    assert_eq!(resp.status(), StatusCode::OK);
    assert!(body_string(resp)
        .await
        .contains("v99.0.0 has been requested."));
    let req: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(dir.join("upgrade.request")).unwrap())
            .unwrap();
    assert_eq!(req["version"], "v99.0.0");
    assert_eq!(
        req["requested_by"], "emergency-token",
        "the session that asked"
    );
    let resp = post("version=v0.0.1&confirm=v0.0.1").await;
    assert_eq!(resp.status(), StatusCode::CONFLICT, "one request at a time");

    // The helper's report is shown.
    std::fs::remove_file(dir.join("upgrade.request")).unwrap();
    std::fs::write(
        dir.join("upgrade.status"),
        serde_json::json!({ "state": "failed", "from": "v0.5.0", "to": "v99.0.0",
                            "finished_at": "2026-10-21T09:00:00Z", "message": "", "log": ["health check failed"] })
            .to_string(),
    )
    .unwrap();
    let html = body_string(get_with_cookie(&app, "/admin/infrastructure", &cookie).await).await;
    assert!(html.contains(
        "The change from v0.5.0 to v99.0.0 failed; v0.5.0 is still the version running."
    ));
    assert!(html.contains("health check failed"));
}
