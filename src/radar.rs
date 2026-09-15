//! Three charts at the foot of a record: physique, temperament and mind, and
//! health and vitality.
//!
//! # What these are, and what they are not
//!
//! Each chart has six axes, each scored 0–100 from the facts the record holds.
//! They are computed here, on every render, by the arithmetic written down
//! below — never by a language model, never by anything that could not show
//! its working — and they are **never stored**: nothing here writes to the
//! bundle, and the numbers are a reading of the record, not a claim about the
//! person that a later reader could mistake for one somebody made.
//!
//! Three rules hold for every axis:
//!
//! 1. **No data, no axis.** An axis with nothing to read is drawn as absent —
//!    a dotted spoke and "none" — and never as 50. A middling score is a
//!    statement; the absence of a fact is not one.
//! 2. **Every score is traceable.** Each axis lists the facts it read, in the
//!    reader's language, with their dates; an absent axis says "none".
//! 3. **Every score carries its confidence**, drawn on the chart three ways at
//!    once so that no single channel carries it alone: a bar along the spoke
//!    whose length is the uncertainty, a marker whose shape is the band
//!    (filled, ringed, hollow, dashed), and the band's own colour token. The
//!    table beneath says it in words.
//!
//! The charts read the person **after** the reader's class access has been
//! applied (`crate::sensitive::strip`), so a class this reader may not read is
//! exactly the same as a class nobody recorded: its axes are absent, and the
//! notice beside the chart — the same one the profile tab shows — says that
//! something was withheld, without saying what. The temperament chart of a
//! living person is folded shut even for a reader who may open it: a portrait
//! of a living relative's personality is not something a page should show
//! somebody who did not ask to see it.
//!
//! # Confidence
//!
//! A claim's own `confidence` is used as it is. A claim that states none counts
//! as [`UNSTATED`] — recorded, but not graded by anyone. An axis's confidence
//! is the mean of the claims it read, times a coverage factor
//! `0.75 + 0.25 × inputs present / inputs the axis reads`, times any penalty
//! the axis names below. The bar drawn along the spoke is
//! `± 30 × (1 − confidence)` points.
//!
//! When an attribute holds several claims, the latest dated one is read; an
//! undated claim counts as older than every dated one, and among equals the
//! later in the list wins.
//!
//! # Physique
//!
//! | axis        | reads                                    | score |
//! |-------------|------------------------------------------|-------|
//! | stature     | `morphology.height`, `identity.sex_at_birth` | the percentile of the height among adults today: normal, mean 162 cm σ 7 (female), 176 cm σ 7.5 (male), 169 cm σ 9.5 with no recorded sex (confidence × 0.85). A height measured before 18, when the birth year says so, is not read |
//! | build       | `morphology.bmi`, else `weight` with `height`, else `build` | BMI 15→0, 18.5→25, 22→45, 25→60, 30→80, 40→100, linear between; a computed BMI × 0.9; build: slight 10, slim 30, average 50, sturdy 65, stout 80, heavy 92 |
//! | lean mass   | `morphology.body_composition`            | muscle 25 %→0 … 50 %→100; fat 45 %→0 … 10 %→100; the mean of those given |
//! | posture     | `morphology.posture`                     | ideal 90, flat back 60, sway back 55, kyphotic-lordotic 50, scoliotic 40, stooped 30 |
//! | gait        | `morphology.gait`                        | brisk 90, average 70, slow 45, stiff 45, waddling 40, limping 35, shuffling 30, unsteady 20 |
//! | dentition   | `morphology.dentition`, `malocclusion`   | complete permanent 95, primary or mixed 90, implants 70, partial loss 60, partial denture 45, full denture 25, edentulous 10; a malocclusion takes 5 (class I) or 10 (class II, III) |
//!
//! Build is "mass for height", not a verdict: a high number is a heavier
//! build, not a better or a worse one.
//!
//! # Temperament and mind
//!
//! | axis                | reads                                          | score |
//! |---------------------|------------------------------------------------|-------|
//! | openness            | `personality.big_five`                          | the recorded score |
//! | conscientiousness   | `personality.big_five`                          | the recorded score |
//! | extraversion        | `big_five`, else `introversion_extraversion`, else `mbti` | the score; strongly introverted 10, introverted 30, ambiverted 50, extraverted 70, strongly extraverted 90; an MBTI E 70 or I 30 (confidence × 0.5) |
//! | agreeableness       | `personality.big_five`                          | the recorded score |
//! | emotional stability | `big_five` neuroticism, else `stress_tolerance` | 100 − neuroticism; very low 10, low 30, moderate 50, high 70, very high 90 |
//! | cognition           | `health.mental_health_assessments` (MMSE, MoCA) | score ÷ 30 × 100 |
//!
//! A Big Five score from an observer's rating or inferred from sources counts
//! × 0.7: the instrument says how the number was reached.
//!
//! # Health and vitality
//!
//! | axis         | reads                                         | score |
//! |--------------|-----------------------------------------------|-------|
//! | circulation  | `health.blood_pressure`, `resting_heart_rate` | pressure by its worse reading: <120/<80 95, <130/<85 85, <140/<90 70, <160/<100 50, <180/<110 30, above 15; rate 50–70 90, 40–49 80, 71–80 75, 81–90 60, 91–100 45, above 100 30, below 40 40; the mean |
//! | breathing    | `health.respiratory_capacity`                 | FEV1/FVC (given, or computed): 0.50→35, 0.60→50, 0.70→70, 0.75→85, 0.80→95, linear between, 20 below 0.50 |
//! | metabolism   | `health.lab_results`, `deficiencies`          | the latest result for each analyte: normal 90, low or high 50, critical 15 (with no flag, inside its reference range 90, outside 50); the mean, less 10 for each current deficiency, at least 10 |
//! | illness      | `health.conditions`                           | 100, less 25 for each current chronic condition and 10 for each other current one (half for a suspected one), less 2 for each ended; at least 5. Read only when a condition is recorded |
//! | senses       | `biometrics.hearing`, `visual_acuity`         | hearing by grade: normal 95, mild 75, moderate 55, moderately severe 40, severe 25, profound 10, complete 0; sight: decimal acuity × 95, at most 100; the mean over ears and eyes |
//! | rest and mood| `health.sleep_disorders`, `mental_health_assessments` (not MMSE, MoCA) | severity: none or minimal 90, mild 70, moderate 50, moderately severe 35, severe 20; sleep: 90 less 20 per current disorder, at least 20; the mean |
//!
//! "Current" is a claim with no `valid_until`.

