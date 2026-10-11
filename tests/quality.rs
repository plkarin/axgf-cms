//! The data-quality detectors: each fires exactly once on a bundle carrying
//! exactly one instance of its anomaly, none fires on a clean bundle, and
//! none tells a reader anything about a person they may not see.

mod common;

use std::collections::BTreeMap;
use std::io::Read;

use axgf_cms::access::Lens;
use axgf_cms::acl::Visibility;
use axgf_cms::quality::{self, Rule};
use serde_json::{json, Map, Value};

fn person(id: &str, given: &str, surname: &str, born: Option<&str>, died: Option<&str>) -> Value {
    let mut p = json!({"id": id, "type": "person", "axgf_version": "1.1", "version_num": 1,
        "identity": {"name": {"display": format!("{given} {surname}"),
            "components": [{"type": "given_name", "value": given},
                           {"type": "family_name", "value": surname}]},
            "gender": {"value": "U"}, "is_living": false, "visibility": "public"}});
    if let Some(b) = born {
        p["birth"] = json!({"date": {"value": b, "precision": "year"}, "confidence": 0.8});
    }
    if let Some(d) = died {
        p["death"] = json!({"date": {"value": d, "precision": "year"}, "confidence": 0.8});
    }
    p
}

fn family(id: &str, partners: &[&str], kids: &[&str], start: Option<&str>) -> Value {
    let mut f = json!({"id": id, "type": "family", "axgf_version": "1.1", "version_num": 1,
        "union": {"type": "marriage",
            "persons": partners.iter().map(|p| json!({"person_id": p, "role": "spouse"})).collect::<Vec<_>>()},
        "children": kids.iter().map(|k| json!({"person_id": k, "confidence": 0.9})).collect::<Vec<_>>()});
    if let Some(s) = start {
        f["union"]["start"] = json!({"date": {"value": s, "precision": "year"}});
    }
    f
}

fn place(id: &str, name: &str, region: Option<&str>) -> Value {
    let mut p = json!({"id": id, "type": "place", "axgf_version": "1.0", "version_num": 1,
        "names": [{"lang": "pl", "value": name, "is_primary": true}]});
    if let Some(r) = region {
        p["region"] = json!(r);
    }
    p
}

fn bundle(persons: Vec<Value>, families: Vec<Value>, places: Vec<Value>) -> Value {
    let map = |v: Vec<Value>| -> Map<String, Value> {
        v.into_iter()
            .map(|e| (e["id"].as_str().unwrap().to_string(), e))
            .collect()
    };
    json!({
        "manifest": {"axgf": "1.1", "created_at": "2026-10-10T00:00:00Z"},
        "persons": map(persons), "families": map(families), "places": map(places),
        "events": {}, "occupations": {}, "links": {}, "sources": {}, "documents": {}
    })
}

/// A family with nothing wrong in it, and a place with a region.
fn clean() -> (Vec<Value>, Vec<Value>, Vec<Value>) {
    (
        vec![
            person("pA", "Jan", "Nowak", Some("1900"), Some("1970")),
            person("pB", "Anna", "Nowak", Some("1902"), None),
            person("pC", "Piotr", "Nowak", Some("1930"), None),
        ],
        vec![family("fA", &["pA", "pB"], &["pC"], Some("1925"))],
        vec![place("pl1", "Lublin", Some("Lubelskie"))],
    )
}

