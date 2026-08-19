//! The record sections at side-panel width.
//!
//! The panel is a clamped column of at most 460px. Everything here is about
//! what that column does to layout the page never had to worry about, and the
//! rule the panel is held to: the *same facts* as the standalone page, laid
//! out differently. A test that only checked the page would have passed
//! throughout the period the panel was unreadable.

mod common;

use axum::http::StatusCode;
use common::*;
use serde_json::json;

const SOLO: &str = "11111111-1111-4111-8111-111111111111";
const MANY: &str = "22222222-2222-4222-8222-222222222222";

/// One person with a single name, one with four, both with evidence.
fn bundle(tag: &str) -> std::path::PathBuf {
    let dir = scratch(tag);
    let path = dir.join("panel.axgf");
    let flat = json!({
        "manifest": {"axgf": "1.0"},
        "persons": {
            SOLO: {
                "id": SOLO, "type": "person", "axgf_version": "1.0",
                "identity": {
                    "name": {"display": "Laura Karin", "components": [
                        {"type": "given_name", "value": "Laura", "order": 1},
                        {"type": "family_name", "value": "Karin", "order": 2}]},
                    "gender": {"value": "F"}, "is_living": true,
                    "visibility": "members"},
                "birth": {"date": {"value": "1991-10-29", "precision": "exact"},
                          "confidence": 0.8,
                          "source_id": "44444444-4444-4444-8444-444444444444"}
            },
            MANY: {
                "id": MANY, "type": "person", "axgf_version": "1.0",
                "identity": {
                    "name": {"display": "Marianna Ludwika Rumiński",
                             "components": [
                                {"type": "given_name", "value": "Marianna Ludwika", "order": 1},
                                {"type": "family_name", "value": "Rumiński", "order": 2}]},
                    "gender": {"value": "F"}, "is_living": false,
                    "names": [
                        {"display": "Rumiński", "type": "alias", "components": []},
                        {"display": "Marianna Ludwika Łęczycki", "type": "alias",
                         "components": [], "valid_from": "1902"},
                        {"display": "Мария Лещицкая", "type": "transliteration",
                         "display_latin": "Mariya Leshchitskaya", "culture": "ru",
                         "direction": "ltr", "components": [], "confidence": 0.5}]}
            }
        },
        "families": {}, "events": {}, "links": {}, "occupations": {},
        "sources": {
            "44444444-4444-4444-8444-444444444444": {
                "id": "44444444-4444-4444-8444-444444444444", "type": "source",
                "axgf_version": "1.0", "title": "Parish register, Nice",
                "source_type": "register", "reliability": "primary", "confidence": 0.9}
        },
        "places": {}, "documents": {}
    });
    let bytes = axgf_cms::state::export_to_bytes(&flat.to_string()).expect("export");
    std::fs::write(&path, bytes).expect("write");
    path
}

/// Everything between the panel's opening `<div class="panel-inner"` and its
/// end, so an assertion about the panel cannot accidentally be satisfied by
/// the surrounding tree page.
async fn panel(app: &axum::Router, id: &str) -> String {
    let body = expect_status(
        get(app, &format!("/tree/panel/{id}")).await,
        StatusCode::OK,
        "panel fragment",
    )
    .await;
    assert!(body.contains("panel-inner"), "not a panel: {body}");
    body
}

async fn page(app: &axum::Router, id: &str) -> String {
    expect_status(
        get(app, &format!("/person/{id}")).await,
        StatusCode::OK,
        "person page",
    )
    .await
}

#[tokio::test]
async fn a_single_name_is_a_heading_not_a_four_column_table() {
    let (app, _p) = app_with_bundle("panel-solo", &bundle("panel-solo-src"));

    for (surface, body) in [
        ("panel", panel(&app, SOLO).await),
        ("page", page(&app, SOLO).await),
    ] {
        assert!(
            body.contains("nb-display") && body.contains("Laura Karin"),
            "{surface}: one name should render as a name block"
        );
        assert!(
            !body.contains("<table class=\"names\">"),
            "{surface}: a single name must not open the comparison table — it \
             spends four columns, three of them empty, to state one fact"
        );
        // The type is still stated, as a small label beside the name.
        assert!(
            body.contains("nb-type"),
            "{surface}: the name's type must still be shown"
        );
    }
}

#[tokio::test]
async fn several_names_compare_as_a_table_on_the_page_and_a_stack_in_the_panel() {
    let (app, _p) = app_with_bundle("panel-many", &bundle("panel-many-src"));

    let page = page(&app, MANY).await;
    assert!(
        page.contains("<table class=\"names\">"),
        "the page has the width for the comparison table, which is the point \
         when several names exist"
    );

    let panel = panel(&app, MANY).await;
    assert!(
        !panel.contains("<table class=\"names\">"),
        "a four-column table in a 460px column scrolls sideways"
    );
    assert_eq!(
        panel.matches("class=\"name-block").count(),
        4,
        "every name is still shown in the panel — one block each"
    );
    // And the comparison still carries what a comparison needs: type, period,
    // script and evidence.
    assert!(panel.contains("transliteration"), "type is shown");
    assert!(
        panel.contains("1902"),
        "the period a name was used is shown"
    );
    assert!(panel.contains("lang=\"ru\""), "the script is tagged");
    assert!(
        panel.contains("Mariya Leshchitskaya"),
        "the transliteration"
    );
    assert!(panel.contains("conf-bar"), "the evidence for a name");
}

