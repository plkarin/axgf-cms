//! What this application recorded before AXGF 1.1, moved to where 1.1 keeps it.
//!
//! Before 1.1 the format had no field for a height, an eye colour or a
//! diagnosis, so this application wrote them into two extension objects of its
//! own (see [`crate::physical`]). 1.1 gives almost all of them first-class
//! attributes, in the same claim shape — value, date, source, confidence,
//! note — so moving one is a change of address, not of meaning.
//!
//! # Read lifted, write lifted, lose nothing
//!
//! [`lift`] is a pure function from a stored person to the same person with
//! every mappable legacy entry appended to its 1.1 attribute and removed from
//! the extension. The record page draws the lifted person, so an old bundle
//! shows its heights under Morphology the day this version is installed. The
//! profile editor starts from the lifted person, so the first save through it
//! writes the move into the bundle — through `update_entity`, like every other
//! write — and a class the editor may not read is put back exactly as stored
//! by the restore that follows, lift or no lift.
//!
//! What has no 1.1 home stays where it is and is still shown and editable:
//!
//! * a free-text religion, because `belief.religions` requires a tradition
//!   from a closed list and choosing one for somebody would invent a fact;
//! * free-text military service and languages, which 1.1 records as
//!   structured claims this text cannot be parsed into;
//! * health notes, which are not conditions;
//! * any single entry whose value does not map: an eye colour outside the
//!   vocabulary, a blood group recorded as unknown, a second blood group when
//!   the attribute holds one claim.

use serde_json::{Map, Value};

use crate::physical::{HEALTH_KEY, TRAITS_KEY};

/// How a legacy value becomes a 1.1 value.
#[derive(Debug, Clone, Copy)]
enum Conversion {
    /// A number, as is.
    Number,
    /// A vocabulary term, renamed where the vocabularies differ.
    Term(&'static [(&'static str, &'static str)]),
    /// Free text, as is.
    Text,
    /// Free text into one field of an object value.
    Into(&'static str),
}

/// One legacy field with a 1.1 home.
struct Route {
    extension: &'static str,
    field: &'static str,
    path: &'static str,
    map: Conversion,
}

const ROUTES: &[Route] = &[
    Route {
        extension: TRAITS_KEY,
        field: "height_cm",
        path: "morphology.height",
        map: Conversion::Number,
    },
    Route {
        extension: TRAITS_KEY,
        field: "weight_kg",
        path: "morphology.weight",
        map: Conversion::Number,
    },
    Route {
        extension: TRAITS_KEY,
        field: "eye_colour",
        path: "morphology.eye_colour",
        map: Conversion::Term(&[]),
    },
    // The earlier list said "fair"; 1.1 files fair under blond.
    Route {
        extension: TRAITS_KEY,
        field: "hair_colour",
        path: "morphology.hair_colour",
        map: Conversion::Term(&[("fair", "blond")]),
    },
    Route {
        extension: TRAITS_KEY,
        field: "build",
        path: "morphology.build",
        map: Conversion::Term(&[]),
    },
    Route {
        extension: TRAITS_KEY,
        field: "handedness",
        path: "biometrics.handedness",
        map: Conversion::Term(&[]),
    },
    Route {
        extension: TRAITS_KEY,
        field: "features",
        path: "morphology.distinguishing_features",
        map: Conversion::Text,
    },
    Route {
        extension: HEALTH_KEY,
        field: "conditions",
        path: "health.conditions",
        map: Conversion::Into("description"),
    },
    Route {
        extension: HEALTH_KEY,
        field: "operations",
        path: "health.surgeries",
        map: Conversion::Into("description"),
    },
    Route {
        extension: HEALTH_KEY,
        field: "cause_of_death",
        path: "death.causes",
        map: Conversion::Into("description"),
    },
];

/// The group a legacy field with no 1.1 home is shown in.
pub fn home_group(field: &str) -> &'static str {
    match field {
        "military" => "military",
        "languages" => "residence",
        "religion" => "belief",
        "blood_group" | "health_notes" | "conditions" | "operations" => "health",
        "cause_of_death" => "death",
        "handedness" => "biometrics",
        _ => "morphology",
    }
}

/// The claim-level keys a legacy entry and a 1.1 claim share.
const PROVENANCE: &[&str] = &["date", "source_id", "confidence", "note"];

fn claim_from(entry: &Value, value: Value) -> Value {
    let mut claim = Map::new();
    claim.insert("value".into(), value);
    for k in PROVENANCE {
        if let Some(v) = entry.get(*k).filter(|v| !v.is_null()) {
            claim.insert((*k).to_string(), v.clone());
        }
    }
    Value::Object(claim)
}

fn mapped_value(map: Conversion, attribute: &str, raw: &Value) -> Option<Value> {
    let a = axgf_rs::model::profile::registry::attribute(attribute)?;
    match map {
        Conversion::Number => {
            let n = match raw {
                Value::Number(n) => n.clone(),
                Value::String(s) => s.trim().parse::<serde_json::Number>().ok()?,
                _ => return None,
            };
            Some(Value::Number(n))
        }
        Conversion::Term(renames) => {
            let s = raw.as_str()?.trim();
            let term = renames
                .iter()
                .find(|(from, _)| *from == s)
                .map(|(_, to)| *to)
                .unwrap_or(s);
            match a.shape {
                axgf_rs::model::profile::registry::Shape::Vocab(v) if v.contains(term) => {
                    Some(Value::String(term.to_string()))
                }
                _ => None,
            }
        }
        Conversion::Text => raw
            .as_str()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|s| Value::String(s.to_string())),
        Conversion::Into(field) => raw
            .as_str()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|s| {
                let mut m = Map::new();
                m.insert(field.to_string(), Value::String(s.to_string()));
                Value::Object(m)
            }),
    }
}

