//! One shape for everything the administration console checks (v0.5.0,
//! docs/adr/0011 §3).
//!
//! The bundle's conformity and the machine's health are different questions
//! with the same answer's shape: what was looked at, whether it is green, a
//! warning or critical, what was found, and — when it is not green — what to
//! do about it. One shape means one way of drawing it and one way of reading
//! it, on both pages and in the JSON a monitor polls.
//!
//! Every sentence is a catalogue key with its numbers beside it, never a
//! string assembled here: the reader's language decides the plural.

use serde_json::{json, Value};

pub use crate::health::{Arg, Level};

/// One thing that was checked.
#[derive(Debug, Clone)]
pub struct Item {
    /// Stable machine name, for the JSON and the tests.
    pub id: &'static str,
    /// Which group the page lists it under.
    pub area: &'static str,
    pub level: Level,
    /// What was looked at: a catalogue key.
    pub title: &'static str,
    /// What was found: a catalogue key, with `args`.
    pub finding: &'static str,
    pub args: Vec<(&'static str, Arg)>,
    /// What to do, when it is not green: a catalogue key, with `args`.
    pub fix: Option<&'static str>,
    /// Where to go to do it, when there is a page for that.
    pub link: Option<String>,
}

impl Item {
    pub fn new(id: &'static str, area: &'static str, title: &'static str) -> Self {
        Self {
            id,
            area,
            level: Level::Ok,
            title,
            finding: "",
            args: Vec::new(),
            fix: None,
            link: None,
        }
    }

    /// Green, and what was found.
    pub fn ok(mut self, finding: &'static str) -> Self {
        self.level = Level::Ok;
        self.finding = finding;
        self
    }

    /// A warning: what was found, and what to do.
    pub fn warn(mut self, finding: &'static str, fix: &'static str) -> Self {
        self.level = Level::Warn;
        self.finding = finding;
        self.fix = Some(fix);
        self
    }

    /// Critical: what was found, and what to do.
    pub fn fail(mut self, finding: &'static str, fix: &'static str) -> Self {
        self.level = Level::Fail;
        self.finding = finding;
        self.fix = Some(fix);
        self
    }

    pub fn arg(mut self, name: &'static str, value: Arg) -> Self {
        self.args.push((name, value));
        self
    }

    pub fn num(self, name: &'static str, n: impl TryInto<i64>) -> Self {
        let n = n.try_into().unwrap_or(i64::MAX);
        self.arg(name, Arg::Num(n))
    }

    pub fn text(self, name: &'static str, s: impl Into<String>) -> Self {
        self.arg(name, Arg::Text(s.into()))
    }

    pub fn link(mut self, to: impl Into<String>) -> Self {
        self.link = Some(to.into());
        self
    }

    fn say(&self, lang: &str, key: &str) -> String {
        let values: Vec<(&str, fluent::FluentValue)> = self
            .args
            .iter()
            .map(|(k, v)| {
                let v = match v {
                    Arg::Num(n) => fluent::FluentValue::from(*n),
                    Arg::Text(t) => fluent::FluentValue::from(t.clone()),
                };
                (*k, v)
            })
            .collect();
        let args = crate::i18n::args(&values);
        crate::i18n::translate(lang, key, Some(&args))
    }

    /// What a page or a monitor is given: every sentence in `lang`.
    pub fn to_json(&self, lang: &str) -> Value {
        json!({
            "id": self.id,
            "area": self.area,
            "level": self.level.as_str(),
            "title": crate::i18n::translate(lang, self.title, None),
            "finding": self.say(lang, self.finding),
            "fix": self.fix.map(|k| self.say(lang, k)),
            "link": self.link,
        })
    }
}

/// How many are green, warnings and critical.
pub fn tally(items: &[Item]) -> (usize, usize, usize) {
    items.iter().fold((0, 0, 0), |(o, w, f), i| match i.level {
        Level::Ok => (o + 1, w, f),
        Level::Warn => (o, w + 1, f),
        Level::Fail => (o, w, f + 1),
    })
}

/// The worst level among them.
pub fn worst(items: &[Item]) -> Level {
    items.iter().map(|i| i.level).max().unwrap_or(Level::Ok)
}

/// The items for a page: grouped by area in `areas`' order, each group in the
/// order its items were checked, the worst first within a group.
pub fn grouped(items: &[Item], areas: &[&str], lang: &str) -> Vec<Value> {
    areas
        .iter()
        .filter_map(|area| {
            let mut mine: Vec<&Item> = items.iter().filter(|i| i.area == *area).collect();
            if mine.is_empty() {
                return None;
            }
            mine.sort_by_key(|i| std::cmp::Reverse(i.level));
            Some(json!({
                "area": area,
                "items": mine.iter().map(|i| i.to_json(lang)).collect::<Vec<_>>(),
            }))
        })
        .collect()
}
