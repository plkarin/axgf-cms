//! Builds and verifies the demonstration bundle shipped with `--with-sample`.
//!
//! # Why the sample is not just a converted GEDCOM
//!
//! The brief asks for a sample seeded from a small demo GEDCOM. Converting one
//! is exactly what `deploy/sample.ged` is for — but a GEDCOM-derived bundle
//! cannot demonstrate most of what this site exists to show. Measured on the
//! operator's real 767-person file:
//!
//! * every fact came out at the same confidence (0.8), and parentages carried
//!   none at all, so the confidence rendering had nothing to differentiate;
//! * there were zero links, because GEDCOM cannot express a godparent;
//! * occupations had titles but no dates, so no span could be drawn.
//!
//! A fresh install seeded from GEDCOM alone would therefore show a working
//! family tree and none of the reasons to prefer the format. So the sample is
//! the converted GEDCOM *plus* the AXGF-native facts GEDCOM has nowhere to
//! put. The contrast is the demonstration.
//!
//! Regenerate with:
//!
//! ```text
//! AXGF_CMS_REGENERATE_SAMPLE=1 cargo test --test sample_bundle -- --nocapture
//! ```

mod common;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde_json::{json, Map, Value};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn sample_ged() -> PathBuf {
    repo_root().join("deploy/sample.ged")
}

fn sample_axgf() -> PathBuf {
    repo_root().join("deploy/sample.axgf")
}

/// Index persons by display name so enrichment can refer to them by name
/// rather than by the random UUID conversion assigns.
fn persons_by_name(bundle: &Value) -> BTreeMap<String, String> {
    bundle
        .get("persons")
        .and_then(Value::as_object)
        .map(|m| {
            m.iter()
                .filter_map(|(id, p)| {
                    let name = p.pointer("/identity/name/display")?.as_str()?;
                    Some((name.to_string(), id.clone()))
                })
                .collect()
        })
        .unwrap_or_default()
}

fn set(entity: &mut Value, path: &str, v: Value) {
    let mut cur = entity;
    let segs: Vec<&str> = path.split('/').collect();
    for (i, seg) in segs.iter().enumerate() {
        if i + 1 == segs.len() {
            if let Some(o) = cur.as_object_mut() {
                o.insert((*seg).to_string(), v);
            }
            return;
        }
        if cur.get(seg).is_none() {
            if let Some(o) = cur.as_object_mut() {
                o.insert((*seg).to_string(), Value::Object(Map::new()));
            }
        }
        cur = cur.get_mut(seg).expect("just inserted");
    }
}

