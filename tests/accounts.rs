//! Accounts, roles, sessions and the family scope.
//!
//! The tests that matter here are the negative ones. It is easy to build a
//! login form that lets the right people in; what has to be pinned is that it
//! keeps the wrong people out, that a role is a ceiling and not a suggestion,
//! and that revoking something takes effect on the next request rather than
//! whenever a cookie happens to expire.

mod common;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use common::*;
use serde_json::json;
use tower::ServiceExt as _;

const ROOT: &str = "11111111-1111-4111-8111-111111111111";
const KID: &str = "22222222-2222-4222-8222-222222222222";
const SPOUSE: &str = "33333333-3333-4333-8333-333333333333";
const OUTSIDER: &str = "44444444-4444-4444-8444-444444444444";
const FAMILY: &str = "55555555-5555-4555-8555-555555555555";

const PASSWORD: &str = "correct-horse-battery-staple";

fn person(id: &str, name: &str) -> serde_json::Value {
    json!({
        "id": id, "type": "person", "axgf_version": "1.0",
        "identity": {
            "name": {"display": name, "components": [
                {"type": "given_name", "value": name, "order": 1}]},
            "visibility": "public", "is_living": false
        }
    })
}

/// A tree with one branch (ROOT → KID, KID married to SPOUSE) and one person
/// standing entirely outside it.
fn bundle(tag: &str) -> std::path::PathBuf {
    let dir = scratch(tag);
    let path = dir.join("acc.axgf");
    let flat = json!({
        "manifest": {"axgf": "1.0", "family": {"name": "Scope"}},
        "persons": {
            ROOT: person(ROOT, "Root"),
            KID: person(KID, "Kid"),
            SPOUSE: person(SPOUSE, "Spouse"),
            OUTSIDER: person(OUTSIDER, "Outsider")
        },
        "families": {
            FAMILY: {
                "id": FAMILY, "type": "family", "axgf_version": "1.0",
                "union": {"persons": [{"person_id": ROOT, "role": "wife"}],
                          "union_type": "marriage"},
                "children": [{"person_id": KID, "confidence": 0.9}]
            },
            "66666666-6666-4666-8666-666666666666": {
                "id": "66666666-6666-4666-8666-666666666666",
                "type": "family", "axgf_version": "1.0",
                "union": {"persons": [{"person_id": KID}, {"person_id": SPOUSE}],
                          "union_type": "marriage"},
                "children": []
            }
        },
        "events": {}, "links": {}, "occupations": {},
        "sources": {}, "places": {}, "documents": {}
    });
    std::fs::write(
        &path,
        axgf_cms::state::export_to_bytes(&flat.to_string()).expect("export"),
    )
    .expect("write");
    path
}

/// An app whose `.acl` holds the given accounts.
fn app_with_accounts(tag: &str, users: &[(&str, axgf_cms::acl::Role, &[&str])]) -> axum::Router {
    let src = bundle(&format!("{tag}-src"));
    let dir = scratch(tag);
    let path = dir.join("family.axgf");
    std::fs::copy(&src, &path).expect("copy");

    let mut acl = axgf_cms::acl::Acl::default();
    for (username, role, scope) in users {
        let mut u = axgf_cms::acl::new_user(username, PASSWORD, *role).expect("new user");
        u.family_scope = scope.iter().map(|s| s.to_string()).collect();
        acl.users.push(u);
    }
    acl.save(&axgf_cms::acl::Acl::path_for(&path))
        .expect("save acl");

    axgf_cms::app(&path, TOKEN).expect("build app")
}

/// Sign in and return the session cookie.
async fn sign_in(app: &axum::Router, username: &str) -> String {
    let resp = post_form(
        app,
        "/admin/login",
        &format!("username={username}&password={PASSWORD}"),
        false,
    )
    .await;
    assert!(
        resp.status().is_redirection(),
        "{username} should sign in, got {}",
        resp.status()
    );
    resp.headers()
        .get(header::SET_COOKIE)
        .and_then(|v| v.to_str().ok())
        .and_then(|c| c.split(';').next())
        .expect("a session cookie")
        .to_string()
}

