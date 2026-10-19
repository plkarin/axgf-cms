//! Portraits reworked by an AI: what a person might have looked like, as a
//! model redrew them from their photographs (v0.5.0, docs/adr/0009).
//!
//! # Never evidence
//!
//! An archive's pictures are evidence: a photograph says somebody stood
//! there, looking like that. A reworked portrait says nothing of the kind —
//! a model coloured it, sharpened it, aged it or imagined it. So the two are
//! kept apart wherever this application draws a person:
//!
//! * a reworked portrait is never the person's picture, chosen or guessed
//!   (`crate::avatar`), never in their gallery or their Media tab, and never
//!   a file the record cites;
//! * it is shown in a band of its own in the record's masthead, each picture
//!   marked "AI", and only to a reader who is signed in — the family's
//!   members, not the visitor who arrives from a search engine;
//! * its bytes are refused to a reader who is not signed in, at the one
//!   door every file goes through (`routes::public::stored_document`).
//!
//! # What is stored
//!
//! An ordinary AXGF Document, so the picture travels with the archive and any
//! reader of the format can open it, marked three ways that do not depend on
//! this application:
//!
//! * linked to the person with the role [`ROLE`] (1.0 §5.5; the role is a free
//!   string), not `subject`;
//! * tagged [`TAG`] (`tags`, every entity's, 1.0 base entity);
//! * and `extensions[`[`EXT_KEY`]`]`, namespaced and versioned like the
//!   avatar's choice, holding what 1.1 names for an artefact reference
//!   (1.1 §3.5.1): the `generator` that made it and the Document it was
//!   `derived_from_id`, both optional.
//!
//! `document_type` is `other`: 1.0's vocabulary has no word for a picture
//! nobody took, and `photo` would be the one claim this module exists not
//! to make.

use serde_json::{json, Value};

/// The role a reworked portrait is linked to its person with.
pub const ROLE: &str = "ai_portrait";

/// The tag every reworked portrait carries, for any reader of the format.
pub const TAG: &str = "ai-generated";

/// Namespaced key for what this application records about one.
pub const EXT_KEY: &str = "axgf-cms:ai-portrait/v1";

/// How many a person may have. Three: enough to compare, few enough that the
/// band beside the face stays a band.
pub const MAX: usize = 3;

/// Whether a stored Document is a reworked portrait.
///
/// Any one of the three marks is enough: a bundle edited by another tool may
/// have kept the role and dropped the extension, or the reverse, and a
/// portrait that loses one mark must not slip into a gallery as a photograph.
pub fn is_ai_portrait(doc: &Value) -> bool {
    doc.get("extensions").and_then(|e| e.get(EXT_KEY)).is_some()
        || doc
            .get("tags")
            .and_then(Value::as_array)
            .is_some_and(|t| t.iter().any(|x| x.as_str() == Some(TAG)))
        || doc
            .get("linked_to")
            .and_then(Value::as_array)
            .is_some_and(|l| {
                l.iter()
                    .any(|x| x.get("role").and_then(Value::as_str) == Some(ROLE))
            })
}

/// Whether the Document with this id, in this bundle, is a reworked portrait.
pub fn is_ai_portrait_id(flat: &Value, document_id: &str) -> bool {
    flat.get("documents")
        .and_then(|d| d.get(document_id))
        .is_some_and(is_ai_portrait)
}

/// The reworked portraits linked to one person, oldest first (then by id, so
/// two uploaded in the same second keep one order), as stored. Every one,
/// even past [`MAX`]: a bundle written elsewhere may carry more, and the page
/// that manages them has to be able to remove them.
pub fn of_person<'a>(flat: &'a Value, person_id: &str) -> Vec<(&'a str, &'a Value)> {
    let Some(docs) = flat.get("documents").and_then(Value::as_object) else {
        return Vec::new();
    };
    let mut out: Vec<(&str, &Value)> = docs
        .iter()
        .filter(|(_, d)| is_ai_portrait(d) && linked_to_person(d, person_id))
        .map(|(id, d)| (id.as_str(), d))
        .collect();
    out.sort_by(|a, b| {
        let at = |d: &Value| {
            d.get("created_at")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_string()
        };
        at(a.1).cmp(&at(b.1)).then(a.0.cmp(b.0))
    });
    out
}

fn linked_to_person(doc: &Value, person_id: &str) -> bool {
    doc.get("linked_to")
        .and_then(Value::as_array)
        .is_some_and(|l| {
            l.iter().any(|x| {
                x.get("entity_type").and_then(Value::as_str) == Some("person")
                    && x.get("entity_id").and_then(Value::as_str) == Some(person_id)
            })
        })
}

