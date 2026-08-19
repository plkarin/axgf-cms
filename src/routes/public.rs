//! Public, read-only pages.

use axum::extract::{Path, Query, State};
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Json;
use minijinja::context;
use serde_json::{json, Value};

use crate::routes::Shared;
use crate::state::COLLECTIONS;
use crate::{auth, render, view};

/// `GET /` — why AXGF, what is in this bundle, entry points.
pub async fn home(State(state): State<Shared>, headers: HeaderMap) -> Response {
    let is_admin = auth::is_admin(&headers, state.admin_token());
    let counts = state.counts();
    let total: usize = counts.iter().map(|(_, n)| n).sum();

    let family_name = state.read(|flat| {
        flat.get("manifest")
            .and_then(|m| m.get("family"))
            .and_then(|f| f.get("name"))
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .unwrap_or("This bundle")
            .to_string()
    });

    let showcase = state.read(showcase_highlights);

    render::page(
        "home.html",
        context! {
            nav => "home",
            is_admin,
            family_name,
            total,
            counts => counts
                .iter()
                .map(|(k, n)| json!({"kind": k, "n": n}))
                .collect::<Vec<_>>(),
            showcase,
        },
    )
}

/// Work out which GEDCOM-impossible features this particular bundle actually
/// contains, so the home page points at real examples instead of advertising
/// features the data does not exercise.
fn showcase_highlights(flat: &Value) -> Vec<Value> {
    let mut out = Vec::new();

    let obj = |key: &str| flat.get(key).and_then(Value::as_object);

    if let Some(links) = obj("links") {
        if !links.is_empty() {
            let example = links.values().next().and_then(|l| {
                l.get("from")
                    .filter(|f| f.get("entity_type").and_then(Value::as_str) == Some("person"))
                    .and_then(|f| f.get("entity_id"))
                    .and_then(Value::as_str)
                    .map(str::to_string)
            });
            out.push(json!({
                "title": format!("{} non-family relationships", links.len()),
                "detail": "Godparents, employers, witnesses and mentors, each with \
                           its own dates, source and confidence. GEDCOM has no way \
                           to state these at all.",
                "example_id": example,
            }));
        }
    }

    if let Some(occs) = obj("occupations") {
        if !occs.is_empty() {
            let example = occs
                .values()
                .next()
                .and_then(|o| o.get("person_id"))
                .and_then(Value::as_str)
                .map(str::to_string);
            out.push(json!({
                "title": format!("{} occupations recorded as spans", occs.len()),
                "detail": "“Schoolteacher, 1948–1978” is a state with a duration, \
                           rendered as a timeline bar rather than flattened into a \
                           dated event.",
                "example_id": example,
            }));
        }
    }

    // Count persons whose birth or death date is anything other than a pinned
    // calendar day — the population that GEDCOM would render as a blank or a
    // fabricated precision.
    if let Some(persons) = obj("persons") {
        let mut uncertain = 0usize;
        let mut preserved = 0usize;
        let mut example: Option<String> = None;
        for (id, p) in persons.iter() {
            for key in ["birth", "death"] {
                let Some(ev) = p.get(key) else { continue };
                let d = view::render_date_field(ev, "date");
                match d.kind {
                    "range" | "approximate" => {
                        uncertain += 1;
                        example.get_or_insert_with(|| id.clone());
                    }
                    "preserved" => {
                        preserved += 1;
                        example = Some(id.clone());
                    }
                    _ => {}
                }
            }
        }
        if uncertain > 0 {
            out.push(json!({
                "title": format!("{uncertain} dates that are honestly imprecise"),
                "detail": "Circa, before, after and between are preserved as \
                           distinct statements. A date the source could not pin \
                           down is not shown as if it were.",
                "example_id": example,
            }));
        }
        if preserved > 0 {
            out.push(json!({
                "title": if preserved == 1 {
                    "an unparseable date kept verbatim".to_string()
                } else {
                    format!("{preserved} unparseable dates kept verbatim")
                },
                "detail": "Text no converter could interpret survives as a note \
                           instead of being silently dropped.",
                "example_id": Value::Null,
            }));
        }
    }

    if let Some(sources) = obj("sources") {
        if !sources.is_empty() {
            let primary = sources
                .values()
                .filter(|s| s.get("reliability").and_then(Value::as_str) == Some("primary"))
                .count();
            out.push(json!({
                "title": format!("{} sources graded by reliability", sources.len()),
                "detail": format!(
                    "{primary} primary. Every fact shows which evidence it rests \
                     on, and how strong that evidence is."),
                "example_id": Value::Null,
            }));
        }
    }

    if let Some(places) = obj("places") {
        let with_history = places
            .values()
            .filter(|p| {
                p.get("country_history")
                    .and_then(Value::as_array)
                    .is_some_and(|a| !a.is_empty())
            })
            .count();
        if with_history > 0 {
            out.push(json!({
                "title": format!("{with_history} places with border history"),
                "detail": "A town can belong to different countries at different \
                           times, and the record says which one applied when.",
                "example_id": Value::Null,
            }));
        }
    }

    out
}

