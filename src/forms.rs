//! Shared machinery for the structured editors.
//!
//! # Why these are not the generic form
//!
//! `admin.rs` maps one input to one dotted path, which is exactly right for a
//! scalar and cannot express a *list*. Nearly everything a genealogical record
//! actually holds is a list: the names somebody was known by, the partners in
//! a union, the children of it, the people at an event, the documents attached
//! to a person. Editing those through the raw-JSON box is not an editing
//! experience, it is a text editor with a schema behind it.
//!
//! So the structured editors share three things, and they are here:
//!
//! * [`rows`], which reads `prefix.N.field` out of a form body. Numbering the
//!   rows in the field name is what lets a form add and remove entries with
//!   scripting off: the page always renders one spare blank row, and a row
//!   whose key field comes back empty is dropped rather than saved.
//! * [`PersonPicker`], which turns "which person" from a UUID field into a
//!   search over the bundle, using a native `<datalist>` and no script at all.
//! * The date and reference builders, so eight forms produce the same
//!   specification shapes rather than eight nearly-identical ones.

use std::collections::BTreeMap;

use serde::Serialize;
use serde_json::{json, Map, Value};

/// One form body, as axum hands it over.
pub type Body = BTreeMap<String, String>;

/// Group `prefix.N.field=value` into rows, in index order.
///
/// The index is whatever the page rendered and need not be dense: deleting the
/// middle row of a form with scripting off leaves a gap, and closing that gap
/// here rather than in the browser is what makes the two behave the same.
pub fn rows(form: &Body, prefix: &str) -> Vec<BTreeMap<String, String>> {
    let mut out: BTreeMap<usize, BTreeMap<String, String>> = BTreeMap::new();
    let prefix = format!("{prefix}.");
    for (k, v) in form {
        let Some(rest) = k.strip_prefix(&prefix) else {
            continue;
        };
        let mut parts = rest.splitn(2, '.');
        let Some(idx) = parts.next().and_then(|i| i.parse::<usize>().ok()) else {
            continue;
        };
        let Some(field) = parts.next() else { continue };
        out.entry(idx)
            .or_default()
            .insert(field.to_string(), v.trim().to_string());
    }
    out.into_values().collect()
}

/// Whether a row's field is present and not blank.
pub fn field<'a>(row: &'a BTreeMap<String, String>, name: &str) -> Option<&'a str> {
    row.get(name).map(String::as_str).filter(|v| !v.is_empty())
}

/// A checkbox: absent from the submission means unchecked.
pub fn checked(row: &BTreeMap<String, String>, name: &str) -> bool {
    matches!(row.get(name).map(String::as_str), Some("on" | "true" | "1"))
}

/// A confidence, clamped, or `None` when the field was left blank.
pub fn confidence(row: &BTreeMap<String, String>, name: &str) -> Option<Value> {
    let v: f64 = field(row, name)?.parse().ok()?;
    serde_json::Number::from_f64(v.clamp(0.0, 1.0)).map(Value::Number)
}

/// Build the specification's date object from a value, a precision and a
/// circa flag, or `None` when no value was typed.
///
/// One builder rather than one per form: a date is the shape most likely to be
/// got subtly wrong, and "1923" stored without its precision renders as a day.
pub fn date_object(value: Option<&str>, precision: Option<&str>, circa: bool) -> Option<Value> {
    let value = value?.trim();
    if value.is_empty() {
        return None;
    }
    let precision = precision.filter(|p| !p.is_empty()).unwrap_or_else(|| {
        // What the value actually supports, rather than what a select
        // defaulted to. A year typed into a form that says "exact" is still a
        // year, and storing the claim would be inventing evidence.
        match value.matches('-').count() {
            0 => "year",
            1 => "month",
            _ => "exact",
        }
    });
    let mut m = Map::new();
    m.insert("value".into(), json!(value));
    m.insert("precision".into(), json!(precision));
    if circa {
        m.insert("circa".into(), json!(true));
    }
    Some(Value::Object(m))
}

/// Drop the keys whose values are `null`, so an absent field is absent rather
/// than present and empty. The two are different claims about the record.
pub fn prune(mut v: Value) -> Value {
    if let Some(obj) = v.as_object_mut() {
        obj.retain(|_, val| !val.is_null());
    }
    v
}

