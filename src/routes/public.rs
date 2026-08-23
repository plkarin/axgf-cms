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
    let viewer = auth::viewer(&state, &headers);
    let is_admin = viewer.is_admin();
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

    let showcase = state.read_as(viewer.ceiling(), showcase_highlights);

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
fn showcase_highlights(flat: &Value, lens: &crate::access::Lens) -> Vec<Value> {
    let mut out = Vec::new();

    let obj = |key: &str| flat.get(key).and_then(Value::as_object);

    // Every count here is a count of what *this* reader can reach, and every
    // "example_id" is a link, so it has to lead somewhere they may open. A
    // showcase advertising 40 relationships and linking to a wall reading
    // "Private" would be worse than not advertising them.
    if let Some(links) = obj("links") {
        let readable: Vec<&Value> = links.values().filter(|l| lens.sees_entity(l)).collect();
        if !readable.is_empty() {
            let example = readable.iter().find_map(|l| {
                l.get("from")
                    .filter(|f| f.get("entity_type").and_then(Value::as_str) == Some("person"))
                    .and_then(|f| f.get("entity_id"))
                    .and_then(Value::as_str)
                    .filter(|id| lens.sees_person(id))
                    .map(str::to_string)
            });
            out.push(json!({
                "title": format!("{} non-family relationships", readable.len()),
                "detail": "Godparents, employers, witnesses and mentors, each with \
                           its own dates, source and confidence. GEDCOM has no way \
                           to state these at all.",
                "example_id": example,
            }));
        }
    }

    if let Some(occs) = obj("occupations") {
        let readable: Vec<&Value> = occs
            .values()
            .filter(|o| lens.sees_entity(o))
            .filter(|o| {
                o.get("person_id")
                    .and_then(Value::as_str)
                    .is_some_and(|id| lens.sees_person(id))
            })
            .collect();
        if !readable.is_empty() {
            let example = readable
                .iter()
                .find_map(|o| o.get("person_id"))
                .and_then(Value::as_str)
                .map(str::to_string);
            out.push(json!({
                "title": format!("{} occupations recorded as spans", readable.len()),
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
            if !lens.sees_person(id) {
                continue;
            }
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
    let viewer = auth::viewer(&state, &headers);
    let is_admin = viewer.is_admin();
    let show_all = q.all.as_deref().is_some_and(|v| v != "0" && !v.is_empty());
    let depth = q.depth.unwrap_or(DEFAULT_DEPTH).min(MAX_DEPTH);

    let started = std::time::Instant::now();
    let (layout, focus, roster, panel, selected, hidden) =
        state.read_as(viewer.ceiling(), |flat, lens| {
            // The lens arrives resolved and memoised per bundle version, under the
            // same read lock as the bundle — so it can never describe a different
            // version than the one being read.
            // The root picker lists only people this reader can actually open;
            // an entry that leads to "Private" is not a destination.
            let roster = person_roster(flat, lens);
            let hidden = flat
                .get("persons")
                .and_then(Value::as_object)
                .map(|p| p.len().saturating_sub(lens.count(p.len())))
                .unwrap_or(0);

            if show_all {
                let mut l = crate::tree::layout(flat);
                crate::tree::redact(&mut l, lens.set());
                return (l, None, roster, None, None, hidden);
            }

            // Choosing a root evaluates every candidate's whole subtree, so it
            // is the most expensive thing on this path and must happen exactly
            // once. When the reader may read nobody — every signed-out visitor
            // to a bundle that marks its family `members`, which is not a rare
            // case — searching readable candidates first would scan all 866
            // people to find none and then scan them all again unrestricted.
            // Ask only the question that can be answered.
            let candidates = lens.set();
            let restricted_search = candidates.is_some_and(|set| !set.is_empty());
            let root = q
                .root
                .clone()
                .filter(|id| flat.get("persons").and_then(|p| p.get(id)).is_some())
                .or_else(|| {
                    if restricted_search {
                        crate::tree::best_root_among(flat, depth, candidates)
                    } else {
                        // Nobody readable: the tree still draws its shape, all
                        // of it redacted, centred where an admin would land.
                        crate::tree::best_root(flat, depth)
                    }
                });

            match root {
                Some(root) => {
                    let sub = crate::tree::select_subtree(flat, &root, depth, depth);
                    let mut l = crate::tree::layout_focused(flat, &sub);
                    crate::tree::redact(&mut l, lens.set());
                    let name = if lens.sees_person(&root) {
                        flat.get("persons")
                            .and_then(|p| p.get(&root))
                            .map(view::person_display_name)
                            .unwrap_or_else(|| "[Unknown]".into())
                    } else {
                        crate::person::RESTRICTED_NAME.to_string()
                    };
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
                    // A selection the reader may not read opens no panel at all,
                    // rather than a panel of blanks.
                    let panel = if lens.sees_person(&sel) {
                        crate::person::build(flat, &sel, lens)
                    } else {
                        None
                    };
                    (l, Some(focus), roster, panel, Some(sel), hidden)
                }
                // An empty bundle has nobody to focus on.
                None => {
                    let mut l = crate::tree::layout(flat);
                    crate::tree::redact(&mut l, lens.set());
                    (l, None, roster, None, None, hidden)
                }
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
            // How many people this reader is not being shown. Stated rather
            // than hidden: a tree with silent gaps looks like a broken import.
            hidden,
            signed_in => viewer.signed_in(),
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
    let viewer = auth::viewer(&state, &headers);
    let is_admin = viewer.is_admin();
    // The panel fetch is a read path like any other, and it is the one most
    // easily forgotten: it returns a fragment rather than a page, so a
    // template-level check would never have covered it. It resolves its own
    // lens and refuses on its own.
    let outcome = state.read_as(viewer.ceiling(), |flat, lens| {
        if flat.get("persons").and_then(|p| p.get(&id)).is_none() {
            return Reading::Absent;
        }
        if !lens.sees_person(&id) {
            return Reading::Restricted;
        }
        match crate::person::build(flat, &id, lens) {
            Some(p) => Reading::Ok(Box::new(p)),
            None => Reading::Absent,
        }
    });
    match outcome {
        Reading::Ok(p) => render::page(
            "_panel.html",
            context! {
                p,
                is_admin,
                compact => true,
                max_upload_mb => crate::documents::MAX_UPLOAD / (1024 * 1024),
            },
        ),
        Reading::Restricted => restricted_page(viewer.signed_in()),
        Reading::Absent => render::error_page(
            StatusCode::NOT_FOUND,
            "No such person",
            "This bundle contains no person with that id.",
        ),
    }
}

/// The three answers a read of one entity can have.
///
/// "Absent" and "restricted" are kept apart deliberately. Collapsing them into
/// a 404 is the reflex, but it buys nothing here: the tree already shows that a
/// hidden person exists, so a 404 would be a lie that protects nothing while
/// telling a legitimate reader — a family member who is simply signed out —
/// that the record they were sent is missing rather than closed to them.
enum Reading {
    Ok(Box<crate::person::PersonView>),
    Restricted,
    Absent,
}

/// The page shown for a record this reader may not read.
fn restricted_page(signed_in: bool) -> Response {
    let detail = if signed_in {
        "This record's visibility puts it above what your account may read.          An administrator can change either the record's visibility or your          role."
    } else {
        "This record is not public. Sign in to see whether your account may          read it."
    };
    render::error_page(StatusCode::FORBIDDEN, "Not visible to you", detail)
}

/// Every readable person as `{id, name}`, sorted by name, for the root picker.
///
/// Hidden people are left out entirely rather than redacted, which is the one
/// place this application omits rather than redacts — and for a reason that
/// does not apply anywhere else. The roster is a *destination list*: every
/// entry is somewhere the reader can go. A row reading "Private" leads
/// nowhere, and a searchable list of them would also be the one surface where
/// existence turns into an enumerable index.
fn person_roster(flat: &Value, lens: &crate::access::Lens) -> Vec<Value> {
    let Some(persons) = flat.get("persons").and_then(Value::as_object) else {
        return Vec::new();
    };
    let mut out: Vec<(String, String)> = persons
        .iter()
        .filter(|(id, _)| lens.sees_person(id))
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
    let viewer = auth::viewer(&state, &headers);
    let is_admin = viewer.is_admin();
    let outcome = state.read_as(viewer.ceiling(), |flat, lens| {
        if flat.get("persons").and_then(|p| p.get(&id)).is_none() {
            return Reading::Absent;
        }
        if !lens.sees_person(&id) {
            return Reading::Restricted;
        }
        match crate::person::build(flat, &id, lens) {
            Some(p) => Reading::Ok(Box::new(p)),
            None => Reading::Absent,
        }
    });

    match outcome {
        Reading::Ok(p) => render::page(
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
        Reading::Restricted => restricted_page(viewer.signed_in()),
        Reading::Absent => render::error_page(
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
fn stored_document(
    state: &Shared,
    viewer: &crate::access::Viewer,
    id: &str,
) -> Option<StoredDocument> {
    let doc = state.read_as(viewer.ceiling(), |flat, lens| {
        // A document is reached through the person who attaches it, so that is
        // what governs its bytes. Checked here rather than in each of the three
        // handlers, because /raw, /view and /thumb are three doors into the
        // same file and a check on two of them is a check on none.
        if !crate::access::may_read_document(flat, lens.visible(), viewer.signed_in(), id) {
            return None;
        }
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
pub async fn document_raw(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Response {
    let viewer = auth::viewer(&state, &headers);
    let Some(doc) = stored_document(&state, &viewer, &id) else {
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
pub async fn document_view(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Response {
    let viewer = auth::viewer(&state, &headers);
    let Some(doc) = stored_document(&state, &viewer, &id) else {
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
    document_raw(State(state), headers, Path(id)).await
}

/// `GET /document/:id/thumb` — a downscaled PNG, or 404 for a non-image.
pub async fn document_thumb(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Response {
    let viewer = auth::viewer(&state, &headers);
    let Some(doc) = stored_document(&state, &viewer, &id) else {
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

/// `GET /health` — liveness, plus the entity counts this requester may see.
///
/// The only JSON endpoint the application serves, and therefore the one a
/// visibility rule is easiest to forget on: nothing here renders through a
/// template, so nothing here would have been covered by a template-level
/// check.
///
/// `persons` is the count this requester may *read*, not the count the bundle
/// holds. It stays unauthenticated: the endpoint's job is liveness, monitors
/// are not signed in, and the number it now reports is one the tree page
/// already states in words — "17 people are shown without their details" — so
/// withholding it here would protect nothing and break the monitor.
pub async fn health(State(state): State<Shared>, headers: HeaderMap) -> Response {
    let viewer = auth::viewer(&state, &headers);
    let counts = state.counts();
    let persons_visible = state.read_as(viewer.ceiling(), |flat, lens| {
        flat.get("persons")
            .and_then(Value::as_object)
            .map(|p| lens.count(p.len()))
    });
    let mut entities = serde_json::Map::new();
    let mut total = 0usize;
    for (k, n) in &counts {
        // Persons are the one collection with a per-entity ceiling, so they
        // are the one collection whose count depends on who is asking.
        let n = if *k == "persons" {
            persons_visible.unwrap_or(*n)
        } else {
            *n
        };
        total += n;
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
