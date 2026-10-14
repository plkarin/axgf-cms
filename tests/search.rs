//! The masthead search, driven through the router.

mod common;

use axum::http::StatusCode;
use common::*;
use serde_json::{json, Value};

fn person(id: &str, given: &str, surname: &str, visibility: &str) -> Value {
    json!({"id": id, "type": "person", "axgf_version": "1.1", "version_num": 1,
        "identity": {"name": {"display": format!("{given} {surname}"),
            "components": [{"type": "given_name", "value": given},
                           {"type": "family_name", "value": surname}]},
            "gender": {"value": "U"}, "is_living": false, "visibility": visibility},
        "birth": {"date": {"value": "1858", "precision": "year"}}})
}

fn app() -> (axum::Router, Scratch) {
    let flat = json!({
        "manifest": {"axgf": "1.1", "created_at": "2026-10-10T00:00:00Z"},
        "persons": {
            "p1": person("p1", "Józef", "Wierzbięta", "public"),
            "p2": person("p2", "Zuzanna", "Ukryta", "members"),
        },
        "families": {}, "events": {}, "places": {}, "occupations": {}, "links": {},
        "sources": {}, "documents": {}
    });
    let src = scratch("search-src");
    let path = src.join("search.axgf");
    std::fs::write(
        &path,
        axgf_cms::state::export_to_bytes(&flat.to_string()).unwrap(),
    )
    .unwrap();
    let out = app_with_bundle("search", &path);
    drop(src);
    out
}

#[tokio::test]
async fn a_name_is_found_however_it_is_typed() {
    let (app, _s) = app();
    for q in ["wierzbieta", "WIERZBIĘTA", "jozef wierz", "Józef"] {
        let body = expect_status(
            get(&app, &format!("/search.json?q={}", urlencoding(q))).await,
            StatusCode::OK,
            q,
        )
        .await;
        let reply: Value = serde_json::from_str(&body).unwrap();
        let hits = &reply["hits"];
        assert_eq!(hits.as_array().unwrap().len(), 1, "{q}: {body}");
        assert_eq!(reply["total"], 1);
        // The river's line for them, and the footer's count (B3).
        assert!(
            (1..=5).contains(&hits[0]["class"].as_u64().unwrap()),
            "{body}"
        );
        assert!(
            reply["summary"].as_str().unwrap().starts_with('1'),
            "{body}"
        );
        assert_eq!(hits[0]["id"], "p1");
        assert_eq!(hits[0]["years"], "1858–");
    }
    let page = expect_status(
        get(&app, "/search?q=wierzbieta").await,
        StatusCode::OK,
        "page",
    )
    .await;
    assert!(
        page.contains(r#"href="/?p=p1""#),
        "the result travels the river"
    );
    assert!(
        page.contains(r#"href="/person/p1""#),
        "and opens the record"
    );
}

/// A visitor learns nothing about a person they may not see: searching their
/// exact name finds nobody, in the suggestions and on the page, exactly as
/// it would for a name that is in nobody's record.
#[tokio::test]
async fn a_hidden_person_is_never_found_by_someone_who_may_not_see_them() {
    let (app, _s) = app();
    for q in ["Ukryta", "zuzanna"] {
        let body = expect_status(
            get(&app, &format!("/search.json?q={q}")).await,
            StatusCode::OK,
            q,
        )
        .await;
        // Exactly what a name in nobody's record gets back.
        let nobody = body_string(get(&app, "/search.json?q=nobodyatall").await).await;
        assert_eq!(body, nobody, "a visitor searching {q:?}");
        let reply: Value = serde_json::from_str(&body).unwrap();
        assert_eq!(reply["total"], 0, "{body}");
        let page = expect_status(
            get(&app, &format!("/search?q={q}")).await,
            StatusCode::OK,
            q,
        )
        .await;
        assert!(
            !page.contains("p2") && !page.contains("Ukryta\u{0020}") && !page.contains("Zuzanna"),
            "{page}"
        );
        let nobody = expect_status(
            get(&app, "/search?q=nobodyatall").await,
            StatusCode::OK,
            "nobody",
        )
        .await;
        let strip = |s: &str| s.replace(q, "").replace("nobodyatall", "");
        assert_eq!(
            strip(&page),
            strip(&nobody),
            "the page for a hidden name is the page for no name"
        );
    }
    // Signed in, the same search finds them.
    let cookie = admin_cookie(&app).await;
    let body = body_string(get_with_cookie(&app, "/search.json?q=ukryta", &cookie).await).await;
    assert!(body.contains("\"p2\""), "{body}");
}

fn urlencoding(s: &str) -> String {
    s.bytes()
        .map(|b| {
            if b.is_ascii_alphanumeric() {
                (b as char).to_string()
            } else {
                format!("%{b:02X}")
            }
        })
        .collect()
}
