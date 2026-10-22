//! Notes and contributions (docs/adr/0014, `crate::notes`).
//!
//! Written by administrators and contributors at `/admin/notes`; read by the
//! family's members on the records they are about, on the family's page, in
//! a window (`/notes/:id?frame=1`) and as a PDF (`/notes/:id/pdf`).
//!
//! A note and its pictures are saved in one commit: the text's file, the
//! record, every picture pasted since the page opened, and the removal of the
//! pictures the text no longer shows. A picture pasted waits in memory
//! (`crate::notes::PendingImages`) until then, so a note abandoned leaves
//! nothing in the family's archive.

use std::collections::HashMap;

use axum::extract::{Multipart, Path, Query, State};
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::{IntoResponse, Redirect, Response};
use axum::Form;
use minijinja::context;
use serde::Deserialize;
use serde_json::{json, Value};

use super::admin::{check_scope, io_error, require, Need};
use super::Shared;
use crate::notes::{self, Draft};
use crate::state::{BatchOutcome, Op};
use crate::{auth, render};

/// What the writing form sent, its people resolved to ids.
fn draft_from(
    state: &Shared,
    fields: &[(String, String)],
) -> Result<Draft, (String, &'static str)> {
    let get = |k: &str| {
        fields
            .iter()
            .find(|(f, _)| f == k)
            .map(|(_, v)| v.clone())
            .unwrap_or_default()
    };
    let mut d = Draft {
        title: get("title"),
        category: get("category"),
        date: get("date"),
        markdown: get("markdown").replace("\r\n", "\n"),
        persons: Vec::new(),
    };
    let typed: Vec<String> = fields
        .iter()
        .filter(|(f, v)| (f == "person" || f == "person_add") && !v.trim().is_empty())
        .map(|(_, v)| v.trim().to_string())
        .collect();
    for t in typed {
        let id = state
            .read(|flat| crate::forms::resolve_person(&t, flat))
            .map_err(|e| (t.clone(), e.key()))?;
        if !d.persons.contains(&id) {
            d.persons.push(id);
        }
    }
    Ok(d)
}

/// Whether this account may change this note: an administrator any, a
/// contributor the ones they wrote.
fn may_change(viewer: &crate::access::Viewer, doc: &Value) -> bool {
    viewer.is_admin() || notes::author(doc).as_deref() == Some(viewer.name())
}

// ---------------------------------------------------------------------------
// writing
// ---------------------------------------------------------------------------

#[derive(Deserialize, Default)]
pub struct ListQuery {
    #[serde(default)]
    saved: Option<String>,
    #[serde(default)]
    deleted: Option<String>,
}

/// `GET /admin/notes` — every note this writer may read, with the way to
/// write another.
pub async fn list(
    State(state): State<Shared>,
    headers: HeaderMap,
    Query(q): Query<ListQuery>,
) -> Response {
    let (viewer, chrome) = match require(&state, &headers, Need::Write) {
        Ok(v) => v,
        Err(r) => return r,
    };
    let rows: Vec<Value> = state.read_as(viewer.ceiling(), |flat, lens| {
        notes::readable_notes(flat, lens, true)
            .into_iter()
            .map(|(id, d)| {
                let v = notes::view_of(flat, id, d, chrome.lang);
                json!({
                    "view": v,
                    "may_change": may_change(&viewer, d),
                })
            })
            .collect()
    });
    render::page_with(
        &chrome,
        "admin_notes.html",
        context! {
            nav => "admin",
            rows,
            saved => q.saved.is_some(),
            deleted => q.deleted.is_some(),
        },
    )
}

#[derive(Deserialize, Default)]
pub struct NewQuery {
    #[serde(default)]
    person: Option<String>,
}

