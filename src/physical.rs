//! Physical traits and health, recorded as dated, sourced claims.
//!
//! # Where this lives, and why it is not in the specification
//!
//! §5.1 models a person's identity, vitals, events, occupations and documents.
//! It has no field for how tall somebody was, what colour their eyes were, or
//! what they died of — and the right response to that is not to invent one.
//! The specification provides `extensions` for exactly this case: an object
//! whose keys are namespaced so that two applications can extend the same
//! entity without colliding, and so that a later first-class field can
//! supersede an extension cleanly rather than fighting it.
//!
//! Two keys, not one, and the split is the load-bearing decision in this
//! module:
//!
//! * [`TRAITS_KEY`] holds what a passport or a conscription register records —
//!   height, weight, eye and hair colour, build, handedness, distinguishing
//!   features, military service, languages. None of it is a medical fact.
//! * [`HEALTH_KEY`] holds what is a **special category of personal data** under
//!   GDPR article 9 and its equivalents: conditions, operations and injuries,
//!   blood group, cause of death — and religion or affiliations, which article
//!   9 lists in the same breath as health and which genealogists record
//!   constantly without thinking of it that way.
//!
//! Keeping them apart is what lets the second be withheld, redacted and
//! excluded from an export as a unit, without also hiding somebody's height.
//! One combined object would have forced a choice between over-restricting a
//! passport detail and under-restricting a diagnosis. See [`crate::access`] for
//! how the withholding is enforced, which is not here: this module decides what
//! the fields *are*, and never decides who may read them.
//!
//! # Every entry is dated, sourced and rated
//!
//! A height is measured at a moment. A 1914 conscription register gives it at
//! twenty and a 1950 passport at fifty-six, and those are two facts rather than
//! one fact revised — so every field holds a *list* of entries, each carrying
//! its own date, source and confidence, exactly as every other fact in this
//! product does. "Diabetes" with no source is a rumour; the shape of the data
//! is what makes that visible rather than a matter of discipline.

//! # Why there is no "also recorded for a parent" note
//!
//! A hereditary condition is worth more than the individual, and a link from a
//! record to the same condition on a parent or a child would be the obvious way
//! to say so. It was considered and is deliberately absent, for a reason about
//! the data rather than about the effort.
//!
//! `conditions` and `cause_of_death` are [`Kind::Text`], because what a record
//! says cannot be anticipated. Matching them across people therefore means
//! matching free text — and "diabetes", "diabetes mellitus", "cukrzyca" and
//! "sugar sickness" are one condition written four ways, while "none" and
//! "unknown" are two records agreeing about nothing. A note driven by string
//! equality would be silent almost always and wrong occasionally, which is the
//! worst pair of properties a hint can have. Making it useful needs coded
//! terms, and coding a diagnosis is a different product.
//!
//! There is a second reason, and it is the one this application has already
//! learned the hard way: a cross-record note is a disclosure surface. Saying
//! "also recorded for Anna" on somebody else's page states something about
//! Anna's health, so it would have to be gated on `may_read_health` for each
//! relative separately, and the history of this feature is that health leaks
//! through the surfaces nobody was thinking about rather than the ones they
//! were. A researcher who wants to know whether a condition runs in a family
//! opens the two records, which are one click apart in the family list.

use std::collections::BTreeMap;

use serde::Serialize;
use serde_json::{json, Map, Value};

/// Namespaced key for the non-sensitive half.
///
/// The version is in the key rather than inside the object: a future
/// `axgf-cms:traits/v2` can sit beside this one during a migration, and a
/// first-class specification field can supersede it by simply being read
/// first. Neither is possible if the version lives in a field nobody checks.
pub const TRAITS_KEY: &str = "axgf-cms:traits/v1";

/// Namespaced key for the special-category half. See the module docs.
pub const HEALTH_KEY: &str = "axgf-cms:health/v1";

/// What kind of value a field takes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// A whole number with a unit — centimetres, kilograms.
    Number,
    /// One of a fixed vocabulary, so it is comparable across generations.
    Closed(&'static [&'static str]),
    /// Anything. The half of a record that cannot be anticipated.
    Text,
}

