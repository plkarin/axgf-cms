//! The two themes that bring a view of their own (docs/adr/0007): Fusion's
//! living tree and Galaxy's solar system. The river's page under them loads
//! the view's script before river.js and not the river's scene, carries the
//! view's dashboard, and words what it says about the drawing for the drawing
//! on screen: no river where a tree or a sky is drawn, on that page or on any
//! page that names it.

mod common;

use axum::http::{header, StatusCode};
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

/// Three generations: roots, a trunk and a crown, or a sun with a world on
/// either side of it.
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
        "manifest": {"axgf": "1.1", "created_at": "2026-10-19T00:00:00Z"},
        "persons": persons, "families": families,
        "events": {}, "places": {}, "occupations": {}, "links": {},
        "sources": {}, "documents": {}
    });
    let src = scratch("views-src");
    let path = src.join("views.axgf");
    std::fs::write(
        &path,
        axgf_cms::state::export_to_bytes(&flat.to_string()).expect("export"),
    )
    .expect("write");
    let out = app_with_bundle("views", &path);
    drop(src);
    out
}

/// A page as a reader who chose `theme` reads it, in English.
async fn page(app: &axum::Router, path: &str, theme: &str) -> String {
    let cookie = format!("axgf_theme={theme}; axgf_lang=en");
    let resp = get_with_cookie(app, path, &cookie).await;
    assert_eq!(resp.status(), StatusCode::OK, "{path} under {theme}");
    body_string(resp).await
}

/// `#` and three to eight hex digits, ending there: a colour literal.
fn colours(text: &str) -> Vec<String> {
    let b = text.as_bytes();
    let mut out = Vec::new();
    for (i, _) in text.match_indices('#') {
        let n = b[i + 1..]
            .iter()
            .take_while(|c| c.is_ascii_hexdigit())
            .count();
        let end = b.get(i + 1 + n).copied().unwrap_or(b' ');
        if (3..=8).contains(&n) && !(end.is_ascii_alphanumeric() || end == b'_' || end == b'-') {
            out.push(text[i..i + 1 + n].to_string());
        }
    }
    out
}

