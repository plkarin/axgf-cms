//! Where a family's record could say more.
//!
//! # Why this exists
//!
//! A tree is never finished, and the useful question is not "is this correct"
//! but "what is thin". This counts the kinds of detail a record can carry —
//! how sure each fact is, how sure each parentage is, relationships beyond
//! blood and marriage, work with a start and an end, sources graded for
//! reliability — and says which of them this family's tree actually uses.
//!
//! It is a to-do list, not a report card. A blank row is somewhere the record
//! could grow, and the copy says so; nothing here is an error and nothing here
//! is manufactured. Every number is a count of the tree in front of the
//! reader, and where a tree already carries rich detail the readout says that
//! instead of inventing a gap.

use serde::Serialize;
use serde_json::Value;

use crate::view;

/// One kind of detail a record can carry, and how much of it this tree uses.
#[derive(Debug, Clone, Serialize)]
pub struct Metric {
    /// Plain description of what is being counted.
    pub title: String,
    /// A stable key for the row, for tests and for anything that needs to
    /// address one metric without matching on its prose.
    pub field: &'static str,
    pub present: usize,
    pub total: usize,
    /// `present of total`, or just `present` when a total is meaningless.
    pub summary: String,
    /// One plain sentence about what the numbers mean here, and why filling
    /// the gap would be worth the trouble.
    pub note: String,
    /// True when this tree records this kind of detail at all.
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

/// Analyse a flat bundle in the reader's language.
pub fn analyse(flat: &Value, lang: &str) -> Report {
    use fluent::FluentValue as F;
    let t = |key: &str, args: &[(&str, F<'_>)]| -> String {
        let mut a = fluent::FluentArgs::new();
        for (k, v) in args {
            a.set(*k, v.clone());
        }
        crate::i18n::translate(lang, key, Some(&a))
    };
    let n = |v: usize| F::from(v as i64);
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
    // Two decimals, formatted here rather than by Fluent: this is a confidence
    // score, and its precision is part of what it says.
    let modal_str = format!("{modal_value:.2}");

    let conf_note = if with_conf == 0 {
        t(
            "completeness-metric-confidence-none",
            &[("slots", n(slots))],
        )
    } else if distinct <= 1 {
        t(
            "completeness-metric-confidence-uniform",
            &[
                ("with", n(with_conf)),
                ("slots", n(slots)),
                ("modal", modal_str.clone().into()),
            ],
        )
    } else if assessed * 4 < with_conf {
        t(
            "completeness-metric-confidence-some",
            &[
                ("with", n(with_conf)),
                ("slots", n(slots)),
                ("modal_count", n(modal_count)),
                ("modal", modal_str.clone().into()),
                ("assessed", n(assessed)),
            ],
        )
    } else {
        t(
            "completeness-metric-confidence-many",
            &[
                ("with", n(with_conf)),
                ("slots", n(slots)),
                ("assessed", n(assessed)),
                ("modal", modal_str.clone().into()),
                ("distinct", n(distinct)),
            ],
        )
    };

    metrics.push(Metric {
        title: t("completeness-metric-confidence", &[]),
        field: "confidence",
        present: assessed,
        // Measured against the facts that carry a score at all: a score that
        // is simply an import's default is not a judgement about that fact.
        total: with_conf,
        summary: format!("{assessed} of {with_conf}"),
        note: conf_note,
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
        title: t("completeness-metric-parentage", &[]),
        field: "family.children[].confidence",
        present: child_conf,
        total: child_slots,
        summary: format!("{child_conf} of {child_slots}"),
        note: if child_conf == 0 {
            t("completeness-metric-parentage-none", &[])
        } else {
            t(
                "completeness-metric-parentage-some",
                &[("n", n(child_conf))],
            )
        },
        carried: child_conf > 0,
    });

    // --- 3. Relationships beyond blood and marriage -----------------------
    let links = obj(flat, "links").map(|m| m.len()).unwrap_or(0);
    metrics.push(Metric {
        title: t("completeness-metric-links", &[]),
        field: "links",
        present: links,
        total: links,
        summary: format!("{links}"),
        note: if links == 0 {
            t("completeness-metric-links-none", &[])
        } else {
            t("completeness-metric-links-some", &[("n", n(links))])
        },
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
        title: t("completeness-metric-occupations", &[]),
        field: "occupation.valid_from / valid_until",
        present: occ_span,
        total: occ_total,
        summary: format!("{occ_span} of {occ_total}"),
        note: if occ_total == 0 {
            t("completeness-metric-occupations-none", &[])
        } else if occ_span == 0 {
            t(
                "completeness-metric-occupations-undated",
                &[("total", n(occ_total))],
            )
        } else {
            t(
                "completeness-metric-occupations-some",
                &[("span", n(occ_span)), ("total", n(occ_total))],
            )
        },
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
        title: t("completeness-metric-sources", &[]),
        field: "source.reliability",
        present: src_graded,
        total: src_total,
        summary: format!("{src_graded} of {src_total}"),
        note: if src_graded == 0 {
            t("completeness-metric-sources-none", &[])
        } else {
            t(
                "completeness-metric-sources-some",
                &[("graded", n(src_graded)), ("total", n(src_total))],
            )
        },
        carried: src_graded > 0,
    });

    let dates = date_shapes(flat);
    let carried = metrics.iter().filter(|m| m.carried).count();
    let empty = metrics.len() - carried;

    let headline = if empty == 0 {
        t("completeness-headline-full", &[])
    } else if carried == 0 {
        t(
            "completeness-headline-empty",
            &[("total", n(metrics.len()))],
        )
    } else {
        t(
            "completeness-headline-partial",
            &[("carried", n(carried)), ("empty", n(empty))],
        )
    };

    Report {
        metrics,
        dates,
        headline,
        carried,
        empty,
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
        let r = analyse(&imported(), "en");

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
        // Stated as somewhere to grow, not as a verdict on a file format.
        assert!(
            r.headline.contains("could say more"),
            "an empty readout is a to-do list, not a report card: {}",
            r.headline
        );
    }

    #[test]
    fn every_metric_carries_a_key_a_title_and_a_sentence() {
        let r = analyse(&imported(), "en");
        for m in &r.metrics {
            assert!(!m.field.is_empty(), "a metric must have a stable key");
            assert!(!m.title.is_empty(), "{} has no title", m.field);
            assert!(!m.note.is_empty(), "{} says nothing", m.field);
        }
    }

    #[test]
    fn nothing_in_the_readout_argues_about_a_file_format() {
        // The readout is a to-do list for a family, not a comparison with
        // GEDCOM. It used to tag empty rows "GEDCOM cannot express this" and
        // link each one into a specification; both are gone, and this is what
        // stops them coming back a sentence at a time.
        let r = analyse(&imported(), "en");
        let mut prose = vec![r.headline.clone()];
        for m in &r.metrics {
            prose.push(m.title.clone());
            prose.push(m.note.clone());
        }
        for line in prose {
            let lower = line.to_lowercase();
            for word in ["gedcom", "axgf", "the format", "specification", "§"] {
                assert!(
                    !lower.contains(word),
                    "the readout speaks to a genealogist, not about a format: \
                     {line:?} contains {word:?}"
                );
            }
        }
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

        let r = analyse(&b, "en");
        assert_eq!(r.empty, 0, "nothing should be reported as missing");
        assert!(
            r.headline.contains("recorded somewhere"),
            "a rich bundle deserves a different sentence: {}",
            r.headline
        );
        let c = metric(&r, "confidence");
        assert!(c.note.contains("real, varying uncertainty"), "{}", c.note);
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

        let d = analyse(&b, "en").dates;
        assert_eq!(d.exact, 1, "p2's birth is a full calendar day");
        assert_eq!(d.approximate, 1, "circa 1500");
        assert_eq!(d.range, 1, "before 1560");
        assert_eq!(d.preserved, 1, "unparseable text kept");
        assert_eq!(d.total, 4);
    }

    #[test]
    fn an_empty_bundle_reports_zeroes_without_panicking() {
        let b = json!({"manifest": {"axgf": "1.0"}});
        let r = analyse(&b, "en");
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
