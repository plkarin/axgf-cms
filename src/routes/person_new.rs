//! `/admin/person/new` — a person created with what surrounds them
//! (docs/adr/0039, `crate::person_new`).
//!
//! One page, one save: the name in its parts, a gender, a birth and a death
//! with their places, a portrait, a biography, a note, the parents, the
//! brothers and sisters, the unions with their children. What the writer
//! names that the archive holds is reused; what it does not hold is created
//! in the same commit, places and people alike.

use axum::extract::{FromRequest, Query, Request, State};
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::{IntoResponse, Redirect, Response};
use minijinja::context;
use serde::Deserialize;
use serde_json::{json, Value};

use super::admin::{require, Need};
use super::Shared;
use crate::access::Viewer;
use crate::person_new::{self, Submission, UnionIn};
use crate::render;
use crate::state::{BatchOutcome, Op};

/// `?name=` — a name typed in the search, to start from.
#[derive(Debug, Default, Deserialize)]
pub struct NewQuery {
    #[serde(default)]
    name: Option<String>,
}

/// The note written on the page, kept to show again when the save fails.
#[derive(Debug, Default, Clone)]
struct NoteIn {
    title: String,
    markdown: String,
}

impl NoteIn {
    fn from_fields(fields: &[(String, String)]) -> Self {
        let mut n = NoteIn::default();
        for (k, v) in fields {
            match k.as_str() {
                "note_title" => n.title = v.trim().to_string(),
                "note" => n.markdown = v.replace("\r\n", "\n"),
                _ => {}
            }
        }
        n
    }
}

/// The portrait sent with the page.
#[derive(Default)]
struct Portrait {
    filename: String,
    bytes: Vec<u8>,
    too_large: bool,
}

/// `GET /admin/person/new`
pub async fn form(
    State(state): State<Shared>,
    headers: HeaderMap,
    Query(q): Query<NewQuery>,
) -> Response {
    let (viewer, chrome) = match require(&state, &headers, Need::Write) {
        Ok(v) => v,
        Err(r) => return r,
    };
    let mut s = Submission {
        gender: "U".into(),
        living: true,
        ..Default::default()
    };
    if let Some(name) = q.name.as_deref() {
        let (given, family) = person_new::split_typed(name.trim());
        s.given = given;
        s.family = family;
    }
    page(
        &state,
        &chrome,
        &viewer,
        &s,
        &NoteIn::default(),
        TABS[0],
        None,
    )
}

/// The page's four tabs, in order (the record's editor's strip).
const TABS: [&str; 4] = ["pn-name", "pn-life", "pn-texts", "pn-family"];

/// The tab a field is on, so a refused save comes back to it.
fn tab_of(field: &str) -> &'static str {
    match field {
        "given" | "family" | "nick" | "gender" | "living" | "portrait" => "pn-name",
        f if f.starts_with("birth_") || f.starts_with("death_") => "pn-life",
        "bio" | "note" | "note_title" => "pn-texts",
        _ => "pn-family",
    }
}