/// Split a legacy blood group — `a-pos` — into the two 1.1 claims it is.
fn blood_group(raw: &Value) -> Option<(&'static str, &'static str)> {
    let s = raw.as_str()?.trim();
    let (abo, rh) = s.rsplit_once('-')?;
    let abo = match abo {
        "a" => "A",
        "b" => "B",
        "ab" => "AB",
        "o" => "O",
        _ => return None,
    };
    let rh = match rh {
        "pos" => "positive",
        "neg" => "negative",
        _ => return None,
    };
    Some((abo, rh))
}

/// The person with every mappable legacy entry moved to its 1.1 attribute.
///
/// A person with no legacy extension is returned unchanged.
pub fn lift(person: &Value) -> Value {
    let mut out = person.clone();
    let has_legacy = [TRAITS_KEY, HEALTH_KEY].iter().any(|k| {
        person
            .pointer(&format!("/extensions/{}", escape(k)))
            .is_some()
    });
    if !has_legacy {
        return out;
    }

    for route in ROUTES {
        let entries = take_entries(&mut out, route.extension, route.field);
        let mut kept = Vec::new();
        for entry in entries {
            let raw = entry.get("value").cloned().unwrap_or(Value::Null);
            let pushed = match mapped_value(route.map, route.path, &raw) {
                Some(value) => push_claim(&mut out, route.path, claim_from(&entry, value)),
                None => false,
            };
            if !pushed {
                kept.push(entry);
            }
        }
        put_entries(&mut out, route.extension, route.field, kept);
    }

    // The blood group is one legacy field and two 1.1 attributes.
    let entries = take_entries(&mut out, HEALTH_KEY, "blood_group");
    let mut kept = Vec::new();
    for entry in entries {
        let raw = entry.get("value").cloned().unwrap_or(Value::Null);
        let lifted = blood_group(&raw).filter(|_| {
            out.pointer("/health/blood_group").is_none() && out.pointer("/health/rhesus").is_none()
        });
        match lifted {
            Some((abo, rh)) => {
                push_claim(
                    &mut out,
                    "health.blood_group",
                    claim_from(&entry, Value::from(abo)),
                );
                push_claim(
                    &mut out,
                    "health.rhesus",
                    claim_from(&entry, Value::from(rh)),
                );
            }
            None => kept.push(entry),
        }
    }
    put_entries(&mut out, HEALTH_KEY, "blood_group", kept);
    tidy_extensions(&mut out);
    out
}

