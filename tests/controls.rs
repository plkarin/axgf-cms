//! Buttons and focus, as Study 06 draws them (B2), held at the stylesheet.
//!
//! Four variants and only four; 36px or 28px and one radius for every one of
//! them (Study 06's 2px, or the reference theme's pill, ADR 0004); a 2px
//! focus ring at 2px offset on every control, never taken away;
//! a disabled secondary is dashed; danger is an outline, never a fill. The
//! one-primary-per-page rule is held where every page is fetched, in
//! `tests/interactive_walk.rs`.

/// app.css without its comments, which quote the rules they replaced.
fn css() -> String {
    let src = include_str!("../static/app.css");
    let mut out = String::new();
    let mut rest = src;
    while let Some(i) = rest.find("/*") {
        out.push_str(&rest[..i]);
        rest = &rest[i + rest[i..].find("*/").expect("closed comment") + 2..];
    }
    out + rest
}

/// Every rule as (selector, body), `@media` wrappers looked through.
fn rules() -> Vec<(String, String)> {
    let css = css();
    let mut out = Vec::new();
    let mut start = 0;
    for (i, c) in css.char_indices() {
        match c {
            '}' => start = i + 1,
            '{' => {
                let end = i + css[i..].find('}').expect("a closing brace");
                let body = &css[i + 1..end];
                if !body.contains('{') {
                    out.push((css[start..i].trim().to_string(), body.trim().to_string()));
                }
                start = i + 1;
            }
            _ => {}
        }
    }
    out
}

fn decl<'a>(body: &'a str, prop: &str) -> Option<&'a str> {
    body.split(';')
        .filter_map(|d| d.split_once(':'))
        .find(|(k, _)| k.trim() == prop)
        .map(|(_, v)| v.trim())
}

fn rule(selector: &str) -> String {
    rules()
        .into_iter()
        .find(|(s, _)| s == selector)
        .unwrap_or_else(|| panic!("no rule for {selector}"))
        .1
}

/// The button rules: any selector that names `.btn`, but not a layout rule
/// in some container that happens to hold one.
fn button_rules() -> Vec<(String, String)> {
    rules()
        .into_iter()
        .filter(|(s, _)| s.split(',').any(|part| part.trim().starts_with(".btn")))
        .collect()
}

#[test]
fn a_button_is_36px_or_28px_and_one_radius_per_theme() {
    let base = rule(".btn");
    // A minimum, so text at the reader's own size grows the control rather
    // than spilling out of it (rem type, ADR 0002 §4).
    assert_eq!(decl(&base, "min-block-size"), Some("36px"));
    // One radius for every button, and it is the theme's: Study 06's 2px in
    // every theme but Canopée, which rounds them into pills (ADR 0004). It
    // used to be the literal "2px" everywhere; a token keeps "one radius,
    // nowhere else" true per theme instead of forbidding the reference
    // theme its shape.
    assert_eq!(decl(&base, "border-radius"), Some("var(--btn-radius)"));
    assert_eq!(decl(&rule(".btn.small"), "min-block-size"), Some("28px"));
    for (sel, body) in rules() {
        if !sel.contains(".btn") {
            continue;
        }
        for prop in [
            "border-radius",
            "height",
            "block-size",
            "min-height",
            "min-block-size",
        ] {
            if let Some(v) = decl(&body, prop) {
                assert!(
                    matches!(
                        (prop, v),
                        ("border-radius", "var(--btn-radius)")
                            | (
                                "block-size" | "min-block-size",
                                "36px" | "28px" | "32px" | "44px"
                            )
                    ),
                    "{sel} sets {prop}: {v}"
                );
            }
        }
    }
    // Where the radius is decided: Study 06's on `:root`, and the five
    // themes that carry a shape of their own (the kit, docs/adr/0007).
    let defs: Vec<(String, String)> = rules()
        .into_iter()
        .filter_map(|(sel, body)| decl(&body, "--btn-radius").map(|v| (sel, v.to_string())))
        .collect();
    assert_eq!(
        defs,
        vec![
            (":root".to_string(), "2px".to_string()),
            ("[data-theme=\"canopee\"]".to_string(), "999px".to_string()),
            ("[data-theme=\"fusion\"]".to_string(), "8px".to_string()),
            ("[data-theme=\"galaxy\"]".to_string(), "999px".to_string()),
            ("[data-theme=\"summit\"]".to_string(), "12px".to_string()),
            ("[data-theme=\"clarity\"]".to_string(), "10px".to_string()),
            (
                "[data-theme=\"genealogy\"]".to_string(),
                "999px".to_string()
            ),
        ],
        "--btn-radius is defined on :root and by the kit's six themes, and nowhere else"
    );
}

