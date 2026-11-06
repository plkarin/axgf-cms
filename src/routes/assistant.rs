//! My personal assistant's pages (the operator's request of 3 November
//! 2026, docs/adr/0026): each account the AFG is open to has its own space,
//! its projects and their conversations, reached from its account page.
//!
//! Everything here is the signed-in account's own and nobody else's: the
//! space is always the session's, never one a form or an address names
//! (`crate::assistant`), and administrators see only theirs. A project,
//! a conversation or a file that is not the account's answers 404 as one
//! that does not exist does.
//!
//! An answer is written in the background ([`start_answer`]); the
//! conversation's page asks for what has been written so far
//! (`…/state`) and shows it as it comes. Without scripting, the page asks
//! again every three seconds until the answer is there.

use axum::extract::{Form, Multipart, Path, Query, State};
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::{IntoResponse, Redirect, Response};
use minijinja::context;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::assistant::{self, Conversation, FileKind, Message, Project, Role, Space, Spaces};
use crate::routes::Shared;
use crate::{auth, render};

/// Where the assistant lives.
pub const BASE: &str = "/account/assistant";

/// The language of a page, by its name in English, for the AFG.
pub(crate) fn language_name(code: &str) -> &'static str {
    match code {
        "fr" => "French",
        "de" => "German",
        "es" => "Spanish",
        "it" => "Italian",
        "pt" => "Portuguese",
        "pl" => "Polish",
        "ru" => "Russian",
        "he" => "Hebrew",
        "ar" => "Arabic",
        "ja" => "Japanese",
        "zh-Hans" => "Simplified Chinese",
        "sw" => "Swahili",
        "ln" => "Lingala",
        _ => "English",
    }
}

/// Whether a script asked, and wants JSON back.
pub(crate) fn wants_json(headers: &HeaderMap) -> bool {
    headers
        .get(header::ACCEPT)
        .and_then(|v| v.to_str().ok())
        .is_some_and(|a| a.contains("application/json"))
}

pub(crate) fn json_reply(status: StatusCode, v: Value) -> Response {
    (
        status,
        [
            (header::CONTENT_TYPE, "application/json"),
            (header::CACHE_CONTROL, "no-store"),
        ],
        v.to_string(),
    )
        .into_response()
}

/// The account signed in, its space opened.
struct Member {
    chrome: render::Chrome,
    user: crate::acl::User,
    space: Space,
}

/// Who may come in: a signed-in account the AFG is open to. Anyone else is
/// told why not, on the page itself. The refusal is the page, as `require`'s
/// is in the administration.
#[allow(clippy::result_large_err)]
fn door(state: &Shared, headers: &HeaderMap, path: &str) -> Result<Member, Response> {
    let viewer = auth::viewer(state, headers);
    if !viewer.signed_in() {
        return Err(Redirect::to("/admin/login").into_response());
    }
    let chrome = render::Chrome::resolve(&viewer, headers, path).with_archive(state);
    // The account as stored now: its access may have changed since it
    // signed in.
    let user = viewer
        .user
        .as_ref()
        .and_then(|u| state.acl_read(|acl| acl.by_id(&u.id).cloned()));
    let shut = |why: &str, status: StatusCode| {
        let mut r = render::page_with(
            &chrome,
            "assistant.html",
            context! { nav => "account", screen => "shut", why },
        );
        *r.status_mut() = status;
        r
    };
    let Some(user) = user else {
        return Err(shut("no-account", StatusCode::OK));
    };
    if !user.afg {
        return Err(shut("not-open", StatusCode::FORBIDDEN));
    }
    match Spaces::beside(state.bundle_path()).of(&crate::brain::Owner::of(&user)) {
        Ok(space) => Ok(Member {
            chrome,
            user,
            space,
        }),
        Err(e) => {
            tracing::error!(error = %e, "assistant: the space could not be opened");
            Err(shut("broken", StatusCode::INTERNAL_SERVER_ERROR))
        }
    }
}

/// A page that does not exist, or is not this account's.
fn not_found(m: &Member) -> Response {
    let mut r = render::page_with(
        &m.chrome,
        "assistant.html",
        context! { nav => "account", screen => "shut", why => "gone" },
    );
    *r.status_mut() = StatusCode::NOT_FOUND;
    r
}

// ---------------------------------------------------------------------------
// what the pages show
// ---------------------------------------------------------------------------

/// The picture of a project or a conversation: its own, a pack avatar
/// chosen, or the one its id stands for.
fn avatar_url(pid: &str, cid: Option<&str>, chosen: &str, id: &str) -> String {
    match chosen {
        "own" => match cid {
            None => format!("{BASE}/{pid}/avatar"),
            Some(cid) => format!("{BASE}/{pid}/c/{cid}/avatar"),
        },
        name => {
            let a = crate::avatar_pack::find(name)
                .unwrap_or_else(|| crate::avatar_pack::default_for(id));
            format!("/static/avatars/{}.webp", a.name)
        }
    }
}

fn project_json(p: &Project) -> Value {
    json!({
        "id": p.id,
        "name": p.name,
        "avatar": p.avatar,
        "avatar_url": avatar_url(&p.id, None, &p.avatar, &p.id),
        "conduct": p.conduct,
        "archive": p.archive,
        "created_at": p.created_at,
        "updated_at": p.updated_at,
    })
}

fn conversation_json(pid: &str, c: &Conversation) -> Value {
    json!({
        "id": c.id,
        "title": c.title,
        "avatar": c.avatar,
        "avatar_url": avatar_url(pid, Some(&c.id), &c.avatar, &c.id),
        "created_at": c.created_at,
        "updated_at": c.updated_at,
        "messages": c.messages,
    })
}

/// The pack, by domain, for the avatar pickers.
fn pack_json() -> Value {
    let domains: Vec<Value> = crate::avatar_pack::DOMAINS
        .iter()
        .map(|d| {
            let avatars: Vec<Value> = crate::avatar_pack::PACK
                .iter()
                .filter(|a| a.domain == *d)
                .map(|a| json!({"name": a.name, "url": format!("/static/avatars/{}.webp", a.name)}))
                .collect();
            json!({"domain": d, "avatars": avatars})
        })
        .collect();
    Value::Array(domains)
}

