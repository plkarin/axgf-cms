//! My personal assistant (the operator's request of 3 November 2026,
//! docs/adr/0026): each account's own space with the AFG. Projects it
//! creates and names, each with a code of conduct the AFG follows and a
//! second brain the AFG keeps; in each project, conversations that open on a
//! first question and carry on, with documents, photographs and media as
//! well as words.
//!
//! ```text
//! family.assistant/              beside family.axgf, as family.brains is
//!   <username>/                  one per account, mode 700
//!     owner.json                 whose space this is: the account's id and name
//!     projects/<project>/
//!       project.json             its name, avatar, code of conduct, and
//!                                whether the family's archive goes with it
//!       memory.md                its second brain: what the AFG chose to keep
//!                                from its conversations, one fact a line
//!       avatar.jpg               an avatar of the account's own, when given one
//!       conversations/<c>.json   a conversation: its title, avatar, dates
//!       conversations/<c>.jsonl  its messages, one JSON object a line
//!       conversations/<c>.jpg    its own avatar, when given one
//!       files/<f>                what was sent with a message, as it was sent
//!       files/<f>.json           its name, type and size
//! ```
//!
//! A space is its account's, not its name's: one whose `owner.json` names
//! another account is set aside under `<name>@<its account>`, as a brain is
//! (docs/adr/0024). Nobody reads another account's space, administrators
//! included. It is not the account's AFG brain: forgetting that brain on the
//! account page leaves the projects as they are.

use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Instant;

use serde::{Deserialize, Serialize};

use crate::afg_chat::{ChatPrompt, Part, Speaker, Turn};
use crate::brain::{append_private, cut, folder_name, make_dir, mark, write_private, Owner};

/// One writer at a time over the spaces of this process.
static WRITING: Mutex<()> = Mutex::new(());

fn writing() -> MutexGuard<'static, ()> {
    WRITING.lock().unwrap_or_else(|e| e.into_inner())
}

/// A project's or a conversation's name, at most.
pub const NAME_MAX: usize = 80;
/// A conversation's title: its first question, cut.
pub const TITLE_MAX: usize = 120;
/// A code of conduct, at most.
pub const CONDUCT_MAX: usize = 8_000;
/// One message's words, at most.
pub const TEXT_MAX: usize = 32_000;
/// What one message may carry.
pub const FILES_PER_MESSAGE: usize = 6;
pub const FILE_MAX: usize = 20 * 1024 * 1024;
pub const MESSAGE_FILES_MAX: usize = 40 * 1024 * 1024;
/// A picture given as an avatar, before it is made one.
pub const AVATAR_MAX: usize = 8 * 1024 * 1024;
/// The edge of an avatar as kept.
const AVATAR_EDGE: u32 = 256;
pub const PROJECTS_MAX: usize = 200;
pub const CONVERSATIONS_MAX: usize = 1_000;
pub const MESSAGES_MAX: usize = 2_000;

/// The second brain, at most: past it, its oldest facts go first.
const MEMORY_MAX: usize = 64 * 1024;
/// How much of it goes back to the AFG with each message.
const RECALL_MEMORY: usize = 16 * 1024;

/// How much of a conversation goes back to the AFG: its last messages, as
/// many as fit.
const HISTORY_MESSAGES: usize = 40;
const HISTORY_CHARS: usize = 80_000;
/// The messages whose files go back whole; older ones are named only.
const FILES_RECENT: usize = 3;
/// A text file's content, at most, as the AFG reads it.
const TEXT_FILE_CHARS: usize = 100_000;
/// The longest edge of a picture as a model is given it.
const MODEL_IMAGE_EDGE: u32 = 1_568;

const OWNER: &str = "owner.json";
const PROJECTS: &str = "projects";
const PROJECT: &str = "project.json";
const MEMORY: &str = "memory.md";
const AVATAR: &str = "avatar.jpg";
const CONVERSATIONS: &str = "conversations";
const FILES: &str = "files";

/// A new id: sixteen hexadecimal digits.
pub fn new_id() -> String {
    uuid::Uuid::new_v4().simple().to_string()[..16].to_string()
}

/// Whether a string is an id, so that it may be a path's part.
pub fn is_id(s: &str) -> bool {
    s.len() == 16
        && s.bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

/// A name as kept: one line, no control characters, at most `max`.
pub fn clean_name(s: &str, max: usize) -> String {
    let one: String = s
        .chars()
        .map(|c| if c.is_control() { ' ' } else { c })
        .collect();
    let one = one.split_whitespace().collect::<Vec<_>>().join(" ");
    one.chars().take(max).collect()
}

fn now() -> String {
    crate::view::now_iso8601()
}

// ---------------------------------------------------------------------------
// what is kept
// ---------------------------------------------------------------------------

/// `owner.json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Card {
    account_id: String,
    username: String,
    created_at: String,
}

/// A project: its name, its avatar, its code of conduct.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    /// Empty for the pack's avatar its id picks, a pack avatar's name, or
    /// `own` for the picture its account gave it.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub avatar: String,
    /// What the AFG is told to keep to in this project, in its account's
    /// words.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub conduct: String,
    /// Whether the family's archive goes with each message.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub archive: bool,
    pub created_at: String,
    pub updated_at: String,
}

/// A conversation: its first question, its avatar, its dates.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Conversation {
    pub id: String,
    pub title: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub avatar: String,
    pub created_at: String,
    pub updated_at: String,
    #[serde(default)]
    pub messages: usize,
}

/// Who wrote a message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
    /// The AFG could not answer: what went wrong, said in the thread.
    Error,
}

/// What a file sent with a message is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FileKind {
    Image,
    Pdf,
    Text,
    Audio,
    Video,
    Other,
}

/// A file sent with a message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileRef {
    pub id: String,
    pub name: String,
    pub mime: String,
    pub size: u64,
    pub kind: FileKind,
}

