//! Every theme is complete, and confidence never rests on colour.
//!
//! These are source-level checks. What a theme actually *looks* like is
//! measured in a browser (see the contrast and confidence sweeps described in
//! CONTRIBUTING.md); what is asserted here is the structural property that
//! makes those sweeps meaningful — that no theme is quietly inheriting another
//! theme's value for something it should define itself.

use std::collections::BTreeSet;

/// The stylesheet as a page loads it: the colours (`tokens.css`), then
/// `app.css`.
fn css() -> String {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let read = |f: &str| std::fs::read_to_string(root.join(f)).expect(f);
    read("static/css/tokens.css") + "\n" + &read("static/app.css")
}

/// `:root` as the interface draws with it: every `:root {` block — Study
/// 06's colours in `tokens.css`, the floor under every theme, and the rest of
/// `:root` in `app.css`.
fn root_block(css: &str) -> String {
    let mut out = String::new();
    let mut rest = css;
    while let Some(i) = rest.find("\n:root {") {
        let block = &rest[i + 1..];
        let end = block.find("\n}").expect("a closing brace");
        out.push_str(&block[..end + 2]);
        out.push('\n');
        rest = &block[end..];
    }
    out
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

/// The custom properties `:root` defines, across all its blocks.
fn root_vars(css: &str) -> BTreeSet<String> {
    root_block(css)
        .split("\n:root {")
        .flat_map(|b| {
            b.lines()
                .filter_map(|l| l.trim().strip_prefix("--"))
                .filter_map(|l| l.split(':').next())
                .map(|n| format!("--{n}"))
                .collect::<Vec<_>>()
        })
        .collect()
}

/// Every theme's own selector: the three since 0.5.0 (docs/adr/0008).
const THEME_SELECTORS: &[&str] = &[
    "[data-theme=\"canopee\"] {",
    "[data-theme=\"fusion\"] {",
    "[data-theme=\"galaxy\"] {",
];

#[test]
fn every_theme_on_offer_has_a_block_and_no_other_does() {
    let css = css();
    let offered: Vec<&str> = axgf_cms::theme::THEMES.iter().map(|t| t.id).collect();
    for (sel, id) in THEME_SELECTORS.iter().zip(&offered) {
        assert!(sel.contains(&format!("\"{id}\"")), "{sel} is not {id}");
    }
    assert_eq!(THEME_SELECTORS.len(), offered.len());
    // A palette for a theme nobody can choose is colour nobody measures.
    let mut rest = css.as_str();
    while let Some(i) = rest.find("[data-theme=\"") {
        let after = &rest[i + "[data-theme=\"".len()..];
        let id = &after[..after.find('"').expect("a closing quote")];
        assert!(
            offered.contains(&id),
            "a rule for a theme not on offer: {id}"
        );
        rest = after;
    }
}

#[test]
fn every_theme_redefines_every_colour_root_defines() {
    // A theme that misses one property inherits `:root`'s — which is Study
    // 06's. One forgotten line is then an element in another theme's colours
    // sitting in this one, and it will be found by a reader rather than by us.
    let css = css();
    // Geometry, typography and timing are the same in every theme by design —
    // a theme changes what the interface looks like, not how fast it moves or
    // how round its corners are — with one exception, written down: Canopée,
    // the reference theme, carries its own round and tempo (ADR 0004), and
    // redefines them under its own selector rather than here. Everything else
    // is a colour, and a theme that misses one inherits Study 06's.
    const NOT_A_COLOUR: &[&str] = &[
        // The presentation scale. A *style* owns these and a theme never
        // touches them; see `src/style.rs` for why the two axes are kept
        // apart and `no_style_block_names_a_colour` for the other half of
        // the guarantee.
        "--step",
        "--text",
        "--leading",
        "--measure",
        "--face",
        "--type-micro",
        "--type-label",
        "--type-small",
        "--type-minor",
        "--type-body",
        "--type-sub",
        "--type-major",
        "--type-title",
        "--type-display",
        // The river's rungs: the sizes its design fixes, a type scale like
        // the one above.
        "--type-rv-eyebrow",
        "--type-rv-data",
        "--type-rv-seg",
        "--type-rv-chip",
        "--type-rv-control",
        "--weight-normal",
        "--weight-medium",
        "--weight-semi",
        "--weight-strong",
        "--h1-size",
        "--h2-size",
        "--h3-size",
        "--h1-weight",
        "--h2-weight",
        "--label-size",
        "--label-track",
        "--small-size",
        "--section-pad",
        "--section-pad-x",
        "--section-gap",
        "--section-fill",
        "--section-shadow",
        "--section-border",
        "--section-radius",
        "--radius",
        "--radius-sm",
        "--radius-lg",
        // The round of a button and of a field (v0.5.0): Study 06's 2px on
        // `:root`, Canopée's own under its selector.
        "--btn-radius",
        "--field-radius",
        // The reference theme's headline, derived from the display rung.
        "--type-hero",
        "--mono",
        "--serif",
        "--tempo",
        "--ease",
        "--shadow-a",
        // Where the pinned tree column comes to rest, and the strip of room
        // kept under it for its horizontal scrollbar. Both are lengths: a
        // theme changes what the tree looks like, not where it stops.
        "--stick-top",
        "--hscroll-gutter",
    ];
    let root: BTreeSet<String> = root_vars(&css)
        .into_iter()
        .filter(|v| !NOT_A_COLOUR.contains(&v.as_str()))
        .collect();

    for sel in THEME_SELECTORS {
        let mine = block_vars(&css, sel);
        let missing: Vec<&String> = root.difference(&mine).collect();
        assert!(
            missing.is_empty(),
            "{sel} does not define {missing:?} and will inherit Study 06's"
        );
    }
}

#[test]
fn the_confidence_track_is_never_the_border_colour() {
    // The bar's track has to read as *empty*. It used to be `--border-hairline`, which
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
        "the track must have its own variable, not borrow --border-hairline:\n{bar}"
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
    // The last theme block in tokens.css, which the stylesheet loads first.
    let themes_end = css
        .find("[data-theme=\"galaxy\"] {")
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

/// A block inside a section is recessed, and carries no elevation of its own.
///
/// The three levels are page, section, block. A shadow reads as "nearer the
/// reader", so a block that sits *inside* a section and also appears nearer
/// than it inverts the containment — the eye reads that before it reads
/// anything the layout meant. The block is therefore dropped to the page's own
/// `--surface-page`, which turns it into a well in the section's surface, and its
/// `box-shadow` is explicitly `none` rather than merely unset.
#[test]
fn a_block_inside_a_section_is_recessed_rather_than_raised() {
    let css = css();
    let start = css
        .find(".rec-section .card,")
        .expect("the level-3 selector list");
    let block = &css[start..start + css[start..].find('}').expect("the rule") + 1];

    assert!(
        block.contains("background: var(--surface-page)"),
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
    // The frame is one declaration per property, and each now names a token
    // the *style* axis owns rather than a literal — so a style can turn a card
    // into a ruled block without touching the colour the theme put in it.
    let body = &css[start..start + css[start..].find('}').expect("the rule") + 1];
    assert!(body.contains("background: var(--section-fill)"));
    assert!(body.contains("border: var(--section-border)"));
    assert!(body.contains("box-shadow: var(--section-shadow)"));

    // And the defaults those tokens carry are the values this rule used to
    // hold literally, so the comfortable style is the interface unchanged.
    let root = root_block(&css);
    for want in [
        "--section-fill: var(--surface-raised)",
        "--section-border: 1px solid var(--border-hairline)",
        "--section-shadow: 0 1px 2px",
    ] {
        assert!(
            root.contains(want),
            "the default for the level-2 frame moved: {want}"
        );
    }
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
        css.contains(".btn.danger { border-color: var(--danger); color: var(--danger); }"),
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

// ---------------------------------------------------------------------------
// The second axis
// ---------------------------------------------------------------------------

/// Every style block defines every value the scale has.
///
/// The mirror of `every_theme_redefines_every_colour_root_defines`, and it
/// exists for the same reason: a style that misses one value inherits the
/// default's, and one forgotten line is a compact page with a comfortable
/// heading on it, found by a reader rather than by us.
#[test]
fn every_style_redefines_every_value_the_scale_has() {
    let css = css();
    // These are the ones a style is expected to retune. `--section-radius`
    // and friends are on the list; `--radius` itself is not, because it is
    // the theme-independent geometry every style shares.
    //
    // The nine `--type-*` rungs replaced `--h1-size` and friends, which are now
    // *names for rungs* defined once on `:root` — a style retunes the scale, not
    // each heading separately, which is what stops a style inventing a fourth
    // size between two rungs. `--h1-weight` and `--h2-weight` stay on the list:
    // paper genuinely wants a lighter heading, because a serif at 2.1rem does
    // not need 700 to carry.
    const SCALE: &[&str] = &[
        "--step",
        "--text",
        "--leading",
        "--measure",
        "--face",
        "--type-micro",
        "--type-label",
        "--type-small",
        "--type-minor",
        "--type-body",
        "--type-sub",
        "--type-major",
        "--type-title",
        "--type-display",
        "--h1-weight",
        "--h2-weight",
        "--label-track",
        "--section-pad",
        // The inline half of it, which a table bleeding to its section's
        // edges bleeds by (v0.5.0): a style that forgot it would bleed by
        // the default's and cross its own card's edge.
        "--section-pad-x",
        "--section-gap",
        "--section-fill",
        "--section-shadow",
        "--section-border",
        "--section-radius",
    ];
    let root = root_vars(&css);
    for v in SCALE {
        assert!(
            root.contains(*v),
            "{v} is on the scale but `:root` does not define it"
        );
    }
    // `comfortable` is the values on :root and has no block.
    for style in axgf_cms::style::STYLES
        .iter()
        .filter(|s| s.id != "comfortable")
    {
        let sel = format!("[data-style=\"{}\"] {{", style.id);
        let mine = block_vars(&css, &sel);
        let missing: Vec<&&str> = SCALE.iter().filter(|v| !mine.contains(**v)).collect();
        assert!(
            missing.is_empty(),
            "{sel} does not define {missing:?} and will inherit the default"
        );
    }
}

/// A style may change a size, a space, a weight or a border. Never a colour.
///
/// This is the load-bearing half of keeping nine combinations to three
/// palettes. A hue in one of these blocks would be a colour nobody measured,
/// under three themes, and would not show up in the theme sweep because the
/// theme sweep does not know styles exist.
#[test]
fn no_style_block_names_a_colour() {
    let css = css();
    for style in axgf_cms::style::STYLES {
        let sel = format!("[data-style=\"{}\"] {{", style.id);
        let Some(start) = css.find(&sel) else {
            continue;
        };
        let open = css[start..].find('{').expect("a block") + start;
        let end = css[open..].find("\n}").expect("a closing brace") + open;
        let body = &css[open..end];
        // References to the theme's own tokens are the point — `var(--surface-raised)`
        // is how a style stays out of the palette's business — so they are
        // removed before looking. What must not survive is a colour *invented*
        // here: a hex, an rgb() or an hsl() of its own.
        let mut stripped = String::with_capacity(body.len());
        let mut rest = body;
        while let Some(at) = rest.find("var(") {
            stripped.push_str(&rest[..at]);
            match rest[at..].find(')') {
                Some(close) => rest = &rest[at + close + 1..],
                None => break,
            }
        }
        stripped.push_str(rest);
        // An *invented* colour is a hex or a function with numbers in it.
        // `rgb(var(--shadow-rgb) / var(--shadow-a))` survives the strip above
        // as `rgb( / )`, which names nothing and is the house idiom for
        // composing a shadow out of the theme's own two values.
        let invents = |marker: &str| {
            stripped.match_indices(marker).any(|(at, _)| {
                stripped[at + marker.len()..]
                    .trim_start()
                    .starts_with(|c: char| c.is_ascii_hexdigit())
            })
        };
        for marker in ["#", "rgb(", "rgba(", "hsl(", "oklch("] {
            assert!(
                !invents(marker),
                "{sel} invents a colour ({marker:?}): a style changes geometry, \
                 not palette. Reference the theme's token instead."
            );
        }
    }
}

/// Every size on the page is a rung of the scale, not a number somebody typed.
///
/// The scale existed before this test and almost nothing used it: 71 of the
/// 101 `font-size` declarations set a literal, across 29 distinct values —
/// 0.93, 0.94 and 0.95rem all appeared, and so did 0.72, 0.74 and 0.75.
/// Differences that small are invisible as hierarchy and perfectly visible as
/// inconsistency, which is the whole of why headings, labels and values read
/// at one weight.
#[test]
fn every_font_size_outside_the_scale_is_a_rung_of_it() {
    let css = css();
    let src = strip_comments(&css);

    // Where the rungs themselves are declared, a literal is the point.
    let decl = src
        .find("--type-micro:")
        .expect("the scale is declared somewhere");
    let layout = src
        .find("/* ---------- layout ---------- */")
        .unwrap_or(decl);
    let _ = layout;

    let mut offenders = Vec::new();
    for (i, line) in src.lines().enumerate() {
        let Some(at) = line.find("font-size:") else {
            continue;
        };
        let value = &line[at + "font-size:".len()..];
        let value = value.split(';').next().unwrap_or(value).trim();
        // A rung, or a length relative to the parent — `code` at 0.9em is
        // deliberately 0.9 of whatever it sits in, which no rung can express.
        if value.starts_with("var(--") || value.ends_with("em") && !value.ends_with("rem") {
            continue;
        }
        // The scale's own declarations.
        if line.contains("--type-") {
            continue;
        }
        offenders.push(format!("line {}: {}", i + 1, line.trim()));
    }
    assert!(
        offenders.is_empty(),
        "these set a size by hand instead of naming a rung:\n{}",
        offenders.join("\n")
    );
}

/// And every weight is one of the four, not a number that rounds differently
/// per font. `650` was in this file and is not a weight most faces have.
#[test]
fn every_font_weight_is_one_of_the_four() {
    let src = strip_comments(&css());
    let mut offenders = Vec::new();
    for (i, line) in src.lines().enumerate() {
        let Some(at) = line.find("font-weight:") else {
            continue;
        };
        let value = &line[at + "font-weight:".len()..];
        let value = value.split(';').next().unwrap_or(value).trim();
        if value.starts_with("var(--") || line.contains("--weight-") {
            continue;
        }
        offenders.push(format!("line {}: {}", i + 1, line.trim()));
    }
    assert!(
        offenders.is_empty(),
        "these set a weight by hand instead of naming one of the four:\n{}",
        offenders.join("\n")
    );
}

/// The system has no say (v0.5.0, docs/adr/0008): every theme is dark, so
/// no `prefers-color-scheme` query is left, and a page with no theme at all
/// is Study 06, which `:root` carries.
#[test]
fn the_system_has_no_say_and_a_page_without_a_theme_is_dark() {
    let css = strip_comments(&css());
    assert!(
        !css.contains("prefers-color-scheme"),
        "no colour-scheme query: no theme follows the system"
    );
    let root = &css[css.find("\n:root {").expect(":root")..];
    let root = &root[..root.find("\n}").expect("its end")];
    assert!(
        root.contains("color-scheme: dark;") && root.contains("--surface-page: #0b110e;"),
        "a page with no data-theme is Study 06"
    );
}

/// A texture is the page's, never a control's (v0.4.3; docs/adr/0002 §8).
#[test]
fn no_texture_is_drawn_on_a_control() {
    let css = css();
    let uses: Vec<&str> = css
        .match_indices("var(--wash-texture)")
        .map(|(i, _)| {
            let rule = css[..i].rfind('}').map_or(0, |j| j + 1);
            css[rule..i].trim_start()
        })
        .collect();
    assert!(!uses.is_empty(), "the texture is drawn somewhere");
    for u in uses {
        let sel = u.split('{').next().unwrap_or("").trim();
        let sel = sel.rsplit("*/").next().unwrap_or(sel).trim();
        assert!(
            sel == "html" || sel.starts_with("[data-route="),
            "the texture reaches something that is not the page: {sel}"
        );
    }
}
