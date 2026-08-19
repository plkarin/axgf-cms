//! Presentation helpers.
//!
//! Everything here turns a structure the library produced into something a
//! browser can show. There is no genealogy logic in this module: no parsing of
//! source data, no merging, no validation. `axgf-rs` decides what a date *is*;
//! this module decides how it *reads*.
//!
//! The two showcase primitives live here:
//!
//! * [`render_date`] — renders every AXGF date shape honestly, so an
//!   un-pinned date looks un-pinned instead of being flattened into a blank
//!   field or a fabricated precision.
//! * [`Confidence`] — turns a 0.0–1.0 score into something visible at a
//!   glance, because a number in fine print is not a differentiator.

use axgf_rs::model::common::AxgfDate;
use minijinja::value::Value as MjValue;
use minijinja::Environment;
use serde::Serialize;
use serde_json::Value;

/// The current UTC instant as an ISO 8601 string, e.g. `2026-08-19T20:48:03Z`.
///
/// The same shape AXGF timestamps use, so an account's `created_at` and a
/// journal entry read like the bundle's own fields rather than like a
/// different system's.
pub fn now_iso8601() -> String {
    time::OffsetDateTime::now_utc()
        .format(&time::format_description::well_known::Rfc3339)
        .unwrap_or_else(|_| "1970-01-01T00:00:00Z".into())
        .replace("+00:00", "Z")
}

// ---------------------------------------------------------------------------
// Confidence
// ---------------------------------------------------------------------------

/// A confidence score prepared for display.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Confidence {
    /// The raw score, 0.0–1.0.
    pub value: f64,
    /// Percentage 0–100, for bar widths and `aria-valuenow`.
    pub percent: u8,
    /// CSS band: `certain`, `high`, `medium`, `low`.
    pub band: &'static str,
    /// Short human label, e.g. "85% confident".
    pub label: String,
    /// Longer text for `title`/screen readers.
    pub description: String,
}

impl Confidence {
    /// Prepare a score for display. Values outside 0.0–1.0 are clamped rather
    /// than rejected: a malformed bundle should still render.
    pub fn new(value: f64) -> Self {
        let v = if value.is_finite() {
            value.clamp(0.0, 1.0)
        } else {
            0.0
        };
        let percent = (v * 100.0).round() as u8;
        let (band, adjective) = match v {
            x if x >= 0.90 => ("certain", "effectively certain"),
            x if x >= 0.75 => ("high", "well supported"),
            x if x >= 0.50 => ("medium", "plausible but unconfirmed"),
            _ => ("low", "speculative"),
        };
        Self {
            value: v,
            percent,
            band,
            label: format!("{percent}% confident"),
            description: format!("Confidence {percent}% — {adjective}"),
        }
    }

    /// Read a confidence from an object field, if present and numeric.
    pub fn from_field(obj: &Value, key: &str) -> Option<Self> {
        obj.get(key).and_then(Value::as_f64).map(Confidence::new)
    }
}

// ---------------------------------------------------------------------------
// Dates
// ---------------------------------------------------------------------------

/// A date prepared for display.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct DateDisplay {
    /// The prose rendering, e.g. "circa 1500", "between 1920 and 1925".
    pub text: String,
    /// What kind of statement this is, for styling:
    /// `exact`, `approximate`, `range`, `preserved`, `unknown`.
    pub kind: &'static str,
    /// True when the date is anything less than a pinned calendar day.
    pub is_uncertain: bool,
    /// A short year-ish string for compact contexts (tree cards). Empty when
    /// no year can be stated honestly.
    pub short: String,
    /// Sort key as `YYYYMMDD`, from the earliest moment the date can refer to.
    /// `None` when the record gives nothing to order by — which is why this is
    /// an `Option` rather than a zero: an undated fact must sort *after* every
    /// dated one on a timeline, not before the year 1.
    pub sort: Option<i64>,
    /// Non-Gregorian calendar name, when the date declares one.
    pub calendar: Option<String>,
    /// Alternative renderings in other calendars.
    pub alternatives: Vec<String>,
    /// Text the converter could not parse, preserved rather than dropped.
    pub note: Option<String>,
    /// Confidence carried by the date itself, when present.
    pub confidence: Option<Confidence>,
}

