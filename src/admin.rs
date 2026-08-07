//! Admin-side form model.
//!
//! # Why a field table rather than eight HTML files
//!
//! The fields below are chosen by hand, per entity kind, for what someone
//! realistically edits. They are **not** derived from the JSON Schema — no
//! introspection, no `$ref` walking, no generated widgets. The table is
//! explicit source you can read and diff.
//!
//! It is rendered by one loop instead of being copied into eight near-identical
//! HTML blocks, which would be several hundred lines of duplicated markup with
//! nothing to stop them drifting apart.
//!
//! Anything not in the table stays editable through the raw JSON textarea that
//! every form carries, so no part of an entity is ever unreachable.

use axgf_rs::EntityKind;
use serde::Serialize;
use serde_json::{Map, Value};

/// How a field is presented and parsed.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FieldKind {
    Text,
    LongText,
    /// A 0.0–1.0 confidence, rendered as a slider.
    Confidence,
    Bool,
    Select,
}

/// One editable field.
#[derive(Debug, Clone, Serialize)]
pub struct Field {
    /// Form input name, and the dotted path into the entity JSON.
    pub path: &'static str,
    pub label: &'static str,
    pub kind: FieldKind,
    pub hint: &'static str,
    pub options: &'static [&'static str],
}

const NO_OPTS: &[&str] = &[];

const PRECISION: &[&str] = &[
    "",
    "exact",
    "month",
    "year",
    "decade",
    "quarter_century",
    "century",
    "unknown",
];

const fn f(path: &'static str, label: &'static str, kind: FieldKind) -> Field {
    Field {
        path,
        label,
        kind,
        hint: "",
        options: NO_OPTS,
    }
}

const fn fh(path: &'static str, label: &'static str, kind: FieldKind, hint: &'static str) -> Field {
    Field {
        path,
        label,
        kind,
        hint,
        options: NO_OPTS,
    }
}

const fn fs(
    path: &'static str,
    label: &'static str,
    options: &'static [&'static str],
    hint: &'static str,
) -> Field {
    Field {
        path,
        label,
        kind: FieldKind::Select,
        hint,
        options,
    }
}

/// Fields shown for a person.
const PERSON_FIELDS: &[Field] = &[
    fh(
        "identity.name.display",
        "Display name",
        FieldKind::Text,
        "The name shown everywhere on the site.",
    ),
    fs(
        "identity.gender.value",
        "Gender",
        &["", "M", "F", "NB", "U"],
        "",
    ),
    f("identity.is_living", "Living", FieldKind::Bool),
    fh(
        "birth.date.value",
        "Birth date",
        FieldKind::Text,
        "ISO-ish: 1923, 1923-04 or 1923-04-12. Leave blank if unknown.",
    ),
    fs(
        "birth.date.precision",
        "Birth precision",
        PRECISION,
        "How precisely the source pins this down.",
    ),
    fh(
        "birth.date.circa",
        "Birth is approximate",
        FieldKind::Bool,
        "Renders as “circa 1923” rather than an exact claim.",
    ),
    f("birth.place_id", "Birth place id", FieldKind::Text),
    fh(
        "birth.confidence",
        "Birth confidence",
        FieldKind::Confidence,
        "How sure you are. This is what the site renders visually.",
    ),
    fh("death.date.value", "Death date", FieldKind::Text, ""),
    fs("death.date.precision", "Death precision", PRECISION, ""),
    f("death.date.circa", "Death is approximate", FieldKind::Bool),
    f("death.place_id", "Death place id", FieldKind::Text),
    f(
        "death.confidence",
        "Death confidence",
        FieldKind::Confidence,
    ),
    f("death.cause", "Cause of death", FieldKind::Text),
    f("bio", "Biography", FieldKind::LongText),
    f("notes", "Notes", FieldKind::LongText),
];

