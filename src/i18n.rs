//! Interface translation.
//!
//! # The rule this module exists to enforce
//!
//! **Translate the interface. Never translate the data.**
//!
//! An English speaker browsing a Polish family wants English buttons and
//! Polish place names. AXGF carries that distinction itself — `place.names[]`
//! has a `lang`, a name component has both `value` and `value_latin` — and
//! throwing it away by "helpfully" rendering *Kraków* as *Cracow* would be a
//! worse product and a false record. So: labels, headings, help text, messages
//! and error pages come from here. Names, places, notes, occupations, source
//! titles and every date's *meaning* come from the bundle, in their own
//! language and their own script, whatever the interface is set to.
//!
//! The test that keeps this honest is in `tests/i18n.rs`: it fails if a
//! template contains a literal English sentence.
//!
//! # Fluent, for the plural rules
//!
//! Chinese and Japanese have no plural forms. Polish has three and Arabic six,
//! and neither set maps onto "one thing or more than one thing". Hand-rolled
//! `if n == 1` logic produces text that is simply wrong in most of the
//! languages here, and it is wrong in a way an English-speaking author cannot
//! see. Fluent carries the CLDR plural rules, so `{ $n ->  [one] … *[other] …}`
//! resolves correctly per locale without this crate knowing anything about
//! Polish.
//!
//! # Missing messages fall back to English, and that is reported
//!
//! A locale file need not be complete. An untranslated key resolves through
//! the English bundle rather than rendering as an error, so a partial locale
//! is a usable one. What is *not* acceptable is claiming it is finished, so
//! [`Locale::coverage`] reports the real number and the language selector
//! shows it.

use std::collections::HashMap;
use std::sync::OnceLock;

use fluent::{FluentArgs, FluentResource, FluentValue};
use unic_langid::LanguageIdentifier;

/// The thread-safe bundle.
///
/// `fluent::FluentBundle` memoises its `intl` formatters behind a `RefCell`,
/// which makes it `!Sync` and therefore impossible to keep in a `static`. The
/// concurrent memoizer is the same bundle with a lock around that cache, which
/// is what a shared, request-serving process needs.
type Bundle =
    fluent::bundle::FluentBundle<FluentResource, intl_memoizer::concurrent::IntlLangMemoizer>;

/// Text direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    Ltr,
    Rtl,
}

impl Dir {
    pub fn as_str(self) -> &'static str {
        match self {
            Dir::Ltr => "ltr",
            Dir::Rtl => "rtl",
        }
    }
}

/// One interface language.
#[derive(Debug, Clone, Copy)]
pub struct Locale {
    /// BCP 47 tag, as used in the cookie, the account and `<html lang>`.
    pub tag: &'static str,
    /// The language's name in English, for an English-speaking administrator.
    pub english_name: &'static str,
    /// The language's name in itself, which is what a speaker of it looks for
    /// in a language menu. A list of endonyms is findable by someone who reads
    /// none of the other options.
    pub native_name: &'static str,
    pub dir: Dir,
    /// Whether a person who speaks this language has read the translation.
    ///
    /// Stated per locale rather than assumed, because the honest position is
    /// that machine-assisted translation of genealogical vocabulary is not
    /// reliable: "union", "affiliation" and "confidence" have established
    /// equivalents that vary by national record-keeping tradition, and a
    /// plausible wrong word is worse than an English one.
    pub reviewed: bool,
    /// The `.ftl` source, embedded at compile time. No build step, no files to
    /// deploy beside the binary.
    source: &'static str,
}

/// Every language the interface is offered in.
pub const LOCALES: &[Locale] = &[
    Locale {
        tag: "en",
        english_name: "English",
        native_name: "English",
        dir: Dir::Ltr,
        reviewed: true,
        source: include_str!("../locales/en.ftl"),
    },
    Locale {
        tag: "fr",
        english_name: "French",
        native_name: "Français",
        dir: Dir::Ltr,
        reviewed: true,
        source: include_str!("../locales/fr.ftl"),
    },
    Locale {
        tag: "pl",
        english_name: "Polish",
        native_name: "Polski",
        dir: Dir::Ltr,
        reviewed: false,
        source: include_str!("../locales/pl.ftl"),
    },
    Locale {
        tag: "de",
        english_name: "German",
        native_name: "Deutsch",
        dir: Dir::Ltr,
        reviewed: false,
        source: include_str!("../locales/de.ftl"),
    },
    Locale {
        tag: "it",
        english_name: "Italian",
        native_name: "Italiano",
        dir: Dir::Ltr,
        reviewed: false,
        source: include_str!("../locales/it.ftl"),
    },
    Locale {
        tag: "es",
        english_name: "Spanish",
        native_name: "Español",
        dir: Dir::Ltr,
        reviewed: false,
        source: include_str!("../locales/es.ftl"),
    },
    Locale {
        tag: "pt",
        english_name: "Portuguese",
        native_name: "Português",
        dir: Dir::Ltr,
        reviewed: false,
        source: include_str!("../locales/pt.ftl"),
    },
    Locale {
        tag: "zh-Hans",
        english_name: "Chinese (Simplified)",
        native_name: "简体中文",
        dir: Dir::Ltr,
        reviewed: false,
        source: include_str!("../locales/zh-Hans.ftl"),
    },
    Locale {
        tag: "ja",
        english_name: "Japanese",
        native_name: "日本語",
        dir: Dir::Ltr,
        reviewed: false,
        source: include_str!("../locales/ja.ftl"),
    },
    Locale {
        tag: "ar",
        english_name: "Arabic",
        native_name: "العربية",
        dir: Dir::Rtl,
        reviewed: false,
        source: include_str!("../locales/ar.ftl"),
    },
];

