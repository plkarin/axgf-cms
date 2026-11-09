//! The AFG on a person's page (the operator's request of 4 November 2026,
//! docs/adr/0031): a member the AFG is open to talks with it about the
//! person, on the record's AFG tab, and finds their conversations there
//! again.
//!
//! The tab is drawn by the record page (`/person/:id?tab=afg`), with
//! [`tab`] for what it holds. The routes here begin a conversation, carry
//! it on, show the answer being written, ask again, rename and delete.
//!
//! Everything here is the signed-in account's own: the conversations live in
//! its assistant space (`crate::assistant`), in the person's project, and a
//! conversation that is not the account's answers 404 as one that does not
//! exist. What the AFG reads goes through the account's lens
//! (`crate::person_chat`).

use axum::extract::{Form, Multipart, Path, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Redirect, Response};
use serde::Deserialize;
use serde_json::{json, Value};

use super::assistant::{json_reply, wants_json};
use crate::access::Viewer;
use crate::assistant::{self, Message, Project, Role, Space, Spaces};
use crate::routes::Shared;
use crate::{auth, render};

/// Whether this reader has the AFG tab on a person's page: a signed-in
/// account the AFG is open to (docs/adr/0024 §5).
pub fn open_to(viewer: &Viewer) -> bool {
    viewer.user.as_ref().is_some_and(|u| u.afg)
}

/// The tab's address, and a conversation's on it.
fn tab_url(person: &str, cid: Option<&str>) -> String {
    match cid {
        Some(c) => format!("/person/{person}?tab=afg&c={c}"),
        None => format!("/person/{person}?tab=afg"),
    }
}

/// The account at the door, the person it may read, its space opened.
struct Chatter {
    viewer: Viewer,
    chrome: render::Chrome,
    user: crate::acl::User,
    space: Space,
    person: String,
    name: String,
}

/// Who may come in: a signed-in account the AFG is open to, on a person it
/// may read. Anyone else is told why not.
#[allow(clippy::result_large_err)]
fn door(state: &Shared, headers: &HeaderMap, person: &str) -> Result<Chatter, Response> {
    let viewer = auth::viewer(state, headers);
    if !viewer.signed_in() {
        return Err(Redirect::to("/admin/login").into_response());
    }
    let chrome =
        render::Chrome::resolve(&viewer, headers, &tab_url(person, None)).with_archive(state);
    // The account as stored now: its access may have changed since it
    // signed in.
    let user = viewer
        .user
        .as_ref()
        .and_then(|u| state.acl_read(|acl| acl.by_id(&u.id).cloned()))
        .filter(|u| u.afg);
    let Some(user) = user else {
        return Err(render::error_page_in(
            &chrome,
            StatusCode::FORBIDDEN,
            "afg-record-title",
            "afg-record-no-access",
        ));
    };
    let name = state.read_as(viewer.ceiling(), |flat, lens| {
        flat.get("persons")
            .and_then(|p| p.get(person))
            .filter(|_| lens.sees_person(person))
            .map(crate::view::person_display_name)
    });
    let Some(name) = name else {
        return Err(render::error_page_in(
            &chrome,
            StatusCode::NOT_FOUND,
            "error-no-such-person-title",
            "error-no-such-person-detail",
        ));
    };
    match Spaces::beside(state.bundle_path()).of(&crate::brain::Owner::of(&user)) {
        Ok(space) => Ok(Chatter {
            viewer,
            chrome,
            user,
            space,
            person: person.to_string(),
            name,
        }),
        Err(e) => {
            tracing::error!(error = %e, "person chat: the space could not be opened");
            Err(render::error_page_in(
                &chrome,
                StatusCode::INTERNAL_SERVER_ERROR,
                "afg-record-title",
                "assistant-error-not-saved",
            ))
        }
    }
}

/// A refusal: said to the script, or on a page with the way back to the
/// tab.
fn refused(
    c: &Chatter,
    headers: &HeaderMap,
    key: &str,
    status: StatusCode,
    cid: Option<&str>,
) -> Response {
    let text = c.chrome.t(key);
    if wants_json(headers) {
        return json_reply(status, json!({"error": text}));
    }
    let back = tab_url(&c.person, cid);
    render::error_page_back_in(
        &c.chrome,
        status,
        "person-afg-title-short",
        key,
        &[],
        Some((&back, "person-afg-back")),
    )
}

