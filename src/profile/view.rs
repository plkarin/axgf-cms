//! A profile group as a reader sees it.
//!
//! The person has been lifted (see [`super::lift`]) before it reaches here,
//! and the reader's scopes decide, attribute by attribute, what is drawn.
//! Nothing withheld is rendered and then hidden: an attribute whose scopes are
//! not within the reader's is never turned into a view at all, and the group
//! says instead which classes it holds that this reader may not see — a record
//! that shows nothing where a diagnosis exists reads as a record with nothing
//! in it.

use std::collections::BTreeSet;

use axgf_rs::model::profile::registry::{Attribute, Cardinality, Group, Shape};
use serde::Serialize;
use serde_json::Value;

use super::lift;
use crate::sensitive::{Scope, Scopes};
use crate::view::{Confidence, DateDisplay};

/// What rendering a group needs to know about the request.
pub struct Reader<'a> {
    pub flat: &'a Value,
    pub lang: &'a str,
    /// The scopes of this person the reader may read.
    pub readable: Scopes,
    /// Documents this reader may not open, however they are referred to.
    pub withheld_documents: &'a BTreeSet<String>,
}

/// One entry in the strip of groups.
#[derive(Debug, Clone, Serialize)]
pub struct GroupTab {
    pub key: &'static str,
    pub title: String,
    /// Claims this reader can see in the group.
    pub count: usize,
    /// Whether the group holds anything this reader may not see.
    pub withheld: bool,
}

/// One group, drawn.
#[derive(Debug, Clone, Serialize)]
pub struct GroupView {
    pub key: &'static str,
    pub title: String,
    pub intro: String,
    pub attributes: Vec<AttributeView>,
    /// The classes present in this group that this reader may not see, named
    /// in the reader's language.
    pub withheld: Vec<String>,
    /// Nothing recorded that this reader can see, and nothing withheld.
    pub is_empty: bool,
    pub count: usize,
    /// The sensitive class every attribute of this group belongs to, named in
    /// the reader's language, when there is one: Health, Genomics, Legal and
    /// Belief are one class throughout, and a badge on every row of them says
    /// the same thing seven times.
    pub class_label: Option<String>,
}

/// One attribute and every claim recorded for it.
#[derive(Debug, Clone, Serialize)]
pub struct AttributeView {
    pub path: &'static str,
    pub label: String,
    pub is_series: bool,
    /// The name of its sensitive class, for the small badge beside a label an
    /// administrator is looking at: they read everything, and should be told
    /// which of it is a special category.
    pub class_label: Option<String>,
    pub claims: Vec<ClaimView>,
    /// True for a field this application recorded before AXGF 1.1 that has no
    /// 1.1 home, shown under its old label.
    pub earlier: bool,
}

/// One claim.
#[derive(Debug, Clone, Serialize, Default)]
pub struct ClaimView {
    /// The value in one line, for anything that is not an object.
    pub text: Option<String>,
    /// An object value, field by field.
    pub parts: Vec<Part>,
    /// The document an artefact refers to.
    pub document: Option<DocumentLink>,
    pub date: Option<DateDisplay>,
    pub from: Option<DateDisplay>,
    pub until: Option<DateDisplay>,
    pub source: Option<crate::person::SourceView>,
    pub confidence: Option<Confidence>,
    pub note: Option<String>,
}

/// One field of an object value.
#[derive(Debug, Clone, Serialize)]
pub struct Part {
    pub label: String,
    pub text: String,
    pub document: Option<DocumentLink>,
}

/// A document a claim refers to.
#[derive(Debug, Clone, Serialize)]
pub struct DocumentLink {
    pub id: String,
    pub name: String,
    /// False when the bundle does not hold it, so the template draws a name
    /// rather than a link into a 404.
    pub known: bool,
}

/// How many claims one attribute holds.
fn claim_count(person: &Value, a: &Attribute) -> usize {
    match person.get(a.block).and_then(|b| b.get(a.key)) {
        None | Some(Value::Null) => 0,
        Some(Value::Array(list)) => list.len(),
        Some(_) => 1,
    }
}