/// Query parameters for `/tree`.
#[derive(serde::Deserialize)]
pub struct TreeQuery {
    /// Centre the view on this person. Defaults to whoever's surroundings
    /// make the fullest first screen at the requested depth.
    #[serde(default)]
    root: Option<String>,
    /// Generations shown in each direction.
    #[serde(default)]
    depth: Option<usize>,
    /// Draw every person in the bundle instead of a focused subtree.
    #[serde(default)]
    all: Option<String>,
    /// Person whose record is shown in the side panel. Defaults to the root.
    /// Kept distinct from `root` so opening a record in the panel does not move
    /// the tree — re-centring is an explicit action.
    #[serde(default)]
    sel: Option<String>,
}

/// Depth shown above and below the root when none is requested.
const DEFAULT_DEPTH: usize = 3;
/// Upper bound on depth. Past this a "focused" view is the whole bundle again.
const MAX_DEPTH: usize = 8;

/// `GET /tree` — a focused subtree by default, the whole bundle with `?all=1`.
///
/// The full view is laid out correctly but is not usable on a real file: the
/// operator's bundle puts 161 people in its widest generation, which is a
/// canvas over 23,000px wide. Nobody scrolls that far to find an ancestor, so
/// the default is a few dozen people around one person, and every card
/// re-roots the view.
pub async fn tree(
    State(state): State<Shared>,
    headers: HeaderMap,
    Query(q): Query<TreeQuery>,
) -> Response {
    let is_admin = auth::is_admin(&headers, state.admin_token());
    let show_all = q.all.as_deref().is_some_and(|v| v != "0" && !v.is_empty());
    let depth = q.depth.unwrap_or(DEFAULT_DEPTH).min(MAX_DEPTH);

    let started = std::time::Instant::now();
    let (layout, focus, roster, panel, selected) = state.read(|flat| {
        // The root picker lists everyone by name, so it is built regardless of
        // which view is showing.
        let roster = person_roster(flat);

        if show_all {
            return (crate::tree::layout(flat), None, roster, None, None);
        }

        let root = q
            .root
            .clone()
            .filter(|id| flat.get("persons").and_then(|p| p.get(id)).is_some())
            .or_else(|| crate::tree::best_root(flat, depth));

        match root {
            Some(root) => {
                let sub = crate::tree::select_subtree(flat, &root, depth, depth);
                let l = crate::tree::layout_focused(flat, &sub);
                let name = flat
                    .get("persons")
                    .and_then(|p| p.get(&root))
                    .map(view::person_display_name)
                    .unwrap_or_else(|| "[Unknown]".into());
                let focus = json!({
                    "root": root,
                    "root_name": name,
                    "ancestors": sub.ancestor_count,
                    "descendants": sub.descendant_count,
                    "spouses": sub.spouse_count,
                });
                // The panel opens on the selection, or the root if none was
                // asked for. Selecting a person never moves the tree.
                let sel = q
                    .sel
                    .clone()
                    .filter(|id| flat.get("persons").and_then(|p| p.get(id)).is_some())
                    .unwrap_or_else(|| root.clone());
                let panel = crate::person::build(flat, &sel);
                (l, Some(focus), roster, panel, Some(sel))
            }
            // An empty bundle has nobody to focus on.
            None => (crate::tree::layout(flat), None, roster, None, None),
        }
    });
    let elapsed = started.elapsed();

    tracing::debug!(
        drawn = layout.person_count,
        total = layout.total_person_count,
        edges = layout.edges.len(),
        ms = elapsed.as_secs_f64() * 1000.0,
        all = show_all,
        "tree laid out"
    );

    // The warning that states the canvas width sits above the full view only,
    // so the focused path must not pay for a second whole-bundle layout to
    // compute a number it never shows.
    let full_width = layout.width;

    render::page(
        "tree.html",
        context! {
            nav => "tree",
            is_admin,
            layout,
            focus,
            roster,
            depth,
            show_all,
            max_depth => MAX_DEPTH,
            full_width => full_width.round() as i64,
            p => panel,
            selected,
            // The record sections inside the panel lay themselves out for a
            // clamped column rather than a page.
            compact => true,
            max_upload_mb => crate::documents::MAX_UPLOAD / (1024 * 1024),
        },
    )
}