/// The dossier the AFG reads about the person, as this account may read it.
fn dossier(
    state: &Shared,
    viewer: &Viewer,
    person: &str,
    lang: &str,
) -> Option<crate::person_chat::Dossier> {
    let cfg = state.site().afg;
    // The sensitive classes go only when the AFG room says they may, as with
    // the archive (docs/adr/0023).
    let allowed = if cfg.sensitive {
        crate::sensitive::Scopes::EVERY
    } else {
        crate::sensitive::Scopes::NONE
    };
    let gathered = state.read_as(viewer.ceiling(), |flat, lens| {
        crate::person_chat::gather(flat, lens, person, lang, allowed)
    })?;
    Some(crate::person_chat::fill(
        gathered,
        &|path| {
            state
                .attachment(path)
                .map(|b| String::from_utf8_lossy(&b).into_owned())
        },
        // A PDF's pages, counted once from its bytes (docs/adr/0040).
        &|pdf| state.pdf_pages(&pdf.sha256, || state.attachment(&pdf.path)),
    ))
}

/// One of the person's photographs as the AFG is given it: read from the
/// bundle and sized once, then kept (`AppState::model_photos`). `None` when
/// its bytes are not there or are not a picture this build decodes.
fn model_photo(state: &Shared, photo: &crate::person_chat::Photo) -> Option<Vec<u8>> {
    state
        .model_photos()
        .get_or_insert(&photo.id, &photo.sha256, || {
            let bytes = state.attachment(&photo.path)?;
            crate::assistant::model_photo(&bytes)
        })
}

/// Begin writing the conversation's next answer in the background. `false`
/// when it is already being answered, the site answers as many as it may, or
/// the person can no longer be read.
fn start_answer(state: &Shared, c: &Chatter, project: &Project, cid: &str) -> bool {
    let Some(d) = dossier(state, &c.viewer, &c.person, c.chrome.lang) else {
        return false;
    };
    let Some(live) = state
        .assistant_answering()
        .begin(&c.user.id, &project.id, cid)
    else {
        return false;
    };
    let messages = c.space.messages(&project.id, cid);
    let me = c.user.person_id.as_ref().and_then(|id| {
        state.read_as(c.viewer.ceiling(), |flat, lens| {
            flat.get("persons")
                .and_then(|ps| ps.get(id))
                .filter(|_| lens.sees_person(id))
                .map(|p| (id.clone(), crate::view::person_display_name(p)))
        })
    });
    let asker = assistant::Asker {
        name: &c.user.username,
        person: me.as_ref().map(|(id, n)| (id.as_str(), n.as_str())),
        language: super::assistant::language_name(c.chrome.lang),
    };
    let prompt = crate::person_chat::chat_prompt(&c.space, &asker, project, &d, &messages);
    tracing::info!(
        account = %c.user.username,
        person = %c.person,
        kind = state.site().afg.kind.slug(),
        turns = prompt.turns.len(),
        notes = d.notes.len(),
        relatives = d.relatives.len(),
        persons = d.persons,
        photos = d.photos.len(),
        pdfs = d.pdfs.len(),
        pdf_pages = d.pdf_pages(),
        "AFG: a question about a person sent"
    );
    // The photographs are read and sized on the way, not while the page
    // waits, and the PDF documents read (docs/adr/0040); a model that takes
    // neither is asked without them.
    let prepare = (!d.photos.is_empty() || !d.pdfs.is_empty()).then(|| {
        let state = state.clone();
        let photos = d.photos.clone();
        let pdfs = d.pdfs.clone();
        let mut instead = prompt.clone();
        crate::person_chat::files_refused(&mut instead, !photos.is_empty(), !pdfs.is_empty());
        super::assistant::Prepare {
            work: Box::new(move |prompt: &mut crate::afg_chat::ChatPrompt| {
                let given = photos
                    .iter()
                    .filter_map(|p| model_photo(&state, p).map(|data| (p.filename.clone(), data)))
                    .collect();
                crate::person_chat::with_photos(prompt, given);
                let read = pdfs
                    .iter()
                    .filter_map(|p| {
                        state
                            .attachment(&p.path)
                            .map(|data| (p.filename.clone(), data))
                    })
                    .collect();
                crate::person_chat::with_pdfs(prompt, read);
            }),
            instead,
        }
    });
    super::assistant::run_answer(
        state,
        c.space.clone(),
        c.user.id.clone(),
        project.id.clone(),
        cid.to_string(),
        prompt,
        prepare,
        live,
        false,
    );
    true
}

