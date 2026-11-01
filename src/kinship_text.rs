//! A relationship in words, in the reader's language (docs/adr/0020).
//!
//! [`crate::kinship`] finds what a relationship is; this says it. Every word
//! is a Fluent message, and each receives every fact a language might make
//! its word agree with: B's sex, the side of the family (a father's brother
//! and a mother's brother are two words in Arabic, Chinese, Swahili and
//! Lingala), an elder or a younger (Chinese and Japanese), one male line
//! (Chinese 堂 against 表), half or full, by adoption. A language uses what
//! it needs and ignores the rest, so a word is never built in Rust out of a
//! language's grammar.
//!
//! Two words are put together where one cannot say it all — a first cousin
//! of A's father, the wife of A's cousin — and each language puts them in
//! its own order through one message that receives both.

use fluent::FluentValue;
use serde::Serialize;
use serde_json::Value;

use crate::access::Lens;
use crate::kinship::{is_unrecorded, Context, Hop, Kin, Kind, Line, Relation, Sex, Step};
use crate::rites::{Rite, Role};

/// A relationship, said.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Said {
    /// The word: "first cousin", "godmother", "sister-in-law".
    pub term: String,
    /// What it needs added to be exact: "by adoption", "partner of a
    /// parent".
    pub note: Option<String>,
    /// Where it is listed among a person's relatives: `kin-group-{group}`.
    pub group: &'static str,
    /// The degree of kinship by descent, counted as civil law counts it: the
    /// generations from one person up to the shared ancestor and down to the
    /// other.
    pub degree: Option<u32>,
    /// The tradition's own word for a tie of rite, untranslated.
    pub native: Option<&'static str>,
}

/// The groups of a person's relatives, in the order they are listed.
pub const GROUPS: &[&str] = &[
    "partners",
    "ancestors",
    "descendants",
    "siblings",
    "uncles",
    "nephews",
    "cousins",
    "in-laws",
    "step",
    "care",
    "rites",
    "others",
];

type Args = Vec<(&'static str, FluentValue<'static>)>;

fn tr(lang: &str, key: &str, args: &Args) -> String {
    crate::i18n::translate(lang, key, Some(&crate::i18n::args(args)))
}

fn yn(b: bool) -> FluentValue<'static> {
    FluentValue::from(if b { "yes" } else { "no" })
}

fn tri(b: Option<bool>) -> FluentValue<'static> {
    FluentValue::from(match b {
        Some(true) => "yes",
        Some(false) => "no",
        None => "unknown",
    })
}

fn sx(s: Option<Sex>) -> FluentValue<'static> {
    FluentValue::from(s.unwrap_or(Sex::U).as_str())
}

fn num(n: u32) -> FluentValue<'static> {
    FluentValue::from(n as i64)
}

fn text(s: String) -> FluentValue<'static> {
    FluentValue::from(s)
}

/// Everything a word by descent may agree with.
fn context_args(sex: Sex, c: &Context, half: bool, adoptive: bool) -> Args {
    vec![
        ("sex", FluentValue::from(sex.as_str())),
        ("half", yn(half)),
        ("adoptive", yn(adoptive)),
        ("side", sx(c.a_next)),
        ("via", sx(c.turn_b)),
        ("turna", sx(c.turn_a)),
        ("next", sx(c.b_next)),
        ("line", yn(c.male_line)),
        ("elder", tri(c.b_elder)),
        ("turnelder", tri(c.turn_elder)),
    ]
}

/// What B is to A, said in `lang`; B's sex is `sex`.
pub fn say(rel: &Relation, sex: Sex, lang: &str) -> Said {
    let c = &rel.context;
    match &rel.kind {
        Kind::Blood {
            up,
            down,
            half,
            adoptive,
        } => blood(lang, *up, *down, *half, *adoptive, sex, c),
        Kind::Partner { married, ended } => Said {
            term: partner(lang, sex, *married, *ended),
            note: None,
            group: "partners",
            degree: None,
            native: None,
        },
        Kind::Affinity {
            core,
            a_side,
            b_side,
        } => affinity(lang, core, *a_side, *b_side, sex, c),
        Kind::Care { line, up } => care(lang, *line, *up, sex),
        Kind::Rite { rite, role } => Said {
            term: rite_word(lang, rite, *role, sex),
            note: None,
            group: "rites",
            degree: None,
            native: (!rite.native.is_empty()).then_some(rite.native),
        },
        Kind::Chain => Said {
            term: tr(
                lang,
                "kin-related",
                &vec![("sex", FluentValue::from(sex.as_str()))],
            ),
            note: None,
            group: "others",
            degree: None,
            native: None,
        },
    }
}

