//! The four sensitive classes of AXGF 1.1 — and the behavioural profile of the
//! living — against every surface they have leaked through before.
//!
//! `health.rs` holds the same rule for the application's own health extension.
//! This file holds it for everything 1.1 adds, one class at a time, because
//! the specification requires the classes to be independent (SPEC_1.1 §4.1):
//! a reader may be shown a criminal record and not a diagnosis, and an export
//! may carry health and leave the genome out. So each class gets its own test
//! on each surface, generated below, and a defect that forgets one class on
//! one surface fails exactly the tests that name them.
//!
//! As in `health.rs`, every assertion is about bytes in a response, the
//! markers appear nowhere else in the bundle, and the subject is a **public
//! living** person: the person lens hides nothing about them, so whatever is
//! withheld is withheld by the class rule alone.
//!
//! The surfaces, in the order the rule came to cover them:
//!
//! | surface   | what it prints                                             |
//! |-----------|------------------------------------------------------------|
//! | record    | the person page, every tab, and its raw-entity dump        |
//! | journal   | the edit history on the record, the panel and the editor   |
//! | raw       | the entity editor's raw-document textarea and its fields   |
//! | conflict  | the conflict page's diff and its copy of the stored entity |
//! | resubmit  | the conflict page's box holding the editor's own document  |
//! | restore   | a save from a stripped form, which must not erase anything |
//! | profile   | a save of the class's own profile group, forged entry and all |
//! | export    | the archive, with each class left out unless chosen        |

mod common;

use axum::http::StatusCode;
use base64::Engine as _;
use common::*;
use serde_json::{json, Value};

const LIVING: &str = "11111111-1111-4111-8111-111111111111";
const DEAD: &str = "22222222-2222-4222-8222-222222222222";
const DOC_FINGER: &str = "33333333-3333-4333-8333-333333333333";

const PASSWORD: &str = "correct-horse-battery-staple";

/// A scope under test: its spelling, a marker that is in the fixture, and a
/// second one the administrator's edit adds.
#[derive(Clone, Copy)]
struct Class {
    name: &'static str,
    first: &'static str,
    second: &'static str,
}

const HEALTH: Class = Class {
    name: "health",
    first: "Sarcoidosis",
    second: "Nephrolithiasis",
};
const BIOMETRICS: Class = Class {
    name: "biometrics",
    first: "Whorlscan",
    second: "Loopreader",
};
const GENOMICS: Class = Class {
    name: "genomics",
    first: "rs429358",
    second: "rs7412",
};
const LEGAL: Class = Class {
    name: "legal",
    first: "saffron smuggling",
    second: "forged passports",
};
const BEHAVIOUR: Class = Class {
    name: "behaviour",
    first: "Origami",
    second: "Kirigami",
};

/// Not class data: must survive every strip, as the positive control.
const OPEN: &str = "Cracow-shaped birthmark";

/// The class data of the living subject, with either marker.
fn class_blocks(marker: fn(&Class) -> &'static str) -> Value {
    json!({
        "health": {"conditions": [{"value": {"description": marker(&HEALTH)}}]},
        "biometrics": {"fingerprints": [{"value": {
            "document_id": DOC_FINGER, "artefact_type": "fingerprint_card",
            "generator": marker(&BIOMETRICS)}}]},
        "genomics": {"risk_variants": [{"value": {"variant": marker(&GENOMICS), "gene": "APOE"}}]},
        "legal": {"criminal_record": [{"value": {"offence": marker(&LEGAL)}}]},
        "personality": {"hobbies": [{"value": marker(&BEHAVIOUR)}]}
    })
}

fn living_person() -> Value {
    let mut p = json!({
        "id": LIVING, "type": "person", "axgf_version": "1.1", "version_num": 1,
        "identity": {
            "name": {"display": "Marek Zaleski", "components": []},
            "gender": {"value": "M"},
            "is_living": true,
            "visibility": "public"
        },
        "birth": {"date": {"value": "1958", "precision": "year"}},
        "morphology": {"distinguishing_features": [{"value": OPEN}]},
        "documents": [{"document_id": DOC_FINGER, "role": "subject"}]
    });
    for (k, v) in class_blocks(|c| c.first).as_object().unwrap() {
        p[k] = v.clone();
    }
    p
}

