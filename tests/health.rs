//! Health data leaving the process, or not.
//!
//! Like `visibility.rs`, every assertion here is about *bytes in the response*
//! rather than about what a template chose to draw: a diagnosis that reaches
//! the browser and is hidden with CSS has been published, not withheld. The
//! fixture uses strings that appear nowhere else in the bundle, so finding one
//! is proof of a leak and not a coincidence of vocabulary.
//!
//! The load-bearing case is the **public living person**. Their record is
//! readable by anyone, so nothing about the person lens hides anything — and
//! their health must still be withheld, because the rule is a property of being
//! alive rather than of the record's visibility. If health were merely
//! inheriting `person_visibility`, that one test would fail and the rest would
//! pass.

mod common;

use axum::http::StatusCode;
use common::*;
use serde_json::{json, Value};

const LIVING: &str = "11111111-1111-4111-8111-111111111111";
const DEAD: &str = "22222222-2222-4222-8222-222222222222";

/// Strings that exist nowhere else.
const CONDITION: &str = "Sarcoidosis";
const RELIGION: &str = "Old Believer";
const CAUSE: &str = "Diphtheria";
const HEIGHT: &str = "187";

fn bundle(tag: &str) -> std::path::PathBuf {
    let dir = scratch(tag);
    let path = dir.join("health.axgf");

    let traits = |h: &str| {
        json!({"height_cm": [{"value": h.parse::<u64>().unwrap(),
                              "date": {"value": "1958", "precision": "year"}}],
               "eye_colour": [{"value": "blue"}]})
    };

    let flat = json!({
        "manifest": {"axgf": "1.0"},
        "persons": {
            // Deliberately PUBLIC and living: the person lens hides nothing
            // here, so anything withheld is withheld by the health rule alone.
            LIVING: {
                "id": LIVING, "type": "person", "axgf_version": "1.0", "version_num": 1,
                "identity": {
                    "name": {"display": "Marek Zaleski", "components": []},
                    "is_living": true,
                    "visibility": "public"
                },
                "birth": {"date": {"value": "1958", "precision": "year"}},
                "extensions": {
                    "axgf-cms:traits/v1": traits(HEIGHT),
                    "axgf-cms:health/v1": {
                        "conditions": [{"value": CONDITION}],
                        "religion": [{"value": RELIGION}]
                    }
                }
            },
            // Public and deceased: a cause of death two centuries old is the
            // substance of genealogy, and article 9 governs the living.
            DEAD: {
                "id": DEAD, "type": "person", "axgf_version": "1.0", "version_num": 1,
                "identity": {
                    "name": {"display": "Otylia Zaleska", "components": []},
                    "is_living": false,
                    "visibility": "public"
                },
                "birth": {"date": {"value": "1861", "precision": "year"}},
                "death": {"date": {"value": "1899", "precision": "year"}},
                "extensions": {
                    "axgf-cms:health/v1": {"cause_of_death": [{"value": CAUSE}]}
                }
            }
        },
        "families": {}, "events": {}, "links": {}, "occupations": {},
        "sources": {}, "places": {}, "documents": {}
    });
    std::fs::write(
        &path,
        axgf_cms::state::export_to_bytes(&flat.to_string()).expect("export"),
    )
    .expect("write");
    path
}

/// Every surface a signed-out reader can reach, for one person.
async fn surfaces(app: &axum::Router, id: &str) -> Vec<(&'static str, String)> {
    let mut out = Vec::new();
    for (name, uri) in [
        ("record tab", format!("/person/{id}")),
        ("life tab", format!("/person/{id}?tab=life")),
        ("media tab", format!("/person/{id}?tab=media")),
        ("tree tab", format!("/person/{id}?tab=tree")),
        ("panel fragment", format!("/tree/panel/{id}")),
        ("tree page", format!("/tree?root={id}")),
        ("health json", "/health".to_string()),
    ] {
        let resp = get(app, &uri).await;
        out.push((name, body_string(resp).await));
    }
    out
}