/// The words of a message: the composer's `text`, files left aside (the
/// person's page takes words only).
async fn read_text(mut multipart: Multipart) -> Result<String, &'static str> {
    let mut text = String::new();
    loop {
        match multipart.next_field().await {
            Ok(Some(field)) => {
                if field.name() == Some("text") {
                    text = field.text().await.map_err(|_| "assistant-error-too-big")?;
                }
            }
            Ok(None) => break,
            Err(_) => return Err("assistant-error-too-big"),
        }
    }
    let text = text.trim().to_string();
    if text.chars().count() > assistant::TEXT_MAX {
        return Err("assistant-error-text-long");
    }
    if text.is_empty() {
        return Err("assistant-error-empty");
    }
    Ok(text)
}

// ---------------------------------------------------------------------------
// the tab
// ---------------------------------------------------------------------------

/// How many conversations this reader has about the person: the tab's count.
pub fn count(state: &Shared, viewer: &Viewer, person: &str) -> usize {
    viewer
        .user
        .as_ref()
        .and_then(|u| Spaces::beside(state.bundle_path()).existing(&crate::brain::Owner::of(u)))
        .and_then(|space| {
            space
                .person_project(person)
                .map(|p| space.conversations(&p.id).len())
        })
        .unwrap_or(0)
}

/// What the AFG tab holds for this reader: its conversations about the
/// person, the one open, its thread and the answer being written, and what
/// the AFG reads. Nothing is made by looking: a reader who never asked has no
/// space opened for it.
pub fn tab(
    state: &Shared,
    viewer: &Viewer,
    chrome: &render::Chrome,
    person: &str,
    open: Option<&str>,
    done: Option<&str>,
) -> Value {
    let space = viewer
        .user
        .as_ref()
        .and_then(|u| Spaces::beside(state.bundle_path()).existing(&crate::brain::Owner::of(u)));
    let project = space.as_ref().and_then(|s| s.person_project(person));
    let conversations: Vec<Value> = match (&space, &project) {
        (Some(s), Some(p)) => s
            .conversations(&p.id)
            .iter()
            .map(|c| {
                json!({"id": c.id, "title": c.title, "updated_at": c.updated_at,
                       "messages": c.messages})
            })
            .collect(),
        _ => Vec::new(),
    };
    let mut conversation = Value::Null;
    let mut thread: Vec<String> = Vec::new();
    let mut live: Option<String> = None;
    let mut orphan = false;
    if let (Some(s), Some(p), Some(cid)) = (&space, &project, open) {
        if let Some(c) = s.conversation(&p.id, cid) {
            let messages = s.messages(&p.id, &c.id);
            thread = messages
                .iter()
                .enumerate()
                .map(|(i, m)| {
                    super::assistant::message_fragment(
                        chrome,
                        "",
                        &p.id,
                        &c.id,
                        Some(person),
                        m,
                        i + 1 == messages.len(),
                    )
                })
                .collect();
            let account = viewer.user.as_ref().map(|u| u.id.as_str()).unwrap_or("");
            live = state
                .assistant_answering()
                .get(account, &p.id, &c.id)
                .map(|l| {
                    let partial = l.partial.lock().map(|s| s.clone()).unwrap_or_default();
                    super::assistant::live_fragment(chrome, &partial, l.since.elapsed().as_secs())
                });
            orphan = live.is_none() && messages.last().is_some_and(|m| m.role == Role::User);
            conversation = json!({"id": c.id, "title": c.title});
        }
    }
    // What the AFG reads, said before the first question: its size.
    let reads = dossier(state, viewer, person, chrome.lang).map(|d| {
        json!({"photos": d.photos.len(), "pdfs": d.pdfs.len(), "notes": d.notes.len(),
               "relatives": d.relatives.len(), "persons": d.persons, "tokens": d.tokens()})
    });
    let notice = match done {
        Some("saved") => Some(chrome.t("assistant-saved")),
        Some("deleted") => Some(chrome.t("assistant-conversation-deleted")),
        Some("deleted-all") => Some(chrome.t("person-afg-deleted-all")),
        _ => None,
    };
    json!({
        "ready": super::assistant::ready(state),
        "conversations": conversations,
        "conversation": conversation,
        "thread": thread,
        "live": live,
        "orphan": orphan,
        "reads": reads,
        "notice": notice,
        "limits": {"text": assistant::TEXT_MAX, "files": 0, "file_mb": 0},
    })
}