/// Which of the two extension objects a field belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Group {
    /// Recorded by a passport office. Follows the record's normal visibility.
    Traits,
    /// Special category under GDPR article 9. Withheld on a living person.
    Health,
}

impl Group {
    pub fn key(self) -> &'static str {
        match self {
            Group::Traits => TRAITS_KEY,
            Group::Health => HEALTH_KEY,
        }
    }
}

/// One recordable field.
#[derive(Debug, Clone, Copy)]
pub struct Field {
    /// The key inside its extension object, and the form field name.
    pub name: &'static str,
    pub group: Group,
    pub kind: Kind,
}

pub const EYE_COLOURS: &[&str] = &[
    "brown", "hazel", "amber", "green", "blue", "grey", "mixed", "other",
];

pub const HAIR_COLOURS: &[&str] = &[
    "black", "brown", "auburn", "red", "fair", "blond", "grey", "white", "none", "other",
];

/// A/B/AB/O crossed with rhesus, plus the honest answer.
pub const BLOOD_GROUPS: &[&str] = &[
    "a-pos", "a-neg", "b-pos", "b-neg", "ab-pos", "ab-neg", "o-pos", "o-neg", "unknown",
];

pub const HANDEDNESS: &[&str] = &["left", "right", "ambidextrous", "unknown"];

/// Deliberately short. A longer list would invite a precision the sources do
/// not have: a register says "slight" or "stout", not a body-mass index.
pub const BUILDS: &[&str] = &["slight", "slim", "average", "sturdy", "stout", "heavy"];

/// Every field, in the order the editor and the record show them.
pub const FIELDS: &[Field] = &[
    // ---- closed lists, comparable across generations --------------------
    Field {
        name: "height_cm",
        group: Group::Traits,
        kind: Kind::Number,
    },
    Field {
        name: "weight_kg",
        group: Group::Traits,
        kind: Kind::Number,
    },
    Field {
        name: "eye_colour",
        group: Group::Traits,
        kind: Kind::Closed(EYE_COLOURS),
    },
    Field {
        name: "hair_colour",
        group: Group::Traits,
        kind: Kind::Closed(HAIR_COLOURS),
    },
    Field {
        name: "build",
        group: Group::Traits,
        kind: Kind::Closed(BUILDS),
    },
    Field {
        name: "handedness",
        group: Group::Traits,
        kind: Kind::Closed(HANDEDNESS),
    },
    // ---- free text that is nobody's business but the family's -----------
    Field {
        name: "features",
        group: Group::Traits,
        kind: Kind::Text,
    },
    Field {
        name: "military",
        group: Group::Traits,
        kind: Kind::Text,
    },
    Field {
        name: "languages",
        group: Group::Traits,
        kind: Kind::Text,
    },
    // ---- special category ------------------------------------------------
    Field {
        name: "blood_group",
        group: Group::Health,
        kind: Kind::Closed(BLOOD_GROUPS),
    },
    Field {
        name: "conditions",
        group: Group::Health,
        kind: Kind::Text,
    },
    Field {
        name: "operations",
        group: Group::Health,
        kind: Kind::Text,
    },
    Field {
        name: "cause_of_death",
        group: Group::Health,
        kind: Kind::Text,
    },
    Field {
        name: "religion",
        group: Group::Health,
        kind: Kind::Text,
    },
    Field {
        name: "health_notes",
        group: Group::Health,
        kind: Kind::Text,
    },
];

impl Field {
    pub fn find(name: &str) -> Option<&'static Field> {
        FIELDS.iter().find(|f| f.name == name)
    }

    /// The catalogue key for this field's label.
    pub fn label_key(&self) -> String {
        format!("phys-field-{}", self.name.replace('_', "-"))
    }

    /// The catalogue key for one of its vocabulary terms.
    pub fn term_key(&self, term: &str) -> String {
        format!("phys-{}-{}", self.name.replace('_', "-"), term)
    }

    /// True when this field is a special category and must be withheld from a
    /// reader who may not see a living person's health data.
    pub fn is_health(&self) -> bool {
        self.group == Group::Health
    }
}

