//! A figure drawn from what the record states, and from nothing else.
//!
//! # What it is not
//!
//! It is not a portrait, and it does not derive from the photograph. Building a
//! face out of a scan would invent a likeness, and an invented face standing
//! beside sourced facts is the exact opposite of what this application argues
//! for. The photograph, where there is one, stays in the avatar untouched. This
//! figure has no face, no hair, no clothing and no period: every one of those
//! would be a claim the record does not make.
//!
//! # What it encodes
//!
//! Three things, all of them recorded:
//!
//! * **Age band**, from the birth and death dates, as *proportions*. An infant
//!   is about four head-heights tall, a young child six, an adolescent seven,
//!   an adult seven and a half. Those ratios are why an infant reads as an
//!   infant at a glance and without a caption — the head does the telling, not
//!   a label and not a colour.
//! * **Height**, when the record states one, drawn to scale against a fixed
//!   reference line so that two people can be held side by side and compared.
//!   With no height recorded the figure is drawn at a nominal size for its band
//!   and the caption *says so*, because a figure silently drawn at an average
//!   height is a measurement nobody took.
//! * **Build**, when the record states one, as the width of the torso.
//!
//! Nothing else. No colour carries meaning either, which is what makes it
//! readable under the three colour-blind themes: it is one flat shape in one
//! theme colour, and everything it says it says with geometry.
//!
//! # Five bands, four figures
//!
//! [`Band::Elderly`] draws the same figure as [`Band::Adult`], and that is
//! deliberate rather than unfinished. The proportion that separates the other
//! bands — how much of a person is head — stops changing at about twenty, so
//! there is no honest proportional difference to draw. What does change with
//! age is stature, and stature is *height*: if the record states a height the
//! figure already shows it, and if it does not then drawing an elderly person
//! shorter would be inventing the measurement this module exists to avoid
//! inventing. A stoop would be worse still — posture is not in the record. The
//! caption names the band; the drawing says only what a drawing can.
//!
//! # Not enough to draw
//!
//! With no age derivable there is no figure at all — not a default adult. On
//! the operator's bundle that is 388 people of 866, and a generic figure
//! standing in for each of them would be a false statement made in pictures,
//! which is harder to argue with than one made in words and therefore worse.

use serde::Serialize;

/// The reference figure: a person of this height reaches the reference line.
///
/// A constant rather than a per-bundle average, so the same recorded height
/// produces the same figure in every bundle and two people compared across two
/// screens are actually comparable.
pub const REFERENCE_CM: u32 = 170;

/// Bounds of the drawing, in user units. Fixed for every person, because a
/// comparison between two figures is only a comparison if the canvas does not
/// move between them.
const W: f64 = 120.0;
const H: f64 = 260.0;
/// The ground the figure stands on.
const BASE: f64 = 250.0;
/// Height in user units of a [`REFERENCE_CM`] figure.
const REF_PX: f64 = 185.0;

/// The shortest and tallest recorded heights this canvas can draw to scale.
///
/// Outside them the figure is drawn at its band's nominal size and the caption
/// says it is not to scale. Clamping instead would put a figure at the top of
/// the canvas and let the reader believe the height had been drawn — a wrong
/// measurement stated confidently, which is worse than an absent one. The
/// editor accepts up to 400 cm because a record may contain a transcription
/// error and refusing to store it does not make it go away; drawing it is a
/// separate question, and this is the answer to that one.
const MIN_TO_SCALE_CM: u32 = 30;
const MAX_TO_SCALE_CM: u32 = 220;

/// How old the record says they were, in bands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Band {
    Infant,
    Child,
    Adolescent,
    Adult,
    Elderly,
}

