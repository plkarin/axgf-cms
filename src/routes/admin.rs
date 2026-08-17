//! The admin panel: server-rendered forms, no client-side state.
//!
//! Every mutation goes through [`AppState::mutate`], so the atomic-write and
//! refuse-cleanly guarantees apply uniformly. Diagnostics are always shown —
//! warnings never block, mirroring the library's non-blocking philosophy.

use std::collections::HashMap;

use axum::extract::{Form, Path, Query, State};
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::{IntoResponse, Redirect, Response};
use minijinja::context;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::admin::{
    apply_form, fields_for, get_path, kind_from_str, paginate, policy_from_str, KINDS,
};
use crate::routes::Shared;
use crate::state::MutationOutcome;
use crate::{auth, documents, render, view};

/// Guard every admin page. Returns `Err(response)` when not signed in.
///
/// The error variant is a whole rendered `Response`, which is large; boxing it
/// keeps the common `Ok` path cheap.
#[allow(clippy::result_large_err)]
fn require_admin(state: &Shared, headers: &HeaderMap) -> Result<(), Response> {
    if auth::is_admin(headers, state.admin_token()) {
        Ok(())
    } else {
        // 401 rather than a redirect: this is the answer an integration test
        // and a script both need, and the body still points at the form.
        Err((
            StatusCode::UNAUTHORIZED,
            render::page(
                "admin_login.html",
                context! {
                    nav => "admin",
                    is_admin => false,
                    error => "Sign in to reach the admin panel.",
                },
            ),
        )
            .into_response())
    }
}

macro_rules! guard {
    ($state:expr, $headers:expr) => {
        if let Err(r) = require_admin(&$state, &$headers) {
            return r;
        }
    };
}

// ---------------------------------------------------------------------------
// auth
// ---------------------------------------------------------------------------

/// `GET /admin/login`
pub async fn login_form(State(state): State<Shared>, headers: HeaderMap) -> Response {
    if auth::is_admin(&headers, state.admin_token()) {
        return Redirect::to("/admin").into_response();
    }
    render::page(
        "admin_login.html",
        context! { nav => "admin", is_admin => false, error => "" },
    )
}

#[derive(Deserialize)]
pub struct LoginForm {
    #[serde(default)]
    token: String,
}

/// `POST /admin/login`
pub async fn login(State(state): State<Shared>, Form(f): Form<LoginForm>) -> Response {
    if !f.token.is_empty() && f.token == state.admin_token() {
        return (
            [(header::SET_COOKIE, auth::set_cookie(&f.token))],
            Redirect::to("/admin"),
        )
            .into_response();
    }
    (
        StatusCode::UNAUTHORIZED,
        render::page(
            "admin_login.html",
            context! {
                nav => "admin",
                is_admin => false,
                error => "That token is not correct.",
            },
        ),
    )
        .into_response()
}

/// `POST /admin/logout`
pub async fn logout() -> Response {
    (
        [(header::SET_COOKIE, auth::clear_cookie())],
        Redirect::to("/"),
    )
        .into_response()
}

// ---------------------------------------------------------------------------
// dashboard
// ---------------------------------------------------------------------------

/// `GET /admin`
pub async fn dashboard(State(state): State<Shared>, headers: HeaderMap) -> Response {
    guard!(state, headers);
    let counts = state.counts();
    let env = state.inspect_with(axgf_rs::validate);
    let diagnostics = diagnostics_json(&env.diagnostics);
    // Validation says what is wrong; this says what is missing.
    let completeness = state.read(crate::completeness::analyse);

    render::page(
        "admin_dashboard.html",
        context! {
            nav => "admin",
            is_admin => true,
            kinds => KINDS,
            // COLLECTIONS and KINDS are the same eight in the same order, so
            // each tile can link to its singular admin listing.
            counts => counts.iter().zip(KINDS.iter())
                            .map(|((k, n), singular)| json!({
                                "kind": k, "n": n, "singular": singular }))
                            .collect::<Vec<_>>(),
            total => counts.iter().map(|(_, n)| n).sum::<usize>(),
            validation => env.data,
            diagnostics,
            bundle_path => state.bundle_path().display().to_string(),
            bundle_size => documents::human_size(state.bundle_size()),
            // The bundle is read into memory whole at startup, so its size is
            // the application's resident cost, not just a number on disk.
            bundle_heavy => state.bundle_size() > state.size_warn(),
            size_warn => documents::human_size(state.size_warn()),
            attachment_count => state.read(|flat| flat.get("attachments")
                .and_then(Value::as_object).map(|m| m.len()).unwrap_or(0)),
            completeness,
        },
    )
}

