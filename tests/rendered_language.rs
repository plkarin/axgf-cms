//! A page rendered in Chinese carries no English.
//!
//! # Why this exists beside `tests/i18n.rs`
//!
//! `tests/i18n.rs` reads the templates and fails on prose outside a `t(...)`
//! call. It cannot see a string Rust assembled: by the time such a value
//! reaches a template it is a variable like any other. That blind spot has
//! produced English on a translated page again and again — union roles
//! ("spouse" on the Arabic page), and the edit history, which put dotted JSON
//! paths into a translated sentence ("zmienił(a) extensions").
//!
//! Neither of those was an English *literal* in the Rust source. One was a
//! vocabulary value from the data, the other a field name from the file
//! format. So a scanner over the Rust source would not have caught them
//! either, however it was written. What does catch them is looking at what
//! the reader actually receives.
//!
//! So this renders the real pages, in Simplified Chinese, over a fixture whose
//! own text — names, places, notes, titles — is all Greek, and fails on any
//! run of Latin letters in what a reader can see or hear. Whatever Latin text
//! is left on such a page did not come from the family's data and did not come
//! from the Chinese catalogue: it is English that escaped translation.

mod common;

use axum::http::StatusCode;
use common::*;
use serde_json::json;

const A: &str = "aaaaaaaa-1111-4111-8111-111111111111";
const B: &str = "bbbbbbbb-2222-4222-8222-222222222222";
const KID: &str = "cccccccc-3333-4333-8333-333333333333";
const FAM: &str = "dddddddd-4444-4444-8444-444444444444";
const EV: &str = "eeeeeeee-5555-4555-8555-555555555555";
const PLACE: &str = "ffffffff-6666-4666-8666-666666666666";
const SRC: &str = "abababab-7777-4777-8777-777777777777";
const OCC: &str = "cdcdcdcd-8888-4888-8888-888888888888";
const LINK: &str = "efefefef-9999-4999-8999-999999999999";

/// Words that are the same in every language this application speaks, or
/// that name a thing rather than describe it.
const UNIVERSAL: &[&str] = &[
    "AXGF",
    "JSON",
    "UUID",
    "GEDCOM",
    "DNA",
    "ISO",
    "ISCED",
    "OCR",
    "PNG",
    "JPEG",
    "ZIP",
    "CSV",
    "URL",
    "HTTP",
    "HTTPS",
    "TLS",
    "API",
    "GPS",
    "BMI",
    "SVG",
    "PDF",
    // The product's one name in every language (`app-name`), and services
    // and registries named as themselves.
    "genealogy",
    "Leaflet",
    "OpenStreetMap",
    "contributors",
    "Google",
    "Wikidata",
    "GeoNames",
    "URI",
    "alpha",
];

fn fixture() -> (axum::Router, Scratch) {
    let person = |id: &str, name: &str| {
        json!({"id": id, "type": "person", "axgf_version": "1.1", "version_num": 1,
               "identity": {"name": {"display": name, "components": []},
                            "is_living": false, "visibility": "public",
                            "gender": {"value": "F"}},
               "birth": {"date": {"value": "1901", "precision": "year"}, "place_id": PLACE},
               "death": {"date": {"value": "1980", "precision": "year"}},
               "notes": "Σημείωση για την οικογένεια."})
    };
    let mut a = person(A, "Ελένη Παπαδοπούλου");
    a["morphology"] = json!({"height": [{"value": 165}]});
    let flat = json!({
        "manifest": {"axgf": "1.1", "created_at": "2026-09-01T00:00:00Z"},
        "persons": {A: a, B: person(B, "Γιώργος Παπαδόπουλος"),
                    KID: person(KID, "Μαρία Παπαδοπούλου")},
        "families": {FAM: {"id": FAM, "type": "family", "axgf_version": "1.0",
            "version_num": 1,
            "union": {"type": "marriage", "status": "ended_by_death",
                      "persons": [{"person_id": A, "role": "spouse"},
                                  {"person_id": B, "role": "spouse"}],
                      "start": {"date": {"value": "1925", "precision": "year"}}},
            "children": [{"person_id": KID}]}},
        "events": {EV: {"id": EV, "type": "event", "axgf_version": "1.0", "version_num": 1,
            "category": "marriage", "date": {"value": "1925", "precision": "year"},
            "place_id": PLACE,
            "participants": [
                {"entity_type": "person", "entity_id": A, "role": "spouse_1"},
                {"entity_type": "person", "entity_id": B, "role": "spouse_2"}]}},
        "places": {PLACE: {"id": PLACE, "type": "place", "axgf_version": "1.0",
            "version_num": 1, "names": [{"lang": "el", "value": "Θεσσαλονίκη", "is_primary": true}]}},
        "sources": {SRC: {"id": SRC, "type": "source", "axgf_version": "1.0",
            "version_num": 1, "title": "Ληξιαρχικό βιβλίο", "source_type": "birth_certificate",
            "reliability": "primary"}},
        "occupations": {OCC: {"id": OCC, "type": "occupation", "axgf_version": "1.0",
            "version_num": 1, "person_id": B, "title": "Ράφτης"}},
        "links": {LINK: {"id": LINK, "type": "link", "axgf_version": "1.1", "version_num": 1,
            "from": {"entity_type": "person", "entity_id": A},
            "to": {"entity_type": "person", "entity_id": KID},
            "label": "νονά", "category": "spiritual", "relation": "godparent"}},
        "documents": {}
    });
    let src = scratch("rendered-src");
    let path = src.join("greek.axgf");
    std::fs::write(
        &path,
        axgf_cms::state::export_to_bytes(&flat.to_string()).expect("export"),
    )
    .expect("write");
    app_with_bundle("rendered", &path)
}