/// The strip of groups, with what each holds for this reader.
pub fn tabs(person: &Value, reader: &Reader<'_>) -> Vec<GroupTab> {
    super::groups()
        .iter()
        .map(|g| {
            let (count, withheld) = tally(person, g, reader);
            GroupTab {
                key: g.key,
                title: super::group_title(reader.lang, g),
                count,
                withheld,
            }
        })
        .collect()
}

/// Claims visible to the reader, and whether anything is withheld.
fn tally(person: &Value, g: &Group, reader: &Reader<'_>) -> (usize, bool) {
    let mut count = 0;
    let mut withheld = false;
    for a in g.attributes {
        let n = claim_count(person, a);
        if n == 0 {
            continue;
        }
        if super::attribute_scopes(a).within(reader.readable) {
            count += n;
        } else {
            withheld = true;
        }
    }
    for left in lift::leftovers(person).iter().filter(|l| l.group == g.key) {
        if leftover_scopes(left).within(reader.readable) {
            count += left.entries.len();
        } else {
            withheld = true;
        }
    }
    (count, withheld)
}

fn leftover_scopes(left: &lift::Leftover) -> Scopes {
    crate::sensitive::location_scopes("extensions", left.extension)
}

/// One group, as this reader may see it.
pub fn group(person: &Value, g: &Group, reader: &Reader<'_>) -> GroupView {
    let mut attributes = Vec::new();
    let mut withheld_scopes = Scopes::NONE;
    for a in g.attributes {
        let Some(held) = person.get(a.block).and_then(|b| b.get(a.key)) else {
            continue;
        };
        if held.is_null() {
            continue;
        }
        let scopes = super::attribute_scopes(a);
        if !scopes.within(reader.readable) {
            withheld_scopes = withheld_scopes.union(scopes.without(reader.readable));
            continue;
        }
        let claims: Vec<&Value> = match (a.cardinality, held) {
            (Cardinality::Series, Value::Array(list)) => list.iter().collect(),
            (Cardinality::Single, v) if v.is_object() => vec![v],
            // A shape the schema does not allow cannot be drawn as a claim.
            // The library reports it; the raw dump still shows it.
            _ => continue,
        };
        let mut views: Vec<ClaimView> = claims
            .into_iter()
            .map(|c| claim_view(a, &a.shape, c, reader))
            .collect();
        sort_by_date(&mut views);
        if views.is_empty() {
            continue;
        }
        attributes.push(AttributeView {
            path: a.path,
            label: super::attribute_label(reader.lang, a),
            is_series: views.len() > 1,
            class_label: a.class.map(|c| {
                crate::i18n::translate(reader.lang, &format!("scope-{}", c.as_str()), None)
            }),
            claims: views,
            earlier: false,
        });
    }

    for left in lift::leftovers(person)
        .into_iter()
        .filter(|l| l.group == g.key)
    {
        let scopes = leftover_scopes(&left);
        if !scopes.within(reader.readable) {
            withheld_scopes = withheld_scopes.union(scopes.without(reader.readable));
            continue;
        }
        let mut views: Vec<ClaimView> = left
            .entries
            .iter()
            .map(|e| ClaimView {
                text: e.get("value").map(scalar),
                ..provenance(e, reader)
            })
            .collect();
        sort_by_date(&mut views);
        attributes.push(AttributeView {
            path: "",
            label: crate::i18n::translate(
                reader.lang,
                &format!("phys-field-{}", left.field.replace('_', "-")),
                None,
            ),
            is_series: views.len() > 1,
            class_label: (left.extension == crate::physical::HEALTH_KEY)
                .then(|| crate::i18n::translate(reader.lang, "scope-health", None)),
            claims: views,
            earlier: true,
        });
    }

    let withheld: Vec<String> = withheld_scopes
        .iter()
        .map(|s| crate::i18n::translate(reader.lang, &format!("scope-{}", s.as_str()), None))
        .collect();
    let count = attributes.iter().map(|a| a.claims.len()).sum();
    let class_label = group_class(g)
        .map(|c| crate::i18n::translate(reader.lang, &format!("scope-{}", c.as_str()), None));
    if class_label.is_some() {
        for a in attributes.iter_mut() {
            a.class_label = None;
        }
    }
    GroupView {
        class_label,
        key: g.key,
        title: super::group_title(reader.lang, g),
        intro: crate::i18n::translate(reader.lang, &super::group_intro_key(g), None),
        is_empty: attributes.is_empty() && withheld.is_empty(),
        attributes,
        withheld,
        count,
    }
}

