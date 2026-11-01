//! A family's emblem: the picture that stands for a family — a coat of arms,
//! a mark, a photograph — for each family the archive records, and one for
//! the whole archive (the operator's request of 1 November 2026,
//! docs/adr/0022).
//!
//! * **A Family entity's** is chosen like a person's avatar ([`crate::avatar`]):
//!   `extensions["axgf-cms:avatar/v1"]` on the Family names one of its
//!   documents, with a focal point, or says "none". Nobody having chosen, it
//!   is the first image the family links as its `emblem`.
//! * **The archive's** stands for the family the whole archive is about, at
//!   the head of every page: `manifest.family.extensions["axgf-cms:emblem/v1"]`,
//!   the same shape. It lives in `manifest.family` because that is the part
//!   of the manifest an administrator edits, journalled and exported with it.
//!
//! A family's documents are the ones whose `linked_to` names it and the ones
//! its own `documents[]` lists, as a person's are (1.0 §4.2, §5.5). Who may
//! see one follows who may see the family's people
//! ([`crate::access::may_read_document`]); the archive's emblem is shown to
//! every visitor, as the family's name is.

use serde::Serialize;
use serde_json::{json, Map, Value};

use crate::avatar::{Choice, Focal};

/// The archive's choice, under `manifest.family.extensions`.
pub const ARCHIVE_KEY: &str = "axgf-cms:emblem/v1";

/// The role a family's emblem is linked with (`linked_to[].role`,
/// `documents[].role`).
pub const ROLE: &str = "emblem";

/// A picture as a template draws it: its thumbnail and where to centre it.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct EmblemView {
    pub id: String,
    pub src: String,
    pub focal: String,
}

impl EmblemView {
    pub fn new(id: &str, focal: Focal) -> Self {
        Self {
            id: id.to_string(),
            src: format!("/document/{id}/thumb"),
            focal: focal.css(),
        }
    }
}

/// Whether a document is an image a page can show.
fn is_image(doc: &Value) -> bool {
    doc.get("mime_type")
        .and_then(Value::as_str)
        .is_some_and(|m| m.starts_with("image/") && m != "image/svg+xml")
}

/// The documents a family links, each with the role it is linked in: its
/// own `documents[]`, then every document whose `linked_to` names it.
pub fn family_documents<'a>(flat: &'a Value, family_id: &str) -> Vec<(&'a str, Option<&'a str>)> {
    let mut out: Vec<(&str, Option<&str>)> = Vec::new();
    let listed = flat
        .pointer(&format!(
            "/families/{}/documents",
            pointer_escape(family_id)
        ))
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|d| {
            Some((
                d.get("document_id").and_then(Value::as_str)?,
                d.get("role").and_then(Value::as_str),
            ))
        });
    let linking = flat
        .get("documents")
        .and_then(Value::as_object)
        .into_iter()
        .flatten()
        .flat_map(|(id, d)| {
            d.get("linked_to")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .filter(move |l| {
                    l.get("entity_type").and_then(Value::as_str) == Some("family")
                        && l.get("entity_id").and_then(Value::as_str) == Some(family_id)
                })
                .map(move |l| (id.as_str(), l.get("role").and_then(Value::as_str)))
        });
    for (id, role) in listed.chain(linking) {
        if !out.iter().any(|(d, _)| *d == id) {
            out.push((id, role));
        }
    }
    out
}

/// A JSON-pointer segment.
fn pointer_escape(s: &str) -> String {
    s.replace('~', "~0").replace('/', "~1")
}

/// The families that link a document.
pub fn families_linking<'a>(flat: &'a Value, document_id: &str) -> Vec<&'a str> {
    let mut out: Vec<&str> = Vec::new();
    if let Some(fams) = flat.get("families").and_then(Value::as_object) {
        for (fid, f) in fams {
            let lists = f
                .get("documents")
                .and_then(Value::as_array)
                .is_some_and(|ds| {
                    ds.iter()
                        .any(|d| d.get("document_id").and_then(Value::as_str) == Some(document_id))
                });
            if lists {
                out.push(fid);
            }
        }
    }
    let linked = flat
        .pointer(&format!(
            "/documents/{}/linked_to",
            pointer_escape(document_id)
        ))
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter(|l| l.get("entity_type").and_then(Value::as_str) == Some("family"))
        .filter_map(|l| l.get("entity_id").and_then(Value::as_str));
    for fid in linked {
        if !out.contains(&fid) {
            out.push(fid);
        }
    }
    out
}

/// A family's images, for the picker.
pub fn family_images<'a>(flat: &'a Value, family_id: &str) -> Vec<&'a Value> {
    family_documents(flat, family_id)
        .into_iter()
        .filter_map(|(id, _)| flat.get("documents")?.get(id))
        .filter(|d| is_image(d))
        .collect()
}

