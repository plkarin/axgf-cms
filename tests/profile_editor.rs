//! The person editor reaches every field of the AXGF 1.1 profile (v0.4.1).
//!
//! The minimalism pass put every empty field of the edit form behind one
//! "n other fields" lump and dropped the only link into the group editor, so
//! the 132 attributes of the fourteen groups could not be reached — and were
//! empty because there was no longer a way to fill them. Hiding what is empty
//! governs reading; in edit mode empty is the normal state. These tests hold
//! the editor to that.

mod common;

use axum::http::StatusCode;
use common::*;
use scraper::{ElementRef, Html, Selector};
use serde_json::{json, Value};

const BARE: &str = "b0000000-0000-4000-8000-000000000001";
const LEGACY: &str = "b0000000-0000-4000-8000-000000000002";

/// One person with a name and nothing else, and one whose height was
/// recorded before 1.1, in this application's own extension.
fn bundle(tag: &str) -> common::Scratch {
    let dir = scratch(tag);
    let path = dir.join("profile.axgf");
    let flat = json!({
        "manifest": {"axgf": "1.1"},
        "persons": {
            BARE: {"id": BARE, "type": "person", "axgf_version": "1.1",
                   "identity": {"name": {"display": "Nobody Recorded", "components": []},
                                "is_living": false, "visibility": "public"}},
            LEGACY: {"id": LEGACY, "type": "person", "axgf_version": "1.1",
                     "identity": {"name": {"display": "Old Height", "components": []},
                                  "is_living": false, "visibility": "public"},
                     "extensions": {"axgf-cms:traits/v1": {"height_cm": [
                         {"value": 172, "date": {"value": "1914", "precision": "year"}}]}}}
        },
        "families": {}, "events": {}, "links": {}, "occupations": {},
        "sources": {}, "places": {}, "documents": {}
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

/// The edit form: the one that posts to the person, whatever tab it is on.
fn edit_form<'a>(doc: &'a Html, id: &str) -> ElementRef<'a> {
    doc.select(&sel(&format!(
        r#"form.person-editor[action^="/admin/person/{id}#"]"#
    )))
    .next()
    .expect("the person's edit form")
}

/// Whether a control can be typed into: named, not disabled, not read-only,
/// not hidden.
fn typeable(el: &ElementRef) -> bool {
    let v = el.value();
    v.attr("name").is_some()
        && v.attr("disabled").is_none()
        && v.attr("readonly").is_none()
        && v.attr("type") != Some("hidden")
}

/// What a browser posts for `form` as drawn: every named control, a select's
/// selected option (or its first), a checkbox only when checked.
fn submission(form: &ElementRef) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for el in form.select(&sel("input[name], select[name], textarea[name]")) {
        let v = el.value();
        if v.attr("disabled").is_some() {
            continue;
        }
        let name = v.attr("name").unwrap().to_string();
        match v.name() {
            "select" => {
                let opts: Vec<_> = el.select(&sel("option")).collect();
                let chosen = opts
                    .iter()
                    .find(|o| o.value().attr("selected").is_some())
                    .or(opts.first());
                if let Some(o) = chosen {
                    let value = o
                        .value()
                        .attr("value")
                        .map(str::to_string)
                        .unwrap_or_else(|| o.text().collect());
                    out.push((name, value));
                }
            }
            "textarea" => out.push((name, el.text().collect())),
            _ => match v.attr("type") {
                Some("checkbox") | Some("radio") => {
                    if v.attr("checked").is_some() {
                        out.push((name, v.attr("value").unwrap_or("on").to_string()));
                    }
                }
                Some("submit") | Some("button") => {}
                _ => out.push((name, v.attr("value").unwrap_or("").to_string())),
            },
        }
    }
    out
}

fn encode(pairs: &[(String, String)]) -> String {
    fn enc(s: &str) -> String {
        let mut out = String::new();
        for b in s.bytes() {
            match b {
                b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                    out.push(b as char)
                }
                b' ' => out.push('+'),
                _ => out.push_str(&format!("%{b:02X}")),
            }
        }
        out
    }
    pairs
        .iter()
        .map(|(k, v)| format!("{}={}", enc(k), enc(v)))
        .collect::<Vec<_>>()
        .join("&")
}

