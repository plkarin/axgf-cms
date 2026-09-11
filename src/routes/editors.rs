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

use super::admin::{add_entity, require, save_entity, submitted_version, Need};
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
        submitted_version(&form),
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

// ---------------------------------------------------------------------------
// families: unions, children and parents
// ---------------------------------------------------------------------------
//
// Every edit on this page is a write to a *Family* entity, never to the person
// whose page it hangs off. That is not a detail of the implementation, it is
// the thing an editor most needs told: removing a spouse changes the family
// both people share, and the other person's record changes with it. The
// interface says so beside the controls rather than in a manual.

const UNION_TYPES: &[&str] = &[
    "marriage",
    "civil_union",
    "cohabitation",
    "religious_only",
    "polygamous",
    "unknown",
];

const UNION_STATUSES: &[&str] = &[
    "",
    "active",
    "ended_by_death",
    "ended_by_divorce",
    "ended_by_separation",
    "annulled",
    "unknown",
];

const PRECISIONS: &[&str] = &["", "exact", "year", "month", "decade", "century", "unknown"];

/// `GET /admin/person/:id/family`
pub async fn family_edit(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Response {
    let (viewer, chrome) = writer!(state, headers);
    let person = person!(state, chrome, id);
    render_family(&state, &chrome, &viewer, &id, &person, None)
}

/// `POST /admin/person/:id/family/:fid` — save or leave one family.
pub async fn family_update(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path((id, fid)): Path<(String, String)>,
    Form(form): Form<Body>,
) -> Response {
    let (viewer, chrome) = writer!(state, headers);
    let person = person!(state, chrome, id);
    let Some(stored) = state.read(|f| f.get("families").and_then(|c| c.get(&fid)).cloned()) else {
        return render::error_page_in(
            &chrome,
            StatusCode::NOT_FOUND,
            "error-no-such-entity-title",
            "error-no-such-entity-detail",
        );
    };

    let leaving = form.get("action").map(String::as_str) == Some("detach");
    let flat = state.read(|f| f.clone());
    let family = if leaving {
        match detach_from_family(&stored, &id) {
            Ok(v) => v,
            Err(key) => return render_family(&state, &chrome, &viewer, &id, &person, Some(key)),
        }
    } else {
        match build_family(&form, &stored, &flat) {
            Ok(v) => v,
            Err(key) => return render_family(&state, &chrome, &viewer, &id, &person, Some(key)),
        }
    };

    let mut entity = family;
    entity["id"] = Value::String(fid.clone());
    save_entity(
        &state,
        &chrome,
        &viewer,
        axgf_rs::EntityKind::Family,
        &fid,
        entity,
        Some(&stored),
        submitted_version(&form),
        &format!("/admin/person/{id}/family"),
        &format!("/admin/person/{id}/family"),
    )
}

/// `POST /admin/person/:id/family` — start a new union around this person.
pub async fn family_create(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(id): Path<String>,
    Form(form): Form<Body>,
) -> Response {
    let (viewer, chrome) = writer!(state, headers);
    let person = person!(state, chrome, id);
    let flat = state.read(|f| f.clone());

    // A union of one is a real thing — a parent the record names with no
    // partner at all — so the partner field may be left blank, and the family
    // is created with this person alone in it.
    let mut persons = vec![json!({"person_id": id, "role": "spouse"})];
    if let Some(raw) = form
        .get("partner")
        .map(String::as_str)
        .filter(|s| !s.trim().is_empty())
    {
        match forms::resolve_person(raw, &flat) {
            Ok(pid) => persons.push(json!({"person_id": pid, "role": "spouse"})),
            Err(e) => return render_family(&state, &chrome, &viewer, &id, &person, Some(e.key())),
        }
    }
    let kind = form
        .get("type")
        .map(String::as_str)
        .filter(|t| UNION_TYPES.contains(t))
        .unwrap_or("unknown");
    let entity = json!({
        "type": "family",
        "axgf_version": "1.0",
        "union": {"type": kind, "persons": persons},
    });
    add_entity(
        &state,
        &chrome,
        &viewer,
        axgf_rs::EntityKind::Family,
        entity,
        &format!("/admin/person/{id}/family"),
    )
}

/// `POST /admin/person/:id/parents` — attach this person to a family as a child.
pub async fn parents_attach(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(id): Path<String>,
    Form(form): Form<Body>,
) -> Response {
    let (viewer, chrome) = writer!(state, headers);
    let person = person!(state, chrome, id);
    let Some(fid) = form
        .get("family_id")
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
    else {
        return render_family(
            &state,
            &chrome,
            &viewer,
            &id,
            &person,
            Some("family-error-no-family"),
        );
    };
    let Some(stored) = state.read(|f| f.get("families").and_then(|c| c.get(&fid)).cloned()) else {
        return render_family(
            &state,
            &chrome,
            &viewer,
            &id,
            &person,
            Some("family-error-no-family"),
        );
    };

    let mut entity = stored.clone();
    let mut kids = entity
        .get("children")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    if kids
        .iter()
        .any(|c| c.get("person_id").and_then(Value::as_str) == Some(id.as_str()))
    {
        return render_family(
            &state,
            &chrome,
            &viewer,
            &id,
            &person,
            Some("family-error-already-child"),
        );
    }
    kids.push(json!({"person_id": id}));
    entity["children"] = Value::Array(kids);
    entity["id"] = Value::String(fid.clone());

    save_entity(
        &state,
        &chrome,
        &viewer,
        axgf_rs::EntityKind::Family,
        &fid,
        entity,
        Some(&stored),
        // The version just read, and this is the one form where that is right.
        // Everywhere else the number comes from the submission, because the
        // editor is overwriting a document they were shown and has to be told
        // when somebody moved it underneath them. Here they picked a family
        // from a list and are appending one child to it: they were never shown
        // its contents, so there is nothing of theirs to be stale.
        crate::state::version_of(&stored),
        &format!("/admin/person/{id}/family"),
        &format!("/admin/person/{id}/family"),
    )
}

/// Take one person out of a family, as partner or as child.
///
/// A union needs at least one person in it, so the last partner cannot simply
/// be dropped: what the editor wants then is to delete the family, which is a
/// different operation with a referential-integrity policy attached to it, and
/// saying so beats writing an entity the library will refuse.
fn detach_from_family(stored: &Value, person_id: &str) -> Result<Value, &'static str> {
    let mut out = stored.clone();
    let partners: Vec<Value> = out
        .pointer("/union/persons")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let was_partner = partners
        .iter()
        .any(|p| p.get("person_id").and_then(Value::as_str) == Some(person_id));
    if was_partner {
        let kept: Vec<Value> = partners
            .into_iter()
            .filter(|p| p.get("person_id").and_then(Value::as_str) != Some(person_id))
            .collect();
        if kept.is_empty() {
            return Err("family-error-last-partner");
        }
        out["union"]["persons"] = Value::Array(kept);
        return Ok(out);
    }
    let kids: Vec<Value> = out
        .get("children")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .filter(|c| c.get("person_id").and_then(Value::as_str) != Some(person_id))
        .collect();
    if kids.is_empty() {
        out.as_object_mut().map(|o| o.remove("children"));
    } else {
        out["children"] = Value::Array(kids);
    }
    Ok(out)
}