/// The clean bundle, plus exactly one instance of every anomaly.
fn one_of_each() -> Value {
    let (mut ps, mut fs, mut pls) = clean();
    // Names: an archivist's sentence in a given name.
    ps.push(person(
        "n1",
        "Anna - w dokumentach występuje także pod tym imieniem",
        "Nowak",
        Some("1931"),
        None,
    ));
    fs[0]["children"]
        .as_array_mut()
        .unwrap()
        .push(json!({"person_id": "n1", "confidence": 0.9}));
    // Relations.
    fs.push(family("fU", &["pA", "pC"], &[], None)); // a father and his son as a couple
    ps.extend([
        person("d1", "Marek", "Kowal", Some("1880"), None),
        person("d2", "Ewa", "Kowal", Some("1882"), None),
        person("dk", "Jan", "Kowal", Some("1905"), None),
    ]);
    fs.push(family("fD", &["d1", "d2"], &["dk", "dk"], None)); // one child listed twice
    ps.extend([
        person("e1", "Adam", "Lis", Some("1850"), None),
        person("e2", "Maria", "Lis", Some("1852"), None),
    ]);
    fs.push(family("fE1", &["e1", "e2"], &[], None));
    fs.push(family("fE2", &["e1", "e2"], &[], None)); // the same couple twice
    ps.push(person("q1", "Tomasz", "Bem", Some("1860"), None));
    fs.push(family("fP", &["q1", "q1"], &[], None)); // one person on both sides
    ps.extend([
        person("f1", "Józef", "Wrona", Some("1800"), None),
        person("f2", "Zofia", "Wrona", Some("1805"), None),
        person("s1", "Jan", "Wrona", Some("1825"), None),
        person("s2", "Ewa", "Wrona", Some("1827"), None),
        person("s3", "Adam", "Kos", Some("1829"), None),
        person("s4", "Ida", "Sowa", Some("1831"), None),
    ]);
    fs.push(family("fF", &["f1", "f2"], &["s1", "s2", "s3", "s4"], None)); // 2 of 4 foreign surnames
                                                                           // Dates.
    ps.push(person("x1", "Karol", "Mak", Some("1900"), Some("1890"))); // death before birth
    ps.extend([
        person("g1", "Leon", "Baran", Some("1900"), None),
        person("g2", "Ida", "Baran", Some("1870"), None),
        person("gk", "Ola", "Baran", Some("1899"), None),
    ]);
    fs.push(family("fG", &["g1", "g2"], &["gk"], None)); // born before one parent
    ps.extend([
        person("h1", "Rafał", "Pyrek", Some("1880"), None),
        person("h2", "Ola", "Pyrek", None, None),
        person("hk", "Ewa", "Pyrek", Some("1890"), None),
    ]);
    fs.push(family("fH", &["h1", "h2"], &["hk"], None)); // a parent of ten
    ps.extend([
        person("i1", "Stefan", "Gil", Some("1800"), None),
        person("i2", "Rita", "Gil", None, None),
        person("ik", "Jan", "Gil", Some("1870"), None),
    ]);
    fs.push(family("fI", &["i1", "i2"], &["ik"], None)); // a parent of seventy
    ps.extend([
        person("j1", "Witold", "Ryba", Some("1900"), None),
        person("j2", "Hanna", "Ryba", Some("1880"), None),
    ]);
    fs.push(family("fJ", &["j1", "j2"], &[], Some("1890"))); // married before he was born
    ps.push(person("z1", "Zenon", "Samotny", None, None)); // no dates, nobody
                                                           // Places.
    pls.push(place("pl2", "Kraków", None)); // no region
    pls.push(place("pl3", "Opatów", Some("dom opieki"))); // a region that is not a place
    pls.push(place("pl4", "Łodź", Some("Łódzkie")));
    pls.push(place("pl5", "Łódź", Some("Łódzkie"))); // the same city twice
    pls.push(place("pl6", "dom Janiny", Some("Mazowieckie"))); // not a place
                                                               // Evidence: certain, and sourced by nothing.
    let mut w = person("w1", "Wanda", "Sroka", Some("1910"), None);
    w["birth"]["confidence"] = json!(1.0);
    ps.push(w);
    bundle(ps, fs, pls)
}

#[test]
fn every_detector_fires_exactly_once_on_one_instance_of_its_anomaly() {
    let flat = one_of_each();
    let mut counts: BTreeMap<Rule, usize> = BTreeMap::new();
    for f in quality::detect(&flat) {
        *counts.entry(f.rule).or_default() += 1;
    }
    for rule in Rule::ALL {
        assert_eq!(
            counts.get(&rule).copied().unwrap_or(0),
            1,
            "{}: {counts:?}",
            rule.slug()
        );
        // And each one runs on its own.
        assert_eq!(quality::run(&flat, rule).len(), 1, "{} alone", rule.slug());
    }
}

#[test]
fn no_detector_fires_on_a_clean_bundle() {
    let (ps, fs, pls) = clean();
    let findings = quality::detect(&bundle(ps, fs, pls));
    assert!(findings.is_empty(), "{findings:?}");
}

