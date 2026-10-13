//! In read mode an empty field does not exist (the minimalism pass).
//!
//! No label without a value, no dash, no "Date unknown", no "no score", no
//! "nothing recorded", no section that draws only its heading — on the
//! record page and on the panel beside the river, for every person.
//!
//! Run over the sample bundle in the ordinary suite, and over a real archive
//! with `AXGF_REAL_BUNDLE=/path/to/family.axgf cargo test --test read_mode --
//! --ignored`, which reads every person in it.

mod common;

use std::path::Path;

use common::*;
use scraper::{ElementRef, Html, Selector};

/// The sentences a read page used to put where a value was missing, in the
/// default language, straight from the catalogue.
fn placeholders() -> Vec<String> {
    [
        "date-unknown",
        "radar-no-score",
        "radar-from-none",
        "radar-empty",
        "record-no-file",
        "record-partner-not-recorded",
        "life-nothing-recorded",
        "record-no-evidence",
        "profile-empty",
        "record-dates-unrecorded",
        "union-status-unknown",
    ]
    .iter()
    .map(|k| axgf_cms::i18n::translate("en", k, None))
    .collect()
}

/// Every empty field on a read page, described.
fn empty_fields(html: &str) -> Vec<String> {
    let doc = Html::parse_document(html);
    let main = Selector::parse("main, .panel-inner").unwrap();
    let mut out = Vec::new();
    let text = |e: &ElementRef| e.text().collect::<String>().trim().to_string();
    let has_media = |e: &ElementRef| {
        e.select(&Selector::parse("img, svg, input, select, a, i, abbr").unwrap())
            .next()
            .is_some()
    };
    for scope in doc.select(&main) {
        for dd in scope.select(&Selector::parse("dd").unwrap()) {
            if text(&dd).is_empty() && !has_media(&dd) {
                let label = dd
                    .prev_siblings()
                    .filter_map(ElementRef::wrap)
                    .next()
                    .map(|dt| text(&dt))
                    .unwrap_or_default();
                out.push(format!("an empty value under {label:?}"));
            }
        }
        for td in scope.select(&Selector::parse("tbody td").unwrap()) {
            if text(&td).is_empty() && !has_media(&td) {
                out.push(format!(
                    "an empty cell: {}",
                    td.html().chars().take(120).collect::<String>()
                ));
            }
        }
        for sec in scope.select(&Selector::parse(".rec-section").unwrap()) {
            let heading: String = sec
                .select(&Selector::parse(".sec-head, h2").unwrap())
                .map(|h| text(&h))
                .collect();
            let body = text(&sec);
            if body.len() <= heading.len() && !has_media(&sec) {
                out.push(format!("a section with only its heading: {heading:?}"));
            }
        }
        let all = text(&scope);
        for p in placeholders() {
            if all.contains(&p) {
                out.push(format!("the placeholder {p:?}"));
            }
        }
        for el in scope.select(&Selector::parse("dd, td, span, p").unwrap()) {
            // A separator between two values is not a value.
            if el
                .value()
                .attr("class")
                .is_some_and(|c| c.split_whitespace().any(|c| c == "sep"))
            {
                continue;
            }
            if matches!(text(&el).as_str(), "—" | "–" | "-") {
                out.push("a dash standing for nothing".to_string());
            }
        }
    }
    out.sort();
    out.dedup();
    out
}

async fn check_every_person(app: &axum::Router, bundle: &Path) -> (usize, Vec<String>) {
    let bytes = std::fs::read(bundle).expect("read the bundle");
    let env = axgf_rs::import_bundle(&bytes);
    let flat = env.data.get("bundle").cloned().unwrap_or(env.data);
    let ids: Vec<String> = flat["persons"]
        .as_object()
        .map(|m| m.keys().cloned().collect())
        .unwrap_or_default();
    let cookie = admin_cookie(app).await;
    let mut found = Vec::new();
    for id in &ids {
        for uri in [format!("/person/{id}"), format!("/tree/panel/{id}?river=1")] {
            let html = body_string(get_with_cookie(app, &uri, &cookie).await).await;
            for f in empty_fields(&html) {
                found.push(format!("{uri}: {f}"));
            }
        }
    }
    (ids.len(), found)
}

#[tokio::test]
async fn no_empty_field_reaches_a_read_page_in_the_sample() {
    let src = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/deploy/sample.axgf"));
    let (app, _s) = app_with_bundle("read-mode-sample", src);
    let (people, found) = check_every_person(&app, src).await;
    assert!(people > 5, "the sample has people: {people}");
    assert!(
        found.is_empty(),
        "{} empty field(s):\n{}",
        found.len(),
        found.join("\n")
    );
}

#[tokio::test]
#[ignore = "reads a real archive: AXGF_REAL_BUNDLE=/path/to/family.axgf"]
async fn no_empty_field_reaches_a_read_page_in_the_real_bundle() {
    let path = std::env::var("AXGF_REAL_BUNDLE").expect("AXGF_REAL_BUNDLE");
    let src = Path::new(&path);
    let (app, _s) = app_with_bundle("read-mode-real", src);
    let (people, found) = check_every_person(&app, src).await;
    println!("{people} people read, {} empty field(s)", found.len());
    let mut kinds: std::collections::BTreeMap<String, usize> = Default::default();
    for f in &found {
        *kinds
            .entry(f.split_once(": ").map_or(f.clone(), |x| x.1.to_string()))
            .or_default() += 1;
    }
    for (k, n) in &kinds {
        println!("{n:>6}  {k}");
    }
    assert!(
        found.is_empty(),
        "{} empty field(s) across {people} people",
        found.len()
    );
}

#[test]
fn the_check_finds_what_it_is_for() {
    let page = r#"<main><section class="rec-section"><div class="sec-head"><h2>Life</h2></div></section>
        <dl><dt>Born</dt><dd></dd></dl><table><tbody><tr><td>—</td><td></td></tr></tbody></table>
        <p>Date unknown</p></main>"#;
    let found = empty_fields(page);
    for want in [
        "only its heading",
        "empty value",
        "empty cell",
        "dash",
        "Date unknown",
    ] {
        assert!(found.iter().any(|f| f.contains(want)), "{want}: {found:?}");
    }
    assert!(empty_fields("<main><dl><dt>Born</dt><dd>1888</dd></dl></main>").is_empty());
}