/// Build one family from its form.
fn build_family(form: &Body, stored: &Value, flat: &Value) -> Result<Value, &'static str> {
    let mut out = stored.clone();

    let mut union = Map::new();
    union.insert(
        "type".into(),
        json!(form
            .get("type")
            .map(String::as_str)
            .filter(|t| UNION_TYPES.contains(t))
            .unwrap_or("unknown")),
    );
    if let Some(s) = form
        .get("status")
        .map(String::as_str)
        .filter(|s| !s.is_empty() && UNION_STATUSES.contains(s))
    {
        union.insert("status".into(), json!(s));
    }

    // Partners. Every row is a person, so a blank row is a removal — the same
    // rule every list on every one of these forms follows.
    let mut persons = Vec::new();
    for r in forms::rows(form, "partner") {
        let Some(raw) = forms::field(&r, "person") else {
            continue;
        };
        let pid = forms::resolve_person(raw, flat).map_err(|e| e.key())?;
        let role = forms::field(&r, "role").unwrap_or("spouse");
        persons.push(json!({"person_id": pid, "role": role}));
    }
    if persons.is_empty() {
        return Err("family-error-last-partner");
    }
    union.insert("persons".into(), json!(persons));

    for (key, prefix) in [("start", "start"), ("end", "end")] {
        let date = forms::date_object(
            form.get(&format!("{prefix}.date")).map(String::as_str),
            form.get(&format!("{prefix}.precision")).map(String::as_str),
            matches!(
                form.get(&format!("{prefix}.circa")).map(String::as_str),
                Some("on" | "true" | "1")
            ),
        );
        let place = form
            .get(&format!("{prefix}.place_id"))
            .map(|s| s.trim())
            .filter(|s| !s.is_empty());
        if date.is_none() && place.is_none() {
            continue;
        }
        let mut o = Map::new();
        if let Some(d) = date {
            o.insert("date".into(), d);
        }
        if let Some(p) = place {
            o.insert("place_id".into(), json!(p));
        }
        // The specification keeps `event_id` on a union's start; it is not
        // this form's to set, but it is not this form's to drop either.
        if let Some(ev) = stored.pointer(&format!("/union/{key}/event_id")).cloned() {
            o.insert("event_id".into(), ev);
        }
        union.insert(key.into(), Value::Object(o));
    }

    if let Some(c) = forms::confidence(&as_row(form, "confidence"), "confidence") {
        union.insert("confidence".into(), c);
    }
    if let Some(s) = form
        .get("source_id")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
    {
        union.insert("source_id".into(), json!(s));
    }
    out["union"] = Value::Object(union);

    // Children, in the order the form draws them. `birth_order` is the
    // record's own claim about who came first and is left alone when the form
    // leaves it blank: an order invented from row position would be a fact
    // nobody stated.
    let mut kids = Vec::new();
    for r in forms::rows(form, "child") {
        let Some(raw) = forms::field(&r, "person") else {
            continue;
        };
        let pid = forms::resolve_person(raw, flat).map_err(|e| e.key())?;
        let mut c = Map::new();
        c.insert("person_id".into(), json!(pid));
        if let Some(n) = forms::field(&r, "birth_order").and_then(|v| v.parse::<i64>().ok()) {
            c.insert("birth_order".into(), json!(n));
        }
        if let Some(v) = forms::confidence(&r, "confidence") {
            c.insert("confidence".into(), v);
        }
        if let Some(n) = forms::field(&r, "note") {
            c.insert("note".into(), json!(n));
        }
        kids.push(Value::Object(c));
    }
    if kids.is_empty() {
        out.as_object_mut().map(|o| o.remove("children"));
    } else {
        out["children"] = Value::Array(kids);
    }

    for (key, field) in [("name", "name"), ("description", "description")] {
        match form.get(field).map(|s| s.trim()).filter(|s| !s.is_empty()) {
            Some(v) => {
                out[key] = json!(v);
            }
            None => {
                out.as_object_mut().map(|o| o.remove(key));
            }
        }
    }
    Ok(out)
}

