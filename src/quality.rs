//! The data-quality screen's detectors.
//!
//! Every family that imports its own GEDCOM arrives with its own mess: an
//! archivist's sentence in a given-name field, a couple entered twice, a
//! child born before their mother, a "place" that is a description of a
//! house. The site used to display all of it without a word. Each detector
//! here finds one kind of thing worth a look, as a named rule ([`Rule`]) that
//! runs on its own ([`run`]) and is tested on its own.
//!
//! Three things are deliberately *not* here:
//!
//! * **Fixing.** Nothing is ever changed. A finding links to the record's
//!   own editor, so every correction is a person's decision, one record at a
//!   time, through the edit path that journals it and locks it.
//! * **Seeing.** Findings are computed over the whole bundle and then
//!   filtered by [`for_reader`]: a finding involving anyone the reader may
//!   not see is dropped entirely — not shown with the name blanked, not
//!   counted — so a reader learns nothing about a hidden person here, not
//!   even that their record has a problem.
//! * **Words.** A finding carries its rule and its arguments; the sentence is
//!   the catalogue's, in the reader's language.

use std::collections::{BTreeMap, BTreeSet, HashMap};

use serde::Serialize;
use serde_json::Value;

use crate::access::Lens;
use crate::text::fold;

/// One kind of thing worth a look.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Rule {
    // Names
    /// A given name or surname that is a sentence rather than a name: longer
    /// than [`NAME_MAX_CHARS`] characters, containing " - ", or more than
    /// [`NAME_MAX_WORDS`] words.
    NameSentence,
    // Relations
    /// A union whose two partners are also parent and child.
    UnionParentChild,
    /// The same person listed twice among one family's children.
    ChildListedTwice,
    /// Two unions recorded between the same two people.
    DuplicateUnion,
    /// One person on both sides of the same union.
    PartnerTwice,
    /// Children carrying surnames that match neither parent's, at least
    /// [`FOREIGN_MIN`] of them and at least a quarter of the family.
    ForeignSurnames,
    // Dates
    /// Death before birth.
    DeathBeforeBirth,
    /// A child born before a parent.
    ChildBeforeParent,
    /// A child born before the parent turned [`PARENT_MIN_AGE`].
    ParentTooYoung,
    /// A child born after the parent turned [`PARENT_MAX_AGE`].
    ParentTooOld,
    /// A marriage dated before one partner's birth.
    MarriageBeforeBirth,
    /// A person with no date at all and no relation to anyone.
    IsolatedRecord,
    // Places
    /// A place with no region recorded.
    PlaceNoRegion,
    /// A region that holds free text rather than a place ([`free_text_region`]).
    RegionFreeText,
    /// Places whose names are the same once accents, case and a trailing
    /// "?" or "." are set aside ("Łodź", "Łódź").
    PlaceNearDuplicate,
    /// An entry in the places list that is not a place: its name begins with
    /// a lowercase letter, as a description does and a place name does not.
    NotAPlace,
    // Evidence
    /// A claim at confidence 1.0 with no source attached.
    CertainWithoutSource,
}

pub const NAME_MAX_CHARS: usize = 30;
pub const NAME_MAX_WORDS: usize = 3;
pub const PARENT_MIN_AGE: i64 = 12;
pub const PARENT_MAX_AGE: i64 = 65;
pub const FOREIGN_MIN: usize = 2;

impl Rule {
    pub const ALL: [Rule; 17] = [
        Rule::NameSentence,
        Rule::UnionParentChild,
        Rule::ChildListedTwice,
        Rule::DuplicateUnion,
        Rule::PartnerTwice,
        Rule::ForeignSurnames,
        Rule::DeathBeforeBirth,
        Rule::ChildBeforeParent,
        Rule::ParentTooYoung,
        Rule::ParentTooOld,
        Rule::MarriageBeforeBirth,
        Rule::IsolatedRecord,
        Rule::PlaceNoRegion,
        Rule::RegionFreeText,
        Rule::PlaceNearDuplicate,
        Rule::NotAPlace,
        Rule::CertainWithoutSource,
    ];

