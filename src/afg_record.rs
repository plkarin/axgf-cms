//! The AFG on a person's record (the operator's request of 3 November 2026,
//! docs/adr/0029): a photograph restored, a document read, and a summary of
//! the person written.
//!
//! # Where each comes from
//!
//! - **A restored photograph** comes from the picture service
//!   ([`crate::afg::ImageConfig`]). Claude and the coding agents write text
//!   only, so pictures have a source of their own:
//!   - OpenAI's `POST /images/edits`;
//!   - Gemini's `generateContent`, asked for an image.
//!
//!   Either is given the photograph and one instruction ([`RESTORE`]).
//! - **Reading a document and summarising a person** go to the AFG's own
//!   computing (`crate::afg_chat`), with no archive and no brain:
//!   - the document alone, to read it;
//!   - the person's record and the notes about them, to summarise them.
//!
//! # What becomes of it
//!
//! Nothing here writes to the archive: `routes::afg_record` does, through the
//! same doors an upload and a note go through. What it writes is marked, for
//! any reader of the format:
//!
//! - **a restored copy** is a Document of the person's, tagged
//!   [`RETOUCH_TAG`], named after its original with `_IA`, and never their
//!   face (`crate::person`);
//! - **a transcription** is a note (docs/adr/0014), tagged
//!   [`TRANSCRIPTION_TAG`], naming the document it was read from;
//! - **a summary** is a note too, tagged [`SUMMARY_TAG`], naming its person:
//!   the record shows it above Identity, and nowhere among the notes.

use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use base64::Engine as _;
use serde_json::{json, Value};

use crate::afg::{self, Config, Failure, ImageConfig, ImageKind, Kind};
use crate::afg_chat::{ChatPrompt, Part, Speaker, Turn};

/// What a restored photograph is asked for.
pub const RESTORE: &str = "Restore this old photograph. If it is black and white, sepia or \
faded, give it natural, true-to-life colours. Sharpen it, and remove scratches, dust, \
stains and noise. Keep everything else exactly as it is: the same people, faces, \
features, expressions, poses, clothes, background and framing. Add nothing and remove \
nothing.";

/// What a document holding no text is answered with.
pub const NO_TEXT: &str = "NO_TEXT";

/// How a document is read.
const READ: &str = "You read documents for a family's archive. Transcribe every word of \
text visible in the document exactly as it is written, in the document's own language \
and script: do not translate it, correct it, summarise it or comment on it. Keep its \
paragraphs and line breaks as far as Markdown allows. Write [illegible] for a word you \
cannot read, and [?] after a word you are unsure of. Answer with the transcription \
alone, in Markdown, with no heading and nothing before or after it. If the document \
holds no text at all, answer NO_TEXT and nothing else.";

/// The tag a restored copy carries, for any reader of the format. Not
/// `ai-generated`: that one is a reworked portrait's (docs/adr/0009), which
/// is kept out of the Media tab, where a restored copy belongs.
pub const RETOUCH_TAG: &str = "ai-retouched";
/// What this application records about a restored copy: what made it, and
/// the photograph it was made from.
pub const RETOUCH_EXT_KEY: &str = "axgf-cms:ai-retouch/v1";

/// The tag a note the AFG read out of a document carries.
pub const TRANSCRIPTION_TAG: &str = "transcription";
/// The document it was read from, and what read it.
pub const TRANSCRIPTION_EXT_KEY: &str = "axgf-cms:transcription/v1";

/// The tag a person's summary carries.
pub const SUMMARY_TAG: &str = "person-summary";
/// The person it sums up; its language and what wrote it, when the AFG did.
pub const SUMMARY_EXT_KEY: &str = "axgf-cms:summary/v1";

/// The longest edge a photograph is sent to a picture service at.
const SEND_EDGE: u32 = 2048;

/// The most a picture service's answer may weigh: a picture, in base64.
const ANSWER_MAX: u64 = 64 * 1024 * 1024;

fn has_tag(doc: &Value, tag: &str) -> bool {
    doc.get("tags")
        .and_then(Value::as_array)
        .is_some_and(|t| t.iter().any(|x| x.as_str() == Some(tag)))
}

fn ext<'a>(doc: &'a Value, key: &str) -> Option<&'a Value> {
    doc.get("extensions").and_then(|e| e.get(key))
}

