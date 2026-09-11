//! Presentation styles: how dense the page is, not what colour it is.
//!
//! # Why this is a second axis and not more themes
//!
//! A theme answers "what does this look like" and owns colour: seven of them,
//! every pair measured against WCAG AA. Density and typographic scale are a
//! different question with a different answer, and multiplying them into the
//! theme list would have produced twenty-one themes and twenty-one palettes to
//! keep in tune. They are orthogonal, so they are stored orthogonally: a reader
//! picks a style and a theme independently and gets the pair they asked for.
//!
//! The division is strict and it is what keeps the contrast work intact. A
//! style may change a size, a space, a weight, a measure or a border width. A
//! style may never change a colour. Nothing in a `[data-style]` block names a
//! hue, and a test fails the build if one ever does — so the twenty-one
//! combinations are seven palettes under three geometries rather than
//! twenty-one palettes.
//!
//! # The three
//!
//! * **Comfortable** is the default and the one the application was designed
//!   in: room to read without spending a screen on it.
//! * **Compact** is for somebody working through records rather than reading
//!   one. Tighter rhythm, smaller type, more of the record per screen; the
//!   point is to cut the scrolling between the thing you just read and the
//!   thing you are comparing it with.
//! * **Paper** is for a record that is going to be read once, slowly, or
//!   printed and handed to a relative. A serif face, a measure that stops
//!   prose running the full width of a monitor, rules instead of cards, and no
//!   shadows at all — which is also what a laser printer does to a card.

use serde::Serialize;

/// One presentation style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Style {
    /// The stored value: what goes in the cookie.
    pub id: &'static str,
    /// The locale key for its name.
    pub key: &'static str,
    /// The locale key for the sentence saying who it is for.
    pub note_key: &'static str,
}

/// Every style, in the order the selector lists them.
pub const STYLES: &[Style] = &[
    Style {
        id: "comfortable",
        key: "style-comfortable",
        note_key: "style-comfortable-note",
    },
    Style {
        id: "compact",
        key: "style-compact",
        note_key: "style-compact-note",
    },
    Style {
        id: "paper",
        key: "style-paper",
        note_key: "style-paper-note",
    },
];

/// The style a reader gets when they have not chosen.
pub const DEFAULT: &str = "comfortable";

/// Name of the cookie carrying the choice.
pub const COOKIE_NAME: &str = "axgf_style";

impl Style {
    pub fn find(id: &str) -> Option<&'static Style> {
        STYLES.iter().find(|s| s.id == id.trim())
    }

    pub fn get(id: &str) -> &'static Style {
        Self::find(id).unwrap_or_else(|| Self::find(DEFAULT).expect("the default is present"))
    }
}

/// Choose the style for a request: the cookie, then the default.
///
/// No account column yet, unlike the language and the theme. The three are
/// stored the same way from the reader's side — one preference panel, one
/// form each — and this one lives only in the cookie until somebody asks for
/// it to follow them between browsers.
pub fn negotiate(cookie: Option<&str>) -> &'static Style {
    cookie
        .and_then(Style::find)
        .unwrap_or_else(|| Style::get(DEFAULT))
}

/// Every style, for the selector.
pub fn selector_entries() -> Vec<serde_json::Value> {
    STYLES
        .iter()
        .map(|s| serde_json::json!({"id": s.id, "key": s.key, "note_key": s.note_key}))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_style_has_a_distinct_id_and_keys_of_its_own() {
        let ids: std::collections::BTreeSet<_> = STYLES.iter().map(|s| s.id).collect();
        let keys: std::collections::BTreeSet<_> = STYLES.iter().map(|s| s.key).collect();
        assert_eq!(ids.len(), STYLES.len());
        assert_eq!(keys.len(), STYLES.len());
        assert!(Style::find(DEFAULT).is_some(), "the default must exist");
    }

    #[test]
    fn an_unknown_style_falls_back_rather_than_failing() {
        // A cookie from a future version, or one somebody typed. The page
        // still renders, in the style the application was designed in.
        assert_eq!(negotiate(Some("hologram")).id, DEFAULT);
        assert_eq!(negotiate(None).id, DEFAULT);
        assert_eq!(negotiate(Some(" compact ")).id, "compact");
    }
}