    /// The rule's name in URLs, exports and the catalogue.
    pub fn slug(self) -> &'static str {
        match self {
            Rule::NameSentence => "name-sentence",
            Rule::UnionParentChild => "union-parent-child",
            Rule::ChildListedTwice => "child-listed-twice",
            Rule::DuplicateUnion => "duplicate-union",
            Rule::PartnerTwice => "partner-twice",
            Rule::ForeignSurnames => "foreign-surnames",
            Rule::DeathBeforeBirth => "death-before-birth",
            Rule::ChildBeforeParent => "child-before-parent",
            Rule::ParentTooYoung => "parent-too-young",
            Rule::ParentTooOld => "parent-too-old",
            Rule::MarriageBeforeBirth => "marriage-before-birth",
            Rule::IsolatedRecord => "isolated-record",
            Rule::PlaceNoRegion => "place-no-region",
            Rule::RegionFreeText => "region-free-text",
            Rule::PlaceNearDuplicate => "place-near-duplicate",
            Rule::NotAPlace => "not-a-place",
            Rule::CertainWithoutSource => "certain-without-source",
        }
    }

    /// Which of the five groups the screen files it under.
    pub fn group(self) -> &'static str {
        match self {
            Rule::NameSentence => "names",
            Rule::UnionParentChild
            | Rule::ChildListedTwice
            | Rule::DuplicateUnion
            | Rule::PartnerTwice
            | Rule::ForeignSurnames => "relations",
            Rule::DeathBeforeBirth
            | Rule::ChildBeforeParent
            | Rule::ParentTooYoung
            | Rule::ParentTooOld
            | Rule::MarriageBeforeBirth
            | Rule::IsolatedRecord => "dates",
            Rule::PlaceNoRegion
            | Rule::RegionFreeText
            | Rule::PlaceNearDuplicate
            | Rule::NotAPlace => "places",
            Rule::CertainWithoutSource => "evidence",
        }
    }
}

pub const GROUPS: [&str; 5] = ["names", "relations", "dates", "places", "evidence"];

/// One thing worth a look.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Finding {
    pub rule: Rule,
    /// The record to open: its kind and id.
    pub kind: &'static str,
    pub id: String,
    /// Everyone the finding is about. A reader must be able to see every one
    /// of them to be shown it at all.
    #[serde(skip)]
    pub persons: Vec<String>,
    /// What the sentence names: people's names, years, values.
    pub args: BTreeMap<String, String>,
}

/// Every rule over the whole bundle, in [`Rule::ALL`] order. Nothing is
/// filtered for a reader here: see [`for_reader`].
pub fn detect(flat: &Value) -> Vec<Finding> {
    let b = Bundle::read(flat);
    Rule::ALL.iter().flat_map(|r| run_on(&b, *r)).collect()
}

/// One rule over the whole bundle.
pub fn run(flat: &Value, rule: Rule) -> Vec<Finding> {
    run_on(&Bundle::read(flat), rule)
}

/// What this reader may be shown: only findings about people they may see,
/// every one of them.
pub fn for_reader(flat: &Value, lens: &Lens) -> Vec<Finding> {
    detect(flat)
        .into_iter()
        .filter(|f| f.persons.iter().all(|p| lens.sees_person(p)))
        .collect()
}

fn run_on(b: &Bundle, rule: Rule) -> Vec<Finding> {
    match rule {
        Rule::NameSentence => name_sentence(b),
        Rule::UnionParentChild => union_parent_child(b),
        Rule::ChildListedTwice => child_listed_twice(b),
        Rule::DuplicateUnion => duplicate_union(b),
        Rule::PartnerTwice => partner_twice(b),
        Rule::ForeignSurnames => foreign_surnames(b),
        Rule::DeathBeforeBirth => death_before_birth(b),
        Rule::ChildBeforeParent | Rule::ParentTooYoung | Rule::ParentTooOld => parent_ages(b, rule),
        Rule::MarriageBeforeBirth => marriage_before_birth(b),
        Rule::IsolatedRecord => isolated_record(b),
        Rule::PlaceNoRegion => place_no_region(b),
        Rule::RegionFreeText => region_free_text(b),
        Rule::PlaceNearDuplicate => place_near_duplicate(b),
        Rule::NotAPlace => not_a_place(b),
        Rule::CertainWithoutSource => certain_without_source(b),
    }
}

// ---------------------------------------------------------------- reading

