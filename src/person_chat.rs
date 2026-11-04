//! The AFG on a person's page (the operator's request of 4 November 2026,
//! docs/adr/0031): a member the AFG is open to talks with it about one
//! person, and the AFG reads, with every message, what that member may read
//! of them and, more broadly, of the family.
//!
//! What it reads, the dossier:
//!
//! 1. the person's record, as the page draws it, without its files;
//! 2. the notes about them, their transcriptions and their summary included;
//! 3. their close relatives (parents, siblings, spouses, children), each with
//!    its record and the notes about it;
//! 4. everyone of the family the member may read, one line each.
//!
//! Everything goes through the member's lens: a person the member may not
//! read is neither in the dossier nor in the index. Each part has a ceiling,
//! so that the dossier stays within what a model reads in one go.
//!
//! The conversations are kept in the member's assistant space, in a project
//! of the person's own (`assistant::person_project_id`).

use serde_json::Value;

use crate::afg_chat::ChatPrompt;
use crate::assistant::{self, Asker, Message, Project, Space};

/// The most of the notes about the person, in characters.
pub const PERSON_NOTES: usize = 40_000;
/// The most of the close relatives' records, in characters.
pub const RELATIVES: usize = 40_000;
/// The most of the notes about the close relatives, in characters.
pub const RELATIVE_NOTES: usize = 30_000;
/// The most of the index of the family, in characters.
pub const INDEX: usize = 40_000;

/// How a close relative is related to the person, as the dossier says it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Relation {
    Parent,
    Spouse,
    Child,
    Sibling,
}

impl Relation {
    fn word(self) -> &'static str {
        match self {
            Relation::Parent => "Parent",
            Relation::Spouse => "Spouse or partner",
            Relation::Child => "Child",
            Relation::Sibling => "Sibling",
        }
    }
}

/// A note, its text not read yet: its title and where its text is kept.
#[derive(Debug, Clone)]
pub struct NoteRef {
    pub title: String,
    pub path: Option<String>,
}

/// What is gathered while the archive is read: everything but the notes'
/// text, which is read once the archive is let go.
#[derive(Debug, Clone)]
pub struct Gathered {
    pub person_id: String,
    pub name: String,
    pub record: Value,
    pub notes: Vec<NoteRef>,
    pub relatives: Vec<(Relation, String, String, Value, Vec<NoteRef>)>,
    /// One line for each person the member may read, by name.
    pub index: Vec<String>,
}

/// A close relative, as the dossier gives it.
#[derive(Debug, Clone)]
pub struct Relative {
    pub relation: Relation,
    pub id: String,
    pub name: String,
    pub record: Value,
    pub notes: Vec<(String, String)>,
}

/// What the AFG reads about one person.
#[derive(Debug, Clone)]
pub struct Dossier {
    pub person_id: String,
    pub name: String,
    pub record: Value,
    pub notes: Vec<(String, String)>,
    pub relatives: Vec<Relative>,
    pub index: Vec<String>,
    /// Everyone the member may read, including those the index had no room
    /// for.
    pub persons: usize,
}

/// The record of one person as the dossier gives it: the page's record,
/// without its files, in the member's language.
fn record_of(
    flat: &Value,
    lens: &crate::access::Lens,
    id: &str,
    lang: &str,
    allowed: Option<crate::sensitive::Scopes>,
) -> Option<(String, Value, crate::person::PersonView)> {
    let view = crate::person::build_in(flat, id, lens, lang)?;
    let profile = match allowed {
        Some(scopes) => crate::person::profile_groups_within(flat, id, lens, lang, scopes),
        None => Vec::new(),
    };
    let record = crate::afg_record::record_for_summary(&view, &profile);
    Some((view.name.clone(), record, view))
}

/// The notes about one person this member may read, newest first.
fn notes_of(flat: &Value, lens: &crate::access::Lens, id: &str) -> Vec<NoteRef> {
    crate::notes::notes_about(flat, lens, true, id)
        .into_iter()
        .map(|(_, d)| NoteRef {
            title: if crate::afg_record::is_summary_of(d, id) {
                format!("Summary — {}", crate::notes::title(d))
            } else {
                crate::notes::title(d)
            },
            path: crate::notes::text_path(d),
        })
        .collect()
}

