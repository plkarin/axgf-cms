//! The river does not cross itself.
//!
//! Every drawn edge — parent → union, union → child, every tail and every
//! sibling stub — is a vertical-tangent cubic. Each is flattened into short
//! segments and every pair of edges is tested for an intersection. Two edges
//! that meet at a node they share (a union, a person) are a confluence, not a
//! crossing: touching within a few units of that shared node does not count.
//!
//! The one permitted exception is a pair whose edges both carry a repeat
//! indicator — the two occurrences of an ancestor reached through two lines
//! (pedigree collapse). Those pairs are listed, never silently skipped.
//!
//! With `AXGF_CMS_BENCH_BUNDLE` set this runs every person of that bundle as
//! the centre at ranges 2, 3 and 5 and reports crossings, compactness (drawn
//! width over available width) and horizontal sweeps. In CI it runs the same
//! check over a generated family.

use std::collections::{BTreeMap, BTreeSet};
use std::io::Read;

use axgf_cms::access::Lens;
use axgf_cms::river::{self, Graph, Layout, Shape, Words};
use serde_json::{json, Map, Value};

/// Segments per cubic.
const PIECES: usize = 24;
/// Touching this close to a shared node is a confluence.
const NEAR_NODE: f64 = 3.0;

#[derive(Debug, Clone)]
struct Edge {
    /// What it joins: node keys, for "shares an endpoint".
    a: String,
    b: String,
    pts: Vec<(f64, f64)>,
    repeat: bool,
    label: String,
}

fn cubic(x1: f64, y1: f64, x2: f64, y2: f64) -> Vec<(f64, f64)> {
    let m = (y1 + y2) / 2.0;
    let (p0, p1, p2, p3) = ((x1, y1), (x1, m), (x2, m), (x2, y2));
    (0..=PIECES)
        .map(|i| {
            let t = i as f64 / PIECES as f64;
            let u = 1.0 - t;
            let b = |a: f64, b: f64, c: f64, d: f64| {
                u * u * u * a + 3.0 * u * u * t * b + 3.0 * u * t * t * c + t * t * t * d
            };
            (b(p0.0, p1.0, p2.0, p3.0), b(p0.1, p1.1, p2.1, p3.1))
        })
        .collect()
}

/// Every edge the renderer draws, from the layout.
fn edges(l: &Layout) -> Vec<Edge> {
    let mut out = Vec::new();
    let at: BTreeMap<&str, (f64, f64, bool)> = l
        .persons
        .iter()
        .map(|p| (p.key.as_str(), (p.x, p.y, p.repeat > 0)))
        .collect();
    for c in &l.couples {
        let ukey = format!("u:{}", c.key);
        for id in &c.parents {
            if let Some(&(x, y, rep)) = at.get(id.as_str()) {
                out.push(Edge {
                    a: format!("p:{id}"),
                    b: ukey.clone(),
                    pts: cubic(x, y, c.x, c.y),
                    repeat: rep || c.repeat,
                    label: format!("{id} → union {}", c.key),
                });
            }
        }
        for id in &c.kids {
            if let Some(&(x, y, rep)) = at.get(id.as_str()) {
                out.push(Edge {
                    a: ukey.clone(),
                    b: format!("p:{id}"),
                    pts: cubic(c.x, c.y, x, y),
                    repeat: rep || c.repeat,
                    label: format!("union {} → {id}", c.key),
                });
            }
        }
        if c.stub > 0 {
            let side = if c.x < 150.0 { 1.0 } else { -1.0 };
            out.push(Edge {
                a: ukey.clone(),
                b: format!("stub:{}", c.key),
                pts: cubic(c.x, c.y, c.x + side * 36.0, c.y - 32.0),
                repeat: c.repeat,
                label: format!("stub of {}", c.key),
            });
        }
    }
    for t in &l.tails {
        // A tail leaves either a person (same point) or a union.
        let from = l
            .persons
            .iter()
            .find(|p| (p.x - t.x1).abs() < 1e-6 && (p.y - t.y1).abs() < 1e-6)
            .map(|p| format!("p:{}", p.key))
            .or_else(|| {
                l.couples
                    .iter()
                    .find(|c| (c.x - t.x1).abs() < 1e-6 && (c.y - t.y1).abs() < 1e-6)
                    .map(|c| format!("u:{}", c.key))
            })
            .unwrap_or_else(|| format!("t:{}", t.key));
        out.push(Edge {
            a: from,
            b: format!("tail:{}", t.key),
            pts: cubic(t.x1, t.y1, t.x2, t.y2),
            repeat: false,
            label: format!("tail {}", t.key),
        });
    }
    out
}