#[tokio::test]
async fn each_theme_loads_the_view_it_brings_and_no_other() {
    let (app, _s) = app();
    for theme in axgf_cms::theme::THEMES.iter().map(|t| t.id) {
        let html = page(&app, &format!("/?p={P}"), theme).await;
        let view = axgf_cms::theme::view_of(theme);
        assert!(
            html.contains(&format!(r#"data-view="{view}""#)),
            "{theme}: the page says which drawing it carries"
        );
        let at = |src: &str| html.find(&format!(r#"<script src="{src}" defer></script>"#));
        let river = at("/static/river.js").expect("river.js, under every theme");
        let (fusion, galaxy, summit, clarity, scene) = (
            at("/static/fusion.js"),
            at("/static/galaxy.js"),
            at("/static/summit.js"),
            at("/static/clarity.js"),
            at("/static/river-scene.js"),
        );
        let genealogy = at("/static/genealogy.js");
        match view {
            "tree" => {
                // Deferred scripts run in order: the view registers itself
                // before river.js looks for it.
                assert!(fusion.expect("fusion.js") < river, "{theme}");
                assert!(
                    galaxy.is_none() && scene.is_none() && summit.is_none() && clarity.is_none(),
                    "{theme}"
                );
            }
            "galaxy" => {
                assert!(galaxy.expect("galaxy.js") < river, "{theme}");
                assert!(
                    fusion.is_none() && scene.is_none() && summit.is_none() && clarity.is_none(),
                    "{theme}"
                );
            }
            "summit" => {
                assert!(summit.expect("summit.js") < river, "{theme}");
                assert!(
                    fusion.is_none() && galaxy.is_none() && scene.is_none() && clarity.is_none(),
                    "{theme}"
                );
            }
            "funnel" => {
                assert!(clarity.expect("clarity.js") < river, "{theme}");
                assert!(
                    fusion.is_none() && galaxy.is_none() && scene.is_none() && summit.is_none(),
                    "{theme}"
                );
            }
            "scroll" => {
                assert!(genealogy.expect("genealogy.js") < river, "{theme}");
                assert!(
                    fusion.is_none()
                        && galaxy.is_none()
                        && scene.is_none()
                        && summit.is_none()
                        && clarity.is_none(),
                    "{theme}"
                );
            }
            _ => {
                assert!(
                    fusion.is_none() && galaxy.is_none() && summit.is_none() && clarity.is_none(),
                    "{theme}"
                );
                assert!(scene.expect("the river's own scene") > river, "{theme}");
            }
        }
        // The dashboard is a view's, between its controls and its drawing;
        // the river shows none. The whole family is a view's control, hidden
        // until its script shows it.
        assert_eq!(html.contains("data-rv-hud"), view != "river", "{theme}");
        assert!(
            html.contains(r#"aria-pressed="false" data-rv-whole hidden>Whole family</button>"#),
            "{theme}"
        );
    }
}

#[tokio::test]
async fn no_river_is_named_where_a_tree_or_a_sky_is_drawn() {
    let (app, _s) = app();
    // (theme, its pause, its compass, its overview's two lines, the link a
    // record gives to it, what Enter does in the masthead's search)
    let cases = [
        (
            "canopee",
            "Hold the landscape still",
            "↓ toward the source · ↑ toward the living",
            ["Every life is a source.", "Every family, a river."],
            "See in the river",
            "Enter travels the river to them",
        ),
        (
            "fusion",
            "Hold the tree still",
            "↓ the roots: ancestors · ↑ the crown: descendants",
            ["Every life is a leaf.", "Every family, a tree."],
            "See in the tree",
            "Enter grows the tree around them",
        ),
        (
            "galaxy",
            "Hold the sky still",
            "↓ ancestors below the sun · ↑ descendants above",
            ["Every life is a star.", "Every family, a galaxy."],
            "See in the galaxy",
            "Enter makes them the sun",
        ),
        (
            "summit",
            "Hold the mountain still",
            "↓ ancestors in the valley · ↑ descendants up toward the summit",
            ["Every life is a step.", "Every family, an ascent."],
            "See on the mountain",
            "Enter takes you to them",
        ),
        (
            "clarity",
            "Hold the funnel still",
            "↓ the oldest at the narrow end · ↑ the newest at the mouth",
            ["Every generation is a ring.", "Every family, a funnel."],
            "See in the funnel",
            "Enter brings them to the front",
        ),
        (
            "genealogy",
            "Hold the leaves still",
            "↓ toward the roots: ancestors · ↑ toward the crown: descendants",
            [
                "Every life is a portrait.",
                "Every family, a tree to climb.",
            ],
            "See in the family tree",
            "Enter sets them on the trunk",
        ),
    ];
    for (theme, still, compass, hero, see, hint) in cases {
        let river = page(&app, &format!("/?p={P}"), theme).await;
        assert!(
            river.contains(&format!("{still}</button>")),
            "{theme}: {still}"
        );
        assert!(river.contains(compass), "{theme}: {compass}");
        assert!(river.contains(hint), "{theme}: {hint}");
        // A river's width is its discharge and its height is time: keys to
        // what a view does not draw are the river's alone.
        let river_only = theme == "canopee";
        assert_eq!(
            river.contains(r#"class="rv-widths""#),
            river_only,
            "{theme}"
        );
        assert_eq!(river.contains("data-rv-scale"), river_only, "{theme}");
        let about = page(&app, "/about", theme).await;
        for line in hero {
            assert!(about.contains(line), "{theme}: {line}");
        }
        let record = page(&app, &format!("/person/{P}"), theme).await;
        assert!(record.contains(&format!("{see}</a>")), "{theme}: {see}");
    }
}

#[tokio::test]
async fn the_views_scripts_and_pictures_are_served_and_name_no_colour() {
    let (app, _s) = app_with_empty_bundle("views-assets");
    for js in [
        "/static/fusion.js",
        "/static/galaxy.js",
        "/static/summit.js",
        "/static/clarity.js",
        "/static/genealogy.js",
    ] {
        let resp = get(&app, js).await;
        assert_eq!(resp.status(), StatusCode::OK, "{js}");
        let kind = resp.headers()[header::CONTENT_TYPE]
            .to_str()
            .expect("ascii")
            .to_string();
        assert!(kind.starts_with("text/javascript"), "{js}: {kind}");
        // Every colour of a view is a token of its theme (tokens.css).
        let found = colours(&body_string(resp).await);
        assert!(found.is_empty(), "{js} names colours: {found:?}");
    }
    for jpg in [
        "/static/img/fusion-hero.jpg",
        "/static/img/fusion-cta.jpg",
        "/static/img/fusion-lab.jpg",
        "/static/img/galaxy-hero.jpg",
        "/static/img/galaxy-cta.jpg",
        "/static/img/summit-hero.jpg",
        "/static/img/summit-cta.jpg",
        "/static/img/summit-panorama.jpg",
        "/static/img/clarity-hero.jpg",
        "/static/img/clarity-cta.jpg",
        "/static/img/genealogy-hero.jpg",
        "/static/img/genealogy-cta.jpg",
        "/static/img/genealogy-vine-l.jpg",
        "/static/img/genealogy-vine-r.jpg",
        "/static/img/notes-genealogy.jpg",
    ] {
        let resp = get(&app, jpg).await;
        assert_eq!(resp.status(), StatusCode::OK, "{jpg}");
        assert_eq!(resp.headers()[header::CONTENT_TYPE], "image/jpeg", "{jpg}");
        assert!(
            body_bytes(resp).await.starts_with(&[0xFF, 0xD8, 0xFF]),
            "{jpg} is a JPEG"
        );
    }
}

/// What river.js asks of a view, each view gives: where everyone stands, how
/// a line bends (the lit route follows it), how a frame is painted, the
/// dashboard at rest, and whom the drawing named — so that the pointer names
/// those it left out, and only those. And each view's scene holds still on
/// the river's terms.
#[test]
fn a_view_gives_river_js_everything_it_asks_for() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let read = |f: &str| std::fs::read_to_string(root.join(f)).expect(f);
    for (file, name) in [
        ("static/fusion.js", "tree"),
        ("static/galaxy.js", "galaxy"),
        ("static/summit.js", "summit"),
        ("static/clarity.js", "funnel"),
        ("static/genealogy.js", "scroll"),
    ] {
        let js = read(file);
        assert!(
            js.contains(&format!(
                "window.axgfView = {{ name: '{name}', shape: shape, edge: edge, paint: paint, rest: rest, named: function () {{ return named; }} }};"
            )),
            "{file}"
        );
        assert!(
            js.contains(&format!("root.dataset.view !== '{name}'")),
            "{file}: only under its own theme"
        );
        for still in [
            "matchMedia('(prefers-reduced-motion: reduce)')",
            "root.dataset.riverMotion === 'cut'",
            "var STORE = 'axgf-river-still'",
            "document.addEventListener('visibilitychange', request);",
            "tired = true",
        ] {
            assert!(js.contains(still), "{file}: {still}");
        }
    }
    let river = read("static/river.js");
    assert!(river.contains("T = view && view.named ? view.named() : tiers(fr);"));
    assert!(river.contains("var curve = view && view.edge ? view.edge : vc;"));
}

#[test]
fn the_colour_scanner_finds_colours_and_leaves_ids_alone() {
    assert_eq!(colours("fill:#fff; stroke:#a1b2c3"), ["#fff", "#a1b2c3"]);
    assert!(colours("url(#tv-soil) '#gx-neb-a' charAt(0) !== '#' #deadline").is_empty());
}

/// Fusion's tree stands in a glass that turns as the reader travels, over
/// its laboratory's picture (2026-10-23); Clarity's counts and the lines
/// beyond its last slice lie over the glass, not under it.
#[test]
fn the_glass_turns_and_the_funnel_draws_its_counts_over_the_glass() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let read = |f: &str| std::fs::read_to_string(root.join(f)).expect(f);
    let fusion = read("static/fusion.js");
    assert!(
        fusion.contains("window.axgfView.tween = tween3;"),
        "Fusion blends frames in its glass"
    );
    assert!(
        fusion.contains("'/static/img/fusion-lab.jpg?v=1'"),
        "over its laboratory"
    );
    assert!(
        fusion.contains("ground.className = 'rv-ground';"),
        "with a room that moves under the drawing"
    );
    let river = read("static/river.js");
    assert!(
        river.contains(
            "view && view.tween ? view.tween(A, B, ease(t), tween) : tween(A, B, ease(t))"
        ),
        "river.js lets a view blend two frames from its own blend"
    );
    let clarity = read("static/clarity.js");
    let order = clarity
        .find("out + base + bands + beam + figs + tails + links + '<g class=\"rv-lit\"></g>' + unions + marks + nodes")
        .is_some();
    assert!(order, "the funnel's tails and counts over the glass");
    assert!(
        clarity.contains("ground.className = 'rv-ground';"),
        "and its laboratory under it"
    );
}

/// Names (2026-10-23): one button, under every view and the river alike,
/// asks for every name or only the near generations' (−1, 0, +1), and the
/// reader's choice is remembered; Fusion lays its tree out in tiers.
#[test]
fn a_reader_asks_for_every_name_or_the_near_ones() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let read = |f: &str| std::fs::read_to_string(root.join(f)).expect(f);
    let tpl = read("templates/river.html");
    assert!(tpl.contains(r#"data-rv-names hidden>{{ t("river-names-all") }}</button>"#));
    let river = read("static/river.js");
    assert!(river.contains("namesAll = localStorage.getItem(NAMES) === 'all';"));
    assert!(
        river.contains("var p = PK[a.getAttribute('data-key')], far = p && Math.abs(p.gen) > 1;")
    );
    assert!(river.contains("g.setAttribute('class', 'rv-names');"));
    let css = read("static/app.css");
    assert!(css.contains(".river.rv-names-near .rv-svg a.rv-p[data-far] :is(text, .tv-panel, .tv-leader) { display: none; }"));
    let fusion = read("static/fusion.js");
    assert!(
        fusion.contains("function crown(key, left)")
            && fusion.contains("function roots(key, left)"),
        "a tree in tiers"
    );
}

/// Genealogy's tree, climbed (docs/adr/0016, redrawn 2026-10-23): the family
/// in levels round a trunk drawn in perspective, with no picture behind it;
/// travelling, the camera climbs the trunk to the person travelled to (or
/// goes down it), blended in the tree's own space; a card counts as a placed
/// word for "All names", and a portrait's initials are not a name; a far
/// person keeps their portrait while only the near generations are named.
#[test]
fn the_family_stands_on_a_tree_the_reader_climbs() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let read = |f: &str| std::fs::read_to_string(root.join(f)).expect(f);
    let js = read("static/genealogy.js");
    assert!(js.contains("function crown(key, left)") && js.contains("function roots(key, left)"));
    assert!(js.contains("window.axgfView.tween = tween3;"));
    assert!(
        js.contains("var lc = to ? to.lv : 0, cam = lc * e;"),
        "the camera rises (or goes down) to the person travelled to"
    );
    assert!(js.contains("return proj(G, th, w - cam);"));
    assert!(
        !js.contains("/static/img/"),
        "the tree is drawn, not a picture"
    );
    assert!(js.contains("ground.className = 'rv-ground';"));
    let river = read("static/river.js");
    assert!(river.contains("svg.querySelectorAll('text, rect.tv-panel, rect.gn-card')"));
    assert!(river.contains("a.querySelector('text:not(.gn-initials)')"));
    let css = read("static/app.css");
    assert!(
        css.contains(".river.rv-names-near .rv-svg a.rv-p[data-far] .gn-plate { display: none; }")
    );
    assert!(css.contains(
        ".river.rv-names-near .rv-svg a.rv-p[data-far] text.gn-initials { display: inline; }"
    ));
    assert!(css.contains(
        "[data-theme=\"genealogy\"]:not([data-style=\"paper\"]) :is(.card, .rec-section,"
    ));
}

/// Summit's sun follows the climb: a step up toward the descendants raises
/// it, a step down toward the ancestors lowers it (the operator's review,
/// 2026-10-23). It used to count the itinerary's people, so that going down
/// raised it too and, after six, it stood still.
#[test]
fn the_sun_goes_down_with_the_reader() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let summit = std::fs::read_to_string(root.join("static/summit.js")).expect("summit.js");
    assert!(
        summit.contains("function shape(fr) {\n    track(fr);"),
        "every layout tells the sun where the reader went"
    );
    assert!(summit.contains("return Math.max(0.12, Math.min(1, 0.42 + climbed * 0.15));"));
    assert!(
        summit.contains("climbed = Math.max(-2, Math.min(4, climbed + (d || 0)));"),
        "within what the sky can show, so the first step back answers"
    );
    assert!(
        !summit.contains("rv-trail"),
        "the itinerary's length no longer moves the sun"
    );
}
