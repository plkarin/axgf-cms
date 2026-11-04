//! `/admin/person/:id/…` — the AFG on a person's record (the operator's
//! request of 3 November 2026, docs/adr/0029).
//!
//! Four buttons, for a member who may write and to whose account the AFG is
//! open, on a person they can read and whose record they may change:
//!
//! - `POST /admin/person/:id/ai-rework` — their face, restored, kept as one
//!   of their reworked portraits (docs/adr/0009);
//! - `POST /admin/person/:id/document/:did/ai-retouch` — one of their
//!   photographs, restored, kept beside it as `<name>_IA`;
//! - `POST /admin/person/:id/document/:did/ai-read` — one of their documents,
//!   read, kept as a note about them;
//! - `POST /admin/person/:id/summary/ai` — their summary, written in the
//!   language asked, kept as the note above Identity.
//!
//! Each waits for the answer, writes it in one commit, and goes back to
//! where it was pressed. The log says what was sent where, never what it
//! holds.

use axum::extract::{Form, Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Redirect, Response};
use serde::Deserialize;
use serde_json::{json, Value};

use super::admin::{check_scope, io_error, require, Need};
use super::Shared;
use crate::access::Viewer;
use crate::afg::Kind;
use crate::afg_record as rec;
use crate::notes::{self, Draft};
use crate::render::{self, Chrome};
use crate::state::{BatchOutcome, Op};

/// The most of the notes' text a summary is given: past it, the rest of the
/// notes are left out rather than the request refused.
const NOTES_BUDGET: usize = 120_000;

/// A member who may press these: one who may write, the AFG open to their
/// account, on a person they can read and whose record they may change.
#[allow(clippy::result_large_err)]
fn member(state: &Shared, headers: &HeaderMap, id: &str) -> Result<(Viewer, Chrome), Response> {
    let (viewer, chrome) = require(state, headers, Need::Write)?;
    if !viewer.user.as_ref().is_some_and(|u| u.afg) {
        return Err(render::error_page_in(
            &chrome,
            StatusCode::FORBIDDEN,
            "afg-record-title",
            "afg-record-no-access",
        ));
    }
    let readable = state.read_as(viewer.ceiling(), |flat, lens| {
        flat.get("persons").and_then(|p| p.get(id)).is_some() && lens.sees_person(id)
    });
    if !readable {
        return Err(render::error_page_in(
            &chrome,
            StatusCode::NOT_FOUND,
            "error-no-such-person-title",
            "error-no-such-person-detail",
        ));
    }
    check_scope(
        state,
        &chrome,
        &viewer,
        axgf_rs::EntityKind::Person,
        &json!({"id": id}),
        None,
    )?;
    Ok((viewer, chrome))
}

