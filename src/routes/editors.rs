//! The structured editors: everything a record holds that is a list.
//!
//! The generic form in [`super::admin`] maps one input to one dotted path,
//! which is right for a scalar and cannot express a list. Nearly everything a
//! genealogical record actually contains is a list — the names somebody was
//! known by, the partners in a union and its children, the people at an event,
//! the documents attached to a person — and editing those through the raw-JSON
//! box is a text editor with a schema behind it, not an editing experience.
//!
//! Every handler here ends at [`super::admin::save_entity`] or
//! [`super::admin::add_entity`], which is where `family_scope`, the version
//! check, the conflict screen and the journal live. None of them writes JSON
//! into the bundle itself: the library's `add_entity`, `update_entity` and
//! `delete_entity` are the only ways in, which is the rule the whole
//! architecture rests on.

use axum::extract::{Form, Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::Response;
use minijinja::context;
use serde_json::{json, Map, Value};

use super::admin::{require, save_entity, Need};
use super::Shared;
use crate::access::Viewer;
use crate::forms::{self, Body};
use crate::render;

/// Bind `(viewer, chrome)` or return the refusal page.
macro_rules! writer {
    ($state:expr, $headers:expr) => {
        match require(&$state, &$headers, Need::Write) {
            Ok(v) => v,
            Err(r) => return r,
        }
    };
}

/// Read one person, or the "no such person" page.
macro_rules! person {
    ($state:expr, $chrome:expr, $id:expr) => {
        match $state.read(|flat| flat.get("persons").and_then(|c| c.get(&$id)).cloned()) {
            Some(p) => p,
            None => {
                return render::error_page_in(
                    &$chrome,
                    StatusCode::NOT_FOUND,
                    "error-no-such-person-title",
                    "error-no-such-person-detail",
                )
            }
        }
    };
}

// ---------------------------------------------------------------------------
// identity
// ---------------------------------------------------------------------------

/// The name types the specification allows for an alternative name.
const NAME_TYPES: &[&str] = &[
    "birth",
    "married",
    "alias",
    "nickname",
    "transliteration",
    "religious",
    "pen_name",
    "other",
];

/// The component types, in the order a form should offer them: the two that
/// occur in nearly every record first, then the rest.
const COMPONENT_TYPES: &[&str] = &[
    "given_name",
    "family_name",
    "patronymic",
    "matronymic",
    "nasab",
    "laqab",
    "kunya",
    "nisbah",
    "nickname",
    "alias",
    "religious_name",
    "pen_name",
];

const DIRECTIONS: &[&str] = &["", "ltr", "rtl", "auto"];
const GENDERS: &[&str] = &["M", "F", "NB", "U"];
const VISIBILITIES: &[&str] = &["", "public", "members", "contributors", "private"];

/// `GET /admin/person/:id/identity`
pub async fn identity_edit(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Response {
    let (viewer, chrome) = writer!(state, headers);
    let person = person!(state, chrome, id);
    render_identity(&state, &chrome, &viewer, &id, &person, None)
}

/// `POST /admin/person/:id/identity`
pub async fn identity_update(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(id): Path<String>,
    Form(form): Form<Body>,
) -> Response {
    let (viewer, chrome) = writer!(state, headers);
    let stored = person!(state, chrome, id);

    let identity = match build_identity(&form, &stored) {
        Ok(v) => v,
        Err(key) => {
            return render_identity(&state, &chrome, &viewer, &id, &stored, Some(key));
        }
    };

    // Start from what the bundle holds and replace one key. Everything else —
    // birth, death, notes, and the health extension the editor was never shown
    // — comes through untouched, which is why this form cannot become the
    // fourth place a diagnosis leaks out of.
    let mut entity = stored.clone();
    entity["identity"] = identity;
    entity["id"] = Value::String(id.clone());

    save_entity(
        &state,
        &chrome,
        &viewer,
        axgf_rs::EntityKind::Person,
        &id,
        entity,
        Some(&stored),
        &format!("/person/{id}"),
        &format!("/admin/person/{id}/identity"),
    )
}

/// Build the `identity` object from the form, or name the problem.
fn build_identity(form: &Body, stored: &Value) -> Result<Value, &'static str> {
    let primary_display = form
        .get("name.display")
        .map(|s| s.trim())
        .unwrap_or_default();
    if primary_display.is_empty() {
        // The schema requires it and every surface in the application renders
        // it. A record with no display name is a row of blank cards.
        return Err("identity-error-no-display");
    }

    let mut name = Map::new();
    name.insert("display".into(), json!(primary_display));
    put_opt(&mut name, "display_latin", form.get("name.display_latin"));
    put_opt(&mut name, "culture", form.get("name.culture"));
    put_opt(&mut name, "direction", form.get("name.direction"));
    put_opt(&mut name, "display_order", form.get("name.display_order"));

    // Components, numbered by their position in the form: `order` is what the
    // specification uses to say which part of a name comes first, and a form
    // that renders them in order already knows.
    let components: Vec<Value> = forms::rows(form, "comp")
        .iter()
        .filter_map(|r| {
            let value = forms::field(r, "value")?;
            let kind = forms::field(r, "type").unwrap_or("given_name");
            if !COMPONENT_TYPES.contains(&kind) {
                return None;
            }
            Some(json!({"type": kind, "value": value, "order": 0}))
        })
        .enumerate()
        .map(|(i, mut c)| {
            c["order"] = json!(i + 1);
            c
        })
        .collect();
    name.insert("components".into(), json!(components));

    // Alternative names. `ci` carries the row's position in the stored array
    // so that components this form does not edit survive it: a converted name
    // carries parsed parts, and dropping them because a form had no input for
    // them is losing data to a user interface.
    let stored_names = stored
        .pointer("/identity/names")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let mut names = Vec::new();
    for r in forms::rows(form, "alt") {
        let Some(display) = forms::field(&r, "display") else {
            continue;
        };
        let mut n = Map::new();
        n.insert("display".into(), json!(display));
        let kept = forms::field(&r, "ci")
            .and_then(|i| i.parse::<usize>().ok())
            .and_then(|i| stored_names.get(i))
            .and_then(|v| v.get("components"))
            .cloned()
            .unwrap_or_else(|| json!([]));
        n.insert("components".into(), kept);
        for (key, field) in [
            ("display_latin", "display_latin"),
            ("culture", "culture"),
            ("direction", "direction"),
            ("valid_from", "valid_from"),
            ("valid_until", "valid_until"),
            ("note", "note"),
        ] {
            if let Some(v) = forms::field(&r, field) {
                n.insert(key.into(), json!(v));
            }
        }
        if let Some(t) = forms::field(&r, "type").filter(|t| NAME_TYPES.contains(t)) {
            n.insert("type".into(), json!(t));
        }
        if let Some(s) = forms::field(&r, "source_id") {
            n.insert("source_id".into(), json!(s));
        }
        if let Some(c) = forms::confidence(&r, "confidence") {
            n.insert("confidence".into(), c);
        }
        names.push(Value::Object(n));
    }

    let mut identity = Map::new();
    identity.insert("name".into(), Value::Object(name));
    if !names.is_empty() {
        identity.insert("names".into(), json!(names));
    }

    let gender = form
        .get("gender.value")
        .map(String::as_str)
        .filter(|g| GENDERS.contains(g))
        .unwrap_or("U");
    let mut g = Map::new();
    g.insert("value".into(), json!(gender));
    put_opt(&mut g, "note", form.get("gender.note"));
    identity.insert("gender".into(), Value::Object(g));

    identity.insert(
        "is_living".into(),
        json!(matches!(
            form.get("is_living").map(String::as_str),
            Some("on" | "true" | "1")
        )),
    );
    if let Some(v) = form
        .get("visibility")
        .map(|s| s.trim())
        .filter(|v| !v.is_empty() && VISIBILITIES.contains(v))
    {
        identity.insert("visibility".into(), json!(v));
    }
    Ok(Value::Object(identity))
}

/// Insert a trimmed value, or nothing at all when it is blank.
fn put_opt(m: &mut Map<String, Value>, key: &str, v: Option<&String>) {
    if let Some(v) = v.map(|s| s.trim()).filter(|s| !s.is_empty()) {
        m.insert(key.into(), json!(v));
    }
}

fn render_identity(
    state: &Shared,
    chrome: &render::Chrome,
    viewer: &Viewer,
    id: &str,
    person: &Value,
    problem: Option<&'static str>,
) -> Response {
    let identity = person.get("identity").cloned().unwrap_or_else(|| json!({}));
    let name = identity.get("name").cloned().unwrap_or_else(|| json!({}));

    let components: Vec<Value> = name
        .get("components")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();

    let alts: Vec<Value> = identity
        .get("names")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .enumerate()
        .map(|(i, mut n)| {
            // The stored index rides with the row so that the components this
            // form does not show survive a save.
            n["ci"] = json!(i);
            n
        })
        .collect();

    let sources = state
        .read(|flat| crate::forms::entity_options(flat, "sources", crate::forms::source_label));

    render::page_with(
        chrome,
        "admin_identity.html",
        context! {
            nav => "admin",
            id,
            person_name => crate::view::person_display_name(person),
            name,
            components,
            alts,
            sources,
            problem,
            name_types => NAME_TYPES,
            component_types => COMPONENT_TYPES,
            directions => DIRECTIONS,
            genders => GENDERS,
            visibilities => VISIBILITIES,
            gender_value => identity.pointer("/gender/value")
                .and_then(Value::as_str).unwrap_or("U"),
            gender_note => identity.pointer("/gender/note")
                .and_then(Value::as_str).unwrap_or_default(),
            is_living => identity.get("is_living")
                .and_then(Value::as_bool).unwrap_or(false),
            visibility => identity.get("visibility")
                .and_then(Value::as_str).unwrap_or_default(),
            may_write => viewer.may_write(),
            base_version => crate::state::version_of(person),
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn body(pairs: &[(&str, &str)]) -> Body {
        pairs
            .iter()
            .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
            .collect()
    }

    fn stored() -> Value {
        json!({
            "identity": {
                "name": {"display": "Maria Klicki", "components": [
                    {"type": "given_name", "value": "Maria", "order": 1}]},
                "gender": {"value": "F"},
                "is_living": false,
                "visibility": "members",
                "names": [
                    {"display": "Maria Fiszczuk", "type": "married",
                     "components": [{"type": "family_name", "value": "Fiszczuk", "order": 1}]}
                ]
            },
            "birth": {"date": {"value": "1895", "precision": "year"}},
            "extensions": {"axgf-cms:health/v1": {"conditions": [{"value": "Sarcoidosis"}]}}
        })
    }

    #[test]
    fn a_record_without_a_display_name_is_refused_rather_than_saved() {
        let out = build_identity(&body(&[("name.display", "  ")]), &stored());
        assert_eq!(out, Err("identity-error-no-display"));
    }

    #[test]
    fn components_are_numbered_by_the_order_the_form_renders_them() {
        // `order` is what the specification uses to say which part of a name
        // comes first, and a form that draws them in order already knows it.
        let v = build_identity(
            &body(&[
                ("name.display", "Maria Fiszczuk"),
                ("comp.0.type", "given_name"),
                ("comp.0.value", "Maria"),
                ("comp.1.type", "family_name"),
                ("comp.1.value", "Fiszczuk"),
                ("comp.2.value", ""),
            ]),
            &stored(),
        )
        .expect("built");
        let comps = v["name"]["components"].as_array().expect("components");
        assert_eq!(comps.len(), 2, "the spare blank row is dropped");
        assert_eq!(comps[0]["order"], 1);
        assert_eq!(comps[1]["order"], 2);
        assert_eq!(comps[1]["type"], "family_name");
    }

    #[test]
    fn an_alternative_name_keeps_the_parts_this_form_does_not_show() {
        // A converted name carries parsed components. Losing them because the
        // form had no input for them is losing data to a user interface.
        let v = build_identity(
            &body(&[
                ("name.display", "Maria Klicki"),
                ("alt.0.ci", "0"),
                ("alt.0.display", "Maria Fiszczuk"),
                ("alt.0.type", "married"),
                ("alt.0.valid_from", "1920"),
            ]),
            &stored(),
        )
        .expect("built");
        let alt = &v["names"][0];
        assert_eq!(alt["display"], "Maria Fiszczuk");
        assert_eq!(alt["valid_from"], "1920");
        assert_eq!(alt["components"][0]["value"], "Fiszczuk");
    }

    #[test]
    fn a_new_alternative_name_starts_with_no_parts() {
        let v = build_identity(
            &body(&[
                ("name.display", "Maria Klicki"),
                ("alt.0.display", "Marysia"),
                ("alt.0.type", "nickname"),
            ]),
            &stored(),
        )
        .expect("built");
        assert_eq!(v["names"][0]["components"], json!([]));
    }

    #[test]
    fn a_blank_row_removes_the_name_it_held() {
        let v = build_identity(
            &body(&[("name.display", "Maria Klicki"), ("alt.0.display", "")]),
            &stored(),
        )
        .expect("built");
        assert!(v.get("names").is_none(), "an empty list is no key at all");
    }

    #[test]
    fn an_unchecked_box_is_not_living_and_an_unknown_vocabulary_is_refused() {
        let v = build_identity(
            &body(&[
                ("name.display", "X"),
                ("gender.value", "wizard"),
                ("visibility", "everyone"),
                ("alt.0.display", "Y"),
                ("alt.0.type", "shouted"),
            ]),
            &stored(),
        )
        .expect("built");
        assert_eq!(v["is_living"], false, "an absent checkbox is off");
        assert_eq!(
            v["gender"]["value"], "U",
            "a word outside the enum is not stored"
        );
        assert!(v.get("visibility").is_none());
        assert!(v["names"][0].get("type").is_none());
    }
}
