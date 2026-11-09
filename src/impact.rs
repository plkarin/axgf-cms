//! The Universal Synoptic Timeline, "Impact" (the operator's request of
//! 6 November 2026, docs/adr/0034).
//!
//! A vertical axis of time: the end of the last ice age at the top, today at
//! the bottom, the history of the world written along it (its civilisations,
//! migrations, epidemics and wars, each with how long it lasted). The
//! family's lives lie beside it. A person whose birth and death are known is
//! a window from one to the other; one of whom a single moment is known is a
//! pin at that moment. Across one band of time, the eye reads what a life
//! went through.
//!
//! Two views:
//!
//! - **the family's**: the world's landmarks (macro), those of the continents
//!   the family lived on (micro), and every person the reader may read;
//! - **a person's**: their life and its milestones, the world's and their
//!   continent's landmarks (macro), and their countries' (micro), each with
//!   its explanation and their age when it began.
//!
//! The landmarks are a catalogue kept with the code (`impact_events.txt`),
//! in English and French; the other languages read them in English. A
//! person's countries come from their places: the country a place states,
//! the last part of its name, or a well-known city in it.
//!
//! Nothing here is drawn on a canvas: the page places blocks of text on the
//! axis, each at its own height (`templates/impact.html`).

use std::collections::{BTreeMap, HashMap};
use std::sync::OnceLock;

use serde::Serialize;
use serde_json::Value;

use crate::access::Lens;

// ---------------------------------------------------------------------------
// the catalogue
// ---------------------------------------------------------------------------

/// What kind of landmark it is: its colour on the axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    Era,
    War,
    Epidemic,
    Migration,
    Event,
    Economy,
    Disaster,
    /// The great inventions (the operator's request of 8 November 2026):
    /// printing, the vaccines, the telephone…
    Invention,
    /// The great discoveries: the Americas, the Moon…
    Discovery,
}

impl Kind {
    pub const ALL: [Kind; 9] = [
        Kind::Era,
        Kind::War,
        Kind::Epidemic,
        Kind::Migration,
        Kind::Event,
        Kind::Economy,
        Kind::Disaster,
        Kind::Invention,
        Kind::Discovery,
    ];

    fn parse(s: &str) -> Option<Kind> {
        Kind::ALL.into_iter().find(|k| k.slug() == s)
    }

    pub fn slug(self) -> &'static str {
        match self {
            Kind::Era => "era",
            Kind::War => "war",
            Kind::Epidemic => "epidemic",
            Kind::Migration => "migration",
            Kind::Event => "event",
            Kind::Economy => "economy",
            Kind::Disaster => "disaster",
            Kind::Invention => "invention",
            Kind::Discovery => "discovery",
        }
    }

    /// Whether how long it lasted is said beside it, as the operator's
    /// repertoire says it of wars.
    fn lasts(self) -> bool {
        matches!(self, Kind::War | Kind::Epidemic | Kind::Disaster)
    }
}

/// The continents the catalogue knows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Continent {
    Europe,
    Africa,
    Asia,
    Americas,
    Oceania,
}

impl Continent {
    pub const ALL: [Continent; 5] = [
        Continent::Europe,
        Continent::Africa,
        Continent::Asia,
        Continent::Americas,
        Continent::Oceania,
    ];

    fn parse(s: &str) -> Option<Continent> {
        Continent::ALL.into_iter().find(|c| c.slug() == s)
    }

    pub fn slug(self) -> &'static str {
        match self {
            Continent::Europe => "europe",
            Continent::Africa => "africa",
            Continent::Asia => "asia",
            Continent::Americas => "americas",
            Continent::Oceania => "oceania",
        }
    }
}

/// Where a landmark matters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Scope {
    World,
    Continent(Continent),
    /// A present-day country, by its ISO 3166 code.
    Country(String),
}

/// A moment as the catalogue writes it: a year (negative before Christ), and
/// its month and day when known.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct When {
    pub year: i32,
    pub month: Option<u32>,
    pub day: Option<u32>,
}

impl When {
    fn parse(s: &str) -> Option<When> {
        let (bc, rest) = match s.strip_prefix('-') {
            Some(r) => (true, r),
            None => (false, s),
        };
        let mut parts = rest.split('-');
        let year: i32 = parts.next()?.parse().ok()?;
        let month = match parts.next() {
            Some(m) => Some(m.parse::<u32>().ok().filter(|m| (1..=12).contains(m))?),
            None => None,
        };
        let day = match parts.next() {
            Some(d) => Some(d.parse::<u32>().ok().filter(|d| (1..=31).contains(d))?),
            None => None,
        };
        if parts.next().is_some() {
            return None;
        }
        Some(When {
            year: if bc { -year } else { year },
            month,
            day,
        })
    }

    /// The moment as a number of years, for its place on the axis.
    pub fn at(self) -> f64 {
        let m = f64::from(self.month.unwrap_or(1)) - 1.0;
        let d = f64::from(self.day.unwrap_or(1)) - 1.0;
        f64::from(self.year) + m / 12.0 + d / 365.25
    }
}

/// One landmark of the world's history.
#[derive(Debug, Clone)]
pub struct Landmark {
    /// Stable within a catalogue: its year and its English name.
    pub key: String,
    pub kind: Kind,
    pub from: When,
    /// `None` for a moment.
    pub to: Option<When>,
    /// Still going on.
    pub ongoing: bool,
    pub scopes: Vec<Scope>,
    names: [String; 2],
    notes: [String; 2],
}

// ---------------------------------------------------------------------------
// the other languages
// ---------------------------------------------------------------------------

/// A landmark's name, note and window in one of the twelve other languages
/// (the operator's request of 9 November 2026, docs/adr/0038).
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Translated {
    pub name: String,
    pub note: String,
    pub context: String,
    pub stakes: String,
    pub victims: String,
}

/// One file a language, in `src/impact_i18n/`: under "## <key>", `name|`,
/// `note|`, `context|`, `stakes|` and `victims|` lines. What a file leaves
/// out reads in English.
pub const TRANSLATED: &[(&str, &str)] = &[
    ("pl", include_str!("impact_i18n/pl.txt")),
    ("de", include_str!("impact_i18n/de.txt")),
];

/// A language's file, read: each landmark's fields by its key.
pub fn parsed_translations(text: &str) -> Result<HashMap<String, Translated>, String> {
    let mut out: HashMap<String, Translated> = HashMap::new();
    let mut key: Option<String> = None;
    for (n, line) in text.lines().enumerate() {
        let line = line.trim_end();
        if line.is_empty() || (line.starts_with('#') && !line.starts_with("## ")) {
            continue;
        }
        if let Some(k) = line.strip_prefix("## ") {
            let k = k.trim().to_string();
            if out.contains_key(&k) {
                return Err(format!("line {}: {k} twice", n + 1));
            }
            out.insert(k.clone(), Translated::default());
            key = Some(k);
            continue;
        }
        let Some(k) = key.as_ref() else {
            return Err(format!("line {}: a field before any key", n + 1));
        };
        let Some((field, value)) = line.split_once('|') else {
            return Err(format!("line {}: no field", n + 1));
        };
        let Some(t) = out.get_mut(k) else {
            return Err(format!("line {}: no landmark", n + 1));
        };
        let slot = match field {
            "name" => &mut t.name,
            "note" => &mut t.note,
            "context" => &mut t.context,
            "stakes" => &mut t.stakes,
            "victims" => &mut t.victims,
            other => return Err(format!("line {}: no field \"{other}\"", n + 1)),
        };
        *slot = value.trim().to_string();
    }
    Ok(out)
}

fn translations() -> &'static HashMap<&'static str, HashMap<String, Translated>> {
    static T: OnceLock<HashMap<&'static str, HashMap<String, Translated>>> = OnceLock::new();
    T.get_or_init(|| {
        TRANSLATED
            .iter()
            .map(|(lang, text)| (*lang, parsed_translations(text).unwrap_or_default()))
            .collect()
    })
}

/// A field of a landmark in the reader's language: French or English from
/// the catalogue, another language from its file when it gives the field,
/// English otherwise.
fn said(pair: &[String; 2], lang: &str, key: &str, field: impl Fn(&Translated) -> &str) -> String {
    if !lang.starts_with("fr") && !lang.starts_with("en") {
        if let Some(v) = translations()
            .get(lang)
            .and_then(|m| m.get(key))
            .map(&field)
            .filter(|v| !v.is_empty())
        {
            return v.to_string();
        }
    }
    pick(pair, lang).to_string()
}

/// The French of a pair when the reader reads French and there is one, the
/// English otherwise.
fn pick<'a>(pair: &'a [String; 2], lang: &str) -> &'a str {
    if lang.starts_with("fr") && !pair[1].is_empty() {
        &pair[1]
    } else {
        &pair[0]
    }
}

impl Landmark {
    /// Its name in the reader's language: French or English from the
    /// catalogue, another language from its file (docs/adr/0038).
    pub fn name(&self, lang: &str) -> String {
        said(&self.names, lang, &self.key, |t| &t.name)
    }

    /// Its explanation, when the catalogue gives one.
    pub fn note(&self, lang: &str) -> String {
        said(&self.notes, lang, &self.key, |t| &t.note)
    }

    pub fn start(&self) -> f64 {
        self.from.at()
    }

    /// When it ended: now for what goes on, its start for a moment.
    pub fn end(&self, now: f64) -> f64 {
        if self.ongoing {
            now
        } else {
            self.to.map_or(self.start(), When::at)
        }
    }

    pub fn is_world(&self) -> bool {
        self.scopes.contains(&Scope::World)
    }

    pub fn continents(&self) -> impl Iterator<Item = Continent> + '_ {
        self.scopes.iter().filter_map(|s| match s {
            Scope::Continent(c) => Some(*c),
            _ => None,
        })
    }

    pub fn countries(&self) -> impl Iterator<Item = &str> + '_ {
        self.scopes.iter().filter_map(|s| match s {
            Scope::Country(c) => Some(c.as_str()),
            _ => None,
        })
    }
}

const SOURCE: &str = include_str!("impact_events.txt");

/// A line of the catalogue, read; or what is wrong with it.
fn parse_line(line: &str) -> Result<Landmark, String> {
    let f: Vec<&str> = line.split('|').map(str::trim).collect();
    if !(6..=8).contains(&f.len()) {
        return Err(format!("{} fields", f.len()));
    }
    let kind = Kind::parse(f[0]).ok_or_else(|| format!("kind {:?}", f[0]))?;
    let from = When::parse(f[1]).ok_or_else(|| format!("from {:?}", f[1]))?;
    let (to, ongoing) = match f[2] {
        "" => (None, false),
        "now" => (None, true),
        s => (
            Some(When::parse(s).ok_or_else(|| format!("to {s:?}"))?),
            false,
        ),
    };
    if let Some(t) = to {
        if t.at() < from.at() {
            return Err("ends before it begins".into());
        }
    }
    let mut scopes = Vec::new();
    for s in f[3].split(',').map(str::trim) {
        scopes.push(match s {
            "world" => Scope::World,
            c if Continent::parse(c).is_some() => Scope::Continent(Continent::parse(c).unwrap()),
            iso if iso.len() == 2 && iso.chars().all(|c| c.is_ascii_uppercase()) => {
                if continent_of(iso).is_none() {
                    return Err(format!("unknown country {iso}"));
                }
                Scope::Country(iso.to_string())
            }
            other => return Err(format!("scope {other:?}")),
        });
    }
    if scopes.is_empty() || f[4].is_empty() || f[5].is_empty() {
        return Err("no scope or no name".into());
    }
    let note = |i: usize| f.get(i).map(|s| s.to_string()).unwrap_or_default();
    let key = format!("{}-{}", f[1], slug(f[4]));
    Ok(Landmark {
        key,
        kind,
        from,
        to,
        ongoing,
        scopes,
        names: [f[4].to_string(), f[5].to_string()],
        notes: [note(6), note(7)],
    })
}

fn slug(s: &str) -> String {
    let mut out = String::new();
    for c in fold(s).chars() {
        if c.is_ascii_alphanumeric() {
            out.push(c);
        } else if !out.ends_with('-') {
            out.push('-');
        }
    }
    out.trim_matches('-').to_string()
}

/// Every line of the catalogue, read: (its line number, what it gave).
pub fn parsed() -> Vec<(usize, Result<Landmark, String>)> {
    SOURCE
        .lines()
        .enumerate()
        .filter(|(_, l)| !l.trim().is_empty() && !l.trim_start().starts_with('#'))
        .map(|(i, l)| (i + 1, parse_line(l)))
        .collect()
}