/// One dated claim about one field, in the shape a form holds it.
///
/// Strings throughout, including the number: a half-typed height has to survive
/// being re-rendered with an error beside it rather than being dropped by a
/// parse that ran too early.
#[derive(Debug, Clone, Default, Serialize)]
pub struct Entry {
    pub value: String,
    /// As recorded — "1914", "1914-08-03". Stored in the specification's date
    /// shape so the same renderer draws it as every other date on the page.
    pub date: String,
    pub source_id: String,
    /// 0.0–1.0, as a string for the same reason `value` is.
    pub confidence: String,
    pub note: String,
}

impl Entry {
    pub fn is_empty(&self) -> bool {
        self.value.trim().is_empty()
    }
}

/// Every entry on one person, keyed by field name.
#[derive(Debug, Clone, Default, Serialize)]
pub struct Detail {
    pub entries: BTreeMap<String, Vec<Entry>>,
}

impl Detail {
    /// Read both extension objects off a stored person.
    ///
    /// Unknown keys inside them are ignored rather than dropped: [`Self::apply`]
    /// writes only the fields this build knows and leaves the rest of the
    /// object alone, so a bundle edited by a newer version does not lose data
    /// by passing through an older one.
    pub fn from_entity(person: &Value) -> Self {
        let mut entries: BTreeMap<String, Vec<Entry>> = BTreeMap::new();
        let ext = person.get("extensions");
        for field in FIELDS {
            let list = ext
                .and_then(|e| e.get(field.group.key()))
                .and_then(|g| g.get(field.name))
                .and_then(Value::as_array);
            let Some(list) = list else { continue };
            let mut out = Vec::new();
            for item in list {
                out.push(Entry {
                    value: scalar(item.get("value")),
                    date: item
                        .get("date")
                        .and_then(|d| d.get("value"))
                        .and_then(Value::as_str)
                        .unwrap_or_default()
                        .to_string(),
                    source_id: item
                        .get("source_id")
                        .and_then(Value::as_str)
                        .unwrap_or_default()
                        .to_string(),
                    confidence: scalar(item.get("confidence")),
                    note: item
                        .get("note")
                        .and_then(Value::as_str)
                        .unwrap_or_default()
                        .to_string(),
                });
            }
            if !out.is_empty() {
                entries.insert(field.name.to_string(), out);
            }
        }
        Self { entries }
    }

    /// Whether anything at all is recorded in the given group.
    pub fn has(&self, group: Group) -> bool {
        FIELDS
            .iter()
            .filter(|f| f.group == group)
            .any(|f| self.entries.get(f.name).is_some_and(|v| !v.is_empty()))
    }

    pub fn is_empty(&self) -> bool {
        self.entries.values().all(|v| v.is_empty())
    }

    /// How many entries one field carries.
    pub fn count(&self, field: &str) -> usize {
        self.entries.get(field).map_or(0, Vec::len)
    }

    /// The entry a single-figure summary should stand on: the latest dated
    /// one, and failing that the first undated one.
    ///
    /// Latest rather than first because a trait measured twice is a trait
    /// measured again, not a trait corrected — a height at twenty and a height
    /// at fifty-six are both true and the later one is the one a figure of
    /// somebody at the end of their life should be drawn from. The full series
    /// is still rendered beside it, dates leading, so nothing is hidden by the
    /// choice; only one of them can be drawn.
    ///
    /// Dates compare as strings, which is exactly right for the shape the
    /// specification stores them in: `1914` sorts before `1914-08` sorts
    /// before `1950`.
    pub fn latest(&self, field: &str) -> Option<&Entry> {
        let rows = self.entries.get(field)?;
        rows.iter()
            .filter(|r| !r.date.trim().is_empty())
            .max_by(|a, b| a.date.cmp(&b.date))
            .or_else(|| rows.first())
    }