/// The writing form's context: the note as it stands (or the draft sent
/// back with an error), the people pickable, the categories offered.
fn form_page(
    state: &Shared,
    viewer: &crate::access::Viewer,
    chrome: &render::Chrome,
    id: Option<&str>,
    version: u64,
    d: &Draft,
    error: Option<(String, StatusCode)>,
) -> Response {
    let status = error.as_ref().map_or(StatusCode::OK, |e| e.1);
    let error = error.map(|e| e.0);
    let (options, chosen, categories) = state.read_as(viewer.ceiling(), |flat, lens| {
        let options = crate::forms::person_options(flat, lens);
        let chosen: Vec<Value> = d
            .persons
            .iter()
            .filter_map(|pid| {
                let p = flat.get("persons")?.get(pid)?;
                Some(json!({"id": pid, "label": crate::forms::person_label(pid, p)}))
            })
            .collect();
        let mut categories: Vec<String> = notes::readable_notes(flat, lens, true)
            .into_iter()
            .filter_map(|(_, d)| notes::category(d))
            .collect();
        for s in chrome.t("notes-category-suggestions").split(',') {
            categories.push(s.trim().to_string());
        }
        categories.retain(|c| !c.is_empty());
        categories.sort();
        categories.dedup();
        (options, chosen, categories)
    });
    let preview = notes::to_html(&d.markdown, &notes::resolve_for_page);
    let mut resp = render::page_with(
        chrome,
        "admin_note.html",
        context! {
            nav => "admin",
            id,
            note_version => version,
            title => d.title,
            category => d.category,
            date => d.date,
            markdown => d.markdown,
            chosen,
            options,
            categories,
            preview,
            error,
            max_image_mb => notes::MAX_IMAGE / (1024 * 1024),
        },
    );
    *resp.status_mut() = status;
    resp
}

/// `GET /admin/notes/new` — a blank note, about a person when one is named.
pub async fn new_form(
    State(state): State<Shared>,
    headers: HeaderMap,
    Query(q): Query<NewQuery>,
) -> Response {
    let (viewer, chrome) = match require(&state, &headers, Need::Write) {
        Ok(v) => v,
        Err(r) => return r,
    };
    let mut d = Draft::default();
    if let Some(p) = q
        .person
        .filter(|p| state.read(|flat| flat.pointer(&format!("/persons/{p}")).is_some()))
    {
        d.persons.push(p);
    }
    form_page(&state, &viewer, &chrome, None, 0, &d, None)
}

/// `GET /admin/notes/:id/edit`
pub async fn edit_form(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Response {
    let (viewer, chrome) = match require(&state, &headers, Need::Write) {
        Ok(v) => v,
        Err(r) => return r,
    };
    let found = state.read_as(viewer.ceiling(), |flat, lens| {
        let doc = flat.get("documents")?.get(&id)?;
        notes::readable(flat, lens, true, doc).then(|| doc.clone())
    });
    let Some(doc) = found else {
        return no_note(&chrome);
    };
    if !may_change(&viewer, &doc) {
        return forbidden(&chrome);
    }
    let text = notes::text_path(&doc)
        .and_then(|p| state.attachment(&p))
        .map(|b| String::from_utf8_lossy(&b).into_owned())
        .unwrap_or_default();
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
    let version = crate::state::version_of(&doc);
    form_page(&state, &viewer, &chrome, Some(&id), version, &d, None)
}

/// The pictures a text names: those pasted since the page opened become
/// Documents of their own, named in the text by their new ids; those the
/// note already had and the text still shows are kept. The ones it no longer
/// shows are `pictures_dropped`.
struct Pictures {
    markdown: String,
    ops: Vec<Op>,
    ids: Vec<String>,
    /// The waiting pictures this save uses: released only once it is
    /// committed, so a save that fails loses none of them.
    tokens: Vec<String>,
}

fn pictures(
    state: &Shared,
    viewer: &crate::access::Viewer,
    note_id: &str,
    markdown: &str,
    stored: Option<&Value>,
) -> Result<Pictures, &'static str> {
    let pending = notes::refs(markdown, notes::PENDING_REF);
    // The pictures the note had and its text still shows stay, first.
    let shown = notes::refs(markdown, notes::DOCUMENT_REF);
    let kept: Vec<String> = stored
        .map(notes::images)
        .unwrap_or_default()
        .into_iter()
        .filter(|i| shown.contains(i))
        .collect();
    if kept.len() + pending.len() > notes::MAX_IMAGES {
        return Err("notes-error-too-many-images");
    }
    let mut markdown = markdown.to_string();
    let mut ops = Vec::new();
    let mut ids = kept;
    for token in &pending {
        let Some(img) = state.note_images().get(viewer.name(), token) else {
            // Pasted in another session, or waited too long: the picture is
            // no longer here, and the text is not saved naming nothing.
            return Err("notes-error-image-gone");
        };
        let image_id = uuid::Uuid::new_v4().to_string();
        markdown = markdown.replace(
            &format!("{}{token}", notes::PENDING_REF),
            &format!("{}{image_id}", notes::DOCUMENT_REF),
        );
        let mut entity = notes::image_entity(note_id, &image_id, &img);
        entity["created_at"] = json!(crate::view::now_iso8601());
        ops.push(Op::AddDocument {
            entity,
            payload: img.bytes,
        });
        ids.push(image_id);
    }
    Ok(Pictures {
        markdown,
        ops,
        ids,
        tokens: pending,
    })
}