fn seg_cross(p: (f64, f64), q: (f64, f64), r: (f64, f64), s: (f64, f64)) -> Option<(f64, f64)> {
    let d = (q.0 - p.0) * (s.1 - r.1) - (q.1 - p.1) * (s.0 - r.0);
    if d.abs() < 1e-12 {
        return None;
    }
    let t = ((r.0 - p.0) * (s.1 - r.1) - (r.1 - p.1) * (s.0 - r.0)) / d;
    let u = ((r.0 - p.0) * (q.1 - p.1) - (r.1 - p.1) * (q.0 - p.0)) / d;
    ((0.0..=1.0).contains(&t) && (0.0..=1.0).contains(&u))
        .then_some((p.0 + t * (q.0 - p.0), p.1 + t * (q.1 - p.1)))
}

fn bbox(pts: &[(f64, f64)]) -> (f64, f64, f64, f64) {
    pts.iter().fold(
        (
            f64::INFINITY,
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::NEG_INFINITY,
        ),
        |b, p| (b.0.min(p.0), b.1.min(p.1), b.2.max(p.0), b.3.max(p.1)),
    )
}

/// Pairs of edges that cross, as (a, b, both repeated).
fn crossings(es: &[Edge]) -> Vec<(String, String, bool)> {
    let boxes: Vec<_> = es.iter().map(|e| bbox(&e.pts)).collect();
    let mut out = Vec::new();
    for i in 0..es.len() {
        for j in i + 1..es.len() {
            let (a, b) = (&boxes[i], &boxes[j]);
            if a.2 < b.0 || b.2 < a.0 || a.3 < b.1 || b.3 < a.1 {
                continue;
            }
            let (e, f) = (&es[i], &es[j]);
            let shared: Vec<(f64, f64)> = [(&e.a, e.pts[0]), (&e.b, *e.pts.last().unwrap())]
                .into_iter()
                .filter(|(k, _)| **k == f.a || **k == f.b)
                .map(|(_, p)| p)
                .collect();
            let mut hit = false;
            'outer: for s in e.pts.windows(2) {
                for t in f.pts.windows(2) {
                    if let Some(x) = seg_cross(s[0], s[1], t[0], t[1]) {
                        let near = shared
                            .iter()
                            .any(|n| (n.0 - x.0).hypot(n.1 - x.1) < NEAR_NODE);
                        if !near {
                            hit = true;
                            break 'outer;
                        }
                    }
                }
            }
            if hit {
                let ends = |x: &Edge| {
                    format!(
                        "({:.0},{:.0})→({:.0},{:.0})",
                        x.pts[0].0,
                        x.pts[0].1,
                        x.pts.last().unwrap().0,
                        x.pts.last().unwrap().1
                    )
                };
                out.push((
                    format!("{} {}", e.label, ends(e)),
                    format!("{} {}", f.label, ends(f)),
                    e.repeat && f.repeat,
                ));
            }
        }
    }
    out
}