/// The language everything falls back to.
pub const DEFAULT: &str = "en";

/// Name of the cookie the language selector sets.
pub const COOKIE_NAME: &str = "axgf_lang";

impl Locale {
    /// Look a locale up by tag, case-insensitively.
    pub fn find(tag: &str) -> Option<&'static Locale> {
        LOCALES
            .iter()
            .find(|l| l.tag.eq_ignore_ascii_case(tag.trim()))
    }

    /// The locale for `tag`, or English.
    pub fn get(tag: &str) -> &'static Locale {
        Self::find(tag).unwrap_or_else(|| Self::find(DEFAULT).expect("English is always present"))
    }

    /// How many of English's messages this locale defines, and out of how many.
    ///
    /// The honest number behind the "machine-quality" label: a reader deciding
    /// whether to switch wants to know whether they are getting a translated
    /// interface or three translated buttons.
    pub fn coverage(&self) -> (usize, usize) {
        let counts = counts();
        let total = counts.get(DEFAULT).copied().unwrap_or(0);
        let mine = counts.get(self.tag).copied().unwrap_or(0);
        (mine, total)
    }

    /// Coverage as a whole-number percentage.
    pub fn coverage_percent(&self) -> u32 {
        let (mine, total) = self.coverage();
        if total == 0 {
            return 100;
        }
        ((mine as f64 / total as f64) * 100.0).round() as u32
    }

    pub fn is_rtl(&self) -> bool {
        self.dir == Dir::Rtl
    }
}

/// Every locale's parsed bundle.
struct Catalog {
    bundles: HashMap<&'static str, Bundle>,
}

impl Catalog {
    fn build() -> Self {
        let mut bundles = HashMap::new();
        for locale in LOCALES {
            let langid: LanguageIdentifier = locale.tag.parse().unwrap_or_else(|_| {
                panic!("{} is not a valid BCP 47 tag", locale.tag);
            });
            let mut bundle = Bundle::new_concurrent(vec![langid]);
            // Fluent wraps every placeable in Unicode isolation marks by
            // default. They are invisible, correct for plain text, and a
            // nuisance here: this output goes into HTML, where the same job is
            // done visibly and debuggably by `dir` attributes and `<bdi>`, and
            // where stray U+2068 characters turn every string assertion in the
            // test suite into a puzzle.
            bundle.set_use_isolating(false);
            match FluentResource::try_new(locale.source.to_string()) {
                Ok(res) => {
                    if let Err(errors) = bundle.add_resource(res) {
                        // A duplicate or malformed message is a bug in a
                        // locale file, not a reason to fail to start: the
                        // messages that did parse still work, and the rest
                        // fall back to English.
                        for e in errors {
                            tracing::error!(locale = locale.tag, "locale error: {e}");
                        }
                    }
                }
                Err((res, errors)) => {
                    for e in errors {
                        tracing::error!(locale = locale.tag, "locale parse error: {e:?}");
                    }
                    let _ = bundle.add_resource(res);
                }
            }
            bundles.insert(locale.tag, bundle);
        }
        Self { bundles }
    }
}

static CATALOG: OnceLock<Catalog> = OnceLock::new();
static COUNTS: OnceLock<HashMap<&'static str, usize>> = OnceLock::new();

fn catalog() -> &'static Catalog {
    CATALOG.get_or_init(Catalog::build)
}

