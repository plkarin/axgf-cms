//! `/admin/uploads` — what a file sent to the site may weigh, and the kinds
//! the archive refuses (docs/adr/0041). A page of the archive's room, for
//! administrators.

use axum::extract::{Form, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Redirect, Response};
use minijinja::context;
use serde_json::{json, Value};

use super::admin::{require, Need};
use super::Shared;
use crate::limits::{Limits, BOUNDS, KINDS};
use crate::render;

/// `GET /admin/uploads`
pub async fn page(
    State(state): State<Shared>,
    headers: HeaderMap,
    Query(q): Query<std::collections::HashMap<String, String>>,
) -> Response {
    let (_viewer, chrome) = match require(&state, &headers, Need::Admin) {
        Ok(v) => v,
        Err(r) => return r,
    };
    let limits = state.limits();
    let values: Vec<(String, String)> = BOUNDS
        .iter()
        .map(|b| (b.key.to_string(), limits.mb(*b).to_string()))
        .collect();
    draw(
        &chrome,
        &values,
        &limits.refused,
        q.contains_key("saved"),
        None,
    )
}

/// `POST /admin/uploads`
pub async fn save(
    State(state): State<Shared>,
    headers: HeaderMap,
    Form(fields): Form<Vec<(String, String)>>,
) -> Response {
    let (viewer, chrome) = match require(&state, &headers, Need::Admin) {
        Ok(v) => v,
        Err(r) => return r,
    };
    let sent = |key: &str| {
        fields
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.trim().to_string())
            .unwrap_or_default()
    };
    let accepted: Vec<&str> = fields
        .iter()
        .filter(|(k, _)| k == "kind")
        .map(|(_, v)| v.as_str())
        .collect();
    let refused: Vec<String> = KINDS
        .iter()
        .flat_map(|(_, kinds)| kinds.iter())
        .map(|(mime, _)| *mime)
        .filter(|mime| !accepted.contains(mime))
        .map(str::to_string)
        .collect();
    let values: Vec<(String, String)> = BOUNDS
        .iter()
        .map(|b| (b.key.to_string(), sent(b.key)))
        .collect();

    // Every limit a whole number of megabytes within its bounds, or the page
    // again, as it was sent, saying which and why.
    let mut limits = Limits {
        refused: refused.clone(),
        ..Limits::default()
    };
    for b in BOUNDS {
        let field = chrome.t(&format!("uploads-limit-{}", b.key));
        match sent(b.key).parse::<u32>() {
            Ok(mb) if (b.min_mb..=b.max_mb).contains(&mb) => limits.set(b, mb),
            _ => {
                let message = chrome.t_args(
                    "uploads-error-range",
                    &[
                        ("field", field.into()),
                        ("min", (b.min_mb as i64).into()),
                        ("max", (b.max_mb as i64).into()),
                    ],
                );
                let mut resp = draw(&chrome, &values, &refused, false, Some(message));
                *resp.status_mut() = StatusCode::BAD_REQUEST;
                return resp;
            }
        }
    }
    let kept = limits.clone();
    if let Err(e) = state.site_mutate(|s| s.uploads = kept) {
        let message = chrome.t_args("uploads-not-saved", &[("error", e.to_string().into())]);
        let mut resp = draw(&chrome, &values, &refused, false, Some(message));
        *resp.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
        return resp;
    }
    tracing::info!(
        by = viewer.name(),
        document_mb = limits.document_mb,
        note_image_mb = limits.note_image_mb,
        assistant_file_mb = limits.assistant_file_mb,
        assistant_message_mb = limits.assistant_message_mb,
        gedcom_mb = limits.gedcom_mb,
        archive_mb = limits.archive_mb,
        refused = limits.refused.join(","),
        "upload limits changed"
    );
    Redirect::to("/admin/uploads?saved=1").into_response()
}

/// The page: each limit as it is (or as it was sent), each kind ticked when
/// it is accepted.
fn draw(
    chrome: &render::Chrome,
    values: &[(String, String)],
    refused: &[String],
    saved: bool,
    error: Option<String>,
) -> Response {
    let bounds: Vec<Value> = BOUNDS
        .iter()
        .map(|b| {
            let value = values
                .iter()
                .find(|(k, _)| k == b.key)
                .map(|(_, v)| v.clone())
                .unwrap_or_default();
            json!({"key": b.key, "value": value, "min": b.min_mb, "max": b.max_mb,
                   "default": b.default_mb})
        })
        .collect();
    let families: Vec<Value> = KINDS
        .iter()
        .map(|(family, kinds)| {
            json!({
                "key": family,
                "kinds": kinds.iter().map(|(mime, name)| json!({
                    "mime": mime,
                    "name": name,
                    "accepted": !refused.iter().any(|r| r == mime),
                })).collect::<Vec<_>>(),
            })
        })
        .collect();
    let total = KINDS.iter().map(|(_, k)| k.len()).sum::<usize>();
    let accepted = total - refused.len().min(total);
    render::page_with(
        chrome,
        "admin_uploads.html",
        context! {
            nav => "admin",
            admin_page => "bundle",
            bounds,
            families,
            accepted,
            total,
            saved,
            error,
        },
    )
}
