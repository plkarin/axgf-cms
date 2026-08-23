//! The admin panel: server-rendered forms, no client-side state.
//!
//! Every mutation goes through [`AppState::mutate`], so the atomic-write and
//! refuse-cleanly guarantees apply uniformly. Diagnostics are always shown —
//! warnings never block, mirroring the library's non-blocking philosophy.

use std::collections::HashMap;

use axum::extract::{Form, Path, Query, State};
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::{IntoResponse, Redirect, Response};
use minijinja::context;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::access::Viewer;
use crate::admin::{
    apply_form, fields_for, get_path, kind_from_str, paginate, policy_from_str, KINDS,
};
use crate::routes::Shared;
use crate::state::MutationOutcome;
use crate::{auth, documents, render, view};

/// Who is asking, and the response to send when they may not be here.
///
/// Two guards, not one, because the panel is no longer an admin-only place:
/// a contributor creates and updates entities through the same forms. What
/// separates them is [`guard_admin!`] on the routes that manage accounts,
/// delete, dedup, validate and export.
///
/// The error variant is a whole rendered `Response`, which is large; boxing it
/// keeps the common `Ok` path cheap.
#[allow(clippy::result_large_err)]
fn require(state: &Shared, headers: &HeaderMap, need: Need) -> Result<Viewer, Response> {
    let viewer = auth::viewer(state, headers);
    let ok = match need {
        Need::Write => viewer.may_write(),
        Need::Admin => viewer.is_admin(),
    };
    if ok {
        return Ok(viewer);
    }
    if viewer.signed_in() {
        // Signed in, but not enough. Saying so is the useful answer: a
        // contributor who lands on the user list should be told their role is
        // the reason, not shown a login form for the account they are already
        // using.
        return Err((
            StatusCode::FORBIDDEN,
            render::error_page(
                StatusCode::FORBIDDEN,
                "Not for your role",
                match need {
                    Need::Admin => {
                        "This is an administrator's page. Your \
                         account can create and edit records, but not manage \
                         accounts, delete entities or export the bundle."
                    }
                    Need::Write => {
                        "Your account can read this bundle but not \
                         change it. An administrator can raise your role to \
                         contributor."
                    }
                },
            ),
        )
            .into_response());
    }
    // 401 rather than a redirect: this is the answer an integration test and a
    // script both need, and the body still carries the form.
    Err((
        StatusCode::UNAUTHORIZED,
        render::page(
            "admin_login.html",
            context! {
                nav => "admin",
                is_admin => false,
                error => "Sign in to reach the admin panel.",
            },
        ),
    )
        .into_response())
}

/// What a route requires of the account reaching it.
#[derive(Clone, Copy)]
enum Need {
    /// Contributor or admin.
    Write,
    /// Admin only.
    Admin,
}

macro_rules! guard {
    ($state:expr, $headers:expr) => {
        match require(&$state, &$headers, Need::Write) {
            Ok(v) => v,
            Err(r) => return r,
        }
    };
}

macro_rules! guard_admin {
    ($state:expr, $headers:expr) => {
        match require(&$state, &$headers, Need::Admin) {
            Ok(v) => v,
            Err(r) => return r,
        }
    };
}

/// Refuse a write that falls outside the account's branch.
///
/// The accessible set is computed **once per request** — it is a walk of the
/// whole family graph, and recomputing it per entity would repeat that walk
/// for an answer that cannot change inside one request.
///
/// `proposed` is the entity as submitted; `existing` is the entity as the
/// bundle currently holds it, or `None` on a create. Both are checked, and
/// that is the point of passing both: checking only the submitted form would
/// let a scoped contributor retarget an in-scope record at people outside the
/// branch, and checking only the stored one would let them edit a record into
/// their branch that never belonged to it.
#[allow(clippy::result_large_err)]
fn check_scope(
    state: &Shared,
    viewer: &Viewer,
    kind: axgf_rs::EntityKind,
    proposed: &Value,
    existing: Option<&Value>,
) -> Result<(), Response> {
    let Some(scope) = state.read(|flat| viewer.scope(flat)) else {
        return Ok(()); // unscoped account: the whole tree
    };
    let mut subjects = crate::access::subjects_of(kind, proposed);
    if let Some(existing) = existing {
        subjects.extend(crate::access::subjects_of(kind, existing));
        subjects.sort();
        subjects.dedup();
    }
    match crate::access::check_write(viewer, Some(&scope), &subjects) {
        Ok(()) => Ok(()),
        Err(crate::access::Denied::Role) => Err(render::error_page(
            StatusCode::FORBIDDEN,
            "Not for your role",
            "Your account may read this bundle but not change it.",
        )),
        Err(crate::access::Denied::Scope) => Err(render::error_page(
            StatusCode::FORBIDDEN,
            "Outside your branch",
            if subjects.is_empty() {
                "Your account is restricted to one branch of the tree, and \
                 this record names nobody it could be measured against. \
                 Sources and places are edited by accounts with access to the \
                 whole tree."
            } else {
                "Your account is restricted to one branch of the tree, and \
                 this record concerns somebody outside it. Every person a \
                 record names has to be inside your branch — a family with one \
                 partner from outside would otherwise be a way to rewrite that \
                 person's parentage."
            },
        )),
    }
}

// ---------------------------------------------------------------------------
// auth
// ---------------------------------------------------------------------------

/// `GET /admin/login`
pub async fn login_form(State(state): State<Shared>, headers: HeaderMap) -> Response {
    let viewer = auth::viewer(&state, &headers);
    if viewer.may_write() {
        return Redirect::to("/admin").into_response();
    }
    render::page(
        "admin_login.html",
        context! {
            nav => "admin",
            is_admin => false,
            error => "",
            // A fresh installation has no accounts at all. Saying so beats a
            // login form that cannot be satisfied.
            no_accounts => state.acl_read(|a| a.users.is_empty()),
        },
    )
}

#[derive(Deserialize)]
pub struct LoginForm {
    #[serde(default)]
    username: String,
    #[serde(default)]
    password: String,
    /// The emergency shared token, on its own field.
    #[serde(default)]
    token: String,
}

