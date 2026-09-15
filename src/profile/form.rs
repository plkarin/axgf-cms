//! A profile group as an editor edits it, and the claims read back out of it.
//!
//! # One row per claim, and always one to spare
//!
//! Every attribute is a fieldset of rows. A row is one claim: its value, as
//! one input per field of the value's shape, and its provenance — date,
//! period, source, confidence, note — behind a disclosure, because a form of
//! thirty attributes each showing nine provenance inputs is a form nobody
//! fills in. A series always renders one blank row after its claims, and a
//! single claim renders one row whether or not it is recorded, so adding an
//! entry works with scripting off; `static/profile.js` only adds a button that
//! copies the blank row.
//!
//! Rows are numbered in the input names — `morphology.height.2.v` — which is
//! what [`crate::forms::rows`] reads back. A row whose value is blank, or whose
//! "remove" box is ticked, is dropped rather than saved.
//!
//! # What the form does not show, it does not write
//!
//! A row carries `ci`, the index of the claim it was rendered from, and the
//! claim is rebuilt *from that stored claim*: the fields this form edits are
//! overwritten and everything else — an `event_id`, a key a newer version
//! wrote — is kept. An attribute the editor may not read is not rendered, is
//! not read back, and is then put back exactly as stored by
//! [`crate::sensitive::restore`] before anything is written.
//!
//! # Every closed vocabulary is a select
//!
//! Its options are the vocabulary's terms in the registry's order, labelled
//! in the reader's language — except the country list, which is sorted by the
//! translated name, because 254 codes in code order is a list nobody can use.
//! A value the select could not have sent is a problem, not a save.

use std::collections::BTreeMap;

use axgf_rs::model::profile::registry::{Attribute, Cardinality, Group, Shape};
use axgf_rs::model::profile::vocab::{self, Vocabulary};
use serde::Serialize;
use serde_json::{json, Map, Value};

use crate::forms::{self, Body};
use crate::sensitive::Scopes;

/// The option lists a group's inputs draw from, resolved once per page.
pub struct Choices {
    pub lang: String,
    pub sources: Vec<Value>,
    pub places: Vec<Value>,
    /// The documents this person's record attaches, which is what an artefact
    /// can refer to without the page carrying every document in the bundle.
    pub documents: Vec<Value>,
}

/// One option of a select.
#[derive(Debug, Clone, Serialize)]
pub struct Opt {
    pub value: String,
    pub label: String,
}

/// A labelled group of options.
#[derive(Debug, Clone, Serialize)]
pub struct OptGroup {
    pub label: String,
    pub options: Vec<Opt>,
}

/// One input of a row.
#[derive(Debug, Clone, Serialize)]
pub struct Input {
    pub name: String,
    pub label: String,
    /// `text`, `textarea`, `number`, `select`, `grouped`, `time` or `yesno`.
    pub kind: &'static str,
    pub value: String,
    pub options: Vec<Opt>,
    pub groups: Vec<OptGroup>,
    pub min: Option<String>,
    pub max: Option<String>,
    pub step: Option<&'static str>,
    pub unit: Option<String>,
    pub required: bool,
}

/// A date as the three inputs a row shows it as.
#[derive(Debug, Clone, Serialize, Default)]
pub struct DateInput {
    pub date: String,
    pub precision: String,
    pub circa: bool,
}

/// One claim, as a row of inputs.
#[derive(Debug, Clone, Serialize)]
pub struct Row {
    pub index: usize,
    pub ci: Option<usize>,
    pub inputs: Vec<Input>,
    pub date: DateInput,
    pub from: DateInput,
    pub until: DateInput,
    pub source_id: String,
    pub confidence: String,
    pub note: String,
    pub has_provenance: bool,
    pub spare: bool,
}

/// One attribute of the group, as the editor draws it.
#[derive(Debug, Clone, Serialize)]
pub struct EditorAttribute {
    pub prefix: String,
    pub anchor: String,
    pub label: String,
    pub class_label: Option<String>,
    pub is_series: bool,
    pub rows: Vec<Row>,
    /// Problems in what was submitted for this attribute, already translated.
    pub problems: Vec<String>,
    /// A field this application recorded before AXGF 1.1 with no 1.1 home.
    pub earlier: bool,
}

/// The group, as the editor draws it.
#[derive(Debug, Clone, Serialize)]
pub struct EditorGroup {
    pub key: &'static str,
    pub title: String,
    pub intro: String,
    pub attributes: Vec<EditorAttribute>,
    /// Classes this group holds that the editor may not read.
    pub withheld: Vec<String>,
    /// The one class the whole group belongs to, when there is one.
    pub class_label: Option<String>,
}

/// The input names a row's value is read from, relative to the row.
type RowValues = BTreeMap<String, String>;

/// Free-text fields long enough to deserve a textarea.
const LONG_TEXT: &[&str] = &[
    "description",
    "summary",
    "findings",
    "inscription",
    "text",
    "offence",
    "sentence",
    "lines",
    "reaction",
    "details",
];

// ---------------------------------------------------------------------------
// drawing
// ---------------------------------------------------------------------------

