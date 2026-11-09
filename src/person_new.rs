//! A person created with what surrounds them (docs/adr/0039).
//!
//! The page `/admin/person/new` takes a name made of its parts, a gender,
//! a birth and a death with their places, a biography, and the people
//! around: a father, a mother, brothers and sisters, and unions, each with
//! a partner and children. Every person and every place it is given is
//! either one the archive already holds or one to create: what the writer
//! picked from a list, or typed exactly as the archive spells it, is reused;
//! anything else is created with the new person, in the same save.
//!
//! This module reads what the page sent and plans the writes, as
//! [`Op`]s for one [`crate::state::AppState::commit`]: new places first,
//! then the new people, then the person, then the families that tie them,
//! so that every record names only records that exist before it. Nothing
//! here touches the archive; the route commits the plan.

use std::collections::{BTreeMap, BTreeSet};

use axgf_rs::EntityKind;
use serde_json::{json, Map, Value};

use crate::forms::PickError;
use crate::state::Op;

/// The unions a union block may be.
pub const UNION_TYPES: &[&str] = &[
    "marriage",
    "civil_union",
    "cohabitation",
    "religious_only",
    "polygamous",
    "unknown",
];

/// Where a person says its biography was written with the notes' editor, and
/// is read as Markdown (docs/adr/0039). A biography that came from a GEDCOM
/// file is plain text — "* 1890" is a birth there, not a list — and stays so.
pub const BIO_EXT_KEY: &str = "axgf-cms:bio/v1";

/// Mark `person`'s biography as written in Markdown.
pub fn mark_bio_markdown(person: &mut Value) {
    if !person.get("extensions").is_some_and(Value::is_object) {
        person["extensions"] = json!({});
    }
    person["extensions"][BIO_EXT_KEY] = json!({"format": "markdown"});
}

/// Whether `person`'s biography is read as Markdown.
pub fn bio_is_markdown(person: &Value) -> bool {
    person
        .get("extensions")
        .and_then(|e| e.get(BIO_EXT_KEY))
        .and_then(|b| b.get("format"))
        .and_then(Value::as_str)
        == Some("markdown")
}

/// What the page sent, read.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Submission {
    pub given: String,
    pub nicks: Vec<String>,
    pub family: String,
    /// `M`, `F`, `NB` or `U`.
    pub gender: String,
    pub living: bool,
    pub birth_date: String,
    pub birth_circa: bool,
    pub birth_place: String,
    pub death_date: String,
    pub death_circa: bool,
    pub death_place: String,
    /// Markdown.
    pub bio: String,
    pub father: String,
    pub mother: String,
    pub siblings: Vec<String>,
    pub unions: Vec<UnionIn>,
}

/// One union block: a partner, the union's kind, its children.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct UnionIn {
    pub partner: String,
    pub kind: String,
    pub children: Vec<String>,
}

impl Submission {
    /// Read the fields: `nick`, `sibling` and `u.N.child` repeat; `u.N.*`
    /// group a union's fields by their number.
    pub fn from_fields(fields: &[(String, String)]) -> Self {
        let mut s = Submission {
            gender: "U".into(),
            ..Default::default()
        };
        let mut unions: BTreeMap<usize, UnionIn> = BTreeMap::new();
        for (k, v) in fields {
            let v = v.trim();
            match k.as_str() {
                "given" => s.given = v.into(),
                "family" => s.family = v.into(),
                "nick" if !v.is_empty() => s.nicks.push(v.into()),
                "gender" if ["M", "F", "NB", "U"].contains(&v) => s.gender = v.into(),
                "living" => s.living = v == "on",
                "birth_date" => s.birth_date = v.into(),
                "birth_circa" => s.birth_circa = v == "on",
                "birth_place" => s.birth_place = v.into(),
                "death_date" => s.death_date = v.into(),
                "death_circa" => s.death_circa = v == "on",
                "death_place" => s.death_place = v.into(),
                "bio" => s.bio = v.replace("\r\n", "\n"),
                "father" => s.father = v.into(),
                "mother" => s.mother = v.into(),
                "sibling" if !v.is_empty() => s.siblings.push(v.into()),
                other => {
                    let Some(rest) = other.strip_prefix("u.") else {
                        continue;
                    };
                    let Some((n, field)) = rest.split_once('.') else {
                        continue;
                    };
                    let Ok(n) = n.parse::<usize>() else {
                        continue;
                    };
                    let u = unions.entry(n).or_default();
                    match field {
                        "partner" => u.partner = v.into(),
                        "type" => u.kind = v.into(),
                        "child" if !v.is_empty() => u.children.push(v.into()),
                        _ => {}
                    }
                }
            }
        }
        s.unions = unions.into_values().collect();
        s
    }

