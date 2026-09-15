//! The sensitive classes, and the one place their rule is applied to a value.
//!
//! # What is governed
//!
//! AXGF 1.1 names four classes of attribute that are the most harmful to
//! disclose — `health`, `biometrics`, `genomics` and `legal` — and fixes the
//! class of every attribute in its schema (`x-axgf-class`). This application
//! governs one more thing the same way: the *Personality and behaviour* group
//! and `digital_legacy.behaviour_models` of a **living** person, which the
//! specification says taken together are an inferred psychological profile
//! and should be governed as a class (SPEC_1.1 §4.6). These five are the
//! [`Scope`]s. Who may read which scope of which person is decided in
//! [`crate::access::readable_scopes`]; this module never decides it.
//!
//! # Why one module
//!
//! Class data does not leak through the record page, which everybody thinks
//! about. It leaks through the surfaces nobody was thinking about: the edit
//! history printing back the value that changed, the raw-document textarea,
//! the conflict screen showing the stored entity, and the resubmit box that
//! holds what the write path just restored. Health data leaked through all
//! four in this application before, one at a time. So every one of them calls
//! the same three functions here, and a fifth surface is safe the day it
//! calls them too:
//!
//! * [`overlay`] — strip a value before it is shown, and put the stored value
//!   back into one that comes back from a form that never showed it. They are
//!   one operation: "every location this reader may not read takes its value
//!   from somewhere else", where somewhere else is nothing or the stored
//!   entity.
//! * [`change_scopes`] — which scopes a recorded change can carry.
//! * [`changes_for_reader`] — a history as one reader may see it.
//!
//! # Driven by the schema, and closed when in doubt
//!
//! Which location holds which class is read from the embedded 1.1 schema, not
//! written out here: an attribute the library gains is governed the day the
//! dependency is bumped. What the schema does *not* describe is treated as
//! every scope at once — a key 1.1 does not define inside a block that can
//! hold class data (`health.blod_group`), a top-level key no version defines,
//! an extension another application wrote. Such a key may well be harmless.
//! This module cannot know that, and the failure it is choosing between is
//! hiding a misspelt field from a contributor or publishing a diagnosis.

use std::collections::{BTreeSet, HashMap};
use std::sync::OnceLock;

use serde_json::{Map, Value};

use axgf_rs::model::profile::registry::SensitiveClass;

/// Something a reader can be denied on one person.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Scope {
    Health,
    Biometrics,
    Genomics,
    Legal,
    /// The behavioural profile of a living person. Not a class of the
    /// specification, so it never appears in `withheld_classes`.
    Behaviour,
}

impl Scope {
    /// Every scope, classes first, in the specification's order.
    pub const ALL: [Scope; 5] = [
        Scope::Health,
        Scope::Biometrics,
        Scope::Genomics,
        Scope::Legal,
        Scope::Behaviour,
    ];

    /// The four the specification defines.
    pub const CLASSES: [Scope; 4] = [
        Scope::Health,
        Scope::Biometrics,
        Scope::Genomics,
        Scope::Legal,
    ];

    /// The scope of a specification class.
    pub fn of(class: SensitiveClass) -> Self {
        match class {
            SensitiveClass::Health => Scope::Health,
            SensitiveClass::Biometrics => Scope::Biometrics,
            SensitiveClass::Genomics => Scope::Genomics,
            SensitiveClass::Legal => Scope::Legal,
        }
    }

    /// The specification class, when this scope is one.
    pub fn class(self) -> Option<SensitiveClass> {
        match self {
            Scope::Health => Some(SensitiveClass::Health),
            Scope::Biometrics => Some(SensitiveClass::Biometrics),
            Scope::Genomics => Some(SensitiveClass::Genomics),
            Scope::Legal => Some(SensitiveClass::Legal),
            Scope::Behaviour => None,
        }
    }

    /// The stable spelling: the class name, or `behaviour`.
    pub fn as_str(self) -> &'static str {
        match self.class() {
            Some(c) => c.as_str(),
            None => "behaviour",
        }
    }

    /// Read a scope back from its spelling.
    pub fn parse(s: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|x| x.as_str() == s)
    }

    fn bit(self) -> u8 {
        1 << (self as u8)
    }
}