/// The group as its editor draws it, from the stored (lifted) person, or —
/// after a refused save — from what was submitted.
pub fn editor_group(
    person: &Value,
    g: &'static Group,
    readable: Scopes,
    choices: &Choices,
    submitted: Option<&Body>,
    problems: &BTreeMap<String, Vec<&'static str>>,
) -> EditorGroup {
    let lang = choices.lang.as_str();
    let mut attributes = Vec::new();
    let mut withheld = Scopes::NONE;
    for a in g.attributes {
        let scopes = super::attribute_scopes(a);
        if !scopes.within(readable) {
            if person.get(a.block).and_then(|b| b.get(a.key)).is_some() {
                withheld = withheld.union(scopes.without(readable));
            }
            continue;
        }
        let rows = match submitted {
            Some(form) => rows_from_form(a, form, choices),
            None => rows_from_person(a, person, choices),
        };
        attributes.push(EditorAttribute {
            prefix: a.path.to_string(),
            anchor: a.path.replace('.', "-"),
            label: super::attribute_label(lang, a),
            class_label: a
                .class
                .map(|c| crate::i18n::translate(lang, &format!("scope-{}", c.as_str()), None)),
            is_series: a.cardinality == Cardinality::Series,
            rows,
            problems: problems
                .get(a.path)
                .map(|ps| {
                    ps.iter()
                        .map(|k| crate::i18n::translate(lang, k, None))
                        .collect()
                })
                .unwrap_or_default(),
            earlier: false,
        });
    }

    for left in super::lift::leftovers(person)
        .into_iter()
        .filter(|l| l.group == g.key)
    {
        let scopes = crate::sensitive::location_scopes("extensions", left.extension);
        if !scopes.within(readable) {
            withheld = withheld.union(scopes.without(readable));
            continue;
        }
        let prefix = earlier_prefix(left.extension, &left.field);
        let mut rows: Vec<Row> = match submitted {
            Some(form) => forms::rows(form, &prefix)
                .into_iter()
                .enumerate()
                .map(|(i, r)| text_row(&prefix, i, &r, choices, false))
                .collect(),
            None => left
                .entries
                .iter()
                .enumerate()
                .map(|(i, e)| {
                    let mut r = provenance_values(e);
                    r.insert("v".into(), scalar(e.get("value").unwrap_or(&Value::Null)));
                    text_row(&prefix, i, &r, choices, false)
                })
                .collect(),
        };
        let next = rows.len();
        rows.push(text_row(&prefix, next, &RowValues::new(), choices, true));
        attributes.push(EditorAttribute {
            anchor: prefix.replace(['.', ':', '/'], "-"),
            prefix,
            label: crate::i18n::translate(
                lang,
                &format!("phys-field-{}", left.field.replace('_', "-")),
                None,
            ),
            class_label: (left.extension == crate::physical::HEALTH_KEY)
                .then(|| crate::i18n::translate(lang, "scope-health", None)),
            is_series: true,
            rows,
            problems: Vec::new(),
            earlier: true,
        });
    }

    let class_label = super::view::group_class(g)
        .map(|c| crate::i18n::translate(lang, &format!("scope-{}", c.as_str()), None));
    if class_label.is_some() {
        for a in attributes.iter_mut() {
            if !a.earlier {
                a.class_label = None;
            }
        }
    }
    EditorGroup {
        class_label,
        key: g.key,
        title: super::group_title(lang, g),
        intro: crate::i18n::translate(lang, &super::group_intro_key(g), None),
        attributes,
        withheld: withheld
            .iter()
            .map(|s| crate::i18n::translate(lang, &format!("scope-{}", s.as_str()), None))
            .collect(),
    }
}

/// The input-name prefix of a legacy field.
fn earlier_prefix(extension: &str, field: &str) -> String {
    let short = if extension == crate::physical::HEALTH_KEY {
        "health"
    } else {
        "traits"
    };
    format!("earlier.{short}-{field}")
}

fn text_row(prefix: &str, index: usize, r: &RowValues, choices: &Choices, spare: bool) -> Row {
    let name = format!("{prefix}.{index}.v");
    let input = Input {
        name,
        label: crate::i18n::translate(&choices.lang, "profile-value", None),
        kind: "textarea",
        value: r.get("v").cloned().unwrap_or_default(),
        ..blank_input()
    };
    finish_row(index, None, vec![input], r, spare)
}

fn rows_from_person(a: &Attribute, person: &Value, choices: &Choices) -> Vec<Row> {
    let held = person.get(a.block).and_then(|b| b.get(a.key));
    let claims: Vec<&Value> = match (a.cardinality, held) {
        (_, None) | (_, Some(Value::Null)) => Vec::new(),
        (Cardinality::Series, Some(Value::Array(list))) => list.iter().collect(),
        (Cardinality::Single, Some(v)) if v.is_object() => vec![v],
        _ => Vec::new(),
    };
    let mut rows: Vec<Row> = claims
        .iter()
        .enumerate()
        .map(|(i, c)| {
            let mut values = provenance_values(c);
            value_strings(
                &a.shape,
                "v",
                c.get("value").unwrap_or(&Value::Null),
                &mut values,
            );
            row(a, i, Some(i), &values, choices, false)
        })
        .collect();
    if a.cardinality == Cardinality::Series || rows.is_empty() {
        let next = rows.len();
        rows.push(row(a, next, None, &RowValues::new(), choices, true));
    }
    rows
}

fn rows_from_form(a: &Attribute, form: &Body, choices: &Choices) -> Vec<Row> {
    let mut rows: Vec<Row> = forms::rows(form, a.path)
        .into_iter()
        .enumerate()
        .map(|(i, r)| {
            let ci = r.get("ci").and_then(|c| c.parse().ok());
            row(a, i, ci, &r, choices, false)
        })
        .collect();
    if a.cardinality == Cardinality::Series || rows.is_empty() {
        let next = rows.len();
        rows.push(row(a, next, None, &RowValues::new(), choices, true));
    }
    rows
}

fn provenance_values(claim: &Value) -> RowValues {
    let mut r = RowValues::new();
    for (name, pointer) in [
        ("d", "/date"),
        ("f", "/valid_from/date"),
        ("u", "/valid_until/date"),
    ] {
        if let Some(d) = claim.pointer(pointer) {
            r.insert(
                format!("{name}.date"),
                scalar(d.get("value").unwrap_or(&Value::Null)),
            );
            r.insert(
                format!("{name}.precision"),
                scalar(d.get("precision").unwrap_or(&Value::Null)),
            );
            if d.get("circa").and_then(Value::as_bool) == Some(true) {
                r.insert(format!("{name}.circa"), "on".into());
            }
        }
    }
    for key in ["source_id", "confidence", "note"] {
        if let Some(v) = claim.get(key).filter(|v| !v.is_null()) {
            r.insert(key.to_string(), scalar(v));
        }
    }
    r
}

