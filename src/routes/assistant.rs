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
//! (`…/state`) and shows it as it comes, and the member may stop it
//! (`…/stop`, docs/adr/0050). Without scripting, the page asks again every
//! three seconds until the answer is there.

use axum::extract::{Form, Multipart, Path, Query, State};
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::{IntoResponse, Redirect, Response};
use minijinja::context;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::assistant::{
    self, Commands, Conversation, FileKind, Message, OpsDone, Project, Role, Space, Spaces,
};
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

/// A project's knowledge as its page shows it (docs/adr/0050): each
/// document with the word for what the AFG makes of it, what they weigh
/// together, whether all of it goes with each message, and what a project
/// may hold.
fn knowledge_json(state: &Shared, m: &Member, pid: &str) -> Value {
    let files = m.space.knowledge(pid);
    let lang = m.chrome.lang;
    let rows: Vec<Value> = files
        .iter()
        .map(|k| {
            // What the AFG reads it as, or that it is kept without being
            // read: a sound, a film, a PDF document a message cannot take.
            let kind = match k.file.kind {
                _ if !crate::knowledge::is_read(k) => "other",
                FileKind::Text => "text",
                FileKind::Pdf => "pdf",
                FileKind::Image => "image",
                FileKind::Audio | FileKind::Video | FileKind::Other => "other",
            };
            json!({
                "id": k.file.id,
                "name": k.file.name,
                "kind": kind,
                "size": crate::documents::human_size_in(k.file.size, lang),
                "pages": k.pages,
                "added": k.added_at,
                "url": format!("{BASE}/{pid}/knowledge/{}", k.file.id),
            })
        })
        .collect();
    let total: u64 = files.iter().map(|k| k.file.size).sum();
    json!({
        "files": rows,
        "size": crate::documents::human_size_in(total, lang),
        "whole": crate::knowledge::all_whole(&files),
        "limits": {
            "file_mb": state.limits().assistant_file_mb,
            "file_bytes": state.limits().assistant_file_bytes(),
            "files": assistant::KNOWLEDGE_FILES,
            "total_mb": assistant::KNOWLEDGE_BYTES / (1024 * 1024),
        },
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
    let partial = match partial.to_ascii_lowercase().find("<memory>") {
        Some(i) => &partial[..i],
        None => partial,
    };
    // Operations being written are drawn once they are all there, as a card.
    match crate::ops::blocks(partial).first() {
        Some(crate::ops::Segment::Ops(_)) => "",
        Some(crate::ops::Segment::Text(t)) => &partial[..t.len()],
        None => partial,
    }
}

/// Whether the answer being written has begun proposing operations.
fn preparing(partial: &str) -> bool {
    !crate::ops::ops_blocks(partial).is_empty()
}

/// A file's size as people read it.
fn size_text(bytes: u64) -> String {
    match bytes {
        b if b < 1024 => format!("{b} B"),
        b if b < 1024 * 1024 => format!("{} KB", b.div_ceil(1024)),
        b => format!("{:.1} MB", b as f64 / (1024.0 * 1024.0)),
    }
}

/// What a thread needs to draw the operations the AFG proposed
/// (docs/adr/0045 §2): who reads them, where they are applied, and what
/// became of each.
pub(crate) struct OpsView<'a> {
    pub state: &'a Shared,
    pub viewer: crate::access::Viewer,
    /// Where the card's form posts: `…/c/<cid>/ops`.
    pub action: String,
    /// What became of each block: (answer, block) → what the member did.
    pub done: std::collections::HashMap<(String, usize), OpsDone>,
}

impl<'a> OpsView<'a> {
    fn of(state: &'a Shared, m: &Member, pid: &str, cid: &str, messages: &[Message]) -> Self {
        OpsView {
            state,
            viewer: crate::access::Viewer {
                user: Some(m.user.clone()),
                emergency: false,
            },
            action: format!("{BASE}/{pid}/c/{cid}/ops"),
            done: messages
                .iter()
                .filter_map(|x| x.ops.clone())
                .map(|o| ((o.message.clone(), o.block), o))
                .collect(),
        }
    }
}

/// One message as the thread draws it; `last` when nothing follows it, the
/// only place an answer that failed may be asked for again.
fn message_html(
    m: &Member,
    pid: &str,
    cid: &str,
    msg: &Message,
    last: bool,
    ops: &OpsView<'_>,
) -> String {
    message_fragment(
        &m.chrome,
        &format!("{BASE}/{pid}/f"),
        pid,
        cid,
        None,
        msg,
        last,
        Some(ops),
    )
}

/// What a card says of a block, checked now against the archive, kept while
/// the archive does not change: a thread is drawn again at every message.
type CardSaid = (u64, Result<Vec<String>, String>);

fn card_cache() -> &'static std::sync::Mutex<std::collections::HashMap<String, CardSaid>> {
    static CACHE: std::sync::OnceLock<
        std::sync::Mutex<std::collections::HashMap<String, CardSaid>>,
    > = std::sync::OnceLock::new();
    CACHE.get_or_init(Default::default)
}

