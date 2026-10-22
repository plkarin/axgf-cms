//! The GEDCOM import campaign's probe (docs/GEDCOM-IMPORT.md): every GEDCOM
//! in a directory read, converted, validated, exported, imported back and
//! served, page by page, each file in its own process so that a crash or a
//! hang is a finding rather than the end of the run. Ignored by the suite: it
//! reads a corpus, and the suite's own tests hold what the corpus found.
//!
//!   python3 scripts/gedcom-corpus.py CORPUS
//!   PROBE_DIR=CORPUS PROBE_OUT=REPORTS \
//!     cargo test --release --test gedcom_probe -- --ignored --exact probe_all --nocapture
//!
//! `PROBE_ONLY=a,b` keeps the files whose names hold one of those words,
//! `PROBE_TIMEOUT` is the seconds a file may take (180), `PROBE_PERSONS` the
//! people whose own pages are opened (12). REPORTS gets a JSON report per
//! file, the child's output, and `summary.json`.

mod common;

use std::time::{Duration, Instant};

use axum::body::Body;
use axum::http::{header, Request};
use common::*;
use serde_json::{json, Map, Value};
use tower::ServiceExt;

const MAX: usize = axgf_cms::convert::MAX_UPLOAD;

fn ms(t: Instant) -> u64 {
    t.elapsed().as_millis() as u64
}

fn summarize(diags: &[axgf_rs::boundary::envelope::Diagnostic]) -> Value {
    let mut by: std::collections::BTreeMap<String, usize> = Default::default();
    for d in diags {
        *by.entry(format!("{}/{:?}", d.code.as_str(), d.severity))
            .or_default() += 1;
    }
    let first: Vec<String> = diags
        .iter()
        .take(12)
        .map(|d| {
            format!(
                "{}: {}",
                d.code.as_str(),
                d.message.chars().take(200).collect::<String>()
            )
        })
        .collect();
    json!({"by_code": by, "first": first})
}

fn emit(rep: &Map<String, Value>) {
    println!("PROBE_JSON {}", Value::Object(rep.clone()));
}