fn ext_str<'a>(doc: &'a Value, key: &str, field: &str) -> Option<&'a str> {
    ext(doc, key)
        .and_then(|e| e.get(field))
        .and_then(Value::as_str)
        .filter(|s| !s.trim().is_empty())
}

/// Add a tag and an extension to an entity, keeping what it had.
fn mark(entity: &mut Value, tag: &str, key: &str, value: Value) {
    let Some(obj) = entity.as_object_mut() else {
        return;
    };
    let tags = obj.entry("tags").or_insert_with(|| json!([]));
    if let Some(list) = tags.as_array_mut() {
        if !list.iter().any(|t| t == tag) {
            list.push(Value::from(tag));
        }
    }
    let extensions = obj.entry("extensions").or_insert_with(|| json!({}));
    if let Some(e) = extensions.as_object_mut() {
        e.insert(key.into(), value);
    }
}

/// `{"generator": …, <field>: …}`, without what is not said.
fn provenance(generator: Option<&str>, field: &str, from: Option<&str>) -> Value {
    let mut out = serde_json::Map::new();
    if let Some(g) = generator.map(str::trim).filter(|g| !g.is_empty()) {
        out.insert("generator".into(), Value::from(g));
    }
    if let Some(f) = from.map(str::trim).filter(|f| !f.is_empty()) {
        out.insert(field.into(), Value::from(f));
    }
    Value::Object(out)
}

// ---------------------------------------------------------------------------
// what is stored
// ---------------------------------------------------------------------------

/// Whether a Document is a copy of a photograph an AI restored. Either mark
/// is enough, as for a reworked portrait.
pub fn is_ai_retouch(doc: &Value) -> bool {
    ext(doc, RETOUCH_EXT_KEY).is_some() || has_tag(doc, RETOUCH_TAG)
}

/// The Document for a restored copy of `original`, one of the person's:
/// every field but the file, which the caller fills from the bytes it sniffed.
/// Not a `photo`, for the reason a reworked portrait is not one
/// (docs/adr/0009): nobody took it.
pub fn retouch_document(
    doc_id: &str,
    person_id: &str,
    original: (&str, &Value),
    filename: &str,
    mime: &str,
    file: Value,
    generator: &str,
) -> Value {
    let (original_id, original) = original;
    let mut doc = json!({
        "id": doc_id,
        "type": "document",
        "axgf_version": "1.0",
        "filename": filename,
        "mime_type": mime,
        "document_type": "other",
        "status": "present",
        "file": file,
        "linked_to": [{"entity_type": "person", "entity_id": person_id, "role": "subject"}],
    });
    for k in ["caption", "date"] {
        if let Some(v) = original.get(k).filter(|v| !v.is_null()) {
            doc[k] = v.clone();
        }
    }
    mark(
        &mut doc,
        RETOUCH_TAG,
        RETOUCH_EXT_KEY,
        provenance(Some(generator), "derived_from_id", Some(original_id)),
    );
    doc
}

/// What made a restored copy.
pub fn retouch_generator(doc: &Value) -> Option<&str> {
    ext_str(doc, RETOUCH_EXT_KEY, "generator")
}

/// Mark a new note as read out of a document by the AFG.
pub fn mark_transcription(entity: &mut Value, document_id: &str, generator: Option<&str>) {
    mark(
        entity,
        TRANSCRIPTION_TAG,
        TRANSCRIPTION_EXT_KEY,
        provenance(generator, "document", Some(document_id)),
    );
}

/// The document a transcription was read from.
pub fn transcribed_from(doc: &Value) -> Option<&str> {
    ext_str(doc, TRANSCRIPTION_EXT_KEY, "document")
}

/// Mark a note as a person's summary: in a language, by the AFG, when it
/// wrote it.
pub fn mark_summary(
    entity: &mut Value,
    person_id: &str,
    language: Option<&str>,
    generator: Option<&str>,
) {
    let mut value = provenance(generator, "person", Some(person_id));
    if let Some(l) = language.filter(|l| !l.is_empty()) {
        value["language"] = Value::from(l);
    }
    mark(entity, SUMMARY_TAG, SUMMARY_EXT_KEY, value);
}

/// Whether a note is somebody's summary.
pub fn is_summary(doc: &Value) -> bool {
    ext(doc, SUMMARY_EXT_KEY).is_some() || has_tag(doc, SUMMARY_TAG)
}