#[tokio::test]
async fn no_page_rendered_in_chinese_carries_an_english_word() {
    let (app, _dir) = fixture();

    // One real edit through the generic editor, so every history on the site
    // has an entry to summarise — the summary is where the paths leaked.
    let edited = {
        let resp = get_admin(&app, &format!("/admin/person/{A}/edit")).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let env = axgf_rs::import_bundle(&std::fs::read(&*_dir).expect("read"));
        let mut p = env.data["persons"][A].clone();
        p["notes"] = json!("Διορθωμένη σημείωση.");
        p["morphology"]["weight"] = json!([{"value": 60}]);
        p["extensions"] = json!({"axgf-cms:avatar/v1": {"document_id": null}});
        p
    };
    let body = format!(
        "identity.name.display={}&base_version=1&raw_json={}",
        enc("Ελένη Παπαδοπούλου"),
        enc(&edited.to_string())
    );
    let resp = post_form(&app, &format!("/admin/person/{A}"), &body, true).await;
    assert!(
        resp.status().is_success() || resp.status().is_redirection(),
        "the edit that seeds the history: {}",
        resp.status()
    );

    let cookie = format!("{}; axgf_lang=zh-Hans", admin_cookie(&app).await);
    let pages = [
        "/".to_string(),
        "/tree".into(),
        format!("/tree/panel/{A}"),
        format!("/person/{A}"),
        format!("/person/{A}?tab=history"),
        format!("/person/{A}?tab=profile"),
        format!("/person/{B}"),
        "/settings".into(),
        "/convert".into(),
        "/admin".into(),
        "/admin/users".into(),
        "/admin/person".into(),
        "/admin/family".into(),
        "/admin/event".into(),
        "/admin/link".into(),
        "/admin/occupation".into(),
        "/admin/source".into(),
        "/admin/place".into(),
        "/admin/document".into(),
        "/admin/person/new".into(),
        format!("/admin/person/{A}/edit"),
        format!("/admin/person/{A}/identity"),
        format!("/admin/person/{A}/family"),
        format!("/admin/person/{A}/events"),
        format!("/admin/person/{A}/occupations"),
        format!("/admin/person/{A}/links"),
        format!("/admin/person/{A}/documents"),
        format!("/admin/person/{A}/profile/morphology"),
        format!("/admin/family/{FAM}/edit"),
        format!("/admin/event/{EV}/edit"),
        format!("/admin/link/{LINK}/edit"),
        format!("/admin/occupation/{OCC}/edit"),
        format!("/admin/source/{SRC}/edit"),
        format!("/admin/place/{PLACE}/edit"),
    ];

    let mut report = Vec::new();
    for uri in &pages {
        let resp = get_with_cookie(&app, uri, &cookie).await;
        let status = resp.status();
        let html = body_string(resp).await;
        if !status.is_success() {
            report.push(format!("{uri}: {status} (not rendered, so not checked)"));
            continue;
        }
        // The tree's side panel is a fragment and carries no <html> of its own.
        assert!(
            !html.contains("<html") || html.contains(r#"lang="zh-Hans""#),
            "{uri} did not render in Chinese at all"
        );
        let found = english_in(&html);
        if !found.is_empty() {
            report.push(format!("{uri}:\n{}", found.join("\n")));
        }
    }
    assert!(
        report.is_empty(),
        "English reached a page rendered in Chinese. Each line is a word a \
         Chinese reader sees, with what surrounds it:\n\n{}",
        report.join("\n\n")
    );
}

/// Every Latin-script word a reader of `html` can see or hear, with context.
fn english_in(html: &str) -> Vec<String> {
    let mut text = html.to_string();
    // Not reader-facing, or verbatim by design: code, the raw-JSON editor,
    // identifiers printed as identifiers.
    for tag in ["script", "style", "textarea", "code", "pre", "datalist"] {
        text = strip_elements(&text, tag);
    }
    let mut spoken = Vec::new();
    // What a screen reader says or a tooltip shows is read too.
    for attr in ["title", "aria-label", "placeholder", "alt"] {
        let needle = format!(" {attr}=\"");
        let mut rest = text.as_str();
        while let Some(i) = rest.find(&needle) {
            let after = &rest[i + needle.len()..];
            let end = after.find('"').unwrap_or(after.len());
            spoken.push(after[..end].to_string());
            rest = &after[end..];
        }
    }
    let mut visible = String::new();
    let mut in_tag = false;
    for ch in text.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => {
                in_tag = false;
                visible.push(' ');
            }
            c if !in_tag => visible.push(c),
            _ => {}
        }
    }
    spoken.push(visible);

    let mut out = Vec::new();
    for chunk in spoken {
        let chunk = unescape(&chunk);
        for (i, w) in words(&chunk) {
            if UNIVERSAL.contains(&w.as_str()) || is_identifier(&chunk, i, w.len()) {
                continue;
            }
            let from = chunk[..i]
                .char_indices()
                .rev()
                .nth(30)
                .map_or(0, |(j, _)| j);
            let ctx: String = chunk[from..].chars().take(70).collect();
            let line = format!(
                "  {w:<16} …{}…",
                ctx.split_whitespace().collect::<Vec<_>>().join(" ")
            );
            if !out.contains(&line) {
                out.push(line);
            }
        }
    }
    out
}