/// The catalogue, read once. A line that cannot be read is left out (the
/// unit tests say which).
pub fn catalogue() -> &'static [Landmark] {
    static CATALOGUE: OnceLock<Vec<Landmark>> = OnceLock::new();
    CATALOGUE.get_or_init(|| {
        parsed()
            .into_iter()
            .filter_map(|(n, r)| match r {
                Ok(l) => Some(l),
                Err(e) => {
                    tracing::warn!(line = n, error = %e, "impact: a landmark left out");
                    None
                }
            })
            .collect()
    })
}

/// What a landmark's window says of it (the operator's request of
/// 6 November 2026): the context, what was at stake, and the victims where
/// there were, in English and French.
#[derive(Debug, Clone, Default)]
pub struct Details {
    pub context: [String; 2],
    pub stakes: [String; 2],
    pub victims: [String; 2],
}

const DETAILS: &str = include_str!("impact_details.txt");

/// The details, read: by landmark key, or what is wrong with a line.
pub fn parsed_details() -> Result<HashMap<String, Details>, String> {
    let mut out: HashMap<String, Details> = HashMap::new();
    let mut key: Option<String> = None;
    for (i, line) in DETAILS.lines().enumerate() {
        let line = line.trim_end();
        if line.is_empty() || (line.starts_with('#') && !line.starts_with("## ")) {
            continue;
        }
        if let Some(k) = line.strip_prefix("## ") {
            let k = k.trim().to_string();
            if out.contains_key(&k) {
                return Err(format!("line {}: {k} twice", i + 1));
            }
            out.insert(k.clone(), Details::default());
            key = Some(k);
            continue;
        }
        let f: Vec<&str> = line.split('|').map(str::trim).collect();
        let which = match f.first() {
            Some(&"en") => 0,
            Some(&"fr") => 1,
            _ => return Err(format!("line {}: {line:?}", i + 1)),
        };
        if f.len() != 4 || f[1].is_empty() {
            return Err(format!("line {}: {} fields", i + 1, f.len()));
        }
        let d = key
            .as_ref()
            .and_then(|k| out.get_mut(k))
            .ok_or_else(|| format!("line {}: no landmark", i + 1))?;
        d.context[which] = f[1].to_string();
        d.stakes[which] = f[2].to_string();
        d.victims[which] = f[3].to_string();
    }
    Ok(out)
}

/// The details, read once; none when they cannot be read (the unit tests
/// say why).
pub fn details() -> &'static HashMap<String, Details> {
    static DETAILED: OnceLock<HashMap<String, Details>> = OnceLock::new();
    DETAILED.get_or_init(|| {
        parsed_details().unwrap_or_else(|e| {
            tracing::warn!(error = %e, "impact: the landmarks' details left out");
            HashMap::new()
        })
    })
}

/// Today, as a number of years.
pub fn now() -> f64 {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs_f64())
        .unwrap_or(0.0);
    1970.0 + secs / (365.2425 * 86_400.0)
}

// ---------------------------------------------------------------------------
// dates and lengths, as the page says them
// ---------------------------------------------------------------------------

