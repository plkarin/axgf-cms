//! The family the archive describes (2026-10-23): an administrator changes
//! `manifest.family` and `manifest.license` from the bundle room; the name
//! reaches the Family page, the accounts follow it, and the change is in the
//! history. And the export's button is idle again once its file starts.

mod common;

use axum::http::{header, StatusCode};
use common::*;
use scraper::{Html, Selector};
use serde_json::{json, Value};

fn bundle(tag: &str) -> common::Scratch {
    let dir = scratch(tag);
    let path = dir.join("m.axgf");
    let person = |id: &str, name: &str, born: &str| {
        json!({"id": id, "type": "person", "axgf_version": "1.1",
               "identity": {"name": {"display": name, "components": []},
                            "is_living": false, "visibility": "public"},
               "birth": {"date": {"value": born, "precision": if born.len() == 4 { "year" } else { "month" }}}})
    };
    let flat = json!({
        "manifest": {"axgf": "1.1", "created_at": "2026-01-02T03:04:05Z",
                     "family": {"name": "Fabre", "primary_culture": "fr", "primary_place": "Vallée de la Sarthe"}},
        "persons": {
            "e1000000-0000-4000-8000-000000000001": person("e1000000-0000-4000-8000-000000000001", "Lucien Fabre", "1702"),
            "e1000000-0000-4000-8000-000000000002": person("e1000000-0000-4000-8000-000000000002", "Alice Fabre", "1998-05"),
            "e1000000-0000-4000-8000-000000000003": person("e1000000-0000-4000-8000-000000000003", "Paul Fabre", "1850"),
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

/// The form's fields as the page gives them.
fn fields_of(html: &str) -> Vec<(String, String)> {
    let doc = Html::parse_document(html);
    let form = doc
        .select(&Selector::parse(r#"form[action="/admin/bundle/family"]"#).unwrap())
        .next()
        .expect("the family's form");
    let mut out = Vec::new();
    for el in form.select(&Selector::parse("input[name], textarea[name], select[name]").unwrap()) {
        let name = el.value().attr("name").unwrap().to_string();
        let value = match el.value().name() {
            "textarea" => el.text().collect(),
            "select" => el
                .select(&Selector::parse("option[selected]").unwrap())
                .next()
                .and_then(|o| o.value().attr("value"))
                .unwrap_or("")
                .to_string(),
            _ => el.value().attr("value").unwrap_or("").to_string(),
        };
        out.push((name, value));
    }
    out
}

fn with(fields: &[(String, String)], changes: &[(&str, &str)]) -> String {
    let mut all: Vec<(&str, &str)> = fields
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();
    for (k, v) in changes {
        match all.iter_mut().find(|(n, _)| n == k) {
            Some(slot) => slot.1 = v,
            None => all.push((k, v)),
        }
    }
    form(&all)
}

async fn manifest(app: &axum::Router, admin: &str) -> Value {
    let resp = get_with_cookie(app, "/admin/export", admin).await;
    assert_eq!(resp.status(), StatusCode::OK);
    axgf_rs::import_bundle(&body_bytes(resp).await).data["manifest"].clone()
}

#[tokio::test]
async fn an_administrator_names_the_family_and_the_accounts_follow() {
    let src = bundle("manifest-src");
    let (app, scratch) = app_with_bundle("manifest", &src);
    let admin = admin_cookie(&app).await;

    let page = body_string(get_with_cookie(&app, "/admin/bundle", &admin).await).await;
    let fields = fields_of(&page);
    assert!(fields.contains(&("name".into(), "Fabre".into())));
    assert!(fields.contains(&("primary_culture".into(), "fr".into())));
    // A place the file already has, outside the list, is kept on offer.
    assert!(fields.contains(&("primary_place".into(), "Vallée de la Sarthe".into())));
    let doc = Html::parse_document(&page);
    let places: Vec<String> = doc
        .select(&Selector::parse(r#"select[name="primary_place"] option"#).unwrap())
        .map(|o| o.value().attr("value").unwrap_or("").to_string())
        .collect();
    for c in [
        "",
        "Vallée de la Sarthe",
        "Africa",
        "Europe",
        "Several continents",
    ] {
        assert!(places.iter().any(|p| p == c), "{c} offered: {places:?}");
    }
    let licences = doc
        .select(&Selector::parse(r#"select[name="license_type"] option"#).unwrap())
        .count();
    assert_eq!(licences, 11, "none, and the ten licences");
    // The period is the births': shown, never typed.
    assert!(
        page.contains("1702 – 1998-05"),
        "the period the births give"
    );
    assert!(!page.contains(r#"name="earliest""#) && !page.contains(r#"name="license_url""#));
    // Each licence says what it lets others do.
    assert!(page.contains("as long as they name its authors"));

    let resp = post_form_as(
        &app,
        &admin,
        "/admin/bundle/family",
        &with(
            &fields,
            &[
                ("name", "Fabre-Delaunay"),
                ("description", "Deux branches, une vallée."),
                ("primary_culture", "fr-FR"),
                ("primary_place", "Europe"),
                ("license_type", "CC-BY-4.0"),
            ],
        ),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    assert_eq!(
        resp.headers()[header::LOCATION],
        "/admin/bundle?family=saved#family"
    );
    let back = body_string(get_with_cookie(&app, "/admin/bundle?family=saved", &admin).await).await;
    assert!(back.contains("notice ok"), "the page says it was saved");
    assert!(
        back.contains("Emergency token"),
        "and who chose the licence"
    );

    // In the file, as the format writes it.
    let m = manifest(&app, &admin).await;
    assert_eq!(m["family"]["name"], "Fabre-Delaunay");
    assert_eq!(m["family"]["description"], "Deux branches, une vallée.");
    assert_eq!(m["family"]["primary_culture"], "fr-FR");
    assert_eq!(m["family"]["primary_place"], "Europe");
    assert_eq!(
        m["family"]["time_span"],
        json!({"earliest": "1702", "latest": "1998-05"})
    );
    assert_eq!(m["license"]["type"], "CC-BY-4.0");
    assert_eq!(
        m["license"]["url"],
        "https://creativecommons.org/licenses/by/4.0/"
    );
    let stamp = &m["license"]["extensions"]["axgf-cms:license/v1"];
    assert_eq!(stamp["by"], "emergency-token");
    let at = stamp["at"].as_str().expect("when").to_string();
    assert_eq!(
        m["created_at"], "2026-01-02T03:04:05Z",
        "the file's own dates are untouched"
    );

    // On the Family page.
    let about = body_string(get(&app, "/about").await).await;
    assert!(about.contains("Fabre-Delaunay"));

    // The accounts are bound to the new name: a restart does not take them
    // for another family's.
    let path = scratch.dir().join("family.axgf");
    let acl = axgf_cms::acl::Acl::load(&axgf_cms::acl::Acl::path_for(&path)).expect("the accounts");
    assert!(matches!(
        acl.check_binding(Some(&m), None),
        axgf_cms::acl::Binding::Ok
    ));

    // In the history, by who and what.
    let dash = body_string(get_with_cookie(&app, "/admin", &admin).await).await;
    assert!(
        dash.contains(r#"href="/admin/bundle#family""#),
        "the history links to the form"
    );
    assert!(dash.contains("Family name"), "and names what changed");

    // Saved again with the same licence, who chose it does not change; the
    // place emptied leaves the file; the licence emptied goes whole.
    let page = body_string(get_with_cookie(&app, "/admin/bundle", &admin).await).await;
    let resp = post_form_as(
        &app,
        &admin,
        "/admin/bundle/family",
        &with(&fields_of(&page), &[("name", "Fabre")]),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    let m = manifest(&app, &admin).await;
    assert_eq!(
        m["license"]["extensions"]["axgf-cms:license/v1"]["at"],
        at.as_str()
    );
    let page = body_string(get_with_cookie(&app, "/admin/bundle", &admin).await).await;
    let resp = post_form_as(
        &app,
        &admin,
        "/admin/bundle/family",
        &with(
            &fields_of(&page),
            &[("primary_place", ""), ("license_type", "")],
        ),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    let m = manifest(&app, &admin).await;
    assert!(m["family"].get("primary_place").is_none());
    assert!(m.get("license").is_none());
    assert_eq!(m["family"]["name"], "Fabre");
}

#[tokio::test]
async fn a_wrong_field_or_a_change_made_meanwhile_keeps_what_was_typed() {
    let src = bundle("manifest-bad-src");
    let (app, _s) = app_with_bundle("manifest-bad", &src);
    let admin = admin_cookie(&app).await;
    let page = body_string(get_with_cookie(&app, "/admin/bundle", &admin).await).await;
    let fields = fields_of(&page);

    for (changes, key_text) in [
        (
            vec![("name", "Gardé"), ("primary_culture", "fr_FR!")],
            "language tag",
        ),
        (
            vec![("name", "Gardé"), ("primary_place", "Atlantis")],
            "main place from the list",
        ),
        (
            vec![("name", "Gardé"), ("license_type", "WTFPL")],
            "licence from the list",
        ),
    ] {
        let resp = post_form_as(
            &app,
            &admin,
            "/admin/bundle/family",
            &with(&fields, &changes),
        )
        .await;
        assert_eq!(
            resp.status(),
            StatusCode::UNPROCESSABLE_ENTITY,
            "{changes:?}"
        );
        let body = body_string(resp).await;
        assert!(body.contains(key_text), "{changes:?}: the reason");
        assert!(
            fields_of(&body).contains(&("name".into(), "Gardé".into())),
            "what was typed is kept"
        );
    }
    assert_eq!(
        manifest(&app, &admin).await["family"]["name"],
        "Fabre",
        "nothing was written"
    );

    // Two administrators from the same page: the second is told, keeps
    // their text, and saving again replaces the first's knowingly.
    let first = post_form_as(
        &app,
        &admin,
        "/admin/bundle/family",
        &with(&fields, &[("name", "Premier")]),
    )
    .await;
    assert_eq!(first.status(), StatusCode::SEE_OTHER);
    let second = post_form_as(
        &app,
        &admin,
        "/admin/bundle/family",
        &with(&fields, &[("name", "Second")]),
    )
    .await;
    assert_eq!(second.status(), StatusCode::CONFLICT);
    let body = body_string(second).await;
    let again = fields_of(&body);
    assert!(again.contains(&("name".into(), "Second".into())));
    assert_eq!(manifest(&app, &admin).await["family"]["name"], "Premier");
    let resp = post_form_as(&app, &admin, "/admin/bundle/family", &with(&again, &[])).await;
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    assert_eq!(manifest(&app, &admin).await["family"]["name"], "Second");
}

#[tokio::test]
async fn only_an_administrator_changes_the_family() {
    let src = bundle("manifest-role-src");
    let (app, _s) = app_with_bundle("manifest-role", &src);
    let admin = admin_cookie(&app).await;
    let made = post_form(
        &app,
        "/admin/users",
        "username=cora&password=correct-horse-battery&role=contributor&email=&family_scope=",
        true,
    )
    .await;
    assert!(made.status().is_redirection() || made.status().is_success());
    let login = post_form(
        &app,
        "/admin/login",
        "username=cora&password=correct-horse-battery",
        false,
    )
    .await;
    let cora = login.headers()[header::SET_COOKIE]
        .to_str()
        .unwrap()
        .split(';')
        .next()
        .unwrap()
        .to_string();
    let page = body_string(get_with_cookie(&app, "/admin/bundle", &admin).await).await;
    let resp = post_form_as(
        &app,
        &cora,
        "/admin/bundle/family",
        &with(&fields_of(&page), &[("name", "Pris")]),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
    let anonymous = post_form(&app, "/admin/bundle/family", "name=Pris", false).await;
    assert_ne!(anonymous.status(), StatusCode::SEE_OTHER);
    assert_eq!(manifest(&app, &admin).await["family"]["name"], "Fabre");
}

#[tokio::test]
async fn an_export_tells_the_page_its_file_has_started() {
    let src = bundle("manifest-dl-src");
    let (app, _s) = app_with_bundle("manifest-dl", &src);
    let admin = admin_cookie(&app).await;
    let page = body_string(get_with_cookie(&app, "/admin/bundle", &admin).await).await;
    let doc = Html::parse_document(&page);
    let forms = doc
        .select(&Selector::parse("form[data-download]").unwrap())
        .filter_map(|f| f.value().attr("action").map(str::to_string))
        .collect::<Vec<_>>();
    assert_eq!(
        forms,
        ["/admin/export", "/admin/bundle/sealed"],
        "both exports wait for their file"
    );

    let resp = get_with_cookie(&app, "/admin/export?download_token=abc123def456", &admin).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let cookie = resp.headers()[header::SET_COOKIE]
        .to_str()
        .unwrap()
        .to_string();
    assert!(
        cookie.starts_with("axgf_download=abc123def456;"),
        "{cookie}"
    );
    assert!(cookie.contains("Path=/admin") && cookie.contains("Max-Age=120"));
    assert!(!cookie.contains("HttpOnly"), "the page's script reads it");

    // Anything but a plain token is not echoed back.
    for bad in [
        "short",
        "abc123def456%3Bpath%3D%2F",
        "abc123def456abc123def456abc123def456abc123def456abc123def456abcde",
    ] {
        let resp =
            get_with_cookie(&app, &format!("/admin/export?download_token={bad}"), &admin).await;
        assert_eq!(resp.status(), StatusCode::OK);
        assert!(resp.headers().get(header::SET_COOKIE).is_none(), "{bad}");
    }
}
