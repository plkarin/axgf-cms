//! A GEDCOM converted and served (v0.5.0, the stress campaign of 22 October
//! 2026): through `/convert/gedcom` as a visitor uses it, and the family it
//! makes served page by page — however strange the family. What the
//! converter itself still gets wrong is in `gedcom_known_defects.rs`.
//!
//! Every name, place and date here is invented.

mod common;

use std::time::{Duration, Instant};

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use common::*;
use serde_json::Value;
use tower::ServiceExt;

const BOUNDARY: &str = "----gedcomconversionboundary";

fn multipart(filename: &str, content: &[u8]) -> Vec<u8> {
    let mut body = Vec::new();
    body.extend_from_slice(
        format!(
            "--{BOUNDARY}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"{filename}\"\r\nContent-Type: application/octet-stream\r\n\r\n"
        )
        .as_bytes(),
    );
    body.extend_from_slice(content);
    body.extend_from_slice(b"\r\n");
    for (name, value) in [("confidence", "0.8"), ("lang", "fr")] {
        body.extend_from_slice(
            format!("--{BOUNDARY}\r\nContent-Disposition: form-data; name=\"{name}\"\r\n\r\n{value}\r\n").as_bytes(),
        );
    }
    body.extend_from_slice(format!("--{BOUNDARY}--\r\n").as_bytes());
    body
}

async fn post_convert(
    app: &axum::Router,
    filename: &str,
    content: &[u8],
) -> axum::http::Response<Body> {
    app.clone()
        .oneshot(
            Request::builder()
                .uri("/convert/gedcom")
                .method("POST")
                .header(
                    header::CONTENT_TYPE,
                    format!("multipart/form-data; boundary={BOUNDARY}"),
                )
                .body(Body::from(multipart(filename, content)))
                .unwrap(),
        )
        .await
        .expect("request")
}

fn ged(body: &[String]) -> String {
    let mut s = String::from("0 HEAD\n1 GEDC\n2 VERS 5.5.1\n2 FORM LINEAGE-LINKED\n1 CHAR UTF-8\n");
    for l in body {
        s.push_str(l);
        s.push('\n');
    }
    s.push_str("0 TRLR\n");
    s
}

fn lines(v: &[&str]) -> Vec<String> {
    v.iter().map(|s| s.to_string()).collect()
}

/// Convert as the CMS does, and package as the download is packaged.
fn converted(text: &str) -> (Value, Vec<u8>) {
    let read =
        axgf_cms::gedcom_in::read(text.as_bytes(), axgf_cms::convert::MAX_UPLOAD).expect("read");
    let env = axgf_rs::convert_gedcom(read.text.as_bytes(), 0.8, "fr");
    let bundle = axgf_cms::state::envelope_into_data(env).expect("converted")["bundle"].clone();
    let zip = axgf_cms::state::export_to_bytes(&bundle.to_string()).expect("packaged");
    (bundle, zip)
}

/// The download id on a result page.
fn download_link(page: &str) -> String {
    let at = page.find("/convert/download/").expect("a download link");
    page[at..].chars().take_while(|c| *c != '"').collect()
}

// ----------------------------------------------------------- the conversion page

