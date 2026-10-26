//! The merge screen (src/routes/merge.rs): two records a person says are
//! one, merged by the library's plan and the administrator's decisions.
//!
//! Contract covered:
//! - The listing's bulk bar offers a merge to an administrator, for the kinds
//!   the library merges; anything but two ticked records is refused with the
//!   reason.
//! - Two ticked records open the plan, which changes nothing.
//! - A merge as planned leaves one record, and the other's old address says
//!   where it went; the absorbed record is in the journal.
//! - A plan made stale by another edit is refused, and nothing changes.
//! - Places merge through their own route, as their bulk bar does.
//!
//! Every name here is invented.

mod common;

use axum::http::{header, StatusCode};
use base64::Engine as _;
use common::*;
use serde_json::{json, Value};

fn u(n: u32) -> String {
    format!("00000000-0000-4000-8000-{n:012x}")
}

fn person(n: u32, display: &str, birth: &str, notes: &str) -> Value {
    json!({
        "id": u(n), "type": "person", "axgf_version": "1.0", "version_num": 1,
        "identity": {"name": {"display": display, "components": []},
                     "gender": {"value": "F"}, "is_living": false},
        "birth": {"date": {"value": birth, "precision": "year"}, "confidence": 0.6},
        "notes": notes
    })
}

fn place(n: u32, name: &str) -> Value {
    json!({"id": u(n), "type": "place", "axgf_version": "1.0",
           "names": [{"lang": "fr", "value": name, "is_primary": true}]})
}

/// A `.axgf` holding these records, written in `dir`.
fn bundle_in(dir: &Scratch, records: &[Value]) -> std::path::PathBuf {
    let mut b = axgf_rs::create_bundle(Some("Fusion")).data;
    for r in records {
        let coll = match r["type"].as_str().unwrap() {
            "person" => "persons",
            "place" => "places",
            other => panic!("{other}"),
        };
        b[coll][r["id"].as_str().unwrap()] = r.clone();
    }
    let env = axgf_rs::export_bundle(&b.to_string());
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(env.data["zip_base64"].as_str().unwrap())
        .unwrap();
    let path = dir.join("source.axgf");
    std::fs::write(&path, bytes).unwrap();
    path
}

/// The grandmother entered twice, and a third woman of another name.
fn grandmothers() -> Vec<Value> {
    vec![
        person(1, "Odile Fictive", "1834", "Baptisée à Ville-Inventée."),
        person(2, "Odile Fictive", "1835", "Inhumée à Bourg-Imaginaire."),
        person(3, "Albertine Inventée", "1840", ""),
    ]
}

async fn english_admin(app: &axum::Router) -> String {
    format!("{}; axgf_lang=en", admin_cookie(app).await)
}

/// Percent-encoded for a form body: everything but the unreserved bytes.
fn enc(s: &str) -> String {
    s.bytes()
        .map(|b| match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                (b as char).to_string()
            }
            _ => format!("%{b:02X}"),
        })
        .collect()
}

/// The values of a form's hidden inputs and checked radios, as a body.
fn form_body(html: &str) -> String {
    let doc = scraper::Html::parse_document(html);
    let sel = scraper::Selector::parse(
        r#"form[action$="/merge"] input[type=hidden], form[action$="/merge"] input[type=radio][checked]"#,
    )
    .unwrap();
    doc.select(&sel)
        .map(|i| {
            let v = i.value();
            format!(
                "{}={}",
                enc(v.attr("name").unwrap()),
                enc(v.attr("value").unwrap_or(""))
            )
        })
        .collect::<Vec<_>>()
        .join("&")
}