fn partner(lang: &str, sex: Sex, married: bool, ended: bool) -> String {
    tr(
        lang,
        "kin-partner",
        &vec![
            ("sex", FluentValue::from(sex.as_str())),
            ("married", yn(married)),
            ("ended", yn(ended)),
        ],
    )
}

/// A rite's word for the person at one end of it.
pub fn rite_word(lang: &str, rite: &Rite, role: Role, sex: Sex) -> String {
    tr(
        lang,
        &format!("kin-rite-{}", rite.key),
        &vec![
            ("role", FluentValue::from(role.as_str())),
            ("sex", FluentValue::from(sex.as_str())),
        ],
    )
}

/// What a rite is, in one line.
pub fn rite_note(lang: &str, rite: &Rite) -> String {
    tr(lang, &format!("kin-rite-{}-note", rite.key), &Vec::new())
}

/// A tradition's name.
pub fn tradition(lang: &str, key: &str) -> String {
    tr(lang, &format!("kin-tradition-{key}"), &Vec::new())
}

fn blood(
    lang: &str,
    up: u32,
    down: u32,
    half: bool,
    adoptive: bool,
    sex: Sex,
    c: &Context,
) -> Said {
    let mut args = context_args(sex, c, half, adoptive);
    // Parents, children and siblings say an adoption in their own word ("an
    // adoptive mother"); further out it is a note.
    let mut adoption_said = false;
    let (key, group) = match (up, down) {
        (1, 0) => {
            adoption_said = true;
            ("kin-parent", "ancestors")
        }
        (u, 0) => {
            args.push(("n", num(u)));
            args.push(("g", num(u - 2)));
            ("kin-ancestor", "ancestors")
        }
        (0, 1) => {
            adoption_said = true;
            ("kin-child", "descendants")
        }
        (0, d) => {
            args.push(("n", num(d)));
            args.push(("g", num(d - 2)));
            ("kin-descendant", "descendants")
        }
        (1, 1) => {
            adoption_said = true;
            ("kin-sibling", "siblings")
        }
        (u, 1) => {
            args.push(("n", num(u - 1)));
            args.push(("g", num(u - 2)));
            ("kin-uncle", "uncles")
        }
        (1, d) => {
            args.push(("n", num(d - 1)));
            args.push(("g", num(d - 2)));
            ("kin-nephew", "nephews")
        }
        (u, d) if u == d => {
            args.push(("k", num(u - 1)));
            ("kin-cousin", "cousins")
        }
        (u, d) => {
            let k = u.min(d) - 1;
            let r = u.abs_diff(d);
            let older = u > d;
            let cousin = {
                let mut a = args.clone();
                a.push(("k", num(k)));
                tr(lang, "kin-cousin", &a)
            };
            let forebear = tr(
                lang,
                "kin-of-forebear",
                &vec![("n", num(r)), ("sex", sx(c.relay))],
            );
            let descendant = tr(
                lang,
                "kin-descendant-word",
                &vec![("n", num(r)), ("sex", FluentValue::from(sex.as_str()))],
            );
            let ofcousin = tr(
                lang,
                "kin-of-cousin",
                &vec![("k", num(k)), ("sex", sx(c.relay)), ("half", yn(half))],
            );
            args.extend([
                ("k", num(k)),
                ("r", num(r)),
                ("older", yn(older)),
                ("cousin", text(cousin)),
                ("forebear", text(forebear)),
                ("descendant", text(descendant)),
                ("ofcousin", text(ofcousin)),
            ]);
            ("kin-cousin-removed", "cousins")
        }
    };
    Said {
        term: tr(lang, key, &args),
        note: (adoptive && !adoption_said).then(|| tr(lang, "kin-note-adoptive", &Vec::new())),
        group,
        degree: Some(up + down),
        native: None,
    }
}

