//! Every word the river draws fits inside the frame.
//!
//! Eyeballing contact sheets missed names cut off at the right edge at ± 3
//! and ± 5. This replaces the eyeball: for every `<text>` in every emitted
//! SVG, estimate its advance width from its characters, its font and its size,
//! and hold it inside the frame — names and years short of the right rail,
//! everything inside the viewBox's window vertically.
//!
//! The estimate is written here, independently of the renderer's own, so the
//! test is not the code checking its own arithmetic. It is deliberately the
//! more generous of the two: Latin text at 0.55 em a character in the serif
//! (0.6 bold), 0.6 em in the monospace, and 1 em for wide scripts.
//!
//! In CI it runs over a generated family of the operator's proportions whose
//! names are long and wide; with `AXGF_CMS_BENCH_BUNDLE` set it runs over that
//! bundle too. Both: 30 centres × ranges 2, 3, 5, as an administrator and as a
//! signed-out visitor.

use std::io::Read;

use axgf_cms::access::Lens;
use axgf_cms::acl::Visibility;
use axgf_cms::river::{self, Graph, Shape, Words};
use serde_json::{json, Map, Value};

/// Labels stop here: the right rail's generation numbers sit beyond it.
const LABEL_EDGE: f64 = river::W - 34.0;
/// Anything else stops inside the frame by this much.
const MARGIN: f64 = 4.0;

#[derive(Debug)]
struct Text {
    x: f64,
    y: f64,
    size: f64,
    mono: bool,
    bold: bool,
    end: bool,
    label: bool,
    content: String,
}

fn attr(tag: &str, name: &str) -> Option<String> {
    let key = format!(" {name}=\"");
    let at = tag.find(&key)? + key.len();
    Some(tag[at..].split('"').next()?.to_string())
}

fn unescape(s: &str) -> String {
    s.replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#x27;", "'")
        .replace("&#x2f;", "/")
        .replace("&amp;", "&")
}

fn texts(svg: &str) -> Vec<Text> {
    svg.split("<text")
        .skip(1)
        .map(|chunk| {
            let close = chunk.find('>').expect("a text tag closes");
            let tag = &chunk[..close];
            let content = &chunk[close + 1..chunk.find("</text>").expect("</text>")];
            Text {
                x: attr(tag, "x").unwrap().parse().unwrap(),
                y: attr(tag, "y").unwrap().parse().unwrap(),
                size: attr(tag, "font-size").unwrap().parse().unwrap(),
                mono: attr(tag, "font-family").unwrap().contains("monospace"),
                bold: attr(tag, "font-weight")
                    .is_some_and(|w| w.parse::<u32>().unwrap_or(400) >= 600),
                end: attr(tag, "text-anchor").as_deref() == Some("end"),
                // Name and year labels carry the halo; band, rail and count text
                // does not.
                label: tag.contains("paint-order"),
                content: unescape(content),
            }
        })
        .collect()
}

/// Estimated advance width, in viewBox units.
fn advance(t: &Text) -> f64 {
    let latin = if t.mono || t.bold { 0.6 } else { 0.55 };
    t.content
        .chars()
        .map(|c| {
            let wide = matches!(c as u32, 0x1100..=0x115F | 0x2E80..=0xA4CF | 0xAC00..=0xD7A3 | 0xF900..=0xFAFF | 0xFE30..=0xFE4F | 0xFF00..=0xFF60 | 0xFFE0..=0xFFE6);
            t.size * if wide { 1.0 } else { latin }
        })
        .sum()
}

fn check(svg: &str, what: &str, failures: &mut Vec<String>) {
    let view = attr(svg, "viewBox").expect("a viewBox");
    let v: Vec<f64> = view.split(' ').map(|n| n.parse().unwrap()).collect();
    let (top, bottom) = (v[1], v[1] + v[3]);
    for t in texts(svg) {
        let w = advance(&t);
        let (left, right) = if t.end {
            (t.x - w, t.x)
        } else {
            (t.x, t.x + w)
        };
        let edge = if t.label {
            LABEL_EDGE
        } else {
            river::W - MARGIN
        };
        if right > edge || left < MARGIN || t.y - t.size < top || t.y > bottom {
            failures.push(format!(
                "{what}: {:?} spans x {left:.1}..{right:.1} (edge {edge}), y {:.1}..{:.1} (window {top}..{bottom})",
                t.content,
                t.y - t.size,
                t.y
            ));
        }
    }
}