/// `POST /admin/notes` — a new note.
pub async fn create(
    State(state): State<Shared>,
    headers: HeaderMap,
    Form(fields): Form<Vec<(String, String)>>,
) -> Response {
    let (viewer, chrome) = match require(&state, &headers, Need::Write) {
        Ok(v) => v,
        Err(r) => return r,
    };
    save(&state, &viewer, &chrome, None, &fields).await
}

/// `POST /admin/notes/:id` — the note, changed.
pub async fn update(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(id): Path<String>,
    Form(fields): Form<Vec<(String, String)>>,
) -> Response {
    let (viewer, chrome) = match require(&state, &headers, Need::Write) {
        Ok(v) => v,
        Err(r) => return r,
    };
    save(&state, &viewer, &chrome, Some(id), &fields).await
}

async fn save(
    state: &Shared,
    viewer: &crate::access::Viewer,
    chrome: &render::Chrome,
    id: Option<String>,
    fields: &[(String, String)],
) -> Response {
    let version: u64 = fields
        .iter()
        .find(|(k, _)| k == "base_version")
        .and_then(|(_, v)| v.parse().ok())
        .unwrap_or(0);
    let mut d = match draft_from(state, fields) {
        Ok(d) => d,
        Err((typed, key)) => {
            let mut d = Draft::default();
            for (k, v) in fields {
                match k.as_str() {
                    "title" => d.title = v.clone(),
                    "category" => d.category = v.clone(),
                    "date" => d.date = v.clone(),
                    "markdown" => d.markdown = v.clone(),
                    _ => {}
                }
            }
            let msg = format!(
                "{} {}",
                chrome.t_args("notes-error-person", &[("name", typed.into())]),
                chrome.t(key)
            );
            return form_page(
                state,
                viewer,
                chrome,
                id.as_deref(),
                version,
                &d,
                Some((msg, StatusCode::BAD_REQUEST)),
            );
        }
    };
    if let Some(key) = notes::refusal(&d) {
        return form_page(
            state,
            viewer,
            chrome,
            id.as_deref(),
            version,
            &d,
            Some((chrome.t(key), StatusCode::BAD_REQUEST)),
        );
    }
    let stored = match &id {
        None => None,
        Some(id) => {
            let found = state.read_as(viewer.ceiling(), |flat, lens| {
                let doc = flat.get("documents")?.get(id)?;
                notes::readable(flat, lens, true, doc).then(|| doc.clone())
            });
            match found {
                Some(doc) if may_change(viewer, &doc) => Some(doc),
                Some(_) => return forbidden(chrome),
                None => return no_note(chrome),
            }
        }
    };
    let note_id = id
        .clone()
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
    let pics = match pictures(state, viewer, &note_id, &d.markdown, stored.as_ref()) {
        Ok(p) => p,
        Err(key) => {
            return form_page(
                state,
                viewer,
                chrome,
                id.as_deref(),
                version,
                &d,
                Some((chrome.t(key), StatusCode::BAD_REQUEST)),
            )
        }
    };
    // What the writer wrote, pending pictures and all: the form shows it again
    // if the save fails, and the pictures are still waiting for it.
    let written = std::mem::replace(&mut d.markdown, pics.markdown);
    let text = d.markdown.clone().into_bytes();
    let next = if stored.is_some() { version + 1 } else { 1 };
    let mut entity = notes::entity(
        &note_id,
        &d,
        viewer.name(),
        next,
        &text,
        &pics.ids,
        stored.as_ref(),
    );
    if stored.is_none() {
        entity["created_at"] = json!(crate::view::now_iso8601());
        entity["updated_at"] = json!(crate::view::now_iso8601());
        entity["version_num"] = json!(1);
    }
    if let Err(r) = check_scope(
        state,
        chrome,
        viewer,
        axgf_rs::EntityKind::Document,
        &entity,
        stored.as_ref(),
    ) {
        return r;
    }
    // The pictures first, so that the note that names them follows them.
    let mut ops = pics.ops;
    ops.push(match &stored {
        None => Op::AddDocument {
            entity,
            payload: text,
        },
        Some(_) => Op::ReplaceDocument {
            id: note_id.clone(),
            expected_version: version,
            entity,
            payload: text,
        },
    });
    if stored.is_some() {
        // What is dropped goes last, once the note no longer names it.
        ops.extend(pictures_dropped(state, &note_id, &d.markdown));
    }
    let (message, status) = match state.commit(ops, viewer.name(), |kind, e| {
        super::admin::label_for(kind, e)
    }) {
        Ok(BatchOutcome::Applied { .. }) => {
            state.note_images().take(viewer.name(), &pics.tokens);
            return Redirect::to(&format!("/notes/{note_id}")).into_response();
        }
        Ok(BatchOutcome::Conflict { .. }) => (chrome.t("notes-error-stale"), StatusCode::CONFLICT),
        Ok(BatchOutcome::Refused { diagnostics, .. }) => (
            crate::state::format_diagnostics(&diagnostics),
            StatusCode::BAD_REQUEST,
        ),
        Err(e) => return io_error(chrome, &e),
    };
    d.markdown = written;
    form_page(
        state,
        viewer,
        chrome,
        id.as_deref(),
        version,
        &d,
        Some((message, status)),
    )
}

