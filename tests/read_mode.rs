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

/// Every message of the English catalogue, as the pieces of fixed text a
/// reader can see: the text between its placeables, and each variant's text.
/// `{ $n -> [one] a { $n } b *[other] c }` gives "a", "b" and "c".
fn catalogue() -> Vec<(String, Vec<String>)> {
    #[derive(PartialEq)]
    enum Frame {
        Placeable,
        Select,
    }
    let src = include_str!("../locales/en.ftl");
    let mut messages: Vec<(String, String)> = Vec::new();
    for line in src.lines() {
        if line.starts_with(|c: char| c.is_ascii_lowercase()) {
            if let Some((key, value)) = line.split_once(" =") {
                messages.push((key.trim().to_string(), value.to_string()));
                continue;
            }
        }
        if line.starts_with(char::is_whitespace) && !line.trim().is_empty() {
            if let Some(m) = messages.last_mut() {
                m.1.push('\n');
                m.1.push_str(line);
            }
        } else if !line.trim().is_empty() {
            // A comment, a section or a term ends the message before it.
            messages.push((String::new(), String::new()));
        }
    }
    let mut out = Vec::new();
    for (key, value) in messages.into_iter().filter(|m| !m.0.is_empty()) {
        let (mut stack, mut cur, mut frags) = (Vec::<Frame>::new(), String::new(), Vec::new());
        let chars: Vec<char> = value.chars().collect();
        let mut i = 0;
        let flush = |cur: &mut String, frags: &mut Vec<String>| {
            frags.push(std::mem::take(cur));
        };
        while i < chars.len() {
            let c = chars[i];
            let text = stack.last().is_none_or(|f| *f == Frame::Select);
            if text {
                match c {
                    '{' => {
                        flush(&mut cur, &mut frags);
                        stack.push(Frame::Placeable);
                    }
                    '}' => {
                        flush(&mut cur, &mut frags);
                        stack.pop();
                    }
                    '*' | '['
                        if !stack.is_empty() && (c == '[' || chars.get(i + 1) == Some(&'[')) =>
                    {
                        flush(&mut cur, &mut frags);
                        while i < chars.len() && chars[i] != ']' {
                            i += 1;
                        }
                    }
                    '\n' => flush(&mut cur, &mut frags),
                    _ => cur.push(c),
                }
            } else {
                match c {
                    '-' if chars.get(i + 1) == Some(&'>') => {
                        *stack.last_mut().unwrap() = Frame::Select;
                        i += 1;
                    }
                    '{' => stack.push(Frame::Placeable),
                    '}' => {
                        stack.pop();
                    }
                    _ => {}
                }
            }
            i += 1;
        }
        flush(&mut cur, &mut frags);
        for f in frags {
            let f = f.split_whitespace().collect::<Vec<_>>().join(" ");
            if f.chars().count() >= 4 && f.chars().any(char::is_alphabetic) {
                out.push((f, key.clone()));
            }
        }
    }
    // One piece, every message that says it; longest first, since a short
    // piece inside a longer one belongs to the longer one.
    let mut grouped: std::collections::BTreeMap<String, Vec<String>> = Default::default();
    for (f, k) in out {
        grouped.entry(f).or_default().push(k);
    }
    let mut out: Vec<(String, Vec<String>)> = grouped.into_iter().collect();
    out.sort_by(|a, b| b.0.len().cmp(&a.0.len()).then(a.0.cmp(&b.0)));
    out
}