/// A set of scopes. Five members fit in a byte, and a set is computed for
/// every field of every change of every history entry a page draws.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct Scopes(u8);

impl Scopes {
    /// Nothing.
    pub const NONE: Scopes = Scopes(0);
    /// All five. What an unknown location is governed by.
    pub const EVERY: Scopes = Scopes(0b1_1111);

    /// A set of one.
    pub fn one(s: Scope) -> Self {
        Scopes(s.bit())
    }

    /// This set with `s` in it.
    pub fn with(self, s: Scope) -> Self {
        Scopes(self.0 | s.bit())
    }

    /// Both sets together.
    pub fn union(self, other: Scopes) -> Self {
        Scopes(self.0 | other.0)
    }

    /// This set without the members of `other`.
    pub fn without(self, other: Scopes) -> Self {
        Scopes(self.0 & !other.0)
    }

    /// Whether `s` is a member.
    pub fn contains(self, s: Scope) -> bool {
        self.0 & s.bit() != 0
    }

    /// Whether there are no members.
    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Whether every member of this set is also in `readable` — which is the
    /// question "may this reader see a value governed by all of these".
    pub fn within(self, readable: Scopes) -> bool {
        self.0 & !readable.0 == 0
    }

    /// The members, in [`Scope::ALL`] order.
    pub fn iter(self) -> impl Iterator<Item = Scope> {
        Scope::ALL.into_iter().filter(move |s| self.contains(*s))
    }

    /// The members that are specification classes, as the bundle spells them.
    pub fn class_names(self) -> Vec<&'static str> {
        self.iter()
            .filter_map(Scope::class)
            .map(SensitiveClass::as_str)
            .collect()
    }
}

impl FromIterator<Scope> for Scopes {
    fn from_iter<I: IntoIterator<Item = Scope>>(iter: I) -> Self {
        iter.into_iter().fold(Scopes::NONE, Scopes::with)
    }
}

// ---------------------------------------------------------------------------
// where the classes live, read from the schema
// ---------------------------------------------------------------------------

/// The extension keys this application writes, and what each holds.
///
/// Every other extension key is somebody else's and is treated as every scope.
const OWN_EXTENSIONS: &[(&str, Option<Scope>)] = &[
    ("axgf-cms:traits/v1", None),
    ("axgf-cms:avatar/v1", None),
    ("axgf-cms:health/v1", Some(Scope::Health)),
];

/// How the value under one top-level key of a person is governed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rule {
    /// Not class data, and nothing inside it can be.
    Open,
    /// Governed key by key, each key by its class in the schema.
    Block,
    /// `identity`: open, except `class_visibility`, whose entries are
    /// governed by the class each one names.
    Identity,
    /// `extensions`: governed per key, see [`OWN_EXTENSIONS`].
    Extensions,
    /// A key no version of the schema defines.
    Unknown,
}

/// The table, built once from the embedded schema.
struct Table {
    /// Every top-level key a person may carry.
    top: BTreeSet<String>,
    /// Blocks governed key by key, with each known key's scopes.
    blocks: HashMap<String, HashMap<String, Scopes>>,
}

fn table() -> &'static Table {
    static TABLE: OnceLock<Table> = OnceLock::new();
    TABLE.get_or_init(|| {
        let schema: Value = serde_json::from_str(axgf_rs::boundary::lifecycle::EMBEDDED_SCHEMA_1_1)
            .expect("the embedded 1.1 schema is JSON");
        build_table(&schema)
    })
}

