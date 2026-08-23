//! Two people editing the same record.
//!
//! Before this, the second save silently overwrote the first: no error, no
//! warning, one person's work simply gone. That is data loss, and it is what
//! made the CMS unusable by a family. What is pinned here is that the second
//! save is *refused*, that the refusal explains itself well enough to act on,
//! and that nothing is ever merged behind anyone's back.

mod common;

use axum::http::StatusCode;
use common::*;
use serde_json::{json, Value};

const ALICE: &str = "11111111-1111-4111-8111-111111111111";
const BOB: &str = "22222222-2222-4222-8222-222222222222";

fn person(id: &str, display: &str, version: u64) -> Value {
    json!({
        "id": id, "type": "person", "axgf_version": "1.0",
        "version_num": version,
        "identity": {
            "name": {"display": display, "components": [
                {"type": "given_name", "value": display, "order": 1}]},
            "gender": {"value": "F"}, "is_living": false,
            "visibility": "public"
        },
        "notes": "original"
    })
}

fn app(tag: &str) -> axum::Router {
    let dir = scratch(&format!("{tag}-src"));
    let path = dir.join("c.axgf");
    let flat = json!({
        "manifest": {"axgf": "1.0", "family": {"name": "Concurrent"}},
        "persons": {ALICE: person(ALICE, "Alice", 1), BOB: person(BOB, "Bob", 1)},
        "families": {}, "events": {}, "links": {}, "occupations": {},
        "sources": {}, "places": {}, "documents": {}
    });
    std::fs::write(
        &path,
        axgf_cms::state::export_to_bytes(&flat.to_string()).expect("export"),
    )
    .expect("write");
    app_with_bundle(tag, &path).0
}

/// Save `entity` for `id`, declaring that it was edited from `base_version`.
///
/// The typed fields are submitted alongside the raw JSON because that is what
/// the browser does, and because the form deliberately writes them over the
/// paths they own — posting the raw document alone would clear every mapped
/// field and the tests would be measuring that instead of the version check.
async fn save(
    app: &axum::Router,
    id: &str,
    base_version: u64,
    entity: &Value,
) -> axum::http::Response<axum::body::Body> {
    let field = |path: &str| -> String {
        let mut cur = entity;
        for seg in path.split('.') {
            match cur.get(seg) {
                Some(v) => cur = v,
                None => return String::new(),
            }
        }
        cur.as_str().unwrap_or_default().to_string()
    };
    let body = format!(
        "base_version={base_version}         &identity.name.display={}         &identity.gender.value={}         &notes={}         &raw_json={}",
        urlencode(&field("identity.name.display")),
        urlencode(&field("identity.gender.value")),
        urlencode(&field("notes")),
        urlencode(&entity.to_string())
    );
    post_form(app, &format!("/admin/person/{id}"), &body, true).await
}

/// The `version_num` the bundle currently holds for `id`.
async fn stored_version(app: &axum::Router, id: &str) -> u64 {
    let page = body_string(get_admin(app, &format!("/admin/person/{id}/edit")).await).await;
    let at = page
        .find("name=\"base_version\" value=\"")
        .expect("the edit form carries the version it was rendered from");
    page[at + "name=\"base_version\" value=\"".len()..]
        .split('"')
        .next()
        .unwrap()
        .parse()
        .expect("a number")
}

#[tokio::test]
async fn the_edit_form_carries_the_version_it_was_rendered_from() {
    // Without this the save has nothing to compare against, and every other
    // test here would be checking a check that cannot fire.
    let app = app("ce-form");
    assert_eq!(stored_version(&app, ALICE).await, 1);
}

