//! Every interactive element does something.
//!
//! A button that does nothing reads as "the site is broken" to anyone who is
//! not an engineer, and on the river page most of them did: a `data-centre`
//! on the river's root made every link inside it resolve to the root, travel
//! to where it already was, and stop there.
//!
//! This walks every page a reader reaches — the river, the grid, the
//! overview, a record, the search, the import flow and every administration
//! page — signed out and signed in, and holds every interactive element
//! (`a[href]`, `button`, `input`, `select`, `textarea`, `summary`, anything
//! with a `role` or a `tabindex`) to one of three things:
//!
//! * **it navigates** — a link whose target answers 200 (following redirects),
//!   or a `#fragment` that names something on the page;
//! * **it submits** — it sits in a form whose action answers 200 for a GET, or
//!   accepts the form's method for a POST (checked without sending it);
//! * **it has a handler bound** — a selector in a script *this page loads*
//!   (`querySelector`, `closest`, `getElementById`) matches it or an ancestor.
//!
//! Anything else is dead and fails the test. So does a delegation target that
//! swallows other controls: an element a script claims through `closest()`
//! must not contain another control, because a click on that control reaches
//! the script as a click on the claimed element — exactly the river's bug.

mod common;

use std::collections::{BTreeMap, BTreeSet};

use axum::body::Body;
use axum::http::{header, Method, Request, StatusCode};
use common::*;
use scraper::{ElementRef, Html, Selector};
use serde_json::{json, Value};
use tower::ServiceExt;

const P: &str = "p07";

fn person(id: &str, given: &str, surname: &str, born: Option<i64>, visibility: &str) -> Value {
    let mut p = json!({"id": id, "type": "person", "axgf_version": "1.1", "version_num": 1,
        "identity": {"name": {"display": format!("{given} {surname}"),
            "components": [{"type": "given_name", "value": given},
                           {"type": "family_name", "value": surname}]},
            "gender": {"value": "M"}, "is_living": false, "visibility": visibility}});
    if let Some(b) = born {
        p["birth"] = json!({"date": {"value": b.to_string(), "precision": "year"}, "place_id": "pl1",
                            "source_id": "s1"});
    }
    p
}

fn family(id: &str, parents: &[&str], kids: &[&str]) -> Value {
    json!({"id": id, "type": "family", "axgf_version": "1.1", "version_num": 1,
        "union": {"type": "marriage",
            "persons": parents.iter().map(|p| json!({"person_id": p, "role": "spouse"})).collect::<Vec<_>>()},
        "children": kids.iter().map(|k| json!({"person_id": k, "confidence": 0.9})).collect::<Vec<_>>()})
}

/// Every kind of record at least once, so every list, tab and editor has
/// something to draw; one person hidden from visitors.
fn bundle() -> Value {
    let persons: serde_json::Map<String, Value> = [
        person("p01", "Jan", "Nowak", Some(1800), "public"),
        person("p02", "Anna", "Lis", Some(1806), "public"),
        person("p03", "Antoni", "Nowak", Some(1830), "public"),
        person("p04", "Zofia", "Mazur", None, "public"),
        person("p05", "Józef", "Nowak", Some(1858), "public"),
        person("p06", "Marianne", "Lefèvre", Some(1863), "public"),
        person("p07", "Stanislas", "Nowak", Some(1888), "public"),
        person("p08", "Hélène", "Mercier", Some(1890), "public"),
        person("p09", "Michel", "Nowak", Some(1914), "public"),
        person("p10", "Ukryta", "Nowak", Some(1950), "members"),
    ]
    .into_iter()
    .map(|p| (p["id"].as_str().unwrap().to_string(), p))
    .collect();
    let families: serde_json::Map<String, Value> = [
        family("f1", &["p01", "p02"], &["p03"]),
        family("f2", &["p03", "p04"], &["p05"]),
        family("f3", &["p05", "p06"], &["p07"]),
        family("f4", &["p07", "p08"], &["p09"]),
        family("f5", &["p09"], &["p10"]),
    ]
    .into_iter()
    .map(|f| (f["id"].as_str().unwrap().to_string(), f))
    .collect();
    json!({
        "manifest": {"axgf": "1.1", "created_at": "2026-10-10T00:00:00Z"},
        "persons": persons,
        "families": families,
        "places": {"pl1": {"id": "pl1", "type": "place", "axgf_version": "1.0", "version_num": 1,
            "name": "Lublin", "hierarchy": {"region": "Lubelskie", "country": "PL"}}},
        "events": {"e1": {"id": "e1", "type": "event", "axgf_version": "1.0", "version_num": 1,
            "event_type": "baptism", "date": {"value": "1888", "precision": "year"},
            "place_id": "pl1", "participants": [{"person_id": "p07", "role": "subject"}]}},
        "occupations": {"o1": {"id": "o1", "type": "occupation", "axgf_version": "1.0", "version_num": 1,
            "person_id": "p07", "title": "Teacher"}},
        "links": {"l1": {"id": "l1", "type": "link", "axgf_version": "1.0", "version_num": 1,
            "from": {"entity_type": "person", "entity_id": "p08"},
            "to": {"entity_type": "person", "entity_id": "p07"},
            "label": "godmother", "label_reverse": "godson", "category": "spiritual"}},
        "sources": {"s1": {"id": "s1", "type": "source", "axgf_version": "1.0", "version_num": 1,
            "title": "Registre des naissances", "source_type": "birth_certificate", "reliability": "primary"}},
        "documents": {"d1": {"id": "d1", "type": "document", "axgf_version": "1.0", "version_num": 1,
            "filename": "acte.pdf", "mime_type": "application/pdf", "document_type": "birth_certificate",
            "status": "referenced",
            "linked_to": [{"entity_type": "person", "entity_id": "p07", "role": "subject"}]}}
    })
}

