//! Notes and contributions (v0.5.0, docs/adr/0014).
//!
//! What the family writes about itself: a memory, a biography, the story of
//! a photograph, the state of a search. Administrators and contributors write
//! them from the administration (`/admin/notes`); the family's members read
//! them — on the record of each person a note is about, and all together on
//! the family's page, where they can be searched, read by letter, opened in a
//! window and taken away as a PDF.
//!
//! # What is stored
//!
//! An ordinary AXGF Document, so a note travels with the archive and any
//! reader of the format can open it, marked three ways, as a reworked
//! portrait is (`crate::ai_portrait`):
//!
//! * its text is the Document's file, Markdown (`text/markdown`), at
//!   `notes/<id>-<version>.md` — a new file per version, so an edit replaces
//!   the bytes in the same write that replaces the record;
//! * its title is the Document's `caption`, its date the Document's `date`
//!   (an `axgf_date`), its author `created_by`;
//! * it is linked to each person it is about (`linked_to`, role [`ROLE`]); a
//!   note linked to nobody is the family's;
//! * it is tagged [`TAG`], and `extensions[`[`EXT_KEY`]`]` holds what the
//!   format has no field for: the category, a summary to list it by without
//!   opening its file, and the pictures it shows.
//!
//! A picture in a note is a Document of its own (`photo`, its bytes in the
//! archive), tagged [`IMAGE_TAG`] and marked `extensions[`[`IMAGE_EXT_KEY`]`]`
//! with the note it belongs to. The note's text names it `axgf:document/<id>`,
//! which this module turns into the address the site serves it at; another
//! reader of the format finds the same id in the archive.
//!
//! # Who reads them
//!
//! The family's members: a reader who is signed in, never a visitor. A note
//! about people is read only by somebody who may see every one of them — a
//! note about a living cousin is the cousin's business before it is the
//! note's. Its pictures are served under the same rule
//! (`routes::public::stored_document`). Notes and their pictures are never
//! among a record's files, its portrait or its gallery.

use std::collections::{BTreeMap, HashMap};
use std::sync::Mutex;
use std::time::{Duration, Instant};

use serde::Serialize;
use serde_json::{json, Value};

/// Namespaced key for what this application records about a note.
pub const EXT_KEY: &str = "axgf-cms:note/v1";
/// The tag every note carries, for any reader of the format.
pub const TAG: &str = "note";
/// The role a note is linked to each person it is about with.
pub const ROLE: &str = "note_subject";
/// Namespaced key on a picture a note shows: `{"note": <id>}`.
pub const IMAGE_EXT_KEY: &str = "axgf-cms:note-image/v1";
/// The tag a note's picture carries.
pub const IMAGE_TAG: &str = "note-image";
/// How a note's text names one of its pictures.
pub const DOCUMENT_REF: &str = "axgf:document/";
/// How a note being written names a picture not saved yet.
pub const PENDING_REF: &str = "axgf:pending/";

/// The longest title, and the longest text, a note may have.
pub const MAX_TITLE: usize = 200;
pub const MAX_TEXT: usize = 200_000;
/// The largest picture a note may show, and how many it may hold.
pub const MAX_IMAGE: usize = 10 * 1024 * 1024;
pub const MAX_IMAGES: usize = 40;

fn has_tag(doc: &Value, tag: &str) -> bool {
    doc.get("tags")
        .and_then(Value::as_array)
        .is_some_and(|t| t.iter().any(|x| x.as_str() == Some(tag)))
}

/// Whether a stored Document is a note. Any one mark is enough, as for a
/// reworked portrait: a bundle edited elsewhere may have kept one and lost
/// another, and a note must not slip into a gallery as a file.
pub fn is_note(doc: &Value) -> bool {
    doc.get("extensions").and_then(|e| e.get(EXT_KEY)).is_some()
        || has_tag(doc, TAG)
        || doc
            .get("linked_to")
            .and_then(Value::as_array)
            .is_some_and(|l| {
                l.iter()
                    .any(|x| x.get("role").and_then(Value::as_str) == Some(ROLE))
            })
}

/// Whether a stored Document is a picture a note shows.
pub fn is_note_image(doc: &Value) -> bool {
    doc.get("extensions")
        .and_then(|e| e.get(IMAGE_EXT_KEY))
        .is_some()
        || has_tag(doc, IMAGE_TAG)
}

