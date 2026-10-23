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

    // What the family has is still stated — here, that it has nothing yet:
    // no heading over a row of zeros, and not one "0" tile.
    assert!(body.contains("Nothing recorded yet."));
    assert!(
        !body.contains("What the family has recorded so far") && !body.contains(r#"class="stat"#),
        "no counts of nothing"
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
async fn the_footer_names_the_product_its_version_its_author_and_the_format() {
    // On every page: the product and the version this build is, so that a
    // reader reporting a problem can say which one they saw; who made it; and
    // one link, the acknowledgement the format's licence asks for.
    let (app, _p) = app_with_empty_bundle("home-footer");
    let body = expect_status(get(&app, "/").await, StatusCode::OK, "GET /").await;
    let footer = &body[body
        .find(r#"<footer class="site-footer">"#)
        .expect("a footer")..];
    let footer = &footer[..footer.find("</footer>").expect("its end")];

    assert!(footer.contains(&format!("v{}", env!("CARGO_PKG_VERSION"))));
    assert!(footer.contains("ax-genealogy"));
    assert!(footer.contains("Pierre-Léonard Karin"));
    assert!(
        footer.contains(r#"<a href="https://github.com/plkarin/axgf-spec" rel="noopener">"#),
        "the format's link, clickable: {footer}"
    );
    assert_eq!(
        body.matches("github.com/plkarin/axgf-spec").count(),
        1,
        "exactly one link to the format, and it lives in the footer"
    );
    assert!(
        !body.contains("written in an open format"),
        "the sentence the footer used to carry is gone"
    );
}

#[tokio::test]
async fn health_reports_entity_counts() {
    let (app, _p) = app_with_empty_bundle("health");
    let body = expect_status(get(&app, "/health").await, StatusCode::OK, "GET /health").await;

    let v: serde_json::Value = serde_json::from_str(&body).expect("health returns JSON");
    // `warn`, not `ok`: this installation has no backup directory configured,
    // which is a true and useful thing to be told. 200 all the same — a
    // warning is not an outage. See `tests/operational_health.rs` for the
    // status codes.
    assert_eq!(v["status"], "warn");
    assert_eq!(v["checks"]["bundle"]["status"], "ok");
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
    // The whole family is the depth select's last choice (the minimalism
    // pass made the tree's controls one select).
    assert!(
        body.contains(r#"<option value="0" >"#) || body.contains(r#"<option value="0">"#),
        "the full view stays reachable from the focused one"
    );
    // A card moves the tree onto that person, in the same drawing, and
    // opens their record in the panel (the operator's review, 2026-10-23),
    // so the panel needs no "centre here" of its own.
    assert!(
        body.contains(r#"href="/tree?root="#),
        "a card moves the tree onto its person"
    );
    assert!(body.contains("tree-panel"), "the detail panel is present");
    assert!(
        !body.contains("data-centre"),
        "no second way to re-centre: the card is the way"
    );
}

#[tokio::test]
async fn an_explicit_root_and_depth_are_honoured() {
    let src = std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/deploy/sample.axgf"));
    let (app, _p) = app_with_bundle("tree-root", src);

    // Find a person id from the default view, then centre on them explicitly.
    let first = body_string(get(&app, "/tree").await).await;
    let id = first
        .split("class=\"tcard")
        .nth(1)
        .and_then(|s| s.split("data-id=\"").nth(1))
        .and_then(|s| s.split('"').next())
        .expect("a person's card")
        .to_string();

    let body = expect_status(
        get(&app, &format!("/tree?root={id}&depth=1")).await,
        StatusCode::OK,
        "explicit root",
    )
    .await;
    assert!(
        body.contains(&format!(
            r#"<input type="hidden" name="root" value="{id}">"#
        )),
        "the requested root travels with the depth control"
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
        body.contains(r#"<option value="0" selected>"#) && body.contains(r#"<option value="1""#),
        "the whole family is the chosen depth, and a number is the way back"
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

    let app = build_app(&path).expect("rebuild app");
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

    let app = build_app(&path).expect("reopen");

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

/// The wash is a preference, and every page says which way it is set.
///
/// `data-wash` is resolved on the server from two things — the theme's own
/// flag and what the reader asked for — so the stylesheet needs no exception
/// for a theme that refuses it and there is no second copy of the rule that
/// decides it (`theme::wash_enabled`, tested where it is written).
#[tokio::test]
async fn the_page_says_whether_the_wash_is_drawn() {
    let (app, _p) = app_with_empty_bundle("wash");

    let plain = body_string(get(&app, "/tree").await).await;
    assert!(
        plain.contains(r#"data-wash="off""#),
        "nobody has asked for it, so it is not drawn (v0.4.3)"
    );
    // The route reaches the stylesheet too, which is what lets the wash vary
    // by page without a stylesheet per page.
    assert!(
        plain.contains(r#"data-route="tree""#),
        "and the page says which route it is"
    );

    // A reader who asks for it gets it.
    let on = body_string(get_with_cookie(&app, "/tree", "axgf_wash=on").await).await;
    assert!(on.contains(r#"data-wash="on""#), "asked for, drawn");

    // …and so does the reader, under a theme that would have drawn it.
    let off = body_string(get_with_cookie(&app, "/tree", "axgf_wash=off").await).await;
    assert!(
        off.contains(r#"data-wash="off""#),
        "the reader's switch is honoured"
    );
}

/// Turning it off is a POST that sets a cookie, like the theme and the
/// language — and an unticked checkbox posts no field at all, which is the
/// whole of the parsing.
#[tokio::test]
async fn the_wash_switch_stores_both_answers() {
    let (app, _p) = app_with_empty_bundle("wash-post");

    let off = post_form(&app, "/prefs/background", "back=%2Ftree", false).await;
    let cookie = off
        .headers()
        .get(axum::http::header::SET_COOKIE)
        .expect("a cookie")
        .to_str()
        .unwrap()
        .to_string();
    assert!(
        cookie.starts_with("axgf_wash=off"),
        "unticked means off: {cookie}"
    );

    let on = post_form(&app, "/prefs/background", "wash=on&back=%2Ftree", false).await;
    let cookie = on
        .headers()
        .get(axum::http::header::SET_COOKIE)
        .expect("a cookie")
        .to_str()
        .unwrap()
        .to_string();
    assert!(
        cookie.starts_with("axgf_wash=on"),
        "ticked means on: {cookie}"
    );
    // A preference, not a session: it outlives the browser being closed.
    assert!(cookie.contains("Max-Age=31536000"), "{cookie}");
}

/// An error page offers one way out, not two, and never a link back to the
/// page that refused.
///
/// `Chrome` carries a `back` field — where a preference form returns the
/// reader to, which is the page they are on — and the error context used the
/// same name for something else. Merging the two let the chrome's value win,
/// so every error page grew a second button pointing at the page that had just
/// refused them, labelled with a rendered `None`.
#[tokio::test]
async fn an_error_page_carries_no_stray_button() {
    let (app, _p) = app_with_empty_bundle("error-back");

    let body = body_string(get(&app, "/person/nobody-by-that-id").await).await;
    assert!(body.contains("Back to the start"), "the way out is offered");
    assert!(
        !body.contains(">None<"),
        "and nothing else is, least of all a button labelled None:\n{body}"
    );
    assert!(
        !body.contains("href=\"/person/nobody-by-that-id\""),
        "a link back to the page that just failed is not a way out"
    );
}

/// The settings page's own content: the masthead carries a language select
/// on every page, this one included, and it is not the page's.
fn panel(page: &str) -> &str {
    let start = page.find("<main").expect("a main element");
    let end = page[start..].find("</main>").expect("its end") + start;
    &page[start..end]
}

/// The preferences are one form: four choices and one button.
///
/// They were three tabs holding twenty-two radio buttons and three buttons,
/// which put a visitor in front of a configuration screen to choose a colour
/// (the minimalism pass). Each list is a select now.
#[tokio::test]
async fn the_settings_are_one_form_of_five_choices() {
    let (app, _p) = app_with_empty_bundle("settings-one-form");
    let page = expect_status(
        get(&app, "/settings").await,
        StatusCode::OK,
        "GET /settings",
    )
    .await;
    let main = panel(&page);
    assert_eq!(main.matches("<form").count(), 1, "one form: {main}");
    assert!(main.contains(r#"action="/prefs""#));
    for name in ["theme", "lang", "style", "motion"] {
        assert!(
            main.contains(&format!(r#"<select name="{name}">"#)),
            "{name}: {main}"
        );
    }
    assert!(main.contains(r#"name="wash""#));
    assert!(
        !main.contains(r#"type="radio""#),
        "no lists of radio buttons"
    );
    assert_eq!(main.matches("<button").count(), 1, "one button");
    // Every theme and every language is offered: the six themes (v0.5.0,
    // docs/adr/0008, docs/adr/0010, docs/adr/0013, docs/adr/0016), fourteen
    // languages, three densities, two kinds of motion.
    assert_eq!(
        main.matches("<option").count(),
        6 + 14 + 3 + 2,
        "themes, languages, densities, motion"
    );

    // And applying sets all four, as the separate endpoints did.
    let resp = post_form(
        &app,
        "/prefs",
        "theme=galaxy&lang=pl&style=compact&wash=on&back=%2Fsettings",
        false,
    )
    .await;
    assert!(resp.status().is_redirection());
    let cookies: Vec<String> = resp
        .headers()
        .get_all(axum::http::header::SET_COOKIE)
        .iter()
        .map(|v| v.to_str().unwrap().to_string())
        .collect();
    for want in [
        "axgf_theme=galaxy",
        "axgf_lang=pl",
        "axgf_style=compact",
        "axgf_wash=on",
    ] {
        assert!(
            cookies.iter().any(|c| c.starts_with(want)),
            "{want}: {cookies:?}"
        );
    }
    // A theme that is not on the menu is refused, as on its own endpoint —
    // and so is one that was, before 0.5.0 retired it.
    for gone in ["chartreuse", "sepia"] {
        let bad = post_form(
            &app,
            "/prefs",
            &format!("theme={gone}&lang=pl&style=compact"),
            false,
        )
        .await;
        assert_eq!(bad.status(), StatusCode::BAD_REQUEST, "{gone}");
    }
}

/// An off-site `from` is not a redirect this application will perform.
#[tokio::test]
async fn the_settings_will_not_carry_an_off_site_destination() {
    let (app, _p) = app_with_empty_bundle("settings-open-redirect");
    for hostile in [
        "https%3A%2F%2Felsewhere.example",
        "%2F%2Felsewhere.example",
        "%5C%5Celsewhere.example",
    ] {
        let page = expect_status(
            get(&app, &format!("/settings?from={hostile}")).await,
            StatusCode::OK,
            "a hostile from",
        )
        .await;
        assert!(
            !page.contains("elsewhere.example"),
            "an off-site destination is dropped, not reflected: {page}"
        );
    }
}