/// The pictures a note had that its text no longer shows.
fn pictures_dropped(state: &Shared, note_id: &str, markdown: &str) -> Vec<Op> {
    let shown = notes::refs(markdown, notes::DOCUMENT_REF);
    state.read(|flat| {
        let Some(doc) = flat.get("documents").and_then(|d| d.get(note_id)) else {
            return Vec::new();
        };
        notes::images(doc)
            .into_iter()
            .filter(|i| !shown.contains(i))
            .filter_map(|i| {
                let d = flat.get("documents")?.get(&i)?;
                Some(Op::DropDocument {
                    expected_version: crate::state::version_of(d),
                    id: i,
                })
            })
            .collect()
    })
}

#[derive(Deserialize)]
pub struct DeleteForm {
    #[serde(default)]
    base_version: u64,
}

/// `POST /admin/notes/:id/delete` — the note and its pictures.
pub async fn delete(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(id): Path<String>,
    Form(form): Form<DeleteForm>,
) -> Response {
    let (viewer, chrome) = match require(&state, &headers, Need::Write) {
        Ok(v) => v,
        Err(r) => return r,
    };
    let found = state.read_as(viewer.ceiling(), |flat, lens| {
        let doc = flat.get("documents")?.get(&id)?;
        notes::readable(flat, lens, true, doc).then(|| doc.clone())
    });
    let Some(doc) = found else {
        return no_note(&chrome);
    };
    if !may_change(&viewer, &doc) {
        return forbidden(&chrome);
    }
    if let Err(r) = check_scope(
        &state,
        &chrome,
        &viewer,
        axgf_rs::EntityKind::Document,
        &doc,
        None,
    ) {
        return r;
    }
    let mut ops = vec![Op::DropDocument {
        id: id.clone(),
        expected_version: form.base_version,
    }];
    state.read(|flat| {
        for i in notes::images(&doc) {
            if let Some(d) = flat.get("documents").and_then(|c| c.get(&i)) {
                ops.push(Op::DropDocument {
                    expected_version: crate::state::version_of(d),
                    id: i,
                });
            }
        }
    });
    match state.commit(ops, viewer.name(), |kind, e| {
        super::admin::label_for(kind, e)
    }) {
        Ok(BatchOutcome::Applied { .. }) => Redirect::to("/admin/notes?deleted=1").into_response(),
        Ok(BatchOutcome::Conflict { .. }) => render::error_page_in(
            &chrome,
            StatusCode::CONFLICT,
            "notes-stale-title",
            "notes-error-stale",
        ),
        Ok(BatchOutcome::Refused { .. }) => render::error_page_in(
            &chrome,
            StatusCode::BAD_REQUEST,
            "notes-stale-title",
            "notes-error-refused",
        ),
        Err(e) => io_error(&chrome, &e),
    }
}