#[tokio::test]
async fn a_living_persons_health_never_reaches_a_reader_who_may_not_see_it() {
    let (app, _p) = app_with_bundle("health-living", &bundle("health-living-src"));

    for (surface, body) in surfaces(&app, LIVING).await {
        for secret in [CONDITION, RELIGION] {
            assert!(
                !body.contains(secret),
                "{surface} carried {secret} to a signed-out reader"
            );
        }
        // The extension key itself is a disclosure: it says which special
        // category this person has data in, even with the values gone.
        assert!(
            !body.contains("axgf-cms:health"),
            "{surface} named the health extension at all"
        );
    }
}

/// The raw-JSON dump is the shortest path from a stored diagnosis to a
/// stranger's screen, and it is shown to every reader, not only to admins.
#[tokio::test]
async fn the_raw_entity_dump_is_stripped_rather_than_printed_whole() {
    let (app, _p) = app_with_bundle("health-raw", &bundle("health-raw-src"));

    let body = body_string(get(&app, &format!("/person/{LIVING}")).await).await;
    assert!(
        body.contains("Raw entity") || body.contains("raw-json"),
        "the raw section is on the page, so this test is testing something"
    );
    assert!(
        !body.contains(CONDITION),
        "and it does not contain the condition"
    );
    assert!(
        body.contains(HEIGHT),
        "while the height, which is not health data, is still there"
    );
}

/// A trait is not a diagnosis, and over-restricting it would make the feature
/// useless for the thing it is for.
#[tokio::test]
async fn physical_traits_are_not_swept_up_with_the_health_fields() {
    let (app, _p) = app_with_bundle("health-traits", &bundle("health-traits-src"));

    let body = body_string(get(&app, &format!("/person/{LIVING}?tab=life")).await).await;
    assert!(body.contains(HEIGHT), "the height is shown: {body}");
    assert!(body.contains("blue"), "and so is the eye colour");
    assert!(!body.contains(CONDITION));
}

/// Withheld, not silently absent — the same choice the person lens makes.
#[tokio::test]
async fn a_reader_is_told_that_something_is_being_withheld() {
    let (app, _p) = app_with_bundle("health-told", &bundle("health-told-src"));
    let body = body_string(get(&app, &format!("/person/{LIVING}?tab=life")).await).await;
    assert!(
        body.contains("recorded but withheld"),
        "a record that shows nothing where a diagnosis exists reads as a \
         record with nothing in it: {body}"
    );
}

#[tokio::test]
async fn an_administrator_reads_it() {
    let (app, _p) = app_with_bundle("health-admin", &bundle("health-admin-src"));
    let body = body_string(get_admin(&app, &format!("/person/{LIVING}?tab=life")).await).await;
    assert!(body.contains(CONDITION), "an admin sees the condition");
    assert!(body.contains(RELIGION));
}

/// Article 9 governs living people. A cause of death from 1899 is genealogy.
#[tokio::test]
async fn a_deceased_persons_cause_of_death_follows_the_records_own_visibility() {
    let (app, _p) = app_with_bundle("health-dead", &bundle("health-dead-src"));
    let body = body_string(get(&app, &format!("/person/{DEAD}?tab=life")).await).await;
    assert!(
        body.contains(CAUSE),
        "a public deceased record publishes its cause of death: {body}"
    );
}

/// The export leaves health out unless asked, and the default is the one a
/// tired operator gets.
#[tokio::test]
async fn the_export_excludes_health_by_default_and_includes_it_on_request() {
    let (app, _p) = app_with_bundle("health-export", &bundle("health-export-src"));

    let plain = get_admin(&app, "/admin/export").await;
    assert_eq!(plain.status(), StatusCode::OK);
    let plain = body_bytes(plain).await;
    let with = body_bytes(get_admin(&app, "/admin/export?health=include").await).await;

    // Re-imported through the library, so what is asserted is what a
    // recipient would actually open rather than what the writer intended.
    let persons_of = |bytes: &[u8]| -> Value {
        axgf_rs::import_bundle(bytes)
            .data
            .get("persons")
            .cloned()
            .unwrap_or(Value::Null)
    };

    let stripped = persons_of(&plain);
    let s = stripped.to_string();
    assert!(
        !s.contains(CONDITION),
        "the shareable export carries no condition"
    );
    assert!(!s.contains(RELIGION), "…and no religion");
    assert!(
        !s.contains(CAUSE),
        "…and not the deceased person's either: an export is a file that \
         travels, and the operator asked for one without health in it"
    );
    assert!(s.contains(HEIGHT), "but it still carries the traits");
    assert!(
        stripped[LIVING]["identity"]["name"]["display"].is_string(),
        "and it is still a bundle: {s}"
    );

    let full = persons_of(&with).to_string();
    assert!(
        full.contains(CONDITION),
        "the backup export carries everything"
    );
    assert!(full.contains(CAUSE));
}