/// The catalogue messages a read page may show. Every one of them is a
/// heading, a label, a value from a closed vocabulary or a sentence about
/// what IS recorded. Anything else from the catalogue on a read page fails —
/// a new way of saying "nothing here" included, whatever its wording. Adding
/// a key here is a decision: it must name something that is there.
///
/// Drawn up from every person of the sample and of the operator's archive
/// (2026-10-13), read key by key. Left out on purpose, and so failing:
/// `gender-U`/`gender-unrecorded`, `union-type-unknown`,
/// `union-status-unknown`, `record-dates-unrecorded`, `date-unknown`,
/// `radar-no-score`, `radar-from-none`, `profile-empty` — every way the
/// catalogue has of saying "nothing here".
const ALLOWED: &[&str] = &[
    "accounts-status",
    "admin-edit",
    "admin-from",
    "confidence-certain",
    "confidence-high",
    "confidence-low",
    "confidence-medium",
    "convert-download-named",
    "convert-download-title",
    "date-after",
    "date-before",
    "date-between",
    "date-circa",
    "date-day-month-year",
    "date-month-year",
    "date-preserved",
    "date-qualifier-circa",
    "diff-section-children",
    "diff-section-file",
    "diff-section-visibility",
    "document-status-present",
    "document-type-baptism_record",
    "document-type-birth_certificate",
    "document-type-letter",
    "document-type-other",
    "document-type-photo",
    "documents-editor-title",
    "event-category-marriage",
    "family-children",
    "family-label-couple",
    "family-label-half",
    "family-parents",
    "field-confidence",
    "field-notes",
    "field-occupation-from",
    "field-person-bio",
    "field-person-gender",
    "field-source-reliability",
    "gender-F",
    "gender-M",
    "identity-display",
    "kind-document",
    "kind-document-plural",
    "link-category-educational",
    "link-category-legal",
    "link-category-professional",
    "link-category-spiritual",
    "links-from",
    "list-kind-label",
    "name-part-alias",
    "name-part-family_name",
    "name-part-given_name",
    "name-part-nickname",
    "name-type-alias",
    "name-type-married",
    "name-type-nickname",
    "name-type-primary",
    "note-birth-imprecise",
    "note-death-imprecise",
    "note-links",
    "note-names",
    "note-occupations",
    "occupations-editor-title",
    "occupations-from",
    "pa-education-diplomas",
    "pa-education-institutions",
    "pa-education-level",
    "pa-morphology-bmi",
    "pa-morphology-body-hair",
    "pa-morphology-build",
    "pa-morphology-dentition",
    "pa-morphology-ear-shape",
    "pa-morphology-eye-colour",
    "pa-morphology-eye-shape",
    "pa-morphology-eye-spacing",
    "pa-morphology-face-shape",
    "pa-morphology-facial-hair",
    "pa-morphology-freckles",
    "pa-morphology-gait",
    "pa-morphology-hair-colour",
    "pa-morphology-hair-texture",
    "pa-morphology-hairline",
    "pa-morphology-height",
    "pa-morphology-lip-shape",
    "pa-morphology-malocclusion",
    "pa-morphology-nose-shape",
    "pa-morphology-posture",
    "pa-morphology-scars",
    "pa-morphology-skin-tone",
    "pa-morphology-skin-undertone",
    "pa-morphology-weight",
    "pa-personality-big-five",
    "pa-personality-decision-style",
    "pa-personality-dietary-habits",
    "pa-personality-introversion-extraversion",
    "pa-personality-mbti",
    "pa-personality-sports",
    "pa-personality-stress-tolerance",
    "person-age-at-death",
    "person-age-now",
    "person-born-in",
    "person-children-count",
    "person-died-in",
    "person-generations-below",
    // The record's tabs (v0.4.1): each names a part of the record that is
    // there, and is drawn only when it is.
    "person-tab-life",
    "person-tab-media",
    "person-tab-profile",
    "person-tab-record",
    "person-visibility-inline",
    "pf-agreeableness",
    "pf-body-region",
    "pf-conscientiousness",
    "pf-diet",
    "pf-extraversion",
    "pf-institution",
    "pf-isced-level",
    "pf-level",
    "pf-neuroticism",
    "pf-openness",
    "pf-score",
    "pg-education",
    "pg-morphology",
    "pg-personality",
    "phys-col-confidence",
    "phys-col-source",
    "phys-field-build",
    "phys-field-eye-colour",
    "phys-field-health-notes",
    "phys-field-height-cm",
    "phys-field-weight-kg",
    "place-history-from",
    "place-name-value",
    "place-type",
    "place-type-village",
    "profile-edit-group",
    "profile-from",
    "profile-summary-link",
    "profile-until",
    "pu-kg-m2",
    "pv-body-hair-none",
    "pv-body-hair-sparse",
    "pv-body-region-right-leg",
    "pv-build-average",
    "pv-build-sturdy",
    "pv-decision-style-rational",
    "pv-dentition-permanent-partial-loss",
    "pv-diet-omnivore",
    "pv-ear-shape-close-set",
    "pv-eye-colour-brown",
    "pv-eye-colour-green",
    "pv-eye-shape-downturned",
    "pv-eye-spacing-average",
    "pv-face-shape-oval",
    "pv-facial-hair-goatee",
    "pv-facial-hair-none",
    "pv-freckles-none",
    "pv-gait-average",
    "pv-hair-colour-brown",
    "pv-hair-texture-straight",
    "pv-hairline-bald",
    "pv-hairline-straight",
    "pv-introversion-extraversion-introverted",
    "pv-isced-level-isced-7",
    "pv-lip-shape-downturned",
    "pv-lip-shape-thin",
    "pv-malocclusion-normal",
    "pv-mole-shape-oval",
    "pv-nose-shape-straight",
    "pv-optical-correction-none",
    "pv-posture-ideal",
    "pv-register-type-marriage",
    "pv-sex-at-birth-female",
    "pv-sex-at-birth-male",
    "pv-skin-tone-type-v",
    "pv-skin-undertone-warm",
    "pv-sport-level-amateur-competitive",
    "pv-sport-level-recreational",
    "pv-sport-level-semi-professional",
    "pv-stress-tolerance-very-high",
    "pv-vocal-timbre-thin",
    "pv-vocal-timbre-warm",
    "quality-download",
    "quality-group-places",
    "radar-axis-agreeableness",
    "radar-axis-build",
    "radar-axis-cognition",
    "radar-axis-conscientiousness",
    "radar-axis-dentition",
    "radar-axis-extraversion",
    "radar-axis-gait",
    "radar-axis-lean-mass",
    "radar-axis-openness",
    "radar-axis-posture",
    "radar-axis-stability",
    "radar-axis-stature",
    "radar-col-axis",
    "radar-col-from",
    "radar-col-score",
    "radar-folded-open",
    "radar-folded-why",
    "radar-mind",
    "radar-physique",
    "radar-section",
    "record-borders-moved",
    "record-born",
    "record-cause",
    "record-children",
    "record-confidence",
    "record-deceased",
    "record-died",
    "record-display-name",
    "record-documents",
    "record-download",
    "record-edit",
    "record-family",
    "record-file",
    "record-gender",
    "record-identity",
    "record-life-events",
    "record-life-status",
    "record-link-incoming",
    "record-link-outgoing",
    "record-living-yes",
    "record-name",
    "record-name-type",
    "record-note-biography",
    "record-note-birth-date-as-recorded",
    "record-note-death-date-as-recorded",
    "record-note-event-date-as-recorded",
    "record-notes",
    "record-notes-title",
    "record-occupation-from",
    "record-occupation-until",
    "record-occupations",
    "record-open-full-page",
    "record-other-relationships",
    "record-parents",
    "record-period-from",
    "record-period-until",
    "record-photographs",
    "record-place-married",
    "record-place-married-to",
    "record-place-worked-as",
    "record-places",
    "record-presumed-deceased",
    "record-see-in-river",
    "record-siblings",
    "record-size",
    "record-source",
    "record-source-reliability",
    "record-source-supports",
    "record-source-use-union-with",
    "record-source-use-working-as",
    "record-sources-documents",
    "record-status",
    "record-type",
    "record-union-duplicate",
    "record-union-duplicate-detail",
    "record-union-from",
    "record-union-until",
    "record-verbatim-note",
    "record-visibility",
    "river-tab-sources",
    "reliability-authored",
    "reliability-oral",
    "reliability-primary",
    "role-spouse",
    "role-spouse_1",
    "role-spouse_2",
    "source-status-unverified",
    "source-status-verified",
    "source-type-baptism_record",
    "source-type-birth_certificate",
    "source-type-letter",
    "source-type-oral_tradition",
    "source-type-photograph",
    "source-type-published_genealogy",
    "union-role-spouse",
    "union-status-ended_by_divorce",
    "union-type-marriage",
    "visibility-members",
    "visibility-public",
];