/// Add the facts AXGF can hold and GEDCOM cannot.
fn enrich(bundle: &mut Value) {
    let people = persons_by_name(bundle);
    let id = |n: &str| -> String {
        people
            .get(n)
            .unwrap_or_else(|| panic!("sample.ged should contain {n}; found {people:?}"))
            .clone()
    };

    // ---- Sources, graded by reliability -------------------------------
    let s_parish = "aa000000-0000-4000-8000-000000000001";
    let s_civil = "aa000000-0000-4000-8000-000000000002";
    let s_letter = "aa000000-0000-4000-8000-000000000003";
    let s_dna = "aa000000-0000-4000-8000-000000000004";
    let s_book = "aa000000-0000-4000-8000-000000000005";

    let sources = json!({
        s_parish: {
            "id": s_parish, "type": "source", "axgf_version": "1.0",
            "title": "Sainte-Colombe parish register, 1791–1840",
            "source_type": "baptism_record", "reliability": "primary",
            "confidence": 0.95, "status": "verified",
            "repository": {"name": "Archives départementales du Rhône",
                           "reference": "4E 1721"}},
        s_civil: {
            "id": s_civil, "type": "source", "axgf_version": "1.0",
            "title": "Lyon civil register, 3e arrondissement",
            "source_type": "birth_certificate", "reliability": "primary",
            "confidence": 0.98, "status": "verified",
            "repository": {"name": "Archives municipales de Lyon"}},
        s_letter: {
            "id": s_letter, "type": "source", "axgf_version": "1.0",
            "title": "Letter from Camille Meunier to her niece, 1932",
            "source_type": "letter", "reliability": "oral", "confidence": 0.6,
            "status": "verified",
            "note": "Recollection written down seventy years after the events \
                     it describes. Useful, but not a record."},
        s_dna: {
            "id": s_dna, "type": "source", "axgf_version": "1.0",
            "title": "Autosomal DNA match, 2024",
            "source_type": "dna", "reliability": "secondary", "confidence": 0.75,
            "dna": {"test_provider": "example-lab", "test_type": "autosomal"},
            "note": "Supports the Fabre connection; does not by itself \
                     establish which generation it runs through."},
        s_book: {
            "id": s_book, "type": "source", "axgf_version": "1.0",
            "title": "Familles du Rhône, vol. II (1908)",
            "source_type": "published_genealogy", "reliability": "authored",
            "confidence": 0.5, "status": "unverified",
            "note": "Cites no sources of its own."}
    });
    bundle["sources"] = sources;

    // ---- Confidence that actually varies -------------------------------
    // A birth from a parish register is near-certain; one inferred from a
    // relative's letter seventy years later is not.
    let facts: &[(&str, &str, f64, &str, f64, &str)] = &[
        // person,        birth conf, birth source,  death conf, death source
        ("Aymeric Meunier", "birth", 0.45, s_book, 0.55, s_book),
        ("Perrine Aubert", "birth", 0.40, s_letter, 0.30, s_letter),
        ("Jules Meunier", "birth", 0.98, s_parish, 0.96, s_civil),
        ("Elise Bonnet", "birth", 0.50, s_book, 0.80, s_civil),
        ("Marguerite Meunier", "birth", 0.92, s_civil, 0.35, s_letter),
        ("Henri Meunier", "birth", 0.90, s_parish, 0.90, s_parish),
        ("Camille Meunier", "birth", 0.93, s_civil, 0.88, s_civil),
        ("Jean Boucher", "birth", 0.60, s_book, 0.85, s_civil),
        ("Louis Fabre", "birth", 0.78, s_civil, 0.82, s_civil),
        ("Sophie Meunier", "birth", 0.95, s_civil, 0.94, s_civil),
    ];
    for (name, _, bconf, bsrc, dconf, dsrc) in facts {
        let pid = id(name);
        let Some(p) = bundle["persons"].get_mut(&pid) else {
            continue;
        };
        if p.get("birth").is_some() {
            set(p, "birth/confidence", json!(bconf));
            set(p, "birth/source_id", json!(bsrc));
        }
        if p.get("death").is_some() {
            set(p, "death/confidence", json!(dconf));
            set(p, "death/source_id", json!(dsrc));
        }
    }

    // A biography, so the identity page has prose to show.
    let jules = id("Jules Meunier");
    if let Some(p) = bundle["persons"].get_mut(&jules) {
        set(
            p,
            "bio",
            json!(
                "Schoolmaster at Lyon from 1848 until his retirement in 1878. The \
             parish register gives his birth exactly; his father's is known \
             only as “before 1798”, which is why the two dates on this page \
             are rendered — and weighted — differently."
            ),
        );
    }

    // ---- Parentage confidence, which GEDCOM cannot carry ---------------
    // Marguerite's parentage is well evidenced; Aymeric's line back is a guess
    // from a published genealogy that cites nothing.
    let parentage: &[(&str, f64)] = &[
        ("Jules Meunier", 0.42),
        ("Marguerite Meunier", 0.97),
        ("Henri Meunier", 0.95),
        ("Camille Meunier", 0.93),
        ("Sophie Meunier", 0.88),
    ];
    let want: BTreeMap<String, f64> = parentage.iter().map(|(n, c)| (id(n), *c)).collect();

    if let Some(fams) = bundle["families"].as_object_mut() {
        for fam in fams.values_mut() {
            if let Some(children) = fam.get_mut("children").and_then(Value::as_array_mut) {
                for child in children.iter_mut() {
                    let cid = child
                        .get("person_id")
                        .and_then(Value::as_str)
                        .unwrap_or_default()
                        .to_string();
                    if let Some(c) = want.get(&cid) {
                        child["confidence"] = json!(c);
                    }
                }
            }
            // Union confidence, so spouse connectors differ too.
            set(fam, "union/confidence", json!(0.9));
            set(fam, "union/source_id", json!(s_parish));
        }
    }

    // ---- Occupations as spans ------------------------------------------
    // The GEDCOM gave a title and a single date. AXGF holds the duration.
    let occ_spans: &[(&str, &str, &str, Option<&str>, f64)] = &[
        (
            "Instituteur",
            "1848",
            "1878",
            Some("École communale de Lyon"),
            0.9,
        ),
        ("Meunier", "1820", "1858", None, 0.65),
        ("Sage-femme", "1875", "1915", None, 0.7),
        ("Notaire", "1845", "1884", Some("Étude Boucher, Lyon"), 0.85),
    ];
    if let Some(occs) = bundle["occupations"].as_object_mut() {
        for occ in occs.values_mut() {
            let title = occ
                .get("title")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_string();
            if let Some((_, from, until, employer, conf)) =
                occ_spans.iter().find(|(t, ..)| *t == title)
            {
                set(
                    occ,
                    "valid_from",
                    json!({"date": {"value": from, "precision": "year"}}),
                );
                set(
                    occ,
                    "valid_until",
                    json!({"date": {"value": until, "precision": "year"}}),
                );
                set(occ, "confidence", json!(conf));
                set(occ, "source_id", json!(s_civil));
                if let Some(e) = employer {
                    set(occ, "employer", json!({"name": e}));
                }
            }
        }
    }

    // ---- Links: the clearest GEDCOM gap --------------------------------
    let l = |n: u8| format!("bb000000-0000-4000-8000-00000000000{n}");
    let links = json!({
        l(1): {
            "id": l(1), "type": "link", "axgf_version": "1.0",
            "from": {"entity_type": "person", "entity_id": id("Jean Boucher")},
            "to": {"entity_type": "person", "entity_id": id("Marguerite Meunier")},
            "label": "godfather", "label_reverse": "goddaughter",
            "category": "spiritual",
            "valid_from": {"date": {"value": "1851", "precision": "year"}},
            "confidence": 0.85, "source_id": s_letter,
            "note": "Named as godfather in Camille's 1932 letter. No parish \
                     entry survives to confirm it."},
        l(2): {
            "id": l(2), "type": "link", "axgf_version": "1.0",
            "from": {"entity_type": "person", "entity_id": id("Jean Boucher")},
            "to": {"entity_type": "person", "entity_id": id("Jules Meunier")},
            "label": "employer", "label_reverse": "employee",
            "category": "professional",
            "valid_from": {"date": {"value": "1848", "precision": "year"}},
            "valid_until": {"date": {"value": "1860", "precision": "year"}},
            "confidence": 0.55, "source_id": s_book,
            "note": "The 1908 genealogy says Jules kept Boucher's accounts \
                     alongside his teaching. Nothing else mentions it."},
        l(3): {
            "id": l(3), "type": "link", "axgf_version": "1.0",
            "from": {"entity_type": "person", "entity_id": id("Jules Meunier")},
            "to": {"entity_type": "person", "entity_id": id("Louis Fabre")},
            "label": "mentor", "label_reverse": "pupil",
            "category": "educational",
            "valid_from": {"date": {"value": "1862", "precision": "year"}},
            "valid_until": {"date": {"value": "1866", "precision": "year"}},
            "confidence": 0.7, "source_id": s_letter,
            "note": "Louis was Jules' pupil before he married Marguerite."},
        l(4): {
            "id": l(4), "type": "link", "axgf_version": "1.0",
            "from": {"entity_type": "person", "entity_id": id("Camille Meunier")},
            "to": {"entity_type": "person", "entity_id": id("Sophie Meunier")},
            "label": "witness at marriage", "label_reverse": "witnessed by",
            "category": "legal",
            "valid_from": {"date": {"value": "1903", "precision": "year"}},
            "confidence": 0.99, "source_id": s_civil,
            "note": "Signed the register."}
    });
    bundle["links"] = links;

    // ---- A place whose country changed ---------------------------------
    if let Some(places) = bundle["places"].as_object_mut() {
        for place in places.values_mut() {
            let is_sainte_colombe = place
                .get("names")
                .and_then(Value::as_array)
                .map(|ns| {
                    ns.iter().any(|n| {
                        n.get("value")
                            .and_then(Value::as_str)
                            .is_some_and(|v| v.contains("Sainte-Colombe"))
                    })
                })
                .unwrap_or(false);
            if is_sainte_colombe {
                set(place, "place_type", json!("village"));
                set(place, "country_current", json!("France"));
                set(
                    place,
                    "country_history",
                    json!([
                        {"country": "Kingdom of France", "until": "1792"},
                        {"country": "French First Republic", "from": "1792", "until": "1804"},
                        {"country": "First French Empire", "from": "1804", "until": "1815"},
                        {"country": "France", "from": "1815"}
                    ]),
                );
                set(
                    place,
                    "note",
                    json!(
                        "Aymeric was born here before 1798 — under which state \
                     depends on exactly when, which is precisely what the \
                     record cannot say."
                    ),
                );
            }
        }
    }
}