/// A date as the record gives it: a year, and a month and day when it is
/// exact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Day {
    year: i64,
    month: Option<u8>,
    day: Option<u8>,
}

impl Day {
    fn of(date: Option<&Value>) -> Option<Day> {
        let v = date?.get("value")?.as_str()?.trim();
        let mut parts = v.split('-');
        let year: i64 = parts.next()?.parse().ok()?;
        let month = parts.next().and_then(|m| m.parse().ok());
        let day = parts.next().and_then(|d| d.parse().ok());
        Some(Day { year, month, day })
    }

    /// Strictly before `other`, as far as both dates say: by the year, and
    /// within one year only when both carry a full date.
    fn before(self, other: Day) -> bool {
        if self.year != other.year {
            return self.year < other.year;
        }
        match (self.month, self.day, other.month, other.day) {
            (Some(m1), Some(d1), Some(m2), Some(d2)) => (m1, d1) < (m2, d2),
            _ => false,
        }
    }

    /// Whole years from `self` to `later`, counting a birthday only when both
    /// dates say on which day it falls.
    fn years_to(self, later: Day) -> i64 {
        let mut y = later.year - self.year;
        if let (Some(m1), Some(d1), Some(m2), Some(d2)) =
            (self.month, self.day, later.month, later.day)
        {
            if (m2, d2) < (m1, d1) {
                y -= 1;
            }
        }
        y
    }
}

struct Person<'a> {
    id: &'a str,
    name: String,
    given: Vec<String>,
    surname: Vec<String>,
    birth: Option<Day>,
    death: Option<Day>,
}

struct Family<'a> {
    id: &'a str,
    partners: Vec<&'a str>,
    children: Vec<&'a str>,
    start: Option<Day>,
}

struct Place<'a> {
    id: &'a str,
    name: String,
    region: Option<String>,
}

struct Bundle<'a> {
    flat: &'a Value,
    persons: Vec<Person<'a>>,
    by_id: HashMap<&'a str, usize>,
    families: Vec<Family<'a>>,
    places: Vec<Place<'a>>,
}

fn components<'a>(p: &'a Value, kind: &str) -> Vec<String> {
    p.get("identity")
        .and_then(|i| i.get("name"))
        .and_then(|n| n.get("components"))
        .and_then(Value::as_array)
        .map(|cs| {
            cs.iter()
                .filter(|c| c.get("type").and_then(Value::as_str) == Some(kind))
                .filter_map(|c| c.get("value").and_then(Value::as_str))
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        })
        .unwrap_or_default()
}

fn entities<'a>(flat: &'a Value, kind: &str) -> Vec<(&'a str, &'a Value)> {
    let mut v: Vec<(&str, &Value)> = flat
        .get(kind)
        .and_then(Value::as_object)
        .map(|m| m.iter().map(|(k, v)| (k.as_str(), v)).collect())
        .unwrap_or_default();
    v.sort_by_key(|(k, _)| *k);
    v
}

impl<'a> Bundle<'a> {
    fn read(flat: &'a Value) -> Self {
        let persons: Vec<Person> = entities(flat, "persons")
            .into_iter()
            .map(|(id, p)| Person {
                id,
                name: crate::view::person_display_name(p),
                given: components(p, "given_name"),
                surname: components(p, "family_name"),
                birth: Day::of(p.get("birth").and_then(|b| b.get("date"))),
                death: Day::of(p.get("death").and_then(|b| b.get("date"))),
            })
            .collect();
        let by_id = persons.iter().enumerate().map(|(i, p)| (p.id, i)).collect();
        let ids = |list: Option<&'a Value>| -> Vec<&'a str> {
            list.and_then(Value::as_array)
                .map(|a| {
                    a.iter()
                        .filter_map(|x| x.get("person_id").and_then(Value::as_str))
                        .collect()
                })
                .unwrap_or_default()
        };
        let families = entities(flat, "families")
            .into_iter()
            .map(|(id, f)| Family {
                id,
                partners: ids(f.get("union").and_then(|u| u.get("persons"))),
                children: ids(f.get("children")),
                start: Day::of(
                    f.get("union")
                        .and_then(|u| u.get("start"))
                        .and_then(|s| s.get("date")),
                ),
            })
            .collect();
        let places = entities(flat, "places")
            .into_iter()
            .map(|(id, p)| {
                let names = p.get("names").and_then(Value::as_array);
                let name = names
                    .and_then(|n| {
                        n.iter()
                            .find(|x| x.get("is_primary").and_then(Value::as_bool) == Some(true))
                            .or_else(|| n.first())
                    })
                    .and_then(|x| x.get("value"))
                    .or_else(|| p.get("name"))
                    .and_then(Value::as_str)
                    .unwrap_or("")
                    .trim()
                    .to_string();
                let region = p
                    .get("region")
                    .or_else(|| p.get("hierarchy").and_then(|h| h.get("region")))
                    .and_then(Value::as_str)
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .map(str::to_string);
                Place { id, name, region }
            })
            .collect();
        Bundle {
            flat,
            persons,
            by_id,
            families,
            places,
        }
    }

