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
}

/// Depth shown above and below the root when none is requested.
const DEFAULT_DEPTH: usize = 3;
/// Upper bound on depth. Past this a "focused" view is the whole bundle again.
const MAX_DEPTH: usize = 8;

/// `GET /tree` — a focused subtree by default, the whole bundle with `?all=1`.
///
/// The full view is laid out correctly but is not usable on a real file: the
/// operator's bundle puts 283 people in generation 0, which is a canvas
/// 17,992px wide. Nobody scrolls that far to find an ancestor, so the default
/// is a few dozen people around one person, and every card re-roots the view.
pub async fn tree(
    State(state): State<Shared>,
    headers: HeaderMap,
    Query(q): Query<TreeQuery>,
) -> Response {
    let is_admin = auth::is_admin(&headers, state.admin_token());
    let show_all = q.all.as_deref().is_some_and(|v| v != "0" && !v.is_empty());
    let depth = q.depth.unwrap_or(DEFAULT_DEPTH).min(MAX_DEPTH);

    let started = std::time::Instant::now();
    let (layout, focus, roster) = state.read(|flat| {
        // The root picker lists everyone by name, so it is built regardless of
        // which view is showing.
        let roster = person_roster(flat);

        if show_all {
            return (crate::tree::layout(flat), None, roster);
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
                (l, Some(focus), roster)
            }
            // An empty bundle has nobody to focus on.
            None => (crate::tree::layout(flat), None, roster),
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
        },
    )
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
            },
        ),
        None => render::error_page(
            StatusCode::NOT_FOUND,
            "No such person",
            "This bundle contains no person with that id.",
        ),
    }
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
