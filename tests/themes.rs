//! Every theme is complete, and confidence never rests on colour.
//!
//! These are source-level checks. What a theme actually *looks* like is
//! measured in a browser (see the contrast and confidence sweeps described in
//! CONTRIBUTING.md); what is asserted here is the structural property that
//! makes those sweeps meaningful — that no theme is quietly inheriting another
//! theme's value for something it should define itself.

use std::collections::BTreeSet;

fn css() -> String {
    std::fs::read_to_string(
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("static/app.css"),
    )
    .expect("read app.css")
}

/// The custom properties defined inside one selector's block.
fn block_vars(css: &str, selector: &str) -> BTreeSet<String> {
    let start = css
        .find(selector)
        .unwrap_or_else(|| panic!("{selector} is not in the stylesheet"));
    let open = css[start..].find('{').expect("a block") + start;
    let end = css[open..].find("\n}").expect("a closing brace") + open;
    css[open..end]
        .lines()
        .filter_map(|l| l.trim().strip_prefix("--"))
        .filter_map(|l| l.split(':').next())
        .map(|n| format!("--{n}"))
        .collect()
}

/// Every theme's own selector. `system` has no block: it is the absence of a
/// `data-theme` attribute, which is what lets `prefers-color-scheme` decide.
const THEME_SELECTORS: &[&str] = &[
    "[data-theme=\"dark\"] {",
    "[data-theme=\"high-contrast\"] {",
    "[data-theme=\"sepia\"] {",
    "[data-theme=\"deuteranopia\"],",
    "[data-theme=\"tritanopia\"] {",
];

#[test]
fn every_theme_redefines_every_colour_root_defines() {
    // A theme that misses one property inherits `:root`'s — which is the light
    // palette. One forgotten line is then a light-coloured element sitting in
    // a dark page, and it will be found by a reader rather than by us.
    let css = css();
    // Geometry, typography and timing are the same in every theme by design:
    // a theme changes what the interface looks like, not how fast it moves or
    // how round its corners are. Everything else is a colour, and a theme that
    // misses one inherits the light palette's.
    const NOT_A_COLOUR: &[&str] = &[
        "--radius",
        "--radius-sm",
        "--radius-lg",
        "--mono",
        "--tempo",
        "--ease",
        "--shadow-a",
    ];
    let root: BTreeSet<String> = block_vars(&css, ":root {")
        .into_iter()
        .filter(|v| !NOT_A_COLOUR.contains(&v.as_str()))
        .collect();

    for sel in THEME_SELECTORS {
        let mine = block_vars(&css, sel);
        let missing: Vec<&String> = root.difference(&mine).collect();
        assert!(
            missing.is_empty(),
            "{sel} does not define {missing:?} and will inherit the light palette"
        );
    }
}

#[test]
fn the_confidence_track_is_never_the_border_colour() {
    // The bar's track has to read as *empty*. It used to be `--border`, which
    // under the high-contrast theme is pure black, so a 12% bar rendered as a
    // mostly-dark bar and looked more certain than a 98% one — inverting the
    // signal in the theme that needs it most.
    let css = css();
    let bar = css
        .split(".conf-bar {")
        .nth(1)
        .expect("a .conf-bar rule")
        .split('}')
        .next()
        .unwrap();
    assert!(
        bar.contains("var(--conf-track)"),
        "the track must have its own variable, not borrow --border:\n{bar}"
    );
}

#[test]
fn the_confidence_dot_is_filled_to_the_value_not_merely_tinted() {
    // The dot is what the tree cards and the dense lists use. Distinguishing
    // the bands by hue alone is precisely what a colour-blind reader does not
    // receive, so the dot is a pie filled to the number.
    let css = css();
    let dot = css
        .split(".conf-dot {")
        .nth(1)
        .expect("a .conf-dot rule")
        .split("\n}")
        .next()
        .unwrap();
    assert!(
        dot.contains("conic-gradient") && dot.contains("--conf-pct"),
        "the dot must encode the value, not the band:\n{dot}"
    );
}