impl DateDisplay {
    /// The rendering used when a date object is absent entirely.
    pub fn absent() -> Self {
        Self {
            text: "Not recorded".into(),
            kind: "unknown",
            is_uncertain: true,
            short: String::new(),
            sort: None,
            calendar: None,
            alternatives: Vec::new(),
            note: None,
            confidence: None,
        }
    }
}

/// Render any AXGF date object for display.
///
/// Handles every shape the format allows, in the order the format resolves
/// them:
///
/// | shape | example JSON | renders as |
/// |---|---|---|
/// | exact | `{"value":"1923-04-12","precision":"exact"}` | 12 April 1923 |
/// | month | `{"value":"1923-04","precision":"month"}` | April 1923 |
/// | year | `{"value":"1923","precision":"year"}` | 1923 |
/// | decade | `{"value":"1920","precision":"decade"}` | the 1920s |
/// | quarter century | `{"value":"1920","precision":"quarter_century"}` | the first quarter of the 20th century |
/// | century | `{"value":"1800","precision":"century"}` | the 19th century |
/// | circa | `{"value":"1500","circa":true}` | circa 1500 |
/// | range, both bounds | `{"range":{"earliest":…,"latest":…}}` | between 1920 and 1925 |
/// | range, upper only | `{"range":{"latest":…}}` | before 1430 |
/// | range, lower only | `{"range":{"earliest":…}}` | after 1920 |
/// | unparseable | `{"precision":"unknown","note":"Michaelmas"}` | preserved verbatim |
/// | unknown | `{"precision":"unknown"}` | Date unknown |
///
/// A range is checked before `value` because the converter emits ranged dates
/// with `precision: "unknown"` and no top-level value.
pub fn render_date(raw: &Value) -> DateDisplay {
    if raw.is_null() {
        return DateDisplay::absent();
    }
    // The library owns this shape; deserializing into its own type means a
    // format change surfaces here as a compile error rather than a silent
    // mis-render. An unexpected shape degrades to "absent" instead of failing.
    let Ok(date) = serde_json::from_value::<AxgfDate>(raw.clone()) else {
        return DateDisplay::absent();
    };

    let confidence = date.confidence.map(Confidence::new);
    let calendar = date
        .calendar
        .as_deref()
        .filter(|c| !c.eq_ignore_ascii_case("gregorian"))
        .map(pretty_calendar);
    let alternatives = date
        .alternatives
        .iter()
        .map(|a| format!("{} ({})", a.value, pretty_calendar(&a.calendar)))
        .collect::<Vec<_>>();

    let mut out = DateDisplay {
        text: String::new(),
        kind: "unknown",
        is_uncertain: true,
        short: String::new(),
        sort: None,
        calendar,
        alternatives,
        note: date.note.clone().filter(|n| !n.trim().is_empty()),
        confidence,
    };

    // 1. A range is a statement about bounds, not a point.
    if let Some(range) = &date.range {
        let lo = range.earliest.as_deref().and_then(point_text);
        let hi = range.latest.as_deref().and_then(point_text);
        match (lo, hi) {
            (Some(a), Some(b)) => {
                out.text = format!("between {a} and {b}");
                out.short = format!("{}–{}", year_of(&a), year_of(&b));
                out.sort = range.earliest.as_deref().and_then(sort_key_of);
            }
            // Plain characters, not HTML entities: templates autoescape, so an
            // entity here would render literally as "&gt;".
            (Some(a), None) => {
                out.text = format!("after {a}");
                out.short = format!(">{}", year_of(&a));
                out.sort = range.earliest.as_deref().and_then(sort_key_of);
            }
            (None, Some(b)) => {
                out.text = format!("before {b}");
                out.short = format!("<{}", year_of(&b));
                out.sort = range.latest.as_deref().and_then(sort_key_of);
            }
            (None, None) => {}
        }
        if !out.text.is_empty() {
            if let Some(n) = range.note.as_deref().filter(|n| !n.trim().is_empty()) {
                out.note = Some(n.to_string());
            }
            out.kind = "range";
            return out;
        }
    }

    // 2. A point value, formatted at whatever precision the source supports.
    if let Some(v) = date.value.as_deref().filter(|v| !v.trim().is_empty()) {
        let precision = date.precision.as_deref().unwrap_or("exact");
        let body = format_point(v, precision);
        let circa = date.circa.unwrap_or(false);
        out.text = if circa { format!("circa {body}") } else { body };
        out.short = year_of(v);
        out.sort = sort_key(v);
        out.is_uncertain = circa || precision != "exact";
        out.kind = if circa {
            "approximate"
        } else if precision == "exact" {
            "exact"
        } else {
            "approximate"
        };
        return out;
    }

    // 3. Text the converter could not parse, kept rather than discarded.
    if let Some(note) = &out.note {
        out.text = format!("recorded as “{note}”");
        out.kind = "preserved";
        return out;
    }

    // 4. Genuinely unknown.
    out.text = "Date unknown".into();
    out.kind = "unknown";
    out
}

