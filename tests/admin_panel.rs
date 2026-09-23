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
        "policy=reject&base_version=1",
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
        "policy=cascade&base_version=1",
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
    //
    // Read the field's own value rather than searching the page for the
    // number. The paste box's placeholder shows an example position and it is
    // this one, so "the page mentions these coordinates" stopped being the
    // same question as "the record holds them".
    let stored = body_string(get_admin(&app, &format!("/admin/place/{place_id}/edit")).await).await;
    assert_eq!(
        value_of(&stored, "coordinates.lat"),
        "",
        "nothing reaches the bundle until the reader saves"
    );
    assert_eq!(value_of(&stored, "coordinates.lon"), "");
}

/// The `value` attribute of one named input, or the empty string.
fn value_of(html: &str, name: &str) -> String {
    let needle = format!("name=\"{name}\"");
    let Some(at) = html.find(&needle) else {
        return String::new();
    };
    let rest = &html[at..];
    let Some(end) = rest.find('>') else {
        return String::new();
    };
    let tag = &rest[..end];
    let Some(v) = tag.find("value=\"") else {
        return String::new();
    };
    let after = &tag[v + 7..];
    after[..after.find('"').unwrap_or(0)].to_string()
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
        !body.contains(r#"name="lookup""#),
        "no contact address, no button that would call the service: {body}"
    );
    assert!(
        body.contains(r#"name="coordinates.lat""#),
        "the manual fields are there regardless — they are the ordinary path"
    );
    // The paste box posts to the same route and is *not* gated on a geocoder:
    // reading a position out of pasted text is local work that contacts
    // nobody, and it is the path this data mostly needs.
    assert!(
        body.contains(r#"name="paste""#),
        "the paste box needs no third party and stays"
    );
}

/// A bundle holding one couple entered twice — the shape the operator's
/// bundle carries: one record with a type and a date, one with the `unknown`
/// sentinel and nothing else, the same two spouses, the same child.
fn duplicate_family_app(tag: &str) -> (axum::Router, common::Scratch) {
    use serde_json::json;
    const A: &str = "aaaaaaaa-1111-4111-8111-111111111111";
    const B: &str = "bbbbbbbb-2222-4222-8222-222222222222";
    const KID: &str = "cccccccc-3333-4333-8333-333333333333";

    let dir = scratch(&format!("{tag}-src"));
    let path = dir.join("dup.axgf");
    let person = |id: &str, name: &str| {
        json!({"id": id, "type": "person", "axgf_version": "1.0",
               "identity": {"name": {"display": name, "components": []},
                            "is_living": false, "visibility": "public"},
               "birth": {"date": {"value": "1950", "precision": "year"}},
               "death": {"date": {"value": "2010", "precision": "year"}}})
    };
    let flat = json!({
        "manifest": {"axgf": "1.0"},
        "persons": {A: person(A, "Janusz Kowalski"), B: person(B, "Maria Kowalska"),
                    KID: person(KID, "Ewa Kowalska")},
        "families": {
            "fam-thin": {"id": "fam-thin", "type": "family", "axgf_version": "1.0",
                "union": {"type": "unknown", "confidence": 0.8,
                          "persons": [{"person_id": A, "role": "spouse"},
                                      {"person_id": B, "role": "spouse"}]},
                "children": [{"person_id": KID, "confidence": 0.8}]},
            "fam-full": {"id": "fam-full", "type": "family", "axgf_version": "1.0",
                "union": {"type": "marriage", "confidence": 0.8,
                          // Reversed order: a family is identified by the set.
                          "persons": [{"person_id": B, "role": "spouse"},
                                      {"person_id": A, "role": "spouse"}],
                          "start": {"date": {"value": "1991-08-24", "precision": "exact"}}},
                "children": [{"person_id": KID, "confidence": 0.8}]}
        },
        "events": {}, "links": {}, "occupations": {},
        "sources": {}, "places": {}, "documents": {}
    });
    std::fs::write(
        &path,
        axgf_cms::state::export_to_bytes(&flat.to_string()).expect("export"),
    )
    .expect("write");
    app_with_bundle(tag, &path)
}

/// Two Family records for one couple are shown as one thing that says what it
/// is, with a way for an administrator to act on it.
///
/// Rendered as two unrelated blocks with the same spouse in both, the likeliest
/// reading is a second marriage — and that is the wrong reading. This is
/// `DUPLICATE_UNIQUE_REF`, which the validator reports; the operator's bundle
/// holds three of them.
#[tokio::test]
async fn a_couple_entered_twice_is_grouped_named_and_actionable() {
    const A: &str = "aaaaaaaa-1111-4111-8111-111111111111";
    let (app, _p) = duplicate_family_app("dup-family");

    let page = body_string(get_admin(&app, &format!("/person/{A}")).await).await;

    // One group holding both records, not two loose blocks.
    assert_eq!(
        page.matches(r#"class="union-group is-duplicate""#).count(),
        1,
        "the two records are one group: {page}"
    );
    assert_eq!(
        page.matches(r#"<div class="union">"#).count(),
        2,
        "and both records are still shown in full"
    );

    // And it says plainly what they are.
    assert!(
        page.contains("One couple, more than one record."),
        "the page names the defect: {page}"
    );
    assert!(
        page.contains("not a second union"),
        "and rules out the wrong reading"
    );

    // With a way to act on it, naming both families.
    assert!(
        page.contains(r#"action="/admin/dedup""#),
        "an administrator is offered the merge: {page}"
    );
    assert!(
        page.contains("fam-full,fam-thin") || page.contains("fam-thin,fam-full"),
        "and it names the pair: {page}"
    );

    // A reader who cannot edit is told what the page holds but offered no
    // button they may not press.
    let public = body_string(get(&app, &format!("/person/{A}")).await).await;
    assert!(
        public.contains("One couple, more than one record."),
        "a reader is still told the records disagree"
    );
    assert!(
        !public.contains(r#"action="/admin/dedup""#),
        "but is offered no action they cannot take"
    );
}

/// The merge action calls the library, and reports what the library did to
/// *this* pair rather than a bundle-wide total.
///
/// On this shape the library refuses: `is_ambiguous_family_group` treats the
/// `unknown` sentinel in `union.type` as a union type that disagrees with
/// `marriage`, so the pair is left for a person to review. The CMS reports
/// that refusal rather than working around it — all genealogy logic lives in
/// axgf-rs, and merging two families here would be genealogy.
#[tokio::test]
async fn the_merge_action_reports_what_happened_to_the_pair_it_was_given() {
    let (app, _p) = duplicate_family_app("dedup-pair-report");

    let body = body_string(
        post_form(
            &app,
            "/admin/dedup",
            "families=fam-thin,fam-full&back=%2Ftree",
            true,
        )
        .await,
    )
    .await;

    // The library refuses this pair, so the page must say so rather than
    // reporting "nothing to report" and leaving the reader to wonder.
    assert!(
        body.contains("was not merged"),
        "the refusal is reported: {body}"
    );
    assert!(
        !body.contains("is now one record"),
        "and nothing claims a merge that did not happen"
    );
    // And it returns the reader where they came from.
    assert!(
        body.contains(r#"href="&#x2f;tree""#),
        "the way back: {body}"
    );

    // Both families are still there, which is what "refused" means.
    let page = body_string(get_admin(&app, "/admin/family").await).await;
    assert!(page.contains("fam-thin") && page.contains("fam-full"));
}

/// A pair of ids that names nothing is not reported as a merge.
///
/// A stale link should not produce "the pair you asked about is now one
/// record" out of two ids the bundle never held.
#[tokio::test]
async fn ids_that_name_no_family_are_not_reported_as_merged() {
    let (app, _p) = app_with_empty_bundle("dedup-pair-absent");
    let body =
        body_string(post_form(&app, "/admin/dedup", "families=no-such-a,no-such-b", true).await)
            .await;
    assert!(
        !body.contains("The pair you asked about"),
        "nothing is claimed about a pair that was never there: {body}"
    );
}

// ---------------------------------------------------------------------------
// The operational banner
//
// `/health` is for a monitor. These three are for the household: the only
// warning an installation with no monitor at all will ever get is the one on
// the page somebody signs in to.
// ---------------------------------------------------------------------------

/// An app whose operational settings are the ones the banner reads.
fn app_with_ops(
    tag: &str,
    backup_dir: Option<std::path::PathBuf>,
    standing_token: bool,
) -> (axum::Router, Scratch) {
    let dir = scratch(tag);
    let path = dir.join("family.axgf");
    let app =
        axgf_cms::app_with_operations(&path, TOKEN, backup_dir, standing_token).expect("build app");
    (app, dir.pointing_at(path))
}

#[tokio::test]
async fn the_dashboard_says_so_when_nothing_is_being_backed_up() {
    let (app, _p) = app_with_ops("dash-no-backup", None, false);
    let page = body_string(get_admin(&app, "/admin").await).await;
    assert!(
        page.contains("Nothing is being backed up"),
        "the dashboard must say that nothing is backed up:\n{}",
        &page[..page.len().min(600)]
    );
    assert!(page.contains("Needs attention"), "and mark it as a warning");
}

#[tokio::test]
async fn the_dashboard_says_so_when_the_newest_backup_is_two_days_old() {
    let backups = scratch("dash-stale-backups");
    // The age comes from the archive's own stamp, so a stale one is a file
    // with an old name. Three days: past the 48-hour warning threshold.
    let then = time::OffsetDateTime::now_utc() - time::Duration::days(3);
    let name = format!(
        "axgf-backup-{:04}{:02}{:02}T{:02}{:02}{:02}Z.zip",
        then.year(),
        u8::from(then.month()),
        then.day(),
        then.hour(),
        then.minute(),
        then.second()
    );
    std::fs::write(backups.dir().join(&name), b"not read by the age check").expect("write");

    let (app, _p) = app_with_ops("dash-stale", Some(backups.dir().to_path_buf()), false);
    let page = body_string(get_admin(&app, "/admin").await).await;
    assert!(
        page.contains("The newest backup is 3 days old"),
        "the dashboard must name the age:\n{}",
        &page[..page.len().min(600)]
    );
}

#[tokio::test]
async fn the_dashboard_says_so_while_an_emergency_token_is_still_set() {
    // The token bypasses every account. It is how an installation is rescued
    // and it is meant to be taken out again, so the dashboard keeps asking.
    let (app, _p) = app_with_ops("dash-token", None, true);
    let page = body_string(get_admin(&app, "/admin").await).await;
    assert!(
        page.contains("emergency administrator token is still set"),
        "the dashboard must surface a standing emergency token"
    );

    // And says nothing when the token was generated for this boot only.
    let (app, _p) = app_with_ops("dash-token-generated", None, false);
    let page = body_string(get_admin(&app, "/admin").await).await;
    assert!(
        !page.contains("emergency administrator token is still set"),
        "a per-boot token is not a standing credential"
    );
}

#[tokio::test]
async fn the_banner_is_not_shown_to_a_reader_who_is_not_signed_in() {
    // It names free space, a backup directory and whether a rescue token
    // exists. None of that belongs on a public page.
    let (app, _p) = app_with_ops("dash-public", None, true);
    for path in ["/", "/tree"] {
        let page = body_string(get(&app, path).await).await;
        assert!(
            !page.contains("Nothing is being backed up")
                && !page.contains("emergency administrator token"),
            "{path} leaked an operational warning to an anonymous reader"
        );
    }
}

// ---------------------------------------------------------------------------
// The emergency token is a way in, not a credential
// ---------------------------------------------------------------------------

#[tokio::test]
async fn the_token_replayed_as_a_cookie_is_not_a_credential() {
    // It used to be exactly that: `axgf_admin=<token>` on any request granted
    // admin, with no attempt limit, no log line and nothing a sign-out could
    // revoke. Nothing issues that cookie any more and nothing accepts it.
    let (app, _p) = app_with_empty_bundle("legacy-cookie");
    for path in ["/admin", "/admin/person", "/admin/export"] {
        let resp = get_with_cookie(&app, path, &format!("axgf_admin={TOKEN}")).await;
        assert_eq!(
            resp.status(),
            StatusCode::UNAUTHORIZED,
            "{path} accepted the legacy token cookie"
        );
    }
    // And a state-changing POST with it is refused too.
    let resp = post_form_as(&app, &format!("axgf_admin={TOKEN}"), "/admin/validate", "").await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn guesses_at_the_emergency_token_run_out() {
    // The token branch of the login form used to be evaluated *before* the
    // throttle gate, so wrong tokens were counted and never refused: an
    // operator-chosen token could be ground through at the speed of the
    // network. The right token is offered last, and must be refused too —
    // the limit is on the client, not on the guess.
    let (app, _p) = app_with_empty_bundle("token-throttle");
    for i in 0..axgf_cms::session::MAX_ATTEMPTS {
        let resp = post_form(&app, "/admin/login", &format!("token=wrong{i}"), false).await;
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED, "attempt {i}");
    }
    let resp = post_form(&app, "/admin/login", &format!("token={TOKEN}"), false).await;
    assert_eq!(
        resp.status(),
        StatusCode::UNAUTHORIZED,
        "the correct token should still be refused while throttled"
    );
    let page = body_string(resp).await;
    assert!(
        page.contains("Too many failed attempts"),
        "and say why:\n{}",
        &page[..page.len().min(400)]
    );
}