fn affinity(lang: &str, core: &Kind, a_side: bool, b_side: bool, sex: Sex, c: &Context) -> Said {
    let s = FluentValue::from(sex.as_str());
    let said = |key: &str, group: &'static str, note: Option<&str>, extra: Args| {
        let mut args: Args = vec![
            ("sex", s.clone()),
            ("via", sx(c.a_next)),
            ("next", sx(c.b_next)),
            ("elder", tri(c.b_elder)),
        ];
        args.extend(extra);
        Said {
            term: tr(lang, key, &args),
            note: note.map(|k| tr(lang, k, &args)),
            group,
            degree: None,
            native: None,
        }
    };
    match (core, a_side, b_side) {
        (Kind::Blood { up: 1, down: 0, .. }, true, false) => {
            said("kin-parent-in-law", "in-laws", None, Vec::new())
        }
        (Kind::Blood { up: 0, down: 1, .. }, false, true) => {
            said("kin-child-in-law", "in-laws", None, Vec::new())
        }
        (Kind::Blood { up: 1, down: 1, .. }, true, false) => said(
            "kin-sibling-in-law",
            "in-laws",
            None,
            vec![("kind", FluentValue::from("partner-sibling"))],
        ),
        (Kind::Blood { up: 1, down: 1, .. }, false, true) => said(
            "kin-sibling-in-law",
            "in-laws",
            None,
            vec![("kind", FluentValue::from("sibling-partner"))],
        ),
        (Kind::Blood { up: 0, down: 1, .. }, true, false) => said(
            "kin-step-child",
            "step",
            Some("kin-note-step-child"),
            Vec::new(),
        ),
        (Kind::Blood { up: 1, down: 0, .. }, false, true) => said(
            "kin-step-parent",
            "step",
            Some("kin-note-step-parent"),
            Vec::new(),
        ),
        (Kind::Blood { .. }, true, false) => {
            // A relative of A's partner: what B is to the partner, then whose.
            let inner = relation_of(core, c);
            let relative = say(&inner, sex, lang).term;
            said(
                "kin-relative-of-partner",
                "in-laws",
                None,
                vec![("relative", text(relative))],
            )
        }
        (Kind::Blood { .. }, false, true) => {
            // The partner of A's relative R: what R is to A, then B's tie.
            let inner = relation_of(core, c);
            let r_sex = c.b_next.unwrap_or(Sex::U);
            let relative = say(&inner, r_sex, lang).term;
            // The same word agreeing with B, for languages that say "an aunt
            // by marriage" rather than "the wife of an uncle".
            let relative_b = say(&inner, sex, lang).term;
            let vowel = takes_an(&relative);
            said(
                "kin-partner-of-relative",
                "in-laws",
                None,
                vec![
                    ("relative", text(relative)),
                    ("relativeb", text(relative_b)),
                    ("rsex", FluentValue::from(r_sex.as_str())),
                    ("rvowel", yn(vowel)),
                ],
            )
        }
        _ => said("kin-affine", "in-laws", None, Vec::new()),
    }
}

/// A bare relationship by descent, to be said on its own inside another.
fn relation_of(core: &Kind, c: &Context) -> Relation {
    Relation {
        kind: core.clone(),
        path: Vec::new(),
        common: Vec::new(),
        context: c.clone(),
    }
}

fn care(lang: &str, line: Line, up: bool, sex: Sex) -> Said {
    let (key, group, note) = match (line, up) {
        (Line::Step, true) => ("kin-step-parent", "step", Some("kin-note-step-parent")),
        (Line::Step, false) => ("kin-step-child", "step", Some("kin-note-step-child")),
        (Line::Foster, true) => ("kin-foster-parent", "care", None),
        (Line::Foster, false) => ("kin-foster-child", "care", None),
        (_, true) => ("kin-guardian", "care", None),
        (_, false) => ("kin-ward", "care", None),
    };
    let args: Args = vec![
        ("sex", FluentValue::from(sex.as_str())),
        ("next", FluentValue::from("U")),
        ("via", FluentValue::from("U")),
    ];
    Said {
        term: tr(lang, key, &args),
        note: note.map(|k| tr(lang, k, &args)),
        group,
        degree: None,
        native: None,
    }
}

