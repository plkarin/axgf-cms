//! Which picture stands for a person, chosen rather than guessed.
//!
//! # Why the guess cannot be fixed
//!
//! The avatar used to be picked automatically: the first document with role
//! `portrait`, else the first with type `portrait`, else simply the first
//! image. On the operator's bundle none of the first two ever match. The
//! GEDCOM converter stamps `photo` on every scan and records exactly one role,
//! `subject`, for all 406 of them — so a death notice, a land deed and a
//! studio portrait are indistinguishable, and the avatar is whichever happened
//! to be linked first. That is how a newspaper clipping ended up as Wincenty's
//! face.
//!
//! No heuristic recovers from that, because the answer is not in the data. The
//! only fix is to let somebody say which one it is, and to store what they
//! said.
//!
//! # What is stored
//!
//! `extensions["axgf-cms:avatar/v1"]`, namespaced and versioned like the other
//! additions in this release, so the choice survives export, re-import and any
//! future reordering of the document list — which is exactly what an
//! index-into-a-list would not.
//!
//! Three states, and the third is not the same as the first:
//!
//! * **Automatic** — no extension at all. The old heuristic, which is right
//!   for a bundle that does say which image is the portrait.
//! * **A document**, with an optional focal point. See [`Focal`].
//! * **None** — show the initials. Some people would rather show initials than
//!   a document that happens to be an image, and "I chose nothing" is a
//!   different statement from "nobody has chosen yet".

use serde::Serialize;
use serde_json::{json, Map, Value};

/// Namespaced key for the choice.
pub const AVATAR_KEY: &str = "axgf-cms:avatar/v1";

/// Where in the picture the face is.
///
/// An avatar is square and most scans are not, so something has to be cropped.
/// The default centres the image, which puts a face in the corner of a group
/// photograph outside the frame. Stored as two fractions of the image's own
/// width and height rather than pixels, so the point survives the image being
/// re-encoded at another size, and rendered as a CSS `object-position` — which
/// is the same arithmetic the browser already does for `object-fit: cover`,
/// and needs no crop editor with handles to produce.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct Focal {
    pub x: f64,
    pub y: f64,
}

impl Default for Focal {
    fn default() -> Self {
        Self { x: 0.5, y: 0.5 }
    }
}

impl Focal {
    /// Clamped to the image. A point outside it is a bug somewhere upstream,
    /// and clamping is better than a crop that shows nothing.
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x: x.clamp(0.0, 1.0),
            y: y.clamp(0.0, 1.0),
        }
    }

    /// `object-position`, as a percentage pair.
    pub fn css(self) -> String {
        format!("{:.1}% {:.1}%", self.x * 100.0, self.y * 100.0)
    }
}

/// What the record says about its own avatar.
#[derive(Debug, Clone, PartialEq)]
pub enum Choice {
    /// Nobody has chosen; fall back to the heuristic.
    Auto,
    /// Somebody chose to show no picture at all.
    None,
    /// This document, framed on this point.
    Document { id: String, focal: Focal },
}

/// Read the stored choice.
///
/// An unrecognised shape is [`Choice::Auto`] rather than an error: the
/// extension is somebody else's data model as far as this build is concerned,
/// and refusing to render a person because their avatar extension is odd would
/// be the wrong trade.
pub fn read(person: &Value) -> Choice {
    let Some(obj) = person
        .get("extensions")
        .and_then(|e| e.get(AVATAR_KEY))
        .and_then(Value::as_object)
    else {
        return Choice::Auto;
    };
    if obj.get("mode").and_then(Value::as_str) == Some("none") {
        return Choice::None;
    }
    match obj.get("document_id").and_then(Value::as_str) {
        Some(id) if !id.trim().is_empty() => {
            let f = obj.get("focal");
            let focal = match (
                f.and_then(|f| f.get("x")).and_then(Value::as_f64),
                f.and_then(|f| f.get("y")).and_then(Value::as_f64),
            ) {
                (Some(x), Some(y)) => Focal::new(x, y),
                _ => Focal::default(),
            };
            Choice::Document {
                id: id.to_string(),
                focal,
            }
        }
        _ => Choice::Auto,
    }
}