#[derive(Deserialize)]
pub struct ListQuery {
    #[serde(default = "one")]
    page: usize,
    #[serde(default)]
    q: String,
}
fn one() -> usize {
    1
}

/// `GET /admin/:kind` — a paginated, filterable listing.
pub async fn list(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(kind): Path<String>,
    Query(q): Query<ListQuery>,
) -> Response {
    guard!(state, headers);
    let Some(k) = kind_from_str(&kind) else {
        return unknown_kind(&kind);
    };

    let needle = q.q.trim().to_lowercase();
    let rows = state.read(|flat| {
        let empty = serde_json::Map::new();
        let coll = flat
            .get(k.collection())
            .and_then(Value::as_object)
            .unwrap_or(&empty);
        let mut rows: Vec<Value> = coll
            .iter()
            .map(|(id, e)| {
                json!({
                    "id": id,
                    "summary": summarize(k, e),
                    "confidence": e.get("confidence").and_then(Value::as_f64)
                                   .map(view::Confidence::new),
                })
            })
            .filter(|r| {
                needle.is_empty()
                    || r["summary"]
                        .as_str()
                        .unwrap_or("")
                        .to_lowercase()
                        .contains(&needle)
                    || r["id"]
                        .as_str()
                        .unwrap_or("")
                        .to_lowercase()
                        .contains(&needle)
            })
            .collect();
        rows.sort_by(|a, b| a["summary"].as_str().cmp(&b["summary"].as_str()));
        rows
    });

    let page = paginate(rows, q.page);
    render::page(
        "admin_list.html",
        context! {
            nav => "admin",
            is_admin => true,
            kind,
            kinds => KINDS,
            rows => page.items,
            page => page.page,
            pages => page.pages,
            total => page.total,
            per_page => page.per_page,
            q => q.q,
            is_person => kind == "person",
        },
    )
}

/// `GET /admin/:kind/new`
pub async fn new_form(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(kind): Path<String>,
) -> Response {
    guard!(state, headers);
    let Some(k) = kind_from_str(&kind) else {
        return unknown_kind(&kind);
    };
    render::page(
        "admin_form.html",
        context! {
            nav => "admin",
            is_admin => true,
            kind,
            kinds => KINDS,
            creating => true,
            id => "",
            fields => field_views(k, &Value::Object(Default::default())),
            raw => "{}",
            action => format!("/admin/{kind}"),
        },
    )
}

/// `GET /admin/:kind/:id/edit`
pub async fn edit_form(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path((kind, id)): Path<(String, String)>,
) -> Response {
    guard!(state, headers);
    let Some(k) = kind_from_str(&kind) else {
        return unknown_kind(&kind);
    };
    let entity = state.read(|flat| flat.get(k.collection()).and_then(|c| c.get(&id)).cloned());
    let Some(entity) = entity else {
        return render::error_page(
            StatusCode::NOT_FOUND,
            "No such entity",
            "This bundle contains no entity with that id.",
        );
    };

    render::page(
        "admin_form.html",
        context! {
            nav => "admin",
            is_admin => true,
            kind,
            kinds => KINDS,
            creating => false,
            id,
            fields => field_views(k, &entity),
            raw => serde_json::to_string_pretty(&entity).unwrap_or_else(|_| "{}".into()),
            action => format!("/admin/{kind}/{id}"),
        },
    )
}