fn app() -> (axum::Router, Scratch) {
    let src = scratch("walk-src");
    let path = src.join("walk.axgf");
    std::fs::write(
        &path,
        axgf_cms::state::export_to_bytes(&bundle().to_string()).expect("export"),
    )
    .expect("write");
    let out = app_with_bundle("walk", &path);
    drop(src);
    out
}

/// A request as the reader: with their session cookie, if any.
async fn send(
    app: &axum::Router,
    method: Method,
    uri: &str,
    cookie: Option<&str>,
) -> axum::http::Response<Body> {
    let mut b = Request::builder().method(method).uri(uri);
    if let Some(c) = cookie {
        b = b.header(header::COOKIE, c);
    }
    app.clone()
        .oneshot(b.body(Body::empty()).unwrap())
        .await
        .expect("request")
}

/// GET, following redirects the way a browser does. The final status, and
/// the body when it is a page.
async fn fetch(
    app: &axum::Router,
    uri: &str,
    cookie: Option<&str>,
) -> (StatusCode, String, String) {
    let mut at = uri.to_string();
    for _ in 0..5 {
        let resp = send(app, Method::GET, &at, cookie).await;
        let status = resp.status();
        if status.is_redirection() {
            let to = resp
                .headers()
                .get(header::LOCATION)
                .and_then(|v| v.to_str().ok())
                .unwrap_or("")
                .to_string();
            at = resolve(&at, &to);
            continue;
        }
        let is_html = resp
            .headers()
            .get(header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .is_some_and(|t| t.starts_with("text/html"));
        let body = if is_html {
            body_string(resp).await
        } else {
            String::new()
        };
        return (status, body, at);
    }
    (StatusCode::LOOP_DETECTED, String::new(), at)
}

/// A link's target as a path on this site, relative to the page it is on.
fn resolve(page: &str, href: &str) -> String {
    let href = href.replace("&amp;", "&");
    if href.starts_with('/') {
        return href;
    }
    let page_path = page.split(['?', '#']).next().unwrap_or("/");
    if let Some(q) = href.strip_prefix('?') {
        return format!("{page_path}?{q}");
    }
    let dir = page_path.rsplit_once('/').map_or("", |(d, _)| d);
    format!("{dir}/{href}")
}

/// Whether the router has a handler for `method` at `path`, asked without
/// running it: an OPTIONS request answers 405 with an `Allow` header naming
/// the methods a route has, and 404 when there is no route at all.
async fn accepts(app: &axum::Router, method: &str, path: &str, cookie: Option<&str>) -> bool {
    let resp = send(app, Method::OPTIONS, path, cookie).await;
    if resp.status() == StatusCode::NOT_FOUND {
        return false;
    }
    let allow = resp
        .headers()
        .get(header::ALLOW)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_ascii_uppercase();
    allow.split(',').any(|m| m.trim() == method) || resp.status().is_success()
}

/// The selectors a script binds: the argument of every `querySelector`,
/// `querySelectorAll`, `closest` and `getElementById` call, the last as
/// `#id`. Those passed to `closest` on an event's target are returned
/// separately: they are delegation targets, which must not swallow other
/// controls.
fn bound_selectors(js: &str) -> (Vec<String>, Vec<String>) {
    let (mut all, mut closest) = (Vec::new(), Vec::new());
    for call in [
        "querySelectorAll(",
        "querySelector(",
        "closest(",
        "getElementById(",
    ] {
        for (at, _) in js.match_indices(call) {
            let rest = js[at + call.len()..].trim_start();
            let Some(q) = rest.chars().next().filter(|c| *c == '\'' || *c == '"') else {
                continue;
            };
            let Some(end) = rest[1..].find(q) else {
                continue;
            };
            let arg = &rest[1..1 + end];
            let sel = if call == "getElementById(" {
                format!("#{arg}")
            } else {
                arg.to_string()
            };
            // Delegation is `closest` on an event's target. On anything else
            // (`frame.closest('label')`) it is a lookup from an element the
            // script already holds, and claims no clicks.
            if call == "closest(" && js[..at].ends_with("target.") {
                closest.push(sel.clone());
            }
            all.push(sel);
        }
    }
    (all, closest)
}

fn interactive() -> Selector {
    Selector::parse(
        "a[href], button, input:not([type=hidden]), select, textarea, summary, \
         [role=button], [role=link], [role=tab], [role=option], [role=menuitem], [tabindex]",
    )
    .unwrap()
}

/// A short, recognisable description of an element for a failure message.
fn describe(el: &ElementRef) -> String {
    let html = el.html();
    let open = html.split('>').next().unwrap_or("").to_string() + ">";
    let text: String = el
        .text()
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let text: String = text.chars().take(40).collect();
    format!("{open} {text:?}")
}

/// The form an element sits in, if any.
fn form_of<'a>(el: &ElementRef<'a>) -> Option<ElementRef<'a>> {
    let mut cur = el.parent().and_then(ElementRef::wrap);
    while let Some(e) = cur {
        if e.value().name() == "form" {
            return Some(e);
        }
        cur = e.parent().and_then(ElementRef::wrap);
    }
    None
}

