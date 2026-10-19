//! The record's editor: six tabs over one form, one save (v0.4.2).
//!
//! The tabs are the reading tabs, all six always present in edit mode; what
//! changed on any of them is written by one Enregistrer, in one commit; and a
//! record on a tab nobody touched is not written at all.

mod common;

use axum::http::StatusCode;
use common::*;
use scraper::{ElementRef, Html, Selector};
use serde_json::{json, Value};

const ANNA: &str = "a0000000-0000-4000-8000-000000000001";
const JAN: &str = "a0000000-0000-4000-8000-000000000002";
const KID: &str = "a0000000-0000-4000-8000-000000000003";
const LONE: &str = "a0000000-0000-4000-8000-000000000004";
const FAM: &str = "f0000000-0000-4000-8000-000000000001";
const EV: &str = "e0000000-0000-4000-8000-000000000001";
const SRC: &str = "50000000-0000-4000-8000-000000000001";

fn person(id: &str, name: &str) -> Value {
    json!({"id": id, "type": "person", "axgf_version": "1.1",
           "identity": {"name": {"display": name, "components": []},
                        "is_living": false, "visibility": "public"}})
}

/// Anna and Jan married, with one child; an event naming Anna; one source;
/// and somebody with nothing at all.
fn bundle(tag: &str) -> common::Scratch {
    let dir = scratch(tag);
    let path = dir.join("editor.axgf");
    let flat = json!({
        "manifest": {"axgf": "1.1"},
        "persons": {
            ANNA: person(ANNA, "Anna Lis"), JAN: person(JAN, "Jan Nowak"),
            KID: person(KID, "Zofia Nowak"), LONE: person(LONE, "Nobody Else"),
        },
        "families": {FAM: {"id": FAM, "type": "family", "axgf_version": "1.0",
            "union": {"type": "marriage", "persons": [
                {"person_id": ANNA, "role": "spouse"}, {"person_id": JAN, "role": "spouse"}],
                "end": {"date": {"value": "1950", "precision": "year"}}},
            "children": [{"person_id": KID, "confidence": 0.9, "birth_order": 1}]}},
        "events": {EV: {"id": EV, "type": "event", "axgf_version": "1.0", "category": "census",
            "date": {"value": "1921", "precision": "year"},
            "participants": [{"entity_type": "person", "entity_id": ANNA, "role": "subject"}]}},
        "sources": {SRC: {"id": SRC, "type": "source", "axgf_version": "1.0",
            "title": "Parish register", "source_type": "baptism_record", "reliability": "primary"}},
        "links": {}, "occupations": {}, "places": {}, "documents": {}
    });
    std::fs::write(
        &path,
        axgf_cms::state::export_to_bytes(&flat.to_string()).expect("export"),
    )
    .expect("write");
    dir.pointing_at(path)
}

fn sel(s: &str) -> Selector {
    Selector::parse(s).unwrap()
}

fn editor<'a>(doc: &'a Html) -> ElementRef<'a> {
    doc.select(&sel("form.person-editor"))
        .next()
        .expect("the editor")
}

/// What a browser posts for the form as drawn.
fn submission(form: &ElementRef) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for el in form.select(&sel("input[name], select[name], textarea[name]")) {
        let v = el.value();
        let name = v.attr("name").unwrap().to_string();
        match v.name() {
            "select" => {
                let opts: Vec<_> = el.select(&sel("option")).collect();
                let o = opts
                    .iter()
                    .find(|o| o.value().attr("selected").is_some())
                    .or(opts.first());
                if let Some(o) = o {
                    out.push((name, o.value().attr("value").unwrap_or("").to_string()));
                }
            }
            "textarea" => out.push((name, el.text().collect())),
            _ => match v.attr("type") {
                Some("checkbox") => {
                    if v.attr("checked").is_some() {
                        out.push((name, "on".into()));
                    }
                }
                // A radio is posted only when it is the one ticked, and as
                // its own value. The delete policy's radios belong to another
                // form (`form="person-delete"`) and are not this one's.
                Some("radio") => {
                    if v.attr("checked").is_some() && v.attr("form").is_none() {
                        out.push((name, v.attr("value").unwrap_or("").to_string()));
                    }
                }
                Some("file") => {}
                _ => out.push((name, v.attr("value").unwrap_or("").to_string())),
            },
        }
    }
    out
}