/// `POST /admin/login`
///
/// Throttled on two keys: the username, so one account cannot be ground
/// through a dictionary, and the client address, so one client cannot grind
/// through the accounts. Either being exhausted refuses the attempt.
///
/// The reply is the same for an unknown username, a wrong password and a
/// disabled account. Distinguishing them would turn this form into an oracle
/// for which accounts exist, and there is no self-registration here to make
/// that knowledge harmless.
pub async fn login(
    State(state): State<Shared>,
    headers: HeaderMap,
    peer: Option<axum::extract::ConnectInfo<std::net::SocketAddr>>,
    Form(f): Form<LoginForm>,
) -> Response {
    let secure = crate::session::is_tls(&headers);
    let client = crate::session::client_key(&headers, peer.map(|c| c.0));

    // The emergency token, kept as a recovery path and nothing more.
    if !f.token.is_empty() {
        if !state.admin_token().is_empty()
            && crate::session::constant_time_eq(f.token.as_bytes(), state.admin_token().as_bytes())
        {
            let cookie = state.sessions().open(None, true);
            tracing::warn!(
                client = %client,
                "emergency admin token used to open a session; this is the \
                 recovery path, not an account"
            );
            return (
                [(
                    header::SET_COOKIE,
                    crate::session::set_cookie(&cookie, secure),
                )],
                Redirect::to("/admin"),
            )
                .into_response();
        }
        state.sessions().record_failure(&client);
        return login_refused(&state, "That token is not correct.");
    }

    let username = f.username.trim().to_ascii_lowercase();
    if state.sessions().is_throttled(&client) || state.sessions().is_throttled(&username) {
        return login_refused(
            &state,
            "Too many failed attempts. Wait a few minutes and try again.",
        );
    }

    let user = state.acl_read(|acl| acl.active(&username).cloned());
    let ok = match &user {
        Some(u) => crate::acl::verify_password(&f.password, &u.password_hash),
        // Hash anyway against a throwaway so an unknown username costs the
        // same wall-clock time as a known one. Argon2id at these parameters
        // takes long enough that skipping it would be plainly measurable.
        None => {
            let _ = crate::acl::verify_password(&f.password, crate::acl::DUMMY_HASH);
            false
        }
    };

    let Some(user) = user.filter(|_| ok) else {
        state.sessions().record_failure(&client);
        if !username.is_empty() {
            state.sessions().record_failure(&username);
        }
        return login_refused(&state, "That username and password do not match.");
    };

    state.sessions().clear_failures(&client);
    state.sessions().clear_failures(&username);

    let id = user.id.clone();
    if let Err(e) = state.acl_mutate(|acl| {
        if let Some(u) = acl.users.iter_mut().find(|u| u.id == id) {
            u.last_login = Some(view::now_iso8601());
        }
    }) {
        // The sign-in itself succeeded; only the timestamp did not persist.
        // Refusing the login over that would be the wrong trade.
        tracing::warn!(error = %e, "could not record last_login");
    }

    let cookie = state.sessions().open(Some(user.id.clone()), false);
    tracing::info!(username = %user.username, role = user.role.as_str(), "signed in");
    (
        [(
            header::SET_COOKIE,
            crate::session::set_cookie(&cookie, secure),
        )],
        Redirect::to("/admin"),
    )
        .into_response()
}

fn login_refused(state: &Shared, error: &str) -> Response {
    (
        StatusCode::UNAUTHORIZED,
        render::page(
            "admin_login.html",
            context! {
                nav => "admin",
                is_admin => false,
                error,
                no_accounts => state.acl_read(|a| a.users.is_empty()),
            },
        ),
    )
        .into_response()
}

/// `POST /admin/logout`
pub async fn logout(State(state): State<Shared>, headers: HeaderMap) -> Response {
    if let Some(cookie) = crate::session::cookie_value(&headers) {
        state.sessions().close(&cookie);
    }
    (
        [
            (header::SET_COOKIE, crate::session::clear_cookie()),
            // The emergency cookie too, so "sign out" means it.
            (header::SET_COOKIE, auth::clear_cookie()),
        ],
        Redirect::to("/"),
    )
        .into_response()
}

// ---------------------------------------------------------------------------
// dashboard
// ---------------------------------------------------------------------------

/// `GET /admin`
pub async fn dashboard(State(state): State<Shared>, headers: HeaderMap) -> Response {
    guard!(state, headers);
    let counts = state.counts();
    let env = state.inspect_with(axgf_rs::validate);
    let diagnostics = diagnostics_json(&env.diagnostics);
    // Validation says what is wrong; this says what is missing.
    let completeness = state.read(crate::completeness::analyse);

    render::page(
        "admin_dashboard.html",
        context! {
            nav => "admin",
            is_admin => true,
            kinds => KINDS,
            // COLLECTIONS and KINDS are the same eight in the same order, so
            // each tile can link to its singular admin listing.
            counts => counts.iter().zip(KINDS.iter())
                            .map(|((k, n), singular)| json!({
                                "kind": k, "n": n, "singular": singular }))
                            .collect::<Vec<_>>(),
            total => counts.iter().map(|(_, n)| n).sum::<usize>(),
            validation => env.data,
            diagnostics,
            bundle_path => state.bundle_path().display().to_string(),
            bundle_size => documents::human_size(state.bundle_size()),
            // Size on disk is no longer the resident cost — payloads are
            // streamed in and out and never held — but it is still what an
            // operator sizes a host and a backup against.
            bundle_heavy => state.bundle_size() > state.size_warn(),
            size_warn => documents::human_size(state.size_warn()),
            // The payloads live outside the flat JSON, so what the bundle
            // declares in `external_payloads` is the count of attached files.
            attachment_count => state.read(|flat| flat.get("external_payloads")
                .and_then(Value::as_object).map(|m| m.len()).unwrap_or(0)),
            completeness,
            recent_edits => state
                .journal()
                .recent(15)
                .into_iter()
                .map(|e| json!({
                    "at": e.at,
                    "who": e.who,
                    "action": e.action,
                    "kind": e.kind,
                    "entity_id": e.entity_id,
                    "label": e.label,
                    "version_num": e.version_num,
                    "summary": e.summary(),
                }))
                .collect::<Vec<_>>(),
            journal_len => state.journal().len(),
            journal_path => state.journal().path().display().to_string(),
            live_sessions => state.sessions().live(),
        },
    )
}