/// The visible text of each node of a read page: what a reader sees, or a
/// screen reader says. Attribute text is not included.
fn text_nodes(html: &str) -> Vec<String> {
    let doc = Html::parse_document(html);
    let scope = Selector::parse("main, .panel-inner").unwrap();
    let mut out = Vec::new();
    for s in doc.select(&scope) {
        for node in s.descendants() {
            if let Some(t) = node.value().as_text() {
                let inside_code = node
                    .ancestors()
                    .filter_map(ElementRef::wrap)
                    .any(|e| matches!(e.value().name(), "script" | "style" | "template"));
                let t = t.split_whitespace().collect::<Vec<_>>().join(" ");
                if !inside_code && !t.is_empty() {
                    out.push(t);
                }
            }
        }
    }
    out
}

/// Which catalogue messages a node's text holds, longest pieces first, each
/// match masked so its own short pieces are not counted again.
fn messages_in<'c>(
    text: &str,
    data: &[String],
    cat: &'c [(String, Vec<String>)],
) -> Vec<&'c [String]> {
    // What the record itself says is the record's, not the catalogue's.
    let mut t = text.to_string();
    for d in data {
        if d.len() <= t.len() && t.contains(d.as_str()) {
            t = t.replace(d.as_str(), "\u{0}");
        }
    }
    let mut found = Vec::new();
    for (frag, keys) in cat {
        if frag.len() <= t.len() && t.contains(frag.as_str()) {
            // A whole word or phrase: "farm" is not inside "farmaceuta".
            let word = |c: Option<char>| c.is_some_and(char::is_alphanumeric);
            let mut at = 0;
            let mut hit = false;
            while let Some(i) = t[at..].find(frag.as_str()).map(|i| i + at) {
                let end = i + frag.len();
                if !word(t[..i].chars().next_back()) && !word(t[end..].chars().next()) {
                    t.replace_range(i..end, "\u{0}");
                    hit = true;
                    at = i + 1;
                } else {
                    at = end;
                }
            }
            if hit {
                found.push(keys.as_slice());
            }
        }
    }
    found
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