// ---------------------------------------------------------------------------
// choosing a person
// ---------------------------------------------------------------------------

/// One entry in the person picker.
#[derive(Debug, Clone, Serialize)]
pub struct PersonOption {
    /// What the reader sees and types against, and what the input receives.
    pub label: String,
    pub id: String,
}

/// Every person this reader may see, as picker options.
///
/// # Why a `<datalist>` and not a UUID field, a `<select>` or a widget
///
/// A UUID field is not an interface. A `<select>` of 866 people is the same
/// list without the typing. A search widget needs script, and this application
/// ships one binary and a static directory with no build step.
///
/// `<datalist>` is the native answer: the browser filters as the reader types,
/// it needs no JavaScript, it works with a screen reader, and with scripting
/// off it degrades to a plain text field that still accepts a typed name. The
/// cost is the list itself in the page, which is why it is rendered once per
/// page and shared by every picker on it.
///
/// The label carries a short id because names in a converted bundle are not
/// unique — this bundle has eight people called "Simla" — so picking by name
/// alone cannot say which. See [`resolve_person`] for the way back.
pub fn person_options(flat: &Value, lens: &crate::access::Lens) -> Vec<PersonOption> {
    let Some(persons) = flat.get("persons").and_then(Value::as_object) else {
        return Vec::new();
    };
    let mut out: Vec<PersonOption> = persons
        .iter()
        .filter(|(id, _)| lens.sees_person(id))
        .map(|(id, p)| PersonOption {
            label: person_label(id, p),
            id: id.clone(),
        })
        .collect();
    out.sort_by(|a, b| a.label.cmp(&b.label));
    out
}

/// "Maria Klicki · 1895–1973 · #00365a01".
pub fn person_label(id: &str, person: &Value) -> String {
    let name = crate::view::person_display_name(person);
    let years = |k: &str| {
        crate::view::latest_year_of_field(person, k)
            .map(|y| y.to_string())
            .unwrap_or_default()
    };
    let (b, d) = (years("birth"), years("death"));
    let span = if b.is_empty() && d.is_empty() {
        String::new()
    } else {
        format!(" · {b}–{d}")
    };
    format!("{name}{span} · #{}", short_id(id))
}

/// The first eight characters of a UUID, which is what the label carries.
pub fn short_id(id: &str) -> &str {
    id.get(..8).unwrap_or(id)
}

/// Read a person id back out of what the picker put in the field.
///
/// Three shapes arrive here and all three have to work, because a text input
/// accepts anything: the label the datalist inserted, a bare id pasted from
/// somewhere else, and a name somebody typed. The first is exact, the second
/// is exact, and the third is only accepted when it matches exactly one
/// person — a guess between two people called Simla is not a thing a genealogy
/// application should make on somebody's behalf.
pub fn resolve_person(input: &str, flat: &Value) -> Result<String, PickError> {
    let input = input.trim();
    if input.is_empty() {
        return Err(PickError::Empty);
    }
    let Some(persons) = flat.get("persons").and_then(Value::as_object) else {
        return Err(PickError::NotFound);
    };
    // The label ends in " · #xxxxxxxx"; a bare id is the whole field.
    let candidate = input.rsplit('#').next().unwrap_or(input).trim();
    if persons.contains_key(input) {
        return Ok(input.to_string());
    }
    let by_prefix: Vec<&String> = persons
        .keys()
        .filter(|id| !candidate.is_empty() && id.starts_with(candidate))
        .collect();
    match by_prefix.len() {
        1 => return Ok(by_prefix[0].clone()),
        n if n > 1 => return Err(PickError::Ambiguous),
        _ => {}
    }
    // Nothing matched an id, so treat it as a typed name.
    let name = input.split(" · ").next().unwrap_or(input).trim();
    let by_name: Vec<&String> = persons
        .iter()
        .filter(|(_, p)| crate::view::person_display_name(p) == name)
        .map(|(id, _)| id)
        .collect();
    match by_name.len() {
        1 => Ok(by_name[0].clone()),
        0 => Err(PickError::NotFound),
        _ => Err(PickError::Ambiguous),
    }
}

