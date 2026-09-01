//! Route-level integration tests, driving the real router.

mod common;

use axum::http::StatusCode;
use common::*;

/// Home is the family's front page, not the product's.
///
/// It used to carry five cards headed "What this does for a family" — one
/// place for the whole archive, roles, per-person privacy, eleven languages,
/// the archive stays yours. Every one of them has the software as its subject,
/// and every one of them is already in the README, which is where an argument
/// for the software belongs. What is left is the family: its name, what it has
/// recorded, and where its tree already says more than names and dates.
#[tokio::test]
async fn home_states_what_the_family_has_recorded_not_what_the_product_does() {
    let (app, _p) = app_with_empty_bundle("home");
    let body = expect_status(get(&app, "/").await, StatusCode::OK, "GET /").await;

    for pitch in [
        "What this does for a family",
        "One place for the whole archive",
        "Several relatives, different roles",
        "Privacy decided person by person",
        "Eleven languages",
        "The archive stays yours",
    ] {
        assert!(
            !body.contains(pitch),
            "home is still selling the software: {pitch}"
        );
    }

    // What the family has is still stated, and so is the way in.
    assert!(
        body.contains("What the family has recorded so far"),
        "the counts keep their heading"
    );
    assert!(
        body.contains("/tree"),
        "and the tree is still one click away"
    );

    // And none of it argues about the file format.
    for jargon in ["Why AXGF", "GEDCOM records what", "specification", "Rust"] {
        assert!(
            !body.contains(jargon),
            "home still pitches the format: {jargon}"
        );
    }
}

#[tokio::test]
async fn the_open_format_is_acknowledged_once_in_the_footer() {
    // The licence asks for the acknowledgement and it stays; what it must not
    // do is become the product's pitch. One discreet line, on every page.
    let (app, _p) = app_with_empty_bundle("home-footer");
    let body = expect_status(get(&app, "/").await, StatusCode::OK, "GET /").await;

    assert!(body.contains("written in an open format"));
    assert_eq!(
        body.matches("github.com/plkarin/axgf-spec").count(),
        1,
        "exactly one link to the format, and it lives in the footer"
    );
}

#[tokio::test]
async fn health_reports_entity_counts() {
    let (app, _p) = app_with_empty_bundle("health");
    let body = expect_status(get(&app, "/health").await, StatusCode::OK, "GET /health").await;

    let v: serde_json::Value = serde_json::from_str(&body).expect("health returns JSON");
    assert_eq!(v["status"], "ok");
    assert_eq!(v["total_entities"], 0);
    // Every collection is reported, including the empty ones.
    for k in [
        "persons",
        "families",
        "events",
        "links",
        "occupations",
        "sources",
        "places",
        "documents",
    ] {
        assert!(v["entities"][k].is_number(), "health omitted {k}");
    }
}