/// Whether a note is this person's summary: marked as a summary, about them.
pub fn is_summary_of(doc: &Value, person_id: &str) -> bool {
    is_summary(doc)
        && match ext_str(doc, SUMMARY_EXT_KEY, "person") {
            Some(p) => p == person_id,
            None => crate::notes::persons(doc).iter().any(|p| p == person_id),
        }
}

/// The language a summary was written in, when the AFG wrote it.
pub fn summary_language(doc: &Value) -> Option<&str> {
    ext_str(doc, SUMMARY_EXT_KEY, "language")
}

/// What wrote a summary, when the AFG did; `None` for one a member wrote,
/// or that a member changed since.
pub fn summary_generator(doc: &Value) -> Option<&str> {
    ext_str(doc, SUMMARY_EXT_KEY, "generator")
}

/// A summary edited by hand is the writer's: what wrote it first is no
/// longer what it says. Its language is kept, as the editor does not change
/// it.
pub fn summary_edited(entity: &mut Value) {
    if let Some(e) = entity
        .get_mut("extensions")
        .and_then(|e| e.get_mut(SUMMARY_EXT_KEY))
        .and_then(Value::as_object_mut)
    {
        e.remove("generator");
    }
}

/// This person's summary, as this reader may read it: the latest written,
/// when two writers each wrote one.
pub fn summary_of_person<'a>(
    flat: &'a Value,
    lens: &crate::access::Lens,
    person_id: &str,
) -> Option<(&'a str, &'a Value)> {
    let when = |d: &Value| {
        d.get("updated_at")
            .or_else(|| d.get("created_at"))
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string()
    };
    crate::notes::notes_about(flat, lens, true, person_id)
        .into_iter()
        .filter(|(_, d)| is_summary_of(d, person_id))
        .max_by(|a, b| when(a.1).cmp(&when(b.1)).then(b.0.cmp(a.0)))
}

/// The language a summary may be asked in: one of the site's.
pub fn language(tag: &str) -> Option<&'static crate::i18n::Locale> {
    crate::i18n::LOCALES.iter().find(|l| l.tag == tag.trim())
}

// ---------------------------------------------------------------------------
// a photograph restored
// ---------------------------------------------------------------------------

/// A picture back from the service.
#[derive(Debug, Clone)]
pub struct Picture {
    pub bytes: Vec<u8>,
    /// What made it, as a document's `generator` says it.
    pub generator: String,
}

/// Ask the picture service to restore a photograph.
pub async fn restore(
    cfg: &ImageConfig,
    timeout: Duration,
    mime: &str,
    bytes: &[u8],
) -> Result<Picture, Failure> {
    let cfg = cfg.clone();
    let (mime, bytes) = for_service(mime, bytes);
    tokio::task::spawn_blocking(move || match cfg.kind {
        ImageKind::Openai => openai_restore(&cfg, timeout, &mime, &bytes),
        ImageKind::Gemini => gemini_restore(&cfg, timeout, &mime, &bytes),
        ImageKind::Off => Err(Failure::new("no picture service is set up")),
    })
    .await
    .map_err(|e| Failure::new(e.to_string()))?
}

/// The photograph as a service takes it: within [`SEND_EDGE`], as a JPEG
/// when it had to be made smaller. Bytes that do not decode go as they are,
/// for the service to refuse in its own words.
fn for_service(mime: &str, bytes: &[u8]) -> (String, Vec<u8>) {
    let Ok(img) = image::load_from_memory(bytes) else {
        return (mime.to_string(), bytes.to_vec());
    };
    let fits = img.width().max(img.height()) <= SEND_EDGE;
    let plain = matches!(mime, "image/jpeg" | "image/png" | "image/webp");
    if fits && plain && bytes.len() <= 20 * 1024 * 1024 {
        return (mime.to_string(), bytes.to_vec());
    }
    let small = if fits {
        img
    } else {
        img.resize(SEND_EDGE, SEND_EDGE, image::imageops::FilterType::Lanczos3)
    };
    match jpeg(&small, 92) {
        Some(out) => ("image/jpeg".into(), out),
        None => (mime.to_string(), bytes.to_vec()),
    }
}

fn jpeg(img: &image::DynamicImage, quality: u8) -> Option<Vec<u8>> {
    let mut out = std::io::Cursor::new(Vec::new());
    let rgb = image::DynamicImage::ImageRgb8(img.to_rgb8());
    image::codecs::jpeg::JpegEncoder::new_with_quality(&mut out, quality)
        .encode_image(&rgb)
        .ok()?;
    Some(out.into_inner())
}

