//! The admin panel, driven through the real router.

mod common;

use axum::http::StatusCode;
use common::*;
use serde_json::Value;

const ADMIN_GETS: [&str; 11] = [
    "/admin",
    "/admin/export",
    "/admin/person",
    "/admin/family",
    "/admin/event",
    "/admin/link",
    "/admin/occupation",
    "/admin/source",
    "/admin/place",
    "/admin/document",
    "/admin/person/new",
];

#[tokio::test]
async fn every_admin_page_is_401_without_the_cookie() {
    let (app, _p) = app_with_empty_bundle("admin-401");
    for path in ADMIN_GETS {
        let resp = get(&app, path).await;
        assert_eq!(
            resp.status(),
            StatusCode::UNAUTHORIZED,
            "{path} should require the admin cookie"
        );
    }
    // Mutating endpoints too.
    for path in ["/admin/validate", "/admin/dedup", "/admin/person"] {
        let resp = post_form(&app, path, "", false).await;
        assert_eq!(
            resp.status(),
            StatusCode::UNAUTHORIZED,
            "POST {path} should require the admin cookie"
        );
    }
}

#[tokio::test]
async fn every_admin_page_loads_with_the_cookie() {
    let (app, _p) = app_with_empty_bundle("admin-ok");
    for path in ADMIN_GETS {
        let resp = get_admin(&app, path).await;
        assert_eq!(
            resp.status(),
            StatusCode::OK,
            "{path} should load for an admin"
        );
    }
}