/// The strings a stored value puts into a row's inputs.
fn value_strings(shape: &Shape, name: &str, v: &Value, out: &mut RowValues) {
    match shape {
        Shape::Object { fields, .. } => {
            for f in fields.iter() {
                if let Some(fv) = v.get(f.key) {
                    value_strings(&f.shape, &format!("{name}.{}", f.key), fv, out);
                }
            }
            // A rank is chosen from one grouped select whose value carries the
            // country it belongs to.
            if let (Some(rank), Some(country)) = (
                v.get("rank").and_then(Value::as_str),
                v.get("country").and_then(Value::as_str),
            ) {
                out.insert(format!("{name}.rank"), format!("{country}:{rank}"));
            }
        }
        Shape::Coordinates => {
            for k in ["lat", "lon", "precision"] {
                if let Some(x) = v.get(k).filter(|x| !x.is_null()) {
                    out.insert(format!("{name}.{k}"), scalar(x));
                }
            }
        }
        Shape::Artefact(_) => {
            for k in [
                "document_id",
                "artefact_type",
                "format",
                "generator",
                "derived_from_id",
                "consent",
            ] {
                if let Some(x) = v.get(k).filter(|x| !x.is_null()) {
                    out.insert(format!("{name}.{k}"), scalar(x));
                }
            }
        }
        Shape::Boolean => {
            if let Some(b) = v.as_bool() {
                out.insert(name.to_string(), if b { "yes" } else { "no" }.into());
            }
        }
        _ => {
            if !v.is_null() {
                out.insert(name.to_string(), scalar(v));
            }
        }
    }
}

fn scalar(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => String::new(),
        other => other.to_string(),
    }
}

fn blank_input() -> Input {
    Input {
        name: String::new(),
        label: String::new(),
        kind: "text",
        value: String::new(),
        options: Vec::new(),
        groups: Vec::new(),
        min: None,
        max: None,
        step: None,
        unit: None,
        required: false,
    }
}

fn row(
    a: &Attribute,
    index: usize,
    ci: Option<usize>,
    values: &RowValues,
    choices: &Choices,
    spare: bool,
) -> Row {
    let base = format!("{}.{index}", a.path);
    let mut inputs = Vec::new();
    push_inputs(a, &a.shape, &base, "v", None, values, choices, &mut inputs);
    finish_row(index, ci, inputs, values, spare)
}

fn finish_row(
    index: usize,
    ci: Option<usize>,
    inputs: Vec<Input>,
    values: &RowValues,
    spare: bool,
) -> Row {
    let date = |p: &str| DateInput {
        date: values
            .get(&format!("{p}.date"))
            .cloned()
            .unwrap_or_default(),
        precision: values
            .get(&format!("{p}.precision"))
            .cloned()
            .unwrap_or_default(),
        circa: values.get(&format!("{p}.circa")).is_some_and(|c| c == "on"),
    };
    let get = |k: &str| values.get(k).cloned().unwrap_or_default();
    let (d, f, u) = (date("d"), date("f"), date("u"));
    let has_provenance = [&d.date, &f.date, &u.date].iter().any(|s| !s.is_empty())
        || ["source_id", "confidence", "note"]
            .iter()
            .any(|k| !get(k).is_empty());
    Row {
        index,
        ci,
        inputs,
        date: d,
        from: f,
        until: u,
        source_id: get("source_id"),
        confidence: get("confidence"),
        note: get("note"),
        has_provenance,
        spare,
    }
}

/// Options for a vocabulary select: the terms in the registry's order, or —
/// for the country list — by translated name.
pub fn vocabulary_options(lang: &str, v: &Vocabulary) -> Vec<Opt> {
    let mut out: Vec<Opt> = v
        .terms
        .iter()
        .map(|t| Opt {
            value: (*t).to_string(),
            label: super::term_label(lang, v, t),
        })
        .collect();
    if v.name == "country" {
        out.sort_by_key(|o| o.label.to_lowercase());
    }
    out
}