/// A file that needed mending says so, in the reader's words, and its
/// download is an archive the library opens again with everyone in it.
#[tokio::test]
async fn the_page_says_what_was_mended_and_the_download_opens_again() {
    let (app, _s) = app_with_empty_bundle("gc-mended");
    // Tab-separated, one stray Windows-1252 byte (an é) in a UTF-8 file.
    let text = ged(&lines(&[
        "0\t@I1@\tINDI",
        "1\tNAME\tBasile /Fontaine/",
        "1\tSEX\tM",
        "1\tFAMS\t@F1@",
        "0\t@I2@\tINDI",
        "1\tNAME\tAdèle /Marchand/",
        "1\tFAMS\t@F1@",
        "1\tNOTE\tnée à Lyon, dit-on : RAW",
        "0\t@I3@\tINDI",
        "1\tNAME\tCéleste /Fontaine/",
        "1\tFAMC\t@F1@",
        "0\t@F1@\tFAM",
        "1\tHUSB\t@I1@",
        "1\tWIFE\t@I2@",
        "1\tCHIL\t@I3@",
    ]));
    let bytes: Vec<u8> = text
        .replace("RAW", "\u{0}")
        .bytes()
        .map(|b| if b == 0 { 0xE9 } else { b })
        .collect();
    let resp = post_convert(&app, "fontaine.ged", &bytes).await;
    let page = expect_status(resp, StatusCode::OK, "convert").await;
    assert!(
        page.contains("Lines laid out in a way the converter does not read"),
        "the mending is said"
    );
    assert!(
        page.contains("Bytes that were not UTF-8 in this UTF-8 file"),
        "and the stray byte"
    );
    let link = download_link(&page);
    let zip = body_bytes(get(&app, &link).await).await;
    let env = axgf_rs::import_bundle(&zip);
    // An import's data is the flat bundle itself.
    let data = axgf_cms::state::envelope_into_data(env).expect("the download opens");
    let persons = data["persons"].as_object().expect("persons");
    let mut names: Vec<&str> = persons
        .values()
        .map(|p| p["identity"]["name"]["display"].as_str().unwrap())
        .collect();
    names.sort();
    assert_eq!(
        names,
        ["Adèle Marchand", "Basile Fontaine", "Céleste Fontaine"]
    );
    let note = persons.values().find_map(|p| p["notes"].as_str()).unwrap();
    assert_eq!(note, "née à Lyon, dit-on : é");
    assert_eq!(data["families"].as_object().map(|m| m.len()), Some(1));
}

/// Markup in a name, a note, a place, a source's title and the upload's own
/// file name is shown as text, never run; the download's name stays one
/// header.
#[tokio::test]
async fn markup_in_the_file_and_in_its_name_is_shown_as_text() {
    let (app, _s) = app_with_empty_bundle("gc-markup");
    let text = ged(&lines(&[
        "0 @I1@ INDI",
        "1 NAME <script>alert(1)</script> /\"><img src=x onerror=alert(2)>/",
        "1 NOTE <script>alert(3)</script>",
        "1 BIRT",
        "2 PLAC <svg onload=alert(4)>",
        "0 @S1@ SOUR",
        "1 TITL <iframe src=javascript:alert(5)>",
    ]));
    let resp = post_convert(&app, "<img src=x onerror=alert(9)>\".ged", text.as_bytes()).await;
    let page = expect_status(resp, StatusCode::OK, "convert").await;
    for raw in ["<script>alert", "<img src=x", "<svg onload", "<iframe"] {
        assert!(!page.contains(raw), "{raw} reached the page unescaped");
    }
    assert!(
        page.contains("&lt;img src=x onerror=alert(9)&gt;"),
        "the file's name, escaped"
    );
    let resp = get(&app, &download_link(&page)).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let disposition = resp.headers()[header::CONTENT_DISPOSITION]
        .to_str()
        .unwrap()
        .to_string();
    assert!(
        disposition.starts_with("attachment; filename=\""),
        "{disposition}"
    );
    assert_eq!(
        disposition.matches('"').count(),
        2,
        "one quoted name: {disposition}"
    );
}

/// A messy file's findings are counted whole and listed up to a point: the
/// page that listed 32,000 of them weighed 4.7 MB.
#[tokio::test]
async fn a_messy_files_findings_are_counted_whole_and_listed_up_to_a_point() {
    let (app, _s) = app_with_empty_bundle("gc-messy");
    let mut body = Vec::new();
    for i in 0..300 {
        body.extend([
            format!("0 @I{i}@ INDI"),
            format!("1 NAME Personne{i} /Isolée/"),
            "1 BIRT".into(),
            format!("2 PLAC Bourg{i}"),
        ]);
    }
    let page = expect_status(
        post_convert(&app, "messy.ged", ged(&body).as_bytes()).await,
        StatusCode::OK,
        "convert",
    )
    .await;
    let list = &page[page.find("class=\"quality-findings\"").expect("the list")..];
    let list = &list[..list.find("</ul>").unwrap()];
    assert_eq!(list.matches("<li>").count(), 200);
    assert!(page.contains("Only the first 200 are listed here"));
    assert!(
        page.contains("600 things worth a look in this file"),
        "the count is whole"
    );
    assert!(page.len() < 400_000, "{} bytes", page.len());
}