/// A note, or one of its pictures: never one of a record's files.
pub fn is_note_part(doc: &Value) -> bool {
    is_note(doc) || is_note_image(doc)
}

/// Whether the Document with this id is a note or a note's picture.
pub fn is_note_part_id(flat: &Value, document_id: &str) -> bool {
    flat.get("documents")
        .and_then(|d| d.get(document_id))
        .is_some_and(is_note_part)
}

fn ext(doc: &Value) -> Option<&Value> {
    doc.get("extensions").and_then(|e| e.get(EXT_KEY))
}

fn ext_str<'a>(doc: &'a Value, key: &str) -> Option<&'a str> {
    ext(doc)
        .and_then(|e| e.get(key))
        .and_then(Value::as_str)
        .filter(|s| !s.trim().is_empty())
}

/// The note's title.
pub fn title(doc: &Value) -> String {
    doc.get("caption")
        .and_then(Value::as_str)
        .filter(|s| !s.trim().is_empty())
        .or_else(|| doc.get("filename").and_then(Value::as_str))
        .unwrap_or("—")
        .to_string()
}

/// The note's category, as its writer named it.
pub fn category(doc: &Value) -> Option<String> {
    ext_str(doc, "category").map(str::to_string)
}

/// Who wrote it.
pub fn author(doc: &Value) -> Option<String> {
    doc.get("created_by")
        .and_then(Value::as_str)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

/// The people a note is about, in the order its writer gave them.
pub fn persons(doc: &Value) -> Vec<String> {
    doc.get("linked_to")
        .and_then(Value::as_array)
        .map(|l| {
            l.iter()
                .filter(|x| x.get("entity_type").and_then(Value::as_str) == Some("person"))
                .filter_map(|x| x.get("entity_id").and_then(Value::as_str))
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default()
}

/// The pictures a note shows, by id.
pub fn images(doc: &Value) -> Vec<String> {
    ext(doc)
        .and_then(|e| e.get("images"))
        .and_then(Value::as_array)
        .map(|a| {
            a.iter()
                .filter_map(Value::as_str)
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default()
}

/// The note a picture belongs to.
pub fn note_of_image(doc: &Value) -> Option<String> {
    doc.get("extensions")
        .and_then(|e| e.get(IMAGE_EXT_KEY))
        .and_then(|e| e.get("note"))
        .and_then(Value::as_str)
        .map(str::to_string)
}

/// Where the note's text is, inside the archive.
pub fn text_path(doc: &Value) -> Option<String> {
    doc.pointer("/file/path")
        .and_then(Value::as_str)
        .map(str::to_string)
}

/// Whether this reader may read this note: signed in, and able to see every
/// person it is about that the archive holds.
pub fn readable(flat: &Value, lens: &crate::access::Lens, signed_in: bool, doc: &Value) -> bool {
    if !signed_in || !is_note(doc) {
        return false;
    }
    let held = flat.get("persons").and_then(Value::as_object);
    persons(doc)
        .iter()
        .filter(|p| held.is_some_and(|h| h.contains_key(p.as_str())))
        .all(|p| lens.sees_person(p))
}

/// Whether this reader may read the note, or the note's picture, this
/// Document is.
pub fn part_readable(
    flat: &Value,
    lens: &crate::access::Lens,
    signed_in: bool,
    doc: &Value,
) -> bool {
    if is_note(doc) {
        return readable(flat, lens, signed_in, doc);
    }
    let Some(note) = note_of_image(doc) else {
        // A picture that has lost its note is nobody's: refused.
        return false;
    };
    flat.get("documents")
        .and_then(|d| d.get(&note))
        .is_some_and(|n| readable(flat, lens, signed_in, n))
}

/// The family's name, as the archive's manifest gives it.
pub fn family_name(flat: &Value) -> Option<String> {
    flat.pointer("/manifest/family/name")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

// ---------------------------------------------------------------------------
// what a page shows
// ---------------------------------------------------------------------------

/// A person a note is about, as a page names them.
#[derive(Debug, Clone, Serialize)]
pub struct NotePerson {
    pub id: String,
    pub name: String,
}

/// A note as a list shows it.
#[derive(Debug, Clone, Serialize)]
pub struct NoteView {
    pub id: String,
    pub title: String,
    pub category: Option<String>,
    pub date: Option<crate::view::DateDisplay>,
    /// The date as stored, for ordering: newest first.
    pub date_key: String,
    pub author: Option<String>,
    pub persons: Vec<NotePerson>,
    /// The first words of the text.
    pub summary: String,
    /// The letter its title is filed under: A to Z, or `#`.
    pub letter: String,
    /// Its first picture, small.
    pub thumb: Option<String>,
    /// What the list's filter matches: title, category, people, author and
    /// summary, in lower case.
    pub haystack: String,
}

/// A note as its window shows it: the list's view and its text.
#[derive(Debug, Clone, Serialize)]
pub struct NoteFull {
    #[serde(flatten)]
    pub view: NoteView,
    pub html: String,
    pub markdown: String,
    pub updated_at: Option<String>,
}

/// The letter a title is filed under: its first letter without its accent,
/// A to Z; anything else under `#`.
pub fn letter_of(title: &str) -> String {
    use unicode_normalization::UnicodeNormalization;
    match title.trim().nfd().find(|c| c.is_alphanumeric()) {
        Some(c) if c.is_ascii_alphabetic() => c.to_ascii_uppercase().to_string(),
        // A letter with a case of its own (Ж, Ω, Ł) files under itself; a
        // script without case, a digit or a sign, under `#`.
        Some(c) if c.is_alphabetic() && c.to_uppercase().ne(c.to_lowercase()) => {
            c.to_uppercase().collect()
        }
        _ => "#".to_string(),
    }
}

/// A list's view of one note.
pub fn view_of(flat: &Value, id: &str, doc: &Value, lang: &str) -> NoteView {
    let title = title(doc);
    let category = category(doc);
    // The account, as the journal names it: the emergency token in words.
    let author = author(doc).map(|a| crate::journal::who_in(&a, lang));
    let held = flat.get("persons");
    let persons: Vec<NotePerson> = persons(doc)
        .into_iter()
        .filter_map(|pid| {
            let p = held?.get(&pid)?;
            Some(NotePerson {
                name: crate::view::person_display_name(p),
                id: pid,
            })
        })
        .collect();
    let summary = ext_str(doc, "summary").unwrap_or_default().to_string();
    let date = doc
        .get("date")
        .filter(|d| d.get("value").is_some())
        .map(|d| crate::view::render_date_field_in(&json!({ "date": d }), "date", lang));
    let date_key = doc
        .pointer("/date/value")
        .and_then(Value::as_str)
        .or_else(|| doc.get("created_at").and_then(Value::as_str))
        .unwrap_or_default()
        .to_string();
    let thumb = images(doc)
        .into_iter()
        .find(|i| {
            flat.get("documents")
                .and_then(|d| d.get(i))
                .is_some_and(|d| d.pointer("/file/path").is_some())
        })
        .map(|i| format!("/document/{i}/thumb"));
    let mut haystack = String::new();
    for part in [
        Some(title.as_str()),
        category.as_deref(),
        author.as_deref(),
        Some(summary.as_str()),
    ]
    .into_iter()
    .flatten()
    {
        haystack.push_str(part);
        haystack.push(' ');
    }
    for p in &persons {
        haystack.push_str(&p.name);
        haystack.push(' ');
    }
    NoteView {
        id: id.to_string(),
        letter: letter_of(&title),
        title,
        category,
        date,
        date_key,
        author,
        persons,
        summary,
        thumb,
        haystack: fold(&haystack),
    }
}

/// Lower case and without accents, for matching: "Éléonore" finds
/// "eleonore".
pub fn fold(s: &str) -> String {
    use unicode_normalization::UnicodeNormalization;
    s.nfd()
        .filter(|c| !('\u{300}'..='\u{36f}').contains(c))
        .flat_map(char::to_lowercase)
        .collect()
}

/// Every note this reader may read, newest first — the date it bears, then
/// the date it was written — and, among notes of one date, by title.
pub fn readable_notes<'a>(
    flat: &'a Value,
    lens: &crate::access::Lens,
    signed_in: bool,
) -> Vec<(&'a str, &'a Value)> {
    let Some(docs) = flat.get("documents").and_then(Value::as_object) else {
        return Vec::new();
    };
    let mut out: Vec<(&str, &Value)> = docs
        .iter()
        .filter(|(_, d)| readable(flat, lens, signed_in, d))
        .map(|(id, d)| (id.as_str(), d))
        .collect();
    let key = |d: &Value| {
        d.pointer("/date/value")
            .and_then(Value::as_str)
            .or_else(|| d.get("created_at").and_then(Value::as_str))
            .unwrap_or_default()
            .to_string()
    };
    out.sort_by(|a, b| {
        key(b.1)
            .cmp(&key(a.1))
            .then_with(|| title(a.1).cmp(&title(b.1)))
    });
    out
}

/// The notes about one person this reader may read.
pub fn notes_about<'a>(
    flat: &'a Value,
    lens: &crate::access::Lens,
    signed_in: bool,
    person_id: &str,
) -> Vec<(&'a str, &'a Value)> {
    readable_notes(flat, lens, signed_in)
        .into_iter()
        .filter(|(_, d)| persons(d).iter().any(|p| p == person_id))
        .collect()
}

// ---------------------------------------------------------------------------
// writing one
// ---------------------------------------------------------------------------

/// What the writing form sends.
#[derive(Debug, Clone, Default)]
pub struct Draft {
    pub title: String,
    pub category: String,
    /// As typed: `1950`, `1950-06` or `1950-06-14`, or empty.
    pub date: String,
    pub persons: Vec<String>,
    pub markdown: String,
}

/// Why a draft cannot be saved, as a message key.
pub fn refusal(d: &Draft) -> Option<&'static str> {
    if d.title.trim().is_empty() {
        return Some("notes-error-title");
    }
    if d.title.chars().count() > MAX_TITLE {
        return Some("notes-error-title-long");
    }
    if d.markdown.len() > MAX_TEXT {
        return Some("notes-error-text-long");
    }
    if !d.date.trim().is_empty() && date_of(&d.date).is_none() {
        return Some("notes-error-date");
    }
    None
}

/// An `axgf_date` from what the form's date field holds: a year, a month, a
/// day, each a real one.
pub fn date_of(input: &str) -> Option<Value> {
    let s = input.trim();
    let parts: Vec<&str> = s.split('-').collect();
    let num = |p: &str, len: usize| {
        (p.len() == len && p.bytes().all(|b| b.is_ascii_digit()))
            .then(|| p.parse::<u32>().ok())
            .flatten()
    };
    match parts.as_slice() {
        [y] => num(y, 4).map(|_| json!({"value": s, "precision": "year"})),
        [y, m] => {
            num(y, 4)?;
            let m = num(m, 2)?;
            (1..=12)
                .contains(&m)
                .then(|| json!({"value": s, "precision": "month"}))
        }
        [y, m, d] => {
            let y = num(y, 4)?;
            let m = num(m, 2)?;
            let d = num(d, 2)?;
            let days = match m {
                1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
                4 | 6 | 9 | 11 => 30,
                2 if (y % 4 == 0 && y % 100 != 0) || y % 400 == 0 => 29,
                2 => 28,
                _ => return None,
            };
            (1..=days)
                .contains(&d)
                .then(|| json!({"value": s, "precision": "exact"}))
        }
        _ => None,
    }
}

/// A file name from a title: letters and digits, the rest a hyphen.
pub fn slug(title: &str) -> String {
    use unicode_normalization::UnicodeNormalization;
    let mut out = String::new();
    for c in title.nfd().filter(|c| !('\u{300}'..='\u{36f}').contains(c)) {
        if c.is_ascii_alphanumeric() {
            out.push(c.to_ascii_lowercase());
        } else if !out.ends_with('-') && !out.is_empty() {
            out.push('-');
        }
        if out.len() >= 60 {
            break;
        }
    }
    let out = out.trim_matches('-').to_string();
    if out.is_empty() {
        "note".to_string()
    } else {
        out
    }
}

/// The path of a version of a note's text inside the archive.
pub fn text_path_for(id: &str, version: u64) -> String {
    format!("notes/{id}-{version}.md")
}

/// The path of one of a note's pictures inside the archive.
pub fn image_path_for(note_id: &str, image_id: &str, ext: &str) -> String {
    format!("notes/{note_id}/{image_id}.{ext}")
}

/// The note's Document, from a draft. `version` is the version this record
/// will have once saved (1 when it is new): the text's file is named by it.
pub fn entity(
    id: &str,
    d: &Draft,
    author: &str,
    version: u64,
    text: &[u8],
    images: &[String],
    stored: Option<&Value>,
) -> Value {
    let mut linked: Vec<Value> = Vec::new();
    for p in &d.persons {
        if !linked.iter().any(|l| l["entity_id"] == p.as_str()) {
            linked.push(json!({"entity_type": "person", "entity_id": p, "role": ROLE}));
        }
    }
    let mut ext = json!({
        "summary": summary_of(&d.markdown),
        "images": images,
    });
    if !d.category.trim().is_empty() {
        ext["category"] = Value::from(d.category.trim());
    }
    let mut doc = stored.cloned().unwrap_or_else(|| json!({}));
    let obj = doc.as_object_mut().expect("an object");
    obj.insert("id".into(), Value::from(id));
    obj.insert("type".into(), Value::from("document"));
    obj.insert("axgf_version".into(), Value::from("1.0"));
    obj.insert(
        "filename".into(),
        Value::from(format!("{}.md", slug(&d.title))),
    );
    obj.insert("mime_type".into(), Value::from("text/markdown"));
    obj.insert("document_type".into(), Value::from("other"));
    obj.insert("status".into(), Value::from("present"));
    obj.insert(
        "file".into(),
        json!({
            "path": text_path_for(id, version),
            "size_bytes": text.len(),
            "sha256": crate::documents::sha256_hex(text),
        }),
    );
    obj.insert("caption".into(), Value::from(d.title.trim()));
    match date_of(&d.date) {
        Some(date) => obj.insert("date".into(), date),
        None => obj.remove("date"),
    };
    obj.insert("linked_to".into(), Value::Array(linked));
    let mut tags: Vec<Value> = obj
        .get("tags")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    if !tags.iter().any(|t| t == TAG) {
        tags.push(Value::from(TAG));
    }
    obj.insert("tags".into(), Value::Array(tags));
    if stored.is_none() {
        obj.insert("created_by".into(), Value::from(author));
    }
    let extensions = obj.entry("extensions").or_insert_with(|| json!({}));
    if let Some(e) = extensions.as_object_mut() {
        e.insert(EXT_KEY.into(), ext);
    }
    doc
}

/// A picture's Document.
pub fn image_entity(note_id: &str, image_id: &str, img: &PendingImage) -> Value {
    json!({
        "id": image_id,
        "type": "document",
        "axgf_version": "1.0",
        "filename": img.filename,
        "mime_type": img.mime,
        "document_type": "photo",
        "status": "present",
        "file": {
            "path": image_path_for(note_id, image_id, img.ext),
            "size_bytes": img.bytes.len(),
            "sha256": crate::documents::sha256_hex(&img.bytes),
        },
        "tags": [IMAGE_TAG],
        "extensions": { IMAGE_EXT_KEY: { "note": note_id } },
    })
}

/// The `axgf:` references a text names, of one kind (`DOCUMENT_REF` or
/// `PENDING_REF`), in order, each once.
pub fn refs(markdown: &str, kind: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut rest = markdown;
    while let Some(at) = rest.find(kind) {
        let tail = &rest[at + kind.len()..];
        let id: String = tail
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '-')
            .collect();
        if !id.is_empty() && !out.contains(&id) {
            out.push(id.clone());
        }
        rest = &tail[id.len()..];
    }
    out
}

// ---------------------------------------------------------------------------
// the text
// ---------------------------------------------------------------------------

/// Where a link may go: the web, a mail, this site. Nothing that runs.
fn safe_link(dest: &str) -> bool {
    let d = dest.trim();
    let lower = d.to_ascii_lowercase();
    lower.starts_with("https://")
        || lower.starts_with("http://")
        || lower.starts_with("mailto:")
        || (d.starts_with('/') && !d.starts_with("//"))
        || d.starts_with('#')
}

/// The note's text as HTML: Markdown, with what is raw HTML in it shown as
/// text, links only to the web, a mail or this site, headings set under the
/// page's own, and pictures only those `resolve` gives an address for —
/// the note's own (`axgf:document/…`), or, while writing, the ones pasted
/// (`axgf:pending/…`). A picture from elsewhere is its description.
pub fn to_html(markdown: &str, resolve: &dyn Fn(&str) -> Option<String>) -> String {
    use pulldown_cmark::{CowStr, Event, HeadingLevel, Options, Parser, Tag, TagEnd};
    let parser = Parser::new_ext(
        markdown,
        Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH,
    );
    // Links and pictures dropped, so that their ends are dropped too.
    let mut dropped: Vec<bool> = Vec::new();
    let shift = |l: HeadingLevel| match l {
        HeadingLevel::H1 => HeadingLevel::H3,
        HeadingLevel::H2 => HeadingLevel::H4,
        HeadingLevel::H3 => HeadingLevel::H5,
        _ => HeadingLevel::H6,
    };
    let events = parser.filter_map(|e| match e {
        Event::Html(h) | Event::InlineHtml(h) => Some(Event::Text(h)),
        Event::Start(Tag::Heading { level, .. }) => Some(Event::Start(Tag::Heading {
            level: shift(level),
            id: None,
            classes: Vec::new(),
            attrs: Vec::new(),
        })),
        Event::End(TagEnd::Heading(level)) => Some(Event::End(TagEnd::Heading(shift(level)))),
        Event::Start(Tag::Link {
            link_type,
            dest_url,
            title,
            id,
        }) => {
            let ok = safe_link(&dest_url);
            dropped.push(!ok);
            ok.then_some(Event::Start(Tag::Link {
                link_type,
                dest_url,
                title,
                id,
            }))
        }
        Event::End(TagEnd::Link) => {
            if dropped.pop().unwrap_or(false) {
                None
            } else {
                Some(Event::End(TagEnd::Link))
            }
        }
        Event::Start(Tag::Image {
            link_type,
            dest_url,
            title,
            id,
        }) => match resolve(&dest_url) {
            Some(url) => {
                dropped.push(false);
                Some(Event::Start(Tag::Image {
                    link_type,
                    dest_url: CowStr::from(url),
                    title,
                    id,
                }))
            }
            None => {
                dropped.push(true);
                None
            }
        },
        Event::End(TagEnd::Image) => {
            if dropped.pop().unwrap_or(false) {
                None
            } else {
                Some(Event::End(TagEnd::Image))
            }
        }
        other => Some(other),
    });
    let mut out = String::new();
    pulldown_cmark::html::push_html(&mut out, events);
    out
}

/// The site's address for one of a note's pictures, or a pasted one.
pub fn resolve_for_page(dest: &str) -> Option<String> {
    if let Some(id) = dest.strip_prefix(DOCUMENT_REF) {
        let id: String = id
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '-')
            .collect();
        return (!id.is_empty()).then(|| format!("/document/{id}/view"));
    }
    if let Some(t) = dest.strip_prefix(PENDING_REF) {
        let t: String = t
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric())
            .collect();
        return (!t.is_empty()).then(|| format!("/admin/notes/pending/{t}"));
    }
    None
}