#[derive(Default, Debug)]
struct Tally {
    walked: usize,
    navigates: usize,
    submits: usize,
    bound: usize,
    native: usize,
    external: usize,
    dead: Vec<String>,
}

/// Walk one page as one reader.
async fn walk(
    app: &axum::Router,
    page: &str,
    cookie: Option<&str>,
    who: &str,
    checked_links: &mut BTreeMap<String, StatusCode>,
    tally: &mut Tally,
) {
    let (status, html, landed) = fetch(app, page, cookie).await;
    assert_eq!(status, StatusCode::OK, "{who}: {page} answers {status}");
    let doc = Html::parse_document(&html);

    // The scripts this page loads, and what they bind.
    let script = Selector::parse("script[src]").unwrap();
    let (mut selectors, mut delegates) = (Vec::new(), Vec::new());
    for s in doc.select(&script) {
        let src = s.value().attr("src").unwrap_or("");
        if !src.starts_with("/static/") {
            continue;
        }
        let body = body_string(send(app, Method::GET, src, cookie).await).await;
        let (a, c) = bound_selectors(&body);
        selectors.extend(a);
        delegates.extend(c);
    }
    let parsed: Vec<Selector> = selectors
        .iter()
        .filter_map(|s| Selector::parse(s).ok())
        .collect();
    let delegated: Vec<Selector> = delegates
        .iter()
        .filter_map(|s| Selector::parse(s).ok())
        .collect();
    let ids: BTreeSet<String> = doc
        .select(&Selector::parse("[id]").unwrap())
        .filter_map(|e| e.value().attr("id").map(str::to_string))
        .collect();

    // No delegation target may swallow another control.
    let controls = interactive();
    for sel in &delegated {
        for claimed in doc.select(sel) {
            let inner: Vec<String> = claimed.select(&controls).map(|c| describe(&c)).collect();
            if !inner.is_empty() {
                tally.dead.push(format!(
                    "{who} {page}: {} is claimed by a script's closest() and swallows {} control(s), e.g. {}",
                    describe(&claimed),
                    inner.len(),
                    inner[0]
                ));
            }
        }
    }

    let is_bound = |el: &ElementRef| -> bool {
        let mut cur = Some(*el);
        while let Some(e) = cur {
            if parsed.iter().any(|s| s.matches(&e)) {
                return true;
            }
            cur = e.parent().and_then(ElementRef::wrap);
        }
        false
    };
    for el in doc.select(&controls) {
        tally.walked += 1;
        let v = el.value();
        let what = describe(&el);
        // A link navigates.
        if v.name() == "a" {
            let href = v.attr("href").unwrap_or("");
            if href.starts_with("http://")
                || href.starts_with("https://")
                || href.starts_with("mailto:")
            {
                tally.external += 1;
                continue;
            }
            if let Some(frag) = href.strip_prefix('#') {
                if ids.contains(frag) || is_bound(&el) {
                    tally.navigates += 1;
                } else {
                    tally.dead.push(format!(
                        "{who} {page}: {what} points at #{frag}, which is not on the page"
                    ));
                }
                continue;
            }
            let target = resolve(&landed, href.split('#').next().unwrap_or(""));
            let status = match checked_links.get(&target) {
                Some(s) => *s,
                None => {
                    let (s, _, _) = fetch(app, &target, cookie).await;
                    checked_links.insert(target.clone(), s);
                    s
                }
            };
            if status == StatusCode::OK {
                tally.navigates += 1;
            } else {
                tally
                    .dead
                    .push(format!("{who} {page}: {what} -> {target} answers {status}"));
            }
            continue;
        }
        // <summary> opens its <details>; that is the browser's own handler.
        if v.name() == "summary" {
            tally.native += 1;
            continue;
        }
        // Inside a form: the form submits. Its action is checked once per
        // element; a control in a dead form is as dead as the form.
        if let Some(form) = form_of(&el) {
            let method = form
                .value()
                .attr("method")
                .unwrap_or("get")
                .to_ascii_uppercase();
            let action = form
                .value()
                .attr("action")
                .map(|a| resolve(&landed, a))
                .unwrap_or_else(|| landed.clone());
            let ok = if method == "GET" {
                let key = format!("GET {action}");
                match checked_links.get(&key) {
                    Some(s) => *s == StatusCode::OK,
                    None => {
                        let (s, _, _) = fetch(app, &action, cookie).await;
                        checked_links.insert(key, s);
                        s == StatusCode::OK
                    }
                }
            } else {
                accepts(
                    app,
                    &method,
                    action.split('?').next().unwrap_or(&action),
                    cookie,
                )
                .await
            };
            if ok {
                tally.submits += 1;
            } else {
                tally.dead.push(format!("{who} {page}: {what} submits a form to {method} {action}, which nothing handles"));
            }
            continue;
        }
        // Anything else needs a script.
        if is_bound(&el) {
            tally.bound += 1;
        } else {
            tally.dead.push(format!(
                "{who} {page}: {what} navigates nowhere, submits nothing, and no script binds it"
            ));
        }
    }
}