fn tr(lang: &str, key: &str, args: &[(&str, fluent::FluentValue<'static>)]) -> String {
    let mut a = fluent::FluentArgs::new();
    for (k, v) in args {
        a.set(*k, v.clone());
    }
    crate::i18n::translate(lang, key, Some(&a))
}

/// A year as the page writes it: "1914", "3500 BC".
pub fn year_text(lang: &str, year: i32) -> String {
    if year < 0 {
        tr(
            lang,
            "impact-year-bc",
            &[("year", (-year).to_string().into())],
        )
    } else {
        year.to_string()
    }
}

/// From one year to another, before Christ said once when both are.
pub fn span_text(lang: &str, from: i32, to: Option<i32>, ongoing: bool) -> String {
    if ongoing {
        return format!(
            "{} – {}",
            year_text(lang, from),
            tr(lang, "impact-now", &[])
        );
    }
    match to {
        Some(t) if t != from => {
            if from < 0 && t < 0 {
                tr(
                    lang,
                    "impact-year-bc",
                    &[("year", format!("{}–{}", -from, -t).into())],
                )
            } else if from < 0 {
                format!("{} – {}", year_text(lang, from), t)
            } else {
                format!("{from}–{t}")
            }
        }
        _ => year_text(lang, from),
    }
}

fn leap(y: i32) -> bool {
    (y % 4 == 0 && y % 100 != 0) || y % 400 == 0
}

fn days_in(y: i32, m: u32) -> i64 {
    match m {
        2 if leap(y) => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

/// How long from one moment to the next, in years, months and days, at the
/// precision both are given.
pub fn length(a: When, b: When) -> (i64, i64, i64) {
    match (a.month, a.day, b.month, b.day) {
        (Some(am), Some(ad), Some(bm), Some(bd)) => {
            let mut y = i64::from(b.year - a.year);
            let mut m = i64::from(bm) - i64::from(am);
            let mut d = i64::from(bd) - i64::from(ad);
            if d < 0 {
                m -= 1;
                let (py, pm) = if bm == 1 {
                    (b.year - 1, 12)
                } else {
                    (b.year, bm - 1)
                };
                d += days_in(py, pm);
            }
            if m < 0 {
                y -= 1;
                m += 12;
            }
            (y, m, d)
        }
        (Some(am), _, Some(bm), _) => {
            let months = i64::from(b.year - a.year) * 12 + i64::from(bm) - i64::from(am);
            (months / 12, months % 12, 0)
        }
        _ => (i64::from(b.year - a.year), 0, 0),
    }
}

/// The days from one day to another, for a short length.
fn days_between(a: When, b: When) -> Option<i64> {
    let day = |w: When| -> Option<i64> {
        let (m, d) = (w.month?, w.day?);
        let mut n = i64::from(d);
        for mm in 1..m {
            n += days_in(w.year, mm);
        }
        Some(i64::from(w.year) * 366 + n)
    };
    // Within one year or across a new year, which is all a short length is.
    if b.year - a.year > 1 {
        return None;
    }
    let (da, db) = (day(a)?, day(b)?);
    let ya = if a.year != b.year {
        (1..=12).map(|m| days_in(a.year, m)).sum::<i64>() - 366
    } else {
        0
    };
    Some(db - da - ya)
}

/// A length as the page says it: "4 years 3 months", "6 days", "100 days".
pub fn length_text(lang: &str, a: When, b: When) -> String {
    let unit = |key: &str, n: i64| tr(lang, key, &[("n", n.into())]);
    let (y, m, d) = length(a, b);
    if y == 0 {
        if let Some(n) = days_between(a, b).filter(|n| (1..=120).contains(n)) {
            return unit("impact-days", n);
        }
    }
    let mut parts = Vec::new();
    for (n, key) in [
        (y, "impact-years"),
        (m, "impact-months"),
        (d, "impact-days"),
    ] {
        if n > 0 {
            parts.push(unit(key, n));
        }
        if parts.len() == 2 {
            break;
        }
    }
    parts.join(" ")
}

// ---------------------------------------------------------------------------
// countries and continents
// ---------------------------------------------------------------------------

/// The ISO 3166 codes of each continent. Russia and Turkey are counted with
/// the continent most of the family's history there happened on.
const CONTINENTS: [(Continent, &str); 5] = [
    (
        Continent::Europe,
        "AD AL AT AX BA BE BG BY CH CY CZ DE DK EE ES FI FO FR GB GG GI GR HR HU IE IM IS IT JE \
         LI LT LU LV MC MD ME MK MT NL NO PL PT RO RS RU SE SI SK SM UA VA XK",
    ),
    (
        Continent::Africa,
        "AO BF BI BJ BW CD CF CG CI CM CV DJ DZ EG EH ER ET GA GH GM GN GQ GW KE KM LR LS LY MA \
         MG ML MR MU MW MZ NA NE NG RE RW SC SD SH SL SN SO SS ST SZ TD TG TN TZ UG YT ZA ZM ZW",
    ),
    (
        Continent::Asia,
        "AE AF AM AZ BD BH BN BT CN GE HK ID IL IN IQ IR JO JP KG KH KP KR KW KZ LA LB LK MM MN \
         MO MV MY NP OM PH PK PS QA SA SG SY TH TJ TL TM TR TW UZ VN YE",
    ),
    (
        Continent::Americas,
        "AG AI AR AW BB BL BM BO BQ BR BS BZ CA CL CO CR CU CW DM DO EC FK GD GF GL GP GT GY HN \
         HT JM KN KY LC MF MQ MS MX NI PA PE PM PR PY SR SV SX TC TT US UY VC VE VG VI",
    ),
    (
        Continent::Oceania,
        "AS AU CK FJ FM GU KI MH MP NC NF NR NU NZ PF PG PN PW SB TK TO TV VU WF WS",
    ),
];

/// The continent a country is on.
pub fn continent_of(iso: &str) -> Option<Continent> {
    CONTINENTS
        .iter()
        .find(|(_, codes)| codes.split_whitespace().any(|c| c == iso))
        .map(|(c, _)| *c)
}

/// The names of countries, folded (`fold`), as places write them in the
/// languages of the family's records.
const COUNTRY_NAMES: &[(&str, &str)] = &[
    ("france", "FR"),
    ("francja", "FR"),
    ("frankreich", "FR"),
    ("francia", "FR"),
    ("franca", "FR"),
    ("frankrijk", "FR"),
    ("pologne", "PL"),
    ("poland", "PL"),
    ("polska", "PL"),
    ("polen", "PL"),
    ("polonia", "PL"),
    ("belgique", "BE"),
    ("belgium", "BE"),
    ("belgie", "BE"),
    ("belgien", "BE"),
    ("belgia", "BE"),
    ("allemagne", "DE"),
    ("germany", "DE"),
    ("deutschland", "DE"),
    ("niemcy", "DE"),
    ("alemania", "DE"),
    ("germania", "DE"),
    ("alemanha", "DE"),
    ("italie", "IT"),
    ("italy", "IT"),
    ("italia", "IT"),
    ("wlochy", "IT"),
    ("italien", "IT"),
    ("espagne", "ES"),
    ("spain", "ES"),
    ("espana", "ES"),
    ("hiszpania", "ES"),
    ("spanien", "ES"),
    ("spagna", "ES"),
    ("portugal", "PT"),
    ("portugalia", "PT"),
    ("royaume-uni", "GB"),
    ("united kingdom", "GB"),
    ("angleterre", "GB"),
    ("england", "GB"),
    ("ecosse", "GB"),
    ("scotland", "GB"),
    ("pays de galles", "GB"),
    ("wales", "GB"),
    ("grande-bretagne", "GB"),
    ("great britain", "GB"),
    ("anglia", "GB"),
    ("wielka brytania", "GB"),
    ("irlande", "IE"),
    ("ireland", "IE"),
    ("eire", "IE"),
    ("irlandia", "IE"),
    ("russie", "RU"),
    ("russia", "RU"),
    ("rosja", "RU"),
    ("russland", "RU"),
    ("rusia", "RU"),
    ("россия", "RU"),
    ("ukraine", "UA"),
    ("ukraina", "UA"),
    ("ucrania", "UA"),
    ("україна", "UA"),
    ("etats-unis", "US"),
    ("etats-unis d'amerique", "US"),
    ("usa", "US"),
    ("united states", "US"),
    ("united states of america", "US"),
    ("stany zjednoczone", "US"),
    ("vereinigte staaten", "US"),
    ("estados unidos", "US"),
    ("canada", "CA"),
    ("kanada", "CA"),
    ("haiti", "HT"),
    ("rdc", "CD"),
    ("rd congo", "CD"),
    ("republique democratique du congo", "CD"),
    ("democratic republic of the congo", "CD"),
    ("dr congo", "CD"),
    ("congo-kinshasa", "CD"),
    ("zaire", "CD"),
    ("congo belge", "CD"),
    ("belgian congo", "CD"),
    ("congo-brazzaville", "CG"),
    ("republique du congo", "CG"),
    ("republic of the congo", "CG"),
    ("congo francais", "CG"),
    ("moyen-congo", "CG"),
    ("algerie", "DZ"),
    ("algeria", "DZ"),
    ("algieria", "DZ"),
    ("algerien", "DZ"),
    ("maroc", "MA"),
    ("morocco", "MA"),
    ("marokko", "MA"),
    ("maroko", "MA"),
    ("marruecos", "MA"),
    ("tunisie", "TN"),
    ("tunisia", "TN"),
    ("senegal", "SN"),
    ("cameroun", "CM"),
    ("cameroon", "CM"),
    ("cote d'ivoire", "CI"),
    ("ivory coast", "CI"),
    ("rwanda", "RW"),
    ("burundi", "BI"),
    ("angola", "AO"),
    ("afrique du sud", "ZA"),
    ("south africa", "ZA"),
    ("pays-bas", "NL"),
    ("netherlands", "NL"),
    ("hollande", "NL"),
    ("holland", "NL"),
    ("nederland", "NL"),
    ("holandia", "NL"),
    ("suisse", "CH"),
    ("switzerland", "CH"),
    ("schweiz", "CH"),
    ("szwajcaria", "CH"),
    ("svizzera", "CH"),
    ("autriche", "AT"),
    ("austria", "AT"),
    ("osterreich", "AT"),
    ("tchequie", "CZ"),
    ("republique tcheque", "CZ"),
    ("czech republic", "CZ"),
    ("czechia", "CZ"),
    ("czechy", "CZ"),
    ("lituanie", "LT"),
    ("lithuania", "LT"),
    ("litwa", "LT"),
    ("lietuva", "LT"),
    ("bielorussie", "BY"),
    ("belarus", "BY"),
    ("bialorus", "BY"),
    ("hongrie", "HU"),
    ("hungary", "HU"),
    ("wegry", "HU"),
    ("magyarorszag", "HU"),
    ("roumanie", "RO"),
    ("romania", "RO"),
    ("rumunia", "RO"),
    ("grece", "GR"),
    ("greece", "GR"),
    ("turquie", "TR"),
    ("turkey", "TR"),
    ("turcja", "TR"),
    ("turkiye", "TR"),
    ("israel", "IL"),
    ("izrael", "IL"),
    ("luxembourg", "LU"),
    ("bresil", "BR"),
    ("brazil", "BR"),
    ("brasil", "BR"),
    ("brazylia", "BR"),
    ("argentine", "AR"),
    ("argentina", "AR"),
    ("mexique", "MX"),
    ("mexico", "MX"),
    ("meksyk", "MX"),
    ("australie", "AU"),
    ("australia", "AU"),
    ("chine", "CN"),
    ("china", "CN"),
    ("chiny", "CN"),
    ("japon", "JP"),
    ("japan", "JP"),
    ("japonia", "JP"),
    ("inde", "IN"),
    ("india", "IN"),
    ("indie", "IN"),
    ("liban", "LB"),
    ("lebanon", "LB"),
    ("vietnam", "VN"),
    ("viet nam", "VN"),
];

/// Well-known cities, folded, for a place that names nothing else.
const CITIES: &[(&str, &str)] = &[
    ("warszawa", "PL"),
    ("warsaw", "PL"),
    ("varsovie", "PL"),
    ("warschau", "PL"),
    ("krakow", "PL"),
    ("cracovie", "PL"),
    ("krakau", "PL"),
    ("lodz", "PL"),
    ("wroclaw", "PL"),
    ("breslau", "PL"),
    ("poznan", "PL"),
    ("posen", "PL"),
    ("gdansk", "PL"),
    ("dantzig", "PL"),
    ("danzig", "PL"),
    ("lublin", "PL"),
    ("katowice", "PL"),
    ("bialystok", "PL"),
    ("czestochowa", "PL"),
    ("radom", "PL"),
    ("kielce", "PL"),
    ("torun", "PL"),
    ("bydgoszcz", "PL"),
    ("szczecin", "PL"),
    ("stettin", "PL"),
    ("rzeszow", "PL"),
    ("tarnow", "PL"),
    ("lwow", "UA"),
    ("lviv", "UA"),
    ("lemberg", "UA"),
    ("lvov", "UA"),
    ("kyiv", "UA"),
    ("kiev", "UA"),
    ("kijow", "UA"),
    ("odessa", "UA"),
    ("odesa", "UA"),
    ("kharkiv", "UA"),
    ("kharkov", "UA"),
    ("wilno", "LT"),
    ("vilnius", "LT"),
    ("kaunas", "LT"),
    ("kowno", "LT"),
    ("minsk", "BY"),
    ("paris", "FR"),
    ("lyon", "FR"),
    ("marseille", "FR"),
    ("lille", "FR"),
    ("bordeaux", "FR"),
    ("toulouse", "FR"),
    ("nice", "FR"),
    ("nantes", "FR"),
    ("strasbourg", "FR"),
    ("rouen", "FR"),
    ("reims", "FR"),
    ("metz", "FR"),
    ("nancy", "FR"),
    ("lens", "FR"),
    ("roubaix", "FR"),
    ("saint-etienne", "FR"),
    ("grenoble", "FR"),
    ("montpellier", "FR"),
    ("le havre", "FR"),
    ("toulon", "FR"),
    ("dijon", "FR"),
    ("brest", "FR"),
    ("angers", "FR"),
    ("limoges", "FR"),
    ("clermont-ferrand", "FR"),
    ("valenciennes", "FR"),
    ("douai", "FR"),
    ("calais", "FR"),
    ("bruxelles", "BE"),
    ("brussel", "BE"),
    ("brussels", "BE"),
    ("anvers", "BE"),
    ("antwerpen", "BE"),
    ("liege", "BE"),
    ("gand", "BE"),
    ("gent", "BE"),
    ("charleroi", "BE"),
    ("namur", "BE"),
    ("louvain", "BE"),
    ("leuven", "BE"),
    ("mons", "BE"),
    ("bruges", "BE"),
    ("brugge", "BE"),
    ("kinshasa", "CD"),
    ("leopoldville", "CD"),
    ("lubumbashi", "CD"),
    ("elisabethville", "CD"),
    ("kisangani", "CD"),
    ("stanleyville", "CD"),
    ("mbandaka", "CD"),
    ("coquilhatville", "CD"),
    ("matadi", "CD"),
    ("boma", "CD"),
    ("bukavu", "CD"),
    ("goma", "CD"),
    ("kananga", "CD"),
    ("luluabourg", "CD"),
    ("mbuji-mayi", "CD"),
    ("kikwit", "CD"),
    ("likasi", "CD"),
    ("kolwezi", "CD"),
    ("bandundu", "CD"),
    ("brazzaville", "CG"),
    ("pointe-noire", "CG"),
    ("dolisie", "CG"),
    ("port-au-prince", "HT"),
    ("cap-haitien", "HT"),
    ("jacmel", "HT"),
    ("les cayes", "HT"),
    ("gonaives", "HT"),
    ("jeremie", "HT"),
    ("petion-ville", "HT"),
    ("berlin", "DE"),
    ("hamburg", "DE"),
    ("hambourg", "DE"),
    ("munchen", "DE"),
    ("munich", "DE"),
    ("koln", "DE"),
    ("cologne", "DE"),
    ("frankfurt", "DE"),
    ("francfort", "DE"),
    ("dusseldorf", "DE"),
    ("dortmund", "DE"),
    ("essen", "DE"),
    ("leipzig", "DE"),
    ("dresden", "DE"),
    ("dresde", "DE"),
    ("stuttgart", "DE"),
    ("bremen", "DE"),
    ("hannover", "DE"),
    ("roma", "IT"),
    ("rome", "IT"),
    ("milano", "IT"),
    ("milan", "IT"),
    ("napoli", "IT"),
    ("naples", "IT"),
    ("torino", "IT"),
    ("turin", "IT"),
    ("firenze", "IT"),
    ("florence", "IT"),
    ("venezia", "IT"),
    ("venise", "IT"),
    ("palermo", "IT"),
    ("genova", "IT"),
    ("bologna", "IT"),
    ("madrid", "ES"),
    ("barcelona", "ES"),
    ("barcelone", "ES"),
    ("valencia", "ES"),
    ("sevilla", "ES"),
    ("seville", "ES"),
    ("lisboa", "PT"),
    ("lisbonne", "PT"),
    ("lisbon", "PT"),
    ("porto", "PT"),
    ("london", "GB"),
    ("londres", "GB"),
    ("manchester", "GB"),
    ("liverpool", "GB"),
    ("birmingham", "GB"),
    ("glasgow", "GB"),
    ("edinburgh", "GB"),
    ("edimbourg", "GB"),
    ("dublin", "IE"),
    ("cork", "IE"),
    ("moskva", "RU"),
    ("moscow", "RU"),
    ("moscou", "RU"),
    ("moskwa", "RU"),
    ("saint-petersbourg", "RU"),
    ("saint petersburg", "RU"),
    ("sankt-peterburg", "RU"),
    ("petrograd", "RU"),
    ("leningrad", "RU"),
    ("new york", "US"),
    ("chicago", "US"),
    ("boston", "US"),
    ("philadelphia", "US"),
    ("detroit", "US"),
    ("pittsburgh", "US"),
    ("los angeles", "US"),
    ("san francisco", "US"),
    ("new orleans", "US"),
    ("buffalo", "US"),
    ("cleveland", "US"),
    ("milwaukee", "US"),
    ("montreal", "CA"),
    ("quebec", "CA"),
    ("toronto", "CA"),
    ("alger", "DZ"),
    ("algiers", "DZ"),
    ("oran", "DZ"),
    ("constantine", "DZ"),
    ("annaba", "DZ"),
    ("casablanca", "MA"),
    ("rabat", "MA"),
    ("marrakech", "MA"),
    ("fes", "MA"),
    ("tanger", "MA"),
    ("tunis", "TN"),
    ("dakar", "SN"),
    ("jerusalem", "IL"),
    ("tel aviv", "IL"),
    ("haifa", "IL"),
    ("geneve", "CH"),
    ("zurich", "CH"),
    ("berne", "CH"),
    ("lausanne", "CH"),
    ("bale", "CH"),
    ("basel", "CH"),
    ("wien", "AT"),
    ("vienne", "AT"),
    ("vienna", "AT"),
    ("graz", "AT"),
    ("salzburg", "AT"),
    ("amsterdam", "NL"),
    ("rotterdam", "NL"),
    ("la haye", "NL"),
    ("den haag", "NL"),
    ("budapest", "HU"),
    ("praha", "CZ"),
    ("prague", "CZ"),
    ("bucuresti", "RO"),
    ("bucarest", "RO"),
    ("bucharest", "RO"),
    ("sao paulo", "BR"),
    ("rio de janeiro", "BR"),
    ("buenos aires", "AR"),
];

/// A country's name for the page, in English or French; the page's script
/// says it in the reader's language when the browser knows how.
const COUNTRY_WORDS: &[(&str, &str, &str)] = &[
    ("FR", "France", "France"),
    ("PL", "Poland", "Pologne"),
    ("BE", "Belgium", "Belgique"),
    ("DE", "Germany", "Allemagne"),
    ("IT", "Italy", "Italie"),
    ("ES", "Spain", "Espagne"),
    ("PT", "Portugal", "Portugal"),
    ("GB", "United Kingdom", "Royaume-Uni"),
    ("IE", "Ireland", "Irlande"),
    ("RU", "Russia", "Russie"),
    ("UA", "Ukraine", "Ukraine"),
    ("US", "United States", "États-Unis"),
    ("CA", "Canada", "Canada"),
    ("HT", "Haiti", "Haïti"),
    ("CD", "DR Congo", "RD Congo"),
    ("CG", "Republic of the Congo", "République du Congo"),
    ("DZ", "Algeria", "Algérie"),
    ("MA", "Morocco", "Maroc"),
    ("TN", "Tunisia", "Tunisie"),
    ("SN", "Senegal", "Sénégal"),
    ("CM", "Cameroon", "Cameroun"),
    ("CI", "Côte d'Ivoire", "Côte d'Ivoire"),
    ("NL", "Netherlands", "Pays-Bas"),
    ("CH", "Switzerland", "Suisse"),
    ("AT", "Austria", "Autriche"),
    ("CZ", "Czechia", "Tchéquie"),
    ("LT", "Lithuania", "Lituanie"),
    ("BY", "Belarus", "Biélorussie"),
    ("HU", "Hungary", "Hongrie"),
    ("RO", "Romania", "Roumanie"),
    ("IL", "Israel", "Israël"),
    ("BR", "Brazil", "Brésil"),
    ("AR", "Argentina", "Argentine"),
    ("LU", "Luxembourg", "Luxembourg"),
];

/// A country's name for the page.
pub fn country_name(iso: &str, lang: &str) -> String {
    COUNTRY_WORDS
        .iter()
        .find(|(c, _, _)| *c == iso)
        .map(|(_, en, fr)| if lang.starts_with("fr") { *fr } else { *en })
        .unwrap_or(iso)
        .to_string()
}

/// A name made comparable: lower case, without its accents.
fn fold(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.trim().chars().flat_map(char::to_lowercase) {
        match c {
            'à' | 'á' | 'â' | 'ã' | 'ä' | 'å' | 'ą' => out.push('a'),
            'ç' | 'ć' | 'č' => out.push('c'),
            'è' | 'é' | 'ê' | 'ë' | 'ę' | 'ě' => out.push('e'),
            'ì' | 'í' | 'î' | 'ï' => out.push('i'),
            'ł' => out.push('l'),
            'ñ' | 'ń' | 'ň' => out.push('n'),
            'ò' | 'ó' | 'ô' | 'õ' | 'ö' | 'ø' => out.push('o'),
            'ś' | 'š' | 'ş' => out.push('s'),
            'ù' | 'ú' | 'û' | 'ü' | 'ů' => out.push('u'),
            'ý' | 'ÿ' => out.push('y'),
            'ź' | 'ż' | 'ž' => out.push('z'),
            'ß' => out.push_str("ss"),
            '’' => out.push('\''),
            _ => out.push(c),
        }
    }
    out
}

fn lookup(table: &[(&str, &'static str)], name: &str) -> Option<&'static str> {
    let f = fold(name);
    table.iter().find(|(k, _)| *k == f).map(|(_, iso)| *iso)
}

/// Every populated place of a few countries (Poland, France, Belgium,
/// Ukraine, the DR Congo, Haiti), by its folded name, from GeoNames
/// (geonames.org, CC BY 4.0): the villages a family's records name alone.
/// A name two of them share is not in it.
const GAZETTEER: &str = include_str!("impact_places.txt");

/// Words a record writes for a country or a continent, never looked for in
/// the gazetteer.
const NOT_VILLAGES: &[&str] = &[
    "congo", "europe", "afrique", "africa", "amerique", "america", "asie", "asia", "oceanie",
];

fn gazetteer() -> &'static HashMap<&'static str, &'static str> {
    static PLACES: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();
    PLACES.get_or_init(|| {
        let mut m = HashMap::new();
        for line in GAZETTEER.lines().filter(|l| !l.starts_with('#')) {
            let mut fields = line.split('|');
            let Some(iso) = fields.next() else {
                continue;
            };
            for name in fields {
                m.insert(name, iso);
            }
        }
        m
    })
}

/// A country's flag as an SVG, for the countries the resolver can name
/// (flag-icons 7.2.3, MIT: `static/flags/LICENSE`).
pub fn flag_svg(iso: &str) -> Option<&'static str> {
    Some(match iso {
        "AO" => include_str!("../static/flags/ao.svg"),
        "AR" => include_str!("../static/flags/ar.svg"),
        "AT" => include_str!("../static/flags/at.svg"),
        "AU" => include_str!("../static/flags/au.svg"),
        "BE" => include_str!("../static/flags/be.svg"),
        "BI" => include_str!("../static/flags/bi.svg"),
        "BR" => include_str!("../static/flags/br.svg"),
        "BY" => include_str!("../static/flags/by.svg"),
        "CA" => include_str!("../static/flags/ca.svg"),
        "CD" => include_str!("../static/flags/cd.svg"),
        "CG" => include_str!("../static/flags/cg.svg"),
        "CH" => include_str!("../static/flags/ch.svg"),
        "CI" => include_str!("../static/flags/ci.svg"),
        "CM" => include_str!("../static/flags/cm.svg"),
        "CN" => include_str!("../static/flags/cn.svg"),
        "CZ" => include_str!("../static/flags/cz.svg"),
        "DE" => include_str!("../static/flags/de.svg"),
        "DZ" => include_str!("../static/flags/dz.svg"),
        "ES" => include_str!("../static/flags/es.svg"),
        "FR" => include_str!("../static/flags/fr.svg"),
        "GB" => include_str!("../static/flags/gb.svg"),
        "GR" => include_str!("../static/flags/gr.svg"),
        "HT" => include_str!("../static/flags/ht.svg"),
        "HU" => include_str!("../static/flags/hu.svg"),
        "IE" => include_str!("../static/flags/ie.svg"),
        "IL" => include_str!("../static/flags/il.svg"),
        "IN" => include_str!("../static/flags/in.svg"),
        "IT" => include_str!("../static/flags/it.svg"),
        "JP" => include_str!("../static/flags/jp.svg"),
        "LB" => include_str!("../static/flags/lb.svg"),
        "LT" => include_str!("../static/flags/lt.svg"),
        "LU" => include_str!("../static/flags/lu.svg"),
        "MA" => include_str!("../static/flags/ma.svg"),
        "MX" => include_str!("../static/flags/mx.svg"),
        "NL" => include_str!("../static/flags/nl.svg"),
        "PL" => include_str!("../static/flags/pl.svg"),
        "PT" => include_str!("../static/flags/pt.svg"),
        "RO" => include_str!("../static/flags/ro.svg"),
        "RU" => include_str!("../static/flags/ru.svg"),
        "RW" => include_str!("../static/flags/rw.svg"),
        "SN" => include_str!("../static/flags/sn.svg"),
        "TN" => include_str!("../static/flags/tn.svg"),
        "TR" => include_str!("../static/flags/tr.svg"),
        "UA" => include_str!("../static/flags/ua.svg"),
        "US" => include_str!("../static/flags/us.svg"),
        "VN" => include_str!("../static/flags/vn.svg"),
        "ZA" => include_str!("../static/flags/za.svg"),
        _ => return None,
    })
}

/// Where the page finds a country's flag, when there is one.
pub fn flag_url(iso: &str) -> Option<String> {
    flag_svg(iso).map(|_| format!("/static/flags/{}.svg", iso.to_ascii_lowercase()))
}

/// The present-day country of a place: the one it states, the one the last
/// part of its name names, the one a well-known city in it is in, the one
/// its region names, or the one a populated place of that name is in.
pub fn country_of_place(place: &Value) -> Option<String> {
    if let Some(c) = place.get("country_current").and_then(Value::as_str) {
        let c = c.trim();
        if c.len() == 2 && c.chars().all(|x| x.is_ascii_alphabetic()) {
            let iso = c.to_ascii_uppercase();
            if continent_of(&iso).is_some() {
                return Some(iso);
            }
        }
        if let Some(iso) = lookup(COUNTRY_NAMES, c) {
            return Some(iso.to_string());
        }
    }
    let names = place.get("names").and_then(Value::as_array);
    for n in names.into_iter().flatten() {
        let Some(v) = n.get("value").and_then(Value::as_str) else {
            continue;
        };
        let parts: Vec<&str> = v
            .split(',')
            .map(str::trim)
            .filter(|p| !p.is_empty())
            .collect();
        if let Some(iso) = parts.last().and_then(|p| lookup(COUNTRY_NAMES, p)) {
            return Some(iso.to_string());
        }
        if let Some(iso) = parts.first().and_then(|p| lookup(CITIES, p)) {
            return Some(iso.to_string());
        }
    }
    if let Some(iso) = place
        .get("region")
        .and_then(Value::as_str)
        .and_then(|r| lookup(COUNTRY_NAMES, r))
    {
        return Some(iso.to_string());
    }
    for n in names.into_iter().flatten() {
        let first = n
            .get("value")
            .and_then(Value::as_str)
            .and_then(|v| v.split(',').map(str::trim).find(|p| !p.is_empty()));
        let Some(name) = first.map(fold) else {
            continue;
        };
        // A word that names a country or a continent rather than a place
        // ("Congo" could be either Congo, and a hamlet in Haiti) is not
        // looked for as a village.
        if NOT_VILLAGES.contains(&name.as_str()) {
            continue;
        }
        if let Some(iso) = gazetteer().get(name.as_str()) {
            return Some(iso.to_string());
        }
    }
    None
}

// ---------------------------------------------------------------------------
// the family's lives
// ---------------------------------------------------------------------------

/// One person's life, as the timeline lays it: always a window, never more
/// than [`LONGEST_LIFE`] years where a date is missing (docs/adr/0036).
#[derive(Debug, Clone, Serialize)]
pub struct Life {
    pub id: String,
    pub name: String,
    /// Where the window begins and ends, in years.
    pub from: f64,
    pub to: f64,
    pub living: bool,
    /// An end recorded only approximately ("about 1780").
    pub fuzzy_from: bool,
    pub fuzzy_to: bool,
    /// An end no date gives: the window is estimated from the other end, or
    /// from the moments the person was seen.
    pub open_from: bool,
    pub open_to: bool,
    /// "1847–1935", "1990–", "° 1750", "† 1810", "~ 1780", "~ 1780–1830".
    pub years: String,
    /// Their countries, the commonest first.
    pub countries: Vec<String>,
    #[serde(skip)]
    pub birth: Option<When>,
}

/// A date of the record as a moment: its year, month and day as far as they
/// are known, and whether it is only approximate.
fn moment_of(holder: Option<&Value>) -> Option<(When, bool)> {
    let holder = holder?;
    let d = crate::view::render_date_field(holder, "date");
    let sort = d.sort?;
    let (y, m, day) = (sort / 10_000, (sort / 100) % 100, sort % 100);
    let when = When {
        year: i32::try_from(y).ok()?,
        month: u32::try_from(m).ok().filter(|m| (1..=12).contains(m)),
        day: u32::try_from(day).ok().filter(|d| (1..=31).contains(d)),
    };
    let circa = holder
        .get("date")
        .and_then(|x| x.get("circa"))
        .and_then(Value::as_bool)
        .unwrap_or(false);
    Some((when, circa || d.kind == "range"))
}

/// For each person, the years of the events they took part in and the places
/// of those events.
struct EventIndex {
    years: HashMap<String, Vec<When>>,
    places: HashMap<String, Vec<String>>,
}

fn event_index(flat: &Value) -> EventIndex {
    let mut years: HashMap<String, Vec<When>> = HashMap::new();
    let mut places: HashMap<String, Vec<String>> = HashMap::new();
    for e in flat
        .get("events")
        .and_then(Value::as_object)
        .into_iter()
        .flat_map(|m| m.values())
    {
        let when = moment_of(Some(e)).map(|(w, _)| w);
        let place = e.get("place_id").and_then(Value::as_str);
        for p in e
            .get("participants")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
        {
            if p.get("entity_type").and_then(Value::as_str) != Some("person") {
                continue;
            }
            let Some(id) = p.get("entity_id").and_then(Value::as_str) else {
                continue;
            };
            if let Some(w) = when {
                years.entry(id.to_string()).or_default().push(w);
            }
            if let Some(pl) = place {
                places
                    .entry(id.to_string())
                    .or_default()
                    .push(pl.to_string());
            }
        }
    }
    EventIndex { years, places }
}

/// The countries of a person's places, the commonest first.
fn countries_of(flat: &Value, person: &Value, more: Option<&Vec<String>>) -> Vec<String> {
    let mut ids: Vec<&str> = ["birth", "death", "burial", "baptism"]
        .iter()
        .filter_map(|k| person.get(k)?.get("place_id")?.as_str())
        .collect();
    ids.extend(more.into_iter().flatten().map(String::as_str));
    let mut count: BTreeMap<String, usize> = BTreeMap::new();
    for id in ids {
        if let Some(iso) = flat
            .get("places")
            .and_then(|p| p.get(id))
            .and_then(country_of_place)
        {
            *count.entry(iso).or_default() += 1;
        }
    }
    let mut out: Vec<(String, usize)> = count.into_iter().collect();
    out.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    out.into_iter().map(|(c, _)| c).collect()
}

/// One person's life, when it can be placed.
/// The longest a life is drawn without both its dates (the operator's rule
/// of 8 November 2026, docs/adr/0036): a hundred years from the one end that
/// is known, or fifty years either side of the moments the person was seen.
/// Never past today.
pub const LONGEST_LIFE: f64 = 100.0;

/// A person's window.
///
/// - Born and died: from the one to the other, as recorded.
/// - Born, and living by the record: to today, if the birth is no more than
///   a hundred years ago. A record that says living of somebody born longer
///   ago than that never learnt of the death; it is read as born only.
/// - Born only: a hundred years from the birth.
/// - Died only: the hundred years before the death.
/// - Only seen, at events: fifty years either side of the middle of those
///   moments, and at least from the first to the last.
fn life_of(flat: &Value, id: &str, p: &Value, idx: &EventIndex, now: f64) -> Option<Life> {
    let said_living = p
        .get("identity")
        .and_then(|i| i.get("is_living"))
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let birth = moment_of(p.get("birth"));
    let living = said_living && birth.is_none_or(|(b, _)| now - b.at() <= LONGEST_LIFE);
    let death = if living {
        None
    } else {
        moment_of(p.get("death"))
    };
    let window = |from: f64, to: f64| Life {
        id: id.to_string(),
        name: crate::view::person_display_name(p),
        from,
        to,
        living: false,
        fuzzy_from: false,
        fuzzy_to: false,
        open_from: false,
        open_to: false,
        years: String::new(),
        countries: Vec::new(),
        birth: None,
    };
    let mut life = match (birth, death) {
        (Some((b, fb)), Some((d, fd))) if d.at() >= b.at() => Life {
            fuzzy_from: fb,
            fuzzy_to: fd,
            years: format!("{}–{}", b.year, d.year),
            birth: Some(b),
            ..window(b.at(), d.at())
        },
        (Some((b, fb)), _) if living => Life {
            living: true,
            fuzzy_from: fb,
            years: format!("{}–", b.year),
            birth: Some(b),
            ..window(b.at(), now.max(b.at()))
        },
        (Some((b, fb)), _) => Life {
            fuzzy_from: fb,
            open_to: true,
            years: format!("° {}", b.year),
            birth: Some(b),
            ..window(b.at(), (b.at() + LONGEST_LIFE).min(now).max(b.at()))
        },
        (None, Some((d, fd))) => Life {
            fuzzy_to: fd,
            open_from: true,
            years: format!("† {}", d.year),
            ..window(d.at() - LONGEST_LIFE, d.at())
        },
        (None, None) => {
            let seen = idx.years.get(id)?;
            let first = seen.iter().min_by(|a, b| a.at().total_cmp(&b.at()))?;
            let last = seen.iter().max_by(|a, b| a.at().total_cmp(&b.at()))?;
            let (a, z) = (first.at(), last.at());
            let mid = (a + z) / 2.0;
            let half = LONGEST_LIFE / 2.0;
            let to = z.max(mid + half).min(now);
            let living = said_living && to >= now;
            Life {
                living,
                open_from: true,
                open_to: !living,
                years: if first.year == last.year {
                    format!("~ {}", first.year)
                } else {
                    format!("~ {}–{}", first.year, last.year)
                },
                ..window(a.min(mid - half), to)
            }
        }
    };
    life.countries = countries_of(flat, p, idx.places.get(id));
    Some(life)
}

/// Every person the reader may read, placed or not.
pub struct Lives {
    pub lives: Vec<Life>,
    /// Readable, but with no date to place them by.
    pub undated: usize,
    /// Not readable by this reader: counted, never named.
    pub hidden: usize,
}

pub fn lives(flat: &Value, lens: &Lens, now: f64) -> Lives {
    let idx = event_index(flat);
    let mut out = Lives {
        lives: Vec::new(),
        undated: 0,
        hidden: 0,
    };
    for (id, p) in flat
        .get("persons")
        .and_then(Value::as_object)
        .into_iter()
        .flat_map(|m| m.iter())
    {
        if !lens.sees_person(id) {
            out.hidden += 1;
            continue;
        }
        match life_of(flat, id, p, &idx, now) {
            Some(l) => out.lives.push(l),
            None => out.undated += 1,
        }
    }
    out.lives
        .sort_by(|a, b| a.from.total_cmp(&b.from).then(a.id.cmp(&b.id)));
    out
}

/// Whether a person can be placed on the timeline: the record's button.
pub fn placeable(flat: &Value, id: &str) -> bool {
    let Some(p) = flat.get("persons").and_then(|m| m.get(id)) else {
        return false;
    };
    moment_of(p.get("birth")).is_some()
        || moment_of(p.get("death")).is_some()
        || event_index(flat).years.contains_key(id)
}

// ---------------------------------------------------------------------------
// the axis
// ---------------------------------------------------------------------------

/// How years become heights: a line through stops, steeper where the family
/// lived.
#[derive(Debug, Clone)]
pub struct Scale {
    stops: Vec<(f64, f64)>,
}

/// The years the axis begins with: the end of the last ice age.
pub const DAWN: f64 = -10_000.0;

impl Scale {
    /// The whole of history: prehistory folded small at the top, the last
    /// two centuries at seven pixels a year.
    pub fn history(now: f64) -> Scale {
        let mut stops = vec![
            (DAWN, 0.0),
            (-3_500.0, 210.0),
            (0.0, 630.0),
            (1_000.0, 980.0),
            (1_500.0, 1_330.0),
            (1_800.0, 1_930.0),
        ];
        stops.push((now, 1_930.0 + (now - 1_800.0) * 7.0));
        Scale { stops }
    }

    /// One straight line, `px` pixels a year.
    pub fn linear(from: f64, to: f64, px: f64) -> Scale {
        Scale {
            stops: vec![(from, 0.0), (to, (to - from) * px)],
        }
    }

    pub fn start(&self) -> f64 {
        self.stops[0].0
    }

    pub fn end(&self) -> f64 {
        self.stops[self.stops.len() - 1].0
    }

    pub fn height(&self) -> f64 {
        self.stops[self.stops.len() - 1].1
    }

    /// The height of a year, held within the axis.
    pub fn y(&self, year: f64) -> f64 {
        let year = year.clamp(self.start(), self.end());
        for w in self.stops.windows(2) {
            let ((y0, p0), (y1, p1)) = (w[0], w[1]);
            if year <= y1 {
                return if y1 > y0 {
                    p0 + (year - y0) / (y1 - y0) * (p1 - p0)
                } else {
                    p0
                };
            }
        }
        self.height()
    }

    /// The years written on the axis: in each stretch, the roundest step
    /// that leaves room between two of them.
    pub fn ticks(&self) -> Vec<f64> {
        const STEPS: [f64; 10] = [
            1.0, 5.0, 10.0, 25.0, 50.0, 100.0, 250.0, 500.0, 1_000.0, 2_500.0,
        ];
        let mut out: Vec<f64> = Vec::new();
        for w in self.stops.windows(2) {
            let ((y0, p0), (y1, p1)) = (w[0], w[1]);
            if y1 <= y0 {
                continue;
            }
            let per_year = (p1 - p0) / (y1 - y0);
            let step = STEPS
                .into_iter()
                .find(|s| s * per_year >= 56.0)
                .unwrap_or(5_000.0);
            let mut t = (y0 / step).ceil() * step;
            while t < y1 {
                out.push(t);
                t += step;
            }
        }
        let mut kept: Vec<f64> = Vec::new();
        for t in out {
            if kept.last().is_none_or(|k| self.y(t) - self.y(*k) >= 40.0) {
                kept.push(t);
            }
        }
        kept
    }

    /// The graduation (the operator's request of 6 November 2026): a mark
    /// for every year wherever a year has four pixels, for every five, ten,
    /// twenty-five… years where it has not. Each with its step.
    pub fn grid(&self) -> Vec<(f64, f64)> {
        const STEPS: [f64; 9] = [1.0, 5.0, 10.0, 25.0, 50.0, 100.0, 250.0, 500.0, 1_000.0];
        let mut out = Vec::new();
        for w in self.stops.windows(2) {
            let ((y0, p0), (y1, p1)) = (w[0], w[1]);
            if y1 <= y0 {
                continue;
            }
            let per_year = (p1 - p0) / (y1 - y0);
            let step = STEPS
                .into_iter()
                .find(|s| s * per_year >= 4.0)
                .unwrap_or(2_500.0);
            let mut t = (y0 / step).ceil() * step;
            while t < y1 {
                out.push((t, step));
                t += step;
            }
        }
        out
    }
}

/// A year written on the axis.
#[derive(Debug, Clone, Serialize)]
pub struct Tick {
    pub y: i64,
    pub label: String,
}

/// A line of the graduation, across every column; the strong ones every ten
/// marks.
#[derive(Debug, Clone, Serialize)]
pub struct GridLine {
    pub y: i64,
    pub strong: bool,
}

fn grid(scale: &Scale) -> Vec<GridLine> {
    scale
        .grid()
        .into_iter()
        .map(|(t, step)| GridLine {
            y: scale.y(t).round() as i64,
            strong: (t / (step * 10.0)).fract() == 0.0
                || step >= 100.0 && (t / 1_000.0).fract() == 0.0,
        })
        .collect()
}

fn ticks(scale: &Scale, lang: &str) -> Vec<Tick> {
    scale
        .ticks()
        .into_iter()
        .map(|t| Tick {
            y: scale.y(t).round() as i64,
            label: year_text(lang, t as i32),
        })
        .collect()
}

// ---------------------------------------------------------------------------
// placing
// ---------------------------------------------------------------------------

/// Spans in lanes, side by side, none over another: each goes into the
/// first lane free at its top. `gap` keeps two in a lane apart.
pub fn pack(spans: &[(f64, f64)], gap: f64) -> (Vec<usize>, usize) {
    let mut order: Vec<usize> = (0..spans.len()).collect();
    order.sort_by(|&a, &b| spans[a].0.total_cmp(&spans[b].0));
    let mut ends: Vec<f64> = Vec::new();
    let mut lane = vec![0; spans.len()];
    for i in order {
        let (top, bottom) = spans[i];
        match ends.iter().position(|e| *e + gap <= top) {
            Some(l) => {
                lane[i] = l;
                ends[l] = bottom;
            }
            None => {
                lane[i] = ends.len();
                ends.push(bottom);
            }
        }
    }
    let n = ends.len();
    (lane, n)
}

/// Labels of one height in `lanes` lanes, each as near its own year as the
/// labels above it let it be: in the first lane free at its year, or pushed
/// down in the lane that frees first. Returns each one's lane and top.
pub fn stack(ys: &[f64], height: f64, lanes: usize, gap: f64) -> Vec<(usize, f64)> {
    let mut order: Vec<usize> = (0..ys.len()).collect();
    order.sort_by(|&a, &b| ys[a].total_cmp(&ys[b]));
    let mut bottoms = vec![f64::NEG_INFINITY; lanes.max(1)];
    let mut out = vec![(0, 0.0); ys.len()];
    for i in order {
        let y = ys[i];
        let (lane, top) = match bottoms.iter().position(|b| *b + gap <= y) {
            Some(l) => (l, y),
            None => {
                let (l, b) = bottoms
                    .iter()
                    .enumerate()
                    .min_by(|a, b| a.1.total_cmp(b.1))
                    .map(|(l, b)| (l, *b))
                    .unwrap_or((0, y));
                (l, b + gap)
            }
        };
        bottoms[lane] = top + height;
        out[i] = (lane, top);
    }
    out
}

// ---------------------------------------------------------------------------
// what the page draws
// ---------------------------------------------------------------------------

/// A landmark on the page.
#[derive(Debug, Clone, Serialize)]
pub struct Mark {
    pub key: String,
    pub kind: &'static str,
    pub name: String,
    pub note: String,
    pub dates: String,
    pub length: String,
    /// The continent or country it is about, beside its name in a column
    /// that holds several: its key, and its name for the page.
    pub tag: String,
    pub tag_label: String,
    /// The country's flag, when it is about one.
    pub tag_flag: Option<String>,
    /// What its window says: the context, what was at stake, the victims.
    pub context: String,
    pub stakes: String,
    pub victims: String,
    /// Its years, for the page's script.
    pub from: f64,
    pub to: f64,
    /// Where its bar runs, at its true years; none for a moment.
    pub bar_top: i64,
    pub bar_height: i64,
    pub bar_x: i64,
    pub bar: bool,
    /// Where its label is drawn, and the height it marks.
    pub x: i64,
    pub top: i64,
    pub y: i64,
    /// The label was pushed below its year: a thread joins them.
    pub moved: bool,
    /// The person's age when it began, in a person's view.
    pub age: String,
    /// It happened while the person lived, in a person's view.
    pub crossed: bool,
}

/// A column of landmarks.
#[derive(Debug, Clone, Serialize)]
pub struct Column {
    pub width: i64,
    /// The width of a label.
    pub lane: i64,
    pub marks: Vec<Mark>,
}

/// How a column lays its labels.
struct Shape {
    lanes: usize,
    lane_width: f64,
    label_height: f64,
    bar_width: f64,
}

/// A tag's name for the page: a continent's, or a country's.
fn tag_label(tag: &str, lang: &str) -> String {
    if tag.is_empty() {
        String::new()
    } else if Continent::parse(tag).is_some() {
        tr(lang, &format!("impact-continent-{tag}"), &[])
    } else {
        country_name(tag, lang)
    }
}

fn column(
    picked: Vec<(&Landmark, String)>,
    scale: &Scale,
    lang: &str,
    now: f64,
    shape: &Shape,
    age: &dyn Fn(&Landmark) -> String,
    crossed: &dyn Fn(&Landmark) -> bool,
) -> Column {
    let bars: Vec<(f64, f64)> = picked
        .iter()
        .map(|(l, _)| (scale.y(l.start()), scale.y(l.end(now))))
        .collect();
    let periods: Vec<usize> = (0..picked.len())
        .filter(|&i| bars[i].1 - bars[i].0 >= 2.0)
        .collect();
    let (bar_lane, bar_lanes) = pack(&periods.iter().map(|&i| bars[i]).collect::<Vec<_>>(), 3.0);
    let bar_lanes = bar_lanes.min(8);
    let bars_width = bar_lanes as f64 * shape.bar_width + if bar_lanes > 0 { 6.0 } else { 0.0 };
    let ys: Vec<f64> = bars.iter().map(|b| b.0).collect();
    let placed = stack(&ys, shape.label_height, shape.lanes, 4.0);
    let marks = picked
        .iter()
        .enumerate()
        .map(|(i, (l, tag))| {
            let (lane, top) = placed[i];
            let bar_at = periods.iter().position(|&p| p == i);
            let more = details().get(&l.key);
            let to_year = l.to.map(|t| t.year);
            Mark {
                key: l.key.clone(),
                kind: l.kind.slug(),
                name: l.name(lang),
                note: l.note(lang),
                dates: span_text(lang, l.from.year, to_year, l.ongoing),
                length: match l.to {
                    Some(t) if l.kind.lasts() => length_text(lang, l.from, t),
                    _ => String::new(),
                },
                tag: tag.clone(),
                tag_label: tag_label(tag, lang),
                tag_flag: (tag.len() == 2).then(|| flag_url(tag)).flatten(),
                context: more
                    .map(|d| said(&d.context, lang, &l.key, |t| &t.context))
                    .unwrap_or_default(),
                stakes: more
                    .map(|d| said(&d.stakes, lang, &l.key, |t| &t.stakes))
                    .unwrap_or_default(),
                victims: more
                    .map(|d| said(&d.victims, lang, &l.key, |t| &t.victims))
                    .unwrap_or_default(),
                from: l.start(),
                to: l.end(now),
                bar: bar_at.is_some(),
                bar_top: bars[i].0.round() as i64,
                bar_height: (bars[i].1 - bars[i].0).round() as i64,
                bar_x: bar_at
                    .map(|b| (bar_lane[b].min(7) as f64 * shape.bar_width).round() as i64)
                    .unwrap_or(0),
                x: (bars_width + lane as f64 * shape.lane_width).round() as i64,
                top: top.round() as i64,
                y: ys[i].round() as i64,
                moved: top - ys[i] > 3.0,
                age: age(l),
                crossed: crossed(l),
            }
        })
        .collect();
    Column {
        width: (bars_width + shape.lanes as f64 * shape.lane_width).round() as i64,
        lane: shape.lane_width.round() as i64,
        marks,
    }
}

/// A life on the page.
#[derive(Debug, Clone, Serialize)]
pub struct LifeMark {
    pub id: String,
    pub name: String,
    pub years: String,
    pub living: bool,
    pub fuzzy_from: bool,
    pub fuzzy_to: bool,
    pub open_from: bool,
    pub open_to: bool,
    pub from: f64,
    pub to: f64,
    pub top: i64,
    pub height: i64,
    /// Its lane in the family's column; the page sets the lanes' width.
    pub lane: usize,
}

/// The family's view.
#[derive(Debug, Clone, Serialize)]
pub struct Family {
    pub height: i64,
    pub ticks: Vec<Tick>,
    pub grid: Vec<GridLine>,
    pub world: Column,
    pub continents: Column,
    /// The continents the family lived on, the commonest first.
    pub continent_keys: Vec<&'static str>,
    pub lives: Vec<LifeMark>,
    /// How many lanes the lives need: as many as there were lives at once.
    pub lanes: usize,
    /// The column at fourteen pixels a lane, its widest: the page narrows the
    /// lanes to the room it has (docs/adr/0036).
    pub lives_width: i64,
    pub placed: usize,
    pub undated: usize,
    pub hidden: usize,
}

/// Which stretch of time the family's view shows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Span {
    /// From the end of the last ice age to now.
    All,
    /// The family's own centuries.
    Family,
}

const LIFE_LANE: f64 = 14.0;

/// The family's view: the world, the family's continents, and every life
/// the reader may read.
pub fn family(flat: &Value, lens: &Lens, lang: &str, span: Span, now: f64) -> Family {
    let found = lives(flat, lens, now);
    let mut by: BTreeMap<Continent, usize> = BTreeMap::new();
    for l in &found.lives {
        for c in l.countries.iter().filter_map(|c| continent_of(c)) {
            *by.entry(c).or_default() += 1;
        }
    }
    let mut continents: Vec<(Continent, usize)> = by.into_iter().collect();
    continents.sort_by_key(|c| std::cmp::Reverse(c.1));
    let continents: Vec<Continent> = continents.into_iter().map(|(c, _)| c).collect();

    let earliest = found
        .lives
        .iter()
        .map(|l| l.from)
        .fold(f64::INFINITY, f64::min);
    let scale = match span {
        Span::Family if earliest.is_finite() => {
            let from = ((earliest - 30.0) / 50.0).floor() * 50.0;
            let px = (3_200.0 / (now - from)).clamp(3.0, 14.0);
            Scale::linear(from, now, px)
        }
        _ => Scale::history(now),
    };
    let within = |l: &Landmark| l.end(now) >= scale.start() && l.start() <= now;
    let world: Vec<(&Landmark, String)> = catalogue()
        .iter()
        .filter(|l| l.is_world() && within(l))
        .map(|l| (l, String::new()))
        .collect();
    let continental: Vec<(&Landmark, String)> = catalogue()
        .iter()
        .filter(|l| !l.is_world() && within(l))
        .filter_map(|l| {
            let c = l.continents().find(|c| continents.contains(c))?;
            Some((l, c.slug().to_string()))
        })
        .collect();
    let shape = Shape {
        lanes: 2,
        lane_width: 224.0,
        label_height: 22.0,
        bar_width: 5.0,
    };
    let none = |_: &Landmark| String::new();
    let never = |_: &Landmark| false;
    let world = column(world, &scale, lang, now, &shape, &none, &never);
    let continents_col = column(continental, &scale, lang, now, &shape, &none, &never);

    let spans: Vec<(f64, f64)> = found
        .lives
        .iter()
        .map(|l| {
            let (a, b) = (scale.y(l.from), scale.y(l.to));
            (a, b.max(a + 8.0))
        })
        .collect();
    let (lane, lanes) = pack(&spans, 3.0);
    let marks = found
        .lives
        .iter()
        .enumerate()
        .map(|(i, l)| LifeMark {
            id: l.id.clone(),
            name: l.name.clone(),
            years: l.years.clone(),
            living: l.living,
            fuzzy_from: l.fuzzy_from,
            fuzzy_to: l.fuzzy_to,
            open_from: l.open_from,
            open_to: l.open_to,
            from: l.from,
            to: l.to,
            top: spans[i].0.round() as i64,
            height: (spans[i].1 - spans[i].0).round() as i64,
            lane: lane[i],
        })
        .collect();
    Family {
        height: scale.height().ceil() as i64,
        ticks: ticks(&scale, lang),
        grid: grid(&scale),
        world,
        continents: continents_col,
        continent_keys: continents.iter().map(|c| c.slug()).collect(),
        lives: marks,
        lanes,
        lives_width: ((lanes.max(1) as f64) * LIFE_LANE).max(160.0).round() as i64,
        placed: found.lives.len(),
        undated: found.undated,
        hidden: found.hidden,
    }
}

// ---------------------------------------------------------------------------
// one person's view
// ---------------------------------------------------------------------------

/// A moment of the person's own life, beside their window.
#[derive(Debug, Clone, Serialize)]
pub struct Milestone {
    pub top: i64,
    pub year: i32,
    pub label: String,
    /// Another person's record it names (a child), when the reader may read it.
    pub link: Option<String>,
}

/// A place of the person's life, and the states that held it while they
/// lived.
#[derive(Debug, Clone, Serialize)]
pub struct Held {
    pub place: String,
    pub states: Vec<String>,
}

/// A country of a person's life, for the page.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Country {
    pub iso: String,
    pub name: String,
    pub flag: Option<String>,
}

/// One person's view.
#[derive(Debug, Clone, Serialize)]
pub struct Person {
    pub id: String,
    pub name: String,
    pub years: String,
    pub height: i64,
    pub ticks: Vec<Tick>,
    pub grid: Vec<GridLine>,
    pub life: LifeMark,
    pub milestones: Vec<Milestone>,
    /// The world's and their continent's landmarks, a column a kind, in the
    /// order of [`Kind::ALL`].
    pub kinds: Vec<KindColumn>,
    pub near: Column,
    /// (ISO code, name) of their countries.
    pub countries: Vec<Country>,
    pub continent_keys: Vec<&'static str>,
    pub held: Vec<Held>,
}

/// One kind's column on a person's view: its kind's key, and its landmarks.
#[derive(Debug, Clone, Serialize)]
pub struct KindColumn {
    pub kind: &'static str,
    pub column: Column,
}

/// The person's milestones: their marriages and their children's births.
fn milestones(
    flat: &Value,
    lens: &Lens,
    id: &str,
    lang: &str,
) -> Vec<(When, String, Option<String>)> {
    let mut out = Vec::new();
    for e in flat
        .get("events")
        .and_then(Value::as_object)
        .into_iter()
        .flat_map(|m| m.values())
    {
        if e.get("category").and_then(Value::as_str) != Some("marriage") {
            continue;
        }
        let theirs = e
            .get("participants")
            .and_then(Value::as_array)
            .is_some_and(|ps| {
                ps.iter()
                    .any(|p| p.get("entity_id").and_then(Value::as_str) == Some(id))
            });
        if let (true, Some((w, _))) = (theirs, moment_of(Some(e))) {
            out.push((w, tr(lang, "impact-milestone-marriage", &[]), None));
        }
    }
    for f in flat
        .get("families")
        .and_then(Value::as_object)
        .into_iter()
        .flat_map(|m| m.values())
    {
        let parent = f
            .get("union")
            .and_then(|u| u.get("persons"))
            .and_then(Value::as_array)
            .is_some_and(|ps| {
                ps.iter()
                    .any(|p| p.get("person_id").and_then(Value::as_str) == Some(id))
            });
        if !parent {
            continue;
        }
        for c in f
            .get("children")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
        {
            let Some(cid) = c.get("person_id").and_then(Value::as_str) else {
                continue;
            };
            if !lens.sees_person(cid) {
                continue;
            }
            let Some(child) = flat.get("persons").and_then(|p| p.get(cid)) else {
                continue;
            };
            if let Some((w, _)) = moment_of(child.get("birth")) {
                let name = crate::view::person_display_name(child);
                out.push((
                    w,
                    tr(lang, "impact-milestone-child", &[("name", name.into())]),
                    Some(cid.to_string()),
                ));
            }
        }
    }
    out
}

/// The states that held the person's places while they lived, as the places
/// record them.
fn held(flat: &Value, person: &Value, life: &Life, more: Option<&Vec<String>>) -> Vec<Held> {
    let mut ids: Vec<String> = ["birth", "death", "burial", "baptism"]
        .iter()
        .filter_map(|k| Some(person.get(k)?.get("place_id")?.as_str()?.to_string()))
        .collect();
    ids.extend(more.into_iter().flatten().cloned());
    ids.dedup();
    let mut seen = Vec::new();
    let mut out = Vec::new();
    for id in ids {
        if seen.contains(&id) {
            continue;
        }
        seen.push(id.clone());
        let Some(place) = flat.get("places").and_then(|p| p.get(&id)) else {
            continue;
        };
        let year = |h: &Value, k: &str| -> Option<f64> {
            h.get(k)?.as_str()?.get(..4)?.parse::<f64>().ok()
        };
        let states: Vec<String> = place
            .get("country_history")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter(|h| {
                year(h, "from").is_none_or(|f| f <= life.to)
                    && year(h, "until").is_none_or(|u| u >= life.from)
            })
            .filter_map(|h| {
                let c = h.get("country")?.as_str()?;
                let f = h.get("from").and_then(Value::as_str);
                let u = h.get("until").and_then(Value::as_str);
                Some(match (f, u) {
                    (Some(f), Some(u)) => format!("{c} ({f}–{u})"),
                    (Some(f), None) => format!("{c} ({f}–)"),
                    (None, Some(u)) => format!("{c} (–{u})"),
                    (None, None) => c.to_string(),
                })
            })
            .collect();
        if !states.is_empty() {
            out.push(Held {
                place: crate::view::place_name(place),
                states,
            });
        }
    }
    out
}

/// One person's view: their life, the world's and their continent's
/// landmarks beside it, their countries' with what they mean, and their age
/// at each. `None` when the reader may not read them, or they cannot be
/// placed.
pub fn person(flat: &Value, lens: &Lens, id: &str, lang: &str, now: f64) -> Option<Person> {
    if !lens.sees_person(id) {
        return None;
    }
    let p = flat.get("persons")?.get(id)?;
    let idx = event_index(flat);
    let life = life_of(flat, id, p, &idx, now)?;
    let (start, end) = (life.from - 20.0, (life.to + 10.0).min(now));
    let continents: Vec<Continent> = {
        let mut cs: Vec<Continent> = Vec::new();
        for c in life.countries.iter().filter_map(|c| continent_of(c)) {
            if !cs.contains(&c) {
                cs.push(c);
            }
        }
        cs
    };
    let within = |l: &Landmark| l.end(now) >= start && l.start() <= end;
    // The macro side: what touched the whole world, and what touched their
    // continent. The repertoire's other continents (the Qing for a Lyonnais)
    // stay on the family's view; one whose continent is unknown keeps them.
    let wide: Vec<(&Landmark, String)> = catalogue()
        .iter()
        .filter(|l| within(l))
        .filter_map(|l| {
            if let Some(c) = l.continents().find(|c| continents.contains(c)) {
                let tag = if l.is_world() {
                    String::new()
                } else {
                    c.slug().to_string()
                };
                return Some((l, tag));
            }
            let global = l.continents().next().is_none() || continents.is_empty();
            (l.is_world() && global).then(|| (l, String::new()))
        })
        .collect();
    let near: Vec<(&Landmark, String)> = catalogue()
        .iter()
        .filter(|l| within(l))
        .filter_map(|l| {
            let c = l
                .countries()
                .find(|c| life.countries.iter().any(|x| x == c))?;
            Some((l, c.to_string()))
        })
        .collect();
    // An age needs a birth: a window estimated from a death or from the
    // moments they were seen says when they lived, not how old they were.
    let born = life.birth.map(|b| b.at());
    let age = |l: &Landmark| -> String {
        let at = l.start();
        let Some(born) = born else {
            return String::new();
        };
        if at > life.to {
            String::new()
        } else if at >= born {
            let n = (at - born).floor() as i64;
            tr(lang, "impact-age", &[("n", n.into())])
        } else if l.end(now) > born {
            tr(lang, "impact-under-way", &[])
        } else {
            String::new()
        }
    };
    // The macro side, a column a kind (the operator's request of 8 November
    // 2026), narrower than the one column it was, so that several sit side
    // by side; a kind with nothing in the person's years has none.
    let by_kind: Vec<(Kind, Vec<(&Landmark, String)>)> = Kind::ALL
        .iter()
        .filter_map(|k| {
            let list: Vec<(&Landmark, String)> =
                wide.iter().filter(|(l, _)| l.kind == *k).cloned().collect();
            (!list.is_empty()).then_some((*k, list))
        })
        .collect();
    let kind_shape = Shape {
        lanes: 1,
        lane_width: 104.0,
        label_height: 58.0,
        bar_width: 4.0,
    };
    let near_shape = Shape {
        lanes: 1,
        lane_width: 236.0,
        label_height: 76.0,
        bar_width: 5.0,
    };
    let crossed = |l: &Landmark| l.end(now) >= life.from && l.start() <= life.to;
    // As many pixels a year as keep each label near its year.
    let mut px = (1_500.0 / (end - start)).clamp(8.0, 24.0);
    let (mut scale, mut kind_cols, mut near_col): (Scale, Vec<KindColumn>, Column);
    loop {
        scale = Scale::linear(start, end, px);
        kind_cols = by_kind
            .iter()
            .map(|(k, list)| KindColumn {
                kind: k.slug(),
                column: column(list.clone(), &scale, lang, now, &kind_shape, &age, &crossed),
            })
            .collect();
        near_col = column(near.clone(), &scale, lang, now, &near_shape, &age, &crossed);
        let drift = kind_cols
            .iter()
            .flat_map(|k| &k.column.marks)
            .chain(&near_col.marks)
            .map(|m| m.top - m.y)
            .max()
            .unwrap_or(0);
        if drift <= 120 || px >= 24.0 {
            break;
        }
        px = (px * 1.4).min(24.0);
    }
    let (a, b) = (scale.y(life.from), scale.y(life.to));
    let life_mark = LifeMark {
        id: life.id.clone(),
        name: life.name.clone(),
        years: life.years.clone(),
        living: life.living,
        fuzzy_from: life.fuzzy_from,
        fuzzy_to: life.fuzzy_to,
        open_from: life.open_from,
        open_to: life.open_to,
        from: life.from,
        to: life.to,
        top: a.round() as i64,
        height: (b - a).max(8.0).round() as i64,
        lane: 0,
    };
    let mut stones: Vec<(When, String, Option<String>)> = Vec::new();
    if let Some(b) = life.birth {
        stones.push((b, tr(lang, "impact-milestone-birth", &[]), None));
    }
    if !life.living {
        let d = moment_of(p.get("death")).map(|(w, _)| w);
        if let Some(d) = d {
            stones.push((d, tr(lang, "impact-milestone-death", &[]), None));
        }
    }
    stones.extend(milestones(flat, lens, id, lang));
    stones.sort_by(|a, b| a.0.at().total_cmp(&b.0.at()));
    let ys: Vec<f64> = stones.iter().map(|s| scale.y(s.0.at())).collect();
    let placed = stack(&ys, 18.0, 1, 2.0);
    let milestones = stones
        .into_iter()
        .zip(placed)
        .map(|((w, label, link), (_, top))| Milestone {
            top: top.round() as i64,
            year: w.year,
            label,
            link,
        })
        .collect();
    Some(Person {
        id: id.to_string(),
        name: life.name.clone(),
        years: life.years.clone(),
        height: scale.height().ceil() as i64,
        ticks: ticks(&scale, lang),
        grid: grid(&scale),
        life: life_mark,
        milestones,
        kinds: kind_cols,
        near: near_col,
        countries: life
            .countries
            .iter()
            .map(|c| Country {
                iso: c.clone(),
                name: country_name(c, lang),
                flag: flag_url(c),
            })
            .collect(),
        continent_keys: continents.iter().map(|c| c.slug()).collect(),
        held: held(flat, p, &life, idx.places.get(id)),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn every_landmark_of_the_catalogue_reads() {
        let mut keys = std::collections::HashSet::new();
        let all = parsed();
        assert!(all.len() > 250, "{} landmarks", all.len());
        for (line, r) in all {
            let l = r.unwrap_or_else(|e| panic!("line {line}: {e}"));
            assert!(
                keys.insert(l.key.clone()),
                "line {line}: key {} twice",
                l.key
            );
            assert!(l.start() >= DAWN, "line {line}");
            // A country's landmark explains itself, or is plain enough not to.
            assert_eq!(
                l.notes[0].is_empty(),
                l.notes[1].is_empty(),
                "line {line}: one note"
            );
        }
        // The operator's repertoire is there, from the top to the bottom.
        let world: Vec<&Landmark> = catalogue().iter().filter(|l| l.is_world()).collect();
        assert!(world
            .iter()
            .any(|l| l.name("fr").starts_with("Site monumental de Göbekli Tepe")));
        assert!(world
            .iter()
            .any(|l| l.name("en") == "Russian invasion of Ukraine" && l.ongoing));
    }

    #[test]
    fn every_landmark_opens_on_its_context_in_both_languages() {
        let details = parsed_details().unwrap_or_else(|e| panic!("{e}"));
        let keys: std::collections::HashSet<&str> =
            catalogue().iter().map(|l| l.key.as_str()).collect();
        for k in details.keys() {
            assert!(keys.contains(k.as_str()), "{k}: no such landmark");
        }
        for l in catalogue() {
            let d = details
                .get(&l.key)
                .unwrap_or_else(|| panic!("{}: no details", l.key));
            for i in 0..2 {
                assert!(!d.context[i].is_empty(), "{}: context", l.key);
                assert!(!d.stakes[i].is_empty(), "{}: stakes", l.key);
            }
            assert_eq!(
                d.victims[0].is_empty(),
                d.victims[1].is_empty(),
                "{}: victims in one language only",
                l.key
            );
        }
        // A war says who died.
        let ww2 = &details["1939-09-01-second-world-war"];
        assert!(ww2.victims[1].starts_with("70 à 85 millions"));
    }

    #[test]
    fn a_village_named_alone_finds_its_country_and_its_flag() {
        let place = |v: Value| country_of_place(&v);
        assert_eq!(
            place(json!({"names": [{"value": "Słomniki"}]})),
            Some("PL".into())
        );
        assert_eq!(
            place(json!({"names": [{"value": "Miechów"}]})),
            Some("PL".into())
        );
        assert_eq!(
            place(json!({"names": [{"value": "Sainte-Colombe-sur-Gand"}]})),
            Some("FR".into())
        );
        assert_eq!(
            place(json!({"names": [{"value": "x"}], "region": "Pologne"})),
            Some("PL".into())
        );
        assert_eq!(flag_url("PL").as_deref(), Some("/static/flags/pl.svg"));
        assert!(flag_svg("FR").is_some_and(|s| s.contains("<svg")));
        assert_eq!(flag_url("ZZ"), None);
    }

    #[test]
    fn the_graduation_marks_every_year_where_a_year_has_room() {
        let s = Scale::linear(1900.0, 1950.0, 12.0);
        let g = s.grid();
        assert_eq!(g.len(), 50, "a line a year");
        assert!(g.iter().all(|(_, step)| *step == 1.0));
        let lines = grid(&s);
        assert!(
            lines.iter().filter(|l| l.strong).count() == 5,
            "every ten years"
        );
        // All of history: years where the family lived, centuries at the top.
        let h = Scale::history(2026.0);
        let g = h.grid();
        assert!(g.iter().any(|(t, step)| *t == 1914.0 && *step == 1.0));
        assert!(g.iter().all(|(t, step)| *t >= -3_500.0 || *step >= 100.0));
    }

    #[test]
    fn a_length_reads_as_the_history_books_give_it() {
        let w = |s: &str| When::parse(s).unwrap();
        assert_eq!(length(w("1939-09-01"), w("1945-09-02")), (6, 0, 1));
        assert_eq!(length(w("1914-07-28"), w("1918-11-11")), (4, 3, 14));
        assert_eq!(length(w("-499"), w("-449")), (50, 0, 0));
        assert_eq!(days_between(w("1967-06-05"), w("1967-06-11")), Some(6));
        assert_eq!(days_between(w("1994-04-07"), w("1994-07-16")), Some(100));
        assert_eq!(days_between(w("1973-10-06"), w("1973-10-25")), Some(19));
        assert_eq!(
            length_text("en", w("1939-09-01"), w("1945-09-02")),
            "6 years 1 day"
        );
        assert_eq!(
            length_text("fr", w("1914-07-28"), w("1918-11-11")),
            "4 ans 3 mois"
        );
        assert_eq!(
            length_text("fr", w("1967-06-05"), w("1967-06-11")),
            "6 jours"
        );
        assert_eq!(
            length_text("en", w("1994-04-07"), w("1994-07-16")),
            "100 days"
        );
        assert_eq!(
            span_text("fr", -3100, Some(-30), false),
            "3100–30 av. J.-C."
        );
        assert_eq!(span_text("en", -753, Some(476), false), "753 BC – 476");
        assert_eq!(span_text("en", 2011, None, true), "2011 – today");
        assert_eq!(span_text("en", 1967, Some(1967), false), "1967");
    }

    #[test]
    fn a_place_says_its_country_one_way_or_another() {
        let place = |v: Value| country_of_place(&v);
        assert_eq!(place(json!({"country_current": "pl"})), Some("PL".into()));
        assert_eq!(
            place(json!({"country_current": "France"})),
            Some("FR".into())
        );
        assert_eq!(
            place(json!({"names": [{"value": "Lyon, Rhone, France"}]})),
            Some("FR".into())
        );
        assert_eq!(
            place(json!({"names": [{"value": "Słomniki, Pologne"}]})),
            Some("PL".into())
        );
        assert_eq!(
            place(json!({"names": [{"value": "Warszawa"}]})),
            Some("PL".into())
        );
        assert_eq!(
            place(json!({"names": [{"value": "Kinshasa"}]})),
            Some("CD".into())
        );
        // "Congo" alone could be either: none rather than a wrong one.
        assert_eq!(place(json!({"names": [{"value": "Congo"}]})), None);
        assert_eq!(continent_of("PL"), Some(Continent::Europe));
        assert_eq!(continent_of("HT"), Some(Continent::Americas));
    }

    #[test]
    fn the_axis_keeps_its_years_in_order_and_the_family_s_centuries_wide() {
        let s = Scale::history(2026.0);
        let mut last = -1.0;
        for y in [
            -10_000.0, -3_500.0, -500.0, 0.0, 800.0, 1_500.0, 1_800.0, 1_914.0, 2_026.0,
        ] {
            assert!(s.y(y) > last, "{y}");
            last = s.y(y);
        }
        assert!(
            s.y(1_918.0) - s.y(1_914.0) >= 28.0,
            "a war of the family's time is tall"
        );
        let t = s.ticks();
        assert!(t.windows(2).all(|w| s.y(w[1]) - s.y(w[0]) >= 40.0));
    }

    #[test]
    fn nothing_is_drawn_over_anything_else() {
        let spans = [
            (0.0, 50.0),
            (10.0, 20.0),
            (40.0, 80.0),
            (60.0, 70.0),
            (85.0, 90.0),
        ];
        let (lane, n) = pack(&spans, 3.0);
        for i in 0..spans.len() {
            for j in i + 1..spans.len() {
                if lane[i] == lane[j] {
                    let (a, b) = (spans[i], spans[j]);
                    assert!(a.1 + 3.0 <= b.0 || b.1 + 3.0 <= a.0, "{i} {j}");
                }
            }
        }
        assert!(n >= 2);
        let placed = stack(&[0.0, 1.0, 2.0, 3.0], 20.0, 2, 4.0);
        assert_eq!(placed[0], (0, 0.0));
        assert_eq!(placed[1], (1, 1.0));
        assert_eq!(placed[2], (0, 24.0), "pushed below the first");
    }

    fn family_flat() -> Value {
        let person = |id: &str, born: &str, died: Option<&str>, vis: &str, place: &str| {
            let mut p = json!({"id": id, "type": "person",
                "identity": {"name": {"display": format!("Person {id}"), "components": []},
                             "is_living": died.is_none() && vis == "members", "visibility": vis},
                "birth": {"date": {"value": born, "precision": "year"}, "place_id": place}});
            if let Some(d) = died {
                p["death"] = json!({"date": {"value": d, "precision": "year"}});
            }
            p
        };
        json!({
            "persons": {
                "a": person("a", "1890", Some("1960"), "public", "warsaw"),
                "b": person("b", "1920", Some("1999"), "public", "lyon"),
                "c": person("c", "1950", None, "members", "lyon"),
                "d": {"id": "d", "type": "person",
                      "identity": {"name": {"display": "Person d", "components": []}}},
            },
            "places": {
                "warsaw": {"names": [{"value": "Warszawa"}]},
                "lyon": {"names": [{"value": "Lyon, Rhone, France"}]},
            },
            "families": {"f": {"union": {"persons": [{"person_id": "a"}]},
                               "children": [{"person_id": "b"}]}},
            "events": {}
        })
    }

    #[test]
    fn the_family_is_laid_beside_the_world_through_the_readers_lens() {
        let flat = family_flat();
        let lens = Lens::resolve(&flat, crate::acl::Visibility::Public);
        let f = family(&flat, &lens, "en", Span::All, 2026.0);
        assert_eq!(f.placed, 2, "a and b");
        assert_eq!(f.hidden, 1, "c is the members'");
        assert_eq!(f.undated, 1, "d has no date");
        assert_eq!(f.continent_keys, ["europe"]);
        assert!(f
            .world
            .marks
            .iter()
            .any(|m| m.name == "First World War" && m.length == "4 years 3 months"));
        assert!(f.continents.marks.iter().all(|m| m.tag == "europe"));
        let a = f.lives.iter().find(|l| l.id == "a").unwrap();
        assert_eq!(a.years, "1890–1960");
        let everyone = Lens::unrestricted();
        assert_eq!(
            family(&flat, &everyone, "en", Span::Family, 2026.0).placed,
            3
        );
    }

    #[test]
    fn a_person_s_view_gives_their_countries_landmarks_and_their_age() {
        let flat = family_flat();
        let lens = Lens::unrestricted();
        let p = person(&flat, &lens, "a", "fr", 2026.0).unwrap();
        assert_eq!(
            p.countries,
            [Country {
                iso: "PL".into(),
                name: "Pologne".into(),
                flag: Some("/static/flags/pl.svg".into()),
            }]
        );
        let wide: Vec<&Mark> = p.kinds.iter().flat_map(|k| &k.column.marks).collect();
        let ww1 = wide
            .iter()
            .find(|m| m.key.ends_with("first-world-war"))
            .unwrap();
        assert_eq!(ww1.age, "à 24 ans");
        let rising = p
            .near
            .marks
            .iter()
            .find(|m| m.name == "Insurrection de Varsovie")
            .unwrap();
        assert_eq!(rising.age, "à 54 ans");
        assert!(rising.note.starts_with("63 jours"));
        // Nothing of a country they did not live in, nor of another
        // continent's history on the world's side.
        assert!(p.near.marks.iter().all(|m| m.tag == "PL"));
        assert!(!wide.iter().any(|m| m.name == "Dynastie Qing (Chine)"));
        assert!(wide.iter().any(|m| m.name == "Grippe espagnole"));
        // A column a kind, in the kinds' order, none of them empty.
        let kinds: Vec<&str> = p.kinds.iter().map(|k| k.kind).collect();
        assert!(kinds.contains(&"war") && kinds.contains(&"epidemic"));
        assert!(
            p.kinds
                .iter()
                .all(|k| !k.column.marks.is_empty()
                    && k.column.marks.iter().all(|m| m.kind == k.kind))
        );
        assert!(p
            .milestones
            .iter()
            .any(|m| m.label == "Naissance de Person b"));
        assert!(person(
            &flat,
            &Lens::resolve(&flat, crate::acl::Visibility::Public),
            "c",
            "fr",
            2026.0
        )
        .is_none());
    }

    /// The operator's rule of 8 November 2026 (docs/adr/0036).
    fn uncertain_flat() -> Value {
        let person = |id: &str, living: bool, birth: Option<(&str, bool)>, death: Option<&str>| {
            let mut p = json!({"id": id, "type": "person",
                "identity": {"name": {"display": format!("Person {id}"), "components": []},
                             "is_living": living, "visibility": "public"}});
            if let Some((b, circa)) = birth {
                p["birth"] = json!({"date": {"value": b, "precision": "year", "circa": circa},
                                    "place_id": "warsaw"});
            }
            if let Some(d) = death {
                p["death"] = json!({"date": {"value": d, "precision": "year"}});
            }
            p
        };
        let seen = |id: &str, year: &str, circa: bool, who: &str| {
            json!({"id": id, "date": {"value": year, "precision": "year", "circa": circa},
                   "participants": [{"entity_type": "person", "entity_id": who}]})
        };
        json!({
            "persons": {
                "old": person("old", true, Some(("1700", false)), None),
                "alive": person("alive", true, Some(("1990", false)), None),
                "born": person("born", false, Some(("1850", true)), None),
                "late": person("late", false, Some(("1950", false)), None),
                "died": person("died", false, None, Some("1810")),
                "seen": person("seen", false, None, None),
                "seen2": person("seen2", false, None, None),
                "both": person("both", false, Some(("1890", false)), Some("1960")),
            },
            "places": {"warsaw": {"names": [{"value": "Warszawa"}]}},
            "events": {
                "e1": seen("e1", "1780", true, "seen"),
                "e2": seen("e2", "1800", false, "seen2"),
                "e3": seen("e3", "1890", false, "seen2"),
            }
        })
    }

    #[test]
    fn a_life_missing_a_date_is_never_drawn_longer_than_a_hundred_years() {
        let flat = uncertain_flat();
        let found = lives(&flat, &Lens::unrestricted(), 2026.0);
        let get = |id: &str| found.lives.iter().find(|l| l.id == id).unwrap();
        let span = |id: &str| (get(id).from, get(id).to);

        // Said living, but born more than a hundred years ago: born only.
        let old = get("old");
        assert_eq!(span("old"), (1700.0, 1800.0));
        assert!(!old.living && old.open_to && !old.open_from);
        assert_eq!(old.years, "° 1700");
        // Living, and young enough to be: to today.
        let alive = get("alive");
        assert_eq!(span("alive"), (1990.0, 2026.0));
        assert!(alive.living && !alive.open_to);
        assert_eq!(alive.years, "1990–");
        // Born about a year, and nothing after: a hundred years from it.
        let born = get("born");
        assert_eq!(span("born"), (1850.0, 1950.0));
        assert!(born.fuzzy_from && born.open_to);
        // Never past today.
        assert_eq!(span("late"), (1950.0, 2026.0));
        assert!(!get("late").living && get("late").open_to);
        // Died only: the hundred years before.
        let died = get("died");
        assert_eq!(span("died"), (1710.0, 1810.0));
        assert!(died.open_from && !died.open_to && died.birth.is_none());
        assert_eq!(died.years, "† 1810");
        // Seen about 1780: fifty years either side.
        assert_eq!(span("seen"), (1730.0, 1830.0));
        assert!(get("seen").open_from && get("seen").open_to);
        assert_eq!(get("seen").years, "~ 1780");
        // Seen in 1800 and in 1890: around the middle, and both moments in.
        assert_eq!(span("seen2"), (1795.0, 1895.0));
        assert_eq!(get("seen2").years, "~ 1800–1890");
        // Both dates recorded: as recorded.
        assert_eq!(span("both"), (1890.0, 1960.0));
        assert!(!get("both").open_from && !get("both").open_to);

        for l in &found.lives {
            if l.open_from || l.open_to {
                assert!(l.to - l.from <= LONGEST_LIFE, "{}", l.id);
            }
            assert!(l.to <= 2026.0 && l.from <= l.to, "{}", l.id);
        }
    }

    #[test]
    fn an_age_is_said_only_from_a_birth_and_only_within_the_life() {
        let flat = uncertain_flat();
        let lens = Lens::unrestricted();
        // Said living since 1700: nothing after their hundredth year, and
        // never "aged 322".
        let p = person(&flat, &lens, "old", "en", 2026.0).unwrap();
        let marks: Vec<&Mark> = p
            .kinds
            .iter()
            .flat_map(|k| &k.column.marks)
            .chain(&p.near.marks)
            .collect();
        assert!(!marks.is_empty());
        for m in &marks {
            assert!(m.from <= 1810.0, "{}", m.name);
            if let Some(n) = m.age.strip_prefix("aged ") {
                assert!(n.parse::<i64>().unwrap() <= 100, "{}: {}", m.name, m.age);
            }
        }
        assert!(p.life.open_to && !p.life.living);
        // A window estimated from a death says when they lived, not their age.
        let d = person(&flat, &lens, "died", "en", 2026.0).unwrap();
        assert!(d
            .kinds
            .iter()
            .flat_map(|k| &k.column.marks)
            .chain(&d.near.marks)
            .all(|m| m.age.is_empty()));
        assert!(d.milestones.iter().any(|m| m.year == 1810));
    }

    #[test]
    fn the_family_s_lives_have_a_lane_each_and_the_column_its_widest() {
        let flat = uncertain_flat();
        let f = family(&flat, &Lens::unrestricted(), "en", Span::Family, 2026.0);
        assert_eq!(f.placed, 8);
        assert!(f.lanes >= 2 && f.lives.iter().all(|l| l.lane < f.lanes));
        assert_eq!(f.lives_width, ((f.lanes as f64) * 14.0).max(160.0) as i64);
        // No two lives in a lane overlap.
        for a in &f.lives {
            for b in f.lives.iter().filter(|b| b.id != a.id && b.lane == a.lane) {
                assert!(
                    a.top + a.height <= b.top || b.top + b.height <= a.top,
                    "{} {}",
                    a.id,
                    b.id
                );
            }
        }
    }

    #[test]
    fn every_translation_names_a_landmark_of_the_catalogue() {
        let keys: std::collections::HashSet<&str> =
            catalogue().iter().map(|l| l.key.as_str()).collect();
        for (lang, text) in TRANSLATED {
            let t = parsed_translations(text).unwrap_or_else(|e| panic!("{lang}: {e}"));
            for k in t.keys() {
                assert!(keys.contains(k.as_str()), "{lang}: {k} is no landmark");
            }
        }
    }

    #[test]
    fn each_language_names_and_explains_every_landmark() {
        for (lang, _) in TRANSLATED {
            let file = translations().get(lang).expect("the language's file");
            for l in catalogue() {
                let t = file
                    .get(&l.key)
                    .unwrap_or_else(|| panic!("{lang}: nothing for {}", l.key));
                assert!(!t.name.is_empty(), "{lang} {}: no name", l.key);
                assert_eq!(
                    t.note.is_empty(),
                    l.notes[0].is_empty(),
                    "{lang} {}: the note",
                    l.key
                );
                if let Some(d) = details().get(&l.key) {
                    assert!(
                        !t.context.is_empty() && !t.stakes.is_empty(),
                        "{lang} {}: the window",
                        l.key
                    );
                    assert_eq!(
                        t.victims.is_empty(),
                        d.victims[0].is_empty(),
                        "{lang} {}: the victims",
                        l.key
                    );
                }
            }
        }
    }

    #[test]
    fn a_landmark_reads_in_the_readers_language_or_in_english() {
        let flat = family_flat();
        let lens = Lens::unrestricted();
        let marks = |lang: &str| -> Vec<Mark> {
            person(&flat, &lens, "a", lang, 2026.0)
                .unwrap()
                .kinds
                .into_iter()
                .flat_map(|k| k.column.marks)
                .collect()
        };
        let ww1 = |lang: &str| {
            marks(lang)
                .into_iter()
                .find(|m| m.key.ends_with("first-world-war"))
                .unwrap()
        };
        let pl = ww1("pl");
        assert_eq!(pl.name, "Pierwsza wojna światowa");
        assert!(pl.context.starts_with("Rywalizujące sojusze"));
        assert!(pl.victims.starts_with("Około 20 milionów"));
        assert_eq!(ww1("fr").name, "Première Guerre mondiale");
        assert_eq!(ww1("de").name, "Erster Weltkrieg");
        // A language with no file yet reads in English.
        assert_eq!(ww1("sw").name, "First World War");
    }
}