/// The text without its markup, on one line. A block that does not end its
/// own sentence (a heading, a list's item) is set off from the next by a
/// middle dot, so that "Ingredients" and "six apples" do not run together.
pub fn plain_text(markdown: &str) -> String {
    use pulldown_cmark::{Event, Parser, TagEnd};
    let mut out = String::new();
    let end_block = |out: &mut String| {
        let trimmed = out.trim_end();
        let closed = trimmed.is_empty()
            || trimmed.ends_with([
                '.', ',', '!', '?', ':', ';', '…', '»', '"', '”', '·', '—', '。', '，', '！', '？',
            ]);
        out.truncate(trimmed.len());
        out.push_str(if closed { " " } else { " · " });
    };
    for e in Parser::new(markdown) {
        match e {
            Event::Text(t) | Event::Code(t) => out.push_str(&t),
            Event::SoftBreak | Event::HardBreak => out.push(' '),
            Event::End(
                TagEnd::Paragraph | TagEnd::Heading(_) | TagEnd::Item | TagEnd::BlockQuote(_),
            ) => end_block(&mut out),
            _ => {}
        }
    }
    let text = out.split_whitespace().collect::<Vec<_>>().join(" ");
    text.trim_end_matches(" ·")
        .trim_end_matches('·')
        .trim_end()
        .to_string()
}

