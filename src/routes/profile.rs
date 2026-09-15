//! The profile editor: one form per AXGF 1.1 group.
//!
//! Every save ends at [`super::admin::save_entity`], which is where
//! `family_scope`, the version check, the conflict screen and the journal live,
//! and so every save is one `update_entity` on the person. Before it gets
//! there the submission has been read back by [`crate::profile::form::apply`]
//! over the *lifted* stored person — so the first save through this editor
//! also moves what the application recorded before 1.1 to where 1.1 keeps it
//! — and then [`crate::sensitive::restore`] has put every location this
//! editor may not read back exactly as the bundle holds it. The group form
//! never draws those locations, so it cannot have meant to change them, and
//! a hand-written POST that names them is ignored the same way.

use axum::extract::{Form, Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Redirect, Response};
use minijinja::context;
use serde_json::Value;

use super::admin::{require, save_entity, submitted_version, Need};
use super::Shared;
use crate::access::Viewer;
use crate::forms::Body;
use crate::profile::form::{self, Choices, Problems};
use crate::render;

/// The precisions a claim's date may state, blank first.
const PRECISIONS: &[&str] = &["", "exact", "year", "month", "decade", "century", "unknown"];

macro_rules! writer {
    ($state:expr, $headers:expr) => {
        match require(&$state, &$headers, Need::Write) {
            Ok(v) => v,
            Err(r) => return r,
        }
    };
}

/// `GET /admin/person/:id/profile` — the first group.
pub async fn edit_first(Path(id): Path<String>) -> Response {
    Redirect::to(&format!("/admin/person/{id}/profile/identity")).into_response()
}

/// `GET /admin/person/:id/profile/:group`
pub async fn edit(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path((id, group)): Path<(String, String)>,
) -> Response {
    let (viewer, chrome) = writer!(state, headers);
    let Some(stored) = stored_person(&state, &id) else {
        return no_person(&chrome);
    };
    let Some(g) = crate::profile::group(&group) else {
        return no_group(&chrome);
    };
    render_editor(
        &state,
        &chrome,
        &viewer,
        &id,
        &stored,
        g,
        None,
        &Problems::new(),
    )
}

/// `POST /admin/person/:id/profile/:group`
pub async fn update(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path((id, group)): Path<(String, String)>,
    Form(body): Form<Body>,
) -> Response {
    let (viewer, chrome) = writer!(state, headers);
    let Some(stored) = stored_person(&state, &id) else {
        return no_person(&chrome);
    };
    let Some(g) = crate::profile::group(&group) else {
        return no_group(&chrome);
    };
    let readable = crate::access::readable_scopes(&stored, viewer.ceiling());
    let lifted = crate::profile::lift::lift(&stored);

    let edited = match form::apply(&lifted, g, readable, &body) {
        Ok(v) => v,
        Err(problems) => {
            let mut resp = render_editor(
                &state,
                &chrome,
                &viewer,
                &id,
                &stored,
                g,
                Some(&body),
                &problems,
            );
            *resp.status_mut() = StatusCode::UNPROCESSABLE_ENTITY;
            return resp;
        }
    };

    // The chokepoint, and the only reason a contributor's save cannot erase a
    // diagnosis they were never shown.
    let mut entity = crate::sensitive::restore(&edited, &stored, readable);
    entity["id"] = Value::String(id.clone());

    save_entity(
        &state,
        &chrome,
        &viewer,
        axgf_rs::EntityKind::Person,
        &id,
        entity,
        Some(&stored),
        submitted_version(&body),
        &format!("/person/{id}?tab=profile&group={}", g.key),
        &format!("/admin/person/{id}/profile/{}", g.key),
    )
}

fn stored_person(state: &Shared, id: &str) -> Option<Value> {
    state.read(|flat| flat.get("persons").and_then(|c| c.get(id)).cloned())
}

fn no_person(chrome: &render::Chrome) -> Response {
    render::error_page_in(
        chrome,
        StatusCode::NOT_FOUND,
        "error-no-such-person-title",
        "error-no-such-person-detail",
    )
}

