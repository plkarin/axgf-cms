//! Every word the river draws fits inside the frame, and no two labels
//! overlap.
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

/// Labels stop this far inside the window's right edge: the right rail's
/// generation numbers sit beyond.
const LABEL_EDGE: f64 = 34.0;
/// Anything else stops inside the window by this much.
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
                // (A `+n` written on its line carries a halo too; it is a
                // count, not a label.)
                label: tag.contains("paint-order") && !tag.contains("rv-count"),
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
    let (left_edge, top, right_edge, bottom) = (v[0], v[1], v[0] + v[2], v[1] + v[3]);
    for t in texts(svg) {
        let w = advance(&t);
        let (left, right) = if t.end {
            (t.x - w, t.x)
        } else {
            (t.x, t.x + w)
        };
        let edge = if t.label {
            right_edge - LABEL_EDGE
        } else {
            right_edge - MARGIN
        };
        if right > edge || left < left_edge + MARGIN || t.y - t.size < top || t.y > bottom {
            failures.push(format!(
                "{what}: {:?} spans x {left:.1}..{right:.1} (edge {edge}), y {:.1}..{:.1} (window {top}..{bottom})",
                t.content,
                t.y - t.size,
                t.y
            ));
        }
    }
}

/// No two people's labels overlap.
///
/// A label is a name line and a years line drawn at the same x; its box is
/// the union of the two, each measured with [`advance`]. Every pair of boxes
/// in one drawing is tested — the rows are what makes overlap possible, but a
/// pairwise check does not have to know that.
fn check_overlap(svg: &str, what: &str, failures: &mut Vec<String>) {
    let mut boxes: Vec<(String, f64, f64, f64, f64)> = Vec::new();
    let labels: Vec<Text> = texts(svg).into_iter().filter(|t| t.label).collect();
    for pair in labels.chunks(2) {
        let [name, years] = pair else {
            failures.push(format!("{what}: a name without its years line"));
            continue;
        };
        assert_eq!(name.x, years.x, "{what}: a label's two lines share an x");
        let right = (name.x + advance(name)).max(years.x + advance(years));
        boxes.push((
            name.content.clone(),
            name.x,
            right,
            name.y - name.size,
            years.y + 2.0,
        ));
    }
    for i in 0..boxes.len() {
        for j in i + 1..boxes.len() {
            let (a, b) = (&boxes[i], &boxes[j]);
            let overlap_x = a.1 < b.2 && b.1 < a.2;
            let overlap_y = a.3 < b.4 && b.3 < a.4;
            if overlap_x && overlap_y {
                failures.push(format!(
                    "{what}: {:?} [{:.1}..{:.1}] overlaps {:?} [{:.1}..{:.1}]",
                    a.0, a.1, a.2, b.0, b.1, b.2
                ));
            }
        }
    }
}