fn build_table(schema: &Value) -> Table {
    let defs = &schema["$defs"];
    let resolve = |node: &Value| -> Value {
        match node.get("$ref").and_then(Value::as_str) {
            Some(r) => defs[r.trim_start_matches("#/$defs/")].clone(),
            None => node.clone(),
        }
    };
    let mut top = BTreeSet::new();
    for def in ["base_entity", "person"] {
        if let Some(props) = defs[def]["properties"].as_object() {
            top.extend(props.keys().cloned());
        }
    }

    let profile: BTreeSet<&str> = axgf_rs::model::profile::registry::PROFILE_BLOCKS
        .iter()
        .copied()
        .collect();
    let mut blocks = HashMap::new();
    if let Some(props) = defs["person"]["properties"].as_object() {
        for (block, node) in props {
            let node = resolve(node);
            let Some(inner) = node["properties"].as_object() else {
                continue;
            };
            let mut keys: HashMap<String, Scopes> = inner
                .iter()
                .map(|(k, v)| {
                    let class = v
                        .get("x-axgf-class")
                        .and_then(Value::as_str)
                        .and_then(SensitiveClass::parse)
                        .map(Scope::of);
                    (k.clone(), class.map(Scopes::one).unwrap_or_default())
                })
                .collect();
            let holds_class = keys.values().any(|s| !s.is_empty());
            // A 1.1 profile block is governed key by key even when none of its
            // attributes carries a class today: it is where a future class
            // attribute would be added, and an unknown key in it is exactly
            // that attribute read by a build that predates it.
            if !(holds_class || profile.contains(block.as_str())) {
                continue;
            }
            if block == "personality" {
                for s in keys.values_mut() {
                    *s = s.with(Scope::Behaviour);
                }
            }
            if block == "digital_legacy" {
                if let Some(s) = keys.get_mut("behaviour_models") {
                    *s = s.with(Scope::Behaviour);
                }
            }
            blocks.insert(block.clone(), keys);
        }
    }
    Table { top, blocks }
}

fn rule(key: &str) -> Rule {
    let t = table();
    if key == "identity" {
        Rule::Identity
    } else if key == "extensions" {
        Rule::Extensions
    } else if t.blocks.contains_key(key) {
        Rule::Block
    } else if t.top.contains(key) {
        Rule::Open
    } else {
        Rule::Unknown
    }
}

/// The scopes of one key inside a governed block.
fn sub(block: &str, key: &str) -> Scopes {
    table()
        .blocks
        .get(block)
        .and_then(|m| m.get(key).copied())
        .unwrap_or(Scopes::EVERY)
}

/// The scopes of one entry of `identity.class_visibility`.
fn class_entry(key: &str) -> Scopes {
    match SensitiveClass::parse(key) {
        Some(c) => Scopes::one(Scope::of(c)),
        None => Scopes::EVERY,
    }
}

/// The scopes of one extension key.
fn extension(key: &str) -> Scopes {
    match OWN_EXTENSIONS.iter().find(|(k, _)| *k == key) {
        Some((_, Some(s))) => Scopes::one(*s),
        Some((_, None)) => Scopes::NONE,
        None => Scopes::EVERY,
    }
}

/// Every scope a location under this top-level key could ever hold, for when
/// the value itself cannot be examined.
fn possible_under(key: &str) -> Scopes {
    match rule(key) {
        Rule::Open => Scopes::NONE,
        _ => Scopes::EVERY,
    }
}

// ---------------------------------------------------------------------------
// values
// ---------------------------------------------------------------------------

/// Put `v` at `key`, or remove the key when there is nothing to put.
fn put(map: &mut Map<String, Value>, key: &str, v: Option<&Value>) {
    match v {
        Some(v) => {
            map.insert(key.to_string(), v.clone());
        }
        None => {
            map.remove(key);
        }
    }
}

/// The keys of both maps, each once.
fn keys_of(a: &Map<String, Value>, b: &Map<String, Value>) -> Vec<String> {
    let mut keys: Vec<String> = a.keys().chain(b.keys()).cloned().collect();
    keys.sort();
    keys.dedup();
    keys
}

