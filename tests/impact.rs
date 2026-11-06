//! The Universal Synoptic Timeline, "Impact" (the operator's request of
//! 6 November 2026, docs/adr/0034): its place in the masthead, the family's
//! lives along the world's history, a person in their times, the way to it
//! from a record, and the reader's lens over all of it.

mod common;

use axum::http::StatusCode;
use common::*;
use serde_json::{json, Value};

const AYMERIC: &str = "25249f60-8614-401f-892d-209ca3db58b6";

fn sample() -> (axum::Router, Scratch) {
    let sample = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("deploy/sample.axgf");
    app_with_bundle("impact", &sample)
}

/// A page in English, its attributes' slashes as a browser reads them.
async fn page(app: &axum::Router, uri: &str, session: Option<&str>) -> (StatusCode, String) {
    let cookie = match session {
        Some(s) => format!("{s}; axgf_lang=en"),
        None => "axgf_lang=en".to_string(),
    };
    let resp = get_with_cookie(app, uri, &cookie).await;
    let status = resp.status();
    (status, body_string(resp).await.replace("&#x2f;", "/"))
}

#[tokio::test]
async fn the_section_sits_between_the_family_and_import() {
    let (app, _s) = sample();
    let (status, html) = page(&app, "/impact", None).await;
    assert_eq!(status, StatusCode::OK);
    let nav = html
        .split("<nav>")
        .nth(1)
        .unwrap()
        .split("</nav>")
        .next()
        .unwrap();
    let at = |href: &str| nav.find(&format!("href=\"{href}\"")).unwrap();
    assert!(
        at("/about") < at("/impact") && at("/impact") < at("/convert"),
        "{nav}"
    );
    assert!(nav.contains("<a href=\"/impact\" aria-current=\"page\">Impact</a>"));
}

#[tokio::test]
async fn the_family_lies_along_the_history_of_the_world() {
    let (app, _s) = sample();
    let (_, html) = page(&app, "/impact", None).await;
    // From the end of the last ice age to today, the wars with their length.
    assert!(html.contains("End of the last ice age and the Neolithic revolution"));
    assert!(html.contains("First World War"));
    assert!(html.contains("4 years 3 months"));
    assert!(html.contains("10000 BC"), "the axis begins at the dawn");
    // The family lived in Europe: its continent's landmarks beside the world's.
    assert!(html.contains("Continents · Europe"));
    assert!(html.contains("Congress of Vienna"));
    // Each life a window, to its own view.
    assert!(html.contains(&format!("href=\"/impact/{AYMERIC}\"")));
    assert!(html.contains("title=\"Aymeric Meunier · 1798–1861\""));
    assert!(html.contains("/static/impact.js"));

    // The family's own centuries: no landmark from before them.
    let (_, html) = page(&app, "/impact?span=family", None).await;
    assert!(html.contains("<a href=\"/impact?span=family\" aria-current=\"page\">"));
    assert!(
        !html.contains(" BC"),
        "nothing before the family's centuries"
    );
    assert!(html.contains("First World War"));
}

#[tokio::test]
async fn a_person_is_seen_in_their_times() {
    let (app, _s) = sample();
    let (status, html) = page(&app, &format!("/impact/{AYMERIC}"), None).await;
    assert_eq!(status, StatusCode::OK);
    assert!(html.contains("Aymeric Meunier in their times"));
    assert!(
        html.contains("data-country=\"FR\""),
        "their country, from their places"
    );
    // Their country's landmarks with what they meant, and their age at each.
    assert!(html.contains("Civil registers pass to the State"));
    assert!(html.contains("Town halls, not parishes, now record births"));
    assert!(html.contains("Napoleonic Wars"));
    assert!(html.contains("<strong>aged 5</strong>"));
    assert!(html.contains("1798 · Birth"));
    // Not another continent's history.
    assert!(!html.contains("Qing dynasty"));
    // Its way back, to the family and to the record.
    assert!(html.contains("href=\"/impact\""));
    assert!(html.contains(&format!("href=\"/person/{AYMERIC}\"")));

    // The record offers the way.
    let (_, record) = page(&app, &format!("/person/{AYMERIC}"), None).await;
    assert!(record.contains(&format!(
        "<a class=\"btn\" href=\"/impact/{AYMERIC}\">See the impact</a>"
    )));
}

fn person(id: &str, name: &str, born: Option<&str>, living: bool, visibility: &str) -> Value {
    let mut p = json!({"id": id, "type": "person", "axgf_version": "1.1", "version_num": 1,
        "identity": {"name": {"display": name, "components": []},
                     "is_living": living, "visibility": visibility}});
    if let Some(b) = born {
        p["birth"] = json!({"date": {"value": b, "precision": "year"}});
    }
    p
}

#[tokio::test]
async fn the_timeline_reads_through_the_readers_lens() {
    let dir = scratch("impact-lens");
    let path = dir.join("lens.axgf");
    let flat = json!({
        "manifest": {"axgf": "1.1"},
        "persons": {
            "p1": person("p1", "Jan Nowak", Some("1890"), false, "public"),
            "p2": person("p2", "Ewa Nowak", Some("1960"), true, "members"),
            "p3": person("p3", "Kto Inny", None, false, "public"),
        },
        "families": {}, "events": {}, "links": {}, "occupations": {}, "places": {},
        "sources": {}, "documents": {}
    });
    std::fs::write(
        &path,
        axgf_cms::state::export_to_bytes(&flat.to_string()).unwrap(),
    )
    .unwrap();
    let (app, _s) = app_with_bundle("impact-lens-app", &path);
    drop(dir);

    // Signed out: the one who lives is counted, never named.
    let (_, html) = page(&app, "/impact", None).await;
    assert!(html.contains("Jan Nowak"));
    assert!(!html.contains("Ewa Nowak") && !html.contains("/impact/p2"));
    assert!(html.contains("Not shown: 1"));
    assert!(html.contains("Without a date: 1"));
    let (status, _) = page(&app, "/impact/p2", None).await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    // A record with nothing to place it by says so, and leads back.
    let (status, html) = page(&app, "/impact/p3", None).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert!(html.contains("this person cannot be placed"));
    assert!(html.contains("href=\"/person/p3\""));
    let (status, _) = page(&app, "/impact/nobody", None).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    // Nor does their record offer the way.
    let (_, record) = page(&app, "/person/p3", None).await;
    assert!(!record.contains("href=\"/impact/p3\""));

    // An administrator reads everyone.
    let admin = admin_cookie(&app).await;
    let (_, html) = page(&app, "/impact", Some(&admin)).await;
    assert!(html.contains("Ewa Nowak"));
    let (status, html) = page(&app, "/impact/p2", Some(&admin)).await;
    assert_eq!(status, StatusCode::OK);
    assert!(html.contains("Ewa Nowak in their times"));
}
