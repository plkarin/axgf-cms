//! The administration's sixth room: the Artificial family genealogist, the
//! AFG (the operator's request of 2 November 2026, docs/adr/0023). Where its
//! computing comes from, and a test that sends it the archive and one
//! question.
//!
//! Administrators only: this room holds an API key and can start a program
//! on this machine, and its test sends the whole archive away.

use axum::extract::{Form, Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Redirect, Response};
use minijinja::context;
use serde::Deserialize;
use serde_json::json;

use super::admin::{require, Need};
use crate::afg::{Config, Kind};
use crate::render;
use crate::routes::Shared;

/// Refuse anyone but an administrator, as every room of the console does.
macro_rules! admin_only {
    ($state:expr, $headers:expr) => {
        match require(&$state, &$headers, Need::Admin) {
            Ok(v) => v,
            Err(r) => return r,
        }
    };
}

/// The kinds, as the page offers them: grouped as the operator named them,
/// each with the fields it uses and its usual address.
fn kinds_json() -> Vec<serde_json::Value> {
    Kind::ALL
        .iter()
        .map(|k| {
            json!({
                "slug": k.slug(),
                "group": k.group(),
                "uses": k.uses(),
                "default_url": k.default_url(),
                "path": k.path(),
                "efforts": k.efforts(),
                "program": k.program(),
            })
        })
        .collect()
}

/// The configuration as the page shows it: everything but the key, which is
/// said as set or not.
fn config_json(cfg: &Config) -> serde_json::Value {
    json!({
        "kind": cfg.kind.slug(),
        "group": cfg.kind.group(),
        "url": cfg.url,
        "key_hint": cfg.key_hint(),
        "model": cfg.model,
        "effort": cfg.effort,
        "program": cfg.program,
        "sensitive": cfg.sensitive,
        "timeout": cfg.timeout().as_secs(),
        "endpoint": cfg.endpoint(),
        "base_url": cfg.base_url(),
    })
}

fn render_room(
    state: &Shared,
    chrome: &render::Chrome,
    status: StatusCode,
    shown: &Config,
    notice: Option<String>,
    error: Option<String>,
) -> Response {
    let stored = state.site().afg;
    // What the test would send, as it would send it; and if it cannot be
    // read, why, rather than a dash.
    let (archive, archive_error) =
        match state.read(|flat| crate::afg::archive(flat, stored.sensitive)) {
            Ok(a) => (Some(a), None),
            Err(e) => {
                tracing::warn!(error = %format!("{e:#}"), "AFG: the archive could not be read");
                (
                    None,
                    Some(chrome.t_args("afg-error-archive", &[("error", format!("{e:#}").into())])),
                )
            }
        };
    let program =
        crate::afg::find_program(shown.kind, &shown.program).map(|p| p.display().to_string());
    let mut resp = render::page_with(
        chrome,
        "admin_afg.html",
        context! {
            nav => "admin",
            admin_page => "afg",
            cfg => config_json(shown),
            saved_kind => stored.kind.slug(),
            ready => stored.kind != Kind::Off && stored.problem().is_none(),
            kinds => kinds_json(),
            program,
            archive,
            archive_error,
            question => chrome.t("afg-test-question"),
            latest => state.afg_runs().latest(),
            notice,
            error,
        },
    );
    *resp.status_mut() = status;
    resp
}

/// `GET /admin/afg`
pub async fn page(State(state): State<Shared>, headers: HeaderMap) -> Response {
    let (_viewer, chrome) = admin_only!(state, headers);
    let cfg = state.site().afg;
    render_room(&state, &chrome, StatusCode::OK, &cfg, None, None)
}

/// The room's form.
#[derive(Debug, Default, Deserialize)]
pub struct AfgForm {
    #[serde(default)]
    kind: String,
    #[serde(default)]
    url: String,
    #[serde(default)]
    api_key: String,
    #[serde(default)]
    clear_key: Option<String>,
    #[serde(default)]
    model: String,
    #[serde(default)]
    effort: String,
    #[serde(default)]
    program: String,
    #[serde(default)]
    sensitive: Option<String>,
    #[serde(default)]
    timeout: String,
}