/// A block of operations as a card: what they would do, checked now against
/// the archive as the reader would write them, and the buttons to apply them
/// or set them aside; or what became of them.
fn card_html(
    chrome: &render::Chrome,
    ops: &OpsView<'_>,
    mid: &str,
    block: usize,
    json: &str,
) -> String {
    let done = ops.done.get(&(mid.to_string(), block));
    let checked = || -> Result<Vec<String>, String> {
        let batch = crate::ops::parse(json)
            .map_err(|e| chrome.t_args("afg-ops-unreadable", &[("detail", e.into())]))?;
        let key = format!("{}/{mid}/{block}/{}", ops.viewer.name(), chrome.lang);
        let generation = ops.state.generation();
        if let Some((g, said)) = card_cache().lock().ok().and_then(|c| c.get(&key).cloned()) {
            if g == generation {
                return said;
            }
        }
        let lang = chrome.lang;
        let quote = |n: &str| crate::ops::quoted(lang, n);
        let said = crate::ops::check(ops.state, &ops.viewer, &batch, &quote)
            .map(|lines| lines.iter().map(|l| l.text(lang)).collect())
            .map_err(|r| r.text(lang));
        if let Ok(mut c) = card_cache().lock() {
            if c.len() > 512 {
                c.clear();
            }
            c.insert(key, (generation, said.clone()));
        }
        said
    };
    let (status, lines, refusal) = match done {
        Some(d) if d.applied => ("applied", Vec::new(), None),
        Some(_) => ("aside", Vec::new(), None),
        None => match checked() {
            Ok(lines) => ("open", lines, None),
            Err(why) => ("refused", Vec::new(), Some(why)),
        },
    };
    let pretty = serde_json::from_str::<Value>(json)
        .ok()
        .and_then(|v| serde_json::to_string_pretty(&v).ok())
        .unwrap_or_else(|| json.trim().to_string());
    render::fragment_with(
        chrome,
        "_afg_ops.html",
        context! {
            card => json!({
                "status": status,
                "lines": lines,
                "refusal": refusal,
                "records": done.map(|d| d.records).unwrap_or(0),
                "action": ops.action,
                "message": mid,
                "block": block,
                "json": pretty,
                "writes": ops.viewer.may_write(),
            }),
        },
    )
    .unwrap_or_default()
}

/// An answer drawn from its Markdown, each `axgf-ops` block in it a card
/// when the thread can apply it, and its JSON otherwise.
fn answer_html(
    chrome: &render::Chrome,
    mid: &str,
    text: &str,
    ops: Option<&OpsView<'_>>,
) -> String {
    let mut out = String::new();
    let mut block = 0;
    for segment in crate::ops::blocks(text) {
        match segment {
            crate::ops::Segment::Text(t) => out.push_str(&crate::notes::to_html(t, &|_| None)),
            crate::ops::Segment::Ops(json) => {
                block += 1;
                match ops {
                    Some(v) => out.push_str(&card_html(chrome, v, mid, block, json)),
                    None => out.push_str(&crate::notes::to_html(
                        &format!("```json\n{}\n```", json.trim()),
                        &|_| None,
                    )),
                }
            }
        }
    }
    out
}

/// One message as a thread draws it, here or on a person's page
/// (docs/adr/0031): its files under `files_at`, and, on a person's page,
/// the way to ask again there.
#[allow(clippy::too_many_arguments)]
pub(crate) fn message_fragment(
    chrome: &render::Chrome,
    files_at: &str,
    pid: &str,
    cid: &str,
    person: Option<&str>,
    msg: &Message,
    last: bool,
    ops: Option<&OpsView<'_>>,
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
        Role::Assistant => answer_html(chrome, &msg.id, &msg.text, ops),
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
                "stopped": msg.stopped,
                "tools": msg.tools,
            }),
            pid,
            cid,
            person,
            last,
        },
    )
    .unwrap_or_default()
}

/// The answer being written in a conversation, as the thread draws it.
fn live_html(m: &Member, pid: &str, cid: &str, live: &assistant::Live) -> String {
    let partial = live.partial.lock().map(|s| s.clone()).unwrap_or_default();
    let tools = live.tools.lock().map(|t| t.clone()).unwrap_or_default();
    live_fragment(
        &m.chrome,
        &partial,
        live.since.elapsed().as_secs(),
        &tools,
        Some(&format!("{BASE}/{pid}/c/{cid}/stop")),
    )
}