fn words() -> Words {
    Words {
        living_band: "VIVANTS".into(),
        circa: "v.".into(),
    }
}

/// Thirty centres spread over the range of descendant counts.
fn centres(flat: &Value, g: &Graph) -> Vec<String> {
    let mut ids: Vec<String> = flat["persons"]
        .as_object()
        .unwrap()
        .keys()
        .cloned()
        .collect();
    ids.sort_by_key(|id| (g.descendant_count(id).unwrap_or(0), id.clone()));
    (0..30)
        .map(|i| ids[i * (ids.len() - 1) / 29].clone())
        .collect()
}

fn run(flat: &Value) -> Vec<String> {
    let g = Graph::build(flat);
    let admin = Lens::unrestricted();
    let visitor = Lens::resolve(flat, Visibility::Public);
    let mut failures = Vec::new();
    let mut checked = 0;
    for centre in centres(flat, &g) {
        for n in river::RANGES {
            for (who, lens, signed_in) in [("admin", &admin, true), ("visitor", &visitor, false)] {
                let r = river::build(
                    flat,
                    lens,
                    &g,
                    &centre,
                    n,
                    Shape::default(),
                    words(),
                    signed_in,
                )
                .unwrap();
                check(
                    &r.svg("river"),
                    &format!("{who} {centre} ±{n}"),
                    &mut failures,
                );
                checked += 1;
            }
        }
    }
    assert_eq!(checked, 30 * 3 * 2);
    failures
}

/// Makes one person: into this map, born that year, of that gender; returns
/// the new id.
type Mk<'a> = dyn FnMut(&mut Map<String, Value>, i64, &str) -> String + 'a;

/// A family of the operator's proportions — wide generations, deep
/// pedigrees, several children a couple — whose names are as long and as
/// wide as real ones get: Polish given-name chains, a CJK name, Arabic.
fn generated() -> Value {
    const GIVEN: &[&str] = &[
        "Alfons Władysław Antoni",
        "Eugeniusz Karol Alojzy",
        "Bronisława",
        "Jan",
        "Konstancja Wiktoria",
        "Ignacy",
        "Marianna",
        "Józef",
        "山田花子",
        "Stanisław Florian",
        "محمد عبد الله",
        "Wojciech",
    ];
    const SURNAME: &[&str] = &[
        "Wierzbięta",
        "Gundelach-Groszkowski",
        "Nowak",
        "Szczepańska",
        "Lis",
        "Chądzyński",
    ];
    let mut persons = Map::new();
    let mut families = Map::new();
    let mut n = 0usize;
    let mut mk = |persons: &mut Map<String, Value>, born: i64, g: &str| {
        let id = format!("{n:08}-0000-4000-8000-000000000000");
        let given = GIVEN[n % GIVEN.len()];
        let surname = SURNAME[(n / 3) % SURNAME.len()];
        persons.insert(id.clone(), json!({"id": id, "type": "person", "axgf_version": "1.1",
            "identity": {"name": {"display": format!("{given} {surname}"),
                "components": [{"type": "given_name", "value": given}, {"type": "family_name", "value": surname}]},
                "gender": {"value": g}, "is_living": born > 1940,
                "visibility": if n.is_multiple_of(4) { "members" } else { "public" }},
            "birth": {"date": {"value": born.to_string(), "precision": "year", "circa": n.is_multiple_of(5)}},
            "death": {"date": {"value": (born + 70).to_string(), "precision": "year"}}}));
        n += 1;
        id
    };
    // Ancestry: a full pedigree five deep above a couple, then four
    // generations of descendants, five children a couple.
    fn pedigree(
        persons: &mut Map<String, Value>,
        families: &mut Map<String, Value>,
        mk: &mut Mk<'_>,
        child: &str,
        born: i64,
        depth: usize,
    ) {
        if depth == 0 {
            return;
        }
        let fa = mk(persons, born - 28, "M");
        let mo = mk(persons, born - 25, "F");
        let fid = format!("anc-{child}");
        families.insert(fid.clone(), json!({"id": fid, "type": "family", "axgf_version": "1.1",
            "union": {"persons": [{"person_id": fa, "role": "spouse"}, {"person_id": mo, "role": "spouse"}]},
            "children": [{"person_id": child, "confidence": 0.9}]}));
        pedigree(persons, families, mk, &fa, born - 28, depth - 1);
        pedigree(persons, families, mk, &mo, born - 25, depth - 1);
    }
    let root = mk(&mut persons, 1850, "M");
    let wife = mk(&mut persons, 1852, "F");
    pedigree(&mut persons, &mut families, &mut mk, &root, 1850, 5);
    let mut couples = vec![(root, wife, 1850)];
    for gen in 0..4 {
        let mut next = Vec::new();
        for (ci, (fa, mo, born)) in couples.into_iter().enumerate() {
            let kids: Vec<String> = (0..4)
                .map(|k| {
                    mk(
                        &mut persons,
                        born + 25 + k * 2,
                        if k % 2 == 0 { "M" } else { "F" },
                    )
                })
                .collect();
            let fid = format!("desc-{gen}-{ci}");
            families.insert(fid.clone(), json!({"id": fid, "type": "family", "axgf_version": "1.1",
                "union": {"persons": [{"person_id": fa, "role": "spouse"}, {"person_id": mo, "role": "spouse"}]},
                "children": kids.iter().map(|k| json!({"person_id": k, "confidence": 0.8})).collect::<Vec<_>>()}));
            for k in kids.into_iter().take(3) {
                let sp = mk(&mut persons, born + 27, "F");
                next.push((k, sp, born + 25));
            }
        }
        couples = next;
    }
    json!({"persons": persons, "families": families})
}