/// What is left of an answer being written once its memory block, which is
/// not shown, is cut off.
pub(crate) fn shown_partial(partial: &str) -> &str {
    match partial.to_ascii_lowercase().find("<memory>") {
        Some(i) => &partial[..i],
        None => partial,
    }
}

/// A file's size as people read it.
fn size_text(bytes: u64) -> String {
    match bytes {
        b if b < 1024 => format!("{b} B"),
        b if b < 1024 * 1024 => format!("{} KB", b.div_ceil(1024)),
        b => format!("{:.1} MB", b as f64 / (1024.0 * 1024.0)),
    }
}

/// One message as the thread draws it; `last` when nothing follows it, the
/// only place an answer that failed may be asked for again.
fn message_html(m: &Member, pid: &str, cid: &str, msg: &Message, last: bool) -> String {
    message_fragment(
        &m.chrome,
        &format!("{BASE}/{pid}/f"),
        pid,
        cid,
        None,
        msg,
        last,
    )
}

/// One message as a thread draws it, here or on a person's page
/// (docs/adr/0031): its files under `files_at`, and, on a person's page,
/// the way to ask again there.
pub(crate) fn message_fragment(
    chrome: &render::Chrome,
    files_at: &str,
    pid: &str,
    cid: &str,
    person: Option<&str>,
    msg: &Message,
    last: bool,
) -> String {
    let files: Vec<Value> = msg
        .files
        .iter()
        .map(|f| {
            json!({
                "name": f.name,
                "url": format!("{files_at}/{}", f.id),
                "image": f.kind == FileKind::Image,
                "kind": f.kind,
                "size_text": size_text(f.size),
            })
        })
        .collect();
    let html = match msg.role {
        Role::Assistant => crate::notes::to_html(&msg.text, &|_| None),
        _ => String::new(),
    };
    render::fragment_with(
        chrome,
        "_assistant_message.html",
        context! {
            m => json!({
                "id": msg.id,
                "role": msg.role,
                "text": msg.text,
                "html": html,
                "files": files,
                "at": msg.at,
                "model": msg.model,
                "seconds": msg.seconds,
                "remembered": msg.remembered,
            }),
            pid,
            cid,
            person,
            last,
        },
    )
    .unwrap_or_default()
}

/// The answer being written, as the thread draws it.
fn live_html(m: &Member, partial: &str, seconds: u64) -> String {
    live_fragment(&m.chrome, partial, seconds)
}

/// The answer being written, as a thread draws it, here or on a person's
/// page.
pub(crate) fn live_fragment(chrome: &render::Chrome, partial: &str, seconds: u64) -> String {
    let shown = shown_partial(partial);
    let html = if shown.trim().is_empty() {
        String::new()
    } else {
        crate::notes::to_html(shown, &|_| None)
    };
    render::fragment_with(chrome, "_assistant_live.html", context! { html, seconds })
        .unwrap_or_default()
}

/// Whether the AFG can answer: its computing set, and complete.
pub(crate) fn ready(state: &Shared) -> bool {
    let cfg = state.site().afg;
    cfg.kind != crate::afg::Kind::Off && cfg.problem().is_none()
}

#[derive(Default)]
struct Said {
    notice: Option<String>,
    error: Option<String>,
}

fn render_view(
    state: &Shared,
    m: &Member,
    view: &str,
    project: Option<&Project>,
    conversation: Option<&Conversation>,
    said: Said,
    status: StatusCode,
) -> Response {
    let projects: Vec<Value> = m
        .space
        .projects()
        .iter()
        .map(|s| {
            let mut v = project_json(&s.project);
            v["conversations"] = json!(s.conversations);
            v["facts"] = json!(s.facts);
            v
        })
        .collect();
    let (conversations, facts) = match project {
        Some(p) => (
            m.space
                .conversations(&p.id)
                .iter()
                .map(|c| conversation_json(&p.id, c))
                .collect::<Vec<_>>(),
            m.space.facts(&p.id),
        ),
        None => (Vec::new(), Vec::new()),
    };
    let mut orphan = false;
    let (thread, live) = match (project, conversation) {
        (Some(p), Some(c)) => {
            let messages = m.space.messages(&p.id, &c.id);
            let thread: Vec<String> = messages
                .iter()
                .enumerate()
                .map(|(i, msg)| message_html(m, &p.id, &c.id, msg, i + 1 == messages.len()))
                .collect();
            let live = state
                .assistant_answering()
                .get(&m.user.id, &p.id, &c.id)
                .map(|l| {
                    let partial = l.partial.lock().map(|s| s.clone()).unwrap_or_default();
                    live_html(m, &partial, l.since.elapsed().as_secs())
                });
            // A message nobody is answering: the process stopped while the
            // AFG wrote. It may be asked again.
            orphan = live.is_none() && messages.last().is_some_and(|msg| msg.role == Role::User);
            (thread, live)
        }
        _ => (Vec::new(), None),
    };
    let archive = project.filter(|p| p.archive).map(|_| {
        let cfg = state.site().afg;
        state
            .read(|flat| crate::afg::archive(flat, cfg.sensitive))
            .ok()
            .map(|a| json!({"count": a.count, "tokens": a.tokens}))
    });
    let mut r = render::page_with(
        &m.chrome,
        "assistant.html",
        context! {
            nav => "account",
            // Not `view`, which the chrome holds: the landing page's drawing.
            screen => view,
            projects,
            project => project.map(project_json),
            conversations,
            facts,
            conversation => project.zip(conversation).map(|(p, c)| conversation_json(&p.id, c)),
            thread,
            live,
            orphan,
            archive,
            ready => ready(state),
            pack => pack_json(),
            limits => json!({
                "files": assistant::FILES_PER_MESSAGE,
                "file_mb": assistant::FILE_MAX / (1024 * 1024),
                "message_mb": assistant::MESSAGE_FILES_MAX / (1024 * 1024),
                "text": assistant::TEXT_MAX,
                "conduct": assistant::CONDUCT_MAX,
                "name": assistant::NAME_MAX,
            }),
            notice => said.notice,
            error => said.error,
        },
    );
    *r.status_mut() = status;
    r
}

