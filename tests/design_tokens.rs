//! No colour is named outside the design tokens (Study 06, "The Furniture").
//!
//! Every page is rendered through the real router — the public pages signed
//! out, the administration signed in — and its HTML, and every stylesheet and
//! script it loads, is scanned for hex colour literals. `static/css/tokens.css`
//! is where colours are named, so it is not scanned. One exemption, named
//! here: **the river's water ramp in `src/river.rs`** (`river::on_water_ramp`)
//! — the colours `river::colour_at` gives the water for a year, and the
//! neutral water of a river without an era. It is a scale of years drawn on
//! the water, not furniture.
//!
//! In HTML only tags and `<style>` blocks are read, not text: a place with no
//! name is shown by its id, and `#9f8e7d6c` there is a name, not a colour.
//!
//! The site is converted page by page (/ → /person → /tree → /about →
//! import → admin). Until it is done this is a ratchet: no page may go above
//! the count it started with ([`START`]), and a page in [`CONVERTED`] must be
//! at zero. When every page is converted, [`START`] goes and zero is the rule.

mod common;

use std::collections::{BTreeMap, BTreeSet};

use axum::http::StatusCode;
use common::*;
use serde_json::{json, Value};

/// Hex literals per page when the tokens arrived (2026-10-09, before any page
/// was converted): HTML + stylesheets + scripts.
const START: &[(&str, usize)] = &[
    ("/", 325),
    ("/ signed in", 342),
    ("/person/:id", 261),
    ("/tree", 261),
    ("/about", 261),
    ("import", 261),
    ("admin", 261),
    ("admin list", 261),
    ("admin edit", 261),
    ("admin users", 261),
    ("404", 261),
];

/// Pages converted to the tokens: no hex literal may remain.
const CONVERTED: &[&str] = &[];

const P: &str = "p03";

fn person(id: &str, given: &str, surname: &str, born: i64) -> Value {
    json!({"id": id, "type": "person", "axgf_version": "1.1", "version_num": 1,
        "identity": {"name": {"display": format!("{given} {surname}"),
            "components": [{"type": "given_name", "value": given},
                           {"type": "family_name", "value": surname}]},
            "gender": {"value": "U"}, "is_living": false, "visibility": "public"},
        "birth": {"date": {"value": born.to_string(), "precision": "year"}}})
}

fn family(id: &str, parents: &[&str], kids: &[&str]) -> Value {
    json!({"id": id, "type": "family", "axgf_version": "1.1", "version_num": 1,
        "union": {"type": "marriage",
            "persons": parents.iter().map(|p| json!({"person_id": p, "role": "spouse"})).collect::<Vec<_>>()},
        "children": kids.iter().map(|k| json!({"person_id": k, "confidence": 0.9})).collect::<Vec<_>>()})
}

fn app() -> (axum::Router, Scratch) {
    let persons: serde_json::Map<String, Value> = [
        person("p01", "Jan", "Nowak", 1800),
        person("p02", "Anna", "Lis", 1806),
        person("p03", "Antoni", "Nowak", 1830),
        person("p04", "Zofia", "Mazur", 1834),
        person("p05", "Józef", "Nowak", 1858),
    ]
    .into_iter()
    .map(|p| (p["id"].as_str().unwrap().to_string(), p))
    .collect();
    let families: serde_json::Map<String, Value> = [
        family("f1", &["p01", "p02"], &["p03"]),
        family("f2", &["p03", "p04"], &["p05"]),
    ]
    .into_iter()
    .map(|f| (f["id"].as_str().unwrap().to_string(), f))
    .collect();
    let flat = json!({
        "manifest": {"axgf": "1.1", "created_at": "2026-10-09T00:00:00Z"},
        "persons": persons, "families": families,
        "events": {}, "places": {}, "occupations": {}, "links": {},
        "sources": {}, "documents": {}
    });
    let src = scratch("tokens-src");
    let path = src.join("tokens.axgf");
    std::fs::write(
        &path,
        axgf_cms::state::export_to_bytes(&flat.to_string()).expect("export"),
    )
    .expect("write");
    let out = app_with_bundle("tokens", &path);
    drop(src);
    out
}

/// The pages, in conversion order: (name, path, signed in).
fn pages() -> Vec<(&'static str, String, bool)> {
    vec![
        ("/", "/".into(), false),
        ("/ signed in", "/".into(), true),
        ("/person/:id", format!("/person/{P}"), false),
        ("/tree", "/tree".into(), false),
        ("/about", "/about".into(), false),
        ("import", "/convert".into(), false),
        ("admin", "/admin".into(), true),
        ("admin list", "/admin/person".into(), true),
        ("admin edit", format!("/admin/person/{P}/edit"), true),
        ("admin users", "/admin/users".into(), true),
        ("404", "/no-such-page".into(), false),
    ]
}

/// `#` and 3, 4, 6 or 8 hex digits, not part of a longer word or an entity.
fn hexes(text: &str) -> Vec<String> {
    let b = text.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'#' && (i == 0 || !(b[i - 1].is_ascii_alphanumeric() || b[i - 1] == b'&')) {
            let mut j = i + 1;
            while j < b.len() && b[j].is_ascii_hexdigit() {
                j += 1;
            }
            let n = j - i - 1;
            let word_ends =
                j == b.len() || !(b[j].is_ascii_alphanumeric() || b[j] == b'_' || b[j] == b'-');
            if matches!(n, 3 | 4 | 6 | 8) && word_ends {
                out.push(text[i..j].to_string());
            }
            i = j;
        } else {
            i += 1;
        }
    }
    out
}

