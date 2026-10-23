//! An account's own page and its record in the tree (2026-10-23): any
//! signed-in account links itself to a person it may read, on its own word,
//! and may take the link away.

mod common;

use axum::http::{header, StatusCode};
use common::*;
use serde_json::json;

const LUC: &str = "f1000000-0000-4000-8000-000000000001";
const HID: &str = "f1000000-0000-4000-8000-000000000002";

fn bundle(tag: &str) -> common::Scratch {
    let dir = scratch(tag);
    let path = dir.join("a.axgf");
    let person = |id: &str, name: &str, vis: &str| {
        json!({"id": id, "type": "person", "axgf_version": "1.1",
               "identity": {"name": {"display": name, "components": []},
                            "is_living": false, "visibility": vis}})
    };
    let flat = json!({
        "manifest": {"axgf": "1.1"},
        "persons": {LUC: person(LUC, "Lucien Fabre", "public"), HID: person(HID, "Hector Fabre", "private")},
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

async fn viewer(app: &axum::Router) -> String {
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
    login.headers()[header::SET_COOKIE]
        .to_str()
        .unwrap()
        .split(';')
        .next()
        .unwrap()
        .to_string()
}

fn linked(scratch: &common::Scratch) -> Option<String> {
    let path = scratch.dir().join("family.axgf");
    let acl = axgf_cms::acl::Acl::load(&axgf_cms::acl::Acl::path_for(&path)).expect("the accounts");
    acl.users
        .iter()
        .find(|u| u.username == "vera")
        .and_then(|u| u.person_id.clone())
}

#[tokio::test]
async fn an_account_links_itself_to_its_record_and_may_take_it_back() {
    let src = bundle("acct-src");
    let (app, scratch) = app_with_bundle("acct", &src);
    let vera = viewer(&app).await;

    // Every signed-in account reaches its page from the masthead.
    let home = body_string(get_with_cookie(&app, "/about", &vera).await).await;
    assert!(home.contains(r#"href="/account""#));
    let page = body_string(get_with_cookie(&app, "/account", &vera).await).await;
    assert!(page.contains("My record in the tree"));
    assert!(page.contains("Nobody checks it"), "each account's own word");
    assert!(
        page.contains("Lucien Fabre"),
        "the records it may read are offered"
    );
    assert!(!page.contains("Hector Fabre"), "and only those");

    let resp = post_form_as(&app, &vera, "/account/person", &format!("person={LUC}")).await;
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    assert_eq!(resp.headers()[header::LOCATION], "/account?done=linked");
    assert_eq!(linked(&scratch).as_deref(), Some(LUC));
    let page = body_string(get_with_cookie(&app, "/account?done=linked", &vera).await).await;
    assert!(page.contains(&format!(r#"href="/person/{LUC}""#)));
    assert!(page.contains("notice ok"));

    // The administrator sees it on the list of accounts.
    let admin = admin_cookie(&app).await;
    let users = body_string(get_with_cookie(&app, "/admin/users", &admin).await).await;
    assert!(users.contains(&format!(
        r#"<a href="/person/{LUC}"><bdi>Lucien Fabre</bdi></a>"#
    )));

    // A record this account may not read, or none of that name: refused.
    let resp = post_form_as(&app, &vera, "/account/person", &format!("person={HID}")).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    let resp = post_form_as(&app, &vera, "/account/person", "person=Personne%20Inconnue").await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    assert!(
        !body_string(resp).await.contains("pick-error-"),
        "a sentence, not a key"
    );
    assert_eq!(linked(&scratch).as_deref(), Some(LUC), "unchanged");

    // Taken back.
    let resp = post_form_as(&app, &vera, "/account/person", "unlink=1").await;
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    assert_eq!(linked(&scratch), None);
}

#[tokio::test]
async fn a_visitor_is_sent_to_sign_in_and_the_token_is_no_one() {
    let src = bundle("acct-anon-src");
    let (app, _s) = app_with_bundle("acct-anon", &src);
    let resp = get(&app, "/account").await;
    assert!(resp.status().is_redirection());
    assert_eq!(resp.headers()[header::LOCATION], "/admin/login");
    let admin = admin_cookie(&app).await;
    let page = body_string(get_with_cookie(&app, "/account", &admin).await).await;
    assert!(page.contains("not an account"));
    let resp = post_form_as(&app, &admin, "/account/person", &format!("person={LUC}")).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}
