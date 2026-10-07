//! The river: `/`, its travel endpoint, its SVG, and what it must never show.

mod common;

use axgf_cms::access::Lens;
use axgf_cms::acl::Visibility;
use axgf_cms::river::{self, Graph, Shape, Words};
use axum::http::StatusCode;
use common::*;
use serde_json::{json, Value};

/// Something written into every sensitive class of one person. None of it
/// may reach any river surface.
const SECRET: &str = "ZZSECRETZZ";

fn person(
    id: &str,
    given: &str,
    surname: &str,
    g: &str,
    born: Option<i64>,
    died: Option<i64>,
    living: bool,
) -> Value {
    let mut p = json!({"id": id, "type": "person", "axgf_version": "1.1", "version_num": 1,
        "identity": {"name": {"display": format!("{given} {surname}"),
            "components": [{"type": "given_name", "value": given},
                           {"type": "family_name", "value": surname}]},
            "gender": {"value": g}, "is_living": living, "visibility": "public"}});
    if let Some(b) = born {
        p["birth"] = json!({"date": {"value": b.to_string(), "precision": "year"}});
    }
    if let Some(d) = died {
        p["death"] = json!({"date": {"value": d.to_string(), "precision": "year"}});
    }
    p
}

fn family(id: &str, parents: &[&str], kids: &[(&str, f64)]) -> Value {
    json!({"id": id, "type": "family", "axgf_version": "1.1", "version_num": 1,
        "union": {"type": "marriage",
            "persons": parents.iter().map(|p| json!({"person_id": p, "role": "spouse"})).collect::<Vec<_>>()},
        "children": kids.iter().map(|(k, c)| json!({"person_id": k, "confidence": c})).collect::<Vec<_>>()})
}

/// Twelve people over five generations, every confidence band, a thin record,
/// a living descendant, and one person whose every sensitive class holds
/// [`SECRET`] and who is hidden from visitors.
fn twelve() -> Value {
    let mut hidden = person("p11", "Hidden", "Living", "F", Some(1990), None, true);
    hidden["identity"]["visibility"] = json!("members");
    for (class, attr) in [
        ("health", "conditions"),
        ("biometrics", "verbal_tics"),
        ("genomics", "risk_variants"),
        ("legal", "criminal_record"),
    ] {
        hidden[class] = json!({attr: [{"value": SECRET, "confidence": 0.9}]});
    }
    hidden["death"] = json!({"cause": SECRET});
    let mut anna = person("p02", "Anna", "Lis", "F", Some(1806), None, false);
    anna["birth"]["date"]["circa"] = json!(true);
    let persons = vec![
        person("p01", "Jan", "Nowak", "M", Some(1800), Some(1870), false),
        anna,
        person("p03", "Antoni", "Nowak", "M", Some(1830), Some(1890), false),
        person("p04", "Zofia", "Mazur", "F", None, None, false),
        person("p05", "Józef", "Nowak", "M", Some(1858), Some(1923), false),
        person(
            "p06",
            "Marianne",
            "Lefèvre",
            "F",
            Some(1863),
            Some(1930),
            false,
        ),
        person(
            "p07",
            "Stanislas",
            "Nowak",
            "M",
            Some(1888),
            Some(1951),
            false,
        ),
        person("p08", "Hélène", "Mercier", "F", Some(1890), None, false),
        person("p09", "Michel", "Nowak", "M", Some(1914), Some(1988), false),
        person("p10", "Irène", "Nowak", "F", Some(1917), Some(1936), false),
        hidden,
        person("p12", "Paul", "Nowak", "M", Some(1950), None, true),
    ];
    let families = vec![
        family("f1", &["p01", "p02"], &[("p03", 0.96)]),
        family("f2", &["p03", "p04"], &[("p05", 0.55)]),
        family("f3", &["p05", "p06"], &[("p07", 0.98)]),
        family("f4", &["p07", "p08"], &[("p09", 0.95), ("p10", 0.3)]),
        family("f5", &["p09"], &[("p12", 0.8), ("p11", 0.9)]),
    ];
    json!({
        "manifest": {"axgf": "1.1", "created_at": "2026-09-01T00:00:00Z"},
        "persons": persons.into_iter().map(|p| (p["id"].as_str().unwrap().to_string(), p)).collect::<serde_json::Map<_, _>>(),
        "families": families.into_iter().map(|f| (f["id"].as_str().unwrap().to_string(), f)).collect::<serde_json::Map<_, _>>(),
        "events": {}, "places": {}, "sources": {}, "occupations": {}, "links": {}, "documents": {}
    })
}