/// A picture the site may keep: within the upload limit, as a JPEG when it
/// was larger. `None` when it is not a picture at all.
pub fn keepable(bytes: Vec<u8>) -> Option<(crate::documents::Kind, Vec<u8>)> {
    let kind = crate::documents::sniff(&bytes).filter(|k| k.raster_image)?;
    if bytes.len() <= crate::documents::MAX_UPLOAD {
        return Some((kind, bytes));
    }
    let img = image::load_from_memory(&bytes).ok()?;
    let smaller = [90u8, 80, 70]
        .into_iter()
        .filter_map(|q| jpeg(&img, q))
        .find(|b| b.len() <= crate::documents::MAX_UPLOAD)?;
    let kind = crate::documents::sniff(&smaller)?;
    Some((kind, smaller))
}

fn b64(bytes: &[u8]) -> String {
    base64::engine::general_purpose::STANDARD.encode(bytes)
}

fn unb64(s: &str) -> Result<Vec<u8>, Failure> {
    base64::engine::general_purpose::STANDARD
        .decode(s.trim())
        .map_err(|e| Failure::new(format!("the service's picture does not decode: {e}")))
}

/// Read an answer whole, within [`ANSWER_MAX`].
fn answer_text(resp: &mut ureq::http::Response<ureq::Body>) -> Result<(u16, String), Failure> {
    let status = resp.status().as_u16();
    let text = resp
        .body_mut()
        .with_config()
        .limit(ANSWER_MAX)
        .read_to_string()
        .map_err(|e| Failure::new(format!("reading the service's answer: {e}")))?;
    Ok((status, text))
}

/// OpenAI: the photograph and the instruction as a form, the picture back
/// in base64.
fn openai_restore(
    cfg: &ImageConfig,
    timeout: Duration,
    mime: &str,
    bytes: &[u8],
) -> Result<Picture, Failure> {
    let boundary = format!("axgf{}", uuid::Uuid::new_v4().simple());
    let ext = match mime {
        "image/png" => "png",
        "image/webp" => "webp",
        _ => "jpg",
    };
    let mut body = Vec::new();
    for (name, value) in [("model", cfg.model()), ("prompt", RESTORE)] {
        body.extend_from_slice(
            format!(
                "--{boundary}\r\nContent-Disposition: form-data; name=\"{name}\"\r\n\r\n{value}\r\n"
            )
            .as_bytes(),
        );
    }
    body.extend_from_slice(
        format!(
            "--{boundary}\r\nContent-Disposition: form-data; name=\"image[]\"; \
             filename=\"photograph.{ext}\"\r\nContent-Type: {mime}\r\n\r\n"
        )
        .as_bytes(),
    );
    body.extend_from_slice(bytes);
    body.extend_from_slice(format!("\r\n--{boundary}--\r\n").as_bytes());

    let url = format!("{}/images/edits", cfg.base_url());
    let mut resp = afg::http_agent(timeout)
        .post(&url)
        .header("Authorization", format!("Bearer {}", cfg.api_key.trim()))
        .header(
            "Content-Type",
            format!("multipart/form-data; boundary={boundary}"),
        )
        .send(&body[..])
        .map_err(|e| Failure::new(format!("{url}: {e}")))?;
    let (status, text) = answer_text(&mut resp)?;
    if !(200..300).contains(&status) {
        return Err(Failure::new(afg::http_error(status, &text)));
    }
    let v: Value = serde_json::from_str(&text)
        .map_err(|e| Failure::new(format!("the service's answer is not JSON: {e}")))?;
    let data = v
        .pointer("/data/0/b64_json")
        .and_then(Value::as_str)
        .ok_or_else(|| Failure::new("the service sent no picture back"))?;
    Ok(Picture {
        bytes: unb64(data)?,
        generator: format!("OpenAI {}", cfg.model()),
    })
}

