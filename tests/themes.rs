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
        // Where the pinned tree column comes to rest, and the strip of room
        // kept under it for its horizontal scrollbar. Both are lengths: a
        // theme changes what the tree looks like, not where it stops.
        "--stick-top",
        "--hscroll-gutter",
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

/// The wash is a gradient, and only a gradient.
///
/// "No images, no external requests, no added bytes" is the whole reason a
/// gradient was chosen over a photograph, so it is worth a test rather than a
/// promise: a `url()` in the page background is a request the page did not
/// used to make and a file the binary did not used to carry.
#[test]
fn the_page_wash_fetches_nothing() {
    let css = css();
    let start = css.find("html {").expect("the root background rule");
    let end = css[start..].find("\nbody {").expect("the body rule") + start;
    let wash = &css[start..end];

    assert!(
        !wash.contains("url("),
        "the wash is drawn, not fetched:\n{wash}"
    );
    assert!(
        wash.contains("radial-gradient"),
        "…and it is a gradient:\n{wash}"
    );
    // Every colour in it comes from the theme's own tokens.
    for stop in ["var(--wash-1)", "var(--wash-2)", "var(--wash-3)"] {
        assert!(wash.contains(stop), "{stop} is not used in the wash");
    }
    assert!(
        !css.contains("background-image: linear-gradient(#"),
        "no rule paints a literal colour into a page background"
    );
}

