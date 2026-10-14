//! The administration tables, as Study 06 draws them (B6).
//!
//! Hairlines and no zebra; a selection column; a bulk bar that takes over
//! the header row and exports or deletes the selected rows — the delete all
//! in one save or none, refusing a record another one still refers to; the
//! filiation drawn in the river's line styles; pagination as "1–50 of 866".

mod common;

use axum::http::{header, StatusCode};
use common::*;
use serde_json::{json, Value};

fn person(id: &str, name: &str) -> Value {
    json!({"id": id, "type": "person", "axgf_version": "1.1", "version_num": 1,
        "identity": {"name": {"display": name, "components": []},
            "gender": {"value": "U"}, "is_living": false, "visibility": "public"}})
}

/// A parent with three children recorded at three confidences, and two
/// people nothing refers to.
fn app() -> (axum::Router, Scratch) {
    let persons: serde_json::Map<String, Value> = [
        person("p0", "Jan Nowak"),
        person("p1", "Anna Nowak"),
        person("p2", "Piotr Nowak"),
        person("p3", "Zofia Nowak"),
        person("x1", "Loose One"),
        person("x2", "Loose Two"),
    ]
    .into_iter()
    .map(|p| (p["id"].as_str().unwrap().to_string(), p))
    .collect();
    let flat = json!({
        "manifest": {"axgf": "1.1", "created_at": "2026-10-12T00:00:00Z"},
        "persons": persons,
        "families": {"f1": {"id": "f1", "type": "family", "axgf_version": "1.1", "version_num": 1,
            "union": {"type": "marriage", "persons": [{"person_id": "p0", "role": "spouse"}]},
            "children": [{"person_id": "p1", "confidence": 0.95},
                         {"person_id": "p2", "confidence": 0.6},
                         {"person_id": "p3", "confidence": 0.3}]}},
        "events": {}, "places": {}, "occupations": {}, "links": {}, "sources": {}, "documents": {}
    });
    let src = scratch("tables-src");
    let path = src.join("tables.axgf");
    std::fs::write(
        &path,
        axgf_cms::state::export_to_bytes(&flat.to_string()).expect("export"),
    )
    .expect("write");
    let out = app_with_bundle("tables", &path);
    drop(src);
    out
}

async fn list(app: &axum::Router, cookie: &str, kind: &str) -> String {
    expect_status(
        get_with_cookie(app, &format!("/admin/{kind}"), cookie).await,
        StatusCode::OK,
        kind,
    )
    .await
}