#[test]
#[ignore]
fn probe_all() {
    let dir = std::env::var("PROBE_DIR").expect("PROBE_DIR");
    let out = std::path::PathBuf::from(std::env::var("PROBE_OUT").expect("PROBE_OUT"));
    let only = std::env::var("PROBE_ONLY").ok();
    let timeout: u64 = std::env::var("PROBE_TIMEOUT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(180);
    std::fs::create_dir_all(&out).unwrap();
    let mut files: Vec<_> = std::fs::read_dir(&dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "ged" || x == "gdz"))
        .collect();
    files.sort();
    let exe = std::env::current_exe().unwrap();
    let mut summary = Vec::new();
    for f in files {
        let name = f.file_name().unwrap().to_string_lossy().to_string();
        if only
            .as_deref()
            .is_some_and(|o| !o.split(',').any(|o| name.contains(o)))
        {
            continue;
        }
        let so = out.join(format!("{name}.stdout"));
        let se = out.join(format!("{name}.stderr"));
        let t = Instant::now();
        let mut child = std::process::Command::new(&exe)
            .args([
                "--ignored",
                "--exact",
                "probe_one",
                "--nocapture",
                "--test-threads=1",
            ])
            .env("PROBE_FILE", &f)
            .stdout(std::fs::File::create(&so).unwrap())
            .stderr(std::fs::File::create(&se).unwrap())
            .spawn()
            .unwrap();
        let status = loop {
            if let Some(s) = child.try_wait().unwrap() {
                break Some(s);
            }
            if t.elapsed() > Duration::from_secs(timeout) {
                let _ = child.kill();
                let _ = child.wait();
                break None;
            }
            std::thread::sleep(Duration::from_millis(20));
        };
        // A child killed or crashed never dropped its scratch directories;
        // they are its, by the process id in their names (common::scratch).
        if !status.as_ref().is_some_and(|s| s.success()) {
            let prefix = format!("axgf-cms-it-{}-", child.id());
            for e in std::fs::read_dir(scratch_base())
                .into_iter()
                .flatten()
                .flatten()
            {
                if e.file_name().to_string_lossy().starts_with(&prefix) {
                    let _ = std::fs::remove_dir_all(e.path());
                }
            }
        }
        let stdout = std::fs::read_to_string(&so).unwrap_or_default();
        let stderr = std::fs::read_to_string(&se).unwrap_or_default();
        // The child reports twice — after the conversion, and again with the
        // pages — so that a crash while serving still leaves the first.
        let report = stdout
            .lines()
            .rev()
            .find_map(|l| l.strip_prefix("PROBE_JSON "))
            .and_then(|j| serde_json::from_str::<Value>(j).ok());
        let last_page = stdout
            .lines()
            .rev()
            .find_map(|l| l.strip_prefix("PROBE_PAGE "))
            .unwrap_or("")
            .to_string();
        let outcome = match (&status, &report) {
            (None, _) => format!("TIMEOUT after {timeout}s (last page: {last_page})"),
            (Some(s), Some(_)) if s.success() => "ok".to_string(),
            (Some(s), _) => {
                let panic = stderr
                    .lines()
                    .find(|l| {
                        l.contains("panicked at") || l.contains("overflow") || l.contains("SIGSEGV")
                    })
                    .unwrap_or("")
                    .to_string();
                let next = stderr
                    .lines()
                    .skip_while(|l| !l.contains("panicked at"))
                    .nth(1)
                    .unwrap_or("")
                    .to_string();
                format!("CRASH {s:?} last page: {last_page} | {panic} {next}")
            }
        };
        let mut rep = report.unwrap_or(json!({}));
        rep["outcome"] = json!(outcome);
        rep["wall_ms"] = json!(ms(t));
        std::fs::write(
            out.join(format!("{name}.json")),
            serde_json::to_string_pretty(&rep).unwrap(),
        )
        .unwrap();
        let c = &rep["counts"];
        println!(
            "{name:40} {outcome:10} {:>6}ms persons={} fam={} ev={} pl={} src={} doc={} occ={} conv={} valid={} {} export={} pages_bad={}",
            ms(t),
            c["persons"], c["families"], c["events"], c["places"], c["sources"], c["documents"], c["occupations"],
            rep["convert_diags"]["by_code"],
            rep["validate_status"],
            rep["validate_diags"]["by_code"],
            rep.get("export_bytes").map(|v| v.to_string()).or(rep.get("export_error").map(|v| v.to_string())).unwrap_or_default(),
            rep["pages_bad"],
        );
        summary.push(rep);
    }
    std::fs::write(
        out.join("summary.json"),
        serde_json::to_string_pretty(&summary).unwrap(),
    )
    .unwrap();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn probe_one() {
    let path = std::env::var("PROBE_FILE").expect("PROBE_FILE");
    let bytes = std::fs::read(&path).unwrap();
    let mut rep = Map::new();
    rep.insert("file".into(), json!(path));

    // The upload through the real route first: what a visitor sees.
    let (app0, _s0) = app_with_empty_bundle("probe-conv");
    let t = Instant::now();
    let body = multipart(
        std::path::Path::new(&path)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap(),
        &bytes,
    );
    println!("PROBE_PAGE POST /convert/gedcom");
    let resp = app0
        .clone()
        .oneshot(
            Request::builder()
                .uri("/convert/gedcom")
                .method("POST")
                .header(
                    header::CONTENT_TYPE,
                    format!("multipart/form-data; boundary={B}"),
                )
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();
    let st = resp.status().as_u16();
    let page = body_string(resp).await;
    rep.insert(
        "http_convert".into(),
        json!({"status": st, "ms": ms(t), "download": page.contains("/convert/download/"), "len": page.len()}),
    );

    let t = Instant::now();
    let read = match axgf_cms::gedcom_in::read(&bytes, MAX) {
        Ok(r) => r,
        Err(e) => {
            rep.insert("refused".into(), json!(format!("{e:?}")));
            emit(&rep);
            return;
        }
    };
    rep.insert("t_read_ms".into(), json!(ms(t)));
    rep.insert(
        "reading".into(),
        serde_json::to_value(&read.reading).unwrap(),
    );

    let t = Instant::now();
    let env = axgf_rs::convert_gedcom(read.text.as_bytes(), 0.8, "fr");
    rep.insert("t_convert_ms".into(), json!(ms(t)));
    rep.insert("convert_status".into(), json!(format!("{:?}", env.status)));
    rep.insert("convert_diags".into(), summarize(&env.diagnostics));
    let data = match axgf_cms::state::envelope_into_data(env) {
        Ok(d) => d,
        Err(e) => {
            rep.insert("convert_error".into(), json!(e.to_string()));
            emit(&rep);
            return;
        }
    };
    let bundle = data["bundle"].clone();
    let counts: Map<String, Value> = axgf_cms::convert::counts_of(&bundle)
        .into_iter()
        .map(|(k, n)| (k.to_string(), json!(n)))
        .collect();
    rep.insert("counts".into(), Value::Object(counts));
    let flat = bundle.to_string();

    let t = Instant::now();
    let v = axgf_rs::validate(&flat);
    rep.insert("t_validate_ms".into(), json!(ms(t)));
    rep.insert("validate_status".into(), json!(format!("{:?}", v.status)));
    rep.insert("validate_diags".into(), summarize(&v.diagnostics));

    let t = Instant::now();
    let zip = match axgf_cms::state::export_to_bytes(&flat) {
        Ok(z) => {
            rep.insert("export_bytes".into(), json!(z.len()));
            let imp = axgf_rs::import_bundle(&z);
            rep.insert("import_status".into(), json!(format!("{:?}", imp.status)));
            rep.insert("import_diags".into(), summarize(&imp.diagnostics));
            Some(z)
        }
        Err(e) => {
            rep.insert(
                "export_error".into(),
                json!(e.to_string().chars().take(500).collect::<String>()),
            );
            None
        }
    };
    rep.insert("t_export_ms".into(), json!(ms(t)));

    let b = bundle.clone();
    rep.insert(
        "completeness".into(),
        json!(std::panic::catch_unwind(move || {
            axgf_cms::completeness::analyse(&b, "fr");
        })
        .is_ok()),
    );
    let b = bundle.clone();
    rep.insert(
        "quality_findings".into(),
        json!(std::panic::catch_unwind(move || axgf_cms::quality::detect(&b).len()).ok()),
    );

    // People, for reading by eye.
    let persons: Vec<Value> = bundle["persons"]
        .as_object()
        .map(|m| {
            let mut v: Vec<&Value> = m.values().collect();
            v.sort_by_key(|p| p["identity"]["name"]["display"].as_str().unwrap_or("").to_string());
            v.into_iter()
                .take(150)
                .map(|p| {
                    json!({
                        "id": p["id"],
                        "name": p["identity"]["name"],
                        "names": p["identity"]["names"],
                        "gender": p["identity"]["gender"],
                        "is_living": p["identity"]["is_living"],
                        "birth": p["birth"],
                        "death": p["death"],
                        "notes": p["notes"].as_str().map(|s| s.chars().take(300).collect::<String>()),
                        "documents": p["documents"],
                    })
                })
                .collect()
        })
        .unwrap_or_default();
    rep.insert("persons".into(), json!(persons));
    rep.insert(
        "families".into(),
        json!(bundle["families"].as_object().map(|m| m
            .values()
            .take(20)
            .cloned()
            .collect::<Vec<_>>())),
    );
    rep.insert(
        "places".into(),
        json!(bundle["places"].as_object().map(|m| m
            .values()
            .take(40)
            .map(|p| p["names"].clone())
            .collect::<Vec<_>>())),
    );
    rep.insert(
        "documents".into(),
        json!(bundle["documents"].as_object().map(|m| m
            .values()
            .take(20)
            .cloned()
            .collect::<Vec<_>>())),
    );
    rep.insert(
        "sources".into(),
        json!(bundle["sources"].as_object().map(|m| m
            .values()
            .take(10)
            .cloned()
            .collect::<Vec<_>>())),
    );
    rep.insert(
        "occupations".into(),
        json!(bundle["occupations"].as_object().map(|m| m
            .values()
            .take(10)
            .cloned()
            .collect::<Vec<_>>())),
    );
    rep.insert(
        "events".into(),
        json!(bundle["events"].as_object().map(|m| m
            .values()
            .take(10)
            .cloned()
            .collect::<Vec<_>>())),
    );
    emit(&rep);

    // The family served: the pages a visitor and an administrator open.
    let Some(zip) = zip else { return };
    let dir = scratch("probe-app");
    let p = dir.join("family.axgf");
    std::fs::write(&p, &zip).unwrap();
    let t = Instant::now();
    let app = match build_app(&p) {
        Ok(a) => a,
        Err(e) => {
            rep.insert("app_error".into(), json!(e.to_string()));
            emit(&rep);
            return;
        }
    };
    rep.insert("t_load_ms".into(), json!(ms(t)));
    let cookie = admin_cookie(&app).await;
    let mut ids: Vec<String> = bundle["persons"]
        .as_object()
        .map(|m| m.keys().cloned().collect())
        .unwrap_or_default();
    let n: usize = std::env::var("PROBE_PERSONS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(12);
    ids.truncate(n);
    let mut pages: Vec<String> = [
        "/",
        "/river/data",
        "/river/data?n=5",
        "/about",
        "/tree",
        "/tree?depth=6",
        "/quality",
        "/quality.json",
        "/search?q=a",
        "/search.json?q=e",
        "/admin",
        "/admin/bundle",
        "/admin/persons",
        "/admin/families",
        "/admin/places",
        "/admin/sources",
        "/admin/documents",
        "/admin/events",
        "/admin/occupations",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    for id in &ids {
        pages.push(format!("/person/{id}"));
        pages.push(format!("/river/data?p={id}&n=5"));
        pages.push(format!("/tree?root={id}&depth=8"));
        pages.push(format!("/tree/panel/{id}"));
        pages.push(format!("/admin/person/{id}/edit"));
    }
    let mut bad = Vec::new();
    let mut slow = Vec::new();
    for uri in &pages {
        println!("PROBE_PAGE {uri}");
        let t = Instant::now();
        let r = tokio::time::timeout(Duration::from_secs(90), get_with_cookie(&app, uri, &cookie))
            .await;
        match r {
            Err(_) => bad.push(json!({"uri": uri, "status": "timeout"})),
            Ok(resp) => {
                let s = resp.status().as_u16();
                let _ = body_bytes(resp).await;
                if s >= 500 {
                    bad.push(json!({"uri": uri, "status": s}));
                }
            }
        }
        if t.elapsed() > Duration::from_millis(1500) {
            slow.push(json!({"uri": uri, "ms": ms(t)}));
        }
    }
    rep.insert("pages".into(), json!(pages.len()));
    rep.insert("pages_bad".into(), json!(bad));
    rep.insert("pages_slow".into(), json!(slow));
    emit(&rep);
}

const B: &str = "----probeboundary";

fn multipart(filename: &str, content: &[u8]) -> Vec<u8> {
    let mut body = Vec::new();
    body.extend_from_slice(format!("--{B}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"{filename}\"\r\nContent-Type: application/octet-stream\r\n\r\n").as_bytes());
    body.extend_from_slice(content);
    body.extend_from_slice(b"\r\n");
    for (n, v) in [("confidence", "0.8"), ("lang", "fr")] {
        body.extend_from_slice(
            format!("--{B}\r\nContent-Disposition: form-data; name=\"{n}\"\r\n\r\n{v}\r\n")
                .as_bytes(),
        );
    }
    body.extend_from_slice(format!("--{B}--\r\n").as_bytes());
    body
}