#[tokio::test]
async fn the_emergency_token_opens_a_session_and_a_wrong_one_does_not() {
    // The shared token is no longer the authentication system; it is the way
    // back in when the .acl is lost or every admin is locked out. It now buys
    // a *session* like any other sign-in rather than being replayed as a
    // credential on every request.
    let (app, _p) = app_with_empty_bundle("admin-login");

    let bad = post_form(&app, "/admin/login", "token=wrong", false).await;
    assert_eq!(bad.status(), StatusCode::UNAUTHORIZED);
    assert!(
        bad.headers().get("set-cookie").is_none(),
        "no cookie on failure"
    );

    let ok = post_form(&app, "/admin/login", &format!("token={TOKEN}"), false).await;
    assert!(ok.status().is_redirection(), "a good token redirects");
    let cookie = ok
        .headers()
        .get("set-cookie")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();
    assert!(
        cookie.contains("axgf_session="),
        "the emergency token opens a session, it is not the session: {cookie}"
    );
    assert!(cookie.contains("HttpOnly"));
    assert!(
        cookie.contains("SameSite=Strict"),
        "every mutating route here is a form POST; Lax buys nothing"
    );
    assert!(
        !cookie.contains("Secure"),
        "the documented localhost deployment is plain http, where a Secure \
         cookie is simply never stored"
    );

    // An empty token must never be accepted, and must not be treated as a
    // token attempt at all — it falls through to the username form.
    let empty = post_form(&app, "/admin/login", "token=", false).await;
    assert_eq!(empty.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn logout_clears_the_cookie() {
    let (app, _p) = app_with_empty_bundle("admin-logout");
    let resp = post_form(&app, "/admin/logout", "", true).await;
    let cookie = resp
        .headers()
        .get("set-cookie")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();
    assert!(
        cookie.contains("Max-Age=0"),
        "logout must expire the cookie"
    );
}

#[tokio::test]
async fn create_then_read_round_trips_through_the_bundle() {
    let (app, path) = app_with_empty_bundle("admin-create");

    let body = "identity.name.display=Ada+Lovelace\
                &identity.gender.value=F\
                &birth.date.value=1815-12-10\
                &birth.date.precision=exact\
                &birth.confidence=0.95\
                &raw_json=";
    let resp = post_form(&app, "/admin/person", body, true).await;
    let out = expect_status(resp, StatusCode::OK, "create person").await;
    assert!(
        out.contains("Created"),
        "creation should be reported: {out}"
    );

    // It is in the listing.
    let list = body_string(get_admin(&app, "/admin/person").await).await;
    assert!(list.contains("Ada Lovelace"));

    // It survived to disk: a fresh app over the same file finds it.
    let reopened = axgf_cms::app(&path, TOKEN).expect("reopen");
    let health = body_string(get(&reopened, "/health").await).await;
    let v: serde_json::Value = serde_json::from_str(&health).unwrap();
    assert_eq!(v["entities"]["persons"], 1, "the person must be on disk");

    // And the public page renders what was entered.
    let listed = body_string(get_admin(&reopened, "/admin/person").await).await;
    let id = first_person_id(&listed);
    let page = body_string(get(&reopened, &format!("/person/{id}")).await).await;
    assert!(page.contains("Ada Lovelace"));
    assert!(
        page.contains("10 December 1815"),
        "the date should render in prose"
    );
}

#[tokio::test]
async fn delete_under_reject_leaves_the_bundle_byte_identical() {
    let (app, path) = app_with_empty_bundle("admin-reject");

    // A person who is referenced by a family cannot be deleted under Reject.
    post_form(
        &app,
        "/admin/person",
        "identity.name.display=Referenced+Person&raw_json=",
        true,
    )
    .await;

    let listed = body_string(get_admin(&app, "/admin/person").await).await;
    let id = first_person_id(&listed);

    let fam = format!(
        "raw_json={}",
        urlencode(&format!(
            r#"{{"union":{{"type":"marriage","persons":[{{"person_id":"{id}","role":"spouse"}}]}}}}"#
        ))
    );
    let r = post_form(&app, "/admin/family", &fam, true).await;
    assert_eq!(r.status(), StatusCode::OK);

    let before = std::fs::read(&path).expect("read before");
    let resp = post_form(
        &app,
        &format!("/admin/person/{id}/delete"),
        "policy=reject",
        true,
    )
    .await;
    let out = expect_status(resp, StatusCode::OK, "rejected delete").await;

    assert!(
        out.contains("DELETE_BLOCKED_BY_REFERENCE"),
        "the blocking diagnostic must be shown: {out}"
    );
    assert!(
        out.contains("unchanged"),
        "the page should say nothing changed"
    );

    let after = std::fs::read(&path).expect("read after");
    assert_eq!(
        before, after,
        "a refused delete must leave the bundle byte-identical"
    );
}

#[tokio::test]
async fn delete_under_cascade_succeeds_where_reject_refused() {
    let (app, _path) = app_with_empty_bundle("admin-cascade");
    post_form(
        &app,
        "/admin/person",
        "identity.name.display=Doomed&raw_json=",
        true,
    )
    .await;
    let listed = body_string(get_admin(&app, "/admin/person").await).await;
    let id = first_person_id(&listed);

    let fam = format!(
        "raw_json={}",
        urlencode(&format!(
            r#"{{"union":{{"type":"marriage","persons":[{{"person_id":"{id}","role":"spouse"}}]}}}}"#
        ))
    );
    post_form(&app, "/admin/family", &fam, true).await;

    let resp = post_form(
        &app,
        &format!("/admin/person/{id}/delete"),
        "policy=cascade",
        true,
    )
    .await;
    let out = expect_status(resp, StatusCode::OK, "cascade delete").await;
    assert!(out.contains("Deleted"), "cascade should succeed: {out}");

    let health = body_string(get(&app, "/health").await).await;
    let v: serde_json::Value = serde_json::from_str(&health).unwrap();
    assert_eq!(v["entities"]["persons"], 0);
}

#[tokio::test]
async fn malformed_raw_json_is_refused_without_touching_the_bundle() {
    let (app, path) = app_with_empty_bundle("admin-badjson");
    let before = std::fs::read(&path).expect("read before");

    let resp = post_form(
        &app,
        "/admin/person",
        "identity.name.display=X&raw_json=%7Bnot+json",
        true,
    )
    .await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    let out = body_string(resp).await;
    assert!(out.contains("did not parse"), "say what was wrong: {out}");
    assert!(out.contains("Nothing was saved"));

    assert_eq!(
        std::fs::read(&path).expect("read after"),
        before,
        "a form error must not write the bundle"
    );
}

#[tokio::test]
async fn schema_warnings_are_surfaced_rather_than_swallowed() {
    // The library's validation is deliberately non-blocking: a source missing
    // its required fields is still added, with warnings. The admin panel's job
    // is to show those warnings, not to hide them behind a success message.
    let (app, _path) = app_with_empty_bundle("admin-warn");

    let resp = post_form(&app, "/admin/source", "title=&reliability=&raw_json=", true).await;
    let out = expect_status(resp, StatusCode::OK, "incomplete source").await;

    assert!(
        out.contains("SCHEMA_VALIDATION_FAILED"),
        "the schema warnings must be visible: {out}"
    );
    assert!(
        out.contains("is a required property"),
        "the warning text must say what is missing"
    );
    assert!(
        out.contains("Diagnostics"),
        "warnings belong under a heading, not buried"
    );
}

#[tokio::test]
async fn a_person_created_through_the_form_validates_cleanly() {
    // Creating through the admin panel must not immediately dirty the bundle
    // with schema warnings the user cannot see the cause of.
    let (app, _path) = app_with_empty_bundle("admin-clean");

    let resp = post_form(
        &app,
        "/admin/person",
        "identity.name.display=Clean+Person&raw_json=",
        true,
    )
    .await;
    let out = expect_status(resp, StatusCode::OK, "create person").await;
    assert!(out.contains("Created"));
    assert!(
        !out.contains("SCHEMA_VALIDATION_FAILED"),
        "a form-created person should already satisfy the schema: {out}"
    );

    // And a full validate agrees.
    let report = body_string(post_form(&app, "/admin/validate", "", true).await).await;
    assert!(
        !report.contains("SCHEMA_VALIDATION_FAILED"),
        "validate should be clean after a form create: {report}"
    );
}

#[tokio::test]
async fn validate_and_dedup_report_without_swallowing_diagnostics() {
    let (app, _p) = app_with_empty_bundle("admin-ops");

    let v = expect_status(
        post_form(&app, "/admin/validate", "", true).await,
        StatusCode::OK,
        "validate",
    )
    .await;
    assert!(v.contains("Validation report"));

    let d = expect_status(
        post_form(&app, "/admin/dedup", "", true).await,
        StatusCode::OK,
        "dedup",
    )
    .await;
    assert!(d.contains("Deduplication"));
    assert!(d.contains("merged") || d.contains("Nothing to report"));
}

#[tokio::test]
async fn export_returns_the_live_bundle() {
    let (app, path) = app_with_empty_bundle("admin-export");
    let resp = get_admin(&app, "/admin/export").await;
    assert_eq!(resp.status(), StatusCode::OK);
    let ct = resp
        .headers()
        .get(axum::http::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();
    assert_eq!(ct, "application/vnd.axgf+zip");

    let bytes = body_bytes(resp).await;
    assert_eq!(&bytes[..2], b"PK");
    assert!(!bytes.is_empty());
    assert!(path.exists());
}

#[tokio::test]
async fn an_unknown_entity_kind_is_a_clean_404() {
    let (app, _p) = app_with_empty_bundle("admin-kind");
    let body = expect_status(
        get_admin(&app, "/admin/wombat").await,
        StatusCode::NOT_FOUND,
        "unknown kind",
    )
    .await;
    // The refusal names what was asked for and what is actually on offer —
    // both come from the locale catalogue now, so this also pins that the
    // error page's arguments reach it.
    assert!(body.contains("Unknown kind"), "{body}");
    assert!(body.contains("wombat"), "it echoes what was asked for");
    assert!(body.contains("person"), "and lists the kinds that do exist");
}

#[tokio::test]
async fn the_delete_form_offers_all_three_policies_with_reject_default() {
    let (app, _p) = app_with_empty_bundle("admin-policies");
    post_form(
        &app,
        "/admin/person",
        "identity.name.display=Someone&raw_json=",
        true,
    )
    .await;
    let list = body_string(get_admin(&app, "/admin/person").await).await;

    assert!(
        list.contains(r#"value="reject" checked"#),
        "Reject is the default"
    );
    assert!(list.contains(r#"value="cascade""#));
    assert!(list.contains(r#"value="orphan""#));
    // Each is explained in a line.
    assert!(list.contains("refuse if anything still references it"));
    assert!(list.contains("physically remove every reference"));
    assert!(list.contains("with the link nulled"));
}

#[tokio::test]
async fn the_dashboard_shows_a_bundle_completeness_readout() {
    let src = std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/deploy/sample.axgf"));
    let (app, _p) = app_with_bundle("admin-complete", src);

    let body = expect_status(get_admin(&app, "/admin").await, StatusCode::OK, "dashboard").await;

    // Framed as room to grow rather than as a fault — by the heading, which is
    // where that framing belongs. The sentence under it that used to spell the
    // same thing out ("Nothing here is an error: a blank row is somewhere the
    // record could grow…") was the table explaining the table, and it went
    // with the rest of the instructional copy.
    assert!(body.contains("Where this tree could say more"));
    assert!(
        !body.contains("Nothing here is an error"),
        "the readout states what is blank; it does not reassure the reader about it"
    );
    for expected in [
        "How sure each fact is",
        "Relationships beyond blood and marriage",
        "Work recorded with a start and an end",
        "Sources graded for how reliable they are",
        "Dates, by the shape they actually have",
    ] {
        assert!(
            body.contains(expected),
            "dashboard panel missing: {expected}"
        );
    }
    // The sample populates everything, so the dashboard must not claim gaps.
    assert!(
        body.contains("recorded somewhere in this tree"),
        "a complete bundle should be reported as complete"
    );
}

#[tokio::test]
async fn the_dashboard_readout_reflects_an_empty_bundle_honestly() {
    let (app, _p) = app_with_empty_bundle("admin-complete-empty");
    let body = body_string(get_admin(&app, "/admin").await).await;
    assert!(body.contains("Where this tree could say more"));
    assert!(
        body.contains("No dates recorded yet"),
        "an empty bundle reports zero dates rather than an empty chart"
    );
}

/// Pull the first person id out of an admin listing.
///
/// Matching on `href="/person/` specifically: a bare `/person/` also occurs
/// inside `/admin/person/new`, which would yield "new".
fn first_person_id(html: &str) -> String {
    html.split("href=\"/person/")
        .nth(1)
        .and_then(|s| s.split('"').next())
        .expect("a person link in the listing")
        .to_string()
}

/// Minimal percent-encoding for form bodies in tests.
fn urlencode(s: &str) -> String {
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

/// A listing row must tell itself apart from its neighbours.
///
/// The bundle here is shaped after what a GEDCOM conversion actually produces:
/// families with no `name` of their own, carrying only a union and a children
/// list. Before this, all four rows read "(unnamed family, N children)" and
/// the id column was a UUID, so the page was a count rather than a list.
#[tokio::test]
async fn a_family_is_labelled_by_the_people_in_it() {
    use serde_json::json;

    let dir = scratch("labels-src");
    let path = dir.join("l.axgf");
    let p = |id: &str, name: &str| {
        json!({
            "id": id, "type": "person", "axgf_version": "1.0",
            "identity": {
                "name": {"display": name, "components": [
                    {"type": "given_name", "value": name, "order": 1}]},
                "gender": {"value": "F"}, "is_living": false,
                "visibility": "public"}
        })
    };
    let ids: Vec<String> = (1..=8)
        .map(|n| format!("{n}{n}111111-1111-4111-8111-111111111111"))
        .collect();
    let names = [
        "Leonard Kasprzyk",
        "Janina Kasprzyk",
        "Marek Kasprzyk",
        "Zofia Kasprzyk",
        "Halina Nowak",
        "Piotr Nowak",
        "Ewa Nowak",
        "Adam Nowak",
    ];
    let mut persons = serde_json::Map::new();
    for (id, name) in ids.iter().zip(names) {
        persons.insert(id.clone(), p(id, name));
    }

    let fam = |n: u8, partners: &[&String], kids: &[&String]| {
        json!({
            "id": format!("f{n}111111-1111-4111-8111-111111111111"),
            "type": "family", "axgf_version": "1.0",
            "union": {"type": "marriage",
                      "persons": partners.iter().map(|id| json!({"person_id": id}))
                                 .collect::<Vec<_>>()},
            "children": kids.iter().enumerate()
                          .map(|(i, id)| json!({"person_id": id, "birth_order": i + 1}))
                          .collect::<Vec<_>>()
        })
    };
    let flat = json!({
        "manifest": {"axgf": "1.0"},
        "persons": Value::Object(persons),
        "families": {
            // both partners, two children
            "f1111111-1111-4111-8111-111111111111":
                fam(1, &[&ids[0], &ids[1]], &[&ids[2], &ids[3]]),
            // both partners, exactly one child — "1 children" was the bug
            "f2111111-1111-4111-8111-111111111111":
                fam(2, &[&ids[4], &ids[5]], &[&ids[6]]),
            // one partner only
            "f3111111-1111-4111-8111-111111111111":
                fam(3, &[&ids[7]], &[]),
            // no partners: children alone
            "f4111111-1111-4111-8111-111111111111":
                fam(4, &[], &[&ids[2], &ids[3], &ids[6]])
        },
        "events": {}, "links": {}, "occupations": {},
        "sources": {}, "places": {}, "documents": {}
    });
    std::fs::write(
        &path,
        axgf_cms::state::export_to_bytes(&flat.to_string()).expect("export"),
    )
    .expect("write");
    let (app, _p) = app_with_bundle("labels", &path);

    let body = body_string(get_admin(&app, "/admin/family").await).await;

    assert!(
        body.contains("Leonard Kasprzyk &amp; Janina Kasprzyk — 2 children"),
        "both partners are named: {body}"
    );
    assert!(
        body.contains("Halina Nowak &amp; Piotr Nowak — one child"),
        "and the plural comes from the catalogue, not from an `s`: {body}"
    );
    assert!(
        !body.contains("1 children"),
        "\"1 children\" is the bug this fixes"
    );
    assert!(
        body.contains("Adam Nowak &amp; [Unknown]"),
        "a missing partner is stated, not omitted — being married to somebody \
         unrecorded is itself a fact: {body}"
    );
    assert!(
        body.contains("Marek Kasprzyk and 2 siblings"),
        "a family with no recorded parents is named by its eldest child, \
         because \"children of [unknown]\" would read the same on every such \
         row: {body}"
    );
    assert!(
        !body.contains("unnamed family"),
        "no row falls back to the placeholder when members are known"
    );
}

/// A scoped contributor may not edit a place, and that is the answer rather
/// than an oversight.
///
/// A scope confines a contributor to one branch of the family. It is expressed
/// in people, and a place names none — 123 of them serve 866 people on the
/// operator's file, so an edit here changes what every branch reads. A blast
/// radius that wide cannot be confined by a scope, so the write is refused;
/// administrators and unscoped contributors make it.
#[tokio::test]
async fn a_scoped_contributor_cannot_edit_a_shared_place() {
    let src = std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/deploy/sample.axgf"));
    let (app, _p) = app_with_bundle("place-scope", src);

    let place_id = {
        let body = body_string(get_admin(&app, "/admin/place").await).await;
        // The row's edit link, not the "place" entry in the kind navigation.
        let marker = "/admin/place/";
        let at = body
            .match_indices(marker)
            .map(|(i, _)| i + marker.len())
            .find(|i| {
                let rest: String = body[*i..].chars().take(48).collect();
                rest.contains("/edit")
            })
            .expect("a place row with an edit link");
        body[at..]
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '-')
            .collect::<String>()
    };

    // An administrator reaches the editor.
    let resp = get_admin(&app, &format!("/admin/place/{place_id}/edit")).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let form = body_string(resp).await;
    assert!(
        form.contains("place-form"),
        "the structured editor, not the generic one"
    );
    assert!(
        form.contains("country_history") || form.contains("history.0.country"),
        "with the border history the generic form could not express"
    );

    // A signed-out reader does not.
    let resp = post_form(
        &app,
        &format!("/admin/place/{place_id}"),
        "names.0.value=X&base_version=1",
        false,
    )
    .await;
    assert_ne!(resp.status(), StatusCode::OK, "writing needs an account");
}

/// Taking a suggestion fills the coordinate fields and saves nothing.
///
/// The lookup and the save are deliberately different routes. A reader who
/// searches in the middle of an edit must not lose the edit, and a coordinate
/// the geocoder proposed must not reach the bundle until a person has looked
/// at it — the service is confidently wrong often enough on this bundle's
/// place names that "found" and "correct" are different claims.
///
/// No geocoder is configured here, and none is needed: the pick path never
/// makes a request. That is the point of it being a separate branch.
#[tokio::test]
async fn taking_a_suggestion_fills_the_fields_without_saving() {
    let src = std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/deploy/sample.axgf"));
    let (app, _p) = app_with_bundle("place-geocode-pick", src);

    let place_id = {
        let body = body_string(get_admin(&app, "/admin/place").await).await;
        // The row's edit link, not the "place" entry in the kind navigation.
        let marker = "/admin/place/";
        let at = body
            .match_indices(marker)
            .map(|(i, _)| i + marker.len())
            .find(|i| {
                let rest: String = body[*i..].chars().take(48).collect();
                rest.contains("/edit")
            })
            .expect("a place row with an edit link");
        body[at..]
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '-')
            .collect::<String>()
    };

    let form = "names.0.value=Karczew&names.0.lang=pl&names.0.primary=0\
                &base_version=1&note=half-typed&pick=52.0782795%7C21.2508068%7Ccity_center";
    let resp = post_form(
        &app,
        &format!("/admin/place/{place_id}/geocode"),
        form,
        true,
    )
    .await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_string(resp).await;

    assert!(
        body.contains("52.0782795") && body.contains("21.2508068"),
        "the picked position is in the form: {body}"
    );
    assert!(
        body.contains(r#"<option value="city_center" selected>"#),
        "and so is the precision the result honestly supports"
    );
    assert!(
        body.contains("half-typed"),
        "an unsaved edit elsewhere on the form survives the round trip"
    );

    // The bundle is untouched: this was a form round trip, not a write.
    let stored = body_string(get_admin(&app, &format!("/admin/place/{place_id}/edit")).await).await;
    assert!(
        !stored.contains("52.0782795"),
        "nothing reaches the bundle until the reader saves"
    );
}

/// With no contact address there is no lookup button, and the editor is whole.
///
/// Nominatim's policy asks for a User-Agent naming the application and how to
/// reach whoever runs it. An installation that will not say does not make
/// automated calls from here — and loses nothing but a button, because the
/// coordinates are typed by hand in the ordinary case anyway.
#[tokio::test]
async fn without_a_contact_address_there_is_no_lookup_button() {
    let src = std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/deploy/sample.axgf"));
    let (app, _p) = app_with_bundle("place-geocode-off", src);

    let place_id = {
        let body = body_string(get_admin(&app, "/admin/place").await).await;
        // The row's edit link, not the "place" entry in the kind navigation.
        let marker = "/admin/place/";
        let at = body
            .match_indices(marker)
            .map(|(i, _)| i + marker.len())
            .find(|i| {
                let rest: String = body[*i..].chars().take(48).collect();
                rest.contains("/edit")
            })
            .expect("a place row with an edit link");
        body[at..]
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '-')
            .collect::<String>()
    };

    let body = body_string(get_admin(&app, &format!("/admin/place/{place_id}/edit")).await).await;
    assert!(
        !body.contains("/geocode"),
        "no geocoder, no button that would call one: {body}"
    );
    assert!(
        body.contains(r#"name="coordinates.lat""#),
        "the manual fields are there regardless — they are the ordinary path"
    );
}