// ---------------------------------------------------------------------------
// conversations
// ---------------------------------------------------------------------------

/// `POST /person/:id/afg` — a new conversation about the person, on its
/// first question.
pub async fn start(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(person): Path<String>,
    multipart: Multipart,
) -> Response {
    let c = match door(&state, &headers, &person) {
        Ok(c) => c,
        Err(r) => return r,
    };
    let text = match read_text(multipart).await {
        Ok(t) => t,
        Err(key) => return refused(&c, &headers, key, StatusCode::UNPROCESSABLE_ENTITY, None),
    };
    if !super::assistant::ready(&state) {
        return refused(
            &c,
            &headers,
            "assistant-not-ready",
            StatusCode::CONFLICT,
            None,
        );
    }
    if state.assistant_answering().full() {
        return refused(
            &c,
            &headers,
            "assistant-error-busy",
            StatusCode::SERVICE_UNAVAILABLE,
            None,
        );
    }
    let project = match c.space.open_person_project(&c.person, &c.name) {
        Ok(p) => p,
        Err(e) => {
            tracing::error!(error = %e, "person chat: the person's project could not be made");
            return refused(
                &c,
                &headers,
                "assistant-error-not-saved",
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
            );
        }
    };
    let conv = match c.space.create_conversation(&project.id, &text, "") {
        Ok(Some(conv)) => conv,
        Ok(None) => {
            return refused(
                &c,
                &headers,
                "assistant-error-too-many",
                StatusCode::UNPROCESSABLE_ENTITY,
                None,
            )
        }
        Err(e) => {
            tracing::error!(error = %e, "person chat: a conversation could not be made");
            return refused(
                &c,
                &headers,
                "assistant-error-not-saved",
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
            );
        }
    };
    let msg = Message::new(Role::User, text, Vec::new());
    if let Err(e) = c.space.append(&project.id, &conv.id, &msg) {
        tracing::error!(error = %e, "person chat: a message could not be kept");
        return refused(
            &c,
            &headers,
            "assistant-error-not-saved",
            StatusCode::INTERNAL_SERVER_ERROR,
            Some(&conv.id),
        );
    }
    start_answer(&state, &c, &project, &conv.id);
    let url = tab_url(&c.person, Some(&conv.id));
    if wants_json(&headers) {
        return json_reply(StatusCode::OK, json!({"redirect": url}));
    }
    Redirect::to(&url).into_response()
}

/// The person's project and one of its conversations, or the 404 a
/// conversation that is not this account's answers.
#[allow(clippy::result_large_err)]
fn conversation_of(c: &Chatter, cid: &str) -> Result<(Project, assistant::Conversation), Response> {
    match c.space.person_project(&c.person).and_then(|p| {
        let conv = c.space.conversation(&p.id, cid)?;
        Some((p, conv))
    }) {
        Some(found) => Ok(found),
        None => Err(render::error_page_back_in(
            &c.chrome,
            StatusCode::NOT_FOUND,
            "person-afg-title-short",
            "person-afg-gone",
            &[],
            Some((&tab_url(&c.person, None), "person-afg-back")),
        )),
    }
}

