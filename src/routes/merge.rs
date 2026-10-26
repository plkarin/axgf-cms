//! The merge screen: two records a person says are one.
//!
//! The listing's bulk bar sends two ticked persons, families, places or
//! sources here. The screen shows the library's plan (`axgf_rs::plan_merge`):
//! what both hold alike or only one holds, each field they disagree on with
//! the choices it allows and the library's suggestion, what else changes, and
//! any refusal. Nothing changes until the administrator confirms; the merge
//! itself is the library's (`axgf_rs::merge_entities`), with every
//! disagreement decided here and nothing chosen silently. The absorbed record
//! goes to the journal field by field, so that nothing is lost, and an old
//! address of a merged person says where it went (B7).
//!
//! This crate does no genealogy: which fields conflict, what is refused and
//! why, and what a value not chosen becomes are all the library's answers.

use axum::extract::{Form, Path, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Redirect, Response};
use minijinja::context;
use serde::Deserialize;
use serde_json::{json, Map, Value};

use super::admin::{check_scope, diagnostics_json, io_error, require, summarize, Need};
use super::Shared;
use crate::admin::{collection_for, kind_from_str};
use crate::render;
use axgf_rs::EntityKind;

/// The kinds the library merges.
pub(super) fn mergeable(kind: EntityKind) -> bool {
    matches!(
        kind,
        EntityKind::Person | EntityKind::Family | EntityKind::Place | EntityKind::Source
    )
}

/// Where a record of this kind is read and edited.
fn record_href(kind: &str, id: &str) -> String {
    if kind == "person" {
        format!("/person/{id}")
    } else {
        format!("/admin/{kind}/{id}/edit")
    }
}

fn plan_href(kind: &str, keep: &str, absorb: &str) -> String {
    format!("/admin/{kind}/merge?keep={keep}&absorb={absorb}")
}

/// The two ticked rows of a listing, in the order they were listed: the
/// first is offered as the one kept. Anything but two is refused with the
/// reason, rather than guessed at.
pub(super) fn from_bulk(chrome: &render::Chrome, kind: &str, ids: &[String]) -> Response {
    match ids {
        [keep, absorb] => Redirect::to(&plan_href(kind, keep, absorb)).into_response(),
        _ => render::error_page_back_in(
            chrome,
            StatusCode::BAD_REQUEST,
            "merge-title",
            "merge-needs-two",
            &[],
            Some((&format!("/admin/{kind}"), "admin-cancel")),
        ),
    }
}

#[derive(Deserialize)]
pub struct Pair {
    keep: String,
    absorb: String,
}

/// A value of either record, as a line a reader can compare: a date in the
/// reader's language, a referenced record by its name, a scalar as itself.
fn value_text(chrome: &render::Chrome, flat: &Value, v: &Value) -> String {
    fn named(flat: &Value, lang: &str, id: &str) -> Option<String> {
        for (coll, kind) in [
            ("persons", EntityKind::Person),
            ("families", EntityKind::Family),
            ("places", EntityKind::Place),
            ("sources", EntityKind::Source),
            ("events", EntityKind::Event),
            ("documents", EntityKind::Document),
        ] {
            if let Some(e) = flat.get(coll).and_then(|c| c.get(id)) {
                return Some(summarize(flat, lang, kind, e));
            }
        }
        None
    }
    fn walk(chrome: &render::Chrome, flat: &Value, key: &str, v: &Value, out: &mut Vec<String>) {
        let label = |text: String| {
            if key.is_empty() {
                text
            } else {
                format!("{key}: {text}")
            }
        };
        match v {
            Value::Null => {}
            Value::Bool(b) => {
                out.push(label(chrome.t(if *b { "record-yes" } else { "record-no" })))
            }
            Value::Number(n) => out.push(label(n.to_string())),
            Value::String(s) if key.ends_with("_id") => out.push(label(
                named(flat, chrome.lang, s).unwrap_or_else(|| s.clone()),
            )),
            Value::String(s) => out.push(label(s.clone())),
            Value::Array(items) => {
                for item in items {
                    walk(chrome, flat, key, item, out);
                }
            }
            Value::Object(m) => {
                // A date is read as a date; a name by its display.
                if m.contains_key("precision") || m.contains_key("range") {
                    let d = crate::view::render_date_in(v, chrome.lang);
                    out.push(label(d.text));
                    return;
                }
                if let Some(display) = m.get("display").and_then(Value::as_str) {
                    out.push(label(display.to_string()));
                    return;
                }
                for (k, inner) in m {
                    walk(chrome, flat, k, inner, out);
                }
            }
        }
    }
    let mut out = Vec::new();
    walk(chrome, flat, "", v, &mut out);
    if out.is_empty() {
        "—".to_string()
    } else {
        out.join(" · ")
    }
}