/// The proportions of one band, in head-heights measured from the crown.
///
/// One table rather than a dozen conditionals: these are the numbers a
/// life-drawing manual uses, they are the whole of what distinguishes one band
/// from another, and having them in one place is what makes the difference
/// between the bands readable as data rather than as code.
#[derive(Debug, Clone, Copy)]
struct Canon {
    /// Total height in head-heights. The number that does the work.
    heads: f64,
    /// Shoulder line, waist, hip line, and where the hands stop.
    shoulder_y: f64,
    waist_y: f64,
    hip_y: f64,
    arm_end_y: f64,
    /// Half-width of the head, as a fraction of one head-height: a head is
    /// taller than it is wide, and rounder on an infant.
    head_w: f64,
    /// Widths across, in head-heights, before build is applied.
    ///
    /// These are the *torso*, not the span across the shoulders: the arms hang
    /// outside it and the span is the sum. Drawing the torso the full shoulder
    /// span instead buried both arms inside the body, which is how the first
    /// version of this came out — a slab with a head on it.
    shoulder_w: f64,
    waist_w: f64,
    hip_w: f64,
    leg_w: f64,
    arm_w: f64,
    /// Drawn height as a fraction of the reference, when no height is
    /// recorded. Deliberately *not* presented as a measurement: the caller
    /// marks the figure unscaled and the caption says so. These are chosen so
    /// the bands are visibly different, not from centile data.
    nominal: f64,
}

const INFANT: Canon = Canon {
    heads: 4.0,
    shoulder_y: 1.15,
    waist_y: 1.80,
    hip_y: 2.35,
    arm_end_y: 2.80,
    head_w: 0.44,
    shoulder_w: 0.70,
    waist_w: 0.68,
    hip_w: 0.70,
    leg_w: 0.40,
    arm_w: 0.26,
    nominal: 0.32,
};

const CHILD: Canon = Canon {
    heads: 6.0,
    shoulder_y: 1.30,
    waist_y: 2.35,
    hip_y: 3.15,
    arm_end_y: 3.75,
    head_w: 0.40,
    shoulder_w: 0.80,
    waist_w: 0.72,
    hip_w: 0.74,
    leg_w: 0.36,
    arm_w: 0.22,
    nominal: 0.56,
};

const ADOLESCENT: Canon = Canon {
    heads: 7.0,
    shoulder_y: 1.35,
    waist_y: 2.60,
    hip_y: 3.60,
    arm_end_y: 4.25,
    head_w: 0.38,
    shoulder_w: 0.94,
    waist_w: 0.80,
    hip_w: 0.84,
    leg_w: 0.36,
    arm_w: 0.22,
    nominal: 0.84,
};

const ADULT: Canon = Canon {
    heads: 7.5,
    shoulder_y: 1.40,
    waist_y: 2.75,
    hip_y: 3.80,
    arm_end_y: 4.50,
    head_w: 0.37,
    shoulder_w: 1.05,
    waist_w: 0.88,
    hip_w: 0.92,
    leg_w: 0.38,
    arm_w: 0.24,
    nominal: 1.0,
};

impl Band {
    /// The band an age in whole years falls in.
    pub fn of(age: i64) -> Band {
        match age {
            ..=2 => Band::Infant,
            3..=9 => Band::Child,
            10..=17 => Band::Adolescent,
            18..=64 => Band::Adult,
            _ => Band::Elderly,
        }
    }

    fn canon(self) -> Canon {
        match self {
            Band::Infant => INFANT,
            Band::Child => CHILD,
            Band::Adolescent => ADOLESCENT,
            // See the module docs: the head-to-body ratio does not change
            // after twenty, so there is nothing honest to draw differently.
            Band::Adult | Band::Elderly => ADULT,
        }
    }

    /// The slug used in the catalogue key and in the figure's `data-band`.
    pub fn slug(self) -> &'static str {
        match self {
            Band::Infant => "infant",
            Band::Child => "child",
            Band::Adolescent => "adolescent",
            Band::Adult => "adult",
            Band::Elderly => "elderly",
        }
    }

    /// The catalogue key for the sentence describing this band's proportions.
    pub fn key(self) -> String {
        format!("silhouette-proportions-{}", self.slug())
    }
}

/// Every band, for the catalogue completeness test.
pub const BANDS: &[Band] = &[
    Band::Infant,
    Band::Child,
    Band::Adolescent,
    Band::Adult,
    Band::Elderly,
];

