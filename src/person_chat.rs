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
//! 4. everyone of the family the member may read, one line each;
//! 5. the photographs of the person's record (the operator's request of
//!    6 November 2026, docs/adr/0033), given with the member's message as
//!    pictures. Films, sound and PDF documents are not given.
//!
//! Everything goes through the member's lens: a person the member may not
//! read is neither in the dossier nor in the index, and a file the member may
//! not open is not given. Each part has a ceiling, so that the dossier stays
//! within what a model reads in one go.
//!
//! The conversations are kept in the member's assistant space, in a project
//! of the person's own (`assistant::person_project_id`).

use serde_json::Value;

use crate::afg_chat::{ChatPrompt, Part, Speaker, Turn};
use crate::assistant::{self, Asker, Message, Project, Space};

/// The most of the notes about the person, in characters.
pub const PERSON_NOTES: usize = 40_000;
/// The most of the close relatives' records, in characters.
pub const RELATIVES: usize = 40_000;
/// The most of the notes about the close relatives, in characters.
pub const RELATIVE_NOTES: usize = 30_000;
/// The most of the index of the family, in characters.
pub const INDEX: usize = 40_000;
/// The most photographs of the person given with a message.
pub const PHOTOS: usize = 12;
/// What a photograph is counted as, in tokens: about what a model reads of
/// a picture at the size it is given (`assistant::model_photo`).
pub const PHOTO_TOKENS: usize = 1_600;
/// The most pages of the record's PDF documents given with a message
/// (docs/adr/0040): a model reads a page as a picture and as its text.
pub const PDF_PAGES: usize = 60;
/// The most of their bytes. Sent as base64, a third larger, they leave room
/// under the 32 MB a request may weigh for the photographs and the dossier.
pub const PDF_BYTES: u64 = 20 * 1024 * 1024;
/// What a page of a PDF is counted as, in tokens.
pub const PDF_PAGE_TOKENS: usize = 2_000;

/// Whether a file of this type is a picture the AFG is given: the raster
/// formats this build decodes. A TIFF, an SVG, a film are not.
pub fn readable_photo(mime: &str) -> bool {
    matches!(
        mime,
        "image/jpeg" | "image/png" | "image/gif" | "image/webp" | "image/bmp"
    )
}

/// One of the person's photographs, before its bytes are read: what the
/// record says of it, and where its bytes are kept.
#[derive(Debug, Clone, PartialEq)]
pub struct Photo {
    pub id: String,
    pub filename: String,
    /// The kind of document, in the member's language.
    pub kind: String,
    pub caption: Option<String>,
    pub note: Option<String>,
    /// Where the bundle keeps its bytes.
    pub path: String,
    pub sha256: String,
}

/// One of the person's PDF documents (docs/adr/0040), before its bytes are
/// read: what the record says of it, where its bytes are kept, how large it
/// is, and its pages once counted.
#[derive(Debug, Clone, PartialEq)]
pub struct Pdf {
    pub id: String,
    pub filename: String,
    /// The kind of document, in the member's language.
    pub kind: String,
    pub caption: Option<String>,
    pub note: Option<String>,
    /// Where the bundle keeps its bytes.
    pub path: String,
    pub sha256: String,
    pub size: u64,
    pub pages: usize,
}

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
    /// The photographs of the person's record the AFG may be given, in the
    /// record's order.
    pub photos: Vec<Photo>,
    /// Its PDF documents, in the record's order, their pages not counted yet.
    pub pdfs: Vec<Pdf>,
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
    /// The photographs given with each message, [`PHOTOS`] at most.
    pub photos: Vec<Photo>,
    /// The photographs of the record left out for want of room.
    pub photos_left: usize,
    /// The PDF documents given with each message, within [`PDF_PAGES`] and
    /// [`PDF_BYTES`].
    pub pdfs: Vec<Pdf>,
    /// Those left out: for want of room, or because they cannot be read.
    pub pdfs_left: usize,
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
    let photos = photos_of(flat, lens, &view, allowed);
    let pdfs = pdfs_of(flat, lens, &view, allowed);

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
        photos,
        pdfs,
    })
}

