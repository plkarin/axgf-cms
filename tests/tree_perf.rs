//! Performance and correctness of `/tree` at the operator's real scale.
//!
//! The bundle under test is set with `AXGF_CMS_BENCH_BUNDLE`. Without it the
//! test still runs, on a synthetic bundle of the same size, so the budget is
//! enforced in CI where the real file is not available.

mod common;

use std::time::Instant;

use axum::http::StatusCode;
use common::*;
use serde_json::json;

/// Build a synthetic bundle of roughly the operator's shape: 767 persons in
/// ~295 families, spread over several generations.
fn synthetic_bundle_path(tag: &str) -> std::path::PathBuf {
    let dir = scratch(tag);
    let path = dir.join("synthetic.axgf");

    let mut persons = serde_json::Map::new();
    let mut families = serde_json::Map::new();

    let n = 767usize;
    let ids: Vec<String> = (0..n)
        .map(|i| format!("{:08}-0000-4000-8000-000000000000", i))
        .collect();
    for (i, id) in ids.iter().enumerate() {
        persons.insert(
            id.clone(),
            json!({
                "id": id, "type": "person", "axgf_version": "1.0",
                "identity": {
                    "name": {"display": format!("Person {i}")},
                    "gender": {"value": if i % 2 == 0 { "M" } else { "F" }},
                    "is_living": i % 9 == 0
                },
                "birth": {"date": {"value": format!("{}", 1700 + (i % 250)), "precision": "year"},
                          "confidence": 0.3 + ((i % 7) as f64) / 10.0},
                "death": {"date": {"value": format!("{}", 1760 + (i % 250)), "precision": "year"},
                          "confidence": 0.8}
            }),
        );
    }

    // A branching tree: person i parents persons 2i+1 and 2i+2.
    let mut fam = 0usize;
    let mut i = 0usize;
    while fam < 295 && 2 * i + 2 < n {
        let a = &ids[2 * i + 1];
        let b = &ids[2 * i + 2];
        let fid = format!("{:08}-1111-4000-8000-000000000000", fam);
        families.insert(
            fid.clone(),
            json!({
                "id": fid, "type": "family", "axgf_version": "1.0",
                "union": {"type": "marriage",
                          "persons": [{"person_id": ids[i], "role": "spouse"}],
                          "confidence": 0.9},
                "children": [
                    {"person_id": a, "confidence": 0.95},
                    {"person_id": b, "confidence": 0.42}
                ]
            }),
        );
        fam += 1;
        i += 1;
    }

    let flat = json!({
        "manifest": {"axgf": "1.0", "created_at": "2026-01-01T00:00:00Z",
                     "updated_at": "2026-01-01T00:00:00Z"},
        "persons": persons, "families": families,
        "events": {}, "links": {}, "occupations": {},
        "sources": {}, "places": {}, "documents": {}
    });

    let bytes = axgf_cms::state::export_to_bytes(&flat.to_string()).expect("export synthetic");
    std::fs::write(&path, bytes).expect("write synthetic bundle");
    path
}

#[tokio::test]
async fn tree_renders_a_full_size_bundle_well_under_a_second() {
    let bundle = match std::env::var("AXGF_CMS_BENCH_BUNDLE") {
        Ok(p) if std::path::Path::new(&p).exists() => std::path::PathBuf::from(p),
        _ => synthetic_bundle_path("perf-src"),
    };

    let (app, _p) = app_with_bundle("perf", &bundle);

    // Both views are measured: the focused one is what a visitor lands on, and
    // the full one is what `?all=1` still has to survive.
    for (label, uri) in [
        ("focused (default)", "/tree"),
        ("full (?all=1)", "/tree?all=1"),
    ] {
        // Warm once so the measurement is steady-state, not first-touch.
        let _ = get(&app, uri).await;

        let mut timings = Vec::new();
        let mut bytes = 0usize;
        let mut cards = 0usize;
        for _ in 0..5 {
            let t = Instant::now();
            let resp = get(&app, uri).await;
            assert_eq!(resp.status(), StatusCode::OK);
            let body = body_string(resp).await;
            timings.push(t.elapsed());
            bytes = body.len();
            cards = body.matches("class=\"tcard-hit\"").count();
        }
        timings.sort();
        let median = timings[timings.len() / 2];
        let worst = *timings.last().unwrap();

        eprintln!(
            "/tree {label:18} over {}: median {:?}, worst {:?}, {} KB, {cards} cards",
            bundle.display(),
            median,
            worst,
            bytes / 1024
        );

        assert!(
            worst < std::time::Duration::from_millis(1000),
            "{label} must render well under a second; worst was {worst:?}"
        );
    }
}

#[tokio::test]
async fn the_full_view_still_places_every_person() {
    let bundle = match std::env::var("AXGF_CMS_BENCH_BUNDLE") {
        Ok(p) if std::path::Path::new(&p).exists() => std::path::PathBuf::from(p),
        _ => synthetic_bundle_path("place-src"),
    };
    let (app, _p) = app_with_bundle("place", &bundle);
    let body = expect_status(
        get(&app, "/tree?all=1").await,
        StatusCode::OK,
        "GET /tree?all=1",
    )
    .await;

    // `?all=1` may not silently drop anyone, placed or not.
    let health = body_string(get(&app, "/health").await).await;
    let h: serde_json::Value = serde_json::from_str(&health).unwrap();
    let people = h["entities"]["persons"].as_u64().unwrap() as usize;

    let cards = body.matches("class=\"tcard-hit\"").count();
    assert_eq!(
        cards, people,
        "every person needs a card in the full view; {people} people but {cards} cards"
    );
}

#[tokio::test]
async fn the_default_view_is_a_small_legible_subtree() {
    // The whole point of the focused default: a visitor must land on something
    // they can read, not on an 18,000px canvas.
    let bundle = match std::env::var("AXGF_CMS_BENCH_BUNDLE") {
        Ok(p) if std::path::Path::new(&p).exists() => std::path::PathBuf::from(p),
        _ => synthetic_bundle_path("focus-src"),
    };
    let (app, _p) = app_with_bundle("focus", &bundle);
    let body = expect_status(get(&app, "/tree").await, StatusCode::OK, "GET /tree").await;

    let cards = body.matches("class=\"tcard-hit\"").count();
    assert!(cards > 0, "the default view must draw somebody");
    assert!(
        cards <= 120,
        "the default view should be a few dozen people, drew {cards}"
    );

    // And its canvas must fit a reasonable number of screens.
    let width: f64 = body
        .split("class=\"tree-canvas\"")
        .nth(1)
        .and_then(|s| s.split("width:").nth(1))
        .and_then(|s| s.split("px").next())
        .and_then(|s| s.trim().parse().ok())
        .expect("a canvas width");
    assert!(
        width < 6000.0,
        "the focused canvas should be scrollable, was {width}px"
    );
    eprintln!("focused canvas: {width}px, {cards} cards");
}