#[derive(Deserialize)]
pub struct ListQuery {
    #[serde(default = "one")]
    page: usize,
    #[serde(default)]
    q: String,
}
fn one() -> usize {
    1
}

/// `GET /admin/:kind` — a paginated, filterable listing.
pub async fn list(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(kind): Path<String>,
    Query(q): Query<ListQuery>,
) -> Response {
    guard!(state, headers);
    let Some(k) = kind_from_str(&kind) else {
        return unknown_kind(&kind);
    };

    let needle = q.q.trim().to_lowercase();
    let rows = state.read(|flat| {
        let empty = serde_json::Map::new();
        let coll = flat
            .get(k.collection())
            .and_then(Value::as_object)
            .unwrap_or(&empty);
        let mut rows: Vec<Value> = coll
            .iter()
            .map(|(id, e)| {
                json!({
                    "id": id,
                    "summary": summarize(k, e),
                    "confidence": e.get("confidence").and_then(Value::as_f64)
                                   .map(view::Confidence::new),
                })
            })
            .filter(|r| {
                needle.is_empty()
                    || r["summary"]
                        .as_str()
                        .unwrap_or("")
                        .to_lowercase()
                        .contains(&needle)
                    || r["id"]
                        .as_str()
                        .unwrap_or("")
                        .to_lowercase()
                        .contains(&needle)
            })
            .collect();
        rows.sort_by(|a, b| a["summary"].as_str().cmp(&b["summary"].as_str()));
        rows
    });

    let page = paginate(rows, q.page);
    render::page(
        "admin_list.html",
        context! {
            nav => "admin",
            is_admin => true,
            kind,
            kinds => KINDS,
            rows => page.items,
            page => page.page,
            pages => page.pages,
            total => page.total,
            per_page => page.per_page,
            q => q.q,
            is_person => kind == "person",
        },
    )
}

/// `GET /admin/:kind/new`
pub async fn new_form(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(kind): Path<String>,
) -> Response {
    guard!(state, headers);
    let Some(k) = kind_from_str(&kind) else {
        return unknown_kind(&kind);
    };
    render::page(
        "admin_form.html",
        context! {
            nav => "admin",
            is_admin => true,
            kind,
            kinds => KINDS,
            creating => true,
            id => "",
            fields => field_views(k, &Value::Object(Default::default())),
            raw => "{}",
            action => format!("/admin/{kind}"),
        },
    )
}

/// `GET /admin/:kind/:id/edit`
pub async fn edit_form(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path((kind, id)): Path<(String, String)>,
) -> Response {
    guard!(state, headers);
    let Some(k) = kind_from_str(&kind) else {
        return unknown_kind(&kind);
    };
    let entity = state.read(|flat| flat.get(k.collection()).and_then(|c| c.get(&id)).cloned());
    let Some(entity) = entity else {
        return render::error_page(
            StatusCode::NOT_FOUND,
            "No such entity",
            "This bundle contains no entity with that id.",
        );
    };

    render::page(
        "admin_form.html",
        context! {
            nav => "admin",
            is_admin => true,
            kind,
            kinds => KINDS,
            creating => false,
            id,
            fields => field_views(k, &entity),
            raw => serde_json::to_string_pretty(&entity).unwrap_or_else(|_| "{}".into()),
            action => format!("/admin/{kind}/{id}"),
            base_version => crate::state::version_of(&entity),
            history => history_json(&state, crate::state::kind_name(k), &id),
        },
    )
}

/// This entity's edit history, newest first, for the form and the record page.
fn history_json(state: &Shared, kind: &str, id: &str) -> Vec<Value> {
    state
        .journal()
        .for_entity(kind, id)
        .into_iter()
        .map(|e| {
            json!({
                "at": e.at,
                "who": e.who,
                "action": e.action,
                "version_num": e.version_num,
                "summary": e.summary(),
                "changes": e.changes,
            })
        })
        .collect()
}

/// `POST /admin/:kind` — create.
pub async fn create(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(kind): Path<String>,
    Form(form): Form<HashMap<String, String>>,
) -> Response {
    let viewer = guard!(state, headers);
    let Some(k) = kind_from_str(&kind) else {
        return unknown_kind(&kind);
    };

    let base = match base_from_raw(&form) {
        Ok(v) => v,
        Err(msg) => return form_error(&kind, None, &msg, &form, k),
    };
    let mut entity = apply_form(base, k, &form);
    if let Err(r) = check_scope(&state, &viewer, k, &entity, None) {
        return r;
    }
    // A new entity starts at version 1, so the first edit of it has a number
    // to check against. The library stores whatever it is given here and does
    // not mint one.
    entity["version_num"] = Value::from(1u64);
    entity["created_at"] = Value::from(view::now_iso8601());
    entity["updated_at"] = Value::from(view::now_iso8601());
    let body = entity.to_string();

    let out = match state.mutate(|flat| axgf_rs::add_entity(flat, k, &body)) {
        Ok(o) => o,
        Err(e) => return io_error(&e),
    };

    let new_id = out
        .data
        .get("id")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();

    if out.applied && !new_id.is_empty() {
        state.journal_mutation(&crate::journal::entry_for(crate::journal::Record {
            who: viewer.name(),
            action: "create",
            kind: crate::state::kind_name(k),
            entity_id: &new_id,
            label: label_for(k, &entity),
            version_num: Some(1),
            before: None,
            after: None,
        }));
    }

    result_page(
        &kind,
        if out.applied {
            "Created"
        } else {
            "Not created"
        },
        &out,
        if out.applied && !new_id.is_empty() {
            Some(format!("/admin/{kind}/{new_id}/edit"))
        } else {
            None
        },
    )
}