/// `POST /admin/afg` — where the computing comes from.
pub async fn save(
    State(state): State<Shared>,
    headers: HeaderMap,
    Form(f): Form<AfgForm>,
) -> Response {
    let (viewer, chrome) = admin_only!(state, headers);
    let stored = state.site().afg;
    let mut proposed = stored.clone();
    proposed.kind = Kind::parse(&f.kind).unwrap_or(Kind::Off);
    // The usual address is kept as nothing, so that it follows the kind.
    proposed.url = match f.url.trim().trim_end_matches('/') {
        u if u == proposed.kind.default_url() => String::new(),
        u => u.to_string(),
    };
    if f.clear_key.is_some() {
        proposed.api_key.clear();
    } else if !f.api_key.trim().is_empty() {
        proposed.api_key = f.api_key.trim().to_string();
    }
    proposed.model = f.model.trim().to_string();
    proposed.effort = f.effort.trim().to_string();
    proposed.program = f.program.trim().to_string();
    proposed.sensitive = f.sensitive.is_some();
    proposed.timeout_secs = f.timeout.trim().parse().unwrap_or(0);
    if let Some(key) = proposed.problem() {
        return render_room(
            &state,
            &chrome,
            StatusCode::UNPROCESSABLE_ENTITY,
            &proposed,
            None,
            Some(chrome.t(key)),
        );
    }
    if let Err(e) = state.site_mutate(|s| s.afg = proposed.clone()) {
        return render_room(
            &state,
            &chrome,
            StatusCode::INTERNAL_SERVER_ERROR,
            &proposed,
            None,
            Some(chrome.t_args("afg-error-not-saved", &[("error", e.to_string().into())])),
        );
    }
    // Who changed where the archive may be sent; never the key itself.
    tracing::warn!(
        by = viewer.name(),
        kind = proposed.kind.slug(),
        model = %proposed.model,
        key_changed = proposed.api_key != stored.api_key,
        sensitive = proposed.sensitive,
        "AFG source changed"
    );
    let saved = state.site().afg;
    render_room(
        &state,
        &chrome,
        StatusCode::OK,
        &saved,
        Some(chrome.t("afg-saved")),
        None,
    )
}

/// `POST /admin/afg/test` — send the archive and the test question, and go
/// to the page that waits for the answer.
pub async fn test(State(state): State<Shared>, headers: HeaderMap) -> Response {
    let (viewer, chrome) = admin_only!(state, headers);
    let cfg = state.site().afg;
    let problem = match cfg.kind {
        Kind::Off => Some("afg-error-off"),
        _ => cfg.problem(),
    };
    if let Some(key) = problem {
        return render_room(
            &state,
            &chrome,
            StatusCode::UNPROCESSABLE_ENTITY,
            &cfg,
            None,
            Some(chrome.t(key)),
        );
    }
    let archive = match state.read(|flat| crate::afg::archive(flat, cfg.sensitive)) {
        Ok(a) => a,
        Err(e) => {
            return render_room(
                &state,
                &chrome,
                StatusCode::INTERNAL_SERVER_ERROR,
                &cfg,
                None,
                Some(chrome.t_args("afg-error-archive", &[("error", e.to_string().into())])),
            )
        }
    };
    let question = chrome.t("afg-test-question");
    let prompt = crate::afg::prompt(&archive, &question, &chrome.t("afg-test-empty"));
    let run = crate::afg::Run::new(&cfg, &archive, &question);
    let id = run.id.clone();
    if let Err(running) = state.afg_runs().begin(run) {
        return Redirect::to(&format!("/admin/afg/run/{}", running.id)).into_response();
    }
    tracing::info!(
        by = viewer.name(),
        kind = cfg.kind.slug(),
        files = archive.count,
        bytes = archive.bytes,
        sensitive = archive.sensitive,
        "AFG test sent"
    );
    let runner = state.clone();
    let base = state.data_dir().to_path_buf();
    let tokens = archive.tokens;
    let run_id = id.clone();
    tokio::spawn(async move {
        let outcome = crate::afg::ask(cfg, prompt, tokens, base).await;
        if let Err(f) = &outcome {
            tracing::warn!(error = %f.message, "AFG test failed");
        }
        runner.afg_runs().end(&run_id, outcome);
    });
    Redirect::to(&format!("/admin/afg/run/{id}")).into_response()
}

/// `GET /admin/afg/run/:id` — a test, waiting or answered.
pub async fn run(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Response {
    let (_viewer, chrome) = admin_only!(state, headers);
    let Some(run) = state.afg_runs().get(&id) else {
        return render::error_page_in(
            &chrome,
            StatusCode::NOT_FOUND,
            "afg-run-gone-title",
            "afg-run-gone-detail",
        );
    };
    render::page_with(
        &chrome,
        "admin_afg_run.html",
        context! {
            nav => "admin",
            admin_page => "afg",
            run,
        },
    )
}
