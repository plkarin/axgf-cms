//! The themes, and how one is chosen for a request.
//!
//! # Rendered server-side, on purpose
//!
//! The chosen theme becomes a `data-theme` attribute on `<html>` in the
//! response body. The alternative — a script that reads a cookie and sets the
//! attribute after load — guarantees a flash of the wrong theme on every
//! navigation, and is worst for exactly the reader who most needs the setting:
//! somebody using the high-contrast theme gets a white flash on every page.
//!
//! # Four themes, all on a dark ground (docs/adr/0008, docs/adr/0010)
//!
//! The order is: the account's stored preference, then the cookie, then
//! [`DEFAULT`], which is `canopee`, written into the page. The operating
//! system's `prefers-color-scheme` has no say: every theme is dark. A stored
//! preference for a theme that is no longer offered — the ten that 0.5.0
//! retired, "match my system" among them — is passed over like any unknown
//! value, and the reader gets the default. A page without any `data-theme`
//! falls back to Study 06's dark, which `tokens.css` writes on `:root`.
//!
//! # Canopée, the reference theme (v0.5.0, docs/adr/0004)
//!
//! A theme was a palette and nothing else until Canopée, the first to carry
//! its own furniture as well — the round of its buttons and fields, its
//! elevation, its motion, its loaders and its two photographs. Fusion,
//! Galaxy and Summit share that furniture (the kit), each in its own colours.
//!
//! # Fusion, Galaxy and Summit: themes with a view of their own
//! (docs/adr/0007, docs/adr/0010)
//!
//! Each carries furniture as Canopée does, and one thing more: its own
//! drawing of the landing page. Under Fusion the river's layout is drawn as
//! a living tree, under Galaxy as a solar system ([`view_of`]). The layout,
//! the people in it and what this reader may see of them are the river's,
//! from the same payload; only the drawing changes.
//!
//! `prefers-reduced-motion` keeps its power: it is a medical setting, and
//! nothing a reader picks here overrides it (`static/app.css`, `river.js`).

use serde::Serialize;

/// One theme.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Theme {
    /// The stored value: what goes in the cookie and on the account.
    pub id: &'static str,
    /// The locale key for its name. Themes are named in the interface
    /// language like everything else.
    pub key: &'static str,
    /// A locale key for a short qualifier, where one earns its place.
    pub note_key: Option<&'static str>,
    /// Whether this theme carries the soft background wash.
    ///
    /// True for all three. It was false for the high-contrast theme until
    /// 0.5.0 retired it, and it stays a property of the theme rather than a
    /// rule written into the stylesheet: a theme built for maximum luminance
    /// separation says so here and gets the same treatment without anything
    /// else being edited.
    pub wash: bool,
}

/// Every theme on offer, in the order the selector lists them: Canopée
/// first, because it is the default, then the three that bring a view of
/// their own. Thirteen themes and "match my system" until 0.5.0
/// (docs/adr/0008); Summit joined the three kept (docs/adr/0010).
pub const THEMES: &[Theme] = &[
    Theme {
        id: "canopee",
        key: "theme-canopee",
        note_key: Some("theme-canopee-note"),
        wash: true,
    },
    Theme {
        id: "fusion",
        key: "theme-fusion",
        note_key: Some("theme-fusion-note"),
        wash: true,
    },
    Theme {
        id: "galaxy",
        key: "theme-galaxy",
        note_key: Some("theme-galaxy-note"),
        wash: true,
    },
    Theme {
        id: "summit",
        key: "theme-summit",
        note_key: Some("theme-summit-note"),
        wash: true,
    },
];

/// What a reader gets when they have expressed no preference, or chose a
/// theme that is no longer offered: `canopee`, the reference theme since
/// v0.5.0 (docs/adr/0004). Before it, `dark` — Study 06 — held the place
/// (docs/adr/0002), and before that `system`, which on most machines meant
/// light, so the signature look was the one almost nobody saw.
pub const DEFAULT: &str = "canopee";