/// A legacy field that stays in its extension, as the profile shows it.
#[derive(Debug, Clone)]
pub struct Leftover {
    /// The extension key it lives under.
    pub extension: &'static str,
    /// Its name inside that extension.
    pub field: String,
    /// The group it is shown in.
    pub group: &'static str,
    /// The entries, in the claim shape both versions share.
    pub entries: Vec<Value>,
}

/// Every legacy field still in a (lifted) person's extensions.
pub fn leftovers(person: &Value) -> Vec<Leftover> {
    let mut out = Vec::new();
    for extension in [TRAITS_KEY, HEALTH_KEY] {
        let Some(obj) = person
            .get("extensions")
            .and_then(|e| e.get(extension))
            .and_then(Value::as_object)
        else {
            continue;
        };
        for (field, entries) in obj {
            let Some(list) = entries.as_array().filter(|l| !l.is_empty()) else {
                continue;
            };
            out.push(Leftover {
                extension,
                field: field.clone(),
                group: home_group(field),
                entries: list.clone(),
            });
        }
    }
    out
}

/// JSON-pointer escaping for one key.
fn escape(key: &str) -> String {
    key.replace('~', "~0").replace('/', "~1")
}

fn take_entries(person: &mut Value, extension: &str, field: &str) -> Vec<Value> {
    person
        .get_mut("extensions")
        .and_then(|e| e.get_mut(extension))
        .and_then(Value::as_object_mut)
        .and_then(|o| o.remove(field))
        .and_then(|v| match v {
            Value::Array(a) => Some(a),
            _ => None,
        })
        .unwrap_or_default()
}

fn put_entries(person: &mut Value, extension: &str, field: &str, kept: Vec<Value>) {
    if kept.is_empty() {
        return;
    }
    if let Some(obj) = person
        .get_mut("extensions")
        .and_then(|e| e.get_mut(extension))
        .and_then(Value::as_object_mut)
    {
        obj.insert(field.to_string(), Value::Array(kept));
    }
}

/// Append a claim to a 1.1 attribute. A single-claim attribute that already
/// holds one is not overwritten, and the caller keeps the legacy entry.
fn push_claim(person: &mut Value, path: &str, claim: Value) -> bool {
    let Some(a) = axgf_rs::model::profile::registry::attribute(path) else {
        return false;
    };
    let Some(obj) = person.as_object_mut() else {
        return false;
    };
    let block = obj
        .entry(a.block.to_string())
        .or_insert_with(|| Value::Object(Map::new()));
    let Some(block) = block.as_object_mut() else {
        return false;
    };
    match a.cardinality {
        axgf_rs::model::profile::registry::Cardinality::Single => {
            if block.contains_key(a.key) {
                return false;
            }
            block.insert(a.key.to_string(), claim);
        }
        axgf_rs::model::profile::registry::Cardinality::Series => {
            match block
                .entry(a.key.to_string())
                .or_insert_with(|| Value::Array(Vec::new()))
            {
                Value::Array(list) => list.push(claim),
                _ => return false,
            }
        }
    }
    true
}