/// One message of a conversation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub role: Role,
    pub text: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<FileRef>,
    pub at: String,
    /// The way of computing that answered, and its model.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub kind: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub model: String,
    /// What the AFG kept from this answer in the project's second brain.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub remembered: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_tokens: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_tokens: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost_usd: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seconds: Option<u64>,
}

impl Message {
    pub fn new(role: Role, text: String, files: Vec<FileRef>) -> Self {
        Message {
            id: new_id(),
            role,
            text,
            files,
            at: now(),
            kind: String::new(),
            model: String::new(),
            remembered: Vec::new(),
            input_tokens: None,
            output_tokens: None,
            cost_usd: None,
            seconds: None,
        }
    }
}

/// A fact of a project's second brain, to forget by its mark.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Fact {
    pub id: String,
    pub date: String,
    pub text: String,
}

/// A project as its list shows it.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ProjectSummary {
    #[serde(flatten)]
    pub project: Project,
    pub conversations: usize,
    pub facts: usize,
}

// ---------------------------------------------------------------------------
// files: what they are
// ---------------------------------------------------------------------------

/// What a file is, from its bytes, and the type it is served as. Its name
/// only tells a text file's flavour: bytes decide everything else.
pub fn sniff(name: &str, bytes: &[u8]) -> (FileKind, &'static str) {
    if let Ok(format) = image::guess_format(bytes) {
        match format {
            image::ImageFormat::Png => return (FileKind::Image, "image/png"),
            image::ImageFormat::Jpeg => return (FileKind::Image, "image/jpeg"),
            image::ImageFormat::Gif => return (FileKind::Image, "image/gif"),
            image::ImageFormat::WebP => return (FileKind::Image, "image/webp"),
            _ => {}
        }
    }
    if bytes.starts_with(b"%PDF-") {
        return (FileKind::Pdf, "application/pdf");
    }
    if bytes.starts_with(b"ID3")
        || bytes.starts_with(&[0xFF, 0xFB])
        || bytes.starts_with(&[0xFF, 0xF3])
    {
        return (FileKind::Audio, "audio/mpeg");
    }
    if bytes.len() > 12 && &bytes[..4] == b"RIFF" && &bytes[8..12] == b"WAVE" {
        return (FileKind::Audio, "audio/wav");
    }
    if bytes.starts_with(b"OggS") {
        return (FileKind::Audio, "audio/ogg");
    }
    if bytes.starts_with(b"fLaC") {
        return (FileKind::Audio, "audio/flac");
    }
    if bytes.len() > 12 && &bytes[4..8] == b"ftyp" {
        return match &bytes[8..12] {
            b"M4A " | b"M4B " => (FileKind::Audio, "audio/mp4"),
            b"qt  " => (FileKind::Video, "video/quicktime"),
            _ => (FileKind::Video, "video/mp4"),
        };
    }
    if bytes.starts_with(&[0x1A, 0x45, 0xDF, 0xA3]) {
        return (FileKind::Video, "video/webm");
    }
    let ext = name
        .rsplit_once('.')
        .map(|(_, e)| e.to_ascii_lowercase())
        .unwrap_or_default();
    let texty = matches!(
        ext.as_str(),
        "txt"
            | "md"
            | "markdown"
            | "csv"
            | "tsv"
            | "json"
            | "xml"
            | "html"
            | "htm"
            | "ged"
            | "log"
            | "yaml"
            | "yml"
            | "ini"
            | "rtf"
            | "srt"
            | "vtt"
    );
    if texty && !bytes.contains(&0) && std::str::from_utf8(bytes).is_ok() {
        let mime = match ext.as_str() {
            "md" | "markdown" => "text/markdown",
            "csv" => "text/csv",
            "json" => "application/json",
            "html" | "htm" => "text/html",
            "xml" => "application/xml",
            _ => "text/plain",
        };
        return (FileKind::Text, mime);
    }
    (FileKind::Other, "application/octet-stream")
}

/// A file's name as kept and shown: its last part, one line, no control
/// characters.
pub fn file_name(name: &str) -> String {
    let base = name.rsplit(['/', '\\']).next().unwrap_or(name);
    let clean = clean_name(base, NAME_MAX);
    if clean.is_empty() || clean.starts_with('.') {
        format!("file{clean}")
    } else {
        clean
    }
}

/// A picture made an avatar: oriented, cut square from its middle, at
/// `AVATAR_EDGE`, as a JPEG. `None` when the bytes are not a picture.
pub fn avatar_jpeg(bytes: &[u8]) -> Option<Vec<u8>> {
    let img = image::load_from_memory(bytes).ok()?;
    let side = img.width().min(img.height());
    if side == 0 {
        return None;
    }
    let x = (img.width() - side) / 2;
    let y = (img.height() - side) / 2;
    let square = img.crop_imm(x, y, side, side).resize_exact(
        AVATAR_EDGE,
        AVATAR_EDGE,
        image::imageops::FilterType::Lanczos3,
    );
    let rgb = image::DynamicImage::ImageRgb8(square.to_rgb8());
    let mut out = std::io::Cursor::new(Vec::new());
    image::codecs::jpeg::JpegEncoder::new_with_quality(&mut out, 88)
        .encode_image(&rgb)
        .ok()?;
    Some(out.into_inner())
}

/// A picture as a model is given it: at most `MODEL_IMAGE_EDGE` on its
/// longest side, as a JPEG unless it was small enough already.
pub fn model_image(mime: &str, bytes: &[u8]) -> (String, Vec<u8>) {
    let Ok(img) = image::load_from_memory(bytes) else {
        return (mime.to_string(), bytes.to_vec());
    };
    let small = img.width().max(img.height()) <= MODEL_IMAGE_EDGE;
    if small && bytes.len() <= 3 * 1024 * 1024 && mime != "image/gif" {
        return (mime.to_string(), bytes.to_vec());
    }
    let img = if small {
        img
    } else {
        img.resize(
            MODEL_IMAGE_EDGE,
            MODEL_IMAGE_EDGE,
            image::imageops::FilterType::Lanczos3,
        )
    };
    let rgb = image::DynamicImage::ImageRgb8(img.to_rgb8());
    let mut out = std::io::Cursor::new(Vec::new());
    match image::codecs::jpeg::JpegEncoder::new_with_quality(&mut out, 85).encode_image(&rgb) {
        Ok(()) => ("image/jpeg".to_string(), out.into_inner()),
        Err(_) => (mime.to_string(), bytes.to_vec()),
    }
}