/// Overlay the nested object at `key`, entry by entry: every entry whose
/// scopes are not within `readable` takes the source's value.
///
/// A side that is not an object cannot be examined, so the whole value is
/// then governed by `whole`.
fn overlay_nested(
    out: &mut Map<String, Value>,
    source: &Map<String, Value>,
    key: &str,
    readable: Scopes,
    whole: Scopes,
    scopes_of: impl Fn(&str) -> Scopes,
) {
    let empty = Map::new();
    let t = out.get(key);
    let s = source.get(key);
    let t_obj = match t {
        None => Some(&empty),
        Some(v) => v.as_object(),
    };
    let s_obj = match s {
        None => Some(&empty),
        Some(v) => v.as_object(),
    };
    let (Some(t_obj), Some(s_obj)) = (t_obj, s_obj) else {
        if !whole.within(readable) {
            put(out, key, s);
        }
        return;
    };
    let target_was_empty_object = t.is_some() && t_obj.is_empty();
    let mut merged = t_obj.clone();
    let mut touched = false;
    for k in keys_of(t_obj, s_obj) {
        if !scopes_of(&k).within(readable) {
            put(&mut merged, &k, s_obj.get(&k));
            touched = true;
        }
    }
    if !touched {
        return;
    }
    if merged.is_empty() && !target_was_empty_object {
        // An object emptied by the redaction is removed rather than left as
        // `{}`, which would still say there had been something in it.
        out.remove(key);
    } else {
        out.insert(key.to_string(), Value::Object(merged));
    }
}

/// Every location of a person this reader may not read, taken from `source`.
///
/// With an empty `source` this is the strip: what a reader is shown in a raw
/// view, a conflict screen or an export. With the stored entity as `source`
/// it is the restore: a form that was handed a stripped document cannot write
/// or erase what it never showed, because whatever it sends for those
/// locations is replaced by what the bundle holds.
///
/// Only persons carry class data; any other value is returned unchanged.
pub fn overlay(target: &Value, source: &Value, readable: Scopes) -> Value {
    let Some(t) = target.as_object() else {
        return target.clone();
    };
    if readable == Scopes::EVERY {
        return target.clone();
    }
    let empty = Map::new();
    let s = source.as_object().unwrap_or(&empty);
    let mut out = t.clone();
    for key in keys_of(t, s) {
        match rule(&key) {
            Rule::Open => {}
            Rule::Unknown => {
                if !Scopes::EVERY.within(readable) {
                    put(&mut out, &key, s.get(&key));
                }
            }
            Rule::Block => {
                overlay_nested(&mut out, s, &key, readable, Scopes::EVERY, |k| sub(&key, k));
            }
            Rule::Extensions => {
                overlay_nested(&mut out, s, &key, readable, Scopes::EVERY, extension);
            }
            Rule::Identity => {
                // The identity itself is never withheld: only its
                // `class_visibility`, entry by entry.
                let empty_identity = Map::new();
                let t_id = out.get("identity").and_then(Value::as_object);
                let s_id = s
                    .get("identity")
                    .and_then(Value::as_object)
                    .unwrap_or(&empty_identity);
                let mut identity = match t_id {
                    Some(m) => m.clone(),
                    // An identity that is not an object is not this
                    // module's to repair; the schema reports it.
                    None if out.contains_key("identity") => continue,
                    None => Map::new(),
                };
                let before = identity.clone();
                overlay_nested(
                    &mut identity,
                    s_id,
                    "class_visibility",
                    readable,
                    Scopes::EVERY,
                    class_entry,
                );
                if identity != before {
                    out.insert("identity".into(), Value::Object(identity));
                }
            }
        }
    }
    Value::Object(out)
}

/// A person as a reader may see it.
pub fn strip(entity: &Value, readable: Scopes) -> Value {
    overlay(entity, &Value::Object(Map::new()), readable)
}

/// A submitted person with every location the submitter may not read put
/// back exactly as `stored` holds it.
pub fn restore(submitted: &Value, stored: &Value, readable: Scopes) -> Value {
    overlay(submitted, stored, readable)
}

/// The scopes a person actually holds data in.
pub fn present(entity: &Value) -> Scopes {
    let Some(obj) = entity.as_object() else {
        return Scopes::NONE;
    };
    let mut out = Scopes::NONE;
    let nested = |v: &Value, scopes_of: &dyn Fn(&str) -> Scopes| -> Scopes {
        match v.as_object() {
            Some(m) => m
                .keys()
                .fold(Scopes::NONE, |acc, k| acc.union(scopes_of(k))),
            // Null is nothing; anything else unexaminable is everything.
            None if v.is_null() => Scopes::NONE,
            None => Scopes::EVERY,
        }
    };
    for (key, v) in obj {
        out = out.union(match rule(key) {
            Rule::Open => Scopes::NONE,
            Rule::Unknown => Scopes::EVERY,
            Rule::Block => nested(v, &|k| sub(key, k)),
            Rule::Extensions => nested(v, &extension),
            Rule::Identity => v
                .get("class_visibility")
                .map(|cv| nested(cv, &class_entry))
                .unwrap_or_default(),
        });
    }
    out
}