fn app(tag: &str) -> (axum::Router, Scratch) {
    let src = scratch(&format!("{tag}-src"));
    let path = src.join("river.axgf");
    std::fs::write(
        &path,
        axgf_cms::state::export_to_bytes(&twelve().to_string()).expect("export"),
    )
    .expect("write");
    let out = app_with_bundle(tag, &path);
    drop(src);
    out
}

fn words() -> Words {
    Words {
        living_band: "VIVANTS".into(),
        circa: "v.".into(),
    }
}

#[tokio::test]
async fn the_river_is_drawn_on_the_server() {
    let (app, _d) = app("river-landing");
    let body = expect_status(get(&app, "/?p=p07").await, StatusCode::OK, "GET /").await;
    assert!(
        body.contains(r#"<svg class="rv-svg""#),
        "the SVG is in the first paint"
    );
    // One terminator, living or not, and the legend says what it means for
    // both: the record ends. "Line lost" described only the dead.
    assert!(body.contains("record ends") && !body.contains("line lost"));
    // Without JavaScript every person is a link that re-centres the river.
    assert!(body.contains(r#"href="/?p=p05&amp;n=3""#));
    // The range segment is three links.
    for n in [2, 3, 5] {
        assert!(body.contains(&format!(r#"data-range="{n}""#)));
    }
    // The grid keeps its route and a link from the river.
    assert!(body.contains(r#"href="/tree?root=p07""#));
    // The record panel is the existing partial.
    assert!(body.contains(r#"class="panel-inner" data-person="p07""#));
    assert!(body.contains(r#"id="rv-data""#));
    assert!(
        !body.contains("feGaussianBlur")
            && !body.contains("<filter")
            && !body.contains("clip-path")
    );
}

#[tokio::test]
async fn range_is_two_three_or_five_and_nothing_else() {
    let (app, _d) = app("river-range");
    let five = body_string(get(&app, "/?p=p07&n=5").await).await;
    assert!(five.contains(">+5</text>"));
    let odd = body_string(get(&app, "/?p=p07&n=9").await).await;
    assert!(
        odd.contains(">+3</text>") && !odd.contains(">+4</text>"),
        "an unknown range falls back to 3"
    );
    let unknown = expect_status(
        get(&app, "/?p=nobody").await,
        StatusCode::OK,
        "unknown centre",
    )
    .await;
    assert!(
        unknown.contains(r#"<svg class="rv-svg""#),
        "an unknown centre falls back to the default"
    );
}

#[tokio::test]
async fn the_travel_endpoint_returns_layout_and_svg() {
    let (app, _d) = app("river-data");
    let resp = get(&app, "/river/data?p=p09&n=2").await;
    assert_eq!(resp.status(), StatusCode::OK);
    let v: Value = serde_json::from_str(&body_string(resp).await).unwrap();
    assert_eq!(v["centre"], "p09");
    assert_eq!(v["river"]["layout"]["meta"]["n"], 2);
    assert!(v["svg"].as_str().unwrap().starts_with("<svg"));
    assert_eq!(v["river"]["down"], "p07", "↓ goes to the father");
    assert_eq!(v["river"]["up"], "p12", "↑ goes to the eldest child");
}

#[tokio::test]
async fn an_empty_archive_lands_on_the_overview() {
    let (app, _d) = app_with_empty_bundle("river-empty");
    let body = expect_status(get(&app, "/").await, StatusCode::OK, "GET /").await;
    assert!(body.contains("What the family has recorded so far"));
}

#[tokio::test]
async fn the_river_is_the_landing_page_and_the_overview_is_at_about() {
    let (app, _d) = app("river-landing-page");
    let body = expect_status(get(&app, "/").await, StatusCode::OK, "GET /").await;
    assert!(body.contains(r#"<svg class="rv-svg""#));
    let about = expect_status(get(&app, "/about").await, StatusCode::OK, "GET /about").await;
    assert!(about.contains("What the family has recorded so far"));
    assert!(!about.contains(r#"<svg class="rv-svg""#));
}

#[tokio::test]
async fn no_sensitive_class_reaches_any_river_surface() {
    let (app, _d) = app("river-classes");
    for who in [None, Some("admin")] {
        for uri in [
            "/?p=p09",
            "/?p=p11&n=5",
            "/river/data?p=p11",
            "/river/data?p=p09&n=5",
        ] {
            let resp = match who {
                Some(_) => get_admin(&app, uri).await,
                None => get(&app, uri).await,
            };
            let body = body_string(resp).await;
            // The page's record panel is the person partial, which has its own
            // class rules; the river's own surfaces are the SVG and payload.
            let river_part = match body.find(r#"<svg class="rv-svg""#) {
                Some(i) => {
                    let svg = &body[i..body[i..]
                        .find("</svg>")
                        .map(|j| i + j)
                        .unwrap_or(body.len())];
                    let data = body
                        .split(r#"id="rv-data">"#)
                        .nth(1)
                        .and_then(|s| s.split("</script>").next())
                        .unwrap_or("");
                    format!("{svg}{data}")
                }
                None => body.clone(),
            };
            assert!(
                !river_part.contains(SECRET),
                "{uri} as {who:?} carried class data"
            );
        }
    }
}

#[test]
fn the_river_reads_no_person_field_itself() {
    // Everything it knows about a person comes through access::river_shape
    // and access::river_label. A field read here would be the bug.
    let src = include_str!("../src/river.rs");
    let code = src.split("#[cfg(test)]").next().unwrap();
    for field in [
        "\"identity\"",
        "\"birth\"",
        "\"death\"",
        "\"health\"",
        "\"biometrics\"",
        "\"genomics\"",
        "\"legal\"",
        "\"morphology\"",
        "\"belief\"",
        "\"personality\"",
        "\"notes\"",
        "\"bio\"",
    ] {
        assert!(!code.contains(field), "river.rs reads {field} directly");
    }
}

#[test]
fn a_redacted_person_keeps_node_edges_and_width_and_nothing_else() {
    let flat = twelve();
    let g = Graph::build(&flat);
    let admin = Lens::unrestricted();
    let visitor = Lens::resolve(&flat, Visibility::Public);
    assert!(!visitor.sees_person("p11"));
    let a = river::build(&flat, &admin, &g, "p09", 3, Shape::default(), words(), true).unwrap();
    let v = river::build(
        &flat,
        &visitor,
        &g,
        "p09",
        3,
        Shape::default(),
        words(),
        true,
    )
    .unwrap();

    // The geometry is the same for both readers: a layout that moved with
    // the reader would leak the hidden person through the difference.
    let geo =
        |r: &river::River| serde_json::to_value((&r.layout.persons, &r.layout.couples)).unwrap();
    assert_eq!(geo(&a), geo(&v));

    let hid = v.shown.iter().find(|s| s.id == "p11").unwrap();
    assert!(hid.redacted && hid.names.iter().all(String::is_empty) && hid.years.is_empty());
    assert!(!hid.living, "no living halo either");
    let node = v.layout.person("p11").unwrap();
    assert_eq!(node.d, 1, "the width is still the descendant count");
    let svg = v.svg("river");
    assert!(!svg.contains("Hidden") && !svg.contains("1990"));
    let payload = serde_json::to_string(&v.payload()).unwrap();
    assert!(!payload.contains("Hidden") && !payload.contains("1990"));
    // Colour comes from the generation, not the record: the redacted person's
    // year is the centre's plus one generation.
    let i = v.layout.persons.iter().position(|p| p.id == "p11").unwrap();
    assert_eq!(hid.colour, river::colour_at(1914.0 + 29.0));
    assert_eq!(v.layout.persons[i].gen, 1);

    // A redacted centre lends the bands no year of its own.
    let c = river::build(
        &flat,
        &visitor,
        &g,
        "p11",
        2,
        Shape::default(),
        words(),
        true,
    )
    .unwrap();
    assert_ne!(c.layout.meta.year, 1990.0);
    assert!(!c.svg("river").contains("1990"));
}

/// The twelve-person river, as SVG, against `tests/fixtures/river-12.svg`.
///
/// `RIVER_SNAPSHOT_UPDATE=1` rewrites the file. Read the diff before
/// committing it: this is the drawing.
#[test]
fn the_twelve_person_river_matches_its_snapshot() {
    let flat = twelve();
    let g = Graph::build(&flat);
    let r = river::build(
        &flat,
        &Lens::unrestricted(),
        &g,
        "p07",
        3,
        Shape::default(),
        words(),
        true,
    )
    .unwrap();
    let svg = r.svg("Rivière autour de Stanislas Nowak");
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/river-12.svg");
    if std::env::var("RIVER_SNAPSHOT_UPDATE").is_ok() {
        std::fs::write(&path, &svg).unwrap();
    }
    let want = std::fs::read_to_string(&path).expect("snapshot; run with RIVER_SNAPSHOT_UPDATE=1");
    assert_eq!(svg, want, "the river's SVG changed");

    // The semantics the snapshot must carry, stated so a regenerated snapshot
    // cannot quietly lose them.
    assert!(svg.contains(r#"stroke-linecap="round""#), "attested");
    assert!(
        svg.contains("stroke-dasharray=\"4 3\"") || svg.contains("stroke-dasharray=\"6.5 3\""),
        "inferred"
    );
    assert!(
        svg.contains(r#"stroke-opacity="0.14""#) && svg.contains(r#"stroke-dasharray="0.1 5.5""#),
        "speculative"
    );
    assert!(
        svg.contains(r#"gradientUnits="userSpaceOnUse""#),
        "lost lines fade in user space"
    );
    assert!(
        svg.contains(r##"r="3.4" fill="#0c1310""##),
        "and end in an open ring"
    );
    assert!(svg.contains("VIVANTS"));
    // A recorded circa reads as one; a plain year does not.
    assert!(svg.contains(">v.1806<") && svg.contains(">1800–1870<"));
    for w in ["1.6", "3", "5"] {
        assert!(svg.contains(&format!(r#"stroke-width="{w}""#)));
    }
    // Every coordinate has at most one decimal: path data, centres, text
    // positions. (Opacities such as 0.14 are not coordinates.)
    let mut coords = Vec::new();
    for attr in [
        " d=\"",
        " cx=\"",
        " cy=\"",
        " x=\"",
        " y=\"",
        " x1=\"",
        " y1=\"",
        " x2=\"",
        " y2=\"",
        " points=\"",
    ] {
        for chunk in svg.split(attr).skip(1) {
            coords.push(chunk.split('"').next().unwrap().to_string());
        }
    }
    let re_two = coords.iter().all(|c| {
        c.split(|ch: char| !(ch.is_ascii_digit() || ch == '.'))
            .filter(|t| t.contains('.'))
            .all(|t| t.split('.').nth(1).is_none_or(|d| d.len() <= 1))
    });
    assert!(re_two, "a number with more than one decimal");
}

/// The fixture with everyone hidden from visitors.
fn all_hidden() -> Value {
    let mut flat = twelve();
    for p in flat["persons"].as_object_mut().unwrap().values_mut() {
        p["identity"]["visibility"] = json!("members");
    }
    flat
}

#[test]
fn a_lost_line_ends_the_same_way_whether_or_not_the_person_is_living() {
    // p10, Irène, is childless. Hidden from visitors, her line's ring must not
    // say whether she is alive.
    let visitor_view = |living: bool, centre: &str| {
        let mut flat = twelve();
        flat["persons"]["p10"]["identity"]["is_living"] = json!(living);
        flat["persons"]["p10"]["identity"]["visibility"] = json!("members");
        flat["persons"]["p10"]["death"] = Value::Null;
        let g = Graph::build(&flat);
        let lens = Lens::resolve(&flat, Visibility::Public);
        assert!(!lens.sees_person("p10"));
        let r = river::build(
            &flat,
            &lens,
            &g,
            centre,
            3,
            Shape::default(),
            words(),
            false,
        )
        .unwrap();
        (r.svg("river"), serde_json::to_string(&r.payload()).unwrap())
    };
    for centre in ["p07", "p10"] {
        assert_eq!(
            visitor_view(true, centre),
            visitor_view(false, centre),
            "centred on {centre}"
        );
    }
    // And the ring is there in both: a childless line ends in it.
    let flat = twelve();
    let g = Graph::build(&flat);
    let l = river::layout(&g, "p07", 3, Shape::default()).unwrap();
    assert!(l
        .tails
        .iter()
        .any(|t| t.key == "ep10" && t.kind == river::TailKind::Lost));
    assert!(
        l.tails
            .iter()
            .any(|t| t.key == "ep11" && t.kind == river::TailKind::Lost),
        "living too"
    );
}

#[test]
fn a_signed_out_visitor_gets_no_bands() {
    let flat = twelve();
    let g = Graph::build(&flat);
    let lens = Lens::resolve(&flat, Visibility::Public);
    let out = river::build(&flat, &lens, &g, "p07", 3, Shape::default(), words(), false).unwrap();
    let svg = out.svg("river");
    assert!(!out.layout.meta.bands);
    assert!(
        !svg.contains(r##"stroke="#18241f""##) && !svg.contains("VIVANTS"),
        "no band rules, no band names"
    );
    let inn = river::build(&flat, &lens, &g, "p07", 3, Shape::default(), words(), true).unwrap();
    assert!(inn.layout.meta.bands && inn.svg("river").contains(r##"stroke="#18241f""##));
}

#[tokio::test]
async fn the_signed_out_page_has_no_bands_and_the_signed_in_page_does() {
    let (app, _d) = app("river-bands");
    let anon = body_string(get(&app, "/?p=p07").await).await;
    assert!(!anon.contains(r##"stroke="#18241f""##));
    let admin = body_string(get_admin(&app, "/?p=p07").await).await;
    assert!(admin.contains(r##"stroke="#18241f""##));
}

#[test]
fn with_no_visible_year_the_river_has_no_era_at_all() {
    // Nobody visible: the year scale has nothing a reader may see to rest on.
    let flat = all_hidden();
    let g = Graph::build(&flat);
    let lens = Lens::resolve(&flat, Visibility::Public);
    for centre in ["p07", "p01", "p12"] {
        let r = river::build(&flat, &lens, &g, centre, 3, Shape::default(), words(), true).unwrap();
        assert!(!r.layout.meta.era && !r.layout.meta.bands, "{centre}");
        let svg = r.svg("river");
        let payload = serde_json::to_string(&r.payload()).unwrap();
        // No band, no year, and one colour.
        assert!(!svg.contains(r##"stroke="#18241f""##));
        for year in [
            "1800", "1830", "1858", "1888", "1914", "1950", "1990", "1900", "1700",
        ] {
            assert!(
                !svg.contains(year) && !payload.contains(year),
                "{centre}: {year} reached the page"
            );
        }
        assert!(r.shown.iter().all(|s| s.colour == river::NEUTRAL));
        for c in svg
            .split("stroke=\"#")
            .skip(1)
            .chain(svg.split("fill=\"#").skip(1))
        {
            let hex = &c[..6];
            assert!(
                ["86b08f", "0c1310", "dcae64", "8d9c92", "4f6158"].contains(&hex),
                "{centre}: a colour that is not the neutral ink or the chrome: #{hex}"
            );
        }
    }
}

#[test]
fn a_label_at_the_right_edge_shortens_instead_of_running_off_the_frame() {
    let s = river::Shown {
        id: "x".into(),
        names: [
            "Jarosław".into(),
            "Jarosław M.".into(),
            "Jarosław Medyński".into(),
        ],
        years: "1901–1980".into(),
        colour: String::new(),
        living: false,
        sparse: false,
        redacted: false,
    };
    assert_eq!(river::fit_edge(3, &s, 300.0, 4.8, false), 3, "room enough");
    assert_eq!(
        river::fit_edge(3, &s, 760.0, 4.8, false),
        2,
        "given name and initial fit"
    );
    assert_eq!(
        river::fit_edge(3, &s, 790.0, 4.8, false),
        1,
        "only the given name fits"
    );
    assert_eq!(river::fit_edge(3, &s, 850.0, 4.8, false), 0, "nothing fits");
    assert_eq!(
        river::fit_edge(3, &s, 850.0, 7.5, true),
        1,
        "the centre keeps at least its given name"
    );
}

/// A generated family: four generations, several children per couple, every
/// third person hidden from visitors. Siblings are deliberately recorded out
/// of birth order, and hidden and visible siblings share families.
fn generated() -> Value {
    let mut persons = Vec::new();
    let mut families = Vec::new();
    let mut next = 0usize;
    let mut mk = |persons: &mut Vec<Value>, born: i64, g: &str| {
        let id = format!("g{next:03}");
        let mut p = person(
            &id,
            &format!("Given{next}"),
            "Family",
            g,
            Some(born),
            Some(born + 60),
            false,
        );
        if next % 3 == 1 {
            p["identity"]["visibility"] = json!("members");
        }
        next += 1;
        persons.push(p);
        id
    };
    let a = mk(&mut persons, 1800, "M");
    let b = mk(&mut persons, 1803, "F");
    let mut parents = vec![(a, b)];
    for gen in 0..3 {
        let mut next_parents = Vec::new();
        for (fi, (fa, mo)) in parents.clone().into_iter().enumerate() {
            let base = 1830 + gen * 30;
            // Three children recorded youngest first.
            let kids: Vec<String> = (0..3)
                .rev()
                .map(|k| {
                    mk(
                        &mut persons,
                        base + k * 3,
                        if k % 2 == 0 { "M" } else { "F" },
                    )
                })
                .collect();
            families.push(family(
                &format!("fam{gen}{fi}"),
                &[&fa, &mo],
                &kids.iter().map(|k| (k.as_str(), 0.9)).collect::<Vec<_>>(),
            ));
            if gen < 2 {
                let spouse = mk(&mut persons, base + 1, "F");
                next_parents.push((kids[0].clone(), spouse));
            }
        }
        parents = next_parents;
    }
    json!({
        "persons": persons.into_iter().map(|p| (p["id"].as_str().unwrap().to_string(), p)).collect::<serde_json::Map<_, _>>(),
        "families": families.into_iter().map(|f| (f["id"].as_str().unwrap().to_string(), f)).collect::<serde_json::Map<_, _>>(),
    })
}

/// Every way a hidden person's dates can differ.
fn redate(flat: &mut Value, lens: &Lens, variant: usize) {
    let ids: Vec<String> = flat["persons"]
        .as_object()
        .unwrap()
        .keys()
        .cloned()
        .collect();
    for (i, id) in ids.iter().enumerate() {
        if lens.sees_person(id) {
            continue;
        }
        let p = &mut flat["persons"][id.as_str()];
        let shift = ((i * 37 + variant * 11) % 90) as i64 - 45;
        match variant {
            0 => {}
            1 => {
                p["birth"] =
                    json!({"date": {"value": (1700 + shift * 3).to_string(), "precision": "year"}})
            }
            2 => p["birth"] = Value::Null,
            3 => {
                p["birth"] = json!({"date": {"value": "1850", "precision": "year", "circa": true}})
            }
            4 => {
                p["birth"] = json!({"date": {"range": {"earliest": {"value": "1600"}, "latest": {"value": "1999"}}}})
            }
            5 => p["birth"] = json!({"date": {"value": "1910", "precision": "decade"}}),
            6 => {
                p["death"] = Value::Null;
                p["birth"] =
                    json!({"date": {"value": (2000 - shift).to_string(), "precision": "year"}});
            }
            _ => p["death"] = json!({"date": {"value": "2024", "precision": "year"}}),
        }
    }
}

#[test]
fn hidden_peoples_dates_never_change_what_a_visitor_receives() {
    // The invariant all the year-scale fixes were cases of: a visitor's SVG
    // and payload are a function of what they may see. Two bundles that
    // differ only in the dates of people hidden from them render
    // byte-identically, from every centre, at every range, signed in or not.
    for (name, base) in [("twelve", twelve()), ("generated", generated())] {
        let lens = Lens::resolve(&base, Visibility::Public);
        let hidden = base["persons"]
            .as_object()
            .unwrap()
            .keys()
            .filter(|id| !lens.sees_person(id))
            .count();
        assert!(hidden > 0, "{name}: the fixture must hide someone");
        let render = |flat: &Value| -> Vec<(String, String)> {
            let g = Graph::build(flat);
            let lens = Lens::resolve(flat, Visibility::Public);
            let mut out = Vec::new();
            for centre in flat["persons"].as_object().unwrap().keys() {
                for n in river::RANGES {
                    for signed_in in [false, true] {
                        let r = river::build(
                            flat,
                            &lens,
                            &g,
                            centre,
                            n,
                            Shape::default(),
                            words(),
                            signed_in,
                        )
                        .unwrap();
                        out.push((r.svg("river"), serde_json::to_string(&r.payload()).unwrap()));
                    }
                }
            }
            out
        };
        let want = render(&base);
        for variant in 1..8 {
            let mut flat = base.clone();
            redate(&mut flat, &lens, variant);
            let got = render(&flat);
            for (i, (a, b)) in want.iter().zip(&got).enumerate() {
                assert!(
                    a.0 == b.0,
                    "{name}, variant {variant}, render {i}: the SVG changed"
                );
                assert!(
                    a.1 == b.1,
                    "{name}, variant {variant}, render {i}: the payload changed"
                );
            }
        }
    }
}