/// Converting a large file takes seconds; the server answers everyone else
/// meanwhile. The conversion used to run on the request's own thread, and
/// two uploads at once held every thread a two-core server has. Both
/// requests here are tasks on the runtime's one worker, as they would be on
/// a server's: the test's own thread does not serve them.
#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn the_server_answers_while_a_large_file_converts() {
    let (app, _s) = app_with_empty_bundle("gc-beside");
    let text = large_family(6_000);
    let a = app.clone();
    let t = Instant::now();
    let conversion = tokio::spawn(async move {
        let r = post_convert(&a, "large.ged", text.as_bytes()).await;
        (r.status(), t.elapsed())
    });
    // Asked again and again for as long as the conversion runs; the longest
    // stretch between two answers is how long a reader would have waited.
    // (A worker held by the conversion holds the runtime's timers too, so a
    // blocked server shows as a gap between answers, not a slow one.)
    let mut last = Instant::now();
    let mut longest = Duration::ZERO;
    while !conversion.is_finished() {
        let b = app.clone();
        let status = tokio::spawn(async move { get(&b, "/health").await.status() })
            .await
            .unwrap();
        assert_eq!(status, StatusCode::OK);
        longest = longest.max(last.elapsed());
        last = Instant::now();
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    let (status, took) = conversion.await.unwrap();
    assert_eq!(status, StatusCode::OK);
    assert!(
        took > Duration::from_secs(1),
        "too quick to measure anything: {took:?}"
    );
    assert!(
        longest < took / 4,
        "for {longest:?} nobody was answered, beside a conversion of {took:?}"
    );
}

// ---------------------------------------------------------- the family served

/// Every page a reader or an administrator opens, for one family: the
/// family's own pages, and each person's — everyone's, or only those named.
async fn serve_everything(app: &axum::Router, bundle: &Value, what: &str, only: Option<&[&str]>) {
    let cookie = admin_cookie(app).await;
    let mut pages: Vec<String> = [
        "/",
        "/river/data",
        "/river/data?n=5",
        "/about",
        "/tree",
        "/tree?depth=6",
        "/quality",
        "/quality.json",
        "/quality.csv",
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
    ]
    .map(String::from)
    .to_vec();
    let ids: Vec<String> = bundle["persons"]
        .as_object()
        .map(|m| {
            m.iter()
                .filter(|(_, p)| {
                    only.is_none_or(|o| o.iter().any(|n| p["identity"]["name"]["display"] == *n))
                })
                .map(|(id, _)| id.clone())
                .collect()
        })
        .unwrap_or_default();
    if let Some(o) = only {
        assert_eq!(ids.len(), o.len(), "{what}: everyone named is there");
    }
    for id in ids {
        pages.extend([
            format!("/person/{id}"),
            format!("/tree/panel/{id}"),
            format!("/river/data?p={id}&n=5"),
            format!("/tree?root={id}&depth=8"),
            format!("/admin/person/{id}/edit"),
        ]);
    }
    for uri in pages {
        let t = Instant::now();
        let resp =
            tokio::time::timeout(Duration::from_secs(60), get_with_cookie(app, &uri, &cookie))
                .await
                .unwrap_or_else(|_| panic!("{what}: {uri} did not answer in 60 s"));
        let status = resp.status();
        let _ = body_bytes(resp).await;
        assert!(status.as_u16() < 500, "{what}: {uri} answered {status}");
        assert!(
            t.elapsed() < Duration::from_secs(20),
            "{what}: {uri} took {:?}",
            t.elapsed()
        );
    }
}

async fn served(text: &str, what: &str, only: Option<&[&str]>) -> Value {
    let (bundle, zip) = converted(text);
    let dir = scratch("gc-served");
    let path = dir.join("family.axgf");
    std::fs::write(&path, zip).unwrap();
    let app = build_app(&path).expect("the converted family loads");
    serve_everything(&app, &bundle, what, only).await;
    bundle
}