fn no_group(chrome: &render::Chrome) -> Response {
    render::error_page_in(
        chrome,
        StatusCode::NOT_FOUND,
        "profile-no-such-group-title",
        "profile-no-such-group-detail",
    )
}

/// The documents this person's record attaches, as select options: what an
/// artefact can refer to. A document a class this editor may not read refers
/// to is left out, as it is everywhere else.
fn person_documents(
    flat: &Value,
    id: &str,
    person: &Value,
    ceiling: crate::acl::Visibility,
) -> Vec<Value> {
    let withheld = crate::access::withheld_documents(flat, ceiling);
    let docs = flat.get("documents").and_then(Value::as_object);
    let mut ids: Vec<String> = person
        .get("documents")
        .and_then(Value::as_array)
        .map(|a| {
            a.iter()
                .filter_map(|d| {
                    d.get("document_id")
                        .and_then(Value::as_str)
                        .map(str::to_string)
                })
                .collect()
        })
        .unwrap_or_default();
    if let Some(docs) = docs {
        for (did, d) in docs {
            let linked = d
                .get("linked_to")
                .and_then(Value::as_array)
                .is_some_and(|l| {
                    l.iter().any(|x| {
                        x.get("entity_type").and_then(Value::as_str) == Some("person")
                            && x.get("entity_id").and_then(Value::as_str) == Some(id)
                    })
                });
            if linked {
                ids.push(did.clone());
            }
        }
    }
    ids.sort();
    ids.dedup();
    ids.into_iter()
        .filter(|d| !withheld.contains(d))
        .map(|d| {
            let label = docs
                .and_then(|m| m.get(&d))
                .and_then(|x| x.get("filename").or_else(|| x.get("caption")))
                .and_then(Value::as_str)
                .map(str::to_string)
                .unwrap_or_else(|| crate::forms::short_id(&d).to_string());
            serde_json::json!({"id": d, "label": label})
        })
        .collect()
}

#[allow(clippy::too_many_arguments)]
fn render_editor(
    state: &Shared,
    chrome: &render::Chrome,
    viewer: &Viewer,
    id: &str,
    stored: &Value,
    g: &'static axgf_rs::model::profile::registry::Group,
    submitted: Option<&Body>,
    problems: &Problems,
) -> Response {
    let readable = crate::access::readable_scopes(stored, viewer.ceiling());
    let lifted = crate::profile::lift::lift(stored);
    let (group, tabs) = state.read(|flat| {
        let choices = Choices {
            lang: chrome.lang.to_string(),
            sources: crate::forms::entity_options(flat, "sources", crate::forms::source_label),
            places: crate::forms::entity_options(flat, "places", crate::forms::place_label),
            documents: person_documents(flat, id, stored, viewer.ceiling()),
        };
        let withheld = crate::access::withheld_documents(flat, viewer.ceiling());
        let reader = crate::profile::view::Reader {
            flat,
            lang: chrome.lang,
            readable,
            withheld_documents: &withheld,
        };
        (
            form::editor_group(&lifted, g, readable, &choices, submitted, problems),
            crate::profile::view::tabs(&lifted, &reader),
        )
    });
    let sources = state
        .read(|flat| crate::forms::entity_options(flat, "sources", crate::forms::source_label));
    let is_living = stored
        .pointer("/identity/is_living")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let holds_class = group.class_label.is_some()
        || group.attributes.iter().any(|a| a.class_label.is_some())
        || (g.key == "personality" && is_living);

    render::page_with(
        chrome,
        "admin_profile.html",
        context! {
            nav => "admin",
            id,
            person_name => crate::view::person_display_name(stored),
            group,
            tabs,
            sources,
            precisions => PRECISIONS,
            problems => !problems.is_empty(),
            is_living,
            holds_class,
            base_version => submitted
                .and_then(|b| b.get("base_version").cloned())
                .unwrap_or_else(|| crate::state::version_of(stored).to_string()),
        },
    )
}