/// `POST /admin/:kind` — create.
pub async fn create(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(kind): Path<String>,
    Form(form): Form<HashMap<String, String>>,
) -> Response {
    guard!(state, headers);
    let Some(k) = kind_from_str(&kind) else {
        return unknown_kind(&kind);
    };

    let base = match base_from_raw(&form) {
        Ok(v) => v,
        Err(msg) => return form_error(&kind, None, &msg, &form, k),
    };
    let entity = apply_form(base, k, &form);
    let body = entity.to_string();

    let out = match state.mutate(|flat| axgf_rs::add_entity(flat, k, &body)) {
        Ok(o) => o,
        Err(e) => return io_error(&e),
    };

    let new_id = out
        .data
        .get("id")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();

    result_page(
        &kind,
        if out.applied {
            "Created"
        } else {
            "Not created"
        },
        &out,
        if out.applied && !new_id.is_empty() {
            Some(format!("/admin/{kind}/{new_id}/edit"))
        } else {
            None
        },
    )
}

/// `POST /admin/:kind/:id` — update.
pub async fn update(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path((kind, id)): Path<(String, String)>,
    Form(form): Form<HashMap<String, String>>,
) -> Response {
    guard!(state, headers);
    let Some(k) = kind_from_str(&kind) else {
        return unknown_kind(&kind);
    };

    let base = match base_from_raw(&form) {
        Ok(v) => v,
        Err(msg) => return form_error(&kind, Some(&id), &msg, &form, k),
    };
    let mut entity = apply_form(base, k, &form);
    // The id in the path is authoritative; a raw-JSON edit must not silently
    // retarget the update at a different entity.
    entity["id"] = Value::String(id.clone());
    let body = entity.to_string();

    let out = match state.mutate(|flat| axgf_rs::update_entity(flat, k, &body)) {
        Ok(o) => o,
        Err(e) => return io_error(&e),
    };

    result_page(
        &kind,
        if out.applied { "Saved" } else { "Not saved" },
        &out,
        Some(format!("/admin/{kind}/{id}/edit")),
    )
}

#[derive(Deserialize)]
pub struct DeleteForm {
    #[serde(default)]
    policy: String,
}

/// `POST /admin/:kind/:id/delete`
pub async fn delete(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path((kind, id)): Path<(String, String)>,
    Form(f): Form<DeleteForm>,
) -> Response {
    guard!(state, headers);
    let Some(k) = kind_from_str(&kind) else {
        return unknown_kind(&kind);
    };
    let policy = policy_from_str(&f.policy);

    let out = match state.mutate(|flat| axgf_rs::delete_entity(flat, k, &id, policy)) {
        Ok(o) => o,
        Err(e) => return io_error(&e),
    };

    result_page(
        &kind,
        if out.applied {
            "Deleted"
        } else {
            "Not deleted — the bundle is unchanged"
        },
        &out,
        Some(format!("/admin/{kind}")),
    )
}

/// `POST /admin/validate`
pub async fn validate(State(state): State<Shared>, headers: HeaderMap) -> Response {
    guard!(state, headers);
    let env = state.inspect_with(axgf_rs::validate);
    render::page(
        "admin_result.html",
        context! {
            nav => "admin",
            is_admin => true,
            title => "Validation report",
            summary => summary_line(&env.data, &[
                ("errors", "error"), ("warnings", "warning"), ("infos", "note")]),
            diagnostics => diagnostics_json(&env.diagnostics),
            back => "/admin",
            applied => true,
        },
    )
}

/// `POST /admin/dedup`
pub async fn dedup(State(state): State<Shared>, headers: HeaderMap) -> Response {
    guard!(state, headers);
    let out = match state.mutate(axgf_rs::deduplicate) {
        Ok(o) => o,
        Err(e) => return io_error(&e),
    };

    let summary = summary_line(
        &out.data,
        &[
            ("merged_persons", "person merged"),
            ("merged_families", "family merged"),
            ("manual_review", "case left for manual review"),
        ],
    );

    render::page(
        "admin_result.html",
        context! {
            nav => "admin",
            is_admin => true,
            title => if out.applied { "Deduplication complete" } else { "Deduplication refused" },
            summary,
            diagnostics => diagnostics_json(&out.diagnostics),
            back => "/admin",
            applied => out.applied,
        },
    )
}

