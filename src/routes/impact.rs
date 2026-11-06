//! The Universal Synoptic Timeline, "Impact" (the operator's request of
//! 6 November 2026, docs/adr/0034): the family's view at `/impact`, a
//! person's at `/impact/:id`, and the page's script. Both read through the
//! reader's lens (`crate::impact`).

use axum::extract::{Path, Query, State};
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use minijinja::context;
use serde::Deserialize;
use serde_json::Value;

use crate::impact::{self, Span};
use crate::routes::Shared;
use crate::{auth, render};

#[derive(Debug, Deserialize)]
pub struct FamilyQuery {
    /// `family` for the family's own centuries; the whole of history
    /// otherwise.
    #[serde(default)]
    span: Option<String>,
}

/// `GET /impact` — the family's lives along the history of the world.
pub async fn family(
    State(state): State<Shared>,
    headers: HeaderMap,
    Query(q): Query<FamilyQuery>,
) -> Response {
    let viewer = auth::viewer(&state, &headers);
    let span = match q.span.as_deref() {
        Some("family") => Span::Family,
        _ => Span::All,
    };
    let here = match span {
        Span::Family => "/impact?span=family",
        Span::All => "/impact",
    };
    let chrome = render::Chrome::resolve(&viewer, &headers, here).with_archive(&state);
    let now = impact::now();
    let tl = state.read_as(viewer.ceiling(), |flat, lens| {
        impact::family(flat, lens, chrome.lang, span, now)
    });
    render::page_with(
        &chrome,
        "impact.html",
        context! {
            nav => "impact",
            tl,
            span => if span == Span::Family { "family" } else { "all" },
            kinds => impact::Kind::ALL.iter().map(|k| k.slug()).collect::<Vec<_>>(),
        },
    )
}

/// What a person's view found.
enum Found {
    Placed(Box<impact::Person>),
    Unplaced,
    Restricted,
    Absent,
}

/// `GET /impact/:id` — one person in their times.
pub async fn person(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Response {
    let viewer = auth::viewer(&state, &headers);
    let chrome =
        render::Chrome::resolve(&viewer, &headers, &format!("/impact/{id}")).with_archive(&state);
    let now = impact::now();
    let found = state.read_as(viewer.ceiling(), |flat, lens| {
        if flat.get("persons").and_then(|p| p.get(&id)).is_none() {
            return Found::Absent;
        }
        if !lens.sees_person(&id) {
            return Found::Restricted;
        }
        match impact::person(flat, lens, &id, chrome.lang, now) {
            Some(p) => Found::Placed(Box::new(p)),
            None => Found::Unplaced,
        }
    });
    match found {
        Found::Placed(p) => render::page_with(
            &chrome,
            "impact_person.html",
            context! {
                nav => "impact",
                p,
                kinds => impact::Kind::ALL.iter().map(|k| k.slug()).collect::<Vec<_>>(),
            },
        ),
        Found::Unplaced => render::error_page_back_in(
            &chrome,
            StatusCode::NOT_FOUND,
            "impact-title",
            "impact-unplaced",
            &[],
            Some((&format!("/person/{id}"), "impact-record")),
        ),
        Found::Restricted => super::public::restricted_page(&chrome, viewer.signed_in()),
        Found::Absent => render::error_page_in(
            &chrome,
            StatusCode::NOT_FOUND,
            "error-no-such-person-title",
            "error-no-such-person-detail",
        ),
    }
}

/// Whether a person's record offers its way to the timeline.
pub fn offered(flat: &Value, id: &str) -> bool {
    impact::placeable(flat, id)
}

/// `GET /static/impact.js` — the band of a life across the timeline.
pub async fn script() -> Response {
    (
        [
            (header::CONTENT_TYPE, "text/javascript; charset=utf-8"),
            (header::CACHE_CONTROL, "public, max-age=3600"),
        ],
        render::IMPACT_JS,
    )
        .into_response()
}