    /// Whether the record says this person died.
    pub fn died(&self) -> bool {
        !self.death_date.is_empty() || !self.death_place.is_empty()
    }
}

/// The name as the record shows it: the given names, each nickname as
/// `quote` sets it, the family name.
pub fn display_name(s: &Submission, quote: &dyn Fn(&str) -> String) -> String {
    let mut parts: Vec<String> = Vec::new();
    if !s.given.is_empty() {
        parts.push(s.given.clone());
    }
    for n in &s.nicks {
        parts.push(quote(n));
    }
    if !s.family.is_empty() {
        parts.push(s.family.clone());
    }
    parts.join(" ")
}

/// The name's parts, in the order they were given.
pub fn components(s: &Submission) -> Vec<Value> {
    let mut out = Vec::new();
    let mut order = 1;
    let mut push = |kind: &str, value: &str| {
        out.push(json!({"type": kind, "value": value, "order": order}));
        order += 1;
    };
    if !s.given.is_empty() {
        push("given_name", &s.given);
    }
    for n in &s.nicks {
        push("nickname", n);
    }
    if !s.family.is_empty() {
        push("family_name", &s.family);
    }
    out
}

/// A name typed for somebody to create: its last word the family name, the
/// rest the given names.
pub fn split_typed(name: &str) -> (String, String) {
    let words: Vec<&str> = name.split_whitespace().collect();
    match words.len() {
        0 => (String::new(), String::new()),
        1 => (words[0].to_string(), String::new()),
        n => (words[..n - 1].join(" "), words[n - 1].to_string()),
    }
}

/// Spaces collapsed, case folded: how two spellings are compared.
fn folded(s: &str) -> String {
    s.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

/// A person the page names: one the archive holds, or one to create.
#[derive(Debug, Clone, PartialEq)]
pub enum Who {
    Existing(String),
    New(String),
}

/// A typed person, resolved without guessing: a label picked from the list
/// (it ends in `· #xxxxxxxx`) or an id names one person; otherwise the name
/// must be spelled as exactly one person's, spaces and case aside. A name
/// nobody has is somebody to create; a name two people share is refused.
pub fn who(input: &str, flat: &Value) -> Result<Option<Who>, PickError> {
    let input = input.trim();
    if input.is_empty() {
        return Ok(None);
    }
    let persons = flat.get("persons").and_then(Value::as_object);
    if input.contains(" · #") || persons.is_some_and(|p| p.contains_key(input)) {
        return crate::forms::resolve_person(input, flat).map(|id| Some(Who::Existing(id)));
    }
    let typed = folded(input);
    let matches: Vec<&String> = persons
        .map(|p| {
            p.iter()
                .filter(|(_, e)| folded(&crate::view::person_display_name(e)) == typed)
                .map(|(id, _)| id)
                .collect()
        })
        .unwrap_or_default();
    match matches.len() {
        0 => Ok(Some(Who::New(
            input.split_whitespace().collect::<Vec<_>>().join(" "),
        ))),
        1 => Ok(Some(Who::Existing(matches[0].clone()))),
        _ => Err(PickError::Ambiguous),
    }
}

/// A place the page names: one the archive holds, or one to create.
#[derive(Debug, Clone, PartialEq)]
pub enum Where {
    Existing(String),
    New(String),
}

/// A typed place, resolved the same way as a person: a label with its
/// `#xxxxxxxx`, an id, or exactly one place's name.
pub fn place(input: &str, flat: &Value) -> Result<Option<Where>, PickError> {
    let input = input.trim();
    if input.is_empty() {
        return Ok(None);
    }
    let places = flat.get("places").and_then(Value::as_object);
    if let Some(p) = places {
        if p.contains_key(input) {
            return Ok(Some(Where::Existing(input.to_string())));
        }
        if let Some((_, short)) = input.rsplit_once(" · #") {
            let ids: Vec<&String> = p.keys().filter(|id| id.starts_with(short.trim())).collect();
            return match ids.len() {
                1 => Ok(Some(Where::Existing(ids[0].clone()))),
                0 => Err(PickError::NotFound),
                _ => Err(PickError::Ambiguous),
            };
        }
    }
    let typed = folded(input);
    let matches: Vec<&String> = places
        .map(|p| {
            p.iter()
                .filter(|(_, e)| folded(&crate::forms::place_label(e)) == typed)
                .map(|(id, _)| id)
                .collect()
        })
        .unwrap_or_default();
    match matches.len() {
        0 => Ok(Some(Where::New(
            input.split_whitespace().collect::<Vec<_>>().join(" "),
        ))),
        1 => Ok(Some(Where::Existing(matches[0].clone()))),
        _ => Err(PickError::Ambiguous),
    }
}

/// Why a submission cannot be saved: the field, what was typed in it, and the
/// message's catalogue key.
#[derive(Debug, Clone, PartialEq)]
pub struct Refusal {
    pub field: String,
    pub typed: String,
    pub key: &'static str,
}

impl Refusal {
    fn new(field: &str, typed: &str, key: &'static str) -> Self {
        Refusal {
            field: field.to_string(),
            typed: typed.to_string(),
            key,
        }
    }
}

/// A record the plan creates, for the page that says what was done.
#[derive(Debug, Clone, PartialEq)]
pub struct Created {
    pub kind: EntityKind,
    pub id: String,
    pub label: String,
}

/// The writes, in order, and what they make.
#[derive(Debug, Clone)]
pub struct Plan {
    pub person_id: String,
    pub ops: Vec<Op>,
    pub created: Vec<Created>,
    /// Existing families the person joins, by id.
    pub joined: Vec<String>,
}

/// What the plan needs besides the submission.
pub struct Context<'a> {
    pub flat: &'a Value,
    /// How a nickname is quoted in the reader's language.
    pub quote: &'a dyn Fn(&str) -> String,
    /// ISO 8601, for `created_at` and `updated_at`.
    pub now: &'a str,
    /// The avatar's document, when one comes with the page.
    pub avatar: Option<&'a str>,
    /// Ids, for tests to name; `None` mints UUIDs.
    pub ids: Option<&'a std::cell::RefCell<Vec<String>>>,
}