#[allow(clippy::too_many_arguments)]
fn push_inputs(
    a: &Attribute,
    shape: &Shape,
    base: &str,
    name: &str,
    field: Option<(&str, bool)>,
    values: &RowValues,
    choices: &Choices,
    out: &mut Vec<Input>,
) {
    let lang = choices.lang.as_str();
    let label = match field {
        Some((key, _)) => super::field_label(lang, a, key),
        None => crate::i18n::translate(lang, "profile-value", None),
    };
    let required = field.is_some_and(|(_, r)| r);
    let value = values.get(name).cloned().unwrap_or_default();
    let full = format!("{base}.{name}");
    let key = field.map(|(k, _)| k).unwrap_or("");
    match shape {
        Shape::Text => out.push(Input {
            name: full,
            label,
            kind: if field.is_none() || LONG_TEXT.contains(&key) {
                "textarea"
            } else {
                "text"
            },
            value,
            required,
            ..blank_input()
        }),
        Shape::Pattern(_) | Shape::CurrencyCode | Shape::LanguageTag => out.push(Input {
            name: full,
            label,
            value,
            required,
            ..blank_input()
        }),
        Shape::TimeOfDay => out.push(Input {
            name: full,
            label,
            kind: "time",
            value,
            step: Some("1"),
            required,
            ..blank_input()
        }),
        Shape::Number { min, max, unit } => out.push(Input {
            name: full,
            label,
            kind: "number",
            value,
            min: min.map(|m| m.to_string()),
            max: max.map(|m| m.to_string()),
            step: Some("any"),
            unit: unit_symbol(lang, unit),
            required,
            ..blank_input()
        }),
        Shape::Integer { min, max, unit } => out.push(Input {
            name: full,
            label,
            kind: "number",
            value,
            min: min.map(|m| m.to_string()),
            max: max.map(|m| m.to_string()),
            step: Some("1"),
            unit: unit_symbol(lang, unit),
            required,
            ..blank_input()
        }),
        Shape::Boolean => out.push(Input {
            name: full,
            label,
            kind: "yesno",
            value,
            options: vec![
                Opt {
                    value: "yes".into(),
                    label: crate::i18n::translate(lang, "profile-yes", None),
                },
                Opt {
                    value: "no".into(),
                    label: crate::i18n::translate(lang, "profile-no", None),
                },
            ],
            required,
            ..blank_input()
        }),
        Shape::Vocab(v) => out.push(Input {
            name: full,
            label,
            kind: "select",
            value,
            options: vocabulary_options(lang, v),
            required,
            ..blank_input()
        }),
        Shape::Uuid => {
            let options = match key {
                "place_id" => entity_opts(&choices.places),
                _ => entity_opts(&choices.documents),
            };
            out.push(Input {
                name: full,
                label,
                kind: "select",
                value,
                options,
                required,
                ..blank_input()
            })
        }
        Shape::Coordinates => {
            for (k, min, max) in [("lat", "-90", "90"), ("lon", "-180", "180")] {
                out.push(Input {
                    name: format!("{full}.{k}"),
                    label: crate::i18n::translate(lang, &super::field_key_shared(k), None),
                    kind: "number",
                    value: values
                        .get(&format!("{name}.{k}"))
                        .cloned()
                        .unwrap_or_default(),
                    min: Some(min.into()),
                    max: Some(max.into()),
                    step: Some("any"),
                    ..blank_input()
                });
            }
            out.push(Input {
                name: format!("{full}.precision"),
                label: crate::i18n::translate(lang, &super::field_key_shared("precision"), None),
                kind: "select",
                value: values
                    .get(&format!("{name}.precision"))
                    .cloned()
                    .unwrap_or_default(),
                options: crate::place::PRECISIONS
                    .iter()
                    .map(|p| Opt {
                        value: (*p).to_string(),
                        label: crate::i18n::translate(lang, &format!("place-precision-{p}"), None),
                    })
                    .collect(),
                ..blank_input()
            });
        }
        Shape::Rank => out.push(Input {
            name: full,
            label,
            kind: "grouped",
            value,
            groups: vocab::MILITARY_RANKS
                .iter()
                .map(|rv| OptGroup {
                    label: super::term_label(lang, &vocab::COUNTRY, rv.country),
                    options: rv
                        .ranks
                        .iter()
                        .map(|r| Opt {
                            value: format!("{}:{}", rv.country, r.term),
                            label: r.title.to_string(),
                        })
                        .collect(),
                })
                .collect(),
            required,
            ..blank_input()
        }),
        Shape::Artefact(types) => {
            let sub = |k: &str| {
                values
                    .get(&format!("{name}.{k}"))
                    .cloned()
                    .unwrap_or_default()
            };
            let fl = |k: &str| crate::i18n::translate(lang, &super::field_key_shared(k), None);
            out.push(Input {
                name: format!("{full}.document_id"),
                label: fl("document_id"),
                kind: "select",
                value: sub("document_id"),
                options: entity_opts(&choices.documents),
                required: true,
                ..blank_input()
            });
            out.push(Input {
                name: format!("{full}.artefact_type"),
                label: fl("artefact_type"),
                kind: "select",
                value: sub("artefact_type"),
                options: types
                    .iter()
                    .map(|t| Opt {
                        value: (*t).to_string(),
                        label: super::term_label(lang, &vocab::ARTEFACT_TYPE, t),
                    })
                    .collect(),
                required: true,
                ..blank_input()
            });
            for k in ["format", "generator"] {
                out.push(Input {
                    name: format!("{full}.{k}"),
                    label: fl(k),
                    value: sub(k),
                    ..blank_input()
                });
            }
            out.push(Input {
                name: format!("{full}.derived_from_id"),
                label: fl("derived_from_id"),
                kind: "select",
                value: sub("derived_from_id"),
                options: entity_opts(&choices.documents),
                ..blank_input()
            });
            out.push(Input {
                name: format!("{full}.consent"),
                label: fl("consent"),
                kind: "select",
                value: sub("consent"),
                options: vocabulary_options(lang, &vocab::CONSENT),
                ..blank_input()
            });
        }
        Shape::Object { fields, .. } => {
            for f in fields.iter() {
                push_inputs(
                    a,
                    &f.shape,
                    base,
                    &format!("{name}.{}", f.key),
                    Some((f.key, f.required)),
                    values,
                    choices,
                    out,
                );
            }
        }
    }
}

fn unit_symbol(lang: &str, unit: &str) -> Option<String> {
    super::unit_key(unit).map(|_| super::with_unit(lang, unit, "").trim().to_string())
}

fn entity_opts(list: &[Value]) -> Vec<Opt> {
    list.iter()
        .filter_map(|e| {
            Some(Opt {
                value: e.get("id")?.as_str()?.to_string(),
                label: e.get("label")?.as_str()?.to_string(),
            })
        })
        .collect()
}

// ---------------------------------------------------------------------------
// reading back
// ---------------------------------------------------------------------------

/// Problems, keyed by attribute path, as catalogue keys.
pub type Problems = BTreeMap<String, Vec<&'static str>>;

fn problem(p: &mut Problems, path: &str, key: &'static str) {
    let list = p.entry(path.to_string()).or_default();
    if !list.contains(&key) {
        list.push(key);
    }
}