/// The one class every attribute of a group belongs to, if there is one.
pub fn group_class(g: &Group) -> Option<axgf_rs::model::profile::registry::SensitiveClass> {
    let first = g.attributes.first()?.class?;
    g.attributes
        .iter()
        .all(|a| a.class == Some(first))
        .then_some(first)
}

/// Dated claims in date order, then the undated ones in the order recorded.
fn sort_by_date(views: &mut [ClaimView]) {
    views.sort_by(|a, b| {
        let key = |v: &ClaimView| v.date.as_ref().and_then(|d| d.sort);
        match (key(a), key(b)) {
            (Some(x), Some(y)) => x.cmp(&y),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => std::cmp::Ordering::Equal,
        }
    });
}

/// A claim's date, period, source, confidence and note.
fn provenance(claim: &Value, reader: &Reader<'_>) -> ClaimView {
    let date_of = |v: Option<&Value>| {
        v.filter(|d| d.is_object())
            .map(|d| crate::view::render_date_in(d, reader.lang))
    };
    ClaimView {
        date: date_of(claim.get("date")),
        from: date_of(claim.pointer("/valid_from/date")),
        until: date_of(claim.pointer("/valid_until/date")),
        source: claim
            .get("source_id")
            .and_then(Value::as_str)
            .map(|id| crate::person::source_view(reader.flat, id, reader.lang)),
        confidence: Confidence::from_field(claim, "confidence"),
        note: claim
            .get("note")
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|n| !n.is_empty())
            .map(str::to_string),
        ..Default::default()
    }
}

/// One claim, drawn.
pub fn claim_view(a: &Attribute, shape: &Shape, claim: &Value, reader: &Reader<'_>) -> ClaimView {
    let mut view = provenance(claim, reader);
    let value = claim.get("value").unwrap_or(&Value::Null);
    match shape {
        Shape::Object { fields, .. } => {
            // The first required free-text field is what the claim *is* — the
            // condition, the offence, the title as written — so it is drawn as
            // the value rather than as one labelled part among the others.
            let main = fields
                .iter()
                .find(|f| f.required && matches!(f.shape, Shape::Text))
                .map(|f| f.key);
            for f in fields.iter() {
                let Some(v) = value.get(f.key).filter(|v| !v.is_null()) else {
                    continue;
                };
                let (text, document) = render(a, &f.shape, f.key, v, value, reader);
                if text.is_empty() && document.is_none() {
                    continue;
                }
                if Some(f.key) == main && view.text.is_none() {
                    view.text = Some(text);
                    continue;
                }
                view.parts.push(Part {
                    label: super::field_label(reader.lang, a, f.key),
                    text,
                    document,
                });
            }
        }
        Shape::Artefact(_) => {
            view.document = value
                .get("document_id")
                .and_then(Value::as_str)
                .and_then(|id| document_link(id, reader));
            for key in ["artefact_type", "format", "generator", "consent"] {
                let Some(v) = value.get(key).and_then(Value::as_str) else {
                    continue;
                };
                let text = match key {
                    "artefact_type" => super::term_label(
                        reader.lang,
                        &axgf_rs::model::profile::vocab::ARTEFACT_TYPE,
                        v,
                    ),
                    "consent" => {
                        super::term_label(reader.lang, &axgf_rs::model::profile::vocab::CONSENT, v)
                    }
                    _ => v.to_string(),
                };
                view.parts.push(Part {
                    label: crate::i18n::translate(reader.lang, &super::field_key_shared(key), None),
                    text,
                    document: None,
                });
            }
        }
        other => {
            let (text, document) = render(a, other, "", value, value, reader);
            view.text = (!text.is_empty()).then_some(text);
            view.document = document;
        }
    }
    view
}