    /// Read the editor's POST body.
    ///
    /// Rows are numbered in the field name — `height_cm.0.value` — and the form
    /// always renders a spare blank row, so adding an entry works with
    /// scripting off. Blank rows are dropped rather than saved.
    pub fn from_post(form: &BTreeMap<String, String>) -> Self {
        let mut entries: BTreeMap<String, Vec<Entry>> = BTreeMap::new();
        for field in FIELDS {
            let mut rows: BTreeMap<usize, Entry> = BTreeMap::new();
            let prefix = format!("{}.", field.name);
            for (k, v) in form {
                let Some(rest) = k.strip_prefix(&prefix) else {
                    continue;
                };
                let mut parts = rest.splitn(2, '.');
                let Some(idx) = parts.next().and_then(|i| i.parse::<usize>().ok()) else {
                    continue;
                };
                let Some(attr) = parts.next() else { continue };
                let row = rows.entry(idx).or_default();
                let v = v.trim().to_string();
                match attr {
                    "value" => row.value = v,
                    "date" => row.date = v,
                    "source_id" => row.source_id = v,
                    "confidence" => row.confidence = v,
                    "note" => row.note = v,
                    _ => {}
                }
            }
            let kept: Vec<Entry> = rows.into_values().filter(|e| !e.is_empty()).collect();
            if !kept.is_empty() {
                entries.insert(field.name.to_string(), kept);
            }
        }
        Self { entries }
    }

    /// What is wrong with this form, as catalogue keys.
    pub fn problems(&self) -> Vec<&'static str> {
        let mut out = Vec::new();
        for field in FIELDS {
            let Some(rows) = self.entries.get(field.name) else {
                continue;
            };
            for row in rows {
                match field.kind {
                    Kind::Number => {
                        // A height in metres, or with a unit typed in, is a
                        // number this cannot store — say so rather than
                        // rounding it to something plausible.
                        if row.value.parse::<u32>().is_err() {
                            out.push("phys-error-number");
                        } else if let Ok(n) = row.value.parse::<u32>() {
                            if n == 0 || n > 400 {
                                out.push("phys-error-range");
                            }
                        }
                    }
                    Kind::Closed(vocab) => {
                        if !vocab.contains(&row.value.as_str()) {
                            out.push("phys-error-vocabulary");
                        }
                    }
                    Kind::Text => {}
                }
                if !row.confidence.is_empty() {
                    match row.confidence.parse::<f64>() {
                        Ok(c) if (0.0..=1.0).contains(&c) => {}
                        _ => out.push("phys-error-confidence"),
                    }
                }
            }
        }
        out.sort_unstable();
        out.dedup();
        out
    }

    /// Write both extension objects onto the stored entity.
    ///
    /// Starts from what the bundle holds and replaces only the two keys this
    /// module owns, so another application's extension on the same person
    /// survives being edited here. An empty group removes its key rather than
    /// leaving `{}` behind, because an empty object is a claim that somebody
    /// looked and found nothing.
    pub fn apply(&self, stored: &Value) -> Value {
        let mut out = stored.clone();
        let obj = match out.as_object_mut() {
            Some(o) => o,
            None => {
                out = json!({});
                out.as_object_mut().expect("just made an object")
            }
        };

        let mut ext = obj
            .get("extensions")
            .and_then(Value::as_object)
            .cloned()
            .unwrap_or_default();

        for group in [Group::Traits, Group::Health] {
            let mut group_obj = Map::new();
            for field in FIELDS.iter().filter(|f| f.group == group) {
                let Some(rows) = self.entries.get(field.name) else {
                    continue;
                };
                let list: Vec<Value> = rows
                    .iter()
                    .filter(|r| !r.is_empty())
                    .map(|r| {
                        let mut m = Map::new();
                        match field.kind {
                            Kind::Number => {
                                if let Ok(n) = r.value.parse::<u64>() {
                                    m.insert("value".into(), json!(n));
                                } else {
                                    m.insert("value".into(), json!(r.value));
                                }
                            }
                            _ => {
                                m.insert("value".into(), json!(r.value));
                            }
                        }
                        if !r.date.is_empty() {
                            // The specification's date shape, so the same
                            // renderer draws it as every other date.
                            m.insert(
                                "date".into(),
                                json!({"value": r.date, "precision": precision_of(&r.date)}),
                            );
                        }
                        if !r.source_id.is_empty() {
                            m.insert("source_id".into(), json!(r.source_id));
                        }
                        if let Ok(c) = r.confidence.parse::<f64>() {
                            m.insert("confidence".into(), json!(c));
                        }
                        if !r.note.is_empty() {
                            m.insert("note".into(), json!(r.note));
                        }
                        Value::Object(m)
                    })
                    .collect();
                if !list.is_empty() {
                    group_obj.insert(field.name.to_string(), Value::Array(list));
                }
            }
            if group_obj.is_empty() {
                ext.remove(group.key());
            } else {
                ext.insert(group.key().to_string(), Value::Object(group_obj));
            }
        }

        if ext.is_empty() {
            obj.remove("extensions");
        } else {
            obj.insert("extensions".into(), Value::Object(ext));
        }
        out
    }
}