// ---------------------------------------------------------------------------
// the spaces
// ---------------------------------------------------------------------------

/// The spaces of one archive.
#[derive(Debug, Clone)]
pub struct Spaces {
    root: PathBuf,
}

/// One account's space, its folder made.
#[derive(Debug, Clone)]
pub struct Space {
    dir: PathBuf,
    pub owner: Owner,
}

fn read_card(dir: &Path) -> Option<Card> {
    serde_json::from_str(&fs::read_to_string(dir.join(OWNER)).ok()?).ok()
}

fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> Option<T> {
    serde_json::from_str(&fs::read_to_string(path).ok()?).ok()
}

fn write_json<T: Serialize>(path: &Path, value: &T) -> io::Result<()> {
    let body = serde_json::to_vec_pretty(value).map_err(io::Error::other)?;
    write_private(path, &body)
}

impl Spaces {
    /// The spaces beside an archive: `family.assistant` for `family.axgf`.
    pub fn beside(bundle: &Path) -> Self {
        Spaces {
            root: bundle.with_extension("assistant"),
        }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    /// The account's space, made the first time it is asked for.
    pub fn of(&self, owner: &Owner) -> io::Result<Space> {
        let _w = writing();
        make_dir(&self.root)?;
        let dir = self.root.join(folder_name(&owner.name));
        if let Some(card) = read_card(&dir) {
            if card.account_id != owner.id {
                let short: String = card.account_id.chars().take(8).collect();
                let mut aside = self
                    .root
                    .join(format!("{}@{short}", folder_name(&card.username)));
                let mut n = 2;
                while aside.exists() {
                    aside = self
                        .root
                        .join(format!("{}@{short}-{n}", folder_name(&card.username)));
                    n += 1;
                }
                fs::rename(&dir, &aside)?;
                tracing::warn!(
                    folder = %dir.display(),
                    moved_to = %aside.display(),
                    "an assistant space under this name was another account's; set aside"
                );
            }
        }
        make_dir(&dir.join(PROJECTS))?;
        if read_card(&dir).is_none() {
            write_json(
                &dir.join(OWNER),
                &Card {
                    account_id: owner.id.clone(),
                    username: owner.name.clone(),
                    created_at: now(),
                },
            )?;
        }
        Ok(Space {
            dir,
            owner: owner.clone(),
        })
    }

    /// The account's space if it has one: never another account's under
    /// the same name.
    pub fn existing(&self, owner: &Owner) -> Option<Space> {
        let dir = self.root.join(folder_name(&owner.name));
        let card = read_card(&dir)?;
        (card.account_id == owner.id).then(|| Space {
            dir,
            owner: owner.clone(),
        })
    }
}

impl Space {
    pub fn dir(&self) -> &Path {
        &self.dir
    }

    fn project_dir(&self, pid: &str) -> Option<PathBuf> {
        is_id(pid).then(|| self.dir.join(PROJECTS).join(pid))
    }

    fn conversation_path(&self, pid: &str, cid: &str, ext: &str) -> Option<PathBuf> {
        if !is_id(cid) {
            return None;
        }
        Some(
            self.project_dir(pid)?
                .join(CONVERSATIONS)
                .join(format!("{cid}.{ext}")),
        )
    }

    // -- projects -----------------------------------------------------------

    /// Every project, the most recently used first.
    pub fn projects(&self) -> Vec<ProjectSummary> {
        let Ok(entries) = fs::read_dir(self.dir.join(PROJECTS)) else {
            return Vec::new();
        };
        let mut out: Vec<ProjectSummary> = entries
            .flatten()
            .filter_map(|e| read_json::<Project>(&e.path().join(PROJECT)))
            .filter(|p| is_id(&p.id))
            .map(|p| ProjectSummary {
                conversations: self.conversation_count(&p.id),
                facts: self.facts(&p.id).len(),
                project: p,
            })
            .collect();
        out.sort_by(|a, b| b.project.updated_at.cmp(&a.project.updated_at));
        out
    }

    pub fn project(&self, pid: &str) -> Option<Project> {
        read_json(&self.project_dir(pid)?.join(PROJECT))
    }

    /// A new project. `None` when the account has as many as it may.
    pub fn create_project(&self, name: &str, avatar: &str) -> io::Result<Option<Project>> {
        if self.projects().len() >= PROJECTS_MAX {
            return Ok(None);
        }
        let _w = writing();
        let id = new_id();
        let dir = self.dir.join(PROJECTS).join(&id);
        make_dir(&dir.join(CONVERSATIONS))?;
        make_dir(&dir.join(FILES))?;
        let at = now();
        let project = Project {
            id,
            name: clean_name(name, NAME_MAX),
            avatar: avatar.to_string(),
            conduct: String::new(),
            archive: false,
            created_at: at.clone(),
            updated_at: at,
        };
        write_json(&dir.join(PROJECT), &project)?;
        Ok(Some(project))
    }

    /// Write a project back, its date of use renewed.
    pub fn save_project(&self, project: &Project) -> io::Result<()> {
        let Some(dir) = self.project_dir(&project.id) else {
            return Err(io::Error::other("not a project"));
        };
        let _w = writing();
        let mut p = project.clone();
        p.updated_at = now();
        write_json(&dir.join(PROJECT), &p)
    }

    /// A project and everything in it: its conversations, its files, its
    /// second brain. Whether there was one.
    pub fn delete_project(&self, pid: &str) -> io::Result<bool> {
        let Some(dir) = self.project_dir(pid) else {
            return Ok(false);
        };
        let _w = writing();
        if !dir.join(PROJECT).is_file() {
            return Ok(false);
        }
        fs::remove_dir_all(&dir).map(|()| true)
    }