/// Convenience: render `obj[key]` as a date, or the absent rendering.
pub fn render_date_field(obj: &Value, key: &str) -> DateDisplay {
    match obj.get(key) {
        Some(v) => render_date(v),
        None => DateDisplay::absent(),
    }
}

/// The sort key of a nested range bound.
fn sort_key_of(d: &AxgfDate) -> Option<i64> {
    d.value.as_deref().and_then(sort_key)
}

/// `YYYYMMDD` for an ISO-ish value, padding the parts the source omits with
/// zeroes so "1923" sorts before "1923-04-12" rather than after it.
fn sort_key(value: &str) -> Option<i64> {
    let (y, m, d) = split_ymd(value);
    let y = y?;
    Some(y * 10_000 + i64::from(m.unwrap_or(0)) * 100 + i64::from(d.unwrap_or(0)))
}

/// Render a nested range bound to prose, if it says anything at all.
fn point_text(d: &AxgfDate) -> Option<String> {
    if let Some(v) = d.value.as_deref().filter(|v| !v.trim().is_empty()) {
        let p = d.precision.as_deref().unwrap_or("year");
        let body = format_point(v, p);
        return Some(if d.circa.unwrap_or(false) {
            format!("circa {body}")
        } else {
            body
        });
    }
    d.note.clone().filter(|n| !n.trim().is_empty())
}

/// Format an ISO-ish value at the stated precision.
///
/// Degrades gracefully: a value with fewer parts than the precision claims is
/// rendered at the precision it actually has, never padded into a date the
/// source never asserted.
fn format_point(value: &str, precision: &str) -> String {
    let v = value.trim();
    match precision {
        "exact" => match split_ymd(v) {
            (Some(y), Some(m), Some(d)) => format!("{d} {} {y}", month_name(m)),
            (Some(y), Some(m), None) => format!("{} {y}", month_name(m)),
            (Some(y), None, None) => y.to_string(),
            _ => v.to_string(),
        },
        "month" => match split_ymd(v) {
            (Some(y), Some(m), _) => format!("{} {y}", month_name(m)),
            (Some(y), None, _) => y.to_string(),
            _ => v.to_string(),
        },
        "year" => match split_ymd(v) {
            (Some(y), _, _) => y.to_string(),
            _ => v.to_string(),
        },
        "decade" => match split_ymd(v) {
            (Some(y), _, _) => format!("the {}s", (y / 10) * 10),
            _ => format!("the {v}s"),
        },
        "quarter_century" => match split_ymd(v) {
            (Some(y), _, _) => {
                let century = y / 100 + 1;
                let quarter = ((y % 100) / 25) as usize;
                let names = ["first", "second", "third", "fourth"];
                format!(
                    "the {} quarter of the {} century",
                    names[quarter.min(3)],
                    ordinal(century)
                )
            }
            _ => v.to_string(),
        },
        "century" => match split_ymd(v) {
            (Some(y), _, _) => format!("the {} century", ordinal(y / 100 + 1)),
            _ => v.to_string(),
        },
        // "unknown" with a value present is contradictory; show the value
        // rather than claiming to know nothing about it.
        _ => v.to_string(),
    }
}