/// `POST /person/:id/afg/:conversation` — the next question.
pub async fn send(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path((person, cid)): Path<(String, String)>,
    multipart: Multipart,
) -> Response {
    let c = match door(&state, &headers, &person) {
        Ok(c) => c,
        Err(r) => return r,
    };
    let (project, conv) = match conversation_of(&c, &cid) {
        Ok(found) => found,
        Err(r) => return r,
    };
    let text = match read_text(multipart).await {
        Ok(t) => t,
        Err(key) => {
            return refused(
                &c,
                &headers,
                key,
                StatusCode::UNPROCESSABLE_ENTITY,
                Some(&cid),
            )
        }
    };
    if state
        .assistant_answering()
        .get(&c.user.id, &project.id, &cid)
        .is_some()
    {
        return refused(
            &c,
            &headers,
            "assistant-error-wait",
            StatusCode::CONFLICT,
            Some(&cid),
        );
    }
    if !super::assistant::ready(&state) {
        return refused(
            &c,
            &headers,
            "assistant-not-ready",
            StatusCode::CONFLICT,
            Some(&cid),
        );
    }
    if state.assistant_answering().full() {
        return refused(
            &c,
            &headers,
            "assistant-error-busy",
            StatusCode::SERVICE_UNAVAILABLE,
            Some(&cid),
        );
    }
    if conv.messages >= assistant::MESSAGES_MAX {
        return refused(
            &c,
            &headers,
            "assistant-error-too-many",
            StatusCode::UNPROCESSABLE_ENTITY,
            Some(&cid),
        );
    }
    let msg = Message::new(Role::User, text, Vec::new());
    if let Err(e) = c.space.append(&project.id, &cid, &msg) {
        tracing::error!(error = %e, "person chat: a message could not be kept");
        return refused(
            &c,
            &headers,
            "assistant-error-not-saved",
            StatusCode::INTERNAL_SERVER_ERROR,
            Some(&cid),
        );
    }
    let answering = start_answer(&state, &c, &project, &cid);
    if wants_json(&headers) {
        let html = super::assistant::message_fragment(
            &c.chrome,
            "",
            &project.id,
            &cid,
            Some(&c.person),
            &msg,
            true,
        );
        return json_reply(
            StatusCode::OK,
            json!({
                "message": {"id": msg.id, "html": html},
                "count": c.space.messages(&project.id, &cid).len(),
                "answering": answering,
            }),
        );
    }
    Redirect::to(&tab_url(&c.person, Some(&cid))).into_response()
}

#[derive(Deserialize, Default)]
pub struct StateQuery {
    /// How many messages the page already shows.
    #[serde(default)]
    after: usize,
}