/// `POST /admin/:kind/:id` — update.
pub async fn update(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path((kind, id)): Path<(String, String)>,
    Form(form): Form<HashMap<String, String>>,
) -> Response {
    let viewer = guard!(state, headers);
    let Some(k) = kind_from_str(&kind) else {
        return unknown_kind(&kind);
    };

    let base = match base_from_raw(&form) {
        Ok(v) => v,
        Err(msg) => return form_error(&kind, Some(&id), &msg, &form, k),
    };
    let mut entity = apply_form(base, k, &form);
    // The id in the path is authoritative; a raw-JSON edit must not silently
    // retarget the update at a different entity.
    entity["id"] = Value::String(id.clone());
    let stored = state.read(|flat| {
        flat.get(crate::admin::collection_for(k))
            .and_then(|c| c.get(&id))
            .cloned()
    });
    if let Err(r) = check_scope(&state, &viewer, k, &entity, stored.as_ref()) {
        return r;
    }

    // The version this form was rendered from. An absent field means a form
    // from before this existed, or a script posting by hand; falling back to
    // the stored version would make the check pass by default, which is the
    // one thing it must never do. Falling back to a value that cannot match
    // makes it fail closed and show the conflict page, which is a bad
    // experience and a correct one.
    let base_version: u64 = form
        .get("base_version")
        .and_then(|v| v.trim().parse().ok())
        .unwrap_or(u64::MAX);

    let label = stored.as_ref().and_then(|e| label_for(k, e));
    let outcome =
        match state.update_checked(k, &id, base_version, entity.clone(), viewer.name(), label) {
            Ok(o) => o,
            Err(e) => return io_error(&e),
        };

    match outcome {
        crate::state::UpdateOutcome::Applied {
            diagnostics,
            version_num,
            changes,
        } => result_page(
            &kind,
            &format!(
                "Saved as version {version_num} — {}",
                crate::diff::summarise(&changes)
            ),
            &MutationOutcome {
                applied: true,
                diagnostics,
                data: Value::Null,
            },
            Some(format!("/admin/{kind}/{id}/edit")),
        ),
        crate::state::UpdateOutcome::Refused { diagnostics } => result_page(
            &kind,
            "Not saved",
            &MutationOutcome {
                applied: false,
                diagnostics,
                data: Value::Null,
            },
            Some(format!("/admin/{kind}/{id}/edit")),
        ),
        crate::state::UpdateOutcome::Missing => render::error_page(
            StatusCode::NOT_FOUND,
            "No such entity",
            "This bundle contains no entity with that id. It may have been \
             deleted while you were editing it.",
        ),
        crate::state::UpdateOutcome::Conflict {
            current,
            current_version,
            expected_version,
        } => conflict_page(
            &state,
            &kind,
            k,
            &id,
            stored.as_ref(),
            &current,
            &entity,
            current_version,
            expected_version,
        ),
    }
}

/// The page shown when somebody else changed the record first.
///
/// It never merges. A silent merge produces a record no human chose, and for
/// a genealogy — where two editors disagreeing about a date usually means they
/// are reading different sources — that is worse than asking. So this shows
/// the three versions side by side and makes the editor decide.
///
/// `base` is the version the editor started from, when the bundle still has
/// it to show. After a conflict it usually does not — the stored entity *is*
/// the other person's version now — so the column that can always be filled is
/// "theirs" against "yours", and the base is used to say which fields each of
/// them actually touched.
#[allow(clippy::too_many_arguments)]
fn conflict_page(
    state: &Shared,
    kind: &str,
    k: axgf_rs::EntityKind,
    id: &str,
    base: Option<&Value>,
    current: &Value,
    mine: &Value,
    current_version: u64,
    expected_version: u64,
) -> Response {
    // Who moved it, and when. The journal knows; the entity's own `updated_at`
    // is the fallback for an edit made before journalling or by another tool.
    let last = state.journal().last_touched(crate::state::kind_name(k), id);
    let (who, when) = match &last {
        Some(e) => (e.who.clone(), e.at.clone()),
        None => (
            "somebody".to_string(),
            current
                .get("updated_at")
                .and_then(Value::as_str)
                .unwrap_or("an unrecorded time")
                .to_string(),
        ),
    };

    // The version this editor started from. The bundle holds only the current
    // one, so it is reconstructed by replaying the journal backwards from
    // `current`. Without it the page could only say "here is theirs, here is
    // yours"; with it, it can say which of you changed what.
    let entries = state.journal().for_entity(crate::state::kind_name(k), id);
    let rebuilt = crate::journal::rewind(current, &entries, expected_version, current_version);
    let base_ref = rebuilt.as_ref().or(base).unwrap_or(current);
    let reconstructed = rebuilt.is_some();

    // Three diffs, because three questions. What did they change, what did I
    // change, and where do we actually disagree?
    let theirs = crate::diff::diff(base_ref, current);
    let ours = crate::diff::diff(base_ref, mine);
    let contested: Vec<String> = theirs
        .iter()
        .filter(|t| ours.iter().any(|o| o.path == t.path))
        .map(|t| t.path.clone())
        .collect();

    let mut resubmit = mine.clone();
    // The re-apply form carries the editor's own text forward against the
    // version that is now current, so accepting the conflict is one click and
    // not a retype.
    resubmit["version_num"] = Value::from(current_version);

    (
        StatusCode::CONFLICT,
        render::page(
            "admin_conflict.html",
            context! {
                nav => "admin",
                is_admin => true,
                kind,
                kinds => KINDS,
                id,
                who,
                when,
                current_version,
                expected_version,
                theirs,
                ours,
                contested,
                had_base => reconstructed,
                mine_raw => serde_json::to_string_pretty(&resubmit)
                    .unwrap_or_else(|_| "{}".into()),
                current_raw => serde_json::to_string_pretty(current)
                    .unwrap_or_else(|_| "{}".into()),
                action => format!("/admin/{kind}/{id}"),
                history => history_json(state, crate::state::kind_name(k), id),
            },
        ),
    )
        .into_response()
}