/// A sentence with a name in it, cut around the name, so that the page sets
/// the name apart (`<code>`) wherever the language puts it: `before`,
/// `name`, `after`.
fn around(chrome: &render::Chrome, key: &str, arg: &str, name: &str) -> Value {
    const MARK: &str = "\u{e000}";
    let said = chrome.t_args(key, &[(arg, MARK.into())]);
    let (before, after) = said.split_once(MARK).unwrap_or((said.as_str(), ""));
    json!({"before": before, "name": name, "after": after})
}

/// The answer being written, as a thread draws it, here or on a person's
/// page: what it has written, the tool it called last (docs/adr/0050), and,
/// for a page read without scripting, where to stop it.
pub(crate) fn live_fragment(
    chrome: &render::Chrome,
    partial: &str,
    seconds: u64,
    tools: &[String],
    stop: Option<&str>,
) -> String {
    let shown = shown_partial(partial);
    let html = if shown.trim().is_empty() {
        String::new()
    } else {
        crate::notes::to_html(shown, &|_| None)
    };
    let preparing = preparing(partial);
    let tool = tools
        .last()
        .map(|t| around(chrome, "assistant-tool-running", "tool", t));
    render::fragment_with(
        chrome,
        "_assistant_live.html",
        context! { html, seconds, preparing, tool, stop },
    )
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
            let ops = OpsView::of(state, m, &p.id, &c.id, &messages);
            let thread: Vec<String> = messages
                .iter()
                .enumerate()
                .map(|(i, msg)| message_html(m, &p.id, &c.id, msg, i + 1 == messages.len(), &ops))
                .collect();
            let live = state
                .assistant_answering()
                .get(&m.user.id, &p.id, &c.id)
                .map(|l| live_html(m, &p.id, &c.id, &l));
            // A message nobody is answering: the process stopped while the
            // AFG wrote. It may be asked again.
            orphan = live.is_none() && messages.last().is_some_and(|msg| msg.role == Role::User);
            (thread, live)
        }
        _ => (Vec::new(), None),
    };
    // What the archive weighs, or that it cannot go: the account does not
    // see all of it.
    let archive = project
        .filter(|p| p.archive)
        .map(|_| match archive_for(state, &m.user) {
            Some(a) => json!({"count": a.count, "tokens": a.tokens}),
            None => json!({"refused": true}),
        });
    // The project's knowledge, on the project's own page (docs/adr/0050).
    // The project a person's page keeps its conversations in has none.
    let knowledge = project
        .filter(|p| view == "project" && p.person.is_none())
        .map(|p| knowledge_json(state, m, &p.id));
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
            knowledge,
            // The answer being written, without a word yet: what the script
            // shows the moment a message goes (docs/adr/0050).
            live_template => conversation.map(|_| live_fragment(&m.chrome, "", 0, &[], None)),
            conversation => project.zip(conversation).map(|(p, c)| conversation_json(&p.id, c)),
            thread,
            live,
            orphan,
            archive,
            ready => ready(state),
            pack => pack_json(),
            limits => json!({
                "files": assistant::FILES_PER_MESSAGE,
                "file_mb": state.limits().assistant_file_mb,
                "message_mb": state.limits().assistant_message_mb,
                "text": assistant::TEXT_MAX,
                "conduct": assistant::CONDUCT_MAX,
                "name": assistant::NAME_MAX,
            }),
            notice => said.notice,
            error => said.error,
            guide => (view == "home").then(|| crate::guide::guide(m.chrome.lang)),
            writes => m.user.role.may_write(),
        },
    );
    *r.status_mut() = status;
    dictation(r)
}