/// `GET /tree/panel/:id` — the side-panel fragment for one person.
///
/// Returns just the panel markup, not a whole page, so a card click can swap it
/// in without reloading the tree. It renders the same `_panel.html` (and thus
/// the same record sections) the initial server-rendered panel uses.
pub async fn tree_panel(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Response {
    let is_admin = auth::is_admin(&headers, state.admin_token());
    let panel = state.read(|flat| crate::person::build(flat, &id));
    match panel {
        Some(p) => render::page(
            "_panel.html",
            context! {
                p,
                is_admin,
                compact => true,
                max_upload_mb => crate::documents::MAX_UPLOAD / (1024 * 1024),
            },
        ),
        None => render::error_page(
            StatusCode::NOT_FOUND,
            "No such person",
            "This bundle contains no person with that id.",
        ),
    }
}

/// Every person as `{id, name}`, sorted by name, for the root picker.
fn person_roster(flat: &Value) -> Vec<Value> {
    let Some(persons) = flat.get("persons").and_then(Value::as_object) else {
        return Vec::new();
    };
    let mut out: Vec<(String, String)> = persons
        .iter()
        .map(|(id, p)| (view::person_display_name(p), id.clone()))
        .collect();
    out.sort();
    out.into_iter()
        .map(|(name, id)| json!({"id": id, "name": name}))
        .collect()
}

/// `GET /person/:id` — everything known about one person.
pub async fn person(
    State(state): State<Shared>,
    Path(id): Path<String>,
    headers: HeaderMap,
) -> Response {
    let is_admin = auth::is_admin(&headers, state.admin_token());
    let view = state.read(|flat| crate::person::build(flat, &id));

    match view {
        Some(p) => render::page(
            "person.html",
            context! {
                nav => "tree",
                is_admin,
                p,
                // The standalone page has the width for the explanatory prose
                // and the comparison tables; the panel does not.
                compact => false,
                max_upload_mb => crate::documents::MAX_UPLOAD / (1024 * 1024),
            },
        ),
        None => render::error_page(
            StatusCode::NOT_FOUND,
            "No such person",
            "This bundle contains no person with that id.",
        ),
    }
}

/// A document's metadata and the ZIP path its payload is cached under.
struct StoredDocument {
    mime: String,
    filename: String,
    sha256: String,
    /// The attachment key; the bytes live in the disk cache under it.
    path: String,
}

/// Resolve a document id to its metadata and the key its payload is stored at.
///
/// `None` covers both "no such document" and "the document exists but this
/// bundle does not carry the file" — a `referenced` document names something
/// held elsewhere, and there is nothing here to serve either way.
fn stored_document(state: &Shared, id: &str) -> Option<StoredDocument> {
    let doc = state.read(|flat| {
        let d = flat.get("documents")?.get(id)?;
        let path = d.get("file")?.get("path")?.as_str()?.to_string();
        Some(StoredDocument {
            mime: d
                .get("mime_type")
                .and_then(Value::as_str)
                .unwrap_or("application/octet-stream")
                .to_string(),
            filename: d
                .get("filename")
                .and_then(Value::as_str)
                .unwrap_or("document")
                .to_string(),
            sha256: d
                .get("file")
                .and_then(|f| f.get("sha256"))
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_string(),
            path,
        })
    })?;
    // Only serve when the payload is actually present in the cache.
    state.payloads().path_of(&doc.path)?;
    Some(doc)
}

/// `GET /document/:id/raw` — the stored bytes.
///
/// Served with `X-Content-Type-Options: nosniff` always, and as an attachment
/// for everything except the raster image formats a browser draws as pixels.
/// The exception matters: an SVG or an HTML file rendered inline from this
/// origin would run its own script against the viewer's admin session, so
/// only formats that cannot carry script are shown in the page.
pub async fn document_raw(State(state): State<Shared>, Path(id): Path<String>) -> Response {
    let Some(doc) = stored_document(&state, &id) else {
        return render::error_page(
            StatusCode::NOT_FOUND,
            "No such file",
            "This bundle has no document with that id, or the document is \
             recorded without a file — a `referenced` document names something \
             held somewhere else.",
        );
    };
    let Some(path) = state.payloads().path_of(&doc.path) else {
        return render::error_page(
            StatusCode::NOT_FOUND,
            "No such file",
            "The payload for that document is not in the cache.",
        );
    };

    let disposition = if crate::documents::serve_inline(&doc.mime) {
        format!("inline; filename=\"{}\"", sanitize_header(&doc.filename))
    } else {
        format!(
            "attachment; filename=\"{}\"",
            sanitize_header(&doc.filename)
        )
    };

    // Stream the bytes from disk rather than reading them into memory. The
    // download is byte-identical to what is in the bundle: EXIF orientation is
    // corrected only for display (see `document_view`), never for the original.
    let Ok(file) = tokio::fs::File::open(&path).await else {
        return render::error_page(
            StatusCode::NOT_FOUND,
            "No such file",
            "The payload for that document could not be opened.",
        );
    };
    let len = file.metadata().await.map(|m| m.len()).ok();
    let stream = tokio_util::io::ReaderStream::new(file);
    let body = axum::body::Body::from_stream(stream);

    let mut headers = header::HeaderMap::new();
    set_header(&mut headers, header::CONTENT_TYPE, &doc.mime);
    set_header(&mut headers, header::CONTENT_DISPOSITION, &disposition);
    set_header(
        &mut headers,
        header::HeaderName::from_static("x-content-type-options"),
        "nosniff",
    );
    // The payload is immutable: a different file means a different document id,
    // because uploads never overwrite in place.
    set_header(&mut headers, header::CACHE_CONTROL, "private, max-age=3600");
    if let Some(len) = len {
        set_header(&mut headers, header::CONTENT_LENGTH, &len.to_string());
    }
    (headers, body).into_response()
}

/// Insert a header, silently skipping a value that cannot be a header value
/// (e.g. a filename with bytes that are not valid in a header). The content
/// type falls back to a safe default rather than being dropped.
fn set_header(headers: &mut header::HeaderMap, name: header::HeaderName, value: &str) {
    match header::HeaderValue::from_str(value) {
        Ok(v) => {
            headers.insert(name, v);
        }
        Err(_) if name == header::CONTENT_TYPE => {
            headers.insert(
                name,
                header::HeaderValue::from_static("application/octet-stream"),
            );
        }
        Err(_) => {}
    }
}

/// `GET /document/:id/view` — a full-size image, EXIF-orientation corrected,
/// for display in the page. Non-images and images already upright stream their
/// stored bytes unchanged; a rotated image is re-encoded so it shows upright.
///
/// This is deliberately distinct from `/raw`: `/raw` is the byte-identical
/// original a reader downloads, while `/view` is what the gallery opens, where
/// a sideways phone photo must appear the right way up.
pub async fn document_view(State(state): State<Shared>, Path(id): Path<String>) -> Response {
    let Some(doc) = stored_document(&state, &id) else {
        return render::error_page(
            StatusCode::NOT_FOUND,
            "No such file",
            "This bundle has no document with that id.",
        );
    };
    let Some(bytes) = state.attachment(&doc.path) else {
        return render::error_page(StatusCode::NOT_FOUND, "No such file", "Payload missing.");
    };

    // Only raster images are corrected; anything else is served as its stored
    // bytes inline where safe, or as an attachment otherwise.
    if crate::documents::serve_inline(&doc.mime) {
        if let Some(png) = crate::documents::oriented_image(&bytes) {
            return (
                [
                    (header::CONTENT_TYPE, "image/png".to_string()),
                    (
                        header::HeaderName::from_static("x-content-type-options"),
                        "nosniff".to_string(),
                    ),
                    (header::CACHE_CONTROL, "private, max-age=3600".to_string()),
                ],
                png,
            )
                .into_response();
        }
    }
    // Fallback: hand off to the byte-identical path.
    document_raw(State(state), Path(id)).await
}

/// `GET /document/:id/thumb` — a downscaled PNG, or 404 for a non-image.
pub async fn document_thumb(State(state): State<Shared>, Path(id): Path<String>) -> Response {
    let Some(doc) = stored_document(&state, &id) else {
        return render::error_page(
            StatusCode::NOT_FOUND,
            "No such file",
            "This bundle has no document with that id.",
        );
    };

    let png = state.thumbs().get_or_insert(&id, &doc.sha256, || {
        state
            .attachment(&doc.path)
            .and_then(|bytes| crate::documents::thumbnail(&bytes))
    });

    let Some(png) = png else {
        return render::error_page(
            StatusCode::NOT_FOUND,
            "Not an image",
            "There is no thumbnail for this document, because it is not an \
             image this build can decode.",
        );
    };

    (
        [
            (header::CONTENT_TYPE, "image/png".to_string()),
            (
                header::HeaderName::from_static("x-content-type-options"),
                "nosniff".to_string(),
            ),
            (header::CACHE_CONTROL, "private, max-age=3600".to_string()),
        ],
        png,
    )
        .into_response()
}

/// Strip what would let a filename break out of the `Content-Disposition`
/// quoting or inject a second header.
fn sanitize_header(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            '"' | '\\' | '\r' | '\n' | '/' => '_',
            c if c.is_control() => '_',
            c => c,
        })
        .collect()
}

