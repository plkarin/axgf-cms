//! Interface strings come from the locale files, not from the templates.
//!
//! # Why a linter rather than a review
//!
//! Translating an application once is easy. Keeping it translated is the hard
//! part, because the next person to add a feature writes `<h2>Sources</h2>`
//! without thinking about it, and nothing complains until somebody reading
//! Polish finds an English heading in the middle of their page. By then the
//! string has been there for months.
//!
//! So this walks the templates and fails on any run of prose that is not
//! inside a `t(...)` call. It is deliberately blunt: markup, entities, numbers
//! and single symbols pass, and everything else has to justify itself.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn templates() -> Vec<PathBuf> {
    let mut out: Vec<PathBuf> = std::fs::read_dir(repo_root().join("templates"))
        .expect("templates/ exists")
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|e| e == "html"))
        .collect();
    out.sort();
    out
}

/// Strip everything that is not literal text a reader would see.
///
/// Template comments, statements and expressions go first, then HTML tags.
/// `{{ t("key") }}` disappears with the expressions, which is the whole point:
/// what is left is what was *not* translated.
///
/// The blanking happens across the whole file rather than line by line,
/// because every one of these constructs can span lines — a `{# … #}` comment
/// explaining a design decision is usually five of them — and a line-by-line
/// pass reports the inside of those comments as untranslated prose. Newlines
/// are preserved so a failure can still name a line.
fn visible_text(src: &str) -> Vec<(usize, String)> {
    // A `{% block …_class %}wide{% endblock %}` carries a CSS class, not
    // prose. It is the one place a bare word legitimately sits between two
    // statements, so it is removed by name rather than by guessing at which
    // single words are safe.
    let mut text = blank_class_blocks(src);
    for (open, close) in [("{#", "#}"), ("{%", "%}"), ("{{", "}}"), ("<", ">")] {
        text = blank_between(&text, open, close);
    }
    text.lines()
        .enumerate()
        .filter_map(|(i, line)| {
            let t = html_unescape(line);
            let t = t.trim();
            (!t.is_empty()).then(|| (i + 1, t.to_string()))
        })
        .collect()
}

/// Blank the body of every `{% block …_class %}` override.
fn blank_class_blocks(src: &str) -> String {
    let mut out = src.to_string();
    while let Some(start) = find_class_block(&out) {
        let Some(end) = out[start..].find("{% endblock %}") else {
            break;
        };
        let body_start = out[start..]
            .find("%}")
            .map(|i| start + i + 2)
            .unwrap_or(start);
        let body_end = start + end;
        let blanked: String = out[body_start..body_end]
            .chars()
            .map(|c| if c == '\n' { c } else { ' ' })
            .collect();
        out.replace_range(body_start..body_end, &blanked);
        // Blank the opening statement too, so the next pass does not see a
        // `block …_class %}` with its braces removed and read it as prose.
        let head: String = out[start..body_start]
            .chars()
            .map(|c| if c == '\n' { c } else { ' ' })
            .collect();
        out.replace_range(start..body_start, &head);
    }
    out
}

fn find_class_block(src: &str) -> Option<usize> {
    let mut from = 0;
    while let Some(i) = src[from..].find("{% block ") {
        let at = from + i;
        let head_end = src[at..].find("%}")? + at;
        if src[at..head_end].trim_end().ends_with("_class") {
            return Some(at);
        }
        from = head_end;
    }
    None
}

/// Replace every `open … close` span with spaces, keeping newlines.
fn blank_between(src: &str, open: &str, close: &str) -> String {
    let mut out: Vec<char> = src.chars().collect();
    let bytes: Vec<char> = out.clone();
    let n = bytes.len();
    let o: Vec<char> = open.chars().collect();
    let c: Vec<char> = close.chars().collect();
    let matches = |at: usize, pat: &[char]| -> bool {
        at + pat.len() <= n && (0..pat.len()).all(|k| bytes[at + k] == pat[k])
    };

    let mut i = 0;
    while i < n {
        if matches(i, &o) {
            // Find the closer; an unterminated construct swallows the rest,
            // which is right — there is no prose after a broken tag.
            let mut j = i + o.len();
            while j < n && !matches(j, &c) {
                j += 1;
            }
            let stop = (j + c.len()).min(n);
            for (k, ch) in out.iter_mut().enumerate().take(stop).skip(i) {
                if bytes[k] != '\n' {
                    *ch = ' ';
                }
            }
            i = stop;
        } else {
            i += 1;
        }
    }
    out.into_iter().collect()
}

fn html_unescape(s: &str) -> String {
    s.replace("&nbsp;", " ")
        .replace("&mdash;", "—")
        .replace("&ndash;", "–")
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#8203;", "")
}