impl Context<'_> {
    fn mint(&self) -> String {
        if let Some(ids) = self.ids {
            if let Some(id) = ids.borrow_mut().pop() {
                return id;
            }
        }
        uuid::Uuid::new_v4().to_string()
    }
}

/// A date the page holds: `1890`, `1890-06` or `1890-06-14`, a real one.
fn date(field: &str, value: &str, circa: bool) -> Result<Option<Value>, Refusal> {
    if value.is_empty() {
        return Ok(None);
    }
    if crate::notes::date_of(value).is_none() {
        return Err(Refusal::new(field, value, "person-new-error-date"));
    }
    Ok(crate::forms::date_object(Some(value), None, circa))
}

fn stamp(entity: &mut Value, now: &str) {
    entity["version_num"] = json!(1);
    entity["created_at"] = json!(now);
    entity["updated_at"] = json!(now);
}

/// Plan the writes for a submission.
pub fn plan(s: &Submission, cx: &Context<'_>) -> Result<Plan, Refusal> {
    let flat = cx.flat;
    let display = display_name(s, cx.quote);
    if display.trim().is_empty() {
        return Err(Refusal::new("given", "", "person-new-error-name"));
    }
    let birth_date = date("birth_date", &s.birth_date, s.birth_circa)?;
    let death_date = date("death_date", &s.death_date, s.death_circa)?;

    let mut ops: Vec<Op> = Vec::new();
    let mut created: Vec<Created> = Vec::new();
    let mut joined: Vec<String> = Vec::new();

    // Places: one created per name, however many fields name it.
    let mut new_places: BTreeMap<String, String> = BTreeMap::new();
    let mut place_id =
        |field: &str, typed: &str, ops: &mut Vec<Op>, created: &mut Vec<Created>| match place(
            typed, flat,
        ) {
            Ok(None) => Ok(None),
            Ok(Some(Where::Existing(id))) => Ok(Some(id)),
            Ok(Some(Where::New(name))) => {
                if let Some(id) = new_places.get(&folded(&name)) {
                    return Ok(Some(id.clone()));
                }
                let id = cx.mint();
                let mut e = json!({"id": id, "type": "place", "axgf_version": "1.0",
                    "names": [{"lang": "und", "value": name, "is_primary": true}]});
                stamp(&mut e, cx.now);
                ops.push(Op::Create {
                    kind: EntityKind::Place,
                    entity: e,
                });
                created.push(Created {
                    kind: EntityKind::Place,
                    id: id.clone(),
                    label: name.clone(),
                });
                new_places.insert(folded(&name), id.clone());
                Ok(Some(id))
            }
            Err(e) => Err(Refusal::new(field, typed, place_key(e))),
        };
    let birth_place = place_id("birth_place", &s.birth_place, &mut ops, &mut created)?;
    let death_place = place_id("death_place", &s.death_place, &mut ops, &mut created)?;

    // People: one created per name, however many fields name them.
    let mut new_people: BTreeMap<String, String> = BTreeMap::new();
    let mut person_id = |field: &str,
                         typed: &str,
                         gender: &str,
                         ops: &mut Vec<Op>,
                         created: &mut Vec<Created>|
     -> Result<Option<String>, Refusal> {
        match who(typed, flat) {
            Ok(None) => Ok(None),
            Ok(Some(Who::Existing(id))) => Ok(Some(id)),
            Ok(Some(Who::New(name))) => {
                if let Some(id) = new_people.get(&folded(&name)) {
                    return Ok(Some(id.clone()));
                }
                let id = cx.mint();
                let (given, family) = split_typed(&name);
                let mut comps = Vec::new();
                if !given.is_empty() {
                    comps.push(json!({"type": "given_name", "value": given, "order": 1}));
                }
                if !family.is_empty() {
                    comps.push(
                        json!({"type": "family_name", "value": family, "order": comps.len() + 1}),
                    );
                }
                let mut e = json!({"id": id, "type": "person", "axgf_version": "1.0",
                    "identity": {"name": {"display": name, "components": comps},
                                 "gender": {"value": gender}, "is_living": true}});
                stamp(&mut e, cx.now);
                ops.push(Op::Create {
                    kind: EntityKind::Person,
                    entity: e,
                });
                created.push(Created {
                    kind: EntityKind::Person,
                    id: id.clone(),
                    label: name.clone(),
                });
                new_people.insert(folded(&name), id.clone());
                Ok(Some(id))
            }
            Err(e) => Err(Refusal::new(field, typed, person_key(e))),
        }
    };
    let father = person_id("father", &s.father, "M", &mut ops, &mut created)?;
    let mother = person_id("mother", &s.mother, "F", &mut ops, &mut created)?;
    if father.is_some() && father == mother {
        return Err(Refusal::new(
            "mother",
            &s.mother,
            "person-new-error-same-parents",
        ));
    }
    let mut siblings: Vec<(String, bool)> = Vec::new();
    for typed in &s.siblings {
        let existed = matches!(who(typed, flat), Ok(Some(Who::Existing(_))));
        if let Some(id) = person_id("sibling", typed, "U", &mut ops, &mut created)? {
            if !siblings.iter().any(|(s, _)| *s == id) {
                siblings.push((id, existed));
            }
        }
    }
    let mut unions: Vec<(Option<String>, String, Vec<String>)> = Vec::new();
    for (n, u) in s.unions.iter().enumerate() {
        let partner = person_id(
            &format!("u.{n}.partner"),
            &u.partner,
            "U",
            &mut ops,
            &mut created,
        )?;
        let mut kids = Vec::new();
        for typed in &u.children {
            if let Some(id) =
                person_id(&format!("u.{n}.child"), typed, "U", &mut ops, &mut created)?
            {
                if !kids.contains(&id) {
                    kids.push(id);
                }
            }
        }
        if partner.is_none() && kids.is_empty() {
            continue;
        }
        let kind = if UNION_TYPES.contains(&u.kind.as_str()) {
            u.kind.clone()
        } else {
            "unknown".into()
        };
        unions.push((partner, kind, kids));
    }

    // The person.
    let id = cx.mint();
    let living = s.living && !s.died();
    let mut person = json!({"id": id, "type": "person", "axgf_version": "1.0",
        "identity": {"name": {"display": display, "components": components(s)},
                     "gender": {"value": s.gender}, "is_living": living}});
    let fact = |date: Option<Value>, place: Option<String>| {
        let mut m = Map::new();
        if let Some(d) = date {
            m.insert("date".into(), d);
        }
        if let Some(p) = place {
            m.insert("place_id".into(), json!(p));
        }
        (!m.is_empty()).then_some(Value::Object(m))
    };
    if let Some(b) = fact(birth_date, birth_place) {
        person["birth"] = b;
    }
    if let Some(d) = fact(death_date, death_place) {
        person["death"] = d;
    }
    if !s.bio.trim().is_empty() {
        person["bio"] = json!(s.bio.trim());
    }
    if let Some(doc) = cx.avatar {
        person = crate::avatar::apply(
            &person,
            &crate::avatar::Choice::Document {
                id: doc.to_string(),
                focal: crate::avatar::Focal::default(),
            },
        );
    }
    if person.get("bio").is_some() {
        mark_bio_markdown(&mut person);
    }
    stamp(&mut person, cx.now);
    ops.push(Op::Create {
        kind: EntityKind::Person,
        entity: person,
    });

    // The family the person is a child of: their parents', or the one their
    // brothers and sisters are children of.
    let kids_with = |extra: &[String]| -> Vec<String> {
        let mut out = vec![id.clone()];
        for s in extra {
            if !out.contains(s) {
                out.push(s.clone());
            }
        }
        out
    };
    let sibling_ids: Vec<String> = siblings.iter().map(|(s, _)| s.clone()).collect();
    let parents: Vec<String> = [father.clone(), mother.clone()]
        .into_iter()
        .flatten()
        .collect();
    if !parents.is_empty() {
        let set: BTreeSet<&str> = parents.iter().map(String::as_str).collect();
        match union_of(flat, &set) {
            Some((fid, stored)) => {
                ops.push(add_children(&fid, &stored, &kids_with(&sibling_ids)));
                joined.push(fid);
            }
            None => {
                let fid = cx.mint();
                let persons: Vec<Value> = parents
                    .iter()
                    .map(|p| json!({"person_id": p, "role": "spouse"}))
                    .collect();
                let mut e = json!({"id": fid, "type": "family", "axgf_version": "1.0",
                    "union": {"type": "unknown", "persons": persons},
                    "children": kids_with(&sibling_ids).iter().map(|k| json!({"person_id": k})).collect::<Vec<_>>()});
                stamp(&mut e, cx.now);
                ops.push(Op::Create {
                    kind: EntityKind::Family,
                    entity: e,
                });
            }
        }
    } else if !siblings.is_empty() {
        let mut families: BTreeSet<String> = BTreeSet::new();
        for (sib, existed) in &siblings {
            if *existed {
                families.extend(families_of_child(flat, sib));
            }
        }
        match families.len() {
            1 => {
                let fid = families.into_iter().next().expect("one");
                let stored = flat
                    .get("families")
                    .and_then(|f| f.get(&fid))
                    .cloned()
                    .unwrap_or(Value::Null);
                ops.push(add_children(&fid, &stored, &kids_with(&sibling_ids)));
                joined.push(fid);
            }
            0 => {
                return Err(Refusal::new(
                    "sibling",
                    s.siblings.first().map(String::as_str).unwrap_or_default(),
                    "person-new-error-siblings-need-parent",
                ))
            }
            _ => {
                return Err(Refusal::new(
                    "sibling",
                    s.siblings.first().map(String::as_str).unwrap_or_default(),
                    "person-new-error-siblings-two-families",
                ))
            }
        }
    }

    // The person's own unions.
    for (partner, kind, kids) in unions {
        let fid = cx.mint();
        let mut persons = vec![json!({"person_id": id, "role": "spouse"})];
        if let Some(p) = partner {
            persons.push(json!({"person_id": p, "role": "spouse"}));
        }
        let mut e = json!({"id": fid, "type": "family", "axgf_version": "1.0",
            "union": {"type": kind, "persons": persons}});
        if !kids.is_empty() {
            e["children"] = json!(kids
                .iter()
                .map(|k| json!({"person_id": k}))
                .collect::<Vec<_>>());
        }
        stamp(&mut e, cx.now);
        ops.push(Op::Create {
            kind: EntityKind::Family,
            entity: e,
        });
    }

    Ok(Plan {
        person_id: id,
        ops,
        created,
        joined,
    })
}

