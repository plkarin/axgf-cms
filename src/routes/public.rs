//! Public, read-only pages.

use axum::extract::{Path, State};
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
                "title": format!("{preserved} unparseable dates kept verbatim"),
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

/// `GET /tree` — the whole bundle in one scrollable page.
pub async fn tree(State(state): State<Shared>, headers: HeaderMap) -> Response {
    let is_admin = auth::is_admin(&headers, state.admin_token());
    let started = std::time::Instant::now();
    let layout = state.read(crate::tree::layout);
    let elapsed = started.elapsed();
    tracing::debug!(
        people = layout.person_count,
        edges = layout.edges.len(),
        ms = elapsed.as_secs_f64() * 1000.0,
        "tree laid out"
    );

    render::page(
        "tree.html",
        context! {
            nav => "tree",
            is_admin,
            layout,
        },
    )
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