/// The first words of a text, to list it by: some 280 characters, cut at a
/// word.
pub fn summary_of(markdown: &str) -> String {
    let text = plain_text(markdown);
    if text.chars().count() <= 280 {
        return text;
    }
    let mut cut: String = text.chars().take(280).collect();
    if let Some(at) = cut.rfind(' ') {
        cut.truncate(at);
    }
    cut.push('…');
    cut
}

// ---------------------------------------------------------------------------
// pictures pasted while writing
// ---------------------------------------------------------------------------

/// A picture pasted or chosen while a note is written, held until the note
/// is saved: only then does it go into the archive, so a note abandoned
/// leaves nothing behind in the family's file.
#[derive(Debug, Clone)]
pub struct PendingImage {
    pub filename: String,
    pub mime: &'static str,
    pub ext: &'static str,
    pub bytes: Vec<u8>,
    /// Who pasted it: only they may use it.
    pub owner: String,
    created: Instant,
}

/// The pictures waiting, by token.
#[derive(Default)]
pub struct PendingImages {
    inner: Mutex<BTreeMap<String, PendingImage>>,
}

/// How long a pasted picture waits for its note.
const PENDING_TTL: Duration = Duration::from_secs(6 * 60 * 60);
/// How many bytes of pictures may wait, all writers together.
const PENDING_BUDGET: usize = 200 * 1024 * 1024;