/// `GET /person/:id/afg/:conversation/state?after=N` — what the page does
/// not show yet: the messages after its `N` first, and the answer being
/// written.
pub async fn state_of(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path((person, cid)): Path<(String, String)>,
    Query(q): Query<StateQuery>,
) -> Response {
    let c = match door(&state, &headers, &person) {
        Ok(c) => c,
        Err(_) => return json_reply(StatusCode::FORBIDDEN, json!({"error": "forbidden"})),
    };
    let Ok((project, _)) = conversation_of(&c, &cid) else {
        return json_reply(StatusCode::NOT_FOUND, json!({"error": "gone"}));
    };
    let messages = c.space.messages(&project.id, &cid);
    let new: Vec<Value> = messages
        .iter()
        .enumerate()
        .skip(q.after)
        .map(|(i, m)| {
            let html = super::assistant::message_fragment(
                &c.chrome,
                "",
                &project.id,
                &cid,
                Some(&c.person),
                m,
                i + 1 == messages.len(),
            );
            json!({"id": m.id, "role": m.role, "html": html})
        })
        .collect();
    let live = state
        .assistant_answering()
        .get(&c.user.id, &project.id, &cid);
    let (answering, live_html) = match &live {
        Some(l) => {
            let partial = l.partial.lock().map(|s| s.clone()).unwrap_or_default();
            (
                true,
                super::assistant::live_fragment(&c.chrome, &partial, l.since.elapsed().as_secs()),
            )
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

/// `POST /person/:id/afg/:conversation/retry` — ask again, after an answer
/// that failed or a question left without one.
pub async fn retry(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path((person, cid)): Path<(String, String)>,
) -> Response {
    let c = match door(&state, &headers, &person) {
        Ok(c) => c,
        Err(r) => return r,
    };
    let (project, _) = match conversation_of(&c, &cid) {
        Ok(found) => found,
        Err(r) => return r,
    };
    let waiting = c
        .space
        .messages(&project.id, &cid)
        .last()
        .is_some_and(|m| m.role != Role::Assistant);
    let answering =
        waiting && super::assistant::ready(&state) && start_answer(&state, &c, &project, &cid);
    if wants_json(&headers) {
        return json_reply(StatusCode::OK, json!({"answering": answering}));
    }
    Redirect::to(&tab_url(&c.person, Some(&cid))).into_response()
}

#[derive(Deserialize, Default)]
pub struct TitleForm {
    #[serde(default)]
    title: String,
}

/// `POST /person/:id/afg/:conversation/edit` — its title.
pub async fn rename(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path((person, cid)): Path<(String, String)>,
    Form(f): Form<TitleForm>,
) -> Response {
    let c = match door(&state, &headers, &person) {
        Ok(c) => c,
        Err(r) => return r,
    };
    let (project, mut conv) = match conversation_of(&c, &cid) {
        Ok(found) => found,
        Err(r) => return r,
    };
    let title = assistant::clean_name(&f.title, assistant::TITLE_MAX);
    if title.is_empty() {
        return refused(
            &c,
            &headers,
            "assistant-error-name",
            StatusCode::UNPROCESSABLE_ENTITY,
            Some(&cid),
        );
    }
    conv.title = title;
    match c.space.save_conversation(&project.id, &conv) {
        Ok(()) => {
            Redirect::to(&format!("{}&done=saved", tab_url(&c.person, Some(&cid)))).into_response()
        }
        Err(e) => {
            tracing::error!(error = %e, "person chat: a conversation could not be saved");
            refused(
                &c,
                &headers,
                "assistant-error-not-saved",
                StatusCode::INTERNAL_SERVER_ERROR,
                Some(&cid),
            )
        }
    }
}

#[derive(Deserialize, Default)]
pub struct ConfirmForm {
    #[serde(default)]
    confirm: Option<String>,
}

/// `POST /person/:id/afg/:conversation/delete` — a conversation, once
/// confirmed.
pub async fn delete(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path((person, cid)): Path<(String, String)>,
    Form(f): Form<ConfirmForm>,
) -> Response {
    let c = match door(&state, &headers, &person) {
        Ok(c) => c,
        Err(r) => return r,
    };
    let (project, _) = match conversation_of(&c, &cid) {
        Ok(found) => found,
        Err(r) => return r,
    };
    if f.confirm.is_none() {
        return refused(
            &c,
            &headers,
            "assistant-error-confirm",
            StatusCode::UNPROCESSABLE_ENTITY,
            Some(&cid),
        );
    }
    if state
        .assistant_answering()
        .get(&c.user.id, &project.id, &cid)
        .is_some()
    {
        return refused(
            &c,
            &headers,
            "assistant-error-wait",
            StatusCode::CONFLICT,
            Some(&cid),
        );
    }
    match c.space.delete_conversation(&project.id, &cid) {
        Ok(_) => {
            Redirect::to(&format!("{}&done=deleted", tab_url(&c.person, None))).into_response()
        }
        Err(e) => {
            tracing::error!(error = %e, "person chat: a conversation could not be deleted");
            refused(
                &c,
                &headers,
                "assistant-error-not-saved",
                StatusCode::INTERNAL_SERVER_ERROR,
                Some(&cid),
            )
        }
    }
}

/// `POST /person/:id/afg/delete` — every conversation this account has about
/// the person, once confirmed.
pub async fn delete_all(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(person): Path<String>,
    Form(f): Form<ConfirmForm>,
) -> Response {
    let c = match door(&state, &headers, &person) {
        Ok(c) => c,
        Err(r) => return r,
    };
    if f.confirm.is_none() {
        return refused(
            &c,
            &headers,
            "assistant-error-confirm",
            StatusCode::UNPROCESSABLE_ENTITY,
            None,
        );
    }
    let Some(project) = c.space.person_project(&c.person) else {
        return Redirect::to(&format!("{}&done=deleted-all", tab_url(&c.person, None)))
            .into_response();
    };
    let answering = c.space.conversations(&project.id).iter().any(|conv| {
        state
            .assistant_answering()
            .get(&c.user.id, &project.id, &conv.id)
            .is_some()
    });
    if answering {
        return refused(
            &c,
            &headers,
            "assistant-error-wait",
            StatusCode::CONFLICT,
            None,
        );
    }
    match c.space.delete_project(&project.id) {
        Ok(_) => {
            Redirect::to(&format!("{}&done=deleted-all", tab_url(&c.person, None))).into_response()
        }
        Err(e) => {
            tracing::error!(error = %e, "person chat: the conversations could not be deleted");
            refused(
                &c,
                &headers,
                "assistant-error-not-saved",
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
            )
        }
    }
}