#[derive(Deserialize, Default)]
pub struct DoneQuery {
    #[serde(default)]
    done: Option<String>,
}

fn said_of(chrome: &render::Chrome, done: Option<&str>) -> Said {
    let key = match done {
        Some("saved") => "assistant-saved",
        Some("deleted") => "assistant-project-deleted",
        Some("conversation-deleted") => "assistant-conversation-deleted",
        Some("forgotten") => "assistant-memory-forgotten",
        Some("forgotten-all") => "assistant-memory-forgotten-all",
        Some("avatar") => "assistant-avatar-saved",
        _ => return Said::default(),
    };
    Said {
        notice: Some(chrome.t(key)),
        error: None,
    }
}

// ---------------------------------------------------------------------------
// the pages
// ---------------------------------------------------------------------------

/// `GET /account/assistant` — the projects.
pub async fn home(
    State(state): State<Shared>,
    headers: HeaderMap,
    Query(q): Query<DoneQuery>,
) -> Response {
    let m = match door(&state, &headers, BASE) {
        Ok(m) => m,
        Err(r) => return r,
    };
    let said = said_of(&m.chrome, q.done.as_deref());
    render_view(&state, &m, "home", None, None, said, StatusCode::OK)
}

/// `GET /account/assistant/:project` — a project: its conversations, its
/// code of conduct, its memory.
pub async fn project(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(pid): Path<String>,
    Query(q): Query<DoneQuery>,
) -> Response {
    let m = match door(&state, &headers, &format!("{BASE}/{pid}")) {
        Ok(m) => m,
        Err(r) => return r,
    };
    let Some(p) = m.space.project(&pid) else {
        return not_found(&m);
    };
    let said = said_of(&m.chrome, q.done.as_deref());
    render_view(&state, &m, "project", Some(&p), None, said, StatusCode::OK)
}

/// `GET /account/assistant/:project/c/:conversation` — a conversation.
pub async fn conversation(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path((pid, cid)): Path<(String, String)>,
    Query(q): Query<DoneQuery>,
) -> Response {
    let m = match door(&state, &headers, &format!("{BASE}/{pid}/c/{cid}")) {
        Ok(m) => m,
        Err(r) => return r,
    };
    let (Some(p), Some(c)) = (m.space.project(&pid), m.space.conversation(&pid, &cid)) else {
        return not_found(&m);
    };
    let said = said_of(&m.chrome, q.done.as_deref());
    render_view(
        &state,
        &m,
        "conversation",
        Some(&p),
        Some(&c),
        said,
        StatusCode::OK,
    )
}

#[derive(Deserialize, Default)]
pub struct StateQuery {
    /// How many messages the page already shows.
    #[serde(default)]
    after: usize,
}

/// `GET /account/assistant/:project/c/:conversation/state?after=N` — what
/// the page does not show yet: the messages after its `N` first, and the
/// answer being written.
pub async fn state_of(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path((pid, cid)): Path<(String, String)>,
    Query(q): Query<StateQuery>,
) -> Response {
    let m = match door(&state, &headers, &format!("{BASE}/{pid}/c/{cid}")) {
        Ok(m) => m,
        Err(_) => return json_reply(StatusCode::FORBIDDEN, json!({"error": "forbidden"})),
    };
    if m.space.conversation(&pid, &cid).is_none() {
        return json_reply(StatusCode::NOT_FOUND, json!({"error": "gone"}));
    }
    let messages = m.space.messages(&pid, &cid);
    let new: Vec<Value> = messages
        .iter()
        .enumerate()
        .skip(q.after)
        .map(|(i, msg)| {
            let last = i + 1 == messages.len();
            json!({"id": msg.id, "role": msg.role, "html": message_html(&m, &pid, &cid, msg, last)})
        })
        .collect();
    let live = state.assistant_answering().get(&m.user.id, &pid, &cid);
    let (answering, live_html) = match &live {
        Some(l) => {
            let partial = l.partial.lock().map(|s| s.clone()).unwrap_or_default();
            (true, live_html(&m, &partial, l.since.elapsed().as_secs()))
        }
        None => (false, String::new()),
    };
    json_reply(
        StatusCode::OK,
        json!({
            "count": messages.len(),
            "messages": new,
            "answering": answering,
            "live": live_html,
        }),
    )
}

// ---------------------------------------------------------------------------
// projects
// ---------------------------------------------------------------------------

#[derive(Deserialize, Default)]
pub struct ProjectForm {
    #[serde(default)]
    name: String,
    #[serde(default)]
    avatar: String,
    #[serde(default)]
    conduct: Option<String>,
    #[serde(default)]
    archive: Option<String>,
    /// Which part of the project the form saves: `settings` (its name, its
    /// avatar and whether the archive goes with it) or `conduct`.
    #[serde(default)]
    part: String,
}

/// A pack avatar's name, or nothing: anything else is not kept.
fn pack_choice(s: &str) -> String {
    match crate::avatar_pack::find(s.trim()) {
        Some(a) => a.name.to_string(),
        None => String::new(),
    }
}