/// Names in every script and shape — right to left, CJK, emoji, joiners,
/// combining accents, markup, a 5,000-letter name, none at all — served.
#[tokio::test]
async fn names_in_every_script_and_shape_are_served() {
    let names = [
        "Jean /Dupont/ Jr.",
        "/Dupont/ Jean",
        "Jean Dupont",
        "Jean /Dupont",
        "Jean /Dupont/ /Martin/",
        "/ /",
        "",
        "Marie-Ève /de la Tour d'Auvergne/",
        "王 /小明/",
        "فاطمة /الزهراء/",
        "שרה /כהן/",
        "Zoë 😀 /Émoji/",
        "Jo\u{200d}sé /Zwj/",
        "e\u{301}mile /Combinant/",
        "<b>Gras</b> /Balise/",
        "Иван /Петров/",
        "Γιώργος /Παπαδόπουλος/",
    ];
    let mut body = Vec::new();
    for (i, n) in names.iter().enumerate() {
        body.extend([
            format!("0 @I{i}@ INDI"),
            format!("1 NAME {n}").trim_end().to_string(),
            "1 SEX U".into(),
        ]);
    }
    body.extend([
        "0 @I99@ INDI".to_string(),
        format!("1 NAME {} /Longue/", "A".repeat(5000)),
    ]);
    let bundle = served(&ged(&body), "names", None).await;
    let displays: Vec<String> = bundle["persons"]
        .as_object()
        .unwrap()
        .values()
        .map(|p| {
            p["identity"]["name"]["display"]
                .as_str()
                .unwrap()
                .to_string()
        })
        .collect();
    for want in [
        "王 小明",
        "فاطمة الزهراء",
        "שרה כהן",
        "Zoë 😀 Émoji",
        "Иван Петров",
        "Γιώργος Παπαδόπουλος",
    ] {
        assert!(displays.iter().any(|d| d == want), "{want} in {displays:?}");
    }
    assert!(
        displays.iter().all(|d| !d.is_empty()),
        "no empty display: {displays:?}"
    );
}

/// Dates in every form a family's file holds them — GEDCOM's, French,
/// German and Polish words, other calendars, ranges, periods, impossible
/// days, free text — served without a failure, and what the converter does
/// read, read right.
#[tokio::test]
async fn dates_in_every_form_are_served_and_the_ones_read_are_read_right() {
    let dates = [
        ("12 MAR 1850", Some(("1850-03-12", "exact"))),
        ("MAR 1850", Some(("1850-03", "month"))),
        ("1850", Some(("1850", "year"))),
        ("12 mar 1850", Some(("1850-03-12", "exact"))),
        ("15 MAR 850", Some(("0850-03-15", "exact"))),
        ("   12   MAR   1850  ", Some(("1850-03-12", "exact"))),
        ("ABT 1850", Some(("1850", "year"))),
        ("12 MARS 1850", Some(("1850-03-12", "exact"))),
        ("12 AOÛT 1850", Some(("1850-08-12", "exact"))),
        ("1 JANVIER 1850", Some(("1850-01-01", "exact"))),
        ("12 MÄRZ 1850", Some(("1850-03-12", "exact"))),
        ("12 KWIETNIA 1850 R.", Some(("1850-04-12", "exact"))),
        ("vers 1850", Some(("1850", "year"))),
        ("um 1850", Some(("1850", "year"))),
        ("około 1850", Some(("1850", "year"))),
        ("1 JAN 0001", Some(("0001-01-01", "exact"))),
        ("BEF 1850", None),
        ("AFT 1850", None),
        ("BET 1850 AND 1860", None),
        ("entre 1850 et 1860", None),
        ("zwischen 1850 und 1860", None),
        ("między 1850 i 1860", None),
        ("CAL 1850", None),
        ("EST 1850", None),
        ("FROM 1850 TO 1860", None),
        ("INT 1850 (selon la famille)", None),
        ("(vers la guerre)", None),
        ("1750/51", None),
        ("@#DJULIAN@ 1 JAN 1700", None),
        ("@#DFRENCH R@ 18 BRUM 0008", None),
        ("@#DHEBREW@ 1 TSH 5600", None),
        ("44 B.C.", None),
        ("31 FEB 1900", None),
        ("0 JAN 1900", None),
        ("1900-03-12", None),
        ("12/03/1900", None),
        ("1850s", None),
        ("unknown", None),
        ("12 MAR 1850 12:30", None),
    ];
    let mut body = Vec::new();
    for (i, (d, _)) in dates.iter().enumerate() {
        body.extend([
            format!("0 @I{i}@ INDI"),
            format!("1 NAME Date /Cas{i:02}/"),
            "1 BIRT".into(),
            format!("2 DATE {d}"),
        ]);
    }
    let bundle = served(&ged(&body), "dates", None).await;
    for (i, (d, want)) in dates.iter().enumerate() {
        let p = bundle["persons"]
            .as_object()
            .unwrap()
            .values()
            .find(|p| p["identity"]["name"]["display"] == format!("Date Cas{i:02}"))
            .unwrap();
        let got = &p["birth"]["date"];
        match want {
            Some((value, precision)) => {
                assert_eq!(got["value"], *value, "{d}: {got}");
                assert_eq!(got["precision"], *precision, "{d}: {got}");
            }
            // Not read, or read as a range: then nothing is lost — the
            // words are kept, or the bounds are.
            None => assert!(
                got.get("note").is_some()
                    || got.get("range").is_some()
                    || got.get("value").is_some(),
                "{d}: kept nowhere: {got}"
            ),
        }
    }
    for (d, side, bound) in [
        ("BEF 1850", "latest", "1850"),
        ("AFT 1850", "earliest", "1850"),
    ] {
        let i = dates.iter().position(|(x, _)| *x == d).unwrap();
        let p = bundle["persons"]
            .as_object()
            .unwrap()
            .values()
            .find(|p| p["identity"]["name"]["display"] == format!("Date Cas{i:02}"))
            .unwrap();
        assert_eq!(p["birth"]["date"]["range"][side]["value"], bound, "{d}");
    }
}