/// A field of a record, by the name the record page already gives it, or by
/// its path where it has none.
fn field_label(chrome: &render::Chrome, field: &str) -> String {
    let key = match field {
        "birth" => "event-category-birth",
        "death" => "event-category-death",
        "notes" => "record-notes",
        "identity.name" => "record-name",
        "identity.gender" => "record-gender",
        "identity.visibility" => "record-visibility",
        "identity.is_living" => "record-living",
        _ => return field.to_string(),
    };
    chrome.t(key)
}

/// A message the catalogue may not have for a value a newer library sends:
/// the value itself then.
fn keyed(chrome: &render::Chrome, prefix: &str, value: &str) -> String {
    let key = format!("{prefix}-{}", value.replace('_', "-"));
    if crate::i18n::has_message(crate::i18n::DEFAULT, &key) {
        chrome.t(&key)
    } else {
        value.to_string()
    }
}

/// `GET /admin/:kind/merge?keep=…&absorb=…` — the plan. Changes nothing.
pub async fn plan(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(kind): Path<String>,
    Query(pair): Query<Pair>,
) -> Response {
    let (viewer, chrome) = match require(&state, &headers, Need::Admin) {
        Ok(v) => v,
        Err(r) => return r,
    };
    let Some(k) = kind_from_str(&kind).filter(|k| mergeable(*k)) else {
        return render::error_page_in(
            &chrome,
            StatusCode::NOT_FOUND,
            "error-not-found-title",
            "error-not-found-detail",
        );
    };
    let collection = collection_for(k);
    let (keep, absorb) = state.read(|flat| {
        let get = |id: &str| flat.get(collection).and_then(|c| c.get(id)).cloned();
        (get(&pair.keep), get(&pair.absorb))
    });
    let (Some(keep), Some(absorb)) = (keep, absorb) else {
        return render::error_page_back_in(
            &chrome,
            StatusCode::NOT_FOUND,
            "merge-title",
            "merge-missing",
            &[],
            Some((&format!("/admin/{kind}"), "admin-cancel")),
        );
    };
    for e in [&keep, &absorb] {
        if let Err(r) = check_scope(&state, &chrome, &viewer, k, e, None) {
            return r;
        }
    }

    let env = state.inspect_with(|flat| axgf_rs::plan_merge(flat, k, &pair.keep, &pair.absorb));
    let back = format!("/admin/{kind}");
    if env.status == axgf_rs::boundary::envelope::Status::Error {
        return render::page_with(
            &chrome,
            "admin_result.html",
            context! {
                nav => "admin",
                title => chrome.t("merge-title"),
                summary => chrome.t("result-refused"),
                diagnostics => diagnostics_json(&env.diagnostics, chrome.lang),
                result_back => back,
                applied => false,
            },
        );
    }
    let plan = env.data;
    let page = state.read(|flat| plan_context(&chrome, flat, &kind, k, &keep, &absorb, &plan));
    render::page_with(
        &chrome,
        "admin_merge.html",
        context! {
            nav => "admin",
            kind,
            list_back => back,
            ..minijinja::Value::from_serialize(&page),
        },
    )
}