#[tokio::test]
async fn a_second_edit_from_a_stale_version_is_refused_with_a_diff() {
    let app = app("ce-refused");

    // Both editors open the record at version 1.
    let base = stored_version(&app, ALICE).await;

    // The first one saves.
    let mut first = person(ALICE, "Alice", base);
    first["notes"] = json!("edited by the first editor");
    let resp = save(&app, ALICE, base, &first).await;
    assert_eq!(resp.status(), StatusCode::OK, "the first save succeeds");

    // The second, still holding version 1, saves something else.
    let mut second = person(ALICE, "Alice", base);
    second["notes"] = json!("edited by the second editor");
    let resp = save(&app, ALICE, base, &second).await;
    assert_eq!(
        resp.status(),
        StatusCode::CONFLICT,
        "the second save must be refused, not silently applied"
    );

    let page = body_string(resp).await;
    // Who, and when.
    assert!(
        page.contains("Someone else changed this"),
        "the refusal says what happened: {page}"
    );
    assert!(
        page.contains("version 1") && page.contains("version 2"),
        "both version numbers are stated: {page}"
    );
    // The field-by-field diff, with both people's values.
    assert!(page.contains("notes"), "the changed field is named");
    assert!(
        page.contains("edited by the first editor"),
        "what the other editor wrote is shown"
    );
    assert!(
        page.contains("edited by the second editor"),
        "and what this editor wrote is carried forward to re-apply"
    );
    assert!(
        page.contains("Nothing is merged automatically"),
        "and it says it did not merge them"
    );

    // Nothing was overwritten: the first editor's text is what is stored.
    let stored = body_string(get_admin(&app, &format!("/admin/person/{ALICE}/edit")).await).await;
    assert!(stored.contains("edited by the first editor"));
    assert!(!stored.contains("edited by the second editor"));
}

#[tokio::test]
async fn edits_to_different_entities_both_succeed() {
    // The lock is per entity, not per bundle. Two people working on two
    // relatives must not block each other.
    let app = app("ce-different");
    let mut a = person(ALICE, "Alice", 1);
    a["notes"] = json!("a note for Alice");
    let mut b = person(BOB, "Bob", 1);
    b["notes"] = json!("a note for Bob");

    assert_eq!(save(&app, ALICE, 1, &a).await.status(), StatusCode::OK);
    assert_eq!(save(&app, BOB, 1, &b).await.status(), StatusCode::OK);

    let page_a = body_string(get_admin(&app, &format!("/admin/person/{ALICE}/edit")).await).await;
    let page_b = body_string(get_admin(&app, &format!("/admin/person/{BOB}/edit")).await).await;
    assert!(page_a.contains("a note for Alice"));
    assert!(page_b.contains("a note for Bob"));
}

#[tokio::test]
async fn an_edit_after_reloading_the_current_version_succeeds() {
    // The way out of a conflict: reload, re-apply, save. If this did not work
    // the refusal would be a dead end rather than a detour.
    let app = app("ce-reload");

    let mut first = person(ALICE, "Alice", 1);
    first["notes"] = json!("first");
    assert_eq!(save(&app, ALICE, 1, &first).await.status(), StatusCode::OK);

    // The second editor is refused…
    let mut second = person(ALICE, "Alice", 1);
    second["notes"] = json!("second");
    assert_eq!(
        save(&app, ALICE, 1, &second).await.status(),
        StatusCode::CONFLICT
    );

    // …reloads, and saves against what is now current.
    let now = stored_version(&app, ALICE).await;
    assert_eq!(now, 2, "the first save moved it to 2");
    let resp = save(&app, ALICE, now, &second).await;
    assert_eq!(resp.status(), StatusCode::OK, "re-applying on top works");

    let page = body_string(get_admin(&app, &format!("/admin/person/{ALICE}/edit")).await).await;
    assert!(page.contains("second"));
    assert_eq!(stored_version(&app, ALICE).await, 3, "and version moves on");
}

#[tokio::test]
async fn a_successful_save_increments_the_version_exactly_once() {
    let app = app("ce-increment");
    for expected in 1..=4u64 {
        assert_eq!(stored_version(&app, ALICE).await, expected);
        let mut e = person(ALICE, "Alice", expected);
        e["notes"] = json!(format!("edit {expected}"));
        assert_eq!(
            save(&app, ALICE, expected, &e).await.status(),
            StatusCode::OK
        );
    }
    assert_eq!(stored_version(&app, ALICE).await, 5);
}

#[tokio::test]
async fn a_save_that_declares_no_version_fails_closed() {
    // A form from before this existed, or a script posting by hand. Falling
    // back to the stored version would make the check pass by default, which
    // is the one thing it must never do.
    let app = app("ce-noversion");
    let mut e = person(ALICE, "Alice", 1);
    e["notes"] = json!("posted without a version");
    let body = format!("raw_json={}", urlencode(&e.to_string()));
    let resp = post_form(&app, &format!("/admin/person/{ALICE}"), &body, true).await;
    assert_eq!(
        resp.status(),
        StatusCode::CONFLICT,
        "no declared version must refuse, never overwrite"
    );
}