fn bundle(tag: &str) -> common::Scratch {
    let dir = scratch(tag);
    let path = dir.join("classes.axgf");
    let card = b"fingerprint card scan".to_vec();
    let zip_path = format!("documents/files/{DOC_FINGER}.txt");
    let flat = json!({
        "manifest": {"axgf": "1.1"},
        "persons": {
            LIVING: living_person(),
            // Public and deceased, with a genome and a conviction the family
            // tightened: the three rules that are not "living means private".
            DEAD: {
                "id": DEAD, "type": "person", "axgf_version": "1.1", "version_num": 1,
                "identity": {
                    "name": {"display": "Otylia Zaleska", "components": []},
                    "gender": {"value": "F"},
                    "is_living": false,
                    "visibility": "public",
                    "class_visibility": {"legal": "private"}
                },
                "death": {"date": {"value": "1899", "precision": "year"}, "cause": "Diphtheria"},
                "genomics": {"mt_haplogroup": {"value": {"major": "H", "subclade": "H1c3"}}},
                "legal": {"criminal_record": [{"value": {"offence": "poaching pheasants"}}]},
                "personality": {"hobbies": [{"value": "Bobbin lace"}]}
            }
        },
        "families": {}, "events": {}, "links": {}, "occupations": {},
        "sources": {}, "places": {},
        "documents": {
            DOC_FINGER: {
                "id": DOC_FINGER, "type": "document", "axgf_version": "1.0",
                "filename": "card.txt", "mime_type": "text/plain",
                "document_type": "other", "status": "present",
                "file": {"path": zip_path, "size_bytes": card.len(),
                         "sha256": axgf_cms::documents::sha256_hex(&card)},
                "linked_to": [{"entity_type": "person", "entity_id": LIVING}]
            }
        },
        "attachments": {zip_path: base64::engine::general_purpose::STANDARD.encode(&card)}
    });
    std::fs::write(
        &path,
        axgf_cms::state::export_to_bytes(&flat.to_string()).expect("export"),
    )
    .expect("write");
    dir.pointing_at(path)
}

/// An app over the fixture with one contributor, who may write but may not
/// read a living person's class data.
fn app_with_contributor(tag: &str) -> (axum::Router, common::Scratch) {
    let src = bundle(&format!("{tag}-src"));
    let dir = scratch(tag);
    let path = dir.join("family.axgf");
    std::fs::copy(&src, &path).expect("copy");
    let mut acl = axgf_cms::acl::Acl::default();
    acl.users.push(
        axgf_cms::acl::new_user("cousin", PASSWORD, axgf_cms::acl::Role::Contributor)
            .expect("new user"),
    );
    acl.users.push(
        axgf_cms::acl::new_user("aunt", PASSWORD, axgf_cms::acl::Role::Viewer).expect("new user"),
    );
    acl.save(&axgf_cms::acl::Acl::path_for(&path))
        .expect("save acl");
    let app = axgf_cms::app(&path, TOKEN).expect("build app");
    (app, dir.pointing_at(path))
}

async fn sign_in(app: &axum::Router, who: &str) -> String {
    let resp = post_form(
        app,
        "/admin/login",
        &format!("username={who}&password={PASSWORD}"),
        false,
    )
    .await;
    assert!(resp.status().is_redirection(), "{who} should sign in");
    resp.headers()
        .get(axum::http::header::SET_COOKIE)
        .and_then(|v| v.to_str().ok())
        .and_then(|c| c.split(';').next())
        .expect("a session cookie")
        .to_string()
}

/// The generic editor's own fields, as a browser submits them beside the
/// document: `apply_form` reads an absent field as cleared.
const FORM_FIELDS: &str = "identity.name.display=Marek+Zaleski&identity.gender.value=M\
     &identity.is_living=on&birth.date.value=1958&birth.date.precision=year";