/// The vocabulary term for a build, as a torso-width multiplier.
///
/// The terms are [`crate::physical::BUILDS`]. A word this build does not know
/// draws an unmodified figure rather than guessing at where it falls.
fn build_width(build: Option<&str>) -> f64 {
    match build {
        Some("slight") => 0.80,
        Some("slim") => 0.90,
        Some("sturdy") => 1.10,
        Some("stout") => 1.22,
        Some("heavy") => 1.35,
        // "average", an unrecognised word, or nothing recorded at all.
        _ => 1.0,
    }
}

/// A drawn figure and the facts it stands on.
#[derive(Debug, Clone, Serialize)]
pub struct Silhouette {
    pub svg: String,
    pub band: Band,
    pub band_key: String,
    pub band_slug: &'static str,
    /// True when the height is a recorded measurement drawn to scale, false
    /// when the figure is at a nominal size for its band.
    pub to_scale: bool,
    /// The height actually drawn, present only when it was drawn to scale.
    pub height_cm: Option<u32>,
    pub reference_cm: u32,
}

/// Draw the figure, or decline to.
///
/// `None` when there is no age to band the figure by. See the module docs for
/// why that is not a default adult.
pub fn draw(age: Option<i64>, height_cm: Option<u32>, build: Option<&str>) -> Option<Silhouette> {
    let band = Band::of(age?);
    let c = band.canon();

    let (figure_px, to_scale) = match height_cm {
        Some(cm) if (MIN_TO_SCALE_CM..=MAX_TO_SCALE_CM).contains(&cm) => {
            (f64::from(cm) / f64::from(REFERENCE_CM) * REF_PX, true)
        }
        _ => (c.nominal * REF_PX, false),
    };

    let head_d = figure_px / c.heads;
    let top = BASE - figure_px;
    let cx = W / 2.0;
    let bw = build_width(build);
    // A build widens a torso. It moves a limb far less, and moves a head not
    // at all: how much of a person is head is proportion, not build.
    let lw = bw.clamp(0.9, 1.2);

    // Everything below is in head units from the crown, so a change to a band
    // is a change to one row of the canon rather than to a dozen coordinates.
    let y = |heads_down: f64| top + head_d * heads_down;
    let shoulder = head_d * c.shoulder_w * bw;
    let waist = head_d * c.waist_w * bw;
    let hip = head_d * c.hip_w * bw;
    let neck = head_d * c.head_w * 0.8;
    let leg_w = head_d * c.leg_w * lw;
    let arm_w = head_d * c.arm_w * lw;

    // Every sub-path is wound the same way — clockwise on screen — so that
    // where two of them overlap the non-zero fill rule unions them instead of
    // punching a hole through the figure.
    let mut parts = String::with_capacity(640);

    // The head, as four quarter-arcs rather than two halves, so that the top
    // of the skull is an endpoint the bounds test can see rather than a
    // property of an arc it would have to re-derive.
    let (rx, ry) = (head_d * c.head_w, head_d * 0.5);
    let cy = top + ry;
    parts.push_str(&path(&format!(
        "M {} {} A {rx} {ry} 0 0 1 {} {} A {rx} {ry} 0 0 1 {} {} \
         A {rx} {ry} 0 0 1 {} {} A {rx} {ry} 0 0 1 {} {} Z",
        n(cx),
        n(cy - ry),
        n(cx + rx),
        n(cy),
        n(cx),
        n(cy + ry),
        n(cx - rx),
        n(cy),
        n(cx),
        n(cy - ry),
        rx = n(rx),
        ry = n(ry),
    )));

    // Neck and torso in one shape: a collar at the jaw, out to the shoulders,
    // tapering to the hips.
    parts.push_str(&quad_path(&[
        (cx - neck / 2.0, y(0.88)),
        (cx + neck / 2.0, y(0.88)),
        (cx + shoulder / 2.0, y(c.shoulder_y)),
        (cx + waist / 2.0, y(c.waist_y)),
        (cx + hip / 2.0, y(c.hip_y)),
        (cx - hip / 2.0, y(c.hip_y)),
        (cx - waist / 2.0, y(c.waist_y)),
        (cx - shoulder / 2.0, y(c.shoulder_y)),
    ]));

    // Two legs, from the hips to the ground, tapering to the ankle. They start
    // a little above the hip line and the arms a little above the shoulder,
    // inside the torso where it cannot be seen: two shapes that meet exactly
    // leave an antialiased hairline between them, and a seam across a
    // one-colour figure reads as a drawing mistake.
    for side in [-1.0_f64, 1.0] {
        let at = cx + side * hip * 0.26;
        let ankle = leg_w * 0.68;
        parts.push_str(&quad_path(&[
            (at - leg_w / 2.0, y(c.hip_y) - head_d * 0.2),
            (at + leg_w / 2.0, y(c.hip_y) - head_d * 0.2),
            (at + ankle / 2.0, BASE),
            (at - ankle / 2.0, BASE),
        ]));
    }

    // Two arms, hanging outside the torso and overlapping it just enough to
    // attach. The span across the shoulders is the torso plus both arms, which
    // is how a shoulder is actually measured.
    for side in [-1.0_f64, 1.0] {
        let at = cx + side * (shoulder / 2.0 + arm_w * 0.4);
        let wrist = arm_w * 0.74;
        parts.push_str(&quad_path(&[
            (at - arm_w / 2.0, y(c.shoulder_y) - head_d * 0.06),
            (at + arm_w / 2.0, y(c.shoulder_y) - head_d * 0.06),
            (at + wrist / 2.0, y(c.arm_end_y)),
            (at - wrist / 2.0, y(c.arm_end_y)),
        ]));
    }

    // The reference line is what makes two figures comparable, and it is drawn
    // only when this one is actually to scale: a scale mark beside a nominal
    // figure would be a measurement claim.
    let reference = if to_scale {
        format!(
            "<line x1=\"8\" y1=\"{ry}\" x2=\"{x2}\" y2=\"{ry}\" class=\"sil-ref\"/>",
            ry = n(BASE - REF_PX),
            x2 = n(W - 8.0),
        )
    } else {
        String::new()
    };

    // `aria-hidden`, and not a `role="img"` with a label: everything the
    // figure states is stated again in the caption beside it, in a sentence,
    // and a screen reader should hear that once rather than twice.
    let svg = format!(
        "<svg class=\"silhouette\" viewBox=\"0 0 {w} {h}\" width=\"{w}\" height=\"{h}\" \
         data-band=\"{band}\" aria-hidden=\"true\" focusable=\"false\">\
         <line x1=\"8\" y1=\"{base}\" x2=\"{x2}\" y2=\"{base}\" class=\"sil-ground\"/>\
         {reference}<g class=\"sil-body\">{parts}</g></svg>",
        w = n(W),
        h = n(H),
        band = band.slug(),
        base = n(BASE),
        x2 = n(W - 8.0),
    );

    Some(Silhouette {
        svg,
        band,
        band_key: band.key(),
        band_slug: band.slug(),
        to_scale,
        height_cm: to_scale.then(|| height_cm.unwrap_or_default()),
        reference_cm: REFERENCE_CM,
    })
}