/// `GET /admin/export`
pub async fn export(State(state): State<Shared>, headers: HeaderMap) -> Response {
    guard!(state, headers);
    match state.export_bytes() {
        Ok(bytes) => {
            let name = state
                .bundle_path()
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_else(|| "family.axgf".into());
            render::bundle_download(&name, bytes)
        }
        Err(e) => io_error(&e),
    }
}

// ---------------------------------------------------------------------------
// helpers
// ---------------------------------------------------------------------------

/// Parse the raw-JSON textarea, if the form carried one.
fn base_from_raw(form: &HashMap<String, String>) -> Result<Value, String> {
    match form.get("raw_json").map(String::as_str).map(str::trim) {
        None | Some("") => Ok(Value::Object(Default::default())),
        Some(s) => serde_json::from_str::<Value>(s)
            .map_err(|e| format!("The raw JSON did not parse: {e}. Nothing was saved.")),
    }
}

/// Field descriptors with their current values, for the form template.
fn field_views(kind: axgf_rs::EntityKind, entity: &Value) -> Vec<Value> {
    fields_for(kind)
        .iter()
        .map(|f| {
            let current = get_path(entity, f.path);
            json!({
                "path": f.path,
                "label": f.label,
                "kind": f.kind,
                "hint": f.hint,
                "options": f.options,
                "value": current,
                "checked": current == "true",
            })
        })
        .collect()
}

/// A one-line human summary of an entity, for listings.
fn summarize(kind: axgf_rs::EntityKind, e: &Value) -> String {
    use axgf_rs::EntityKind as K;
    let s = |k: &str| e.get(k).and_then(Value::as_str).unwrap_or("");
    match kind {
        K::Person => view::person_display_name(e),
        K::Family => {
            let n = s("name");
            if n.is_empty() {
                let kids = e
                    .get("children")
                    .and_then(Value::as_array)
                    .map(|a| a.len())
                    .unwrap_or(0);
                format!("(unnamed family, {kids} children)")
            } else {
                n.to_string()
            }
        }
        K::Event => {
            let c = s("category");
            let d = view::render_date_field(e, "date");
            format!("{c} — {}", d.text)
        }
        K::Link => format!("{} → {}", s("label"), s("label_reverse")),
        K::Occupation => s("title").to_string(),
        K::Source => format!("{} ({})", s("title"), s("reliability")),
        K::Place => view::place_name(e),
        K::Document => format!("{} — {}", s("filename"), s("document_type")),
    }
}

/// Diagnostics as plain JSON for the templates.
fn diagnostics_json(diags: &[axgf_rs::boundary::envelope::Diagnostic]) -> Vec<Value> {
    diags
        .iter()
        .map(|d| {
            json!({
                "code": d.code.as_str(),
                "severity": format!("{:?}", d.severity).to_lowercase(),
                "message": d.message,
                "entity_ref": d.entity_ref,
            })
        })
        .collect()
}

/// Turn numeric fields of an envelope's data into "2 merged, 1 left" prose.
fn summary_line(data: &Value, fields: &[(&str, &str)]) -> String {
    let parts: Vec<String> = fields
        .iter()
        .filter_map(|(key, noun)| {
            let n = data.get(key).and_then(Value::as_u64)?;
            Some(format!(
                "{n} {noun}{}",
                if n == 1 {
                    ""
                } else if noun.ends_with('h') {
                    "es"
                } else {
                    "s"
                }
            ))
        })
        .collect();
    if parts.is_empty() {
        "Nothing to report.".into()
    } else {
        parts.join(", ")
    }
}

/// The page shown after a mutation.
fn result_page(kind: &str, title: &str, out: &MutationOutcome, back: Option<String>) -> Response {
    render::page(
        "admin_result.html",
        context! {
            nav => "admin",
            is_admin => true,
            title,
            summary => if out.applied {
                "The bundle was written to disk."
            } else {
                "The library refused this operation. The bundle on disk is unchanged."
            },
            diagnostics => diagnostics_json(&out.diagnostics),
            back => back.unwrap_or_else(|| format!("/admin/{kind}")),
            applied => out.applied,
        },
    )
}