/// Families in every shape: a same-sex couple, a single parent, a sibling
/// group, an empty family, a child of itself, two people each other's
/// parent, pointers to nobody and to the wrong kind of record, three hundred
/// children, fifty partners, three hundred generations. Served; the
/// validator names what is impossible in them.
#[tokio::test]
async fn families_in_every_shape_are_served_and_the_impossible_named() {
    let mut body = lines(&[
        "0 @A1@ INDI",
        "1 NAME Un /Couple/",
        "1 SEX M",
        "0 @A2@ INDI",
        "1 NAME Deux /Couple/",
        "1 SEX M",
        "0 @FA@ FAM",
        "1 HUSB @A1@",
        "1 HUSB @A2@",
        "1 MARR",
        "0 @B1@ INDI",
        "1 NAME Seul /Parent/",
        "1 SEX F",
        "0 @B2@ INDI",
        "1 NAME Enfant /Seul/",
        "0 @FB@ FAM",
        "1 WIFE @B1@",
        "1 CHIL @B2@",
        "0 @FC@ FAM",
        "1 CHIL @B2@",
        "0 @FD@ FAM",
        "0 @FE@ FAM",
        "1 HUSB @B2@",
        "1 CHIL @B2@",
        "0 @C1@ INDI",
        "1 NAME Boucle /Un/",
        "0 @C2@ INDI",
        "1 NAME Boucle /Deux/",
        "0 @FF@ FAM",
        "1 HUSB @C1@",
        "1 CHIL @C2@",
        "0 @FG@ FAM",
        "1 HUSB @C2@",
        "1 CHIL @C1@",
        "0 @D1@ INDI",
        "1 NAME Seul /Pointeur/",
        "1 NOTE @N404@",
        "1 OBJE @M404@",
        "0 @FH@ FAM",
        "1 HUSB @D1@",
        "1 WIFE @I404@",
        "1 CHIL @I405@",
        "0 @FI@ FAM",
        "1 HUSB @FI@",
        "1 CHIL @S1@",
        "0 @S1@ SOUR",
        "1 TITL Une source",
        "0 @W0@ INDI",
        "1 NAME Patriarche /Large/",
        "1 SEX M",
        "0 @FW@ FAM",
        "1 HUSB @W0@",
    ]);
    body.extend((1..=300).map(|i| format!("1 CHIL @W{i}@")));
    for i in 1..=300 {
        body.extend([
            format!("0 @W{i}@ INDI"),
            format!("1 NAME Enfant{i} /Large/"),
        ]);
    }
    body.extend(lines(&["0 @V0@ INDI", "1 NAME Volage /Large/", "1 SEX M"]));
    for i in 1..=50 {
        body.extend([
            format!("0 @V{i}@ INDI"),
            format!("1 NAME Partenaire{i} /Large/"),
            format!("0 @FV{i}@ FAM"),
            "1 HUSB @V0@".into(),
            format!("1 WIFE @V{i}@"),
        ]);
    }
    for i in 1..=300 {
        body.extend([
            format!("0 @G{i}@ INDI"),
            format!("1 NAME Génération{i} /Chaîne/"),
            "1 BIRT".into(),
            format!("2 DATE {}", 1000 + i),
        ]);
        if i < 300 {
            body.extend([
                format!("0 @FG{i}@ FAM"),
                format!("1 HUSB @G{i}@"),
                format!("1 CHIL @G{}@", i + 1),
            ]);
        }
    }
    let text = ged(&body);
    // Each person's pages for those who carry a shape; the family's pages
    // for everyone.
    let bundle = served(
        &text,
        "shapes",
        Some(&[
            "Un Couple",
            "Seul Parent",
            "Enfant Seul",
            "Boucle Un",
            "Boucle Deux",
            "Seul Pointeur",
            "Patriarche Large",
            "Enfant150 Large",
            "Volage Large",
            "Partenaire50 Large",
            "Génération1 Chaîne",
            "Génération150 Chaîne",
            "Génération300 Chaîne",
        ]),
    )
    .await;
    let v = axgf_rs::validate(&bundle.to_string());
    let codes: Vec<&str> = v.diagnostics.iter().map(|d| d.code.as_str()).collect();
    assert!(codes.contains(&"CYCLE_DETECTED"), "{codes:?}");
    assert!(codes.contains(&"DANGLING_REFERENCE"), "{codes:?}");
    let wide = bundle["families"]
        .as_object()
        .unwrap()
        .values()
        .map(|f| f["children"].as_array().map_or(0, Vec::len))
        .max();
    assert_eq!(wide, Some(300));
}