#[tokio::test]
async fn the_conflict_page_marks_the_fields_both_editors_touched() {
    let app = app("ce-contested");

    let mut first = person(ALICE, "Alice", 1);
    first["notes"] = json!("theirs");
    first["identity"]["gender"]["value"] = json!("NB");
    assert_eq!(save(&app, ALICE, 1, &first).await.status(), StatusCode::OK);

    // The second editor changed `notes` too, and nothing else.
    let mut second = person(ALICE, "Alice", 1);
    second["notes"] = json!("mine");
    let page = body_string(save(&app, ALICE, 1, &second).await).await;

    assert!(
        page.contains("You both changed these"),
        "an overlapping field is called out: {page}"
    );
    assert!(page.contains("contested"), "and marked in the table");
}

#[tokio::test]
async fn every_successful_edit_lands_in_the_journal_and_the_history() {
    let app = app("ce-journal");
    let mut e = person(ALICE, "Alice", 1);
    e["notes"] = json!("a corrected note");
    assert_eq!(save(&app, ALICE, 1, &e).await.status(), StatusCode::OK);

    let page = body_string(get_admin(&app, &format!("/admin/person/{ALICE}/edit")).await).await;
    assert!(
        page.contains("History") || page.contains("history"),
        "the record shows its own history: {page}"
    );
    assert!(
        page.contains("changed notes"),
        "and names the field that changed: {page}"
    );

    let dash = body_string(get_admin(&app, "/admin").await).await;
    assert!(
        dash.contains("changed notes"),
        "the dashboard surfaces recent edits: {dash}"
    );
}

#[tokio::test]
async fn the_journal_lives_beside_the_bundle_and_not_inside_it() {
    // The bundle is copied, mailed and published. A journal naming the family's
    // editors and quoting every value they corrected must not travel with it.
    let dir = scratch("ce-outside-src");
    let path = dir.join("c.axgf");
    let flat = json!({
        "manifest": {"axgf": "1.0"},
        "persons": {ALICE: person(ALICE, "Alice", 1)},
        "families": {}, "events": {}, "links": {}, "occupations": {},
        "sources": {}, "places": {}, "documents": {}
    });
    std::fs::write(
        &path,
        axgf_cms::state::export_to_bytes(&flat.to_string()).expect("export"),
    )
    .expect("write");
    let (app, live) = app_with_bundle("ce-outside", &path);

    let mut e = person(ALICE, "Alice", 1);
    e["notes"] = json!("a private correction");
    assert_eq!(save(&app, ALICE, 1, &e).await.status(), StatusCode::OK);

    let journal = live.with_extension("journal");
    assert!(journal.exists(), "the journal is written beside the bundle");
    let text = std::fs::read_to_string(&journal).expect("read journal");
    assert!(text.contains("a private correction"));

    // And nothing of it is in the bundle itself.
    let bytes = std::fs::read(&live).expect("read bundle");
    let env = axgf_rs::import_bundle(&bytes);
    let dump = env.data.to_string();
    assert!(
        !dump.contains("\"who\""),
        "no journal entry may travel inside the .axgf"
    );

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        let mode = std::fs::metadata(&journal).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, 0o600, "it names people and quotes what they wrote");
    }
}

/// Minimal form encoding for the raw-JSON field.
fn urlencode(s: &str) -> String {
    let mut out = String::with_capacity(s.len() * 3);
    for b in s.as_bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(*b as char)
            }
            b' ' => out.push('+'),
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

#[tokio::test]
async fn the_history_is_shown_to_signed_in_readers_and_to_nobody_else() {
    // The journal names this family's editors and quotes what each of them
    // corrected. That is precisely why it is kept out of the shareable bundle,
    // so publishing it on the public record page would put it straight back
    // where it was kept out of.
    let app = app("ce-history-private");
    let mut e = person(ALICE, "Alice", 1);
    e["notes"] = json!("a correction somebody made");
    assert_eq!(save(&app, ALICE, 1, &e).await.status(), StatusCode::OK);

    let signed_in = body_string(get_admin(&app, &format!("/person/{ALICE}")).await).await;
    assert!(
        signed_in.contains("History") && signed_in.contains("changed notes"),
        "a signed-in reader sees who changed what"
    );

    let anonymous = body_string(get(&app, &format!("/person/{ALICE}")).await).await;
    assert!(
        !anonymous.contains("changed notes"),
        "a signed-out reader is not shown the editors' names or their edits"
    );
    assert!(
        !anonymous.contains("emergency-token"),
        "and certainly not who was signed in at the time"
    );
    // The record itself is still public — it is the *history* that is not.
    assert!(anonymous.contains("Alice"));
}