/// It does not move, and not only under `prefers-reduced-motion`.
///
/// A gradient that drifts is a distraction on a page somebody reads for an
/// hour. That is a decision for every reader rather than one the motion query
/// makes for a few, so there is nothing to disable: nothing animates.
/// The wash moves only when the reader does.
///
/// It used to be forbidden from moving at all, and the reason was right: a
/// gradient that drifts on its own is a distraction on a page somebody reads
/// for an hour, and peripheral motion captures attention involuntarily —
/// which is the whole reason animated advertising is intolerable. What
/// changed is not that judgement but the mechanism available to honour it.
/// `animation-timeline: scroll()` ties the keyframes to scroll position rather
/// than to a clock, so there is still nothing that moves while the reader is
/// still. Stop scrolling and it stops.
///
/// So the rule this enforces is not "no animation" but "no clock": no
/// `infinite`, no `alternate`, no duration in seconds, and every animation on
/// the wash driven by a scroll timeline.
#[test]
fn the_wash_moves_only_when_the_reader_does() {
    let css = css();
    // The `html` rule itself — its own braces, not everything up to `body`,
    // which now has the keyframes and the guarded animation between them.
    let start = css.find("html {").expect("the root background rule");
    let wash = &css[start..start + css[start..].find('}').expect("its closing brace")];
    for forbidden in ["animation", "transition", "@keyframes"] {
        assert!(
            !wash.contains(forbidden),
            "the html rule itself still declares no motion:\n{wash}"
        );
    }

    // The one animation that does exist is scroll-driven, and is the only one.
    let n = css.matches("animation: wash-drift").count();
    assert_eq!(n, 1, "exactly one wash animation, declared once");
    let at = css
        .find("animation: wash-drift")
        .expect("the wash animation");
    let rule = &css[at..at + css[at..].find('}').expect("its rule") + 1];
    assert!(
        rule.contains("animation-timeline: scroll("),
        "it is driven by scroll position, not by a clock:\n{rule}"
    );
    for clock in ["infinite", "alternate", "s;", "ms;"] {
        assert!(
            !rule.contains(clock),
            "a wash animation must carry no duration or repeat ({clock}):\n{rule}"
        );
    }

    // And it is gated three ways: reduced motion, the reader's switch, and a
    // page that has declared itself a diagram.
    let guard_at = css[..at]
        .rfind("@media (prefers-reduced-motion: no-preference)")
        .expect("the reduced-motion guard wrapping it");
    let guard = &css[guard_at..at];
    assert!(
        guard.contains("@supports (animation-timeline: scroll())"),
        "…and a browser without scroll timelines gets the static wash:\n{guard}"
    );
    assert!(
        css[at - 200..at].contains(r#":not([data-wash="off"])"#),
        "…and the reader's switch turns the movement off with the wash"
    );
    assert!(
        css[at - 200..at].contains(r#":not([data-motion="still"])"#),
        "…and a page that declares itself a diagram keeps still"
    );
}

/// Turning it off leaves nothing behind, on any route.
///
/// The per-route rules are more specific than the plain `html` rule, so the
/// switch has to beat them too — a reader who turned the wash off and then
/// opened the tree must not get it back.
#[test]
fn the_wash_can_be_turned_off_on_every_route() {
    let css = css();
    assert!(
        css.contains(r#"[data-wash="off"] { background-image: none; }"#),
        "the plain case"
    );
    assert!(
        css.contains(r#"[data-wash="off"][data-route] { background-image: none; }"#),
        "and the routed one, which is otherwise the more specific selector"
    );
    // Every route that varies the wash is covered by that second rule.
    for route in ["tree", "convert", "admin"] {
        assert!(
            css.contains(&format!(r#"[data-route="{route}"] {{"#)),
            "{route} varies the wash and so must be beatable by the switch"
        );
    }
}

/// High-contrast carries wash tokens, and they are its own background.
///
/// The theme's `wash` flag is what actually stops the gradient being drawn —
/// the server never writes `data-wash="on"` for it. These values are the
/// second lock: if that flag were ever wrong, the gradient would still be the
/// background painted on the background, and the page would look the same.
#[test]
fn the_high_contrast_wash_would_be_invisible_even_if_it_were_drawn() {
    let css = css();
    let start = css
        .find(r#"[data-theme="high-contrast"] {"#)
        .expect("the high-contrast block");
    let end = css[start..].find("\n}").expect("its closing brace") + start;
    let block = &css[start..end];

    let bg = block
        .lines()
        .find_map(|l| l.trim().strip_prefix("--bg:"))
        .expect("--bg")
        .trim()
        .trim_end_matches(';')
        .to_string();
    for n in 1..=3 {
        let stop = block
            .lines()
            .find_map(|l| l.trim().strip_prefix(&format!("--wash-{n}:")))
            .unwrap_or_else(|| panic!("--wash-{n} is missing"))
            .trim()
            .trim_end_matches(';');
        assert_eq!(stop, bg, "--wash-{n} must be the background itself");
    }
}

/// A block inside a section is recessed, and carries no elevation of its own.
///
/// The three levels are page, section, block. A shadow reads as "nearer the
/// reader", so a block that sits *inside* a section and also appears nearer
/// than it inverts the containment — the eye reads that before it reads
/// anything the layout meant. The block is therefore dropped to the page's own
/// `--bg`, which turns it into a well in the section's surface, and its
/// `box-shadow` is explicitly `none` rather than merely unset.
#[test]
fn a_block_inside_a_section_is_recessed_rather_than_raised() {
    let css = css();
    let start = css
        .find(".rec-section .card,")
        .expect("the level-3 selector list");
    let block = &css[start..start + css[start..].find('}').expect("the rule") + 1];

    assert!(
        block.contains("background: var(--bg)"),
        "a block drops to the page's own background — recessed, not raised:\n{block}"
    );
    assert!(
        block.contains("box-shadow: none"),
        "…and gives up its elevation explicitly, so no other rule lends it one:\n{block}"
    );
    assert!(
        block.contains("border-radius: var(--radius)") && !block.contains("var(--radius-lg)"),
        "…and takes the smaller radius, which is the third signal:\n{block}"
    );
}

/// Section and card are one treatment, not two that happen to look alike.
///
/// A boundary has to look the same wherever it appears — a record section, an
/// admin section, the import report, the tree's controls — or the framing
/// teaches the reader nothing. That is enforced by them literally sharing a
/// rule rather than by matching declarations kept in step by hand.
#[test]
fn every_level_two_surface_is_declared_in_one_place() {
    let css = css();
    let start = css
        .find(".card,\n.rec-section,")
        .expect("the level-2 selector list");
    let head = &css[start..start + css[start..].find('{').expect("the brace")];
    for sel in [
        ".card",
        ".rec-section",
        ".tree-controls",
        ".stat",
        ".notice",
    ] {
        assert!(
            head.contains(sel),
            "{sel} is not in the shared level-2 rule, so its frame can drift \
             from every other frame in the interface:\n{head}"
        );
    }
    let body = &css[start..start + css[start..].find('}').expect("the rule") + 1];
    assert!(body.contains("background: var(--surface)"));
    assert!(body.contains("border: 1px solid var(--border)"));
    assert!(body.contains("box-shadow: 0 1px 2px"));
}

/// Destructive actions have a colour of their own, chosen to be read.
///
/// It used to borrow `--conf-low`, the lightest stop of the confidence ramp.
/// A ramp's lightest stop exists to be filled into a bar; set as text it put
/// the Delete label at 2.5:1 on the three colour-blind themes, against the
/// 4.5:1 WCAG AA asks. Every theme states its own value.
#[test]
fn the_danger_colour_is_not_the_low_confidence_fill() {
    let css = css();
    assert!(
        css.contains(".btn.danger { color: var(--danger)"),
        "the destructive button uses the token meant for reading"
    );
    for theme in THEME_SELECTORS {
        let vars = block_vars(&css, theme);
        assert!(
            vars.contains("--danger"),
            "{theme} does not define --danger and would inherit a value \
             chosen against a different background"
        );
    }
}