fn set(pairs: &mut Vec<(String, String)>, name: &str, value: &str) {
    match pairs.iter_mut().find(|(k, _)| k == name) {
        Some(p) => p.1 = value.to_string(),
        None => pairs.push((name.to_string(), value.to_string())),
    }
}

fn encode(pairs: &[(String, String)]) -> String {
    fn enc(s: &str) -> String {
        s.bytes()
            .map(|b| match b {
                b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                    (b as char).to_string()
                }
                b' ' => "+".into(),
                _ => format!("%{b:02X}"),
            })
            .collect()
    }
    pairs
        .iter()
        .map(|(k, v)| format!("{}={}", enc(k), enc(v)))
        .collect::<Vec<_>>()
        .join("&")
}

async fn page(app: &axum::Router, cookie: &str, id: &str) -> String {
    body_string(get_with_cookie(app, &format!("/admin/person/{id}/edit"), cookie).await).await
}

async fn drawn(app: &axum::Router, cookie: &str, id: &str) -> Vec<(String, String)> {
    let html = page(app, cookie, id).await;
    submission(&editor(&Html::parse_document(&html)))
}

/// The bundle as the server holds it now, from its own export.
async fn flat(app: &axum::Router, cookie: &str, who: &str) -> Value {
    // Each record's raw document, as its editor's raw-JSON box carries it.
    let html = body_string(get_with_cookie(app, &format!("/admin/{who}/edit"), cookie).await).await;
    let doc = Html::parse_document(&html);
    let raw: String = doc
        .select(&sel(r#"textarea[name="raw_json"]"#))
        .next()
        .expect("a raw document")
        .text()
        .collect();
    serde_json::from_str(&raw).expect("JSON")
}

async fn save(
    app: &axum::Router,
    cookie: &str,
    id: &str,
    pairs: &[(String, String)],
) -> StatusCode {
    post_form_as(app, cookie, &format!("/admin/person/{id}"), &encode(pairs))
        .await
        .status()
}

/// All six tabs, by the reading tabs' names, in every case — a tab with
/// nothing in it included: an empty tab is where things are added.
#[tokio::test]
async fn every_tab_renders_in_edit_mode_even_with_nothing_in_it() {
    let src = bundle("tabs-src");
    let (app, _p) = app_with_bundle("tabs", &src);
    let cookie = format!("{}; axgf_lang=fr", admin_cookie(&app).await);
    let html = page(&app, &cookie, LONE).await;
    let doc = Html::parse_document(&html);
    let tabs: Vec<String> = doc
        .select(&sel("nav.ed-tabs a[data-edit-tab] .ed-tab-name"))
        .map(|a| a.text().collect::<String>().trim().to_string())
        .collect();
    assert_eq!(
        tabs,
        [
            "Fiche",
            "Famille",
            "Événements",
            "Sources",
            "Médias",
            "Profil"
        ]
    );
    for key in [
        "fiche",
        "famille",
        "evenements",
        "sources",
        "medias",
        "profil",
    ] {
        let panel = doc
            .select(&sel(&format!("form.person-editor section.ed-panel#{key}")))
            .next()
            .unwrap_or_else(|| panic!("the {key} tab"));
        // Each has a way to add to it.
        let adds = panel
            .select(&sel("details.ed-add, details.pf-group, .ed-fields"))
            .count();
        assert!(adds > 0, "{key} offers something to fill");
    }
    // Reading the same person hides those empty tabs.
    let read = body_string(get_with_cookie(&app, &format!("/person/{LONE}"), &cookie).await).await;
    assert!(
        tab_hrefs(&read).is_empty(),
        "reading draws no strip for one tab"
    );
}

/// Modifier opens the editor on the tab being read.
#[tokio::test]
async fn modifier_lands_on_the_tab_being_read() {
    let src = bundle("modifier-src");
    let (app, _p) = app_with_bundle("modifier", &src);
    let cookie = admin_cookie(&app).await;
    for (read, edit) in [
        ("", "fiche"),
        ("?tab=family", "famille"),
        ("?tab=life", "evenements"),
    ] {
        let html =
            body_string(get_with_cookie(&app, &format!("/person/{ANNA}{read}"), &cookie).await)
                .await;
        assert!(
            html.contains(&format!(r#"href="/admin/person/{ANNA}/edit#{edit}""#)),
            "{read} → #{edit}"
        );
    }
}

/// Changes on two tabs — a name on the Fiche, a child's confidence on
/// Famille, a new event on Événements — are all written by one save.
#[tokio::test]
async fn changes_in_different_tabs_are_persisted_by_one_save() {
    let src = bundle("two-tabs-src");
    let (app, _p) = app_with_bundle("two-tabs", &src);
    let cookie = admin_cookie(&app).await;
    let mut pairs = drawn(&app, &cookie, ANNA).await;
    set(&mut pairs, "identity.name.display", "Anna Lis-Nowak");
    set(&mut pairs, &format!("fam.{FAM}.child.0.confidence"), "0.6");
    set(&mut pairs, "new.ev.category", "migration");
    set(&mut pairs, "new.ev.date.value", "1923");
    set(&mut pairs, "tab", "famille");
    let resp = post_form_as(
        &app,
        &cookie,
        &format!("/admin/person/{ANNA}"),
        &encode(&pairs),
    )
    .await;
    assert_eq!(
        resp.status(),
        StatusCode::SEE_OTHER,
        "one save, then back to the editor"
    );
    let to = resp.headers()["location"].to_str().unwrap().to_string();
    assert_eq!(
        to,
        format!("/admin/person/{ANNA}/edit?saved=3#famille"),
        "on the tab it was made on"
    );

    let anna = flat(&app, &cookie, &format!("person/{ANNA}")).await;
    assert_eq!(anna["identity"]["name"]["display"], "Anna Lis-Nowak");
    let fam = flat(&app, &cookie, &format!("family/{FAM}")).await;
    assert_eq!(fam["children"][0]["confidence"], 0.6);
    assert_eq!(
        fam["children"][0]["birth_order"], 1,
        "what the tab does not draw is kept"
    );
    assert_eq!(
        fam["union"]["end"]["date"]["value"], "1950",
        "the union's end too"
    );
    let html = page(&app, &cookie, ANNA).await;
    assert!(
        html.contains("1923"),
        "the new event is on the Événements tab"
    );
}

/// A save that changes only a name writes only the person: the family and
/// the event drawn on the other tabs keep their versions.
#[tokio::test]
async fn untouched_tabs_write_nothing() {
    let src = bundle("untouched-src");
    let (app, _p) = app_with_bundle("untouched", &src);
    let cookie = admin_cookie(&app).await;
    let fam_before = flat(&app, &cookie, &format!("family/{FAM}")).await;
    let ev_before = flat(&app, &cookie, &format!("event/{EV}")).await;
    let mut pairs = drawn(&app, &cookie, ANNA).await;
    set(&mut pairs, "identity.name.display", "Anna L.");
    assert_eq!(
        save(&app, &cookie, ANNA, &pairs).await,
        StatusCode::SEE_OTHER
    );
    assert_eq!(
        flat(&app, &cookie, &format!("family/{FAM}")).await,
        fam_before
    );
    assert_eq!(flat(&app, &cookie, &format!("event/{EV}")).await, ev_before);
}

/// The Famille tab is operations on a list: start a family with a partner and
/// a child, take a child out of a union, leave the family one is a child of.
#[tokio::test]
async fn the_family_tab_creates_changes_and_detaches() {
    let src = bundle("famille-src");
    let (app, _p) = app_with_bundle("famille", &src);
    let cookie = admin_cookie(&app).await;

    // Start a family: Nobody Else with Jan, and Anna as their child.
    let mut pairs = drawn(&app, &cookie, LONE).await;
    set(&mut pairs, "new.fam.partner", JAN);
    set(&mut pairs, "new.fam.type", "cohabitation");
    set(&mut pairs, "new.fam.child", ANNA);
    assert_eq!(
        save(&app, &cookie, LONE, &pairs).await,
        StatusCode::SEE_OTHER
    );
    let html = page(&app, &cookie, LONE).await;
    let doc = Html::parse_document(&html);
    let unions: Vec<_> = doc
        .select(&sel("section#famille article.ed-union"))
        .collect();
    assert_eq!(unions.len(), 1, "the new union is on the tab");
    let text: String = unions[0].text().collect();
    assert!(
        text.contains("Jan Nowak") && text.contains("Anna Lis"),
        "{text}"
    );

    // Take the child out of Anna's union, and Anna out of Jan's new family.
    let mut pairs = drawn(&app, &cookie, ANNA).await;
    set(&mut pairs, &format!("fam.{FAM}.child.0.remove"), "on");
    let parents = Html::parse_document(&page(&app, &cookie, ANNA).await)
        .select(&sel("section#famille article.ed-parents"))
        .next()
        .and_then(|a| a.value().id().map(str::to_string))
        .expect("Anna is now somebody's child");
    set(&mut pairs, &format!("{parents}.leave"), "on");
    assert_eq!(
        save(&app, &cookie, ANNA, &pairs).await,
        StatusCode::SEE_OTHER
    );
    let fam = flat(&app, &cookie, &format!("family/{FAM}")).await;
    assert!(
        fam.get("children").is_none(),
        "the child was taken out: {fam}"
    );
    let html = page(&app, &cookie, ANNA).await;
    assert!(
        Html::parse_document(&html)
            .select(&sel("section#famille article.ed-parents"))
            .next()
            .is_none(),
        "Anna is nobody's child again"
    );
}

/// A refused save comes back on the tab it was refused on, with everything
/// typed on every tab still there and those tabs marked unsaved.
#[tokio::test]
async fn a_refused_save_keeps_every_tab_as_it_was_typed() {
    let src = bundle("refused-src");
    let (app, _p) = app_with_bundle("refused", &src);
    let cookie = admin_cookie(&app).await;
    let mut pairs = drawn(&app, &cookie, ANNA).await;
    set(&mut pairs, "identity.name.display", "Typed On The Fiche");
    // An event with no category: refused.
    set(&mut pairs, "new.ev.description", "Something happened");
    let resp = post_form_as(
        &app,
        &cookie,
        &format!("/admin/person/{ANNA}"),
        &encode(&pairs),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);
    let html = body_string(resp).await;
    let doc = Html::parse_document(&html);
    let form = editor(&doc);
    assert_eq!(
        form.value().attr("data-tab"),
        Some("evenements"),
        "back on its tab"
    );
    assert!(html.contains("Typed On The Fiche"), "the Fiche kept");
    assert!(html.contains("Something happened"), "the event kept");
    for key in ["fiche", "evenements"] {
        let a = doc
            .select(&sel(&format!(r#"nav.ed-tabs a[data-edit-tab="{key}"]"#)))
            .next()
            .unwrap();
        assert!(
            a.value()
                .attr("class")
                .unwrap_or("")
                .contains("has-changes"),
            "{key} is marked unsaved"
        );
    }
    let anna = flat(&app, &cookie, &format!("person/{ANNA}")).await;
    assert_eq!(
        anna["identity"]["name"]["display"], "Anna Lis",
        "nothing saved"
    );
}

/// Sources: a new source, cited for the birth, in the same save; and the
/// fields of a source the person rests on.
#[tokio::test]
async fn the_sources_tab_adds_a_source_and_cites_it() {
    let src = bundle("sources-src");
    let (app, _p) = app_with_bundle("sources", &src);
    let cookie = admin_cookie(&app).await;
    let mut pairs = drawn(&app, &cookie, KID).await;
    set(
        &mut pairs,
        "new.src.title",
        "Birth certificate, Kraków 1925",
    );
    set(&mut pairs, "new.src.attach", "birth");
    assert_eq!(
        save(&app, &cookie, KID, &pairs).await,
        StatusCode::SEE_OTHER
    );
    let kid = flat(&app, &cookie, &format!("person/{KID}")).await;
    let sid = kid["birth"]["source_id"]
        .as_str()
        .expect("the birth cites it")
        .to_string();
    let source = flat(&app, &cookie, &format!("source/{sid}")).await;
    assert_eq!(source["title"], "Birth certificate, Kraków 1925");

    // Now it is a source this person rests on, with its own fields here.
    let mut pairs = drawn(&app, &cookie, KID).await;
    set(&mut pairs, &format!("src.{sid}.reliability"), "primary");
    assert_eq!(
        save(&app, &cookie, KID, &pairs).await,
        StatusCode::SEE_OTHER
    );
    let source = flat(&app, &cookie, &format!("source/{sid}")).await;
    assert_eq!(source["reliability"], "primary");
    // And detaching it is choosing none.
    let mut pairs = drawn(&app, &cookie, KID).await;
    set(&mut pairs, "cite.birth", "");
    assert_eq!(
        save(&app, &cookie, KID, &pairs).await,
        StatusCode::SEE_OTHER
    );
    let kid = flat(&app, &cookie, &format!("person/{KID}")).await;
    assert!(kid["birth"].get("source_id").is_none(), "{kid}");
}

fn png() -> Vec<u8> {
    let img = image::RgbImage::from_pixel(4, 3, image::Rgb([120, 90, 60]));
    let mut out = std::io::Cursor::new(Vec::new());
    img.write_to(&mut out, image::ImageFormat::Png)
        .expect("png");
    out.into_inner()
}

/// Médias: a file uploaded with the rest of the form — a name changed on the
/// Fiche in the same save — and then captioned, then detached.
#[tokio::test]
async fn the_media_tab_uploads_captions_and_detaches_in_the_one_save() {
    use axum::body::Body;
    use axum::http::{header, Request};
    use tower::ServiceExt;
    let src = bundle("medias-src");
    let (app, _p) = app_with_bundle("medias", &src);
    let cookie = admin_cookie(&app).await;
    let mut pairs = drawn(&app, &cookie, KID).await;
    set(&mut pairs, "identity.name.display", "Zofia N.");
    set(&mut pairs, "new.doc.caption", "On the steps, 1931");
    set(&mut pairs, "tab", "medias");
    let boundary = "----axgfeditorboundary";
    let mut body = Vec::new();
    for (k, v) in &pairs {
        body.extend_from_slice(
            format!("--{boundary}\r\nContent-Disposition: form-data; name=\"{k}\"\r\n\r\n{v}\r\n")
                .as_bytes(),
        );
    }
    body.extend_from_slice(
        format!(
            "--{boundary}\r\nContent-Disposition: form-data; name=\"new.doc.file\"; \
             filename=\"steps.png\"\r\nContent-Type: image/png\r\n\r\n"
        )
        .as_bytes(),
    );
    body.extend_from_slice(&png());
    body.extend_from_slice(format!("\r\n--{boundary}--\r\n").as_bytes());
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/admin/person/{KID}"))
                .method("POST")
                .header(
                    header::CONTENT_TYPE,
                    format!("multipart/form-data; boundary={boundary}"),
                )
                .header(header::COOKIE, &cookie)
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    assert!(resp.headers()["location"]
        .to_str()
        .unwrap()
        .ends_with("saved=2#medias"));

    let html = page(&app, &cookie, KID).await;
    let doc = Html::parse_document(&html);
    let file = doc
        .select(&sel("section#medias article.ed-document"))
        .next()
        .expect("the file is on the Médias tab");
    let did = file
        .value()
        .id()
        .unwrap()
        .trim_start_matches("doc.")
        .to_string();
    assert!(html.contains("On the steps, 1931"), "with its caption");
    let kid = flat(&app, &cookie, &format!("person/{KID}")).await;
    assert_eq!(
        kid["identity"]["name"]["display"], "Zofia N.",
        "and the Fiche, same save"
    );

    // Re-caption it, then detach it.
    let mut pairs = drawn(&app, &cookie, KID).await;
    set(
        &mut pairs,
        &format!("doc.{did}.caption"),
        "On the church steps, 1931",
    );
    assert_eq!(
        save(&app, &cookie, KID, &pairs).await,
        StatusCode::SEE_OTHER
    );
    assert!(page(&app, &cookie, KID)
        .await
        .contains("On the church steps, 1931"));
    let mut pairs = drawn(&app, &cookie, KID).await;
    set(&mut pairs, &format!("doc.{did}.leave"), "on");
    assert_eq!(
        save(&app, &cookie, KID, &pairs).await,
        StatusCode::SEE_OTHER
    );
    let html = page(&app, &cookie, KID).await;
    assert!(
        Html::parse_document(&html)
            .select(&sel("section#medias article.ed-document"))
            .next()
            .is_none(),
        "detached"
    );
}

/// The editor's save, as multipart, with a file under `new.doc.file`.
async fn save_with_file(
    app: &axum::Router,
    cookie: &str,
    id: &str,
    pairs: &[(String, String)],
    file: &[u8],
) -> axum::response::Response {
    use axum::body::Body;
    use axum::http::{header, Request};
    use tower::ServiceExt;
    let boundary = "----axgfavatarboundary";
    let mut body = Vec::new();
    for (k, v) in pairs {
        body.extend_from_slice(
            format!("--{boundary}\r\nContent-Disposition: form-data; name=\"{k}\"\r\n\r\n{v}\r\n")
                .as_bytes(),
        );
    }
    body.extend_from_slice(
        format!(
            "--{boundary}\r\nContent-Disposition: form-data; name=\"new.doc.file\"; \
             filename=\"face.png\"\r\nContent-Type: image/png\r\n\r\n"
        )
        .as_bytes(),
    );
    body.extend_from_slice(file);
    body.extend_from_slice(format!("\r\n--{boundary}--\r\n").as_bytes());
    app.clone()
        .oneshot(
            Request::builder()
                .uri(format!("/admin/person/{id}"))
                .method("POST")
                .header(
                    header::CONTENT_TYPE,
                    format!("multipart/form-data; boundary={boundary}"),
                )
                .header(header::COOKIE, cookie)
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap()
}

/// The stored avatar choice, as the raw document carries it.
async fn avatar_of(app: &axum::Router, cookie: &str, id: &str) -> Value {
    flat(app, cookie, &format!("person/{id}")).await["extensions"]["axgf-cms:avatar/v1"].clone()
}

/// Médias: which picture stands for the person is chosen where the pictures
/// are (it had been reachable only from a link the record page lost in the
/// minimalism pass). An upload can be the face in the same save; a picture
/// on the tab is chosen by its radio; the two answers that are not a picture
/// are there too; and the choice is saved with the rest of the form.
#[tokio::test]
async fn the_media_tab_chooses_the_picture_that_stands_for_the_person() {
    let src = bundle("avatar-src");
    let (app, _p) = app_with_bundle("avatar", &src);
    let cookie = admin_cookie(&app).await;

    // Nobody without a picture is offered a choice between none.
    let html = page(&app, &cookie, LONE).await;
    assert!(
        !html.contains(r#"name="avatar.choice""#),
        "no picture, no choice"
    );

    // An upload ticked as the face is the face, in the one save.
    let mut pairs = drawn(&app, &cookie, KID).await;
    set(&mut pairs, "new.doc.avatar", "on");
    set(&mut pairs, "tab", "medias");
    let resp = save_with_file(&app, &cookie, KID, &pairs, &png()).await;
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    let html = page(&app, &cookie, KID).await;
    let doc = Html::parse_document(&html);
    let did = doc
        .select(&sel("section#medias article.ed-document"))
        .next()
        .expect("the upload is on the tab")
        .value()
        .id()
        .unwrap()
        .trim_start_matches("doc.")
        .to_string();
    assert_eq!(avatar_of(&app, &cookie, KID).await["document_id"], did);
    // The tab says so: the box above the files shows the face, and the
    // picture's own radio is the one ticked.
    assert!(
        doc.select(&sel("section#medias .ed-avatar img.ed-avatar-now"))
            .next()
            .is_some(),
        "the face as it is shown now"
    );
    let ticked: Vec<String> = doc
        .select(&sel(r#"input[name="avatar.choice"][checked]"#))
        .map(|r| r.value().attr("value").unwrap().to_string())
        .collect();
    assert_eq!(ticked, vec![did.clone()]);
    assert!(
        doc.select(&sel(&format!(
            r#"a[href="/admin/person/{KID}/avatar?from=edit"]"#
        )))
        .next()
        .is_some(),
        "framing the face is the picker's, one link away"
    );

    // Initials instead, then the software's pick: each a radio, each saved
    // with the form.
    let mut pairs = drawn(&app, &cookie, KID).await;
    set(&mut pairs, "avatar.choice", "none");
    assert_eq!(
        save(&app, &cookie, KID, &pairs).await,
        StatusCode::SEE_OTHER
    );
    assert_eq!(avatar_of(&app, &cookie, KID).await, json!({"mode": "none"}));
    let mut pairs = drawn(&app, &cookie, KID).await;
    set(&mut pairs, "avatar.choice", "auto");
    assert_eq!(
        save(&app, &cookie, KID, &pairs).await,
        StatusCode::SEE_OTHER
    );
    assert!(
        avatar_of(&app, &cookie, KID).await.is_null(),
        "automatic is no choice stored"
    );

    // The picture again, by its radio; a save that leaves the radios alone
    // leaves the choice alone.
    let mut pairs = drawn(&app, &cookie, KID).await;
    set(&mut pairs, "avatar.choice", &did);
    assert_eq!(
        save(&app, &cookie, KID, &pairs).await,
        StatusCode::SEE_OTHER
    );
    assert_eq!(avatar_of(&app, &cookie, KID).await["document_id"], did);
    let mut pairs = drawn(&app, &cookie, KID).await;
    set(&mut pairs, "identity.name.display", "Zofia N.");
    assert_eq!(
        save(&app, &cookie, KID, &pairs).await,
        StatusCode::SEE_OTHER
    );
    assert_eq!(avatar_of(&app, &cookie, KID).await["document_id"], did);

    // A file that is not this person's is refused, and nothing is written.
    let mut pairs = drawn(&app, &cookie, KID).await;
    set(
        &mut pairs,
        "avatar.choice",
        "d0000000-0000-4000-8000-00000000dead",
    );
    assert_eq!(
        save(&app, &cookie, KID, &pairs).await,
        StatusCode::UNPROCESSABLE_ENTITY
    );
    assert_eq!(avatar_of(&app, &cookie, KID).await["document_id"], did);

    // Detached in the same save, the picture cannot stay the face: the
    // choice goes back to the software rather than to a file the record no
    // longer holds.
    let mut pairs = drawn(&app, &cookie, KID).await;
    set(&mut pairs, &format!("doc.{did}.leave"), "on");
    assert_eq!(
        save(&app, &cookie, KID, &pairs).await,
        StatusCode::SEE_OTHER
    );
    assert!(avatar_of(&app, &cookie, KID).await.is_null());
}

/// Just saved, there is nothing to cancel: the editor's way out reads "Back
/// to the record" and goes there. Before a save it is still "Cancel".
#[tokio::test]
async fn after_a_save_the_way_out_is_back_to_the_record() {
    let src = bundle("back-src");
    let (app, _p) = app_with_bundle("back", &src);
    let cookie = admin_cookie(&app).await;
    let way_out = |html: &str| -> (String, String) {
        let doc = Html::parse_document(html);
        let a = doc
            .select(&sel(".ed-save a.btn"))
            .next()
            .expect("the way out");
        (
            a.value().attr("href").unwrap().to_string(),
            a.text().collect::<String>().trim().to_string(),
        )
    };
    let before = page(&app, &cookie, ANNA).await;
    assert_eq!(
        way_out(&before),
        (format!("/person/{ANNA}"), "Cancel".into())
    );

    let mut pairs = drawn(&app, &cookie, ANNA).await;
    set(&mut pairs, "identity.name.display", "Anna L.");
    let resp = post_form_as(
        &app,
        &cookie,
        &format!("/admin/person/{ANNA}"),
        &encode(&pairs),
    )
    .await;
    let to = resp.headers()["location"].to_str().unwrap().to_string();
    let after =
        body_string(get_with_cookie(&app, to.split('#').next().unwrap(), &cookie).await).await;
    assert_eq!(
        way_out(&after),
        (format!("/person/{ANNA}"), "Back to the record".into())
    );
}

/// The picker, opened from the editor to frame the face, goes back there.
#[tokio::test]
async fn the_picker_opened_from_the_editor_goes_back_to_it() {
    let src = bundle("picker-src");
    let (app, _p) = app_with_bundle("picker", &src);
    let cookie = admin_cookie(&app).await;
    let mut pairs = drawn(&app, &cookie, KID).await;
    set(&mut pairs, "new.doc.avatar", "on");
    assert_eq!(
        save_with_file(&app, &cookie, KID, &pairs, &png())
            .await
            .status(),
        StatusCode::SEE_OTHER
    );
    let did = avatar_of(&app, &cookie, KID).await["document_id"]
        .as_str()
        .expect("the upload is the face")
        .to_string();

    let html = body_string(
        get_with_cookie(
            &app,
            &format!("/admin/person/{KID}/avatar?from=edit"),
            &cookie,
        )
        .await,
    )
    .await;
    assert!(html.contains(r#"<input type="hidden" name="from" value="edit">"#));
    assert!(html.contains(&format!(r#"href="/admin/person/{KID}/edit#medias""#)));
    let version = Html::parse_document(&html)
        .select(&sel(r#"input[name="base_version"]"#))
        .next()
        .and_then(|i| i.value().attr("value").map(str::to_string))
        .expect("the version the picker was drawn from");
    let plain =
        body_string(get_with_cookie(&app, &format!("/admin/person/{KID}/avatar"), &cookie).await)
            .await;
    assert!(
        !plain.contains(r#"name="from""#),
        "opened from the record, it goes back there"
    );

    let resp = post_form_as(
        &app,
        &cookie,
        &format!("/admin/person/{KID}/avatar"),
        &format!("choice={did}&focal_x=0.5&focal_y=0.2&from=edit&base_version={version}"),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    assert_eq!(
        resp.headers()["location"],
        format!("/admin/person/{KID}/edit?saved=1#medias").as_str()
    );
    let stored = avatar_of(&app, &cookie, KID).await;
    assert_eq!(stored["focal"]["y"], 0.2);

    // Chosen again from the tab, the same picture keeps its framing.
    let mut pairs = drawn(&app, &cookie, KID).await;
    set(&mut pairs, "identity.name.display", "Zofia F.");
    set(&mut pairs, "avatar.choice", &did);
    assert_eq!(
        save(&app, &cookie, KID, &pairs).await,
        StatusCode::SEE_OTHER
    );
    assert_eq!(avatar_of(&app, &cookie, KID).await["focal"]["y"], 0.2);
}