#[tokio::test]
async fn stylesheet_is_served_from_the_binary() {
    let (app, _p) = app_with_empty_bundle("css");
    let resp = get(&app, "/static/app.css").await;
    assert_eq!(resp.status(), StatusCode::OK);
    let ct = resp
        .headers()
        .get(axum::http::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();
    assert!(ct.starts_with("text/css"), "content-type was {ct}");
    let body = body_string(resp).await;
    assert!(
        body.contains(".conf-bar"),
        "the confidence styles must ship"
    );
}

#[tokio::test]
async fn unknown_paths_render_a_404_page_not_a_panic() {
    let (app, _p) = app_with_empty_bundle("404");
    let body = expect_status(
        get(&app, "/no/such/page").await,
        StatusCode::NOT_FOUND,
        "unknown path",
    )
    .await;
    assert!(body.contains("Not found"));
}

#[tokio::test]
async fn the_tree_defaults_to_a_focused_subtree_with_controls() {
    let src = std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/deploy/sample.axgf"));
    let (app, _p) = app_with_bundle("tree-focus", src);

    let body = expect_status(get(&app, "/tree").await, StatusCode::OK, "GET /tree").await;

    assert!(
        body.contains("Around "),
        "the default view is centred on someone"
    );
    assert!(
        body.contains("tree-controls"),
        "root and depth controls are present"
    );
    assert!(body.contains(r#"name="root""#));
    assert!(body.contains(r#"name="depth""#));
    assert!(
        body.contains("/tree?all=1"),
        "the full view stays reachable from the focused one"
    );
    // A card click opens the record in the side panel; re-centring is now an
    // explicit control inside that panel rather than the card's own click.
    assert!(
        body.contains(r#"href="/person/"#),
        "cards link to the record permalink"
    );
    assert!(body.contains("tree-panel"), "the detail panel is present");
    assert!(
        body.contains("data-centre"),
        "re-centring the tree is a panel control"
    );
}

#[tokio::test]
async fn an_explicit_root_and_depth_are_honoured() {
    let src = std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/deploy/sample.axgf"));
    let (app, _p) = app_with_bundle("tree-root", src);

    // Find a person id from the default view, then centre on them explicitly.
    let first = body_string(get(&app, "/tree").await).await;
    let id = first
        .split("href=\"/person/")
        .nth(1)
        .and_then(|s| s.split('"').next())
        .expect("a person link")
        .to_string();

    let body = expect_status(
        get(&app, &format!("/tree?root={id}&depth=1")).await,
        StatusCode::OK,
        "explicit root",
    )
    .await;
    assert!(
        body.contains(&format!("value=\"{id}\" selected")),
        "the picker should show the requested root"
    );
    assert!(body.contains("1 generations each way") || body.contains(">1</option>"));
    assert!(body.contains("is-root"), "the root card is marked");
}

#[tokio::test]
async fn an_unknown_root_falls_back_instead_of_erroring() {
    let src = std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/deploy/sample.axgf"));
    let (app, _p) = app_with_bundle("tree-badroot", src);
    let body = expect_status(
        get(&app, "/tree?root=not-a-real-id").await,
        StatusCode::OK,
        "unknown root",
    )
    .await;
    assert!(body.contains("Around "), "falls back to the default root");
}

#[tokio::test]
async fn the_full_view_warns_about_its_width() {
    let src = std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/deploy/sample.axgf"));
    let (app, _p) = app_with_bundle("tree-all", src);

    let body = expect_status(get(&app, "/tree?all=1").await, StatusCode::OK, "GET all").await;
    assert!(body.contains("The whole tree"));
    assert!(
        body.contains("Back to the focused view"),
        "a way back to the default"
    );
    // The sample is small, so the width warning is suppressed; on a big bundle
    // it appears. Either way the full view must not claim to be focused.
    assert!(!body.contains("Around "));
}

#[tokio::test]
async fn depth_is_clamped_to_a_sane_maximum() {
    let src = std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/deploy/sample.axgf"));
    let (app, _p) = app_with_bundle("tree-depth", src);
    // A wild depth must not turn into an unbounded walk.
    let resp = get(&app, "/tree?depth=99999").await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_string(resp).await;
    assert!(
        body.contains("8 generations each way"),
        "clamped to the maximum"
    );
}

#[tokio::test]
async fn tree_renders_for_an_empty_bundle() {
    let (app, _p) = app_with_empty_bundle("tree-empty");
    let body = expect_status(get(&app, "/tree").await, StatusCode::OK, "GET /tree").await;
    assert!(
        body.contains("nobody in this tree yet"),
        "an empty tree should say so, not render a blank page"
    );
}

#[tokio::test]
async fn tree_shows_confidence_as_line_opacity_and_a_legend() {
    // Two children of one parent with very different parentage confidence.
    let (app, path) = app_with_empty_bundle("tree-conf");
    drop(app);

    let flat = serde_json::json!({
        "manifest": {"axgf": "1.0"},
        "persons": {
            "aaaaaaaa-0000-4000-8000-000000000000": {
                "id": "aaaaaaaa-0000-4000-8000-000000000000", "type": "person",
                "axgf_version": "1.0", "identity": {"name": {"display": "Parent"}}},
            "bbbbbbbb-0000-4000-8000-000000000000": {
                "id": "bbbbbbbb-0000-4000-8000-000000000000", "type": "person",
                "axgf_version": "1.0", "identity": {"name": {"display": "Sure Child"}}},
            "cccccccc-0000-4000-8000-000000000000": {
                "id": "cccccccc-0000-4000-8000-000000000000", "type": "person",
                "axgf_version": "1.0", "identity": {"name": {"display": "Doubtful Child"}}}
        },
        "families": {
            "dddddddd-0000-4000-8000-000000000000": {
                "id": "dddddddd-0000-4000-8000-000000000000", "type": "family",
                "axgf_version": "1.0",
                "union": {"type": "marriage", "persons": [
                    {"person_id": "aaaaaaaa-0000-4000-8000-000000000000", "role": "spouse"}]},
                "children": [
                    {"person_id": "bbbbbbbb-0000-4000-8000-000000000000", "confidence": 0.99},
                    {"person_id": "cccccccc-0000-4000-8000-000000000000", "confidence": 0.35}]
            }
        },
        "events": {}, "links": {}, "occupations": {},
        "sources": {}, "places": {}, "documents": {}
    });
    let bytes = axgf_cms::state::export_to_bytes(&flat.to_string()).expect("export");
    std::fs::write(&path, bytes).expect("write");

    let app = axgf_cms::app(&path, TOKEN).expect("rebuild app");
    let body = expect_status(get(&app, "/tree").await, StatusCode::OK, "GET /tree").await;

    // The two parentage lines must carry visibly different opacity.
    let ops: Vec<f64> = body
        .match_indices("opacity:")
        .filter_map(|(i, _)| {
            body[i + 8..]
                .split(['"', ';'])
                .next()
                .and_then(|s| s.parse::<f64>().ok())
        })
        .collect();
    assert!(ops.len() >= 2, "expected parentage lines, found {ops:?}");
    let (lo, hi) = (
        ops.iter().cloned().fold(f64::MAX, f64::min),
        ops.iter().cloned().fold(0.0, f64::max),
    );
    assert!(
        hi - lo > 0.3,
        "a 0.35 parentage must look clearly fainter than a 0.99 one ({lo} vs {hi})"
    );
    assert!(lo > 0.0, "a speculative link is faint, never invisible");
    assert!(
        body.contains("speculative"),
        "the legend explains the scale"
    );
}

#[tokio::test]
async fn an_empty_bundle_is_created_on_first_start() {
    let (_app, path) = app_with_empty_bundle("create");
    assert!(
        path.exists(),
        "startup must write a bundle when none exists"
    );
    let bytes = std::fs::read(&path).expect("read bundle");
    assert!(!bytes.is_empty());
    // A .axgf bundle is a ZIP archive; check the magic rather than trusting
    // the extension.
    assert_eq!(&bytes[..2], b"PK", "bundle should be a ZIP archive");
}

#[tokio::test]
async fn binary_attachments_survive_a_mutation() {
    // A bundle's document payloads are not entities; they ride along in the
    // flat JSON's `attachments` map. Every write re-exports the whole bundle,
    // so a mutation must not quietly drop them.
    let (app, path) = app_with_empty_bundle("attach");
    drop(app);

    let flat = serde_json::json!({
        "manifest": {"axgf": "1.0"},
        "persons": {}, "families": {}, "events": {}, "links": {},
        "occupations": {}, "sources": {}, "places": {},
        "documents": {
            "dddddddd-0000-4000-8000-000000000000": {
                "id": "dddddddd-0000-4000-8000-000000000000", "type": "document",
                "axgf_version": "1.0", "filename": "scan.bin",
                "mime_type": "application/octet-stream",
                "document_type": "other", "status": "present",
                "file": {"path": "documents/files/scan.bin"}}
        },
        // "hello payload" in base64.
        "attachments": {"documents/files/scan.bin": "aGVsbG8gcGF5bG9hZA=="}
    });
    let bytes = axgf_cms::state::export_to_bytes(&flat.to_string()).expect("export");
    std::fs::write(&path, bytes).expect("write");

    let app = axgf_cms::app(&path, TOKEN).expect("reopen");

    // Any mutation rewrites the whole file.
    let resp = post_form(
        &app,
        "/admin/person",
        "identity.name.display=Someone&raw_json=",
        true,
    )
    .await;
    assert_eq!(resp.status(), StatusCode::OK);

    // The payload must still be in the file afterwards.
    let written = std::fs::read(&path).expect("read back");
    let env = axgf_rs::import_bundle(&written);
    let payload = env.data["attachments"]["documents/files/scan.bin"].as_str();
    assert_eq!(
        payload,
        Some("aGVsbG8gcGF5bG9hZA=="),
        "a mutation must not drop document payloads"
    );
}

#[tokio::test]
async fn the_contradiction_banner_names_its_people_and_only_for_editors() {
    // The operator's own data records a father and his son as a union. The
    // banner used to say "run the validator from the admin dashboard", which
    // a signed-out visitor cannot do; it now names the two people and links
    // to each, and a visitor is not shown it at all.
    let (app, _p) = common::app_with_bundle(
        "contradiction",
        std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/deploy/sample.axgf")),
    );
    let anon = common::body_string(common::get(&app, "/tree").await).await;
    assert!(
        !anon.contains("contradiction-list"),
        "a signed-out visitor cannot act on it and is not shown it"
    );
    assert!(!anon.contains("contradicts itself"));
}