/// One `<path>` per part, all inheriting the group's single fill.
///
/// Separate elements rather than one path with many sub-paths: the parts
/// overlap on purpose — a neck into a jaw, an arm into a shoulder — and
/// separate elements cannot interact through a fill rule at all, so no future
/// edit can accidentally punch a hole in the figure by winding a shape the
/// other way round.
fn path(d: &str) -> String {
    format!("<path d=\"{d}\"/>")
}

/// A closed polygon through the given points, in the order given.
fn quad_path(points: &[(f64, f64)]) -> String {
    let mut d = String::with_capacity(96);
    for (i, (x, y)) in points.iter().enumerate() {
        d.push_str(if i == 0 { "M " } else { "L " });
        d.push_str(&n(*x));
        d.push(' ');
        d.push_str(&n(*y));
        d.push(' ');
    }
    d.push('Z');
    path(&d)
}

/// One decimal place, without a trailing `.0`.
///
/// SVG path data does not need more, and a shorter `d` is a smaller page on
/// every person in the bundle.
fn n(v: f64) -> String {
    let r = (v * 10.0).round() / 10.0;
    // -0 is a real f64 and formats as "-0", which is valid path data and still
    // reads as a defect to anyone looking at the markup.
    if r == 0.0 {
        return "0".to_string();
    }
    if r == r.trunc() {
        format!("{}", r as i64)
    } else {
        format!("{r}")
    }
}