/// No text runs over a node: a person's dot (r 4.8, the centre's 7.5) or a
/// ring where a line ends.
///
/// Every `<text>` in the drawing — names, years, `×n` markers, counts — is
/// boxed by its advance and its em (ascent 0.8, descent 0.2 of its size);
/// every drawn circle is tested against every box. The invisible r-20 hit
/// circles and the living glow are not nodes and are skipped. Labels start
/// at r + 6 from their own dot, so the collision is with a neighbour's dot
/// or ring: what a name runs into when its row is tight.
fn check_circles(svg: &str, what: &str, failures: &mut Vec<String>) {
    let mut circles: Vec<(f64, f64, f64)> = Vec::new();
    for chunk in svg.split("<circle").skip(1) {
        let tag = &chunk[..chunk.find('>').expect("a circle tag closes")];
        if tag.contains(r#"fill="transparent""#) || tag.contains("fill-opacity") {
            continue;
        }
        let num = |n: &str| attr(tag, n).unwrap().parse::<f64>().unwrap();
        circles.push((num("cx"), num("cy"), num("r")));
    }
    for t in texts(svg) {
        let w = advance(&t);
        let (left, right) = if t.end {
            (t.x - w, t.x)
        } else {
            (t.x, t.x + w)
        };
        let (top, bottom) = (t.y - 0.8 * t.size, t.y + 0.2 * t.size);
        for &(cx, cy, r) in &circles {
            let dx = (left - cx).max(cx - right).max(0.0);
            let dy = (top - cy).max(cy - bottom).max(0.0);
            if dx.hypot(dy) < r {
                failures.push(format!(
                    "{what}: {:?} [{left:.1}..{right:.1}, {top:.1}..{bottom:.1}] runs over the circle at ({cx:.1}, {cy:.1}) r {r}",
                    t.content
                ));
            }
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

fn run(flat: &Value, every_centre: bool) -> Vec<String> {
    let g = Graph::build(flat);
    let chosen = if every_centre {
        let mut all: Vec<String> = flat["persons"]
            .as_object()
            .unwrap()
            .keys()
            .cloned()
            .collect();
        all.sort();
        all
    } else {
        centres(flat, &g)
    };
    let expected = chosen.len() * 3 * 2;
    let admin = Lens::unrestricted();
    let visitor = Lens::resolve(flat, Visibility::Public);
    let mut failures = Vec::new();
    let mut checked = 0;
    for centre in chosen {
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
                let svg = r.svg("river");
                let what = format!("{who} {centre} ±{n}");
                // RIVER_TEXT_DEBUG="who centre ±n" prints that layout.
                if std::env::var("RIVER_TEXT_DEBUG").ok().as_deref() == Some(what.as_str()) {
                    println!("{}", serde_json::to_string(&r.layout).unwrap());
                }
                check(&svg, &what, &mut failures);
                check_overlap(&svg, &what, &mut failures);
                check_circles(&svg, &what, &mut failures);
                checked += 1;
            }
        }
    }
    assert_eq!(checked, expected);
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
    let failures = run(&flat, false);
    assert!(
        failures.is_empty(),
        "{} text node(s) leave the frame or run over a node:\n{}",
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
    // Every person as the centre, not thirty: the known overlap, Demetriusz
    // Kaniewski's name over his wife's, was not among the thirty.
    let failures = run(&json!({"persons": persons, "families": families}), true);
    // RIVER_TEXT_ALL=path writes every failure, not only the first forty.
    if let Ok(path) = std::env::var("RIVER_TEXT_ALL") {
        std::fs::write(path, failures.join("\n")).unwrap();
    }
    assert!(
        failures.is_empty(),
        "{} text node(s) leave the frame or run over a node:\n{}",
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
        let (left, w) = r.layout.meta.hview;
        for p in &r.layout.persons {
            assert!(
                p.x - 4.8 >= left && p.x + 4.8 <= left + w,
                "±{n}: {} at x {} outside {left}..{}",
                p.id,
                p.x,
                left + w
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

/// The window depends on the range and nothing else, so the fixed point
/// stays where it is on screen while the river travels: from every centre at
/// one range the viewBox is the same and the centre is at (CX, CY) in it.
/// (`river.js` tweens the window between two frames; between two frames of
/// one range that is no change at all.)
#[test]
fn the_fixed_point_does_not_move_while_the_river_travels() {
    let flat = generated();
    let g = Graph::build(&flat);
    let ids: Vec<String> = flat["persons"]
        .as_object()
        .unwrap()
        .keys()
        .cloned()
        .collect();
    for n in river::RANGES {
        let mut seen: Option<String> = None;
        for id in &ids {
            let r = river::build(
                &flat,
                &Lens::unrestricted(),
                &g,
                id,
                n,
                Shape::default(),
                words(),
                true,
            )
            .unwrap();
            let c = r.layout.person(id).unwrap();
            assert_eq!(
                (c.x, c.y),
                (river::CX, river::CY),
                "±{n} {id}: the centre is the fixed point"
            );
            assert_eq!(r.layout.meta.hview, river::h_window(n));
            let view = attr(&r.svg("river"), "viewBox").unwrap();
            match &seen {
                None => seen = Some(view),
                Some(v) => assert_eq!(&view, v, "±{n} {id}: the window moved"),
            }
        }
    }
}

/// The known overlap, as a fixed shape rather than whatever a generated
/// family happens to place: a centre with a long name and a spouse 200 units
/// to the right, which is where Demetriusz Kaniewski's name ran over Marianna
/// Dola's. The generated family stopped containing this case when sibling
/// order changed, and the check above went on passing without it.
#[test]
fn a_long_centre_name_does_not_run_over_the_spouse() {
    let person = |id: &str, given: &str, surname: &str, g: &str, born: i64| {
        json!({"id": id, "type": "person", "axgf_version": "1.1",
            "identity": {"name": {"display": format!("{given} {surname}"),
                "components": [{"type": "given_name", "value": given}, {"type": "family_name", "value": surname}]},
                "gender": {"value": g}, "is_living": false, "visibility": "public"},
            "birth": {"date": {"value": born.to_string(), "precision": "year"}}})
    };
    let mut persons = Map::new();
    for p in [
        person(
            "c",
            "Konstancja Wiktoria",
            "Gundelach-Groszkowska",
            "F",
            1790,
        ),
        person("s", "Demetriusz", "Kaniewski", "M", 1786),
        person("k", "Tekla", "Kaniewska", "F", 1811),
        // A given name that is a sentence: the centre's floor label must be
        // cut to fit, not run off the frame.
        person(
            "n",
            "Anna - w dokumentach wystepuje także pod tym imieniem",
            "Kaniewska",
            "F",
            1814,
        ),
    ] {
        persons.insert(p["id"].as_str().unwrap().to_string(), p);
    }
    // A tight row of short given names over full date ranges: thirteen
    // children, squeezed toward the centre by the spouse on the right, stand
    // 47.7 apart — wide enough for "Jan" at tier 1, not for "1815–1880" under
    // it — and a fit that measured only the name drew the years into the next
    // label.
    let mut kids = vec![json!({"person_id": "k"}), json!({"person_id": "n"})];
    for (i, given) in [
        "Jan", "Ola", "Ewa", "Iza", "Ida", "Józ", "Ala", "Lex", "Ula", "Ola", "Eda",
    ]
    .iter()
    .enumerate()
    {
        let id = format!("t{i}");
        let mut p = person(&id, given, "Kaniewska", "F", 1815 + i as i64);
        p["death"] = json!({"date": {"value": (1880 + i).to_string(), "precision": "year"}});
        persons.insert(id.clone(), p);
        kids.push(json!({"person_id": id}));
    }
    let families = json!({"f": {"id": "f", "type": "family", "axgf_version": "1.1",
        "union": {"persons": [{"person_id": "s", "role": "spouse"}, {"person_id": "c", "role": "spouse"}]},
        "children": kids}});
    let flat = json!({"persons": persons, "families": families});
    let g = Graph::build(&flat);
    let mut failures = Vec::new();
    for centre in ["c", "s", "n"] {
        for n in river::RANGES {
            let r = river::build(
                &flat,
                &Lens::unrestricted(),
                &g,
                centre,
                n,
                Shape::default(),
                words(),
                true,
            )
            .unwrap();
            let svg = r.svg("river");
            check(&svg, &format!("{centre} ±{n}"), &mut failures);
            check_overlap(&svg, &format!("{centre} ±{n}"), &mut failures);
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
