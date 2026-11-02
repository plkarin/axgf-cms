//! The administration's sixth room: the Artificial family genealogist, the
//! AFG (the operator's request of 2 November 2026, docs/adr/0023). Where its
//! computing comes from, and a test that sends it the archive and one
//! question, its result in a window over the room (docs/adr/0024). The test
//! goes through the asker's brain, as every exchange with the AFG will.
//!
//! Administrators only: this room holds an API key and can start a program
//! on this machine, and its test sends the whole archive away.

use axum::extract::{Form, Path, Query, State};
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
/// each with the fields it uses, its usual address, and the models of the
/// moment it suggests with their efforts (docs/adr/0025).
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
                "suggestions": k.suggestions(),
                "efforts": k.efforts(),
                "program": k.program(),
            })
        })
        .collect()
}

/// The configuration as the page shows it: everything but the key, which is
/// said as set or not. With the efforts offered for its model, and whether
/// its model and its effort are among the suggestions: either may be
/// another, which is sent as it is.
fn config_json(cfg: &Config) -> serde_json::Value {
    let model = cfg.model.trim();
    let effort = cfg.effort.trim();
    let (efforts, default_effort) = crate::afg_models::efforts_for(cfg.kind, model);
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
        "efforts": efforts,
        "default_effort": default_effort,
        "model_unlisted": !model.is_empty()
            && !cfg.kind.suggestions().is_empty()
            && crate::afg_models::find(cfg.kind, model).is_none(),
        "effort_unlisted": !effort.is_empty()
            && !efforts.is_empty()
            && !efforts.contains(&effort),
    })
}

/// What the room says besides its form: a notice, an error, a test's
/// result in its window.
#[derive(Default)]
struct Said {
    notice: Option<String>,
    error: Option<String>,
    run: Option<serde_json::Value>,
}

impl Said {
    fn notice(s: String) -> Self {
        Said {
            notice: Some(s),
            ..Said::default()
        }
    }

    fn error(s: String) -> Self {
        Said {
            error: Some(s),
            ..Said::default()
        }
    }
}

/// A test as the window shows it: its answer's Markdown drawn as the notes'
/// is, what is raw HTML in it shown as text.
fn run_view(run: &crate::afg::Run) -> serde_json::Value {
    let mut v = serde_json::to_value(run).unwrap_or_default();
    if let Some(a) = &run.answer {
        v["answer_html"] = json!(crate::notes::to_html(&a.text, &|_| None));
    }
    v
}

/// The brains beside this archive (docs/adr/0024).
fn brains(state: &Shared) -> crate::brain::Brains {
    crate::brain::Brains::beside(state.bundle_path())
}

fn render_room(
    state: &Shared,
    chrome: &render::Chrome,
    viewer: &crate::access::Viewer,
    status: StatusCode,
    shown: &Config,
    said: Said,
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
    // The asker's brain at a glance; a session with no account has none.
    let owner = viewer.user.as_ref().map(crate::brain::Owner::of);
    let all = brains(state);
    let brain = owner.as_ref().map(|o| all.peek(o).unwrap_or_default());
    let allowed = state.acl_read(|a| {
        a.users
            .iter()
            .filter(|u| u.afg && u.status == crate::acl::Status::Active)
            .count()
    });
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
            brain,
            brain_count => all.count(),
            allowed,
            run => said.run,
            notice => said.notice,
            error => said.error,
        },
    );
    *resp.status_mut() = status;
    resp
}

#[derive(Debug, Default, Deserialize)]
pub struct RoomQuery {
    /// A test whose result the room opens in its window.
    #[serde(default)]
    run: Option<String>,
}

/// `GET /admin/afg`, and `?run=<id>` with a test's result in its window.
pub async fn page(
    State(state): State<Shared>,
    headers: HeaderMap,
    Query(q): Query<RoomQuery>,
) -> Response {
    let (viewer, chrome) = admin_only!(state, headers);
    let cfg = state.site().afg;
    let said = match q.run.as_deref().filter(|id| !id.is_empty()) {
        None => Said::default(),
        Some(id) => match state.afg_runs().get(id) {
            Some(r) => Said {
                run: Some(run_view(&r)),
                ..Said::default()
            },
            None => Said::error(chrome.t("afg-run-gone-detail")),
        },
    };
    render_room(&state, &chrome, &viewer, StatusCode::OK, &cfg, said)
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
            &viewer,
            StatusCode::UNPROCESSABLE_ENTITY,
            &proposed,
            Said::error(chrome.t(key)),
        );
    }
    if let Err(e) = state.site_mutate(|s| s.afg = proposed.clone()) {
        return render_room(
            &state,
            &chrome,
            &viewer,
            StatusCode::INTERNAL_SERVER_ERROR,
            &proposed,
            Said::error(chrome.t_args("afg-error-not-saved", &[("error", e.to_string().into())])),
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
        &viewer,
        StatusCode::OK,
        &saved,
        Said::notice(chrome.t("afg-saved")),
    )
}