/// An administrator adds a second claim to every class. The journal then holds
/// every class's values, and the version moves on.
async fn admin_adds_second_claims(app: &axum::Router) {
    let form = body_string(get_admin(app, &format!("/admin/person/{LIVING}/edit")).await).await;
    let mut entity: Value = serde_json::from_str(&textarea(&form, "raw_json")).expect("json");
    let version = entity["version_num"].as_u64().expect("a version");
    let second = class_blocks(|c| c.second);
    for (block, attrs) in second.as_object().unwrap() {
        for (attr, claims) in attrs.as_object().unwrap() {
            let list = entity[block][attr].as_array_mut().expect("a series");
            list.push(claims[0].clone());
        }
    }
    let body = format!(
        "base_version={version}&{FORM_FIELDS}&raw_json={}",
        form_encode(&entity.to_string())
    );
    let resp = post_form(app, &format!("/admin/person/{LIVING}"), &body, true).await;
    assert_eq!(
        resp.status(),
        StatusCode::OK,
        "the administrator's edit saves"
    );
}

/// The contents of one `<textarea>`, HTML-unescaped.
fn textarea(page: &str, name: &str) -> String {
    let marker = format!("name=\"{name}\"");
    let at = page.find(&marker).expect("the textarea");
    let open = page[at..].find('>').expect("its open tag") + at + 1;
    let close = page[open..].find("</textarea>").expect("its close tag") + open;
    unescape(&page[open..close])
}

fn unescape(s: &str) -> String {
    s.replace("&quot;", "\"")
        .replace("&#x2f;", "/")
        .replace("&#x27;", "'")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
}

/// Every `<ol class="history">` on a page, joined.
fn histories(page: &str) -> String {
    let mut out = String::new();
    let mut rest = page;
    while let Some(at) = rest.find("<ol class=\"history\">") {
        let end = rest[at..].find("</ol>").expect("closed") + at;
        out.push_str(&rest[at..end]);
        rest = &rest[end..];
    }
    unescape(&out)
}

/// A page with every `<textarea>` removed, for the assertions about what the
/// rest of it prints.
fn without_textareas(page: &str) -> String {
    let mut out = String::new();
    let mut rest = page;
    while let Some(at) = rest.find("<textarea") {
        out.push_str(&rest[..at]);
        let end = rest[at..].find("</textarea>").expect("closed") + at + "</textarea>".len();
        rest = &rest[end..];
    }
    out.push_str(rest);
    unescape(&out)
}