/// What a picture is, from its first bytes: the formats a page and a PDF can
/// both show. Anything else is refused.
pub fn image_kind(bytes: &[u8]) -> Option<(&'static str, &'static str)> {
    if bytes.starts_with(b"\x89PNG\r\n\x1a\n") {
        Some(("image/png", "png"))
    } else if bytes.starts_with(b"\xFF\xD8\xFF") {
        Some(("image/jpeg", "jpg"))
    } else if bytes.starts_with(b"GIF87a") || bytes.starts_with(b"GIF89a") {
        Some(("image/gif", "gif"))
    } else if bytes.len() > 12 && &bytes[..4] == b"RIFF" && &bytes[8..12] == b"WEBP" {
        Some(("image/webp", "webp"))
    } else {
        None
    }
}

impl PendingImages {
    /// Hold a picture; its token, or `None` when it is not a picture or
    /// there is no room.
    pub fn put(&self, owner: &str, filename: &str, bytes: Vec<u8>) -> Option<String> {
        let (mime, ext) = image_kind(&bytes)?;
        if bytes.len() > MAX_IMAGE {
            return None;
        }
        let mut guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        guard.retain(|_, i| i.created.elapsed() < PENDING_TTL);
        while guard.values().map(|i| i.bytes.len()).sum::<usize>() + bytes.len() > PENDING_BUDGET {
            let oldest = guard
                .iter()
                .min_by_key(|(_, i)| i.created)
                .map(|(k, _)| k.clone())?;
            guard.remove(&oldest);
        }
        let token = uuid::Uuid::new_v4().simple().to_string();
        let name: String = filename
            .rsplit(['/', '\\'])
            .next()
            .unwrap_or("image")
            .chars()
            .filter(|c| !c.is_control() && *c != '"')
            .take(120)
            .collect();
        guard.insert(
            token.clone(),
            PendingImage {
                filename: if name.trim().is_empty() {
                    format!("image.{ext}")
                } else {
                    name
                },
                mime,
                ext,
                bytes,
                owner: owner.to_string(),
                created: Instant::now(),
            },
        );
        Some(token)
    }