/// Fields shown for a family.
const FAMILY_FIELDS: &[Field] = &[
    f("name", "Family name", FieldKind::Text),
    f("description", "Description", FieldKind::LongText),
    fs(
        "union.type",
        "Union type",
        &[
            "",
            "marriage",
            "civil_union",
            "cohabitation",
            "religious_only",
            "polygamous",
            "unknown",
        ],
        "",
    ),
    fs(
        "union.status",
        "Union status",
        &[
            "",
            "active",
            "ended_by_death",
            "ended_by_divorce",
            "ended_by_separation",
            "annulled",
            "unknown",
        ],
        "",
    ),
    fh(
        "union.confidence",
        "Union confidence",
        FieldKind::Confidence,
        "Drives the opacity of the spouse connector on the tree.",
    ),
    fh("union.start.date.value", "Union start", FieldKind::Text, ""),
    fh("union.end.date.value", "Union end", FieldKind::Text, ""),
    fh(
        "notes",
        "Notes",
        FieldKind::LongText,
        "Partners and children are lists — edit them in the raw JSON below.",
    ),
];

/// Fields shown for a event.
const EVENT_FIELDS: &[Field] = &[
    fs(
        "category",
        "Category",
        &[
            "",
            "birth",
            "death",
            "marriage",
            "divorce",
            "adoption",
            "migration",
            "naturalization",
            "military",
            "incarceration",
            "name_change",
            "census",
            "legal",
            "religious",
            "social",
            "historical",
            "other",
        ],
        "Required.",
    ),
    f("subcategory", "Subcategory", FieldKind::Text),
    fh(
        "date.value",
        "Date",
        FieldKind::Text,
        "Required by the schema.",
    ),
    fs("date.precision", "Precision", PRECISION, ""),
    f("date.circa", "Approximate", FieldKind::Bool),
    f("place_id", "Place id", FieldKind::Text),
    f("description", "Description", FieldKind::LongText),
    f("confidence", "Confidence", FieldKind::Confidence),
    f("source_id", "Source id", FieldKind::Text),
];

/// Fields shown for a link.
const LINK_FIELDS: &[Field] = &[
    fs(
        "from.entity_type",
        "From type",
        &["person", "family", "event"],
        "",
    ),
    fh("from.entity_id", "From id", FieldKind::Text, "Required."),
    fs(
        "to.entity_type",
        "To type",
        &["person", "family", "event"],
        "",
    ),
    fh("to.entity_id", "To id", FieldKind::Text, "Required."),
    fh(
        "label",
        "Label",
        FieldKind::Text,
        "Reads forward: “godfather”, “employer”, “witness”. Required.",
    ),
    fh(
        "label_reverse",
        "Reverse label",
        FieldKind::Text,
        "How it reads from the other end: “godson”, “employee”.",
    ),
    fs(
        "category",
        "Category",
        &[
            "",
            "spiritual",
            "professional",
            "social",
            "legal",
            "medical",
            "educational",
            "conflict",
            "other",
        ],
        "",
    ),
    f("bidirectional", "Bidirectional", FieldKind::Bool),
    fh(
        "valid_from.date.value",
        "Valid from",
        FieldKind::Text,
        "When the relationship started.",
    ),
    f("valid_until.date.value", "Valid until", FieldKind::Text),
    fh(
        "confidence",
        "Confidence",
        FieldKind::Confidence,
        "“85% confident, per a family letter” — the thing GEDCOM cannot say.",
    ),
    f("source_id", "Source id", FieldKind::Text),
    f("note", "Note", FieldKind::LongText),
];

/// Fields shown for a occupation.
const OCCUPATION_FIELDS: &[Field] = &[
    fh("person_id", "Person id", FieldKind::Text, "Required."),
    fh(
        "title",
        "Title",
        FieldKind::Text,
        "Required. e.g. Schoolteacher.",
    ),
    f("title_latin", "Title (Latin script)", FieldKind::Text),
    f("employer.name", "Employer", FieldKind::Text),
    f("place_id", "Place id", FieldKind::Text),
    fh(
        "valid_from.date.value",
        "From",
        FieldKind::Text,
        "An occupation is a span. Giving both bounds is what makes it a bar.",
    ),
    f("valid_until.date.value", "Until", FieldKind::Text),
    f("confidence", "Confidence", FieldKind::Confidence),
    f("source_id", "Source id", FieldKind::Text),
    f("note", "Note", FieldKind::LongText),
];