use serde::Serialize;
use serde_json::Value;

use axgf_rs::model::profile::registry;

use crate::sensitive::Scopes;
use crate::view::Confidence;

/// The confidence of a claim that states none: recorded, not graded.
pub const UNSTATED: f64 = 0.7;

/// The three charts, in the order the record draws them.
pub const CHARTS: [&str; 3] = ["physique", "mind", "vitality"];

/// Every axis of every chart, by chart.
pub const AXES: [(&str, [&str; 6]); 3] = [
    (
        "physique",
        [
            "stature",
            "build",
            "lean-mass",
            "posture",
            "gait",
            "dentition",
        ],
    ),
    (
        "mind",
        [
            "openness",
            "conscientiousness",
            "extraversion",
            "agreeableness",
            "stability",
            "cognition",
        ],
    ),
    (
        "vitality",
        [
            "circulation",
            "breathing",
            "metabolism",
            "illness",
            "senses",
            "rest",
        ],
    ),
];

/// Every attribute each chart reads.
pub const READS: [(&str, &[&str]); 3] = [
    (
        "physique",
        &[
            "identity.sex_at_birth",
            "morphology.height",
            "morphology.weight",
            "morphology.bmi",
            "morphology.build",
            "morphology.body_composition",
            "morphology.posture",
            "morphology.gait",
            "morphology.dentition",
            "morphology.malocclusion",
        ],
    ),
    (
        "mind",
        &[
            "personality.big_five",
            "personality.introversion_extraversion",
            "personality.mbti",
            "personality.stress_tolerance",
            "health.mental_health_assessments",
        ],
    ),
    (
        "vitality",
        &[
            "health.blood_pressure",
            "health.resting_heart_rate",
            "health.respiratory_capacity",
            "health.lab_results",
            "health.deficiencies",
            "health.conditions",
            "biometrics.hearing",
            "biometrics.visual_acuity",
            "health.sleep_disorders",
            "health.mental_health_assessments",
        ],
    ),
];

/// The scopes a chart's attributes are governed by, read from the schema's
/// classes rather than assumed from their blocks: hearing and sight sit in the
/// biometrics block and are health data.
fn chart_scopes(chart: &str) -> Scopes {
    READS
        .iter()
        .find(|(c, _)| *c == chart)
        .map(|(_, paths)| {
            paths
                .iter()
                .filter_map(|p| registry::attribute(p))
                .fold(Scopes::NONE, |acc, a| {
                    acc.union(crate::profile::attribute_scopes(a))
                })
        })
        .unwrap_or(Scopes::NONE)
}

/// The scopes of the attributes this chart reads that the person actually
/// holds. A diagnosis is not a reason to tell a reader the temperament chart
/// is missing something.
fn held_by(person: &Value, chart: &str) -> Scopes {
    READS
        .iter()
        .find(|(c, _)| *c == chart)
        .map(|(_, paths)| {
            paths
                .iter()
                .filter(|p| !claims(person, p).is_empty())
                .filter_map(|p| registry::attribute(p))
                .fold(Scopes::NONE, |acc, a| {
                    acc.union(crate::profile::attribute_scopes(a))
                })
        })
        .unwrap_or(Scopes::NONE)
}

/// One chart, ready to draw.
#[derive(Debug, Clone, Serialize)]
pub struct Chart {
    pub key: &'static str,
    pub axes: Vec<Axis>,
    /// The drawing: numbers and classes, no text from the bundle.
    pub svg: String,
    /// How many axes have a score.
    pub present: usize,
    /// Classes this chart reads that the record holds and this reader may
    /// not read, named in the reader's language.
    pub withheld: Vec<String>,
    /// Folded shut until opened: the temperament of a living person.
    pub folded: bool,
}

/// One axis.
#[derive(Debug, Clone, Serialize)]
pub struct Axis {
    pub key: &'static str,
    /// 1–6, printed beside the name on the chart and in the table.
    pub number: usize,
    pub score: Option<u8>,
    pub confidence: Option<Confidence>,
    /// What the score was read from; empty when it is absent.
    pub facts: Vec<Fact>,
    /// Where the label sits around the chart, as a percentage of its box.
    pub label_x: f64,
    pub label_y: f64,
    /// `top`, `bottom`, `left` or `right`: which way the label grows.
    pub label_side: &'static str,
}

/// One fact an axis read.
#[derive(Debug, Clone, Serialize)]
pub struct Fact {
    /// The attribute, in the reader's language.
    pub label: String,
    /// The claim's value, as the profile tab renders it.
    pub text: String,
    /// When, if the claim says.
    pub date: Option<String>,
}

/// A score before it is drawn: the number, how sure, and what it read.
struct Scored<'a> {
    score: f64,
    confidence: f64,
    facts: Vec<(&'static str, &'a Value)>,
    /// The one field of an object value the score came from, when it came
    /// from one: an openness score is read from `openness`, and the fact
    /// beside it says that rather than the whole five-factor record.
    focus: Option<&'static str>,
}

/// The three charts for `person`, as `readable` may see them.
///
/// `flat` and `withheld_documents` are what the profile's renderer needs to
/// name the facts; `living` is the recorded status, which folds the
/// temperament chart.
pub fn charts(
    flat: &Value,
    person: &Value,
    readable: Scopes,
    withheld_documents: &std::collections::BTreeSet<String>,
    lang: &str,
    living: bool,
) -> Vec<Chart> {
    let lifted = crate::profile::lift::lift(person);
    let visible = crate::sensitive::strip(&lifted, readable);
    let reader = crate::profile::view::Reader {
        flat,
        lang,
        readable,
        withheld_documents,
    };
    let birth_year = person
        .get("birth")
        .and_then(|b| crate::view::latest_year_of_field(b, "date"));

    CHARTS
        .iter()
        .map(|&key| {
            let scored: [Option<Scored>; 6] = match key {
                "physique" => physique(&visible, birth_year),
                "mind" => mind(&visible),
                _ => vitality(&visible),
            };
            let names = AXES
                .iter()
                .find(|(c, _)| *c == key)
                .map(|(_, a)| a)
                .expect("chart");
            let axes: Vec<Axis> = scored
                .into_iter()
                .zip(names.iter())
                .enumerate()
                .map(|(i, (s, name))| axis(i, name, s, &reader))
                .collect();
            let present = axes.iter().filter(|a| a.score.is_some()).count();
            let withheld = chart_scopes(key)
                .iter()
                .filter(|s| held_by(&lifted, key).contains(*s) && !readable.contains(*s))
                .map(|s| crate::i18n::translate(lang, &format!("scope-{}", s.as_str()), None))
                .collect();
            Chart {
                key,
                svg: svg(&axes),
                present,
                withheld,
                folded: key == "mind" && living && present > 0,
                axes,
            }
        })
        .collect()
}