// ---------------------------------------------------------------------------
// rendering
// ---------------------------------------------------------------------------

/// The figure and its caption, as the reader gets them.
///
/// The caption is assembled by the template out of whole translated sentences
/// rather than here out of fragments: a sentence built in Rust escapes the
/// hardcoded-string test, and a vocabulary term dropped into the middle of one
/// needs a case ending in Polish that English does not have.
#[derive(Debug, Clone, Serialize)]
pub struct View {
    /// The `<svg>` element, rendered into the page as trusted markup. It
    /// contains no data from the bundle — only numbers this module computed.
    pub svg: String,
    pub band_key: String,
    pub band_slug: &'static str,
    pub to_scale: bool,
    pub height_cm: Option<u32>,
    /// When the drawn height was measured, in the reader's language.
    pub height_date: Option<crate::view::DateDisplay>,
    /// True when more than one height is recorded, so the caption can say
    /// which of them the figure draws.
    pub several_heights: bool,
    /// The recorded build, said in the reader's language.
    pub build: Option<String>,
    pub reference_cm: u32,
}

/// Build the figure for one person, or `None` when the record cannot support
/// one.
///
/// `age` is the same number the masthead prints — age at death, or age now for
/// somebody living — so the figure and the heading beside it can never
/// disagree about how old this person was.
///
/// Health is not consulted and cannot be: height, weight and build live in
/// [`crate::physical::Group::Traits`], which is not special-category data and
/// follows the record's own visibility. A figure that changed shape when an
/// administrator signed in would be a health disclosure drawn as a picture.
pub fn view_for(age: Option<i64>, detail: &crate::physical::Detail, lang: &str) -> Option<View> {
    let height = detail
        .latest("height_cm")
        .and_then(|e| e.value.trim().parse::<u32>().ok());
    let build_term = detail.latest("build").map(|e| e.value.clone());

    let s = draw(age, height, build_term.as_deref())?;

    let height_date = s.height_cm.and(detail.latest("height_cm")).and_then(|e| {
        (!e.date.trim().is_empty()).then(|| {
            crate::view::render_date_in(
                &serde_json::json!({"value": e.date, "precision": precision_of(&e.date)}),
                lang,
            )
        })
    });

    let build = build_term
        .as_deref()
        .filter(|t| crate::physical::BUILDS.contains(t))
        .map(|t| crate::i18n::translate(lang, &format!("phys-build-{t}"), None));

    Some(View {
        svg: s.svg,
        band_key: s.band_key,
        band_slug: s.band_slug,
        to_scale: s.to_scale,
        height_cm: s.height_cm,
        height_date,
        several_heights: detail.count("height_cm") > 1,
        build,
        reference_cm: s.reference_cm,
    })
}