/// Fields shown for a source.
const SOURCE_FIELDS: &[Field] = &[
    fh("title", "Title", FieldKind::Text, "Required."),
    fs(
        "source_type",
        "Type",
        &[
            "birth_certificate",
            "death_certificate",
            "marriage_certificate",
            "census",
            "baptism_record",
            "burial_record",
            "will",
            "land_record",
            "military_record",
            "immigration_record",
            "naturalization",
            "passport",
            "photograph",
            "letter",
            "diary",
            "newspaper",
            "oral_tradition",
            "dna",
            "family_bible",
            "gravestone",
            "published_genealogy",
            "other",
        ],
        "Required.",
    ),
    fs(
        "reliability",
        "Reliability",
        &[
            "primary",
            "secondary",
            "derivative",
            "authored",
            "oral",
            "unknown",
        ],
        "Required. Shown as a badge next to every fact resting on this source.",
    ),
    fs(
        "status",
        "Status",
        &["", "verified", "unverified", "lost", "known_missing"],
        "",
    ),
    f("confidence", "Confidence", FieldKind::Confidence),
    f("repository.name", "Repository", FieldKind::Text),
    f(
        "repository.reference",
        "Repository reference",
        FieldKind::Text,
    ),
    f("transcription", "Transcription", FieldKind::LongText),
    f("note", "Note", FieldKind::LongText),
];

/// Fields shown for a place.
const PLACE_FIELDS: &[Field] = &[
    fh(
        "names.0.value",
        "Primary name",
        FieldKind::Text,
        "Required.",
    ),
    fh(
        "names.0.lang",
        "Language",
        FieldKind::Text,
        "BCP 47, e.g. en, fr, pl.",
    ),
    fs(
        "place_type",
        "Type",
        &[
            "",
            "continent",
            "country",
            "region",
            "department",
            "city",
            "village",
            "district",
            "street",
            "building",
            "farm",
            "island",
            "historical",
            "unknown",
        ],
        "",
    ),
    f("region", "Region", FieldKind::Text),
    fh(
        "country_current",
        "Country today",
        FieldKind::Text,
        "Border history is a list — edit it in the raw JSON below.",
    ),
    f("note", "Note", FieldKind::LongText),
];

/// Fields shown for a document.
const DOCUMENT_FIELDS: &[Field] = &[
    fh("filename", "Filename", FieldKind::Text, "Required."),
    fh(
        "mime_type",
        "MIME type",
        FieldKind::Text,
        "Required, e.g. image/jpeg.",
    ),
    fs(
        "document_type",
        "Type",
        &[
            "photo",
            "birth_certificate",
            "death_certificate",
            "marriage_certificate",
            "census_page",
            "baptism_record",
            "military_record",
            "will",
            "land_record",
            "letter",
            "diary",
            "newspaper_clipping",
            "gravestone_photo",
            "family_tree_drawing",
            "audio",
            "video",
            "other",
        ],
        "Required.",
    ),
    fs(
        "status",
        "Status",
        &["present", "referenced", "known_missing", "lost", "unknown"],
        "Required.",
    ),
    f("url", "URL", FieldKind::Text),
    f("caption", "Caption", FieldKind::Text),
    f("note", "Note", FieldKind::LongText),
];

/// The fields shown for each entity kind.
pub fn fields_for(kind: EntityKind) -> &'static [Field] {
    match kind {
        EntityKind::Person => PERSON_FIELDS,
        EntityKind::Family => FAMILY_FIELDS,
        EntityKind::Event => EVENT_FIELDS,
        EntityKind::Link => LINK_FIELDS,
        EntityKind::Occupation => OCCUPATION_FIELDS,
        EntityKind::Source => SOURCE_FIELDS,
        EntityKind::Place => PLACE_FIELDS,
        EntityKind::Document => DOCUMENT_FIELDS,
    }
}