/// `POST /admin/person/new`
pub async fn create(State(state): State<Shared>, headers: HeaderMap, req: Request) -> Response {
    let (viewer, chrome) = match require(&state, &headers, Need::Write) {
        Ok(v) => v,
        Err(r) => return r,
    };
    let limits = state.limits();
    let (fields, portrait) = read_body(req, limits.document_bytes()).await;
    let s = Submission::from_fields(&fields);
    let note = NoteIn::from_fields(&fields);
    // The tab the save was made on, which a refusal not tied to a field
    // comes back to.
    let sent_tab = fields
        .iter()
        .find(|(k, _)| k == "tab")
        .and_then(|(_, v)| TABS.iter().find(|t| **t == v.as_str()))
        .copied()
        .unwrap_or(TABS[0]);
    let refuse = |message: String, field: Option<&str>| {
        let tab = field.map(tab_of).unwrap_or(sent_tab);
        page(
            &state,
            &chrome,
            &viewer,
            &s,
            &note,
            tab,
            Some((message, field.map(str::to_string))),
        )
    };
    if portrait.too_large {
        return refuse(
            chrome.t_args(
                "error-upload-too-large",
                &[("mb", (limits.document_mb as i64).into())],
            ),
            Some("portrait"),
        );
    }
    // The portrait: a picture, or nothing.
    let portrait = if portrait.bytes.is_empty() {
        None
    } else {
        match crate::documents::sniff(&portrait.bytes) {
            Some(kind) if kind.raster_image && limits.allows(kind.mime) => Some((portrait, kind)),
            Some(kind) if kind.raster_image => {
                return refuse(
                    chrome.t_args(
                        "error-upload-kind-refused",
                        &[(
                            "kind",
                            crate::limits::kind_name(kind.mime).to_string().into(),
                        )],
                    ),
                    Some("portrait"),
                )
            }
            _ => return refuse(chrome.t("person-new-error-portrait"), Some("portrait")),
        }
    };
    let portrait_id = portrait.as_ref().map(|_| uuid::Uuid::new_v4().to_string());

    let now = crate::view::now_iso8601();
    let quote = |n: &str| {
        chrome.t_args(
            "person-new-nickname-quoted",
            &[("nick", n.to_string().into())],
        )
    };
    let planned = state.read(|flat| {
        person_new::plan(
            &s,
            &person_new::Context {
                flat,
                quote: &quote,
                now: &now,
                avatar: portrait_id.as_deref(),
                ids: None,
            },
        )
    });
    let mut plan = match planned {
        Ok(p) => p,
        Err(r) => {
            let message = if r.typed.is_empty() {
                chrome.t(r.key)
            } else {
                chrome.t_args(
                    "person-new-field-error",
                    &[
                        ("typed", r.typed.clone().into()),
                        ("message", chrome.t(r.key).into()),
                    ],
                )
            };
            return refuse(message, Some(&r.field));
        }
    };
    let pid = plan.person_id.clone();
    let display = person_new::display_name(&s, &quote);

    // The portrait's document, after the person it shows.
    if let (Some((p, kind)), Some(doc_id)) = (portrait, portrait_id) {
        let entity = json!({
            "id": doc_id,
            "type": "document",
            "axgf_version": "1.0",
            "filename": super::admin::clean_filename(&p.filename, kind.ext),
            "mime_type": kind.mime,
            "document_type": "photo",
            "status": "present",
            "file": {
                "path": crate::documents::attachment_path(&doc_id, kind.ext),
                "size_bytes": p.bytes.len() as u64,
                "sha256": crate::documents::sha256_hex(&p.bytes),
            },
            "linked_to": [{"entity_type": "person", "entity_id": pid, "role": "subject"}],
            "version_num": 1,
            "created_at": now,
            "updated_at": now,
        });
        plan.ops.push(Op::AddDocument {
            entity,
            payload: p.bytes,
        });
    }

    // The note, with the pictures pasted into it, after the person.
    let mut tokens: Vec<String> = Vec::new();
    if !note.markdown.trim().is_empty() {
        let note_id = uuid::Uuid::new_v4().to_string();
        let pics = match super::notes::pictures(&state, &viewer, &note_id, &note.markdown, None) {
            Ok(p) => p,
            Err(key) => return refuse(chrome.t(key), Some("note")),
        };
        let draft = crate::notes::Draft {
            title: if note.title.is_empty() {
                display.clone()
            } else {
                note.title.clone()
            },
            category: String::new(),
            date: String::new(),
            persons: vec![pid.clone()],
            markdown: pics.markdown.clone(),
        };
        if let Some(key) = crate::notes::refusal(&draft) {
            return refuse(chrome.t(key), Some("note"));
        }
        let text = draft.markdown.clone().into_bytes();
        let mut entity =
            crate::notes::entity(&note_id, &draft, viewer.name(), 1, &text, &pics.ids, None);
        entity["created_at"] = json!(now);
        entity["updated_at"] = json!(now);
        entity["version_num"] = json!(1);
        plan.ops.extend(pics.ops);
        plan.ops.push(Op::AddDocument {
            entity,
            payload: text,
        });
        tokens = pics.tokens;
    }

    // What every save checks: the writer's scope, and that nothing names a
    // record that is neither in the archive nor in this save.
    let pending: Vec<String> = plan
        .ops
        .iter()
        .filter_map(|op| match op {
            Op::Create { entity, .. } | Op::AddDocument { entity, .. } => {
                entity.get("id").and_then(Value::as_str).map(str::to_string)
            }
            _ => None,
        })
        .collect();
    for op in &plan.ops {
        let (kind, entity, before) = match op {
            Op::Create { kind, entity } => (*kind, entity, None),
            Op::AddDocument { entity, .. } => (axgf_rs::EntityKind::Document, entity, None),
            Op::Update {
                kind, id, entity, ..
            } => (
                *kind,
                entity,
                state.read(|f| {
                    f.get(crate::admin::collection_for(*kind))
                        .and_then(|c| c.get(id))
                        .cloned()
                }),
            ),
            _ => continue,
        };
        if let Err(r) =
            super::admin::check_scope(&state, &chrome, &viewer, kind, entity, before.as_ref())
        {
            return r;
        }
        if let Err(msg) = super::admin::refuse_new_dangling(
            &state,
            entity,
            before.as_ref(),
            chrome.lang,
            &pending,
        ) {
            return refuse(msg, None);
        }
    }

    let outcome = match state.commit(plan.ops, viewer.name(), |kind, e| {
        super::admin::label_for(kind, e)
    }) {
        Ok(o) => o,
        Err(e) => return super::admin::io_error(&chrome, &e),
    };
    match outcome {
        BatchOutcome::Applied { records } => {
            state.note_images().take(viewer.name(), &tokens);
            Redirect::to(&format!("/person/{pid}?created={records}")).into_response()
        }
        BatchOutcome::Conflict { .. } => refuse(chrome.t("person-new-error-conflict"), None),
        BatchOutcome::Refused { diagnostics, .. } => refuse(
            format!(
                "{} {}",
                chrome.t("person-new-error-refused"),
                crate::state::format_diagnostics(&diagnostics)
            ),
            None,
        ),
    }
}

