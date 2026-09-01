//! The seven themes, and how one is chosen for a request.
//!
//! # Rendered server-side, on purpose
//!
//! The chosen theme becomes a `data-theme` attribute on `<html>` in the
//! response body. The alternative — a script that reads a cookie and sets the
//! attribute after load — guarantees a flash of the wrong theme on every
//! navigation, and is worst for exactly the reader who most needs the setting:
//! somebody using the high-contrast theme gets a white flash on every page.
//!
//! # `system` is a choice, not the absence of one
//!
//! A reader who picks "match my system" has expressed a preference, and it is
//! stored like any other. It renders as *no* `data-theme` attribute, which
//! lets the stylesheet's `prefers-color-scheme` media queries decide. That is
//! why [`Theme::attribute`] can return `None` while [`Theme::id`] cannot: what
//! is stored and what is rendered are different questions.

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
    /// True for the three themes built around a form of colour blindness.
    pub colour_blind: bool,
    /// Whether this theme carries the soft background wash.
    ///
    /// False for high-contrast, and it is a property of the theme rather than
    /// a rule written into the stylesheet: the point of that theme is maximum
    /// luminance separation, and a wash — however faint — spends some of it.
    /// Any future theme with the same purpose says so here and gets the same
    /// treatment without anything else being edited.
    pub wash: bool,
}

/// Every theme on offer, in the order the selector lists them.
pub const THEMES: &[Theme] = &[
    Theme {
        id: "system",
        key: "theme-system",
        note_key: None,
        wash: true,
        colour_blind: false,
    },
    Theme {
        id: "light",
        key: "theme-light",
        note_key: None,
        wash: true,
        colour_blind: false,
    },
    Theme {
        id: "dark",
        key: "theme-dark",
        note_key: None,
        wash: true,
        colour_blind: false,
    },
    Theme {
        id: "high-contrast",
        key: "theme-high-contrast",
        note_key: Some("theme-contrast-note"),
        wash: false,
        colour_blind: false,
    },
    Theme {
        id: "sepia",
        key: "theme-sepia",
        note_key: None,
        wash: true,
        colour_blind: false,
    },
    Theme {
        id: "deuteranopia",
        key: "theme-deuteranopia",
        note_key: Some("theme-colour-blind-note"),
        wash: true,
        colour_blind: true,
    },
    Theme {
        id: "protanopia",
        key: "theme-protanopia",
        note_key: Some("theme-colour-blind-note"),
        wash: true,
        colour_blind: true,
    },
    Theme {
        id: "tritanopia",
        key: "theme-tritanopia",
        note_key: Some("theme-colour-blind-note"),
        wash: true,
        colour_blind: true,
    },
];

/// What a reader gets when they have expressed no preference.
pub const DEFAULT: &str = "system";

/// Name of the cookie the theme selector sets.
pub const COOKIE_NAME: &str = "axgf_theme";

/// Name of the cookie carrying "show the background wash, or not".
///
/// A separate preference from the theme, because it is a separate question: a
/// reader can want sepia and want it flat. It is stored the same way the theme
/// and the language are — cookie first, and on the account as well when there
/// is one — so all three follow a reader to another browser or none of them
/// do.
pub const WASH_COOKIE_NAME: &str = "axgf_wash";

/// The cookie value that turns the wash off. Anything else, including a
/// missing cookie, leaves it on.
pub const WASH_OFF: &str = "off";

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
/// it: turning high-contrast on should not silently forget that they had
/// asked for the wash under sepia.
pub fn reader_wants_wash(account: Option<bool>, cookie: Option<&str>) -> bool {
    if let Some(stored) = account {
        return stored;
    }
    !matches!(cookie, Some(WASH_OFF))
}

impl Theme {
    /// Look a theme up by its stored id.
    pub fn find(id: &str) -> Option<&'static Theme> {
        THEMES.iter().find(|t| t.id == id.trim())
    }

    /// The theme for `id`, or the default.
    pub fn get(id: &str) -> &'static Theme {
        Self::find(id).unwrap_or_else(|| Self::find(DEFAULT).expect("system is always present"))
    }

    /// The value for `data-theme`, or `None` to let the system decide.
    pub fn attribute(&self) -> Option<&'static str> {
        (self.id != "system").then_some(self.id)
    }
}

/// Choose the theme for a request: the account, then the cookie, then the
/// default — which defers to `prefers-color-scheme`.
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
                "colour_blind": t.colour_blind,
                "wash": t.wash,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_seven_themes_plus_system_are_offered() {
        // Seven themes were asked for; `system` is the eighth entry because
        // "follow the browser" is a choice a reader can make, not one of them.
        assert_eq!(THEMES.len(), 8);
        let ids: Vec<&str> = THEMES.iter().map(|t| t.id).collect();
        assert_eq!(
            ids,
            vec![
                "system",
                "light",
                "dark",
                "high-contrast",
                "sepia",
                "deuteranopia",
                "protanopia",
                "tritanopia"
            ]
        );
    }

    #[test]
    fn system_renders_no_attribute_so_the_media_query_decides() {
        assert_eq!(Theme::get("system").attribute(), None);
        assert_eq!(Theme::get("dark").attribute(), Some("dark"));
        assert_eq!(Theme::get("deuteranopia").attribute(), Some("deuteranopia"));
    }

    #[test]
    fn the_account_beats_the_cookie_which_beats_the_default() {
        assert_eq!(negotiate(Some("sepia"), Some("dark")).id, "sepia");
        assert_eq!(negotiate(None, Some("dark")).id, "dark");
        assert_eq!(negotiate(None, None).id, "system");
        // Nonsense at any level falls through rather than failing.
        assert_eq!(negotiate(Some("chartreuse"), Some("dark")).id, "dark");
        assert_eq!(negotiate(None, Some("chartreuse")).id, "system");
    }

    #[test]
    fn only_high_contrast_refuses_the_wash() {
        // A property of the theme, not a rule written into the stylesheet:
        // the next theme built for legibility says so here and needs no
        // stylesheet exception of its own.
        let no_wash: Vec<&str> = THEMES.iter().filter(|t| !t.wash).map(|t| t.id).collect();
        assert_eq!(no_wash, vec!["high-contrast"]);
    }

    #[test]
    fn the_theme_can_veto_the_wash_and_so_can_the_reader() {
        let sepia = Theme::get("sepia");
        let hc = Theme::get("high-contrast");

        // Nobody asked: on.
        assert!(wash_enabled(sepia, None, None));
        // The reader turned it off in this browser.
        assert!(!wash_enabled(sepia, None, Some(WASH_OFF)));
        // …and on their account, which outranks the cookie either way round.
        assert!(!wash_enabled(sepia, Some(false), None));
        assert!(wash_enabled(sepia, Some(true), Some(WASH_OFF)));
        assert!(!wash_enabled(sepia, Some(false), Some("on")));

        // High-contrast refuses regardless of what either of them says…
        assert!(!wash_enabled(hc, Some(true), None));
        assert!(!wash_enabled(hc, None, None));
        // …but what the reader asked for is remembered underneath it, so the
        // switch still reads correctly and sepia gets it back.
        assert!(reader_wants_wash(Some(true), None));
        assert!(!reader_wants_wash(Some(false), None));
    }

    #[test]
    fn exactly_three_themes_are_built_for_colour_blindness() {
        let cb: Vec<&str> = THEMES
            .iter()
            .filter(|t| t.colour_blind)
            .map(|t| t.id)
            .collect();
        assert_eq!(cb, vec!["deuteranopia", "protanopia", "tritanopia"]);
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