/// A scalar or nested value in one line. `sibling` is the object holding it,
/// which a rank needs for its country.
fn render(
    a: &Attribute,
    shape: &Shape,
    key: &str,
    v: &Value,
    sibling: &Value,
    reader: &Reader<'_>,
) -> (String, Option<DocumentLink>) {
    let lang = reader.lang;
    match shape {
        Shape::Text | Shape::Pattern(_) | Shape::TimeOfDay | Shape::CurrencyCode => {
            (scalar(v), None)
        }
        Shape::Number { unit, .. } | Shape::Integer { unit, .. } => {
            (super::with_unit(lang, unit, &scalar(v)), None)
        }
        Shape::Boolean => (
            crate::i18n::translate(
                lang,
                if v.as_bool() == Some(true) {
                    "profile-yes"
                } else {
                    "profile-no"
                },
                None,
            ),
            None,
        ),
        Shape::Vocab(voc) => (
            v.as_str()
                .map(|t| super::term_label(lang, voc, t))
                .unwrap_or_else(|| scalar(v)),
            None,
        ),
        Shape::LanguageTag => (
            v.as_str()
                .map(|t| language_name(lang, t))
                .unwrap_or_default(),
            None,
        ),
        Shape::Coordinates => (coordinates(lang, v), None),
        Shape::Rank => {
            let country = sibling.get("country").and_then(Value::as_str).unwrap_or("");
            let term = v.as_str().unwrap_or("");
            (
                super::rank_title(country, term)
                    .map(str::to_string)
                    .unwrap_or_else(|| term.to_string()),
                None,
            )
        }
        Shape::Uuid => {
            let Some(id) = v.as_str() else {
                return (String::new(), None);
            };
            match key {
                "document_id" | "derived_from_id" => {
                    let link = document_link(id, reader);
                    (
                        link.as_ref().map(|l| l.name.clone()).unwrap_or_default(),
                        link,
                    )
                }
                "place_id" => (place_name(reader.flat, id, lang), None),
                _ => (id.to_string(), None),
            }
        }
        Shape::Artefact(_) | Shape::Object { .. } => {
            let _ = a;
            (String::new(), None)
        }
    }
}

/// A value as the record states it.
fn scalar(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => String::new(),
        other => other.to_string(),
    }
}

fn coordinates(lang: &str, v: &Value) -> String {
    let (Some(lat), Some(lon)) = (
        v.get("lat").and_then(Value::as_f64),
        v.get("lon").and_then(Value::as_f64),
    ) else {
        return String::new();
    };
    let base = format!("{lat:.5}, {lon:.5}");
    match v.get("precision").and_then(Value::as_str) {
        Some(p) if crate::place::PRECISIONS.contains(&p) => format!(
            "{base} ({})",
            crate::i18n::translate(lang, &format!("place-precision-{p}"), None)
        ),
        _ => base,
    }
}

/// A language tag as its name, when the catalogue has one, else as written.
fn language_name(lang: &str, tag: &str) -> String {
    let base = tag
        .split(['-', '_'])
        .next()
        .unwrap_or(tag)
        .to_ascii_lowercase();
    let key = format!("lang-{base}");
    if crate::i18n::has_message(crate::i18n::DEFAULT, &key) {
        let name = crate::i18n::translate(lang, &key, None);
        if base.len() == tag.len() {
            name
        } else {
            format!("{name} ({tag})")
        }
    } else {
        tag.to_string()
    }
}

fn place_name(flat: &Value, id: &str, lang: &str) -> String {
    match flat.get("places").and_then(|p| p.get(id)) {
        Some(p) => crate::view::place_name(p),
        None => crate::i18n::translate(lang, "record-unknown-place", None),
    }
}

fn document_link(id: &str, reader: &Reader<'_>) -> Option<DocumentLink> {
    if reader.withheld_documents.contains(id) {
        return None;
    }
    let doc = reader.flat.get("documents").and_then(|d| d.get(id));
    Some(DocumentLink {
        id: id.to_string(),
        name: doc
            .and_then(|d| d.get("filename").or_else(|| d.get("caption")))
            .and_then(Value::as_str)
            .map(str::to_string)
            .unwrap_or_else(|| {
                crate::i18n::translate(reader.lang, "record-missing-document", None)
            }),
        known: doc.is_some(),
    })
}

/// Whether a person holds anything at all in the profile groups, for the tab
/// count on the record.
pub fn total(person: &Value, reader: &Reader<'_>) -> usize {
    super::groups()
        .iter()
        .map(|g| tally(person, g, reader).0)
        .sum()
}

/// Whether one scope is readable, for templates that ask about one class.
pub fn may_read(reader: &Reader<'_>, scope: Scope) -> bool {
    reader.readable.contains(scope)
}
