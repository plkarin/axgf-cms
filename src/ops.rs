//! One set of operations on the family's archive (docs/adr/0045).
//!
//! The AFG's commands and the MCP's tools are both these, checked by the same
//! rules as a page's writes and committed by the same function. An operation
//! is JSON (`{"op": "create_person", …}`), so that the AFG can write a block
//! of them in its answer and an AI can send one through the MCP.
//!
//! A batch is compiled against a working copy of the archive, one operation
//! after another, so that a later one sees what an earlier one made — by its
//! `ref` (`"new:1"`), or by its name. Each write is checked as the record's
//! editor checks its own (`access::check_write`: the role, then the account's
//! branch, on the record before and after). The writes are then folded, one
//! per record, and committed by [`AppState::commit`]: version-checked, all or
//! nothing, validated by the AXGF library, one journal entry a record. Reads
//! go through the reader's lens (docs/adr/0001): a person they may not see is
//! a person they cannot find.

use std::collections::{BTreeMap, BTreeSet};

use axgf_rs::EntityKind;
use fluent::{FluentArgs, FluentValue};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};

use crate::access::{self, Denied, Viewer};
use crate::person_new::{self, Who};
use crate::state::{AppState, BatchOutcome, Op};

/// The most operations one batch may hold.
pub const MOST: usize = 60;

/// A person's birth or death, as an operation gives it.
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq)]
pub struct Fact {
    /// `1890`, `1890-06` or `1890-06-14`.
    #[serde(default)]
    pub date: Option<String>,
    /// The date is approximate.
    #[serde(default)]
    pub circa: Option<bool>,
    /// A place, by id or by its name.
    #[serde(default)]
    pub place: Option<String>,
}

/// A union a new person comes with.
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq)]
pub struct UnionSpec {
    #[serde(default)]
    pub partner: Option<String>,
    #[serde(default)]
    pub kind: Option<String>,
    #[serde(default)]
    pub children: Vec<String>,
}

/// A person to create, with what surrounds them (docs/adr/0039).
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq)]
pub struct NewPerson {
    /// A name for this person later in the batch: `"new:1"`.
    #[serde(default, rename = "ref")]
    pub reference: Option<String>,
    #[serde(default)]
    pub given: String,
    #[serde(default)]
    pub family: String,
    #[serde(default)]
    pub nicknames: Vec<String>,
    /// `M`, `F`, `NB` or `U`.
    #[serde(default)]
    pub gender: Option<String>,
    #[serde(default)]
    pub living: Option<bool>,
    #[serde(default)]
    pub birth: Option<Fact>,
    #[serde(default)]
    pub death: Option<Fact>,
    /// Markdown.
    #[serde(default)]
    pub bio: Option<String>,
    #[serde(default)]
    pub father: Option<String>,
    #[serde(default)]
    pub mother: Option<String>,
    #[serde(default)]
    pub siblings: Vec<String>,
    #[serde(default)]
    pub unions: Vec<UnionSpec>,
}

/// What changes on a person. A field left out is kept; an empty string
/// removes it.
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq)]
pub struct PersonChange {
    pub person: String,
    #[serde(default)]
    pub given: Option<String>,
    #[serde(default)]
    pub family: Option<String>,
    #[serde(default)]
    pub nicknames: Option<Vec<String>>,
    #[serde(default)]
    pub gender: Option<String>,
    #[serde(default)]
    pub living: Option<bool>,
    /// `public`, `members`, `contributors` or `private`.
    #[serde(default)]
    pub visibility: Option<String>,
    #[serde(default)]
    pub bio: Option<String>,
}

/// A birth or a death set, changed or removed.
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq)]
pub struct FactChange {
    pub person: String,
    /// An empty string removes the date.
    #[serde(default)]
    pub date: Option<String>,
    #[serde(default)]
    pub circa: Option<bool>,
    /// An empty string removes the place.
    #[serde(default)]
    pub place: Option<String>,
    /// Remove the fact altogether.
    #[serde(default)]
    pub clear: bool,
}

/// Someone else in an event.
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq)]
pub struct Participant {
    pub person: String,
    #[serde(default)]
    pub role: Option<String>,
}

/// An event to add to a person.
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq)]
pub struct NewEvent {
    #[serde(default, rename = "ref")]
    pub reference: Option<String>,
    pub person: String,
    /// `religious`, `marriage`, `military`, `migration`, `census`, `other`…
    pub category: String,
    /// `baptism`, `burial`, `emigration`…
    #[serde(default)]
    pub subcategory: Option<String>,
    #[serde(default)]
    pub date: Option<String>,
    #[serde(default)]
    pub circa: Option<bool>,
    #[serde(default)]
    pub place: Option<String>,
    /// The person's role: `subject` by default.
    #[serde(default)]
    pub role: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub with: Vec<Participant>,
}

/// What changes on an event. A field left out is kept; an empty string
/// removes it.
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq)]
pub struct EventChange {
    pub event: String,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default)]
    pub subcategory: Option<String>,
    #[serde(default)]
    pub date: Option<String>,
    #[serde(default)]
    pub circa: Option<bool>,
    #[serde(default)]
    pub place: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

/// A union to make between people the archive holds.
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq)]
pub struct NewUnion {
    #[serde(default, rename = "ref")]
    pub reference: Option<String>,
    /// One or two people.
    pub partners: Vec<String>,
    /// `marriage`, `civil_union`, `cohabitation`, `religious_only`,
    /// `polygamous` or `unknown`.
    #[serde(default)]
    pub kind: Option<String>,
    /// The marriage's date and place, when there is one: an event of its own.
    #[serde(default)]
    pub date: Option<String>,
    #[serde(default)]
    pub circa: Option<bool>,
    #[serde(default)]
    pub place: Option<String>,
    #[serde(default)]
    pub children: Vec<String>,
}

/// One operation.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum Operation {
    /// People whose name holds every word of the query.
    Search {
        query: String,
        #[serde(default)]
        limit: Option<usize>,
    },
    /// A person: names, facts, relatives, events.
    GetPerson {
        person: String,
    },
    /// A family: its partners, its union, its children.
    GetFamily {
        family: String,
    },
    CreatePerson(NewPerson),
    UpdatePerson(PersonChange),
    SetBirth(FactChange),
    SetDeath(FactChange),
    AddEvent(NewEvent),
    UpdateEvent(EventChange),
    DeleteEvent {
        event: String,
    },
    AddUnion(NewUnion),
    /// A child added to a family: the one named, or the one whose partners
    /// are exactly the parents named (made if there is none).
    AddChild {
        child: String,
        #[serde(default)]
        parents: Vec<String>,
        #[serde(default)]
        family: Option<String>,
    },
    RemoveChild {
        child: String,
        family: String,
    },
    /// A union undone: the family goes; its children stay, without these
    /// parents.
    DeleteUnion {
        family: String,
    },
    /// A person, taken out of every family and event, then deleted.
    /// Administrators only.
    DeletePerson {
        person: String,
    },
}

impl Operation {
    /// Its name, as JSON names it.
    pub fn name(&self) -> &'static str {
        match self {
            Operation::Search { .. } => "search",
            Operation::GetPerson { .. } => "get_person",
            Operation::GetFamily { .. } => "get_family",
            Operation::CreatePerson(_) => "create_person",
            Operation::UpdatePerson(_) => "update_person",
            Operation::SetBirth(_) => "set_birth",
            Operation::SetDeath(_) => "set_death",
            Operation::AddEvent(_) => "add_event",
            Operation::UpdateEvent(_) => "update_event",
            Operation::DeleteEvent { .. } => "delete_event",
            Operation::AddUnion(_) => "add_union",
            Operation::AddChild { .. } => "add_child",
            Operation::RemoveChild { .. } => "remove_child",
            Operation::DeleteUnion { .. } => "delete_union",
            Operation::DeletePerson { .. } => "delete_person",
        }
    }

    /// Whether it only reads.
    pub fn reads(&self) -> bool {
        matches!(
            self,
            Operation::Search { .. } | Operation::GetPerson { .. } | Operation::GetFamily { .. }
        )
    }
}

/// A batch, read from JSON: an array of operations, one operation, or
/// `{"operations": [...]}`.
pub fn parse(text: &str) -> Result<Vec<Operation>, String> {
    let value: Value = serde_json::from_str(text.trim()).map_err(|e| e.to_string())?;
    let list = match value {
        Value::Array(a) => a,
        Value::Object(mut o) => match o.remove("operations") {
            Some(Value::Array(a)) => a,
            Some(_) => return Err("`operations` is not a list".into()),
            None => vec![Value::Object(o)],
        },
        _ => return Err("not an operation".into()),
    };
    if list.len() > MOST {
        return Err(format!("{} operations, {MOST} at most", list.len()));
    }
    list.into_iter()
        .enumerate()
        .map(|(i, v)| serde_json::from_value(v).map_err(|e| format!("operation {}: {e}", i + 1)))
        .collect()
}

/// An argument of a line, said in the reader's language when the line is.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Piece {
    Text(String),
    /// A vocabulary's value (`event-category`, `union-type`), then text.
    Vocab {
        family: &'static str,
        value: String,
        then: String,
    },
    /// Catalogue keys, each in the reader's language, joined by commas.
    Keys(Vec<&'static str>),
}

impl Piece {
    fn said(&self, lang: &str) -> String {
        match self {
            Piece::Text(t) => t.clone(),
            Piece::Vocab {
                family,
                value,
                then,
            } => format!("{}{then}", crate::i18n::vocab(lang, family, value)),
            Piece::Keys(keys) => keys
                .iter()
                .map(|k| crate::i18n::translate(lang, k, None))
                .collect::<Vec<_>>()
                .join(", "),
        }
    }
}