/// Split an ISO-ish `YYYY[-MM[-DD]]` value into numeric parts.
fn split_ymd(v: &str) -> (Option<i64>, Option<u32>, Option<u32>) {
    // Tolerate a leading '-' for BCE years by parsing the year with its sign.
    let (sign, rest) = match v.strip_prefix('-') {
        Some(r) => (-1i64, r),
        None => (1i64, v),
    };
    let mut parts = rest.split('-');
    let y = parts
        .next()
        .and_then(|s| s.parse::<i64>().ok())
        .map(|y| y * sign);
    let m = parts.next().and_then(|s| s.parse::<u32>().ok());
    let d = parts.next().and_then(|s| s.parse::<u32>().ok());
    (y, m, d)
}

/// The year portion of a value or rendered string, for compact display.
fn year_of(v: &str) -> String {
    let cleaned = v.trim_start_matches("circa ").trim();
    // Rendered prose like "the 1920s" or "12 April 1923" — take the first
    // 3-or-4 digit run, which is the year in every rendering above.
    let mut best = String::new();
    let mut cur = String::new();
    for c in cleaned.chars() {
        if c.is_ascii_digit() {
            cur.push(c);
        } else {
            if cur.len() >= 3 && best.is_empty() {
                best = cur.clone();
            }
            cur.clear();
        }
    }
    if cur.len() >= 3 && best.is_empty() {
        best = cur;
    }
    best
}

fn month_name(m: u32) -> &'static str {
    const MONTHS: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    MONTHS
        .get(m.saturating_sub(1) as usize)
        .copied()
        .unwrap_or("")
}

fn ordinal(n: i64) -> String {
    let suffix = match (n % 10, n % 100) {
        (_, 11..=13) => "th",
        (1, _) => "st",
        (2, _) => "nd",
        (3, _) => "rd",
        _ => "th",
    };
    format!("{n}{suffix}")
}

fn pretty_calendar(c: &str) -> String {
    match c {
        "gregorian" => "Gregorian".into(),
        "julian" => "Julian".into(),
        "hebrew" => "Hebrew".into(),
        "hijri" => "Hijri".into(),
        "persian" => "Persian".into(),
        "chinese" => "Chinese".into(),
        "ethiopian" => "Ethiopian".into(),
        "japanese_era" => "Japanese era".into(),
        "republican_french" => "French Republican".into(),
        "roman" => "Roman".into(),
        other => other.replace('_', " "),
    }
}

// ---------------------------------------------------------------------------
// Small shared readers
// ---------------------------------------------------------------------------

/// The display name of a person entity, falling back through the name shape.
pub fn person_display_name(person: &Value) -> String {
    let identity = person.get("identity");
    let display = identity
        .and_then(|i| i.get("name"))
        .and_then(|n| n.get("display"))
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|s| !s.is_empty());
    match display {
        Some(d) => d.to_string(),
        None => "[Unnamed]".to_string(),
    }
}

/// The primary name of a place, preferring the flagged primary, then any name.
pub fn place_name(place: &Value) -> String {
    let names = place.get("names").and_then(Value::as_array);
    let Some(names) = names else {
        return "[Unknown place]".into();
    };
    let pick = names
        .iter()
        .find(|n| n.get("is_primary").and_then(Value::as_bool) == Some(true))
        .or_else(|| names.first());
    pick.and_then(|n| n.get("value"))
        .and_then(Value::as_str)
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("[Unknown place]")
        .to_string()
}

/// Human label for a source reliability level.
pub fn reliability_label(r: &str) -> &'static str {
    match r {
        "primary" => "Primary source",
        "secondary" => "Secondary source",
        "derivative" => "Derivative",
        "authored" => "Authored work",
        "oral" => "Oral tradition",
        _ => "Reliability unknown",
    }
}

/// How much weight a reliability level carries, for styling. Higher is
/// stronger evidence.
pub fn reliability_rank(r: &str) -> u8 {
    match r {
        "primary" => 4,
        "secondary" => 3,
        "derivative" | "authored" => 2,
        "oral" => 1,
        _ => 0,
    }
}