/// The person with this group's readable attributes replaced by what the form
/// holds, or the problems that stop it being saved.
///
/// `person` is the lifted stored person the form was drawn from. Attributes
/// the editor may not read are left exactly as they are here, and the caller
/// restores them from the stored entity afterwards.
pub fn apply(person: &Value, g: &Group, readable: Scopes, form: &Body) -> Result<Value, Problems> {
    let mut out = person.clone();
    let mut problems = Problems::new();
    for a in g.attributes {
        if !super::attribute_scopes(a).within(readable) {
            continue;
        }
        let stored: Vec<Value> = match person.get(a.block).and_then(|b| b.get(a.key)) {
            Some(Value::Array(list)) => list.clone(),
            Some(v) if v.is_object() => vec![v.clone()],
            _ => Vec::new(),
        };
        let mut claims = Vec::new();
        for r in forms::rows(form, a.path) {
            if forms::checked(&r, "remove") {
                continue;
            }
            let base = r
                .get("ci")
                .and_then(|c| c.parse::<usize>().ok())
                .and_then(|i| stored.get(i))
                .cloned();
            let base_value = base.as_ref().and_then(|b| b.get("value")).cloned();
            let Some(value) = parse_value(a, &a.shape, "v", base_value.as_ref(), &r, &mut problems)
            else {
                continue;
            };
            claims.push(claim(base, value, &r, a.path, &mut problems));
        }
        set_attribute(&mut out, a, claims);
    }

    for left in super::lift::leftovers(person)
        .into_iter()
        .filter(|l| l.group == g.key)
    {
        if !crate::sensitive::location_scopes("extensions", left.extension).within(readable) {
            continue;
        }
        let prefix = earlier_prefix(left.extension, &left.field);
        let mut entries = Vec::new();
        for r in forms::rows(form, &prefix) {
            let Some(text) = forms::field(&r, "v") else {
                continue;
            };
            entries.push(claim(None, json!(text), &r, &prefix, &mut problems));
        }
        set_earlier(&mut out, left.extension, &left.field, entries);
    }

    if problems.is_empty() {
        Ok(out)
    } else {
        Err(problems)
    }
}

/// Build one claim from a row, starting from the stored claim it was drawn
/// from so that nothing this form does not edit is lost.
fn claim(
    base: Option<Value>,
    value: Value,
    r: &RowValues,
    path: &str,
    problems: &mut Problems,
) -> Value {
    let mut c = match base {
        Some(Value::Object(m)) => m,
        _ => Map::new(),
    };
    c.insert("value".into(), value);
    let date = |p: &str| {
        forms::date_object(
            r.get(&format!("{p}.date")).map(String::as_str),
            r.get(&format!("{p}.precision")).map(String::as_str),
            r.get(&format!("{p}.circa")).is_some_and(|v| v == "on"),
        )
    };
    match date("d") {
        Some(d) => {
            c.insert("date".into(), d);
        }
        None => {
            c.remove("date");
        }
    }
    for (key, p) in [("valid_from", "f"), ("valid_until", "u")] {
        let event = c.get(key).and_then(|b| b.get("event_id")).cloned();
        match (date(p), event) {
            (Some(d), event) => {
                let mut bound = Map::new();
                bound.insert("date".into(), d);
                if let Some(e) = event {
                    bound.insert("event_id".into(), e);
                }
                c.insert(key.into(), Value::Object(bound));
            }
            (None, Some(e)) => {
                c.insert(key.into(), json!({"event_id": e}));
            }
            (None, None) => {
                c.remove(key);
            }
        }
    }
    match forms::field(r, "source_id") {
        Some(s) => {
            c.insert("source_id".into(), json!(s));
        }
        None => {
            c.remove("source_id");
        }
    }
    match forms::field(r, "confidence") {
        Some(raw) => match raw.parse::<f64>() {
            Ok(x) if (0.0..=1.0).contains(&x) => {
                c.insert("confidence".into(), json!(x));
            }
            _ => problem(problems, path, "profile-error-confidence"),
        },
        None => {
            c.remove("confidence");
        }
    }
    match forms::field(r, "note") {
        Some(n) => {
            c.insert("note".into(), json!(n));
        }
        None => {
            c.remove("note");
        }
    }
    Value::Object(c)
}

/// A number as JSON, keeping the spelling it was typed in.
fn number(raw: &str) -> Option<Value> {
    let n: serde_json::Number = raw.trim().parse().ok()?;
    n.as_f64().filter(|x| x.is_finite())?;
    Some(Value::Number(n))
}