/// Parse the `:kind` path segment.
pub fn kind_from_str(s: &str) -> Option<EntityKind> {
    Some(match s {
        "person" => EntityKind::Person,
        "family" => EntityKind::Family,
        "event" => EntityKind::Event,
        "link" => EntityKind::Link,
        "occupation" => EntityKind::Occupation,
        "source" => EntityKind::Source,
        "place" => EntityKind::Place,
        "document" => EntityKind::Document,
        _ => return None,
    })
}

/// Every kind, for navigation.
pub const KINDS: [&str; 8] = [
    "person",
    "family",
    "event",
    "link",
    "occupation",
    "source",
    "place",
    "document",
];

/// Parse the delete policy chosen on the form. Reject is the default because
/// it is the only one that cannot lose data.
pub fn policy_from_str(s: &str) -> axgf_rs::DeletePolicy {
    match s {
        "cascade" => axgf_rs::DeletePolicy::Cascade,
        "orphan" => axgf_rs::DeletePolicy::Orphan,
        _ => axgf_rs::DeletePolicy::Reject,
    }
}

/// Read a dotted path out of an entity, as a string for form pre-fill.
pub fn get_path(entity: &Value, path: &str) -> String {
    let mut cur = entity;
    for seg in path.split('.') {
        cur = match seg.parse::<usize>() {
            Ok(i) => match cur.get(i) {
                Some(v) => v,
                None => return String::new(),
            },
            Err(_) => match cur.get(seg) {
                Some(v) => v,
                None => return String::new(),
            },
        };
    }
    match cur {
        Value::String(s) => s.clone(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        Value::Null => String::new(),
        other => other.to_string(),
    }
}

/// Write a dotted path into an entity, creating containers as needed.
///
/// A `None` value removes the key instead of writing a null or an empty
/// string: an absent field and a field explicitly set to "" are different
/// things to a JSON Schema, and the empty one is usually invalid.
pub fn set_path(entity: &mut Value, path: &str, value: Option<Value>) {
    let segs: Vec<&str> = path.split('.').collect();
    set_path_inner(entity, &segs, value);
}

fn set_path_inner(cur: &mut Value, segs: &[&str], value: Option<Value>) {
    let Some((head, rest)) = segs.split_first() else {
        return;
    };

    if let Ok(idx) = head.parse::<usize>() {
        if !cur.is_array() {
            if value.is_none() {
                return;
            }
            *cur = Value::Array(Vec::new());
        }
        let arr = cur.as_array_mut().expect("just ensured array");
        if rest.is_empty() {
            match value {
                Some(v) => {
                    while arr.len() <= idx {
                        arr.push(Value::Null);
                    }
                    arr[idx] = v;
                }
                None => {
                    if idx < arr.len() {
                        arr.remove(idx);
                    }
                }
            }
            return;
        }
        if arr.len() <= idx {
            if value.is_none() {
                return;
            }
            while arr.len() <= idx {
                arr.push(Value::Object(Map::new()));
            }
        }
        set_path_inner(&mut arr[idx], rest, value);
        return;
    }

    if !cur.is_object() {
        if value.is_none() {
            return;
        }
        *cur = Value::Object(Map::new());
    }
    let obj = cur.as_object_mut().expect("just ensured object");

    if rest.is_empty() {
        match value {
            Some(v) => {
                obj.insert((*head).to_string(), v);
            }
            None => {
                obj.remove(*head);
            }
        }
        return;
    }

    if !obj.contains_key(*head) {
        if value.is_none() {
            return;
        }
        obj.insert((*head).to_string(), Value::Object(Map::new()));
    }
    let removing = value.is_none();
    let child = obj.get_mut(*head).expect("just ensured present");
    set_path_inner(child, rest, value);

    // Drop containers the removal emptied, so a cleared date does not leave
    // `{"date": {}}` behind to fail validation.
    if removing {
        let empty = match obj.get(*head) {
            Some(Value::Object(m)) => m.is_empty(),
            _ => false,
        };
        if empty {
            obj.remove(*head);
        }
    }
}

/// Apply submitted form values onto a base entity.
///
/// `base` is the raw-JSON textarea when the user supplied one, otherwise the
/// entity being edited (or `{}` for a create). The typed fields are then
/// written over it, so a field covered by the form always reflects what the
/// form shows.
pub fn apply_form(
    base: Value,
    kind: EntityKind,
    form: &std::collections::HashMap<String, String>,
) -> Value {
    let mut entity = if base.is_object() {
        base
    } else {
        Value::Object(Map::new())
    };

    for field in fields_for(kind) {
        let raw = form.get(field.path).map(String::as_str).unwrap_or("");
        let trimmed = raw.trim();

        let value = match field.kind {
            // An unchecked checkbox is simply absent from the submission.
            FieldKind::Bool => {
                let on = matches!(trimmed, "on" | "true" | "1" | "yes");
                if on {
                    Some(Value::Bool(true))
                } else {
                    None
                }
            }
            FieldKind::Confidence => match trimmed.parse::<f64>() {
                Ok(v) if !trimmed.is_empty() => {
                    serde_json::Number::from_f64(v.clamp(0.0, 1.0)).map(Value::Number)
                }
                _ => None,
            },
            _ => {
                if trimmed.is_empty() {
                    None
                } else {
                    Some(Value::String(trimmed.to_string()))
                }
            }
        };

        set_path(&mut entity, field.path, value);
    }

    ensure_required(kind, &mut entity);
    entity
}

/// Fill in the schema-required pieces a form cannot sensibly ask for.
///
/// The schema requires `identity.gender`, `identity.is_living` and
/// `name.components` on every person. Leaving them out produces an entity the
/// library accepts — validation is non-blocking — but flags with
/// `SCHEMA_VALIDATION_FAILED` warnings on every subsequent validate. Filling
/// them with honest defaults ("U" for unrecorded gender, an empty component
/// list) means the admin panel creates valid entities rather than immediately
/// dirtying the bundle.
///
/// Only structural requirements are filled. Nothing here invents a fact: an
/// unrecorded gender becomes the schema's own "unknown", not a guess.
fn ensure_required(kind: EntityKind, entity: &mut Value) {
    match kind {
        EntityKind::Person => {
            // Only complete an identity that exists; conjuring one for an
            // empty submission would hide the real problem.
            if entity.get("identity").map(Value::is_object) != Some(true) {
                return;
            }
            if get_path(entity, "identity.name.display").is_empty() {
                return;
            }
            if entity
                .pointer("/identity/name/components")
                .map(Value::is_array)
                != Some(true)
            {
                set_path(
                    entity,
                    "identity.name.components",
                    Some(Value::Array(vec![])),
                );
            }
            if get_path(entity, "identity.gender.value").is_empty() {
                set_path(
                    entity,
                    "identity.gender.value",
                    Some(Value::String("U".into())),
                );
            }
            if entity.pointer("/identity/is_living").map(Value::is_boolean) != Some(true) {
                set_path(entity, "identity.is_living", Some(Value::Bool(false)));
            }
        }
        // A place name needs a language tag alongside its value.
        EntityKind::Place
            if !get_path(entity, "names.0.value").is_empty()
                && get_path(entity, "names.0.lang").is_empty() =>
        {
            set_path(entity, "names.0.lang", Some(Value::String("en".into())));
        }
        _ => {}
    }
}

/// One page of a listing.
pub struct Page<T> {
    pub items: Vec<T>,
    pub page: usize,
    pub pages: usize,
    pub total: usize,
    pub per_page: usize,
}

/// Entities per admin listing page.
pub const PER_PAGE: usize = 50;

/// Slice `items` into the requested page, clamping out-of-range requests.
pub fn paginate<T>(items: Vec<T>, page: usize) -> Page<T> {
    let total = items.len();
    let pages = total.div_ceil(PER_PAGE).max(1);
    let page = page.clamp(1, pages);
    let start = (page - 1) * PER_PAGE;
    let items = items.into_iter().skip(start).take(PER_PAGE).collect();
    Page {
        items,
        page,
        pages,
        total,
        per_page: PER_PAGE,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::collections::HashMap;

    fn form(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs
            .iter()
            .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
            .collect()
    }

    #[test]
    fn every_kind_parses_and_has_fields() {
        for k in KINDS {
            let kind = kind_from_str(k).unwrap_or_else(|| panic!("{k} should parse"));
            assert!(!fields_for(kind).is_empty(), "{k} has no fields");
        }
        assert!(kind_from_str("nonsense").is_none());
        assert!(kind_from_str("persons").is_none(), "plural is not a kind");
    }

    #[test]
    fn set_path_creates_nested_objects() {
        let mut v = json!({});
        set_path(&mut v, "identity.name.display", Some(json!("Ada")));
        assert_eq!(v, json!({"identity": {"name": {"display": "Ada"}}}));
    }

    #[test]
    fn set_path_with_none_prunes_every_container_it_empties() {
        // Clearing the only value leaves no empty husks: `{"date": {}}` would
        // fail schema validation, and an empty `birth` asserts nothing either.
        let mut v = json!({"birth": {"date": {"value": "1923"}}});
        set_path(&mut v, "birth.date.value", None);
        assert_eq!(v, json!({}), "got {v}");

        // A sibling with content stops the cascade at the right level.
        let mut v = json!({"birth": {"date": {"value": "1923"}, "confidence": 0.9}});
        set_path(&mut v, "birth.date.value", None);
        assert_eq!(v, json!({"birth": {"confidence": 0.9}}), "got {v}");
    }

    #[test]
    fn set_path_handles_array_indices() {
        let mut v = json!({});
        set_path(&mut v, "names.0.value", Some(json!("Warsaw")));
        set_path(&mut v, "names.0.lang", Some(json!("en")));
        assert_eq!(v, json!({"names": [{"value": "Warsaw", "lang": "en"}]}));
    }

    #[test]
    fn get_path_reads_back_what_set_path_wrote() {
        let mut v = json!({});
        set_path(&mut v, "a.b.c", Some(json!("x")));
        assert_eq!(get_path(&v, "a.b.c"), "x");
        assert_eq!(get_path(&v, "a.b.missing"), "");
        assert_eq!(get_path(&v, "nope.nope"), "");
    }

    #[test]
    fn get_path_stringifies_scalars_for_prefill() {
        let v = json!({"n": 0.85, "b": true, "s": "t", "z": null});
        assert_eq!(get_path(&v, "n"), "0.85");
        assert_eq!(get_path(&v, "b"), "true");
        assert_eq!(get_path(&v, "s"), "t");
        assert_eq!(get_path(&v, "z"), "");
    }

    #[test]
    fn empty_form_fields_are_omitted_not_written_as_empty_strings() {
        // A schema with minLength would reject "", and an absent field is what
        // "not recorded" actually means.
        let e = apply_form(
            json!({}),
            EntityKind::Person,
            &form(&[
                ("identity.name.display", "Ada Lovelace"),
                ("birth.date.value", "   "),
                ("bio", ""),
            ]),
        );
        assert_eq!(e["identity"]["name"]["display"], "Ada Lovelace");
        assert!(e.get("bio").is_none(), "empty bio must not be written");
        assert!(
            e.get("birth").is_none() || e["birth"].get("date").is_none(),
            "a blank date must not create an empty date object: {e}"
        );
    }

    #[test]
    fn an_unchecked_checkbox_clears_the_flag() {
        let base = json!({"identity": {"is_living": true, "name": {"display": "X"}}});
        let e = apply_form(
            base,
            EntityKind::Person,
            &form(&[("identity.name.display", "X")]),
        );
        // is_living is schema-required, so it becomes false rather than absent.
        assert_eq!(
            e["identity"]["is_living"],
            json!(false),
            "an unchecked box means false, got {e}"
        );
    }

    #[test]
    fn a_created_person_satisfies_the_schemas_required_fields() {
        // Without this the library accepts the entity but flags
        // SCHEMA_VALIDATION_FAILED warnings on every later validate.
        let e = apply_form(
            json!({}),
            EntityKind::Person,
            &form(&[("identity.name.display", "Ada Lovelace")]),
        );
        assert!(e["identity"]["name"]["components"].is_array());
        assert_eq!(e["identity"]["gender"]["value"], json!("U"));
        assert_eq!(e["identity"]["is_living"], json!(false));
    }

    #[test]
    fn an_empty_person_submission_is_not_papered_over() {
        // Nothing typed means nothing to complete; the library should get the
        // empty entity and say what is missing.
        let e = apply_form(json!({}), EntityKind::Person, &form(&[]));
        assert!(e.get("identity").is_none(), "got {e}");
    }

    #[test]
    fn a_place_name_gains_a_language_tag() {
        let e = apply_form(
            json!({}),
            EntityKind::Place,
            &form(&[("names.0.value", "Warszawa")]),
        );
        assert_eq!(e["names"][0]["lang"], json!("en"));
        // An explicit tag is respected.
        let e = apply_form(
            json!({}),
            EntityKind::Place,
            &form(&[("names.0.value", "Warszawa"), ("names.0.lang", "pl")]),
        );
        assert_eq!(e["names"][0]["lang"], json!("pl"));
    }

    #[test]
    fn a_checked_checkbox_sets_true() {
        let e = apply_form(
            json!({}),
            EntityKind::Person,
            &form(&[("identity.name.display", "X"), ("identity.is_living", "on")]),
        );
        assert_eq!(e["identity"]["is_living"], json!(true));
    }

    #[test]
    fn confidence_is_parsed_and_clamped() {
        let e = apply_form(
            json!({}),
            EntityKind::Person,
            &form(&[
                ("identity.name.display", "X"),
                ("birth.confidence", "0.35"),
                ("death.confidence", "5"),
            ]),
        );
        assert_eq!(e["birth"]["confidence"], json!(0.35));
        assert_eq!(e["death"]["confidence"], json!(1.0));
    }

    #[test]
    fn typed_fields_are_written_over_the_raw_json_base() {
        // The raw textarea supplies fields the form does not cover; the typed
        // fields own the paths they show.
        let base = json!({
            "identity": {"name": {"display": "From raw"}},
            "tags": ["kept-from-raw"]
        });
        let e = apply_form(
            base,
            EntityKind::Person,
            &form(&[("identity.name.display", "From the form")]),
        );
        assert_eq!(e["identity"]["name"]["display"], "From the form");
        assert_eq!(
            e["tags"],
            json!(["kept-from-raw"]),
            "uncovered fields survive"
        );
    }

    #[test]
    fn a_non_object_base_is_replaced_rather_than_crashing() {
        let e = apply_form(
            json!("not an object"),
            EntityKind::Person,
            &form(&[("identity.name.display", "X")]),
        );
        assert!(e.is_object());
        assert_eq!(e["identity"]["name"]["display"], "X");
    }

    #[test]
    fn delete_policy_defaults_to_reject() {
        assert_eq!(policy_from_str("cascade"), axgf_rs::DeletePolicy::Cascade);
        assert_eq!(policy_from_str("orphan"), axgf_rs::DeletePolicy::Orphan);
        assert_eq!(policy_from_str("reject"), axgf_rs::DeletePolicy::Reject);
        // Anything unexpected must fall back to the policy that cannot lose data.
        assert_eq!(policy_from_str(""), axgf_rs::DeletePolicy::Reject);
        assert_eq!(policy_from_str("garbage"), axgf_rs::DeletePolicy::Reject);
    }

    #[test]
    fn pagination_clamps_and_reports_totals() {
        let items: Vec<usize> = (0..120).collect();
        let p = paginate(items.clone(), 1);
        assert_eq!(p.items.len(), 50);
        assert_eq!((p.page, p.pages, p.total), (1, 3, 120));

        let p = paginate(items.clone(), 3);
        assert_eq!(p.items.len(), 20);
        assert_eq!(p.items[0], 100);

        // Out of range in both directions clamps into the valid range.
        assert_eq!(paginate(items.clone(), 99).page, 3);
        assert_eq!(paginate(items.clone(), 0).page, 1);

        let empty: Vec<usize> = Vec::new();
        let p = paginate(empty, 1);
        assert_eq!((p.page, p.pages, p.total), (1, 1, 0));
    }
}