/// What one sweep found: empty fields, and catalogue text no read page may
/// show, each with where; and every allowed message seen, with a count.
#[derive(Default)]
struct Sweep {
    people: usize,
    empty: Vec<String>,
    unexpected: Vec<String>,
    seen: std::collections::BTreeMap<String, usize>,
}

async fn check_every_person(app: &axum::Router, bundle: &Path) -> Sweep {
    let bytes = std::fs::read(bundle).expect("read the bundle");
    let env = axgf_rs::import_bundle(&bytes);
    let flat = env.data.get("bundle").cloned().unwrap_or(env.data);
    let ids: Vec<String> = flat["persons"]
        .as_object()
        .map(|m| m.keys().cloned().collect())
        .unwrap_or_default();
    let cookie = format!("{}; axgf_lang=en", admin_cookie(app).await);
    let cat = catalogue();
    let data = record_text(&flat);
    let mut sweep = Sweep {
        people: ids.len(),
        ..Default::default()
    };
    // The same label recurs on every page: each distinct text is read once.
    let mut read: std::collections::HashMap<String, Vec<&[String]>> = Default::default();
    for id in &ids {
        // The record page and the panel beside the river, each on every tab
        // it offers (v0.4.1): a tab holds part of the record, and the rule
        // holds inside each.
        let mut uris = Vec::new();
        for first in [format!("/person/{id}"), format!("/tree/panel/{id}?river=1")] {
            let html = body_string(get_with_cookie(app, &first, &cookie).await).await;
            let tabs: Vec<String> = tab_hrefs(&html)
                .into_iter()
                .filter_map(|h| h.split("tab=").nth(1).map(str::to_string))
                .collect();
            for t in tabs {
                let sep = if first.contains('?') { '&' } else { '?' };
                uris.push(format!("{first}{sep}tab={t}"));
            }
            uris.push(first);
        }
        for uri in uris {
            let html = body_string(get_with_cookie(app, &uri, &cookie).await).await;
            for f in empty_fields(&html) {
                sweep.empty.push(format!("{uri}: {f}"));
            }
            for node in text_nodes(&html) {
                let groups = read
                    .entry(node.clone())
                    .or_insert_with(|| messages_in(&node, &data, &cat));
                for keys in groups.iter() {
                    if let Some(k) = keys.iter().find(|k| ALLOWED.contains(&k.as_str())) {
                        *sweep.seen.entry(k.clone()).or_default() += 1;
                    } else {
                        sweep
                            .unexpected
                            .push(format!("{uri}: {} in {node:?}", keys.join(" / ")));
                    }
                }
            }
        }
    }
    sweep
}