    fn person(&self, id: &str) -> Option<&Person<'a>> {
        self.by_id.get(id).map(|&i| &self.persons[i])
    }

    fn name(&self, id: &str) -> String {
        self.person(id).map(|p| p.name.clone()).unwrap_or_default()
    }
}

fn finding(
    rule: Rule,
    kind: &'static str,
    id: &str,
    persons: Vec<String>,
    args: &[(&str, String)],
) -> Finding {
    Finding {
        rule,
        kind,
        id: id.to_string(),
        persons,
        args: args
            .iter()
            .map(|(k, v)| (k.to_string(), v.clone()))
            .collect(),
    }
}

// --------------------------------------------------------------- the rules

fn name_sentence(b: &Bundle) -> Vec<Finding> {
    let mut out = Vec::new();
    for p in &b.persons {
        for value in p.given.iter().chain(&p.surname) {
            let words = value.split_whitespace().count();
            if value.chars().count() > NAME_MAX_CHARS
                || value.contains(" - ")
                || words > NAME_MAX_WORDS
            {
                out.push(finding(
                    Rule::NameSentence,
                    "person",
                    p.id,
                    vec![p.id.to_string()],
                    &[("value", value.clone())],
                ));
            }
        }
    }
    out
}

fn union_parent_child(b: &Bundle) -> Vec<Finding> {
    // Who is a parent of whom, by every family that records it.
    let mut parent_of: BTreeSet<(&str, &str)> = BTreeSet::new();
    for f in &b.families {
        for &p in &f.partners {
            for &c in &f.children {
                parent_of.insert((p, c));
            }
        }
    }
    let mut out = Vec::new();
    for f in &b.families {
        let [a, c] = f.partners[..] else { continue };
        if a == c {
            continue;
        }
        let (parent, child) = if parent_of.contains(&(a, c)) {
            (a, c)
        } else if parent_of.contains(&(c, a)) {
            (c, a)
        } else {
            continue;
        };
        out.push(finding(
            Rule::UnionParentChild,
            "family",
            f.id,
            vec![parent.to_string(), child.to_string()],
            &[("parent", b.name(parent)), ("child", b.name(child))],
        ));
    }
    out
}

fn child_listed_twice(b: &Bundle) -> Vec<Finding> {
    let mut out = Vec::new();
    for f in &b.families {
        let mut seen = BTreeSet::new();
        let mut twice = BTreeSet::new();
        for &c in &f.children {
            if !seen.insert(c) {
                twice.insert(c);
            }
        }
        for c in twice {
            let mut persons: Vec<String> = f.partners.iter().map(|s| s.to_string()).collect();
            persons.push(c.to_string());
            out.push(finding(
                Rule::ChildListedTwice,
                "family",
                f.id,
                persons,
                &[("child", b.name(c))],
            ));
        }
    }
    out
}

fn duplicate_union(b: &Bundle) -> Vec<Finding> {
    let mut pairs: BTreeMap<(&str, &str), Vec<&str>> = BTreeMap::new();
    for f in &b.families {
        let [x, y] = f.partners[..] else { continue };
        if x == y {
            continue;
        }
        let key = if x < y { (x, y) } else { (y, x) };
        pairs.entry(key).or_default().push(f.id);
    }
    pairs
        .into_iter()
        .filter(|(_, fams)| fams.len() > 1)
        .map(|((x, y), fams)| {
            finding(
                Rule::DuplicateUnion,
                "family",
                fams[1],
                vec![x.to_string(), y.to_string()],
                &[
                    ("a", b.name(x)),
                    ("b", b.name(y)),
                    ("count", fams.len().to_string()),
                ],
            )
        })
        .collect()
}