#[test]
fn every_word_on_the_river_fits_the_frame() {
    let flat = generated();
    assert!(flat["persons"].as_object().unwrap().len() > 250);
    let failures = run(&flat);
    assert!(
        failures.is_empty(),
        "{} text node(s) leave the frame:\n{}",
        failures.len(),
        failures
            .iter()
            .take(40)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    );
}

#[test]
#[ignore = "needs AXGF_CMS_BENCH_BUNDLE"]
fn every_word_on_the_operators_river_fits_the_frame() {
    let Ok(path) = std::env::var("AXGF_CMS_BENCH_BUNDLE") else {
        return;
    };
    let mut zip = zip::ZipArchive::new(std::fs::File::open(path).unwrap()).unwrap();
    let (mut persons, mut families) = (Map::new(), Map::new());
    for i in 0..zip.len() {
        let mut e = zip.by_index(i).unwrap();
        let name = e.name().to_string();
        let target = if name.starts_with("persons/") && name.ends_with(".json") {
            &mut persons
        } else if name.starts_with("families/") && name.ends_with(".json") {
            &mut families
        } else {
            continue;
        };
        let mut s = String::new();
        e.read_to_string(&mut s).unwrap();
        let v: Value = serde_json::from_str(&s).unwrap();
        target.insert(v["id"].as_str().unwrap().to_string(), v);
    }
    let failures = run(&json!({"persons": persons, "families": families}));
    assert!(
        failures.is_empty(),
        "{} text node(s) leave the frame:\n{}",
        failures.len(),
        failures
            .iter()
            .take(40)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    );
}

#[test]
fn the_window_holds_everything_and_crops_only_dead_space() {
    let flat = generated();
    let g = Graph::build(&flat);
    let centre = centres(&flat, &g)[29].clone();
    for n in river::RANGES {
        let r = river::build(
            &flat,
            &Lens::unrestricted(),
            &g,
            &centre,
            n,
            Shape::default(),
            words(),
            true,
        )
        .unwrap();
        let (top, h) = r.layout.meta.view;
        let bottom = top + h;
        for p in &r.layout.persons {
            assert!(
                p.y >= top && p.y <= bottom,
                "±{n}: {} at y {} outside {top}..{bottom}",
                p.id,
                p.y
            );
        }
        for t in &r.layout.tails {
            assert!(
                t.y2 - 4.0 >= top && t.y2 + 4.0 <= bottom,
                "±{n}: tail {} ends at {}",
                t.key,
                t.y2
            );
        }
        if n == 2 {
            // Cropped to what the drawing can reach: 384 units of rows and
            // the tails, arrows and rings beyond them.
            assert!(h < 0.85 * river::H, "±2 is cropped: height {h}");
            assert_eq!(r.layout.meta.view, river::view_window(2, 96.0));
        } else {
            assert!(h >= 0.9 * river::H, "±{n} fills the frame: height {h}");
        }
    }
}