/// Read one value out of a row. `None` means blank: nothing to save.
fn parse_value(
    a: &Attribute,
    shape: &Shape,
    name: &str,
    base: Option<&Value>,
    r: &RowValues,
    problems: &mut Problems,
) -> Option<Value> {
    let raw = forms::field(r, name);
    let path = a.path;
    match shape {
        Shape::Text => raw.map(|s| json!(s)),
        Shape::Pattern(_) => raw.map(|s| json!(s)),
        Shape::Uuid => raw.map(|s| json!(s)),
        Shape::Number { min, max, .. } => {
            let s = raw?;
            let Some(v) = number(s) else {
                problem(problems, path, "profile-error-number");
                return None;
            };
            let x = v.as_f64().unwrap_or(f64::NAN);
            if min.is_some_and(|m| x < m) || max.is_some_and(|m| x > m) {
                problem(problems, path, "profile-error-range");
                return None;
            }
            Some(v)
        }
        Shape::Integer { min, max, .. } => {
            let s = raw?;
            let Ok(x) = s.trim().parse::<i64>() else {
                problem(problems, path, "profile-error-integer");
                return None;
            };
            if min.is_some_and(|m| x < m) || max.is_some_and(|m| x > m) {
                problem(problems, path, "profile-error-range");
                return None;
            }
            Some(json!(x))
        }
        Shape::Boolean => match raw? {
            "yes" | "true" => Some(json!(true)),
            "no" | "false" => Some(json!(false)),
            _ => {
                problem(problems, path, "profile-error-term");
                None
            }
        },
        Shape::TimeOfDay => {
            let s = raw?;
            if valid_time(s) {
                Some(json!(s))
            } else {
                problem(problems, path, "profile-error-time");
                None
            }
        }
        Shape::CurrencyCode => {
            let s = raw?.to_ascii_uppercase();
            if s.len() == 3 && s.chars().all(|c| c.is_ascii_uppercase()) {
                Some(json!(s))
            } else {
                problem(problems, path, "profile-error-currency");
                None
            }
        }
        Shape::LanguageTag => {
            let s = raw?;
            if valid_language_tag(s) {
                Some(json!(s))
            } else {
                problem(problems, path, "profile-error-language");
                None
            }
        }
        Shape::Vocab(v) => {
            let s = raw?;
            if v.contains(s) {
                Some(json!(s))
            } else {
                problem(problems, path, "profile-error-term");
                None
            }
        }
        Shape::Rank => raw.map(|s| json!(s)),
        Shape::Coordinates => {
            let lat = forms::field(r, &format!("{name}.lat"));
            let lon = forms::field(r, &format!("{name}.lon"));
            let precision = forms::field(r, &format!("{name}.precision"));
            if lat.is_none() && lon.is_none() {
                return None;
            }
            let (Some(lat), Some(lon)) = (lat.and_then(number), lon.and_then(number)) else {
                problem(problems, path, "profile-error-coordinates");
                return None;
            };
            let (la, lo) = (lat.as_f64().unwrap_or(99.0), lon.as_f64().unwrap_or(999.0));
            if !(-90.0..=90.0).contains(&la) || !(-180.0..=180.0).contains(&lo) {
                problem(problems, path, "profile-error-coordinates");
                return None;
            }
            let mut m = Map::new();
            m.insert("lat".into(), lat);
            m.insert("lon".into(), lon);
            if let Some(p) = precision {
                m.insert("precision".into(), json!(p));
            }
            Some(Value::Object(m))
        }
        Shape::Artefact(types) => {
            let mut m = match base {
                Some(Value::Object(m)) => m.clone(),
                _ => Map::new(),
            };
            let mut any = false;
            for k in [
                "document_id",
                "artefact_type",
                "format",
                "generator",
                "derived_from_id",
                "consent",
            ] {
                match forms::field(r, &format!("{name}.{k}")) {
                    Some(s) => {
                        any = true;
                        m.insert(k.into(), json!(s));
                    }
                    None => {
                        m.remove(k);
                    }
                }
            }
            if !any {
                return None;
            }
            match m.get("artefact_type").and_then(Value::as_str) {
                Some(t) if types.contains(&t) => {}
                _ => problem(problems, path, "profile-error-term"),
            }
            if m.get("document_id").is_none() {
                problem(problems, path, "profile-error-required");
            }
            if let Some(c) = m.get("consent").and_then(Value::as_str) {
                if !vocab::CONSENT.contains(c) {
                    problem(problems, path, "profile-error-term");
                }
            }
            Some(Value::Object(m))
        }
        Shape::Object { fields, one_of } => {
            let mut m = match base {
                Some(Value::Object(m)) => m.clone(),
                _ => Map::new(),
            };
            let mut any = false;
            for f in fields.iter() {
                let sub = format!("{name}.{}", f.key);
                let before = problems.get(path).map(Vec::len).unwrap_or(0);
                let parsed = parse_value(a, &f.shape, &sub, m.get(f.key), r, problems);
                let failed = problems.get(path).map(Vec::len).unwrap_or(0) > before;
                match parsed {
                    Some(v) => {
                        any = true;
                        m.insert(f.key.into(), v);
                    }
                    None if failed => any = true,
                    None => {
                        m.remove(f.key);
                    }
                }
            }
            if !any {
                return None;
            }
            // A rank carries its country: take it from the rank's option when
            // the country field was left blank, and refuse a contradiction.
            if let Some(Value::String(choice)) = m.get("rank").cloned() {
                match choice.split_once(':') {
                    Some((country, term)) => {
                        match m.get("country").and_then(Value::as_str) {
                            None => {
                                m.insert("country".into(), json!(country));
                            }
                            Some(c) if c != country => {
                                problem(problems, path, "profile-error-rank-country")
                            }
                            _ => {}
                        }
                        if super::rank_title(country, term).is_none() {
                            problem(problems, path, "profile-error-term");
                        }
                        m.insert("rank".into(), json!(term));
                    }
                    None => problem(problems, path, "profile-error-term"),
                }
            }
            for f in fields.iter().filter(|f| f.required) {
                if !m.contains_key(f.key) {
                    problem(problems, path, "profile-error-required");
                }
            }
            // At least one of the groups, whole: a rank or a rank as written.
            if !one_of.is_empty()
                && !one_of
                    .iter()
                    .any(|group| group.iter().all(|k| m.contains_key(*k)))
            {
                problem(problems, path, "profile-error-one-of");
            }
            Some(Value::Object(m))
        }
    }
}

fn valid_time(s: &str) -> bool {
    let parts: Vec<&str> = s.split(':').collect();
    let ok = |p: &str, max: u32| p.len() == 2 && p.parse::<u32>().is_ok_and(|n| n <= max);
    match parts.as_slice() {
        [h, m] => ok(h, 23) && ok(m, 59),
        [h, m, sec] => ok(h, 23) && ok(m, 59) && ok(sec, 59),
        _ => false,
    }
}