/// Whether a fragment is prose a reader would notice being in the wrong
/// language.
///
/// Punctuation, separators, single letters and bare numbers are not. Two or
/// more letters in a row, forming a word, are.
fn is_prose(text: &str) -> bool {
    let words: Vec<&str> = text
        .split_whitespace()
        .filter(|w| w.chars().filter(|c| c.is_alphabetic()).count() >= 2)
        .collect();
    if words.is_empty() {
        return false;
    }
    // Things that are the same in every language, or are not words at all.
    const ALLOWED: &[&str] = &[
        "axgf",
        "axgf-cms",
        "axgf-rs",
        "axgf-spec",
        "GEDCOM",
        "AXGF",
        "JSON",
        "MB",
        "KB",
        "px",
        "SVG",
        "PDF",
        "UUID",
        "ZIP",
        "URL",
        "id",
        "ids",
        "SHA-256",
        "Argon2id",
        "GPG",
        "CLDR",
        "ISO",
        "RFC",
        "HTML",
        "CSS",
        // The wordmark, which is a product name and not translated.
        "cms",
    ];
    words.iter().any(|w| {
        let bare: String = w
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_' || *c == '.')
            .collect();
        let bare = bare.trim_matches(|c: char| c == '.' || c == '-' || c == '_');
        !bare.is_empty()
            && !ALLOWED
                .iter()
                .any(|a| a.eq_ignore_ascii_case(bare) || bare.starts_with(a))
    })
}

#[test]
fn no_template_carries_a_hardcoded_english_string() {
    let mut offences: Vec<String> = Vec::new();
    for path in templates() {
        let src = std::fs::read_to_string(&path).expect("read template");
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        for (line, text) in visible_text(&src) {
            if is_prose(&text) {
                offences.push(format!("  {name}:{line}  {text}"));
            }
        }
    }
    assert!(
        offences.is_empty(),
        "{} literal string(s) in templates. Every user-visible string must \
         come from a locale file, or a reader who switched the interface to \
         Polish will find English in the middle of their page:\n{}",
        offences.len(),
        offences.join("\n")
    );
}

/// Message ids defined in one `.ftl` source.
fn ids_of(path: &Path) -> BTreeSet<String> {
    let src = std::fs::read_to_string(path).expect("read locale");
    axgf_cms::i18n::message_ids(&src).into_iter().collect()
}

#[test]
fn every_key_a_template_asks_for_exists_in_english() {
    // English is the fallback for every other locale, so a key missing *there*
    // renders as the key itself on every page in every language.
    let english = ids_of(&repo_root().join("locales/en.ftl"));
    let mut missing: Vec<String> = Vec::new();

    for path in templates() {
        let src = std::fs::read_to_string(&path).expect("read template");
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        for (line_no, line) in src.lines().enumerate() {
            for key in keys_in(line) {
                if !english.contains(&key) {
                    missing.push(format!("  {name}:{}  {key}", line_no + 1));
                }
            }
        }
    }
    assert!(
        missing.is_empty(),
        "{} key(s) asked for by a template and defined nowhere:\n{}",
        missing.len(),
        missing.join("\n")
    );
}

/// Every `t("…")` / `t('…')` key on a line.
fn keys_in(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    let bytes = line.as_bytes();
    let mut i = 0;
    while let Some(at) = line[i..].find("t(") {
        let start = i + at;
        // `t(` must not be the tail of a longer identifier — `format(` and
        // `int(` would otherwise match.
        let preceded_ok = start == 0 || {
            let c = bytes[start - 1] as char;
            !(c.is_alphanumeric() || c == '_' || c == '.')
        };
        i = start + 2;
        if !preceded_ok {
            continue;
        }
        let rest = &line[i..];
        let quote = match rest.chars().next() {
            Some(q @ ('"' | '\'')) => q,
            _ => continue,
        };
        let after = &rest[1..];
        if let Some(end) = after.find(quote) {
            out.push(after[..end].to_string());
        }
    }
    out
}

#[test]
fn no_locale_defines_a_key_english_does_not() {
    // A key only a translation defines is dead weight: nothing renders it, and
    // it inflates the coverage number that is supposed to be honest.
    let english = ids_of(&repo_root().join("locales/en.ftl"));
    for locale in axgf_cms::i18n::LOCALES {
        if locale.tag == "en" {
            continue;
        }
        let mine = ids_of(&repo_root().join(format!("locales/{}.ftl", locale.tag)));
        let extra: Vec<&String> = mine.difference(&english).collect();
        assert!(
            extra.is_empty(),
            "{} defines {:?}, which English does not",
            locale.tag,
            extra
        );
    }
}

#[test]
fn the_coverage_number_the_selector_shows_is_the_real_one() {
    // The whole honesty claim rests on this number, so it is checked against
    // the files rather than trusted.
    let english = ids_of(&repo_root().join("locales/en.ftl"));
    for locale in axgf_cms::i18n::LOCALES {
        let mine = ids_of(&repo_root().join(format!("locales/{}.ftl", locale.tag)));
        let shared = mine.intersection(&english).count();
        let expected = ((shared as f64 / english.len() as f64) * 100.0).round() as u32;
        assert_eq!(
            locale.coverage_percent(),
            expected,
            "{} reports the wrong coverage",
            locale.tag
        );
        if locale.reviewed {
            assert_eq!(
                shared,
                english.len(),
                "{} claims to be reviewed but is not complete",
                locale.tag
            );
        }
    }
}