    fn touch_project(&self, pid: &str) {
        if let (Some(dir), Some(mut p)) = (self.project_dir(pid), self.project(pid)) {
            p.updated_at = now();
            let _ = write_json(&dir.join(PROJECT), &p);
        }
    }

    // -- conversations ------------------------------------------------------

    fn conversation_count(&self, pid: &str) -> usize {
        self.project_dir(pid)
            .and_then(|d| fs::read_dir(d.join(CONVERSATIONS)).ok())
            .map(|entries| {
                entries
                    .flatten()
                    .filter(|e| e.path().extension().is_some_and(|x| x == "json"))
                    .count()
            })
            .unwrap_or(0)
    }

    /// A project's conversations, the most recently used first.
    pub fn conversations(&self, pid: &str) -> Vec<Conversation> {
        let Some(dir) = self.project_dir(pid) else {
            return Vec::new();
        };
        let Ok(entries) = fs::read_dir(dir.join(CONVERSATIONS)) else {
            return Vec::new();
        };
        let mut out: Vec<Conversation> = entries
            .flatten()
            .filter(|e| e.path().extension().is_some_and(|x| x == "json"))
            .filter_map(|e| read_json::<Conversation>(&e.path()))
            .filter(|c| is_id(&c.id))
            .collect();
        out.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        out
    }

    pub fn conversation(&self, pid: &str, cid: &str) -> Option<Conversation> {
        read_json(&self.conversation_path(pid, cid, "json")?)
    }

    /// A new conversation, named after its first question. `None` when the
    /// project has as many as it may, or is none.
    pub fn create_conversation(
        &self,
        pid: &str,
        first: &str,
        avatar: &str,
    ) -> io::Result<Option<Conversation>> {
        if self.project(pid).is_none() || self.conversation_count(pid) >= CONVERSATIONS_MAX {
            return Ok(None);
        }
        let _w = writing();
        let id = new_id();
        let Some(path) = self.conversation_path(pid, &id, "json") else {
            return Ok(None);
        };
        if let Some(parent) = path.parent() {
            make_dir(parent)?;
        }
        let at = now();
        let title = match clean_name(first, TITLE_MAX) {
            t if t.is_empty() => "…".to_string(),
            t => t,
        };
        let c = Conversation {
            id,
            title,
            avatar: avatar.to_string(),
            created_at: at.clone(),
            updated_at: at,
            messages: 0,
        };
        write_json(&path, &c)?;
        Ok(Some(c))
    }

    /// Write a conversation's card back (its title, its avatar).
    pub fn save_conversation(&self, pid: &str, c: &Conversation) -> io::Result<()> {
        let Some(path) = self.conversation_path(pid, &c.id, "json") else {
            return Err(io::Error::other("not a conversation"));
        };
        let _w = writing();
        write_json(&path, c)
    }

    /// A conversation, its messages and its avatar. The files its messages
    /// carried stay in the project until it goes.
    pub fn delete_conversation(&self, pid: &str, cid: &str) -> io::Result<bool> {
        let Some(card) = self.conversation_path(pid, cid, "json") else {
            return Ok(false);
        };
        let _w = writing();
        if !card.is_file() {
            return Ok(false);
        }
        for ext in ["jsonl", "jpg"] {
            if let Some(p) = self.conversation_path(pid, cid, ext) {
                let _ = fs::remove_file(p);
            }
        }
        fs::remove_file(card).map(|()| true)
    }

    /// A conversation's messages, oldest first.
    pub fn messages(&self, pid: &str, cid: &str) -> Vec<Message> {
        let Some(path) = self.conversation_path(pid, cid, "jsonl") else {
            return Vec::new();
        };
        fs::read_to_string(path)
            .unwrap_or_default()
            .lines()
            .filter_map(|l| serde_json::from_str(l).ok())
            .collect()
    }

    /// Add a message to a conversation, and say so on its card and its
    /// project's.
    pub fn append(&self, pid: &str, cid: &str, m: &Message) -> io::Result<()> {
        let (Some(lines), Some(card)) = (
            self.conversation_path(pid, cid, "jsonl"),
            self.conversation_path(pid, cid, "json"),
        ) else {
            return Err(io::Error::other("not a conversation"));
        };
        let _w = writing();
        let mut line = serde_json::to_string(m).map_err(io::Error::other)?;
        line.push('\n');
        append_private(&lines, line.as_bytes())?;
        if let Some(mut c) = read_json::<Conversation>(&card) {
            c.messages += 1;
            c.updated_at = now();
            write_json(&card, &c)?;
        }
        self.touch_project(pid);
        Ok(())
    }

    // -- files --------------------------------------------------------------

    /// Keep a file sent with a message.
    pub fn save_file(&self, pid: &str, name: &str, bytes: &[u8]) -> io::Result<FileRef> {
        let Some(dir) = self.project_dir(pid) else {
            return Err(io::Error::other("not a project"));
        };
        let _w = writing();
        let files = dir.join(FILES);
        make_dir(&files)?;
        let name = file_name(name);
        let (kind, mime) = sniff(&name, bytes);
        let f = FileRef {
            id: new_id(),
            name,
            mime: mime.to_string(),
            size: bytes.len() as u64,
            kind,
        };
        write_private(&files.join(&f.id), bytes)?;
        write_json(&files.join(format!("{}.json", f.id)), &f)?;
        Ok(f)
    }

    /// A kept file: what it is, and its bytes.
    pub fn file(&self, pid: &str, fid: &str) -> Option<(FileRef, Vec<u8>)> {
        if !is_id(fid) {
            return None;
        }
        let files = self.project_dir(pid)?.join(FILES);
        let f: FileRef = read_json(&files.join(format!("{fid}.json")))?;
        let bytes = fs::read(files.join(fid)).ok()?;
        Some((f, bytes))
    }