/// The drawing of the landing page under a theme: the river, or the view a
/// theme brings of its own (docs/adr/0007) — a living tree under Fusion, a
/// solar system under Galaxy, an ascent under Summit (docs/adr/0010). Under
/// each, the same layout, the same people and the same privacy; only the
/// drawing changes.
pub fn view_of(id: &str) -> &'static str {
    match id {
        "fusion" => "tree",
        "galaxy" => "galaxy",
        "summit" => "summit",
        _ => "river",
    }
}

/// Name of the cookie the theme selector sets.
pub const COOKIE_NAME: &str = "axgf_theme";

/// Name of the cookie carrying "show the background wash, or not".
///
/// A separate preference from the theme, because it is a separate question: a
/// reader can want Galaxy and want it flat. It is stored the same way the theme
/// and the language are — cookie first, and on the account as well when there
/// is one — so all three follow a reader to another browser or none of them
/// do.
pub const WASH_COOKIE_NAME: &str = "axgf_wash";

/// The cookie value that turns the wash off.
pub const WASH_OFF: &str = "off";

/// The cookie value that turns it on. Nothing else does: since v0.4.3 the
/// wash — and the texture a theme may carry with it — is off until a reader
/// asks for it.
pub const WASH_ON: &str = "on";

/// Whether the page wash should be drawn for this request.
///
/// Three things have to agree, and any one of them can say no: the theme (see
/// [`Theme::wash`]), the account, and the cookie. The account beats the cookie
/// for the same reason it does everywhere else — it is the only one of the two
/// the reader chose durably.
pub fn wash_enabled(theme: &Theme, account: Option<bool>, cookie: Option<&str>) -> bool {
    theme.wash && reader_wants_wash(account, cookie)
}

/// What the *reader* asked for, with no theme having a say.
///
/// Separate from [`wash_enabled`] because the preference control has to show
/// the switch as the reader left it even under a theme that will not honour
/// it: choosing such a theme should not silently forget that they had asked
/// for the wash under another.
pub fn reader_wants_wash(account: Option<bool>, cookie: Option<&str>) -> bool {
    if let Some(stored) = account {
        return stored;
    }
    matches!(cookie, Some(WASH_ON))
}

impl Theme {
    /// Look a theme up by its stored id.
    pub fn find(id: &str) -> Option<&'static Theme> {
        THEMES.iter().find(|t| t.id == id.trim())
    }

    /// The theme for `id`, or the default.
    pub fn get(id: &str) -> &'static Theme {
        Self::find(id)
            .unwrap_or_else(|| Self::find(DEFAULT).expect("the default is always present"))
    }

    /// The value for `data-theme`: a page always says which theme it is in.
    pub fn attribute(&self) -> &'static str {
        self.id
    }
}

/// Choose the theme for a request: the account, then the cookie, then the
/// default.
///
/// Deliberately the same order as the language, and for the same reason: the
/// account is the only one of the three the reader chose durably.
pub fn negotiate(account: Option<&str>, cookie: Option<&str>) -> &'static Theme {
    if let Some(t) = account.and_then(Theme::find) {
        return t;
    }
    if let Some(t) = cookie.and_then(Theme::find) {
        return t;
    }
    Theme::get(DEFAULT)
}