#[tokio::test]
async fn the_listing_offers_a_merge_to_an_administrator() {
    let src = scratch("merge-src");
    let (app, _s) = app_with_bundle("merge-list", &bundle_in(&src, &grandmothers()));
    let cookie = english_admin(&app).await;
    let html = body_string(get_with_cookie(&app, "/admin/person", &cookie).await).await;
    assert!(
        html.contains(r#"value="merge" data-bulk-pair"#),
        "no merge button"
    );
    // Events are not merged by the library: no button there.
    let html = body_string(get_with_cookie(&app, "/admin/event", &cookie).await).await;
    assert!(!html.contains("data-bulk-pair"));
}

#[tokio::test]
async fn anything_but_two_ticked_records_is_refused_with_the_reason() {
    let src = scratch("merge-src");
    let (app, _s) = app_with_bundle("merge-one", &bundle_in(&src, &grandmothers()));
    let cookie = english_admin(&app).await;
    let resp = post_form_as(
        &app,
        &cookie,
        "/admin/person/bulk",
        &format!("action=merge&id={}%7C1", u(1)),
    )
    .await;
    let html = expect_status(resp, StatusCode::BAD_REQUEST, "one ticked").await;
    assert!(html.contains("Tick exactly two records"), "{html}");
}

#[tokio::test]
async fn two_ticked_records_open_the_plan_which_changes_nothing() {
    let src = scratch("merge-src");
    let (app, _s) = app_with_bundle("merge-plan", &bundle_in(&src, &grandmothers()));
    let cookie = english_admin(&app).await;
    let resp = post_form_as(
        &app,
        &cookie,
        "/admin/person/bulk",
        &format!("action=merge&id={}%7C1&id={}%7C1", u(1), u(2)),
    )
    .await;
    assert!(resp.status().is_redirection());
    let to = resp.headers()[header::LOCATION]
        .to_str()
        .unwrap()
        .to_string();
    assert_eq!(
        to,
        format!("/admin/person/merge?keep={}&absorb={}", u(1), u(2))
    );

    let html = expect_status(
        get_with_cookie(&app, &to, &cookie).await,
        StatusCode::OK,
        "plan",
    )
    .await;
    assert!(html.contains(r#"name="fingerprint""#));
    assert!(html.contains("Where they differ"));
    // The births disagree: a choice, pre-set to the library's suggestion.
    assert!(html.contains(r#"name="choice:birth""#), "{html}");
    assert!(html.contains("Keep the other one"));
    // Nothing changed: both are still there.
    for n in [1, 2] {
        let resp = get_with_cookie(&app, &format!("/person/{}", u(n)), &cookie).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}

#[tokio::test]
async fn a_merge_as_planned_keeps_one_and_the_other_says_where_it_went() {
    let src = scratch("merge-src");
    let (app, _s) = app_with_bundle("merge-apply", &bundle_in(&src, &grandmothers()));
    let cookie = english_admin(&app).await;
    let plan = format!("/admin/person/merge?keep={}&absorb={}", u(1), u(2));
    let html = body_string(get_with_cookie(&app, &plan, &cookie).await).await;
    let body = form_body(&html);
    assert!(body.contains("choice%3Abirth="), "{body}");

    let resp = post_form_as(&app, &cookie, "/admin/person/merge", &body).await;
    let done = expect_status(resp, StatusCode::OK, "merge").await;
    assert!(done.contains("is now part of"), "{done}");
    assert!(done.contains("The merged record, as it was"));

    let kept = get_with_cookie(&app, &format!("/person/{}", u(1)), &cookie).await;
    assert_eq!(kept.status(), StatusCode::OK);
    let gone = get_with_cookie(&app, &format!("/person/{}", u(2)), &cookie).await;
    let gone = expect_status(gone, StatusCode::NOT_FOUND, "absorbed").await;
    assert!(gone.contains("It was merged into"), "{gone}");
    // The third woman is untouched.
    let other = get_with_cookie(&app, &format!("/person/{}", u(3)), &cookie).await;
    assert_eq!(other.status(), StatusCode::OK);
}

#[tokio::test]
async fn a_plan_made_stale_by_another_edit_is_refused_and_nothing_changes() {
    let src = scratch("merge-src");
    let mut records = grandmothers();
    records.push(person(4, "Odile Fictive", "1834", "Recensée en 1872."));
    let (app, _s) = app_with_bundle("merge-stale", &bundle_in(&src, &records));
    let cookie = english_admin(&app).await;
    let html = body_string(
        get_with_cookie(
            &app,
            &format!("/admin/person/merge?keep={}&absorb={}", u(1), u(2)),
            &cookie,
        )
        .await,
    )
    .await;
    let stale = form_body(&html);

    // Meanwhile, another merge changes the kept record.
    let html = body_string(
        get_with_cookie(
            &app,
            &format!("/admin/person/merge?keep={}&absorb={}", u(1), u(4)),
            &cookie,
        )
        .await,
    )
    .await;
    let resp = post_form_as(&app, &cookie, "/admin/person/merge", &form_body(&html)).await;
    expect_status(resp, StatusCode::OK, "the other merge").await;

    let resp = post_form_as(&app, &cookie, "/admin/person/merge", &stale).await;
    let html = body_string(resp).await;
    assert!(html.contains("STALE_PLAN"), "{html}");
    let still = get_with_cookie(&app, &format!("/person/{}", u(2)), &cookie).await;
    assert_eq!(
        still.status(),
        StatusCode::OK,
        "the stale plan changed something"
    );
}

#[tokio::test]
async fn places_merge_through_their_own_route() {
    let src = scratch("merge-src");
    let (app, _s) = app_with_bundle(
        "merge-place",
        &bundle_in(
            &src,
            &[place(10, "Ville-Inventée"), place(11, "Ville Inventée")],
        ),
    );
    let cookie = english_admin(&app).await;
    let resp = post_form_as(
        &app,
        &cookie,
        "/admin/place/bulk",
        &format!("action=merge&id={}%7C1&id={}%7C1", u(10), u(11)),
    )
    .await;
    assert!(resp.status().is_redirection(), "{}", resp.status());
    let to = resp.headers()[header::LOCATION]
        .to_str()
        .unwrap()
        .to_string();
    let html = expect_status(
        get_with_cookie(&app, &to, &cookie).await,
        StatusCode::OK,
        "plan",
    )
    .await;
    let resp = post_form_as(&app, &cookie, "/admin/place/merge", &form_body(&html)).await;
    let done = expect_status(resp, StatusCode::OK, "place merge").await;
    assert!(done.contains("is now part of"), "{done}");
}