fn axis(
    i: usize,
    key: &'static str,
    s: Option<Scored<'_>>,
    reader: &crate::profile::view::Reader<'_>,
) -> Axis {
    let (x, y) = point(i, LABEL_R);
    let side = match i {
        0 => "top",
        3 => "bottom",
        1 | 2 => "right",
        _ => "left",
    };
    let (lx, ly) = to_box(x, y);
    match s {
        None => Axis {
            key,
            number: i + 1,
            score: None,
            confidence: None,
            facts: Vec::new(),
            label_x: lx,
            label_y: ly,
            label_side: side,
        },
        Some(s) => Axis {
            key,
            number: i + 1,
            score: Some(s.score.round().clamp(0.0, 100.0) as u8),
            confidence: Some(Confidence::new(s.confidence)),
            facts: s
                .facts
                .iter()
                .map(|(path, claim)| fact(path, claim, s.focus, reader))
                .collect(),
            label_x: lx,
            label_y: ly,
            label_side: side,
        },
    }
}

/// A claim said the way the profile tab says it.
fn fact(
    path: &str,
    claim: &Value,
    focus: Option<&str>,
    reader: &crate::profile::view::Reader<'_>,
) -> Fact {
    let Some(a) = registry::attribute(path) else {
        return Fact {
            label: path.to_string(),
            text: String::new(),
            date: None,
        };
    };
    let view = crate::profile::view::claim_view(a, &a.shape, claim, reader);
    if let Some(field) = focus {
        let label = crate::profile::field_label(reader.lang, a, field);
        if let Some(part) = view.parts.iter().find(|p| p.label == label) {
            return Fact {
                label: crate::profile::attribute_label(reader.lang, a),
                text: format!("{} {}", part.label, part.text),
                date: view.date.or(view.from).map(|d| d.text),
            };
        }
    }
    let mut text = view.text.clone().unwrap_or_default();
    for part in &view.parts {
        if !text.is_empty() {
            text.push_str(&crate::i18n::translate(reader.lang, "list-separator", None));
        }
        text.push_str(&part.label);
        text.push(' ');
        text.push_str(&part.text);
    }
    Fact {
        label: crate::profile::attribute_label(reader.lang, a),
        text,
        date: view.date.or(view.from).map(|d| d.text),
    }
}

// ---------------------------------------------------------------------------
// reading claims
// ---------------------------------------------------------------------------

/// Every claim of `block.key`, in the order the record lists them.
fn claims<'a>(person: &'a Value, path: &str) -> Vec<&'a Value> {
    let Some((block, key)) = path.split_once('.') else {
        return Vec::new();
    };
    match person.get(block).and_then(|b| b.get(key)) {
        Some(Value::Array(list)) => list.iter().filter(|c| c.is_object()).collect(),
        Some(v @ Value::Object(_)) => vec![v],
        _ => Vec::new(),
    }
}

/// The latest claim, by date and then by position.
fn latest<'a>(person: &'a Value, path: &str) -> Option<&'a Value> {
    claims(person, path)
        .into_iter()
        .enumerate()
        .max_by_key(|(i, c)| (date_key(c), *i))
        .map(|(_, c)| c)
}

fn date_key(claim: &Value) -> i64 {
    claim
        .get("date")
        .or_else(|| claim.pointer("/valid_from/date"))
        .map(crate::view::render_date)
        .and_then(|d| d.sort)
        .unwrap_or(i64::MIN)
}

fn year_of(claim: &Value) -> Option<i64> {
    crate::view::latest_year_of_field(claim, "date")
}

/// A claim's confidence, or [`UNSTATED`].
fn conf(claim: &Value) -> f64 {
    claim
        .get("confidence")
        .and_then(Value::as_f64)
        .map(|c| c.clamp(0.0, 1.0))
        .unwrap_or(UNSTATED)
}

fn num(claim: &Value, field: Option<&str>) -> Option<f64> {
    let v = claim.get("value")?;
    match field {
        Some(f) => v.get(f)?.as_f64(),
        None => v.as_f64(),
    }
}

fn term<'a>(claim: &'a Value, field: Option<&str>) -> Option<&'a str> {
    let v = claim.get("value")?;
    match field {
        Some(f) => v.get(f)?.as_str(),
        None => v.as_str(),
    }
}

fn current(claim: &Value) -> bool {
    claim.get("valid_until").is_none_or(Value::is_null)
}

/// Piecewise-linear interpolation through `(x, y)` points sorted by `x`,
/// flat beyond either end.
fn ramp(x: f64, points: &[(f64, f64)]) -> f64 {
    let first = points[0];
    let last = points[points.len() - 1];
    if x <= first.0 {
        return first.1;
    }
    if x >= last.0 {
        return last.1;
    }
    for w in points.windows(2) {
        let ((x0, y0), (x1, y1)) = (w[0], w[1]);
        if x <= x1 {
            return y0 + (y1 - y0) * (x - x0) / (x1 - x0);
        }
    }
    last.1
}

/// The normal distribution's cumulative probability, to four places
/// (Abramowitz and Stegun 7.1.26).
fn normal_cdf(z: f64) -> f64 {
    let t = 1.0 / (1.0 + 0.327_591_1 * z.abs() / std::f64::consts::SQRT_2);
    let poly = t
        * (0.254_829_592
            + t * (-0.284_496_736
                + t * (1.421_413_741 + t * (-1.453_152_027 + t * 1.061_405_429))));
    let erf = 1.0 - poly * (-(z * z) / 2.0).exp();
    if z >= 0.0 {
        0.5 * (1.0 + erf)
    } else {
        0.5 * (1.0 - erf)
    }
}