/// Gemini: the photograph inline with the instruction, asked to answer with
/// an image.
fn gemini_restore(
    cfg: &ImageConfig,
    timeout: Duration,
    mime: &str,
    bytes: &[u8],
) -> Result<Picture, Failure> {
    let url = format!(
        "{}/models/{}:generateContent",
        cfg.base_url(),
        afg::gemini_model(cfg.model())
    );
    let body = json!({
        "contents": [{
            "role": "user",
            "parts": [
                {"inline_data": {"mime_type": mime, "data": b64(bytes)}},
                {"text": RESTORE},
            ],
        }],
        "generationConfig": {"responseModalities": ["IMAGE"]},
    });
    let mut resp = afg::http_agent(timeout)
        .post(&url)
        .header("x-goog-api-key", cfg.api_key.trim())
        .header("Content-Type", "application/json")
        .send(body.to_string())
        .map_err(|e| Failure::new(format!("{url}: {e}")))?;
    let (status, text) = answer_text(&mut resp)?;
    if !(200..300).contains(&status) {
        return Err(Failure::new(afg::http_error(status, &text)));
    }
    let v: Value = serde_json::from_str(&text)
        .map_err(|e| Failure::new(format!("the service's answer is not JSON: {e}")))?;
    let parts = v
        .pointer("/candidates/0/content/parts")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let data = parts.iter().find_map(|p| {
        p.get("inlineData")
            .or_else(|| p.get("inline_data"))
            .and_then(|d| d.get("data"))
            .and_then(Value::as_str)
    });
    match data {
        Some(d) => Ok(Picture {
            bytes: unb64(d)?,
            generator: format!("Google {}", cfg.model()),
        }),
        None => {
            let why = v
                .pointer("/candidates/0/finishReason")
                .and_then(Value::as_str)
                .map(str::to_string)
                .or_else(|| {
                    parts
                        .iter()
                        .find_map(|p| p.get("text").and_then(Value::as_str))
                        .map(|t| afg::tail(t, 300))
                })
                .or_else(|| {
                    v.pointer("/promptFeedback/blockReason")
                        .and_then(Value::as_str)
                        .map(str::to_string)
                })
                .unwrap_or_else(|| "no reason given".into());
            Err(Failure::new(format!(
                "the service sent no picture back: {why}"
            )))
        }
    }
}

// ---------------------------------------------------------------------------
// a document read, a person summarised
// ---------------------------------------------------------------------------

/// Whether the AFG's computing can be given a PDF to read: the services and
/// agents that take one as a document. The others are given its name only.
pub fn reads_pdf(kind: Kind) -> bool {
    matches!(
        kind,
        Kind::Openai | Kind::Anthropic | Kind::Gemini | Kind::ClaudeCode | Kind::Opencode
    )
}

/// What the AFG is given to read one document: the instructions, and the
/// document as a picture or a PDF.
pub fn read_prompt(name: &str, mime: &str, bytes: &[u8]) -> ChatPrompt {
    let part = if mime == "application/pdf" {
        Part::Pdf {
            name: name.to_string(),
            data: bytes.to_vec(),
        }
    } else {
        let (mime, data) = crate::assistant::model_image(mime, bytes);
        Part::Image {
            mime,
            name: name.to_string(),
            data,
        }
    };
    ChatPrompt {
        system: READ.to_string(),
        context: String::new(),
        turns: vec![Turn {
            speaker: Speaker::User,
            parts: vec![
                Part::Text(format!("Transcribe the text of this document, {name}.")),
                part,
            ],
        }],
    }
}

/// Whether a reading found no text.
pub fn found_no_text(answer: &str) -> bool {
    answer.trim().trim_matches(['.', '`', '*', '"']).trim() == NO_TEXT
}

/// What the AFG is given to summarise one person: the instructions, in the
/// language asked for, the record (its files left out) and the notes.
pub fn summary_prompt(language: &str, record: &Value, notes: &[(String, String)]) -> ChatPrompt {
    let system = format!(
        "You write the short summary at the top of one person's page in a family's \
         archive. Write it in {language}. One paragraph of three to eight sentences: who \
         they were, when and where they lived, their family and their work, as the record \
         and the notes say. Say only what they say; mark an approximate date as \
         approximate, and leave out what is not known; invent nothing. Write the names of \
         people and places as the record writes them. Plain prose, with no heading, no \
         list, nothing before or after it, and no mention of the record or of the notes."
    );
    let mut text = format!(
        "The record:\n\n```json\n{}\n```\n",
        serde_json::to_string_pretty(record).unwrap_or_default()
    );
    if !notes.is_empty() {
        text.push_str("\nThe notes about this person:\n");
        for (title, body) in notes {
            text.push_str(&format!("\n### {title}\n\n{}\n", body.trim()));
        }
    }
    ChatPrompt {
        system,
        context: String::new(),
        turns: vec![Turn {
            speaker: Speaker::User,
            parts: vec![Part::Text(text)],
        }],
    }
}