/// Why a typed person could not be turned into one person.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PickError {
    Empty,
    NotFound,
    Ambiguous,
}

impl PickError {
    /// The catalogue key for what to tell the editor.
    pub fn key(self) -> &'static str {
        match self {
            PickError::Empty => "pick-error-empty",
            PickError::NotFound => "pick-error-not-found",
            PickError::Ambiguous => "pick-error-ambiguous",
        }
    }
}

/// Every entity a Link may point at, as picker options.
///
/// One list rather than one per kind, and one input rather than a type select
/// feeding a dependent picker: without script a dependent picker cannot swap
/// its options, and with script it is a widget this application does not want.
/// The kind comes back out of which collection holds the id, so the reader
/// picks a thing and the type looks after itself.
pub fn linkable_options(flat: &Value, lens: &crate::access::Lens) -> Vec<PersonOption> {
    let mut out = person_options(flat, lens);
    for (collection, label) in [
        ("families", family_brief as fn(&Value, &Value) -> String),
        ("events", event_brief as fn(&Value, &Value) -> String),
    ] {
        let Some(map) = flat.get(collection).and_then(Value::as_object) else {
            continue;
        };
        for (id, e) in map {
            out.push(PersonOption {
                label: format!("{} · #{}", label(flat, e), short_id(id)),
                id: id.clone(),
            });
        }
    }
    out.sort_by(|a, b| a.label.cmp(&b.label));
    out
}

/// A family said briefly: the partners' names, which is how anybody knows it.
fn family_brief(flat: &Value, fam: &Value) -> String {
    let names: Vec<String> = fam
        .pointer("/union/persons")
        .and_then(Value::as_array)
        .map(|ps| {
            ps.iter()
                .filter_map(|p| p.get("person_id").and_then(Value::as_str))
                .map(|pid| match flat.get("persons").and_then(|c| c.get(pid)) {
                    Some(p) => crate::view::person_display_name(p),
                    None => short_id(pid).to_string(),
                })
                .collect()
        })
        .unwrap_or_default();
    names.join(" + ")
}

/// An event said briefly: what kind it was and when.
fn event_brief(_flat: &Value, ev: &Value) -> String {
    let cat = ev
        .get("category")
        .and_then(Value::as_str)
        .unwrap_or("event")
        .replace('_', " ");
    match crate::view::latest_year_of_field(ev, "date") {
        Some(y) => format!("{cat} {y}"),
        None => cat,
    }
}