/// Every text the archive holds — names, places, notes, titles — longest
/// first. A code (`marriage`, `unknown`) is not text: it reaches the page
/// through the catalogue, and must be checked there.
fn record_text(flat: &serde_json::Value) -> Vec<String> {
    fn walk(v: &serde_json::Value, out: &mut std::collections::BTreeSet<String>) {
        match v {
            serde_json::Value::String(s) => {
                let code = s
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c == '_' || c == '-');
                if !code && s.trim().chars().count() >= 3 {
                    out.insert(s.split_whitespace().collect::<Vec<_>>().join(" "));
                }
            }
            serde_json::Value::Array(a) => a.iter().for_each(|x| walk(x, out)),
            serde_json::Value::Object(o) => o.values().for_each(|x| walk(x, out)),
            _ => {}
        }
    }
    let mut set = Default::default();
    walk(flat, &mut set);
    let mut out: Vec<String> = set.into_iter().collect();
    out.sort_by_key(|s| std::cmp::Reverse(s.len()));
    out
}

fn report(sweep: &Sweep) -> String {
    let mut kinds: std::collections::BTreeMap<String, usize> = Default::default();
    for f in sweep.empty.iter().chain(&sweep.unexpected) {
        *kinds
            .entry(f.split_once(": ").map_or(f.clone(), |x| x.1.to_string()))
            .or_default() += 1;
    }
    let mut out = format!(
        "{} people read: {} empty field(s), {} catalogue text(s) not allowed in read mode\n",
        sweep.people,
        sweep.empty.len(),
        sweep.unexpected.len()
    );
    for (k, n) in &kinds {
        out.push_str(&format!("{n:>6}  {k}\n"));
    }
    out
}

#[tokio::test]
async fn a_read_page_shows_only_what_is_recorded_in_the_sample() {
    let src = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/deploy/sample.axgf"));
    let (app, _s) = app_with_bundle("read-mode-sample", src);
    let sweep = check_every_person(&app, src).await;
    assert!(sweep.people > 5, "the sample has people: {}", sweep.people);
    assert!(
        sweep.empty.is_empty() && sweep.unexpected.is_empty(),
        "{}",
        report(&sweep)
    );
}

#[tokio::test]
#[ignore = "reads a real archive: AXGF_REAL_BUNDLE=/path/to/family.axgf"]
async fn a_read_page_shows_only_what_is_recorded_in_the_real_bundle() {
    let path = std::env::var("AXGF_REAL_BUNDLE").expect("AXGF_REAL_BUNDLE");
    let src = Path::new(&path);
    let (app, _s) = app_with_bundle("read-mode-real", src);
    let sweep = check_every_person(&app, src).await;
    println!("{}", report(&sweep));
    if std::env::var("AXGF_SURVEY").is_ok() {
        for (k, n) in &sweep.seen {
            println!("SEEN {n:>6}  {k}");
        }
    }
    assert!(sweep.empty.is_empty() && sweep.unexpected.is_empty());
}

#[test]
fn the_check_finds_what_it_is_for() {
    let page = r#"<main><section class="rec-section"><div class="sec-head"><h2>Life</h2></div></section>
        <dl><dt>Born</dt><dd></dd></dl><table><tbody><tr><td>—</td><td></td></tr></tbody></table></main>"#;
    let found = empty_fields(page);
    for want in ["only its heading", "empty value", "empty cell", "dash"] {
        assert!(found.iter().any(|f| f.contains(want)), "{want}: {found:?}");
    }
    assert!(empty_fields("<main><dl><dt>Born</dt><dd>1888</dd></dl></main>").is_empty());
}

#[test]
fn the_catalogue_is_read_variant_by_variant() {
    let cat = catalogue();
    let keys_of = |f: &str| {
        cat.iter()
            .find(|(frag, _)| frag == f)
            .map(|(_, k)| k.clone())
            .unwrap_or_default()
    };
    // Plain text, the text around a placeable, and each plural's variants.
    assert!(keys_of("Sign in").contains(&"nav-sign-in".to_string()));
    assert!(keys_of("not recorded").contains(&"union-status-unknown".to_string()));
    assert!(keys_of("dates unrecorded").contains(&"record-dates-unrecorded".to_string()));
    // A phrase nobody listed is still caught: what is not allowed fails,
    // whatever its wording.
    let found = messages_in("dates unrecorded", &[], &cat);
    assert!(!found.is_empty());
    assert!(found
        .iter()
        .all(|keys| keys.iter().all(|k| !ALLOWED.contains(&k.as_str()))));
}