/// Drawn width over available width: how much of [70, W − 44] the people use.
fn compactness(l: &Layout) -> f64 {
    let (lo, hi) = l
        .persons
        .iter()
        .fold((f64::INFINITY, f64::NEG_INFINITY), |a, p| {
            (a.0.min(p.x), a.1.max(p.x))
        });
    if !lo.is_finite() || hi <= lo {
        return 0.0;
    }
    (hi - lo) / (river::W - 44.0 - 70.0)
}

/// A long, shallow run: over 150 units across and under ~22° from horizontal.
fn sweeps(l: &Layout) -> (usize, usize) {
    let (mut n, mut s) = (0, 0);
    for c in &l.couples {
        for id in c.parents.iter().chain(&c.kids) {
            if let Some(p) = l.person(id) {
                let (dx, dy) = ((p.x - c.x).abs(), (p.y - c.y).abs().max(0.5));
                n += 1;
                if dx > 150.0 && dx / dy > 2.5 {
                    s += 1;
                }
            }
        }
    }
    (s, n)
}

fn words() -> Words {
    Words {
        living_band: "LIVING".into(),
        circa: "c.".into(),
    }
}

#[derive(Default)]
struct Report {
    kinds: BTreeMap<String, usize>,
    renders: usize,
    crossing_renders: usize,
    crossings: usize,
    repeat_pairs: Vec<String>,
    worst: Vec<String>,
    compact: Vec<f64>,
    sweeps: (usize, usize),
    /// Renders drawing an ancestor more than once (pedigree collapse), and
    /// the people so repeated.
    collapse: usize,
    collapsed: BTreeSet<String>,
    /// Renders drawing a descendant again beside a third or later spouse.
    remarried: usize,
}

fn run(flat: &Value, centres: &[String], n: usize) -> Report {
    let g = Graph::build(flat);
    let lens = Lens::unrestricted();
    let mut r = Report::default();
    for c in centres {
        let rv = river::build(flat, &lens, &g, c, n, Shape::default(), words(), true).unwrap();
        let l = &rv.layout;
        // RIVER_SHEET_DIR=dir writes every ±3 drawing, for a contact sheet.
        if let Ok(dir) = std::env::var("RIVER_SHEET_DIR") {
            if n == 3 {
                std::fs::write(format!("{dir}/{c}.svg"), rv.svg("sheet")).unwrap();
            }
        }
        // RIVER_CROSS_LAYOUT=centre:n prints that layout, for debugging.
        if std::env::var("RIVER_CROSS_LAYOUT").ok() == Some(format!("{c}:{n}")) {
            println!("{}", serde_json::to_string(l).unwrap());
        }
        let xs = crossings(&edges(l));
        r.renders += 1;
        let real: Vec<_> = xs.iter().filter(|x| !x.2).collect();
        if !real.is_empty() {
            r.crossing_renders += 1;
            if r.worst.len() < 12 {
                r.worst.push(format!(
                    "{c} ±{n}: {} crossing(s), e.g. {} × {}",
                    real.len(),
                    real[0].0,
                    real[0].1
                ));
            }
        }
        r.crossings += real.len();
        // Dump the drawing of a render with an edge-on-edge crossing, for a
        // human to look at: RIVER_CROSS_DUMP=dir.
        if let Ok(dir) = std::env::var("RIVER_CROSS_DUMP") {
            if real
                .iter()
                .any(|x| !x.0.starts_with("tail") && !x.1.starts_with("tail"))
            {
                let path = format!("{dir}/{c}-n{n}.svg");
                if !std::path::Path::new(&path).exists() {
                    std::fs::write(&path, rv.svg("debug")).unwrap();
                    let list: Vec<String> = real
                        .iter()
                        .filter(|x| !x.0.starts_with("tail") && !x.1.starts_with("tail"))
                        .map(|x| format!("{} × {}", x.0, x.1))
                        .collect();
                    std::fs::write(format!("{dir}/{c}-n{n}.txt"), list.join("\n")).unwrap();
                }
            }
        }
        for x in &real {
            let kind = |s: &str| {
                if s.starts_with("tail ") {
                    format!("tail:{}", &s[5..6])
                } else if s.starts_with("stub") {
                    "stub".to_string()
                } else if s.starts_with("union") {
                    "union→child".to_string()
                } else {
                    "parent→union".to_string()
                }
            };
            let (a, b) = (kind(&x.0), kind(&x.1));
            let k = if a <= b {
                format!("{a} × {b}")
            } else {
                format!("{b} × {a}")
            };
            *r.kinds.entry(k).or_default() += 1;
        }
        for x in xs.iter().filter(|x| x.2) {
            r.repeat_pairs.push(format!("{c} ±{n}: {} × {}", x.0, x.1));
        }
        // A person drawn again beside a third spouse: every occurrence on
        // one row, and a parent of three or more unions. Any other repeat is
        // pedigree collapse — a person reached by two lines.
        let mut rows: BTreeMap<&str, BTreeSet<i64>> = BTreeMap::new();
        for p in l.persons.iter().filter(|p| p.repeat > 0) {
            rows.entry(p.id.as_str())
                .or_default()
                .insert(p.y.round() as i64);
        }
        let unions_of = |id: &str| {
            l.couples
                .iter()
                .filter(|c| {
                    c.parents
                        .iter()
                        .any(|k| k == id || k.starts_with(&format!("{id}~")))
                })
                .count()
        };
        let (mut col, mut rem) = (false, false);
        for (id, ys) in rows {
            if ys.len() == 1 && unions_of(id) >= 3 {
                rem = true;
            } else {
                col = true;
                r.collapsed.insert(id.to_string());
            }
        }
        r.collapse += usize::from(col);
        r.remarried += usize::from(rem);
        r.compact.push(compactness(l));
        let (s, e) = sweeps(l);
        r.sweeps.0 += s;
        r.sweeps.1 += e;
    }
    r
}