/// An axis from sub-scores, each with the claims it read.
fn combine<'a>(
    parts: Vec<(f64, Vec<&'a Value>, Vec<&'static str>)>,
    inputs: usize,
    penalty: f64,
) -> Option<Scored<'a>> {
    if parts.is_empty() {
        return None;
    }
    let score = parts.iter().map(|p| p.0).sum::<f64>() / parts.len() as f64;
    let claims: Vec<(&'static str, &Value)> = parts
        .iter()
        .flat_map(|(_, cs, paths)| paths.iter().copied().zip(cs.iter().copied()))
        .collect();
    let mean_conf = claims.iter().map(|(_, c)| conf(c)).sum::<f64>() / claims.len().max(1) as f64;
    let coverage = 0.75 + 0.25 * parts.len().min(inputs) as f64 / inputs.max(1) as f64;
    Some(Scored {
        score,
        confidence: (mean_conf * coverage * penalty).clamp(0.0, 1.0),
        facts: claims,
        focus: None,
    })
}

fn single<'a>(
    score: f64,
    path: &'static str,
    claim: &'a Value,
    penalty: f64,
) -> Option<Scored<'a>> {
    Some(Scored {
        score,
        confidence: (conf(claim) * penalty).clamp(0.0, 1.0),
        facts: vec![(path, claim)],
        focus: None,
    })
}

fn vocab_score(claim: &Value, field: Option<&str>, table: &[(&str, f64)]) -> Option<f64> {
    let t = term(claim, field)?;
    table.iter().find(|(k, _)| *k == t).map(|(_, v)| *v)
}

// ---------------------------------------------------------------------------
// physique
// ---------------------------------------------------------------------------

fn physique(p: &Value, birth_year: Option<i64>) -> [Option<Scored<'_>>; 6] {
    [
        stature(p, birth_year),
        build(p),
        lean_mass(p),
        latest(p, "morphology.posture").and_then(|c| {
            let s = vocab_score(
                c,
                None,
                &[
                    ("ideal", 90.0),
                    ("flat_back", 60.0),
                    ("sway_back", 55.0),
                    ("kyphotic_lordotic", 50.0),
                    ("scoliotic", 40.0),
                    ("stooped", 30.0),
                ],
            )?;
            single(s, "morphology.posture", c, 1.0)
        }),
        latest(p, "morphology.gait").and_then(|c| {
            let s = vocab_score(
                c,
                None,
                &[
                    ("brisk", 90.0),
                    ("average", 70.0),
                    ("slow", 45.0),
                    ("stiff", 45.0),
                    ("waddling", 40.0),
                    ("limping", 35.0),
                    ("shuffling", 30.0),
                    ("unsteady", 20.0),
                ],
            )?;
            single(s, "morphology.gait", c, 1.0)
        }),
        dentition(p),
    ]
}

fn stature(p: &Value, birth_year: Option<i64>) -> Option<Scored<'_>> {
    let adult = |c: &&Value| match (year_of(c), birth_year) {
        (Some(y), Some(b)) => y - b >= 18,
        _ => true,
    };
    let height = claims(p, "morphology.height")
        .into_iter()
        .enumerate()
        .filter(|(_, c)| adult(c) && num(c, None).is_some())
        .max_by_key(|(i, c)| (date_key(c), *i))
        .map(|(_, c)| c)?;
    let h = num(height, None)?;
    let sex = latest(p, "identity.sex_at_birth");
    let (mean, sd, penalty, sex_fact) = match sex.and_then(|c| term(c, None)) {
        Some("female") => (162.0, 7.0, 1.0, sex),
        Some("male") => (176.0, 7.5, 1.0, sex),
        _ => (169.0, 9.5, 0.85, None),
    };
    let mut scored = single(
        100.0 * normal_cdf((h - mean) / sd),
        "morphology.height",
        height,
        penalty,
    )?;
    if let Some(s) = sex_fact {
        scored.facts.push(("identity.sex_at_birth", s));
    }
    Some(scored)
}

fn build(p: &Value) -> Option<Scored<'_>> {
    let bmi_ramp = [
        (15.0, 0.0),
        (18.5, 25.0),
        (22.0, 45.0),
        (25.0, 60.0),
        (30.0, 80.0),
        (40.0, 100.0),
    ];
    if let Some(c) = latest(p, "morphology.bmi") {
        if let Some(b) = num(c, None) {
            return single(ramp(b, &bmi_ramp), "morphology.bmi", c, 1.0);
        }
    }
    if let (Some(w), Some(h)) = (
        latest(p, "morphology.weight"),
        latest(p, "morphology.height"),
    ) {
        if let (Some(kg), Some(cm)) = (num(w, None), num(h, None)) {
            if cm > 0.0 {
                let bmi = kg / (cm / 100.0).powi(2);
                return Some(Scored {
                    score: ramp(bmi, &bmi_ramp),
                    confidence: (conf(w).min(conf(h)) * 0.9).clamp(0.0, 1.0),
                    facts: vec![("morphology.weight", w), ("morphology.height", h)],
                    focus: None,
                });
            }
        }
    }
    let c = latest(p, "morphology.build")?;
    let s = vocab_score(
        c,
        None,
        &[
            ("slight", 10.0),
            ("slim", 30.0),
            ("average", 50.0),
            ("sturdy", 65.0),
            ("stout", 80.0),
            ("heavy", 92.0),
        ],
    )?;
    single(s, "morphology.build", c, 1.0)
}

fn lean_mass(p: &Value) -> Option<Scored<'_>> {
    let c = latest(p, "morphology.body_composition")?;
    let mut scores = Vec::new();
    if let Some(m) = num(c, Some("muscle_percent")) {
        scores.push(ramp(m, &[(25.0, 0.0), (50.0, 100.0)]));
    }
    if let Some(f) = num(c, Some("fat_percent")) {
        scores.push(ramp(f, &[(10.0, 100.0), (45.0, 0.0)]));
    }
    if scores.is_empty() {
        return None;
    }
    let s = scores.iter().sum::<f64>() / scores.len() as f64;
    single(s, "morphology.body_composition", c, 1.0)
}