/// A reader who may not see a living person learns nothing about them here:
/// not their name, not their dates, not that their record has a problem.
/// Their finding is not blanked — it is not there, and the count is the
/// count without it.
#[test]
fn a_hidden_person_leaves_no_trace_in_the_findings() {
    let (mut ps, fs, pls) = clean();
    let mut hidden = person(
        "v1",
        "Ukryta - w dokumentach inaczej zapisana",
        "Tajna",
        Some("1990"),
        Some("1980"),
    );
    hidden["identity"]["is_living"] = json!(true);
    hidden["identity"]["visibility"] = json!("members");
    ps.push(hidden);
    let flat = bundle(ps, fs, pls);

    let admin = quality::for_reader(&flat, &Lens::unrestricted());
    assert!(
        admin.iter().any(|f| f.persons.contains(&"v1".to_string())),
        "an administrator sees it"
    );

    let visitor = Lens::resolve(&flat, Visibility::Public);
    let shown = quality::for_reader(&flat, &visitor);
    let text = serde_json::to_string(&shown).unwrap();
    for trace in ["v1", "Ukryta", "Tajna", "1990", "1980"] {
        assert!(!text.contains(trace), "{trace} reached a visitor: {text}");
    }
    let (ps, fs, pls) = clean();
    assert_eq!(
        shown.len(),
        quality::for_reader(&bundle(ps, fs, pls), &visitor).len(),
        "the visitor's count is the count of a bundle without the hidden person"
    );
}

/// Over the operator's 866-person archive: how often each rule fires.
#[test]
#[ignore = "needs AXGF_CMS_BENCH_BUNDLE"]
fn the_operators_archive_per_rule() {
    let path = std::env::var("AXGF_CMS_BENCH_BUNDLE").expect("AXGF_CMS_BENCH_BUNDLE");
    let mut zip = zip::ZipArchive::new(std::fs::File::open(path).unwrap()).unwrap();
    let mut flat = json!({"persons": {}, "families": {}, "places": {}, "events": {}, "occupations": {}, "links": {}});
    for i in 0..zip.len() {
        let mut e = zip.by_index(i).unwrap();
        let name = e.name().to_string();
        let Some((kind, _)) = name.split_once('/') else {
            continue;
        };
        if !name.ends_with(".json") || flat.get(kind).is_none() {
            continue;
        }
        let mut s = String::new();
        e.read_to_string(&mut s).unwrap();
        let v: Value = serde_json::from_str(&s).unwrap();
        let id = v["id"].as_str().unwrap().to_string();
        flat[kind][id] = v;
    }
    let findings = quality::detect(&flat);
    let mut counts: BTreeMap<Rule, usize> = BTreeMap::new();
    for f in &findings {
        *counts.entry(f.rule).or_default() += 1;
    }
    println!("{} findings", findings.len());
    // Every foreign-surnames family in full, to judge the rule by.
    let name = |id: &str| -> String {
        flat["persons"][id]["identity"]["name"]["display"]
            .as_str()
            .unwrap_or("?")
            .to_string()
    };
    for f in findings.iter().filter(|f| f.rule == Rule::ForeignSurnames) {
        let fam = &flat["families"][&f.id];
        let partners: Vec<String> = fam["union"]["persons"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|p| p["person_id"].as_str())
            .map(name)
            .collect();
        let kids: Vec<String> = fam["children"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|k| k["person_id"].as_str())
            .map(name)
            .collect();
        println!(
            "  FAMILY {} · parents: {} · {} → children: {}",
            &f.id[..8],
            partners.join(" & "),
            f.args["who"],
            kids.join("; ")
        );
    }
    for rule in Rule::ALL {
        println!(
            "  {:<24} {:>4}",
            rule.slug(),
            counts.get(&rule).copied().unwrap_or(0)
        );
    }
    for rule in [
        Rule::NameSentence,
        Rule::UnionParentChild,
        Rule::DuplicateUnion,
        Rule::ForeignSurnames,
        Rule::ChildListedTwice,
        Rule::RegionFreeText,
        Rule::PlaceNearDuplicate,
        Rule::NotAPlace,
    ] {
        for f in findings.iter().filter(|f| f.rule == rule).take(6) {
            println!(
                "    {} {} {:?}",
                rule.slug(),
                &f.id[..8.min(f.id.len())],
                f.args
            );
        }
    }
}