#[tokio::test]
async fn a_name_component_is_one_box_so_it_cannot_wrap_mid_value() {
    let (app, _p) = app_with_bundle("panel-parts", &bundle("panel-parts-src"));
    let body = panel(&app, SOLO).await;

    // "given name: Laura" was one run of text in a longer string, so a narrow
    // column could break it anywhere. Each part is now its own box with the
    // label and the value inside it.
    for want in [
        "<span class=\"name-part\"><span class=\"np-k\">given name</span>\
         <span class=\"np-v\">Laura</span></span>",
        "<span class=\"name-part\"><span class=\"np-k\">family name</span>\
         <span class=\"np-v\">Karin</span></span>",
    ] {
        assert!(
            body.contains(want),
            "the label and its value must be one unbreakable unit; missing:\n{want}"
        );
    }
    assert!(
        !body.contains("given name: Laura"),
        "the joined form is what wrapped mid-value"
    );
}

#[tokio::test]
async fn gender_living_and_visibility_are_labelled_chips_not_a_run_on_string() {
    let (app, _p) = app_with_bundle("panel-facts", &bundle("panel-facts-src"));
    let body = panel(&app, SOLO).await;

    assert!(
        body.contains("<dl class=\"fact-chips\">"),
        "the three facts are a definition list of chips"
    );
    for want in [
        "<div class=\"fc\"><dt>Gender</dt><dd>Female</dd></div>",
        "<div class=\"fc\"><dt>Living</dt><dd>yes</dd></div>",
        "<div class=\"fc\"><dt>Visibility</dt><dd>members</dd></div>",
    ] {
        assert!(
            body.contains(want),
            "each fact is its own labelled box: {want}"
        );
    }
    // Every label is inside a <dt> that its value follows in a <dd>; nothing
    // is left as bare interleaved text.
    assert!(
        !body.contains("inline-facts"),
        "the old inline run is gone, not merely restyled"
    );
}

#[tokio::test]
async fn section_prose_is_on_the_page_and_behind_a_question_mark_in_the_panel() {
    let (app, _p) = app_with_bundle("panel-help", &bundle("panel-help-src"));
    let opening = "Birth, death and every event this person took part in";

    let page = page(&app, SOLO).await;
    assert!(
        page.contains(opening) && !page.contains("sec-help"),
        "the standalone page prints the explanation under the heading"
    );

    let panel = panel(&app, SOLO).await;
    assert!(
        panel.contains("sec-help"),
        "the panel puts it behind a small affordance"
    );
    assert!(
        panel.contains(opening),
        "…without dropping the words: the panel must not be a lesser record"
    );
    // The prose is inside the collapsed <details>, not loose in the section.
    let after_summary = panel.split("</summary>").nth(1).unwrap_or("");
    assert!(
        after_summary.starts_with("\n    <p class=\"small muted\">"),
        "the text must live inside the disclosure, not beside it"
    );
}

#[tokio::test]
async fn the_wide_tables_carry_the_labels_that_let_the_panel_stack_them() {
    let (app, _p) = app_with_bundle("panel-tables", &bundle("panel-tables-src"));
    let body = panel(&app, SOLO).await;

    assert!(
        body.contains("<table class=\"stackable\">"),
        "the evidence tables are marked as stackable"
    );
    for want in [
        "data-label=\"Source\"",
        "data-label=\"Type\"",
        "data-label=\"Reliability\"",
        "data-label=\"Supports\"",
        "data-label=\"Confidence\"",
    ] {
        assert!(
            body.contains(want),
            "every cell needs the label the stacked row shows in place of the \
             column head: {want}"
        );
    }
}

#[tokio::test]
async fn the_panel_and_the_page_state_the_same_facts() {
    // The whole rule the compact layout is held to. Anything the page says
    // about this person, the panel says too — only the arrangement differs.
    let (app, _p) = app_with_bundle("panel-parity", &bundle("panel-parity-src"));
    let panel = panel(&app, SOLO).await;
    let page = page(&app, SOLO).await;

    for fact in [
        "Laura Karin",
        "Laura",
        "Karin",
        "Female",
        "members",
        "29 October 1991",
        "Parish register, Nice",
        "80%",
    ] {
        assert!(panel.contains(fact), "panel is missing {fact}");
        assert!(page.contains(fact), "page is missing {fact}");
    }
}