fn valid_language_tag(s: &str) -> bool {
    let mut parts = s.split('-');
    let Some(first) = parts.next() else {
        return false;
    };
    (2..=3).contains(&first.len())
        && first.chars().all(|c| c.is_ascii_alphabetic())
        && parts.all(|p| (1..=8).contains(&p.len()) && p.chars().all(|c| c.is_ascii_alphanumeric()))
}

/// Write an attribute's claims, or remove it when there are none.
fn set_attribute(person: &mut Value, a: &Attribute, mut claims: Vec<Value>) {
    let Some(obj) = person.as_object_mut() else {
        return;
    };
    let value = match a.cardinality {
        Cardinality::Single if claims.is_empty() => None,
        Cardinality::Single => Some(claims.swap_remove(0)),
        Cardinality::Series if claims.is_empty() => None,
        Cardinality::Series => Some(Value::Array(claims)),
    };
    match value {
        Some(v) => {
            let block = obj
                .entry(a.block.to_string())
                .or_insert_with(|| Value::Object(Map::new()));
            if let Some(b) = block.as_object_mut() {
                b.insert(a.key.to_string(), v);
            }
        }
        None => {
            let emptied = match obj.get_mut(a.block).and_then(Value::as_object_mut) {
                Some(b) => {
                    b.remove(a.key);
                    b.is_empty()
                }
                None => false,
            };
            // An empty block says nothing, and says it as though somebody had
            // looked. `identity` is required and is never removed.
            if emptied && a.block != "identity" {
                obj.remove(a.block);
            }
        }
    }
}