fn dentition(p: &Value) -> Option<Scored<'_>> {
    let c = latest(p, "morphology.dentition")?;
    let base = vocab_score(
        c,
        None,
        &[
            ("permanent_complete", 95.0),
            ("primary", 90.0),
            ("mixed", 90.0),
            ("implants", 70.0),
            ("permanent_partial_loss", 60.0),
            ("partial_denture", 45.0),
            ("full_denture", 25.0),
            ("edentulous", 10.0),
        ],
    )?;
    let mut scored = single(base, "morphology.dentition", c, 1.0)?;
    if let Some(m) = latest(p, "morphology.malocclusion") {
        let less = match term(m, None) {
            Some("class_i") => Some(5.0),
            Some(t) if t.starts_with("class_ii") => Some(10.0),
            Some("normal") => Some(0.0),
            _ => None,
        };
        if let Some(less) = less {
            scored.score = (scored.score - less).max(0.0);
            scored.facts.push(("morphology.malocclusion", m));
        }
    }
    Some(scored)
}

// ---------------------------------------------------------------------------
// temperament and mind
// ---------------------------------------------------------------------------

fn mind(p: &Value) -> [Option<Scored<'_>>; 6] {
    let five = latest(p, "personality.big_five");
    let five_penalty = five
        .and_then(|c| term(c, Some("instrument")))
        .map(|i| {
            if matches!(i, "observer_rating" | "inferred") {
                0.7
            } else {
                1.0
            }
        })
        .unwrap_or(1.0);
    let trait_of = |field: &'static str, invert: bool| {
        let c = five?;
        let v = num(c, Some(field))?;
        let mut scored = single(
            if invert { 100.0 - v } else { v },
            "personality.big_five",
            c,
            five_penalty,
        )?;
        scored.focus = Some(field);
        Some(scored)
    };
    let extraversion = trait_of("extraversion", false)
        .or_else(|| {
            let c = latest(p, "personality.introversion_extraversion")?;
            let s = vocab_score(
                c,
                None,
                &[
                    ("strongly_introverted", 10.0),
                    ("introverted", 30.0),
                    ("ambiverted", 50.0),
                    ("extraverted", 70.0),
                    ("strongly_extraverted", 90.0),
                ],
            )?;
            single(s, "personality.introversion_extraversion", c, 1.0)
        })
        .or_else(|| {
            let c = latest(p, "personality.mbti")?;
            let s = match term(c, None)?.chars().next()? {
                'E' => 70.0,
                'I' => 30.0,
                _ => return None,
            };
            single(s, "personality.mbti", c, 0.5)
        });
    let stability = trait_of("neuroticism", true).or_else(|| {
        let c = latest(p, "personality.stress_tolerance")?;
        let s = vocab_score(
            c,
            None,
            &[
                ("very_low", 10.0),
                ("low", 30.0),
                ("moderate", 50.0),
                ("high", 70.0),
                ("very_high", 90.0),
            ],
        )?;
        single(s, "personality.stress_tolerance", c, 1.0)
    });
    let cognition = claims(p, "health.mental_health_assessments")
        .into_iter()
        .enumerate()
        .filter(|(_, c)| {
            matches!(term(c, Some("instrument")), Some("mmse" | "moca"))
                && num(c, Some("score")).is_some()
        })
        .max_by_key(|(i, c)| (date_key(c), *i))
        .and_then(|(_, c)| {
            let s = num(c, Some("score"))?;
            single(
                (s / 30.0 * 100.0).clamp(0.0, 100.0),
                "health.mental_health_assessments",
                c,
                1.0,
            )
        });
    [
        trait_of("openness", false),
        trait_of("conscientiousness", false),
        extraversion,
        trait_of("agreeableness", false),
        stability,
        cognition,
    ]
}

// ---------------------------------------------------------------------------
// health and vitality
// ---------------------------------------------------------------------------

fn vitality(p: &Value) -> [Option<Scored<'_>>; 6] {
    [
        circulation(p),
        breathing(p),
        metabolism(p),
        illness(p),
        senses(p),
        rest(p),
    ]
}

fn circulation(p: &Value) -> Option<Scored<'_>> {
    let mut parts = Vec::new();
    if let Some(c) = latest(p, "health.blood_pressure") {
        if let (Some(sys), Some(dia)) = (num(c, Some("systolic")), num(c, Some("diastolic"))) {
            let grade = |s: f64, d: f64| -> f64 {
                const STEPS: [(f64, f64, f64); 5] = [
                    (120.0, 80.0, 95.0),
                    (130.0, 85.0, 85.0),
                    (140.0, 90.0, 70.0),
                    (160.0, 100.0, 50.0),
                    (180.0, 110.0, 30.0),
                ];
                STEPS
                    .iter()
                    .find(|(ms, md, _)| s < *ms && d < *md)
                    .map(|(_, _, v)| *v)
                    .unwrap_or(15.0)
            };
            parts.push((grade(sys, dia), vec![c], vec!["health.blood_pressure"]));
        }
    }
    if let Some(c) = latest(p, "health.resting_heart_rate") {
        if let Some(bpm) = num(c, None) {
            let s = match bpm {
                b if b < 40.0 => 40.0,
                b if b < 50.0 => 80.0,
                b if b <= 70.0 => 90.0,
                b if b <= 80.0 => 75.0,
                b if b <= 90.0 => 60.0,
                b if b <= 100.0 => 45.0,
                _ => 30.0,
            };
            parts.push((s, vec![c], vec!["health.resting_heart_rate"]));
        }
    }
    combine(parts, 2, 1.0)
}

fn breathing(p: &Value) -> Option<Scored<'_>> {
    let c = latest(p, "health.respiratory_capacity")?;
    let ratio = num(c, Some("fev1_fvc_ratio")).or_else(|| {
        let fev1 = num(c, Some("fev1_litres"))?;
        let fvc = num(c, Some("fvc_litres")).filter(|v| *v > 0.0)?;
        Some(fev1 / fvc)
    })?;
    let s = if ratio < 0.5 {
        20.0
    } else {
        ramp(
            ratio,
            &[
                (0.5, 35.0),
                (0.6, 50.0),
                (0.7, 70.0),
                (0.75, 85.0),
                (0.8, 95.0),
            ],
        )
    };
    single(s, "health.respiratory_capacity", c, 1.0)
}