/// A short human label for an entity, for the journal listing.
fn label_for(kind: axgf_rs::EntityKind, entity: &Value) -> Option<String> {
    match kind {
        axgf_rs::EntityKind::Person => Some(view::person_display_name(entity)),
        _ => entity
            .get("title")
            .or_else(|| entity.get("label"))
            .or_else(|| entity.get("name"))
            .and_then(Value::as_str)
            .map(str::to_string),
    }
}

#[derive(Deserialize)]
pub struct DeleteForm {
    #[serde(default)]
    policy: String,
}

/// `POST /admin/:kind/:id/delete`
pub async fn delete(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path((kind, id)): Path<(String, String)>,
    Form(f): Form<DeleteForm>,
) -> Response {
    let viewer = guard_admin!(state, headers);
    let Some(k) = kind_from_str(&kind) else {
        return unknown_kind(&kind);
    };
    // Deleting is an admin's right, and a scope is still a scope: an admin
    // who has one set may not delete outside it. Normally there is none, and
    // this costs one `None` check.
    let stored = state.read(|flat| {
        flat.get(crate::admin::collection_for(k))
            .and_then(|c| c.get(&id))
            .cloned()
    });
    if let Some(stored) = stored.as_ref() {
        if let Err(r) = check_scope(&state, &viewer, k, stored, None) {
            return r;
        }
    }
    let policy = policy_from_str(&f.policy);

    let out = match state.mutate(|flat| axgf_rs::delete_entity(flat, k, &id, policy)) {
        Ok(o) => o,
        Err(e) => return io_error(&e),
    };

    if out.applied {
        state.journal_mutation(&crate::journal::entry_for(crate::journal::Record {
            who: viewer.name(),
            action: "delete",
            kind: crate::state::kind_name(k),
            entity_id: &id,
            label: stored.as_ref().and_then(|e| label_for(k, e)),
            version_num: None,
            before: None,
            after: None,
        }));
    }

    result_page(
        &kind,
        if out.applied {
            "Deleted"
        } else {
            "Not deleted — the bundle is unchanged"
        },
        &out,
        Some(format!("/admin/{kind}")),
    )
}

/// `POST /admin/validate`
pub async fn validate(State(state): State<Shared>, headers: HeaderMap) -> Response {
    guard_admin!(state, headers);
    let env = state.inspect_with(axgf_rs::validate);
    render::page(
        "admin_result.html",
        context! {
            nav => "admin",
            is_admin => true,
            title => "Validation report",
            summary => summary_line(&env.data, &[
                ("errors", "error"), ("warnings", "warning"), ("infos", "note")]),
            diagnostics => diagnostics_json(&env.diagnostics),
            back => "/admin",
            applied => true,
        },
    )
}

/// `POST /admin/dedup`
pub async fn dedup(State(state): State<Shared>, headers: HeaderMap) -> Response {
    guard_admin!(state, headers);
    let out = match state.mutate(axgf_rs::deduplicate) {
        Ok(o) => o,
        Err(e) => return io_error(&e),
    };

    let summary = summary_line(
        &out.data,
        &[
            ("merged_persons", "person merged"),
            ("merged_families", "family merged"),
            ("manual_review", "case left for manual review"),
        ],
    );

    render::page(
        "admin_result.html",
        context! {
            nav => "admin",
            is_admin => true,
            title => if out.applied { "Deduplication complete" } else { "Deduplication refused" },
            summary,
            diagnostics => diagnostics_json(&out.diagnostics),
            back => "/admin",
            applied => out.applied,
        },
    )
}

/// `GET /admin/export`
///
/// Streams the bundle rather than building it in memory: the archive is written
/// to a temp file one payload at a time, then sent from that file. Downloading a
/// 400 MiB bundle costs a file handle, not 400 MiB of process.
pub async fn export(State(state): State<Shared>, headers: HeaderMap) -> Response {
    guard_admin!(state, headers);
    let tmp = match state.export_to_temp_file() {
        Ok(t) => t,
        Err(e) => return io_error(&e),
    };
    let name = state
        .bundle_path()
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| "family.axgf".into());
    render::bundle_download_from(&name, tmp).await
}

// ---------------------------------------------------------------------------
// helpers
// ---------------------------------------------------------------------------

/// Parse the raw-JSON textarea, if the form carried one.
fn base_from_raw(form: &HashMap<String, String>) -> Result<Value, String> {
    match form.get("raw_json").map(String::as_str).map(str::trim) {
        None | Some("") => Ok(Value::Object(Default::default())),
        Some(s) => serde_json::from_str::<Value>(s)
            .map_err(|e| format!("The raw JSON did not parse: {e}. Nothing was saved.")),
    }
}

/// Field descriptors with their current values, for the form template.
fn field_views(kind: axgf_rs::EntityKind, entity: &Value) -> Vec<Value> {
    fields_for(kind)
        .iter()
        .map(|f| {
            let current = get_path(entity, f.path);
            json!({
                "path": f.path,
                "label": f.label,
                "kind": f.kind,
                "hint": f.hint,
                "options": f.options,
                "value": current,
                "checked": current == "true",
            })
        })
        .collect()
}

/// A one-line human summary of an entity, for listings.
fn summarize(kind: axgf_rs::EntityKind, e: &Value) -> String {
    use axgf_rs::EntityKind as K;
    let s = |k: &str| e.get(k).and_then(Value::as_str).unwrap_or("");
    match kind {
        K::Person => view::person_display_name(e),
        K::Family => {
            let n = s("name");
            if n.is_empty() {
                let kids = e
                    .get("children")
                    .and_then(Value::as_array)
                    .map(|a| a.len())
                    .unwrap_or(0);
                format!("(unnamed family, {kids} children)")
            } else {
                n.to_string()
            }
        }
        K::Event => {
            let c = s("category");
            let d = view::render_date_field(e, "date");
            format!("{c} — {}", d.text)
        }
        K::Link => format!("{} → {}", s("label"), s("label_reverse")),
        K::Occupation => s("title").to_string(),
        K::Source => format!("{} ({})", s("title"), s("reliability")),
        K::Place => view::place_name(e),
        K::Document => format!("{} — {}", s("filename"), s("document_type")),
    }
}