/// What one upload says about the portrait beyond its bytes.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Provenance {
    /// The software or model that made it, as the uploader names it.
    pub generator: Option<String>,
    /// The person's photograph it was made from.
    pub derived_from_id: Option<String>,
    pub caption: Option<String>,
}

/// The Document for a new reworked portrait: every field but the file, which
/// the caller fills from the bytes it sniffed.
pub fn document(
    doc_id: &str,
    person_id: &str,
    filename: &str,
    mime: &str,
    file: Value,
    p: &Provenance,
) -> Value {
    let mut ext = serde_json::Map::new();
    if let Some(g) = p
        .generator
        .as_deref()
        .map(str::trim)
        .filter(|g| !g.is_empty())
    {
        ext.insert("generator".into(), json!(g));
    }
    if let Some(d) = p
        .derived_from_id
        .as_deref()
        .map(str::trim)
        .filter(|d| !d.is_empty())
    {
        ext.insert("derived_from_id".into(), json!(d));
    }
    let mut doc = json!({
        "id": doc_id,
        "type": "document",
        "axgf_version": "1.0",
        "filename": filename,
        "mime_type": mime,
        "document_type": "other",
        "status": "present",
        "file": file,
        "linked_to": [{"entity_type": "person", "entity_id": person_id, "role": ROLE}],
        "tags": [TAG],
        "extensions": {EXT_KEY: Value::Object(ext)},
    });
    if let Some(c) = p
        .caption
        .as_deref()
        .map(str::trim)
        .filter(|c| !c.is_empty())
    {
        doc["caption"] = json!(c);
    }
    doc
}

/// The software that made a stored portrait, when the uploader said.
pub fn generator(doc: &Value) -> Option<&str> {
    doc.get("extensions")
        .and_then(|e| e.get(EXT_KEY))
        .and_then(|x| x.get("generator"))
        .and_then(Value::as_str)
        .filter(|g| !g.trim().is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn any_one_mark_is_enough() {
        let plain =
            json!({"linked_to": [{"entity_type": "person", "entity_id": "p", "role": "subject"}]});
        assert!(!is_ai_portrait(&plain));
        assert!(is_ai_portrait(&json!({"extensions": {EXT_KEY: {}}})));
        assert!(is_ai_portrait(&json!({"tags": ["family", TAG]})));
        assert!(is_ai_portrait(
            &json!({"linked_to": [{"entity_type": "person", "entity_id": "p", "role": ROLE}]})
        ));
    }

    #[test]
    fn a_new_portrait_is_marked_three_ways_and_claims_no_photograph() {
        let d = document(
            "d1",
            "p1",
            "face.png",
            "image/png",
            json!({"path": "documents/files/d1.png"}),
            &Provenance {
                generator: Some(" Restorer 2 ".into()),
                derived_from_id: Some("d0".into()),
                caption: Some("  ".into()),
            },
        );
        assert_eq!(d["document_type"], "other");
        assert_eq!(d["linked_to"][0]["role"], ROLE);
        assert_eq!(d["tags"], json!([TAG]));
        assert_eq!(d["extensions"][EXT_KEY]["generator"], "Restorer 2");
        assert_eq!(d["extensions"][EXT_KEY]["derived_from_id"], "d0");
        assert!(d.get("caption").is_none(), "a blank caption is no caption");
        assert!(is_ai_portrait(&d));
        assert_eq!(generator(&d), Some("Restorer 2"));
    }

    #[test]
    fn a_persons_portraits_come_oldest_first() {
        let mk = |at: &str, who: &str| {
            json!({"created_at": at, "tags": [TAG],
                   "linked_to": [{"entity_type": "person", "entity_id": who, "role": ROLE}]})
        };
        let flat = json!({"documents": {
            "b": mk("2026-10-19T10:00:00Z", "p1"),
            "a": mk("2026-10-19T11:00:00Z", "p1"),
            "c": mk("2026-10-19T09:00:00Z", "p2"),
            "photo": {"linked_to": [{"entity_type": "person", "entity_id": "p1", "role": "subject"}]},
        }});
        let ids: Vec<&str> = of_person(&flat, "p1")
            .into_iter()
            .map(|(id, _)| id)
            .collect();
        assert_eq!(ids, vec!["b", "a"]);
        assert!(is_ai_portrait_id(&flat, "c"));
        assert!(!is_ai_portrait_id(&flat, "photo"));
    }
}