async fn get_as(app: &axum::Router, cookie: &str, uri: &str) -> axum::http::Response<Body> {
    app.clone()
        .oneshot(
            Request::builder()
                .uri(uri)
                .header(header::COOKIE, cookie)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("request")
}

async fn post_as(
    app: &axum::Router,
    cookie: &str,
    uri: &str,
    body: &str,
) -> axum::http::Response<Body> {
    app.clone()
        .oneshot(
            Request::builder()
                .uri(uri)
                .method("POST")
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .header(header::COOKIE, cookie)
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .expect("request")
}

// ---------------------------------------------------------------------------
// roles
// ---------------------------------------------------------------------------

#[tokio::test]
async fn a_viewer_may_read_but_not_reach_the_panel_at_all() {
    use axgf_cms::acl::Role;
    let app = app_with_accounts("acc-viewer", &[("vera", Role::Viewer, &[])]);
    let cookie = sign_in(&app, "vera").await;

    // Reading is fine.
    assert_eq!(
        get_as(&app, &cookie, &format!("/person/{ROOT}"))
            .await
            .status(),
        StatusCode::OK
    );
    // Writing is not, and the refusal says the role is why rather than showing
    // a login form for the account they are already using.
    for uri in ["/admin", "/admin/person", "/admin/person/new"] {
        let resp = get_as(&app, &cookie, uri).await;
        assert_eq!(
            resp.status(),
            StatusCode::FORBIDDEN,
            "{uri} must refuse a viewer"
        );
        let body = body_string(resp).await;
        assert!(
            body.contains("role"),
            "{uri}: the refusal should name the reason: {body}"
        );
    }
}

#[tokio::test]
async fn a_contributor_edits_but_may_not_delete_dedup_validate_or_export() {
    use axgf_cms::acl::Role;
    let app = app_with_accounts("acc-contrib", &[("carl", Role::Contributor, &[])]);
    let cookie = sign_in(&app, "carl").await;

    assert_eq!(
        get_as(&app, &cookie, "/admin").await.status(),
        StatusCode::OK,
        "a contributor reaches the panel"
    );
    assert_eq!(
        get_as(&app, &cookie, "/admin/person/new").await.status(),
        StatusCode::OK
    );

    // The admin-only operations.
    for (uri, body) in [
        ("/admin/validate", ""),
        ("/admin/dedup", ""),
        (
            concat!(
                "/admin/person/",
                "11111111-1111-4111-8111-111111111111",
                "/delete"
            ),
            "policy=reject",
        ),
    ] {
        assert_eq!(
            post_as(&app, &cookie, uri, body).await.status(),
            StatusCode::FORBIDDEN,
            "{uri} is an administrator's operation"
        );
    }
    assert_eq!(
        get_as(&app, &cookie, "/admin/export").await.status(),
        StatusCode::FORBIDDEN
    );
    assert_eq!(
        get_as(&app, &cookie, "/admin/users").await.status(),
        StatusCode::FORBIDDEN,
        "accounts are managed by administrators only"
    );
}

#[tokio::test]
async fn an_admin_reaches_everything() {
    use axgf_cms::acl::Role;
    let app = app_with_accounts("acc-admin", &[("ada", Role::Admin, &[])]);
    let cookie = sign_in(&app, "ada").await;
    for uri in ["/admin", "/admin/users", "/admin/export", "/admin/person"] {
        let resp = get_as(&app, &cookie, uri).await;
        let st = resp.status();
        let b = body_string(resp).await;
        assert_eq!(st, StatusCode::OK, "{uri} should open for an admin: {b}");
    }
}

// ---------------------------------------------------------------------------
// family scope
// ---------------------------------------------------------------------------

#[tokio::test]
async fn a_scoped_contributor_edits_inside_the_branch_and_nowhere_else() {
    use axgf_cms::acl::Role;
    let app = app_with_accounts("acc-scope", &[("sam", Role::Contributor, &[ROOT])]);
    let cookie = sign_in(&app, "sam").await;

    let edit =
        // `base_version` is what the edit form carries, and a save fails
        // closed without it. The fixture's people have no `version_num`,
        // which reads as version 0.
        |id: &str, name: &str| {
            format!(
                "base_version=0&identity.name.display={}&raw_json={}",
                urlencode(name),
                urlencode(&person(id, name).to_string())
            )
        };

    // ROOT is the branch root; KID is a descendant; SPOUSE married in.
    for (id, who) in [(ROOT, "root"), (KID, "a descendant"), (SPOUSE, "a spouse")] {
        let resp = post_as(
            &app,
            &cookie,
            &format!("/admin/person/{id}"),
            &edit(id, "Edited"),
        )
        .await;
        assert_eq!(
            resp.status(),
            StatusCode::OK,
            "{who} is inside the branch and must be editable"
        );
    }

    // The outsider is not reachable from the root by descent or marriage.
    let resp = post_as(
        &app,
        &cookie,
        &format!("/admin/person/{OUTSIDER}"),
        &edit(OUTSIDER, "Edited"),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
    let body = body_string(resp).await;
    assert!(
        body.contains("branch"),
        "the refusal names the reason: {body}"
    );
}

#[tokio::test]
async fn a_scope_limits_writing_but_never_reading() {
    // The rule that keeps the two systems apart. A branch-scoped account reads
    // the whole tree at its ceiling; the scope is about what it may change.
    use axgf_cms::acl::Role;
    let app = app_with_accounts("acc-scope-read", &[("sam", Role::Contributor, &[ROOT])]);
    let cookie = sign_in(&app, "sam").await;
    let body = expect_status(
        get_as(&app, &cookie, &format!("/person/{OUTSIDER}")).await,
        StatusCode::OK,
        "a scoped account still reads outside its branch",
    )
    .await;
    assert!(body.contains("Outsider"));
}

#[tokio::test]
async fn a_scoped_account_may_not_edit_a_record_that_names_nobody() {
    // A source or a place is about evidence and geography, not people, so
    // there is no branch to measure it against. Permitting it would be a hole
    // in the scope rather than an exception to it.
    use axgf_cms::acl::Role;
    let app = app_with_accounts("acc-scope-source", &[("sam", Role::Contributor, &[ROOT])]);
    let cookie = sign_in(&app, "sam").await;
    let src = json!({"type": "source", "axgf_version": "1.0",
                     "title": "A register", "source_type": "register"});
    let resp = post_as(
        &app,
        &cookie,
        "/admin/source",
        &format!("raw_json={}", urlencode(&src.to_string())),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn a_scoped_account_may_not_retarget_a_record_out_of_its_branch() {
    // Checking only the submitted form would let a scoped contributor take a
    // family they may edit and point it at somebody they may not — rewriting
    // that person's parentage from inside the branch.
    use axgf_cms::acl::Role;
    let app = app_with_accounts("acc-scope-retarget", &[("sam", Role::Contributor, &[ROOT])]);
    let cookie = sign_in(&app, "sam").await;

    let hijacked = json!({
        "id": FAMILY, "type": "family", "axgf_version": "1.0",
        "union": {"persons": [{"person_id": ROOT}], "union_type": "marriage"},
        "children": [{"person_id": KID}, {"person_id": OUTSIDER}]
    });
    let resp = post_as(
        &app,
        &cookie,
        &format!("/admin/family/{FAMILY}"),
        &format!(
            "base_version=0&raw_json={}",
            urlencode(&hijacked.to_string())
        ),
    )
    .await;
    assert_eq!(
        resp.status(),
        StatusCode::FORBIDDEN,
        "one foot outside the branch is outside the branch"
    );
}

#[tokio::test]
async fn a_scoped_account_may_not_attach_a_file_outside_its_branch() {
    // Document upload is a separate route with its own handler, so it is the
    // one most easily left out when a rule is added to the others — and it was,
    // once, in this very release. A write against a person's record is a write
    // against their record whatever entity the form happens to create.
    use axgf_cms::acl::Role;
    let app = app_with_accounts("acc-scope-upload", &[("sam", Role::Contributor, &[ROOT])]);
    let cookie = sign_in(&app, "sam").await;

    let png = {
        use base64::Engine as _;
        base64::engine::general_purpose::STANDARD
            .decode("iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8BQDwAEhQGAhKmMIQAAAABJRU5ErkJggg==")
            .unwrap()
    };

    let upload = |person: &str| {
        let boundary = "----axgfcmsscopeboundary";
        let mut body: Vec<u8> = Vec::new();
        body.extend_from_slice(
            format!(
                "--{boundary}\r\nContent-Disposition: form-data; name=\"file\"; \
                 filename=\"scan.png\"\r\nContent-Type: image/png\r\n\r\n"
            )
            .as_bytes(),
        );
        body.extend_from_slice(&png);
        body.extend_from_slice(format!("\r\n--{boundary}--\r\n").as_bytes());
        (
            format!("/admin/person/{person}/document"),
            format!("multipart/form-data; boundary={boundary}"),
            body,
        )
    };

    for (person, expect_refusal) in [(KID, false), (OUTSIDER, true)] {
        let (uri, ct, body) = upload(person);
        let resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri(&uri)
                    .method("POST")
                    .header(header::CONTENT_TYPE, ct)
                    .header(header::COOKIE, &cookie)
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .expect("request");
        if expect_refusal {
            assert_eq!(
                resp.status(),
                StatusCode::FORBIDDEN,
                "attaching a file to somebody outside the branch must be refused"
            );
        } else {
            assert!(
                resp.status().is_success() || resp.status().is_redirection(),
                "inside the branch it must work, got {}",
                resp.status()
            );
        }
    }
}

// ---------------------------------------------------------------------------
// sessions
// ---------------------------------------------------------------------------

#[tokio::test]
async fn a_forged_session_cookie_is_refused() {
    use axgf_cms::acl::Role;
    let app = app_with_accounts("acc-forge", &[("ada", Role::Admin, &[])]);
    let real = sign_in(&app, "ada").await;

    // Same shape, wrong signature.
    let (name_id, sig) = real.split_once('.').unwrap();
    let forged = format!("{name_id}.{}", "0".repeat(sig.len()));
    assert_eq!(
        get_as(&app, &forged, "/admin").await.status(),
        StatusCode::UNAUTHORIZED
    );
    assert_eq!(
        get_as(&app, "axgf_session=nonsense", "/admin")
            .await
            .status(),
        StatusCode::UNAUTHORIZED
    );
}

#[tokio::test]
async fn disabling_an_account_takes_effect_on_the_next_request() {
    // A live cookie that outlived the change would make the change advisory.
    use axgf_cms::acl::Role;
    let app = app_with_accounts(
        "acc-disable",
        &[("ada", Role::Admin, &[]), ("carl", Role::Contributor, &[])],
    );
    let admin = sign_in(&app, "ada").await;
    let carl = sign_in(&app, "carl").await;
    assert_eq!(get_as(&app, &carl, "/admin").await.status(), StatusCode::OK);

    // Find carl's id from the account list, then disable him.
    let list = body_string(get_as(&app, &admin, "/admin/users").await).await;
    let id = user_id_of(&list, "carl");

    let resp = post_as(
        &app,
        &admin,
        &format!("/admin/users/{id}"),
        "role=contributor&status=disabled&email=&family_scope=&password=",
    )
    .await;
    assert_eq!(resp.status(), StatusCode::OK);

    assert_eq!(
        get_as(&app, &carl, "/admin").await.status(),
        StatusCode::UNAUTHORIZED,
        "a disabled account's live session must stop working immediately"
    );
}

#[tokio::test]
async fn the_last_administrator_cannot_lock_everyone_out() {
    use axgf_cms::acl::Role;
    let app = app_with_accounts("acc-lastadmin", &[("ada", Role::Admin, &[])]);
    let cookie = sign_in(&app, "ada").await;
    let list = body_string(get_as(&app, &cookie, "/admin/users").await).await;
    let id = user_id_of(&list, "ada");

    let body = expect_status(
        post_as(
            &app,
            &cookie,
            &format!("/admin/users/{id}"),
            "role=viewer&status=active&email=&family_scope=&password=",
        )
        .await,
        StatusCode::OK,
        "demoting the last admin",
    )
    .await;
    assert!(
        body.contains("only active administrator"),
        "it must be refused with the reason: {body}"
    );
    assert_eq!(
        get_as(&app, &cookie, "/admin/users").await.status(),
        StatusCode::OK,
        "and the account keeps its rights"
    );
}

#[tokio::test]
async fn a_wrong_password_is_indistinguishable_from_an_unknown_account() {
    // The login form must not be an oracle for which accounts exist — there is
    // no self-registration here to make that knowledge harmless.
    use axgf_cms::acl::Role;
    let app = app_with_accounts("acc-oracle", &[("ada", Role::Admin, &[])]);

    let known = post_form(&app, "/admin/login", "username=ada&password=wrong", false).await;
    let unknown = post_form(
        &app,
        "/admin/login",
        "username=nobody&password=wrong",
        false,
    )
    .await;
    assert_eq!(known.status(), unknown.status());
    assert_eq!(
        body_string(known).await,
        body_string(unknown).await,
        "the two answers must be byte-identical"
    );
}

#[tokio::test]
async fn repeated_failures_are_throttled() {
    use axgf_cms::acl::Role;
    let app = app_with_accounts("acc-throttle", &[("ada", Role::Admin, &[])]);
    let mut throttled = false;
    for _ in 0..12 {
        let body = body_string(
            post_form(&app, "/admin/login", "username=ada&password=wrong", false).await,
        )
        .await;
        if body.contains("Too many failed attempts") {
            throttled = true;
            break;
        }
    }
    assert!(throttled, "a dictionary run must be cut off");

    // And the throttle holds even against the correct password, so it cannot
    // be used as a probe for when the guess was right.
    let body = body_string(
        post_form(
            &app,
            "/admin/login",
            &format!("username=ada&password={PASSWORD}"),
            false,
        )
        .await,
    )
    .await;
    assert!(body.contains("Too many failed attempts"));
}

#[tokio::test]
async fn signing_out_ends_the_session() {
    use axgf_cms::acl::Role;
    let app = app_with_accounts("acc-signout", &[("ada", Role::Admin, &[])]);
    let cookie = sign_in(&app, "ada").await;
    assert_eq!(
        get_as(&app, &cookie, "/admin").await.status(),
        StatusCode::OK
    );
    post_as(&app, &cookie, "/admin/logout", "").await;
    assert_eq!(
        get_as(&app, &cookie, "/admin").await.status(),
        StatusCode::UNAUTHORIZED
    );
}

#[tokio::test]
async fn the_acl_never_carries_a_password_into_a_page() {
    use axgf_cms::acl::Role;
    let app = app_with_accounts("acc-nohash", &[("ada", Role::Admin, &[])]);
    let cookie = sign_in(&app, "ada").await;
    let body = body_string(get_as(&app, &cookie, "/admin/users").await).await;
    assert!(
        !body.contains("$argon2id$"),
        "a hash has no business in a template context"
    );
    assert!(!body.contains(PASSWORD));
}

/// The account id of `username`, from the rendered account list.
///
/// The row states the username first and carries the form action after it, so
/// the id is the first one appearing past the name. Anchoring on the name
/// rather than on row order is what keeps this from silently testing the wrong
/// account when the list is reordered.
fn user_id_of(list: &str, username: &str) -> String {
    let at = list
        .find(&format!("<strong>{username}</strong>"))
        .unwrap_or_else(|| panic!("{username} should be listed"));
    let rest = &list[at..];
    let marker = "/admin/users/";
    let idx = rest
        .find(marker)
        .unwrap_or_else(|| panic!("no form action after {username}"));
    rest[idx + marker.len()..]
        .split('"')
        .next()
        .expect("an id")
        .to_string()
}

/// Minimal form encoding for the raw-JSON field.
fn urlencode(s: &str) -> String {
    let mut out = String::with_capacity(s.len() * 3);
    for b in s.as_bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(*b as char)
            }
            b' ' => out.push('+'),
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}