/// `POST /admin/notes/images` — pictures pasted, dropped or chosen while
/// writing, held until the note is saved. Answers with how the text names
/// each: `[{"ref": "axgf:pending/…", "url": "…", "name": "…"}]`.
pub async fn images(
    State(state): State<Shared>,
    headers: HeaderMap,
    mut multipart: Multipart,
) -> Response {
    let (viewer, chrome) = match require(&state, &headers, Need::Write) {
        Ok(v) => v,
        Err(r) => return r,
    };
    let mut out = Vec::new();
    loop {
        let field = match multipart.next_field().await {
            Ok(Some(f)) => f,
            Ok(None) => break,
            Err(_) => return json_error(StatusCode::BAD_REQUEST, &chrome.t("notes-error-image")),
        };
        let name = field.file_name().unwrap_or("image").to_string();
        let bytes = match field.bytes().await {
            Ok(b) => b.to_vec(),
            Err(_) => {
                return json_error(
                    StatusCode::PAYLOAD_TOO_LARGE,
                    &chrome.t("notes-error-image-large"),
                )
            }
        };
        if bytes.is_empty() {
            continue;
        }
        if bytes.len() > notes::MAX_IMAGE {
            return json_error(
                StatusCode::PAYLOAD_TOO_LARGE,
                &chrome.t("notes-error-image-large"),
            );
        }
        match state.note_images().put(viewer.name(), &name, bytes) {
            Some(token) => out.push(json!({
                "ref": format!("{}{token}", notes::PENDING_REF),
                "url": format!("/admin/notes/pending/{token}"),
                "name": name,
            })),
            None => {
                return json_error(
                    StatusCode::UNSUPPORTED_MEDIA_TYPE,
                    &chrome.t("notes-error-image"),
                )
            }
        }
    }
    (
        [(header::CONTENT_TYPE, "application/json")],
        Value::Array(out).to_string(),
    )
        .into_response()
}

fn json_error(status: StatusCode, message: &str) -> Response {
    (
        status,
        [(header::CONTENT_TYPE, "application/json")],
        json!({ "error": message }).to_string(),
    )
        .into_response()
}

/// `GET /admin/notes/pending/:token` — a pasted picture, to the one who
/// pasted it, for the writing form's preview.
pub async fn pending(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(token): Path<String>,
) -> Response {
    let (viewer, chrome) = match require(&state, &headers, Need::Write) {
        Ok(v) => v,
        Err(r) => return r,
    };
    match state.note_images().get(viewer.name(), &token) {
        Some(img) => (
            [
                (header::CONTENT_TYPE, img.mime.to_string()),
                (header::CACHE_CONTROL, "private, no-store".to_string()),
                (
                    header::HeaderName::from_static("x-content-type-options"),
                    "nosniff".to_string(),
                ),
            ],
            img.bytes,
        )
            .into_response(),
        None => no_note(&chrome),
    }
}

#[derive(Deserialize)]
pub struct PreviewForm {
    #[serde(default)]
    markdown: String,
}