/// Wrap a flat form in the one-row shape the row helpers read.
fn as_row(form: &Body, key: &str) -> std::collections::BTreeMap<String, String> {
    let mut m = std::collections::BTreeMap::new();
    if let Some(v) = form.get(key) {
        m.insert(key.to_string(), v.clone());
    }
    m
}

/// Draw the whole relationships page: every union this person is in, every
/// family they are a child of, and the two ways to make a new connection.
fn render_family(
    state: &Shared,
    chrome: &render::Chrome,
    viewer: &Viewer,
    id: &str,
    person: &Value,
    problem: Option<&'static str>,
) -> Response {
    let (unions, parented, people, places, sources, families) =
        state.read_as(viewer.ceiling(), |flat, lens| {
            let mut unions = Vec::new();
            let mut parented = Vec::new();
            if let Some(fams) = flat.get("families").and_then(Value::as_object) {
                for (fid, fam) in fams {
                    let partners: Vec<&str> = fam
                        .pointer("/union/persons")
                        .and_then(Value::as_array)
                        .map(|ps| {
                            ps.iter()
                                .filter_map(|p| p.get("person_id").and_then(Value::as_str))
                                .collect()
                        })
                        .unwrap_or_default();
                    let kids: Vec<&str> = fam
                        .get("children")
                        .and_then(Value::as_array)
                        .map(|cs| {
                            cs.iter()
                                .filter_map(|c| c.get("person_id").and_then(Value::as_str))
                                .collect()
                        })
                        .unwrap_or_default();
                    if partners.contains(&id) {
                        unions.push(family_view(flat, lens, fid, fam));
                    } else if kids.contains(&id) {
                        parented.push(family_view(flat, lens, fid, fam));
                    }
                }
            }
            (
                unions,
                parented,
                crate::forms::person_options(flat, lens),
                crate::forms::entity_options(flat, "places", crate::forms::place_label),
                crate::forms::entity_options(flat, "sources", crate::forms::source_label),
                family_options(flat, lens),
            )
        });

    render::page_with(
        chrome,
        "admin_family.html",
        context! {
            nav => "admin",
            id,
            person_name => crate::view::person_display_name(person),
            unions,
            parented,
            people,
            places,
            sources,
            families,
            problem,
            union_types => UNION_TYPES,
            union_statuses => UNION_STATUSES,
            precisions => PRECISIONS,
        },
    )
}