/// The word on one step of a path: what this person is to the one before.
pub fn hop(step: &Step, sex: Sex, lang: &str) -> String {
    let s = FluentValue::from(sex.as_str());
    match &step.hop {
        Hop::Parent(line) => tr(
            lang,
            "kin-hop-parent",
            &vec![("sex", s), ("line", FluentValue::from(line.as_str()))],
        ),
        Hop::Child(line) => tr(
            lang,
            "kin-hop-child",
            &vec![("sex", s), ("line", FluentValue::from(line.as_str()))],
        ),
        Hop::Partner { married, ended } => partner(lang, sex, *married, *ended),
        Hop::Rite(rite, role) => rite_word(lang, rite, *role, sex),
    }
}

/// "of Lucien", in the reader's language: relative to whom a word is said.
pub fn of(name: &str, lang: &str) -> String {
    tr(
        lang,
        "kin-of",
        &vec![
            ("name", text(name.to_string())),
            ("vowel", yn(starts_with_vowel(name))),
        ],
    )
}

/// Whether a word or a name begins with a vowel sound a language may elide
/// before ("d'Anna", "l'oncle"). A mute h counts, as French counts it in
/// names ("d'Henri").
pub fn starts_with_vowel(s: &str) -> bool {
    s.chars()
        .find(|c| c.is_alphabetic())
        .map(|c| c.to_lowercase().next().unwrap_or(c))
        .is_some_and(|c| "aeiouyhàâäáéèêëíìîïóòôöúùûüœæ".contains(c))
}

/// Whether English says "an" rather than "a" before a word of kinship (the
/// `rvowel` of `kin-partner-of-relative`): a vowel sound, which neither
/// "half-" nor "great-" makes, and which "8×", "11×" and "18×" do.
fn takes_an(s: &str) -> bool {
    let s = s.trim_start();
    let digits: String = s.chars().take_while(|c| c.is_ascii_digit()).collect();
    if !digits.is_empty() {
        return digits.starts_with('8') || digits == "11" || digits == "18";
    }
    s.chars()
        .find(|c| c.is_alphabetic())
        .map(|c| c.to_lowercase().next().unwrap_or(c))
        .is_some_and(|c| "aeiou".contains(c))
}

/// One relative in a person's list.
#[derive(Debug, Clone, Serialize)]
pub struct Row {
    pub id: String,
    pub name: String,
    /// "1924–1998", to tell two people of one name apart.
    pub years: String,
    pub said: Said,
    /// The other ways they are related, said.
    pub also: Vec<String>,
}

/// One group of a person's relatives: the ancestors, the cousins.
#[derive(Debug, Clone, Serialize)]
pub struct Group {
    pub key: &'static str,
    pub title: String,
    pub rows: Vec<Row>,
}

/// One step of a path as it is drawn: who, and what they are to the one
/// before.
#[derive(Debug, Clone, Serialize)]
pub struct PathStep {
    pub id: String,
    pub name: String,
    /// A person this reader may not see: neither named nor linked.
    pub hidden: bool,
    pub word: String,
    /// One of the ancestors the two people share.
    pub common: bool,
}

/// One way the two people are related, said and shown.
#[derive(Debug, Clone, Serialize)]
pub struct Found {
    pub said: Said,
    /// "of Lucien".
    pub of_a: String,
    pub degree: Option<String>,
    /// The shared ancestors, named.
    pub common: Vec<String>,
    pub path: Vec<PathStep>,
    /// What a tie of rite is, in one line.
    pub rite_note: Option<String>,
}

/// The answer to "how is B related to A".
#[derive(Debug, Clone, Serialize)]
pub struct Finding {
    pub id: String,
    pub name: String,
    pub relations: Vec<Found>,
}

/// The Kinship tab: every relative of one person, and the way to one of
/// them if asked.
#[derive(Debug, Clone, Serialize)]
pub struct KinPage {
    pub id: String,
    pub name: String,
    pub groups: Vec<Group>,
    pub count: usize,
    pub finding: Option<Finding>,
}