/// Read the archive for the dossier, through the member's lens. `None` when
/// the member may not read the person, or there is none.
pub fn gather(
    flat: &Value,
    lens: &crate::access::Lens,
    person_id: &str,
    lang: &str,
    allowed: crate::sensitive::Scopes,
) -> Option<Gathered> {
    if !lens.sees_person(person_id) {
        return None;
    }
    let (name, record, view) = record_of(flat, lens, person_id, lang, Some(allowed))?;
    let notes = notes_of(flat, lens, person_id);

    // The close relatives, the nearest first, each once.
    let mut near: Vec<(Relation, String)> = Vec::new();
    let mut add = |rel: Relation, r: &crate::person::PersonRef| {
        if r.known && !r.restricted && r.id != person_id && !near.iter().any(|(_, id)| *id == r.id)
        {
            near.push((rel, r.id.clone()));
        }
    };
    for r in &view.parents {
        add(Relation::Parent, r);
    }
    for g in &view.unions {
        for u in &g.records {
            if let Some(s) = &u.spouse {
                add(Relation::Spouse, s);
            }
        }
    }
    for g in &view.unions {
        for u in &g.records {
            for c in &u.children {
                add(Relation::Child, c);
            }
        }
    }
    for r in &view.siblings {
        add(Relation::Sibling, r);
    }
    let relatives = near
        .into_iter()
        .filter(|(_, id)| lens.sees_person(id))
        .filter_map(|(rel, id)| {
            let (name, record, _) = record_of(flat, lens, &id, lang, None)?;
            let notes = notes_of(flat, lens, &id);
            Some((rel, id, name, record, notes))
        })
        .collect();

    // Everyone the member may read, by name.
    let mut index: Vec<(String, String)> = flat
        .get("persons")
        .and_then(Value::as_object)
        .map(|ps| {
            ps.iter()
                .filter(|(id, _)| lens.sees_person(id))
                .map(|(id, p)| {
                    let name = crate::view::person_display_name(p);
                    let line = match crate::person::lifespan_of(p, lang) {
                        Some(span) => format!("- {name} ({span}) — persons/{id}"),
                        None => format!("- {name} — persons/{id}"),
                    };
                    (name.to_lowercase(), line)
                })
                .collect()
        })
        .unwrap_or_default();
    index.sort();
    Some(Gathered {
        person_id: person_id.to_string(),
        name,
        record,
        notes,
        relatives,
        index: index.into_iter().map(|(_, line)| line).collect(),
    })
}

/// A text cut to `n` characters, said to be cut when it is.
fn clip(s: &str, n: usize) -> String {
    let s = s.trim();
    if s.chars().count() <= n {
        return s.to_string();
    }
    let mut out: String = s.chars().take(n).collect();
    out.push_str(" […]");
    out
}

/// Read the notes' text, each part within its ceiling: the dossier.
pub fn fill(g: Gathered, read: &dyn Fn(&str) -> Option<String>) -> Dossier {
    let read_notes = |refs: &[NoteRef], budget: &mut usize| -> Vec<(String, String)> {
        let mut out = Vec::new();
        for n in refs {
            if *budget == 0 {
                break;
            }
            let Some(body) = n.path.as_deref().and_then(read) else {
                continue;
            };
            let body = clip(&body, *budget);
            *budget = budget.saturating_sub(body.chars().count());
            out.push((n.title.clone(), body));
        }
        out
    };
    let mut person_budget = PERSON_NOTES;
    let notes = read_notes(&g.notes, &mut person_budget);
    let mut records_budget = RELATIVES;
    let mut notes_budget = RELATIVE_NOTES;
    let mut relatives = Vec::new();
    for (relation, id, name, record, refs) in g.relatives {
        let size = record.to_string().chars().count();
        if size > records_budget {
            break;
        }
        records_budget -= size;
        let notes = read_notes(&refs, &mut notes_budget);
        relatives.push(Relative {
            relation,
            id,
            name,
            record,
            notes,
        });
    }
    let persons = g.index.len();
    let mut index = Vec::new();
    let mut room = INDEX;
    for line in g.index {
        let size = line.chars().count() + 1;
        if size > room {
            break;
        }
        room -= size;
        index.push(line);
    }
    Dossier {
        person_id: g.person_id,
        name: g.name,
        record: g.record,
        notes,
        relatives,
        index,
        persons,
    }
}

impl Dossier {
    /// The dossier as the AFG reads it.
    pub fn text(&self) -> String {
        let json = |v: &Value| serde_json::to_string_pretty(v).unwrap_or_default();
        let mut s = format!(
            "# {} (persons/{})\n\nThe record:\n\n```json\n{}\n```\n",
            self.name,
            self.person_id,
            json(&self.record)
        );
        if self.notes.is_empty() {
            s.push_str("\nNo note about this person.\n");
        } else {
            s.push_str(&format!("\n## The notes about {}\n", self.name));
            for (title, body) in &self.notes {
                s.push_str(&format!("\n### {title}\n\n{body}\n"));
            }
        }
        if !self.relatives.is_empty() {
            s.push_str("\n## Close relatives\n");
            for r in &self.relatives {
                s.push_str(&format!(
                    "\n### {}: {} (persons/{})\n\n```json\n{}\n```\n",
                    r.relation.word(),
                    r.name,
                    r.id,
                    serde_json::to_string(&r.record).unwrap_or_default()
                ));
                for (title, body) in &r.notes {
                    s.push_str(&format!("\nA note about {}: {title}\n\n{body}\n", r.name));
                }
            }
        }
        s.push_str(&format!(
            "\n## The family: everyone this member may read ({} persons)\n\n",
            self.persons
        ));
        for line in &self.index {
            s.push_str(line);
            s.push('\n');
        }
        if self.index.len() < self.persons {
            s.push_str(&format!(
                "[{} more are not listed here, for want of room.]\n",
                self.persons - self.index.len()
            ));
        }
        s
    }

