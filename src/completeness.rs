//! What a bundle records, against what AXGF can hold.
//!
//! # Why this exists
//!
//! The visitor's most likely first action — convert my GEDCOM, look at it — is
//! the path that shows the format at its weakest. Measured on the operator's
//! real 767-person file, a GEDCOM import produces:
//!
//! * every fact at the same confidence, because the converter stamps one value
//!   on everything and GEDCOM carried none;
//! * no parentage confidence at all;
//! * zero links, because GEDCOM cannot express a godparent;
//! * occupations with titles and no dates, so no span exists to draw.
//!
//! A hand-authored demo does not answer that, because a sceptical reader
//! assumes a curated demo is curated. What answers it is the visitor's own
//! data, counted honestly: here is what your file carried, here is the field
//! it would live in, and here is why it is empty.
//!
//! Nothing here is marketing. Every number is a count of the bundle in front
//! of the reader, and when a bundle *does* carry rich data the report says so
//! rather than manufacturing a gap.

use serde::Serialize;
use serde_json::Value;

use crate::view;

/// Where a field is defined in the specification.
const SPEC: &str = "https://github.com/plkarin/axgf-spec/blob/main/SPEC_1.0.md";

/// One field of AXGF, and how much of it this bundle uses.
#[derive(Debug, Clone, Serialize)]
pub struct Metric {
    /// Plain description of what is being counted.
    pub title: String,
    /// The AXGF field itself, named exactly as it appears in a bundle.
    pub field: &'static str,
    /// Link into the specification section that defines it.
    pub spec_url: String,
    /// Section label, e.g. "§4.4".
    pub spec_ref: &'static str,
    pub present: usize,
    pub total: usize,
    /// `present of total`, or just `present` when a total is meaningless.
    pub summary: String,
    /// One plain sentence about what the numbers mean here.
    pub note: String,
    /// True when GEDCOM has no way to express this at all, so an import can
    /// only ever produce zero.
    pub gedcom_cannot: bool,
    /// True when this bundle populates the field at all.
    pub carried: bool,
}

/// How many dates of each shape the bundle holds.
#[derive(Debug, Clone, Serialize, Default)]
pub struct DateShapes {
    pub exact: usize,
    pub approximate: usize,
    pub range: usize,
    pub preserved: usize,
    pub unknown: usize,
    pub total: usize,
}

/// The full readout.
#[derive(Debug, Clone, Serialize)]
pub struct Report {
    pub metrics: Vec<Metric>,
    pub dates: DateShapes,
    /// A sentence stating what this bundle carried, without overstating.
    pub headline: String,
    /// How many of the metrics this bundle populates.
    pub carried: usize,
    /// How many are empty.
    pub empty: usize,
    pub spec_url: String,
}

fn obj<'a>(flat: &'a Value, key: &str) -> Option<&'a serde_json::Map<String, Value>> {
    flat.get(key).and_then(Value::as_object)
}

/// Every place a confidence score can live, flattened into one list of values.
///
/// Counting these together is what makes the "one value stamped on everything"
/// pattern visible: a converter sets the same number in all of them.
fn confidence_values(flat: &Value) -> (Vec<f64>, usize) {
    let mut values = Vec::new();
    let mut slots = 0usize;

    let mut take = |holder: Option<&Value>| {
        if let Some(h) = holder.filter(|v| !v.is_null()) {
            slots += 1;
            if let Some(c) = h.get("confidence").and_then(Value::as_f64) {
                values.push(c);
            }
        }
    };

    if let Some(persons) = obj(flat, "persons") {
        for p in persons.values() {
            take(p.get("birth"));
            take(p.get("death"));
            if let Some(names) = p.pointer("/identity/names").and_then(Value::as_array) {
                for n in names {
                    take(Some(n));
                }
            }
        }
    }
    if let Some(families) = obj(flat, "families") {
        for f in families.values() {
            take(f.get("union"));
            if let Some(children) = f.get("children").and_then(Value::as_array) {
                for c in children {
                    take(Some(c));
                }
            }
        }
    }
    for key in ["events", "links", "occupations", "sources"] {
        if let Some(c) = obj(flat, key) {
            for e in c.values() {
                take(Some(e));
            }
        }
    }

    (values, slots)
}