/// The pages a reader reaches, signed out and as an administrator.
fn pages(signed_in: bool, converted: Option<&str>) -> Vec<String> {
    let mut out: Vec<String> = [
        "/".to_string(),
        format!("/?p={P}&n=3"),
        format!("/?p={P}&n=2&tab=family"),
        "/tree".into(),
        format!("/tree?root={P}"),
        "/about".into(),
        format!("/person/{P}"),
        format!("/person/{P}?tab=family"),
        "/search?q=nowak".into(),
        "/settings".into(),
        "/convert".into(),
    ]
    .into();
    if let Some(page) = converted {
        out.push(page.to_string());
    }
    if signed_in {
        out.push("/quality".into());
        out.push("/quality?group=places".into());
        out.push("/quality?show=dismissed".into());
        out.push("/admin".into());
        for kind in axgf_cms::admin::KINDS {
            out.push(format!("/admin/{kind}"));
            out.push(format!("/admin/{kind}/new"));
        }
        for (kind, id) in [
            ("person", P),
            ("family", "f3"),
            ("event", "e1"),
            ("source", "s1"),
            ("place", "pl1"),
        ] {
            out.push(format!("/admin/{kind}/{id}/edit"));
        }
        // The record's own editors, one per facet.
        for facet in [
            "identity",
            "family",
            "events",
            "occupations",
            "links",
            "documents",
            "avatar",
            "ai-portraits",
            "profile",
            "physical",
        ] {
            out.push(format!("/admin/person/{P}/{facet}"));
        }
        out.push("/admin/place/new".into());
        out.push("/admin/users".into());
    } else {
        out.push("/admin/login".into());
    }
    out
}

const BOUNDARY: &str = "----axgfwalkboundary";

/// The import flow's second page: a GEDCOM converted, with its download.
async fn convert_once(app: &axum::Router, cookie: Option<&str>) -> String {
    let ged = std::fs::read(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/small.ged"),
    )
    .expect("fixture");
    let mut body = Vec::new();
    body.extend_from_slice(format!("--{BOUNDARY}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"small.ged\"\r\nContent-Type: application/octet-stream\r\n\r\n").as_bytes());
    body.extend_from_slice(&ged);
    body.extend_from_slice(b"\r\n");
    for (n, v) in [("confidence", "0.8"), ("lang", "fr")] {
        body.extend_from_slice(
            format!("--{BOUNDARY}\r\nContent-Disposition: form-data; name=\"{n}\"\r\n\r\n{v}\r\n")
                .as_bytes(),
        );
    }
    body.extend_from_slice(format!("--{BOUNDARY}--\r\n").as_bytes());
    let mut b = Request::builder()
        .method(Method::POST)
        .uri("/convert/gedcom")
        .header(
            header::CONTENT_TYPE,
            format!("multipart/form-data; boundary={BOUNDARY}"),
        );
    if let Some(c) = cookie {
        b = b.header(header::COOKIE, c);
    }
    let resp = app
        .clone()
        .oneshot(b.body(Body::from(body)).unwrap())
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK, "the import converts");
    body_string(resp).await
}

/// The import result is a page answered to a POST, so it is walked from its
/// HTML rather than fetched: every element on it is held to the same rule.
async fn walk_html(
    app: &axum::Router,
    html: &str,
    cookie: Option<&str>,
    who: &str,
    tally: &mut Tally,
) {
    let doc = Html::parse_document(html);
    for el in doc.select(&interactive()) {
        tally.walked += 1;
        let what = describe(&el);
        if let Some(href) = el.value().attr("href") {
            if href.starts_with("http://") || href.starts_with("https://") {
                tally.external += 1;
                continue;
            }
            let (s, _, _) = fetch(app, &resolve("/convert/gedcom", href), cookie).await;
            if s == StatusCode::OK {
                tally.navigates += 1;
            } else {
                tally
                    .dead
                    .push(format!("{who} import result: {what} -> {href} answers {s}"));
            }
        } else if el.value().name() == "summary" {
            tally.native += 1;
        } else {
            // The result page's forms are the masthead's and the
            // preferences'; they are walked on every other page.
            tally.submits += 1;
        }
    }
}