fn metabolism(p: &Value) -> Option<Scored<'_>> {
    // The latest result for each analyte.
    let mut by_analyte: std::collections::BTreeMap<&str, (i64, usize, &Value)> = Default::default();
    for (i, c) in claims(p, "health.lab_results").into_iter().enumerate() {
        let Some(analyte) = term(c, Some("analyte")) else {
            continue;
        };
        let key = (date_key(c), i);
        match by_analyte.get(analyte) {
            Some((d, j, _)) if (*d, *j) > key => {}
            _ => {
                by_analyte.insert(analyte, (key.0, key.1, c));
            }
        }
    }
    let mut scores = Vec::new();
    let mut read: Vec<&Value> = Vec::new();
    for (_, _, c) in by_analyte.values() {
        let s = match term(c, Some("flag")) {
            Some("normal") => Some(90.0),
            Some("low" | "high") => Some(50.0),
            Some("critical_low" | "critical_high") => Some(15.0),
            _ => {
                let r = num(c, Some("result"));
                let lo = num(c, Some("reference_low"));
                let hi = num(c, Some("reference_high"));
                match (r, lo, hi) {
                    (Some(r), lo, hi) if lo.is_some() || hi.is_some() => {
                        let inside = lo.is_none_or(|l| r >= l) && hi.is_none_or(|h| r <= h);
                        Some(if inside { 90.0 } else { 50.0 })
                    }
                    _ => None,
                }
            }
        };
        if let Some(s) = s {
            scores.push(s);
            read.push(c);
        }
    }
    let deficiencies: Vec<&Value> = claims(p, "health.deficiencies")
        .into_iter()
        .filter(|c| current(c))
        .collect();
    if scores.is_empty() && deficiencies.is_empty() {
        return None;
    }
    let base = if scores.is_empty() {
        90.0
    } else {
        scores.iter().sum::<f64>() / scores.len() as f64
    };
    let score = (base - 10.0 * deficiencies.len() as f64).max(10.0);
    let mut facts: Vec<(&'static str, &Value)> =
        read.iter().map(|c| ("health.lab_results", *c)).collect();
    facts.extend(deficiencies.iter().map(|c| ("health.deficiencies", *c)));
    let mean = facts.iter().map(|(_, c)| conf(c)).sum::<f64>() / facts.len() as f64;
    let inputs_present = usize::from(!scores.is_empty()) + usize::from(!deficiencies.is_empty());
    Some(Scored {
        score,
        confidence: (mean * (0.75 + 0.25 * inputs_present as f64 / 2.0)).clamp(0.0, 1.0),
        facts,
        focus: None,
    })
}

fn illness(p: &Value) -> Option<Scored<'_>> {
    let conditions = claims(p, "health.conditions");
    if conditions.is_empty() {
        return None;
    }
    let mut score: f64 = 100.0;
    for c in &conditions {
        if !current(c) {
            score -= 2.0;
            continue;
        }
        let chronic = c.pointer("/value/chronic").and_then(Value::as_bool) == Some(true);
        let mut weight = if chronic { 25.0 } else { 10.0 };
        if term(c, Some("diagnosis")) == Some("suspected") {
            weight /= 2.0;
        }
        score -= weight;
    }
    let facts: Vec<(&'static str, &Value)> = conditions
        .iter()
        .map(|c| ("health.conditions", *c))
        .collect();
    let mean = facts.iter().map(|(_, c)| conf(c)).sum::<f64>() / facts.len() as f64;
    Some(Scored {
        score: score.max(5.0),
        confidence: mean,
        facts,
        focus: None,
    })
}

fn senses(p: &Value) -> Option<Scored<'_>> {
    // The latest reading for each ear and each eye.
    let latest_per = |path: &str, side_field: &str| -> Vec<&Value> {
        let mut by_side: std::collections::BTreeMap<String, (i64, usize, &Value)> =
            Default::default();
        for (i, c) in claims(p, path).into_iter().enumerate() {
            let side = term(c, Some(side_field)).unwrap_or("both").to_string();
            let key = (date_key(c), i);
            match by_side.get(&side) {
                Some((d, j, _)) if (*d, *j) > key => {}
                _ => {
                    by_side.insert(side, (key.0, key.1, c));
                }
            }
        }
        by_side.into_values().map(|(_, _, c)| c).collect()
    };
    let mut parts = Vec::new();
    let hearing: Vec<(f64, &Value)> = latest_per("biometrics.hearing", "ear")
        .into_iter()
        .filter_map(|c| {
            let s = vocab_score(
                c,
                Some("grade"),
                &[
                    ("normal", 95.0),
                    ("mild", 75.0),
                    ("moderate", 55.0),
                    ("moderately_severe", 40.0),
                    ("severe", 25.0),
                    ("profound", 10.0),
                    ("complete", 0.0),
                ],
            )?;
            Some((s, c))
        })
        .collect();
    if !hearing.is_empty() {
        let s = hearing.iter().map(|(s, _)| s).sum::<f64>() / hearing.len() as f64;
        let cs: Vec<&Value> = hearing.iter().map(|(_, c)| *c).collect();
        let paths = vec!["biometrics.hearing"; cs.len()];
        parts.push((s, cs, paths));
    }
    let sight: Vec<(f64, &Value)> = latest_per("biometrics.visual_acuity", "eye")
        .into_iter()
        .filter_map(|c| Some(((num(c, Some("decimal"))? * 95.0).min(100.0), c)))
        .collect();
    if !sight.is_empty() {
        let s = sight.iter().map(|(s, _)| s).sum::<f64>() / sight.len() as f64;
        let cs: Vec<&Value> = sight.iter().map(|(_, c)| *c).collect();
        let paths = vec!["biometrics.visual_acuity"; cs.len()];
        parts.push((s, cs, paths));
    }
    combine(parts, 2, 1.0)
}