/// Register the template filters used across the templates.
pub fn register_filters(env: &mut Environment<'static>) {
    env.add_filter("confidence", |v: f64| -> MjValue {
        MjValue::from_serialize(Confidence::new(v))
    });
    env.add_filter("ordinal", |n: i64| ordinal(n));
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn d(v: Value) -> DateDisplay {
        render_date(&v)
    }

    // -- every date shape --------------------------------------------------

    #[test]
    fn exact_date_reads_as_a_day() {
        let r = d(json!({"value":"1923-04-12","precision":"exact"}));
        assert_eq!(r.text, "12 April 1923");
        assert_eq!(r.kind, "exact");
        assert!(!r.is_uncertain);
        assert_eq!(r.short, "1923");
    }

    #[test]
    fn month_precision_omits_the_day() {
        let r = d(json!({"value":"1923-04","precision":"month"}));
        assert_eq!(r.text, "April 1923");
        assert!(r.is_uncertain);
    }

    #[test]
    fn year_precision_is_just_the_year() {
        let r = d(json!({"value":"1911","precision":"year"}));
        assert_eq!(r.text, "1911");
        assert_eq!(r.kind, "approximate");
    }

    #[test]
    fn decade_precision_reads_as_a_decade() {
        assert_eq!(
            d(json!({"value":"1924","precision":"decade"})).text,
            "the 1920s"
        );
    }

    #[test]
    fn quarter_century_reads_as_a_quarter() {
        let r = d(json!({"value":"1920","precision":"quarter_century"}));
        assert_eq!(r.text, "the first quarter of the 20th century");
        let r = d(json!({"value":"1880","precision":"quarter_century"}));
        assert_eq!(r.text, "the fourth quarter of the 19th century");
    }

    #[test]
    fn century_precision_reads_as_a_century() {
        assert_eq!(
            d(json!({"value":"1800","precision":"century"})).text,
            "the 19th century"
        );
        assert_eq!(
            d(json!({"value":"1900","precision":"century"})).text,
            "the 20th century"
        );
    }

    #[test]
    fn circa_is_stated_not_hidden() {
        let r = d(json!({"value":"1500","circa":true,"precision":"year"}));
        assert_eq!(r.text, "circa 1500");
        assert_eq!(r.kind, "approximate");
        assert!(r.is_uncertain);
    }

    #[test]
    fn range_with_both_bounds_reads_as_between() {
        let r = d(json!({"precision":"unknown","range":{
            "earliest":{"value":"1920","precision":"year"},
            "latest":{"value":"1925","precision":"year"}}}));
        assert_eq!(r.text, "between 1920 and 1925");
        assert_eq!(r.kind, "range");
        assert!(r.is_uncertain);
    }

    #[test]
    fn range_with_only_a_latest_reads_as_before() {
        let r = d(json!({"precision":"unknown","range":{
            "latest":{"value":"1430","precision":"year"}}}));
        assert_eq!(r.text, "before 1430");
        assert_eq!(r.kind, "range");
        // The compact form holds a plain character; templates escape it.
        assert_eq!(r.short, "<1430");
    }

    #[test]
    fn range_with_only_an_earliest_reads_as_after() {
        let r = d(json!({"precision":"unknown","range":{
            "earliest":{"value":"1920","precision":"year"}}}));
        assert_eq!(r.text, "after 1920");
    }

    #[test]
    fn range_note_is_surfaced() {
        let r = d(json!({"precision":"unknown","range":{
            "earliest":{"value":"1920","precision":"year"},
            "latest":{"value":"1925","precision":"year"},
            "note":"based on census records"}}));
        assert_eq!(r.note.as_deref(), Some("based on census records"));
    }

    #[test]
    fn unparseable_text_is_preserved_verbatim() {
        let r = d(json!({"precision":"unknown","note":"Michaelmas term, 3 Edw. III"}));
        assert_eq!(r.kind, "preserved");
        assert!(r.text.contains("Michaelmas term, 3 Edw. III"));
        assert_eq!(r.note.as_deref(), Some("Michaelmas term, 3 Edw. III"));
    }

    #[test]
    fn unknown_date_says_so_rather_than_rendering_blank() {
        let r = d(json!({"precision":"unknown"}));
        assert_eq!(r.text, "Date unknown");
        assert_eq!(r.kind, "unknown");
        assert!(!r.text.is_empty(), "never render an empty field");
    }

    #[test]
    fn absent_date_object_is_not_a_blank() {
        assert_eq!(render_date(&Value::Null).text, "Not recorded");
        assert_eq!(DateDisplay::absent().kind, "unknown");
    }

    #[test]
    fn non_gregorian_calendar_is_named() {
        let r = d(json!({"value":"5683-01-25","precision":"exact","calendar":"hebrew"}));
        assert_eq!(r.calendar.as_deref(), Some("Hebrew"));
        // Gregorian is the default and should not be called out.
        let g = d(json!({"value":"1923","precision":"year","calendar":"gregorian"}));
        assert_eq!(g.calendar, None);
    }

    #[test]
    fn alternatives_in_other_calendars_are_listed() {
        let r = d(json!({"value":"1923-04-12","precision":"exact",
            "alternatives":[{"value":"5683-01-25","calendar":"hebrew"}]}));
        assert_eq!(r.alternatives, vec!["5683-01-25 (Hebrew)"]);
    }

    #[test]
    fn date_confidence_is_carried_through() {
        let r = d(json!({"value":"1923","precision":"year","confidence":0.35}));
        let c = r.confidence.expect("confidence present");
        assert_eq!(c.band, "low");
        assert_eq!(c.percent, 35);
    }

    #[test]
    fn value_shorter_than_its_precision_claims_is_not_padded() {
        // "exact" but only a year present: render the year, never invent a day.
        let r = d(json!({"value":"1923","precision":"exact"}));
        assert_eq!(r.text, "1923");
    }

    #[test]
    fn garbage_shape_degrades_instead_of_panicking() {
        assert_eq!(render_date(&json!("1923")).text, "Not recorded");
        assert_eq!(render_date(&json!([1, 2])).text, "Not recorded");
    }

    #[test]
    fn empty_value_string_falls_through_to_unknown() {
        assert_eq!(
            d(json!({"value":"   ","precision":"year"})).text,
            "Date unknown"
        );
    }

    // -- confidence --------------------------------------------------------

    #[test]
    fn confidence_bands_separate_certain_from_speculative() {
        assert_eq!(Confidence::new(0.98).band, "certain");
        assert_eq!(Confidence::new(0.80).band, "high");
        assert_eq!(Confidence::new(0.60).band, "medium");
        assert_eq!(Confidence::new(0.35).band, "low");
        // The showcase case: 0.98 and 0.35 must not look the same.
        assert_ne!(Confidence::new(0.98).band, Confidence::new(0.35).band);
    }

    #[test]
    fn confidence_is_clamped_not_rejected() {
        assert_eq!(Confidence::new(-5.0).percent, 0);
        assert_eq!(Confidence::new(9.0).percent, 100);
        assert_eq!(Confidence::new(f64::NAN).percent, 0);
    }

    #[test]
    fn confidence_reads_from_a_field() {
        let obj = json!({"confidence": 0.42});
        assert_eq!(
            Confidence::from_field(&obj, "confidence").unwrap().percent,
            42
        );
        assert!(Confidence::from_field(&obj, "missing").is_none());
    }

    // -- small readers -----------------------------------------------------

    #[test]
    fn person_name_falls_back_when_missing() {
        assert_eq!(
            person_display_name(&json!({"identity":{"name":{"display":"Ada"}}})),
            "Ada"
        );
        assert_eq!(person_display_name(&json!({"identity":{}})), "[Unnamed]");
        assert_eq!(person_display_name(&json!({})), "[Unnamed]");
    }

    #[test]
    fn place_name_prefers_the_primary() {
        let p = json!({"names":[
            {"lang":"pl","value":"Warszawa"},
            {"lang":"en","value":"Warsaw","is_primary":true}]});
        assert_eq!(place_name(&p), "Warsaw");
        let q = json!({"names":[{"lang":"pl","value":"Kraków"}]});
        assert_eq!(place_name(&q), "Kraków");
        assert_eq!(place_name(&json!({})), "[Unknown place]");
    }

    #[test]
    fn reliability_ranks_primary_above_oral() {
        assert!(reliability_rank("primary") > reliability_rank("oral"));
        assert_eq!(
            reliability_label("dna_unknown_value"),
            "Reliability unknown"
        );
    }

    #[test]
    fn ordinals_handle_the_teens() {
        assert_eq!(ordinal(1), "1st");
        assert_eq!(ordinal(11), "11th");
        assert_eq!(ordinal(21), "21st");
        assert_eq!(ordinal(13), "13th");
        assert_eq!(ordinal(20), "20th");
    }
}