    // -- avatars ------------------------------------------------------------

    fn own_avatar_path(&self, pid: &str, cid: Option<&str>) -> Option<PathBuf> {
        match cid {
            None => Some(self.project_dir(pid)?.join(AVATAR)),
            Some(cid) => self.conversation_path(pid, cid, "jpg"),
        }
    }

    /// Keep a picture as a project's avatar, or a conversation's. `false`
    /// when the bytes are not a picture.
    pub fn save_own_avatar(&self, pid: &str, cid: Option<&str>, bytes: &[u8]) -> io::Result<bool> {
        let Some(path) = self.own_avatar_path(pid, cid) else {
            return Ok(false);
        };
        let Some(jpeg) = avatar_jpeg(bytes) else {
            return Ok(false);
        };
        let _w = writing();
        write_private(&path, &jpeg)?;
        Ok(true)
    }

    pub fn own_avatar(&self, pid: &str, cid: Option<&str>) -> Option<Vec<u8>> {
        fs::read(self.own_avatar_path(pid, cid)?).ok()
    }

    // -- the second brain ---------------------------------------------------

    fn memory_path(&self, pid: &str) -> Option<PathBuf> {
        Some(self.project_dir(pid)?.join(MEMORY))
    }

    fn fact_lines(&self, pid: &str) -> Vec<String> {
        self.memory_path(pid)
            .and_then(|p| fs::read_to_string(p).ok())
            .unwrap_or_default()
            .lines()
            .filter(|l| l.starts_with("- "))
            .map(str::to_string)
            .collect()
    }

    /// The project's second brain, oldest first, each fact with its mark.
    pub fn facts(&self, pid: &str) -> Vec<Fact> {
        self.fact_lines(pid)
            .iter()
            .map(|line| {
                let rest = line.trim_start_matches("- ");
                let (date, text) = match rest.strip_prefix('[').and_then(|r| r.split_once("] ")) {
                    Some((d, t)) => (d.to_string(), t.to_string()),
                    None => (String::new(), rest.to_string()),
                };
                Fact {
                    id: mark(line),
                    date,
                    text,
                }
            })
            .collect()
    }

    fn write_facts(&self, pid: &str, lines: &[String]) -> io::Result<()> {
        let Some(path) = self.memory_path(pid) else {
            return Err(io::Error::other("not a project"));
        };
        let name = self.project(pid).map(|p| p.name).unwrap_or_default();
        let body = format!(
            "# What the AFG remembers of the project {name}\n\n{}\n",
            lines.join("\n")
        );
        write_private(&path, body.as_bytes())
    }

    /// Keep what the AFG chose to remember from an answer, dated. The
    /// oldest facts go first past `MEMORY_MAX`. What was kept.
    pub fn remember(&self, pid: &str, facts: &[String]) -> io::Result<Vec<String>> {
        let day: String = now().chars().take(10).collect();
        let _w = writing();
        let mut lines = self.fact_lines(pid);
        let mut kept = Vec::new();
        for f in facts {
            let f = f.trim();
            if f.is_empty() || lines.iter().any(|l| l.ends_with(&format!("] {f}"))) {
                continue;
            }
            lines.push(format!("- [{day}] {f}"));
            kept.push(f.to_string());
        }
        if kept.is_empty() {
            return Ok(kept);
        }
        while lines.iter().map(|l| l.len() + 1).sum::<usize>() > MEMORY_MAX && lines.len() > 1 {
            lines.remove(0);
        }
        self.write_facts(pid, &lines)?;
        Ok(kept)
    }

    /// Forget one fact by its mark. Whether it was there.
    pub fn forget_fact(&self, pid: &str, id: &str) -> io::Result<bool> {
        let _w = writing();
        let lines = self.fact_lines(pid);
        let kept: Vec<String> = lines.iter().filter(|l| mark(l) != id).cloned().collect();
        if kept.len() == lines.len() {
            return Ok(false);
        }
        self.write_facts(pid, &kept)?;
        Ok(true)
    }

    /// Forget the whole second brain of a project.
    pub fn forget_memory(&self, pid: &str) -> io::Result<bool> {
        let Some(path) = self.memory_path(pid) else {
            return Ok(false);
        };
        let _w = writing();
        match fs::remove_file(path) {
            Ok(()) => Ok(true),
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(false),
            Err(e) => Err(e),
        }
    }

    /// The second brain as the AFG reads it: its most recent part when long.
    pub fn recall(&self, pid: &str) -> String {
        let memory = self
            .fact_lines(pid)
            .iter()
            .map(|l| l.trim_start_matches("- ").to_string())
            .collect::<Vec<_>>()
            .join("\n");
        if memory.len() <= RECALL_MEMORY {
            return memory;
        }
        let mut at = memory.len() - RECALL_MEMORY;
        while !memory.is_char_boundary(at) {
            at += 1;
        }
        let start = memory[at..].find('\n').map_or(at, |i| at + i + 1);
        memory[start..].to_string()
    }
}

// ---------------------------------------------------------------------------
// what the AFG is told
// ---------------------------------------------------------------------------

/// Who is talking, and in which language their pages are.
#[derive(Debug, Clone, Copy)]
pub struct Asker<'a> {
    pub name: &'a str,
    /// The person in the tree the account says it is: its id and name.
    pub person: Option<(&'a str, &'a str)>,
    /// The language of their pages, by its name in English.
    pub language: &'a str,
}