fn partner_twice(b: &Bundle) -> Vec<Finding> {
    let mut out = Vec::new();
    for f in &b.families {
        let mut seen = BTreeSet::new();
        for &p in &f.partners {
            if !seen.insert(p) {
                out.push(finding(
                    Rule::PartnerTwice,
                    "family",
                    f.id,
                    vec![p.to_string()],
                    &[("person", b.name(p))],
                ));
            }
        }
    }
    out
}

/// A surname reduced to what a Polish family shares across genders:
/// Klicki/Klicka, Gutowski/Gutowska, Dzierżyński/Dzierżyńska.
fn surname_stem(s: &str) -> String {
    let f = fold(s);
    for (suffix, stem) in [
        ("ska", "sk"),
        ("ski", "sk"),
        ("cka", "ck"),
        ("cki", "ck"),
        ("dzka", "dzk"),
        ("dzki", "dzk"),
    ] {
        if let Some(base) = f.strip_suffix(suffix) {
            return format!("{base}{stem}");
        }
    }
    f
}

fn foreign_surnames(b: &Bundle) -> Vec<Finding> {
    let mut out = Vec::new();
    for f in &b.families {
        let parents: BTreeSet<String> = f
            .partners
            .iter()
            .filter_map(|p| b.person(p))
            .flat_map(|p| p.surname.iter().map(|s| surname_stem(s)))
            .collect();
        let kids: Vec<&Person> = f.children.iter().filter_map(|c| b.person(c)).collect();
        if parents.is_empty() || kids.len() < FOREIGN_MIN {
            continue;
        }
        let foreign: Vec<&&Person> = kids
            .iter()
            .filter(|k| {
                !k.surname.is_empty()
                    && !k.surname.iter().any(|s| parents.contains(&surname_stem(s)))
            })
            .collect();
        if foreign.len() >= FOREIGN_MIN && foreign.len() * 4 >= kids.len() {
            let names: BTreeSet<String> = foreign
                .iter()
                .flat_map(|k| k.surname.iter().cloned())
                .collect();
            let mut persons: Vec<String> = f.partners.iter().map(|s| s.to_string()).collect();
            persons.extend(kids.iter().map(|k| k.id.to_string()));
            out.push(finding(
                Rule::ForeignSurnames,
                "family",
                f.id,
                persons,
                &[
                    ("count", foreign.len().to_string()),
                    ("children", kids.len().to_string()),
                    ("surnames", names.into_iter().collect::<Vec<_>>().join(", ")),
                ],
            ));
        }
    }
    out
}

fn death_before_birth(b: &Bundle) -> Vec<Finding> {
    b.persons
        .iter()
        .filter_map(|p| {
            let (birth, death) = (p.birth?, p.death?);
            death.before(birth).then(|| {
                finding(
                    Rule::DeathBeforeBirth,
                    "person",
                    p.id,
                    vec![p.id.to_string()],
                    &[
                        ("person", p.name.clone()),
                        ("birth", birth.year.to_string()),
                        ("death", death.year.to_string()),
                    ],
                )
            })
        })
        .collect()
}

fn parent_ages(b: &Bundle, rule: Rule) -> Vec<Finding> {
    let mut out = Vec::new();
    for f in &b.families {
        for &c in &f.children {
            let Some(child) = b.person(c) else { continue };
            let Some(born) = child.birth else { continue };
            for &p in &f.partners {
                let Some(parent) = b.person(p) else { continue };
                let Some(pborn) = parent.birth else { continue };
                let which = if born.before(pborn) {
                    Rule::ChildBeforeParent
                } else {
                    let age = pborn.years_to(born);
                    if age < PARENT_MIN_AGE {
                        Rule::ParentTooYoung
                    } else if age > PARENT_MAX_AGE {
                        Rule::ParentTooOld
                    } else {
                        continue;
                    }
                };
                if which != rule {
                    continue;
                }
                out.push(finding(
                    rule,
                    "family",
                    f.id,
                    vec![p.to_string(), c.to_string()],
                    &[
                        ("parent", parent.name.clone()),
                        ("child", child.name.clone()),
                        ("parentborn", pborn.year.to_string()),
                        ("childborn", born.year.to_string()),
                        ("age", pborn.years_to(born).to_string()),
                    ],
                ));
            }
        }
    }
    out
}