/// A part of an answer of the AFG: Markdown, or an `axgf-ops` block's JSON.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Segment<'a> {
    Text(&'a str),
    Ops(&'a str),
}

/// An answer cut into its Markdown and its `axgf-ops` blocks, in order
/// (docs/adr/0045 §2). A block left open runs to the end of the answer.
pub fn blocks(text: &str) -> Vec<Segment<'_>> {
    let mut out = Vec::new();
    let (mut from, mut at) = (0, 0);
    let mut open: Option<(usize, char, usize)> = None;
    for line in text.split_inclusive('\n') {
        let t = line.trim();
        match open {
            None => {
                let mark = t.chars().next().filter(|c| *c == '`' || *c == '~');
                if let Some(c) = mark {
                    let run = t.chars().take_while(|x| *x == c).count();
                    if run >= 3 && t[run..].trim().eq_ignore_ascii_case("axgf-ops") {
                        if at > from {
                            out.push(Segment::Text(&text[from..at]));
                        }
                        open = Some((at + line.len(), c, run));
                    }
                }
            }
            Some((start, c, run)) => {
                if t.len() >= run && t.chars().all(|x| x == c) {
                    out.push(Segment::Ops(&text[start..at]));
                    open = None;
                    from = at + line.len();
                }
            }
        }
        at += line.len();
    }
    match open {
        Some((start, _, _)) => out.push(Segment::Ops(&text[start..])),
        None if from < text.len() => out.push(Segment::Text(&text[from..])),
        None => {}
    }
    out
}

/// The `axgf-ops` blocks of an answer, in order.
pub fn ops_blocks(text: &str) -> Vec<&str> {
    blocks(text)
        .into_iter()
        .filter_map(|s| match s {
            Segment::Ops(j) => Some(j),
            Segment::Text(_) => None,
        })
        .collect()
}

/// A nickname as a name is written with it in `lang`.
pub fn quoted(lang: &str, nick: &str) -> String {
    let mut args = FluentArgs::new();
    args.set("nick", FluentValue::from(nick.to_string()));
    crate::i18n::translate(lang, "person-new-nickname-quoted", Some(&args))
}

/// A line of what a batch does, for the reader: a catalogue key and its
/// arguments.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Line {
    pub key: &'static str,
    pub args: Vec<(&'static str, Piece)>,
}

impl Line {
    fn new(key: &'static str, args: &[(&'static str, String)]) -> Self {
        Line {
            key,
            args: args
                .iter()
                .map(|(k, v)| (*k, Piece::Text(v.clone())))
                .collect(),
        }
    }

    fn with(key: &'static str, args: Vec<(&'static str, Piece)>) -> Self {
        Line { key, args }
    }

    /// The line in a language.
    pub fn text(&self, lang: &str) -> String {
        let args: Vec<(&'static str, String)> =
            self.args.iter().map(|(k, p)| (*k, p.said(lang))).collect();
        say(lang, self.key, &args)
    }
}

/// Why a batch cannot be applied: the operation (from 1, or 0 for the batch
/// as a whole), a catalogue key and its arguments.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Refusal {
    pub index: usize,
    pub key: &'static str,
    pub args: Vec<(&'static str, String)>,
    /// The new-person page's own refusal, by its key, given as `$why`.
    pub why: Option<&'static str>,
}

impl Refusal {
    fn of(key: &'static str, args: &[(&'static str, String)]) -> Self {
        Refusal {
            index: 0,
            key,
            args: args.to_vec(),
            why: None,
        }
    }

    /// The refusal in a language.
    pub fn text(&self, lang: &str) -> String {
        let mut args = self.args.clone();
        if let Some(k) = self.why {
            args.push(("why", say(lang, k, &[])));
        }
        let why = say(lang, self.key, &args);
        if self.index == 0 {
            return why;
        }
        say(
            lang,
            "ops-refused-at",
            &[("n", self.index.to_string()), ("why", why)],
        )
    }
}

fn say(lang: &str, key: &str, args: &[(&'static str, String)]) -> String {
    let mut fa = FluentArgs::new();
    for (k, v) in args {
        fa.set(*k, FluentValue::from(v.clone()));
    }
    crate::i18n::translate(lang, key, Some(&fa))
}

/// A batch compiled: the writes, folded one per record, what they do, and the
/// ids its `ref`s name.
#[derive(Debug, Clone)]
pub struct Compiled {
    pub ops: Vec<Op>,
    pub lines: Vec<Line>,
    pub refs: BTreeMap<String, String>,
}

/// A batch applied.
#[derive(Debug, Clone, Serialize)]
pub struct Done {
    pub records: usize,
    pub lines: Vec<Line>,
    pub refs: BTreeMap<String, String>,
}

struct Work<'a> {
    flat: Value,
    viewer: &'a Viewer,
    scope: Option<BTreeSet<String>>,
    refs: BTreeMap<String, String>,
    ops: Vec<Op>,
    lines: Vec<Line>,
    now: &'a str,
    quote: &'a dyn Fn(&str) -> String,
    index: usize,
}

const VISIBILITIES: &[&str] = &["public", "members", "contributors", "private"];
/// An event's categories, as the events editor offers them.
pub const EVENT_CATEGORIES: &[&str] = &[
    "birth",
    "death",
    "marriage",
    "divorce",
    "adoption",
    "migration",
    "naturalization",
    "military",
    "incarceration",
    "name_change",
    "census",
    "legal",
    "religious",
    "social",
    "historical",
    "other",
];

/// A person's roles in an event, as the events editor offers them.
pub const ROLES: &[&str] = &[
    "subject",
    "participant",
    "spouse",
    "spouse_1",
    "spouse_2",
    "witness",
    "officiant",
    "informant",
    "godparent",
];

fn folded(s: &str) -> String {
    s.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

fn gender_of(input: &str) -> Option<&'static str> {
    match folded(input).as_str() {
        "m" | "male" | "man" | "homme" | "masculin" | "h" => Some("M"),
        "f" | "female" | "woman" | "femme" | "féminin" | "feminin" => Some("F"),
        "nb" | "non-binary" | "nonbinary" | "non binaire" | "non-binaire" => Some("NB"),
        "u" | "unknown" | "inconnu" | "" => Some("U"),
        _ => None,
    }
}

fn arg(k: &'static str, v: impl Into<String>) -> (&'static str, String) {
    (k, v.into())
}

/// A field of the new-person page, named as `create_person` names it.
fn field_name(field: &str) -> String {
    match field {
        "birth_date" => "birth.date".into(),
        "birth_place" => "birth.place".into(),
        "death_date" => "death.date".into(),
        "death_place" => "death.place".into(),
        "sibling" => "siblings".into(),
        f => match f.strip_prefix("u.").and_then(|r| r.split_once('.')) {
            Some((n, "partner")) => format!("unions[{n}].partner"),
            Some((n, "child")) => format!("unions[{n}].children"),
            _ => f.to_string(),
        },
    }
}

impl<'a> Work<'a> {
    fn refuse(&self, key: &'static str, args: &[(&'static str, String)]) -> Refusal {
        Refusal {
            index: self.index,
            ..Refusal::of(key, args)
        }
    }

    fn get(&self, collection: &str, id: &str) -> Option<&Value> {
        self.flat.get(collection).and_then(|c| c.get(id))
    }

    /// What a typed reference names: a `ref` of this batch, or the input.
    fn deref(&self, input: &str) -> String {
        let input = input.trim();
        self.refs
            .get(input)
            .cloned()
            .unwrap_or_else(|| input.to_string())
    }

    /// A person the archive holds (or this batch made): by `ref`, by id, by a
    /// list label, or by a name exactly one person bears.
    fn person(&self, input: &str) -> Result<String, Refusal> {
        let typed = self.deref(input);
        if typed.is_empty() {
            return Err(self.refuse("ops-refused-no-person", &[]));
        }
        match person_new::who(&typed, &self.flat) {
            Ok(Some(Who::Existing(id))) => Ok(id),
            Ok(_) => Err(self.refuse("ops-refused-not-found", &[arg("what", input.trim())])),
            Err(crate::forms::PickError::Ambiguous) => {
                Err(self.refuse("ops-refused-ambiguous", &[arg("what", input.trim())]))
            }
            Err(_) => Err(self.refuse("ops-refused-not-found", &[arg("what", input.trim())])),
        }
    }

    fn record(&self, collection: &'static str, input: &str) -> Result<String, Refusal> {
        let id = self.deref(input);
        let kind = match collection {
            "families" => Some(EntityKind::Family),
            "events" => Some(EntityKind::Event),
            _ => None,
        };
        // A family or an event of nobody the reader may see is nothing to
        // them: the working copy holds only the people they may see.
        let seen = self.get(collection, &id).is_some_and(|e| {
            let subjects = kind.map(|k| access::subjects_of(k, e)).unwrap_or_default();
            subjects.is_empty() || subjects.iter().any(|s| self.get("persons", s).is_some())
        });
        if seen {
            Ok(id)
        } else {
            Err(self.refuse("ops-refused-not-found", &[arg("what", input.trim())]))
        }
    }

    fn name(&self, id: &str) -> String {
        self.get("persons", id)
            .map(crate::view::person_display_name)
            .filter(|n| !n.trim().is_empty())
            .unwrap_or_else(|| id.chars().take(8).collect())
    }

    /// A place by id or by name; a name nobody has makes a new place.
    fn place(&mut self, input: &str) -> Result<Option<String>, Refusal> {
        let typed = self.deref(input);
        match person_new::place(&typed, &self.flat) {
            Ok(None) => Ok(None),
            Ok(Some(person_new::Where::Existing(id))) => Ok(Some(id)),
            Ok(Some(person_new::Where::New(name))) => {
                let id = uuid::Uuid::new_v4().to_string();
                let mut e = json!({"id": id, "type": "place", "axgf_version": "1.0",
                    "names": [{"lang": "und", "value": name, "is_primary": true}]});
                stamp(&mut e, self.now);
                self.push(Op::Create {
                    kind: EntityKind::Place,
                    entity: e,
                })?;
                self.lines
                    .push(Line::new("ops-line-create-place", &[arg("name", name)]));
                Ok(Some(id))
            }
            Err(crate::forms::PickError::Ambiguous) => {
                Err(self.refuse("ops-refused-ambiguous", &[arg("what", input.trim())]))
            }
            Err(_) => Err(self.refuse("ops-refused-not-found", &[arg("what", input.trim())])),
        }
    }

    fn date(&self, value: &str, circa: bool) -> Result<Option<Value>, Refusal> {
        let value = value.trim();
        if value.is_empty() {
            return Ok(None);
        }
        if crate::notes::date_of(value).is_none() {
            return Err(self.refuse("ops-refused-date", &[arg("value", value)]));
        }
        Ok(crate::forms::date_object(Some(value), None, circa))
    }

    /// A write, checked as a page's are, then made on the working copy.
    fn push(&mut self, op: Op) -> Result<(), Refusal> {
        let (kind, proposed, existing) = match &op {
            Op::Create { kind, entity } => (*kind, Some(entity.clone()), None),
            Op::Update {
                kind, id, entity, ..
            } => (
                *kind,
                Some(entity.clone()),
                self.get(crate::admin::collection_for(*kind), id).cloned(),
            ),
            Op::Delete { kind, id, .. } => (
                *kind,
                None,
                self.get(crate::admin::collection_for(*kind), id).cloned(),
            ),
            _ => return Err(self.refuse("ops-refused-invalid", &[arg("detail", "document")])),
        };
        let mut subjects = proposed
            .as_ref()
            .map(|e| access::subjects_of(kind, e))
            .unwrap_or_default();
        if let Some(e) = &existing {
            subjects.extend(access::subjects_of(kind, e));
        }
        subjects.sort();
        subjects.dedup();
        match access::check_write(self.viewer, self.scope.as_ref(), &subjects) {
            Ok(()) => {}
            Err(Denied::Role) => return Err(self.refuse("ops-refused-role", &[])),
            Err(Denied::Scope) => return Err(self.refuse("ops-refused-scope", &[])),
        }
        let collection = crate::admin::collection_for(kind);
        let root = self.flat.as_object_mut().expect("an archive is an object");
        let table = root
            .entry(collection)
            .or_insert_with(|| Value::Object(Map::new()));
        match &op {
            Op::Create { entity, .. } | Op::Update { entity, .. } => {
                let id = entity
                    .get("id")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_string();
                table[id] = entity.clone();
            }
            Op::Delete { id, .. } => {
                if let Some(t) = table.as_object_mut() {
                    t.remove(id);
                }
            }
            _ => {}
        }
        self.ops.push(op);
        Ok(())
    }

    fn update(&mut self, kind: EntityKind, id: &str, mut entity: Value) -> Result<(), Refusal> {
        entity["updated_at"] = json!(self.now);
        entity["id"] = json!(id);
        self.push(Op::Update {
            kind,
            id: id.to_string(),
            expected_version: 0,
            entity,
        })
    }

    fn delete(&mut self, kind: EntityKind, id: &str) -> Result<(), Refusal> {
        self.push(Op::Delete {
            kind,
            id: id.to_string(),
            expected_version: 0,
        })
    }
}