/// Events, attributes, citations, lineage, media and places of every kind a
/// family's software writes: served, whatever the converter keeps of them.
#[tokio::test]
async fn every_kind_of_record_is_served() {
    let body = lines(&[
        "0 @I1@ INDI",
        "1 NAME Toute /Saisons/",
        "1 SEX F",
        "1 BIRT",
        "2 DATE 1 JAN 1850",
        "2 PLAC Nantes, Loire-Atlantique, France",
        "3 MAP",
        "4 LATI N47.2184",
        "4 LONG W1.5536",
        "2 SOUR @S1@",
        "3 PAGE acte 12",
        "3 QUAY 3",
        "2 NOTE né à la maison",
        "1 CHR",
        "2 DATE 3 JAN 1850",
        "1 RESI",
        "2 PLAC Rennes",
        "1 CENS",
        "2 DATE 1876",
        "1 EVEN",
        "2 TYPE Médaille",
        "1 FACT Pianiste",
        "2 TYPE Talent",
        "1 TITL Comtesse",
        "1 OCCU Couturière",
        "2 DATE FROM 1868 TO 1880",
        "2 PLAC Rennes",
        "1 OCCU",
        "1 DEAT",
        "2 DATE 1930",
        "2 CAUS Grippe",
        "1 BURI",
        "2 PLAC Cimetière fictif",
        "1 OBJE @M1@",
        "1 OBJE @M1@",
        "1 OBJE",
        "2 FILE C:\\Users\\Fictif\\portrait.JPG",
        "2 FORM JPEG",
        "1 OBJE",
        "2 FILE ../../../../etc/passwd",
        "3 FORM txt",
        "1 OBJE",
        "2 FILE https://example.org/scan.pdf",
        "1 FAMS @F1@",
        "0 @I2@ INDI",
        "1 NAME Époux /Saisons/",
        "1 SEX M",
        "1 FAMS @F1@",
        "0 @I3@ INDI",
        "1 NAME Adopté /Saisons/",
        "1 FAMC @F1@",
        "2 PEDI adopted",
        "0 @F1@ FAM",
        "1 HUSB @I2@",
        "1 WIFE @I1@",
        "1 CHIL @I3@",
        "2 _FREL Adopted",
        "1 ENGA",
        "2 DATE 1874",
        "1 MARR",
        "2 DATE 1875",
        "1 DIV",
        "2 DATE 1890",
        "0 @S1@ SOUR",
        "1 TITL Registre fictif",
        "1 AUTH Mairie fictive",
        "1 REPO @R1@",
        "0 @R1@ REPO",
        "1 NAME Archives fictives",
        "0 @M1@ OBJE",
        "1 FILE photos/portrait.jpg",
        "2 FORM jpg",
        "2 TITL Portrait",
        "0 @M2@ OBJE",
        "1 FORM bmp",
        "1 BLOB",
        "2 CONT .HM.......k.1..F.jJ..",
        "0 @U1@ SUBM",
        "1 NAME Fictif",
        "0 @X1@ _CUSTOM",
        "1 _TAG valeur",
    ]);
    served(&ged(&body), "records", None).await;
}