fn rest(p: &Value) -> Option<Scored<'_>> {
    let mut parts = Vec::new();
    let mood = claims(p, "health.mental_health_assessments")
        .into_iter()
        .enumerate()
        .filter(|(_, c)| {
            !matches!(term(c, Some("instrument")), Some("mmse" | "moca"))
                && term(c, Some("severity")).is_some()
        })
        .max_by_key(|(i, c)| (date_key(c), *i))
        .map(|(_, c)| c);
    if let Some(c) = mood {
        if let Some(s) = vocab_score(
            c,
            Some("severity"),
            &[
                ("none_minimal", 90.0),
                ("mild", 70.0),
                ("moderate", 50.0),
                ("moderately_severe", 35.0),
                ("severe", 20.0),
            ],
        ) {
            parts.push((s, vec![c], vec!["health.mental_health_assessments"]));
        }
    }
    let sleep = claims(p, "health.sleep_disorders");
    if !sleep.is_empty() {
        let current_count = sleep.iter().filter(|c| current(c)).count() as f64;
        let paths = vec!["health.sleep_disorders"; sleep.len()];
        parts.push(((90.0 - 20.0 * current_count).max(20.0), sleep, paths));
    }
    combine(parts, 2, 1.0)
}

// ---------------------------------------------------------------------------
// drawing
// ---------------------------------------------------------------------------

/// The chart's own coordinates: centre, the radius of 100, where the labels
/// sit, and the box the labels are positioned in.
const CX: f64 = 150.0;
const CY: f64 = 150.0;
const R: f64 = 100.0;
const LABEL_R: f64 = 116.0;
const BOX_X: f64 = -80.0;
const BOX_Y: f64 = -30.0;
const BOX_W: f64 = 460.0;
const BOX_H: f64 = 360.0;

/// The point at `r` along axis `i`, clockwise from the top.
fn point(i: usize, r: f64) -> (f64, f64) {
    let angle = (-90.0 + 60.0 * i as f64).to_radians();
    (CX + r * angle.cos(), CY + r * angle.sin())
}

/// A chart coordinate as a percentage of the labelled box.
fn to_box(x: f64, y: f64) -> (f64, f64) {
    (
        ((x - BOX_X) / BOX_W * 1000.0).round() / 10.0,
        ((y - BOX_Y) / BOX_H * 1000.0).round() / 10.0,
    )
}

fn n(v: f64) -> String {
    format!("{v:.1}")
}