#[test]
fn every_crossing_marker_carries_a_dash_pattern_as_well_as_a_hue() {
    // Colour alone does not survive the themes this file exists for. Two
    // crossing lines that converge in hue must still differ in rhythm.
    let css = css();
    let mut seen = 0;
    let mut patterns: BTreeSet<String> = BTreeSet::new();
    for n in 1..=8 {
        let sel = format!(".wire-crossing[data-hue=\"{n}\"]");
        let rule = css
            .split(&sel)
            .nth(1)
            .unwrap_or_else(|| panic!("{sel} has no rule"))
            .split('}')
            .next()
            .unwrap();
        assert!(
            rule.contains(&format!("var(--hue-{n})")),
            "{sel} has no hue"
        );
        assert!(
            rule.contains("stroke-dasharray"),
            "{sel} carries a hue and nothing else"
        );
        let dash = rule
            .split("stroke-dasharray:")
            .nth(1)
            .unwrap()
            .split(';')
            .next()
            .unwrap()
            .trim()
            .to_string();
        patterns.insert(dash);
        seen += 1;
    }
    assert_eq!(seen, 8);
    assert_eq!(
        patterns.len(),
        8,
        "two markers share a dash pattern, so they differ only by hue: {patterns:?}"
    );
}

#[test]
fn reduced_motion_is_respected() {
    // A medical setting, not a taste. The tree's hover transitions fire
    // hundreds of times as a pointer crosses a canvas of cards.
    let css = css();
    assert!(css.contains("@media (prefers-reduced-motion: reduce)"));
    let block = css
        .split("@media (prefers-reduced-motion: reduce)")
        .nth(1)
        .unwrap();
    assert!(block.contains("transition-duration"));
    assert!(block.contains("animation-duration"));
}

#[test]
fn nothing_outside_the_theme_blocks_hardcodes_a_colour() {
    // The point of the palette is that adding a theme is a palette and nothing
    // else. A literal colour further down the file is one that no theme can
    // override, and it will be the thing that looks wrong in dark mode.
    let css = css();
    let themes_end = css
        .find(".tl-open-l")
        .or_else(|| css.find("/* --- colour-blind themes"))
        .map(|_| css.rfind("[data-theme=\"tritanopia\"]").unwrap())
        .expect("the theme blocks");
    let end_of_themes = css[themes_end..].find("\n}").unwrap() + themes_end;
    let body = &css[end_of_themes..];

    // Comments span lines, and several of them quote a colour while
    // explaining why it was wrong. Strip them wholesale rather than per line.
    let body = strip_comments(body);
    let mut offenders = Vec::new();
    for (i, line) in body.lines().enumerate() {
        let code = line;
        if code.contains('#') {
            // A hex colour is `#` followed by 3, 4, 6 or 8 hex digits.
            for part in code.split('#').skip(1) {
                let run: String = part.chars().take_while(|c| c.is_ascii_hexdigit()).collect();
                if matches!(run.len(), 3 | 4 | 6 | 8) {
                    offenders.push(format!("  line {}: {}", i + 1, line.trim()));
                    break;
                }
            }
        }
    }
    assert!(
        offenders.is_empty(),
        "{} literal colour(s) outside the theme blocks — no theme can override \
         these:\n{}",
        offenders.len(),
        offenders.join("\n")
    );
}

/// Remove every `/* … */`, keeping newlines so line numbers still mean
/// something.
fn strip_comments(src: &str) -> String {
    let mut out = String::with_capacity(src.len());
    let mut rest = src;
    loop {
        match rest.find("/*") {
            None => {
                out.push_str(rest);
                return out;
            }
            Some(i) => {
                out.push_str(&rest[..i]);
                let after = &rest[i..];
                let end = after.find("*/").map(|j| j + 2).unwrap_or(after.len());
                for c in after[..end].chars() {
                    if c == '\n' {
                        out.push('\n');
                    }
                }
                rest = &after[end..];
            }
        }
    }
}
