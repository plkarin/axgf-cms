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
use crate::afg::{Config, ImageConfig, ImageKind, Kind};
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

/// The picture service as the page shows it (docs/adr/0029): everything but
/// the key, which is said as set or not.
fn images_json(cfg: &ImageConfig) -> serde_json::Value {
    json!({
        "kind": cfg.kind.slug(),
        "url": cfg.url,
        "key_hint": cfg.key_hint(),
        "model": cfg.model,
        "workflow": cfg.workflow,
        "default_url": cfg.kind.default_url(),
        "default_model": cfg.kind.models().first(),
        // Canva's connection: whether there is one, and whose; never its
        // tokens (docs/adr/0030).
        "canva_connected": cfg.canva.connected(),
        "canva_account": cfg.canva.account,
    })
}

/// The picture services, with the fields each uses, the models of the
/// moment it suggests, and whether it runs at home.
fn image_kinds_json() -> Vec<serde_json::Value> {
    ImageKind::ALL
        .iter()
        .map(|k| {
            let mut uses: Vec<&str> = k.uses().to_vec();
            if *k != ImageKind::Off {
                uses.push(if k.at_home() { "home" } else { "away" });
            }
            json!({
                "slug": k.slug(),
                "models": k.models(),
                "uses": uses.join(" "),
                "default_url": k.default_url(),
            })
        })
        .collect()
}

