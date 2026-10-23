//! An account's own page (2026-10-23): who it is, and which person in the
//! tree it says it is. Any signed-in account has one, whatever its role; the
//! link to a record is each account's own word and nobody checks it.

use axum::extract::{Form, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Redirect, Response};
use minijinja::context;
use serde::Deserialize;
use serde_json::json;

use crate::routes::Shared;
use crate::{auth, render, view};

/// `?done=linked|unlinked`: back from the form.
#[derive(Deserialize, Default)]
pub struct AccountQuery {
    #[serde(default)]
    done: Option<String>,
}

/// `GET /account` — the account's page.
pub async fn page(
    State(state): State<Shared>,
    headers: HeaderMap,
    Query(q): Query<AccountQuery>,
) -> Response {
    let viewer = auth::viewer(&state, &headers);
    if !viewer.signed_in() {
        return Redirect::to("/admin/login").into_response();
    }
    let chrome = render::Chrome::resolve(&viewer, &headers, "/account");
    let notice = match q.done.as_deref() {
        Some("linked") => Some(chrome.t("account-person-linked")),
        Some("unlinked") => Some(chrome.t("account-person-unlinked")),
        _ => None,
    };
    render_page(&state, &chrome, &viewer, notice, None, StatusCode::OK)
}

fn render_page(
    state: &Shared,
    chrome: &render::Chrome,
    viewer: &crate::access::Viewer,
    notice: Option<String>,
    error: Option<String>,
    status: StatusCode,
) -> Response {
    // The account as stored now, not as the session saw it at sign-in.
    let user = viewer
        .user
        .as_ref()
        .and_then(|u| state.acl_read(|acl| acl.by_id(&u.id).cloned()));
    let (options, person) = state.read_as(viewer.ceiling(), |flat, lens| {
        let options = crate::forms::person_options(flat, lens);
        let person = user
            .as_ref()
            .and_then(|u| u.person_id.as_ref())
            .and_then(|pid| {
                let p = flat.get("persons")?.get(pid)?;
                Some(if lens.sees_person(pid) {
                    json!({ "id": pid, "name": view::person_display_name(p), "readable": true })
                } else {
                    json!({ "id": pid, "readable": false })
                })
            });
        (options, person)
    });
    let mut resp = render::page_with(
        chrome,
        "account.html",
        context! {
            nav => "account",
            emergency => user.is_none(),
            account => user.as_ref().map(|u| json!({
                "username": u.username,
                "role": u.role.as_str(),
                "created_at": u.created_at,
                "email": u.email,
            })),
            person,
            options,
            notice,
            error,
        },
    );
    *resp.status_mut() = status;
    resp
}

#[derive(Deserialize)]
pub struct PersonForm {
    /// The record chosen, as the picker's label or a bare id.
    #[serde(default)]
    person: String,
    /// Present to take the link away.
    #[serde(default)]
    unlink: Option<String>,
}

/// `POST /account/person` — link this account to a person in the tree, or
/// unlink it. Any record this account may read; no one else is asked.
pub async fn person(
    State(state): State<Shared>,
    headers: HeaderMap,
    Form(form): Form<PersonForm>,
) -> Response {
    let viewer = auth::viewer(&state, &headers);
    if !viewer.signed_in() {
        return Redirect::to("/admin/login").into_response();
    }
    let chrome = render::Chrome::resolve(&viewer, &headers, "/account");
    let Some(id) = viewer.user.as_ref().map(|u| u.id.clone()) else {
        // The emergency token is no one: there is no account to link.
        return render_page(
            &state,
            &chrome,
            &viewer,
            None,
            Some(chrome.t("account-emergency-page")),
            StatusCode::BAD_REQUEST,
        );
    };
    let chosen = if form.unlink.is_some() {
        None
    } else {
        let typed = form.person.trim().to_string();
        let found = state.read_as(viewer.ceiling(), |flat, lens| {
            crate::forms::resolve_person(&typed, flat).map(|pid| (lens.sees_person(&pid), pid))
        });
        match found {
            Ok((true, pid)) => Some(pid),
            Ok((false, _)) => {
                let msg = format!(
                    "{} {}",
                    chrome.t("account-person-error"),
                    chrome.t("pick-error-not-found")
                );
                return render_page(
                    &state,
                    &chrome,
                    &viewer,
                    None,
                    Some(msg),
                    StatusCode::BAD_REQUEST,
                );
            }
            Err(e) => {
                let msg = format!("{} {}", chrome.t("account-person-error"), chrome.t(e.key()));
                return render_page(
                    &state,
                    &chrome,
                    &viewer,
                    None,
                    Some(msg),
                    StatusCode::BAD_REQUEST,
                );
            }
        }
    };
    let linked = chosen.is_some();
    if let Err(e) = state.acl_mutate(move |acl| {
        if let Some(u) = acl.users.iter_mut().find(|u| u.id == id) {
            u.person_id = chosen;
        }
    }) {
        tracing::error!(error = %e, "could not store the account's person");
        return render_page(
            &state,
            &chrome,
            &viewer,
            None,
            Some(chrome.t("account-person-io")),
            StatusCode::INTERNAL_SERVER_ERROR,
        );
    }
    Redirect::to(if linked {
        "/account?done=linked"
    } else {
        "/account?done=unlinked"
    })
    .into_response()
}