/// Remove the special-category half from a stored entity.
///
/// Used on two paths that must never carry it: the raw-JSON dump shown on the
/// record page, and an export the operator asked to be shareable. It takes the
/// whole key rather than walking fields, so a field added to the health group
/// later is covered by construction rather than by remembering to add it here.
pub fn strip_health(entity: &Value) -> Value {
    let mut out = entity.clone();
    let Some(obj) = out.as_object_mut() else {
        return out;
    };
    let Some(ext) = obj.get_mut("extensions").and_then(Value::as_object_mut) else {
        return out;
    };
    ext.remove(HEALTH_KEY);
    if ext.is_empty() {
        obj.remove("extensions");
    }
    out
}

/// Whether one recorded change can carry special-category data.
///
/// Three shapes reach this and each has to be answered differently.
///
/// * A path that walks into the health object names it outright.
/// * A change to `extensions` as a whole carries the object verbatim, health
///   and all. That is not an edge case: it is what the *first* edit to record
///   any of this on a person produces, because a field going from absent to
///   present is one change with the whole value in it.
/// * A value the diff had to truncate cannot be examined, so it is treated as
///   though it did. `serde_json` is built here with `preserve_order`, so which
///   half of the object survives a truncation depends on the order somebody
///   happened to write the keys in — which is not a thing to hang a disclosure
///   rule on.
pub fn change_touches_health(c: &crate::diff::Change) -> bool {
    let under = format!("extensions.{HEALTH_KEY}");
    if c.path == under || c.path.starts_with(&format!("{under}.")) {
        return true;
    }
    if c.path.is_empty() || c.path == "extensions" {
        return [c.from.as_deref(), c.to.as_deref()]
            .into_iter()
            .flatten()
            .any(|v| v.contains(HEALTH_KEY) || v.ends_with('\u{2026}'));
    }
    false
}

/// Recorded changes as this reader may see them.
///
/// The edit journal was the surface this application forgot. Everything a
/// reader is shown *of the record* goes through the health rule, and then the
/// history section beside it printed the same diagnosis back out of the diff
/// — one shared with every signed-in relative, on the person page, in the tree
/// panel, on the tree page and in the editor. A redaction that covers the
/// record and not the account of how the record got that way is not a
/// redaction.
///
/// A change that can carry health keeps its row and loses its values, rather
/// than being dropped: a reader told "this field changed and you may not see
/// how" has been told the truth, and one shown a diff with a row silently
/// missing has been shown a diff that is wrong. It is the same choice
/// [`DetailView::health_withheld`] makes for the section itself.
pub fn changes_for_reader(changes: &[crate::diff::Change], may_read_health: bool) -> Vec<Value> {
    changes
        .iter()
        .map(|c| {
            let withheld = !may_read_health && change_touches_health(c);
            json!({
                "path": c.path,
                "from": (!withheld).then(|| c.from.clone()).flatten(),
                "to": (!withheld).then(|| c.to.clone()).flatten(),
                "withheld": withheld,
            })
        })
        .collect()
}

/// Put the special-category half back onto an entity that was edited without
/// it.
///
/// The counterpart of [`strip_health`], for the form that was *given* a
/// stripped document: an absence in what comes back from such a form is the
/// redaction returning, not somebody deleting a diagnosis. Whatever the
/// submission says about the health key is discarded and the stored value put
/// back, so a hand-crafted POST cannot write one either.
pub fn restore_health(entity: Value, stored: &Value) -> Value {
    let mut out = entity;
    let Some(obj) = out.as_object_mut() else {
        return out;
    };
    let mut ext = obj
        .get("extensions")
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();
    ext.remove(HEALTH_KEY);
    if let Some(kept) = stored
        .get("extensions")
        .and_then(|e| e.get(HEALTH_KEY))
        .cloned()
    {
        ext.insert(HEALTH_KEY.to_string(), kept);
    }
    if ext.is_empty() {
        obj.remove("extensions");
    } else {
        obj.insert("extensions".into(), Value::Object(ext));
    }
    out
}