/// The system prompt of a project's conversations.
pub fn system_prompt(
    asker: &Asker,
    project: &Project,
    memory: &str,
    archive: Option<&crate::afg::Archive>,
) -> String {
    let mut s = String::from(
        "You are the Artificial family genealogist (AFG) of an ax·genealogy archive, here \
         the personal assistant of one member of the family, in a project of theirs. Help \
         them with whatever the project is about, kindly and precisely. ",
    );
    match asker.person {
        Some((id, name)) => s.push_str(&format!(
            "You are talking to {}, who says they are {name} (persons/{id}.json) in the \
             family's archive. ",
            asker.name
        )),
        None => s.push_str(&format!("You are talking to {}. ", asker.name)),
    }
    s.push_str(&format!(
        "Answer in the language they write in; their pages are in {}. Write in Markdown. \
         When they send a file, read it as part of their message; when a file is only named \
         because you cannot read that kind of file, say so plainly rather than guess.\n\n",
        asker.language
    ));
    s.push_str(&format!("The project is called \"{}\".\n", project.name));
    let conduct = project.conduct.trim();
    if !conduct.is_empty() {
        s.push_str(
            "Its code of conduct, which they wrote for you: follow it in every answer of \
             this project, unless it asks you to deceive them or to cause harm.\n\
             <code-of-conduct>\n",
        );
        s.push_str(conduct);
        s.push_str("\n</code-of-conduct>\n");
    }
    s.push_str("\nThe project's memory, what you chose to keep from its conversations:\n");
    if memory.trim().is_empty() {
        s.push_str("(nothing yet)\n");
    } else {
        s.push_str(memory.trim());
        s.push('\n');
    }
    s.push_str(
        "\nWhen this exchange taught you something durable about the project or about \
         them that will help in its later conversations (a goal, a decision, a preference, \
         a fact they gave you), end your answer with a <memory> block, one new fact per \
         line, and nothing after it. It is taken out of your answer and kept in the \
         project's memory. Never repeat there what the memory already holds.",
    );
    match archive {
        Some(a) => s.push_str(&format!(
            "\n\nThe family's archive goes with this project: the AXGF bundle given with \
             the conversation, {} JSON files, {} persons, {} families, without the \
             photographs. When you answer from it, name the records your answer rests on.",
            a.count, a.persons, a.families
        )),
        None => s.push_str(
            "\n\nThe family's archive does not go with this project: if they ask about the \
             family's records, tell them it can be given to you in the project's settings.",
        ),
    }
    s
}

/// What the AFG is given of a file: the file itself when it can read it,
/// a note naming it otherwise, or when it was sent long ago.
fn file_part(space: &Space, pid: &str, f: &FileRef, whole: bool) -> Part {
    let note = |why: &str| {
        Part::Text(format!(
            "[A file was sent: \"{}\", {}, {} bytes. {why}]",
            f.name, f.mime, f.size
        ))
    };
    if !whole {
        return note("It was sent earlier in the conversation and is not given again.");
    }
    let Some((_, bytes)) = space.file(pid, &f.id) else {
        return note("It could not be read back.");
    };
    match f.kind {
        FileKind::Image => {
            let (mime, data) = model_image(&f.mime, &bytes);
            Part::Image {
                mime,
                name: f.name.clone(),
                data,
            }
        }
        FileKind::Pdf => Part::Pdf {
            name: f.name.clone(),
            data: bytes,
        },
        FileKind::Audio | FileKind::Video => Part::Media {
            mime: f.mime.clone(),
            name: f.name.clone(),
            data: bytes,
        },
        FileKind::Text => {
            let text = String::from_utf8_lossy(&bytes);
            Part::Text(format!(
                "File \"{}\":\n```\n{}\n```",
                f.name,
                cut(&text, TEXT_FILE_CHARS)
            ))
        }
        FileKind::Other => note("It is not a kind of file that can be read here."),
    }
}

/// A conversation as the AFG is given it: its last messages, as many as
/// fit, the files of the last few whole and the older ones named. Errors
/// said in the thread are the site's, not the AFG's, and are left out.
pub fn turns(space: &Space, pid: &str, messages: &[Message]) -> Vec<Turn> {
    let said: Vec<&Message> = messages.iter().filter(|m| m.role != Role::Error).collect();
    let mut start = said.len().saturating_sub(HISTORY_MESSAGES);
    while start < said.len()
        && said[start..].iter().map(|m| m.text.len()).sum::<usize>() > HISTORY_CHARS
    {
        start += 1;
    }
    // Begin on the member's words, as every protocol wants.
    while start < said.len() && said[start].role != Role::User {
        start += 1;
    }
    let users_after: Vec<usize> = said
        .iter()
        .enumerate()
        .filter(|(_, m)| m.role == Role::User)
        .map(|(i, _)| i)
        .collect();
    let recent_from = users_after
        .len()
        .checked_sub(FILES_RECENT)
        .map(|k| users_after[k])
        .unwrap_or(0);
    let mut out: Vec<Turn> = Vec::new();
    if start > 0 && start < said.len() {
        out.push(Turn {
            speaker: Speaker::User,
            parts: vec![Part::Text(format!(
                "[{start} earlier messages of this conversation are not repeated here.]"
            ))],
        });
    }
    for (i, m) in said.iter().enumerate().skip(start) {
        let speaker = match m.role {
            Role::User => Speaker::User,
            _ => Speaker::Assistant,
        };
        let mut parts = Vec::new();
        if m.role == Role::User {
            for f in &m.files {
                parts.push(file_part(space, pid, f, i >= recent_from));
            }
        }
        if !m.text.trim().is_empty() {
            parts.push(Part::Text(m.text.clone()));
        }
        if parts.is_empty() {
            continue;
        }
        // Two turns of the same speaker in a row become one.
        match out.last_mut() {
            Some(last) if last.speaker == speaker => last.parts.extend(parts),
            _ => out.push(Turn { speaker, parts }),
        }
    }
    out
}

/// The family's archive as the AFG reads it: every JSON file of the bundle.
pub fn archive_context(a: &crate::afg::Archive) -> String {
    let mut out = format!(
        "The AXGF bundle: {} JSON files, {} persons, {} families.\n",
        a.count, a.persons, a.families
    );
    for (name, content) in &a.files {
        out.push_str("\n=== ");
        out.push_str(name);
        out.push_str(" ===\n");
        out.push_str(content);
        out.push('\n');
    }
    out
}