/// What a family's emblem is: the chosen picture, or the first image it
/// links as its emblem when nobody has chosen. `None` when it has none, or
/// when somebody chose none.
pub fn of_family(flat: &Value, family_id: &str) -> Option<(String, Focal)> {
    let family = flat.get("families")?.get(family_id)?;
    let exists = |id: &str| {
        flat.get("documents")
            .and_then(|d| d.get(id))
            .is_some_and(is_image)
    };
    match crate::avatar::read(family) {
        Choice::None => None,
        Choice::Document { id, focal } if exists(&id) => Some((id, focal)),
        _ => family_documents(flat, family_id)
            .into_iter()
            .find(|(id, role)| *role == Some(ROLE) && exists(id))
            .map(|(id, _)| (id.to_string(), Focal::default())),
    }
}

/// The archive's emblem, as the manifest records it.
pub fn of_archive(flat: &Value) -> Option<(String, Focal)> {
    let stored = flat
        .pointer("/manifest/family/extensions")?
        .get(ARCHIVE_KEY)?;
    let as_entity = json!({ "extensions": { crate::avatar::AVATAR_KEY: stored } });
    match crate::avatar::read(&as_entity) {
        Choice::Document { id, focal }
            if flat
                .get("documents")
                .and_then(|d| d.get(&id))
                .is_some_and(is_image) =>
        {
            Some((id, focal))
        }
        _ => None,
    }
}

/// The archive's emblem document, whatever it shows: the one document every
/// visitor may read for it.
pub fn archive_document(flat: &Value) -> Option<&str> {
    flat.pointer("/manifest/family/extensions")?
        .get(ARCHIVE_KEY)?
        .get("document_id")?
        .as_str()
}

/// Write the archive's choice into `manifest.family`: a picture with its
/// focal point, or none. Every other extension is left alone.
pub fn set_archive(manifest: &mut Map<String, Value>, choice: Option<(&str, Focal)>) {
    let family = manifest
        .entry("family")
        .or_insert_with(|| Value::Object(Map::new()));
    let Some(family) = family.as_object_mut() else {
        return;
    };
    let mut ext = family
        .get("extensions")
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();
    match choice {
        Some((id, focal)) => {
            let mut m = Map::new();
            m.insert("document_id".into(), json!(id));
            if focal != Focal::default() {
                m.insert("focal".into(), json!({"x": focal.x, "y": focal.y}));
            }
            ext.insert(ARCHIVE_KEY.to_string(), Value::Object(m));
        }
        None => {
            ext.remove(ARCHIVE_KEY);
        }
    }
    if ext.is_empty() {
        family.remove("extensions");
    } else {
        family.insert("extensions".into(), Value::Object(ext));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn flat() -> Value {
        json!({
            "manifest": {"axgf": "1.1", "family": {"name": "Mensah",
                "extensions": {ARCHIVE_KEY: {"document_id": "arms", "focal": {"x": 0.5, "y": 0.2}}}}},
            "families": {
                "f1": {"id": "f1", "type": "family",
                       "documents": [{"document_id": "photo", "role": "family_photo"}]},
                "f2": {"id": "f2", "type": "family",
                       "extensions": {"axgf-cms:avatar/v1": {"mode": "none"}}}
            },
            "documents": {
                "arms": {"id": "arms", "mime_type": "image/png"},
                "photo": {"id": "photo", "mime_type": "image/jpeg"},
                "crest": {"id": "crest", "mime_type": "image/png",
                          "linked_to": [{"entity_type": "family", "entity_id": "f1", "role": "emblem"}]},
                "deed": {"id": "deed", "mime_type": "application/pdf",
                         "linked_to": [{"entity_type": "family", "entity_id": "f1", "role": "emblem"}]}
            }
        })
    }

    #[test]
    fn a_familys_emblem_is_the_one_chosen_else_the_one_it_links_as_its_emblem() {
        let f = flat();
        // Nobody chose: the image linked as the emblem, not the photograph
        // listed first, and never a PDF.
        assert_eq!(
            of_family(&f, "f1"),
            Some(("crest".to_string(), Focal::default()))
        );
        assert_eq!(family_images(&f, "f1").len(), 2);
        assert_eq!(families_linking(&f, "crest"), vec!["f1"]);
        assert_eq!(families_linking(&f, "photo"), vec!["f1"]);
        // Somebody chose none.
        assert_eq!(of_family(&f, "f2"), None);
        // Somebody chose the photograph.
        let mut g = f.clone();
        g["families"]["f1"] = crate::avatar::apply(
            &g["families"]["f1"],
            &Choice::Document {
                id: "photo".into(),
                focal: Focal::new(0.3, 0.4),
            },
        );
        assert_eq!(
            of_family(&g, "f1"),
            Some(("photo".to_string(), Focal::new(0.3, 0.4)))
        );
    }

    #[test]
    fn the_archives_emblem_lives_in_the_manifests_family() {
        let f = flat();
        assert_eq!(
            of_archive(&f),
            Some(("arms".to_string(), Focal::new(0.5, 0.2)))
        );
        assert_eq!(archive_document(&f), Some("arms"));
        let mut m = f["manifest"].as_object().cloned().unwrap();
        set_archive(&mut m, None);
        assert!(m["family"].get("extensions").is_none(), "{m:?}");
        set_archive(&mut m, Some(("crest", Focal::default())));
        assert_eq!(
            m["family"]["extensions"][ARCHIVE_KEY],
            json!({"document_id": "crest"})
        );
        assert_eq!(m["family"]["name"], "Mensah");
    }
}