/// Diagnostics as plain JSON for the templates.
fn diagnostics_json(diags: &[axgf_rs::boundary::envelope::Diagnostic]) -> Vec<Value> {
    diags
        .iter()
        .map(|d| {
            json!({
                "code": d.code.as_str(),
                "severity": format!("{:?}", d.severity).to_lowercase(),
                "message": d.message,
                "entity_ref": d.entity_ref,
            })
        })
        .collect()
}

/// Turn numeric fields of an envelope's data into "2 merged, 1 left" prose.
fn summary_line(data: &Value, fields: &[(&str, &str)]) -> String {
    let parts: Vec<String> = fields
        .iter()
        .filter_map(|(key, noun)| {
            let n = data.get(key).and_then(Value::as_u64)?;
            Some(format!(
                "{n} {noun}{}",
                if n == 1 {
                    ""
                } else if noun.ends_with('h') {
                    "es"
                } else {
                    "s"
                }
            ))
        })
        .collect();
    if parts.is_empty() {
        "Nothing to report.".into()
    } else {
        parts.join(", ")
    }
}

/// The page shown after a mutation.
fn result_page(kind: &str, title: &str, out: &MutationOutcome, back: Option<String>) -> Response {
    render::page(
        "admin_result.html",
        context! {
            nav => "admin",
            is_admin => true,
            title,
            summary => if out.applied {
                "The bundle was written to disk."
            } else {
                "The library refused this operation. The bundle on disk is unchanged."
            },
            diagnostics => diagnostics_json(&out.diagnostics),
            back => back.unwrap_or_else(|| format!("/admin/{kind}")),
            applied => out.applied,
        },
    )
}

/// Re-render a form after a client-side error, keeping what was typed.
fn form_error(
    kind: &str,
    id: Option<&str>,
    message: &str,
    form: &HashMap<String, String>,
    k: axgf_rs::EntityKind,
) -> Response {
    let raw = form.get("raw_json").cloned().unwrap_or_else(|| "{}".into());
    let entity = serde_json::from_str::<Value>(&raw).unwrap_or(Value::Object(Default::default()));
    let mut resp = render::page(
        "admin_form.html",
        context! {
            nav => "admin",
            is_admin => true,
            kind,
            kinds => KINDS,
            creating => id.is_none(),
            id => id.unwrap_or(""),
            fields => field_views(k, &entity),
            raw,
            error => message,
            action => match id {
                Some(i) => format!("/admin/{kind}/{i}"),
                None => format!("/admin/{kind}"),
            },
        },
    );
    *resp.status_mut() = StatusCode::BAD_REQUEST;
    resp
}

fn unknown_kind(kind: &str) -> Response {
    render::error_page(
        StatusCode::NOT_FOUND,
        "Unknown entity kind",
        &format!("“{kind}” is not one of: {}.", KINDS.join(", ")),
    )
}

fn io_error(e: &anyhow::Error) -> Response {
    tracing::error!(error = %e, "admin operation failed");
    render::error_page(
        StatusCode::INTERNAL_SERVER_ERROR,
        "The bundle could not be written",
        &format!("{e}. The previous bundle is intact."),
    )
}

// ---------------------------------------------------------------------------
// document upload
// ---------------------------------------------------------------------------

/// Fields pulled out of the document upload form.
#[derive(Default)]
struct DocUpload {
    filename: String,
    bytes: Vec<u8>,
    document_type: String,
    caption: String,
    /// Set when the body limit fired while reading, so the handler can answer
    /// 413 instead of "no file was chosen".
    too_large: bool,
}