/// How many of *English's* messages each locale defines.
///
/// `FluentBundle` does not expose its message list, so the ids are read from
/// the `.ftl` source. Counting the intersection with English rather than the
/// locale's own total is what makes the number mean "how much of the interface
/// is translated" — a file with ten messages of its own and none of English's
/// would otherwise report ten.
fn counts() -> &'static HashMap<&'static str, usize> {
    COUNTS.get_or_init(|| {
        let english: std::collections::BTreeSet<String> = LOCALES
            .iter()
            .find(|l| l.tag == DEFAULT)
            .map(|l| message_ids(l.source).into_iter().collect())
            .unwrap_or_default();
        LOCALES
            .iter()
            .map(|l| {
                let mine = message_ids(l.source)
                    .into_iter()
                    .filter(|id| english.contains(id))
                    .count();
                (l.tag, mine)
            })
            .collect()
    })
}

/// The message ids defined in one `.ftl` source.
pub fn message_ids(source: &str) -> Vec<String> {
    source
        .lines()
        .filter(|l| !l.starts_with(['#', ' ', '\t', '.', '*', '[', '}']))
        .filter_map(|l| l.split_once('='))
        .map(|(id, _)| id.trim().to_string())
        .filter(|id| {
            !id.is_empty()
                && id
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        })
        .collect()
}

/// Translate `key` into `tag`, falling back to English and then to the key.
///
/// Returning the key itself as a last resort rather than an empty string is
/// deliberate: a missing message then shows up as `tree-hidden-count` on the
/// page, which is ugly and unmistakable, where a blank would silently look
/// like a design choice.
pub fn translate(tag: &str, key: &str, args: Option<&FluentArgs>) -> String {
    if let Some(s) = lookup(tag, key, args) {
        return s;
    }
    if tag != DEFAULT {
        if let Some(s) = lookup(DEFAULT, key, args) {
            return s;
        }
    }
    key.to_string()
}

fn lookup(tag: &str, key: &str, args: Option<&FluentArgs>) -> Option<String> {
    let bundle = catalog().bundles.get(tag)?;
    let msg = bundle.get_message(key)?;
    let pattern = msg.value()?;
    let mut errors = vec![];
    let out = bundle.format_pattern(pattern, args, &mut errors);
    if !errors.is_empty() {
        tracing::debug!(locale = tag, key, "formatting errors: {errors:?}");
    }
    Some(out.into_owned())
}

/// Whether `tag` defines `key` itself, rather than inheriting it.
pub fn has_message(tag: &str, key: &str) -> bool {
    catalog()
        .bundles
        .get(tag)
        .and_then(|b| b.get_message(key))
        .is_some()
}

/// Choose the interface language for a request.
///
/// In order: the signed-in account's stored preference, then the cookie the
/// selector sets, then `Accept-Language` negotiated against what is on offer,
/// then English.
///
/// The account first, because it is the only one of the three the reader
/// chose *deliberately and durably*; a cookie is per browser, and
/// `Accept-Language` is usually whatever the operating system was installed
/// with rather than a statement about this site.
pub fn negotiate(
    account: Option<&str>,
    cookie: Option<&str>,
    accept_language: Option<&str>,
) -> &'static Locale {
    if let Some(l) = account.and_then(Locale::find) {
        return l;
    }
    if let Some(l) = cookie.and_then(Locale::find) {
        return l;
    }
    if let Some(header) = accept_language {
        if let Some(l) = from_accept_language(header) {
            return l;
        }
    }
    Locale::get(DEFAULT)
}

/// Parse `Accept-Language` and pick the best available match.
///
/// Matches the exact tag first, then the base language — so `pt-BR` finds
/// `pt`, and `zh-CN` finds `zh-Hans`, which is what a reader sending either
/// actually wants. An unparseable q-value sorts last rather than failing the
/// header.
fn from_accept_language(header: &str) -> Option<&'static Locale> {
    let mut candidates: Vec<(f32, &str)> = header
        .split(',')
        .filter_map(|part| {
            let mut bits = part.split(';');
            let tag = bits.next()?.trim();
            if tag.is_empty() || tag == "*" {
                return None;
            }
            let q = bits
                .find_map(|b| b.trim().strip_prefix("q=").and_then(|v| v.parse().ok()))
                .unwrap_or(1.0);
            Some((q, tag))
        })
        .collect();
    // Descending by quality, stable so equal weights keep header order.
    candidates.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

    for (_, tag) in &candidates {
        if let Some(l) = Locale::find(tag) {
            return Some(l);
        }
        let base = tag.split('-').next().unwrap_or(tag);
        // `zh` on its own, and `zh-CN`, both mean Simplified here; the
        // Traditional script is not offered, so matching the base language is
        // the closest true answer rather than falling through to English.
        if let Some(l) = LOCALES
            .iter()
            .find(|l| l.tag.split('-').next() == Some(base))
        {
            return Some(l);
        }
    }
    None
}