/// Classify every date in the bundle by the shape it actually has.
fn date_shapes(flat: &Value) -> DateShapes {
    let mut out = DateShapes::default();
    let mut classify = |holder: Option<&Value>| {
        let Some(h) = holder.filter(|v| !v.is_null()) else {
            return;
        };
        if h.get("date").is_none() {
            return;
        }
        let d = view::render_date_field(h, "date");
        out.total += 1;
        match d.kind {
            "exact" => out.exact += 1,
            "approximate" => out.approximate += 1,
            "range" => out.range += 1,
            "preserved" => out.preserved += 1,
            _ => out.unknown += 1,
        }
    };

    if let Some(persons) = obj(flat, "persons") {
        for p in persons.values() {
            classify(p.get("birth"));
            classify(p.get("death"));
        }
    }
    if let Some(families) = obj(flat, "families") {
        for f in families.values() {
            classify(f.pointer("/union/start"));
            classify(f.pointer("/union/end"));
        }
    }
    if let Some(events) = obj(flat, "events") {
        for e in events.values() {
            classify(Some(e));
        }
    }
    for key in ["occupations", "links"] {
        if let Some(c) = obj(flat, key) {
            for e in c.values() {
                classify(e.get("valid_from"));
                classify(e.get("valid_until"));
            }
        }
    }
    for key in ["sources", "documents"] {
        if let Some(c) = obj(flat, key) {
            for e in c.values() {
                classify(Some(e));
            }
        }
    }
    out
}

/// The most common value in a list, with how many times it occurs.
fn modal(values: &[f64]) -> Option<(f64, usize)> {
    if values.is_empty() {
        return None;
    }
    // Bucket on two decimal places: confidences are authored to that precision
    // and exact float equality would split 0.8 from 0.8000000001.
    let mut counts: std::collections::BTreeMap<i64, usize> = Default::default();
    for v in values {
        *counts.entry((v * 100.0).round() as i64).or_default() += 1;
    }
    counts
        .into_iter()
        .max_by_key(|&(k, n)| (n, std::cmp::Reverse(k)))
        .map(|(k, n)| (k as f64 / 100.0, n))
}