/// `POST /admin/person/:id/document` — attach a file to a person.
///
/// The bytes go into the flat bundle's `attachments` map and the metadata into
/// a Document entity, both inside one call to
/// [`AppState::mutate_and_adjust`][crate::state::AppState::mutate_and_adjust]
/// so that a single atomic write carries both. `export_bundle` then puts the
/// file back at its ZIP path on the way out — no change to `axgf-rs` was
/// needed for any of this.
pub async fn upload_document(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(id): Path<String>,
    multipart: axum::extract::Multipart,
) -> Response {
    let viewer = guard!(state, headers);
    // The subject is the person the file is being attached to. Uploading a
    // document is a write against *their* record whatever the Document entity
    // itself says, so it is checked as one.
    if let Err(r) = check_scope(
        &state,
        &viewer,
        axgf_rs::EntityKind::Person,
        &json!({"id": id}),
        None,
    ) {
        return r;
    }

    let person_exists = state.read(|flat| {
        flat.get("persons")
            .and_then(|p| p.get(&id))
            .is_some_and(|p| !p.is_null())
    });
    if !person_exists {
        return render::error_page(
            StatusCode::NOT_FOUND,
            "No such person",
            "This bundle contains no person with that id, so there is nothing \
             to attach a document to.",
        );
    }

    let up = read_document_upload(multipart).await;

    if up.too_large || up.bytes.len() > documents::MAX_UPLOAD {
        return upload_refused(
            &id,
            StatusCode::PAYLOAD_TOO_LARGE,
            &format!(
                "That file is larger than the {} MB limit. Nothing was stored, \
                 and the bundle is unchanged.",
                documents::MAX_UPLOAD / (1024 * 1024)
            ),
        );
    }
    if up.bytes.is_empty() {
        return upload_refused(
            &id,
            StatusCode::BAD_REQUEST,
            "No file was uploaded. Choose a file first.",
        );
    }

    // The filename and the client's Content-Type are both attacker-controlled,
    // so neither is consulted: the type comes from the bytes.
    let Some(kind) = documents::sniff(&up.bytes) else {
        return upload_refused(
            &id,
            StatusCode::UNSUPPORTED_MEDIA_TYPE,
            "That file is not a type this archive stores. Images, PDF, plain \
             text, audio and video are accepted; the type is read from the \
             file's own bytes, so renaming an executable does not get it in. \
             SVG is refused outright, because an SVG can carry script.",
        );
    };

    let sha256 = documents::sha256_hex(&up.bytes);
    let size = up.bytes.len() as u64;
    let filename = clean_filename(&up.filename, kind.ext);
    let doc_type = if up.document_type.trim().is_empty() {
        if kind.raster_image {
            "photo".to_string()
        } else {
            "other".to_string()
        }
    } else {
        up.document_type.trim().to_string()
    };

    // The Document is created without its `file.path`, because the path
    // contains the id the library is about to mint. The adjust step fills in
    // the path and stores the payload beside it, inside the same write.
    let mut entity = json!({
        "type": "document",
        "axgf_version": "1.0",
        "filename": filename,
        "mime_type": kind.mime,
        "document_type": doc_type,
        "status": "present",
        "file": {"size_bytes": size, "sha256": sha256},
        "linked_to": [{"entity_type": "person", "entity_id": id, "role": "subject"}],
    });
    if !up.caption.trim().is_empty() {
        entity["caption"] = json!(up.caption.trim());
    }
    let body = entity.to_string();

    // The payload goes straight to the disk cache, never into the in-memory
    // bundle; add_document mints the id, fills in the file path, and persists.
    let (out, new_id) = match state.add_document(&body, &up.bytes, kind.ext) {
        Ok(o) => o,
        Err(e) => return io_error(&e),
    };

    if out.applied {
        // Journalled against the *person*, not the document: "Anna attached a
        // photograph to grandmother's record" is what somebody reading the
        // history is looking for, and a document id on its own is not that.
        state.journal_mutation(&crate::journal::entry_for(crate::journal::Record {
            who: viewer.name(),
            action: "upload",
            kind: "person",
            entity_id: &id,
            label: new_id.clone(),
            version_num: None,
            before: None,
            after: None,
        }));
    }

    if !out.applied {
        return upload_refused(
            &id,
            StatusCode::BAD_REQUEST,
            &format!(
                "The library refused the document: {}. The bundle is unchanged.",
                crate::state::format_diagnostics(&out.diagnostics)
            ),
        );
    }

    Redirect::to(&format!("/person/{id}#evidence")).into_response()
}

/// Refuse an upload with a reason and a link back to the person.
///
/// The status carries the distinction a script needs — 413 for too big, 415
/// for a type this archive does not store — and the body carries the sentence
/// a person needs.
fn upload_refused(person: &str, status: StatusCode, message: &str) -> Response {
    render::error_page_back(
        status,
        "That upload was not stored",
        message,
        Some((&format!("/person/{person}"), "Back to this person")),
    )
}

/// A filename safe to store and to echo back into a header.
///
/// Path separators and quotes are stripped rather than escaped, and the
/// extension is forced to match what the bytes actually are, so a file called
/// `holiday.jpg` that is really a PDF is stored as `holiday.pdf`.
fn clean_filename(raw: &str, ext: &str) -> String {
    let base = raw
        .rsplit(['/', '\\'])
        .next()
        .unwrap_or(raw)
        .trim()
        .trim_matches('.');
    let stem = base.rsplit_once('.').map(|(s, _)| s).unwrap_or(base);
    let stem: String = stem
        .chars()
        .filter(|c| !c.is_control() && !matches!(c, '"' | '\\' | '/'))
        .take(120)
        .collect();
    let stem = stem.trim();
    if stem.is_empty() {
        format!("upload.{ext}")
    } else {
        format!("{stem}.{ext}")
    }
}

/// Read the upload form, tolerating fields in any order.
async fn read_document_upload(mut multipart: axum::extract::Multipart) -> DocUpload {
    let mut out = DocUpload::default();
    loop {
        let field = match multipart.next_field().await {
            Ok(Some(f)) => f,
            Ok(None) => break,
            Err(e) => {
                // The body-limit layer surfaces as an error here, and it is
                // the one failure worth telling apart from a malformed body.
                out.too_large = e.status() == StatusCode::PAYLOAD_TOO_LARGE;
                break;
            }
        };
        match field.name().unwrap_or_default().to_string().as_str() {
            "file" => {
                out.filename = field.file_name().unwrap_or("upload").to_string();
                match field.bytes().await {
                    Ok(b) => out.bytes = b.to_vec(),
                    Err(_) => out.too_large = true,
                }
            }
            "document_type" => out.document_type = field.text().await.unwrap_or_default(),
            "caption" => out.caption = field.text().await.unwrap_or_default(),
            _ => {
                let _ = field.bytes().await;
            }
        }
    }
    out
}

// ---------------------------------------------------------------------------
// accounts
// ---------------------------------------------------------------------------

/// `GET /admin/users` — the account list.
///
/// Admin-only, and the only place accounts are created: there is no
/// self-registration and no invitation flow in this release. For a family CMS
/// an administrator who knows everyone is sufficient, and it removes an entire
/// abuse surface — open registration, invitation tokens, email delivery and
/// the account-enumeration oracle each of those carries — rather than
/// defending it.
pub async fn users(State(state): State<Shared>, headers: HeaderMap) -> Response {
    let viewer = guard_admin!(state, headers);
    render_users(&state, &viewer, None, None)
}