/// The record as the summary reads it: the person's page with its files,
/// pictures and page furniture left out, and the profile's groups as this
/// reader may read them. The notes about the person go apart, in full.
pub fn record_for_summary(
    view: &crate::person::PersonView,
    profile: &[crate::profile::view::GroupView],
) -> Value {
    let mut v = serde_json::to_value(view).unwrap_or(Value::Null);
    if let Some(o) = v.as_object_mut() {
        for k in [
            "documents",
            "images",
            "ai_portraits",
            "showcase_notes",
            "raw_json",
            "radars",
            "profile_tabs",
            "profile_count",
            "timeline_from",
            "timeline_to",
            "has_timeline",
            "living_key",
        ] {
            o.remove(k);
        }
        if let Some(h) = o.get_mut("header").and_then(Value::as_object_mut) {
            h.remove("avatar");
            h.remove("initials");
        }
        let groups: Vec<Value> = profile
            .iter()
            .filter(|g| !g.attributes.is_empty())
            .map(|g| {
                json!({
                    "group": g.title,
                    "attributes": g.attributes.iter().map(|a| json!({
                        "label": a.label,
                        "claims": a.claims,
                    })).collect::<Vec<_>>(),
                })
            })
            .collect();
        if !groups.is_empty() {
            o.insert("profile".into(), Value::Array(groups));
        }
    }
    prune(v)
}

/// A value without its empty parts: no null, no empty text, list or object.
fn prune(v: Value) -> Value {
    match v {
        Value::Object(m) => Value::Object(
            m.into_iter()
                .map(|(k, v)| (k, prune(v)))
                .filter(|(_, v)| !empty(v))
                .collect(),
        ),
        Value::Array(a) => Value::Array(a.into_iter().map(prune).filter(|v| !empty(v)).collect()),
        other => other,
    }
}

fn empty(v: &Value) -> bool {
    match v {
        Value::Null => true,
        Value::String(s) => s.trim().is_empty(),
        Value::Array(a) => a.is_empty(),
        Value::Object(m) => m.is_empty(),
        _ => false,
    }
}

/// Ask the AFG's computing once, with no brain and no memory: its words, and
/// the model that wrote them when it says.
pub async fn ask(
    cfg: &Config,
    prompt: ChatPrompt,
    base: PathBuf,
) -> Result<(String, Option<String>), Failure> {
    let partial = Arc::new(Mutex::new(String::new()));
    let answer = crate::afg_chat::chat(cfg.clone(), prompt, base, partial).await?;
    let text = answer.text.trim();
    if text.is_empty() {
        return Err(Failure::new("the AFG gave an empty answer"));
    }
    let model = answer
        .model
        .filter(|m| !m.trim().is_empty())
        .or_else(|| Some(cfg.model.trim().to_string()).filter(|m| !m.is_empty()));
    Ok((text.to_string(), model))
}

/// What wrote a text, as a note records it: the AFG, its kind and model.
pub fn text_generator(cfg: &Config, model: Option<&str>) -> String {
    match model {
        Some(m) => format!("AFG {} {m}", cfg.kind.slug()),
        None => format!("AFG {}", cfg.kind.slug()),
    }
}

