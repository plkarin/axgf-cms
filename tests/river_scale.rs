//! The river at the operator's scale.
//!
//! Set `AXGF_CMS_BENCH_BUNDLE` to an `.axgf` file and run
//!
//! ```sh
//! AXGF_CMS_BENCH_BUNDLE=… cargo test --release --test river_scale -- --ignored --nocapture
//! ```
//!
//! It renders the river from thirty centres at ranges 2, 3 and 5, records
//! wall-clock and SVG size per render, counts horizontal sweeps for each
//! layout shape, and — when `RIVER_SHEET_DIR` is set — writes every SVG there
//! so they can be tiled into one contact sheet. The grid view stays the
//! landing page until this passes: a river that takes 400 ms to lay out is
//! not a landing page.

use std::io::Read;
use std::time::Instant;

use axgf_cms::access::Lens;
use axgf_cms::river::{self, Graph, Shape, Words};
use serde_json::{Map, Value};

/// The per-render budget, in milliseconds, for layout plus SVG.
const BUDGET_MS: f64 = 400.0;

/// Read the persons and families out of a bundle, leaving the payloads alone.
fn flat_from(path: &str) -> Value {
    let file = std::fs::File::open(path).expect("open bundle");
    let mut zip = zip::ZipArchive::new(file).expect("read bundle");
    let mut persons = Map::new();
    let mut families = Map::new();
    for i in 0..zip.len() {
        let mut entry = zip.by_index(i).expect("entry");
        let name = entry.name().to_string();
        let target = if name.starts_with("persons/") && name.ends_with(".json") {
            &mut persons
        } else if name.starts_with("families/") && name.ends_with(".json") {
            &mut families
        } else {
            continue;
        };
        let mut s = String::new();
        entry.read_to_string(&mut s).expect("utf-8");
        let v: Value = serde_json::from_str(&s).expect("json");
        let id = v["id"].as_str().expect("id").to_string();
        target.insert(id, v);
    }
    serde_json::json!({"persons": persons, "families": families})
}

fn name_of(flat: &Value, id: &str) -> String {
    flat["persons"][id]["identity"]["name"]["display"]
        .as_str()
        .unwrap_or("?")
        .to_string()
}

/// A sweep: an edge whose run is more than 2.5× its rise (under ~22° from
/// horizontal) *and* longer than 150 units. The length floor matters: a
/// spouse stands 200 units from the centre by design, so every short hop from
/// a partner into their union is shallow without being a sweep.
const SWEEP: f64 = 2.5;
const SWEEP_MIN_RUN: f64 = 150.0;

struct Sweeps {
    edges: usize,
    sweeps: usize,
    worst: f64,
}

fn sweeps(l: &river::Layout) -> Sweeps {
    let mut s = Sweeps {
        edges: 0,
        sweeps: 0,
        worst: 0.0,
    };
    let at = |id: &str| l.person(id).map(|p| (p.x, p.y));
    for c in &l.couples {
        for e in c.parents.iter().chain(&c.kids) {
            let Some((x, y)) = at(e) else { continue };
            let (dx, dy) = ((x - c.x).abs(), (y - c.y).abs().max(0.5));
            s.edges += 1;
            let r = dx / dy;
            if r > SWEEP && dx > SWEEP_MIN_RUN {
                s.sweeps += 1;
                if std::env::var("RIVER_SWEEP_DEBUG").is_ok() {
                    let role = l
                        .person(e)
                        .map(|p| format!("{:?}", p.role))
                        .unwrap_or_default();
                    let kind = if c.parents.contains(e) {
                        "parent→union"
                    } else {
                        "union→child"
                    };
                    println!(
                        "    sweep {kind:13} {role:8} gen {:+} dx {dx:5.0} dy {dy:4.0} kids {}",
                        l.person(e).map(|p| p.gen).unwrap_or(0),
                        c.kids.len()
                    );
                }
            }
            if dx > SWEEP_MIN_RUN {
                s.worst = s.worst.max(r);
            }
        }
    }
    s
}

fn centres(flat: &Value, g: &Graph) -> Vec<String> {
    let mut ids: Vec<String> = flat["persons"]
        .as_object()
        .unwrap()
        .keys()
        .cloned()
        .collect();
    // Spread over the whole range of descendant counts, so the thirty include
    // the trunk, the leaves and everything between.
    ids.sort_by_key(|id| (g.descendant_count(id).unwrap_or(0), id.clone()));
    let mut out: Vec<String> = (0..28)
        .map(|i| ids[i * (ids.len() - 1) / 27].clone())
        .collect();
    // The known bad record: a father-son union.
    for id in &ids {
        let n = name_of(flat, id);
        if (n.contains("Jakub") || n.contains("Bronisław"))
            && n.contains("Klicki")
            && !out.contains(id)
        {
            out.push(id.clone());
        }
    }
    out.truncate(30);
    out
}

fn stats(v: &mut [f64]) -> (f64, f64) {
    v.sort_by(f64::total_cmp);
    (v[v.len() / 2], *v.last().unwrap())
}