    /// A waiting picture, for the one who pasted it.
    pub fn get(&self, owner: &str, token: &str) -> Option<PendingImage> {
        let guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        guard
            .get(token)
            .filter(|i| i.owner == owner && i.created.elapsed() < PENDING_TTL)
            .cloned()
    }

    /// The pictures a saved note took, gone from the waiting room.
    pub fn take(&self, owner: &str, tokens: &[String]) -> HashMap<String, PendingImage> {
        let mut guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        let mut out = HashMap::new();
        for t in tokens {
            if guard.get(t).is_some_and(|i| i.owner == owner) {
                if let Some(i) = guard.remove(t) {
                    out.insert(t.clone(), i);
                }
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn page(md: &str) -> String {
        to_html(md, &resolve_for_page)
    }

    #[test]
    fn raw_html_is_shown_as_text_never_run() {
        let h = page("Avant <script>alert(1)</script> après\n\n<img src=x onerror=alert(2)>");
        assert!(!h.contains("<script>") && !h.contains("<img src=x"), "{h}");
        assert!(h.contains("&lt;script&gt;"), "{h}");
    }

    #[test]
    fn links_go_to_the_web_a_mail_or_this_site_only() {
        let h = page("[a](javascript:alert(1)) [b](https://example.org) [c](/person/x) [d](data:text/html,x) [e](//evil.example)");
        assert!(
            !h.contains("javascript:") && !h.contains("data:") && !h.contains("//evil"),
            "{h}"
        );
        assert!(h.contains(r#"href="https://example.org""#) && h.contains(r#"href="/person/x""#));
        assert!(h.contains('a') && h.contains('d'), "the words stay: {h}");
    }

    #[test]
    fn pictures_are_the_notes_own_or_their_description() {
        let id = "3f0c1a2b-0000-4000-8000-000000000001";
        let h = page(&format!(
            "![Portrait](axgf:document/{id}) ![ailleurs](https://tracker.example/p.gif)"
        ));
        assert!(h.contains(&format!(r#"src="/document/{id}/view""#)), "{h}");
        assert!(
            !h.contains("tracker.example") && h.contains("ailleurs"),
            "{h}"
        );
    }

    #[test]
    fn headings_sit_under_the_pages_own() {
        let h = page("# Un\n\n## Deux");
        assert!(
            h.contains("<h3>Un</h3>") && h.contains("<h4>Deux</h4>"),
            "{h}"
        );
    }

    #[test]
    fn a_summary_is_the_first_words_cut_at_a_word() {
        assert_eq!(
            summary_of("# Titre\n\nUn *souvenir* d'été."),
            "Titre · Un souvenir d'été."
        );
        // A heading and a list's items do not run together; a sentence that
        // ends itself needs no dot.
        assert_eq!(
            summary_of("## Ingrédients\n\n1. six pommes\n2. trois œufs\n\nCuire. Servir."),
            "Ingrédients · six pommes · trois œufs · Cuire. Servir."
        );
        assert_eq!(
            summary_of("> « Une citation. »\n\nEnsuite"),
            "« Une citation. » Ensuite"
        );
        let long = "mot ".repeat(200);
        let s = summary_of(&long);
        assert!(s.ends_with('…') && s.chars().count() <= 281, "{s}");
    }

    #[test]
    fn a_date_is_a_real_year_month_or_day() {
        assert_eq!(date_of("1950").unwrap()["precision"], "year");
        assert_eq!(date_of("1950-06").unwrap()["precision"], "month");
        assert_eq!(date_of("1952-02-29").unwrap()["precision"], "exact");
        for bad in [
            "1950-13",
            "1951-02-29",
            "1950-06-31",
            "50",
            "1950-6-1",
            "hier",
            "1950-06-14T10",
        ] {
            assert!(date_of(bad).is_none(), "{bad}");
        }
    }

    #[test]
    fn a_title_is_filed_under_its_first_letter() {
        assert_eq!(letter_of("Éléonore"), "E");
        assert_eq!(letter_of("  zinc"), "Z");
        assert_eq!(letter_of("1914"), "#");
        assert_eq!(letter_of("Łódź"), "Ł");
        assert_eq!(letter_of("жизнь"), "Ж");
        assert_eq!(letter_of("王"), "#");
    }

    #[test]
    fn references_are_read_once_each_in_order() {
        let md = "![a](axgf:pending/abc) ![b](axgf:document/1-2) ![c](axgf:pending/abc) ![d](axgf:pending/def)";
        assert_eq!(refs(md, PENDING_REF), ["abc", "def"]);
        assert_eq!(refs(md, DOCUMENT_REF), ["1-2"]);
    }

    #[test]
    fn only_pictures_wait_and_only_for_who_pasted_them() {
        let p = PendingImages::default();
        assert!(p.put("ana", "x.txt", b"hello".to_vec()).is_none());
        let t = p
            .put(
                "ana",
                "C:\\photos\\mamie.png",
                b"\x89PNG\r\n\x1a\nrest".to_vec(),
            )
            .unwrap();
        assert!(p.get("bob", &t).is_none());
        let img = p.get("ana", &t).unwrap();
        assert_eq!(
            (img.filename.as_str(), img.mime),
            ("mamie.png", "image/png")
        );
        assert!(p.take("bob", std::slice::from_ref(&t)).is_empty());
        assert_eq!(p.take("ana", std::slice::from_ref(&t)).len(), 1);
        assert!(p.get("ana", &t).is_none(), "taken once");
    }

    #[test]
    fn a_slug_is_letters_and_hyphens() {
        assert_eq!(slug("La maison d'Été, 1950 !"), "la-maison-d-ete-1950");
        assert_eq!(slug("王"), "note");
    }
}