/// Everything the AFG is given for the next answer of a conversation: its
/// instructions, the archive when the project takes it, and the turns.
pub fn chat_prompt(
    space: &Space,
    asker: &Asker,
    project: &Project,
    messages: &[Message],
    archive: Option<&crate::afg::Archive>,
) -> ChatPrompt {
    ChatPrompt {
        system: system_prompt(asker, project, &space.recall(&project.id), archive),
        context: archive.map(archive_context).unwrap_or_default(),
        turns: turns(space, &project.id, messages),
    }
}

// ---------------------------------------------------------------------------
// answers being written
// ---------------------------------------------------------------------------

/// An answer the AFG is writing: what it has written so far.
#[derive(Debug)]
pub struct Live {
    pub since: Instant,
    pub partial: crate::afg_chat::Partial,
}

/// The answers being written, one at most per conversation, a few at most
/// across the site.
#[derive(Debug, Default)]
pub struct Answering {
    live: Mutex<HashMap<String, Arc<Live>>>,
}

/// How many answers may be written at once across the site: each holds a
/// model's attention, or a program on this machine.
pub const ANSWERING_MAX: usize = 4;

fn key(account: &str, pid: &str, cid: &str) -> String {
    format!("{account}/{pid}/{cid}")
}

impl Answering {
    fn live(&self) -> MutexGuard<'_, HashMap<String, Arc<Live>>> {
        self.live.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// Mark a conversation as being answered. `None` when it already is, or
    /// when the site is answering as many as it may.
    pub fn begin(&self, account: &str, pid: &str, cid: &str) -> Option<Arc<Live>> {
        let mut live = self.live();
        let k = key(account, pid, cid);
        if live.contains_key(&k) || live.len() >= ANSWERING_MAX {
            return None;
        }
        let l = Arc::new(Live {
            since: Instant::now(),
            partial: Default::default(),
        });
        live.insert(k, l.clone());
        Some(l)
    }

    pub fn get(&self, account: &str, pid: &str, cid: &str) -> Option<Arc<Live>> {
        self.live().get(&key(account, pid, cid)).cloned()
    }

    pub fn end(&self, account: &str, pid: &str, cid: &str) {
        self.live().remove(&key(account, pid, cid));
    }

    /// Whether the site is answering as many as it may.
    pub fn full(&self) -> bool {
        self.live().len() >= ANSWERING_MAX
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn owner(id: &str, name: &str) -> Owner {
        Owner {
            id: id.into(),
            name: name.into(),
            person_id: None,
        }
    }

    fn png(w: u32, h: u32) -> Vec<u8> {
        let img = image::RgbImage::from_pixel(w, h, image::Rgb([30, 120, 60]));
        let mut out = std::io::Cursor::new(Vec::new());
        image::DynamicImage::ImageRgb8(img)
            .write_to(&mut out, image::ImageFormat::Png)
            .unwrap();
        out.into_inner()
    }

    #[test]
    fn ids_and_names_are_safe_for_a_path() {
        assert!(is_id(&new_id()));
        for bad in [
            "",
            "../x",
            "0123456789abcdeF",
            "0123456789abcde",
            "0123456789abcdef0",
        ] {
            assert!(!is_id(bad), "{bad}");
        }
        assert_eq!(
            clean_name("  Les  Delaunay\n\tde Lyon ", 80),
            "Les Delaunay de Lyon"
        );
        assert_eq!(clean_name("abcdef", 3), "abc");
        assert_eq!(file_name("../../etc/passwd"), "passwd");
        assert_eq!(file_name("C:\\photos\\mamie.jpg"), "mamie.jpg");
        assert_eq!(file_name(".env"), "file.env");
    }

    #[test]
    fn a_file_is_what_its_bytes_say() {
        assert_eq!(sniff("x.txt", &png(4, 4)), (FileKind::Image, "image/png"));
        assert_eq!(
            sniff("x.jpg", b"%PDF-1.7\n"),
            (FileKind::Pdf, "application/pdf")
        );
        assert_eq!(
            sniff("notes.md", "# Ma grand-mère\n".as_bytes()),
            (FileKind::Text, "text/markdown")
        );
        assert_eq!(
            sniff("famille.ged", b"0 HEAD\n1 CHAR UTF-8\n"),
            (FileKind::Text, "text/plain")
        );
        assert_eq!(
            sniff("run.sh", b"#!/bin/sh\n"),
            (FileKind::Other, "application/octet-stream")
        );
        assert_eq!(
            sniff("x.txt", b"bin\0ary"),
            (FileKind::Other, "application/octet-stream")
        );
        let mut mp4 = vec![0, 0, 0, 24];
        mp4.extend_from_slice(b"ftypisom\0\0\0\0");
        assert_eq!(sniff("film", &mp4), (FileKind::Video, "video/mp4"));
        assert_eq!(
            sniff("voix", b"ID3\x03\0\0\0"),
            (FileKind::Audio, "audio/mpeg")
        );
    }

    #[test]
    fn an_avatar_is_a_square_jpeg_and_a_big_picture_is_made_smaller_for_a_model() {
        let jpeg = avatar_jpeg(&png(400, 300)).unwrap();
        let img = image::load_from_memory(&jpeg).unwrap();
        assert_eq!((img.width(), img.height()), (AVATAR_EDGE, AVATAR_EDGE));
        assert!(avatar_jpeg(b"not a picture").is_none());
        let (mime, data) = model_image("image/png", &png(3000, 1000));
        assert_eq!(mime, "image/jpeg");
        let img = image::load_from_memory(&data).unwrap();
        assert_eq!(img.width(), MODEL_IMAGE_EDGE);
        let small = png(10, 10);
        assert_eq!(
            model_image("image/png", &small),
            ("image/png".to_string(), small)
        );
    }

    #[test]
    fn a_space_holds_projects_conversations_files_and_a_second_brain() {
        let dir = crate::scratch::Dir::new("assistant-space");
        let spaces = Spaces::beside(&dir.join("family.axgf"));
        let vera = owner("acc-vera", "vera");
        assert!(spaces.existing(&vera).is_none(), "nothing made by looking");
        let space = spaces.of(&vera).unwrap();
        let p = space
            .create_project("Santé de mamie", "medicine-heart")
            .unwrap()
            .unwrap();
        assert!(is_id(&p.id));
        let mut p2 = space.project(&p.id).unwrap();
        p2.conduct = "Réponds en deux phrases.".into();
        p2.archive = true;
        space.save_project(&p2).unwrap();
        assert_eq!(
            space.project(&p.id).unwrap().conduct,
            "Réponds en deux phrases."
        );

        let c = space
            .create_conversation(&p.id, "Quels vaccins à 90 ans ?\nMerci", "")
            .unwrap()
            .unwrap();
        assert_eq!(c.title, "Quels vaccins à 90 ans ? Merci");
        let f = space.save_file(&p.id, "radio.png", &png(8, 8)).unwrap();
        assert_eq!(f.kind, FileKind::Image);
        space
            .append(
                &p.id,
                &c.id,
                &Message::new(Role::User, "Voici sa radio.".into(), vec![f.clone()]),
            )
            .unwrap();
        space
            .append(
                &p.id,
                &c.id,
                &Message::new(Role::Assistant, "Je la vois.".into(), vec![]),
            )
            .unwrap();
        let ms = space.messages(&p.id, &c.id);
        assert_eq!(ms.len(), 2);
        assert_eq!(space.conversation(&p.id, &c.id).unwrap().messages, 2);
        assert_eq!(space.file(&p.id, &f.id).unwrap().1, png(8, 8));
        assert!(space.file(&p.id, "../project.json").is_none());

        let kept = space
            .remember(&p.id, &["Mamie a 91 ans".into(), "".into()])
            .unwrap();
        assert_eq!(kept, ["Mamie a 91 ans"]);
        assert!(
            space
                .remember(&p.id, &["Mamie a 91 ans".into()])
                .unwrap()
                .is_empty(),
            "not twice"
        );
        let facts = space.facts(&p.id);
        assert_eq!(facts.len(), 1);
        assert_eq!(
            space.recall(&p.id),
            format!("[{}] Mamie a 91 ans", facts[0].date)
        );
        assert!(space.forget_fact(&p.id, &facts[0].id).unwrap());
        assert!(space.facts(&p.id).is_empty());

        let list = space.projects();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].conversations, 1);

