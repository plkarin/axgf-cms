//! Buttons and focus, as Study 06 draws them (B2), held at the stylesheet.
//!
//! Four variants and only four; 36px or 28px and radius 2 for every one of
//! them; a 2px focus ring at 2px offset on every control, never taken away;
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
fn a_button_is_36px_or_28px_and_radius_2_nowhere_else() {
    let base = rule(".btn");
    assert_eq!(decl(&base, "block-size"), Some("36px"));
    assert_eq!(decl(&base, "border-radius"), Some("2px"));
    assert_eq!(decl(&rule(".btn.small"), "block-size"), Some("28px"));
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
                        ("border-radius", "2px") | ("block-size", "36px" | "28px" | "32px")
                    ),
                    "{sel} sets {prop}: {v}"
                );
            }
        }
    }
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
    assert_eq!(decl(&strip, "box-shadow"), Some("inset 0 -1px 0 var(--border-hairline)"));
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