/// Runs of three or more Latin letters, with their byte offset.
fn words(s: &str) -> Vec<(usize, String)> {
    let mut out = Vec::new();
    let mut start = None;
    for (i, c) in s.char_indices().chain(std::iter::once((s.len(), ' '))) {
        if c.is_ascii_alphabetic() {
            start.get_or_insert(i);
        } else if let Some(b) = start.take() {
            if i - b >= 3 {
                out.push((b, s[b..i].to_string()));
            }
        }
    }
    out
}

/// Whether the word at `at` sits inside something to type rather than to
/// read: a UUID or the `#xxxxxxxx` a picker prints, a path, a file extension,
/// a command-line flag, a `geo:` URI or a language tag. The token is the run
/// of identifier characters around the word, so this works in Chinese text,
/// which puts no space before or after one.
fn is_identifier(s: &str, at: usize, len: usize) -> bool {
    let ident = |c: char| c.is_ascii_alphanumeric() || "-_./:#'°\"".contains(c);
    let start = s[..at]
        .char_indices()
        .rev()
        .take_while(|(_, c)| ident(*c))
        .last()
        .map_or(at, |(i, _)| i);
    let end = s[at + len..]
        .char_indices()
        .find(|(_, c)| !ident(*c))
        .map_or(s.len(), |(i, _)| at + len + i);
    let raw = &s[start..end];
    let token = raw.trim_end_matches(['.', ':', ',']);
    let hexish = token
        .trim_start_matches('#')
        .chars()
        .all(|c| c.is_ascii_hexdigit() || c == '-')
        && token.len() >= 8;
    hexish
        || token.contains('/')
        || token.starts_with('.')
        || token.starts_with("--")
        || raw.starts_with("geo:")
        || token == "zh-Hans"
        // The journal's literal name for the emergency token, quoted as such
        // by the text that explains it.
        || token == "emergency-token"
        // Roman numerals: a skin type or a malocclusion class is numbered so.
        || token.chars().all(|c| "IVX".contains(c))
}

fn strip_elements(s: &str, tag: &str) -> String {
    let open = format!("<{tag}");
    let close = format!("</{tag}>");
    let mut out = String::with_capacity(s.len());
    let mut rest = s;
    while let Some(i) = rest.find(&open) {
        out.push_str(&rest[..i]);
        match rest[i..].find(&close) {
            Some(j) => rest = &rest[i + j + close.len()..],
            None => {
                rest = "";
            }
        }
    }
    out.push_str(rest);
    out
}

fn unescape(s: &str) -> String {
    s.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#x27;", "'")
        .replace("&#39;", "'")
        .replace("&nbsp;", " ")
        .replace("&#x2f;", "/")
}

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