// ---------------------------------------------------------------------------
// recorded changes
// ---------------------------------------------------------------------------

/// The scopes one recorded change to a person can carry.
///
/// A change is a dotted path and two rendered values. Below a governed
/// location the path alone answers it. At or above one — `health`, or the
/// empty path of a whole entity, or `extensions` on the first save that adds
/// any — the change carries the whole object, so the answer is in the value:
/// it is parsed back and examined. A value that cannot be parsed, including
/// one the diff truncated, is assumed to hold everything it could.
pub fn change_scopes(c: &crate::diff::Change) -> Scopes {
    let segs: Vec<&str> = if c.path.is_empty() {
        Vec::new()
    } else {
        c.path.split('.').collect()
    };
    let Some(first) = segs.first().copied() else {
        return examine(c, &[], Scopes::EVERY);
    };
    match (rule(first), segs.get(1).copied()) {
        (Rule::Open, _) => Scopes::NONE,
        (Rule::Unknown, _) => Scopes::EVERY,
        (Rule::Block, Some(k)) => sub(first, k),
        (Rule::Extensions, Some(k)) => extension(k),
        (Rule::Identity, Some("class_visibility")) => match segs.get(2) {
            Some(k) => class_entry(k),
            None => examine(c, &segs, Scopes::EVERY),
        },
        (Rule::Identity, Some(_)) => Scopes::NONE,
        (_, None) => examine(c, &segs, possible_under(first)),
    }
}

/// The scopes present in a change's values, read back as JSON and placed at
/// the change's path; `otherwise` when either value cannot be read.
fn examine(c: &crate::diff::Change, segs: &[&str], otherwise: Scopes) -> Scopes {
    let mut out = Scopes::NONE;
    for rendered in [c.from.as_deref(), c.to.as_deref()].into_iter().flatten() {
        if rendered.ends_with('\u{2026}') {
            return otherwise;
        }
        let Ok(value) = serde_json::from_str::<Value>(rendered) else {
            return otherwise;
        };
        // Rebuild the entity around the value, so the same table that
        // governs a stored person governs this fragment of one.
        let mut wrapped = value;
        for seg in segs.iter().rev() {
            let mut m = Map::new();
            m.insert((*seg).to_string(), wrapped);
            wrapped = Value::Object(m);
        }
        out = out.union(present(&wrapped));
    }
    out
}

/// Recorded changes as one reader may see them.
///
/// A change that can carry a scope the reader may not read keeps its row and
/// loses its values. A reader told "this field changed and you may not see
/// how" has been told the truth; one shown a diff with a row silently missing
/// has been shown a diff that is wrong.
pub fn changes_for_reader(changes: &[crate::diff::Change], readable: Scopes) -> Vec<Value> {
    changes
        .iter()
        .map(|c| {
            let withheld = !change_scopes(c).within(readable);
            serde_json::json!({
                "path": c.path,
                "from": (!withheld).then(|| c.from.clone()).flatten(),
                "to": (!withheld).then(|| c.to.clone()).flatten(),
                "withheld": withheld,
            })
        })
        .collect()
}

// ---------------------------------------------------------------------------
// documents
// ---------------------------------------------------------------------------