/// A page whose composer may take dictation (docs/adr/0050): the browser's
/// speech recognition needs the microphone, which every other page of the
/// site refuses (`routes::PERMISSIONS`). The camera stays refused: a photo is
/// taken through the system's own camera, not in the page.
pub(crate) fn dictation(mut r: Response) -> Response {
    r.headers_mut().insert(
        header::HeaderName::from_static("permissions-policy"),
        header::HeaderValue::from_static(crate::routes::PERMISSIONS_DICTATION),
    );
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
        Some("knowledge-added") => "assistant-knowledge-added",
        Some("knowledge-removed") => "assistant-knowledge-removed",
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
    let ops = OpsView::of(&state, &m, &pid, &cid, &messages);
    let new: Vec<Value> = messages
        .iter()
        .enumerate()
        .skip(q.after)
        .map(|(i, msg)| {
            let last = i + 1 == messages.len();
            json!({"id": msg.id, "role": msg.role, "html": message_html(&m, &pid, &cid, msg, last, &ops)})
        })
        .collect();
    let live = state.assistant_answering().get(&m.user.id, &pid, &cid);
    let (answering, live_html) = match &live {
        Some(l) => (true, live_html(&m, &pid, &cid, l)),
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
    // An answer still being written in it is stopped first: its agent ends,
    // and nothing is left writing into a project that is gone.
    state.assistant_answering().stop_project(&m.user.id, &pid);
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
async fn read_sent(
    mut multipart: Multipart,
    limits: &crate::limits::Limits,
) -> Result<Sent, &'static str> {
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
                if bytes.len() > limits.assistant_file_bytes() {
                    return Err("assistant-error-file-big");
                }
                total += bytes.len();
                if total > limits.assistant_message_bytes() {
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

/// The family's archive as a project of this account may send it to the
/// AFG (docs/adr/0026), or `None` when this account may not read all of
/// it. The archive is the whole archive: sent for an account whose lens
/// leaves someone out, it would hand the model, and through its answers
/// the account, the private and living people the account may not see
/// (the security review of 27 November 2026). So it goes only for an
/// account that sees every person and every link at its own ceiling, to
/// whom the records are open; the sensitive classes, when the AFG's room
/// sends them, only for an administrator's.
fn archive_for(state: &Shared, user: &crate::acl::User) -> Option<crate::afg::Archive> {
    let role = user.role;
    let records = state.acl_read(|acl| acl.site.doors.level(crate::site::Part::Records));
    if !records.admits(true, role.may_write(), role.is_admin()) {
        return None;
    }
    let ceiling = role.ceiling();
    let sensitive = state.site().afg.sensitive && role.is_admin();
    state.read_as(ceiling, |flat, lens| {
        let links_seen = flat
            .get("links")
            .and_then(Value::as_object)
            .is_none_or(|ls| {
                ls.values()
                    .all(|l| crate::access::link_visibility(l) <= ceiling)
            });
        (lens.is_all() && links_seen)
            .then(|| crate::afg::archive(flat, sensitive).ok())
            .flatten()
    })
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
        archive_for(state, &m.user)
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
    let commands = if m.user.role.may_write() {
        Commands::Write
    } else {
        Commands::Read
    };
    // The MCP servers of the account Claude Code runs as come with an
    // administrator's question, or with everyone's when the room says so
    // (docs/adr/0050).
    let mcp = cfg.mcp_for(m.user.role.is_admin());
    let prompt = assistant::chat_prompt(
        &m.space,
        &asker,
        project,
        &messages,
        archive.as_ref(),
        commands,
        mcp,
    );
    tracing::info!(
        account = %m.user.username,
        kind = cfg.kind.slug(),
        turns = prompt.turns.len(),
        archive = archive.is_some(),
        mcp,
        "assistant: a message sent to the AFG"
    );
    // The project's knowledge goes with every question (docs/adr/0050). It
    // is read, cut and weighed on the way, not while the page waits, against
    // the member's last words: this message and the one before it, so that
    // a question that only says "and her sister?" still finds its passages.
    // The project a person's page keeps its conversations in has none.
    let prepare = project.person.is_none().then(|| {
        let space = m.space.clone();
        let pid = project.id.clone();
        let query = messages
            .iter()
            .rev()
            .filter(|msg| msg.role == Role::User)
            .take(2)
            .map(|msg| msg.text.as_str())
            .collect::<Vec<_>>()
            .join("\n");
        Prepare {
            work: Box::new(move |prompt: &mut crate::afg_chat::ChatPrompt| {
                crate::knowledge::join(&space, &pid, &query, prompt)
            }),
            instead: None,
        }
    });
    run_answer(
        state,
        m.space.clone(),
        m.user.id.clone(),
        project.id.clone(),
        cid.to_string(),
        prompt,
        prepare,
        live,
        true,
        mcp,
    );
    true
}

/// What a `Prepare` does to a question, and the question it may give back to
/// ask instead.
pub(crate) type Work =
    Box<dyn FnOnce(&mut crate::afg_chat::ChatPrompt) -> Option<crate::afg_chat::ChatPrompt> + Send>;

/// Work done on a prompt before it is sent, off the page's way, and what to
/// send instead when a service refuses what the work added: a person's
/// photographs read and sized, and the question without them, for a model
/// that takes no picture (docs/adr/0033); a project's knowledge read and
/// joined, and the question with its texts but without its PDF documents
/// and pictures (docs/adr/0050).
pub(crate) struct Prepare {
    /// Reads and joins. It gives back what to ask instead when only the work
    /// can make it: the knowledge's texts are read there, and stay in it.
    pub work: Work,
    /// What to ask instead, when it could be made before the work, without
    /// reading anything. Also what is asked when the work breaks.
    pub instead: Option<crate::afg_chat::ChatPrompt>,
}

/// Write a conversation's next answer in the background, into `live` as it
/// comes, then into the conversation: here, and on a person's page
/// (docs/adr/0031). `remember`: whether a `<memory>` block goes into the
/// project's second brain; on a person's page, nothing is asked to be kept.
/// `mcp`: whether Claude Code brings the MCP servers of its account
/// (docs/adr/0050). Stopped by the member (`live.cancel`), what was written
/// is kept as a stopped answer: one message, as for any other end.
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
    mcp: bool,
) {
    let cfg = state.site().afg;
    let runner = state.clone();
    let base = state.data_dir().to_path_buf();
    let kind = cfg.kind.slug();
    let model = cfg.model.trim().to_string();
    let run = crate::afg_chat::Run {
        mcp,
        cancel: live.cancel.clone(),
        tools: live.tools.clone(),
    };
    tokio::spawn(async move {
        let outcome = if cfg.kind == crate::afg::Kind::Off || cfg.problem().is_some() {
            Err(crate::afg::Failure::new(
                "the AFG's computing is not set up",
            ))
        } else {
            let prepared = match prepare {
                Some(Prepare { work, instead }) => {
                    tokio::task::spawn_blocking(move || {
                        let mut prompt = prompt;
                        // Work that breaks does not take the question with
                        // it: the question is still there to be asked.
                        let made = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                            work(&mut prompt)
                        }));
                        (prompt, made)
                    })
                    .await
                    .map(|(prompt, made)| match made {
                        Ok(made) => (prompt, made.or(instead)),
                        // Work that failed sends the question as it would be
                        // sent without it, or as far as the work had got.
                        Err(_) => {
                            tracing::error!("AFG: the work on a question broke");
                            (instead.unwrap_or(prompt), None)
                        }
                    })
                    .map_err(|_| crate::afg::Failure::new("the question could not be prepared"))
                }
                None => Ok((prompt, None)),
            };
            match prepared {
                Ok((prompt, instead)) => {
                    let first = crate::afg_chat::chat(
                        cfg.clone(),
                        prompt,
                        base.clone(),
                        live.partial.clone(),
                        run.clone(),
                    )
                    .await;
                    match (first, instead) {
                        // A service that refuses the request (HTTP 4xx), as
                        // one whose model takes no picture does, is asked
                        // once more without what the work added; not once
                        // the member stopped it.
                        (Err(f), Some(instead))
                            if f.message.starts_with("HTTP 4") && !live.cancel.is_cancelled() =>
                        {
                            tracing::warn!(error = %f.message, "AFG: refused with the files, asked again without them");
                            live.partial
                                .lock()
                                .unwrap_or_else(|e| e.into_inner())
                                .clear();
                            crate::afg_chat::chat(cfg, instead, base, live.partial.clone(), run)
                                .await
                        }
                        (outcome, _) => outcome,
                    }
                }
                Err(f) => Err(f),
            }
        };
        let seconds = live.since.elapsed().as_secs();
        let tools =
            assistant::tools_used(&live.tools.lock().map(|t| t.clone()).unwrap_or_default());
        // A stop that came once the answer was whole changes nothing: only a
        // run the stop cut short is kept as stopped.
        let msg = if live.cancel.is_cancelled() && outcome.is_err() {
            // Stopped (docs/adr/0050): what the page showed is kept, as it
            // showed it (no memory block, no half-written operations), and
            // nothing goes into the project's memory.
            let partial = live.partial.lock().map(|s| s.clone()).unwrap_or_default();
            let mut msg = Message::new(
                Role::Assistant,
                shown_partial(&partial).trim_end().to_string(),
                Vec::new(),
            );
            msg.stopped = true;
            msg.kind = kind.to_string();
            msg.model = model;
            msg.seconds = Some(seconds);
            msg.tools = tools;
            tracing::info!(seconds, "assistant: an answer stopped by its member");
            msg
        } else {
            match outcome {
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
                    msg.tools = tools;
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
    let sent = match read_sent(multipart, &state.limits()).await {
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
    let sent = match read_sent(multipart, &state.limits()).await {
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
                "message": {"id": msg.id, "html": message_html(&m, &pid, &cid, &msg, true, &OpsView::of(&state, &m, &pid, &cid, &[]))},
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
    // Only a message left without an answer, whose answer failed, or whose
    // answer the member stopped, is asked again.
    let waiting = m
        .space
        .messages(&pid, &cid)
        .last()
        .is_some_and(|msg| matches!(msg.role, Role::User | Role::Error) || msg.stopped);
    let answering = waiting && start_answer(&state, &m, &p, &cid);
    if wants_json(&headers) {
        return json_reply(StatusCode::OK, json!({"answering": answering}));
    }
    Redirect::to(&format!("{BASE}/{pid}/c/{cid}")).into_response()
}

/// `POST /account/assistant/:project/c/:conversation/stop` — stop the
/// answer being written (docs/adr/0050). What it wrote so far is kept,
/// marked as stopped, and the conversation takes the next message. Nothing
/// being written is no error: pressed twice, or once the answer came, it
/// changes nothing. `{"stopping": bool}` to a script.
pub async fn stop(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path((pid, cid)): Path<(String, String)>,
) -> Response {
    let m = match door(&state, &headers, &format!("{BASE}/{pid}/c/{cid}")) {
        Ok(m) => m,
        Err(r) => return r,
    };
    if m.space.project(&pid).is_none() || m.space.conversation(&pid, &cid).is_none() {
        return not_found(&m);
    }
    let stopping = state.assistant_answering().stop(&m.user.id, &pid, &cid);
    if stopping {
        tracing::info!(account = %m.user.username, "assistant: an answer asked to stop");
    }
    if wants_json(&headers) {
        return json_reply(StatusCode::OK, json!({"stopping": stopping}));
    }
    Redirect::to(&format!("{BASE}/{pid}/c/{cid}")).into_response()
}

#[derive(Deserialize)]
pub struct OpsForm {
    /// The answer that proposed the operations, and which of its blocks.
    msg: String,
    block: usize,
    /// `apply`, or anything else to set them aside.
    #[serde(default, rename = "do")]
    action: String,
}

/// `POST /account/assistant/:project/c/:conversation/ops` — apply the
/// operations an answer of the AFG proposed, as the account and through the
/// same rules as its own pages, or set them aside (docs/adr/0045 §2). What
/// became of them is said in the thread, for the member and for the AFG.
pub async fn apply_ops(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path((pid, cid)): Path<(String, String)>,
    Form(f): Form<OpsForm>,
) -> Response {
    let back = format!("{BASE}/{pid}/c/{cid}");
    let m = match door(&state, &headers, &back) {
        Ok(m) => m,
        Err(r) => return r,
    };
    let (Some(p), Some(c)) = (m.space.project(&pid), m.space.conversation(&pid, &cid)) else {
        return not_found(&m);
    };
    let messages = m.space.messages(&pid, &cid);
    let Some(json) = messages
        .iter()
        .find(|x| x.id == f.msg && x.role == Role::Assistant)
        .and_then(|x| {
            crate::ops::ops_blocks(&x.text)
                .get(f.block.wrapping_sub(1))
                .map(|j| j.to_string())
        })
    else {
        return not_found(&m);
    };
    // Applied or set aside already: once is all.
    if messages.iter().any(|x| {
        x.ops
            .as_ref()
            .is_some_and(|o| o.message == f.msg && o.block == f.block)
    }) {
        return Redirect::to(&back).into_response();
    }
    let lang = m.chrome.lang.to_string();
    let mut done = OpsDone {
        message: f.msg.clone(),
        block: f.block,
        applied: false,
        records: 0,
        refs: Default::default(),
    };
    let text = if f.action == "apply" {
        let batch = match crate::ops::parse(&json) {
            Ok(b) => b,
            Err(e) => {
                let why = m
                    .chrome
                    .t_args("afg-ops-unreadable", &[("detail", e.into())]);
                return refused_ops(&state, &m, &p, &c, why);
            }
        };
        let viewer = crate::access::Viewer {
            user: Some(m.user.clone()),
            emergency: false,
        };
        let (st, l) = (state.clone(), lang.clone());
        let outcome = tokio::task::spawn_blocking(move || {
            let quote = |n: &str| crate::ops::quoted(&l, n);
            crate::ops::apply(&st, &viewer, "AFG", &batch, &quote)
        })
        .await;
        let applied = match outcome {
            Ok(Ok(d)) => d,
            Ok(Err(r)) => return refused_ops(&state, &m, &p, &c, r.text(&lang)),
            Err(_) => {
                return refused_ops(&state, &m, &p, &c, m.chrome.t("assistant-error-not-saved"))
            }
        };
        done.applied = true;
        done.records = applied.records;
        let mut text = m
            .chrome
            .t_args("afg-ops-applied-text", &[("n", applied.records.into())]);
        for l in &applied.lines {
            text.push_str("\n– ");
            text.push_str(&l.text(&lang));
        }
        done.refs = applied.refs;
        text
    } else {
        m.chrome.t("afg-ops-aside-text")
    };
    let mut msg = Message::new(Role::Done, text, Vec::new());
    msg.ops = Some(done);
    if let Err(e) = m.space.append(&pid, &cid, &msg) {
        tracing::error!(error = %e, "assistant: what became of the operations could not be kept");
    }
    Redirect::to(&format!("{back}#m-{}", msg.id)).into_response()
}

/// Operations that could not be applied: the conversation, saying why.
fn refused_ops(state: &Shared, m: &Member, p: &Project, c: &Conversation, why: String) -> Response {
    render_view(
        state,
        m,
        "conversation",
        Some(p),
        Some(c),
        Said {
            notice: None,
            error: Some(why),
        },
        StatusCode::CONFLICT,
    )
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
/// to the account that sent it.
pub async fn file(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path((pid, fid)): Path<(String, String)>,
) -> Response {
    let m = match door(&state, &headers, &format!("{BASE}/{pid}")) {
        Ok(m) => m,
        Err(r) => return r,
    };
    match m.space.file(&pid, &fid) {
        Some((f, bytes)) => served(&f, bytes),
        None => not_found(&m),
    }
}

/// A file an account gave, to that account: one sent with a message, or a
/// document of a project's knowledge. Pictures are shown, everything else is
/// downloaded. Its own Content-Security-Policy, stricter than the site's on
/// every directive, is the one the browser gets (`routes::security_headers`
/// keeps it): a file someone sent runs nothing, loads nothing but itself, and
/// is never framed.
fn served(f: &assistant::FileRef, bytes: Vec<u8>) -> Response {
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
            // An account's own file: not kept by the browser past the page,
            // so that nobody opens it again there after a sign-out.
            (header::CACHE_CONTROL, "private, no-store".to_string()),
            (
                header::HeaderName::from_static("x-content-type-options"),
                "nosniff".to_string(),
            ),
            (header::CONTENT_SECURITY_POLICY, FILE_POLICY.to_string()),
        ],
        bytes,
    )
        .into_response()
}

/// The Content-Security-Policy a sent file is served with (docs/adr/0026
/// §5): nothing runs and nothing loads but the picture itself, no form goes
/// anywhere, and no page frames it. Each directive is the site's own or
/// stricter, so the site's layer leaves it in place.
const FILE_POLICY: &str = "default-src 'none'; img-src 'self'; style-src 'unsafe-inline'; \
                           frame-ancestors 'none'; form-action 'none'; base-uri 'none'; sandbox";

// ---------------------------------------------------------------------------
// the project's knowledge (docs/adr/0050)
// ---------------------------------------------------------------------------

/// The project a knowledge route is about: one of the account's own. The
/// project a person's page keeps its conversations in has no knowledge.
fn knowledge_project(m: &Member, pid: &str) -> Option<Project> {
    m.space.project(pid).filter(|p| p.person.is_none())
}

/// The project's page again, saying why its knowledge was left as it was.
fn knowledge_refused(
    state: &Shared,
    m: &Member,
    project: &Project,
    key: &str,
    status: StatusCode,
) -> Response {
    let said = Said {
        notice: None,
        error: Some(m.chrome.t(key)),
    };
    render_view(state, m, "project", Some(project), None, said, status)
}

/// What a knowledge form sent: its files, in the order they were chosen, or
/// why it was refused, as a catalogue key. Read piece by piece, so that no
/// more is ever held than may be kept: a file up to `file_max` bytes, and
/// together what the project has room for (`files_room`, `bytes_room`).
/// Past a limit what was held is let go and the rest of the form is read
/// and dropped: a browser shows the answer once its upload has been taken
/// to the end, and the request itself is weighed before it comes here.
async fn read_knowledge(
    mut multipart: Multipart,
    file_max: usize,
    files_room: usize,
    bytes_room: u64,
) -> Result<Vec<(String, Vec<u8>)>, &'static str> {
    let mut files: Vec<(String, Vec<u8>)> = Vec::new();
    let mut total = 0u64;
    let mut refused: Option<&'static str> = None;
    loop {
        let mut field = match multipart.next_field().await {
            Ok(Some(f)) => f,
            Ok(None) => break,
            // The form weighed more than a request may.
            Err(_) => return Err(refused.unwrap_or("assistant-error-file-big")),
        };
        if field.name() != Some("files") {
            continue;
        }
        let name = field.file_name().unwrap_or("file").to_string();
        let mut bytes: Vec<u8> = Vec::new();
        loop {
            let chunk = match field.chunk().await {
                Ok(Some(chunk)) => chunk,
                Ok(None) => break,
                Err(_) => return Err(refused.unwrap_or("assistant-error-file-big")),
            };
            if refused.is_some() || chunk.is_empty() {
                continue;
            }
            total += chunk.len() as u64;
            if files.len() >= files_room || total > bytes_room {
                refused = Some("assistant-knowledge-full");
            } else if bytes.len() + chunk.len() > file_max {
                refused = Some("assistant-error-file-big");
            } else {
                bytes.extend_from_slice(&chunk);
            }
            if refused.is_some() {
                files = Vec::new();
                bytes = Vec::new();
            }
        }
        // A file input left empty sends a part that holds nothing.
        if refused.is_none() && !bytes.is_empty() {
            files.push((name, bytes));
        }
    }
    match refused {
        Some(key) => Err(key),
        None if files.is_empty() => Err("assistant-knowledge-error-none"),
        None => Ok(files),
    }
}

/// `POST /account/assistant/:project/knowledge` — documents added to the
/// project's knowledge (docs/adr/0050): everything that was chosen, or
/// nothing.
pub async fn add_knowledge(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(pid): Path<String>,
    multipart: Multipart,
) -> Response {
    let m = match door(&state, &headers, &format!("{BASE}/{pid}")) {
        Ok(m) => m,
        Err(r) => return r,
    };
    let Some(p) = knowledge_project(&m, &pid) else {
        return not_found(&m);
    };
    // The room the project has left bounds what is read of the form. What
    // decides is counted again where the files are written, under the lock.
    let held = m.space.knowledge(&pid);
    let weight: u64 = held.iter().map(|k| k.file.size).sum();
    let sent = read_knowledge(
        multipart,
        state.limits().assistant_file_bytes(),
        assistant::KNOWLEDGE_FILES.saturating_sub(held.len()),
        assistant::KNOWLEDGE_BYTES.saturating_sub(weight),
    )
    .await;
    let files = match sent {
        Ok(files) => files,
        Err(key) => {
            return knowledge_refused(&state, &m, &p, key, StatusCode::UNPROCESSABLE_ENTITY)
        }
    };
    // Sniffing, counting a PDF document's pages and writing are a disk's and
    // a processor's work: off the requests' way.
    let space = m.space.clone();
    let pid2 = pid.clone();
    let kept = tokio::task::spawn_blocking(move || space.add_knowledge(&pid2, &files)).await;
    match kept {
        Ok(Ok(Some(_))) => {
            Redirect::to(&format!("{BASE}/{pid}?done=knowledge-added#knowledge")).into_response()
        }
        Ok(Ok(None)) => knowledge_refused(
            &state,
            &m,
            &p,
            "assistant-knowledge-full",
            StatusCode::UNPROCESSABLE_ENTITY,
        ),
        Ok(Err(e)) => {
            tracing::error!(error = %e, "assistant: the knowledge could not be written");
            knowledge_refused(
                &state,
                &m,
                &p,
                "assistant-error-not-saved",
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        }
        Err(e) => {
            tracing::error!(error = %e, "assistant: the knowledge could not be written");
            knowledge_refused(
                &state,
                &m,
                &p,
                "assistant-error-not-saved",
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        }
    }
}

/// `GET /account/assistant/:project/knowledge/:file` — a document of the
/// project's knowledge, to the account that gave it: served as a file sent
/// with a message is.
pub async fn knowledge_file(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path((pid, fid)): Path<(String, String)>,
) -> Response {
    let m = match door(&state, &headers, &format!("{BASE}/{pid}")) {
        Ok(m) => m,
        Err(r) => return r,
    };
    if knowledge_project(&m, &pid).is_none() {
        return not_found(&m);
    }
    match m.space.knowledge_file(&pid, &fid) {
        Some((k, bytes)) => served(&k.file, bytes),
        None => not_found(&m),
    }
}

/// `POST /account/assistant/:project/knowledge/:file/delete` — a document
/// taken out of the project's knowledge.
pub async fn remove_knowledge(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path((pid, fid)): Path<(String, String)>,
) -> Response {
    let m = match door(&state, &headers, &format!("{BASE}/{pid}")) {
        Ok(m) => m,
        Err(r) => return r,
    };
    let Some(p) = knowledge_project(&m, &pid) else {
        return not_found(&m);
    };
    match m.space.remove_knowledge(&pid, &fid) {
        Ok(true) => {
            Redirect::to(&format!("{BASE}/{pid}?done=knowledge-removed#knowledge")).into_response()
        }
        Ok(false) => not_found(&m),
        Err(e) => {
            tracing::error!(error = %e, "assistant: the knowledge could not be written");
            knowledge_refused(
                &state,
                &m,
                &p,
                "assistant-error-not-saved",
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        }
    }
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
    fn the_answer_being_written_names_its_last_tool_and_how_to_stop_it() {
        let reader = crate::access::Viewer::anonymous();
        let chrome = render::Chrome::resolve(&reader, &HeaderMap::new(), "/");
        let tools = ["chembl · x".to_string(), "ax-genealogy · get_person".into()];
        let html = live_fragment(&chrome, "Je cherche.", 3, &tools, Some("/s"));
        assert!(
            html.contains("using <code>ax-genealogy · get_person</code>"),
            "{html}"
        );
        assert!(!html.contains("chembl"), "the last one called only");
        assert!(
            html.contains(
                r#"<noscript><form class="as-live-stop" method="post" action="&#x2f;s">"#
            ),
            "{html}"
        );
        // Where another language puts the name, it is set apart there.
        let mut headers = HeaderMap::new();
        headers.insert(
            header::COOKIE,
            format!("{}=ja", crate::i18n::COOKIE_NAME).parse().unwrap(),
        );
        let ja = render::Chrome::resolve(&reader, &headers, "/");
        let html = live_fragment(&ja, "", 0, &tools[1..], None);
        assert!(
            html.contains("<code>ax-genealogy · get_person</code> を使用中"),
            "{html}"
        );
        assert!(!html.contains("noscript"), "no stop where none is given");
        assert!(!live_fragment(&chrome, "", 0, &[], None).contains("as-live-tool"));
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