fn render_users(
    state: &Shared,
    viewer: &Viewer,
    error: Option<&str>,
    notice: Option<&str>,
) -> Response {
    let roster = state.read(|flat| {
        flat.get("persons")
            .and_then(Value::as_object)
            .map(|persons| {
                let mut out: Vec<(String, String)> = persons
                    .iter()
                    .map(|(id, p)| (view::person_display_name(p), id.clone()))
                    .collect();
                out.sort();
                out.into_iter()
                    .map(|(name, id)| json!({"id": id, "name": name}))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default()
    });

    let users = state.acl_read(|acl| {
        acl.users
            .iter()
            .map(|u| {
                json!({
                    "id": u.id,
                    "username": u.username,
                    "email": u.email,
                    "role": u.role.as_str(),
                    "status": u.status.as_str(),
                    "created_at": u.created_at,
                    "last_login": u.last_login,
                    "family_scope": u.family_scope,
                    // Never the hash, not even truncated. It has no business
                    // in a template context that a future edit might print.
                    "is_me": Some(&u.id) == viewer.user.as_ref().map(|m| &m.id),
                })
            })
            .collect::<Vec<_>>()
    });
    let admins = state.acl_read(|acl| acl.active_admins());

    render::page(
        "admin_users.html",
        context! {
            nav => "admin",
            is_admin => true,
            users,
            roster,
            admins,
            error,
            notice,
            emergency => viewer.emergency,
            acl_path => state.acl_path().display().to_string(),
            min_password => crate::acl::MIN_PASSWORD,
        },
    )
}

#[derive(Deserialize)]
pub struct NewUserForm {
    #[serde(default)]
    username: String,
    #[serde(default)]
    password: String,
    #[serde(default)]
    email: String,
    #[serde(default)]
    role: String,
    #[serde(default)]
    family_scope: String,
}

/// `POST /admin/users` — create an account.
pub async fn create_user(
    State(state): State<Shared>,
    headers: HeaderMap,
    Form(f): Form<NewUserForm>,
) -> Response {
    let viewer = guard_admin!(state, headers);

    let role = match crate::acl::Role::parse(&f.role) {
        Some(r) => r,
        None => return render_users(&state, &viewer, Some("Pick a role."), None),
    };
    // A generated password when the field is left blank, so the common case —
    // an administrator setting up a relative — never invites a weak one.
    let generated = f.password.trim().is_empty();
    let password = if generated {
        crate::acl::generate_password()
    } else {
        f.password.clone()
    };

    let mut user = match crate::acl::new_user(&f.username, &password, role) {
        Ok(u) => u,
        Err(e) => return render_users(&state, &viewer, Some(&e.to_string()), None),
    };
    if state.acl_read(|a| a.has_username(&user.username)) {
        return render_users(&state, &viewer, Some("That username is taken."), None);
    }
    let email = f.email.trim();
    if !email.is_empty() {
        user.email = Some(email.to_string());
    }
    user.family_scope = parse_scope(&f.family_scope);

    let username = user.username.clone();
    if let Err(e) = state.acl_mutate(|acl| acl.users.push(user)) {
        return render_users(&state, &viewer, Some(&format!("Not saved: {e}")), None);
    }

    let notice = if generated {
        format!(
            "Created {username}. Their password is {password} — it is shown \
             once and stored only as an Argon2id hash, so pass it on now."
        )
    } else {
        format!("Created {username}.")
    };
    render_users(&state, &viewer, None, Some(&notice))
}

#[derive(Deserialize)]
pub struct EditUserForm {
    #[serde(default)]
    role: String,
    #[serde(default)]
    status: String,
    #[serde(default)]
    email: String,
    #[serde(default)]
    family_scope: String,
    /// Blank leaves the password alone.
    #[serde(default)]
    password: String,
}

/// `POST /admin/users/:id` — change a role, a scope, a status or a password.
///
/// Every one of those is a change to what a live cookie grants, so each of
/// them closes that account's sessions. A demoted admin who kept their rights
/// until they happened to sign out would make the demotion advisory.
pub async fn update_user(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(id): Path<String>,
    Form(f): Form<EditUserForm>,
) -> Response {
    let viewer = guard_admin!(state, headers);

    let Some(existing) = state.acl_read(|a| a.by_id(&id).cloned()) else {
        return render_users(&state, &viewer, Some("No such account."), None);
    };
    let role = crate::acl::Role::parse(&f.role).unwrap_or(existing.role);
    let disabling = f.status == "disabled";

    // The last active administrator may not remove their own way back in.
    // Losing every admin means the .acl has to be edited by hand or the
    // emergency token used, and neither should be the result of a stray click.
    let lowering_last_admin = existing.role == crate::acl::Role::Admin
        && existing.status == crate::acl::Status::Active
        && (role != crate::acl::Role::Admin || disabling)
        && state.acl_read(|a| a.active_admins()) <= 1;
    if lowering_last_admin {
        return render_users(
            &state,
            &viewer,
            Some(
                "That is the only active administrator. Promote somebody else \
                 first — an installation with no administrator can only be \
                 recovered by editing the .acl file or using the emergency \
                 token.",
            ),
            None,
        );
    }

    let new_password = f.password.trim().to_string();
    if !new_password.is_empty() {
        if let Err(e) = crate::acl::validate_password(&new_password) {
            return render_users(&state, &viewer, Some(&e.to_string()), None);
        }
    }
    let hash = if new_password.is_empty() {
        None
    } else {
        match crate::acl::hash_password(&new_password) {
            Ok(h) => Some(h),
            Err(e) => return render_users(&state, &viewer, Some(&e.to_string()), None),
        }
    };

    let email = f.email.trim().to_string();
    let scope = parse_scope(&f.family_scope);
    let id2 = id.clone();
    if let Err(e) = state.acl_mutate(move |acl| {
        if let Some(u) = acl.users.iter_mut().find(|u| u.id == id2) {
            u.role = role;
            u.status = if disabling {
                crate::acl::Status::Disabled
            } else {
                crate::acl::Status::Active
            };
            u.email = (!email.is_empty()).then_some(email);
            u.family_scope = scope;
            if let Some(h) = hash {
                u.password_hash = h;
            }
        }
    }) {
        return render_users(&state, &viewer, Some(&format!("Not saved: {e}")), None);
    }

    // Whatever changed, the cookie that was issued before it is now describing
    // rights the account no longer has.
    state.sessions().close_all_for(&id);
    render_users(
        &state,
        &viewer,
        None,
        Some(&format!(
            "Updated {}. Any session it had open has been signed out.",
            existing.username
        )),
    )
}

/// Parse the scope textarea: one person id per line, blanks ignored.
fn parse_scope(raw: &str) -> Vec<String> {
    raw.split(['\n', ',', ' '])
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .collect()
}