/// The parts of an HTML page that can carry a colour: its tags (attributes,
/// inline styles, SVG) and its `<style>` blocks — not its text.
fn markup(html: &str) -> String {
    let mut out = String::new();
    let mut rest = html;
    while let Some(lt) = rest.find('<') {
        rest = &rest[lt..];
        let gt = rest.find('>').map_or(rest.len(), |g| g + 1);
        let tag = &rest[..gt];
        out.push_str(tag);
        rest = &rest[gt..];
        if tag.starts_with("<style") {
            let end = rest.find("</style>").unwrap_or(rest.len());
            out.push_str(&rest[..end]);
            rest = &rest[end..];
        }
    }
    out
}

/// `/static/…` assets a page links: stylesheets and scripts.
fn assets(html: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for attr in ["href=\"", "src=\""] {
        for (at, _) in html.match_indices(attr) {
            let v = &html[at + attr.len()..];
            let v = &v[..v.find('"').unwrap_or(0)];
            if v.starts_with("/static/") && (v.ends_with(".css") || v.ends_with(".js")) {
                out.insert(v.to_string());
            }
        }
    }
    out
}

struct Count {
    html: Vec<String>,
    by_asset: BTreeMap<String, Vec<String>>,
}

impl Count {
    fn total(&self) -> usize {
        self.html.len() + self.by_asset.values().map(Vec::len).sum::<usize>()
    }
}

async fn count(app: &axum::Router, cookie: &str, path: &str, signed_in: bool) -> Count {
    let resp = if signed_in {
        get_with_cookie(app, path, cookie).await
    } else {
        get(app, path).await
    };
    let status = resp.status();
    let html = body_string(resp).await;
    assert!(
        status == StatusCode::OK || (path == "/no-such-page" && status == StatusCode::NOT_FOUND),
        "{path}: {status}"
    );
    let html_hex: Vec<String> = hexes(&markup(&html))
        .into_iter()
        .filter(|h| !axgf_cms::river::on_water_ramp(h))
        .collect();
    let mut by_asset = BTreeMap::new();
    for a in assets(&html) {
        if a == "/static/css/tokens.css" {
            continue;
        }
        let body = body_string(get(app, &a).await).await;
        by_asset.insert(a, hexes(&body));
    }
    Count {
        html: html_hex,
        by_asset,
    }
}

#[tokio::test]
async fn no_colour_is_named_outside_the_tokens() {
    let (app, _s) = app();
    let cookie = admin_cookie(&app).await;
    let start: BTreeMap<&str, usize> = START.iter().copied().collect();
    let mut failures = Vec::new();
    println!("{:<14} {:>5} {:>5}  assets", "page", "total", "html");
    for (name, path, signed_in) in pages() {
        let c = count(&app, &cookie, &path, signed_in).await;
        let total = c.total();
        println!(
            "{name:<14} {total:>5} {:>5}  {}",
            c.html.len(),
            c.by_asset
                .iter()
                .map(|(a, h)| format!("{a} {}", h.len()))
                .collect::<Vec<_>>()
                .join(", ")
        );
        if CONVERTED.contains(&name) {
            if total > 0 {
                let mut seen: BTreeSet<&String> = c.html.iter().collect();
                for h in c.by_asset.values() {
                    seen.extend(h);
                }
                failures.push(format!(
                    "{name}: {total} hex literal(s) on a converted page: {seen:?}"
                ));
            }
        } else {
            match start.get(name) {
                Some(&was) if total > was => {
                    failures.push(format!("{name}: {total} hex literals, up from {was}"))
                }
                None => failures.push(format!("{name}: no starting count recorded ({total})")),
                _ => {}
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn the_tokens_are_the_studys() {
    let css = include_str!("../static/css/tokens.css");
    for (name, value) in [
        ("--surface-page", "#0b110e"),
        ("--surface-raised", "#111a16"),
        ("--surface-sunken", "#0e1512"),
        ("--surface-hover", "#17221d"),
        ("--border-hairline", "#22302a"),
        ("--border-strong", "#3a4b42"),
        ("--text-primary", "#ece6d6"),
        ("--text-secondary", "#a7b3aa"),
        ("--text-tertiary", "#75847b"),
        ("--accent", "#dcae64"),
        ("--focus-ring", "#dcae64"),
        ("--danger", "#d98068"),
    ] {
        assert!(
            css.contains(&format!("{name}: {value};")),
            "tokens.css: {name} should be {value}"
        );
    }
}

#[test]
fn the_scanner_finds_colours_and_leaves_ids_and_entities() {
    assert_eq!(
        hexes("color:#fff; fill:#0b110e; stroke:#dcae64cc"),
        ["#fff", "#0b110e", "#dcae64cc"]
    );
    assert!(hexes("&#127795; href=\"#main\" #deadbeefcafe url(#g1)").is_empty());
    assert_eq!(
        markup("<p style=\"color:#123\">lieu #9f8e7d6c</p>"),
        "<p style=\"color:#123\"></p>"
    );
    assert!(axgf_cms::river::on_water_ramp(&axgf_cms::river::colour_at(
        1850.0
    )));
    assert!(!axgf_cms::river::on_water_ramp("#dcae64"));
}