/// The specification's precision word for a date written as recorded.
///
/// The same rule [`crate::physical`] applies on the way in, applied again on
/// the way out so the entry's date is drawn by the same renderer as every
/// other date on the page.
fn precision_of(date: &str) -> &'static str {
    match date.chars().filter(|c| *c == '-').count() {
        0 => "year",
        1 => "month",
        _ => "day",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every coordinate the figure actually names, from its path data.
    ///
    /// A parser rather than a sweep for anything that looks like a number:
    /// the sweep also picked up the viewBox, the arc flags and the radii, and
    /// a bounds check that includes the canvas it is checking against is not a
    /// bounds check.
    fn points(svg: &str) -> Vec<(f64, f64)> {
        let mut out = Vec::new();
        for chunk in svg.split(" d=\"").skip(1) {
            let d = chunk.split('"').next().expect("a path");
            let mut tokens = d.split_whitespace().peekable();
            while let Some(tok) = tokens.next() {
                let take = match tok {
                    "M" | "L" => 0,
                    // rx ry rotation large-arc sweep, then the endpoint.
                    "A" => 5,
                    _ => continue,
                };
                for _ in 0..take {
                    tokens.next();
                }
                let x: f64 = tokens.next().expect("x").parse().expect("a number");
                let y: f64 = tokens.next().expect("y").parse().expect("a number");
                out.push((x, y));
            }
        }
        assert!(!out.is_empty(), "no path data in {svg}");
        out
    }

    /// The topmost point of the figure: the crown of the head.
    fn crown(s: &Silhouette) -> f64 {
        points(&s.svg).iter().map(|p| p.1).fold(f64::MAX, f64::min)
    }

    /// The head's radius, which is what carries the band.
    fn head_radius(s: &Silhouette) -> f64 {
        let d = s.svg.split(" d=\"").nth(1).expect("the head");
        d.split(" A ")
            .nth(1)
            .expect("an arc")
            .split_whitespace()
            .next()
            .expect("rx")
            .parse()
            .expect("a number")
    }

    #[test]
    fn no_age_means_no_figure() {
        // 388 of the operator's 866 people have no dates at all. A generic
        // adult standing in for each of them would be a false statement made
        // in pictures.
        assert!(draw(None, None, None).is_none());
        assert!(draw(None, Some(180), Some("slim")).is_none());
    }

    #[test]
    fn the_bands_fall_where_the_ages_do() {
        for (age, want) in [
            (0, Band::Infant),
            (2, Band::Infant),
            (3, Band::Child),
            (9, Band::Child),
            (10, Band::Adolescent),
            (17, Band::Adolescent),
            (18, Band::Adult),
            (64, Band::Adult),
            (65, Band::Elderly),
            (101, Band::Elderly),
        ] {
            assert_eq!(Band::of(age), want, "age {age}");
        }
    }

    #[test]
    fn the_head_to_body_ratio_is_what_makes_a_band_legible() {
        assert_eq!(INFANT.heads, 4.0);
        assert_eq!(CHILD.heads, 6.0);
        assert_eq!(ADULT.heads, 7.5);

        // Drawn at the *same overall height*, an infant's head is nearly twice
        // an adult's. That is the whole mechanism, and it is measurable here
        // rather than a matter of looking at the picture.
        let infant = draw(Some(1), Some(170), None).expect("drawn");
        let adult = draw(Some(40), Some(170), None).expect("drawn");
        let (hi, ha) = (head_radius(&infant), head_radius(&adult));
        assert!(hi > ha * 1.7, "infant {hi} against adult {ha}");
    }

    #[test]
    fn the_elderly_band_draws_an_adult_and_says_why() {
        // Documented, not overlooked: see the module docs. A stoop or a
        // shortened figure would be posture or a measurement, and the record
        // states neither.
        let adult = draw(Some(40), Some(170), None).expect("drawn");
        let elderly = draw(Some(80), Some(170), None).expect("drawn");
        assert_eq!(
            adult.svg.replace("adult", "elderly"),
            elderly.svg,
            "the figures differ only in the band they name"
        );
        assert_ne!(adult.band_key, elderly.band_key);
    }

    #[test]
    fn a_recorded_height_is_drawn_to_scale_and_an_absent_one_says_so() {
        let tall = draw(Some(30), Some(190), None).expect("drawn");
        let short = draw(Some(30), Some(150), None).expect("drawn");
        assert!(tall.to_scale && short.to_scale);
        assert_eq!(tall.height_cm, Some(190));
        assert!(
            crown(&tall) < crown(&short),
            "the taller figure starts higher up the canvas"
        );
        assert!(tall.svg.contains("sil-ref"), "a scale mark, being to scale");

        let unknown = draw(Some(30), None, None).expect("drawn");
        assert!(!unknown.to_scale);
        assert_eq!(unknown.height_cm, None);
        assert!(
            !unknown.svg.contains("sil-ref"),
            "no scale mark beside a figure that is not to scale"
        );
    }

    #[test]
    fn two_people_are_comparable_because_the_scale_is_one_constant() {
        // The point of the reference: the same centimetre is the same number
        // of user units on every page, so two figures on two screens can be
        // held side by side.
        let a = draw(Some(40), Some(160), None).expect("drawn");
        let b = draw(Some(40), Some(180), None).expect("drawn");
        let (ha, hb) = (BASE - crown(&a), BASE - crown(&b));
        assert!(
            ((hb / ha) - (180.0 / 160.0)).abs() < 0.01,
            "drawn heights {ha} and {hb} are not in the recorded ratio"
        );
        // And the reference line sits at the same place on both.
        assert!(a.svg.contains("sil-ref") && b.svg.contains("sil-ref"));
    }

    #[test]
    fn an_undrawable_height_falls_back_rather_than_lying_about_the_scale() {
        // The editor stores up to 400 cm because a record may contain a
        // transcription error. Drawing one clamped to the canvas would state a
        // measurement that is not the one recorded.
        for cm in [1u32, 29, 221, 400] {
            let s = draw(Some(40), Some(cm), None).expect("drawn");
            assert!(!s.to_scale, "{cm} cm is not a height this can draw");
            assert_eq!(s.height_cm, None);
        }
        for cm in [MIN_TO_SCALE_CM, 170, MAX_TO_SCALE_CM] {
            assert!(draw(Some(40), Some(cm), None).expect("drawn").to_scale);
        }
    }

    #[test]
    fn build_changes_the_width_and_leaves_the_height_alone() {
        let slight = draw(Some(40), Some(170), Some("slight")).expect("drawn");
        let heavy = draw(Some(40), Some(170), Some("heavy")).expect("drawn");
        assert_ne!(slight.svg, heavy.svg);
        assert_eq!(crown(&slight), crown(&heavy), "build is not height");

        let widest = |s: &Silhouette| {
            points(&s.svg)
                .iter()
                .map(|p| (p.0 - W / 2.0).abs())
                .fold(0.0, f64::max)
        };
        assert!(widest(&heavy) > widest(&slight) * 1.4);

        // A word the vocabulary does not contain draws an unmodified figure
        // rather than a guess at where it falls.
        let plain = draw(Some(40), Some(170), None).expect("drawn");
        let average = draw(Some(40), Some(170), Some("average")).expect("drawn");
        let unknown = draw(Some(40), Some(170), Some("wiry")).expect("drawn");
        assert_eq!(plain.svg, average.svg);
        assert_eq!(plain.svg, unknown.svg);
    }

    #[test]
    fn every_build_in_the_vocabulary_draws_a_distinct_width() {
        let mut seen: Vec<(String, f64)> = Vec::new();
        for term in crate::physical::BUILDS {
            let w = build_width(Some(term));
            assert!(
                (0.7..=1.5).contains(&w),
                "{term} is outside anything a torso can be"
            );
            seen.push(((*term).to_string(), w));
        }
        // The list is ordered from slightest to heaviest, and the drawing
        // must agree with the order the vocabulary states.
        for pair in seen.windows(2) {
            assert!(
                pair[1].1 > pair[0].1,
                "{} should draw wider than {}",
                pair[1].0,
                pair[0].0
            );
        }
    }

    #[test]
    fn nothing_in_it_carries_meaning_by_colour() {
        // The three colour-blind themes are the reason. Every part of the
        // figure is inside one group; the stylesheet gives that group one
        // fill, and no branch here picks a different one for a different fact.
        let s = draw(Some(40), Some(170), Some("stout")).expect("drawn");
        assert_eq!(s.svg.matches("class=\"sil-body\"").count(), 1);
        for forbidden in ["fill=", "stroke=", "#", "rgb(", "style=", "opacity"] {
            assert!(
                !s.svg.contains(forbidden),
                "the figure names no colour of its own ({forbidden}): {}",
                s.svg
            );
        }
    }

    #[test]
    fn it_has_no_face() {
        let s = draw(Some(40), Some(170), None).expect("drawn");
        assert!(!s.svg.contains("<text"));
        assert!(!s.svg.contains("<image"));
        // Head, torso, two legs, two arms. Nothing left over to be a feature.
        assert_eq!(s.svg.matches("<path").count(), 6);
    }

    #[test]
    fn the_figure_stays_inside_its_box() {
        for age in [0, 5, 14, 40, 88] {
            for h in [None, Some(30u32), Some(45), Some(120), Some(170), Some(220)] {
                for build in [None, Some("slight"), Some("heavy")] {
                    let s = draw(Some(age), h, build).expect("drawn");
                    for (x, y) in points(&s.svg) {
                        assert!(
                            (0.0..=W).contains(&x) && (0.0..=H).contains(&y),
                            "age {age} height {h:?} build {build:?} drew ({x}, {y}) \
                             outside the {W} by {H} canvas"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn every_figure_stands_on_the_ground_line() {
        // Two figures are only comparable if they share a baseline. A gap
        // under one of them would read as a shorter person.
        for age in [0, 5, 14, 40, 88] {
            for h in [None, Some(30u32), Some(170), Some(220)] {
                let s = draw(Some(age), h, None).expect("drawn");
                let lowest = points(&s.svg).iter().map(|p| p.1).fold(0.0, f64::max);
                assert_eq!(lowest, BASE, "age {age} height {h:?}");
            }
        }
    }

    #[test]
    fn a_band_names_a_catalogue_key_and_a_slug_that_agree() {
        for band in BANDS {
            assert_eq!(
                band.key(),
                format!("silhouette-proportions-{}", band.slug())
            );
            assert!(!band.slug().contains('_'), "keys use dashes");
        }
        let slugs: std::collections::BTreeSet<_> = BANDS.iter().map(|b| b.slug()).collect();
        assert_eq!(slugs.len(), BANDS.len());
    }

    #[test]
    fn numbers_are_short_and_never_negative_zero() {
        assert_eq!(n(12.0), "12");
        assert_eq!(n(12.34), "12.3");
        assert_eq!(n(-0.02), "0");
        assert_eq!(n(0.0), "0");
        assert_eq!(n(-3.5), "-3.5");
    }

    #[test]
    fn the_view_draws_the_latest_of_several_heights() {
        use crate::physical::Detail;
        let stored = serde_json::json!({
            "extensions": {"axgf-cms:traits/v1": {
                "height_cm": [
                    {"value": 171, "date": {"value": "1914", "precision": "year"}},
                    {"value": 168, "date": {"value": "1934", "precision": "year"}}
                ],
                "build": [{"value": "sturdy"}]
            }}
        });
        let detail = Detail::from_entity(&stored);
        let v = view_for(Some(70), &detail, "en").expect("a figure");
        assert_eq!(v.height_cm, Some(168), "the later measurement");
        assert!(v.several_heights);
        assert_eq!(
            v.height_date.as_ref().map(|d| d.text.as_str()),
            Some("1934")
        );
        assert_eq!(v.build.as_deref(), Some("sturdy"));
        assert_eq!(v.band_slug, "elderly");
    }

    #[test]
    fn a_height_the_record_never_states_leaves_the_view_unscaled() {
        use crate::physical::Detail;
        let detail = Detail::from_entity(&serde_json::json!({}));
        let v = view_for(Some(6), &detail, "en").expect("a figure");
        assert!(!v.to_scale);
        assert!(v.height_cm.is_none() && v.height_date.is_none());
        assert!(!v.several_heights);
        assert!(v.build.is_none());
        assert_eq!(v.band_slug, "child");
    }
}