/// `POST /admin/notes/preview` — the text as it will read.
pub async fn preview(
    State(state): State<Shared>,
    headers: HeaderMap,
    Form(form): Form<PreviewForm>,
) -> Response {
    if let Err(r) = require(&state, &headers, Need::Write) {
        return r;
    }
    (
        [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
        notes::to_html(&form.markdown, &notes::resolve_for_page),
    )
        .into_response()
}

// ---------------------------------------------------------------------------
// reading
// ---------------------------------------------------------------------------

#[derive(Deserialize, Default)]
pub struct NoteQuery {
    #[serde(default)]
    frame: Option<String>,
}

/// The note, whole, for this reader — or nothing, whether it is not there or
/// not theirs to read: a visitor is not told a note exists.
fn full(
    state: &Shared,
    viewer: &crate::access::Viewer,
    lang: &str,
    id: &str,
) -> Option<(notes::NoteFull, Value)> {
    let found = state.read_as(viewer.ceiling(), |flat, lens| {
        let doc = flat.get("documents")?.get(id)?;
        notes::readable(flat, lens, viewer.signed_in(), doc)
            .then(|| (notes::view_of(flat, id, doc, lang), doc.clone()))
    });
    let (view, doc) = found?;
    let markdown = notes::text_path(&doc)
        .and_then(|p| state.attachment(&p))
        .map(|b| String::from_utf8_lossy(&b).into_owned())
        .unwrap_or_default();
    let html = notes::to_html(&markdown, &|dest: &str| {
        // A picture of this note's only; a pending one never reaches a reader.
        dest.starts_with(notes::DOCUMENT_REF)
            .then(|| notes::resolve_for_page(dest))
            .flatten()
    });
    Some((
        notes::NoteFull {
            view,
            html,
            markdown,
            updated_at: doc
                .get("updated_at")
                .and_then(Value::as_str)
                .map(str::to_string),
        },
        doc,
    ))
}

/// `GET /notes/:id` — the note: a page, or the window's content with
/// `?frame=1`.
pub async fn show(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(id): Path<String>,
    Query(q): Query<NoteQuery>,
) -> Response {
    let viewer = auth::viewer(&state, &headers);
    let chrome = render::Chrome::resolve(&viewer, &headers, &format!("/notes/{id}"));
    let Some((note, doc)) = full(&state, &viewer, chrome.lang, &id) else {
        return no_note(&chrome);
    };
    let may = viewer.may_write() && may_change(&viewer, &doc);
    let template = if q.frame.is_some() {
        "_note_frame.html"
    } else {
        "note.html"
    };
    let mut resp = render::page_with(
        &chrome,
        template,
        context! { nav => "about", note, may_change => may },
    );
    resp.headers_mut().insert(
        header::CACHE_CONTROL,
        header::HeaderValue::from_static("private, no-store"),
    );
    resp
}

/// `GET /notes/:id/pdf` — the note as a PDF, its pictures in it.
pub async fn pdf(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Response {
    let viewer = auth::viewer(&state, &headers);
    let chrome = render::Chrome::resolve(&viewer, &headers, &format!("/notes/{id}"));
    let Some((note, _)) = full(&state, &viewer, chrome.lang, &id) else {
        return no_note(&chrome);
    };
    let v = &note.view;
    let mut meta = Vec::new();
    let mut first: Vec<String> = Vec::new();
    if let Some(c) = &v.category {
        first.push(c.clone());
    }
    if let Some(d) = &v.date {
        first.push(d.text.clone());
    }
    if let Some(a) = &v.author {
        first.push(chrome.t_args("notes-by", &[("author", a.clone().into())]));
    }
    if !first.is_empty() {
        meta.push(first.join("  ·  "));
    }
    if v.persons.is_empty() {
        meta.push(chrome.t("notes-about-family"));
    } else {
        let names: Vec<String> = v.persons.iter().map(|p| p.name.clone()).collect();
        meta.push(chrome.t_args("notes-about", &[("names", names.join(", ").into())]));
    }
    let family = state
        .read(notes::family_name)
        .unwrap_or_else(|| chrome.t("home-unnamed-family"));
    let footer = format!("{family} — {}", chrome.t("app-name"));
    let readable_image = |dest: &str| -> Option<Vec<u8>> {
        let img = dest.strip_prefix(notes::DOCUMENT_REF)?;
        let img: String = img
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '-')
            .collect();
        let path = state.read_as(viewer.ceiling(), |flat, lens| {
            let d = flat.get("documents")?.get(&img)?;
            (notes::is_note_image(d) && notes::part_readable(flat, lens, viewer.signed_in(), d))
                .then(|| notes::text_path(d))
                .flatten()
        })?;
        state.attachment(&path)
    };
    let title = v.title.clone();
    let rendered = crate::note_pdf::render(
        &crate::note_pdf::PdfNote {
            title: &title,
            meta,
            markdown: &note.markdown,
            footer,
            language: chrome.lang.to_string(),
        },
        &readable_image,
    );
    match rendered {
        Ok(bytes) => (
            [
                (header::CONTENT_TYPE, "application/pdf".to_string()),
                (
                    header::CONTENT_DISPOSITION,
                    format!("attachment; filename=\"{}.pdf\"", notes::slug(&title)),
                ),
                (header::CACHE_CONTROL, "private, no-store".to_string()),
            ],
            bytes,
        )
            .into_response(),
        Err(e) => {
            tracing::error!(note = %id, error = %e, "note pdf failed");
            render::error_page_in(
                &chrome,
                StatusCode::INTERNAL_SERVER_ERROR,
                "notes-pdf-failed-title",
                "notes-pdf-failed",
            )
        }
    }
}

fn no_note(chrome: &render::Chrome) -> Response {
    render::error_page_in(
        chrome,
        StatusCode::NOT_FOUND,
        "notes-missing-title",
        "notes-missing",
    )
}

fn forbidden(chrome: &render::Chrome) -> Response {
    render::error_page_in(
        chrome,
        StatusCode::FORBIDDEN,
        "access-role-title",
        "notes-not-yours",
    )
}

/// The family page's notes: what this reader may read, filtered by what was
/// searched and the letter chosen, and the letters there are.
pub fn family_context(
    state: &Shared,
    viewer: &crate::access::Viewer,
    lang: &str,
    q: &str,
    letter: &str,
) -> Value {
    if !viewer.signed_in() {
        // A visitor is told notes exist, so as to sign in, and nothing more.
        let exist = state.read(|flat| {
            flat.get("documents")
                .and_then(Value::as_object)
                .is_some_and(|d| d.values().any(notes::is_note))
        });
        return json!({ "signed_in": false, "exist": exist });
    }
    let needle = notes::fold(q.trim());
    let (all, letters): (Vec<notes::NoteView>, Vec<String>) =
        state.read_as(viewer.ceiling(), |flat, lens| {
            let found = notes::readable_notes(flat, lens, true);
            let mut letters: Vec<String> = Vec::new();
            let mut out = Vec::new();
            for (id, d) in found {
                let v = notes::view_of(flat, id, d, lang);
                if !letters.contains(&v.letter) {
                    letters.push(v.letter.clone());
                }
                out.push(v);
            }
            (out, letters)
        });
    // The search reads the whole text too, not only what the card shows.
    let texts: HashMap<String, String> = if needle.is_empty() {
        HashMap::new()
    } else {
        all.iter()
            .filter(|v| !v.haystack.contains(&needle))
            .filter_map(|v| {
                let path = state
                    .read(|flat| flat.get("documents")?.get(&v.id).and_then(notes::text_path))?;
                let text = state.attachment(&path)?;
                Some((
                    v.id.clone(),
                    notes::fold(&notes::plain_text(&String::from_utf8_lossy(&text))),
                ))
            })
            .collect()
    };
    let total = all.len();
    let shown: Vec<notes::NoteView> = all
        .into_iter()
        .filter(|v| letter.is_empty() || v.letter == letter)
        .filter(|v| {
            needle.is_empty()
                || v.haystack.contains(&needle)
                || texts.get(&v.id).is_some_and(|t| t.contains(&needle))
        })
        .collect();
    // A to Z, then any other letter a title begins with (Cyrillic, Greek…),
    // then `#`, each said present or not, so that the row stays one width and
    // a letter without notes is not a dead link.
    let mut others: Vec<String> = letters
        .iter()
        .filter(|l| l.as_str() != "#" && !(l.len() == 1 && l.as_bytes()[0].is_ascii_uppercase()))
        .cloned()
        .collect();
    others.sort();
    let alphabet: Vec<Value> = ('A'..='Z')
        .map(|c| c.to_string())
        .chain(others)
        .chain(std::iter::once("#".to_string()))
        .map(|l| {
            let query: String = l
                .bytes()
                .map(|b| {
                    if b.is_ascii_alphanumeric() {
                        (b as char).to_string()
                    } else {
                        format!("%{b:02X}")
                    }
                })
                .collect();
            json!({ "present": letters.contains(&l), "letter": l, "query": query })
        })
        .collect();
    json!({
        "signed_in": true,
        "may_write": viewer.may_write(),
        "notes": shown,
        "total": total,
        "alphabet": alphabet,
        "q": q,
        "letter": letter,
    })
}