/// Everything the plan page draws, worked out from the library's plan.
fn plan_context(
    chrome: &render::Chrome,
    flat: &Value,
    kind: &str,
    k: EntityKind,
    keep: &Value,
    absorb: &Value,
    plan: &Value,
) -> Value {
    let id = |e: &Value| {
        e.get("id")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string()
    };
    let (keep_id, absorb_id) = (id(keep), id(absorb));
    let side = |e: &Value, id: &str| {
        json!({
            "id": id,
            "summary": summarize(flat, chrome.lang, k, e),
            "href": record_href(kind, id),
        })
    };
    let list = |key: &str| {
        plan.get(key)
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default()
    };

    let agreements: Vec<Value> = list("agreements")
        .iter()
        .map(|a| {
            let field = a["field"].as_str().unwrap_or("");
            json!({
                "label": field_label(chrome, field),
                "from": keyed(chrome, "merge-from", a["from"].as_str().unwrap_or("")),
            })
        })
        .collect();

    let conflicts: Vec<Value> = list("conflicts")
        .iter()
        .map(|c| {
            let field = c["field"].as_str().unwrap_or("");
            let suggestion = c["suggestion"].as_str().unwrap_or("");
            let choices: Vec<Value> = c["choices"]
                .as_array()
                .into_iter()
                .flatten()
                .filter_map(Value::as_str)
                .map(|choice| {
                    let text = match choice {
                        "keep" => value_text(chrome, flat, &c["keep"]),
                        "absorb" => value_text(chrome, flat, &c["absorb"]),
                        _ => String::new(),
                    };
                    json!({
                        "value": choice,
                        "label": keyed(chrome, "merge-choice", choice),
                        "text": text,
                        "checked": choice == suggestion,
                    })
                })
                .collect();
            json!({
                "field": field,
                "label": field_label(chrome, field),
                "choices": choices,
                "because": c["because"].as_str().map(|b| keyed(chrome, "merge-because", b)),
                "becomes": c["not_chosen_becomes"].as_str().map(|b| keyed(chrome, "merge-becomes", b)),
            })
        })
        .collect();

    let effects = plan.get("effects").cloned().unwrap_or(Value::Null);
    let references: Vec<Value> = effects
        .get("references")
        .and_then(Value::as_object)
        .map(|m| {
            m.iter()
                .filter_map(|(coll, n)| {
                    let n = n.as_i64()?;
                    let kind = [
                        EntityKind::Person,
                        EntityKind::Family,
                        EntityKind::Event,
                        EntityKind::Link,
                        EntityKind::Occupation,
                        EntityKind::Source,
                        EntityKind::Place,
                        EntityKind::Document,
                    ]
                    .into_iter()
                    .find(|k| collection_for(*k) == coll)?;
                    Some(json!({
                        "label": chrome.t_args(
                            &format!("kind-{}-plural", crate::state::kind_name(kind)),
                            &[("n", n.into())],
                        ),
                        "n": n,
                    }))
                })
                .collect()
        })
        .unwrap_or_default();
    let count = |key: &str| {
        effects
            .get(key)
            .and_then(Value::as_array)
            .map_or(0, Vec::len)
    };

    let refusals: Vec<Value> = list("refusals")
        .iter()
        .map(|r| {
            json!({
                "reason": keyed(chrome, "merge-refusal", r["reason"].as_str().unwrap_or("")),
                "message": r["message"],
                "confirm": r["confirm"],
            })
        })
        .collect();
    // A refusal with a phrase is lifted by typing it; any other is not.
    let blocked = list("refusals")
        .iter()
        .any(|r| r.get("confirm").and_then(Value::as_str).is_none());
    let confirm = list("refusals")
        .iter()
        .find_map(|r| r.get("confirm").and_then(Value::as_str).map(str::to_string));

    json!({
        "title": chrome.t("merge-title"),
        "keep": side(keep, &keep_id),
        "absorb": side(absorb, &absorb_id),
        "swap_href": plan_href(kind, &absorb_id, &keep_id),
        "fingerprint": plan["fingerprint"],
        "agreements": agreements,
        "conflicts": conflicts,
        "references": references,
        "self_links": count("self_links_removed"),
        "listed_twice": count("listed_twice"),
        "same_couple": count("same_couple_families"),
        "refusals": refusals,
        "blocked": blocked || plan.get("mergeable") == Some(&Value::Bool(false)),
        "confirm": confirm,
    })
}

