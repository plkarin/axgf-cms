//! No colour is named outside the design tokens (Study 06, "The Furniture").
//!
//! Every page is rendered through the real router — the public pages signed
//! out, the administration signed in — and its HTML, and every stylesheet and
//! script it loads, is scanned for hex colour literals. `static/css/tokens.css`
//! is where colours are named, so it is not scanned. One exemption, named
//! here: **the river's water ramp in `src/river.rs`** (`river::on_water_ramp`),
//! and its port in `static/river.js`
//! — the colours `river::colour_at` gives the water for a year, and the
//! neutral water of a river without an era. It is a scale of years drawn on
//! the water, not furniture.
//!
//! In HTML only tags and `<style>` blocks are read, not text: a place with no
//! name is shown by its id, and `#9f8e7d6c` there is a name, not a colour.
//!
//! Every page is at zero, and zero is the rule. It got there in two steps:
//! app.css, which every page shares, moved its colours to tokens.css at once
//! (261 literals on every page), and then the river's drawing and its script
//! moved theirs to classes (64 on `/`, 81 signed in).

mod common;

use std::collections::{BTreeMap, BTreeSet};

use axum::http::StatusCode;
use common::*;
use serde_json::{json, Value};

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
        ("settings", "/settings?tab=theme".into(), false),
        ("quality", "/quality".into(), true),
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
        // river.js is river::render_svg's port for frames in motion, and
        // carries the same water ramp; the exemption is the ramp, wherever
        // it is drawn.
        let found = hexes(&body);
        let found = if a == "/static/river.js" {
            found
                .into_iter()
                .filter(|h| !axgf_cms::river::on_water_ramp(h))
                .collect()
        } else {
            found
        };
        by_asset.insert(a, found);
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
        if total > 0 {
            let mut seen: BTreeSet<&String> = c.html.iter().collect();
            for h in c.by_asset.values() {
                seen.extend(h);
            }
            failures.push(format!("{name}: {total} hex literal(s): {seen:?}"));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

/// Every block in tokens.css that defines custom properties, comments
/// stripped: (selector, [(name, value)]). An `@media` wrapper is looked
/// through, so the system-dark block is read as `:root:not([data-theme])`.
fn token_blocks() -> Vec<(String, BTreeMap<String, String>)> {
    let css = include_str!("../static/css/tokens.css");
    let mut src = String::new();
    let mut rest = css;
    while let Some(i) = rest.find("/*") {
        src.push_str(&rest[..i]);
        rest = &rest[i + rest[i..].find("*/").expect("closed comment") + 2..];
    }
    src.push_str(rest);
    let mut out = Vec::new();
    let mut start = 0;
    for (i, c) in src.char_indices() {
        match c {
            '}' => start = i + 1,
            '{' => {
                let sel = src[start..i]
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .join(" ");
                let end = i + src[i..].find('}').expect("closed block");
                let body = &src[i + 1..end];
                if !body.contains('{') {
                    let vars: BTreeMap<String, String> = body
                        .split(';')
                        .filter_map(|d| d.trim().split_once(':'))
                        .filter(|(k, _)| k.trim().starts_with("--"))
                        .map(|(k, v)| (k.trim().to_string(), v.trim().to_string()))
                        .collect();
                    if !vars.is_empty() {
                        out.push((
                            sel.trim_start_matches("@media (prefers-color-scheme: dark) ")
                                .to_string(),
                            vars,
                        ));
                    }
                }
                start = i + 1;
            }
            _ => {}
        }
    }
    out
}

fn block(sel: &str) -> BTreeMap<String, String> {
    token_blocks()
        .into_iter()
        .find(|(s, _)| s.starts_with(sel))
        .unwrap_or_else(|| panic!("tokens.css has no {sel} block"))
        .1
}

const STUDY_06: &[(&str, &str)] = &[
    ("--surface-page", "#0b110e"),
    ("--surface-raised", "#111a16"),
    ("--surface-sunken", "#0e1512"),
    ("--surface-hover", "#17221d"),
    ("--border-hairline", "#22302a"),
    // Not the study's #3a4b42: measured at 2.06:1 on its own page, under
    // the 3:1 of WCAG 1.4.11 (ADR 0002, §3).
    ("--border-strong", "#526a5d"),
    ("--text-primary", "#ece6d6"),
    ("--text-secondary", "#a7b3aa"),
    ("--text-tertiary", "#75847b"),
    ("--accent", "#dcae64"),
    ("--focus-ring", "#dcae64"),
    ("--danger", "#d98068"),
];

#[test]
fn the_dark_theme_is_study_06_and_so_is_the_river() {
    // Study 06 is the `dark` theme, written twice (the explicit choice and
    // the page with no choice, then "match my system" on a dark OS), and the
    // river carries the same values under every theme through its own
    // palette. Three copies of one palette: this is what keeps them one.
    let river = block(".river");
    for sel in [
        "[data-theme=\"dark\"], :root:not([data-theme])",
        "[data-theme=\"system\"]",
    ] {
        let b = block(sel);
        for (name, value) in STUDY_06 {
            assert_eq!(
                b.get(*name).map(String::as_str),
                Some(*value),
                "{sel} {name}"
            );
        }
    }
    for (name, value) in STUDY_06 {
        let via = river
            .get(*name)
            .unwrap_or_else(|| panic!(".river does not set {name}"));
        let rv = via
            .strip_prefix("var(")
            .and_then(|v| v.strip_suffix(')'))
            .unwrap_or_else(|| panic!(".river {name} should name one of its own: {via}"));
        assert_eq!(
            river.get(rv).map(String::as_str),
            Some(*value),
            ".river {name} -> {rv}"
        );
    }
}

fn luminance(hex: &str) -> f64 {
    let h = hex.trim_start_matches('#');
    let c = |i: usize| {
        let v = u8::from_str_radix(&h[i..i + 2], 16).expect("a hex colour") as f64 / 255.0;
        if v <= 0.03928 {
            v / 12.92
        } else {
            ((v + 0.055) / 1.055).powf(2.4)
        }
    };
    0.2126 * c(0) + 0.7152 * c(2) + 0.0722 * c(4)
}

/// The grain's strongest pixel: the alpha in its SVG's colour matrix.
const GRAIN: f64 = 0.05;

/// `fg` laid over `bg` at `alpha`, as a hex colour.
fn blend(fg: &str, bg: &str, alpha: f64) -> String {
    let ch = |h: &str, i: usize| {
        u8::from_str_radix(&h.trim_start_matches('#')[i..i + 2], 16).expect("hex") as f64
    };
    let mut out = String::from("#");
    for i in [0, 2, 4] {
        let v = alpha * ch(fg, i) + (1.0 - alpha) * ch(bg, i);
        out.push_str(&format!("{:02x}", v.round() as u8));
    }
    out
}

fn contrast(a: &str, b: &str) -> f64 {
    let (a, b) = (luminance(a), luminance(b));
    (a.max(b) + 0.05) / (a.min(b) + 0.05)
}

#[test]
fn every_theme_reads() {
    // WCAG AA for text (4.5:1) and for the graphics a control is known by
    // (3:1), measured on every surface the token can sit on, in every theme.
    let all = token_blocks();
    let root = all
        .iter()
        .find(|(s, _)| s == ":root")
        .expect(":root")
        .1
        .clone();
    let surfaces = ["--surface-page", "--surface-raised", "--surface-sunken"];
    let mut rules: Vec<(&str, &str, f64)> = Vec::new();
    for bg in surfaces.iter().chain(["--surface-hover"].iter()) {
        rules.push(("--text-primary", bg, 4.5));
        rules.push(("--text-secondary", bg, 4.5));
    }
    for bg in surfaces {
        rules.push(("--text-tertiary", bg, 4.5));
        rules.push(("--focus-ring", bg, 3.0));
        rules.push(("--border-strong", bg, 3.0));
    }
    for bg in ["--surface-page", "--surface-raised"] {
        rules.push(("--danger", bg, 4.5));
        rules.push(("--warning", bg, 4.5));
    }
    // The page wash's stops are grounds too: text and control edges sit on
    // the page wherever no card does (v0.4.3).
    for bg in ["--wash-1", "--wash-2", "--wash-3"] {
        for fg in ["--text-primary", "--text-secondary", "--text-tertiary"] {
            rules.push((fg, bg, 4.5));
        }
        rules.push(("--border-strong", bg, 3.0));
        rules.push(("--focus-ring", bg, 3.0));
        rules.push(("--danger", bg, 4.5));
        rules.push(("--warning", bg, 4.5));
    }
    rules.push(("--on-accent", "--accent", 4.5));

    let mut short = Vec::new();
    let mut themes = 0;
    for (sel, vars) in &all {
        if sel.starts_with(".river") {
            continue;
        }
        themes += 1;
        let mut v = root.clone();
        v.extend(vars.clone());
        for (fg, bg, min) in &rules {
            let r = contrast(&v[*fg], &v[*bg]);
            if r < *min {
                short.push(format!("{sel}: {fg} on {bg} is {r:.2}:1, under {min}:1"));
            }
        }
        // A theme that carries a texture is measured under it too: its
        // grain is at most 5% black on a light ground and 5% white on a dark
        // one, and every ground it can darken or lighten is taken at that.
        if v.get("--wash-texture").is_some_and(|t| t.contains("url(")) {
            let dark = luminance(&v["--surface-page"]) < 0.2;
            let veil = if dark { "#ffffff" } else { "#000000" };
            for ground in ["--surface-page", "--wash-1", "--wash-2", "--wash-3"] {
                let g = blend(veil, &v[ground], GRAIN);
                for (fg, min) in [
                    ("--text-primary", 4.5),
                    ("--text-secondary", 4.5),
                    ("--text-tertiary", 4.5),
                    ("--border-strong", 3.0),
                    ("--focus-ring", 3.0),
                    ("--danger", 4.5),
                    ("--warning", 4.5),
                ] {
                    let r = contrast(&v[fg], &g);
                    if r < min {
                        short.push(format!(
                            "{sel}: {fg} on {ground} under the grain is {r:.2}:1, under {min}:1"
                        ));
                    }
                }
            }
        }
    }
    assert_eq!(
        themes, 13,
        "light, system-dark, dark, canopée, fusion, galaxy, high contrast, sepia, \
         crépuscule, encre, fleuve d'hiver, the red-green pair, tritanopia"
    );
    assert!(short.is_empty(), "{}", short.join("\n"));
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