fn label(flat: &Value, lens: &Lens, id: &str, lang: &str) -> (String, String, bool) {
    let l = crate::access::river_label(flat, lens, id);
    if l.redacted || l.full.is_empty() {
        return (tr(lang, "kin-hidden", &Vec::new()), String::new(), true);
    }
    let years = match (l.birth_year, l.death_year) {
        (Some(b), Some(d)) => format!("{b}–{d}"),
        (Some(b), None) => format!("{b}–"),
        (None, Some(d)) => format!("–{d}"),
        (None, None) => String::new(),
    };
    (l.full, years, false)
}

/// What B is to A, every way, said and shown step by step.
pub fn found(kin: &Kin, flat: &Value, lens: &Lens, lang: &str, a: &str, b: &str) -> Vec<Found> {
    let (a_name, _, _) = label(flat, lens, a, lang);
    let sex = kin.sex_of(b);
    kin.between(a, b)
        .iter()
        .map(|rel| {
            let said = say(rel, sex, lang);
            let common: Vec<String> = rel
                .common
                .iter()
                .filter(|c| !is_unrecorded(c))
                .map(|c| label(flat, lens, c, lang).0)
                .collect();
            // The unrecorded parent of a sibling group is no step anyone can
            // read: the way goes from one child straight to the other, and
            // that step says what they are to each other.
            let mut path = Vec::new();
            let mut before: &str = a;
            let mut sibling = false;
            for step in &rel.path {
                if is_unrecorded(&step.person) {
                    sibling = true;
                    continue;
                }
                let step_sex = kin.sex_of(&step.person);
                let word = if sibling {
                    kin.between(before, &step.person)
                        .first()
                        .map(|r| say(r, step_sex, lang).term)
                        .unwrap_or_else(|| hop(step, step_sex, lang))
                } else {
                    hop(step, step_sex, lang)
                };
                let (name, _, hidden) = label(flat, lens, &step.person, lang);
                path.push(PathStep {
                    id: if hidden {
                        String::new()
                    } else {
                        step.person.clone()
                    },
                    name,
                    hidden,
                    word,
                    common: rel.common.contains(&step.person),
                });
                sibling = false;
                before = &step.person;
            }
            let rite_note = match &rel.kind {
                Kind::Rite { rite, .. } => Some(rite_note(lang, rite)),
                _ => None,
            };
            Found {
                degree: said
                    .degree
                    .map(|n| tr(lang, "kin-degree", &vec![("n", num(n))])),
                said,
                of_a: of(&a_name, lang),
                common,
                path,
                rite_note,
            }
        })
        .collect()
}

