//! `/admin/person/:id/ai-portraits` — the portraits an AI reworked of one
//! person: up to three pictures beside the face, for the family's members
//! (docs/adr/0009, `crate::ai_portrait`).
//!
//! One page and one save. The page shows the ones there are, each with a
//! box that removes it, and as many file slots as are left; the save writes
//! every removal and every new picture in one commit, so a replacement is one
//! act and the archive is rewritten once.

use axum::extract::{Multipart, Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Redirect, Response};
use minijinja::context;
use serde_json::{json, Value};

use super::admin::{check_scope, clean_filename, io_error, require, Need};
use super::Shared;
use crate::ai_portrait::{self, Provenance, MAX};
use crate::render;
use crate::state::Op;

/// `GET /admin/person/:id/ai-portraits` — the page.
pub async fn page(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Response {
    let (viewer, chrome) = match require(&state, &headers, Need::Write) {
        Ok(v) => v,
        Err(r) => return r,
    };
    let view = state.read_as(viewer.ceiling(), |flat, lens| {
        if !lens.sees_person(&id) {
            return None;
        }
        crate::person::build_in(flat, &id, lens, chrome.lang)
    });
    let Some(view) = view else {
        return no_person(&chrome);
    };
    // Every one stored, even past three: a bundle written elsewhere may carry
    // more, and this is the page that can take them off.
    let current: Vec<Value> = state.read(|flat| {
        ai_portrait::of_person(flat, &id)
            .into_iter()
            .map(|(did, d)| {
                json!({
                    "id": did,
                    "filename": d.get("filename").and_then(Value::as_str).unwrap_or(did),
                    "caption": d.get("caption").and_then(Value::as_str),
                    "generator": ai_portrait::generator(d),
                })
            })
            .collect()
    });
    let slots = MAX.saturating_sub(current.len());
    render::page_with(
        &chrome,
        "admin_ai_portraits.html",
        context! {
            nav => "admin",
            id,
            person_name => view.name,
            current,
            slots,
            // The photographs a picture may have been made from: the
            // person's own images, as their gallery shows them.
            sources => view.images,
            max_upload_mb => crate::documents::MAX_UPLOAD / (1024 * 1024),
        },
    )
}

/// One new picture, as read from the form.
struct Upload {
    filename: String,
    bytes: Vec<u8>,
}

/// The form, read in any order.
#[derive(Default)]
struct Form {
    files: Vec<Upload>,
    remove: Vec<String>,
    generator: String,
    derived_from: String,
    caption: String,
    too_large: bool,
}

async fn read_form(mut multipart: Multipart) -> Form {
    let mut out = Form::default();
    loop {
        let field = match multipart.next_field().await {
            Ok(Some(f)) => f,
            Ok(None) => break,
            Err(e) => {
                out.too_large = e.status() == StatusCode::PAYLOAD_TOO_LARGE;
                break;
            }
        };
        match field.name().unwrap_or_default().to_string().as_str() {
            "file" => {
                let filename = field.file_name().unwrap_or("upload").to_string();
                match field.bytes().await {
                    // A slot left empty posts an empty part: not a picture.
                    Ok(b) if b.is_empty() => {}
                    Ok(b) => out.files.push(Upload {
                        filename,
                        bytes: b.to_vec(),
                    }),
                    Err(_) => out.too_large = true,
                }
            }
            "remove" => out.remove.push(field.text().await.unwrap_or_default()),
            "generator" => out.generator = field.text().await.unwrap_or_default(),
            "derived_from" => out.derived_from = field.text().await.unwrap_or_default(),
            "caption" => out.caption = field.text().await.unwrap_or_default(),
            _ => {
                let _ = field.bytes().await;
            }
        }
    }
    out
}

/// `POST /admin/person/:id/ai-portraits` — remove the ones ticked and add the
/// pictures sent, in one commit.
pub async fn save(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(id): Path<String>,
    multipart: Multipart,
) -> Response {
    let (viewer, chrome) = match require(&state, &headers, Need::Write) {
        Ok(v) => v,
        Err(r) => return r,
    };
    // Adding or removing a picture of somebody is a write against their
    // record, and is checked as one, as an upload is.
    if let Err(r) = check_scope(
        &state,
        &chrome,
        &viewer,
        axgf_rs::EntityKind::Person,
        &json!({"id": id}),
        None,
    ) {
        return r;
    }
    let sources: Option<Vec<String>> = state.read_as(viewer.ceiling(), |flat, lens| {
        if !lens.sees_person(&id) {
            return None;
        }
        crate::person::build_in(flat, &id, lens, chrome.lang)
            .map(|v| v.images.into_iter().map(|d| d.id).collect())
    });
    let Some(sources) = sources else {
        return no_person(&chrome);
    };

    let form = read_form(multipart).await;
    let refuse = |status: StatusCode, key: &str, args: &[(&str, fluent::FluentValue<'_>)]| {
        render::error_page_back_in(
            &chrome,
            status,
            "error-upload-title",
            key,
            args,
            Some((
                &format!("/admin/person/{id}/ai-portraits"),
                "ai-portraits-back",
            )),
        )
    };
    let too_large = || {
        refuse(
            StatusCode::PAYLOAD_TOO_LARGE,
            "error-upload-too-large",
            &[(
                "mb",
                ((crate::documents::MAX_UPLOAD / (1024 * 1024)) as i64).into(),
            )],
        )
    };
    if form.too_large {
        return too_large();
    }

    // What goes: only this person's own reworked portraits, at the version
    // they are stored at.
    let (stored, removals): (usize, Vec<(String, u64)>) = state.read(|flat| {
        let mine = ai_portrait::of_person(flat, &id);
        let removals = mine
            .iter()
            .filter(|(did, _)| form.remove.iter().any(|r| r.trim() == *did))
            .map(|(did, d)| (did.to_string(), crate::state::version_of(d)))
            .collect();
        (mine.len(), removals)
    });

    // What comes: pictures only, each within the upload limit, the type
    // read from the bytes and never from the name or the browser.
    let mut adds = Vec::new();
    for up in &form.files {
        if up.bytes.len() > crate::documents::MAX_UPLOAD {
            return too_large();
        }
        match crate::documents::sniff(&up.bytes) {
            Some(kind) if kind.raster_image => adds.push((up, kind)),
            _ => {
                return refuse(
                    StatusCode::UNSUPPORTED_MEDIA_TYPE,
                    "ai-portraits-images-only",
                    &[],
                )
            }
        }
    }
    if stored - removals.len() + adds.len() > MAX {
        return refuse(StatusCode::BAD_REQUEST, "ai-portraits-too-many", &[]);
    }

    let provenance = Provenance {
        generator: Some(form.generator.trim().to_string()),
        // Made from one of the person's own photographs, or not said: an id
        // the page did not offer is not recorded.
        derived_from_id: Some(form.derived_from.trim().to_string())
            .filter(|d| sources.iter().any(|s| s == d)),
        caption: Some(form.caption.trim().to_string()),
    };
    let mut ops: Vec<Op> = Vec::new();
    for (up, kind) in adds {
        let doc_id = uuid::Uuid::new_v4().to_string();
        let file = json!({
            "path": crate::documents::attachment_path(&doc_id, kind.ext),
            "size_bytes": up.bytes.len() as u64,
            "sha256": crate::documents::sha256_hex(&up.bytes),
        });
        let mut entity = ai_portrait::document(
            &doc_id,
            &id,
            &clean_filename(&up.filename, kind.ext),
            kind.mime,
            file,
            &provenance,
        );
        // Their order in the band is the order they arrived in.
        entity["created_at"] = json!(crate::view::now_iso8601());
        ops.push(Op::AddDocument {
            entity,
            payload: up.bytes.clone(),
        });
    }
    for (did, version) in removals {
        ops.push(Op::Delete {
            kind: axgf_rs::EntityKind::Document,
            id: did,
            expected_version: version,
        });
    }
    if ops.is_empty() {
        return Redirect::to(&format!("/admin/person/{id}/ai-portraits")).into_response();
    }

    match state.commit(ops, viewer.name(), |kind, e| {
        super::admin::label_for(kind, e)
    }) {
        Ok(crate::state::BatchOutcome::Applied { .. }) => {
            Redirect::to(&format!("/person/{id}#ai-portraits")).into_response()
        }
        Ok(crate::state::BatchOutcome::Conflict { .. }) => {
            refuse(StatusCode::CONFLICT, "ai-portraits-stale", &[])
        }
        Ok(crate::state::BatchOutcome::Refused { diagnostics, .. }) => refuse(
            StatusCode::BAD_REQUEST,
            "error-upload-refused",
            &[(
                "reason",
                crate::state::format_diagnostics(&diagnostics).into(),
            )],
        ),
        Err(e) => io_error(&chrome, &e),
    }
}

fn no_person(chrome: &render::Chrome) -> Response {
    render::error_page_in(
        chrome,
        StatusCode::NOT_FOUND,
        "error-no-such-person-title",
        "error-no-such-person-detail",
    )
}