        // Another account under the same name never reads it.
        let other = owner("acc-other", "vera");
        assert!(spaces.existing(&other).is_none());
        let fresh = spaces.of(&other).unwrap();
        assert!(fresh.projects().is_empty(), "the old space is set aside");

        assert!(spaces.existing(&vera).is_none());
        assert!(
            spaces.root().join("vera@acc-vera").join(PROJECTS).is_dir(),
            "vera's projects are set aside whole"
        );
    }

    #[test]
    fn the_afg_is_given_the_conduct_the_memory_and_the_conversation() {
        let dir = crate::scratch::Dir::new("assistant-prompt");
        let spaces = Spaces::beside(&dir.join("family.axgf"));
        let space = spaces.of(&owner("acc-1", "marie")).unwrap();
        let mut p = space.create_project("Cuisine", "").unwrap().unwrap();
        p.conduct = "Tutoie-moi.".into();
        space.save_project(&p).unwrap();
        space
            .remember(&p.id, &["Elle cuisine sans gluten".into()])
            .unwrap();
        let c = space
            .create_conversation(&p.id, "Une tarte ?", "")
            .unwrap()
            .unwrap();
        let f = space
            .save_file(&p.id, "recette.txt", "200 g de farine".as_bytes())
            .unwrap();
        let mut ms = vec![
            Message::new(Role::User, "Une tarte ?".into(), vec![f]),
            Message::new(Role::Assistant, "Aux pommes.".into(), vec![]),
            Message::new(Role::Error, "HTTP 500".into(), vec![]),
            Message::new(Role::User, "Et sans farine ?".into(), vec![]),
        ];
        for m in &ms {
            space.append(&p.id, &c.id, m).unwrap();
        }
        let asker = Asker {
            name: "marie",
            person: None,
            language: "French",
        };
        let prompt = chat_prompt(&space, &asker, &p, &ms, None);
        assert!(prompt
            .system
            .contains("<code-of-conduct>\nTutoie-moi.\n</code-of-conduct>"));
        assert!(prompt.system.contains("Elle cuisine sans gluten"));
        assert!(prompt.system.contains("in French"));
        assert!(prompt.system.contains("does not go with this project"));
        assert_eq!(prompt.turns.len(), 3, "the error is the site's, left out");
        assert_eq!(prompt.turns[0].speaker, Speaker::User);
        match &prompt.turns[0].parts[0] {
            Part::Text(t) => assert!(t.contains("200 g de farine"), "{t}"),
            other => panic!("{other:?}"),
        }
        // Long ago, a file is named and not sent again.
        ms.truncate(2);
        for _ in 0..4 {
            ms.push(Message::new(Role::User, "Encore ?".into(), vec![]));
            ms.push(Message::new(Role::Assistant, "Oui.".into(), vec![]));
        }
        let turns = turns(&space, &p.id, &ms);
        match &turns[0].parts[0] {
            Part::Text(t) => assert!(t.contains("sent earlier"), "{t}"),
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn one_answer_at_a_time_in_a_conversation_and_a_few_across_the_site() {
        let a = Answering::default();
        assert!(a.begin("u", "p", "c").is_some());
        assert!(a.begin("u", "p", "c").is_none(), "one at a time");
        for i in 1..ANSWERING_MAX {
            assert!(a.begin("u", "p", &format!("c{i}")).is_some());
        }
        assert!(a.full());
        assert!(a.begin("u", "p", "other").is_none());
        a.end("u", "p", "c");
        assert!(!a.full());
        assert!(a.get("u", "p", "c").is_none());
    }
}