/// `photo.jpg` and `jpg` → `photo_IA.jpg`: the name a restored copy is kept
/// under, beside the one it was made from.
pub fn ia_name(filename: &str, ext: &str) -> String {
    let stem = std::path::Path::new(filename)
        .file_stem()
        .and_then(|s| s.to_str())
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("photo");
    format!("{stem}_IA.{ext}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn png(w: u32, h: u32) -> Vec<u8> {
        let img = image::RgbImage::from_pixel(w, h, image::Rgb([120, 90, 60]));
        let mut out = std::io::Cursor::new(Vec::new());
        image::DynamicImage::ImageRgb8(img)
            .write_to(&mut out, image::ImageFormat::Png)
            .unwrap();
        out.into_inner()
    }

    #[test]
    fn a_restored_copy_is_named_after_its_original() {
        assert_eq!(
            ia_name("Grand-mère 1932.jpg", "png"),
            "Grand-mère 1932_IA.png"
        );
        assert_eq!(ia_name("scan", "jpg"), "scan_IA.jpg");
        assert_eq!(ia_name("", "jpg"), "photo_IA.jpg");
    }

    #[test]
    fn a_large_photograph_is_sent_smaller_and_a_small_one_as_it_is() {
        let (mime, bytes) = for_service("image/png", &png(3000, 1000));
        assert_eq!(mime, "image/jpeg");
        assert_eq!(image::load_from_memory(&bytes).unwrap().width(), SEND_EDGE);

        let small = png(40, 30);
        let (mime, bytes) = for_service("image/png", &small);
        assert_eq!((mime.as_str(), bytes), ("image/png", small));
    }

    #[test]
    fn a_restored_copy_is_marked_and_keeps_what_its_original_said() {
        let original =
            json!({"caption": "Le mariage", "date": {"value": "1932", "precision": "year"}});
        let d = retouch_document(
            "c1",
            "p1",
            ("o1", &original),
            "mariage_IA.png",
            "image/png",
            json!({"path": "documents/c1.png"}),
            "OpenAI gpt-image-2.5-sunburst",
        );
        assert!(is_ai_retouch(&d));
        assert!(!crate::ai_portrait::is_ai_portrait(&d));
        assert_eq!(d["document_type"], "other");
        assert_eq!(d["caption"], "Le mariage");
        assert_eq!(d["date"]["value"], "1932");
        assert_eq!(d["linked_to"][0]["entity_id"], "p1");
        assert_eq!(d["extensions"][RETOUCH_EXT_KEY]["derived_from_id"], "o1");
        assert_eq!(retouch_generator(&d), Some("OpenAI gpt-image-2.5-sunburst"));
    }

    #[test]
    fn a_summary_is_its_persons_and_an_edit_makes_it_the_writers() {
        let mut n = json!({"tags": ["note"], "linked_to": [
            {"entity_type": "person", "entity_id": "p1", "role": "note_subject"}]});
        assert!(!is_summary(&n));
        mark_summary(&mut n, "p1", Some("pl"), Some("AFG claudecode opus"));
        assert!(is_summary_of(&n, "p1") && !is_summary_of(&n, "p2"));
        assert_eq!(n["tags"], json!(["note", SUMMARY_TAG]));
        assert_eq!(summary_language(&n), Some("pl"));
        assert_eq!(summary_generator(&n), Some("AFG claudecode opus"));
        summary_edited(&mut n);
        assert_eq!(summary_generator(&n), None);
        assert_eq!(summary_language(&n), Some("pl"));
    }

    #[test]
    fn a_transcription_names_its_document() {
        let mut n = json!({"tags": ["note"]});
        mark_transcription(&mut n, "d7", Some("AFG gemini gemini-4-argon"));
        assert_eq!(transcribed_from(&n), Some("d7"));
        assert!(!is_summary(&n));
    }

    #[test]
    fn the_summary_is_asked_in_the_language_chosen_with_the_notes_apart() {
        let p = summary_prompt(
            "Polish",
            &json!({"name": "Jan"}),
            &[("Wspomnienie".into(), "Był stolarzem.".into())],
        );
        assert!(p.system.contains("Write it in Polish"));
        let Part::Text(t) = &p.turns[0].parts[0] else {
            panic!("text")
        };
        assert!(t.contains("\"name\": \"Jan\"") && t.contains("### Wspomnienie"));
    }

    #[test]
    fn an_empty_part_of_the_record_is_not_sent() {
        let v = prune(json!({"a": null, "b": "", "c": [], "d": {}, "e": [null, "x"], "f": false}));
        assert_eq!(v, json!({"e": ["x"], "f": false}));
    }

    #[test]
    fn a_document_without_text_is_told_apart() {
        assert!(found_no_text(" NO_TEXT. "));
        assert!(found_no_text("`NO_TEXT`"));
        assert!(!found_no_text("NO_TEXT, mais une signature"));
    }

    #[test]
    fn a_picture_too_heavy_to_keep_is_kept_as_a_lighter_jpeg() {
        let (kind, bytes) = keepable(png(64, 64)).unwrap();
        assert_eq!(kind.mime, "image/png");
        assert!(!bytes.is_empty());
        assert!(keepable(b"not a picture".to_vec()).is_none());
    }
}