/// A refusal or a failure, with the way back to the record.
fn failed(
    chrome: &Chrome,
    status: StatusCode,
    key: &str,
    args: &[(&str, fluent::FluentValue<'_>)],
    back: &str,
) -> Response {
    render::error_page_back_in(
        chrome,
        status,
        "afg-record-title",
        key,
        args,
        Some((back, "afg-record-back")),
    )
}

/// Write it, and go back to where the button was.
fn commit(state: &Shared, viewer: &Viewer, chrome: &Chrome, ops: Vec<Op>, back: &str) -> Response {
    match state.commit(ops, viewer.name(), |kind, e| {
        super::admin::label_for(kind, e)
    }) {
        Ok(BatchOutcome::Applied { .. }) => Redirect::to(back).into_response(),
        Ok(BatchOutcome::Conflict { .. }) => {
            failed(chrome, StatusCode::CONFLICT, "afg-record-stale", &[], back)
        }
        Ok(BatchOutcome::Refused { diagnostics, .. }) => failed(
            chrome,
            StatusCode::BAD_REQUEST,
            "afg-record-refused",
            &[(
                "reason",
                crate::state::format_diagnostics(&diagnostics).into(),
            )],
            back,
        ),
        Err(e) => io_error(chrome, &e),
    }
}

/// At most `max` characters of a text.
fn clip(s: &str, max: usize) -> String {
    s.chars().take(max).collect()
}

/// A file's name as Markdown shows it, its signs taken as signs.
fn md_text(s: &str) -> String {
    let mut out = String::new();
    for c in s.chars() {
        if "\\`*_[]()<>#!|~".contains(c) {
            out.push('\\');
        }
        out.push(c);
    }
    out
}

/// One of the person's own files, as this reader may read it, with its bytes
/// and the type they are. Never a picture an AI made: that one is not
/// restored or read again.
struct Source {
    id: String,
    filename: String,
    caption: Option<String>,
    doc: Value,
    kind: crate::documents::Kind,
    bytes: Vec<u8>,
}

fn source(state: &Shared, viewer: &Viewer, chrome: &Chrome, id: &str, did: &str) -> Option<Source> {
    let (view, doc) = state.read_as(viewer.ceiling(), |flat, lens| {
        let p = crate::person::build_in(flat, id, lens, chrome.lang)?;
        let d = p
            .documents
            .into_iter()
            .find(|d| d.id == did && d.known && d.has_payload && !d.ai)?;
        let doc = flat.get("documents")?.get(did)?.clone();
        Some((d, doc))
    })?;
    let bytes = state.attachment(doc.pointer("/file/path").and_then(Value::as_str)?)?;
    // The type of the bytes, never what the record or a browser said.
    let kind = crate::documents::sniff(&bytes)?;
    Some(Source {
        id: did.to_string(),
        filename: view.filename,
        caption: view.caption,
        doc,
        kind,
        bytes,
    })
}

/// Keep Canva's connection as a picture left it (docs/adr/0030): its token
/// renewed, its folder and base design made. Only while Canva is still the
/// service chosen: one changed meanwhile has forgotten the account.
fn keep_canva(state: &Shared, link: Option<crate::afg::CanvaLink>) {
    let Some(link) = link else { return };
    let kept = state.site_mutate(|s| {
        if s.afg.images.kind == crate::afg::ImageKind::Canva && s.afg.images.canva.connected() {
            s.afg.images.canva = link;
        }
    });
    if let Err(e) = kept {
        tracing::warn!(error = %e, "AFG: Canva's connection not kept");
    }
}

/// A picture back from the service, as a Document's file and its bytes.
fn stored_picture(bytes: Vec<u8>) -> Option<(String, crate::documents::Kind, Value, Vec<u8>)> {
    let (kind, bytes) = rec::keepable(bytes)?;
    let doc_id = uuid::Uuid::new_v4().to_string();
    let file = json!({
        "path": crate::documents::attachment_path(&doc_id, kind.ext),
        "size_bytes": bytes.len() as u64,
        "sha256": crate::documents::sha256_hex(&bytes),
    });
    Some((doc_id, kind, file, bytes))
}

/// `POST /admin/person/:id/ai-rework` — the person's face, restored, kept as
/// one of their reworked portraits: what a picture uploaded on their page
/// becomes (docs/adr/0009).
pub async fn rework(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Response {
    let (viewer, chrome) = match member(&state, &headers, &id) {
        Ok(v) => v,
        Err(r) => return r,
    };
    let back = format!("/person/{id}#ai-portraits");
    let cfg = state.site().afg;
    if !cfg.images.ready() {
        return failed(
            &chrome,
            StatusCode::CONFLICT,
            "afg-record-images-off",
            &[],
            &back,
        );
    }
    let count = || state.read(|flat| crate::ai_portrait::of_person(flat, &id).len());
    if count() >= crate::ai_portrait::MAX {
        return failed(
            &chrome,
            StatusCode::CONFLICT,
            "ai-portraits-too-many",
            &[],
            &back,
        );
    }
    let face = state.read_as(viewer.ceiling(), |flat, lens| {
        crate::person::build_in(flat, &id, lens, chrome.lang).and_then(|p| p.header.avatar)
    });
    let Some(face) = face else {
        return failed(
            &chrome,
            StatusCode::NOT_FOUND,
            "afg-record-no-face",
            &[],
            &back,
        );
    };
    let Some(src) = source(&state, &viewer, &chrome, &id, &face.id).filter(|s| s.kind.raster_image)
    else {
        return failed(
            &chrome,
            StatusCode::NOT_FOUND,
            "afg-record-no-face",
            &[],
            &back,
        );
    };
    tracing::info!(
        account = viewer.name(),
        person = %id,
        document = %src.id,
        service = cfg.images.kind.slug(),
        "AFG: a face sent to be restored"
    );
    let (restored, link) =
        rec::restore(&cfg.images, cfg.timeout(), src.kind.mime, &src.bytes).await;
    keep_canva(&state, link);
    let pic = match restored {
        Ok(p) => p,
        Err(f) => {
            tracing::warn!(person = %id, error = %f.message, "AFG: no restored face");
            return failed(
                &chrome,
                StatusCode::BAD_GATEWAY,
                "afg-record-failed",
                &[("reason", f.message.into())],
                &back,
            );
        }
    };
    let Some((doc_id, kind, file, bytes)) = stored_picture(pic.bytes) else {
        return failed(
            &chrome,
            StatusCode::BAD_GATEWAY,
            "afg-record-bad-picture",
            &[],
            &back,
        );
    };
    // Another may have been added while this one was being made.
    if count() >= crate::ai_portrait::MAX {
        return failed(
            &chrome,
            StatusCode::CONFLICT,
            "ai-portraits-too-many",
            &[],
            &back,
        );
    }
    let mut entity = crate::ai_portrait::document(
        &doc_id,
        &id,
        &rec::ia_name(&src.filename, kind.ext),
        kind.mime,
        file,
        &crate::ai_portrait::Provenance {
            generator: Some(pic.generator),
            derived_from_id: Some(src.id.clone()),
            caption: None,
        },
    );
    entity["created_at"] = json!(crate::view::now_iso8601());
    commit(
        &state,
        &viewer,
        &chrome,
        vec![Op::AddDocument {
            entity,
            payload: bytes,
        }],
        &back,
    )
}

/// `POST /admin/person/:id/document/:did/ai-retouch` — one of the person's
/// photographs, restored, kept in their Media tab beside it, under its name
/// and `_IA`, marked as an AI's.
pub async fn retouch(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path((id, did)): Path<(String, String)>,
) -> Response {
    let (viewer, chrome) = match member(&state, &headers, &id) {
        Ok(v) => v,
        Err(r) => return r,
    };
    let back = format!("/person/{id}?tab=media#evidence");
    let cfg = state.site().afg;
    if !cfg.images.ready() {
        return failed(
            &chrome,
            StatusCode::CONFLICT,
            "afg-record-images-off",
            &[],
            &back,
        );
    }
    let Some(src) = source(&state, &viewer, &chrome, &id, &did) else {
        return failed(
            &chrome,
            StatusCode::NOT_FOUND,
            "afg-record-no-document",
            &[],
            &back,
        );
    };
    if !src.kind.raster_image {
        return failed(
            &chrome,
            StatusCode::UNSUPPORTED_MEDIA_TYPE,
            "afg-record-not-a-picture",
            &[],
            &back,
        );
    }
    tracing::info!(
        account = viewer.name(),
        person = %id,
        document = %src.id,
        service = cfg.images.kind.slug(),
        "AFG: a photograph sent to be restored"
    );
    let (restored, link) =
        rec::restore(&cfg.images, cfg.timeout(), src.kind.mime, &src.bytes).await;
    keep_canva(&state, link);
    let pic = match restored {
        Ok(p) => p,
        Err(f) => {
            tracing::warn!(person = %id, error = %f.message, "AFG: no restored photograph");
            return failed(
                &chrome,
                StatusCode::BAD_GATEWAY,
                "afg-record-failed",
                &[("reason", f.message.into())],
                &back,
            );
        }
    };
    let Some((doc_id, kind, file, bytes)) = stored_picture(pic.bytes) else {
        return failed(
            &chrome,
            StatusCode::BAD_GATEWAY,
            "afg-record-bad-picture",
            &[],
            &back,
        );
    };
    let mut entity = rec::retouch_document(
        &doc_id,
        &id,
        (&src.id, &src.doc),
        &rec::ia_name(&src.filename, kind.ext),
        kind.mime,
        file,
        &pic.generator,
    );
    entity["created_at"] = json!(crate::view::now_iso8601());
    commit(
        &state,
        &viewer,
        &chrome,
        vec![Op::AddDocument {
            entity,
            payload: bytes,
        }],
        &back,
    )
}

/// `POST /admin/person/:id/document/:did/ai-read` — one of the person's
/// documents, a photograph of a letter or a PDF, read by the AFG in its own
/// language and kept as a note about them, dated as the document is.
pub async fn read(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path((id, did)): Path<(String, String)>,
) -> Response {
    let (viewer, chrome) = match member(&state, &headers, &id) {
        Ok(v) => v,
        Err(r) => return r,
    };
    let back = format!("/person/{id}?tab=media#media-notes");
    let cfg = state.site().afg;
    if cfg.kind == Kind::Off || cfg.problem().is_some() {
        return failed(
            &chrome,
            StatusCode::CONFLICT,
            "afg-record-text-off",
            &[],
            &back,
        );
    }
    let Some(src) = source(&state, &viewer, &chrome, &id, &did) else {
        return failed(
            &chrome,
            StatusCode::NOT_FOUND,
            "afg-record-no-document",
            &[],
            &back,
        );
    };
    let pdf = src.kind.mime == "application/pdf";
    if !src.kind.raster_image && !pdf {
        return failed(
            &chrome,
            StatusCode::UNSUPPORTED_MEDIA_TYPE,
            "afg-record-not-readable",
            &[],
            &back,
        );
    }
    if pdf && !rec::reads_pdf(cfg.kind) {
        return failed(
            &chrome,
            StatusCode::UNSUPPORTED_MEDIA_TYPE,
            "afg-record-pdf-unread",
            &[],
            &back,
        );
    }
    tracing::info!(
        account = viewer.name(),
        person = %id,
        document = %src.id,
        kind = cfg.kind.slug(),
        "AFG: a document sent to be read"
    );
    let prompt = rec::read_prompt(&src.filename, src.kind.mime, &src.bytes);
    let (text, model) = match rec::ask(&cfg, prompt, state.data_dir().to_path_buf()).await {
        Ok(a) => a,
        Err(f) => {
            tracing::warn!(person = %id, error = %f.message, "AFG: the document was not read");
            return failed(
                &chrome,
                StatusCode::BAD_GATEWAY,
                "afg-record-failed",
                &[("reason", f.message.into())],
                &back,
            );
        }
    };
    if rec::found_no_text(&text) {
        return failed(
            &chrome,
            StatusCode::UNPROCESSABLE_ENTITY,
            "afg-record-no-text",
            &[],
            &back,
        );
    }
    let name = src.caption.clone().unwrap_or_else(|| src.filename.clone());
    // The way to the document it was read from, under the transcription.
    let plain_id = src
        .id
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_');
    let markdown = if plain_id {
        format!(
            "{}\n\n---\n\n[{}](/document/{}/view)\n",
            text.trim(),
            md_text(&src.filename),
            src.id
        )
    } else {
        format!("{}\n", text.trim())
    };
    let d = Draft {
        title: clip(
            &chrome.t_args("afg-read-note-title", &[("name", name.into())]),
            notes::MAX_TITLE,
        ),
        category: chrome.t("afg-read-category"),
        date: src
            .doc
            .pointer("/date/value")
            .and_then(Value::as_str)
            .filter(|v| notes::date_of(v).is_some())
            .unwrap_or_default()
            .to_string(),
        persons: vec![id.clone()],
        markdown,
    };
    if let Some(key) = notes::refusal(&d) {
        return failed(&chrome, StatusCode::UNPROCESSABLE_ENTITY, key, &[], &back);
    }
    let note_id = uuid::Uuid::new_v4().to_string();
    let bytes = d.markdown.clone().into_bytes();
    let mut entity = notes::entity(&note_id, &d, viewer.name(), 1, &bytes, &[], None);
    entity["created_at"] = json!(crate::view::now_iso8601());
    entity["updated_at"] = json!(crate::view::now_iso8601());
    entity["version_num"] = json!(1);
    rec::mark_transcription(
        &mut entity,
        &src.id,
        Some(&rec::text_generator(&cfg, model.as_deref())),
    );
    if let Err(r) = check_scope(
        &state,
        &chrome,
        &viewer,
        axgf_rs::EntityKind::Document,
        &entity,
        None,
    ) {
        return r;
    }
    commit(
        &state,
        &viewer,
        &chrome,
        vec![Op::AddDocument {
            entity,
            payload: bytes,
        }],
        &back,
    )
}

#[derive(Debug, Default, Deserialize)]
pub struct SummaryForm {
    /// One of the site's languages; the page's when it is none of them.
    #[serde(default)]
    lang: String,
}

/// `POST /admin/person/:id/summary/ai` — the person's summary, written by
/// the AFG in the language asked from their record and the notes about them,
/// their files left out. It replaces the one there is, when this member may
/// change it.
pub async fn summary(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(id): Path<String>,
    Form(f): Form<SummaryForm>,
) -> Response {
    let (viewer, chrome) = match member(&state, &headers, &id) {
        Ok(v) => v,
        Err(r) => return r,
    };
    let back = format!("/person/{id}#summary");
    let cfg = state.site().afg;
    if cfg.kind == Kind::Off || cfg.problem().is_some() {
        return failed(
            &chrome,
            StatusCode::CONFLICT,
            "afg-record-text-off",
            &[],
            &back,
        );
    }
    let Some(target) = rec::language(&f.lang)
        .or_else(|| rec::language(chrome.lang))
        .or_else(|| crate::i18n::LOCALES.first())
    else {
        return failed(
            &chrome,
            StatusCode::BAD_REQUEST,
            "afg-record-text-off",
            &[],
            &back,
        );
    };
    // The sensitive classes go only when the AFG room says they may, as with
    // the archive (docs/adr/0023).
    let allowed = if cfg.sensitive {
        crate::sensitive::Scopes::EVERY
    } else {
        crate::sensitive::Scopes::NONE
    };
    let gathered = state.read_as(viewer.ceiling(), |flat, lens| {
        // Its labels in the summary's language, so the record and the
        // summary speak one.
        let view = crate::person::build_in(flat, &id, lens, target.tag)?;
        let profile = crate::person::profile_groups_within(flat, &id, lens, target.tag, allowed);
        let record = rec::record_for_summary(&view, &profile);
        let stored =
            rec::summary_of_person(flat, lens, &id).map(|(s, d)| (s.to_string(), d.clone()));
        let notes: Vec<(String, Option<String>)> = notes::notes_about(flat, lens, true, &id)
            .into_iter()
            .filter(|(_, d)| !rec::is_summary(d))
            .map(|(_, d)| (notes::title(d), notes::text_path(d)))
            .collect();
        Some((view.name, record, stored, notes))
    });
    let Some((name, record, stored, note_paths)) = gathered else {
        return failed(
            &chrome,
            StatusCode::NOT_FOUND,
            "error-no-such-person-detail",
            &[],
            &back,
        );
    };
    if let Some((_, doc)) = &stored {
        if !super::notes::may_change(&viewer, doc) {
            return failed(
                &chrome,
                StatusCode::FORBIDDEN,
                "afg-record-summary-not-yours",
                &[],
                &back,
            );
        }
    }
    let mut written = Vec::new();
    let mut budget = NOTES_BUDGET;
    for (title, path) in note_paths {
        if budget == 0 {
            break;
        }
        let Some(body) = path
            .and_then(|p| state.attachment(&p))
            .map(|b| String::from_utf8_lossy(&b).into_owned())
        else {
            continue;
        };
        let body = clip(&body, budget);
        budget = budget.saturating_sub(body.chars().count());
        written.push((title, body));
    }
    tracing::info!(
        account = viewer.name(),
        person = %id,
        notes = written.len(),
        language = target.tag,
        kind = cfg.kind.slug(),
        "AFG: a record sent to be summarised"
    );
    let prompt = rec::summary_prompt(target.english_name, &record, &written);
    let (text, model) = match rec::ask(&cfg, prompt, state.data_dir().to_path_buf()).await {
        Ok(a) => a,
        Err(f) => {
            tracing::warn!(person = %id, error = %f.message, "AFG: no summary");
            return failed(
                &chrome,
                StatusCode::BAD_GATEWAY,
                "afg-record-failed",
                &[("reason", f.message.into())],
                &back,
            );
        }
    };
    let generator = rec::text_generator(&cfg, model.as_deref());
    let bytes = text.clone().into_bytes();
    let ops = match stored {
        None => {
            let args = crate::i18n::args(&[("name", name.into())]);
            let d = Draft {
                title: clip(
                    &crate::i18n::translate(target.tag, "summary-note-title", Some(&args)),
                    notes::MAX_TITLE,
                ),
                category: crate::i18n::translate(target.tag, "summary-category", None),
                date: String::new(),
                persons: vec![id.clone()],
                markdown: text,
            };
            if let Some(key) = notes::refusal(&d) {
                return failed(&chrome, StatusCode::UNPROCESSABLE_ENTITY, key, &[], &back);
            }
            let sid = uuid::Uuid::new_v4().to_string();
            let mut entity = notes::entity(&sid, &d, viewer.name(), 1, &bytes, &[], None);
            entity["created_at"] = json!(crate::view::now_iso8601());
            entity["updated_at"] = json!(crate::view::now_iso8601());
            entity["version_num"] = json!(1);
            rec::mark_summary(&mut entity, &id, Some(target.tag), Some(&generator));
            if let Err(r) = check_scope(
                &state,
                &chrome,
                &viewer,
                axgf_rs::EntityKind::Document,
                &entity,
                None,
            ) {
                return r;
            }
            vec![Op::AddDocument {
                entity,
                payload: bytes,
            }]
        }
        Some((sid, doc)) => {
            let version = crate::state::version_of(&doc);
            let d = Draft {
                title: notes::title(&doc),
                category: notes::category(&doc).unwrap_or_default(),
                date: doc
                    .pointer("/date/value")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_string(),
                persons: notes::persons(&doc),
                markdown: text,
            };
            if let Some(key) = notes::refusal(&d) {
                return failed(&chrome, StatusCode::UNPROCESSABLE_ENTITY, key, &[], &back);
            }
            let mut entity = notes::entity(
                &sid,
                &d,
                viewer.name(),
                version + 1,
                &bytes,
                &[],
                Some(&doc),
            );
            rec::mark_summary(&mut entity, &id, Some(target.tag), Some(&generator));
            if let Err(r) = check_scope(
                &state,
                &chrome,
                &viewer,
                axgf_rs::EntityKind::Document,
                &entity,
                Some(&doc),
            ) {
                return r;
            }
            let mut ops = vec![Op::ReplaceDocument {
                id: sid.clone(),
                expected_version: version,
                entity,
                payload: bytes,
            }];
            // Pictures the summary showed and no longer does go with it.
            ops.extend(super::notes::pictures_dropped(&state, &sid, &d.markdown));
            ops
        }
    };
    commit(&state, &viewer, &chrome, ops, &back)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_file_name_is_shown_as_it_is_in_markdown() {
        assert_eq!(md_text("lettre [1932]_a.jpg"), "lettre \\[1932\\]\\_a.jpg");
        assert_eq!(clip("Grand-mère", 5), "Grand");
    }
}