/// `POST /admin/:kind/merge` — merge as decided on the plan page.
pub async fn apply(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(kind): Path<String>,
    Form(form): Form<Vec<(String, String)>>,
) -> Response {
    let (viewer, chrome) = match require(&state, &headers, Need::Admin) {
        Ok(v) => v,
        Err(r) => return r,
    };
    let Some(k) = kind_from_str(&kind).filter(|k| mergeable(*k)) else {
        return render::error_page_in(
            &chrome,
            StatusCode::NOT_FOUND,
            "error-not-found-title",
            "error-not-found-detail",
        );
    };
    let field = |name: &str| {
        form.iter()
            .find(|(k, _)| k == name)
            .map(|(_, v)| v.trim().to_string())
            .unwrap_or_default()
    };
    let (keep_id, absorb_id) = (field("keep"), field("absorb"));
    let mut choices = Map::new();
    for (name, value) in &form {
        if let Some(f) = name.strip_prefix("choice:") {
            choices.insert(f.to_string(), Value::String(value.clone()));
        }
    }
    let mut resolutions = json!({"fingerprint": field("fingerprint"), "choices": choices});
    let typed = field("confirm");
    if !typed.is_empty() {
        resolutions["confirm"] = Value::String(typed);
    }

    let collection = collection_for(k);
    let (keep_before, absorb_before) = state.read(|flat| {
        let get = |id: &str| flat.get(collection).and_then(|c| c.get(id)).cloned();
        (get(&keep_id), get(&absorb_id))
    });
    let (Some(keep_before), Some(absorb_before)) = (keep_before, absorb_before) else {
        return render::error_page_back_in(
            &chrome,
            StatusCode::NOT_FOUND,
            "merge-title",
            "merge-missing",
            &[],
            Some((&format!("/admin/{kind}"), "admin-cancel")),
        );
    };
    for e in [&keep_before, &absorb_before] {
        if let Err(r) = check_scope(&state, &chrome, &viewer, k, e, None) {
            return r;
        }
    }
    let labels = state.read(|flat| {
        (
            summarize(flat, chrome.lang, k, &keep_before),
            summarize(flat, chrome.lang, k, &absorb_before),
        )
    });

    let resolutions = resolutions.to_string();
    let out = match state
        .mutate(|flat| axgf_rs::merge_entities(flat, k, &keep_id, &absorb_id, &resolutions))
    {
        Ok(o) => o,
        Err(e) => return io_error(&chrome, &e),
    };
    if !out.applied {
        // A stale plan, a choice missing, a refusal not lifted: back to a
        // fresh plan, with the library's reasons.
        return render::page_with(
            &chrome,
            "admin_result.html",
            context! {
                nav => "admin",
                title => chrome.t("merge-title"),
                summary => chrome.t("result-refused"),
                diagnostics => diagnostics_json(&out.diagnostics, chrome.lang),
                result_back => plan_href(&kind, &keep_id, &absorb_id),
                applied => false,
            },
        );
    }

    let report = out.data.get("report").cloned().unwrap_or(Value::Null);
    let absorbed = report
        .get("absorbed_record")
        .cloned()
        .unwrap_or(absorb_before);
    let kind_name = crate::state::kind_name(k);
    // The absorbed record, field by field: every value it held is in the
    // journal's change list, against nothing.
    let gone = crate::journal::Entry {
        merged_into: Some(keep_id.clone()),
        ..crate::journal::entry_for(crate::journal::Record {
            who: viewer.name(),
            action: "merge",
            kind: kind_name,
            entity_id: &absorb_id,
            label: Some(labels.1.clone()),
            version_num: None,
            before: Some(&absorbed),
            after: Some(&json!({})),
        })
    };
    state.journal_mutation(&gone);
    let keep_after = state.read(|flat| flat.get(collection).and_then(|c| c.get(&keep_id)).cloned());
    if let Some(after) = &keep_after {
        let kept = crate::journal::entry_for(crate::journal::Record {
            who: viewer.name(),
            action: "update",
            kind: kind_name,
            entity_id: &keep_id,
            label: Some(labels.0.clone()),
            version_num: Some(crate::state::version_of(after)),
            before: Some(&keep_before),
            after: Some(after),
        });
        state.journal_mutation(&kept);
    }

    let n = |key: &str| -> i64 {
        match report.get(key) {
            Some(Value::Array(a)) => a.len() as i64,
            Some(Value::Object(m)) => m.values().filter_map(Value::as_i64).sum(),
            Some(Value::Number(x)) => x.as_i64().unwrap_or(0),
            _ => 0,
        }
    };
    let facts: Vec<Value> = [
        ("references_rewritten", "merge-report-references"),
        ("events_created", "merge-report-events"),
        ("source_conflicts_added", "merge-report-source-conflicts"),
        ("links_removed", "merge-report-links"),
        ("entries_tidied", "merge-report-tidied"),
    ]
    .iter()
    .map(|(key, label)| json!({"label": chrome.t(label), "n": n(key)}))
    .filter(|f| f["n"].as_i64().unwrap_or(0) > 0)
    .collect();
    render::page_with(
        &chrome,
        "admin_merge_done.html",
        context! {
            nav => "admin",
            kind,
            title => chrome.t("merge-done-title"),
            lede => chrome.t_args(
                "merge-done-lede",
                &[("absorbed", labels.1.clone().into()), ("kept", labels.0.clone().into())],
            ),
            kept_href => record_href(&kind, &keep_id),
            facts,
            diagnostics => diagnostics_json(&out.diagnostics, chrome.lang),
            absorbed => serde_json::to_string_pretty(&absorbed).unwrap_or_default(),
            list_back => format!("/admin/{kind}"),
        },
    )
}