/// Build the sample bundle from the committed GEDCOM plus enrichment.
fn build_sample() -> Vec<u8> {
    let ged = std::fs::read(sample_ged()).expect("deploy/sample.ged must exist");
    let env = axgf_rs::convert_gedcom(&ged, 0.8, "fr");
    assert_eq!(
        env.status,
        axgf_rs::boundary::envelope::Status::Ok,
        "sample.ged must convert: {:?}",
        env.diagnostics
    );
    let mut bundle = env
        .data
        .get("bundle")
        .cloned()
        .expect("conversion returns a bundle");

    enrich(&mut bundle);

    axgf_cms::state::export_to_bytes(&bundle.to_string()).expect("sample must export")
}

#[test]
fn sample_bundle_is_current() {
    let bytes = build_sample();

    if std::env::var("AXGF_CMS_REGENERATE_SAMPLE").is_ok() {
        std::fs::write(sample_axgf(), &bytes).expect("write sample.axgf");
        eprintln!(
            "wrote {} ({} KB)",
            sample_axgf().display(),
            bytes.len() / 1024
        );
        return;
    }

    assert!(
        sample_axgf().exists(),
        "deploy/sample.axgf is missing. Regenerate with \
         AXGF_CMS_REGENERATE_SAMPLE=1 cargo test --test sample_bundle"
    );
}