/// The photographs of the person's record the AFG may be given: the
/// pictures of the record the member may open, as the page lists them,
/// without a picture an AI made, and without one that sensitive data refers
/// to unless the AFG room lets those go (docs/adr/0023).
fn photos_of(
    flat: &Value,
    lens: &crate::access::Lens,
    view: &crate::person::PersonView,
    allowed: crate::sensitive::Scopes,
) -> Vec<Photo> {
    let sensitive = sensitive_documents(flat, allowed);
    view.documents
        .iter()
        .filter(|d| d.known && d.has_payload && !d.ai)
        .filter(|d| d.mime_type.as_deref().is_some_and(readable_photo))
        .filter(|d| !sensitive.contains(&d.id))
        .filter(|d| crate::access::may_read_document(flat, lens, true, &d.id))
        .filter_map(|d| {
            let file = flat.get("documents")?.get(&d.id)?.get("file")?;
            Some(Photo {
                id: d.id.clone(),
                filename: d.filename.clone(),
                kind: d.document_type_label.clone(),
                caption: d.caption.clone(),
                note: d.note.clone(),
                path: file.get("path")?.as_str()?.to_string(),
                sha256: file
                    .get("sha256")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_string(),
            })
        })
        .collect()
}

/// The documents that sensitive data the AFG room does not let go refers
/// to (docs/adr/0023).
fn sensitive_documents(
    flat: &Value,
    allowed: crate::sensitive::Scopes,
) -> std::collections::BTreeSet<String> {
    flat.get("persons")
        .and_then(Value::as_object)
        .map(|ps| {
            ps.values()
                .flat_map(crate::sensitive::documents_referenced)
                .filter(|(_, scopes)| !scopes.within(allowed))
                .map(|(doc, _)| doc)
                .collect()
        })
        .unwrap_or_default()
}

/// The PDF documents of the person's record the AFG may be given
/// (docs/adr/0040), on the photographs' terms: the record's own the member
/// may open, none an AI made, none sensitive data refers to unless the AFG
/// room lets it go.
fn pdfs_of(
    flat: &Value,
    lens: &crate::access::Lens,
    view: &crate::person::PersonView,
    allowed: crate::sensitive::Scopes,
) -> Vec<Pdf> {
    let sensitive = sensitive_documents(flat, allowed);
    view.documents
        .iter()
        .filter(|d| d.known && d.has_payload && !d.ai)
        .filter(|d| d.mime_type.as_deref() == Some("application/pdf"))
        .filter(|d| !sensitive.contains(&d.id))
        .filter(|d| crate::access::may_read_document(flat, lens, true, &d.id))
        .filter_map(|d| {
            let file = flat.get("documents")?.get(&d.id)?.get("file")?;
            Some(Pdf {
                id: d.id.clone(),
                filename: d.filename.clone(),
                kind: d.document_type_label.clone(),
                caption: d.caption.clone(),
                note: d.note.clone(),
                path: file.get("path")?.as_str()?.to_string(),
                sha256: file
                    .get("sha256")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_string(),
                size: file.get("size_bytes").and_then(Value::as_u64).unwrap_or(0),
                pages: 0,
            })
        })
        .collect()
}