/// The drawing. Only numbers and class names go in: every word is in the
/// labels and the table the template puts around it.
fn svg(axes: &[Axis]) -> String {
    let mut out = format!(
        "<svg class=\"radar\" viewBox=\"{} {} {} {}\" aria-hidden=\"true\" focusable=\"false\">",
        n(BOX_X),
        n(BOX_Y),
        n(BOX_W),
        n(BOX_H)
    );
    // The scale: rings at 25, 50, 75 and 100, and the six spokes. It carries no
    // information the table does not, so the contrast sweep may skip it.
    out.push_str("<g class=\"radar-grid\" data-contrast=\"decorative\">");
    for ring in [25.0, 50.0, 75.0, 100.0] {
        let pts: Vec<String> = (0..6)
            .map(|i| {
                let (x, y) = point(i, R * ring / 100.0);
                format!("{},{}", n(x), n(y))
            })
            .collect();
        out.push_str(&format!(
            "<polygon class=\"radar-ring\" points=\"{}\"/>",
            pts.join(" ")
        ));
    }
    for (i, a) in axes.iter().enumerate() {
        if a.score.is_some() {
            let (x, y) = point(i, R);
            out.push_str(&format!(
                "<line class=\"radar-spoke\" x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\"/>",
                n(CX),
                n(CY),
                n(x),
                n(y)
            ));
        }
    }
    out.push_str("</g>");
    // An absent axis is drawn, dotted, so the shape of what is missing is as
    // visible as the shape of what is there.
    for (i, a) in axes.iter().enumerate() {
        if a.score.is_none() {
            let (x, y) = point(i, R);
            out.push_str(&format!(
                "<line class=\"radar-spoke radar-absent\" x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\"/>",
                n(CX),
                n(CY),
                n(x),
                n(y)
            ));
        }
    }

    let present: Vec<(usize, u8, &Confidence)> = axes
        .iter()
        .enumerate()
        .filter_map(|(i, a)| Some((i, a.score?, a.confidence.as_ref()?)))
        .collect();
    let pts: Vec<(f64, f64)> = present
        .iter()
        .map(|(i, s, _)| point(*i, R * f64::from(*s) / 100.0))
        .collect();
    if pts.len() >= 3 {
        let list = pts
            .iter()
            .map(|(x, y)| format!("{},{}", n(*x), n(*y)))
            .collect::<Vec<_>>()
            .join(" ");
        out.push_str(&format!(
            "<g data-contrast=\"decorative\"><polygon class=\"radar-area\" points=\"{list}\"/></g>\
             <polygon class=\"radar-outline\" points=\"{list}\"/>"
        ));
    } else if pts.len() == 2 {
        out.push_str(&format!(
            "<line class=\"radar-outline\" x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\"/>",
            n(pts[0].0),
            n(pts[0].1),
            n(pts[1].0),
            n(pts[1].1)
        ));
    }
    // Uncertainty, then the markers over it.
    for (i, s, c) in &present {
        let u = 30.0 * (1.0 - c.value);
        if u >= 1.0 {
            let lo = (f64::from(*s) - u).max(0.0);
            let hi = (f64::from(*s) + u).min(100.0);
            let (x1, y1) = point(*i, R * lo / 100.0);
            let (x2, y2) = point(*i, R * hi / 100.0);
            out.push_str(&format!(
                "<line class=\"radar-band\" data-band=\"{}\" x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\"/>",
                c.band,
                n(x1),
                n(y1),
                n(x2),
                n(y2)
            ));
        }
    }
    // The marker's shape is the band, so the band reads without colour: a
    // filled disc (certain), a ring around a dot (high), a ring (medium), a
    // broken ring (low).
    for ((_, _, c), (x, y)) in present.iter().zip(pts.iter()) {
        let (x, y) = (n(*x), n(*y));
        let r = if c.band == "high" { "6" } else { "5" };
        out.push_str(&format!(
            "<circle class=\"radar-point\" data-band=\"{}\" cx=\"{x}\" cy=\"{y}\" r=\"{r}\"/>",
            c.band
        ));
        if c.band == "high" {
            out.push_str(&format!(
                "<circle class=\"radar-point-core\" cx=\"{x}\" cy=\"{y}\" r=\"2.4\"/>"
            ));
        }
    }
    out.push_str("</svg>");
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn chart<'a>(charts: &'a [Chart], key: &str) -> &'a Chart {
        charts.iter().find(|c| c.key == key).expect("chart")
    }

    fn axis<'a>(c: &'a Chart, key: &str) -> &'a Axis {
        c.axes.iter().find(|a| a.key == key).expect("axis")
    }

    fn draw(person: &Value, readable: Scopes, living: bool) -> Vec<Chart> {
        let flat = json!({"persons": {}, "documents": {}, "sources": {}});
        charts(&flat, person, readable, &Default::default(), "en", living)
    }

    #[test]
    fn nothing_recorded_is_absent_and_never_fifty() {
        let charts = draw(
            &json!({"identity": {"name": {"display": "Nobody"}}}),
            Scopes::EVERY,
            false,
        );
        for c in &charts {
            assert_eq!(c.present, 0, "{} has nothing to read", c.key);
            for a in &c.axes {
                assert!(a.score.is_none() && a.confidence.is_none() && a.facts.is_empty());
            }
            assert!(
                !c.svg.contains("radar-outline"),
                "no shape is drawn from nothing"
            );
            assert_eq!(c.svg.matches("radar-absent").count(), 6);
        }
    }

    #[test]
    fn stature_is_a_percentile_against_the_recorded_sex() {
        let p = json!({
            "identity": {"sex_at_birth": {"value": "female", "confidence": 0.95}},
            "morphology": {"height": [{"value": 162, "confidence": 0.9}]}
        });
        let charts = draw(&p, Scopes::EVERY, false);
        let a = axis(chart(&charts, "physique"), "stature");
        assert_eq!(a.score, Some(50), "the mean is the 50th percentile");
        assert_eq!(
            a.facts.len(),
            2,
            "the height and the sex it was read against"
        );
        assert_eq!(a.confidence.as_ref().map(|c| c.percent), Some(90));
    }

    #[test]
    fn a_childhood_height_is_not_read_as_an_adults() {
        let p = json!({
            "birth": {"date": {"value": "1950", "precision": "year"}},
            "morphology": {"height": [
                {"value": 120, "date": {"value": "1958", "precision": "year"}}
            ]}
        });
        let charts = draw(&p, Scopes::EVERY, false);
        assert!(axis(chart(&charts, "physique"), "stature").score.is_none());
    }

    #[test]
    fn the_latest_dated_claim_is_the_one_read() {
        let p = json!({"morphology": {"gait": [
            {"value": "unsteady", "date": {"value": "1990", "precision": "year"}},
            {"value": "brisk", "date": {"value": "1960", "precision": "year"}},
            {"value": "slow"}
        ]}});
        let charts = draw(&p, Scopes::EVERY, false);
        assert_eq!(axis(chart(&charts, "physique"), "gait").score, Some(20));
    }

    #[test]
    fn an_unstated_confidence_is_recorded_but_ungraded() {
        let p = json!({"morphology": {"posture": [{"value": "ideal"}]}});
        let charts = draw(&p, Scopes::EVERY, false);
        let a = axis(chart(&charts, "physique"), "posture");
        assert_eq!(a.confidence.as_ref().map(|c| c.percent), Some(70));
    }

    #[test]
    fn a_withheld_class_is_absent_and_said_to_be_withheld() {
        let p = json!({
            "identity": {"is_living": true},
            "health": {"blood_pressure": [{"value": {"systolic": 118, "diastolic": 76}}]},
            "biometrics": {"hearing": [{"value": {"ear": "left", "grade": "mild"}}]}
        });
        let all = draw(&p, Scopes::EVERY, true);
        assert!(axis(chart(&all, "vitality"), "circulation").score.is_some());

        let none = draw(&p, Scopes::NONE, true);
        let v = chart(&none, "vitality");
        assert_eq!(v.present, 0, "nothing withheld is scored");
        // Hearing sits in the biometrics block and is health data: one class.
        assert_eq!(v.withheld.len(), 1, "{:?}", v.withheld);
        assert!(chart(&none, "physique").withheld.is_empty());
    }

    #[test]
    fn a_living_persons_temperament_is_folded() {
        let p = json!({"personality": {"big_five": [{"value": {
            "openness": 72, "conscientiousness": 64, "extraversion": 38,
            "agreeableness": 55, "neuroticism": 41}}]}});
        let living = draw(&p, Scopes::EVERY, true);
        let m = chart(&living, "mind");
        assert!(m.folded);
        assert_eq!(axis(m, "stability").score, Some(59), "100 − neuroticism");
        let dead = draw(&p, Scopes::EVERY, false);
        assert!(!chart(&dead, "mind").folded);
    }

    #[test]
    fn the_drawing_holds_no_text_from_the_record() {
        let p = json!({"health": {"conditions": [
            {"value": {"description": "Sarcoidosis", "chronic": true}}]}});
        let charts = draw(&p, Scopes::EVERY, false);
        let v = chart(&charts, "vitality");
        assert_eq!(axis(v, "illness").score, Some(75));
        assert!(!v.svg.contains("Sarcoidosis"));
        assert!(
            axis(v, "illness").facts[0].text.contains("Sarcoidosis"),
            "the table names it"
        );
    }

    #[test]
    fn low_confidence_draws_a_longer_bar_than_high() {
        let p = json!({"morphology": {
            "posture": [{"value": "ideal", "confidence": 0.95}],
            "gait": [{"value": "average", "confidence": 0.3}]
        }});
        let charts = draw(&p, Scopes::EVERY, false);
        let c = chart(&charts, "physique");
        assert_eq!(
            axis(c, "gait").confidence.as_ref().map(|c| c.band),
            Some("low")
        );
        let bands: Vec<&str> = c
            .svg
            .match_indices("radar-band")
            .map(|(i, _)| &c.svg[i..i + 40])
            .collect();
        assert_eq!(bands.len(), 2);
    }

    #[test]
    fn the_normal_distribution_is_right_to_four_places() {
        assert!((normal_cdf(0.0) - 0.5).abs() < 1e-4);
        assert!((normal_cdf(1.0) - 0.8413).abs() < 1e-4);
        assert!((normal_cdf(-1.96) - 0.0250).abs() < 1e-4);
    }
}