/// The page, filled with what was sent, and what went wrong if anything did.
fn page(
    state: &Shared,
    chrome: &render::Chrome,
    viewer: &Viewer,
    s: &Submission,
    note: &NoteIn,
    tab: &str,
    error: Option<(String, Option<String>)>,
) -> Response {
    let (persons, places) = state.read_as(viewer.ceiling(), |flat, lens| {
        (
            crate::forms::person_options(flat, lens),
            place_options(flat),
        )
    });
    // One empty row to type into: with scripting a row is added at need, and
    // without it this is the row there is.
    let mut nicks = s.nicks.clone();
    nicks.push(String::new());
    let mut siblings = s.siblings.clone();
    siblings.push(String::new());
    let mut unions: Vec<UnionIn> = s.unions.clone();
    if unions.is_empty() {
        unions.push(UnionIn::default());
    }
    let unions: Vec<Value> = unions
        .iter()
        .map(|u| {
            let mut children = u.children.clone();
            children.push(String::new());
            json!({"partner": u.partner, "kind": if u.kind.is_empty() { "marriage" } else { u.kind.as_str() },
                   "children": children})
        })
        .collect();
    let union_types: Vec<Value> = person_new::UNION_TYPES
        .iter()
        .map(|t| json!({"value": t, "label": chrome.t(&format!("union-type-{t}"))}))
        .collect();
    let genders: Vec<Value> = ["M", "F", "NB", "U"]
        .iter()
        .map(|g| json!({"value": g, "label": chrome.t(&format!("gender-{g}"))}))
        .collect();
    let tabs: Vec<Value> = TABS
        .iter()
        .zip([
            "person-new-section-name",
            "person-new-section-life",
            "person-new-section-texts",
            "record-family",
        ])
        .map(|(key, label)| json!({"key": key, "label": chrome.t(label)}))
        .collect();
    let (message, field) = match error {
        Some((m, f)) => (Some(m), f),
        None => (None, None),
    };
    let status = if message.is_some() {
        StatusCode::BAD_REQUEST
    } else {
        StatusCode::OK
    };
    let mut response = render::page_with(
        chrome,
        "admin_person_new.html",
        context! {
            nav => "admin",
            tab,
            tabs,
            s => json!({
                "given": s.given, "family": s.family, "gender": s.gender, "living": s.living,
                "birth_date": s.birth_date, "birth_circa": s.birth_circa, "birth_place": s.birth_place,
                "death_date": s.death_date, "death_circa": s.death_circa, "death_place": s.death_place,
                "bio": s.bio, "father": s.father, "mother": s.mother,
            }),
            nicks,
            siblings,
            unions,
            union_types,
            genders,
            note_title => note.title,
            note => note.markdown,
            error => message,
            error_field => field,
            persons,
            places,
            max_upload_bytes => state.limits().document_bytes(),
            max_upload_mb => state.limits().document_mb,
        },
    );
    *response.status_mut() = status;
    response
}