/// Re-render a form after a client-side error, keeping what was typed.
fn form_error(
    kind: &str,
    id: Option<&str>,
    message: &str,
    form: &HashMap<String, String>,
    k: axgf_rs::EntityKind,
) -> Response {
    let raw = form.get("raw_json").cloned().unwrap_or_else(|| "{}".into());
    let entity = serde_json::from_str::<Value>(&raw).unwrap_or(Value::Object(Default::default()));
    let mut resp = render::page(
        "admin_form.html",
        context! {
            nav => "admin",
            is_admin => true,
            kind,
            kinds => KINDS,
            creating => id.is_none(),
            id => id.unwrap_or(""),
            fields => field_views(k, &entity),
            raw,
            error => message,
            action => match id {
                Some(i) => format!("/admin/{kind}/{i}"),
                None => format!("/admin/{kind}"),
            },
        },
    );
    *resp.status_mut() = StatusCode::BAD_REQUEST;
    resp
}

fn unknown_kind(kind: &str) -> Response {
    render::error_page(
        StatusCode::NOT_FOUND,
        "Unknown entity kind",
        &format!("“{kind}” is not one of: {}.", KINDS.join(", ")),
    )
}

fn io_error(e: &anyhow::Error) -> Response {
    tracing::error!(error = %e, "admin operation failed");
    render::error_page(
        StatusCode::INTERNAL_SERVER_ERROR,
        "The bundle could not be written",
        &format!("{e}. The previous bundle is intact."),
    )
}

// ---------------------------------------------------------------------------
// document upload
// ---------------------------------------------------------------------------

/// Fields pulled out of the document upload form.
#[derive(Default)]
struct DocUpload {
    filename: String,
    bytes: Vec<u8>,
    document_type: String,
    caption: String,
    /// Set when the body limit fired while reading, so the handler can answer
    /// 413 instead of "no file was chosen".
    too_large: bool,
}

/// `POST /admin/person/:id/document` — attach a file to a person.
///
/// The bytes go into the flat bundle's `attachments` map and the metadata into
/// a Document entity, both inside one call to
/// [`AppState::mutate_and_adjust`][crate::state::AppState::mutate_and_adjust]
/// so that a single atomic write carries both. `export_bundle` then puts the
/// file back at its ZIP path on the way out — no change to `axgf-rs` was
/// needed for any of this.
pub async fn upload_document(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(id): Path<String>,
    multipart: axum::extract::Multipart,
) -> Response {
    guard!(state, headers);

    let person_exists = state.read(|flat| {
        flat.get("persons")
            .and_then(|p| p.get(&id))
            .is_some_and(|p| !p.is_null())
    });
    if !person_exists {
        return render::error_page(
            StatusCode::NOT_FOUND,
            "No such person",
            "This bundle contains no person with that id, so there is nothing \
             to attach a document to.",
        );
    }

    let up = read_document_upload(multipart).await;

    if up.too_large || up.bytes.len() > documents::MAX_UPLOAD {
        return upload_refused(
            &id,
            StatusCode::PAYLOAD_TOO_LARGE,
            &format!(
                "That file is larger than the {} MB limit. Nothing was stored, \
                 and the bundle is unchanged.",
                documents::MAX_UPLOAD / (1024 * 1024)
            ),
        );
    }
    if up.bytes.is_empty() {
        return upload_refused(
            &id,
            StatusCode::BAD_REQUEST,
            "No file was uploaded. Choose a file first.",
        );
    }

    // The filename and the client's Content-Type are both attacker-controlled,
    // so neither is consulted: the type comes from the bytes.
    let Some(kind) = documents::sniff(&up.bytes) else {
        return upload_refused(
            &id,
            StatusCode::UNSUPPORTED_MEDIA_TYPE,
            "That file is not a type this archive stores. Images, PDF, plain \
             text, audio and video are accepted; the type is read from the \
             file's own bytes, so renaming an executable does not get it in. \
             SVG is refused outright, because an SVG can carry script.",
        );
    };

    let sha256 = documents::sha256_hex(&up.bytes);
    let size = up.bytes.len() as u64;
    let filename = clean_filename(&up.filename, kind.ext);
    let doc_type = if up.document_type.trim().is_empty() {
        if kind.raster_image {
            "photo".to_string()
        } else {
            "other".to_string()
        }
    } else {
        up.document_type.trim().to_string()
    };

    // The Document is created without its `file.path`, because the path
    // contains the id the library is about to mint. The adjust step fills in
    // the path and stores the payload beside it, inside the same write.
    let mut entity = json!({
        "type": "document",
        "axgf_version": "1.0",
        "filename": filename,
        "mime_type": kind.mime,
        "document_type": doc_type,
        "status": "present",
        "file": {"size_bytes": size, "sha256": sha256},
        "linked_to": [{"entity_type": "person", "entity_id": id, "role": "subject"}],
    });
    if !up.caption.trim().is_empty() {
        entity["caption"] = json!(up.caption.trim());
    }
    let body = entity.to_string();

    // The payload goes straight to the disk cache, never into the in-memory
    // bundle; add_document mints the id, fills in the file path, and persists.
    let (out, _new_id) = match state.add_document(&body, &up.bytes, kind.ext) {
        Ok(o) => o,
        Err(e) => return io_error(&e),
    };

    if !out.applied {
        return upload_refused(
            &id,
            StatusCode::BAD_REQUEST,
            &format!(
                "The library refused the document: {}. The bundle is unchanged.",
                crate::state::format_diagnostics(&out.diagnostics)
            ),
        );
    }

    Redirect::to(&format!("/person/{id}#evidence")).into_response()
}