/// A file names documents it does not hold — a path out of the server's
/// directory, a URL, a Windows path. The site serves only what it holds:
/// asked for any of them, it says there is no such file.
#[tokio::test]
async fn documents_a_file_names_but_does_not_hold_are_never_served() {
    let text = ged(&lines(&[
        "0 @I1@ INDI",
        "1 NAME Chemins /Hostiles/",
        "1 OBJE",
        "2 FILE ../../../../etc/passwd",
        "1 OBJE",
        "2 FILE /etc/hostname",
        "1 OBJE",
        "2 FILE file:///etc/hostname",
        "1 OBJE",
        "2 FILE https://example.org/scan.pdf",
    ]));
    let (bundle, zip) = converted(&text);
    let dir = scratch("gc-docs");
    let path = dir.join("family.axgf");
    std::fs::write(&path, zip).unwrap();
    let app = build_app(&path).unwrap();
    let cookie = admin_cookie(&app).await;
    let docs: Vec<String> = bundle["documents"]
        .as_object()
        .unwrap()
        .keys()
        .cloned()
        .collect();
    assert_eq!(docs.len(), 4);
    for id in docs {
        for kind in ["raw", "view", "thumb"] {
            let resp = get_with_cookie(&app, &format!("/document/{id}/{kind}"), &cookie).await;
            let status = resp.status();
            let body = body_string(resp).await;
            assert_eq!(status, StatusCode::NOT_FOUND, "{kind}");
            assert!(
                !body.contains("root:"),
                "{kind}: a file of the server's reached the page"
            );
        }
    }
}

// --------------------------------------------------------------------- scale

/// A family of `n` people that reads like a real one: generations of
/// couples, each child born twenty to forty years after its father, places,
/// a death, an occupation and a note for some. Built here rather than
/// committed: at 40,000 people it is 9.4 MB.
fn large_family(n: usize) -> String {
    let mut seed: u64 = 7;
    let mut next = move |m: u64| {
        seed = seed
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        (seed >> 33) % m
    };
    let given = [
        "Basile", "Adèle", "Céleste", "Gaspard", "Zofia", "Lucien", "Hélène", "Jules", "Marthe",
        "Octave",
    ];
    let surnames = [
        "Fontaine", "Marchand", "Delaunay", "Zając", "Lefèvre", "Morin", "Garnier", "Nowak",
    ];
    let months = [
        "JAN", "FEB", "MAR", "APR", "MAY", "JUN", "JUL", "AUG", "SEP", "OCT", "NOV", "DEC",
    ];
    struct Born {
        male: bool,
        surname: usize,
        year: u64,
        famc: Option<usize>,
        fams: Vec<usize>,
    }
    let born = |male, surname, year, famc| Born {
        male,
        surname,
        year,
        famc,
        fams: vec![],
    };
    let mut people: Vec<Born> = Vec::new();
    let mut fams: Vec<(usize, usize, Vec<usize>)> = Vec::new();
    let mut couples: Vec<(usize, usize)> = Vec::new();
    for _ in 0..60 {
        let s = next(8) as usize;
        people.push(born(true, s, 1600 + next(10), None));
        people.push(born(false, next(8) as usize, 1600 + next(10), None));
        couples.push((people.len() - 2, people.len() - 1));
    }
    while people.len() < n && !couples.is_empty() {
        let mut men = Vec::new();
        let mut women = Vec::new();
        for (h, w) in couples.drain(..) {
            let f = fams.len();
            fams.push((h, w, vec![]));
            people[h].fams.push(f);
            people[w].fams.push(f);
            for _ in 0..[0, 1, 2, 2, 3, 3, 4, 5, 6][next(9) as usize] {
                if people.len() >= n {
                    break;
                }
                let male = next(2) == 0;
                let year = people[h].year + 20 + next(21);
                people.push(born(male, people[h].surname, year, Some(f)));
                let c = people.len() - 1;
                fams[f].2.push(c);
                if male {
                    men.push(c)
                } else {
                    women.push(c)
                }
            }
        }
        couples = men.into_iter().zip(women).collect();
    }
    let mut out =
        String::from("0 HEAD\n1 GEDC\n2 VERS 5.5.1\n2 FORM LINEAGE-LINKED\n1 CHAR UTF-8\n");
    for (
        i,
        Born {
            male,
            surname: s,
            year,
            famc,
            fams: fs,
        },
    ) in people.iter().enumerate()
    {
        out.push_str(&format!(
            "0 @I{i}@ INDI\n1 NAME {} /{}/\n1 SEX {}\n1 BIRT\n2 DATE {} {} {year}\n2 PLAC Bourg{}, Pays{}, Contrée{}\n",
            given[next(10) as usize], surnames[*s], if *male { "M" } else { "F" }, 1 + next(28), months[next(12) as usize],
            next(800), next(30), next(5)
        ));
        if *year < 1930 {
            out.push_str(&format!(
                "1 DEAT\n2 DATE {}\n2 PLAC Bourg{}, Pays{}, Contrée{}\n",
                year + 30 + next(56),
                next(800),
                next(30),
                next(5)
            ));
        }
        if next(10) < 3 {
            out.push_str("1 OCCU Tisserand\n");
        }
        if next(100) < 15 {
            out.push_str(&format!(
                "1 NOTE {}\n",
                "Une note fictive. ".repeat(1 + next(8) as usize)
            ));
        }
        if let Some(f) = famc {
            out.push_str(&format!("1 FAMC @F{f}@\n"));
        }
        for f in fs {
            out.push_str(&format!("1 FAMS @F{f}@\n"));
        }
    }
    for (f, (h, w, kids)) in fams.iter().enumerate() {
        out.push_str(&format!("0 @F{f}@ FAM\n1 HUSB @I{h}@\n1 WIFE @I{w}@\n"));
        for k in kids {
            out.push_str(&format!("1 CHIL @I{k}@\n"));
        }
        out.push_str(&format!(
            "1 MARR\n2 DATE {}\n2 PLAC Bourg{}, Pays{}, Contrée{}\n",
            people[*h].year + 18 + next(13),
            next(800),
            next(30),
            next(5)
        ));
    }
    out.push_str("0 TRLR\n");
    out
}

