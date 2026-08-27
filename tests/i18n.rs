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
    // A command, a filename or a field path is the same in every language.
    // `<code>` and `<pre>` mark exactly that, so their contents are not prose
    // — while the sentence around them still is, and is still checked.
    for (open, close) in [("<code>", "</code>"), ("<pre", "</pre>")] {
        text = blank_between(&text, open, close);
    }
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
            // `t("kind-" ~ kind)` builds its key at render time, so the
            // literal here is a prefix and not a message id. Those families
            // are checked by expansion in
            // `every_dynamic_key_family_is_fully_defined` instead.
            let tail = after[end + 1..].trim_start();
            if !tail.starts_with('~') {
                out.push(after[..end].to_string());
            }
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

#[test]
fn every_dynamic_key_family_is_fully_defined() {
    // A template that builds its key — `t("kind-" ~ kind)` — cannot be checked
    // by reading the source, so the families are enumerated here. The cost of
    // this list is that it has to be kept in step; the cost of not having it
    // is a page that renders `kind-family` to the reader.
    let english = ids_of(&repo_root().join("locales/en.ftl"));
    let mut expected: Vec<String> = Vec::new();

    for kind in [
        "person",
        "family",
        "event",
        "link",
        "occupation",
        "source",
        "place",
        "document",
    ] {
        expected.push(format!("kind-{kind}"));
        expected.push(format!("kind-{kind}-plural"));
    }
    for shape in ["exact", "approximate", "ranged", "preserved", "unknown"] {
        expected.push(format!("completeness-shape-{shape}"));
        expected.push(format!("completeness-shape-{shape}-note"));
    }
    for role in ["viewer", "contributor", "admin"] {
        expected.push(format!("accounts-role-{role}"));
    }
    for theme in axgf_cms::theme::THEMES {
        expected.push(theme.key.to_string());
        if let Some(note) = theme.note_key {
            expected.push(note.to_string());
        }
    }
    let missing: Vec<&String> = expected.iter().filter(|k| !english.contains(*k)).collect();
    assert!(missing.is_empty(), "English is missing {missing:?}");
}

#[test]
fn no_template_passes_a_literal_sentence_into_an_expression() {
    // The first version of this file only looked at text *between* tags, and
    // so missed `{{ m.sec("Sources and documents", …) }}` — seven section
    // headings and their help paragraphs, sitting in plain sight inside an
    // expression. Anything a template hands to a macro is just as visible to a
    // reader as anything it prints directly.
    let mut offences: Vec<String> = Vec::new();
    for path in templates() {
        let src = std::fs::read_to_string(&path).expect("read template");
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        // Line numbers survive because the comment blanking keeps newlines.
        let scrubbed = blank_between(&src, "{#", "#}");
        for (i, line) in scrubbed.lines().enumerate() {
            for literal in string_literals(line) {
                if is_sentence(&literal) && !keys_in(line).contains(&literal) {
                    offences.push(format!("  {name}:{}  {literal:?}", i + 1));
                }
            }
        }
    }
    assert!(
        offences.is_empty(),
        "{} literal sentence(s) passed into a template expression. A string \
         handed to a macro reaches the reader exactly like one printed \
         directly, so it has to come from a locale file too:\n{}",
        offences.len(),
        offences.join("\n")
    );
}

/// Whether a literal inside an expression is a sentence rather than an
/// identifier.
///
/// Stricter than [`is_prose`], because an expression legitimately contains
/// template names, message-key prefixes, filter arguments and comparison
/// values — `"base.html"`, `"kind-"`, `"eq"`, `"preserved"`. What none of
/// those are is *two words*, so that is the line: two or more runs of at least
/// two letters, separated by space.
fn is_sentence(text: &str) -> bool {
    text.split_whitespace()
        .filter(|w| w.chars().filter(|c| c.is_alphabetic()).count() >= 2)
        .count()
        >= 2
        && is_prose(text)
}

/// Every quoted string inside a `{{ … }}` or `{% … %}` on this line.
///
/// Attribute values are *not* included: this reads only what is inside
/// template constructs, because `class="lede"` is markup and `"Identity"` in
/// an expression is a sentence.
fn string_literals(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    for (open, close) in [("{{", "}}"), ("{%", "%}")] {
        let mut from = 0;
        while let Some(i) = line[from..].find(open) {
            let start = from + i + open.len();
            let end = line[start..]
                .find(close)
                .map(|j| start + j)
                .unwrap_or(line.len());
            let expr = &line[start..end];
            let mut rest = expr;
            while let Some(q) = rest.find(['"', '\'']) {
                let quote = rest.as_bytes()[q] as char;
                let after = &rest[q + 1..];
                match after.find(quote) {
                    Some(e) => {
                        out.push(after[..e].to_string());
                        rest = &after[e + 1..];
                    }
                    None => break,
                }
            }
            from = end;
        }
    }
    out
}

#[test]
fn no_template_carries_a_hardcoded_attribute() {
    // The third blind spot, after text between tags and literals inside
    // expressions: `title="This person's record is not visible to you"` is
    // read aloud by a screen reader and shown on hover, and was sitting in
    // English while everything around it was translated. Attributes that a
    // reader perceives are checked; `class` and `href` are not.
    const PERCEIVED: [&str; 4] = ["title", "aria-label", "placeholder", "alt"];
    let mut offences: Vec<String> = Vec::new();

    for path in templates() {
        let src = std::fs::read_to_string(&path).expect("read template");
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        let scrubbed = blank_between(&src, "{#", "#}");
        for (i, line) in scrubbed.lines().enumerate() {
            for attr in PERCEIVED {
                for value in attribute_values(line, attr) {
                    // Whatever the value builds from expressions and
                    // statements is already going through the catalogue; only
                    // what is left as literal text is a hardcoded string.
                    let bare = blank_between(&value, "{{", "}}");
                    let bare = blank_between(&bare, "{%", "%}");
                    if is_prose(bare.trim()) {
                        offences.push(format!("  {name}:{}  {attr}={value:?}", i + 1));
                    }
                }
            }
        }
    }
    assert!(
        offences.is_empty(),
        "{} hardcoded attribute(s). A title or an aria-label is text a reader \
         hears or hovers, so it comes from a locale file too:\n{}",
        offences.len(),
        offences.join("\n")
    );
}

/// Every `attr="…"` value on this line.
fn attribute_values(line: &str, attr: &str) -> Vec<String> {
    let needle = format!("{attr}=\"");
    let mut out = Vec::new();
    let mut from = 0;
    while let Some(i) = line[from..].find(&needle) {
        let at = from + i;
        // Not the tail of another attribute name: `data-title=` is not
        // `title=`.
        let ok = at == 0 || {
            let c = line.as_bytes()[at - 1] as char;
            c.is_whitespace()
        };
        let start = at + needle.len();
        match line[start..].find('"') {
            Some(j) => {
                if ok {
                    out.push(line[start..start + j].to_string());
                }
                from = start + j + 1;
            }
            None => break,
        }
    }
    out
}