/// `POST /account/assistant` — a new project.
pub async fn create_project(
    State(state): State<Shared>,
    headers: HeaderMap,
    Form(f): Form<ProjectForm>,
) -> Response {
    let m = match door(&state, &headers, BASE) {
        Ok(m) => m,
        Err(r) => return r,
    };
    let name = assistant::clean_name(&f.name, assistant::NAME_MAX);
    if name.is_empty() {
        let said = Said {
            notice: None,
            error: Some(m.chrome.t("assistant-error-name")),
        };
        return render_view(
            &state,
            &m,
            "home",
            None,
            None,
            said,
            StatusCode::UNPROCESSABLE_ENTITY,
        );
    }
    match m.space.create_project(&name, &pack_choice(&f.avatar)) {
        Ok(Some(p)) => Redirect::to(&format!("{BASE}/{}", p.id)).into_response(),
        Ok(None) => {
            let said = Said {
                notice: None,
                error: Some(m.chrome.t("assistant-error-too-many")),
            };
            render_view(
                &state,
                &m,
                "home",
                None,
                None,
                said,
                StatusCode::UNPROCESSABLE_ENTITY,
            )
        }
        Err(e) => {
            tracing::error!(error = %e, "assistant: a project could not be made");
            let said = Said {
                notice: None,
                error: Some(m.chrome.t("assistant-error-not-saved")),
            };
            render_view(
                &state,
                &m,
                "home",
                None,
                None,
                said,
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        }
    }
}

/// `POST /account/assistant/:project` — its settings, or its code of
/// conduct.
pub async fn save_project(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(pid): Path<String>,
    Form(f): Form<ProjectForm>,
) -> Response {
    let m = match door(&state, &headers, &format!("{BASE}/{pid}")) {
        Ok(m) => m,
        Err(r) => return r,
    };
    let Some(mut p) = m.space.project(&pid) else {
        return not_found(&m);
    };
    if f.part == "conduct" {
        let conduct: String = f.conduct.unwrap_or_default().trim().to_string();
        if conduct.chars().count() > assistant::CONDUCT_MAX {
            let said = Said {
                notice: None,
                error: Some(m.chrome.t("assistant-error-conduct-long")),
            };
            return render_view(
                &state,
                &m,
                "project",
                Some(&p),
                None,
                said,
                StatusCode::UNPROCESSABLE_ENTITY,
            );
        }
        p.conduct = conduct;
    } else {
        let name = assistant::clean_name(&f.name, assistant::NAME_MAX);
        if name.is_empty() {
            let said = Said {
                notice: None,
                error: Some(m.chrome.t("assistant-error-name")),
            };
            return render_view(
                &state,
                &m,
                "project",
                Some(&p),
                None,
                said,
                StatusCode::UNPROCESSABLE_ENTITY,
            );
        }
        p.name = name;
        p.archive = f.archive.is_some();
        // `own` stays when the picture of its own is kept; a pack avatar
        // replaces it.
        let chosen = pack_choice(&f.avatar);
        if !chosen.is_empty() || f.avatar.trim().is_empty() {
            p.avatar = chosen;
        }
    }
    match m.space.save_project(&p) {
        Ok(()) => Redirect::to(&format!("{BASE}/{pid}?done=saved")).into_response(),
        Err(e) => {
            tracing::error!(error = %e, "assistant: a project could not be saved");
            let said = Said {
                notice: None,
                error: Some(m.chrome.t("assistant-error-not-saved")),
            };
            render_view(
                &state,
                &m,
                "project",
                Some(&p),
                None,
                said,
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        }
    }
}

#[derive(Deserialize, Default)]
pub struct ConfirmForm {
    #[serde(default)]
    confirm: Option<String>,
}

/// `POST /account/assistant/:project/delete` — a project and all of it,
/// once its deletion is confirmed.
pub async fn delete_project(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(pid): Path<String>,
    Form(f): Form<ConfirmForm>,
) -> Response {
    let m = match door(&state, &headers, &format!("{BASE}/{pid}")) {
        Ok(m) => m,
        Err(r) => return r,
    };
    let Some(p) = m.space.project(&pid) else {
        return not_found(&m);
    };
    if f.confirm.is_none() {
        let said = Said {
            notice: None,
            error: Some(m.chrome.t("assistant-error-confirm")),
        };
        return render_view(
            &state,
            &m,
            "project",
            Some(&p),
            None,
            said,
            StatusCode::UNPROCESSABLE_ENTITY,
        );
    }
    match m.space.delete_project(&pid) {
        Ok(_) => {
            tracing::info!(account = %m.user.username, "assistant: a project deleted");
            Redirect::to(&format!("{BASE}?done=deleted")).into_response()
        }
        Err(e) => {
            tracing::error!(error = %e, "assistant: a project could not be deleted");
            let said = Said {
                notice: None,
                error: Some(m.chrome.t("assistant-error-not-saved")),
            };
            render_view(
                &state,
                &m,
                "project",
                Some(&p),
                None,
                said,
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        }
    }
}

#[derive(Deserialize, Default)]
pub struct ForgetForm {
    #[serde(default)]
    fact: Option<String>,
    #[serde(default)]
    all: Option<String>,
    #[serde(default)]
    confirm: Option<String>,
}

/// `POST /account/assistant/:project/memory/forget` — one fact of the
/// project's second brain, by its mark, or all of it once confirmed.
pub async fn forget(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(pid): Path<String>,
    Form(f): Form<ForgetForm>,
) -> Response {
    let m = match door(&state, &headers, &format!("{BASE}/{pid}")) {
        Ok(m) => m,
        Err(r) => return r,
    };
    let Some(p) = m.space.project(&pid) else {
        return not_found(&m);
    };
    let outcome = match (f.fact.as_deref(), f.all.is_some(), f.confirm.is_some()) {
        (Some(id), _, _) => m.space.forget_fact(&pid, id).map(|_| "forgotten"),
        (None, true, true) => m.space.forget_memory(&pid).map(|_| "forgotten-all"),
        _ => {
            let said = Said {
                notice: None,
                error: Some(m.chrome.t("assistant-error-confirm")),
            };
            return render_view(
                &state,
                &m,
                "project",
                Some(&p),
                None,
                said,
                StatusCode::UNPROCESSABLE_ENTITY,
            );
        }
    };
    match outcome {
        Ok(done) => Redirect::to(&format!("{BASE}/{pid}?done={done}#memory")).into_response(),
        Err(e) => {
            tracing::error!(error = %e, "assistant: the memory could not be written");
            let said = Said {
                notice: None,
                error: Some(m.chrome.t("assistant-error-not-saved")),
            };
            render_view(
                &state,
                &m,
                "project",
                Some(&p),
                None,
                said,
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        }
    }
}

// ---------------------------------------------------------------------------
// messages
// ---------------------------------------------------------------------------

/// What a message's form sent: its words and its files.
struct Sent {
    text: String,
    files: Vec<(String, Vec<u8>)>,
}

/// Why a message was refused, as a catalogue key.
async fn read_sent(mut multipart: Multipart) -> Result<Sent, &'static str> {
    let mut sent = Sent {
        text: String::new(),
        files: Vec::new(),
    };
    let mut total = 0usize;
    loop {
        let field = match multipart.next_field().await {
            Ok(Some(f)) => f,
            Ok(None) => break,
            Err(_) => return Err("assistant-error-too-big"),
        };
        match field.name().unwrap_or_default() {
            "text" => {
                sent.text = field.text().await.map_err(|_| "assistant-error-too-big")?;
            }
            "files" => {
                let name = field.file_name().unwrap_or("file").to_string();
                let bytes = field.bytes().await.map_err(|_| "assistant-error-too-big")?;
                if bytes.is_empty() {
                    continue;
                }
                if bytes.len() > assistant::FILE_MAX {
                    return Err("assistant-error-file-big");
                }
                total += bytes.len();
                if total > assistant::MESSAGE_FILES_MAX {
                    return Err("assistant-error-too-big");
                }
                sent.files.push((name, bytes.to_vec()));
                if sent.files.len() > assistant::FILES_PER_MESSAGE {
                    return Err("assistant-error-too-many-files");
                }
            }
            _ => {}
        }
    }
    sent.text = sent.text.trim().to_string();
    if sent.text.chars().count() > assistant::TEXT_MAX {
        return Err("assistant-error-text-long");
    }
    if sent.text.is_empty() && sent.files.is_empty() {
        return Err("assistant-error-empty");
    }
    Ok(sent)
}

/// Keep a message's files in the project, then the message in the
/// conversation.
fn keep_message(space: &Space, pid: &str, cid: &str, sent: Sent) -> std::io::Result<Message> {
    let mut refs = Vec::new();
    for (name, bytes) in &sent.files {
        refs.push(space.save_file(pid, name, bytes)?);
    }
    let msg = Message::new(Role::User, sent.text, refs);
    space.append(pid, cid, &msg)?;
    Ok(msg)
}

/// Start writing a conversation's next answer, in the background. `false`
/// when it is already being answered, or the site answers as many as it may.
fn start_answer(state: &Shared, m: &Member, project: &Project, cid: &str) -> bool {
    let Some(live) = state
        .assistant_answering()
        .begin(&m.user.id, &project.id, cid)
    else {
        return false;
    };
    let cfg = state.site().afg;
    let messages = m.space.messages(&project.id, cid);
    let archive = if project.archive {
        state
            .read(|flat| crate::afg::archive(flat, cfg.sensitive))
            .ok()
    } else {
        None
    };
    let person = m.user.person_id.as_ref().and_then(|id| {
        state.read(|flat| {
            flat.get("persons")
                .and_then(|ps| ps.get(id))
                .map(|p| (id.clone(), crate::view::person_display_name(p)))
        })
    });
    let asker = assistant::Asker {
        name: &m.user.username,
        person: person.as_ref().map(|(id, n)| (id.as_str(), n.as_str())),
        language: language_name(m.chrome.lang),
    };
    let prompt = assistant::chat_prompt(&m.space, &asker, project, &messages, archive.as_ref());
    tracing::info!(
        account = %m.user.username,
        kind = cfg.kind.slug(),
        turns = prompt.turns.len(),
        archive = archive.is_some(),
        "assistant: a message sent to the AFG"
    );
    run_answer(
        state,
        m.space.clone(),
        m.user.id.clone(),
        project.id.clone(),
        cid.to_string(),
        prompt,
        None,
        live,
        true,
    );
    true
}

/// Work done on a prompt before it is sent, off the page's way, and what to
/// send instead when a service refuses what the work added: a person's
/// photographs read and sized, and the question without them, for a model
/// that takes no picture (docs/adr/0033).
pub(crate) struct Prepare {
    pub work: Box<dyn FnOnce(&mut crate::afg_chat::ChatPrompt) + Send>,
    pub instead: crate::afg_chat::ChatPrompt,
}

/// Write a conversation's next answer in the background, into `live` as it
/// comes, then into the conversation: here, and on a person's page
/// (docs/adr/0031). `remember`: whether a `<memory>` block goes into the
/// project's second brain; on a person's page, nothing is asked to be kept.
#[allow(clippy::too_many_arguments)]
pub(crate) fn run_answer(
    state: &Shared,
    space: Space,
    account: String,
    pid: String,
    cid: String,
    prompt: crate::afg_chat::ChatPrompt,
    prepare: Option<Prepare>,
    live: std::sync::Arc<assistant::Live>,
    remember: bool,
) {
    let cfg = state.site().afg;
    let runner = state.clone();
    let base = state.data_dir().to_path_buf();
    let kind = cfg.kind.slug();
    let model = cfg.model.trim().to_string();
    tokio::spawn(async move {
        let outcome = if cfg.kind == crate::afg::Kind::Off || cfg.problem().is_some() {
            Err(crate::afg::Failure::new(
                "the AFG's computing is not set up",
            ))
        } else {
            let (prompt, instead) = match prepare {
                Some(Prepare { work, instead }) => {
                    let prepared = tokio::task::spawn_blocking(move || {
                        let mut prompt = prompt;
                        work(&mut prompt);
                        prompt
                    })
                    .await;
                    // Work that failed sends the question as it would be
                    // sent without it.
                    match prepared {
                        Ok(prompt) => (prompt, Some(instead)),
                        Err(_) => (instead, None),
                    }
                }
                None => (prompt, None),
            };
            let first =
                crate::afg_chat::chat(cfg.clone(), prompt, base.clone(), live.partial.clone())
                    .await;
            match (first, instead) {
                // A service that refuses the request (HTTP 4xx), as one whose
                // model takes no picture does, is asked once more without
                // what the work added.
                (Err(f), Some(instead)) if f.message.starts_with("HTTP 4") => {
                    tracing::warn!(error = %f.message, "AFG: refused with the files, asked again without them");
                    live.partial
                        .lock()
                        .unwrap_or_else(|e| e.into_inner())
                        .clear();
                    crate::afg_chat::chat(cfg, instead, base, live.partial.clone()).await
                }
                (outcome, _) => outcome,
            }
        };
        let seconds = live.since.elapsed().as_secs();
        let msg = match outcome {
            Ok(answer) => {
                let (shown, facts) = crate::brain::take_memory(&answer.text);
                let kept = if remember {
                    let space2 = space.clone();
                    let pid2 = pid.clone();
                    tokio::task::spawn_blocking(move || space2.remember(&pid2, &facts))
                        .await
                        .ok()
                        .and_then(Result::ok)
                        .unwrap_or_default()
                } else {
                    Vec::new()
                };
                let mut msg = Message::new(Role::Assistant, shown, Vec::new());
                msg.kind = kind.to_string();
                msg.model = answer.model.unwrap_or(model);
                msg.remembered = kept;
                msg.input_tokens = answer.input_tokens;
                msg.output_tokens = answer.output_tokens;
                msg.cost_usd = answer.cost_usd;
                msg.seconds = Some(seconds);
                msg
            }
            Err(f) => {
                tracing::warn!(error = %f.message, "assistant: the AFG did not answer");
                let mut text = f.message;
                if let Some(d) = f.detail {
                    text.push_str("\n\n");
                    text.push_str(&d);
                }
                let mut msg = Message::new(Role::Error, text, Vec::new());
                msg.kind = kind.to_string();
                msg.seconds = Some(seconds);
                msg
            }
        };
        let space3 = space.clone();
        let (p3, c3) = (pid.clone(), cid.clone());
        let written = tokio::task::spawn_blocking(move || space3.append(&p3, &c3, &msg)).await;
        if !matches!(written, Ok(Ok(()))) {
            tracing::error!("assistant: an answer could not be kept");
        }
        runner.assistant_answering().end(&account, &pid, &cid);
    });
}

/// A message refused: said to the script, or on the page.
fn refused(
    state: &Shared,
    m: &Member,
    headers: &HeaderMap,
    key: &str,
    status: StatusCode,
    project: &Project,
    conversation: Option<&Conversation>,
) -> Response {
    let text = m.chrome.t(key);
    if wants_json(headers) {
        return json_reply(status, json!({"error": text}));
    }
    let view = if conversation.is_some() {
        "conversation"
    } else {
        "project"
    };
    let said = Said {
        notice: None,
        error: Some(text),
    };
    render_view(state, m, view, Some(project), conversation, said, status)
}

/// `POST /account/assistant/:project/c` — a new conversation, on its first
/// message.
pub async fn start_conversation(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(pid): Path<String>,
    multipart: Multipart,
) -> Response {
    let m = match door(&state, &headers, &format!("{BASE}/{pid}")) {
        Ok(m) => m,
        Err(r) => return r,
    };
    let Some(p) = m.space.project(&pid) else {
        return not_found(&m);
    };
    let sent = match read_sent(multipart).await {
        Ok(s) => s,
        Err(key) => {
            return refused(
                &state,
                &m,
                &headers,
                key,
                StatusCode::UNPROCESSABLE_ENTITY,
                &p,
                None,
            )
        }
    };
    if state.assistant_answering().full() {
        return refused(
            &state,
            &m,
            &headers,
            "assistant-error-busy",
            StatusCode::SERVICE_UNAVAILABLE,
            &p,
            None,
        );
    }
    let first = if sent.text.is_empty() {
        sent.files
            .first()
            .map(|(n, _)| n.clone())
            .unwrap_or_default()
    } else {
        sent.text.clone()
    };
    let c = match m.space.create_conversation(&pid, &first, "") {
        Ok(Some(c)) => c,
        Ok(None) => {
            return refused(
                &state,
                &m,
                &headers,
                "assistant-error-too-many",
                StatusCode::UNPROCESSABLE_ENTITY,
                &p,
                None,
            )
        }
        Err(e) => {
            tracing::error!(error = %e, "assistant: a conversation could not be made");
            return refused(
                &state,
                &m,
                &headers,
                "assistant-error-not-saved",
                StatusCode::INTERNAL_SERVER_ERROR,
                &p,
                None,
            );
        }
    };
    if let Err(e) = keep_message(&m.space, &pid, &c.id, sent) {
        tracing::error!(error = %e, "assistant: a message could not be kept");
        return refused(
            &state,
            &m,
            &headers,
            "assistant-error-not-saved",
            StatusCode::INTERNAL_SERVER_ERROR,
            &p,
            Some(&c),
        );
    }
    start_answer(&state, &m, &p, &c.id);
    let url = format!("{BASE}/{pid}/c/{}", c.id);
    if wants_json(&headers) {
        return json_reply(StatusCode::OK, json!({"redirect": url}));
    }
    Redirect::to(&url).into_response()
}

/// `POST /account/assistant/:project/c/:conversation` — the next message.
pub async fn send(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path((pid, cid)): Path<(String, String)>,
    multipart: Multipart,
) -> Response {
    let m = match door(&state, &headers, &format!("{BASE}/{pid}/c/{cid}")) {
        Ok(m) => m,
        Err(r) => return r,
    };
    let (Some(p), Some(c)) = (m.space.project(&pid), m.space.conversation(&pid, &cid)) else {
        return not_found(&m);
    };
    let sent = match read_sent(multipart).await {
        Ok(s) => s,
        Err(key) => {
            return refused(
                &state,
                &m,
                &headers,
                key,
                StatusCode::UNPROCESSABLE_ENTITY,
                &p,
                Some(&c),
            )
        }
    };
    if state
        .assistant_answering()
        .get(&m.user.id, &pid, &cid)
        .is_some()
    {
        return refused(
            &state,
            &m,
            &headers,
            "assistant-error-wait",
            StatusCode::CONFLICT,
            &p,
            Some(&c),
        );
    }
    if state.assistant_answering().full() {
        return refused(
            &state,
            &m,
            &headers,
            "assistant-error-busy",
            StatusCode::SERVICE_UNAVAILABLE,
            &p,
            Some(&c),
        );
    }
    if c.messages >= assistant::MESSAGES_MAX {
        return refused(
            &state,
            &m,
            &headers,
            "assistant-error-too-many",
            StatusCode::UNPROCESSABLE_ENTITY,
            &p,
            Some(&c),
        );
    }
    let msg = match keep_message(&m.space, &pid, &cid, sent) {
        Ok(msg) => msg,
        Err(e) => {
            tracing::error!(error = %e, "assistant: a message could not be kept");
            return refused(
                &state,
                &m,
                &headers,
                "assistant-error-not-saved",
                StatusCode::INTERNAL_SERVER_ERROR,
                &p,
                Some(&c),
            );
        }
    };
    let answering = start_answer(&state, &m, &p, &cid);
    if wants_json(&headers) {
        return json_reply(
            StatusCode::OK,
            json!({
                "message": {"id": msg.id, "html": message_html(&m, &pid, &cid, &msg, true)},
                "count": m.space.messages(&pid, &cid).len(),
                "answering": answering,
            }),
        );
    }
    Redirect::to(&format!("{BASE}/{pid}/c/{cid}")).into_response()
}

/// `POST /account/assistant/:project/c/:conversation/retry` — ask again,
/// after an answer that failed.
pub async fn retry(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path((pid, cid)): Path<(String, String)>,
) -> Response {
    let m = match door(&state, &headers, &format!("{BASE}/{pid}/c/{cid}")) {
        Ok(m) => m,
        Err(r) => return r,
    };
    let (Some(p), Some(_)) = (m.space.project(&pid), m.space.conversation(&pid, &cid)) else {
        return not_found(&m);
    };
    // Only a message left without an answer, or whose answer failed, is
    // asked again.
    let waiting = m
        .space
        .messages(&pid, &cid)
        .last()
        .is_some_and(|msg| msg.role != Role::Assistant);
    let answering = waiting && start_answer(&state, &m, &p, &cid);
    if wants_json(&headers) {
        return json_reply(StatusCode::OK, json!({"answering": answering}));
    }
    Redirect::to(&format!("{BASE}/{pid}/c/{cid}")).into_response()
}

#[derive(Deserialize, Default)]
pub struct ConversationForm {
    #[serde(default)]
    title: String,
    #[serde(default)]
    avatar: String,
}

/// `POST /account/assistant/:project/c/:conversation/edit` — its title and
/// its avatar.
pub async fn edit_conversation(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path((pid, cid)): Path<(String, String)>,
    Form(f): Form<ConversationForm>,
) -> Response {
    let m = match door(&state, &headers, &format!("{BASE}/{pid}/c/{cid}")) {
        Ok(m) => m,
        Err(r) => return r,
    };
    let (Some(p), Some(mut c)) = (m.space.project(&pid), m.space.conversation(&pid, &cid)) else {
        return not_found(&m);
    };
    let title = assistant::clean_name(&f.title, assistant::TITLE_MAX);
    if title.is_empty() {
        let said = Said {
            notice: None,
            error: Some(m.chrome.t("assistant-error-name")),
        };
        return render_view(
            &state,
            &m,
            "conversation",
            Some(&p),
            Some(&c),
            said,
            StatusCode::UNPROCESSABLE_ENTITY,
        );
    }
    c.title = title;
    let chosen = pack_choice(&f.avatar);
    if !chosen.is_empty() || f.avatar.trim().is_empty() {
        c.avatar = chosen;
    }
    match m.space.save_conversation(&pid, &c) {
        Ok(()) => Redirect::to(&format!("{BASE}/{pid}/c/{cid}?done=saved")).into_response(),
        Err(e) => {
            tracing::error!(error = %e, "assistant: a conversation could not be saved");
            let said = Said {
                notice: None,
                error: Some(m.chrome.t("assistant-error-not-saved")),
            };
            render_view(
                &state,
                &m,
                "conversation",
                Some(&p),
                Some(&c),
                said,
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        }
    }
}

/// `POST /account/assistant/:project/c/:conversation/delete` — a
/// conversation, once confirmed.
pub async fn delete_conversation(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path((pid, cid)): Path<(String, String)>,
    Form(f): Form<ConfirmForm>,
) -> Response {
    let m = match door(&state, &headers, &format!("{BASE}/{pid}/c/{cid}")) {
        Ok(m) => m,
        Err(r) => return r,
    };
    let (Some(p), Some(c)) = (m.space.project(&pid), m.space.conversation(&pid, &cid)) else {
        return not_found(&m);
    };
    if f.confirm.is_none() {
        let said = Said {
            notice: None,
            error: Some(m.chrome.t("assistant-error-confirm")),
        };
        return render_view(
            &state,
            &m,
            "conversation",
            Some(&p),
            Some(&c),
            said,
            StatusCode::UNPROCESSABLE_ENTITY,
        );
    }
    if state
        .assistant_answering()
        .get(&m.user.id, &pid, &cid)
        .is_some()
    {
        let said = Said {
            notice: None,
            error: Some(m.chrome.t("assistant-error-wait")),
        };
        return render_view(
            &state,
            &m,
            "conversation",
            Some(&p),
            Some(&c),
            said,
            StatusCode::CONFLICT,
        );
    }
    match m.space.delete_conversation(&pid, &cid) {
        Ok(_) => Redirect::to(&format!("{BASE}/{pid}?done=conversation-deleted")).into_response(),
        Err(e) => {
            tracing::error!(error = %e, "assistant: a conversation could not be deleted");
            let said = Said {
                notice: None,
                error: Some(m.chrome.t("assistant-error-not-saved")),
            };
            render_view(
                &state,
                &m,
                "conversation",
                Some(&p),
                Some(&c),
                said,
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        }
    }
}

// ---------------------------------------------------------------------------
// files and pictures
// ---------------------------------------------------------------------------

/// `GET /account/assistant/:project/f/:file` — a file sent with a message,
/// to the account that sent it. Pictures are shown, everything else is
/// downloaded.
pub async fn file(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path((pid, fid)): Path<(String, String)>,
) -> Response {
    let m = match door(&state, &headers, &format!("{BASE}/{pid}")) {
        Ok(m) => m,
        Err(r) => return r,
    };
    let Some((f, bytes)) = m.space.file(&pid, &fid) else {
        return not_found(&m);
    };
    let disposition = if f.kind == FileKind::Image {
        "inline".to_string()
    } else {
        // The name as an RFC 5987 parameter, so that any letter survives.
        let enc: String = f
            .name
            .bytes()
            .map(|b| match b {
                b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'.' | b'-' | b'_' => {
                    (b as char).to_string()
                }
                _ => format!("%{b:02X}"),
            })
            .collect();
        format!("attachment; filename*=UTF-8''{enc}")
    };
    (
        [
            (header::CONTENT_TYPE, f.mime.clone()),
            (header::CONTENT_DISPOSITION, disposition),
            (header::CACHE_CONTROL, "private, max-age=86400".to_string()),
            (
                header::HeaderName::from_static("x-content-type-options"),
                "nosniff".to_string(),
            ),
            (
                header::CONTENT_SECURITY_POLICY,
                "default-src 'none'; img-src 'self'; style-src 'unsafe-inline'; sandbox"
                    .to_string(),
            ),
        ],
        bytes,
    )
        .into_response()
}

fn own_picture(bytes: Vec<u8>) -> Response {
    (
        [
            (header::CONTENT_TYPE, "image/jpeg"),
            (header::CACHE_CONTROL, "private, no-cache"),
        ],
        bytes,
    )
        .into_response()
}

/// `GET /account/assistant/:project/avatar` — a project's picture of its own.
pub async fn project_avatar(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(pid): Path<String>,
) -> Response {
    let m = match door(&state, &headers, &format!("{BASE}/{pid}")) {
        Ok(m) => m,
        Err(r) => return r,
    };
    match m.space.own_avatar(&pid, None) {
        Some(b) => own_picture(b),
        None => not_found(&m),
    }
}

/// `GET /account/assistant/:project/c/:conversation/avatar`.
pub async fn conversation_avatar(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path((pid, cid)): Path<(String, String)>,
) -> Response {
    let m = match door(&state, &headers, &format!("{BASE}/{pid}/c/{cid}")) {
        Ok(m) => m,
        Err(r) => return r,
    };
    match m.space.own_avatar(&pid, Some(&cid)) {
        Some(b) => own_picture(b),
        None => not_found(&m),
    }
}

/// Read the one picture an avatar form sent.
async fn read_picture(mut multipart: Multipart) -> Option<Vec<u8>> {
    while let Ok(Some(field)) = multipart.next_field().await {
        if field.name() == Some("picture") {
            let bytes = field.bytes().await.ok()?;
            if bytes.is_empty() || bytes.len() > assistant::AVATAR_MAX {
                return None;
            }
            return Some(bytes.to_vec());
        }
    }
    None
}

/// `POST /account/assistant/:project/avatar` — a picture of the account's
/// own as the project's avatar.
pub async fn upload_project_avatar(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(pid): Path<String>,
    multipart: Multipart,
) -> Response {
    let m = match door(&state, &headers, &format!("{BASE}/{pid}")) {
        Ok(m) => m,
        Err(r) => return r,
    };
    let Some(mut p) = m.space.project(&pid) else {
        return not_found(&m);
    };
    let saved = match read_picture(multipart).await {
        Some(bytes) => m.space.save_own_avatar(&pid, None, &bytes).unwrap_or(false),
        None => false,
    };
    if !saved {
        let said = Said {
            notice: None,
            error: Some(m.chrome.t("assistant-error-picture")),
        };
        return render_view(
            &state,
            &m,
            "project",
            Some(&p),
            None,
            said,
            StatusCode::UNPROCESSABLE_ENTITY,
        );
    }
    p.avatar = "own".into();
    let _ = m.space.save_project(&p);
    Redirect::to(&format!("{BASE}/{pid}?done=avatar")).into_response()
}

/// `POST /account/assistant/:project/c/:conversation/avatar`.
pub async fn upload_conversation_avatar(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path((pid, cid)): Path<(String, String)>,
    multipart: Multipart,
) -> Response {
    let m = match door(&state, &headers, &format!("{BASE}/{pid}/c/{cid}")) {
        Ok(m) => m,
        Err(r) => return r,
    };
    let (Some(p), Some(mut c)) = (m.space.project(&pid), m.space.conversation(&pid, &cid)) else {
        return not_found(&m);
    };
    let saved = match read_picture(multipart).await {
        Some(bytes) => m
            .space
            .save_own_avatar(&pid, Some(&cid), &bytes)
            .unwrap_or(false),
        None => false,
    };
    if !saved {
        let said = Said {
            notice: None,
            error: Some(m.chrome.t("assistant-error-picture")),
        };
        return render_view(
            &state,
            &m,
            "conversation",
            Some(&p),
            Some(&c),
            said,
            StatusCode::UNPROCESSABLE_ENTITY,
        );
    }
    c.avatar = "own".into();
    let _ = m.space.save_conversation(&pid, &c);
    Redirect::to(&format!("{BASE}/{pid}/c/{cid}?done=avatar")).into_response()
}

/// `GET /static/avatars/:file` — a picture of the pack.
pub async fn pack_avatar(Path(file): Path<String>) -> Response {
    let Some(a) = file
        .strip_suffix(".webp")
        .and_then(crate::avatar_pack::find)
    else {
        return StatusCode::NOT_FOUND.into_response();
    };
    (
        [
            (header::CONTENT_TYPE, "image/webp"),
            (header::CACHE_CONTROL, "public, max-age=604800"),
        ],
        a.bytes,
    )
        .into_response()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn what_is_being_written_is_shown_without_its_memory() {
        assert_eq!(shown_partial("Un.\n\n<memory>\n- x"), "Un.\n\n");
        assert_eq!(shown_partial("Deux <MEMORY>"), "Deux ");
        assert_eq!(shown_partial("Trois"), "Trois");
    }

    #[test]
    fn a_picture_is_the_projects_own_a_chosen_one_or_its_ids() {
        assert_eq!(
            avatar_url("p", None, "own", "p"),
            "/account/assistant/p/avatar"
        );
        assert_eq!(
            avatar_url("p", Some("c"), "own", "c"),
            "/account/assistant/p/c/c/avatar"
        );
        assert_eq!(
            avatar_url("p", None, "genealogy-tree", "p"),
            "/static/avatars/genealogy-tree.webp"
        );
        let auto = avatar_url("p", None, "", "0123456789abcdef");
        assert_eq!(auto, avatar_url("q", None, "nonsense", "0123456789abcdef"));
        assert!(auto.starts_with("/static/avatars/"));
    }
}