/// Read an entity id back out of a picker field, with the kind it turned out
/// to be. See [`resolve_person`] for how the three input shapes are handled.
pub fn resolve_linkable(input: &str, flat: &Value) -> Result<(&'static str, String), PickError> {
    let input = input.trim();
    if input.is_empty() {
        return Err(PickError::Empty);
    }
    let candidate = input.rsplit('#').next().unwrap_or(input).trim();
    let mut hits: Vec<(&'static str, String)> = Vec::new();
    for (collection, kind) in [
        ("persons", "person"),
        ("families", "family"),
        ("events", "event"),
    ] {
        let Some(map) = flat.get(collection).and_then(Value::as_object) else {
            continue;
        };
        for id in map.keys() {
            if id == input || (!candidate.is_empty() && id.starts_with(candidate)) {
                hits.push((kind, id.clone()));
            }
        }
    }
    match hits.len() {
        1 => Ok(hits.remove(0)),
        0 => resolve_person(input, flat).map(|id| ("person", id)),
        _ => Err(PickError::Ambiguous),
    }
}

/// Options for a `<select>` over an entity collection, sorted by label.
pub fn entity_options(flat: &Value, collection: &str, label: fn(&Value) -> String) -> Vec<Value> {
    let Some(map) = flat.get(collection).and_then(Value::as_object) else {
        return Vec::new();
    };
    let mut out: Vec<(String, String)> = map
        .iter()
        .map(|(id, e)| (label(e), id.clone()))
        .filter(|(l, _)| !l.is_empty())
        .collect();
    out.sort();
    out.into_iter()
        .map(|(label, id)| json!({"id": id, "label": label}))
        .collect()
}

/// The label a place carries in a picker.
pub fn place_label(place: &Value) -> String {
    crate::view::place_name(place)
}

/// The label a source carries in a picker.
pub fn source_label(source: &Value) -> String {
    source
        .get("title")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn body(pairs: &[(&str, &str)]) -> Body {
        pairs
            .iter()
            .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
            .collect()
    }

    #[test]
    fn rows_group_by_index_and_close_the_gaps() {
        // Row 1 was deleted with scripting off, so the numbering skips it.
        let f = body(&[
            ("n.0.display", "Anna"),
            ("n.0.type", "birth"),
            ("n.2.display", "Hanna"),
            ("other.0.display", "not this"),
        ]);
        let r = rows(&f, "n");
        assert_eq!(r.len(), 2);
        assert_eq!(field(&r[0], "display"), Some("Anna"));
        assert_eq!(field(&r[0], "type"), Some("birth"));
        assert_eq!(field(&r[1], "display"), Some("Hanna"));
    }

    #[test]
    fn a_blank_field_is_absent_rather_than_empty() {
        let f = body(&[("n.0.display", "  "), ("n.0.note", "x")]);
        let r = rows(&f, "n");
        assert_eq!(field(&r[0], "display"), None);
        assert_eq!(field(&r[0], "note"), Some("x"));
    }

    #[test]
    fn a_date_keeps_the_precision_its_value_supports() {
        let d = date_object(Some("1923"), None, false).expect("a date");
        assert_eq!(d["precision"], "year");
        let d = date_object(Some("1923-04-12"), None, false).expect("a date");
        assert_eq!(d["precision"], "exact");
        // An explicit precision wins, because a source can say "this is only
        // good to the month" about a value that looks like a day.
        let d = date_object(Some("1923-04-12"), Some("month"), true).expect("a date");
        assert_eq!(d["precision"], "month");
        assert_eq!(d["circa"], true);
        assert!(date_object(Some("  "), None, false).is_none());
        assert!(date_object(None, Some("year"), false).is_none());
    }

    #[test]
    fn a_person_is_found_by_label_by_id_and_by_an_unambiguous_name() {
        let flat = json!({"persons": {
            "aaaaaaaa-0000-4000-8000-000000000001":
                {"identity": {"name": {"display": "Maria Klicki"}}},
            "bbbbbbbb-0000-4000-8000-000000000002":
                {"identity": {"name": {"display": "Simla"}}},
            "cccccccc-0000-4000-8000-000000000003":
                {"identity": {"name": {"display": "Simla"}}}
        }});
        let label = person_label(
            "aaaaaaaa-0000-4000-8000-000000000001",
            &flat["persons"]["aaaaaaaa-0000-4000-8000-000000000001"],
        );
        assert!(label.starts_with("Maria Klicki"));
        assert_eq!(
            resolve_person(&label, &flat).unwrap(),
            "aaaaaaaa-0000-4000-8000-000000000001"
        );
        assert_eq!(
            resolve_person("aaaaaaaa-0000-4000-8000-000000000001", &flat).unwrap(),
            "aaaaaaaa-0000-4000-8000-000000000001"
        );
        assert_eq!(
            resolve_person("Maria Klicki", &flat).unwrap(),
            "aaaaaaaa-0000-4000-8000-000000000001"
        );
        // Two people of that name, so this application does not pick one.
        assert_eq!(resolve_person("Simla", &flat), Err(PickError::Ambiguous));
        assert_eq!(resolve_person("Nobody", &flat), Err(PickError::NotFound));
        assert_eq!(resolve_person("  ", &flat), Err(PickError::Empty));
    }

    #[test]
    fn a_pruned_object_drops_the_fields_nobody_filled_in() {
        let v = prune(json!({"a": 1, "b": null, "c": "x"}));
        assert_eq!(v, json!({"a": 1, "c": "x"}));
    }

    #[test]
    fn a_checkbox_is_off_when_the_browser_omits_it() {
        let r = rows(&body(&[("x.0.flag", "on"), ("x.1.other", "y")]), "x");
        assert!(checked(&r[0], "flag"));
        assert!(!checked(&r[1], "flag"));
    }
}