fn person_key(e: PickError) -> &'static str {
    match e {
        PickError::Ambiguous => "pick-error-ambiguous",
        PickError::NotFound | PickError::Empty => "pick-error-not-found",
    }
}

fn place_key(e: PickError) -> &'static str {
    e.key_for("place")
}

/// The family whose union is exactly these people, if there is one.
fn union_of(flat: &Value, set: &BTreeSet<&str>) -> Option<(String, Value)> {
    let families = flat.get("families")?.as_object()?;
    families.iter().find_map(|(fid, f)| {
        let partners: BTreeSet<&str> = f
            .pointer("/union/persons")?
            .as_array()?
            .iter()
            .filter_map(|p| p.get("person_id").and_then(Value::as_str))
            .collect();
        (partners == *set).then(|| (fid.clone(), f.clone()))
    })
}

/// The families somebody is a child of.
fn families_of_child(flat: &Value, person: &str) -> Vec<String> {
    flat.get("families")
        .and_then(Value::as_object)
        .map(|fs| {
            fs.iter()
                .filter(|(_, f)| {
                    f.get("children")
                        .and_then(Value::as_array)
                        .is_some_and(|c| {
                            c.iter()
                                .any(|k| k.get("person_id").and_then(Value::as_str) == Some(person))
                        })
                })
                .map(|(fid, _)| fid.clone())
                .collect()
        })
        .unwrap_or_default()
}

