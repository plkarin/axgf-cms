//! Canopée's world (docs/adr/0043): its shape, the same for every reader and
//! kept a year by the browser under its fingerprint, and its words, the
//! names and years the reader's lens admits.

use axum::extract::{Query, State};
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use serde::Deserialize;
use serde_json::{json, Map, Value};

use super::Shared;
use crate::auth;

#[derive(Debug, Default, Deserialize)]
pub struct WorldQuery {
    /// The fingerprint the page was drawn with.
    #[serde(default)]
    v: Option<String>,
}

/// `GET /canopee/world.bin?v=<fingerprint>` — the world, built once for each
/// shape of the family. Asked for by its fingerprint, it never changes, and
/// the browser keeps it; asked for by another, it is the world as it is now,
/// to be asked again.
pub async fn bin(
    State(state): State<Shared>,
    headers: HeaderMap,
    Query(q): Query<WorldQuery>,
) -> Response {
    let st = state.clone();
    let Ok((fp, bytes)) = tokio::task::spawn_blocking(move || st.world()).await else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };
    let etag = format!("\"{fp}\"");
    let cache = if q.v.as_deref() == Some(fp.as_str()) {
        "private, max-age=31536000, immutable"
    } else {
        "private, no-cache"
    };
    if headers
        .get(header::IF_NONE_MATCH)
        .and_then(|v| v.to_str().ok())
        .is_some_and(|v| v == etag)
    {
        return (
            StatusCode::NOT_MODIFIED,
            [
                (header::ETAG, etag),
                (header::CACHE_CONTROL, cache.to_string()),
            ],
        )
            .into_response();
    }
    (
        [
            (header::CONTENT_TYPE, "application/octet-stream".to_string()),
            (header::CACHE_CONTROL, cache.to_string()),
            (header::ETAG, etag),
        ],
        bytes.as_ref().clone(),
    )
        .into_response()
}

/// `GET /canopee/words.json` — the words laid over the world: for each
/// person the reader may read, their given name, family name and years. The
/// others have none (ADR 0001).
pub async fn words(State(state): State<Shared>, headers: HeaderMap) -> Response {
    let viewer = auth::viewer(&state, &headers);
    let fp = state.world_fingerprint();
    let people = state.read_as(viewer.ceiling(), |flat, lens| {
        let mut out = Map::new();
        let Some(persons) = flat.get("persons").and_then(Value::as_object) else {
            return out;
        };
        for id in persons.keys() {
            let l = crate::access::river_label(flat, lens, id);
            if l.redacted {
                continue;
            }
            let years = match (l.birth_year, l.death_year) {
                (Some(b), Some(d)) => format!("{b}–{d}"),
                (Some(b), None) => format!("{b}–"),
                (None, Some(d)) => format!("–{d}"),
                (None, None) => String::new(),
            };
            out.insert(id.clone(), json!([l.given, l.surname, years]));
        }
        out
    });
    (
        [(header::CACHE_CONTROL, "private, no-store")],
        axum::Json(json!({"fingerprint": fp, "people": people})),
    )
        .into_response()
}