/// `GET /static/tree.js`
pub async fn tree_js() -> Response {
    (
        [
            (header::CONTENT_TYPE, "text/javascript; charset=utf-8"),
            (header::CACHE_CONTROL, "public, max-age=3600"),
        ],
        render::TREE_JS,
    )
        .into_response()
}

/// `GET /health` — liveness plus entity counts.
pub async fn health(State(state): State<Shared>) -> Response {
    let counts = state.counts();
    let total: usize = counts.iter().map(|(_, n)| n).sum();
    let mut entities = serde_json::Map::new();
    for (k, n) in &counts {
        entities.insert((*k).to_string(), json!(n));
    }
    Json(json!({
        "status": "ok",
        "total_entities": total,
        "entities": Value::Object(entities),
        "collections": COLLECTIONS,
    }))
    .into_response()
}

/// `GET /static/app.css`
pub async fn css() -> Response {
    (
        [
            (header::CONTENT_TYPE, "text/css; charset=utf-8"),
            (header::CACHE_CONTROL, "public, max-age=3600"),
        ],
        render::APP_CSS,
    )
        .into_response()
}

/// Fallback for unmatched paths.
pub async fn not_found() -> Response {
    render::error_page(
        StatusCode::NOT_FOUND,
        "Not found",
        "That page does not exist in this bundle.",
    )
}