fn set(pairs: &mut [(String, String)], name: &str, value: &str) {
    let p = pairs
        .iter_mut()
        .find(|(k, _)| k == name)
        .unwrap_or_else(|| panic!("the form has {name}"));
    p.1 = value.to_string();
}

/// The stored person, as the raw-JSON half of the edit form carries it.
async fn stored(app: &axum::Router, cookie: &str, id: &str) -> Value {
    let html =
        body_string(get_with_cookie(app, &format!("/admin/person/{id}/edit"), cookie).await).await;
    let doc = Html::parse_document(&html);
    let raw: String = doc
        .select(&sel(r#"textarea[name="raw_json"]"#))
        .next()
        .expect("the raw document")
        .text()
        .collect();
    serde_json::from_str(&raw).expect("raw JSON")
}

/// The regression the minimalism pass shipped, held so it cannot recur: on a
/// person with none of them filled, every attribute of every group is in the
/// edit form, as a control that can be typed into, under its group's name.
#[tokio::test]
async fn every_profile_attribute_is_reachable_for_a_person_with_none_filled() {
    let src = bundle("reach-src");
    let (app, _p) = app_with_bundle("reach", &src);
    let cookie = format!("{}; axgf_lang=en", admin_cookie(&app).await);
    let resp = get_with_cookie(&app, &format!("/admin/person/{BARE}/edit"), &cookie).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let html = body_string(resp).await;
    let doc = Html::parse_document(&html);
    let form = edit_form(&doc, BARE);

    // All fourteen groups, by name, each with its count — closed, since this
    // person has nothing in any of them.
    let groups = axgf_cms::profile::groups();
    assert_eq!(groups.len(), 14, "the 1.1 registry has fourteen groups");
    let drawn: Vec<_> = form.select(&sel("details.pf-group")).collect();
    assert_eq!(drawn.len(), groups.len(), "every group is listed, always");
    for (d, g) in drawn.iter().zip(groups) {
        assert_eq!(d.value().id(), Some(format!("group-{}", g.key).as_str()));
        let summary = d
            .select(&sel("summary"))
            .next()
            .expect("one control per group");
        let title: String = summary
            .select(&sel(".pf-group-title"))
            .next()
            .expect("a name")
            .text()
            .collect();
        assert_eq!(title, axgf_cms::profile::group_title("en", g), "{}", g.key);
        let count: String = summary
            .select(&sel(".pf-group-count"))
            .next()
            .expect("a count")
            .text()
            .collect();
        assert_eq!(count.trim(), "0", "{}: nothing filled", g.key);
        assert!(
            d.value().attr("open").is_none(),
            "{} is closed by default",
            g.key
        );
    }

    // Every attribute: a fieldset in its group, with a control for its first
    // row that can be typed into and posts with the form.
    let mut reached = 0;
    for g in groups {
        let group = form
            .select(&sel(&format!("details#group-{}", g.key)))
            .next()
            .unwrap_or_else(|| panic!("group {}", g.key));
        for a in g.attributes {
            let anchor = a.path.replace('.', "-");
            let fieldset = group
                .select(&sel(&format!(r#"fieldset.pf-attr[id="{anchor}"]"#)))
                .next()
                .unwrap_or_else(|| panic!("{} is in the editor, in {}", a.path, g.key));
            let row = format!("{}.0.", a.path);
            let controls = fieldset
                .select(&sel("input, select, textarea"))
                .filter(|c| typeable(c) && c.value().attr("name").unwrap().starts_with(&row))
                .count();
            assert!(controls > 0, "{} can be typed into", a.path);
            reached += 1;
        }
    }
    let registry = axgf_rs::model::profile::registry::attributes().count();
    assert_eq!(reached, registry, "every attribute of the registry");
    assert_eq!(reached, 132, "the 132 attributes of AXGF 1.1");

    // No lump: nothing of the person's form waits behind "n more fields".
    assert!(form.select(&sel("details.more-fields")).next().is_none());

    // The identity fields — names, dates, places, parents — are on screen
    // without opening anything.
    for name in [
        "identity.name.display",
        "birth.date.value",
        "birth.place_id",
        "death.date.value",
        "death.place_id",
    ] {
        let el = form
            .select(&sel(&format!(r#"[name="{name}"]"#)))
            .next()
            .unwrap_or_else(|| panic!("{name} is drawn"));
        assert!(
            !el.ancestors()
                .filter_map(ElementRef::wrap)
                .any(|a| a.value().name() == "details"),
            "{name} is not behind a disclosure"
        );
    }
    // Parents are the Famille tab's: attaching this person to a family.
    assert!(
        form.select(&sel("section#famille details.ed-add"))
            .next()
            .is_some(),
        "the parents, on the Famille tab"
    );
}

/// A field in a group nobody has filled saves, and the group then says so.
#[tokio::test]
async fn a_field_typed_into_an_empty_group_is_saved() {
    let src = bundle("fill-src");
    let (app, _p) = app_with_bundle("fill", &src);
    let cookie = format!("{}; axgf_lang=en", admin_cookie(&app).await);
    let html =
        body_string(get_with_cookie(&app, &format!("/admin/person/{BARE}/edit"), &cookie).await)
            .await;
    let doc = Html::parse_document(&html);
    let mut pairs = submission(&edit_form(&doc, BARE));
    set(&mut pairs, "morphology.height.0.v", "181");

    let resp = post_form_as(
        &app,
        &cookie,
        &format!("/admin/person/{BARE}"),
        &encode(&pairs),
    )
    .await;
    let body = body_string(resp).await;
    let saved = stored(&app, &cookie, BARE).await;
    assert_eq!(
        saved["morphology"]["height"][0]["value"],
        181,
        "saved: {}",
        body.chars().take(400).collect::<String>()
    );
    assert_eq!(saved["identity"]["name"]["display"], "Nobody Recorded");

    let html =
        body_string(get_with_cookie(&app, &format!("/admin/person/{BARE}/edit"), &cookie).await)
            .await;
    let doc = Html::parse_document(&html);
    let morph = doc
        .select(&sel("details#group-morphology"))
        .next()
        .expect("morphology");
    assert!(
        morph.value().attr("open").is_some(),
        "a group with content opens"
    );
    let summary: String = morph
        .select(&sel("summary"))
        .next()
        .unwrap()
        .text()
        .collect();
    assert!(summary.contains("1 filled"), "{summary:?}");
    let health = doc.select(&sel("details#group-health")).next().unwrap();
    assert!(
        health.value().attr("open").is_none(),
        "the others stay closed"
    );

    // And the reader sees it, on the record's Profil tab.
    let profile = body_string(get(&app, &format!("/person/{BARE}?tab=profile")).await).await;
    assert!(profile.contains("181"), "the record shows the height");
}

/// The form posts all fourteen groups on every save. A save that touches
/// none of them must leave the profile exactly as stored — here, a height
/// recorded before 1.1 that only an edit of its group may move.
#[tokio::test]
async fn a_save_that_touches_no_group_leaves_the_profile_as_stored() {
    let src = bundle("untouched-src");
    let (app, _p) = app_with_bundle("untouched", &src);
    let cookie = format!("{}; axgf_lang=en", admin_cookie(&app).await);
    let before = stored(&app, &cookie, LEGACY).await;
    let html =
        body_string(get_with_cookie(&app, &format!("/admin/person/{LEGACY}/edit"), &cookie).await)
            .await;
    let doc = Html::parse_document(&html);
    let mut pairs = submission(&edit_form(&doc, LEGACY));
    set(&mut pairs, "identity.name.display", "Old Height, renamed");

    post_form_as(
        &app,
        &cookie,
        &format!("/admin/person/{LEGACY}"),
        &encode(&pairs),
    )
    .await;
    let after = stored(&app, &cookie, LEGACY).await;
    assert_eq!(after["identity"]["name"]["display"], "Old Height, renamed");
    assert_eq!(
        after["extensions"], before["extensions"],
        "the legacy height stays where it was"
    );
    assert!(
        after.get("morphology").is_none(),
        "and is not moved by a name"
    );
}

/// Changing a group reads that group back — and, as the one-group editor
/// always did, moves what was recorded before 1.1 to its 1.1 home.
#[tokio::test]
async fn a_save_that_changes_a_group_reads_it_back() {
    let src = bundle("touched-src");
    let (app, _p) = app_with_bundle("touched", &src);
    let cookie = format!("{}; axgf_lang=en", admin_cookie(&app).await);
    let html =
        body_string(get_with_cookie(&app, &format!("/admin/person/{LEGACY}/edit"), &cookie).await)
            .await;
    let doc = Html::parse_document(&html);
    let mut pairs = submission(&edit_form(&doc, LEGACY));
    set(&mut pairs, "morphology.height.0.v", "173");

    post_form_as(
        &app,
        &cookie,
        &format!("/admin/person/{LEGACY}"),
        &encode(&pairs),
    )
    .await;
    let after = stored(&app, &cookie, LEGACY).await;
    assert_eq!(after["morphology"]["height"][0]["value"], 173);
    assert_eq!(
        after["morphology"]["height"][0]["date"]["value"], "1914",
        "provenance kept"
    );
}

/// A save refused over the profile keeps everything typed, and opens the
/// group with the problem in it.
#[tokio::test]
async fn a_refused_profile_save_keeps_what_was_typed() {
    let src = bundle("refused-src");
    let (app, _p) = app_with_bundle("refused", &src);
    let cookie = format!("{}; axgf_lang=en", admin_cookie(&app).await);
    let html =
        body_string(get_with_cookie(&app, &format!("/admin/person/{BARE}/edit"), &cookie).await)
            .await;
    let doc = Html::parse_document(&html);
    let mut pairs = submission(&edit_form(&doc, BARE));
    set(
        &mut pairs,
        "identity.name.display",
        "Typed Before The Problem",
    );
    set(&mut pairs, "morphology.height.0.v", "not a number");

    let resp = post_form_as(
        &app,
        &cookie,
        &format!("/admin/person/{BARE}"),
        &encode(&pairs),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);
    let html = body_string(resp).await;
    let doc = Html::parse_document(&html);
    let morph = doc.select(&sel("details#group-morphology")).next().unwrap();
    assert!(
        morph.value().attr("open").is_some(),
        "the group with the problem is open"
    );
    assert!(
        morph.select(&sel(".pf-problems")).next().is_some(),
        "and says what"
    );
    assert!(
        html.contains("Typed Before The Problem"),
        "the name typed is kept"
    );
    let saved = stored(&app, &cookie, BARE).await;
    assert_eq!(
        saved["identity"]["name"]["display"], "Nobody Recorded",
        "nothing saved"
    );
}

/// A submission that does not carry a group — a form opened before this
/// version, or a script that posts only the raw document — must not clear
/// it: an absent field is not an edit.
#[tokio::test]
async fn a_form_without_the_groups_clears_none_of_them() {
    let src = bundle("absent-src");
    let (app, _p) = app_with_bundle("absent", &src);
    let cookie = format!("{}; axgf_lang=en", admin_cookie(&app).await);
    // Give the bare person a height through the editor first.
    let html =
        body_string(get_with_cookie(&app, &format!("/admin/person/{BARE}/edit"), &cookie).await)
            .await;
    let doc = Html::parse_document(&html);
    let mut pairs = submission(&edit_form(&doc, BARE));
    set(&mut pairs, "morphology.height.0.v", "181");
    post_form_as(
        &app,
        &cookie,
        &format!("/admin/person/{BARE}"),
        &encode(&pairs),
    )
    .await;
    assert_eq!(
        stored(&app, &cookie, BARE).await["morphology"]["height"][0]["value"],
        181
    );

    // Then post the form as it was before v0.4.1: its fields and the raw
    // document, no group.
    let html =
        body_string(get_with_cookie(&app, &format!("/admin/person/{BARE}/edit"), &cookie).await)
            .await;
    let doc = Html::parse_document(&html);
    let mut pairs: Vec<_> = submission(&edit_form(&doc, BARE))
        .into_iter()
        .filter(|(k, _)| !k.starts_with("pf.") && !k.contains(".0.") && !k.contains(".1."))
        .collect();
    set(
        &mut pairs,
        "identity.name.display",
        "Renamed Without Groups",
    );
    post_form_as(
        &app,
        &cookie,
        &format!("/admin/person/{BARE}"),
        &encode(&pairs),
    )
    .await;
    let after = stored(&app, &cookie, BARE).await;
    assert_eq!(
        after["identity"]["name"]["display"],
        "Renamed Without Groups"
    );
    assert_eq!(
        after["morphology"]["height"][0]["value"], 181,
        "the height survives"
    );
}