/// Remove legacy extension objects the lift emptied, and `extensions` itself
/// when nothing is left in it.
fn tidy_extensions(person: &mut Value) {
    let Some(obj) = person.as_object_mut() else {
        return;
    };
    let empty = match obj.get_mut("extensions").and_then(Value::as_object_mut) {
        Some(ext) => {
            for key in [TRAITS_KEY, HEALTH_KEY] {
                if ext
                    .get(key)
                    .and_then(Value::as_object)
                    .is_some_and(|o| o.is_empty())
                {
                    ext.remove(key);
                }
            }
            ext.is_empty()
        }
        None => false,
    };
    if empty {
        obj.remove("extensions");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn legacy() -> Value {
        json!({
            "id": "p", "type": "person",
            "identity": {"name": {"display": "N"}, "is_living": false},
            "extensions": {
                TRAITS_KEY: {
                    "height_cm": [
                        {"value": 172, "date": {"value": "1914", "precision": "year"}, "source_id": "s1", "confidence": 0.9},
                        {"value": 169, "date": {"value": "1950", "precision": "year"}}
                    ],
                    "eye_colour": [{"value": "blue"}, {"value": "puce"}],
                    "hair_colour": [{"value": "fair", "note": "as a child"}],
                    "military": [{"value": "1st Uhlans, 1939"}],
                    "languages": [{"value": "Polish, Yiddish"}]
                },
                HEALTH_KEY: {
                    "blood_group": [{"value": "ab-neg"}, {"value": "o-pos"}],
                    "conditions": [{"value": "Sarcoidosis", "confidence": 0.6}],
                    "religion": [{"value": "Old Believer"}],
                    "cause_of_death": [{"value": "Typhus"}]
                },
                "axgf-cms:avatar/v1": {"document_id": "d"}
            }
        })
    }

    #[test]
    fn a_mapped_entry_moves_with_its_provenance() {
        let p = lift(&legacy());
        let heights = p["morphology"]["height"].as_array().expect("a series");
        assert_eq!(heights.len(), 2);
        assert_eq!(heights[0]["value"], 172);
        assert_eq!(heights[0]["date"]["value"], "1914");
        assert_eq!(heights[0]["source_id"], "s1");
        assert_eq!(heights[0]["confidence"], 0.9);
        assert_eq!(
            p["morphology"]["hair_colour"][0]["value"], "blond",
            "fair is blond in 1.1"
        );
        assert_eq!(p["morphology"]["hair_colour"][0]["note"], "as a child");
        assert_eq!(
            p["health"]["conditions"][0]["value"]["description"],
            "Sarcoidosis"
        );
        assert_eq!(p["death"]["causes"][0]["value"]["description"], "Typhus");
    }

    #[test]
    fn a_blood_group_becomes_two_claims_and_a_second_one_stays() {
        let p = lift(&legacy());
        assert_eq!(p["health"]["blood_group"]["value"], "AB");
        assert_eq!(p["health"]["rhesus"]["value"], "negative");
        let left = &p["extensions"][HEALTH_KEY]["blood_group"];
        assert_eq!(
            left.as_array().map(Vec::len),
            Some(1),
            "the single claim is not overwritten"
        );
        assert_eq!(left[0]["value"], "o-pos");
    }

    #[test]
    fn what_has_no_home_stays_where_it_was() {
        let p = lift(&legacy());
        let traits = &p["extensions"][TRAITS_KEY];
        assert_eq!(traits["eye_colour"][0]["value"], "puce", "not a term: kept");
        assert!(
            traits.get("height_cm").is_none(),
            "moved: gone from the extension"
        );
        assert_eq!(traits["military"][0]["value"], "1st Uhlans, 1939");
        assert_eq!(
            p["extensions"][HEALTH_KEY]["religion"][0]["value"],
            "Old Believer"
        );
        assert_eq!(p["extensions"]["axgf-cms:avatar/v1"]["document_id"], "d");
        let groups: Vec<&str> = leftovers(&p).iter().map(|l| l.group).collect();
        assert!(
            groups.contains(&"military")
                && groups.contains(&"residence")
                && groups.contains(&"belief")
        );
    }

    #[test]
    fn a_fully_lifted_extension_is_removed_and_a_plain_person_is_untouched() {
        let p = json!({"identity": {}, "extensions": {TRAITS_KEY: {"build": [{"value": "slim"}]}}});
        let lifted = lift(&p);
        assert_eq!(lifted["morphology"]["build"][0]["value"], "slim");
        assert!(lifted.get("extensions").is_none(), "{lifted}");
        let plain = json!({"identity": {}, "morphology": {"height": [{"value": 150}]}});
        assert_eq!(lift(&plain), plain);
    }

    #[test]
    fn lifting_twice_changes_nothing_the_second_time() {
        let once = lift(&legacy());
        assert_eq!(lift(&once), once);
    }
}