fn stamp(entity: &mut Value, now: &str) {
    entity["version_num"] = json!(1);
    entity["created_at"] = json!(now);
    entity["updated_at"] = json!(now);
}

/// The family whose partners are exactly these people.
fn union_of(flat: &Value, set: &BTreeSet<String>) -> Option<String> {
    flat.get("families")?
        .as_object()?
        .iter()
        .find_map(|(fid, f)| {
            let partners: BTreeSet<String> = f
                .pointer("/union/persons")?
                .as_array()?
                .iter()
                .filter_map(|p| p.get("person_id").and_then(Value::as_str))
                .map(str::to_string)
                .collect();
            (partners == *set).then(|| fid.clone())
        })
}

fn children_of(family: &Value) -> Vec<String> {
    family
        .get("children")
        .and_then(Value::as_array)
        .map(|c| {
            c.iter()
                .filter_map(|k| k.get("person_id").and_then(Value::as_str))
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default()
}

fn partners_of(family: &Value) -> Vec<String> {
    family
        .pointer("/union/persons")
        .and_then(Value::as_array)
        .map(|c| {
            c.iter()
                .filter_map(|k| k.get("person_id").and_then(Value::as_str))
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default()
}

/// The name's components rebuilt with what changes, and the display name
/// they make.
fn renamed(person: &mut Value, change: &PersonChange, quote: &dyn Fn(&str) -> String) {
    if change.given.is_none() && change.family.is_none() && change.nicknames.is_none() {
        return;
    }
    let comps: Vec<Value> = person
        .pointer("/identity/name/components")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let of = |kind: &str| -> Vec<String> {
        comps
            .iter()
            .filter(|c| c.get("type").and_then(Value::as_str) == Some(kind))
            .filter_map(|c| c.get("value").and_then(Value::as_str))
            .map(str::to_string)
            .collect()
    };
    let given = change
        .given
        .clone()
        .unwrap_or_else(|| of("given_name").join(" "));
    let family = change
        .family
        .clone()
        .unwrap_or_else(|| of("family_name").join(" "));
    let nicks = change.nicknames.clone().unwrap_or_else(|| of("nickname"));
    let sub = person_new::Submission {
        given: given.trim().to_string(),
        family: family.trim().to_string(),
        nicks: nicks
            .iter()
            .map(|n| n.trim().to_string())
            .filter(|n| !n.is_empty())
            .collect(),
        ..Default::default()
    };
    let mut kept: Vec<Value> = comps
        .into_iter()
        .filter(|c| {
            !matches!(
                c.get("type").and_then(Value::as_str),
                Some("given_name" | "family_name" | "nickname")
            )
        })
        .collect();
    let mut fresh = person_new::components(&sub);
    fresh.append(&mut kept);
    for (i, c) in fresh.iter_mut().enumerate() {
        c["order"] = json!(i + 1);
    }
    person["identity"]["name"]["components"] = Value::Array(fresh);
    person["identity"]["name"]["display"] = json!(person_new::display_name(&sub, quote));
}

/// Compile a batch against `flat`, as `viewer` would write it.
pub fn compile(
    flat: &Value,
    viewer: &Viewer,
    batch: &[Operation],
    now: &str,
    quote: &dyn Fn(&str) -> String,
) -> Result<Compiled, Refusal> {
    if batch.is_empty() {
        return Err(Refusal::of("ops-refused-empty", &[]));
    }
    // The reader's lens (docs/adr/0001): a person they may not see is nobody
    // to them here either, neither a name to match nor an id to name. Those
    // they may see, and every other record, make the working copy.
    let lens = viewer.lens(flat);
    let mut copy = flat.clone();
    if let Some(persons) = copy.get_mut("persons").and_then(Value::as_object_mut) {
        persons.retain(|id, _| lens.sees_person(id));
    }
    let mut w = Work {
        flat: copy,
        viewer,
        scope: viewer.scope(flat),
        refs: BTreeMap::new(),
        ops: Vec::new(),
        lines: Vec::new(),
        now,
        quote,
        index: 0,
    };
    if !w.flat.is_object() {
        w.flat = json!({});
    }
    for (i, op) in batch.iter().enumerate() {
        w.index = i + 1;
        step(&mut w, op)?;
    }
    let ops = fold(std::mem::take(&mut w.ops), flat);
    dangling(&ops, flat, &w.flat)?;
    Ok(Compiled {
        ops,
        lines: w.lines,
        refs: w.refs,
    })
}

fn step(w: &mut Work<'_>, op: &Operation) -> Result<(), Refusal> {
    match op {
        Operation::Search { .. } | Operation::GetPerson { .. } | Operation::GetFamily { .. } => {
            Ok(())
        }
        Operation::CreatePerson(np) => create_person(w, np),
        Operation::UpdatePerson(pc) => update_person(w, pc),
        Operation::SetBirth(fc) => set_fact(w, fc, "birth"),
        Operation::SetDeath(fc) => set_fact(w, fc, "death"),
        Operation::AddEvent(ne) => add_event(w, ne),
        Operation::UpdateEvent(ec) => update_event(w, ec),
        Operation::DeleteEvent { event } => {
            let id = w.record("events", event)?;
            let label = event_label(w, &id);
            w.delete(EntityKind::Event, &id)?;
            w.lines
                .push(Line::with("ops-line-delete-event", vec![("event", label)]));
            Ok(())
        }
        Operation::AddUnion(nu) => add_union(w, nu),
        Operation::AddChild {
            child,
            parents,
            family,
        } => add_child(w, child, parents, family.as_deref()),
        Operation::RemoveChild { child, family } => {
            let kid = w.person(child)?;
            let fid = w.record("families", family)?;
            let mut fam = w.get("families", &fid).cloned().unwrap_or_default();
            let kids = children_of(&fam);
            if !kids.contains(&kid) {
                return Err(w.refuse("ops-refused-not-a-child", &[arg("child", w.name(&kid))]));
            }
            let rest: Vec<Value> = fam
                .get("children")
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default()
                .into_iter()
                .filter(|c| c.get("person_id").and_then(Value::as_str) != Some(kid.as_str()))
                .collect();
            fam["children"] = Value::Array(rest);
            let parents = parents_label(w, &fam);
            w.update(EntityKind::Family, &fid, fam)?;
            w.lines.push(Line::new(
                "ops-line-remove-child",
                &[arg("child", w.name(&kid)), arg("parents", parents)],
            ));
            Ok(())
        }
        Operation::DeleteUnion { family } => {
            let fid = w.record("families", family)?;
            let fam = w.get("families", &fid).cloned().unwrap_or_default();
            let partners = parents_label(w, &fam);
            w.delete(EntityKind::Family, &fid)?;
            w.lines.push(Line::new(
                "ops-line-delete-union",
                &[arg("partners", partners)],
            ));
            Ok(())
        }
        Operation::DeletePerson { person } => delete_person(w, person),
    }
}

fn create_person(w: &mut Work<'_>, np: &NewPerson) -> Result<(), Refusal> {
    let birth = np.birth.clone().unwrap_or_default();
    let death = np.death.clone().unwrap_or_default();
    let died = death.date.as_deref().is_some_and(|d| !d.trim().is_empty())
        || death.place.as_deref().is_some_and(|p| !p.trim().is_empty());
    let born = birth
        .date
        .as_deref()
        .and_then(|d| d.trim().get(..4))
        .and_then(|y| y.parse::<i32>().ok());
    let this_year: i32 = w.now.get(..4).and_then(|y| y.parse().ok()).unwrap_or(2026);
    // Unknown: living, as the new-person page takes a relative to be, unless
    // they died or were born more than 110 years ago.
    let living = np
        .living
        .unwrap_or(!died && born.is_none_or(|y| this_year - y <= 110));
    let gender = match np.gender.as_deref() {
        None => "U",
        Some(g) => {
            gender_of(g).ok_or_else(|| w.refuse("ops-refused-gender", &[arg("value", g)]))?
        }
    };
    let sub = person_new::Submission {
        given: np.given.trim().to_string(),
        nicks: np
            .nicknames
            .iter()
            .map(|n| n.trim().to_string())
            .filter(|n| !n.is_empty())
            .collect(),
        family: np.family.trim().to_string(),
        gender: gender.into(),
        living,
        birth_date: birth.date.clone().unwrap_or_default(),
        birth_circa: birth.circa.unwrap_or(false),
        birth_place: birth
            .place
            .as_deref()
            .map(|p| w.deref(p))
            .unwrap_or_default(),
        death_date: death.date.clone().unwrap_or_default(),
        death_circa: death.circa.unwrap_or(false),
        death_place: death
            .place
            .as_deref()
            .map(|p| w.deref(p))
            .unwrap_or_default(),
        bio: np.bio.clone().unwrap_or_default(),
        father: np.father.as_deref().map(|p| w.deref(p)).unwrap_or_default(),
        mother: np.mother.as_deref().map(|p| w.deref(p)).unwrap_or_default(),
        siblings: np.siblings.iter().map(|s| w.deref(s)).collect(),
        unions: np
            .unions
            .iter()
            .map(|u| person_new::UnionIn {
                partner: u.partner.as_deref().map(|p| w.deref(p)).unwrap_or_default(),
                kind: u.kind.clone().unwrap_or_else(|| "unknown".into()),
                children: u.children.iter().map(|c| w.deref(c)).collect(),
            })
            .collect(),
    };
    let quote = w.quote;
    let plan = person_new::plan(
        &sub,
        &person_new::Context {
            flat: &w.flat,
            quote,
            now: w.now,
            avatar: None,
            ids: None,
        },
    )
    .map_err(|r| Refusal {
        why: Some(r.key),
        ..w.refuse(
            "ops-refused-field",
            &[
                arg("field", field_name(&r.field)),
                arg("typed", r.typed.clone()),
            ],
        )
    })?;
    for op in plan.ops {
        w.push(op)?;
    }
    for c in &plan.created {
        if c.kind == EntityKind::Person {
            w.lines.push(Line::new(
                "ops-line-create-relative",
                &[arg("name", c.label.clone())],
            ));
        } else if c.kind == EntityKind::Place {
            w.lines.push(Line::new(
                "ops-line-create-place",
                &[arg("name", c.label.clone())],
            ));
        }
    }
    w.lines.push(Line::new(
        "ops-line-create-person",
        &[arg("name", w.name(&plan.person_id))],
    ));
    // Where the new person stands: whose child, whose partner.
    let families: Vec<Value> = w
        .flat
        .get("families")
        .and_then(Value::as_object)
        .map(|fs| fs.values().cloned().collect())
        .unwrap_or_default();
    for fam in &families {
        if children_of(fam).contains(&plan.person_id) && !partners_of(fam).is_empty() {
            w.lines.push(Line::new(
                "ops-line-add-child",
                &[
                    arg("child", w.name(&plan.person_id)),
                    arg("parents", parents_label(w, fam)),
                ],
            ));
        }
        if partners_of(fam).contains(&plan.person_id) {
            let kind = fam
                .pointer("/union/type")
                .and_then(Value::as_str)
                .unwrap_or("unknown")
                .to_string();
            w.lines.push(Line::with(
                "ops-line-add-union",
                vec![
                    ("partners", Piece::Text(parents_label(w, fam))),
                    (
                        "kind",
                        Piece::Vocab {
                            family: "union-type",
                            value: kind,
                            then: String::new(),
                        },
                    ),
                ],
            ));
            for k in children_of(fam) {
                w.lines.push(Line::new(
                    "ops-line-add-child",
                    &[
                        arg("child", w.name(&k)),
                        arg("parents", parents_label(w, fam)),
                    ],
                ));
            }
        }
    }
    if let Some(r) = &np.reference {
        w.refs.insert(r.trim().to_string(), plan.person_id.clone());
    }
    Ok(())
}

fn update_person(w: &mut Work<'_>, pc: &PersonChange) -> Result<(), Refusal> {
    let id = w.person(&pc.person)?;
    let mut person = w.get("persons", &id).cloned().unwrap_or_default();
    let mut what: Vec<&'static str> = Vec::new();
    if pc.given.is_some() || pc.family.is_some() || pc.nicknames.is_some() {
        let quote = w.quote;
        renamed(&mut person, pc, quote);
        if person
            .pointer("/identity/name/display")
            .and_then(Value::as_str)
            .is_none_or(|d| d.trim().is_empty())
        {
            return Err(w.refuse("ops-refused-name", &[]));
        }
        what.push("ops-field-name");
    }
    if let Some(g) = &pc.gender {
        let g = gender_of(g).ok_or_else(|| w.refuse("ops-refused-gender", &[arg("value", g)]))?;
        person["identity"]["gender"] = json!({"value": g});
        what.push("ops-field-gender");
    }
    if let Some(l) = pc.living {
        person["identity"]["is_living"] = json!(l);
        what.push("ops-field-living");
    }
    if let Some(v) = &pc.visibility {
        let v = v.trim();
        if !VISIBILITIES.contains(&v) {
            return Err(w.refuse("ops-refused-visibility", &[arg("value", v)]));
        }
        person["identity"]["visibility"] = json!(v);
        what.push("ops-field-visibility");
    }
    if let Some(b) = &pc.bio {
        if b.trim().is_empty() {
            if let Some(o) = person.as_object_mut() {
                o.remove("bio");
            }
        } else {
            person["bio"] = json!(b.trim());
            person_new::mark_bio_markdown(&mut person);
        }
        what.push("ops-field-bio");
    }
    if what.is_empty() {
        return Err(w.refuse("ops-refused-nothing", &[]));
    }
    w.update(EntityKind::Person, &id, person)?;
    w.lines.push(Line::with(
        "ops-line-update-person",
        vec![
            ("name", Piece::Text(w.name(&id))),
            ("fields", Piece::Keys(what)),
        ],
    ));
    Ok(())
}

fn set_fact(w: &mut Work<'_>, fc: &FactChange, key: &'static str) -> Result<(), Refusal> {
    let id = w.person(&fc.person)?;
    let mut person = w.get("persons", &id).cloned().unwrap_or_default();
    let mut fact = person.get(key).cloned().unwrap_or_else(|| json!({}));
    if fc.clear {
        if let Some(o) = person.as_object_mut() {
            o.remove(key);
        }
    } else {
        if fc.date.is_none() && fc.place.is_none() && fc.circa.is_none() {
            return Err(w.refuse("ops-refused-nothing", &[]));
        }
        let circa = fc.circa.unwrap_or_else(|| {
            fact.pointer("/date/circa")
                .and_then(Value::as_bool)
                .unwrap_or(false)
        });
        match &fc.date {
            Some(d) if d.trim().is_empty() => {
                fact.as_object_mut().map(|o| o.remove("date"));
            }
            Some(d) => fact["date"] = w.date(d, circa)?.unwrap_or(Value::Null),
            None => {
                if let Some(v) = fact.get("date").and_then(|d| d.get("value")).cloned() {
                    let v = v.as_str().unwrap_or_default().to_string();
                    fact["date"] = w.date(&v, circa)?.unwrap_or(Value::Null);
                }
            }
        }
        match &fc.place {
            Some(p) if p.trim().is_empty() => {
                fact.as_object_mut().map(|o| o.remove("place_id"));
            }
            Some(p) => {
                if let Some(pid) = w.place(p)? {
                    fact["place_id"] = json!(pid);
                }
            }
            None => {}
        }
        if fact.as_object().is_some_and(Map::is_empty) {
            person.as_object_mut().map(|o| o.remove(key));
        } else {
            person[key] = fact;
        }
        if key == "death" {
            person["identity"]["is_living"] = json!(false);
        }
    }
    let text = fact_text(w, person.get(key));
    w.update(EntityKind::Person, &id, person)?;
    w.lines.push(Line::new(
        if key == "birth" {
            "ops-line-set-birth"
        } else {
            "ops-line-set-death"
        },
        &[arg("name", w.name(&id)), arg("fact", text)],
    ));
    Ok(())
}

/// A fact in a few words: its date, its place.
fn fact_text(w: &Work<'_>, fact: Option<&Value>) -> String {
    let Some(f) = fact else {
        return "—".into();
    };
    let date = f
        .pointer("/date/value")
        .and_then(Value::as_str)
        .unwrap_or_default();
    let circa = f
        .pointer("/date/circa")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let place = f
        .get("place_id")
        .and_then(Value::as_str)
        .and_then(|p| w.get("places", p))
        .map(crate::forms::place_label)
        .unwrap_or_default();
    let mut out = String::new();
    if circa && !date.is_empty() {
        out.push('~');
    }
    out.push_str(date);
    if !place.is_empty() {
        if !out.is_empty() {
            out.push_str(", ");
        }
        out.push_str(&place);
    }
    if out.is_empty() {
        "—".into()
    } else {
        out
    }
}

/// An event in a few words: what it is, in the reader's language, then who
/// was in it and when.
fn event_label(w: &Work<'_>, id: &str) -> Piece {
    let Some(e) = w.get("events", id) else {
        return Piece::Text(id.chars().take(8).collect());
    };
    let cat = e
        .get("subcategory")
        .and_then(Value::as_str)
        .filter(|s| !s.trim().is_empty())
        .or_else(|| e.get("category").and_then(Value::as_str));
    let value = cat.unwrap_or("other").trim().to_string();
    let who: Vec<String> = e
        .get("participants")
        .and_then(Value::as_array)
        .map(|ps| {
            ps.iter()
                .filter_map(|p| p.get("entity_id").and_then(Value::as_str))
                .map(|p| w.name(p))
                .collect()
        })
        .unwrap_or_default();
    let date = e
        .pointer("/date/value")
        .and_then(Value::as_str)
        .unwrap_or_default();
    let mut then = String::new();
    if !who.is_empty() {
        then.push_str(" — ");
        then.push_str(&who.join(", "));
    }
    if !date.is_empty() {
        then.push_str(&format!(" ({date})"));
    }
    Piece::Vocab {
        family: "event-category",
        value,
        then,
    }
}

fn parents_label(w: &Work<'_>, family: &Value) -> String {
    let names: Vec<String> = partners_of(family).iter().map(|p| w.name(p)).collect();
    if names.is_empty() {
        "—".into()
    } else {
        names.join(" & ")
    }
}

fn add_event(w: &mut Work<'_>, ne: &NewEvent) -> Result<(), Refusal> {
    let id = w.person(&ne.person)?;
    let category = ne.category.trim();
    if !EVENT_CATEGORIES.contains(&category) {
        return Err(w.refuse("ops-refused-category", &[arg("value", category)]));
    }
    let role = ne.role.as_deref().unwrap_or("subject").trim();
    if !ROLES.contains(&role) {
        return Err(w.refuse("ops-refused-role-name", &[arg("value", role)]));
    }
    let mut participants = vec![json!({"entity_type": "person", "entity_id": id, "role": role})];
    for p in &ne.with {
        let pid = w.person(&p.person)?;
        let r = p.role.as_deref().unwrap_or("participant").trim();
        if !ROLES.contains(&r) {
            return Err(w.refuse("ops-refused-role-name", &[arg("value", r)]));
        }
        participants.push(json!({"entity_type": "person", "entity_id": pid, "role": r}));
    }
    let eid = uuid::Uuid::new_v4().to_string();
    let mut ev = json!({"id": eid, "type": "event", "axgf_version": "1.0",
        "category": category, "participants": participants});
    if let Some(s) = ne.subcategory.as_deref().filter(|s| !s.trim().is_empty()) {
        ev["subcategory"] = json!(s.trim());
    }
    if let Some(d) = ne.date.as_deref() {
        if let Some(date) = w.date(d, ne.circa.unwrap_or(false))? {
            ev["date"] = date;
        }
    }
    if let Some(p) = ne.place.as_deref() {
        if let Some(pid) = w.place(p)? {
            ev["place_id"] = json!(pid);
        }
    }
    if let Some(d) = ne.description.as_deref().filter(|d| !d.trim().is_empty()) {
        ev["description"] = json!(d.trim());
    }
    stamp(&mut ev, w.now);
    w.push(Op::Create {
        kind: EntityKind::Event,
        entity: ev,
    })?;
    if let Some(r) = &ne.reference {
        w.refs.insert(r.trim().to_string(), eid.clone());
    }
    let label = event_label(w, &eid);
    w.lines
        .push(Line::with("ops-line-add-event", vec![("event", label)]));
    Ok(())
}

fn update_event(w: &mut Work<'_>, ec: &EventChange) -> Result<(), Refusal> {
    let id = w.record("events", &ec.event)?;
    let mut ev = w.get("events", &id).cloned().unwrap_or_default();
    let mut changed = false;
    if let Some(c) = &ec.category {
        if !EVENT_CATEGORIES.contains(&c.trim()) {
            return Err(w.refuse("ops-refused-category", &[arg("value", c.trim())]));
        }
        ev["category"] = json!(c.trim());
        changed = true;
    }
    if let Some(s) = &ec.subcategory {
        if s.trim().is_empty() {
            ev.as_object_mut().map(|o| o.remove("subcategory"));
        } else {
            ev["subcategory"] = json!(s.trim());
        }
        changed = true;
    }
    let circa = ec.circa.unwrap_or_else(|| {
        ev.pointer("/date/circa")
            .and_then(Value::as_bool)
            .unwrap_or(false)
    });
    match &ec.date {
        Some(d) if d.trim().is_empty() => {
            ev.as_object_mut().map(|o| o.remove("date"));
            changed = true;
        }
        Some(d) => {
            ev["date"] = w.date(d, circa)?.unwrap_or(Value::Null);
            changed = true;
        }
        None if ec.circa.is_some() => {
            if let Some(v) = ev.pointer("/date/value").and_then(Value::as_str) {
                let v = v.to_string();
                ev["date"] = w.date(&v, circa)?.unwrap_or(Value::Null);
                changed = true;
            }
        }
        None => {}
    }
    match &ec.place {
        Some(p) if p.trim().is_empty() => {
            ev.as_object_mut().map(|o| o.remove("place_id"));
            changed = true;
        }
        Some(p) => {
            if let Some(pid) = w.place(p)? {
                ev["place_id"] = json!(pid);
                changed = true;
            }
        }
        None => {}
    }
    if let Some(d) = &ec.description {
        if d.trim().is_empty() {
            ev.as_object_mut().map(|o| o.remove("description"));
        } else {
            ev["description"] = json!(d.trim());
        }
        changed = true;
    }
    if !changed {
        return Err(w.refuse("ops-refused-nothing", &[]));
    }
    w.update(EntityKind::Event, &id, ev)?;
    let label = event_label(w, &id);
    w.lines
        .push(Line::with("ops-line-update-event", vec![("event", label)]));
    Ok(())
}

fn add_union(w: &mut Work<'_>, nu: &NewUnion) -> Result<(), Refusal> {
    if nu.partners.is_empty() || nu.partners.len() > 2 {
        return Err(w.refuse("ops-refused-partners", &[]));
    }
    let mut partners: Vec<String> = Vec::new();
    for p in &nu.partners {
        let id = w.person(p)?;
        if !partners.contains(&id) {
            partners.push(id);
        }
    }
    let kind = nu.kind.as_deref().unwrap_or("unknown").trim();
    if !person_new::UNION_TYPES.contains(&kind) {
        return Err(w.refuse("ops-refused-union-kind", &[arg("value", kind)]));
    }
    let mut kids: Vec<String> = Vec::new();
    for c in &nu.children {
        let id = w.person(c)?;
        if !kids.contains(&id) {
            kids.push(id);
        }
    }
    let set: BTreeSet<String> = partners.iter().cloned().collect();
    let fid = if let Some(fid) = union_of(&w.flat, &set) {
        // The union is there: its kind and children are brought up to date.
        let mut fam = w.get("families", &fid).cloned().unwrap_or_default();
        if nu.kind.is_some() {
            fam["union"]["type"] = json!(kind);
        }
        let mut children = fam
            .get("children")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        for k in &kids {
            if !children
                .iter()
                .any(|c| c.get("person_id").and_then(Value::as_str) == Some(k.as_str()))
            {
                children.push(json!({"person_id": k}));
            }
        }
        fam["children"] = Value::Array(children);
        w.update(EntityKind::Family, &fid, fam)?;
        fid
    } else {
        let fid = uuid::Uuid::new_v4().to_string();
        let persons: Vec<Value> = partners
            .iter()
            .map(|p| json!({"person_id": p, "role": "spouse"}))
            .collect();
        let mut fam = json!({"id": fid, "type": "family", "axgf_version": "1.0",
            "union": {"type": kind, "persons": persons}});
        if !kids.is_empty() {
            fam["children"] = json!(kids
                .iter()
                .map(|k| json!({"person_id": k}))
                .collect::<Vec<_>>());
        }
        stamp(&mut fam, w.now);
        w.push(Op::Create {
            kind: EntityKind::Family,
            entity: fam,
        })?;
        fid
    };
    let has_date = nu.date.as_deref().is_some_and(|d| !d.trim().is_empty());
    let has_place = nu.place.as_deref().is_some_and(|p| !p.trim().is_empty());
    if has_date || has_place {
        let participants: Vec<Value> = partners
            .iter()
            .map(|p| json!({"entity_type": "person", "entity_id": p, "role": "spouse"}))
            .collect();
        let eid = uuid::Uuid::new_v4().to_string();
        let mut ev = json!({"id": eid, "type": "event", "axgf_version": "1.0",
            "category": "marriage", "participants": participants});
        if let Some(d) = nu.date.as_deref() {
            if let Some(date) = w.date(d, nu.circa.unwrap_or(false))? {
                ev["date"] = date;
            }
        }
        if let Some(p) = nu.place.as_deref() {
            if let Some(pid) = w.place(p)? {
                ev["place_id"] = json!(pid);
            }
        }
        stamp(&mut ev, w.now);
        w.push(Op::Create {
            kind: EntityKind::Event,
            entity: ev,
        })?;
    }
    if let Some(r) = &nu.reference {
        w.refs.insert(r.trim().to_string(), fid.clone());
    }
    let names: Vec<String> = partners.iter().map(|p| w.name(p)).collect();
    w.lines.push(Line::with(
        "ops-line-add-union",
        vec![
            ("partners", Piece::Text(names.join(" & "))),
            (
                "kind",
                Piece::Vocab {
                    family: "union-type",
                    value: kind.to_string(),
                    then: String::new(),
                },
            ),
        ],
    ));
    for k in &kids {
        w.lines.push(Line::new(
            "ops-line-add-child",
            &[arg("child", w.name(k)), arg("parents", names.join(" & "))],
        ));
    }
    Ok(())
}

fn add_child(
    w: &mut Work<'_>,
    child: &str,
    parents: &[String],
    family: Option<&str>,
) -> Result<(), Refusal> {
    let kid = w.person(child)?;
    let fid = match family {
        Some(f) => w.record("families", f)?,
        None => {
            if parents.is_empty() || parents.len() > 2 {
                return Err(w.refuse("ops-refused-parents", &[]));
            }
            let mut ids = BTreeSet::new();
            for p in parents {
                let id = w.person(p)?;
                if id == kid {
                    return Err(w.refuse("ops-refused-own-parent", &[]));
                }
                ids.insert(id);
            }
            match union_of(&w.flat, &ids) {
                Some(fid) => fid,
                None => {
                    let fid = uuid::Uuid::new_v4().to_string();
                    let persons: Vec<Value> = ids
                        .iter()
                        .map(|p| json!({"person_id": p, "role": "spouse"}))
                        .collect();
                    let mut fam = json!({"id": fid, "type": "family", "axgf_version": "1.0",
                        "union": {"type": "unknown", "persons": persons}});
                    stamp(&mut fam, w.now);
                    w.push(Op::Create {
                        kind: EntityKind::Family,
                        entity: fam,
                    })?;
                    fid
                }
            }
        }
    };
    let mut fam = w.get("families", &fid).cloned().unwrap_or_default();
    if partners_of(&fam).contains(&kid) {
        return Err(w.refuse("ops-refused-own-parent", &[]));
    }
    if children_of(&fam).contains(&kid) {
        return Err(w.refuse("ops-refused-already-child", &[arg("child", w.name(&kid))]));
    }
    let mut children = fam
        .get("children")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    children.push(json!({"person_id": kid}));
    fam["children"] = Value::Array(children);
    let parents = parents_label(w, &fam);
    w.update(EntityKind::Family, &fid, fam)?;
    w.lines.push(Line::new(
        "ops-line-add-child",
        &[arg("child", w.name(&kid)), arg("parents", parents)],
    ));
    Ok(())
}

fn delete_person(w: &mut Work<'_>, person: &str) -> Result<(), Refusal> {
    if !w.viewer.is_admin() {
        return Err(w.refuse("ops-refused-admin", &[]));
    }
    let id = w.person(person)?;
    let name = w.name(&id);
    // Out of every family: a family left with nobody goes too.
    let families: Vec<(String, Value)> = w
        .flat
        .get("families")
        .and_then(Value::as_object)
        .map(|fs| fs.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
        .unwrap_or_default();
    for (fid, mut fam) in families {
        let mut touched = false;
        if let Some(ps) = fam
            .pointer_mut("/union/persons")
            .and_then(Value::as_array_mut)
        {
            let before = ps.len();
            ps.retain(|p| p.get("person_id").and_then(Value::as_str) != Some(id.as_str()));
            touched |= ps.len() != before;
        }
        if let Some(us) = fam.get_mut("unions").and_then(Value::as_array_mut) {
            for u in us.iter_mut() {
                if let Some(ps) = u.get_mut("persons").and_then(Value::as_array_mut) {
                    let before = ps.len();
                    ps.retain(|p| p.get("person_id").and_then(Value::as_str) != Some(id.as_str()));
                    touched |= ps.len() != before;
                }
            }
        }
        if let Some(cs) = fam.get_mut("children").and_then(Value::as_array_mut) {
            let before = cs.len();
            cs.retain(|c| c.get("person_id").and_then(Value::as_str) != Some(id.as_str()));
            touched |= cs.len() != before;
        }
        if !touched {
            continue;
        }
        if partners_of(&fam).is_empty() && children_of(&fam).is_empty() {
            w.delete(EntityKind::Family, &fid)?;
        } else {
            w.update(EntityKind::Family, &fid, fam)?;
        }
    }
    // Out of every event: an event left with nobody goes too.
    let events: Vec<(String, Value)> = w
        .flat
        .get("events")
        .and_then(Value::as_object)
        .map(|es| es.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
        .unwrap_or_default();
    for (eid, mut ev) in events {
        let Some(ps) = ev.get_mut("participants").and_then(Value::as_array_mut) else {
            continue;
        };
        let before = ps.len();
        ps.retain(|p| p.get("entity_id").and_then(Value::as_str) != Some(id.as_str()));
        if ps.len() == before {
            continue;
        }
        if ps.is_empty() {
            w.delete(EntityKind::Event, &eid)?;
        } else {
            w.update(EntityKind::Event, &eid, ev)?;
        }
    }
    w.delete(EntityKind::Person, &id)?;
    w.lines
        .push(Line::new("ops-line-delete-person", &[arg("name", name)]));
    Ok(())
}

/// The writes folded one per record, in the order they were first made: a
/// record created then changed is created as it ends; changed twice, changed
/// once from the version the archive holds; created then deleted, never
/// written.
fn fold(ops: Vec<Op>, original: &Value) -> Vec<Op> {
    let version = |kind: EntityKind, id: &str| -> u64 {
        original
            .get(crate::admin::collection_for(kind))
            .and_then(|c| c.get(id))
            .map(crate::state::version_of)
            .unwrap_or(0)
    };
    let mut out: Vec<Option<Op>> = Vec::new();
    let mut at: BTreeMap<(String, String), usize> = BTreeMap::new();
    let key = |kind: EntityKind, id: &str| {
        (
            crate::admin::collection_for(kind).to_string(),
            id.to_string(),
        )
    };
    for op in ops {
        match op {
            Op::Create { kind, entity } => {
                let id = entity
                    .get("id")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_string();
                at.insert(key(kind, &id), out.len());
                out.push(Some(Op::Create { kind, entity }));
            }
            Op::Update {
                kind, id, entity, ..
            } => match at.get(&key(kind, &id)).copied() {
                Some(i) => {
                    out[i] = match out[i].take() {
                        Some(Op::Create { kind, .. }) => Some(Op::Create { kind, entity }),
                        _ => Some(Op::Update {
                            kind,
                            expected_version: version(kind, &id),
                            id: id.clone(),
                            entity,
                        }),
                    };
                }
                None => {
                    at.insert(key(kind, &id), out.len());
                    out.push(Some(Op::Update {
                        kind,
                        expected_version: version(kind, &id),
                        id,
                        entity,
                    }));
                }
            },
            Op::Delete { kind, id, .. } => match at.get(&key(kind, &id)).copied() {
                Some(i) => {
                    let created = matches!(out[i], Some(Op::Create { .. }));
                    // A record created then deleted is never written; a
                    // record changed then deleted is deleted after what came
                    // before it is written, where the delete was asked.
                    out[i] = None;
                    if !created {
                        at.remove(&key(kind, &id));
                        out.push(Some(Op::Delete {
                            kind,
                            expected_version: version(kind, &id),
                            id,
                        }));
                    }
                }
                None => out.push(Some(Op::Delete {
                    kind,
                    expected_version: version(kind, &id),
                    id,
                })),
            },
            other => out.push(Some(other)),
        }
    }
    out.into_iter().flatten().collect()
}

/// Refuse a write that names a record neither the archive nor the batch
/// holds, the rule every page keeps (`refuse_new_dangling`); a reference the
/// stored record already carried is left alone.
fn dangling(ops: &[Op], original: &Value, after: &Value) -> Result<(), Refusal> {
    use crate::routes::admin::reference_ids;
    for op in ops {
        let (kind, entity, id) = match op {
            Op::Create { kind, entity } => (*kind, entity, None),
            Op::Update {
                kind, id, entity, ..
            } => (*kind, entity, Some(id)),
            _ => continue,
        };
        let mut had = Vec::new();
        if let Some(stored) = id.and_then(|id| {
            original
                .get(crate::admin::collection_for(kind))
                .and_then(|c| c.get(id))
        }) {
            reference_ids(stored, "", None, &mut had);
        }
        let mut refs = Vec::new();
        reference_ids(entity, "", None, &mut refs);
        let own = entity.get("id").and_then(Value::as_str);
        let missing = refs.into_iter().find(|(_, t)| {
            Some(t.as_str()) != own
                && !had.iter().any(|(_, h)| h == t)
                && !after
                    .as_object()
                    .is_some_and(|c| c.values().any(|m| m.get(t).is_some()))
        });
        if let Some((field, target)) = missing {
            return Err(Refusal::of(
                "ops-refused-dangling",
                &[arg("field", field), arg("target", target)],
            ));
        }
    }
    Ok(())
}

/// Check a batch without writing: what it would do, or why it cannot.
pub fn check(
    state: &AppState,
    viewer: &Viewer,
    batch: &[Operation],
    quote: &dyn Fn(&str) -> String,
) -> Result<Vec<Line>, Refusal> {
    let now = crate::view::now_iso8601();
    state.read(|flat| compile(flat, viewer, batch, &now, quote).map(|c| c.lines))
}

/// Apply a batch as `viewer`, `via` the AFG or a token, all or nothing.
pub fn apply(
    state: &AppState,
    viewer: &Viewer,
    via: &str,
    batch: &[Operation],
    quote: &dyn Fn(&str) -> String,
) -> Result<Done, Refusal> {
    let now = crate::view::now_iso8601();
    let compiled = state.read(|flat| compile(flat, viewer, batch, &now, quote))?;
    if compiled.ops.is_empty() {
        return Ok(Done {
            records: 0,
            lines: compiled.lines,
            refs: compiled.refs,
        });
    }
    let who = format!("{} · {via}", viewer.name());
    let outcome = state
        .commit(compiled.ops, &who, crate::routes::admin::label_for)
        .map_err(|e| Refusal::of("ops-refused-io", &[arg("detail", e.to_string())]))?;
    match outcome {
        BatchOutcome::Applied { records } => Ok(Done {
            records,
            lines: compiled.lines,
            refs: compiled.refs,
        }),
        BatchOutcome::Conflict { .. } => Err(Refusal::of("ops-refused-conflict", &[])),
        BatchOutcome::Refused { diagnostics, .. } => Err(Refusal::of(
            "ops-refused-library",
            &[arg(
                "detail",
                crate::state::format_diagnostics(&diagnostics),
            )],
        )),
    }
}

/// The years of a person's life, as the river writes them.
fn years(flat: &Value, lens: &access::Lens, id: &str) -> String {
    let l = access::river_label(flat, lens, id);
    match (l.birth_year, l.death_year) {
        (Some(b), Some(d)) => format!("{b}–{d}"),
        (Some(b), None) => format!("{b}–"),
        (None, Some(d)) => format!("–{d}"),
        (None, None) => String::new(),
    }
}

/// A person in a word, as the reader may see them: id, name and years, or
/// nothing at all.
fn brief(flat: &Value, lens: &access::Lens, id: &str) -> Option<Value> {
    if !lens.sees_person(id) {
        return None;
    }
    let p = flat.get("persons")?.get(id)?;
    Some(
        json!({"id": id, "name": crate::view::person_display_name(p), "years": years(flat, lens, id)}),
    )
}

fn fact_json(flat: &Value, fact: Option<&Value>) -> Value {
    let Some(f) = fact else {
        return Value::Null;
    };
    let place = f
        .get("place_id")
        .and_then(Value::as_str)
        .and_then(|p| flat.get("places").and_then(|c| c.get(p)))
        .map(crate::forms::place_label);
    json!({
        "date": f.pointer("/date/value"),
        "circa": f.pointer("/date/circa").and_then(Value::as_bool).unwrap_or(false),
        "place_id": f.get("place_id"),
        "place": place,
    })
}

/// A read operation's answer, through the reader's lens.
pub fn read(flat: &Value, viewer: &Viewer, op: &Operation) -> Result<Value, Refusal> {
    let lens = viewer.lens(flat);
    let none = |what: &str| Refusal::of("ops-refused-not-found", &[arg("what", what.trim())]);
    match op {
        Operation::Search { query, limit } => {
            let words: Vec<String> = folded(query)
                .split(' ')
                .filter(|w| !w.is_empty())
                .map(str::to_string)
                .collect();
            let most = limit.unwrap_or(20).clamp(1, 100);
            let mut found: Vec<(String, Value)> = flat
                .get("persons")
                .and_then(Value::as_object)
                .map(|ps| {
                    ps.iter()
                        .filter(|(id, _)| lens.sees_person(id))
                        .filter_map(|(id, p)| {
                            let name = crate::view::person_display_name(p);
                            let hay = folded(&name);
                            (words.is_empty() || words.iter().all(|w| hay.contains(w.as_str())))
                                .then(|| (hay, brief(flat, &lens, id).unwrap_or(Value::Null)))
                        })
                        .collect()
                })
                .unwrap_or_default();
            found.sort_by(|a, b| a.0.cmp(&b.0));
            let total = found.len();
            let people: Vec<Value> = found.into_iter().take(most).map(|(_, v)| v).collect();
            Ok(json!({"total": total, "people": people}))
        }
        Operation::GetPerson { person } => {
            let id = match person_new::who(person, flat) {
                Ok(Some(Who::Existing(id))) if lens.sees_person(&id) => id,
                _ => return Err(none(person)),
            };
            let p = &flat["persons"][&id];
            let comps = p
                .pointer("/identity/name/components")
                .cloned()
                .unwrap_or(Value::Null);
            let mut parents = Vec::new();
            let mut siblings = Vec::new();
            let mut unions = Vec::new();
            if let Some(fs) = flat.get("families").and_then(Value::as_object) {
                for (fid, f) in fs {
                    let ps = partners_of(f);
                    let ks = children_of(f);
                    if ks.contains(&id) {
                        parents.extend(ps.iter().filter_map(|q| brief(flat, &lens, q)));
                        siblings.extend(
                            ks.iter()
                                .filter(|k| **k != id)
                                .filter_map(|k| brief(flat, &lens, k)),
                        );
                    }
                    if ps.contains(&id) {
                        unions.push(json!({
                            "family": fid,
                            "kind": f.pointer("/union/type"),
                            "partners": ps.iter().filter(|q| **q != id).filter_map(|q| brief(flat, &lens, q)).collect::<Vec<_>>(),
                            "children": ks.iter().filter_map(|k| brief(flat, &lens, k)).collect::<Vec<_>>(),
                        }));
                    }
                }
            }
            let events: Vec<Value> = flat
                .get("events")
                .and_then(Value::as_object)
                .map(|es| {
                    es.iter()
                        .filter_map(|(eid, e)| {
                            let role = e
                                .get("participants")?
                                .as_array()?
                                .iter()
                                .find(|q| {
                                    q.get("entity_id").and_then(Value::as_str) == Some(id.as_str())
                                })?
                                .get("role")
                                .cloned();
                            Some(json!({
                                "id": eid,
                                "category": e.get("category"),
                                "subcategory": e.get("subcategory"),
                                "role": role,
                                "date": e.pointer("/date/value"),
                                "place": e.get("place_id").and_then(Value::as_str)
                                    .and_then(|pl| flat.get("places").and_then(|c| c.get(pl)))
                                    .map(crate::forms::place_label),
                                "description": e.get("description"),
                            }))
                        })
                        .collect()
                })
                .unwrap_or_default();
            Ok(json!({
                "id": id,
                "name": crate::view::person_display_name(p),
                "components": comps,
                "gender": p.pointer("/identity/gender/value"),
                "living": p.pointer("/identity/is_living"),
                "visibility": p.pointer("/identity/visibility"),
                "birth": fact_json(flat, p.get("birth")),
                "death": fact_json(flat, p.get("death")),
                "bio": p.get("bio"),
                "parents": parents,
                "siblings": siblings,
                "unions": unions,
                "events": events,
            }))
        }
        Operation::GetFamily { family } => {
            let f = flat
                .get("families")
                .and_then(|c| c.get(family.trim()))
                .ok_or_else(|| none(family))?;
            let partners = partners_of(f);
            if !partners.is_empty() && !partners.iter().any(|p| lens.sees_person(p)) {
                return Err(none(family));
            }
            Ok(json!({
                "id": family.trim(),
                "kind": f.pointer("/union/type"),
                "partners": partners.iter().filter_map(|p| brief(flat, &lens, p)).collect::<Vec<_>>(),
                "children": children_of(f).iter().filter_map(|k| brief(flat, &lens, k)).collect::<Vec<_>>(),
            }))
        }
        _ => Err(Refusal::of(
            "ops-refused-invalid",
            &[arg("detail", op.name())],
        )),
    }
}

/// The operations, as the AFG is told them and the MCP describes them: what
/// each takes, and the rules every one of them keeps.
pub const CATALOGUE: &str = r#"Operations on the family archive (AXGF). Each is a JSON object with "op" and its fields.
Name a person by id (best), or by a name exactly one person bears. A batch may name what it creates with "ref": "new:1", and later operations use "new:1" in its place. Dates: "1890", "1890-06" or "1890-06-14"; "circa": true when approximate. Places: an id, or a name (a name nobody has makes a new place). An empty string removes a field.

Reads:
- {"op":"search","query":"Dupont Jean","limit":20}
- {"op":"get_person","person":"<id>"}
- {"op":"get_family","family":"<id>"}

Writes:
- {"op":"create_person","ref":"new:1","given":"Jean","family":"Dupont","nicknames":[],"gender":"M|F|NB|U","living":false,
   "birth":{"date":"1890-06-14","circa":false,"place":"Lyon"},"death":{"date":"1960","place":"Paris"},"bio":"Markdown",
   "father":"<id or name>","mother":"<id or name>","siblings":["<id>"],
   "unions":[{"partner":"<id or name>","kind":"marriage","children":["<id or name>"]}]}
   (a father, mother, sibling, partner or child named by a name nobody has is created too)
- {"op":"update_person","person":"<id>","given":"…","family":"…","nicknames":["…"],"gender":"F","living":false,"visibility":"public|members|contributors|private","bio":"…"}
- {"op":"set_birth","person":"<id>","date":"1890","circa":true,"place":"Lyon"}   (or "clear": true)
- {"op":"set_death","person":"<id>","date":"1960-03-02","place":"Paris"}   (marks the person not living)
- {"op":"add_event","person":"<id>","category":"religious","subcategory":"baptism","date":"1890-06-20","place":"Lyon","role":"subject","description":"…","with":[{"person":"<id>","role":"godparent"}]}
   categories: religious (subcategory baptism, christening, confirmation, first communion, bar mitzvah…), marriage, divorce, military, migration (emigration, immigration), naturalization, census, social (graduation, retirement), legal (will, probate), adoption, incarceration, name_change, historical, other (burial, cremation, residence), birth, death
   roles: subject, participant, spouse, spouse_1, spouse_2, witness, officiant, informant, godparent
- {"op":"update_event","event":"<id>","date":"1891","place":"Lyon","category":"…","subcategory":"…","description":"…"}
- {"op":"delete_event","event":"<id>"}
- {"op":"add_union","ref":"new:2","partners":["<id>","<id>"],"kind":"marriage|civil_union|cohabitation|religious_only|polygamous|unknown","date":"1920-05-01","place":"Lyon","children":["<id>"]}
- {"op":"add_child","child":"<id>","parents":["<id>","<id>"]}   (or "family":"<family id>")
- {"op":"remove_child","child":"<id>","family":"<family id>"}
- {"op":"delete_union","family":"<family id>"}   (the union goes; its children stay, without these parents; its marriage event is deleted with delete_event)
- {"op":"delete_person","person":"<id>"}   (administrators only: the person leaves every family and event, then goes)

Rules: never invent an id; use the ids the archive holds. Everything a batch does is applied together, or nothing is."#;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::acl::{Role, User};

    fn viewer(role: Role) -> Viewer {
        let user: User = serde_json::from_value(json!({
            "id": "u1", "username": "karin", "password_hash": "", "role": role,
            "status": "active", "created_at": "2026-11-09T00:00:00Z"
        }))
        .unwrap();
        Viewer {
            user: Some(user),
            emergency: false,
        }
    }

    fn archive() -> Value {
        json!({
            "persons": {
                "a0000000-0000-4000-8000-000000000001": {"id": "a0000000-0000-4000-8000-000000000001", "type": "person", "version_num": 3,
                    "identity": {"name": {"display": "Pierre Dupont", "components": [
                        {"type": "given_name", "value": "Pierre", "order": 1},
                        {"type": "family_name", "value": "Dupont", "order": 2}]},
                        "gender": {"value": "M"}, "is_living": false, "visibility": "public"}},
                "a0000000-0000-4000-8000-000000000002": {"id": "a0000000-0000-4000-8000-000000000002", "type": "person", "version_num": 1,
                    "identity": {"name": {"display": "Marie Durand", "components": []},
                        "gender": {"value": "F"}, "is_living": false, "visibility": "public"}}
            },
            "families": {}, "events": {}, "places": {}
        })
    }

    fn quote(n: &str) -> String {
        format!("« {n} »")
    }

    #[test]
    fn a_batch_reads_back_from_json() {
        let b = parse(
            r#"[{"op":"set_birth","person":"x","date":"1890"},{"op":"delete_event","event":"e"}]"#,
        )
        .unwrap();
        assert_eq!(b.len(), 2);
        assert_eq!(b[0].name(), "set_birth");
        assert!(parse(r#"{"operations":[{"op":"search","query":"x"}]}"#).unwrap()[0].reads());
        assert!(parse(r#"{"op":"fly"}"#).is_err());
    }

    #[test]
    fn a_new_person_and_their_parents_union_is_one_folded_batch() {
        let b = parse(r#"[
            {"op":"create_person","ref":"new:1","given":"Jean","family":"Dupont","gender":"M","birth":{"date":"1921","place":"Lyon"}},
            {"op":"add_union","partners":["Pierre Dupont","a0000000-0000-4000-8000-000000000002"],"kind":"marriage","date":"1920-05-01","children":["new:1"]},
            {"op":"set_death","person":"new:1","date":"1990"}
        ]"#).unwrap();
        let c = compile(
            &archive(),
            &viewer(Role::Contributor),
            &b,
            "2026-11-09T00:00:00Z",
            &quote,
        )
        .unwrap();
        // The person, the place, the family, the marriage: four records, the
        // person written once, dead in 1990.
        let creates: Vec<&Op> = c
            .ops
            .iter()
            .filter(|o| matches!(o, Op::Create { .. }))
            .collect();
        assert_eq!(creates.len(), 4, "{:#?}", c.ops);
        let person = c
            .ops
            .iter()
            .find_map(|o| match o {
                Op::Create {
                    kind: EntityKind::Person,
                    entity,
                } => Some(entity),
                _ => None,
            })
            .unwrap();
        assert_eq!(person["death"]["date"]["value"], "1990");
        assert_eq!(person["identity"]["is_living"], false);
        assert_eq!(
            c.refs.get("new:1").map(String::as_str),
            person["id"].as_str()
        );
        assert!(c.lines.iter().any(|l| l.key == "ops-line-add-child"));
    }

    #[test]
    fn a_new_child_says_whose_child_it_is() {
        let b = parse(r#"[{"op":"create_person","given":"Lucien","family":"Dupont","father":"Pierre Dupont","living":false}]"#).unwrap();
        let c = compile(
            &archive(),
            &viewer(Role::Contributor),
            &b,
            "2026-11-09T00:00:00Z",
            &quote,
        )
        .unwrap();
        let said: Vec<String> = c.lines.iter().map(|l| l.text("en")).collect();
        assert!(
            said.contains(&"Lucien Dupont, child of Pierre Dupont".to_string()),
            "{said:?}"
        );
    }

    #[test]
    fn a_reader_cannot_write_and_a_contributor_cannot_delete_a_person() {
        let b = parse(r#"[{"op":"set_birth","person":"Pierre Dupont","date":"1890"}]"#).unwrap();
        let r = compile(
            &archive(),
            &viewer(Role::Viewer),
            &b,
            "2026-11-09T00:00:00Z",
            &quote,
        )
        .unwrap_err();
        assert_eq!(r.key, "ops-refused-role");
        let b = parse(r#"[{"op":"delete_person","person":"Pierre Dupont"}]"#).unwrap();
        let r = compile(
            &archive(),
            &viewer(Role::Contributor),
            &b,
            "2026-11-09T00:00:00Z",
            &quote,
        )
        .unwrap_err();
        assert_eq!(r.key, "ops-refused-admin");
        let c = compile(
            &archive(),
            &viewer(Role::Admin),
            &b,
            "2026-11-09T00:00:00Z",
            &quote,
        )
        .unwrap();
        assert!(matches!(
            c.ops.last(),
            Some(Op::Delete {
                kind: EntityKind::Person,
                expected_version: 3,
                ..
            })
        ));
    }

    #[test]
    fn an_update_keeps_the_version_the_archive_holds_and_a_bad_date_is_refused() {
        let b = parse(r#"[{"op":"update_person","person":"a0000000-0000-4000-8000-000000000001","given":"Pierre Louis","visibility":"members"},
                          {"op":"set_birth","person":"a0000000-0000-4000-8000-000000000001","date":"1890-06","circa":true}]"#).unwrap();
        let c = compile(
            &archive(),
            &viewer(Role::Contributor),
            &b,
            "2026-11-09T00:00:00Z",
            &quote,
        )
        .unwrap();
        assert_eq!(c.ops.len(), 1, "two changes, one write");
        match &c.ops[0] {
            Op::Update {
                expected_version,
                entity,
                ..
            } => {
                assert_eq!(*expected_version, 3);
                assert_eq!(entity["identity"]["name"]["display"], "Pierre Louis Dupont");
                assert_eq!(entity["identity"]["visibility"], "members");
                assert_eq!(entity["birth"]["date"]["circa"], true);
            }
            other => panic!("{other:?}"),
        }
        let b =
            parse(r#"[{"op":"set_birth","person":"Pierre Dupont","date":"the spring"}]"#).unwrap();
        let r = compile(
            &archive(),
            &viewer(Role::Contributor),
            &b,
            "2026-11-09T00:00:00Z",
            &quote,
        )
        .unwrap_err();
        assert_eq!((r.index, r.key), (1, "ops-refused-date"));
    }

    #[test]
    fn a_name_two_people_bear_is_refused_and_nobody_is_not_found() {
        let mut a = archive();
        a["persons"]["a0000000-0000-4000-8000-000000000003"] = json!({"id": "a0000000-0000-4000-8000-000000000003", "type": "person",
            "identity": {"name": {"display": "Pierre Dupont"}, "visibility": "public", "is_living": false}});
        let b = parse(r#"[{"op":"set_birth","person":"Pierre Dupont","date":"1890"}]"#).unwrap();
        assert_eq!(
            compile(&a, &viewer(Role::Admin), &b, "2026-11-09T00:00:00Z", &quote)
                .unwrap_err()
                .key,
            "ops-refused-ambiguous"
        );
        let b = parse(r#"[{"op":"set_birth","person":"Nobody Here","date":"1890"}]"#).unwrap();
        assert_eq!(
            compile(&a, &viewer(Role::Admin), &b, "2026-11-09T00:00:00Z", &quote)
                .unwrap_err()
                .key,
            "ops-refused-not-found"
        );
    }

    #[test]
    fn an_answer_is_cut_into_its_words_and_its_blocks() {
        let text = "Je propose ceci :\n\n```axgf-ops\n[{\"op\":\"delete_event\",\"event\":\"e1\"}]\n```\n\nEt ensuite ?\n````json\n{}\n````\n";
        let b = blocks(text);
        assert_eq!(b.len(), 3, "{b:?}");
        assert_eq!(b[0], Segment::Text("Je propose ceci :\n\n"));
        assert_eq!(
            b[1],
            Segment::Ops("[{\"op\":\"delete_event\",\"event\":\"e1\"}]\n")
        );
        assert!(matches!(b[2], Segment::Text(t) if t.contains("````json")));
        assert_eq!(
            ops_blocks("~~~ AXGF-OPS\n[]").len(),
            1,
            "a block left open runs to the end"
        );
        assert!(ops_blocks("```json\n[]\n```").is_empty());
    }

    #[test]
    fn a_person_the_writer_may_not_see_is_nobody_to_them() {
        let mut a = archive();
        let hidden = "a0000000-0000-4000-8000-000000000002";
        a["persons"][hidden]["identity"]["visibility"] = json!("private");
        a["families"]["f0000000-0000-4000-8000-000000000001"] = json!({
            "id": "f0000000-0000-4000-8000-000000000001", "type": "family", "version_num": 1,
            "union": {"type": "unknown", "persons": [{"person_id": hidden, "role": "spouse"}]}});
        for op in [
            format!(r#"{{"op":"set_birth","person":"{hidden}","date":"1890"}}"#),
            r#"{"op":"set_birth","person":"Marie Durand","date":"1890"}"#.to_string(),
            r#"{"op":"delete_union","family":"f0000000-0000-4000-8000-000000000001"}"#.to_string(),
        ] {
            let b = parse(&op).unwrap();
            let r = compile(
                &a,
                &viewer(Role::Contributor),
                &b,
                "2026-11-09T00:00:00Z",
                &quote,
            )
            .unwrap_err();
            assert_eq!(r.key, "ops-refused-not-found", "{op}");
            assert!(
                compile(&a, &viewer(Role::Admin), &b, "2026-11-09T00:00:00Z", &quote).is_ok(),
                "an administrator sees everyone: {op}"
            );
        }
        // Named as a relative, the hidden name is somebody new, never her.
        let b = parse(r#"[{"op":"create_person","given":"Paul","family":"Durand","mother":"Marie Durand","living":false}]"#).unwrap();
        let c = compile(
            &a,
            &viewer(Role::Contributor),
            &b,
            "2026-11-09T00:00:00Z",
            &quote,
        )
        .unwrap();
        let mothers: Vec<&Value> = c
            .ops
            .iter()
            .filter_map(|o| match o {
                Op::Create {
                    kind: EntityKind::Person,
                    entity,
                } => Some(entity),
                _ => None,
            })
            .collect();
        assert_eq!(mothers.len(), 2, "Paul, and a new Marie Durand");
        assert!(mothers.iter().all(|p| p["id"] != hidden));
    }

    #[test]
    fn reads_go_through_the_lens() {
        let mut a = archive();
        a["persons"]["a0000000-0000-4000-8000-000000000002"]["identity"]["visibility"] =
            json!("private");
        let found = read(
            &a,
            &Viewer::anonymous(),
            &Operation::Search {
                query: "".into(),
                limit: None,
            },
        )
        .unwrap();
        assert_eq!(
            found["total"], 1,
            "the private person is nobody to a visitor"
        );
        assert!(read(
            &a,
            &Viewer::anonymous(),
            &Operation::GetPerson {
                person: "Marie Durand".into()
            }
        )
        .is_err());
        let p = read(
            &a,
            &viewer(Role::Admin),
            &Operation::GetPerson {
                person: "Marie Durand".into(),
            },
        )
        .unwrap();
        assert_eq!(p["name"], "Marie Durand");
    }
}