#[tokio::test]
async fn every_interactive_element_does_something_signed_out_and_signed_in() {
    let (app, _s) = app();
    let cookie = admin_cookie(&app).await;
    let mut report = Vec::new();
    let mut dead = Vec::new();
    for (who, c) in [("signed out", None), ("signed in", Some(cookie.as_str()))] {
        let mut tally = Tally::default();
        let mut links = BTreeMap::new();
        for page in pages(c.is_some(), None) {
            walk(&app, &page, c, who, &mut links, &mut tally).await;
        }
        let result = convert_once(&app, c).await;
        walk_html(&app, &result, c, who, &mut tally).await;
        report.push(format!(
            "{who}: {} elements walked — {} navigate, {} submit, {} bound by a script, {} native, {} external",
            tally.walked, tally.navigates, tally.submits, tally.bound, tally.native, tally.external
        ));
        dead.extend(tally.dead);
    }
    println!("{}", report.join("\n"));
    for d in &dead {
        println!("DEAD {d}");
    }
    assert!(
        dead.is_empty(),
        "{} dead element(s):\n{}",
        dead.len(),
        dead.join("\n")
    );
}

#[test]
fn the_walk_reads_what_scripts_bind() {
    let (all, closest) = bound_selectors(
        "var t = e.target.closest('[data-range],[data-go]'); document.getElementById(\"rv-back\"); x.querySelectorAll('.tcard');",
    );
    assert_eq!(closest, ["[data-range],[data-go]"]);
    assert!(all.contains(&"#rv-back".to_string()) && all.contains(&".tcard".to_string()));
    // A lookup from an element in hand delegates nothing.
    let (looked_up, delegated) = bound_selectors("var label = frame.closest('label');");
    assert_eq!(looked_up, ["label"]);
    assert!(delegated.is_empty());
    assert_eq!(
        resolve("/person/p1", "?tab=family"),
        "/person/p1?tab=family"
    );
    assert_eq!(
        resolve("/admin/person", "/admin/person/new"),
        "/admin/person/new"
    );
}

/// At most one primary button on a page (B2): the primary is the one thing
/// the page is for. A form per record makes every save a secondary; the
/// river has none at all.
#[tokio::test]
async fn no_page_has_more_than_one_primary_button() {
    let (app, _s) = app();
    let cookie = admin_cookie(&app).await;
    let primary = Selector::parse(".btn.primary").unwrap();
    let mut over = Vec::new();
    let mut seen = 0;
    for (who, c) in [("signed out", None), ("signed in", Some(cookie.as_str()))] {
        for page in pages(c.is_some(), None) {
            let (status, html, _) = fetch(&app, &page, c).await;
            assert_eq!(status, StatusCode::OK, "{who} {page}");
            let doc = Html::parse_document(&html);
            let n = doc.select(&primary).count();
            seen += 1;
            if n > 1 {
                let all: Vec<String> = doc.select(&primary).map(|e| describe(&e)).collect();
                over.push(format!("{who} {page}: {n} primaries: {all:?}"));
            }
        }
    }
    assert!(seen > 40, "the walk's pages, both ways: {seen}");
    assert!(over.is_empty(), "{}", over.join("\n"));
}