#[tokio::test]
async fn a_listing_selects_rows_and_draws_filiation_in_the_rivers_lines() {
    let (app, _s) = app();
    let cookie = admin_cookie(&app).await;
    let html = list(&app, &cookie, "family").await;
    assert!(html.contains(r#"<form id="bulk" method="post" action="/admin/family/bulk">"#));
    assert!(
        html.contains(r#"name="id" value="f1|1""#),
        "a row's box carries its version"
    );
    assert!(html.contains("data-bulk-all"));
    assert!(html.contains(r#"<tr class="bulk-bar" data-bulk-bar data-js-hidden>"#));
    assert!(html.contains(r#"value="export""#) && html.contains(r#"value="confirm""#));
    // Exactly those two. No merge, not even a disabled one: the library has
    // no record merge, and a control that cannot work is not shipped.
    let bar = &html[html.find("data-bulk-bar").unwrap()..];
    let bar = &bar[..bar.find("</tr>").unwrap()];
    assert_eq!(bar.matches("<button").count(), 2, "{bar}");
    assert!(
        !bar.contains("disabled") && !bar.to_lowercase().contains("merge"),
        "{bar}"
    );
    // 0.95, 0.6, 0.3: attested, inferred, speculative — solid, dashed, dotted.
    let cell = &html[html
        .find(r#"<td class="filiation">"#)
        .expect("the filiation cell")..];
    let cell = &cell[..cell.find("</td>").unwrap()];
    let marks: Vec<&str> = cell
        .match_indices("fil fil-")
        .map(|(i, _)| &cell[i + 8..i + 9])
        .collect();
    assert_eq!(marks, ["a", "d", "s"], "{cell}");
    // A person's filiation is their own link to their parents.
    let people = list(&app, &cookie, "person").await;
    assert!(
        people.contains(r#"<span class="fil fil-d""#),
        "Piotr's is inferred"
    );
    // Nobody here has a confidence of their own: no column for it, and the
    // bulk bar spans the columns that are drawn.
    assert!(!people.contains("<th>Confidence</th>"), "an empty column");
    assert!(people.contains(r#"<th colspan="4">"#));
    // Pagination reads as a range of the whole.
    assert!(
        people.contains(r#"<span class="pager-range">1–6 of 6</span>"#),
        "the range"
    );
}

#[tokio::test]
async fn the_selection_exports_exactly_what_was_selected() {
    let (app, _s) = app();
    let cookie = admin_cookie(&app).await;
    let resp = post_form_as(
        &app,
        &cookie,
        "/admin/person/bulk",
        "action=export&id=x1%7C1&id=p2%7C1",
    )
    .await;
    assert_eq!(resp.status(), StatusCode::OK);
    assert!(resp
        .headers()
        .get(header::CONTENT_DISPOSITION)
        .and_then(|v| v.to_str().ok())
        .is_some_and(|v| v.contains("attachment") && v.contains("axgf-person-2.json")));
    let body: Value = serde_json::from_str(&body_string(resp).await).unwrap();
    let ids: Vec<&String> = body["persons"].as_object().unwrap().keys().collect();
    assert_eq!(ids, ["p2", "x1"]);
}

#[tokio::test]
async fn a_bulk_delete_asks_first_then_deletes_all_or_none() {
    let (app, _s) = app();
    let cookie = admin_cookie(&app).await;

    // Asked first, by name; nothing has happened yet.
    let ask = post_form_as(
        &app,
        &cookie,
        "/admin/person/bulk",
        "action=confirm&id=x1%7C1&id=x2%7C1",
    )
    .await;
    let ask = expect_status(ask, StatusCode::OK, "confirm").await;
    assert!(
        ask.contains("Delete 2 records?") && ask.contains("Loose One") && ask.contains("Loose Two")
    );
    assert!(ask.contains(r#"value="delete""#));
    assert!(list(&app, &cookie, "person").await.contains("Loose One"));

    // One of them still referred to: refused, and nothing is deleted.
    let refused = post_form_as(
        &app,
        &cookie,
        "/admin/person/bulk",
        "action=delete&id=x1%7C1&id=p1%7C1",
    )
    .await;
    let refused = body_string(refused).await;
    assert!(
        refused.contains("Nothing was deleted") && refused.contains("Anna Nowak"),
        "{refused}"
    );
    let people = list(&app, &cookie, "person").await;
    assert!(people.contains("Loose One") && people.contains("Anna Nowak"));

    // A stale version: refused before anything is tried.
    let stale = post_form_as(
        &app,
        &cookie,
        "/admin/person/bulk",
        "action=delete&id=x1%7C1&id=x2%7C7",
    )
    .await;
    assert_eq!(stale.status(), StatusCode::CONFLICT);
    assert!(list(&app, &cookie, "person").await.contains("Loose Two"));

    // Both free: both go, in one save.
    let done = post_form_as(
        &app,
        &cookie,
        "/admin/person/bulk",
        "action=delete&id=x1%7C1&id=x2%7C1",
    )
    .await;
    let done = body_string(done).await;
    assert!(done.contains("2 records deleted."), "{done}");
    let people = list(&app, &cookie, "person").await;
    assert!(!people.contains("Loose One") && !people.contains("Loose Two"));
    assert!(people.contains("Anna Nowak"));
}

#[tokio::test]
async fn the_bars_count_is_in_the_pages_own_plural() {
    // ui.js cannot run Fluent, so the page carries the count's forms; Polish
    // has four, chosen by n, n mod 10 and n mod 100.
    let (app, _s) = app();
    let cookie = format!("{}; axgf_lang=pl", admin_cookie(&app).await);
    let html = list(&app, &cookie, "person").await;
    let at = html.find(r#"data-forms=""#).unwrap() + 12;
    let raw = &html[at..at + html[at..].find('"').unwrap()];
    let forms: Value =
        serde_json::from_str(&raw.replace("&quot;", "\"").replace("&#x2f;", "/")).unwrap();
    let say = |n: usize| {
        let i = forms["index"][if n < 200 { n } else { 100 + n % 100 }]
            .as_u64()
            .unwrap() as usize;
        forms["forms"][i]
            .as_str()
            .unwrap()
            .replace("{n}", &n.to_string())
    };
    assert_eq!(say(1), "1 zaznaczona");
    assert_eq!(say(3), "3 zaznaczone");
    assert_eq!(say(5), "5 zaznaczonych");
    assert_eq!(say(22), "22 zaznaczone");
    assert_eq!(say(112), "112 zaznaczonych");
    assert_eq!(say(1024), "1024 zaznaczone");
}

#[test]
fn rows_are_ruled_by_hairlines_with_no_zebra_and_no_coloured_edge() {
    let css = include_str!("../static/app.css");
    assert!(
        !css.contains("nth-child(even)") && !css.contains("nth-child(odd)"),
        "no zebra"
    );
    assert!(
        css.contains(".admin-table tbody td { border-bottom: 1px solid var(--border-hairline); }")
    );
    let selected =
        ".admin-table tbody tr:has(> .sel input:checked) { background: var(--surface-hover); }";
    assert!(css.contains(selected), "a selected row tints the surface");
    let rule = &css[css.find(".admin-table tbody tr:has").unwrap()..];
    let rule = &rule[..rule.find('}').unwrap()];
    assert!(!rule.contains("border"), "and gains no edge: {rule}");
}

#[tokio::test]
async fn the_dashboard_draws_no_tile_for_a_kind_with_none() {
    // Six people and one family; no event, place or source.
    let (app, _s) = app();
    let cookie = admin_cookie(&app).await;
    let html = body_string(get_with_cookie(&app, "/admin", &cookie).await).await;
    assert!(html.contains(r#"<a class="stat" href="/admin/person""#));
    assert!(html.contains(r#"<a class="stat" href="/admin/family""#));
    for none in ["event", "place", "source"] {
        assert!(
            !html.contains(&format!(r#"class="stat" href="/admin/{none}""#)),
            "no 0 tile for {none}"
        );
    }
}