/// Close to the largest family the upload takes — 40,000 people, 9.4 MB of
/// the limit's 10 — converted through the page and served. Measured in release on
/// the operator's two-core server: 7.5 s for the conversion page, 3.3 s to
/// load the archive; the river and search pages then take 9–14 s and the
/// whole tree a minute (docs/GEDCOM-IMPORT.md): this guards the
/// conversion's time and that every page answers, not the pages' speed at a
/// size no family here has. Run it in release:
/// `cargo test --release --test gedcom_conversion -- --ignored`.
#[tokio::test]
#[ignore = "stress: minutes in a debug build; run in release with --ignored"]
async fn forty_thousand_people_convert_and_are_served() {
    let text = large_family(40_000);
    assert!(
        text.len() > 8 << 20 && text.len() < axgf_cms::convert::MAX_UPLOAD,
        "{} bytes",
        text.len()
    );
    let (app, _s) = app_with_empty_bundle("gc-forty");
    let t = Instant::now();
    let page = expect_status(
        post_convert(&app, "large.ged", text.as_bytes()).await,
        StatusCode::OK,
        "convert",
    )
    .await;
    assert!(
        t.elapsed() < Duration::from_secs(30),
        "conversion took {:?}",
        t.elapsed()
    );
    assert!(page.len() < 400_000, "the page is {} bytes", page.len());
    let zip = body_bytes(get(&app, &download_link(&page)).await).await;
    let dir = scratch("gc-forty-served");
    let path = dir.join("family.axgf");
    std::fs::write(&path, &zip).unwrap();
    let t = Instant::now();
    let served = build_app(&path).expect("loads");
    assert!(
        t.elapsed() < Duration::from_secs(30),
        "loading took {:?}",
        t.elapsed()
    );
    let cookie = admin_cookie(&served).await;
    let data = axgf_cms::state::envelope_into_data(axgf_rs::import_bundle(&zip)).unwrap();
    let some: Vec<String> = data["persons"]
        .as_object()
        .unwrap()
        .keys()
        .take(3)
        .cloned()
        .collect();
    let mut pages = vec![
        "/".to_string(),
        "/river/data".into(),
        "/about".into(),
        "/search?q=Basile".into(),
        "/admin".into(),
    ];
    for id in &some {
        pages.push(format!("/person/{id}"));
        pages.push(format!("/river/data?p={id}&n=5"));
    }
    for uri in pages {
        let t = Instant::now();
        let resp = get_with_cookie(&served, &uri, &cookie).await;
        assert!(resp.status().as_u16() < 500, "{uri}: {}", resp.status());
        let _ = body_bytes(resp).await;
        assert!(
            t.elapsed() < Duration::from_secs(60),
            "{uri} took {:?}",
            t.elapsed()
        );
    }
}