/// Every document a person's class data refers to, with the scopes of the
/// location that refers to it.
///
/// A fingerprint card, a genome file or a voice corpus is stored as a Document
/// and referred to from the attribute that holds it. The document's bytes are
/// that attribute's data, so they are governed by its class wherever else the
/// document happens to be attached. Any `document_id` or `derived_from_id`
/// anywhere inside a governed location counts.
pub fn documents_referenced(person: &Value) -> Vec<(String, Scopes)> {
    let mut out = Vec::new();
    let Some(obj) = person.as_object() else {
        return out;
    };
    for (key, v) in obj {
        let scopes_of: Box<dyn Fn(&str) -> Scopes> = match rule(key) {
            Rule::Open | Rule::Identity => continue,
            Rule::Unknown => {
                collect_ids(v, Scopes::EVERY, &mut out);
                continue;
            }
            Rule::Block => {
                let block = key.clone();
                Box::new(move |k| sub(&block, k))
            }
            Rule::Extensions => Box::new(extension),
        };
        match v.as_object() {
            Some(m) => {
                for (k, inner) in m {
                    let s = scopes_of(k);
                    if !s.is_empty() {
                        collect_ids(inner, s, &mut out);
                    }
                }
            }
            None => collect_ids(v, Scopes::EVERY, &mut out),
        }
    }
    out
}

