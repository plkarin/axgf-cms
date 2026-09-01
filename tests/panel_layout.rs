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
/// Read as an admin throughout. The fixture's living person is `members` on
/// purpose — that is what a real record of a living relative looks like — so
/// an anonymous fetch would now be refused, and these tests are about how the
/// record *lays out*, not about who may read it. Visibility has its own file.
async fn panel(app: &axum::Router, id: &str) -> String {
    let body = expect_status(
        get_admin(app, &format!("/tree/panel/{id}")).await,
        StatusCode::OK,
        "panel fragment",
    )
    .await;
    assert!(body.contains("panel-inner"), "not a panel: {body}");
    body
}

async fn page(app: &axum::Router, id: &str) -> String {
    expect_status(
        get_admin(app, &format!("/person/{id}")).await,
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

/// Section prose is one click away on both surfaces.
///
/// It used to be printed under the heading on the standalone page and folded
/// behind a `?` only in the narrow panel. Printed, it is the page explaining
/// itself above content that says it better; the words are still there for
/// whoever wants them.
#[tokio::test]
async fn section_prose_is_behind_a_question_mark_on_both_surfaces() {
    let (app, _p) = app_with_bundle("panel-help", &bundle("panel-help-src"));
    let opening = "Birth, death and every event this person took part in";

    for (surface, body) in [
        ("the standalone page", page(&app, SOLO).await),
        ("the panel", panel(&app, SOLO).await),
    ] {
        assert!(
            body.contains("sec-help"),
            "{surface} puts the explanation behind a small affordance"
        );
        assert!(
            body.contains(opening),
            "…without dropping the words: {surface} must not be a lesser record"
        );
        // The prose is inside the collapsed <details>, not loose in the
        // section. Anchored on `sec-help` because the full page's first
        // disclosure is the masthead's preferences menu, which the panel
        // fragment does not carry.
        let after_summary = body
            .split("class=\"sec-help\"")
            .nth(1)
            .expect("a section disclosure")
            .split("</summary>")
            .nth(1)
            .unwrap_or("");
        assert!(
            after_summary
                .trim_start()
                .starts_with("<p class=\"small muted\">"),
            "on {surface} the text must live inside the disclosure, not beside \
             it:\n{after_summary:.120}"
        );
    }
}

/// The tree page is a record, not a manual.
///
/// Two sentences taught the reader how to work the interface — what a faint
/// connector meant, and that clicking a card opens the panel. The confidence
/// legend on the same page is the key to the first, and the second described a
/// gesture the reader had already made by the time they could read it.
#[test]
fn the_tree_page_does_not_explain_how_to_use_it() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let en = std::fs::read_to_string(root.join("locales/en.ftl")).expect("en.ftl");
    let html = std::fs::read_to_string(root.join("templates/tree.html")).expect("tree.html");

    for key in ["tree-click-hint", "tree-lede-whole"] {
        assert!(
            !en.contains(&format!("\n{key} =")),
            "{key} is a tutorial and was removed; it must not come back"
        );
        assert!(
            !html.contains(key),
            "{key} is still asked for by the template"
        );
    }

    // The counts survive: how many ancestors, descendants and partners are
    // drawn is a fact about the family, not about the software.
    assert!(
        en.contains("\ntree-lede-focused ="),
        "the record summary stays"
    );
    let lede = en
        .split("\ntree-lede-focused =")
        .nth(1)
        .unwrap()
        .split("\ntree-")
        .next()
        .unwrap();
    assert!(
        !lede.contains("opacity") && !lede.contains("Oldest at the bottom"),
        "…without the lesson in how to read the drawing:\n{lede}"
    );

    // The legend is a key, not a tutorial, and the encoding needs one.
    for key in [
        "tree-confidence-label",
        "tree-band-certain",
        "tree-band-low",
    ] {
        assert!(
            en.contains(&format!("\n{key} =")),
            "{key} is the legend, kept"
        );
    }
    // So is the banner that reports a defect in the data.
    assert!(
        en.contains("\ntree-contradicts-title ="),
        "the self-contradiction banner names a data defect and stays"
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

/// The split follows the tree's real width rather than a fixed ratio.
///
/// Measured in a browser rather than reasoned about — the numbers are in the
/// release notes — but the *structure* those measurements depend on is pinned
/// here, because it is the part a later edit could quietly undo: the width has
/// to travel from the layout into the stylesheet, and the panel has to be
/// derived from it rather than left to the grid's own free-space sharing.
#[test]
fn the_tree_width_reaches_the_stylesheet_and_the_panel_is_derived_from_it() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let html = std::fs::read_to_string(root.join("templates/tree.html")).expect("tree.html");
    let css = std::fs::read_to_string(root.join("static/app.css")).expect("app.css");

    assert!(
        html.contains("--tree-width:{{ layout.width }}px"),
        "the split must carry the canvas width the layout computed"
    );
    assert!(
        html.contains("width:{{ layout.width }}px"),
        "and it must be the same figure the canvas itself uses — one number, \
         one source"
    );

    let rule = css
        .split(".tree-split {")
        .nth(1)
        .expect("a .tree-split rule")
        .split("\n}")
        .next()
        .unwrap();

    assert!(
        rule.contains("minmax(0, var(--tree-width"),
        "the tree track caps at the drawing and floors at zero, or a full \
         view pushes the panel off the screen:\n{rule}"
    );
    assert!(
        rule.contains("100% - var(--tree-width"),
        "the panel takes what the tree leaves. Grid shares free space equally \
         until each track hits its growth limit, so a panel merely *allowed* \
         to reach its maximum gets there before a wide tree has taken \
         anything — which at 1280px left the tree 656px where it could have \
         had 883:\n{rule}"
    );
    assert!(
        rule.contains("var(--panel-min)") && rule.contains("var(--panel-max)"),
        "the panel stays bounded at both ends:\n{rule}"
    );
    // The subtraction has to use the same gap the grid draws, or the panel is
    // out by however far the two drift. Both references, one variable.
    assert!(
        rule.contains("gap: var(--split-gap)"),
        "the drawn gap must come from the variable:\n{rule}"
    );
    assert!(
        rule.matches("var(--split-gap)").count() >= 2,
        "the gap subtracted from the panel must be the gap the grid draws, \
         not a second number that can drift from it:\n{rule}"
    );
}

/// One page scroll, not three.
///
/// The tree page used to nest three scrollable regions: the page, the tree
/// column inside its own fixed box, and the record panel inside its own. Which
/// one a wheel gesture drove depended on where the pointer happened to be. The
/// browser sweep in CONTRIBUTING.md is what proves the result at each viewport;
/// what is asserted here is the source-level shape that produces it, so a
/// `max-height` cannot quietly come back.
#[test]
fn neither_the_tree_nor_the_panel_scrolls_inside_itself() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let css = std::fs::read_to_string(root.join("static/app.css")).expect("app.css");
    let html = std::fs::read_to_string(root.join("templates/tree.html")).expect("tree.html");

    let rule = |selector: &str| -> String {
        css.split(selector)
            .nth(1)
            .unwrap_or_else(|| panic!("{selector} is not in the stylesheet"))
            .split("\n}")
            .next()
            .unwrap()
            .to_string()
    };

    let tree = rule(".tree-scroll {");
    assert!(
        !tree.contains("max-height"),
        "the tree column grows to its content; a ceiling here is a second \
         vertical scrollbar:\n{tree}"
    );
    assert!(
        tree.contains("overflow-x: auto") && tree.contains("overflow-y: hidden"),
        "sideways only — and stated on both axes, because a lone `overflow-x` \
         computes the other one back to `auto`:\n{tree}"
    );

    let panel = rule(".tree-panel {");
    assert!(
        !panel.contains("max-height") && !panel.contains("overflow"),
        "the record grows with the page rather than scrolling inside itself:\n{panel}"
    );

    // The tree is the pinned column, not the panel: the panel is the taller of
    // the two on almost every person, and the tallest item in a grid row has
    // no room to move inside it.
    assert!(
        tree.contains("position: sticky"),
        "the tree stays on screen while the record beside it scrolls:\n{tree}"
    );
    assert!(
        !panel.contains("position: sticky"),
        "pinning the panel is the thing that does not work:\n{panel}"
    );

    // Pinning by the bottom edge when the tree is too tall needs the height,
    // and it has to be the same figure the canvas is drawn at.
    assert!(
        html.contains("--tree-height:{{ layout.height }}px")
            && html.contains("height:{{ layout.height }}px"),
        "the canvas height reaches the stylesheet, from the one source"
    );
    assert!(
        tree.contains("var(--tree-height"),
        "and the sticky offset is derived from it:\n{tree}"
    );
}

/// The frames are gone, and the surface they used to divide is one surface.
#[test]
fn the_tree_and_the_record_read_as_one_surface() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let css = std::fs::read_to_string(root.join("static/app.css")).expect("app.css");

    let block = |selector: &str| -> String {
        css.split(selector)
            .nth(1)
            .unwrap_or_else(|| panic!("{selector} is not in the stylesheet"))
            .split("\n}")
            .next()
            .unwrap()
            .to_string()
    };

    for selector in [".tree-scroll {", ".tree-panel {"] {
        let rule = block(selector);
        assert!(
            !rule.contains("border:") && !rule.contains("background:"),
            "{selector} draws no frame of its own — those borders were the edge \
             of a scroll region that no longer exists:\n{rule}"
        );
    }

    let split = block(".tree-split {");
    assert!(
        split.contains("background: var(--surface)"),
        "the surface belongs to the split, once, so the two columns read as \
         one:\n{split}"
    );
}