/// Refuse an upload with a reason and a link back to the person.
///
/// The status carries the distinction a script needs — 413 for too big, 415
/// for a type this archive does not store — and the body carries the sentence
/// a person needs.
fn upload_refused(person: &str, status: StatusCode, message: &str) -> Response {
    render::error_page_back(
        status,
        "That upload was not stored",
        message,
        Some((&format!("/person/{person}"), "Back to this person")),
    )
}

/// A filename safe to store and to echo back into a header.
///
/// Path separators and quotes are stripped rather than escaped, and the
/// extension is forced to match what the bytes actually are, so a file called
/// `holiday.jpg` that is really a PDF is stored as `holiday.pdf`.
fn clean_filename(raw: &str, ext: &str) -> String {
    let base = raw
        .rsplit(['/', '\\'])
        .next()
        .unwrap_or(raw)
        .trim()
        .trim_matches('.');
    let stem = base.rsplit_once('.').map(|(s, _)| s).unwrap_or(base);
    let stem: String = stem
        .chars()
        .filter(|c| !c.is_control() && !matches!(c, '"' | '\\' | '/'))
        .take(120)
        .collect();
    let stem = stem.trim();
    if stem.is_empty() {
        format!("upload.{ext}")
    } else {
        format!("{stem}.{ext}")
    }
}

/// Read the upload form, tolerating fields in any order.
async fn read_document_upload(mut multipart: axum::extract::Multipart) -> DocUpload {
    let mut out = DocUpload::default();
    loop {
        let field = match multipart.next_field().await {
            Ok(Some(f)) => f,
            Ok(None) => break,
            Err(e) => {
                // The body-limit layer surfaces as an error here, and it is
                // the one failure worth telling apart from a malformed body.
                out.too_large = e.status() == StatusCode::PAYLOAD_TOO_LARGE;
                break;
            }
        };
        match field.name().unwrap_or_default().to_string().as_str() {
            "file" => {
                out.filename = field.file_name().unwrap_or("upload").to_string();
                match field.bytes().await {
                    Ok(b) => out.bytes = b.to_vec(),
                    Err(_) => out.too_large = true,
                }
            }
            "document_type" => out.document_type = field.text().await.unwrap_or_default(),
            "caption" => out.caption = field.text().await.unwrap_or_default(),
            _ => {
                let _ = field.bytes().await;
            }
        }
    }
    out
}