/// An existing family with these children added to the ones it has.
fn add_children(fid: &str, stored: &Value, kids: &[String]) -> Op {
    let mut entity = stored.clone();
    let mut children = entity
        .get("children")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    for k in kids {
        if !children
            .iter()
            .any(|c| c.get("person_id").and_then(Value::as_str) == Some(k.as_str()))
        {
            children.push(json!({"person_id": k}));
        }
    }
    entity["children"] = Value::Array(children);
    entity["id"] = json!(fid);
    Op::Update {
        kind: EntityKind::Family,
        id: fid.to_string(),
        expected_version: crate::state::version_of(stored),
        entity,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    fn fields(pairs: &[(&str, &str)]) -> Vec<(String, String)> {
        pairs
            .iter()
            .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
            .collect()
    }

    fn archive() -> Value {
        json!({
            "persons": {
                "p-jan": {"id": "p-jan", "type": "person",
                    "identity": {"name": {"display": "Jan Kowalski", "components": []},
                                 "gender": {"value": "M"}, "is_living": false}},
                "p-anna": {"id": "p-anna", "type": "person",
                    "identity": {"name": {"display": "Anna Nowak", "components": []},
                                 "gender": {"value": "F"}, "is_living": false}},
                "p-ewa": {"id": "p-ewa", "type": "person",
                    "identity": {"name": {"display": "Ewa Kowalska", "components": []},
                                 "gender": {"value": "F"}, "is_living": false}},
                "p-twin1": {"id": "p-twin1", "type": "person",
                    "identity": {"name": {"display": "Piotr Lis", "components": []},
                                 "gender": {"value": "M"}, "is_living": false}},
                "p-twin2": {"id": "p-twin2", "type": "person",
                    "identity": {"name": {"display": "Piotr Lis", "components": []},
                                 "gender": {"value": "M"}, "is_living": false}}
            },
            "families": {
                "f-jan-anna": {"id": "f-jan-anna", "type": "family", "version_num": 3,
                    "union": {"type": "marriage", "persons": [
                        {"person_id": "p-jan", "role": "spouse"},
                        {"person_id": "p-anna", "role": "spouse"}]},
                    "children": [{"person_id": "p-ewa"}]}
            },
            "places": {
                "pl-krakow": {"id": "pl-krakow", "type": "place",
                    "names": [{"lang": "pl", "value": "Kraków", "is_primary": true}]}
            }
        })
    }

    fn plan_of(pairs: &[(&str, &str)], flat: &Value) -> Result<Plan, Refusal> {
        let ids = RefCell::new(
            (1..=40)
                .rev()
                .map(|n| format!("new-{n}"))
                .collect::<Vec<_>>(),
        );
        let quote = |n: &str| format!("«{n}»");
        let cx = Context {
            flat,
            quote: &quote,
            now: "2026-11-09T12:00:00Z",
            avatar: None,
            ids: Some(&ids),
        };
        plan(&Submission::from_fields(&fields(pairs)), &cx)
    }

    fn creates(p: &Plan, kind: EntityKind) -> Vec<Value> {
        p.ops
            .iter()
            .filter_map(|op| match op {
                Op::Create { kind: k, entity } if *k == kind => Some(entity.clone()),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn the_name_is_made_of_its_parts() {
        let s = Submission::from_fields(&fields(&[
            ("given", "Józef Maria"),
            ("nick", "Józek"),
            ("nick", ""),
            ("nick", "Dziadek"),
            ("family", "Gęsiorek"),
        ]));
        assert_eq!(
            display_name(&s, &|n| format!("« {n} »")),
            "Józef Maria « Józek » « Dziadek » Gęsiorek"
        );
        let c = components(&s);
        assert_eq!(c.len(), 4);
        assert_eq!(
            c[0],
            json!({"type": "given_name", "value": "Józef Maria", "order": 1})
        );
        assert_eq!(c[1]["type"], "nickname");
        assert_eq!(
            c[3],
            json!({"type": "family_name", "value": "Gęsiorek", "order": 4})
        );
        assert_eq!(
            split_typed("Jean Pierre Martin"),
            ("Jean Pierre".into(), "Martin".into())
        );
        assert_eq!(split_typed("Martin"), ("Martin".into(), String::new()));
    }

    #[test]
    fn a_name_is_needed_and_dates_must_be_real() {
        let flat = archive();
        assert_eq!(
            plan_of(&[("given", "")], &flat).unwrap_err().key,
            "person-new-error-name"
        );
        let r = plan_of(&[("given", "Ala"), ("birth_date", "1890-13")], &flat).unwrap_err();
        assert_eq!(
            (r.field.as_str(), r.key),
            ("birth_date", "person-new-error-date")
        );
    }

    #[test]
    fn places_are_reused_or_created_once() {
        let flat = archive();
        let p = plan_of(
            &[
                ("given", "Ala"),
                ("birth_date", "1890"),
                ("birth_circa", "on"),
                ("birth_place", "kraków"),
                ("death_place", "Słomniki"),
                ("death_date", "1950-06-14"),
            ],
            &flat,
        )
        .unwrap();
        let places = creates(&p, EntityKind::Place);
        assert_eq!(places.len(), 1, "Kraków exists; only Słomniki is new");
        assert_eq!(places[0]["names"][0]["value"], "Słomniki");
        let person = creates(&p, EntityKind::Person).pop().unwrap();
        assert_eq!(person["birth"]["place_id"], "pl-krakow");
        assert_eq!(
            person["birth"]["date"],
            json!({"value": "1890", "precision": "year", "circa": true})
        );
        assert_eq!(person["death"]["place_id"], places[0]["id"]);
        assert_eq!(person["identity"]["is_living"], false, "a death says so");
        // The same new place twice is one place.
        let p = plan_of(
            &[
                ("given", "Ala"),
                ("birth_place", "Wola"),
                ("death_place", " wola "),
            ],
            &flat,
        )
        .unwrap();
        assert_eq!(creates(&p, EntityKind::Place).len(), 1);
    }

    #[test]
    fn parents_already_a_couple_take_the_child_into_their_family() {
        let flat = archive();
        let p = plan_of(
            &[
                ("given", "Zofia"),
                ("family", "Kowalska"),
                ("living", "on"),
                ("father", "Jan Kowalski"),
                ("mother", "Anna Nowak · #p-anna"),
            ],
            &flat,
        )
        .unwrap();
        assert_eq!(p.joined, vec!["f-jan-anna".to_string()]);
        let update = p
            .ops
            .iter()
            .find_map(|op| match op {
                Op::Update {
                    id,
                    expected_version,
                    entity,
                    ..
                } => Some((id.clone(), *expected_version, entity.clone())),
                _ => None,
            })
            .expect("the family updated");
        assert_eq!((update.0.as_str(), update.1), ("f-jan-anna", 3));
        let kids: Vec<&str> = update.2["children"]
            .as_array()
            .unwrap()
            .iter()
            .map(|c| c["person_id"].as_str().unwrap())
            .collect();
        assert_eq!(kids, vec!["p-ewa", p.person_id.as_str()]);
        assert!(creates(&p, EntityKind::Family).is_empty());
        assert!(
            creates(&p, EntityKind::Person).len() == 1,
            "nobody else created"
        );
        let person = creates(&p, EntityKind::Person).pop().unwrap();
        assert_eq!(person["identity"]["is_living"], true);
    }

    #[test]
    fn new_parents_siblings_and_unions_are_created_with_the_person() {
        let flat = archive();
        let p = plan_of(
            &[
                ("given", "Marek"),
                ("family", "Testowski"),
                ("father", "Adam Testowski"),
                ("mother", "Ewa Kowalska"),
                ("sibling", "Basia Wiśniewska"),
                ("sibling", "Basia  Wiśniewska"),
                ("u.0.partner", "Hanna Zielińska"),
                ("u.0.type", "marriage"),
                ("u.0.child", "Tomek Testowski"),
                ("u.0.child", "Ola Wiśniewska"),
                ("u.1.partner", ""),
                ("u.1.child", ""),
                ("u.2.partner", "Hanna Zielińska"),
                ("u.2.type", "bogus"),
            ],
            &flat,
        )
        .unwrap();
        let people: Vec<String> = creates(&p, EntityKind::Person)
            .iter()
            .map(|e| {
                e["identity"]["name"]["display"]
                    .as_str()
                    .unwrap()
                    .to_string()
            })
            .collect();
        assert_eq!(
            people,
            vec![
                "Adam Testowski",
                "Basia Wiśniewska",
                "Hanna Zielińska",
                "Tomek Testowski",
                "Ola Wiśniewska",
                "Marek Testowski"
            ],
            "each new name once, the person after the people they name"
        );
        let adam = &creates(&p, EntityKind::Person)[0];
        assert_eq!(adam["identity"]["gender"]["value"], "M");
        assert_eq!(
            adam["identity"]["name"]["components"][1]["value"],
            "Testowski"
        );
        let families = creates(&p, EntityKind::Family);
        assert_eq!(families.len(), 3, "the parents' family and two unions");
        let parents = &families[0];
        assert_eq!(parents["union"]["persons"][1]["person_id"], "p-ewa");
        assert_eq!(parents["children"].as_array().unwrap().len(), 2);
        assert_eq!(families[1]["union"]["type"], "marriage");
        assert_eq!(families[1]["children"].as_array().unwrap().len(), 2);
        assert_eq!(
            families[2]["union"]["type"], "unknown",
            "an unknown kind is unknown"
        );
        assert_eq!(
            families[2]["union"]["persons"][1]["person_id"],
            families[1]["union"]["persons"][1]["person_id"],
            "the same partner in both unions"
        );
        // Every record is created before a record names it.
        let pos = |id: &str| {
            p.ops
                .iter()
                .position(|op| matches!(op, Op::Create { entity, .. } if entity["id"] == id))
                .unwrap()
        };
        let me = pos(&p.person_id);
        for f in &families {
            assert!(pos(f["id"].as_str().unwrap()) > me);
        }
    }

    #[test]
    fn siblings_alone_find_their_parents_or_are_refused() {
        let flat = archive();
        let p = plan_of(&[("given", "Kasia"), ("sibling", "Ewa Kowalska")], &flat).unwrap();
        assert_eq!(p.joined, vec!["f-jan-anna".to_string()]);
        let r = plan_of(&[("given", "Kasia"), ("sibling", "Somebody New")], &flat).unwrap_err();
        assert_eq!(r.key, "person-new-error-siblings-need-parent");
    }

    #[test]
    fn a_name_two_people_share_and_one_parent_twice_are_refused() {
        let flat = archive();
        let r = plan_of(&[("given", "Kasia"), ("father", "Piotr Lis")], &flat).unwrap_err();
        assert_eq!(
            (r.field.as_str(), r.key),
            ("father", "pick-error-ambiguous")
        );
        let r = plan_of(
            &[
                ("given", "Kasia"),
                ("father", "Jan Kowalski"),
                ("mother", "jan kowalski"),
            ],
            &flat,
        )
        .unwrap_err();
        assert_eq!(r.key, "person-new-error-same-parents");
        // A short word is a name to create, never an id's beginning.
        let p = plan_of(&[("given", "Kasia"), ("father", "p")], &flat).unwrap();
        assert_eq!(
            creates(&p, EntityKind::Person)[0]["identity"]["name"]["display"],
            "p"
        );
    }

    #[test]
    fn the_avatar_is_the_chosen_document() {
        let flat = archive();
        let ids = RefCell::new(vec!["new-1".to_string()]);
        let quote = |n: &str| n.to_string();
        let cx = Context {
            flat: &flat,
            quote: &quote,
            now: "2026-11-09T12:00:00Z",
            avatar: Some("doc-1"),
            ids: Some(&ids),
        };
        let p = plan(&Submission::from_fields(&fields(&[("given", "Ala")])), &cx).unwrap();
        let person = creates(&p, EntityKind::Person).pop().unwrap();
        assert!(matches!(
            crate::avatar::read(&person),
            crate::avatar::Choice::Document { id, .. } if id == "doc-1"
        ));
        assert_eq!(person["version_num"], 1);
    }
}