/// The PDF documents given with a message: in the record's order, each whole,
/// while the pages and the bytes stay within [`PDF_PAGES`] and
/// [`PDF_BYTES`]; one whose pages cannot be counted is not a PDF a model
/// reads either. The rest are left out, and counted.
pub fn pdfs_within(pdfs: Vec<Pdf>, pages: &dyn Fn(&Pdf) -> Option<usize>) -> (Vec<Pdf>, usize) {
    let (mut given, mut left) = (Vec::new(), 0);
    let (mut n_pages, mut n_bytes) = (0usize, 0u64);
    for mut p in pdfs {
        match pages(&p) {
            Some(n) if n > 0 && n_pages + n <= PDF_PAGES && n_bytes + p.size <= PDF_BYTES => {
                n_pages += n;
                n_bytes += p.size;
                p.pages = n;
                given.push(p);
            }
            _ => left += 1,
        }
    }
    (given, left)
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
pub fn fill(
    g: Gathered,
    read: &dyn Fn(&str) -> Option<String>,
    pages: &dyn Fn(&Pdf) -> Option<usize>,
) -> Dossier {
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
    let photos_left = g.photos.len().saturating_sub(PHOTOS);
    let mut photos = g.photos;
    photos.truncate(PHOTOS);
    let (pdfs, pdfs_left) = pdfs_within(g.pdfs, pages);
    Dossier {
        person_id: g.person_id,
        name: g.name,
        record: g.record,
        notes,
        relatives,
        index,
        persons,
        photos,
        photos_left,
        pdfs,
        pdfs_left,
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
        if !self.photos.is_empty() {
            s.push_str(&format!(
                "\n## The photographs of {}\n\nThe pictures of the record, given with \
                 the member's message in this order:\n\n",
                self.name
            ));
            for (i, p) in self.photos.iter().enumerate() {
                s.push_str(&format!("{}. \"{}\" ({})", i + 1, p.filename, p.kind));
                for words in [&p.caption, &p.note].into_iter().flatten() {
                    s.push_str(&format!(" — {words}"));
                }
                s.push('\n');
            }
            if self.photos_left > 0 {
                s.push_str(&format!(
                    "[{} more photographs of the record are not given, for want of room.]\n",
                    self.photos_left
                ));
            }
        }
        if !self.pdfs.is_empty() || self.pdfs_left > 0 {
            s.push_str(&format!("\n## The PDF documents of {}\n\n", self.name));
            if !self.pdfs.is_empty() {
                s.push_str("Given with the member's message in this order:\n\n");
            }
            for (i, p) in self.pdfs.iter().enumerate() {
                s.push_str(&format!(
                    "P{}. \"{}\" ({}, {} pages)",
                    i + 1,
                    p.filename,
                    p.kind,
                    p.pages
                ));
                for words in [&p.caption, &p.note].into_iter().flatten() {
                    s.push_str(&format!(" — {words}"));
                }
                s.push('\n');
            }
            if self.pdfs_left > 0 {
                s.push_str(&format!(
                    "[{} more PDF documents of the record are not given: too long for one \
                     message, or unreadable.]\n",
                    self.pdfs_left
                ));
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
    /// counts the archive's, [`PHOTO_TOKENS`] for each photograph and
    /// [`PDF_PAGE_TOKENS`] for each page of a PDF.
    pub fn tokens(&self) -> usize {
        self.text().chars().count() / 3
            + self.photos.len() * PHOTO_TOKENS
            + self.pdfs.iter().map(|p| p.pages).sum::<usize>() * PDF_PAGE_TOKENS
    }

    /// The pages of the PDF documents given.
    pub fn pdf_pages(&self) -> usize {
        self.pdfs.iter().map(|p| p.pages).sum()
    }
}

/// The member's message being answered: the last of the conversation's
/// turns, which is theirs.
fn last_said(prompt: &mut ChatPrompt) -> &mut Turn {
    if prompt
        .turns
        .last()
        .is_none_or(|t| t.speaker != Speaker::User)
    {
        prompt.turns.push(Turn {
            speaker: Speaker::User,
            parts: Vec::new(),
        });
    }
    let last = prompt.turns.len() - 1;
    &mut prompt.turns[last]
}

/// The question as it goes to a model that took no file: the AFG is told
/// that what the dossier lists — its photographs, its PDF documents
/// (docs/adr/0040), or both — could not be given, so that it says so, in
/// the member's language, when the question needs them.
pub fn files_refused(prompt: &mut ChatPrompt, photos: bool, pdfs: bool) {
    let what = match (photos, pdfs) {
        (true, true) => "The photographs and the PDF documents of the record",
        (false, true) => "The PDF documents of the record",
        _ => "The photographs of the record",
    };
    last_said(prompt).parts.push(Part::Text(format!(
        "[{what} could not be given: the AFG's model does not take them. If the question \
         needs them, say so.]"
    )));
}

/// The person's PDF documents, joined to the member's message being
/// answered (docs/adr/0040). Each is named by its number and file name, as
/// the dossier lists them.
pub fn with_pdfs(prompt: &mut ChatPrompt, pdfs: Vec<(String, Vec<u8>)>) {
    if pdfs.is_empty() {
        return;
    }
    let turn = last_said(prompt);
    turn.parts.push(Part::Text(
        "[The PDF documents of the record, as the dossier lists them.]".to_string(),
    ));
    for (i, (filename, data)) in pdfs.into_iter().enumerate() {
        turn.parts.push(Part::Pdf {
            name: format!("P{}. {filename}", i + 1),
            data,
        });
    }
}

/// The person's photographs, read and sized, joined to the member's message
/// being answered. Each is named by its number and file name, as the
/// dossier lists them.
pub fn with_photos(prompt: &mut ChatPrompt, photos: Vec<(String, Vec<u8>)>) {
    if photos.is_empty() {
        return;
    }
    let turn = last_said(prompt);
    turn.parts.push(Part::Text(
        "[The photographs of the record, as the dossier lists them.]".to_string(),
    ));
    for (i, (filename, data)) in photos.into_iter().enumerate() {
        turn.parts.push(Part::Image {
            mime: "image/jpeg".to_string(),
            name: format!("{}. {filename}", i + 1),
            data,
        });
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
         line each. The photographs and the PDF documents of the person's record, when it has \
         some, come with the member's message. It is all you know of this family: there is \
         no other record to look up.\n\
         - Look at the photographs: what they show (faces, clothes, uniforms and insignia, \
         places, objects, handwriting) may answer a question. Say which photograph, and how \
         sure you are of what you see in it.\n\
         - Read the PDF documents (deeds, registers, letters, certificates): say which \
         document and which page, and how sure you are of a hand you read.\n\
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
            photos: Vec::new(),
            pdfs: Vec::new(),
        }
    }

    fn pdf(i: usize, size: u64) -> Pdf {
        Pdf {
            id: format!("f{i}"),
            filename: format!("acte-{i}.pdf"),
            kind: "Acte".into(),
            caption: None,
            note: None,
            path: format!("documents/f{i}.pdf"),
            sha256: format!("s{i}"),
            size,
            pages: 0,
        }
    }

    #[test]
    fn the_pdfs_given_stay_within_sixty_pages_and_twenty_megabytes() {
        let pages = |p: &Pdf| match p.id.as_str() {
            "f0" => Some(30),
            "f1" => Some(40), // past sixty with the first: left out
            "f2" => Some(20),
            "f3" => None, // unreadable
            "f4" => Some(5),
            _ => Some(1),
        };
        let mb = 1024 * 1024;
        let all = vec![
            pdf(0, mb),
            pdf(1, mb),
            pdf(2, mb),
            pdf(3, mb),
            pdf(4, 19 * mb),
        ];
        let (given, left) = pdfs_within(all, &pages);
        let ids: Vec<&str> = given.iter().map(|p| p.id.as_str()).collect();
        // The fifth fits in pages (55) but not in bytes (21 MB).
        assert_eq!(ids, ["f0", "f2"]);
        assert_eq!(left, 3);
        assert_eq!(given[0].pages, 30);

        let mut g = gathered(0, 1);
        g.pdfs = vec![pdf(0, mb), pdf(3, mb)];
        let d = fill(g, &|_| None, &pages);
        let text = d.text();
        assert!(
            text.contains("P1. \"acte-0.pdf\" (Acte, 30 pages)"),
            "{text}"
        );
        assert!(text.contains("[1 more PDF documents"));
        assert_eq!(d.pdf_pages(), 30);
        assert_eq!(d.tokens(), text.chars().count() / 3 + 30 * PDF_PAGE_TOKENS);
    }

    #[test]
    fn the_pdfs_go_with_the_question_named_as_the_dossier_lists_them() {
        let mut prompt = ChatPrompt::default();
        prompt.turns.push(Turn {
            speaker: Speaker::User,
            parts: vec![Part::Text("Que dit l'acte ?".into())],
        });
        with_pdfs(&mut prompt, vec![("acte.pdf".into(), b"%PDF-1.4".to_vec())]);
        let parts = &prompt.turns[0].parts;
        assert!(
            matches!(&parts[2], Part::Pdf { name, data } if name == "P1. acte.pdf" && data.starts_with(b"%PDF"))
        );
        let mut instead = ChatPrompt::default();
        files_refused(&mut instead, true, true);
        assert!(matches!(&instead.turns[0].parts[0],
            Part::Text(t) if t.contains("photographs and the PDF documents")));
    }

    fn photo(i: usize) -> Photo {
        Photo {
            id: format!("d{i}"),
            filename: format!("photo-{i}.jpg"),
            kind: "Photographie".into(),
            caption: (i == 0).then(|| "Jean en uniforme, 1916".to_string()),
            note: None,
            path: format!("documents/d{i}.jpg"),
            sha256: String::new(),
        }
    }

    #[test]
    fn the_dossier_keeps_to_its_ceilings_and_says_what_it_left_out() {
        let long = "régiment ".repeat(10_000);
        let d = fill(gathered(3, 10_000), &|_| Some(long.clone()), &|_| None);
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
        let d = fill(
            gathered(2, 1),
            &|p| (p == "n1").then(|| "Verdun, 1916.".to_string()),
            &|_| None,
        );
        assert_eq!(
            d.notes,
            vec![("Note 1".to_string(), "Verdun, 1916.".to_string())]
        );
        assert!(!d.text().contains("more are not listed"));
    }

    #[test]
    fn the_photographs_are_listed_within_their_ceiling_and_counted() {
        let mut g = gathered(0, 1);
        g.photos = (0..PHOTOS + 3).map(photo).collect();
        let d = fill(g, &|_| None, &|_| None);
        assert_eq!(d.photos.len(), PHOTOS);
        assert_eq!(d.photos_left, 3);
        let text = d.text();
        assert!(text.contains("## The photographs of Jean Martin"));
        assert!(text.contains("1. \"photo-0.jpg\" (Photographie) — Jean en uniforme, 1916\n"));
        assert!(text.contains(&format!("{PHOTOS}. \"photo-{}.jpg\"", PHOTOS - 1)));
        assert!(text.contains("[3 more photographs of the record are not given"));
        assert_eq!(d.tokens(), text.chars().count() / 3 + PHOTOS * PHOTO_TOKENS);
        // None: no word of them.
        let d = fill(gathered(0, 1), &|_| None, &|_| None);
        assert!(!d.text().contains("photographs"));
    }

    #[test]
    fn the_photographs_go_with_the_message_being_answered() {
        let said = |speaker, s: &str| Turn {
            speaker,
            parts: vec![Part::Text(s.into())],
        };
        let mut p = ChatPrompt {
            turns: vec![
                said(Speaker::User, "Qui est-ce ?"),
                said(Speaker::Assistant, "Jean."),
                said(Speaker::User, "Et son uniforme ?"),
            ],
            ..ChatPrompt::default()
        };
        with_photos(
            &mut p,
            vec![("a.jpg".into(), vec![1]), ("b.jpg".into(), vec![2])],
        );
        assert_eq!(p.turns.len(), 3);
        assert_eq!(p.turns[0].parts.len(), 1, "not with the earlier ones");
        let last = &p.turns[2].parts;
        assert_eq!(last[0], Part::Text("Et son uniforme ?".into()));
        assert!(matches!(&last[2], Part::Image { name, mime, data }
            if name == "1. a.jpg" && mime == "image/jpeg" && data == &[1]));
        assert!(matches!(&last[3], Part::Image { name, .. } if name == "2. b.jpg"));
        // Nothing to give: nothing changes.
        let before = p.clone();
        with_photos(&mut p, Vec::new());
        assert_eq!(p, before);
        // Refused: the AFG is told, with the message being answered.
        let mut refused = before.clone();
        files_refused(&mut refused, true, false);
        assert_eq!(refused.turns.len(), 3);
        assert!(matches!(refused.turns[2].parts.last(), Some(Part::Text(s))
            if s.contains("could not be given")));
    }

    /// Jean's record and its files: a photograph, one a health record
    /// refers to, a film, a PDF and a copy an AI restored.
    fn jean_with_files() -> Value {
        let doc = |id: &str, name: &str, mime: &str, tags: &[&str]| {
            json!({"id": id, "type": "document", "filename": name, "mime_type": mime,
                   "status": "present", "document_type": "photo", "tags": tags,
                   "file": {"path": format!("documents/{id}"), "sha256": format!("sha-{id}")}})
        };
        let attached: Vec<Value> = ["d1", "d2", "d3", "d4", "d5"]
            .iter()
            .map(|d| json!({"document_id": d}))
            .collect();
        json!({
            "persons": {"p1": {
                "id": "p1", "type": "person",
                "identity": {"name": {"display": "Jean Martin", "components": []},
                             "is_living": false, "visibility": "public"},
                "documents": attached,
                "medical_scan": {"document_id": "d2"}
            }},
            "families": {}, "events": {}, "links": {}, "occupations": {}, "places": {},
            "sources": {},
            "documents": {
                "d1": doc("d1", "uniforme.jpg", "image/jpeg", &[]),
                "d2": doc("d2", "radio.jpg", "image/jpeg", &[]),
                "d3": doc("d3", "mariage.mp4", "video/mp4", &[]),
                "d4": doc("d4", "acte.pdf", "application/pdf", &[]),
                "d5": doc("d5", "uniforme-restaure.jpg", "image/jpeg",
                          &[crate::afg_record::RETOUCH_TAG]),
            }
        })
    }

    #[test]
    fn the_photographs_are_the_records_own_pictures_and_never_its_films() {
        use crate::sensitive::Scopes;
        let flat = jean_with_files();
        let lens = crate::access::Lens::unrestricted();
        let names = |scopes| -> Vec<String> {
            gather(&flat, &lens, "p1", "fr", scopes)
                .unwrap()
                .photos
                .into_iter()
                .map(|p| p.filename)
                .collect()
        };
        // The health record's picture only when the AFG room lets sensitive
        // data go; never the film, the PDF or the AI's copy.
        assert_eq!(names(Scopes::NONE), ["uniforme.jpg"]);
        assert_eq!(names(Scopes::EVERY), ["uniforme.jpg", "radio.jpg"]);
        let g = gather(&flat, &lens, "p1", "fr", Scopes::NONE).unwrap();
        assert_eq!(g.photos[0].path, "documents/d1");
        assert_eq!(g.photos[0].sha256, "sha-d1");
    }

    #[test]
    fn only_the_pictures_a_model_reads_are_photographs() {
        for mime in [
            "image/jpeg",
            "image/png",
            "image/webp",
            "image/gif",
            "image/bmp",
        ] {
            assert!(readable_photo(mime), "{mime}");
        }
        for mime in [
            "video/mp4",
            "audio/mpeg",
            "application/pdf",
            "image/tiff",
            "image/svg+xml",
        ] {
            assert!(!readable_photo(mime), "{mime}");
        }
    }

    #[test]
    fn the_instructions_name_the_person_and_the_member() {
        let d = fill(gathered(0, 0), &|_| None, &|_| None);
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
        assert!(s.contains("Look at the photographs"));
        assert!(d.text().contains("No note about this person."));
    }
}