#[test]
#[ignore = "needs AXGF_CMS_BENCH_BUNDLE"]
fn the_river_at_the_operators_scale() {
    let Ok(path) = std::env::var("AXGF_CMS_BENCH_BUNDLE") else {
        eprintln!("AXGF_CMS_BENCH_BUNDLE not set; skipping");
        return;
    };
    let sheet = std::env::var("RIVER_SHEET_DIR").ok();
    let flat = flat_from(&path);
    let lens = Lens::unrestricted();
    let words = Words {
        living_band: "VIVANTS".into(),
        circa: "v.".into(),
    };

    let t0 = Instant::now();
    let g = Graph::build(&flat);
    let graph_ms = t0.elapsed().as_secs_f64() * 1000.0;
    println!("persons {} · graph built in {graph_ms:.1} ms", g.len());

    let centres = centres(&flat, &g);
    println!("centres: {}", centres.len());
    for (i, id) in centres.iter().enumerate() {
        println!(
            "  c{i:02} {id} {} — {} desc",
            name_of(&flat, id),
            g.descendant_count(id).unwrap_or(0)
        );
    }

    let shapes = [
        (
            "reference",
            Shape {
                hourglass: false,
                log_spacing: false,
            },
        ),
        (
            "hourglass",
            Shape {
                hourglass: true,
                log_spacing: false,
            },
        ),
        (
            "hourglass+log",
            Shape {
                hourglass: true,
                log_spacing: true,
            },
        ),
    ];
    for n in river::RANGES {
        for (label, shape) in shapes {
            let (mut ms, mut kb) = (Vec::new(), Vec::new());
            let (mut edges, mut sw, mut worst) = (0, 0, 0f64);
            let mut worst_at = String::new();
            for (i, id) in centres.iter().enumerate() {
                let t = Instant::now();
                let r = river::build(&flat, &lens, &g, id, n, shape, words.clone(), true)
                    .expect("layout");
                let svg = r.svg("river");
                let json = serde_json::to_string(&r.payload()).unwrap();
                ms.push(t.elapsed().as_secs_f64() * 1000.0);
                kb.push(svg.len() as f64 / 1024.0);
                let s = sweeps(&r.layout);
                edges += s.edges;
                sw += s.sweeps;
                if s.worst > worst {
                    worst = s.worst;
                    worst_at = format!("c{i:02}");
                }
                let _ = json;
                if let Some(dir) = &sheet {
                    let file = format!("{dir}/{label}-n{n}-c{i:02}.svg");
                    std::fs::write(file, &svg).unwrap();
                }
            }
            let (ms_med, ms_max) = stats(&mut ms);
            let (kb_med, kb_max) = stats(&mut kb);
            println!(
                "n={n} {label:14} ms median {ms_med:6.2} worst {ms_max:6.2} · SVG KiB median {kb_med:6.1} worst {kb_max:6.1} · sweeps {sw}/{edges} ({:.1}%), steepest-failing ratio {worst:.1} at {worst_at}",
                100.0 * sw as f64 / edges.max(1) as f64
            );
            assert!(ms_max < BUDGET_MS, "a render took {ms_max:.0} ms");
        }
    }
}

/// Where the time in `Graph::build` goes, cold and warm.
#[test]
#[ignore = "needs AXGF_CMS_BENCH_BUNDLE"]
fn graph_build_cost() {
    let Ok(path) = std::env::var("AXGF_CMS_BENCH_BUNDLE") else {
        return;
    };
    let flat = flat_from(&path);
    if std::env::var("RIVER_WARM_I18N").is_ok() {
        let t = Instant::now();
        let _ = axgf_cms::i18n::translate("en", "nav-river", None);
        println!(
            "i18n first use: {:.2} ms",
            t.elapsed().as_secs_f64() * 1000.0
        );
    }
    for i in 0..5 {
        let t = Instant::now();
        let g = Graph::build(&flat);
        println!(
            "build {i}: {:.2} ms ({} persons)",
            t.elapsed().as_secs_f64() * 1000.0,
            g.len()
        );
    }
    // The descendant count on its own: one traversal per person.
    let g = Graph::build(&flat);
    let ids: Vec<String> = flat["persons"]
        .as_object()
        .unwrap()
        .keys()
        .cloned()
        .collect();
    let total: usize = ids
        .iter()
        .map(|id| g.descendant_count(id).unwrap_or(0))
        .sum();
    let edges: usize = flat["families"]
        .as_object()
        .unwrap()
        .values()
        .map(|f| {
            f["children"].as_array().map_or(0, Vec::len)
                * f["union"]["persons"].as_array().map_or(0, Vec::len)
        })
        .sum();
    println!(
        "descendant sets: sum of sizes {total}, max {}, parent→child edges {edges}",
        ids.iter()
            .map(|id| g.descendant_count(id).unwrap_or(0))
            .max()
            .unwrap_or(0)
    );
}