fn print(name: &str, n: usize, r: &mut Report) {
    r.compact.sort_by(f64::total_cmp);
    let med = r.compact[r.compact.len() / 2];
    let worst = r.compact.last().copied().unwrap_or(0.0);
    println!(
        "{name} ±{n}: {} crossing pair(s) in {} of {} renders · repeat-marked pairs {} · compactness median {:.2} worst {:.2} · sweeps {}/{} ({:.1}%)",
        r.crossings,
        r.crossing_renders,
        r.renders,
        r.repeat_pairs.len(),
        med,
        worst,
        r.sweeps.0,
        r.sweeps.1,
        100.0 * r.sweeps.0 as f64 / r.sweeps.1.max(1) as f64
    );
    for (k, v) in &r.kinds {
        println!("    kind {k}: {v}");
    }
    for w in r.worst.iter().take(4) {
        println!("    {w}");
    }
    println!(
        "    pedigree collapse in {} of {} renders ({} people drawn twice or more); a descendant redrawn beside a third spouse in {}",
        r.collapse,
        r.renders,
        r.collapsed.len(),
        r.remarried
    );
    // Every permitted crossing, listed: none is passed over silently.
    for p in &r.repeat_pairs {
        println!("    repeat (permitted): {p}");
    }
}

fn load(path: &str) -> Value {
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
    json!({"persons": persons, "families": families})
}

#[test]
#[ignore = "needs AXGF_CMS_BENCH_BUNDLE"]
fn the_operators_river_does_not_cross_itself() {
    let Ok(path) = std::env::var("AXGF_CMS_BENCH_BUNDLE") else {
        return;
    };
    let flat = load(&path);
    let mut centres: Vec<String> = flat["persons"]
        .as_object()
        .unwrap()
        .keys()
        .cloned()
        .collect();
    centres.sort();
    let mut total = 0;
    for n in river::RANGES {
        let mut r = run(&flat, &centres, n);
        print("operator", n, &mut r);
        total += r.crossings;
    }
    assert_eq!(total, 0, "the river crosses itself");
}