/// What the room says besides its form: a notice, an error, a test's
/// result in its window.
#[derive(Default)]
struct Said {
    notice: Option<String>,
    error: Option<String>,
    run: Option<serde_json::Value>,
    /// Said by the picture service's form, whose section then opens.
    images: bool,
    /// Where to send the administrator to connect Canva (docs/adr/0030).
    canva_authorize: Option<String>,
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
            images => images_json(&shown.images),
            image_kinds => image_kinds_json(),
            images_ready => stored.images.ready(),
            images_open => said.images || said.canva_authorize.is_some(),
            canva_authorize => said.canva_authorize,
            comfyui_placeholder => crate::comfyui::PLACEHOLDER,
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

/// The picture service's form (docs/adr/0029).
#[derive(Debug, Default, Deserialize)]
pub struct ImagesForm {
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
    /// ComfyUI's workflow, in its API format.
    #[serde(default)]
    workflow: String,
}

/// `POST /admin/afg/images` — where the AFG's pictures are made. A key
/// belongs to the service it was given for: another service, or none, forgets
/// it unless a new one comes with the change.
pub async fn save_images(
    State(state): State<Shared>,
    headers: HeaderMap,
    Form(f): Form<ImagesForm>,
) -> Response {
    let (viewer, chrome) = admin_only!(state, headers);
    let stored = state.site().afg;
    let mut images = stored.images.clone();
    images.kind = ImageKind::parse(&f.kind).unwrap_or(ImageKind::Off);
    images.url = match f.url.trim().trim_end_matches('/') {
        u if u == images.kind.default_url() => String::new(),
        u => u.to_string(),
    };
    if f.clear_key.is_some() || (images.kind != stored.images.kind && f.api_key.trim().is_empty()) {
        images.api_key.clear();
    } else if !f.api_key.trim().is_empty() {
        images.api_key = f.api_key.trim().to_string();
    }
    images.model = f.model.trim().to_string();
    images.workflow = if images.kind == ImageKind::Comfyui {
        f.workflow.trim().to_string()
    } else {
        String::new()
    };
    // Canva's account goes with the service: another one forgets it.
    if images.kind != ImageKind::Canva {
        images.canva.forget_account();
    }
    if images.kind == ImageKind::Off {
        images = ImageConfig::default();
    }
    let mut proposed = stored.clone();
    proposed.images = images;
    // Canva is chosen first and connected after: its connection is not a
    // reason to refuse the choice.
    if let Some(key) = proposed
        .images
        .problem()
        .filter(|k| *k != "afg-error-canva")
    {
        return render_room(
            &state,
            &chrome,
            &viewer,
            StatusCode::UNPROCESSABLE_ENTITY,
            &proposed,
            Said {
                images: true,
                ..Said::error(chrome.t(key))
            },
        );
    }
    if let Err(e) = state.site_mutate(|s| s.afg.images = proposed.images.clone()) {
        return render_room(
            &state,
            &chrome,
            &viewer,
            StatusCode::INTERNAL_SERVER_ERROR,
            &proposed,
            Said {
                images: true,
                ..Said::error(
                    chrome.t_args("afg-error-not-saved", &[("error", e.to_string().into())]),
                )
            },
        );
    }
    // Who changed where the family's photographs may be sent; never the key.
    tracing::warn!(
        by = viewer.name(),
        kind = proposed.images.kind.slug(),
        model = %proposed.images.model,
        key_changed = proposed.images.api_key != stored.images.api_key,
        "AFG picture service changed"
    );
    let saved = state.site().afg;
    render_room(
        &state,
        &chrome,
        &viewer,
        StatusCode::OK,
        &saved,
        Said {
            images: true,
            ..Said::notice(chrome.t(
                if saved.images.kind == ImageKind::Canva && !saved.images.canva.connected() {
                    "afg-images-saved-canva"
                } else {
                    "afg-images-saved"
                },
            ))
        },
    )
}

/// `POST /admin/afg/canva/connect` — begin connecting a Canva account
/// (docs/adr/0030): this site registered with Canva when it was not, and the
/// room opened with the way to Canva and the field its return is pasted in.
pub async fn canva_connect(State(state): State<Shared>, headers: HeaderMap) -> Response {
    let (viewer, chrome) = admin_only!(state, headers);
    let cfg = state.site().afg;
    if cfg.images.kind != ImageKind::Canva {
        return render_room(
            &state,
            &chrome,
            &viewer,
            StatusCode::CONFLICT,
            &cfg,
            Said {
                images: true,
                ..Said::error(chrome.t("afg-canva-save-first"))
            },
        );
    }
    let host = headers
        .get(axum::http::header::HOST)
        .and_then(|h| h.to_str().ok())
        .unwrap_or("localhost");
    let redirect = crate::canva::redirect_uri_for(host);
    match crate::canva::start(
        cfg.images.clone(),
        redirect,
        cfg.timeout().min(std::time::Duration::from_secs(60)),
    )
    .await
    {
        Ok((url, link)) => {
            // The registration kept, whether or not the account follows.
            let _ = state.site_mutate(|s| {
                if s.afg.images.kind == ImageKind::Canva {
                    s.afg.images.canva.client_id = link.client_id.clone();
                    s.afg.images.canva.redirect_uri = link.redirect_uri.clone();
                }
            });
            tracing::warn!(by = viewer.name(), "AFG: a Canva connection started");
            let shown = state.site().afg;
            render_room(
                &state,
                &chrome,
                &viewer,
                StatusCode::OK,
                &shown,
                Said {
                    canva_authorize: Some(url),
                    ..Said::default()
                },
            )
        }
        Err(f) => render_room(
            &state,
            &chrome,
            &viewer,
            StatusCode::BAD_GATEWAY,
            &cfg,
            Said {
                images: true,
                ..Said::error(chrome.t_args("afg-record-failed", &[("reason", f.message.into())]))
            },
        ),
    }
}

/// The form the return from Canva is pasted in.
#[derive(Debug, Default, Deserialize)]
pub struct CanvaFinishForm {
    #[serde(default)]
    returned: String,
}

/// `POST /admin/afg/canva/finish` — the address Canva sent the administrator
/// back to, pasted: the account connected.
pub async fn canva_finish(
    State(state): State<Shared>,
    headers: HeaderMap,
    Form(f): Form<CanvaFinishForm>,
) -> Response {
    let (viewer, chrome) = admin_only!(state, headers);
    canva_finished(&state, &chrome, &viewer, f.returned).await
}

/// `GET /admin/afg/canva/callback` — the same return, reached directly when
/// the administrator's browser finds this site at `localhost`.
pub async fn canva_callback(
    State(state): State<Shared>,
    headers: HeaderMap,
    axum::extract::RawQuery(query): axum::extract::RawQuery,
) -> Response {
    let (viewer, chrome) = admin_only!(state, headers);
    canva_finished(&state, &chrome, &viewer, query.unwrap_or_default()).await
}

async fn canva_finished(
    state: &Shared,
    chrome: &render::Chrome,
    viewer: &crate::access::Viewer,
    returned: String,
) -> Response {
    let cfg = state.site().afg;
    if cfg.images.kind != ImageKind::Canva {
        return render_room(
            state,
            chrome,
            viewer,
            StatusCode::CONFLICT,
            &cfg,
            Said {
                images: true,
                ..Said::error(chrome.t("afg-canva-save-first"))
            },
        );
    }
    let timeout = cfg.timeout().min(std::time::Duration::from_secs(60));
    match crate::canva::finish(cfg.images.clone(), returned, timeout).await {
        Ok(link) => {
            if let Err(e) = state.site_mutate(|s| s.afg.images.canva = link.clone()) {
                return render_room(
                    state,
                    chrome,
                    viewer,
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &cfg,
                    Said {
                        images: true,
                        ..Said::error(
                            chrome
                                .t_args("afg-error-not-saved", &[("error", e.to_string().into())]),
                        )
                    },
                );
            }
            let ready = !link.base_design_id.is_empty();
            tracing::warn!(
                by = viewer.name(),
                base_design = ready,
                "AFG: a Canva account connected"
            );
            let saved = state.site().afg;
            render_room(
                state,
                chrome,
                viewer,
                StatusCode::OK,
                &saved,
                Said {
                    images: true,
                    ..Said::notice(chrome.t(if ready {
                        "afg-canva-done-ready"
                    } else {
                        "afg-canva-done"
                    }))
                },
            )
        }
        Err(f) => render_room(
            state,
            chrome,
            viewer,
            StatusCode::BAD_REQUEST,
            &cfg,
            Said {
                images: true,
                ..Said::error(chrome.t_args("afg-record-failed", &[("reason", f.message.into())]))
            },
        ),
    }
}

/// `POST /admin/afg/canva/disconnect` — the account let go: its tokens
/// forgotten, this site's registration kept.
pub async fn canva_disconnect(State(state): State<Shared>, headers: HeaderMap) -> Response {
    let (viewer, chrome) = admin_only!(state, headers);
    if let Err(e) = state.site_mutate(|s| s.afg.images.canva.forget_account()) {
        let cfg = state.site().afg;
        return render_room(
            &state,
            &chrome,
            &viewer,
            StatusCode::INTERNAL_SERVER_ERROR,
            &cfg,
            Said::error(chrome.t_args("afg-error-not-saved", &[("error", e.to_string().into())])),
        );
    }
    tracing::warn!(by = viewer.name(), "AFG: the Canva account let go");
    let cfg = state.site().afg;
    render_room(
        &state,
        &chrome,
        &viewer,
        StatusCode::OK,
        &cfg,
        Said {
            images: true,
            ..Said::notice(chrome.t("afg-canva-gone"))
        },
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
    // The asker's brain (docs/adr/0024): what it remembers goes with the
    // question. It is made once an exchange has an answer, not before: a test
    // that fails (a program not signed in, a key refused) leaves no empty
    // brain behind. A session with no account has none.
    let owner = viewer.user.as_ref().map(crate::brain::Owner::of);
    let recall = owner.as_ref().map(|o| {
        brains(&state)
            .existing(o)
            .map(|b| b.recall())
            .unwrap_or_default()
    });
    let person = owner
        .as_ref()
        .and_then(|o| o.person_id.clone())
        .and_then(|pid| {
            state.read(|flat| {
                flat.get("persons")
                    .and_then(|m| m.get(&pid))
                    .map(|p| (pid.clone(), crate::view::person_display_name(p)))
            })
        });
    let question = chrome.t("afg-test-question");
    let asker = match (&owner, &recall) {
        (Some(o), Some(r)) => Some(crate::afg::Asker {
            name: &o.name,
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
        owner.as_ref().map(|o| o.name.as_str()),
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
        brain = owner.is_some(),
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
                if let Some(o) = owner {
                    let all = brains(&runner);
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
                    let kept = tokio::task::spawn_blocking(move || {
                        all.of(&o).and_then(|b| b.remember(&exchange, &facts))
                    });
                    match kept.await {
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