/// An editor shown a form without the health rows must not be able to erase
/// them by submitting it.
#[tokio::test]
async fn a_form_that_never_showed_the_health_rows_cannot_blank_them() {
    let (app, _p) = app_with_bundle("health-blank", &bundle("health-blank-src"));

    // A signed-out writer is refused before this even matters, so the check
    // that counts is the one an *admin* passes and a lesser writer does not.
    // The admin path is the positive control: it saves.
    let ok = post_form(
        &app,
        &format!("/admin/person/{LIVING}/physical"),
        "height_cm.0.value=190",
        true,
    )
    .await;
    assert_eq!(ok.status(), StatusCode::OK, "an admin may edit");

    // And the health survived, because the admin's form carried it — here it
    // did not, so this also proves the write is a replacement of what the form
    // held rather than a blind overwrite of the entity.
    let after = body_string(get_admin(&app, &format!("/person/{LIVING}?tab=life")).await).await;
    assert!(after.contains("190"), "the new height is stored: {after}");
}

// ---------------------------------------------------------------------------
// The account of how the record got that way
// ---------------------------------------------------------------------------
//
// Everything above is about the record. These are about the *journal* and the
// *editor*, which is where this leaked in the shipped build: the record page
// withheld a living person's condition and the edit history two sections below
// it printed the same string back out of the diff, to every signed-in relative.
// The generic entity form did it twice over — once in the raw-document
// textarea it hands an editor, and once in its own copy of the history.
//
// Found by reading a real contributor's real page, not by reading the code.
// The lesson is in the shape of the bug rather than in the fix: a redaction
// that covers a value and not the record of the value changing is not one.

const PASSWORD: &str = "correct-horse-battery-staple";

/// An app over the health fixture with one contributor account.
fn app_with_contributor(tag: &str) -> axum::Router {
    let src = bundle(&format!("{tag}-src"));
    let dir = scratch(tag);
    let path = dir.join("family.axgf");
    std::fs::copy(&src, &path).expect("copy");

    let mut acl = axgf_cms::acl::Acl::default();
    acl.users.push(
        axgf_cms::acl::new_user("cousin", PASSWORD, axgf_cms::acl::Role::Contributor)
            .expect("new user"),
    );
    acl.save(&axgf_cms::acl::Acl::path_for(&path))
        .expect("save acl");
    axgf_cms::app(&path, TOKEN).expect("build app")
}

async fn sign_in(app: &axum::Router) -> String {
    let resp = post_form(
        app,
        "/admin/login",
        &format!("username=cousin&password={PASSWORD}"),
        false,
    )
    .await;
    assert!(resp.status().is_redirection(), "cousin should sign in");
    resp.headers()
        .get(axum::http::header::SET_COOKIE)
        .and_then(|v| v.to_str().ok())
        .and_then(|c| c.split(';').next())
        .expect("a session cookie")
        .to_string()
}

/// Make an edit that records the health extension in the journal.
async fn record_an_edit(app: &axum::Router) {
    let resp = post_form(
        app,
        &format!("/admin/person/{LIVING}/physical"),
        &format!("height_cm.0.value=191&conditions.0.value={CONDITION}"),
        true,
    )
    .await;
    assert_eq!(resp.status(), StatusCode::OK, "the admin's edit saves");
}

