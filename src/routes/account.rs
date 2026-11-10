//! An account's own page (2026-10-23): who it is, and which person in the
//! tree it says it is. Any signed-in account has one, whatever its role; the
//! link to a record is each account's own word and nobody checks it.
//!
//! And what the AFG remembers of it, its brain (2026-11-02, docs/adr/0024
//! §6): the account's own to see and to forget, a fact, an exchange or all
//! of it. Nobody sees another account's brain here, administrators
//! included.

use axum::extract::{Form, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Redirect, Response};
use minijinja::context;
use serde::Deserialize;
use serde_json::json;

use crate::routes::Shared;
use crate::{auth, render, view};

/// How many of an account's exchanges with the AFG its page lists, the most
/// recent first.
const EXCHANGES_LISTED: usize = 200;

/// `?done=linked|unlinked|forgotten|forgotten-all|already`: back from a form.
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
    let chrome = render::Chrome::resolve(&viewer, &headers, "/account").with_archive(&state);
    let notice = match q.done.as_deref() {
        Some("linked") => Some(chrome.t("account-person-linked")),
        Some("unlinked") => Some(chrome.t("account-person-unlinked")),
        Some("forgotten") => Some(chrome.t("account-brain-forgotten")),
        Some("forgotten-all") => Some(chrome.t("account-brain-forgotten-all")),
        Some("already") => Some(chrome.t("account-brain-already")),
        Some("revoked") => Some(chrome.t("account-token-revoked")),
        _ => None,
    };
    render_page(
        &state,
        &chrome,
        &viewer,
        &headers,
        notice,
        None,
        StatusCode::OK,
    )
}

fn render_page(
    state: &Shared,
    chrome: &render::Chrome,
    viewer: &crate::access::Viewer,
    headers: &HeaderMap,
    notice: Option<String>,
    error: Option<String>,
    status: StatusCode,
) -> Response {
    render_with(state, chrome, viewer, headers, notice, error, status, None)
}

/// The address AI assistants reach the MCP at: under the public address
/// when one is set, else under the one this page was asked for.
fn mcp_url(state: &Shared, headers: &HeaderMap) -> String {
    let base = state.site().public_base().unwrap_or_else(|| {
        let host = headers
            .get("x-forwarded-host")
            .or_else(|| headers.get(axum::http::header::HOST))
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.split(',').next())
            .map(str::trim)
            .unwrap_or("localhost");
        let scheme = if crate::session::is_tls(headers) {
            "https"
        } else {
            "http"
        };
        format!("{scheme}://{host}")
    });
    format!("{base}/mcp")
}