/// Every place as the page's list offers it: its name, and its short id
/// after it when two places share the name.
fn place_options(flat: &Value) -> Vec<Value> {
    let Some(map) = flat.get("places").and_then(Value::as_object) else {
        return Vec::new();
    };
    let mut named: Vec<(String, String)> = map
        .iter()
        .map(|(id, p)| (crate::forms::place_label(p), id.clone()))
        .filter(|(l, _)| !l.is_empty())
        .collect();
    named.sort();
    let mut counts = std::collections::HashMap::<String, usize>::new();
    for (l, _) in &named {
        *counts.entry(l.to_lowercase()).or_default() += 1;
    }
    named
        .into_iter()
        .map(|(label, id)| {
            let shown = if counts.get(&label.to_lowercase()).copied().unwrap_or(0) > 1 {
                format!("{label} · #{}", crate::forms::short_id(&id))
            } else {
                label
            };
            json!({"id": id, "label": shown})
        })
        .collect()
}

/// The fields as sent, repeated names kept in order, and the portrait.
async fn read_body(req: Request, most: usize) -> (Vec<(String, String)>, Portrait) {
    let multipart = req
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .is_some_and(|ct| ct.starts_with("multipart/form-data"));
    if !multipart {
        let form = axum::extract::Form::<Vec<(String, String)>>::from_request(req, &())
            .await
            .map(|f| f.0)
            .unwrap_or_default();
        return (form, Portrait::default());
    }
    let mut fields = Vec::new();
    let mut portrait = Portrait::default();
    let Ok(mut mp) = axum::extract::Multipart::from_request(req, &()).await else {
        return (fields, portrait);
    };
    loop {
        let field = match mp.next_field().await {
            Ok(Some(f)) => f,
            Ok(None) => break,
            Err(e) => {
                portrait.too_large = e.status() == StatusCode::PAYLOAD_TOO_LARGE;
                break;
            }
        };
        let name = field.name().unwrap_or_default().to_string();
        if name == "portrait" {
            portrait.filename = field.file_name().unwrap_or("portrait").to_string();
            match super::uploads::read_capped(field, most).await {
                Ok(Some(b)) => portrait.bytes = b,
                Ok(None) => portrait.too_large = true,
                Err(_) => {
                    portrait.too_large = true;
                    break;
                }
            }
        } else {
            fields.push((name, field.text().await.unwrap_or_default()));
        }
    }
    (fields, portrait)
}