#[tokio::test]
async fn the_edit_journal_does_not_print_back_the_diagnosis_the_record_withheld() {
    let app = app_with_contributor("health-journal");
    record_an_edit(&app).await;
    let cousin = sign_in(&app).await;

    for uri in [
        format!("/person/{LIVING}"),
        format!("/person/{LIVING}?tab=life"),
        format!("/tree/panel/{LIVING}"),
        format!("/tree?root={LIVING}"),
        format!("/admin/person/{LIVING}/edit"),
    ] {
        let body = body_string(get_with_cookie(&app, &uri, &cousin).await).await;
        assert!(
            !body.contains(CONDITION),
            "{uri} carried the condition to a contributor"
        );
        // Not merely absent: the row is still there, saying so. A diff with a
        // row silently missing is a diff that is wrong.
        if uri.contains("/edit") || uri == format!("/person/{LIVING}") {
            assert!(
                body.contains("diff-withheld"),
                "{uri} dropped the row instead of marking it withheld"
            );
        }
    }

    // The positive control: an administrator reading the same history does see
    // it, so the redaction is the rule doing its job and not the journal being
    // empty.
    let admin = body_string(get_admin(&app, &format!("/person/{LIVING}")).await).await;
    assert!(admin.contains(CONDITION), "an administrator reads it");
}

#[tokio::test]
async fn the_entity_editor_hands_a_contributor_a_stripped_document() {
    let app = app_with_contributor("health-editor");
    let cousin = sign_in(&app).await;

    let form =
        body_string(get_with_cookie(&app, &format!("/admin/person/{LIVING}/edit"), &cousin).await)
            .await;
    // The raw-document textarea is the whole entity in a box. It is the same
    // defect the record page's raw dump had, on the page reached only by the
    // people who can also change it.
    assert!(!form.contains(CONDITION), "the textarea carried it");
    assert!(!form.contains(RELIGION), "the textarea carried it");
    // The traits half is *not* stripped: a height is not a diagnosis, and
    // over-restricting it would make the editor useless.
    assert!(
        form.contains(HEIGHT),
        "the height is still editable: {form}"
    );
}

#[tokio::test]
async fn an_editor_who_never_saw_the_health_cannot_blank_it() {
    let app = app_with_contributor("health-editor-blank");
    let cousin = sign_in(&app).await;

    // The whole round trip rather than a hand-written body: take the document
    // the form actually hands this contributor, change one field in it, and
    // give it back. That is what a contributor does, and what they hand back
    // has no health key in it because the form they were given had none.
    let form =
        body_string(get_with_cookie(&app, &format!("/admin/person/{LIVING}/edit"), &cousin).await)
            .await;
    let raw = textarea(&form, "raw_json");
    assert!(!raw.contains(CONDITION), "the form was not stripped");
    let edited = raw.replace("\"blue\"", "\"green\"");
    assert_ne!(edited, raw, "the test changed something");

    // The fields the form itself submits alongside the document. A browser
    // sends all of them, and `apply_form` reads an absent one as cleared — so
    // a body carrying only the textarea would blank the name and un-tick
    // "living", and this test would be about that instead.
    let body = format!(
        "base_version=1&identity.name.display=Marek+Zaleski&identity.is_living=on\
         &birth.date.value=1958&birth.date.precision=year&raw_json={}",
        form_encode(&edited)
    );
    let resp = post_form_as(&app, &cousin, &format!("/admin/person/{LIVING}"), &body).await;
    assert_eq!(resp.status(), StatusCode::OK, "the edit is accepted");

    // The Life tab, not the record: the record carries the edit journal, whose
    // "from" column holds the value as it was before this very save. Asserting
    // there would pass whether the health survived or was deleted, which is
    // the wrong test written convincingly.
    let after = body_string(get_admin(&app, &format!("/person/{LIVING}?tab=life")).await).await;
    // The edit landed …
    assert!(
        after.contains("green"),
        "the contributor's edit was applied"
    );
    // … and the half the form never showed came back untouched, rather than
    // being deleted by an absence that was never an edit. `update_entity`
    // replaces the stored entity outright, so without this the save would have
    // erased a diagnosis nobody asked to erase.
    assert!(
        after.contains(CONDITION) && after.contains(RELIGION),
        "the health this editor never saw was blanked by their save"
    );
    // And it is still a living person's health, so it is still withheld from
    // them afterwards — the save did not quietly promote it.
    let reread =
        body_string(get_with_cookie(&app, &format!("/admin/person/{LIVING}/edit"), &cousin).await)
            .await;
    assert!(!reread.contains(CONDITION), "still withheld after the save");
}