/// The Kinship tab of `a`'s record, and the answer for `with` if asked.
pub fn page(flat: &Value, lens: &Lens, lang: &str, a: &str, with: Option<&str>) -> KinPage {
    let kin = Kin::new(flat, lens);
    let (name, _, _) = label(flat, lens, a, lang);
    let mut groups: Vec<Group> = GROUPS
        .iter()
        .map(|&key| Group {
            key,
            title: tr(lang, &format!("kin-group-{key}"), &Vec::new()),
            rows: Vec::new(),
        })
        .collect();
    let relatives = kin.relatives(a);
    let count = relatives.len();
    for (id, rels) in relatives {
        let sex = kin.sex_of(&id);
        let said = say(&rels[0], sex, lang);
        let also = rels[1..]
            .iter()
            .map(|r| say(r, sex, lang).term)
            .filter(|t| *t != said.term)
            .collect();
        let (who, years, _) = label(flat, lens, &id, lang);
        if let Some(g) = groups.iter_mut().find(|g| g.key == said.group) {
            g.rows.push(Row {
                id,
                name: who,
                years,
                said,
                also,
            });
        }
    }
    groups.retain(|g| !g.rows.is_empty());
    let finding = with
        .filter(|b| *b != a && lens.sees_person(b))
        .filter(|b| flat.get("persons").and_then(|p| p.get(*b)).is_some())
        .map(|b| Finding {
            id: b.to_string(),
            name: label(flat, lens, b, lang).0,
            relations: found(&kin, flat, lens, lang, a, b),
        });
    KinPage {
        id: a.to_string(),
        name,
        groups,
        count,
        finding,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn said(lang: &str, a: &str, b: &str) -> Vec<String> {
        let f = crate::kinship::tests::family();
        let lens = Lens::unrestricted();
        let k = Kin::new(&f, &lens);
        k.between(a, b)
            .iter()
            .map(|r| say(r, k.sex_of(b), lang).term)
            .collect()
    }

    #[test]
    fn english_names_the_family() {
        assert_eq!(said("en", "ego", "p1"), vec!["father"]);
        assert_eq!(said("en", "ego", "p1w"), vec!["mother"]);
        assert_eq!(said("en", "ego", "gg"), vec!["great-grandfather"]);
        assert_eq!(said("en", "ego", "gkid"), vec!["granddaughter"]);
        assert_eq!(said("en", "ego", "sis"), vec!["sister"]);
        assert_eq!(said("en", "ego", "half"), vec!["half-brother"]);
        assert_eq!(said("en", "ego", "adop"), vec!["adoptive sister"]);
        assert_eq!(said("en", "ego", "p2"), vec!["aunt"]);
        assert_eq!(said("en", "ego", "g3"), vec!["great-uncle"]);
        assert_eq!(said("en", "ego", "neph"), vec!["nephew"]);
        assert_eq!(said("en", "ego", "cous"), vec!["second cousin"]);
        assert_eq!(said("en", "ego", "c1"), vec!["first cousin once removed"]);
        assert_eq!(
            said("en", "ego", "cous2"),
            vec!["second cousin once removed"]
        );
        assert_eq!(said("en", "ego", "wife"), vec!["wife"]);
        assert_eq!(said("en", "ego", "wifeb"), vec!["brother-in-law"]);
        assert_eq!(said("en", "ego", "sish"), vec!["brother-in-law"]);
        assert_eq!(said("en", "ego", "kidw"), vec!["daughter-in-law"]);
        assert_eq!(said("en", "ego", "p1w2"), vec!["stepmother"]);
        assert_eq!(said("en", "ego", "step"), vec!["stepfather"]);
        assert_eq!(said("en", "ego", "god"), vec!["godmother"]);
        assert_eq!(said("en", "god", "ego"), vec!["godson"]);
        assert_eq!(said("en", "ego", "milk"), vec!["milk sister"]);
    }

    #[test]
    fn french_names_the_family() {
        assert_eq!(said("fr", "ego", "gg"), vec!["arrière-grand-père"]);
        assert_eq!(said("fr", "ego", "half"), vec!["demi-frère"]);
        assert_eq!(said("fr", "ego", "adop"), vec!["sœur adoptive"]);
        assert_eq!(said("fr", "ego", "p2"), vec!["tante"]);
        assert_eq!(said("fr", "ego", "g3"), vec!["grand-oncle"]);
        assert_eq!(said("fr", "ego", "cous"), vec!["cousine issue de germain"]);
        // Older by a generation: a first cousin of ego's father.
        assert_eq!(said("fr", "ego", "c1"), vec!["cousine germaine du père"]);
        assert_eq!(said("fr", "ego", "h1"), vec!["cousin germain du père"]);
        // Younger by one: the son of a second cousin.
        assert_eq!(
            said("fr", "ego", "cous2"),
            vec!["fils d’une cousine issue de germain"]
        );
        assert_eq!(said("fr", "ego", "kidw"), vec!["belle-fille"]);
        assert_eq!(said("fr", "ego", "god"), vec!["marraine"]);
        assert_eq!(said("fr", "god", "ego"), vec!["filleul"]);
        assert_eq!(said("fr", "ego", "milk"), vec!["sœur de lait"]);
    }

    #[test]
    fn of_a_name_elides_where_the_language_does() {
        assert_eq!(of("Lucien", "fr"), "de Lucien");
        assert_eq!(of("Anna", "fr"), "d’Anna");
        assert_eq!(of("Hélène", "fr"), "d’Hélène");
        assert_eq!(of("Anna", "en"), "of Anna");
    }

    /// A sibling group (AXGF 1.0 §4.2.3): nobody recorded the parents, and
    /// the children are brother and sister all the same.
    #[test]
    fn children_whose_parents_nobody_recorded_are_brother_and_sister() {
        let mut ps = serde_json::Map::new();
        for (id, s, b) in [
            ("ann", "F", "1900"),
            ("bob", "M", "1902"),
            ("bobw", "F", "1903"),
            ("kid", "M", "1930"),
        ] {
            let (k, v) = crate::kinship::tests::person(id, s, b);
            ps.insert(k, v);
        }
        let f = serde_json::json!({
            "persons": ps,
            "families": {
                "g": {"id": "g", "type": "family", "axgf_version": "1.0",
                      "children": [{"person_id": "ann"}, {"person_id": "bob"}]},
                "h": {"id": "h", "type": "family", "axgf_version": "1.0",
                      "union": {"type": "marriage", "persons": [
                          {"person_id": "bob", "role": "spouse"},
                          {"person_id": "bobw", "role": "spouse"}]},
                      "children": [{"person_id": "kid"}]}
            },
            "links": {}
        });
        let lens = Lens::unrestricted();
        let k = Kin::new(&f, &lens);
        let brother = found(&k, &f, &lens, "en", "ann", "bob");
        assert_eq!(brother[0].said.term, "brother");
        // Nobody stands between them, and nobody is named as shared.
        assert_eq!(brother[0].path.len(), 1);
        assert_eq!(brother[0].path[0].word, "brother");
        assert!(brother[0].common.is_empty());
        let nephew = found(&k, &f, &lens, "en", "ann", "kid");
        assert_eq!(nephew[0].said.term, "nephew");
        let words: Vec<&str> = nephew[0].path.iter().map(|s| s.word.as_str()).collect();
        assert_eq!(words, vec!["brother", "son"]);
        // Her relatives: her brother, his wife and their son, and not the
        // parent nobody recorded.
        let mut ids: Vec<String> = k.relatives("ann").into_iter().map(|(id, _)| id).collect();
        ids.sort();
        assert_eq!(ids, vec!["bob", "bobw", "kid"]);
    }

    #[test]
    fn english_says_a_half_nephew_and_an_aunt() {
        // The wife of ego's son, seen from ego's half-brother.
        assert_eq!(said("en", "half", "kidw"), vec!["wife of a half-nephew"]);
        // The husband of h1's father's sister.
        assert_eq!(said("en", "h1", "g2h"), vec!["husband of an aunt"]);
        assert!(takes_an("8× great-uncle"));
        assert!(!takes_an("3× great-uncle"));
    }

    #[test]
    fn the_tab_lists_every_relative_by_group_and_answers_for_one() {
        let f = crate::kinship::tests::family();
        let lens = Lens::unrestricted();
        let page = page(&f, &lens, "en", "ego", Some("cous"));
        let keys: Vec<&str> = page.groups.iter().map(|g| g.key).collect();
        for k in [
            "partners",
            "ancestors",
            "descendants",
            "siblings",
            "cousins",
            "in-laws",
            "rites",
        ] {
            assert!(keys.contains(&k), "{k} missing from {keys:?}");
        }
        let finding = page.finding.expect("asked for");
        let way = &finding.relations[0];
        assert_eq!(way.said.term, "second cousin");
        assert_eq!(way.of_a, "of ego");
        // Up three generations to the shared couple, down three to her.
        assert_eq!(way.path.len(), 6);
        assert_eq!(way.common, vec!["gg".to_string(), "ggw".to_string()]);
        assert!(way.path.iter().any(|s| s.common));
        assert_eq!(way.degree.as_deref(), Some("kinship in the 6th degree"));
    }

    #[test]
    fn every_group_has_a_name_and_every_rite_its_words() {
        for g in GROUPS {
            let k = format!("kin-group-{g}");
            assert!(crate::i18n::has_message("en", &k), "{k}");
        }
        for r in crate::rites::RITES {
            assert!(crate::i18n::has_message(
                "en",
                &format!("kin-rite-{}", r.key)
            ));
            assert!(crate::i18n::has_message(
                "en",
                &format!("kin-rite-{}-note", r.key)
            ));
        }
        for t in crate::rites::TRADITIONS {
            assert!(crate::i18n::has_message(
                "en",
                &format!("kin-tradition-{t}")
            ));
        }
    }
}