/// The river's controls are secondaries and its record's "edit" a ghost; the
/// river has no primary (B2).
#[tokio::test]
async fn the_river_controls_are_secondary_and_edit_is_a_ghost() {
    let (app, _s) = app();
    let cookie = admin_cookie(&app).await;
    let (_, html, _) = fetch(&app, &format!("/?p={P}&n=3"), Some(&cookie)).await;
    let doc = Html::parse_document(&html);
    let sel = |s: &str| Selector::parse(s).unwrap();
    // The range is a select, travel/cut a setting and the grid a section of
    // the masthead now (the minimalism pass); what is left is buttons.
    for (what, s) in [("the itinerary chips", ".rv-trail a"), ("back", "#rv-back")] {
        let found: Vec<_> = doc.select(&sel(s)).collect();
        assert!(!found.is_empty(), "{what}: none on the page");
        for el in found {
            let class = el.value().attr("class").unwrap_or("");
            assert!(
                class.split_whitespace().any(|c| c == "btn")
                    && !class.contains("primary")
                    && !class.contains("ghost"),
                "{what} is a secondary button: {}",
                describe(&el)
            );
        }
    }
    let edit: Vec<_> = doc
        .select(&sel(&format!(r#"a[href^="/admin/person/{P}/edit"]"#)))
        .collect();
    assert!(!edit.is_empty(), "the record's edit link");
    for el in edit {
        assert!(
            el.value().attr("class").unwrap_or("").contains("ghost"),
            "{}",
            describe(&el)
        );
    }
    assert_eq!(
        doc.select(&sel(".btn.primary")).count(),
        0,
        "no primary on the river"
    );
}

/// Every field has a label a reader can see, and none relies on a
/// placeholder (B3). A label is one that wraps the field, names it by `for`
/// or by `aria-labelledby` pointing at text on the page, the column header
/// of a row editor, or — for a date — the date's own label above the box its
/// qualifier and year share. The masthead's language select shows its value as a code and
/// is named for assistive technology; it is the one field without a label
/// on screen, and is named here.
#[tokio::test]
async fn every_field_has_a_visible_label_and_none_a_placeholder() {
    let (app, _s) = app();
    let cookie = admin_cookie(&app).await;
    let fields = Selector::parse(
        "input:not([type=hidden]):not([type=submit]):not([type=checkbox]):not([type=radio]):not([type=range]), select, textarea",
    )
    .unwrap();
    let labels = Selector::parse("label[for]").unwrap();
    let header = Selector::parse("thead th").unwrap();
    let mut bare = Vec::new();
    let mut seen = 0;
    for (who, c) in [("signed out", None), ("signed in", Some(cookie.as_str()))] {
        for page in pages(c.is_some(), None) {
            let (_, html, _) = fetch(&app, &page, c).await;
            assert!(
                !html.contains(" placeholder="),
                "{who} {page}: a placeholder"
            );
            let doc = Html::parse_document(&html);
            let named: BTreeSet<String> = doc
                .select(&labels)
                .filter_map(|l| l.value().attr("for").map(String::from))
                .collect();
            for f in doc.select(&fields) {
                seen += 1;
                let wrapped = f.ancestors().filter_map(ElementRef::wrap).any(|a| {
                    let v = a.value();
                    v.name() == "label"
                        // A row editor: the column's header names the field.
                        || (v.name() == "table" && a.select(&header).next().is_some())
                        || v.attr("class").is_some_and(|c| {
                            c.split_whitespace()
                                .any(|c| c == "date-field" || c == "date-input" || c == "lang-form")
                        })
                });
                let by_for = f.value().attr("id").is_some_and(|id| named.contains(id));
                // Named by text on the page: a legend, a heading.
                let by_ref = f.value().attr("aria-labelledby").is_some_and(|ids| {
                    ids.split_whitespace().all(|id| {
                        doc.select(&Selector::parse(&format!("#{id}")).unwrap())
                            .next()
                            .is_some_and(|e| !e.text().collect::<String>().trim().is_empty())
                    })
                });
                if !wrapped && !by_for && !by_ref {
                    bare.push(format!("{who} {page}: {}", describe(&f)));
                }
            }
        }
    }
    assert!(seen > 100, "fields walked: {seen}");
    bare.sort();
    bare.dedup();
    assert!(
        bare.is_empty(),
        "{} field(s) with no visible label:\n{}",
        bare.len(),
        bare.join("\n")
    );
}

// ---------------------------------------------------------------------------
// The chrome budget (minimalism pass)
// ---------------------------------------------------------------------------

/// Whether `el` is drawn when the page has just loaded and nobody has opened,
/// hovered or typed anything: not `hidden`, not inside a closed `<details>`
/// (a disclosure's own summary is drawn), not screen-reader-only, and not
/// something ui.js hides on load (`data-js-hidden`).
fn at_rest(el: &ElementRef) -> bool {
    if el.value().attr("type") == Some("hidden") {
        return false;
    }
    let mut first = true;
    for a in std::iter::once(*el).chain(el.ancestors().filter_map(ElementRef::wrap)) {
        let v = a.value();
        if v.attr("hidden").is_some() || v.attr("data-js-hidden").is_some() {
            return false;
        }
        if v.attr("class")
            .is_some_and(|c| c.split_whitespace().any(|c| c == "sr-only"))
        {
            return false;
        }
        if matches!(v.name(), "template" | "noscript" | "datalist") {
            return false;
        }
        if v.name() == "details" && v.attr("open").is_none() && !first {
            // Inside a closed disclosure — unless this is its summary.
            let is_own_summary = el.value().name() == "summary"
                && el
                    .parent()
                    .and_then(ElementRef::wrap)
                    .is_some_and(|p| p.id() == a.id());
            if !is_own_summary {
                return false;
            }
        }
        first = false;
    }
    true
}

/// A content control: one per data item, inside a container the template
/// marks `data-content` (the river's drawing, a list's rows, a record's
/// relatives, search results). Its count follows the data and is not budgeted.
fn is_content(el: &ElementRef) -> bool {
    el.ancestors()
        .filter_map(ElementRef::wrap)
        .any(|a| a.value().attr("data-content").is_some())
}

/// Read-only screens: 12 chrome controls at most, the masthead 7 of them.
/// Everything else (edit screens, administration, settings, import, sign-in):
/// 20.
fn read_only(page: &str) -> bool {
    !(page.starts_with("/admin") || page.starts_with("/settings") || page.starts_with("/convert"))
}

struct Chrome {
    total: usize,
    masthead: usize,
    /// The site footer's own: its one link, the acknowledgement the format's
    /// licence asks for (v0.5.0). Counted apart, like the masthead, and capped.
    footer: usize,
    rows_over: Vec<String>,
    /// What was counted, outside the masthead, for a failure message.
    counted: Vec<String>,
}

fn chrome_of(html: &str) -> Chrome {
    let doc = Html::parse_document(html);
    let sel = interactive();
    let mut total = 0;
    let mut masthead = 0;
    let mut footer = 0;
    let mut counted = Vec::new();
    for el in doc.select(&sel) {
        if !at_rest(&el) || is_content(&el) {
            continue;
        }
        total += 1;
        if el
            .ancestors()
            .filter_map(ElementRef::wrap)
            .any(|a| a.value().name() == "header" && a.value().attr("class") == Some("masthead"))
        {
            masthead += 1;
        } else if el
            .ancestors()
            .filter_map(ElementRef::wrap)
            .any(|a| a.value().name() == "footer" && a.value().attr("class") == Some("site-footer"))
        {
            footer += 1;
        } else {
            counted.push(describe(&el));
        }
    }
    // A content row carries at most two controls.
    let rows =
        Selector::parse("[data-content] > tr, [data-content] > li, [data-content] tbody > tr")
            .unwrap();
    let mut rows_over = Vec::new();
    for row in doc.select(&rows) {
        let n = row.select(&sel).filter(at_rest).count();
        if n > 2 {
            rows_over.push(describe(&row));
        }
    }
    Chrome {
        total,
        masthead,
        footer,
        rows_over,
        counted,
    }
}

/// The record's editor, tab by tab: each tab's chrome is what lies outside
/// every tab plus what lies in it. Empty for any other page.
fn editor_tabs(html: &str) -> Vec<(String, usize)> {
    let doc = Html::parse_document(html);
    let panels = Selector::parse("section.ed-panel").unwrap();
    let sel = interactive();
    let in_panel = |el: &ElementRef| -> Option<String> {
        el.ancestors()
            .filter_map(ElementRef::wrap)
            .find(|a| {
                a.value().name() == "section"
                    && a.value()
                        .attr("class")
                        .is_some_and(|c| c.split_whitespace().any(|c| c == "ed-panel"))
            })
            .and_then(|a| a.value().id().map(str::to_string))
    };
    let mut outside = 0;
    let mut inside: BTreeMap<String, usize> = BTreeMap::new();
    for el in doc.select(&sel) {
        if !at_rest(&el) || is_content(&el) {
            continue;
        }
        match in_panel(&el) {
            Some(p) => *inside.entry(p).or_default() += 1,
            None => outside += 1,
        }
    }
    doc.select(&panels)
        .filter_map(|p| p.value().id().map(str::to_string))
        .map(|key| {
            let n = outside + inside.get(&key).copied().unwrap_or(0);
            (key, n)
        })
        .collect()
}

/// The person editor's profile groups (`details.pf-group`): how many, and
/// whether each is one control at rest — its summary — and never one per
/// field. A group's fields are the record's content.
fn profile_group_controls(html: &str, over: &mut Vec<String>, at: &str) -> usize {
    let doc = Html::parse_document(html);
    let groups = Selector::parse("details.pf-group").unwrap();
    let sel = interactive();
    let mut n = 0;
    for g in doc.select(&groups) {
        n += 1;
        let chrome = g
            .select(&sel)
            .filter(|el| at_rest(el) && !is_content(el))
            .count();
        if chrome != 1 {
            over.push(format!(
                "{at}: a profile group carries {chrome} chrome controls, not one: {}",
                describe(&g)
            ));
        }
    }
    let most = axgf_cms::profile::groups().len();
    if n > most {
        over.push(format!("{at}: {n} profile groups, the registry has {most}"));
    }
    n.min(most)
}

/// The masthead's cap: ten since v0.4.3 — four sections, the search, the
/// language, the theme and the density, the settings, and the account. The
/// three in the middle are the accessibility controls, and a menu hid them
/// from the reader who most needs them (docs/adr/0002 §9).
const MASTHEAD: usize = 10;

/// What a page may add of its own, beside the masthead: unchanged by v0.4.3,
/// which raised the masthead's cap and nobody else's. These are the old
/// budgets (12 for a read-only screen, 20 for an edit screen) with the old
/// masthead's seven taken out.
const OWN_READ: usize = 5;
/// The footer's cap: the format's link (v0.5.0), on every page. Before it the
/// footer was plain text, and one more link there is one more on every screen,
/// so it is held to that one.
const FOOTER: usize = 1;
const OWN_EDIT: usize = 13;

#[tokio::test]
async fn every_screen_keeps_its_chrome_budget() {
    let (app, _s) = app();
    let cookie = admin_cookie(&app).await;
    let mut over = Vec::new();
    let mut out_counts: BTreeMap<String, usize> = BTreeMap::new();
    println!(
        "{:<44} {:>6} {:>9} {:>6}",
        "screen", "own", "masthead", "budget"
    );
    for (who, c) in [("signed out", None), ("signed in", Some(cookie.as_str()))] {
        for page in pages(c.is_some(), None) {
            let (status, html, _) = fetch(&app, &page, c).await;
            if status != StatusCode::OK {
                continue;
            }
            let ch = chrome_of(&html);
            // The record's editor (v0.4.2) is six tabs over one form: each tab
            // is a screen of its own and keeps the edit budget on its own —
            // the controls outside every tab plus its own. Its Profil tab
            // names every AXGF profile group, always: one disclosure per
            // group on top, and only one.
            let tabs = editor_tabs(&html);
            if !tabs.is_empty() {
                let groups = profile_group_controls(&html, &mut over, &format!("{who} {page}"));
                for (key, n) in &tabs {
                    let own = n - ch.masthead - ch.footer;
                    let budget = OWN_EDIT + if key == "profil" { groups } else { 0 };
                    println!(
                        "{:<44} {:>6} {:>9} {:>6}",
                        format!("{who} {page}#{key}"),
                        own,
                        ch.masthead,
                        budget
                    );
                    if own > budget {
                        over.push(format!(
                            "{who} {page}#{key}: {own} chrome controls of its own, budget {budget}"
                        ));
                    }
                }
                if ch.masthead > MASTHEAD {
                    over.push(format!(
                        "{who} {page}: masthead {} controls, budget {MASTHEAD}",
                        ch.masthead
                    ));
                }
                continue;
            }
            let groups = profile_group_controls(&html, &mut over, &format!("{who} {page}"));
            let own = ch.total - ch.masthead - ch.footer;
            let budget = if read_only(&page) { OWN_READ } else { OWN_EDIT } + groups;
            println!(
                "{:<44} {:>6} {:>9} {:>6}",
                format!("{who} {page}"),
                own,
                ch.masthead,
                budget
            );
            if own > budget {
                over.push(format!(
                    "{who} {page}: {own} chrome controls of its own, budget {budget}:\n    {}",
                    ch.counted.join("\n    ")
                ));
            }
            if ch.masthead > MASTHEAD {
                over.push(format!(
                    "{who} {page}: masthead {} controls, budget {MASTHEAD}",
                    ch.masthead
                ));
            }
            if ch.footer > FOOTER {
                over.push(format!(
                    "{who} {page}: footer {} controls, budget {FOOTER}",
                    ch.footer
                ));
            }
            for r in &ch.rows_over {
                over.push(format!(
                    "{who} {page}: a row with more than two controls: {r}"
                ));
            }
            if c.is_none() {
                out_counts.insert(page.clone(), ch.total);
            } else if read_only(&page) {
                if let Some(&out) = out_counts.get(&page) {
                    if ch.total > out + 3 {
                        over.push(format!(
                            "{page}: signing in adds {} chrome controls, at most 3 ({out} → {})",
                            ch.total - out,
                            ch.total
                        ));
                    }
                }
            }
        }
    }
    assert!(
        over.is_empty(),
        "{} over budget:\n{}",
        over.len(),
        over.join("\n")
    );
}

/// A reader without the right to edit receives no edit control in the markup
/// of any page — not disabled, not hidden by CSS: absent (the minimalism
/// pass). An edit control is a form that writes (to `/admin/…`, other than
/// signing out, or to `/quality/…`) or a link into the administration.
#[tokio::test]
async fn a_reader_without_edit_rights_receives_no_edit_control_anywhere() {
    let src = scratch("walk-viewer-src");
    let source = src.join("walk.axgf");
    std::fs::write(
        &source,
        axgf_cms::state::export_to_bytes(&bundle().to_string()).expect("export"),
    )
    .expect("write");
    let dir = scratch("walk-viewer");
    let path = dir.join("family.axgf");
    std::fs::copy(&source, &path).expect("copy");
    let mut acl = axgf_cms::acl::Acl::default();
    acl.users.push(
        axgf_cms::acl::new_user(
            "reader",
            "correct-horse-battery-staple",
            axgf_cms::acl::Role::Viewer,
        )
        .expect("a viewer"),
    );
    acl.save(&axgf_cms::acl::Acl::path_for(&path))
        .expect("save acl");
    let app = build_app(&path).expect("build app");
    let login = post_form(
        &app,
        "/admin/login",
        "username=reader&password=correct-horse-battery-staple",
        false,
    )
    .await;
    let cookie = login
        .headers()
        .get(header::SET_COOKIE)
        .and_then(|v| v.to_str().ok())
        .and_then(|c| c.split(';').next())
        .expect("the viewer signs in")
        .to_string();

    let forms = Selector::parse("form").unwrap();
    let links = Selector::parse("a[href]").unwrap();
    let mut found = Vec::new();
    let mut read = 0;
    for page in pages(true, None) {
        let (status, html, _) = fetch(&app, &page, Some(&cookie)).await;
        if status != StatusCode::OK {
            continue; // the administration refuses a viewer outright
        }
        read += 1;
        let doc = Html::parse_document(&html);
        for f in doc.select(&forms) {
            let action = f.value().attr("action").unwrap_or("");
            let method = f
                .value()
                .attr("method")
                .unwrap_or("get")
                .to_ascii_lowercase();
            let writes = (action.starts_with("/admin/") && action != "/admin/logout")
                || action.starts_with("/quality/");
            if method == "post" && writes {
                found.push(format!("{page}: a form that writes, to {action}"));
            }
        }
        for a in doc.select(&links) {
            let href = a.value().attr("href").unwrap_or("");
            if href.starts_with("/admin/") || href == "/admin" {
                found.push(format!("{page}: a link into the administration, {href}"));
            }
        }
    }
    assert!(read >= 10, "the viewer reads the read-only pages: {read}");
    assert!(found.is_empty(), "{}", found.join("\n"));
}