/// Write a choice onto the stored entity, leaving every other extension alone.
pub fn apply(stored: &Value, choice: &Choice) -> Value {
    let mut out = stored.clone();
    let Some(obj) = out.as_object_mut() else {
        return out;
    };
    let mut ext = obj
        .get("extensions")
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();

    match choice {
        // Automatic is the *absence* of a choice, so it removes the key rather
        // than writing `{"mode": "auto"}`. A bundle that has never been told
        // and a bundle that has been told "work it out" are the same bundle,
        // and storing them differently would be a distinction without one.
        Choice::Auto => {
            ext.remove(AVATAR_KEY);
        }
        Choice::None => {
            ext.insert(AVATAR_KEY.to_string(), json!({"mode": "none"}));
        }
        Choice::Document { id, focal } => {
            let mut m = Map::new();
            m.insert("document_id".into(), json!(id));
            // A centred focal point is the default, so it is not written: the
            // stored object should say what somebody decided, not repeat what
            // the renderer would have done anyway.
            if *focal != Focal::default() {
                m.insert("focal".into(), json!({"x": focal.x, "y": focal.y}));
            }
            ext.insert(AVATAR_KEY.to_string(), Value::Object(m));
        }
    }

    if ext.is_empty() {
        obj.remove("extensions");
    } else {
        obj.insert("extensions".into(), Value::Object(ext));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_extension_means_the_heuristic_still_runs() {
        assert_eq!(read(&json!({"id": "x"})), Choice::Auto);
        assert_eq!(read(&json!({"extensions": {}})), Choice::Auto);
    }

    #[test]
    fn choosing_nothing_is_not_the_same_as_choosing_nothing_yet() {
        let stored = apply(&json!({"id": "x"}), &Choice::None);
        assert_eq!(stored["extensions"][AVATAR_KEY]["mode"], "none");
        assert_eq!(read(&stored), Choice::None);

        // …and clearing it removes the key rather than writing "auto".
        let cleared = apply(&stored, &Choice::Auto);
        assert!(cleared.get("extensions").is_none(), "{cleared}");
        assert_eq!(read(&cleared), Choice::Auto);
    }

    #[test]
    fn a_choice_is_stored_by_document_id_not_by_position() {
        // An index into the document list would be silently wrong the moment
        // another document is attached or removed.
        let stored = apply(
            &json!({"id": "x"}),
            &Choice::Document {
                id: "doc-7".into(),
                focal: Focal::default(),
            },
        );
        assert_eq!(stored["extensions"][AVATAR_KEY]["document_id"], "doc-7");
        assert!(
            stored["extensions"][AVATAR_KEY].get("focal").is_none(),
            "a centred focal point is the default and is not written"
        );
    }

    #[test]
    fn a_focal_point_round_trips_and_is_clamped() {
        let stored = apply(
            &json!({}),
            &Choice::Document {
                id: "d".into(),
                focal: Focal::new(0.22, 0.13),
            },
        );
        match read(&stored) {
            Choice::Document { focal, .. } => {
                assert!((focal.x - 0.22).abs() < 1e-9);
                assert!((focal.y - 0.13).abs() < 1e-9);
                assert_eq!(focal.css(), "22.0% 13.0%");
            }
            other => panic!("{other:?}"),
        }
        assert_eq!(Focal::new(-3.0, 9.0), Focal { x: 0.0, y: 1.0 });
    }

    #[test]
    fn another_applications_extension_survives() {
        let stored = json!({"extensions": {"org.example/x": {"keep": 1}}});
        let out = apply(&stored, &Choice::None);
        assert_eq!(out["extensions"]["org.example/x"]["keep"], json!(1));
        let back = apply(&out, &Choice::Auto);
        assert_eq!(back["extensions"]["org.example/x"]["keep"], json!(1));
        assert!(back["extensions"].get(AVATAR_KEY).is_none());
    }

    #[test]
    fn an_odd_extension_falls_back_rather_than_failing() {
        assert_eq!(read(&json!({"extensions": {AVATAR_KEY: 42}})), Choice::Auto);
        assert_eq!(
            read(&json!({"extensions": {AVATAR_KEY: {"document_id": "  "}}})),
            Choice::Auto
        );
    }
}