/// Every theme, for the selector.
pub fn selector_entries() -> Vec<serde_json::Value> {
    THEMES
        .iter()
        .map(|t| {
            serde_json::json!({
                "id": t.id,
                "key": t.key,
                "note_key": t.note_key,
                "wash": t.wash,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn four_themes_are_offered() {
        // Canopée first, because it is the default, then the three that
        // bring a view of their own. Thirteen themes and "match my system"
        // until 0.5.0, which kept three (docs/adr/0008) and added Summit
        // (docs/adr/0010).
        let ids: Vec<&str> = THEMES.iter().map(|t| t.id).collect();
        assert_eq!(ids, vec!["canopee", "fusion", "galaxy", "summit"]);
    }

    #[test]
    fn three_themes_bring_a_view_of_their_own() {
        assert_eq!(view_of("fusion"), "tree");
        assert_eq!(view_of("galaxy"), "galaxy");
        assert_eq!(view_of("summit"), "summit");
        assert_eq!(view_of("canopee"), "river");
    }

    #[test]
    fn every_theme_is_written_into_the_page() {
        for t in THEMES {
            assert_eq!(Theme::get(t.id).attribute(), t.id);
        }
    }

    #[test]
    fn the_account_beats_the_cookie_which_beats_the_default() {
        assert_eq!(negotiate(Some("galaxy"), Some("fusion")).id, "galaxy");
        assert_eq!(negotiate(None, Some("fusion")).id, "fusion");
        // No preference is Canopée, the reference theme, written into the
        // page rather than left to the system — whatever the system asks for.
        assert_eq!(negotiate(None, None).id, "canopee");
        assert_eq!(negotiate(None, None).attribute(), "canopee");
        // Nonsense at any level falls through rather than failing.
        assert_eq!(negotiate(Some("chartreuse"), Some("fusion")).id, "fusion");
        assert_eq!(negotiate(None, Some("chartreuse")).id, "canopee");
    }

    #[test]
    fn a_reader_who_chose_a_retired_theme_gets_the_default() {
        // Stored on an account or in a cookie before 0.5.0: passed over like
        // any unknown value, never an error, and a later choice still wins.
        for retired in [
            "dark",
            "light",
            "high-contrast",
            "sepia",
            "crepuscule",
            "encre",
            "fleuve-hiver",
            "deuteranopia",
            "protanopia",
            "tritanopia",
            "system",
        ] {
            assert!(Theme::find(retired).is_none(), "{retired}");
            assert_eq!(negotiate(Some(retired), None).id, "canopee", "{retired}");
            assert_eq!(negotiate(None, Some(retired)).id, "canopee", "{retired}");
            assert_eq!(negotiate(Some(retired), Some("galaxy")).id, "galaxy");
        }
    }

    #[test]
    fn every_theme_takes_the_wash() {
        assert!(THEMES.iter().all(|t| t.wash));
    }

    #[test]
    fn the_theme_can_veto_the_wash_and_so_can_the_reader() {
        let galaxy = Theme::get("galaxy");
        // No theme on offer refuses it any more; one built for maximum
        // contrast would, by saying so on itself.
        let flat = Theme {
            id: "flat",
            key: "theme-canopee",
            note_key: None,
            wash: false,
        };

        // Nobody asked: off (v0.4.3).
        assert!(!wash_enabled(galaxy, None, None));
        // The reader turned it on in this browser, then off.
        assert!(wash_enabled(galaxy, None, Some(WASH_ON)));
        assert!(!wash_enabled(galaxy, None, Some(WASH_OFF)));
        // …and on their account, which outranks the cookie either way round.
        assert!(!wash_enabled(galaxy, Some(false), None));
        assert!(wash_enabled(galaxy, Some(true), Some(WASH_OFF)));
        assert!(!wash_enabled(galaxy, Some(false), Some("on")));

        // A theme that refuses it refuses regardless of what either says…
        assert!(!wash_enabled(&flat, Some(true), None));
        assert!(!wash_enabled(&flat, None, Some(WASH_ON)));
        // …but what the reader asked for is remembered underneath it.
        assert!(reader_wants_wash(Some(true), None));
        assert!(!reader_wants_wash(Some(false), None));
    }

    #[test]
    fn every_theme_names_itself_through_the_catalogue() {
        // Themes are named in the interface language like everything else, so
        // each one needs a message that actually exists.
        for t in THEMES {
            assert!(
                crate::i18n::has_message("en", t.key),
                "{} has no English name under {}",
                t.id,
                t.key
            );
            if let Some(note) = t.note_key {
                assert!(crate::i18n::has_message("en", note), "{note} is missing");
            }
        }
    }
}