/// Analyse a flat bundle.
pub fn analyse(flat: &Value) -> Report {
    let mut metrics = Vec::new();

    // --- 1. Confidence that was assessed, not stamped ---------------------
    let (values, slots) = confidence_values(flat);
    let with_conf = values.len();
    let distinct = {
        let mut v: Vec<i64> = values.iter().map(|x| (x * 100.0).round() as i64).collect();
        v.sort_unstable();
        v.dedup();
        v.len()
    };
    let (modal_value, modal_count) = modal(&values).unwrap_or((0.0, 0));
    let assessed = with_conf.saturating_sub(modal_count);

    let conf_note = if with_conf == 0 {
        format!(
            "None of the {slots} facts here carries a confidence score at all. \
             AXGF treats a fact without one as incomplete."
        )
    } else if distinct <= 1 {
        format!(
            "{with_conf} of {slots} facts carry a score, and every one of them is \
             the same number ({modal_value:.2}). That is what a bulk import \
             writes: the converter had to put something there, because the GEDCOM \
             said nothing about how sure anyone was. None has been judged \
             individually yet."
        )
    } else if assessed * 4 < with_conf {
        format!(
            "{with_conf} of {slots} facts carry a score. {modal_count} share one \
             value ({modal_value:.2}), which is the mark of a bulk import; \
             {assessed} differ from it and so have been looked at individually."
        )
    } else {
        format!(
            "{with_conf} of {slots} facts carry a score, {assessed} of them \
             differing from the commonest value ({modal_value:.2}) across \
             {distinct} distinct levels. This bundle records genuine, varying \
             uncertainty."
        )
    };

    metrics.push(Metric {
        title: "Facts with an individually judged confidence".into(),
        field: "confidence",
        spec_url: format!("{SPEC}#8-confidence-model"),
        spec_ref: "§8",
        present: assessed,
        // Measured against the facts that carry a score at all: a score that
        // is simply the import default is not a judgement about that fact.
        total: with_conf,
        summary: format!("{assessed} of {with_conf}"),
        note: conf_note,
        // GEDCOM has no confidence field, but a converter can stamp a default,
        // so this is not a hard zero.
        gedcom_cannot: false,
        carried: assessed > 0,
    });

    // --- 2. Parentage confidence ------------------------------------------
    let mut child_slots = 0usize;
    let mut child_conf = 0usize;
    if let Some(families) = obj(flat, "families") {
        for f in families.values() {
            if let Some(children) = f.get("children").and_then(Value::as_array) {
                for c in children {
                    child_slots += 1;
                    if c.get("confidence").and_then(Value::as_f64).is_some() {
                        child_conf += 1;
                    }
                }
            }
        }
    }
    metrics.push(Metric {
        title: "Parent–child links with their own confidence".into(),
        field: "family.children[].confidence",
        spec_url: format!("{SPEC}#42-family"),
        spec_ref: "§4.2",
        present: child_conf,
        total: child_slots,
        summary: format!("{child_conf} of {child_slots}"),
        note: if child_conf == 0 {
            "In GEDCOM a person is a child of a family or they are not; there is \
             nowhere to say the connection is only probable. AXGF scores each \
             parentage separately, and the tree draws a less certain one as a \
             fainter line."
                .into()
        } else {
            format!(
                "{child_conf} parentages carry their own score, so a speculative \
                 line is visibly weaker than a documented one."
            )
        },
        gedcom_cannot: true,
        carried: child_conf > 0,
    });

    // --- 3. Non-family links ----------------------------------------------
    let links = obj(flat, "links").map(|m| m.len()).unwrap_or(0);
    metrics.push(Metric {
        title: "Non-family relationships".into(),
        field: "links",
        spec_url: format!("{SPEC}#44-link"),
        spec_ref: "§4.4",
        present: links,
        total: links,
        summary: format!("{links}"),
        note: if links == 0 {
            "Godparent, employer, witness, mentor, guardian. GEDCOM has no record \
             type for any of them, so an import can only ever produce none. In \
             AXGF each is an entity with its own dates, source and confidence."
                .into()
        } else {
            format!("{links} recorded, each with its own dates, source and confidence.")
        },
        gedcom_cannot: true,
        carried: links > 0,
    });

    // --- 4. Occupations as spans ------------------------------------------
    let mut occ_total = 0usize;
    let mut occ_span = 0usize;
    if let Some(occs) = obj(flat, "occupations") {
        occ_total = occs.len();
        occ_span = occs
            .values()
            .filter(|o| {
                let has = |k: &str| {
                    o.get(k)
                        .filter(|v| !v.is_null())
                        .and_then(|v| v.get("date"))
                        .is_some()
                };
                has("valid_from") && has("valid_until")
            })
            .count();
    }
    metrics.push(Metric {
        title: "Occupations recorded as a span".into(),
        field: "occupation.valid_from / valid_until",
        spec_url: format!("{SPEC}#45-occupation"),
        spec_ref: "§4.5",
        present: occ_span,
        total: occ_total,
        summary: format!("{occ_span} of {occ_total}"),
        note: if occ_total == 0 {
            "No occupations in this bundle.".into()
        } else if occ_span == 0 {
            "GEDCOM's OCCU carries a title and at most one date, so these arrived \
             as bare titles. An AXGF occupation is a state with a start and an \
             end — “schoolteacher, 1948–1978” — which is what lets it be drawn as \
             a bar rather than a bullet point."
                .into()
        } else {
            format!(
                "{occ_span} of {occ_total} have both bounds, so they render as \
                 comparable bars on one timeline."
            )
        },
        gedcom_cannot: false,
        carried: occ_span > 0,
    });

    // --- 5. Source reliability --------------------------------------------
    let mut src_total = 0usize;
    let mut src_graded = 0usize;
    if let Some(sources) = obj(flat, "sources") {
        src_total = sources.len();
        src_graded = sources
            .values()
            .filter(|s| {
                s.get("reliability")
                    .and_then(Value::as_str)
                    .is_some_and(|r| !r.is_empty() && r != "unknown")
            })
            .count();
    }
    metrics.push(Metric {
        title: "Sources graded for reliability".into(),
        field: "source.reliability",
        spec_url: format!("{SPEC}#542-reliability-levels"),
        spec_ref: "§5.4.2",
        present: src_graded,
        total: src_total,
        summary: format!("{src_graded} of {src_total}"),
        note: if src_total == 0 {
            "No sources in this bundle. GEDCOM's SOUR records carry a title, but \
             nothing that distinguishes a birth certificate from a family story."
                .into()
        } else if src_graded == 0 {
            "These sources have titles but no grade. AXGF asks whether a source is \
             primary, secondary, derivative, authored, oral or DNA, so a claim \
             resting on a certificate is visibly not the same as one resting on \
             recollection."
                .into()
        } else {
            format!("{src_graded} of {src_total} declare how strong they are.")
        },
        gedcom_cannot: false,
        carried: src_graded > 0,
    });

    let dates = date_shapes(flat);
    let carried = metrics.iter().filter(|m| m.carried).count();
    let empty = metrics.len() - carried;

    let headline = if empty == 0 {
        "This bundle populates every field below. Nothing here is empty — the \
         data already uses what the format offers."
            .to_string()
    } else if carried == 0 {
        format!(
            "None of the {} fields below carry data. That is the normal result of \
             a GEDCOM import: the format it came from has nowhere to put any of \
             it.",
            metrics.len()
        )
    } else {
        format!(
            "This bundle carries {carried} of the {} fields below; {empty} are \
             empty.",
            metrics.len()
        )
    };

    Report {
        metrics,
        dates,
        headline,
        carried,
        empty,
        spec_url: SPEC.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// A bundle shaped like a GEDCOM import: one stamped confidence, no
    /// parentage scores, no links, undated occupations, ungraded sources.
    fn imported() -> Value {
        json!({
            "manifest": {"axgf": "1.0"},
            "persons": {
                "p1": {"id": "p1", "type": "person", "axgf_version": "1.0",
                       "identity": {"name": {"display": "A"}},
                       "birth": {"date": {"value": "1900", "precision": "year"},
                                 "confidence": 0.8},
                       "death": {"date": {"value": "1960", "precision": "year"},
                                 "confidence": 0.8}},
                "p2": {"id": "p2", "type": "person", "axgf_version": "1.0",
                       "identity": {"name": {"display": "B"}},
                       "birth": {"date": {"value": "1925-04-12", "precision": "exact"},
                                 "confidence": 0.8}}
            },
            "families": {
                "f1": {"id": "f1", "type": "family", "axgf_version": "1.0",
                       "union": {"type": "marriage",
                                 "persons": [{"person_id": "p1", "role": "spouse"}]},
                       "children": [{"person_id": "p2"}]}
            },
            "links": {},
            "occupations": {
                "o1": {"id": "o1", "type": "occupation", "axgf_version": "1.0",
                       "person_id": "p1", "title": "Farmer", "confidence": 0.8}
            },
            "sources": {
                "s1": {"id": "s1", "type": "source", "axgf_version": "1.0",
                       "title": "A book", "source_type": "other",
                       "reliability": "unknown"}
            },
            "events": {}, "places": {}, "documents": {}
        })
    }

    fn metric<'a>(r: &'a Report, field: &str) -> &'a Metric {
        r.metrics
            .iter()
            .find(|m| m.field == field)
            .unwrap_or_else(|| panic!("no metric for {field}"))
    }

    #[test]
    fn an_imported_bundle_reports_every_gap() {
        let r = analyse(&imported());

        // A single stamped confidence is not an assessment.
        let c = metric(&r, "confidence");
        assert_eq!(
            c.present, 0,
            "one repeated value means nothing was assessed"
        );
        assert!(
            c.note.contains("0.80"),
            "the stamped value is named: {}",
            c.note
        );
        assert!(c.note.contains("bulk import"));

        assert_eq!(metric(&r, "family.children[].confidence").present, 0);
        assert_eq!(metric(&r, "links").present, 0);
        assert_eq!(metric(&r, "occupation.valid_from / valid_until").present, 0);
        assert_eq!(metric(&r, "source.reliability").present, 0);

        assert_eq!(r.carried, 0);
        assert_eq!(r.empty, r.metrics.len());
        assert!(r.headline.contains("GEDCOM import"), "{}", r.headline);
    }

    #[test]
    fn every_metric_names_a_field_and_links_the_spec() {
        let r = analyse(&imported());
        for m in &r.metrics {
            assert!(!m.field.is_empty(), "a metric must name its AXGF field");
            assert!(
                m.spec_url
                    .starts_with("https://github.com/plkarin/axgf-spec"),
                "{} has no spec link",
                m.field
            );
            assert!(m.spec_url.contains('#'), "{} links no section", m.field);
            assert!(m.spec_ref.starts_with('§'));
            assert!(!m.note.is_empty());
        }
    }

    #[test]
    fn the_gedcom_impossible_fields_are_marked_as_such() {
        let r = analyse(&imported());
        // These two can only ever be zero after an import.
        assert!(metric(&r, "links").gedcom_cannot);
        assert!(metric(&r, "family.children[].confidence").gedcom_cannot);
        // These can be populated by a converter, so they are not "impossible".
        assert!(!metric(&r, "source.reliability").gedcom_cannot);
        assert!(!metric(&r, "occupation.valid_from / valid_until").gedcom_cannot);
    }

    #[test]
    fn a_rich_bundle_is_not_told_it_is_poor() {
        // The honest framing cuts both ways: if the data is already good, the
        // report must say so rather than manufacturing a gap.
        let mut b = imported();
        b["persons"]["p1"]["birth"]["confidence"] = json!(0.35);
        b["persons"]["p1"]["death"]["confidence"] = json!(0.97);
        b["persons"]["p2"]["birth"]["confidence"] = json!(0.62);
        b["families"]["f1"]["children"][0]["confidence"] = json!(0.9);
        b["links"] = json!({
            "l1": {"id": "l1", "type": "link", "axgf_version": "1.0",
                   "from": {"entity_type": "person", "entity_id": "p1"},
                   "to": {"entity_type": "person", "entity_id": "p2"},
                   "label": "godfather", "confidence": 0.8}});
        b["occupations"]["o1"]["valid_from"] = json!({"date": {"value": "1920"}});
        b["occupations"]["o1"]["valid_until"] = json!({"date": {"value": "1955"}});
        b["sources"]["s1"]["reliability"] = json!("primary");

        let r = analyse(&b);
        assert_eq!(r.empty, 0, "nothing should be reported as missing");
        assert!(
            r.headline.contains("every field"),
            "a rich bundle deserves a different sentence: {}",
            r.headline
        );
        let c = metric(&r, "confidence");
        assert!(
            c.note.contains("genuine, varying uncertainty"),
            "{}",
            c.note
        );
    }

    #[test]
    fn date_shapes_are_counted_separately() {
        let mut b = imported();
        b["persons"]["p1"]["birth"]["date"] =
            json!({"value": "1500", "circa": true, "precision": "year"});
        b["persons"]["p1"]["death"]["date"] =
            json!({"precision": "unknown", "range": {"latest": {"value": "1560"}}});
        b["persons"]["p2"]["death"] =
            json!({"date": {"precision": "unknown", "note": "Michaelmas"}});

        let d = analyse(&b).dates;
        assert_eq!(d.exact, 1, "p2's birth is a full calendar day");
        assert_eq!(d.approximate, 1, "circa 1500");
        assert_eq!(d.range, 1, "before 1560");
        assert_eq!(d.preserved, 1, "unparseable text kept");
        assert_eq!(d.total, 4);
    }

    #[test]
    fn an_empty_bundle_reports_zeroes_without_panicking() {
        let b = json!({"manifest": {"axgf": "1.0"}});
        let r = analyse(&b);
        assert_eq!(r.carried, 0);
        assert_eq!(r.dates.total, 0);
        assert!(!r.metrics.is_empty(), "the fields are still named");
    }

    #[test]
    fn modal_buckets_to_two_decimals() {
        assert_eq!(modal(&[0.8, 0.8, 0.5]), Some((0.8, 2)));
        assert_eq!(modal(&[]), None);
        // Float noise must not split one value into two.
        let (v, n) = modal(&[0.8, 0.800000001, 0.7999999]).unwrap();
        assert_eq!(n, 3);
        assert!((v - 0.8).abs() < 1e-9);
    }

    #[test]
    fn confidence_slots_count_places_a_score_could_go() {
        // Two births, one death, one union, one child, one occupation, one
        // source = 7 slots in the imported fixture.
        let (values, slots) = confidence_values(&imported());
        assert_eq!(slots, 7, "slots: {slots}");
        assert_eq!(values.len(), 4, "only four actually carry one");
    }
}
