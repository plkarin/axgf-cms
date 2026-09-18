//! The settings surface: three tabs over the preferences a reader owns.
//!
//! # Why three tabs rather than one panel
//!
//! Theme, language and presentation style are three unrelated questions, and
//! they used to be four stacked forms inside one dropdown in the masthead. That
//! put eleven languages, eight themes, three styles and a checkbox — twenty-three
//! controls — behind a single chevron, so a reader looking for one of them read
//! past the other twenty-two. Each is now a tab that shows only its own choices.
//!
//! # Why real URLs
//!
//! The same reason the record's tabs are real URLs: each is a link somebody can
//! send, the back button goes back a tab, and the page works with scripting off.
//! The whole application does, and the screen where a reader sets their
//! *language* is the last one to make dependent on a script.
//!
//! # Where the reader came from
//!
//! `from` carries it, and every form posts a `back` that returns to this page
//! on this tab — so choosing a theme shows the theme rather than throwing the
//! reader back to their record to look at it. The link out is theirs to take
//! when they are done. Both values are sanitised by [`crate::render`]'s
//! `safe_back` before they reach a `Location` header.

use serde::Serialize;

/// Which group of preferences the reader asked for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Tab {
    /// What colour the page is: the seven themes and `system`.
    Theme,
    /// What language it is in, and how complete each catalogue is.
    Language,
    /// How dense it is, and whether the page carries its wash. Both are
    /// geometry and neither is colour — see [`crate::style`].
    Appearance,
}

/// The tabs, in the order they are shown.
pub const TABS: &[Tab] = &[Tab::Theme, Tab::Language, Tab::Appearance];

impl Tab {
    /// The slug used in `?tab=` and as the template's discriminator.
    pub fn slug(self) -> &'static str {
        match self {
            Self::Theme => "theme",
            Self::Language => "language",
            Self::Appearance => "appearance",
        }
    }

    /// The catalogue key for its label.
    pub fn key(self) -> &'static str {
        match self {
            Self::Theme => "settings-tab-theme",
            Self::Language => "settings-tab-language",
            Self::Appearance => "settings-tab-appearance",
        }
    }

    /// Read a tab out of `?tab=`.
    ///
    /// An unknown value is the theme rather than a 404, for the reason the
    /// record's tabs do the same: a stale link should land the reader on the
    /// page they asked for, with the thing they wanted one click away.
    pub fn from_query(value: Option<&str>) -> Self {
        match value.map(str::trim) {
            Some("language") => Self::Language,
            Some("appearance") => Self::Appearance,
            _ => Self::Theme,
        }
    }

    /// This page, on this tab, remembering where the reader came from.
    pub fn url(self, from: &str) -> String {
        url(self.slug(), from)
    }

    /// The tabs as the template wants them: slug and label key.
    pub fn entries() -> Vec<serde_json::Value> {
        TABS.iter()
            .map(|t| serde_json::json!({"slug": t.slug(), "key": t.key()}))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_tab_round_trips_through_its_own_slug() {
        for tab in TABS {
            assert_eq!(Tab::from_query(Some(tab.slug())), *tab, "{}", tab.slug());
        }
    }

    #[test]
    fn an_unknown_tab_is_the_theme_rather_than_an_error() {
        assert_eq!(Tab::from_query(None), Tab::Theme);
        assert_eq!(Tab::from_query(Some("")), Tab::Theme);
        assert_eq!(Tab::from_query(Some("nonsense")), Tab::Theme);
        // Case and padding are a mistyped link, not a different tab.
        assert_eq!(Tab::from_query(Some("Language")), Tab::Theme);
        assert_eq!(Tab::from_query(Some(" language ")), Tab::Language);
    }

    #[test]
    fn the_way_back_survives_a_query_string_without_ending_the_url() {
        // A `from` carrying its own `?` and `&` must not turn into two more
        // parameters on the settings URL. `/` is left as itself: it is legal
        // in a query value and the alternative is an unreadable URL for no
        // gain.
        let u = url("theme", "/person/abc?tab=life&x=1");
        assert_eq!(
            u,
            "/settings?tab=theme&from=/person/abc%3Ftab%3Dlife%26x%3D1"
        );
        // The root needs no `from` at all: there is nothing to return to that
        // the masthead does not already offer.
        assert_eq!(url("theme", "/"), "/settings?tab=theme");
        assert_eq!(entry_url("/"), "/settings");
    }

    #[test]
    fn every_tab_has_a_distinct_slug_and_key() {
        let slugs: std::collections::BTreeSet<_> = TABS.iter().map(|t| t.slug()).collect();
        let keys: std::collections::BTreeSet<_> = TABS.iter().map(|t| t.key()).collect();
        assert_eq!(slugs.len(), TABS.len());
        assert_eq!(keys.len(), TABS.len());
    }
}

/// The settings page on one tab, carrying where the reader came from.
pub fn url(slug: &str, from: &str) -> String {
    if from == "/" {
        format!("/settings?tab={slug}")
    } else {
        format!("/settings?tab={slug}&from={}", urlencode(from))
    }
}

/// The way in from the masthead: the default tab, carrying the way back.
pub fn entry_url(from: &str) -> String {
    if from == "/" {
        "/settings".to_string()
    } else {
        format!("/settings?from={}", urlencode(from))
    }
}

/// Percent-encode a same-site path for use as a query value.
///
/// Small and local rather than a dependency. The input is always a path that
/// has already been through [`crate::render::safe_back`], so what is left to
/// encode is the handful of characters that would otherwise end the query
/// value or start another parameter.
fn urlencode(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 8);
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' | b'/' => {
                out.push(b as char)
            }
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}