    /// A rough size, a token for every three characters, as the AFG room
    /// counts the archive's.
    pub fn tokens(&self) -> usize {
        self.text().chars().count() / 3
    }
}

/// The instructions of a conversation about one person.
pub fn system_prompt(asker: &Asker, dossier: &Dossier) -> String {
    let mut s = format!(
        "You are the Artificial family genealogist (AFG) of an ax·genealogy archive. On the \
         page of one person of the family, a member asks you about that person: {} \
         (persons/{}). Help them understand this person's life and look further into it, \
         kindly and precisely. ",
        dossier.name, dossier.person_id
    );
    match asker.person {
        Some((id, name)) => s.push_str(&format!(
            "You are talking to {}, who says they are {name} (persons/{id}) in the family's \
             archive. ",
            asker.name
        )),
        None => s.push_str(&format!("You are talking to {}. ", asker.name)),
    }
    s.push_str(&format!(
        "Answer in the language they write in; their pages are in {}. Write in Markdown, \
         briefly unless they ask for more.\n\n",
        asker.language
    ));
    s.push_str(
        "The dossier given with the conversation is what this member may read of the \
         family's archive: the person's record, the notes about them, their close relatives \
         with the notes about them, and every person of the family the member may read, one \
         line each. It is all you know of this family: there is no other record to look up.\n\
         - Say what the dossier says, and where it says it: a note's title, an event, a \
         source, a relative's record.\n\
         - When you go beyond it (history, places, military units, customs, registers), say \
         so plainly and say how sure you are. A likely answer is welcome, marked as likely.\n\
         - Never invent a fact about the family. When the dossier does not say, say so, and \
         suggest where it could be found: civil and parish registers, military records, \
         archives, the family's papers.\n\
         - When you name someone of the family who is in the dossier, link them by their id: \
         [their name](/person/<id>).",
    );
    s
}

/// Everything the AFG is given for the next answer of a conversation about
/// one person: its instructions, the dossier, and the conversation's turns.
pub fn chat_prompt(
    space: &Space,
    asker: &Asker,
    project: &Project,
    dossier: &Dossier,
    messages: &[Message],
) -> ChatPrompt {
    ChatPrompt {
        system: system_prompt(asker, dossier),
        context: dossier.text(),
        turns: assistant::turns(space, &project.id, messages),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn gathered(notes: usize, index: usize) -> Gathered {
        Gathered {
            person_id: "p1".into(),
            name: "Jean Martin".into(),
            record: json!({"name": "Jean Martin"}),
            notes: (0..notes)
                .map(|i| NoteRef {
                    title: format!("Note {i}"),
                    path: Some(format!("n{i}")),
                })
                .collect(),
            relatives: vec![(
                Relation::Parent,
                "p0".into(),
                "Paul Martin".into(),
                json!({"name": "Paul Martin"}),
                vec![NoteRef {
                    title: "Carnet".into(),
                    path: Some("c".into()),
                }],
            )],
            index: (0..index)
                .map(|i| format!("- Person {i} — persons/x{i}"))
                .collect(),
        }
    }

    #[test]
    fn the_dossier_keeps_to_its_ceilings_and_says_what_it_left_out() {
        let long = "régiment ".repeat(10_000);
        let d = fill(gathered(3, 10_000), &|_| Some(long.clone()));
        let total: usize = d.notes.iter().map(|(_, b)| b.chars().count()).sum();
        assert!(total <= PERSON_NOTES + 3 * 4, "{total}");
        assert!(d.notes[0].1.ends_with("[…]"));
        assert_eq!(d.persons, 10_000);
        assert!(d.index.len() < 10_000);
        let text = d.text();
        assert!(text.contains("# Jean Martin (persons/p1)"));
        assert!(text.contains("### Parent: Paul Martin (persons/p0)"));
        assert!(text.contains("A note about Paul Martin: Carnet"));
        assert!(text.contains("more are not listed here"));
        assert!(d.tokens() > 0);
    }

    #[test]
    fn a_note_whose_text_cannot_be_read_is_left_out() {
        let d = fill(gathered(2, 1), &|p| {
            (p == "n1").then(|| "Verdun, 1916.".to_string())
        });
        assert_eq!(
            d.notes,
            vec![("Note 1".to_string(), "Verdun, 1916.".to_string())]
        );
        assert!(!d.text().contains("more are not listed"));
    }

    #[test]
    fn the_instructions_name_the_person_and_the_member() {
        let d = fill(gathered(0, 0), &|_| None);
        let asker = Asker {
            name: "vera",
            person: Some(("p9", "Véra Martin")),
            language: "French",
        };
        let s = system_prompt(&asker, &d);
        assert!(s.contains("Jean Martin (persons/p1)"));
        assert!(s.contains("Véra Martin (persons/p9)"));
        assert!(s.contains("their pages are in French"));
        assert!(s.contains("[their name](/person/<id>)"));
        assert!(d.text().contains("No note about this person."));
    }
}