#[test]
fn there_are_four_variants_and_only_four() {
    let mut variants = std::collections::BTreeSet::new();
    for (sel, _) in button_rules() {
        for part in sel.split(',') {
            let first = part.trim().split([' ', ':', '[', '>']).next().unwrap_or("");
            for class in first.split('.').skip(2) {
                variants.insert(class.to_string());
            }
        }
    }
    // `small` is a size, not a variant.
    variants.remove("small");
    assert_eq!(
        variants.into_iter().collect::<Vec<_>>(),
        ["danger", "ghost", "primary"],
        "secondary is `.btn` itself"
    );
}

#[test]
fn danger_is_an_outline_never_a_fill_and_disabled_is_dashed() {
    for (sel, body) in button_rules() {
        if sel.contains("danger") {
            for prop in ["background", "background-color"] {
                if let Some(v) = decl(&body, prop) {
                    assert!(
                        !v.contains("--danger"),
                        "{sel} fills with the danger colour: {v}"
                    );
                }
            }
        }
    }
    let danger = rule(".btn.danger");
    assert_eq!(decl(&danger, "border-color"), Some("var(--danger)"));
    let disabled = rule(r#".btn:disabled, .btn[aria-disabled="true"]"#);
    assert_eq!(decl(&disabled, "border-style"), Some("dashed"));
}

#[test]
fn every_control_carries_the_focus_ring_and_nothing_removes_it() {
    let ring = rules()
        .into_iter()
        .find(|(s, _)| {
            s.contains(":focus-visible") && s.contains("button") && s.contains("summary")
        })
        .expect("one focus rule for every control");
    for control in [
        "a",
        "button",
        "input",
        "select",
        "textarea",
        "summary",
        "[tabindex]",
    ] {
        assert!(
            ring.0.contains(control),
            "{control} is not in the focus rule: {}",
            ring.0
        );
    }
    assert_eq!(
        decl(&ring.1, "outline"),
        Some("2px solid var(--focus-ring)")
    );
    assert_eq!(decl(&ring.1, "outline-offset"), Some("2px"));
    for (sel, body) in rules() {
        if let Some(v) = decl(&body, "outline") {
            assert!(
                v == "2px solid var(--focus-ring)" || sel.contains("print"),
                "{sel} sets its own outline: {v}"
            );
        }
        if let Some(v) = decl(&body, "outline-style") {
            assert_ne!(v, "none", "{sel} removes the outline");
        }
    }
}

#[test]
fn a_button_that_submits_work_says_so_while_it_is_busy() {
    // Every primary that submits carries the words it shows while the
    // request is out: no spinner, and no button that looks dead.
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("templates");
    let mut missing = Vec::new();
    for entry in std::fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        let src = std::fs::read_to_string(&path).unwrap();
        let mut rest = src.as_str();
        while let Some(i) = rest.find("<button") {
            let tag = &rest[i..i + rest[i..].find('>').unwrap()];
            if tag.contains("btn primary")
                && !tag.contains(r#"type="button""#)
                && !tag.contains("data-busy")
            {
                missing.push(format!(
                    "{}: {tag}>",
                    path.file_name().unwrap().to_string_lossy()
                ));
            }
            rest = &rest[i + 7..];
        }
    }
    assert!(missing.is_empty(), "{}", missing.join("\n"));
}

/// The record's tab strip is one row at any width (B4). It scrolls sideways
/// rather than wrapping — production wrapped at phone width — and its tabs
/// never shrink or break their labels. Logical properties throughout, so
/// `dir="rtl"` mirrors the order and the underline stays at the block end.
/// The row is measured in a browser at 390px in French, German and Arabic for
/// the 0.4.0 report; this holds the rules that make it so.
#[test]
fn the_tab_strip_is_one_row_that_scrolls_and_mirrors() {
    let strip = rule(".person-tabs");
    let tab = rule(".person-tab");
    assert_eq!(decl(&strip, "flex-wrap"), Some("nowrap"), "{strip}");
    assert_eq!(decl(&strip, "overflow-x"), Some("auto"), "{strip}");
    // The hairline inside the strip: a scrolling strip clips anything hung
    // below it, which is how the current tab's rule lost half its width.
    assert_eq!(
        decl(&strip, "box-shadow"),
        Some("inset 0 -1px 0 var(--border-hairline)")
    );
    assert!(decl(&tab, "margin-block-end").is_none(), "{tab}");
    let tab = rule(".person-tab");
    assert_eq!(
        decl(&tab, "flex"),
        Some("none"),
        "a tab never shrinks: {tab}"
    );
    assert_eq!(decl(&tab, "white-space"), Some("nowrap"), "{tab}");
    assert_eq!(
        decl(&tab, "border-block-end"),
        Some("2px solid transparent")
    );
    for (sel, body) in rules() {
        if sel.contains("person-tab") {
            for physical in [
                "border-bottom",
                "margin-left",
                "margin-right",
                "padding-left",
                "padding-right",
            ] {
                assert!(decl(&body, physical).is_none(), "{sel} uses {physical}");
            }
            assert_ne!(decl(&body, "flex-wrap"), Some("wrap"), "{sel} wraps");
        }
    }
    let current = rule(".person-tab.is-current");
    assert_eq!(
        decl(&current, "border-block-end-color"),
        Some("var(--accent)")
    );
    assert_eq!(decl(&current, "color"), Some("var(--text-primary)"));
    let count = rule(".tab-count");
    assert_eq!(decl(&count, "font-family"), Some("var(--mono)"));
}

/// The river's controls stand in order (2026-10-22): the itinerary on a row
/// of its own that scrolls sideways rather than wrapping, the tools on the
/// next, their last two held to its far end; the legend on a grid.
#[test]
fn the_rivers_controls_are_two_ordered_rows_and_the_legend_a_grid() {
    let controls = rule(".rv-controls");
    assert_eq!(decl(&controls, "display"), Some("flex"));
    assert_eq!(decl(&controls, "flex-direction"), Some("column"));
    let trail = rule(".rv-trail");
    assert_eq!(
        decl(&trail, "flex-wrap"),
        Some("nowrap"),
        "the itinerary does not wrap"
    );
    assert_eq!(
        decl(&trail, "overflow-x"),
        Some("auto"),
        "it scrolls instead"
    );
    assert_eq!(decl(&rule(".rv-trail > li"), "flex"), Some("none"));
    assert_eq!(decl(&rule(".rv-tools"), "display"), Some("flex"));
    assert_eq!(
        decl(&rule(".rv-tools-end"), "margin-inline-start"),
        Some("auto")
    );
    let legend = rule(".rv-legend");
    assert_eq!(decl(&legend, "display"), Some("grid"));
    assert!(decl(&legend, "grid-template-columns").is_some_and(|v| v.contains("auto-fill")));
    assert_eq!(
        decl(
            &rule(".rv-legend > :is(.rv-legend-title, .rv-widths, .rv-direction, .rv-hint)"),
            "grid-column"
        ),
        Some("1 / -1"),
        "the title, the widths, the direction and the hint run across it"
    );

    let river = include_str!("../templates/river.html");
    let at = |needle: &str| river.find(needle).unwrap_or_else(|| panic!("{needle}"));
    assert!(at(r#"class="rv-controls""#) < at(r#"class="rv-itinerary""#));
    assert!(at(r#"class="rv-itinerary""#) < at(r#"class="rv-trail""#));
    assert!(at(r#"class="rv-trail""#) < at(r#"class="rv-tools""#));
    assert!(at(r#"class="rv-tools""#) < at(r#"id="rv-back""#));
    assert!(at(r#"id="rv-back""#) < at(r#"class="rv-range""#));
    assert!(at(r#"class="rv-range""#) < at(r#"class="rv-tools-end""#));
}

/// At phone width the river's buttons share their row and never break a
/// word, a table whose cells carry their labels becomes a list of cards, and
/// the tree arrives on the person it stands on (2026-10-25).
#[test]
fn on_a_phone_the_controls_line_up() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let css = std::fs::read_to_string(root.join("static/app.css")).expect("app.css");
    assert!(css.contains(
        ".rv-tools-end > .btn { flex: 1 1 auto; justify-content: center; text-align: center; white-space: nowrap; }"
    ));
    assert!(css.contains(
        ".table-wrap > table:has(> tbody > tr > td[data-label]) > thead { display: none; }"
    ));
    let tree = std::fs::read_to_string(root.join("static/tree.js")).expect("tree.js");
    assert!(tree.contains(
        "centreOn(document.querySelector('#tree-canvas .tcard.is-selected') || document.querySelector('#tree-canvas .tcard.is-root'));"
    ));
}