fn marriage_before_birth(b: &Bundle) -> Vec<Finding> {
    let mut out = Vec::new();
    for f in &b.families {
        let Some(married) = f.start else { continue };
        for &p in &f.partners {
            let Some(person) = b.person(p) else { continue };
            let Some(born) = person.birth else { continue };
            if married.before(born) {
                out.push(finding(
                    Rule::MarriageBeforeBirth,
                    "family",
                    f.id,
                    f.partners.iter().map(|s| s.to_string()).collect(),
                    &[
                        ("person", person.name.clone()),
                        ("married", married.year.to_string()),
                        ("born", born.year.to_string()),
                    ],
                ));
            }
        }
    }
    out
}

fn isolated_record(b: &Bundle) -> Vec<Finding> {
    let mut related: BTreeSet<&str> = BTreeSet::new();
    for f in &b.families {
        related.extend(f.partners.iter().copied());
        related.extend(f.children.iter().copied());
    }
    for (_, l) in entities(b.flat, "links") {
        for end in ["from", "to"] {
            if let Some(id) = l
                .get(end)
                .and_then(|e| e.get("entity_id"))
                .and_then(Value::as_str)
            {
                related.insert(id);
            }
        }
    }
    let mut dated: BTreeSet<&str> = BTreeSet::new();
    for (_, e) in entities(b.flat, "events") {
        if Day::of(e.get("date")).is_none() {
            continue;
        }
        if let Some(ps) = e.get("participants").and_then(Value::as_array) {
            dated.extend(
                ps.iter()
                    .filter_map(|x| x.get("person_id").and_then(Value::as_str)),
            );
        }
    }
    b.persons
        .iter()
        .filter(|p| {
            p.birth.is_none()
                && p.death.is_none()
                && !dated.contains(p.id)
                && !related.contains(p.id)
        })
        .map(|p| {
            finding(
                Rule::IsolatedRecord,
                "person",
                p.id,
                vec![p.id.to_string()],
                &[("person", p.name.clone())],
            )
        })
        .collect()
}

fn place_no_region(b: &Bundle) -> Vec<Finding> {
    b.places
        .iter()
        .filter(|p| p.region.is_none())
        .map(|p| {
            finding(
                Rule::PlaceNoRegion,
                "place",
                p.id,
                Vec::new(),
                &[("place", p.name.clone())],
            )
        })
        .collect()
}

/// Administrative words a region may begin with in lowercase: a county, a
/// parish, a province. "powiat kalwaryjski" is a region; "dom opieki" is not.
const ADMINISTRATIVE: [&str; 9] = [
    "pow.",
    "powiat",
    "gubernia",
    "województwo",
    "wojewodztwo",
    "parafia",
    "gmina",
    "obwód",
    "obwod",
];

/// Whether a region field holds free text rather than a place: it contains a
/// digit (a grave plot, a house number), runs past five words (a sentence),
/// or begins with a lowercase word that is neither an administrative term
/// ([`ADMINISTRATIVE`]) nor a region named by its adjective ("ostrołęckie",
/// "lubelskie") — a place name begins with a capital.
pub fn free_text_region(region: &str) -> bool {
    let r = region.trim();
    if r.chars().any(|c| c.is_ascii_digit()) || r.split_whitespace().count() > 5 {
        return true;
    }
    let first = r.split_whitespace().next().unwrap_or("");
    let lower = first.chars().next().is_some_and(char::is_lowercase);
    if !lower {
        return false;
    }
    let f = first.to_lowercase();
    let adjectival = ["skie", "ckie", "dzkie"].iter().any(|s| f.ends_with(s));
    !(ADMINISTRATIVE.contains(&f.as_str()) || adjectival)
}