/// One family, as its form needs it.
fn family_view(flat: &Value, lens: &crate::access::Lens, fid: &str, fam: &Value) -> Value {
    let label_for_person = |pid: &str| match flat.get("persons").and_then(|c| c.get(pid)) {
        Some(p) if lens.sees_person(pid) => crate::forms::person_label(pid, p),
        // A partner this reader may not read keeps their place in the union —
        // the shape of a family is the same for everyone — and is offered back
        // as the id alone, so saving the form does not quietly drop them.
        _ => pid.to_string(),
    };
    let partners: Vec<Value> = fam
        .pointer("/union/persons")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default()
        .iter()
        .filter_map(|p| {
            let pid = p.get("person_id").and_then(Value::as_str)?;
            Some(json!({
                "label": label_for_person(pid),
                "role": p.get("role").and_then(Value::as_str).unwrap_or("spouse"),
            }))
        })
        .collect();
    let children: Vec<Value> = fam
        .get("children")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default()
        .iter()
        .filter_map(|c| {
            let pid = c.get("person_id").and_then(Value::as_str)?;
            Some(json!({
                "label": label_for_person(pid),
                "birth_order": c.get("birth_order").and_then(Value::as_i64),
                "confidence": c.get("confidence").and_then(Value::as_f64),
                "note": c.get("note").and_then(Value::as_str).unwrap_or_default(),
            }))
        })
        .collect();
    let point = |key: &str| {
        let d = fam.pointer(&format!("/union/{key}/date"));
        json!({
            "date": d.and_then(|x| x.get("value")).and_then(Value::as_str).unwrap_or_default(),
            "precision": d.and_then(|x| x.get("precision")).and_then(Value::as_str).unwrap_or_default(),
            "circa": d.and_then(|x| x.get("circa")).and_then(Value::as_bool).unwrap_or(false),
            "place_id": fam.pointer(&format!("/union/{key}/place_id"))
                .and_then(Value::as_str).unwrap_or_default(),
        })
    };
    json!({
        "id": fid,
        "version": crate::state::version_of(fam),
        "name": fam.get("name").and_then(Value::as_str).unwrap_or_default(),
        "description": fam.get("description").and_then(Value::as_str).unwrap_or_default(),
        "type": fam.pointer("/union/type").and_then(Value::as_str).unwrap_or("unknown"),
        "status": fam.pointer("/union/status").and_then(Value::as_str).unwrap_or_default(),
        "confidence": fam.pointer("/union/confidence").and_then(Value::as_f64),
        "source_id": fam.pointer("/union/source_id").and_then(Value::as_str).unwrap_or_default(),
        "partners": partners,
        "children": children,
        "start": point("start"),
        "end": point("end"),
    })
}

/// Families this reader may attach a child to, labelled by their partners.
fn family_options(flat: &Value, lens: &crate::access::Lens) -> Vec<Value> {
    let Some(fams) = flat.get("families").and_then(Value::as_object) else {
        return Vec::new();
    };
    let mut out: Vec<(String, String)> = fams
        .iter()
        .map(|(fid, fam)| {
            let names: Vec<String> = fam
                .pointer("/union/persons")
                .and_then(Value::as_array)
                .map(|ps| {
                    ps.iter()
                        .filter_map(|p| p.get("person_id").and_then(Value::as_str))
                        .map(|pid| match flat.get("persons").and_then(|c| c.get(pid)) {
                            Some(p) if lens.sees_person(pid) => crate::view::person_display_name(p),
                            _ => crate::forms::short_id(pid).to_string(),
                        })
                        .collect()
                })
                .unwrap_or_default();
            (
                format!("{} · #{}", names.join(" + "), crate::forms::short_id(fid)),
                fid.clone(),
            )
        })
        .collect();
    out.sort();
    out.into_iter()
        .map(|(label, id)| json!({"id": id, "label": label}))
        .collect()
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