fn collect_ids(v: &Value, scopes: Scopes, out: &mut Vec<(String, Scopes)>) {
    match v {
        Value::Object(m) => {
            for (k, inner) in m {
                if matches!(k.as_str(), "document_id" | "derived_from_id") {
                    if let Some(id) = inner.as_str() {
                        out.push((id.to_string(), scopes));
                    }
                }
                collect_ids(inner, scopes, out);
            }
        }
        Value::Array(items) => {
            for inner in items {
                collect_ids(inner, scopes, out);
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn change(path: &str, from: Option<&str>, to: Option<&str>) -> crate::diff::Change {
        crate::diff::Change {
            path: path.into(),
            from: from.map(str::to_string),
            to: to.map(str::to_string),
        }
    }

    fn only(s: Scope) -> Scopes {
        Scopes::one(s)
    }

    fn all_but(s: Scope) -> Scopes {
        Scope::ALL.into_iter().filter(|x| *x != s).collect()
    }

    fn person() -> Value {
        json!({
            "id": "p", "type": "person", "axgf_version": "1.1",
            "identity": {"name": {"display": "N", "components": []}, "gender": {"value": "F"},
                         "is_living": true,
                         "class_visibility": {"legal": "private", "health": "members"}},
            "death": {"date": {"value": "1950"}, "cause": "typhus",
                      "causes": [{"value": {"description": "typhus"}}]},
            "morphology": {"height": [{"value": 158}]},
            "biometrics": {"handedness": [{"value": "left"}],
                           "hearing": [{"value": {"status": "impaired"}}],
                           "fingerprints": [{"value": {"document_id": "doc-finger", "artefact_type": "fingerprint_card"}}]},
            "health": {"blood_group": {"value": "A"}, "blod_group": {"value": "B"}},
            "genomics": {"y_haplogroup": {"value": {"major": "R"}}},
            "legal": {"criminal_record": [{"value": {"offence": "x"}}]},
            "belief": {"religions": [{"value": {"religion": "catholic"}}]},
            "personality": {"hobbies": [{"value": "chess"}], "dependencies": [{"value": {"substance": "tobacco"}}]},
            "digital_legacy": {"behaviour_models": [{"value": {"document_id": "doc-model", "artefact_type": "behaviour_model"}}],
                               "text_corpora": [{"value": {"document_id": "doc-letters", "artefact_type": "text_corpus"}}]},
            "extensions": {"axgf-cms:health/v1": {"conditions": [{"value": "x"}]},
                           "axgf-cms:avatar/v1": {"document_id": "doc-face"},
                           "org.example/notes": {"anything": true}},
            "future_block": {"a": 1}
        })
    }

    #[test]
    fn the_table_is_read_from_the_schema() {
        assert_eq!(sub("health", "blood_group"), only(Scope::Health));
        assert_eq!(sub("biometrics", "fingerprints"), only(Scope::Biometrics));
        assert_eq!(
            sub("biometrics", "hearing"),
            only(Scope::Health),
            "classes follow the data"
        );
        assert_eq!(sub("biometrics", "handedness"), Scopes::NONE);
        assert_eq!(
            sub("death", "cause"),
            only(Scope::Health),
            "1.0's own cause of death"
        );
        assert_eq!(sub("death", "date"), Scopes::NONE);
        assert_eq!(sub("personality", "hobbies"), only(Scope::Behaviour));
        assert_eq!(
            sub("personality", "dependencies"),
            only(Scope::Health).with(Scope::Behaviour)
        );
        assert_eq!(
            sub("digital_legacy", "behaviour_models"),
            only(Scope::Behaviour)
        );
        assert_eq!(sub("morphology", "height"), Scopes::NONE);
        assert_eq!(rule("notes"), Rule::Open);
        assert_eq!(rule("future_block"), Rule::Unknown);
    }

    #[test]
    fn an_unknown_key_is_every_scope_at_once() {
        assert_eq!(sub("health", "blod_group"), Scopes::EVERY);
        assert_eq!(sub("morphology", "future_attribute"), Scopes::EVERY);
        assert_eq!(extension("org.example/notes"), Scopes::EVERY);
        assert_eq!(extension("axgf-cms:avatar/v1"), Scopes::NONE);
        assert_eq!(class_entry("politics"), Scopes::EVERY);
    }

    #[test]
    fn stripping_one_class_leaves_every_other_class_and_the_open_data() {
        let p = person();
        let s = strip(&p, all_but(Scope::Genomics));
        assert!(s.get("genomics").is_none(), "the whole block goes: {s}");
        assert_eq!(s["legal"], p["legal"]);
        assert_eq!(s["health"]["blood_group"], p["health"]["blood_group"]);
        assert_eq!(s["morphology"], p["morphology"]);
        // Unknown keys are every scope, so they go with any one of them.
        assert!(s["health"].get("blod_group").is_none());
        assert!(s.get("future_block").is_none());
        assert!(s["extensions"].get("org.example/notes").is_none());
        assert!(s["extensions"].get("axgf-cms:avatar/v1").is_some());
    }

    #[test]
    fn stripping_health_reaches_every_block_health_lives_in() {
        let p = person();
        let s = strip(&p, all_but(Scope::Health));
        let text = s.to_string();
        for gone in [
            "blood_group",
            "hearing",
            "\"cause\"",
            "causes",
            "religions",
            "dependencies",
            "axgf-cms:health/v1",
        ] {
            assert!(!text.contains(gone), "{gone} survived: {text}");
        }
        assert_eq!(s["biometrics"]["handedness"], p["biometrics"]["handedness"]);
        assert_eq!(s["death"]["date"], p["death"]["date"]);
        assert_eq!(s["personality"]["hobbies"], p["personality"]["hobbies"]);
        assert!(s.get("belief").is_none(), "an emptied block is removed");
        assert_eq!(
            s["identity"]["class_visibility"],
            json!({"legal": "private"})
        );
    }

    #[test]
    fn a_reader_of_everything_gets_the_entity_untouched() {
        let p = person();
        assert_eq!(strip(&p, Scopes::EVERY), p);
    }

    #[test]
    fn restore_puts_back_what_was_withheld_and_ignores_what_was_sent_for_it() {
        let stored = person();
        let readable = all_but(Scope::Legal);
        let mut submitted = strip(&stored, readable);
        // What the form sent for the class it never showed is discarded…
        submitted["legal"] = json!({"criminal_record": [{"value": {"offence": "forged"}}]});
        // …and an edit to what it did show is kept.
        submitted["morphology"]["height"] = json!([{"value": 160}]);
        let back = restore(&submitted, &stored, readable);
        assert_eq!(back["legal"], stored["legal"]);
        assert_eq!(back["morphology"]["height"][0]["value"], 160);
        assert_eq!(back["identity"]["class_visibility"]["legal"], "private");
    }

    #[test]
    fn restore_removes_a_class_the_stored_entity_never_had() {
        let mut stored = person();
        stored.as_object_mut().unwrap().remove("genomics");
        let mut submitted = stored.clone();
        submitted["genomics"] = json!({"y_haplogroup": {"value": {"major": "I"}}});
        let back = restore(&submitted, &stored, all_but(Scope::Genomics));
        assert!(back.get("genomics").is_none(), "{back}");
    }

    #[test]
    fn what_a_person_holds_is_reported_per_scope() {
        let p = person();
        assert_eq!(
            present(&p),
            Scopes::EVERY,
            "the unknown block alone makes it everything"
        );
        let mut q = p.clone();
        for k in ["future_block", "extensions"] {
            q.as_object_mut().unwrap().remove(k);
        }
        q["health"].as_object_mut().unwrap().remove("blod_group");
        assert_eq!(present(&q), Scopes::EVERY, "every scope is really there");
        let plain = json!({"identity": {"name": {"display": "N"}}, "morphology": {"height": []}});
        assert_eq!(present(&plain), Scopes::NONE);
    }

    #[test]
    fn a_change_below_a_class_location_is_that_class() {
        let c = change("health.blood_group.value", Some("A"), Some("B"));
        assert_eq!(change_scopes(&c), only(Scope::Health));
        let c = change("genomics.y_haplogroup.value.major", Some("R"), Some("I"));
        assert_eq!(change_scopes(&c), only(Scope::Genomics));
        let c = change("identity.class_visibility.legal", None, Some("private"));
        assert_eq!(change_scopes(&c), only(Scope::Legal));
        let c = change(
            "extensions.axgf-cms:health/v1.conditions.0.value",
            None,
            Some("x"),
        );
        assert_eq!(change_scopes(&c), only(Scope::Health));
        let c = change("identity.name.display", Some("A"), Some("B"));
        assert_eq!(change_scopes(&c), Scopes::NONE);
    }

    #[test]
    fn a_whole_object_change_is_judged_by_what_it_carries() {
        // The first save that gives a person any extension at all records one
        // change at `extensions`, carrying the whole object.
        let avatar = change(
            "extensions",
            None,
            Some(r#"{"axgf-cms:avatar/v1":{"document_id":"d"}}"#),
        );
        assert_eq!(change_scopes(&avatar), Scopes::NONE);
        let health = change(
            "extensions",
            None,
            Some(r#"{"axgf-cms:health/v1":{"conditions":[]}}"#),
        );
        assert_eq!(change_scopes(&health), only(Scope::Health));
        let block = change(
            "biometrics",
            None,
            Some(r#"{"handedness":[],"fingerprints":[]}"#),
        );
        assert_eq!(change_scopes(&block), only(Scope::Biometrics));
        let cv = change(
            "identity.class_visibility",
            None,
            Some(r#"{"genomics":"private"}"#),
        );
        assert_eq!(change_scopes(&cv), only(Scope::Genomics));
    }

    #[test]
    fn a_value_that_cannot_be_examined_is_assumed_to_hold_everything() {
        let truncated = change(
            "biometrics",
            None,
            Some("{\"handedness\":[{\"value\":\"le\u{2026}"),
        );
        assert_eq!(change_scopes(&truncated), Scopes::EVERY);
        let whole = change("", None, Some("not json"));
        assert_eq!(change_scopes(&whole), Scopes::EVERY);
        let open = change("notes", None, Some("{\"a\u{2026}"));
        assert_eq!(
            change_scopes(&open),
            Scopes::NONE,
            "an open field has nothing to withhold"
        );
    }

    #[test]
    fn a_history_keeps_the_row_and_loses_the_values() {
        let changes = vec![
            change("health.rhesus.value", Some("positive"), Some("negative")),
            change("morphology.height.0.value", Some("158"), Some("160")),
        ];
        let rows = changes_for_reader(&changes, all_but(Scope::Health));
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0]["withheld"], true);
        assert!(rows[0]["from"].is_null() && rows[0]["to"].is_null());
        assert_eq!(rows[1]["withheld"], false);
        assert_eq!(rows[1]["to"], "160");
    }

    #[test]
    fn documents_behind_class_data_are_found_with_their_class() {
        let refs = documents_referenced(&person());
        let find = |id: &str| refs.iter().find(|(d, _)| d == id).map(|(_, s)| *s);
        assert_eq!(find("doc-finger"), Some(only(Scope::Biometrics)));
        assert_eq!(find("doc-model"), Some(only(Scope::Behaviour)));
        assert_eq!(find("doc-letters"), None, "a text corpus is not class data");
        assert_eq!(find("doc-face"), None, "the avatar extension is open");
    }
}