fn region_free_text(b: &Bundle) -> Vec<Finding> {
    b.places
        .iter()
        .filter_map(|p| {
            let r = p.region.as_deref()?;
            free_text_region(r).then(|| {
                finding(
                    Rule::RegionFreeText,
                    "place",
                    p.id,
                    Vec::new(),
                    &[("place", p.name.clone()), ("value", r.to_string())],
                )
            })
        })
        .collect()
}

/// A place name as compared for duplicates: folded, and without a trailing
/// "?" or "." — "Poznań?" is a doubt about Poznań, not another town.
fn place_key(name: &str) -> String {
    fold(name.trim_end_matches(|c: char| c == '?' || c == '.' || c.is_whitespace()))
}

fn place_near_duplicate(b: &Bundle) -> Vec<Finding> {
    let mut groups: BTreeMap<String, Vec<&Place>> = BTreeMap::new();
    for p in &b.places {
        if !p.name.is_empty() {
            groups.entry(place_key(&p.name)).or_default().push(p);
        }
    }
    groups
        .into_values()
        .filter(|g| g.len() > 1)
        .map(|g| {
            let names: Vec<String> = g.iter().map(|p| format!("“{}”", p.name)).collect();
            finding(
                Rule::PlaceNearDuplicate,
                "place",
                g[1].id,
                Vec::new(),
                &[("names", names.join(", ")), ("count", g.len().to_string())],
            )
        })
        .collect()
}

fn not_a_place(b: &Bundle) -> Vec<Finding> {
    b.places
        .iter()
        .filter(|p| p.name.chars().next().is_some_and(char::is_lowercase))
        .map(|p| {
            finding(
                Rule::NotAPlace,
                "place",
                p.id,
                Vec::new(),
                &[("place", p.name.clone())],
            )
        })
        .collect()
}

/// Whether an object, or the fact it belongs to, cites a source.
fn cites(v: &Value) -> bool {
    ["source_id", "source_ids", "sources", "citations"]
        .iter()
        .any(|k| match v.get(*k) {
            Some(Value::String(s)) => !s.is_empty(),
            Some(Value::Array(a)) => !a.is_empty(),
            Some(Value::Null) | None => false,
            Some(_) => true,
        })
}

fn certain_without_source(b: &Bundle) -> Vec<Finding> {
    fn walk(v: &Value, parent_cites: bool, path: &str, out: &mut Vec<String>) {
        match v {
            Value::Object(m) => {
                let here = parent_cites || cites(v);
                if m.get("confidence")
                    .and_then(Value::as_f64)
                    .is_some_and(|c| c >= 0.999)
                    && !here
                {
                    out.push(path.to_string());
                }
                for (k, x) in m {
                    if k != "confidence" {
                        walk(x, here, &format!("{path}.{k}"), out);
                    }
                }
            }
            Value::Array(a) => {
                for x in a {
                    walk(x, parent_cites, path, out);
                }
            }
            _ => {}
        }
    }
    let mut out = Vec::new();
    for (kind, entity) in [
        ("persons", "person"),
        ("families", "family"),
        ("events", "event"),
        ("occupations", "occupation"),
        ("links", "link"),
    ] {
        for (id, e) in entities(b.flat, kind) {
            let mut paths = Vec::new();
            walk(e, false, "", &mut paths);
            if paths.is_empty() {
                continue;
            }
            let subjects = subjects(b, entity, id, e);
            let label = match entity {
                "person" => b.name(id),
                _ => subjects
                    .iter()
                    .map(|s| b.name(s))
                    .filter(|n| !n.is_empty())
                    .collect::<Vec<_>>()
                    .join(", "),
            };
            let what: BTreeSet<&str> = paths
                .iter()
                .map(|p| p.trim_start_matches('.').split('.').next().unwrap_or(""))
                .filter(|s| !s.is_empty())
                .collect();
            out.push(finding(
                Rule::CertainWithoutSource,
                entity,
                id,
                subjects,
                &[
                    ("record", label),
                    ("claims", what.into_iter().collect::<Vec<_>>().join(", ")),
                ],
            ));
        }
    }
    out
}