#[allow(clippy::too_many_arguments)]
fn render_with(
    state: &Shared,
    chrome: &render::Chrome,
    viewer: &crate::access::Viewer,
    headers: &HeaderMap,
    notice: Option<String>,
    error: Option<String>,
    status: StatusCode,
    minted: Option<(String, String)>,
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
    // The AFG's brain of this account, and only of this one.
    let brain = user.as_ref().map(|u| {
        let b =
            crate::brain::Brains::beside(state.bundle_path()).existing(&crate::brain::Owner::of(u));
        let (listed, total) = b
            .as_ref()
            .map(|b| b.listed_exchanges(EXCHANGES_LISTED))
            .unwrap_or_default();
        let exchanges: Vec<serde_json::Value> = listed
            .iter()
            .map(|l| {
                let mut v = serde_json::to_value(l).unwrap_or_default();
                v["answer_html"] = json!(crate::notes::to_html(&l.exchange.answer, &|_| None));
                v
            })
            .collect();
        json!({
            "allowed": u.afg,
            "exists": b.is_some(),
            "facts": b.as_ref().map(|b| b.listed_facts()).unwrap_or_default(),
            "older": total.saturating_sub(exchanges.len()),
            "total": total,
            "exchanges": exchanges,
        })
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
            brain,
            notice,
            error,
            tokens => user.as_ref().map(|u| u.api_tokens.iter().map(|t| json!({
                "id": t.id,
                "label": t.label,
                "write": t.write,
                "created_at": t.created_at,
                "last_used": t.last_used,
            })).collect::<Vec<_>>()).unwrap_or_default(),
            mcp => json!({
                "open": state.site().mcp,
                "admin": viewer.is_admin(),
                "may_write": viewer.may_write(),
                "url": mcp_url(state, headers),
                "most": crate::tokens::MOST,
                "label_max": crate::tokens::LABEL_MAX,
            }),
            minted => minted.map(|(label, token)| json!({"label": label, "token": token})),
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
    let chrome = render::Chrome::resolve(&viewer, &headers, "/account").with_archive(&state);
    let Some(id) = viewer.user.as_ref().map(|u| u.id.clone()) else {
        // The emergency token is no one: there is no account to link.
        return render_page(
            &state,
            &chrome,
            &viewer,
            &headers,
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
                    &headers,
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
                    &headers,
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
            &headers,
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

#[derive(Deserialize, Default)]
pub struct ForgetForm {
    /// A fact's mark, from the list.
    #[serde(default)]
    fact: Option<String>,
    /// An exchange's mark, from the list.
    #[serde(default)]
    exchange: Option<String>,
    /// Present to forget everything, with `confirm`.
    #[serde(default)]
    all: Option<String>,
    #[serde(default)]
    confirm: Option<String>,
}

/// `POST /account/brain/forget` — forget a fact, an exchange, or everything
/// the AFG remembers of this account (docs/adr/0024 §6). Whose brain is the
/// session's account, never a field of the form.
pub async fn forget(
    State(state): State<Shared>,
    headers: HeaderMap,
    Form(form): Form<ForgetForm>,
) -> Response {
    let viewer = auth::viewer(&state, &headers);
    if !viewer.signed_in() {
        return Redirect::to("/admin/login").into_response();
    }
    let chrome = render::Chrome::resolve(&viewer, &headers, "/account").with_archive(&state);
    let Some(user) = viewer.user.as_ref() else {
        // The emergency token is no one: it has no brain.
        return render_page(
            &state,
            &chrome,
            &viewer,
            &headers,
            None,
            Some(chrome.t("account-emergency-page")),
            StatusCode::BAD_REQUEST,
        );
    };
    let owner = crate::brain::Owner::of(user);
    let brains = crate::brain::Brains::beside(state.bundle_path());
    let said = |done: bool, what: &'static str| if done { what } else { "already" };
    let (what, outcome) = if form.all.is_some() {
        if form.confirm.is_none() {
            return render_page(
                &state,
                &chrome,
                &viewer,
                &headers,
                None,
                Some(chrome.t("account-brain-confirm-needed")),
                StatusCode::BAD_REQUEST,
            );
        }
        (
            "all",
            brains.forget_all(&owner).map(|d| said(d, "forgotten-all")),
        )
    } else {
        match (
            brains.existing(&owner),
            form.fact.as_deref(),
            form.exchange.as_deref(),
        ) {
            (Some(b), Some(id), _) => ("fact", b.forget_fact(id).map(|d| said(d, "forgotten"))),
            (Some(b), None, Some(id)) => (
                "exchange",
                b.forget_exchange(id).map(|d| said(d, "forgotten")),
            ),
            _ => ("nothing", Ok("already")),
        }
    };
    match outcome {
        Ok(done) => {
            // That the account forgot, and what kind of thing; never what.
            tracing::info!(account = %user.username, what, done, "AFG brain: forgotten by its owner");
            Redirect::to(&format!("/account?done={done}#account-brain")).into_response()
        }
        Err(e) => {
            tracing::error!(error = %e, account = %user.username, "AFG brain: could not forget");
            render_page(
                &state,
                &chrome,
                &viewer,
                &headers,
                None,
                Some(chrome.t_args("account-brain-error", &[("error", e.to_string().into())])),
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        }
    }
}

#[derive(Deserialize, Default)]
pub struct TokenForm {
    #[serde(default)]
    label: String,
    /// Present for a token that may write.
    #[serde(default)]
    write: Option<String>,
}

/// `POST /account/tokens` — a new access token for the MCP (docs/adr/0045
/// §3), shown on the page this once and never again: only its digest is
/// kept. Refused while the MCP is closed.
pub async fn make_token(
    State(state): State<Shared>,
    headers: HeaderMap,
    Form(form): Form<TokenForm>,
) -> Response {
    let viewer = auth::viewer(&state, &headers);
    if !viewer.signed_in() {
        return Redirect::to("/admin/login").into_response();
    }
    let chrome = render::Chrome::resolve(&viewer, &headers, "/account").with_archive(&state);
    let refuse = |key: &str, status| {
        render_page(
            &state,
            &chrome,
            &viewer,
            &headers,
            None,
            Some(chrome.t(key)),
            status,
        )
    };
    let Some(id) = viewer.user.as_ref().map(|u| u.id.clone()) else {
        return refuse("account-emergency-page", StatusCode::BAD_REQUEST);
    };
    if !state.site().mcp {
        return refuse("account-tokens-closed", StatusCode::FORBIDDEN);
    }
    let write = form.write.is_some() && viewer.may_write();
    let (kept, token) = crate::tokens::mint(&form.label, write, &crate::view::now_iso8601());
    let label = kept.label.clone();
    let added = state.acl_mutate(move |acl| {
        let Some(u) = acl.users.iter_mut().find(|u| u.id == id) else {
            return false;
        };
        if u.api_tokens.len() >= crate::tokens::MOST {
            return false;
        }
        u.api_tokens.push(kept);
        true
    });
    match added {
        Ok(true) => {
            tracing::info!(account = %viewer.name(), write, "an access token for the MCP was made");
            render_with(
                &state,
                &chrome,
                &viewer,
                &headers,
                None,
                None,
                StatusCode::OK,
                Some((label, token)),
            )
        }
        Ok(false) => refuse("account-tokens-too-many", StatusCode::BAD_REQUEST),
        Err(e) => {
            tracing::error!(error = %e, "could not store an access token");
            refuse("account-person-io", StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// `POST /account/tokens/:id/revoke` — a token of this account's, gone: it
/// opens nothing any more.
pub async fn revoke_token(
    State(state): State<Shared>,
    headers: HeaderMap,
    axum::extract::Path(tid): axum::extract::Path<String>,
) -> Response {
    let viewer = auth::viewer(&state, &headers);
    if !viewer.signed_in() {
        return Redirect::to("/admin/login").into_response();
    }
    let Some(id) = viewer.user.as_ref().map(|u| u.id.clone()) else {
        return Redirect::to("/account").into_response();
    };
    let removed = state.acl_mutate(move |acl| {
        acl.users
            .iter_mut()
            .find(|u| u.id == id)
            .map(|u| {
                let before = u.api_tokens.len();
                u.api_tokens.retain(|t| t.id != tid);
                before != u.api_tokens.len()
            })
            .unwrap_or(false)
    });
    match removed {
        Ok(_) => Redirect::to("/account?done=revoked#account-tokens").into_response(),
        Err(e) => {
            tracing::error!(error = %e, "could not revoke an access token");
            let chrome =
                render::Chrome::resolve(&viewer, &headers, "/account").with_archive(&state);
            render_page(
                &state,
                &chrome,
                &viewer,
                &headers,
                None,
                Some(chrome.t("account-person-io")),
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        }
    }
}