/// `POST /admin/afg/test` — send the archive, the asker's brain and the
/// test question, and open the room with the window that waits for the
/// answer.
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
            &viewer,
            StatusCode::UNPROCESSABLE_ENTITY,
            &cfg,
            Said::error(chrome.t(key)),
        );
    }
    let archive = match state.read(|flat| crate::afg::archive(flat, cfg.sensitive)) {
        Ok(a) => a,
        Err(e) => {
            return render_room(
                &state,
                &chrome,
                &viewer,
                StatusCode::INTERNAL_SERVER_ERROR,
                &cfg,
                Said::error(chrome.t_args("afg-error-archive", &[("error", e.to_string().into())])),
            )
        }
    };
    // The asker's brain (docs/adr/0024), made at their first exchange. A
    // session with no account has none, and a brain that cannot be made
    // leaves the test to go without one rather than fail.
    let brain = viewer.user.as_ref().and_then(|u| {
        brains(&state)
            .of(&crate::brain::Owner::of(u))
            .map_err(|e| tracing::warn!(error = %e, user = %u.username, "AFG: no brain"))
            .ok()
    });
    let recall = brain.as_ref().map(crate::brain::Brain::recall);
    let person = brain
        .as_ref()
        .and_then(|b| b.owner.person_id.clone())
        .and_then(|pid| {
            state.read(|flat| {
                flat.get("persons")
                    .and_then(|m| m.get(&pid))
                    .map(|p| (pid.clone(), crate::view::person_display_name(p)))
            })
        });
    let question = chrome.t("afg-test-question");
    let asker = match (&brain, &recall) {
        (Some(b), Some(r)) => Some(crate::afg::Asker {
            name: &b.owner.name,
            person: person
                .as_ref()
                .map(|(id, name)| (id.as_str(), name.as_str())),
            recall: r,
        }),
        _ => None,
    };
    let prompt = crate::afg::prompt(
        &archive,
        &question,
        &chrome.t("afg-test-empty"),
        asker.as_ref(),
    );
    let run = crate::afg::Run::new(
        &cfg,
        &archive,
        &question,
        brain.as_ref().map(|b| b.owner.name.as_str()),
    );
    let id = run.id.clone();
    if let Err(running) = state.afg_runs().begin(run) {
        return Redirect::to(&format!("/admin/afg?run={}", running.id)).into_response();
    }
    tracing::info!(
        by = viewer.name(),
        kind = cfg.kind.slug(),
        files = archive.count,
        bytes = archive.bytes,
        sensitive = archive.sensitive,
        brain = brain.is_some(),
        "AFG test sent"
    );
    let runner = state.clone();
    let base = state.data_dir().to_path_buf();
    let tokens = archive.tokens;
    let run_id = id.clone();
    let kind = cfg.kind.slug();
    let model = cfg.model.trim().to_string();
    tokio::spawn(async move {
        let outcome = crate::afg::ask(cfg, prompt, tokens, base).await;
        let mut remembered = Vec::new();
        let outcome = match outcome {
            Ok(mut answer) => {
                // What the AFG asked to remember is not shown: it is kept.
                let (shown, facts) = crate::brain::take_memory(&answer.text);
                if !shown.is_empty() {
                    answer.text = shown;
                }
                if let Some(b) = brain {
                    let exchange = crate::brain::Exchange {
                        at: crate::view::now_iso8601(),
                        context: "test".into(),
                        kind: kind.into(),
                        model: answer.model.clone().unwrap_or(model),
                        question,
                        answer: answer.text.clone(),
                        remembered: Vec::new(),
                        run: run_id.clone(),
                    };
                    match tokio::task::spawn_blocking(move || b.remember(&exchange, &facts)).await {
                        Ok(Ok(kept)) => remembered = kept,
                        Ok(Err(e)) => tracing::warn!(error = %e, "AFG: the brain was not written"),
                        Err(e) => tracing::warn!(error = %e, "AFG: the brain was not written"),
                    }
                }
                Ok(answer)
            }
            Err(f) => {
                tracing::warn!(error = %f.message, "AFG test failed");
                Err(f)
            }
        };
        runner.afg_runs().end(&run_id, outcome, remembered);
    });
    Redirect::to(&format!("/admin/afg?run={id}")).into_response()
}

/// `GET /admin/afg/run/:id` — a test as the room's window draws it, for its
/// script while it waits: `{"state", "html"}`.
pub async fn run(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Response {
    let (_viewer, chrome) = admin_only!(state, headers);
    let Some(run) = state.afg_runs().get(&id) else {
        return (
            StatusCode::NOT_FOUND,
            axum::Json(json!({"state": "gone", "text": chrome.t("afg-run-gone-detail")})),
        )
            .into_response();
    };
    match render::fragment_with(
        &chrome,
        "_afg_result.html",
        context! { run => run_view(&run) },
    ) {
        Ok(html) => {
            axum::Json(json!({"id": run.id, "state": run.state, "html": html})).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            axum::Json(json!({"state": "error", "text": e})),
        )
            .into_response(),
    }
}