/// The people a record is about, for the access filter.
fn subjects(b: &Bundle, kind: &str, id: &str, e: &Value) -> Vec<String> {
    match kind {
        "person" => vec![id.to_string()],
        "family" => b
            .families
            .iter()
            .find(|f| f.id == id)
            .map(|f| {
                f.partners
                    .iter()
                    .chain(&f.children)
                    .map(|s| s.to_string())
                    .collect()
            })
            .unwrap_or_default(),
        "event" => e
            .get("participants")
            .and_then(Value::as_array)
            .map(|a| {
                a.iter()
                    .filter_map(|x| x.get("person_id").and_then(Value::as_str))
                    .map(str::to_string)
                    .collect()
            })
            .unwrap_or_default(),
        "occupation" => e
            .get("person_id")
            .and_then(Value::as_str)
            .map(|s| vec![s.to_string()])
            .unwrap_or_default(),
        "link" => ["from", "to"]
            .iter()
            .filter_map(|end| e.get(*end))
            .filter(|x| x.get("entity_type").and_then(Value::as_str) == Some("person"))
            .filter_map(|x| x.get("entity_id").and_then(Value::as_str))
            .map(str::to_string)
            .collect(),
        _ => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_region_that_is_a_place_is_not_free_text() {
        for r in [
            "Warszawa",
            "Gubernia Warszawska",
            "ostrołęckie",
            "powiat kalwaryjski",
            "pow. kaźmierski",
            "parafia Łączki Kucharskie",
            "Polesie, obecnie Białoruś",
            "France",
        ] {
            assert!(!free_text_region(r), "{r}");
        }
        for r in [
            "dom opieki",
            "powstanie",
            "tam pochowana",
            "kolegiata Św Marcina",
            "Cmentarz Stare Powązki, kwatera 2, rzad 1, grób 15/16",
        ] {
            assert!(free_text_region(r), "{r}");
        }
    }

    #[test]
    fn surnames_match_across_polish_gender_forms() {
        assert_eq!(surname_stem("Klicki"), surname_stem("Klicka"));
        assert_eq!(surname_stem("Gutowska"), surname_stem("Gutowski"));
        assert_ne!(surname_stem("Nowak"), surname_stem("Kasprzyk"));
    }

    #[test]
    fn dates_compare_only_as_far_as_they_say() {
        let d = |v: &str| Day::of(Some(&serde_json::json!({ "value": v }))).unwrap();
        assert!(d("1850").before(d("1851")));
        assert!(
            !d("1850").before(d("1850-05-01")),
            "a year is not before a day within it"
        );
        assert!(d("1850-04-01").before(d("1850-05-01")));
        assert_eq!(d("1850-06-01").years_to(d("1862-05-31")), 11);
        assert_eq!(d("1850").years_to(d("1862")), 12);
    }
}

/// The finding in one sentence, in the reader's language: what is wrong, then
/// what to do about it.
pub fn message(f: &Finding, lang: &str) -> String {
    let mut args = fluent::FluentArgs::new();
    for (k, v) in &f.args {
        args.set(k.as_str(), v.clone());
    }
    crate::i18n::translate(lang, &format!("quality-{}", f.rule.slug()), Some(&args))
}

/// Where the finding is fixed: the record's own editor for someone who may
/// write, the person's record for someone who may only read. `None` when a
/// reader has nowhere to go (a place, for someone who may not edit it).
pub fn link(f: &Finding, may_write: bool) -> Option<String> {
    if may_write {
        return Some(format!("/admin/{}/{}/edit", f.kind, f.id));
    }
    match f.kind {
        "person" => Some(format!("/person/{}", f.id)),
        _ => f.persons.first().map(|p| format!("/person/{p}")),
    }
}

/// The findings as CSV, one per line: group, rule, kind, id, link, message.
pub fn csv(findings: &[Finding], lang: &str, may_write: bool) -> String {
    fn field(s: &str) -> String {
        if s.contains([',', '"', '\n', '\r']) {
            format!("\"{}\"", s.replace('"', "\"\""))
        } else {
            s.to_string()
        }
    }
    let mut out = String::from("group,rule,kind,id,link,message\n");
    for f in findings {
        let row = [
            f.rule.group().to_string(),
            f.rule.slug().to_string(),
            f.kind.to_string(),
            f.id.clone(),
            link(f, may_write).unwrap_or_default(),
            message(f, lang),
        ];
        out.push_str(&row.iter().map(|s| field(s)).collect::<Vec<_>>().join(","));
        out.push('\n');
    }
    out
}