fn form_encode(s: &str) -> String {
    let mut out = String::with_capacity(s.len() * 3);
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

fn assert_absent(page: &str, class: &Class, surface: &str) {
    for m in [class.first, class.second] {
        assert!(
            !page.contains(m),
            "{surface} carried the {} marker {m:?} to a reader who may not see it",
            class.name
        );
    }
}

// ---------------------------------------------------------------------------
// the surfaces
// ---------------------------------------------------------------------------

async fn record(class: Class) {
    let (app, _scratch) = app_with_contributor(&format!("cls-record-{}", class.name));
    let cousin = sign_in(&app, "cousin").await;
    for tab in ["", "?tab=life", "?tab=media", "?tab=tree", "?tab=profile"] {
        let uri = format!("/person/{LIVING}{tab}");
        let anon = body_string(get(&app, &uri).await).await;
        assert_absent(&anon, &class, &format!("{uri} (signed out)"));
        let body = body_string(get_with_cookie(&app, &uri, &cousin).await).await;
        assert_absent(&body, &class, &format!("{uri} (contributor)"));
    }
    let panel =
        body_string(get_with_cookie(&app, &format!("/tree/panel/{LIVING}"), &cousin).await).await;
    assert_absent(&panel, &class, "the tree panel");
    // The raw dump is on the page, and the open data in it is not withheld.
    let page = body_string(get(&app, &format!("/person/{LIVING}")).await).await;
    assert!(
        page.contains("raw-json") && page.contains(OPEN),
        "the dump is there"
    );
    // An administrator reads it, so it is withheld rather than absent.
    let admin = body_string(get_admin(&app, &format!("/person/{LIVING}")).await).await;
    assert!(
        admin.contains(class.first),
        "an administrator reads {}",
        class.name
    );
}

async fn journal(class: Class) {
    let (app, _scratch) = app_with_contributor(&format!("cls-journal-{}", class.name));
    admin_adds_second_claims(&app).await;
    let cousin = sign_in(&app, "cousin").await;
    for uri in [
        // The record's history is its own tab; the panel and the editor still
        // draw it inline.
        format!("/person/{LIVING}?tab=history"),
        format!("/tree/panel/{LIVING}"),
        format!("/admin/person/{LIVING}/edit"),
    ] {
        let page = body_string(get_with_cookie(&app, &uri, &cousin).await).await;
        let history = histories(&page);
        assert!(!history.is_empty(), "{uri} shows a history");
        assert_absent(&history, &class, &format!("the history on {uri}"));
        // Not merely absent: the row is still there, saying so.
        assert!(
            history.contains("diff-withheld"),
            "{uri} dropped the row instead of marking it withheld"
        );
    }
    let admin = histories(
        &body_string(get_admin(&app, &format!("/person/{LIVING}?tab=history")).await).await,
    );
    assert!(
        admin.contains(class.second),
        "an administrator reads {} in the history",
        class.name
    );
}

async fn raw(class: Class) {
    let (app, _scratch) = app_with_contributor(&format!("cls-raw-{}", class.name));
    let cousin = sign_in(&app, "cousin").await;
    let form =
        body_string(get_with_cookie(&app, &format!("/admin/person/{LIVING}/edit"), &cousin).await)
            .await;
    let raw = textarea(&form, "raw_json");
    assert_absent(&raw, &class, "the raw-document textarea");
    assert!(raw.contains(OPEN), "the open data is still editable: {raw}");
    // The inputs above the textarea are the entity too.
    let fields = without_textareas(&form);
    assert_absent(&fields, &class, "the editor's fields");
}

/// Load the editor, let the administrator move the record, then save.
async fn conflict_page(class: Class, tag: &str) -> String {
    let (app, _scratch) = app_with_contributor(&format!("cls-{tag}-{}", class.name));
    let cousin = sign_in(&app, "cousin").await;
    let form =
        body_string(get_with_cookie(&app, &format!("/admin/person/{LIVING}/edit"), &cousin).await)
            .await;
    let raw = textarea(&form, "raw_json");
    admin_adds_second_claims(&app).await;
    let body = format!(
        "base_version=1&{FORM_FIELDS}&raw_json={}",
        form_encode(&raw.replace("Cracow-shaped", "Gdansk-shaped"))
    );
    let resp = post_form_as(&app, &cousin, &format!("/admin/person/{LIVING}"), &body).await;
    expect_status(resp, StatusCode::CONFLICT, "the record moved under them").await
}

async fn conflict(class: Class) {
    let page = conflict_page(class, "conflict").await;
    let rest = without_textareas(&page);
    assert_absent(&rest, &class, "the conflict page");
    assert!(
        rest.contains("diff-withheld"),
        "the conflict rows are marked, not dropped"
    );
    assert!(rest.contains("Gdansk-shaped"), "their own change is shown");
}

async fn resubmit(class: Class) {
    let page = conflict_page(class, "resubmit").await;
    let mine = textarea(&page, "raw_json");
    assert_absent(&mine, &class, "the resubmit box");
    assert!(
        mine.contains("Gdansk-shaped"),
        "their document is carried forward"
    );
}

async fn restore(class: Class) {
    let (app, _scratch) = app_with_contributor(&format!("cls-restore-{}", class.name));
    let cousin = sign_in(&app, "cousin").await;
    let form =
        body_string(get_with_cookie(&app, &format!("/admin/person/{LIVING}/edit"), &cousin).await)
            .await;
    let raw = textarea(&form, "raw_json");
    // A hand-written body could also try to write the class: it is discarded.
    let mut sent: Value =
        serde_json::from_str(&raw.replace("Cracow-shaped", "Gdansk-shaped")).expect("json");
    if class.name != "behaviour" {
        sent[class.name] = json!({"forged": [{"value": "forged by the form"}]});
    }
    let body = format!(
        "base_version=1&{FORM_FIELDS}&raw_json={}",
        form_encode(&sent.to_string())
    );
    let resp = post_form_as(&app, &cousin, &format!("/admin/person/{LIVING}"), &body).await;
    expect_status(resp, StatusCode::OK, "the contributor's save").await;

    let stored = textarea(
        &body_string(get_admin(&app, &format!("/admin/person/{LIVING}/edit")).await).await,
        "raw_json",
    );
    assert!(stored.contains("Gdansk-shaped"), "the edit landed");
    assert!(
        stored.contains(class.first),
        "the {} data this editor never saw was erased by their save",
        class.name
    );
    assert!(
        !stored.contains("forged by the form"),
        "a class the editor may not read was written through the form"
    );
}

/// The profile editor is the other way into a class, one group at a time. A
/// contributor posts that class's group with a forged entry in it: nothing of
/// it is written, and the entry already recorded is not erased by a save of a
/// group whose class they may not read.
///
/// Two barriers stand here, and each alone holds: the form refuses to apply
/// an attribute its reader may not read, and the save puts every withheld
/// location back from the stored person. So a mutation that removes one of
/// them is caught by nothing — it changes nothing — and only removing both
/// fails this test. That is the defence in depth working, and the mutation
/// table says so rather than hiding it.
async fn profile_editor(class: Class) {
    let (app, _scratch) = app_with_contributor(&format!("cls-profile-{}", class.name));
    let cousin = sign_in(&app, "cousin").await;
    let (group, forged, witness) = match class.name {
        "health" => ("health", "health.conditions.0.v.description=forged+by+the+form", "forged by the form"),
        "biometrics" => ("biometrics", "biometrics.vocal_timbre.0.v=hoarse", "\"hoarse\""),
        "genomics" => (
            "genomics",
            "genomics.risk_variants.0.v.variant=forged+by+the+form&genomics.risk_variants.0.v.gene=TP53",
            "forged by the form",
        ),
        "legal" => ("legal", "legal.criminal_record.0.v.offence=forged+by+the+form", "forged by the form"),
        _ => ("personality", "personality.hobbies.0.v=forged+by+the+form", "forged by the form"),
    };
    let resp = post_form_as(
        &app,
        &cousin,
        &format!("/admin/person/{LIVING}/profile/{group}"),
        &format!("base_version=1&{forged}"),
    )
    .await;
    expect_status(resp, StatusCode::OK, "the contributor's save of the group").await;

    let stored = textarea(
        &body_string(get_admin(&app, &format!("/admin/person/{LIVING}/edit")).await).await,
        "raw_json",
    );
    assert!(
        !stored.contains(witness),
        "a {} value was written through the profile editor by a reader who may not read it",
        class.name
    );
    assert!(
        stored.contains(class.first),
        "the {} entry this editor never saw was erased by their save of the group",
        class.name
    );
}

async fn export(class: Class) {
    let (app, _scratch) = app_with_contributor(&format!("cls-export-{}", class.name));
    let import = |bytes: Vec<u8>| -> Value { axgf_rs::import_bundle(&bytes).data };

    let plain = import(body_bytes(get_admin(&app, "/admin/export").await).await);
    let s = plain["persons"][LIVING].to_string();
    assert!(
        !s.contains(class.first),
        "the default export carries {}",
        class.name
    );
    assert!(s.contains(OPEN), "and it is still the record: {s}");
    let withheld = plain["manifest"]["privacy"]["withheld_classes"].to_string();
    if class.name == "behaviour" {
        assert!(
            !withheld.contains("behaviour"),
            "not a class of the specification"
        );
    } else {
        assert!(
            withheld.contains(class.name),
            "the manifest says {} was left out: {withheld}",
            class.name
        );
    }

    let chosen = import(
        body_bytes(get_admin(&app, &format!("/admin/export?{}=include", class.name)).await).await,
    );
    let s = chosen["persons"][LIVING].to_string();
    assert!(
        s.contains(class.first),
        "{} included on request",
        class.name
    );
    // …and only that one.
    for other in [HEALTH, BIOMETRICS, GENOMICS, LEGAL, BEHAVIOUR] {
        if other.name != class.name {
            assert!(
                !s.contains(other.first),
                "asking for {} also sent {}",
                class.name,
                other.name
            );
        }
    }
    let withheld = chosen["manifest"]["privacy"]["withheld_classes"].to_string();
    assert!(
        class.name == "behaviour" || !withheld.contains(class.name),
        "{} is listed as withheld from an export that carries it",
        class.name
    );
}

macro_rules! matrix {
    ($($name:ident => $surface:ident($class:expr)),* $(,)?) => {
        $(
            #[tokio::test]
            async fn $name() {
                $surface($class).await;
            }
        )*
    };
}

matrix! {
    health_record => record(HEALTH),
    health_journal => journal(HEALTH),
    health_raw => raw(HEALTH),
    health_conflict => conflict(HEALTH),
    health_resubmit => resubmit(HEALTH),
    health_restore => restore(HEALTH),
    health_profile => profile_editor(HEALTH),
    health_export => export(HEALTH),

    biometrics_record => record(BIOMETRICS),
    biometrics_journal => journal(BIOMETRICS),
    biometrics_raw => raw(BIOMETRICS),
    biometrics_conflict => conflict(BIOMETRICS),
    biometrics_resubmit => resubmit(BIOMETRICS),
    biometrics_restore => restore(BIOMETRICS),
    biometrics_profile => profile_editor(BIOMETRICS),
    biometrics_export => export(BIOMETRICS),

    genomics_record => record(GENOMICS),
    genomics_journal => journal(GENOMICS),
    genomics_raw => raw(GENOMICS),
    genomics_conflict => conflict(GENOMICS),
    genomics_resubmit => resubmit(GENOMICS),
    genomics_restore => restore(GENOMICS),
    genomics_profile => profile_editor(GENOMICS),
    genomics_export => export(GENOMICS),

    legal_record => record(LEGAL),
    legal_journal => journal(LEGAL),
    legal_raw => raw(LEGAL),
    legal_conflict => conflict(LEGAL),
    legal_resubmit => resubmit(LEGAL),
    legal_restore => restore(LEGAL),
    legal_profile => profile_editor(LEGAL),
    legal_export => export(LEGAL),

    behaviour_record => record(BEHAVIOUR),
    behaviour_journal => journal(BEHAVIOUR),
    behaviour_raw => raw(BEHAVIOUR),
    behaviour_conflict => conflict(BEHAVIOUR),
    behaviour_resubmit => resubmit(BEHAVIOUR),
    behaviour_restore => restore(BEHAVIOUR),
    behaviour_profile => profile_editor(BEHAVIOUR),
    behaviour_export => export(BEHAVIOUR),
}

// ---------------------------------------------------------------------------
// the rules that are not "living means private"
// ---------------------------------------------------------------------------

/// A deceased person's class data follows the record — except a genome, which
/// is never public, and a class the family tightened.
#[tokio::test]
async fn a_deceased_record_publishes_its_history_but_never_its_genome() {
    let (app, _scratch) = app_with_contributor("cls-deceased");
    let uri = format!("/person/{DEAD}");

    let anon = body_string(get(&app, &uri).await).await;
    assert!(
        anon.contains("Diphtheria"),
        "a public deceased record publishes its cause of death"
    );
    assert!(
        anon.contains("Bobbin lace"),
        "and a deceased person's hobbies are family history"
    );
    assert!(
        !anon.contains("H1c3"),
        "a genome is never public, even of the dead"
    );
    assert!(
        !anon.contains("poaching"),
        "the family made this one private"
    );

    let aunt = sign_in(&app, "aunt").await;
    let member = body_string(get_with_cookie(&app, &uri, &aunt).await).await;
    assert!(
        member.contains("H1c3"),
        "a signed-in member reads the genome of the dead"
    );
    assert!(
        !member.contains("poaching"),
        "tightened to private, so not a member"
    );

    let cousin = sign_in(&app, "cousin").await;
    let contributor = body_string(get_with_cookie(&app, &uri, &cousin).await).await;
    assert!(!contributor.contains("poaching"), "nor a contributor");

    let admin = body_string(get_admin(&app, &uri).await).await;
    assert!(admin.contains("poaching"), "an administrator reads it");
}

/// Documents a class attribute refers to are that attribute's data.
#[tokio::test]
async fn a_fingerprint_card_is_withheld_with_the_class_that_refers_to_it() {
    let (app, _scratch) = app_with_contributor("cls-document");
    let uri = format!("/document/{DOC_FINGER}/raw");
    let cousin = sign_in(&app, "cousin").await;
    for (who, resp) in [
        ("signed out", get(&app, &uri).await),
        ("a contributor", get_with_cookie(&app, &uri, &cousin).await),
    ] {
        assert_ne!(
            resp.status(),
            StatusCode::OK,
            "{who} fetched a biometric document attached to a person they may read"
        );
    }
    let resp = get_admin(&app, &uri).await;
    assert_eq!(resp.status(), StatusCode::OK, "an administrator fetches it");

    // Not listed on the record either, where its name would say what it is.
    let media =
        body_string(get_with_cookie(&app, &format!("/person/{LIVING}?tab=media"), &cousin).await)
            .await;
    assert!(!media.contains(DOC_FINGER), "the media tab listed it");

    // And not in the shareable export, bytes or entity or reference.
    let flat =
        axgf_rs::import_bundle(&body_bytes(get_admin(&app, "/admin/export").await).await).data;
    assert!(
        flat["documents"].get(DOC_FINGER).is_none(),
        "the entity was exported"
    );
    assert!(
        !flat["persons"][LIVING].to_string().contains(DOC_FINGER),
        "a reference to it was exported"
    );
    assert!(
        flat.get("attachments")
            .and_then(|a| a.as_object())
            .is_none_or(|a| a.is_empty()),
        "its bytes were exported"
    );
}

/// The presumption of death is a display rule and opens no class.
#[tokio::test]
async fn a_presumed_death_opens_no_class() {
    use axgf_cms::acl::Visibility as V;
    use axgf_cms::sensitive::Scope;
    let presumed = json!({
        "identity": {"is_living": true, "visibility": "public"},
        "birth": {"date": {"value": "1850", "precision": "year"}}
    });
    assert!(axgf_cms::living::status(&presumed).is_presumed());
    for scope in Scope::ALL {
        assert!(
            !axgf_cms::access::readable_scopes(&presumed, V::Contributors).contains(scope),
            "{} opened by a presumption",
            scope.as_str()
        );
    }
}

/// A value this build cannot read is not permission.
#[tokio::test]
async fn an_unreadable_class_visibility_closes_rather_than_opens() {
    use axgf_cms::acl::Visibility as V;
    use axgf_cms::sensitive::Scope;
    let person = json!({"identity": {"is_living": false, "visibility": "public",
                                     "class_visibility": {"health": "everyone"}}});
    let r = axgf_cms::access::readable_scopes(&person, V::Contributors);
    assert!(
        !r.contains(Scope::Health),
        "an unparseable entry is private"
    );
    assert!(
        r.contains(Scope::Legal),
        "and says nothing about the other classes"
    );
}

/// The documents editor lists every file in the archive to attach, and the
/// files already attached. A fingerprint card the reader may not open was in
/// both: its name printed in a select, and — since the attached list is
/// rebuilt from the form — detached by any save of a form that never showed it.
#[tokio::test]
async fn the_documents_editor_neither_names_nor_detaches_a_withheld_file() {
    let (app, _scratch) = app_with_contributor("cls-documents-editor");
    let cousin = sign_in(&app, "cousin").await;
    let page = body_string(
        get_with_cookie(&app, &format!("/admin/person/{LIVING}/documents"), &cousin).await,
    )
    .await;
    assert!(
        !page.contains("card.txt") && !page.contains(DOC_FINGER),
        "the withheld card is not offered or listed"
    );

    // The contributor saves the attachment list they were shown, which is
    // empty. The card stays attached.
    let resp = post_form_as(
        &app,
        &cousin,
        &format!("/admin/person/{LIVING}/documents"),
        "base_version=1",
    )
    .await;
    expect_status(resp, StatusCode::OK, "the contributor's save").await;
    let raw = body_string(get_admin(&app, &format!("/admin/person/{LIVING}/edit")).await).await;
    assert!(
        raw.contains(DOC_FINGER),
        "a file the editor could not see is not detached by their save"
    );

    // A hand-made form naming the card cannot attach it to somebody else.
    let resp = post_form_as(
        &app,
        &cousin,
        &format!("/admin/person/{DEAD}/documents"),
        &format!("base_version=1&doc.0.document_id={DOC_FINGER}"),
    )
    .await;
    expect_status(resp, StatusCode::OK, "the forged save").await;
    let raw = body_string(get_admin(&app, &format!("/admin/person/{DEAD}/edit")).await).await;
    assert!(
        !raw.contains(DOC_FINGER),
        "the card was not attached by name"
    );
}