/// `GET /admin/person/merge`: [`plan`] for persons, which need their own
/// route, as their bulk bar does: `/admin/person/:id` would otherwise take
/// "merge" for an id.
pub async fn plan_person(state: State<Shared>, headers: HeaderMap, pair: Query<Pair>) -> Response {
    plan(state, headers, Path("person".to_string()), pair).await
}

/// `POST /admin/person/merge`, for the same reason.
pub async fn apply_person(
    state: State<Shared>,
    headers: HeaderMap,
    form: Form<Vec<(String, String)>>,
) -> Response {
    apply(state, headers, Path("person".to_string()), form).await
}

/// `GET /admin/place/merge`, for the same reason: `/admin/place/:id`.
pub async fn plan_place(state: State<Shared>, headers: HeaderMap, pair: Query<Pair>) -> Response {
    plan(state, headers, Path("place".to_string()), pair).await
}

/// `POST /admin/place/merge`, for the same reason.
pub async fn apply_place(
    state: State<Shared>,
    headers: HeaderMap,
    form: Form<Vec<(String, String)>>,
) -> Response {
    apply(state, headers, Path("place".to_string()), form).await
}

/// `POST /admin/place/bulk`: the places listing's bulk bar, which
/// `/admin/place/:id` took for an update of a place called "bulk" until the
/// merge needed it.
pub async fn place_bulk(
    state: State<Shared>,
    headers: HeaderMap,
    form: Form<Vec<(String, String)>>,
) -> Response {
    super::admin::bulk(state, headers, Path("place".to_string()), form).await
}