/// The committed sample must actually demonstrate the things the site claims.
/// An empty showcase is a wasted first impression; a sample that shows only
/// what GEDCOM can already do is barely better.
#[test]
fn committed_sample_demonstrates_every_showcase_feature() {
    let path = sample_axgf();
    if !path.exists() {
        panic!("deploy/sample.axgf is missing; regenerate it");
    }
    let bytes = std::fs::read(&path).expect("read sample.axgf");
    let env = axgf_rs::import_bundle(&bytes);
    let bundle = env.data;
    assert!(
        !bundle.is_null(),
        "sample.axgf must import: {:?}",
        env.diagnostics
    );

    let count = |k: &str| {
        bundle
            .get(k)
            .and_then(Value::as_object)
            .map(|m| m.len())
            .unwrap_or(0)
    };

    assert!(count("persons") >= 8, "enough people to show a tree");
    assert!(count("families") >= 3, "at least three generations");

    // Non-family relationships: the clearest GEDCOM gap.
    assert!(
        count("links") >= 4,
        "the sample must show links; a GEDCOM import has none"
    );

    // Sources graded by reliability, spanning the range.
    let reliabilities: Vec<String> = bundle["sources"]
        .as_object()
        .map(|m| {
            m.values()
                .filter_map(|s| s.get("reliability").and_then(Value::as_str))
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default();
    assert!(reliabilities.contains(&"primary".to_string()));
    assert!(reliabilities.contains(&"oral".to_string()));
    assert!(
        reliabilities.len() >= 4,
        "several grades, so the badges differ: {reliabilities:?}"
    );

    // Confidence must actually vary, or the headline feature shows nothing.
    let mut confs: Vec<f64> = Vec::new();
    for p in bundle["persons"].as_object().unwrap().values() {
        for k in ["birth", "death"] {
            if let Some(c) = p
                .pointer(&format!("/{k}/confidence"))
                .and_then(Value::as_f64)
            {
                confs.push(c);
            }
        }
    }
    let lo = confs.iter().cloned().fold(f64::MAX, f64::min);
    let hi = confs.iter().cloned().fold(0.0f64, f64::max);
    assert!(
        hi - lo > 0.5,
        "confidence must span a wide range or the rendering shows nothing: \
         {lo}..{hi}"
    );

    // Parentage confidence, which GEDCOM cannot carry at all.
    let mut parent_confs: Vec<f64> = Vec::new();
    for fam in bundle["families"].as_object().unwrap().values() {
        if let Some(children) = fam.get("children").and_then(Value::as_array) {
            for c in children {
                if let Some(v) = c.get("confidence").and_then(Value::as_f64) {
                    parent_confs.push(v);
                }
            }
        }
    }
    let plo = parent_confs.iter().cloned().fold(f64::MAX, f64::min);
    let phi = parent_confs.iter().cloned().fold(0.0f64, f64::max);
    assert!(
        phi - plo > 0.4,
        "tree connectors need differing parentage confidence: {plo}..{phi}"
    );

    // Occupations as spans, not bare titles.
    let with_span = bundle["occupations"]
        .as_object()
        .map(|m| {
            m.values()
                .filter(|o| o.get("valid_from").is_some() && o.get("valid_until").is_some())
                .count()
        })
        .unwrap_or(0);
    assert!(
        with_span >= 3,
        "occupations need both bounds to render as bars, found {with_span}"
    );

    // Every date shape the format distinguishes.
    let mut kinds: Vec<&str> = Vec::new();
    for p in bundle["persons"].as_object().unwrap().values() {
        for k in ["birth", "death"] {
            if let Some(f) = p.get(k) {
                let d = axgf_cms::view::render_date_field(f, "date");
                kinds.push(d.kind);
            }
        }
    }
    for want in ["exact", "approximate", "range", "preserved"] {
        assert!(
            kinds.contains(&want),
            "the sample must include a {want} date; got {kinds:?}"
        );
    }

    // A place whose country changed under it.
    let with_history = bundle["places"]
        .as_object()
        .map(|m| {
            m.values()
                .filter(|p| {
                    p.get("country_history")
                        .and_then(Value::as_array)
                        .is_some_and(|a| a.len() > 1)
                })
                .count()
        })
        .unwrap_or(0);
    assert!(with_history >= 1, "a place with border history");
}

#[tokio::test]
async fn the_sample_bundle_serves_every_page() {
    let path = sample_axgf();
    if !path.exists() {
        panic!("deploy/sample.axgf is missing; regenerate it");
    }
    let (app, _p) = common::app_with_bundle("sample-serve", Path::new(&path));

    for uri in ["/", "/tree", "/convert", "/health"] {
        let resp = common::get(&app, uri).await;
        assert_eq!(
            resp.status(),
            axum::http::StatusCode::OK,
            "{uri} should render for the sample bundle"
        );
    }

    // The home page should advertise the features the sample actually has.
    let home = common::body_string(common::get(&app, "/").await).await;
    assert!(home.contains("non-family relationships"), "{home}");
    assert!(home.contains("occupations recorded as spans"));

    // And a person page should show a link section with real content.
    let tree = common::body_string(common::get(&app, "/tree").await).await;
    let id = tree
        .split("href=\"/person/")
        .nth(1)
        .and_then(|s| s.split('"').next())
        .expect("a person link");
    let page = common::body_string(common::get(&app, &format!("/person/{id}")).await).await;
    assert!(page.contains("links-section"));
}