/// A dismissal is a person's decision, recorded on the record through the
/// edit path: the finding leaves the open list, is kept behind the filter
/// with who, when and why, and the record's version goes up by one, like any
/// other edit. A dismissal made from a stale list is refused, not merged.
#[tokio::test]
async fn a_finding_is_dismissed_through_the_edit_path_and_kept_behind_the_filter() {
    let src = common::scratch("quality-dismiss-src");
    let path = src.join("q.axgf");
    std::fs::write(
        &path,
        axgf_cms::state::export_to_bytes(&one_of_each().to_string()).unwrap(),
    )
    .unwrap();
    let (app, _s) = common::app_with_bundle("quality-dismiss", &path);
    drop(src);
    let cookie = common::admin_cookie(&app).await;
    let list = |c: String| {
        let app = app.clone();
        async move {
            let body =
                common::body_string(common::get_with_cookie(&app, "/quality.json", &c).await).await;
            serde_json::from_str::<Vec<Value>>(&body).unwrap()
        }
    };
    let before = list(cookie.clone()).await;
    let f = before
        .iter()
        .find(|f| f["rule"] == "place-no-region")
        .expect("Kraków has no region")
        .clone();
    let (fp, version) = (
        f["fingerprint"].as_str().unwrap().to_string(),
        f["version"].as_u64().unwrap(),
    );

    let body = format!(
        "kind=place&id=pl2&fingerprint={}&base_version={version}&note=ours&back=/quality",
        fp.replace(':', "%3A")
    );
    let resp = common::post_form_as(&app, &cookie, "/quality/dismiss", &body).await;
    assert!(resp.status().is_redirection(), "{}", resp.status());

    let after = list(cookie.clone()).await;
    let g = after
        .iter()
        .find(|x| x["fingerprint"] == fp.as_str())
        .expect("kept, not deleted");
    assert_eq!(g["dismissed"]["note"], "ours");
    assert_eq!(
        g["version"].as_u64().unwrap(),
        version + 1,
        "a dismissal is an edit like any other"
    );
    let open =
        common::body_string(common::get_with_cookie(&app, "/quality?group=places", &cookie).await)
            .await;
    let checked = common::body_string(
        common::get_with_cookie(&app, "/quality?show=dismissed", &cookie).await,
    )
    .await;
    assert!(
        !open.contains("Kraków has no region") && !open.contains(">Kraków"),
        "gone from the open list"
    );
    assert!(
        checked.contains("Kraków") && checked.contains("ours"),
        "and behind the filter, with the note"
    );

    // From a list rendered before that change: refused, nothing written.
    let h = after.iter().find(|x| x["rule"] == "not-a-place").unwrap();
    let stale = format!(
        "kind=place&id={}&fingerprint={}&base_version={}&back=/quality",
        h["id"].as_str().unwrap(),
        h["fingerprint"].as_str().unwrap().replace(':', "%3A"),
        h["version"].as_u64().unwrap() + 5
    );
    let resp = common::post_form_as(&app, &cookie, "/quality/dismiss", &stale).await;
    assert!(
        !resp.status().is_redirection(),
        "a stale dismissal is not applied"
    );
    let again = list(cookie).await;
    assert!(again.iter().find(|x| x["rule"] == "not-a-place").unwrap()["dismissed"].is_null());
}

/// The finding's identity carries a hash of what it is about, so a dismissal
/// covers that value only: change the value and the finding is back.
#[test]
fn changing_the_value_brings_a_dismissed_finding_back() {
    let mut flat = one_of_each();
    let f = quality::run(&flat, Rule::RegionFreeText).remove(0);
    flat["places"][&f.id]["extensions"] =
        json!({ quality::DISMISSED_KEY: [{"fingerprint": f.fingerprint, "by": "test"}] });
    let now = quality::detect(&flat)
        .into_iter()
        .find(|x| x.rule == Rule::RegionFreeText)
        .unwrap();
    assert!(
        now.dismissed.is_some(),
        "dismissed while the value is the same"
    );
    flat["places"][&f.id]["region"] = json!("tam pochowana");
    let now = quality::detect(&flat)
        .into_iter()
        .find(|x| x.rule == Rule::RegionFreeText)
        .unwrap();
    assert!(
        now.dismissed.is_none(),
        "a different value is a different finding"
    );
    assert_ne!(now.fingerprint, f.fingerprint);
}