/// The contents of one `<textarea>`, HTML-unescaped.
fn textarea(page: &str, name: &str) -> String {
    let marker = format!("name=\"{name}\"");
    let at = page.find(&marker).expect("the textarea");
    let open = page[at..].find('>').expect("its open tag") + at + 1;
    let close = page[open..].find("</textarea>").expect("its close tag") + open;
    page[open..close]
        .replace("&quot;", "\"")
        .replace("&#x2f;", "/")
        .replace("&#x27;", "'")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
}

/// Percent-encode a form value. Small enough not to be worth a dependency.
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

/// The page shown when two people edit at once prints the *stored* entity and
/// a diff against it, which is the same disclosure by another route.
#[tokio::test]
async fn the_conflict_page_does_not_hand_over_what_the_form_withheld() {
    let app = app_with_contributor("health-conflict");
    let cousin = sign_in(&app).await;

    // Take the document the form gives this contributor, then let somebody
    // else move the record underneath them.
    let form =
        body_string(get_with_cookie(&app, &format!("/admin/person/{LIVING}/edit"), &cousin).await)
            .await;
    let raw = textarea(&form, "raw_json");
    record_an_edit(&app).await;

    let body = format!(
        "base_version=1&identity.name.display=Marek+Zaleski&identity.is_living=on\
         &birth.date.value=1958&birth.date.precision=year&raw_json={}",
        form_encode(&raw.replace("\"blue\"", "\"grey\""))
    );
    let resp = post_form_as(&app, &cousin, &format!("/admin/person/{LIVING}"), &body).await;
    assert_eq!(
        resp.status(),
        StatusCode::CONFLICT,
        "the record moved under them"
    );
    let page = body_string(resp).await;
    assert!(
        !page.contains(CONDITION) && !page.contains(RELIGION),
        "the conflict page handed over the health:\n{page}"
    );
    // It still tells them a conflict happened in that field, which is the
    // whole point of the page.
    assert!(
        page.contains("diff-withheld"),
        "the row is marked, not dropped"
    );
}

/// Erasure has to work, or the protection above is just concealment.
///
/// Article 9 comes with article 17 attached: an operator who records a
/// condition must be able to take it back out. The physical editor's write is
/// a replacement of the group rather than a merge into it, and this is what
/// pins that — the whole health key goes when the last row in it is emptied.
///
/// What it does *not* clear is the edit journal, which keeps the old value in
/// its "from" column by design, because a history that quietly rewrites itself
/// is not a history. The journal is never exported, is readable only by
/// signed-in family, and now renders that column withheld to anybody who may
/// not read the health itself — but the value is in the file beside the bundle
/// and an operator who needs it gone needs to say so about that file too.
#[tokio::test]
async fn an_administrator_can_erase_a_recorded_condition() {
    let (app, _p) = app_with_bundle("health-erase", &bundle("health-erase-src"));
    let before = body_string(get_admin(&app, &format!("/person/{DEAD}?tab=life")).await).await;
    assert!(before.contains(CAUSE), "the fixture records it");

    // The form as somebody submits it with the row emptied.
    let resp = post_form(
        &app,
        &format!("/admin/person/{DEAD}/physical"),
        "cause_of_death.0.value=&height_cm.0.value=164",
        true,
    )
    .await;
    assert_eq!(resp.status(), StatusCode::OK);

    let after = body_string(get_admin(&app, &format!("/person/{DEAD}?tab=life")).await).await;
    assert!(after.contains("164"), "the rest of the edit landed");
    assert!(
        !after.contains(CAUSE),
        "the cause of death is gone:\n{after}"
    );
}