/// Build Fluent arguments from name/value pairs.
pub fn args<'a>(pairs: &[(&'a str, FluentValue<'a>)]) -> FluentArgs<'a> {
    let mut a = FluentArgs::new();
    for (k, v) in pairs {
        a.set(*k, v.clone());
    }
    a
}

/// Every locale, with the numbers the selector shows.
pub fn selector_entries() -> Vec<serde_json::Value> {
    LOCALES
        .iter()
        .map(|l| {
            serde_json::json!({
                "tag": l.tag,
                "english_name": l.english_name,
                "native_name": l.native_name,
                "dir": l.dir.as_str(),
                "reviewed": l.reviewed,
                "coverage": l.coverage_percent(),
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_declared_locale_parses() {
        // A locale file that does not parse would silently serve English for
        // every one of its messages, which is the failure mode most likely to
        // go unnoticed.
        for l in LOCALES {
            assert!(
                !message_ids(l.source).is_empty(),
                "{} defines no messages at all",
                l.tag
            );
            assert!(
                has_message(l.tag, "app-name"),
                "{} did not parse: even app-name is missing",
                l.tag
            );
        }
    }

    #[test]
    fn english_is_complete_by_definition() {
        let en = Locale::get("en");
        assert_eq!(en.coverage_percent(), 100);
        assert!(en.reviewed);
    }

    #[test]
    fn a_missing_message_falls_back_to_english_not_to_a_blank() {
        let out = translate("pl", "app-name", None);
        assert!(!out.is_empty());
        // An id nothing defines shows as itself, which is unmistakable on a
        // page; a blank would look like a design decision.
        assert_eq!(
            translate("en", "no-such-message-anywhere", None),
            "no-such-message-anywhere"
        );
    }

    #[test]
    fn the_account_beats_the_cookie_which_beats_the_header() {
        assert_eq!(negotiate(Some("pl"), Some("fr"), Some("de")).tag, "pl");
        assert_eq!(negotiate(None, Some("fr"), Some("de")).tag, "fr");
        assert_eq!(negotiate(None, None, Some("de")).tag, "de");
        assert_eq!(negotiate(None, None, None).tag, "en");
        // Nonsense at any level falls through rather than failing.
        assert_eq!(negotiate(Some("klingon"), Some("fr"), None).tag, "fr");
        assert_eq!(negotiate(None, Some("klingon"), Some("de")).tag, "de");
        assert_eq!(negotiate(None, None, Some("klingon")).tag, "en");
    }

    #[test]
    fn accept_language_honours_quality_and_falls_back_to_the_base_language() {
        assert_eq!(
            negotiate(None, None, Some("de;q=0.3, fr;q=0.9")).tag,
            "fr",
            "the highest quality wins, not the first listed"
        );
        assert_eq!(
            negotiate(None, None, Some("pt-BR,pt;q=0.9")).tag,
            "pt",
            "a regional variant finds its base language"
        );
        assert_eq!(
            negotiate(None, None, Some("zh-CN,zh;q=0.9")).tag,
            "zh-Hans",
            "zh-CN means Simplified, which is what is on offer"
        );
        assert_eq!(
            negotiate(None, None, Some("xx-YY, ar")).tag,
            "ar",
            "an unavailable language is skipped, not fallen back on"
        );
        assert_eq!(negotiate(None, None, Some("*")).tag, "en");
        assert_eq!(negotiate(None, None, Some("")).tag, "en");
    }

    #[test]
    fn arabic_is_the_only_right_to_left_locale() {
        for l in LOCALES {
            assert_eq!(
                l.is_rtl(),
                l.tag == "ar",
                "{} has the wrong direction",
                l.tag
            );
        }
        assert_eq!(Locale::get("ar").dir.as_str(), "rtl");
        assert_eq!(Locale::get("en").dir.as_str(), "ltr");
    }

    #[test]
    fn only_reviewed_locales_claim_to_be_reviewed() {
        // The honest position, asserted rather than described. If somebody
        // flips a flag without a native speaker having read the file, this is
        // what should stop them.
        let reviewed: Vec<&str> = LOCALES
            .iter()
            .filter(|l| l.reviewed)
            .map(|l| l.tag)
            .collect();
        assert_eq!(
            reviewed,
            vec!["en", "fr"],
            "only English and French have been read by someone who speaks them"
        );
    }

    #[test]
    fn every_locale_has_a_native_name_a_speaker_can_find() {
        // A language menu listing only English names is unusable by exactly
        // the people who need it.
        for l in LOCALES {
            assert!(!l.native_name.is_empty());
            if l.tag != "en" {
                assert_ne!(
                    l.native_name, l.english_name,
                    "{} lists its English name as its endonym",
                    l.tag
                );
            }
        }
    }
}
