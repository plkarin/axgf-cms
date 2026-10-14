//! The masthead search: people by name, through the reader's lens.
//!
//! Only a person the reader may open is a candidate at all — the list starts
//! from [`crate::access::Lens::sees_person`], so a hidden person's name can
//! never match, nor be confirmed by a search that comes back empty for it.
//! The grid's client-side filter was made to follow the same rule.

use serde::Serialize;
use serde_json::Value;

use crate::access::Lens;
use crate::text::fold;

/// One person found.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Hit {
    pub id: String,
    pub name: String,
    /// "1823–1901", "c.1798–", or empty: the river's own years line.
    pub years: String,
    /// The river's discharge class, 1–5 (B3): drawn as a small bar so a
    /// founder reads differently from a childless cousin before the reader
    /// chooses. Family shape, which every reader may see (ADR 0001).
    pub class: usize,
}

/// People whose name holds every word of `query`, folded ([`fold`]): names
/// that start with the first word first, then alphabetically. At most
/// `limit`. An empty query finds nobody.
pub fn people(flat: &Value, lens: &Lens, query: &str, limit: usize) -> Vec<Hit> {
    people_counted(flat, lens, query, limit).0
}

/// [`people`], with how many there were in all — the suggestions show eight
/// and say how many matched.
pub fn people_counted(flat: &Value, lens: &Lens, query: &str, limit: usize) -> (Vec<Hit>, usize) {
    let words: Vec<String> = fold(query)
        .split(' ')
        .filter(|w| !w.is_empty())
        .map(String::from)
        .collect();
    if words.is_empty() {
        return (Vec::new(), 0);
    }
    let Some(persons) = flat.get("persons").and_then(Value::as_object) else {
        return (Vec::new(), 0);
    };
    let mut found: Vec<(bool, String, Hit)> = Vec::new();
    for (id, p) in persons {
        if !lens.sees_person(id) {
            continue;
        }
        let name = crate::view::person_display_name(p);
        let folded = fold(&name);
        if !words.iter().all(|w| folded.contains(w.as_str())) {
            continue;
        }
        let starts = folded.starts_with(words[0].as_str())
            || folded
                .split(' ')
                .any(|part| part.starts_with(words[0].as_str()));
        let label = crate::access::river_label(flat, lens, id);
        let years = match (label.birth_year, label.death_year) {
            (None, None) => String::new(),
            (b, d) => format!(
                "{}{}–{}",
                if label.circa { "c." } else { "" },
                b.map(|y| y.to_string()).unwrap_or_default(),
                d.map(|y| y.to_string()).unwrap_or_default()
            ),
        };
        found.push((
            !starts,
            folded,
            Hit {
                id: id.clone(),
                name,
                years,
                class: 1,
            },
        ));
    }
    found.sort_by(|a, b| (a.0, &a.1, &a.2.id).cmp(&(b.0, &b.1, &b.2.id)));
    let total = found.len();
    let mut hits: Vec<Hit> = found.into_iter().take(limit).map(|f| f.2).collect();
    if !hits.is_empty() {
        let g = crate::river::Graph::build(flat);
        for h in &mut hits {
            h.class = g.discharge_class(&h.id).unwrap_or(1);
        }
    }
    (hits, total)
}
