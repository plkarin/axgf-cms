//! The guide to keeping the family's archive through the AFG (docs/adr/0045
//! §6), on every member's personal assistant: its chapters, each with
//! requests to make, in the reader's language.
//!
//! Each language's guide is a Markdown file, `locales/guide/<lang>.md`: a
//! lead, then one `## ` heading a chapter, the requests written as quotes.
//! Embedded at compile time like the catalogues, and drawn once a language.

use std::collections::HashMap;
use std::sync::OnceLock;

use serde::Serialize;

/// Every language's guide.
const GUIDES: &[(&str, &str)] = &[
    ("en", include_str!("../locales/guide/en.md")),
    ("fr", include_str!("../locales/guide/fr.md")),
    ("es", include_str!("../locales/guide/es.md")),
    ("de", include_str!("../locales/guide/de.md")),
    ("it", include_str!("../locales/guide/it.md")),
    ("pt", include_str!("../locales/guide/pt.md")),
    ("pl", include_str!("../locales/guide/pl.md")),
    ("ru", include_str!("../locales/guide/ru.md")),
    ("he", include_str!("../locales/guide/he.md")),
    ("ar", include_str!("../locales/guide/ar.md")),
    ("ja", include_str!("../locales/guide/ja.md")),
    ("zh-Hans", include_str!("../locales/guide/zh-Hans.md")),
    ("sw", include_str!("../locales/guide/sw.md")),
    ("ln", include_str!("../locales/guide/ln.md")),
];

/// One chapter, drawn.
#[derive(Debug, Clone, Serialize)]
pub struct Chapter {
    /// `guide-1`, for a link.
    pub id: String,
    pub title: String,
    pub html: String,
}

/// A language's guide, drawn.
#[derive(Debug, Clone, Serialize)]
pub struct Guide {
    pub lead: String,
    pub chapters: Vec<Chapter>,
}

/// The guide's Markdown in `lang`, or in English when it has none.
pub fn source(lang: &str) -> &'static str {
    GUIDES
        .iter()
        .find(|(l, _)| *l == lang)
        .or_else(|| GUIDES.first())
        .map(|(_, s)| *s)
        .unwrap_or_default()
}

/// The languages the guide is written in.
pub fn languages() -> impl Iterator<Item = &'static str> {
    GUIDES.iter().map(|(l, _)| *l)
}

/// A guide's Markdown cut into its lead and its chapters: (title, body).
pub fn split(src: &str) -> (&str, Vec<(&str, &str)>) {
    let mut parts = src.split("\n## ");
    let lead = parts.next().unwrap_or_default();
    let chapters = parts
        .map(|c| match c.split_once('\n') {
            Some((title, body)) => (title.trim(), body),
            None => (c.trim(), ""),
        })
        .collect();
    (lead, chapters)
}

/// The guide in `lang`, drawn.
pub fn guide(lang: &str) -> Guide {
    static DRAWN: OnceLock<std::sync::Mutex<HashMap<String, Guide>>> = OnceLock::new();
    let drawn = DRAWN.get_or_init(Default::default);
    if let Some(g) = drawn.lock().ok().and_then(|d| d.get(lang).cloned()) {
        return g;
    }
    let (lead, chapters) = split(source(lang));
    let g = Guide {
        lead: crate::notes::to_html(lead.trim(), &|_| None),
        chapters: chapters
            .into_iter()
            .enumerate()
            .map(|(i, (title, body))| Chapter {
                id: format!("guide-{}", i + 1),
                title: title.to_string(),
                html: crate::notes::to_html(body.trim(), &|_| None),
            })
            .collect(),
    };
    if let Ok(mut d) = drawn.lock() {
        d.insert(lang.to_string(), g.clone());
    }
    g
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_guide_has_the_same_chapters_each_with_requests() {
        let (_, english) = split(source("en"));
        assert!(english.len() >= 10, "{} chapters", english.len());
        for lang in languages() {
            let (lead, chapters) = split(source(lang));
            assert!(!lead.trim().is_empty(), "{lang}: a lead");
            assert_eq!(
                chapters.len(),
                english.len(),
                "{lang}: as many chapters as English"
            );
            for (title, body) in &chapters {
                assert!(
                    body.lines().any(|l| l.starts_with("> ")),
                    "{lang}: «{title}» gives a request to make"
                );
            }
        }
    }

    #[test]
    fn a_language_without_a_guide_reads_the_english_one() {
        assert_eq!(source("xx"), source("en"));
        let g = guide("fr");
        assert!(g.chapters[0].html.contains("<ul>") || g.chapters[0].html.contains("<li>"));
        assert_eq!(g.chapters[0].id, "guide-1");
    }
}