fn set_earlier(person: &mut Value, extension: &str, field: &str, entries: Vec<Value>) {
    let Some(obj) = person.as_object_mut() else {
        return;
    };
    let ext = obj
        .entry("extensions")
        .or_insert_with(|| Value::Object(Map::new()));
    let Some(ext) = ext.as_object_mut() else {
        return;
    };
    let holder = ext
        .entry(extension.to_string())
        .or_insert_with(|| Value::Object(Map::new()));
    if let Some(h) = holder.as_object_mut() {
        if entries.is_empty() {
            h.remove(field);
        } else {
            h.insert(field.to_string(), Value::Array(entries));
        }
        if h.is_empty() {
            ext.remove(extension);
        }
    }
    if ext.is_empty() {
        obj.remove("extensions");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axgf_rs::model::profile::registry;

    fn body(pairs: &[(&str, &str)]) -> Body {
        pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }

    fn g(key: &str) -> &'static Group {
        registry::group(key).expect("group")
    }

    fn choices() -> Choices {
        Choices {
            lang: "en".into(),
            sources: vec![json!({"id": "s1", "label": "Parish register"})],
            places: vec![json!({"id": "p1", "label": "Kraków"})],
            documents: vec![json!({"id": "d1", "label": "card.png"})],
        }
    }

    #[test]
    fn a_series_is_read_back_in_row_order_with_its_provenance() {
        let form = body(&[
            ("morphology.height.0.v", "172"),
            ("morphology.height.0.d.date", "1914"),
            ("morphology.height.0.source_id", "s1"),
            ("morphology.height.0.confidence", "0.9"),
            ("morphology.height.1.v", "169.5"),
            ("morphology.height.1.f.date", "1950-06"),
            ("morphology.height.2.v", ""),
        ]);
        let out = apply(
            &json!({"identity": {}}),
            g("morphology"),
            Scopes::EVERY,
            &form,
        )
        .unwrap();
        let h = out["morphology"]["height"].as_array().unwrap();
        assert_eq!(h.len(), 2, "the blank spare row is not a claim");
        assert_eq!(
            h[0]["value"].to_string(),
            "172",
            "a whole number stays whole"
        );
        assert_eq!(h[0]["date"]["precision"], "year");
        assert_eq!(h[0]["source_id"], "s1");
        assert_eq!(h[1]["value"].to_string(), "169.5");
        assert_eq!(h[1]["valid_from"]["date"]["value"], "1950-06");
    }

    #[test]
    fn a_claim_keeps_what_the_form_does_not_edit() {
        let stored = json!({"identity": {}, "health": {"conditions": [
            {"value": {"description": "Asthma", "future_field": 1}, "event_id": "e1",
             "valid_from": {"event_id": "e2"}, "unknown_claim_key": true}
        ]}});
        let form = body(&[
            ("health.conditions.0.ci", "0"),
            ("health.conditions.0.v.description", "Asthma, severe"),
            ("health.conditions.0.v.chronic", "yes"),
        ]);
        let out = apply(&stored, g("health"), Scopes::EVERY, &form).unwrap();
        let c = &out["health"]["conditions"][0];
        assert_eq!(c["value"]["description"], "Asthma, severe");
        assert_eq!(c["value"]["chronic"], true);
        assert_eq!(c["value"]["future_field"], 1);
        assert_eq!(c["event_id"], "e1");
        assert_eq!(c["valid_from"]["event_id"], "e2");
        assert_eq!(c["unknown_claim_key"], true);
    }

    #[test]
    fn an_emptied_attribute_and_block_are_removed() {
        let stored =
            json!({"identity": {}, "legal": {"criminal_record": [{"value": {"offence": "x"}}]}});
        let form = body(&[
            ("legal.criminal_record.0.ci", "0"),
            ("legal.criminal_record.0.remove", "on"),
            ("legal.criminal_record.0.v.offence", "x"),
        ]);
        let out = apply(&stored, g("legal"), Scopes::EVERY, &form).unwrap();
        assert!(out.get("legal").is_none(), "{out}");
    }

    #[test]
    fn a_value_no_select_could_send_is_a_problem() {
        let form = body(&[("morphology.eye_colour.0.v", "puce")]);
        let err = apply(
            &json!({"identity": {}}),
            g("morphology"),
            Scopes::EVERY,
            &form,
        )
        .unwrap_err();
        assert_eq!(err["morphology.eye_colour"], vec!["profile-error-term"]);
        let form = body(&[("morphology.height.0.v", "900")]);
        let err = apply(
            &json!({"identity": {}}),
            g("morphology"),
            Scopes::EVERY,
            &form,
        )
        .unwrap_err();
        assert_eq!(err["morphology.height"], vec!["profile-error-range"]);
        let form = body(&[("health.conditions.0.v.chronic", "yes")]);
        let err = apply(&json!({"identity": {}}), g("health"), Scopes::EVERY, &form).unwrap_err();
        assert_eq!(
            err["health.conditions"],
            vec!["profile-error-required"],
            "a description is required"
        );
    }

    #[test]
    fn a_rank_takes_its_country_from_the_option_and_refuses_a_contradiction() {
        let form = body(&[("military.ranks.0.v.rank", "PL:kapral")]);
        let out = apply(
            &json!({"identity": {}}),
            g("military"),
            Scopes::EVERY,
            &form,
        )
        .unwrap();
        assert_eq!(
            out["military"]["ranks"][0]["value"],
            json!({"country": "PL", "rank": "kapral"})
        );
        let form = body(&[
            ("military.ranks.0.v.rank", "PL:kapral"),
            ("military.ranks.0.v.country", "FR"),
        ]);
        let err = apply(
            &json!({"identity": {}}),
            g("military"),
            Scopes::EVERY,
            &form,
        )
        .unwrap_err();
        assert!(err["military.ranks"].contains(&"profile-error-rank-country"));
    }

    #[test]
    fn an_attribute_the_editor_may_not_read_is_not_read_back() {
        use crate::sensitive::Scope;
        let stored = json!({"identity": {}, "biometrics": {
            "handedness": [{"value": "left"}],
            "hearing": [{"value": {"grade": "mild"}}]}});
        let form = body(&[
            ("biometrics.handedness.0.v", "right"),
            ("biometrics.hearing.0.v.grade", "severe"),
        ]);
        let readable = Scopes::EVERY.without(Scopes::one(Scope::Health));
        let out = apply(&stored, g("biometrics"), readable, &form).unwrap();
        assert_eq!(out["biometrics"]["handedness"][0]["value"], "right");
        assert_eq!(
            out["biometrics"]["hearing"][0]["value"]["grade"], "mild",
            "untouched"
        );
    }

    #[test]
    fn a_stored_person_draws_one_row_per_claim_and_one_to_spare() {
        let person = json!({"identity": {}, "morphology": {
            "height": [{"value": 172, "date": {"value": "1914", "precision": "year"}}],
            "skin_tone": {"value": "type_iii"}}});
        let eg = editor_group(
            &person,
            g("morphology"),
            Scopes::EVERY,
            &choices(),
            None,
            &Problems::new(),
        );
        let height = eg
            .attributes
            .iter()
            .find(|a| a.prefix == "morphology.height")
            .unwrap();
        assert_eq!(height.rows.len(), 2);
        assert_eq!(height.rows[0].inputs[0].value, "172");
        assert_eq!(height.rows[0].date.date, "1914");
        assert!(height.rows[0].has_provenance);
        assert!(height.rows[1].spare);
        let tone = eg
            .attributes
            .iter()
            .find(|a| a.prefix == "morphology.skin_tone")
            .unwrap();
        assert_eq!(tone.rows.len(), 1, "a single claim has no spare row");
        assert_eq!(tone.rows[0].inputs[0].kind, "select");
        assert_eq!(tone.rows[0].inputs[0].value, "type_iii");
    }

    #[test]
    fn a_drawn_group_reads_back_to_the_same_person() {
        // Render every attribute of every group from a filled person, submit
        // exactly what the inputs hold, and nothing may change.
        let person = json!({"identity": {}, "military": {"ranks": [
            {"value": {"country": "PL", "rank": "kapral", "category": "non_commissioned"},
             "date": {"value": "1939", "precision": "year"}, "confidence": 0.7}]},
            "residence": {"addresses": [{"value": {"lines": "ul. Długa 5", "country": "PL",
             "coordinates": {"lat": 50.06, "lon": 19.94, "precision": "street"}}}]}});
        for key in ["military", "residence"] {
            let eg = editor_group(
                &person,
                g(key),
                Scopes::EVERY,
                &choices(),
                None,
                &Problems::new(),
            );
            let mut form = Body::new();
            for a in &eg.attributes {
                for r in &a.rows {
                    if let Some(ci) = r.ci {
                        form.insert(format!("{}.{}.ci", a.prefix, r.index), ci.to_string());
                    }
                    for i in &r.inputs {
                        form.insert(i.name.clone(), i.value.clone());
                    }
                    for (p, d) in [("d", &r.date), ("f", &r.from), ("u", &r.until)] {
                        form.insert(format!("{}.{}.{p}.date", a.prefix, r.index), d.date.clone());
                        form.insert(
                            format!("{}.{}.{p}.precision", a.prefix, r.index),
                            d.precision.clone(),
                        );
                    }
                    form.insert(
                        format!("{}.{}.confidence", a.prefix, r.index),
                        r.confidence.clone(),
                    );
                }
            }
            let back = apply(&person, g(key), Scopes::EVERY, &form).unwrap();
            assert_eq!(back[key], person[key], "{key} changed on a round trip");
        }
    }

    #[test]
    fn times_and_language_tags_are_checked() {
        assert!(valid_time("05:40") && valid_time("23:59:59"));
        assert!(!valid_time("24:00") && !valid_time("5:40") && !valid_time("05:60"));
        assert!(
            valid_language_tag("pl")
                && valid_language_tag("zh-Hans-CN")
                && valid_language_tag("und")
        );
        assert!(!valid_language_tag("polish-language") && !valid_language_tag("p"));
    }
}