/// Whether an entity carries any special-category data at all.
pub fn has_health(entity: &Value) -> bool {
    entity
        .get("extensions")
        .and_then(|e| e.get(HEALTH_KEY))
        .and_then(Value::as_object)
        .is_some_and(|o| !o.is_empty())
}

/// A number or a string, as a string. The bundle may hold either.
fn scalar(v: Option<&Value>) -> String {
    match v {
        Some(Value::String(s)) => s.clone(),
        Some(Value::Number(n)) => n.to_string(),
        Some(Value::Bool(b)) => b.to_string(),
        _ => String::new(),
    }
}

/// The specification's precision word for a date written as recorded.
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

    fn form(pairs: &[(&str, &str)]) -> BTreeMap<String, String> {
        pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }

    #[test]
    fn the_two_groups_are_separate_keys_in_extensions() {
        // This is the split the whole feature rests on: one key can be
        // withheld and excluded from an export without touching the other.
        let d = Detail::from_post(&form(&[
            ("height_cm.0.value", "172"),
            ("conditions.0.value", "diabetes"),
        ]));
        let out = d.apply(&json!({"id": "x", "type": "person"}));
        let ext = out.get("extensions").expect("extensions written");
        assert!(ext.get(TRAITS_KEY).is_some(), "height is a trait");
        assert!(ext.get(HEALTH_KEY).is_some(), "a condition is not");
        assert!(
            ext.get(TRAITS_KEY).unwrap().get("conditions").is_none(),
            "and they do not share an object"
        );
    }

    #[test]
    fn stripping_health_leaves_the_traits_untouched() {
        let d = Detail::from_post(&form(&[
            ("height_cm.0.value", "172"),
            ("eye_colour.0.value", "blue"),
            ("conditions.0.value", "diabetes"),
            ("religion.0.value", "Roman Catholic"),
        ]));
        let full = d.apply(&json!({"id": "x"}));
        assert!(has_health(&full));

        let safe = strip_health(&full);
        assert!(!has_health(&safe));
        let s = serde_json::to_string(&safe).unwrap();
        assert!(!s.contains("diabetes"), "the condition is gone: {s}");
        assert!(
            !s.contains("Roman Catholic"),
            "and so is the religion, which article 9 lists beside health: {s}"
        );
        assert!(
            s.contains("172") && s.contains("blue"),
            "the traits stay: {s}"
        );
    }

    #[test]
    fn several_dated_entries_for_one_trait_are_kept_in_order() {
        // A conscription register at twenty and a passport at fifty-six are
        // two facts, not one fact revised.
        let d = Detail::from_post(&form(&[
            ("height_cm.0.value", "172"),
            ("height_cm.0.date", "1914"),
            ("height_cm.1.value", "169"),
            ("height_cm.1.date", "1950-06"),
        ]));
        let rows = &d.entries["height_cm"];
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].date, "1914");
        assert_eq!(rows[1].date, "1950-06");

        let out = d.apply(&json!({}));
        let list = out["extensions"][TRAITS_KEY]["height_cm"]
            .as_array()
            .expect("a list, not a single value");
        assert_eq!(list.len(), 2);
        assert_eq!(list[0]["value"], json!(172), "a number is stored as one");
        assert_eq!(list[0]["date"]["precision"], "year");
        assert_eq!(list[1]["date"]["precision"], "month");
    }

    #[test]
    fn a_blank_row_is_dropped_and_an_empty_group_removes_its_key() {
        let d = Detail::from_post(&form(&[
            ("height_cm.0.value", ""),
            ("height_cm.0.date", "1914"),
            ("conditions.0.value", "  "),
        ]));
        assert!(d.is_empty(), "a row with no value is not an entry");
        let out = d.apply(&json!({"id": "x"}));
        assert!(
            out.get("extensions").is_none(),
            "and an empty object is not written: {out}"
        );
    }

    #[test]
    fn another_applications_extension_survives_an_edit() {
        let stored = json!({
            "id": "x",
            "extensions": {"org.example/notes": {"kept": true}}
        });
        let d = Detail::from_post(&form(&[("height_cm.0.value", "172")]));
        let out = d.apply(&stored);
        assert_eq!(out["extensions"]["org.example/notes"]["kept"], json!(true));
        assert!(out["extensions"][TRAITS_KEY].is_object());
    }

    #[test]
    fn a_value_outside_the_vocabulary_is_refused_rather_than_stored() {
        let d = Detail::from_post(&form(&[("eye_colour.0.value", "puce")]));
        assert!(d.problems().contains(&"phys-error-vocabulary"));
        let d = Detail::from_post(&form(&[("eye_colour.0.value", "blue")]));
        assert!(d.problems().is_empty());
    }

    #[test]
    fn a_height_that_is_not_a_whole_number_of_centimetres_is_refused() {
        for bad in ["1.72", "172cm", "five foot", "0", "500"] {
            let d = Detail::from_post(&form(&[("height_cm.0.value", bad)]));
            assert!(!d.problems().is_empty(), "{bad} should be refused");
        }
        let d = Detail::from_post(&form(&[("height_cm.0.value", "172")]));
        assert!(d.problems().is_empty());
    }

    #[test]
    fn a_confidence_outside_zero_to_one_is_refused() {
        let d = Detail::from_post(&form(&[
            ("height_cm.0.value", "172"),
            ("height_cm.0.confidence", "80"),
        ]));
        assert!(d.problems().contains(&"phys-error-confidence"));
        let d = Detail::from_post(&form(&[
            ("height_cm.0.value", "172"),
            ("height_cm.0.confidence", "0.8"),
        ]));
        assert!(d.problems().is_empty());
    }

    #[test]
    fn a_stored_entity_round_trips_through_the_form() {
        let d = Detail::from_post(&form(&[
            ("height_cm.0.value", "172"),
            ("height_cm.0.date", "1914"),
            ("height_cm.0.confidence", "0.9"),
            ("height_cm.0.source_id", "src-1"),
            ("conditions.0.value", "diabetes"),
            ("conditions.0.note", "from a letter"),
        ]));
        let stored = d.apply(&json!({"id": "x"}));
        let back = Detail::from_entity(&stored);
        assert_eq!(back.entries["height_cm"][0].value, "172");
        assert_eq!(back.entries["height_cm"][0].date, "1914");
        assert_eq!(back.entries["height_cm"][0].confidence, "0.9");
        assert_eq!(back.entries["height_cm"][0].source_id, "src-1");
        assert_eq!(back.entries["conditions"][0].note, "from a letter");
        assert!(back.has(Group::Traits) && back.has(Group::Health));
    }

    #[test]
    fn every_field_has_a_distinct_name_and_a_label_key() {
        let names: std::collections::BTreeSet<_> = FIELDS.iter().map(|f| f.name).collect();
        assert_eq!(names.len(), FIELDS.len(), "field names are unique");
        for f in FIELDS {
            assert!(f.label_key().starts_with("phys-field-"));
            if let Kind::Closed(v) = f.kind {
                assert!(!v.is_empty());
                for term in v {
                    assert!(!f.term_key(term).contains('_'), "keys use dashes");
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// rendering
// ---------------------------------------------------------------------------

/// One dated claim, ready to draw.
#[derive(Debug, Clone, Serialize)]
pub struct EntryView {
    /// The value said in the reader's language: a vocabulary term translated,
    /// a number with its unit, free text verbatim.
    pub display: String,
    /// The raw stored value, for the comparison chart and for `data-` hooks.
    pub raw: String,
    pub date: crate::view::DateDisplay,
    pub has_date: bool,
    pub source: Option<String>,
    pub source_id: Option<String>,
    pub confidence: Option<crate::view::Confidence>,
    pub note: Option<String>,
}

/// One field and everything recorded for it.
#[derive(Debug, Clone, Serialize)]
pub struct FieldView {
    pub name: String,
    pub label: String,
    /// Several entries make a small series — the point of dating them.
    pub entries: Vec<EntryView>,
    pub is_series: bool,
}

/// The whole section, as the reader gets it.
#[derive(Debug, Clone, Default, Serialize)]
pub struct DetailView {
    pub traits: Vec<FieldView>,
    pub health: Vec<FieldView>,
    /// True when this person *has* health data that this reader may not see.
    ///
    /// Stating that something is withheld rather than omitting it silently is
    /// the same choice the person lens makes: omission is a false statement,
    /// and a record that shows nothing where a diagnosis exists reads as a
    /// record with nothing in it.
    pub health_withheld: bool,
    /// True when the subject is living, so the editor can say whose data this
    /// is and who will be able to read it.
    pub subject_is_living: bool,
    /// Nothing to draw. A serialised field rather than a method, because the
    /// template asks the question and templates cannot call methods.
    pub is_empty: bool,
}

/// Build the section for one person, applying the health rule.
///
/// `may_read_health` comes from [`crate::access::may_read_health`] and is
/// passed in rather than computed here, so the rule lives in one place and this
/// function cannot disagree with the export path about what it means.
pub fn view_for(person: &Value, flat: &Value, lang: &str, may_read_health: bool) -> DetailView {
    let detail = Detail::from_entity(person);
    let source_title = |id: &str| -> Option<String> {
        flat.get("sources")
            .and_then(|c| c.get(id))
            .and_then(|s| s.get("title"))
            .and_then(Value::as_str)
            .map(str::to_string)
    };

    let mut traits = Vec::new();
    let mut health = Vec::new();
    for field in FIELDS {
        if field.is_health() && !may_read_health {
            continue;
        }
        let Some(rows) = detail.entries.get(field.name) else {
            continue;
        };
        if rows.is_empty() {
            continue;
        }
        let entries: Vec<EntryView> = rows
            .iter()
            .map(|r| {
                let display = match field.kind {
                    Kind::Closed(_) => {
                        crate::i18n::translate(lang, &field.term_key(&r.value), None)
                    }
                    Kind::Number => {
                        let unit = if field.name == "height_cm" {
                            "phys-unit-cm"
                        } else {
                            "phys-unit-kg"
                        };
                        let mut args = fluent::FluentArgs::new();
                        args.set("n", fluent::FluentValue::from(r.value.as_str()));
                        crate::i18n::translate(lang, unit, Some(&args))
                    }
                    Kind::Text => r.value.clone(),
                };
                let date_raw = json!({
                    "value": r.date,
                    "precision": precision_of(&r.date),
                });
                let has_date = !r.date.trim().is_empty();
                EntryView {
                    display,
                    raw: r.value.clone(),
                    date: crate::view::render_date_in(&date_raw, lang),
                    has_date,
                    source: (!r.source_id.is_empty())
                        .then(|| source_title(&r.source_id))
                        .flatten(),
                    source_id: (!r.source_id.is_empty()).then(|| r.source_id.clone()),
                    confidence: r
                        .confidence
                        .parse::<f64>()
                        .ok()
                        .map(crate::view::Confidence::new),
                    note: (!r.note.is_empty()).then(|| r.note.clone()),
                }
            })
            .collect();
        let fv = FieldView {
            name: field.name.to_string(),
            label: crate::i18n::translate(lang, &field.label_key(), None),
            is_series: entries.len() > 1,
            entries,
        };
        if field.is_health() {
            health.push(fv);
        } else {
            traits.push(fv);
        }
    }

    let health_withheld = !may_read_health && has_health(person);
    DetailView {
        is_empty: traits.is_empty() && health.is_empty() && !health_withheld,
        traits,
        health,
        health_withheld,
        subject_is_living: person
            .get("identity")
            .and_then(|i| i.get("is_living"))
            .and_then(Value::as_bool)
            .unwrap_or(false),
    }
}

/// The single recorded value for a closed field, if there is exactly one
/// usable one. Used by the silhouette, which draws a build and a height and
/// has nothing sensible to do with a disagreement between two sources.
pub fn latest_value(person: &Value, field_name: &str) -> Option<String> {
    let detail = Detail::from_entity(person);
    let rows = detail.entries.get(field_name)?;
    // The best-dated entry wins; an undated one only if it is all there is.
    rows.iter()
        .filter(|r| !r.date.trim().is_empty())
        .max_by(|a, b| a.date.cmp(&b.date))
        .or_else(|| rows.first())
        .map(|r| r.value.clone())
}
