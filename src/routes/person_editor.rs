//! The record's editor (v0.4.2): six tabs over one form, one save.
//!
//! Reading has six tabs — Fiche · Famille · Événements · Sources · Médias ·
//! Profil — and so does editing, so the tab somebody was reading is the tab
//! they land on when they press Modifier. Reading hides an empty tab; editing
//! never does, because an empty tab is where things are added.
//!
//! # One form, one save
//!
//! The tabs show and hide parts of a single form. A save rewrites the whole
//! archive — 12 to 17 seconds on the operator's 434 MB bundle — so six pages
//! with six saves would turn an editing session into six waits. One
//! Enregistrer turns everything changed, on every tab, into a list of
//! [`Op`]s and writes them with one [`AppState::commit`].
//!
//! # What was drawn and left alone is not written
//!
//! Every part of a tab that writes a record — a union, the family somebody is
//! a child of, an event, a source, a file, a profile group — is a [`Block`]:
//! the inputs it drew, under one name prefix, and a marker saying it was
//! drawn. On save a block is read back only when its marker came back and
//! what was submitted under its prefix differs from what was drawn. That is
//! the rule the profile groups have had since v0.4.1, applied to every other
//! record the editor touches: a save that fixes a name does not rewrite a
//! family nobody opened, and a form opened before a block existed cannot
//! clear it.
//!
//! [`Op`]: crate::state::Op
//! [`AppState::commit`]: crate::state::AppState::commit

use std::collections::{BTreeMap, HashMap};

use axum::extract::{FromRequest, Path, Request, State};
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use minijinja::context;
use serde::Serialize;
use serde_json::{json, Map, Value};

use super::admin::{require, Need};
use super::Shared;
use crate::access::Viewer;
use crate::forms::{self, Body};
use crate::render;
use crate::state::Op;
use axgf_rs::model::profile::vocab::LINEAGE;
use axgf_rs::EntityKind;

/// The editor's tabs: the fragment in the URL, the catalogue key of its name
/// (the reading tab's own), and the reading tab it answers.
pub const TABS: &[(&str, &str, &str)] = &[
    ("fiche", "person-tab-record", "record"),
    ("famille", "record-family", "family"),
    ("evenements", "person-tab-life", "life"),
    ("sources", "river-tab-sources", "sources"),
    ("medias", "person-tab-media", "media"),
    ("profil", "person-tab-profile", "profile"),
];

/// The editor's fragment for a reading tab's slug: Modifier on the Famille
/// tab opens the editor on `#famille`.
pub fn fragment_for(read_slug: &str) -> &'static str {
    TABS.iter()
        .find(|(_, _, r)| *r == read_slug)
        .map(|(f, _, _)| *f)
        .unwrap_or("fiche")
}

fn known_tab(t: &str) -> Option<&'static str> {
    TABS.iter().find(|(f, _, _)| *f == t).map(|(f, _, _)| *f)
}

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
const ROLES: &[&str] = &[
    "subject",
    "participant",
    "spouse",
    "witness",
    "officiant",
    "informant",
    "godparent",
];
const PRECISIONS: &[&str] = &[
    "",
    "exact",
    "month",
    "year",
    "decade",
    "quarter_century",
    "century",
    "unknown",
];

// ---------------------------------------------------------------------------
// the model
// ---------------------------------------------------------------------------

/// One option of a select.
#[derive(Debug, Clone, Serialize)]
pub struct Opt {
    pub value: String,
    pub label: String,
}

/// One input. Everything a template needs to draw it, and everything the save
/// needs to know what the browser will post for it untouched.
#[derive(Debug, Clone, Serialize, Default)]
pub struct Field {
    pub name: String,
    pub label: String,
    /// `text`, `textarea`, `select`, `check`, `picker`, `number`, `hidden`
    /// or `file`.
    pub kind: &'static str,
    pub value: String,
    pub options: Vec<Opt>,
    /// The datalist a picker searches.
    pub list: String,
    pub hint: String,
    /// A date's qualifier (`vers ▾ 1890`), posted under its own name.
    pub qualifier: Option<String>,
    pub qualifier_on: bool,
    pub min: Option<&'static str>,
    pub max: Option<&'static str>,
    pub step: Option<&'static str>,
    /// Drawn wide: a description, a note.
    pub wide: bool,
}

impl Field {
    fn new(kind: &'static str, name: String, label: String, value: impl Into<String>) -> Self {
        Field {
            kind,
            name,
            label,
            value: value.into(),
            ..Default::default()
        }
    }

    fn select(name: String, label: String, value: &str, options: Vec<Opt>) -> Self {
        Field {
            options,
            ..Field::new("select", name, label, value)
        }
    }

    fn confidence(name: String, label: String, value: Option<f64>) -> Self {
        Field {
            min: Some("0"),
            max: Some("1"),
            step: Some("0.05"),
            ..Field::new(
                "number",
                name,
                label,
                value.map(|v| format!("{v}")).unwrap_or_default(),
            )
        }
    }

    /// What a browser posts for this field as drawn, if nobody touches it. A
    /// select cannot post a value it has no option for: such a value counts
    /// as the first option, as it would be sent.
    fn posted(&self, out: &mut Body) {
        match self.kind {
            "file" => {}
            "check" => {
                if self.value == "on" {
                    out.insert(self.name.clone(), "on".into());
                }
            }
            "select" => {
                let v = if self.options.iter().any(|o| o.value == self.value) {
                    self.value.clone()
                } else {
                    self.options
                        .first()
                        .map(|o| o.value.clone())
                        .unwrap_or_default()
                };
                out.insert(self.name.clone(), v);
            }
            _ => {
                out.insert(self.name.clone(), self.value.clone());
            }
        }
        if let Some(q) = &self.qualifier {
            if self.qualifier_on {
                out.insert(q.clone(), "on".into());
            }
        }
    }

    /// Take what was submitted, after a refused save: the form comes back as
    /// it was sent, not as it was first drawn.
    fn fill(&mut self, body: &Body) {
        match self.kind {
            "file" => {}
            "check" => {
                self.value = if body.get(&self.name).is_some_and(|v| v == "on") {
                    "on".into()
                } else {
                    String::new()
                }
            }
            _ => self.value = body.get(&self.name).cloned().unwrap_or_default(),
        }
        if let Some(q) = &self.qualifier {
            self.qualifier_on = body.get(q).is_some_and(|v| v == "on");
        }
    }
}

/// One item of a list inside a block: a partner, a child.
#[derive(Debug, Clone, Serialize, Default)]
pub struct Row {
    pub title: String,
    pub href: Option<String>,
    pub fields: Vec<Field>,
    /// "Retirer": a checkbox, drawn as an action.
    pub remove: Option<Field>,
}

/// A list of rows with the action that adds to it.
#[derive(Debug, Clone, Serialize, Default)]
pub struct RowList {
    pub title: String,
    pub rows: Vec<Row>,
    /// The row an "add" action opens.
    pub add: Option<Row>,
    pub add_label: String,
}

/// Everything one record's part of a tab draws, under one name prefix.
#[derive(Debug, Clone, Serialize, Default)]
pub struct Block {
    /// `fam.<id>`, `par.<id>`, `ev.<id>`, `src.<id>`, `doc.<id>`, `cite`,
    /// `new.fam`, `new.par`, `new.ev`, `new.src`, `new.doc`.
    pub prefix: String,
    /// What the template draws it as: `union`, `parents`, `event`, `source`,
    /// `document`, `cite` or `new`.
    pub kind: &'static str,
    pub title: String,
    pub subtitle: String,
    pub href: Option<String>,
    pub thumb: Option<String>,
    pub fields: Vec<Field>,
    pub lists: Vec<RowList>,
    /// The block-level action: leave this union, take this event off the
    /// record, detach this file.
    pub remove: Option<Field>,
    /// For a `new` block: the action that opens it.
    pub add_label: String,
    /// Changed in what was just submitted (a refused save): drawn open.
    pub touched: bool,
}

impl Block {
    fn fields(&self) -> impl Iterator<Item = &Field> {
        self.fields
            .iter()
            .chain(self.lists.iter().flat_map(|l| {
                l.rows
                    .iter()
                    .chain(l.add.iter())
                    .flat_map(|r| r.fields.iter().chain(r.remove.iter()))
            }))
            .chain(self.remove.iter())
    }

    fn fields_mut(&mut self) -> Vec<&mut Field> {
        let mut out: Vec<&mut Field> = self.fields.iter_mut().collect();
        for l in &mut self.lists {
            for r in l.rows.iter_mut().chain(l.add.iter_mut()) {
                out.extend(r.fields.iter_mut());
                out.extend(r.remove.iter_mut());
            }
        }
        out.extend(self.remove.iter_mut());
        out
    }

    /// The hidden input that says this block was drawn.
    pub fn marker(&self) -> String {
        format!("{}.drawn", self.prefix)
    }

    fn drawn(&self) -> Body {
        let mut out = Body::new();
        for f in self.fields() {
            f.posted(&mut out);
        }
        out.retain(|_, v| !v.trim().is_empty());
        out
    }

    fn submitted(&self, body: &Body) -> Body {
        let lead = format!("{}.", self.prefix);
        let marker = self.marker();
        body.iter()
            .filter(|(k, v)| k.starts_with(&lead) && **k != marker && !v.trim().is_empty())
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    /// Drawn on the form that was submitted, and changed there.
    fn changed_in(&self, body: &Body) -> bool {
        body.contains_key(&self.marker()) && self.drawn() != self.submitted(body)
    }

    /// The submission under this block's prefix, with the prefix taken off.
    fn sub(&self, body: &Body) -> HashMap<String, String> {
        let lead = format!("{}.", self.prefix);
        body.iter()
            .filter_map(|(k, v)| k.strip_prefix(&lead).map(|s| (s.to_string(), v.clone())))
            .collect()
    }
}

/// One tab of the editor.
#[derive(Debug, Clone, Serialize)]
pub struct Tab {
    pub key: &'static str,
    pub label: String,
    pub blocks: Vec<Block>,
    /// Something on this tab was changed in a submission that was refused:
    /// the strip marks it, so the editor knows what is still unsaved.
    pub changed: bool,
}

/// The whole editor, before the Fiche fields and the profile, which have
/// their own drawing.
pub struct Model {
    pub tabs: Vec<Tab>,
    /// The datalists the pickers search: `people`, `pick-place`,
    /// `pick-source`.
    pub lists: BTreeMap<&'static str, Vec<Value>>,
}

impl Model {
    fn blocks(&self) -> impl Iterator<Item = (&'static str, &Block)> {
        self.tabs
            .iter()
            .flat_map(|t| t.blocks.iter().map(move |b| (t.key, b)))
    }
}

// ---------------------------------------------------------------------------
// drawing
// ---------------------------------------------------------------------------

struct Ctx<'a> {
    flat: &'a Value,
    lens: &'a crate::access::Lens,
    lang: &'a str,
    id: &'a str,
}

impl Ctx<'_> {
    fn t(&self, key: &str) -> String {
        crate::i18n::translate(self.lang, key, None)
    }

    fn t_args(&self, key: &str, pairs: &[(&str, String)]) -> String {
        let mut args = fluent::FluentArgs::new();
        for (k, v) in pairs {
            args.set(*k, fluent::FluentValue::from(v.clone()));
        }
        crate::i18n::translate(self.lang, key, Some(&args))
    }

    fn vocab(&self, family: &str, values: &[&str]) -> Vec<Opt> {
        values
            .iter()
            .map(|v| Opt {
                value: (*v).to_string(),
                label: if v.is_empty() {
                    self.t("admin-not-set")
                } else {
                    crate::i18n::vocab(self.lang, family, v)
                },
            })
            .collect()
    }

    fn not_set(&self) -> Opt {
        Opt {
            value: String::new(),
            label: self.t("admin-not-set"),
        }
    }

    fn lineages(&self) -> Vec<Opt> {
        std::iter::once(self.not_set())
            .chain(
                crate::profile::form::vocabulary_options(self.lang, &LINEAGE)
                    .into_iter()
                    .map(|o| Opt {
                        value: o.value,
                        label: o.label,
                    }),
            )
            .collect()
    }

    fn sources(&self) -> Vec<Opt> {
        std::iter::once(self.not_set())
            .chain(
                forms::entity_options(self.flat, "sources", forms::source_label)
                    .into_iter()
                    .map(|o| Opt {
                        value: o["id"].as_str().unwrap_or_default().to_string(),
                        label: o["label"].as_str().unwrap_or_default().to_string(),
                    }),
            )
            .collect()
    }

    /// A person as a reader of this editor may see them.
    fn person_name(&self, pid: &str) -> String {
        match self.flat.get("persons").and_then(|c| c.get(pid)) {
            Some(p) if self.lens.sees_person(pid) => crate::view::person_display_name(p),
            Some(_) => self.t(crate::person::RESTRICTED_KEY),
            None => forms::short_id(pid).to_string(),
        }
    }

    fn person_href(&self, pid: &str) -> Option<String> {
        (self.lens.sees_person(pid) && self.flat.get("persons").and_then(|c| c.get(pid)).is_some())
            .then(|| format!("/admin/person/{pid}/edit#famille"))
    }

    fn date_fields(&self, prefix: &str, label: &str, d: Option<&Value>) -> Vec<Field> {
        let get = |k: &str| {
            d.and_then(|x| x.get(k))
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_string()
        };
        vec![
            Field {
                qualifier: Some(format!("{prefix}.circa")),
                qualifier_on: d
                    .and_then(|x| x.get("circa"))
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
                ..Field::new(
                    "text",
                    format!("{prefix}.date"),
                    label.to_string(),
                    get("value"),
                )
            },
            Field::select(
                format!("{prefix}.precision"),
                self.t("record-precision"),
                &get("precision"),
                PRECISIONS
                    .iter()
                    .map(|p| Opt {
                        value: (*p).to_string(),
                        label: if p.is_empty() {
                            self.t("admin-not-set")
                        } else {
                            self.t(&format!("precision-{p}"))
                        },
                    })
                    .collect(),
            ),
        ]
    }
}

/// The generic fields of a kind (`crate::admin`'s tables), as fields of a
/// block under `prefix`: labels, vocabularies and pickers as the generic form
/// has them, and read back by the same `apply_form`.
pub fn generic_fields(views: &[Value], prefix: &str) -> Vec<Field> {
    let name = |p: &str| {
        if prefix.is_empty() {
            p.to_string()
        } else {
            format!("{prefix}.{p}")
        }
    };
    views
        .iter()
        .filter(|v| !v["inside_date"].as_bool().unwrap_or(false))
        .map(|v| {
            let s = |k: &str| v[k].as_str().unwrap_or_default().to_string();
            let path = s("path");
            let mut f = Field {
                name: name(&path),
                label: s("label"),
                hint: s("hint"),
                value: s("value"),
                ..Default::default()
            };
            match v["kind"].as_str().unwrap_or("text") {
                "longtext" => {
                    f.kind = "textarea";
                    f.wide = true;
                }
                "bool" => {
                    f.kind = "check";
                    f.value = if v["checked"].as_bool().unwrap_or(false) {
                        "on".into()
                    } else {
                        String::new()
                    };
                }
                "select" => {
                    f.kind = "select";
                    f.options = v["options"]
                        .as_array()
                        .map(|os| {
                            os.iter()
                                .map(|o| Opt {
                                    value: o["value"].as_str().unwrap_or_default().to_string(),
                                    label: o["label"].as_str().unwrap_or_default().to_string(),
                                })
                                .collect()
                        })
                        .unwrap_or_default();
                }
                // A number, not a slider: a slider has no "not set", and a
                // browser posts the middle of its range for one drawn
                // without a value — a confidence of 0.5 nobody chose.
                "confidence" => {
                    f.kind = "number";
                    f.min = Some("0");
                    f.max = Some("1");
                    f.step = Some("0.05");
                }
                "picker" => {
                    f.kind = "picker";
                    f.list = format!("pick-{}", s("picks"));
                }
                _ => f.kind = "text",
            }
            if let Some(q) = v["qualifier"].as_object() {
                f.qualifier = q.get("path").and_then(Value::as_str).map(name);
                f.qualifier_on = q.get("on").and_then(Value::as_bool).unwrap_or(false);
            }
            f
        })
        .collect()
}

fn hidden(name: String, value: impl Into<String>) -> Field {
    Field::new("hidden", name, String::new(), value)
}

fn check(name: String, label: String) -> Field {
    Field::new("check", name, label, "")
}

/// Every block of the editor, drawn from the bundle as it is.
pub fn model(
    state: &Shared,
    viewer: &Viewer,
    lang: &str,
    id: &str,
    person: &Value,
    lists: &BTreeMap<String, Vec<Value>>,
) -> Model {
    state.read_as(viewer.ceiling(), |flat, lens| {
        let cx = Ctx {
            flat,
            lens,
            lang,
            id,
        };
        let pick = |kind: EntityKind, entity: &Value, prefix: &str| {
            generic_fields(
                &super::admin::field_views(kind, entity, lang, lists),
                prefix,
            )
        };
        let mut tabs: Vec<Tab> = TABS
            .iter()
            .map(|(key, label, _)| Tab {
                key,
                label: cx.t(label),
                blocks: Vec::new(),
                changed: false,
            })
            .collect();
        let tab = |key: &str| TABS.iter().position(|(k, _, _)| *k == key).unwrap_or(0);

        // ---- Famille ---------------------------------------------------
        let fams = flat.get("families").and_then(Value::as_object);
        let mut unions = Vec::new();
        let mut parented = Vec::new();
        for (fid, fam) in fams.into_iter().flatten() {
            let ids = |ptr: &str| -> Vec<String> {
                fam.pointer(ptr)
                    .and_then(Value::as_array)
                    .map(|a| {
                        a.iter()
                            .filter_map(|p| p.get("person_id").and_then(Value::as_str))
                            .map(str::to_string)
                            .collect()
                    })
                    .unwrap_or_default()
            };
            if ids("/union/persons").iter().any(|p| p == id) {
                unions.push(union_block(&cx, fid, fam));
            } else if ids("/children").iter().any(|p| p == id) {
                parented.push(parents_block(&cx, fid, fam));
            }
        }
        let fb = &mut tabs[tab("famille")].blocks;
        fb.extend(parented);
        fb.extend(unions);
        fb.push(new_family_block(&cx));
        fb.push(new_parents_block(&cx));

        // ---- Événements ------------------------------------------------
        let mut events: Vec<(String, Block)> = Vec::new();
        for (eid, ev) in flat
            .get("events")
            .and_then(Value::as_object)
            .into_iter()
            .flatten()
        {
            let role = ev
                .get("participants")
                .and_then(Value::as_array)
                .and_then(|ps| {
                    ps.iter().find(|p| {
                        p.get("entity_type").and_then(Value::as_str) == Some("person")
                            && p.get("entity_id").and_then(Value::as_str) == Some(id)
                    })
                })
                .map(|p| {
                    p.get("role")
                        .and_then(Value::as_str)
                        .unwrap_or("participant")
                        .to_string()
                });
            let Some(role) = role else { continue };
            let prefix = format!("ev.{eid}");
            let mut fields = pick(EntityKind::Event, ev, &prefix);
            fields.push(Field::select(
                format!("{prefix}.role"),
                cx.t("family-role"),
                &role,
                cx.vocab("role", ROLES),
            ));
            fields.push(hidden(
                format!("{prefix}.v"),
                crate::state::version_of(ev).to_string(),
            ));
            let category = ev
                .get("category")
                .and_then(Value::as_str)
                .unwrap_or("other");
            let when = ev
                .pointer("/date/value")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_string();
            events.push((
                when.clone(),
                Block {
                    prefix: prefix.clone(),
                    kind: "event",
                    title: crate::i18n::vocab(lang, "event-category", category),
                    subtitle: when,
                    fields,
                    remove: Some(check(format!("{prefix}.leave"), cx.t("events-remove"))),
                    ..Default::default()
                },
            ));
        }
        events.sort_by(|a, b| a.0.cmp(&b.0));
        let eb = &mut tabs[tab("evenements")].blocks;
        eb.extend(events.into_iter().map(|(_, b)| b));
        let mut fields = pick(EntityKind::Event, &json!({}), "new.ev");
        fields.push(Field::select(
            "new.ev.role".into(),
            cx.t("family-role"),
            "subject",
            cx.vocab("role", ROLES),
        ));
        eb.push(Block {
            prefix: "new.ev".into(),
            kind: "new",
            add_label: cx.t("events-new"),
            subtitle: cx.t("events-new-help"),
            fields,
            ..Default::default()
        });

        // ---- Sources ---------------------------------------------------
        let sources = cx.sources();
        let cite = |fact: &str, label: String| {
            Field::select(
                format!("cite.{fact}"),
                label,
                person
                    .pointer(&format!("/{fact}/source_id"))
                    .and_then(Value::as_str)
                    .unwrap_or_default(),
                sources.clone(),
            )
        };
        let sb = &mut tabs[tab("sources")].blocks;
        sb.push(Block {
            prefix: "cite".into(),
            kind: "cite",
            title: cx.t("edit-cite-title"),
            fields: vec![
                cite("birth", cx.t("event-category-birth")),
                cite("death", cx.t("event-category-death")),
            ],
            ..Default::default()
        });
        let rests_on = crate::person::build_in(flat, id, lens, lang)
            .map(|p| p.sources.into_iter().map(|s| s.id).collect::<Vec<_>>())
            .unwrap_or_default();
        let mut cited: Vec<String> = rests_on;
        for fact in ["birth", "death"] {
            if let Some(s) = person
                .pointer(&format!("/{fact}/source_id"))
                .and_then(Value::as_str)
            {
                if !cited.iter().any(|c| c == s) {
                    cited.push(s.to_string());
                }
            }
        }
        for sid in cited {
            let Some(src) = flat.get("sources").and_then(|c| c.get(&sid)) else {
                continue;
            };
            let prefix = format!("src.{sid}");
            let mut fields = pick(EntityKind::Source, src, &prefix);
            fields.push(hidden(
                format!("{prefix}.v"),
                crate::state::version_of(src).to_string(),
            ));
            sb.push(Block {
                prefix,
                kind: "source",
                title: forms::source_label(src),
                fields,
                ..Default::default()
            });
        }
        let mut fields = pick(EntityKind::Source, &json!({}), "new.src");
        fields.push(Field::select(
            "new.src.attach".into(),
            cx.t("edit-attach-to"),
            "",
            vec![
                cx.not_set(),
                Opt {
                    value: "birth".into(),
                    label: cx.t("event-category-birth"),
                },
                Opt {
                    value: "death".into(),
                    label: cx.t("event-category-death"),
                },
            ],
        ));
        sb.push(Block {
            prefix: "new.src".into(),
            kind: "new",
            add_label: cx.t("edit-add-source"),
            fields,
            ..Default::default()
        });

        // ---- Médias ----------------------------------------------------
        let mut attached: Vec<String> = person
            .get("documents")
            .and_then(Value::as_array)
            .map(|a| {
                a.iter()
                    .filter_map(|d| d.get("document_id").and_then(Value::as_str))
                    .map(str::to_string)
                    .collect()
            })
            .unwrap_or_default();
        for (did, d) in flat
            .get("documents")
            .and_then(Value::as_object)
            .into_iter()
            .flatten()
        {
            let linked = d
                .get("linked_to")
                .and_then(Value::as_array)
                .is_some_and(|l| {
                    l.iter().any(|x| {
                        x.get("entity_type").and_then(Value::as_str) == Some("person")
                            && x.get("entity_id").and_then(Value::as_str) == Some(id)
                    })
                });
            if linked && !attached.contains(did) {
                attached.push(did.clone());
            }
        }
        let types: Vec<Opt> = crate::documents::document_type_options(lang)
            .into_iter()
            .map(|o| Opt {
                value: o["value"].as_str().unwrap_or_default().to_string(),
                label: o["label"].as_str().unwrap_or_default().to_string(),
            })
            .collect();
        let mb = &mut tabs[tab("medias")].blocks;
        for did in attached {
            if !crate::access::may_read_document(flat, lens, true, &did) {
                continue;
            }
            let Some(d) = flat.get("documents").and_then(|c| c.get(&did)) else {
                continue;
            };
            let s = |k: &str| d.get(k).and_then(Value::as_str).unwrap_or_default();
            let prefix = format!("doc.{did}");
            // Linked only when this archive holds the bytes: a file the
            // record names but keeps elsewhere has nothing here to open.
            let held = d
                .pointer("/file/path")
                .and_then(Value::as_str)
                .is_some_and(|p| state.payloads().path_of(p).is_some());
            let image = held && s("mime_type").starts_with("image/");
            mb.push(Block {
                prefix: prefix.clone(),
                kind: "document",
                title: s("filename").to_string(),
                subtitle: crate::i18n::vocab(lang, "document-type", s("document_type")),
                href: held.then(|| format!("/document/{did}/view")),
                thumb: image.then(|| format!("/document/{did}/thumb")),
                fields: vec![
                    Field::new(
                        "text",
                        format!("{prefix}.caption"),
                        cx.t("documents-caption"),
                        s("caption"),
                    ),
                    Field::select(
                        format!("{prefix}.document_type"),
                        cx.t("record-type"),
                        s("document_type"),
                        types.clone(),
                    ),
                    hidden(
                        format!("{prefix}.v"),
                        crate::state::version_of(d).to_string(),
                    ),
                ],
                remove: Some(check(format!("{prefix}.leave"), cx.t("edit-detach-file"))),
                ..Default::default()
            });
        }
        mb.push(Block {
            prefix: "new.doc".into(),
            kind: "new",
            add_label: cx.t("documents-upload"),
            subtitle: cx.t_args(
                "documents-upload-help",
                &[(
                    "mb",
                    (crate::documents::MAX_UPLOAD / (1024 * 1024)).to_string(),
                )],
            ),
            fields: vec![
                Field::new("file", "new.doc.file".into(), cx.t("record-file"), ""),
                Field::new(
                    "text",
                    "new.doc.caption".into(),
                    cx.t("documents-caption"),
                    "",
                ),
                Field::select(
                    "new.doc.document_type".into(),
                    cx.t("record-type"),
                    "",
                    std::iter::once(cx.not_set()).chain(types.clone()).collect(),
                ),
            ],
            ..Default::default()
        });

        let mut out_lists: BTreeMap<&'static str, Vec<Value>> = BTreeMap::new();
        out_lists.insert(
            "people",
            forms::person_options(flat, lens)
                .into_iter()
                .map(|o| json!({"id": o.id, "label": o.label}))
                .collect(),
        );
        Model {
            tabs,
            lists: out_lists,
        }
    })
}

fn union_block(cx: &Ctx, fid: &str, fam: &Value) -> Block {
    let prefix = format!("fam.{fid}");
    let partners: Vec<&Value> = fam
        .pointer("/union/persons")
        .and_then(Value::as_array)
        .map(|a| a.iter().collect())
        .unwrap_or_default();
    let others: Vec<String> = partners
        .iter()
        .filter_map(|p| p.get("person_id").and_then(Value::as_str))
        .filter(|p| *p != cx.id)
        .map(|p| cx.person_name(p))
        .collect();
    let mut partner_rows = Vec::new();
    for (i, p) in partners.iter().enumerate() {
        let Some(pid) = p.get("person_id").and_then(Value::as_str) else {
            continue;
        };
        if pid == cx.id {
            continue;
        }
        partner_rows.push(Row {
            title: cx.person_name(pid),
            href: cx.person_href(pid),
            fields: Vec::new(),
            remove: Some(check(
                format!("{prefix}.partner.{i}.remove"),
                cx.t("edit-remove"),
            )),
        });
    }
    let kids = fam
        .get("children")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let mut child_rows = Vec::new();
    for (i, c) in kids.iter().enumerate() {
        let Some(pid) = c.get("person_id").and_then(Value::as_str) else {
            continue;
        };
        child_rows.push(Row {
            title: cx.person_name(pid),
            href: cx.person_href(pid),
            fields: child_fields(
                cx,
                &format!("{prefix}.child.{i}"),
                c.get("lineage").and_then(Value::as_str).unwrap_or_default(),
                c.get("confidence").and_then(Value::as_f64),
            ),
            remove: Some(check(
                format!("{prefix}.child.{i}.remove"),
                cx.t("edit-remove"),
            )),
        });
    }
    let mut fields = vec![
        Field::select(
            format!("{prefix}.type"),
            cx.t("family-type"),
            fam.pointer("/union/type")
                .and_then(Value::as_str)
                .unwrap_or("unknown"),
            cx.vocab("union-type", UNION_TYPES),
        ),
        Field::select(
            format!("{prefix}.status"),
            cx.t("family-status"),
            fam.pointer("/union/status")
                .and_then(Value::as_str)
                .unwrap_or_default(),
            cx.vocab("union-status", UNION_STATUSES),
        ),
    ];
    fields.extend(cx.date_fields(
        &format!("{prefix}.start"),
        &cx.t("family-started"),
        fam.pointer("/union/start/date"),
    ));
    fields.push(hidden(
        format!("{prefix}.v"),
        crate::state::version_of(fam).to_string(),
    ));
    Block {
        prefix: prefix.clone(),
        kind: "union",
        title: if others.is_empty() {
            cx.t("edit-union-alone")
        } else {
            cx.t_args("edit-union-with", &[("names", others.join(" · "))])
        },
        fields,
        lists: vec![
            RowList {
                title: cx.t("family-partners"),
                rows: partner_rows,
                add: Some(Row {
                    fields: vec![Field {
                        list: "people".into(),
                        ..Field::new(
                            "picker",
                            format!("{prefix}.partner.new"),
                            cx.t("family-partner"),
                            "",
                        )
                    }],
                    ..Default::default()
                }),
                add_label: cx.t("edit-add-partner"),
            },
            RowList {
                title: cx.t("family-children"),
                rows: child_rows,
                add: Some(Row {
                    fields: std::iter::once(Field {
                        list: "people".into(),
                        ..Field::new(
                            "picker",
                            format!("{prefix}.child.new.person"),
                            cx.t("family-child"),
                            "",
                        )
                    })
                    .chain(child_fields(cx, &format!("{prefix}.child.new"), "", None))
                    .collect(),
                    ..Default::default()
                }),
                add_label: cx.t("edit-add-child"),
            },
        ],
        remove: Some(check(format!("{prefix}.leave"), cx.t("family-leave"))),
        ..Default::default()
    }
}

fn child_fields(cx: &Ctx, prefix: &str, lineage: &str, confidence: Option<f64>) -> Vec<Field> {
    vec![
        Field::select(
            format!("{prefix}.lineage"),
            cx.t("family-lineage"),
            lineage,
            cx.lineages(),
        ),
        Field::confidence(
            format!("{prefix}.confidence"),
            cx.t("phys-col-confidence"),
            confidence,
        ),
    ]
}

fn parents_block(cx: &Ctx, fid: &str, fam: &Value) -> Block {
    let prefix = format!("par.{fid}");
    let names: Vec<String> = fam
        .pointer("/union/persons")
        .and_then(Value::as_array)
        .map(|a| {
            a.iter()
                .filter_map(|p| p.get("person_id").and_then(Value::as_str))
                .map(|p| cx.person_name(p))
                .collect()
        })
        .unwrap_or_default();
    let mine = fam.get("children").and_then(Value::as_array).and_then(|a| {
        a.iter()
            .find(|c| c.get("person_id").and_then(Value::as_str) == Some(cx.id))
    });
    let mut fields = child_fields(
        cx,
        &prefix,
        mine.and_then(|c| c.get("lineage"))
            .and_then(Value::as_str)
            .unwrap_or_default(),
        mine.and_then(|c| c.get("confidence"))
            .and_then(Value::as_f64),
    );
    fields.push(hidden(
        format!("{prefix}.v"),
        crate::state::version_of(fam).to_string(),
    ));
    Block {
        prefix: prefix.clone(),
        kind: "parents",
        title: if names.is_empty() {
            cx.t("family-label-empty")
        } else {
            cx.t_args("edit-child-of", &[("names", names.join(" · "))])
        },
        fields,
        remove: Some(check(
            format!("{prefix}.leave"),
            cx.t("family-detach-child"),
        )),
        ..Default::default()
    }
}

fn new_family_block(cx: &Ctx) -> Block {
    Block {
        prefix: "new.fam".into(),
        kind: "new",
        add_label: cx.t("edit-new-family"),
        subtitle: cx.t("family-new-union-help"),
        fields: vec![
            Field {
                list: "people".into(),
                ..Field::new(
                    "picker",
                    "new.fam.partner".into(),
                    cx.t("family-partner"),
                    "",
                )
            },
            Field::select(
                "new.fam.type".into(),
                cx.t("family-type"),
                "",
                std::iter::once(cx.not_set())
                    .chain(cx.vocab("union-type", &UNION_TYPES[..5]))
                    .collect(),
            ),
            Field {
                list: "people".into(),
                ..Field::new("picker", "new.fam.child".into(), cx.t("family-child"), "")
            },
        ],
        ..Default::default()
    }
}

fn new_parents_block(cx: &Ctx) -> Block {
    let families: Vec<Opt> = std::iter::once(cx.not_set())
        .chain(
            super::editors::family_options(cx.flat, cx.lens)
                .into_iter()
                .map(|o| Opt {
                    value: o["id"].as_str().unwrap_or_default().to_string(),
                    label: o["label"].as_str().unwrap_or_default().to_string(),
                }),
        )
        .collect();
    let mut fields = vec![Field::select(
        "new.par.family".into(),
        cx.t("family-the-family"),
        "",
        families,
    )];
    fields.extend(child_fields(cx, "new.par", "", None));
    Block {
        prefix: "new.par".into(),
        kind: "new",
        add_label: cx.t("family-attach-parents"),
        subtitle: cx.t("family-attach-help"),
        fields,
        ..Default::default()
    }
}

/// Take a refused submission back into the model: every block drawn on that
/// form shows what was sent, and every tab that changed is marked.
pub fn refill(model: &mut Model, body: &Body) {
    for tab in &mut model.tabs {
        for b in &mut tab.blocks {
            if !body.contains_key(&b.marker()) {
                continue;
            }
            let touched = b.changed_in(body);
            for f in b.fields_mut() {
                f.fill(body);
            }
            b.touched = touched;
            tab.changed |= touched;
        }
    }
}

// ---------------------------------------------------------------------------
// reading back
// ---------------------------------------------------------------------------

/// A refusal of the save, and the tab it belongs to.
pub struct Refusal {
    pub tab: &'static str,
    pub message: String,
}

/// The records a save is about to write, each as it was and as it will be.
#[derive(Default)]
pub struct Working {
    /// `(collection, id)` → (kind, stored, edited, expected version).
    pub records: BTreeMap<(String, String), (EntityKind, Value, Value, u64)>,
    pub creates: Vec<Op>,
    pub deletes: Vec<Op>,
}

impl Working {
    /// The record as this save has it so far, read from the bundle the
    /// first time. `expected` is the version the form was drawn from.
    fn get(
        &mut self,
        flat: &Value,
        kind: EntityKind,
        id: &str,
        expected: Option<u64>,
    ) -> Option<&mut Value> {
        let collection = crate::admin::collection_for(kind).to_string();
        let key = (collection.clone(), id.to_string());
        if !self.records.contains_key(&key) {
            let stored = flat.get(&collection)?.get(id)?.clone();
            let version = expected.unwrap_or_else(|| crate::state::version_of(&stored));
            self.records
                .insert(key.clone(), (kind, stored.clone(), stored, version));
        }
        self.records.get_mut(&key).map(|r| &mut r.2)
    }

    fn deleting(&mut self, kind: EntityKind, id: &str, expected: u64) {
        let collection = crate::admin::collection_for(kind).to_string();
        self.records.remove(&(collection, id.to_string()));
        self.deletes.push(Op::Delete {
            kind,
            id: id.to_string(),
            expected_version: expected,
        });
    }
}

/// An uploaded file from the Médias tab.
#[derive(Default)]
pub struct Upload {
    pub filename: String,
    pub bytes: Vec<u8>,
    pub too_large: bool,
}

fn version(sub: &HashMap<String, String>) -> Option<u64> {
    sub.get("v").and_then(|v| v.trim().parse().ok())
}

fn on(sub: &HashMap<String, String>, key: &str) -> bool {
    sub.get(key).is_some_and(|v| v == "on")
}

fn text<'a>(sub: &'a HashMap<String, String>, key: &str) -> Option<&'a str> {
    sub.get(key).map(|s| s.trim()).filter(|s| !s.is_empty())
}

fn set_or_remove(o: &mut Map<String, Value>, key: &str, v: Option<Value>) {
    match v {
        Some(v) => {
            o.insert(key.into(), v);
        }
        None => {
            o.remove(key);
        }
    }
}

fn confidence_of(sub: &HashMap<String, String>, key: &str) -> Option<Value> {
    let mut row = BTreeMap::new();
    if let Some(v) = sub.get(key) {
        row.insert("c".to_string(), v.clone());
    }
    forms::confidence(&row, "c")
}

fn lineage_of(sub: &HashMap<String, String>, key: &str) -> Option<Value> {
    text(sub, key)
        .filter(|l| LINEAGE.contains(l))
        .map(|l| json!(l))
}

/// Read every changed block of the submission into `work` (other records)
/// and `person` (this one). Blocks that were not drawn, or were drawn and
/// left alone, are not read.
#[allow(clippy::too_many_arguments)]
pub fn read_back(
    state: &Shared,
    lang: &str,
    model: &Model,
    body: &Body,
    upload: Option<Upload>,
    id: &str,
    person: &mut Value,
    work: &mut Working,
) -> Result<(), Refusal> {
    let flat = state.read(|f| f.clone());
    let t = |key: &str| crate::i18n::translate(lang, key, None);
    let refuse = |tab: &'static str, key: &str| Refusal {
        tab,
        message: t(key),
    };
    let mut upload = upload;
    for (tab, b) in model.blocks() {
        if !b.changed_in(body) {
            continue;
        }
        let sub = b.sub(body);
        let (head, rest) = b.prefix.split_once('.').unwrap_or((&b.prefix, ""));
        match (head, rest) {
            ("fam", fid) => {
                let fam = work
                    .get(&flat, EntityKind::Family, fid, version(&sub))
                    .ok_or_else(|| refuse(tab, "family-error-no-family"))?;
                apply_union(fam, &sub, id, &flat).map_err(|k| refuse(tab, k))?;
            }
            ("par", fid) => {
                let fam = work
                    .get(&flat, EntityKind::Family, fid, version(&sub))
                    .ok_or_else(|| refuse(tab, "family-error-no-family"))?;
                apply_parents(fam, &sub, id);
            }
            ("ev", eid) => {
                let expected = version(&sub);
                let mut ev = work
                    .get(&flat, EntityKind::Event, eid, expected)
                    .ok_or_else(|| refuse(tab, "error-no-such-entity-title"))?
                    .clone();
                let mut map = sub.clone();
                super::admin::resolve_pickers(state, EntityKind::Event, &mut map, lang)
                    .map_err(|m| Refusal { tab, message: m })?;
                if text(&map, "category").is_none() {
                    return Err(refuse(tab, "event-error-no-category"));
                }
                ev = crate::admin::apply_form(ev, EntityKind::Event, &map);
                let mut ps: Vec<Value> = ev
                    .get("participants")
                    .and_then(Value::as_array)
                    .cloned()
                    .unwrap_or_default();
                let me = |p: &Value| {
                    p.get("entity_type").and_then(Value::as_str) == Some("person")
                        && p.get("entity_id").and_then(Value::as_str) == Some(id)
                };
                if on(&sub, "leave") {
                    ps.retain(|p| !me(p));
                } else if let Some(role) = text(&sub, "role").filter(|r| ROLES.contains(r)) {
                    for p in ps.iter_mut().filter(|p| me(p)) {
                        p["role"] = json!(role);
                    }
                }
                if ps.is_empty() {
                    // An event with nobody left in it was this person's alone.
                    let v = expected.unwrap_or(u64::MAX);
                    work.deleting(EntityKind::Event, eid, v);
                } else {
                    ev["participants"] = json!(ps);
                    if let Some(slot) = work.get(&flat, EntityKind::Event, eid, expected) {
                        *slot = ev;
                    }
                }
            }
            ("src", sid) => {
                let src = work
                    .get(&flat, EntityKind::Source, sid, version(&sub))
                    .ok_or_else(|| refuse(tab, "error-no-such-entity-title"))?;
                *src = crate::admin::apply_form(src.clone(), EntityKind::Source, &sub);
            }
            ("doc", did) => {
                if on(&sub, "leave") {
                    detach_document(person, did);
                    if let Some(d) = work.get(&flat, EntityKind::Document, did, version(&sub)) {
                        if let Some(l) = d.get_mut("linked_to").and_then(Value::as_array_mut) {
                            l.retain(|x| {
                                !(x.get("entity_type").and_then(Value::as_str) == Some("person")
                                    && x.get("entity_id").and_then(Value::as_str) == Some(id))
                            });
                        }
                    }
                } else {
                    let d = work
                        .get(&flat, EntityKind::Document, did, version(&sub))
                        .ok_or_else(|| refuse(tab, "error-no-such-entity-title"))?;
                    let o = d.as_object_mut().expect("a document is an object");
                    set_or_remove(o, "caption", text(&sub, "caption").map(|c| json!(c)));
                    if let Some(ty) = text(&sub, "document_type")
                        .filter(|t| crate::documents::DOCUMENT_TYPES.contains(t))
                    {
                        o.insert("document_type".into(), json!(ty));
                    }
                }
            }
            ("cite", "") => {
                for fact in ["birth", "death"] {
                    let chosen = text(&sub, fact).map(str::to_string);
                    if chosen.as_deref()
                        == person
                            .pointer(&format!("/{fact}/source_id"))
                            .and_then(Value::as_str)
                    {
                        continue;
                    }
                    cite(person, fact, chosen);
                }
            }
            ("new", "fam") => {
                let mut persons = vec![json!({"person_id": id, "role": "spouse"})];
                if let Some(raw) = text(&sub, "partner") {
                    let pid =
                        forms::resolve_person(raw, &flat).map_err(|e| refuse(tab, e.key()))?;
                    if pid != id {
                        persons.push(json!({"person_id": pid, "role": "spouse"}));
                    }
                }
                let kind = text(&sub, "type")
                    .filter(|k| UNION_TYPES.contains(k))
                    .unwrap_or("unknown");
                let mut fam = json!({
                    "id": uuid::Uuid::new_v4().to_string(),
                    "type": "family",
                    "axgf_version": "1.0",
                    "union": {"type": kind, "persons": persons},
                });
                if let Some(raw) = text(&sub, "child") {
                    let pid =
                        forms::resolve_person(raw, &flat).map_err(|e| refuse(tab, e.key()))?;
                    fam["children"] = json!([{"person_id": pid}]);
                }
                work.creates.push(Op::Create {
                    kind: EntityKind::Family,
                    entity: fam,
                });
            }
            ("new", "par") => {
                let fid = text(&sub, "family")
                    .ok_or_else(|| refuse(tab, "family-error-no-family"))?
                    .to_string();
                let fam = work
                    .get(&flat, EntityKind::Family, &fid, None)
                    .ok_or_else(|| refuse(tab, "family-error-no-family"))?;
                let mut kids = fam
                    .get("children")
                    .and_then(Value::as_array)
                    .cloned()
                    .unwrap_or_default();
                if kids
                    .iter()
                    .any(|c| c.get("person_id").and_then(Value::as_str) == Some(id))
                {
                    return Err(refuse(tab, "family-error-already-child"));
                }
                let mut c = Map::new();
                c.insert("person_id".into(), json!(id));
                set_or_remove(&mut c, "lineage", lineage_of(&sub, "lineage"));
                set_or_remove(&mut c, "confidence", confidence_of(&sub, "confidence"));
                kids.push(Value::Object(c));
                fam["children"] = json!(kids);
            }
            ("new", "ev") => {
                let mut map = sub.clone();
                super::admin::resolve_pickers(state, EntityKind::Event, &mut map, lang)
                    .map_err(|m| Refusal { tab, message: m })?;
                if text(&map, "category").is_none() {
                    return Err(refuse(tab, "event-error-no-category"));
                }
                let mut ev = crate::admin::apply_form(
                    json!({"type": "event", "axgf_version": "1.0"}),
                    EntityKind::Event,
                    &map,
                );
                ev["id"] = json!(uuid::Uuid::new_v4().to_string());
                let role = text(&sub, "role")
                    .filter(|r| ROLES.contains(r))
                    .unwrap_or("subject");
                ev["participants"] =
                    json!([{"entity_type": "person", "entity_id": id, "role": role}]);
                work.creates.push(Op::Create {
                    kind: EntityKind::Event,
                    entity: ev,
                });
            }
            ("new", "src") => {
                let mut map = sub.clone();
                let attach = map.remove("attach");
                if text(&map, "title").is_none() {
                    return Err(refuse(tab, "edit-error-source-title"));
                }
                let sid = uuid::Uuid::new_v4().to_string();
                let mut src = crate::admin::apply_form(
                    json!({"type": "source", "axgf_version": "1.0"}),
                    EntityKind::Source,
                    &map,
                );
                src["id"] = json!(sid);
                work.creates.push(Op::Create {
                    kind: EntityKind::Source,
                    entity: src,
                });
                if let Some(fact) = attach.as_deref().filter(|f| ["birth", "death"].contains(f)) {
                    cite(person, fact, Some(sid));
                }
            }
            ("new", "doc") => {
                let up = upload.take().unwrap_or_default();
                if up.too_large || up.bytes.len() > crate::documents::MAX_UPLOAD {
                    return Err(Refusal {
                        tab,
                        message: crate::i18n::translate(
                            lang,
                            "error-upload-too-large",
                            Some(&fluent::FluentArgs::from_iter([(
                                "mb",
                                fluent::FluentValue::from(
                                    (crate::documents::MAX_UPLOAD / (1024 * 1024)) as i64,
                                ),
                            )])),
                        ),
                    });
                }
                if up.bytes.is_empty() {
                    // A caption or a type with no file: nothing to add.
                    continue;
                }
                let Some(kind) = crate::documents::sniff(&up.bytes) else {
                    return Err(refuse(tab, "error-upload-unsupported"));
                };
                let doc_id = uuid::Uuid::new_v4().to_string();
                let doc_type = match text(&sub, "document_type") {
                    Some(t) if crate::documents::DOCUMENT_TYPES.contains(&t) => t.to_string(),
                    _ if kind.raster_image => "photo".to_string(),
                    _ => "other".to_string(),
                };
                let mut entity = json!({
                    "id": doc_id,
                    "type": "document",
                    "axgf_version": "1.0",
                    "filename": super::admin::clean_filename(&up.filename, kind.ext),
                    "mime_type": kind.mime,
                    "document_type": doc_type,
                    "status": "present",
                    "file": {
                        "path": crate::documents::attachment_path(&doc_id, kind.ext),
                        "size_bytes": up.bytes.len() as u64,
                        "sha256": crate::documents::sha256_hex(&up.bytes),
                    },
                    "linked_to": [{"entity_type": "person", "entity_id": id, "role": "subject"}],
                });
                if let Some(c) = text(&sub, "caption") {
                    entity["caption"] = json!(c);
                }
                work.creates.push(Op::AddDocument {
                    entity,
                    payload: up.bytes,
                });
            }
            _ => {}
        }
    }
    Ok(())
}

/// Point one of this person's own facts at a source, or at none.
fn cite(person: &mut Value, fact: &str, source: Option<String>) {
    match source {
        Some(s) => {
            if !person.get(fact).is_some_and(Value::is_object) {
                person[fact] = json!({});
            }
            person[fact]["source_id"] = json!(s);
        }
        None => {
            if let Some(o) = person.get_mut(fact).and_then(Value::as_object_mut) {
                o.remove("source_id");
            }
        }
    }
}

fn detach_document(person: &mut Value, did: &str) {
    if let Some(list) = person.get_mut("documents").and_then(Value::as_array_mut) {
        list.retain(|d| d.get("document_id").and_then(Value::as_str) != Some(did));
        if list.is_empty() {
            person.as_object_mut().map(|o| o.remove("documents"));
        }
    }
}

/// One union, as its block changed it. Rows are the stored arrays' own
/// positions, so a removal names exactly the partner or child it was drawn
/// beside; what the block does not draw — the end of the union, its place,
/// its source, a child's birth order and note — is kept.
fn apply_union(
    fam: &mut Value,
    sub: &HashMap<String, String>,
    me: &str,
    flat: &Value,
) -> Result<(), &'static str> {
    let union = fam
        .get_mut("union")
        .and_then(Value::as_object_mut)
        .ok_or("family-error-last-partner")?;
    union.insert(
        "type".into(),
        json!(text(sub, "type")
            .filter(|t| UNION_TYPES.contains(t))
            .unwrap_or("unknown")),
    );
    set_or_remove(
        union,
        "status",
        text(sub, "status")
            .filter(|s| UNION_STATUSES.contains(s))
            .map(|s| json!(s)),
    );
    let date = forms::date_object(
        sub.get("start.date").map(String::as_str),
        sub.get("start.precision").map(String::as_str),
        on(sub, "start.circa"),
    );
    match (date, union.get_mut("start").and_then(Value::as_object_mut)) {
        (Some(d), Some(start)) => {
            start.insert("date".into(), d);
        }
        (Some(d), None) => {
            union.insert("start".into(), json!({"date": d}));
        }
        (None, Some(start)) => {
            start.remove("date");
            if start.is_empty() {
                union.remove("start");
            }
        }
        (None, None) => {}
    }

    let stored: Vec<Value> = union
        .get("persons")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let mut persons = Vec::new();
    for (i, p) in stored.into_iter().enumerate() {
        let is_me = p.get("person_id").and_then(Value::as_str) == Some(me);
        let gone = if is_me {
            on(sub, "leave")
        } else {
            on(sub, &format!("partner.{i}.remove"))
        };
        if !gone {
            persons.push(p);
        }
    }
    if let Some(raw) = text(sub, "partner.new") {
        let pid = forms::resolve_person(raw, flat).map_err(|e| e.key())?;
        if !persons
            .iter()
            .any(|p| p.get("person_id").and_then(Value::as_str) == Some(pid.as_str()))
        {
            persons.push(json!({"person_id": pid, "role": "spouse"}));
        }
    }
    if persons.is_empty() {
        return Err("family-error-last-partner");
    }
    union.insert("persons".into(), json!(persons));

    let stored: Vec<Value> = fam
        .get("children")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let mut kids = Vec::new();
    for (i, c) in stored.into_iter().enumerate() {
        if on(sub, &format!("child.{i}.remove")) {
            continue;
        }
        let mut c = c.as_object().cloned().unwrap_or_default();
        set_or_remove(
            &mut c,
            "lineage",
            lineage_of(sub, &format!("child.{i}.lineage")),
        );
        set_or_remove(
            &mut c,
            "confidence",
            confidence_of(sub, &format!("child.{i}.confidence")),
        );
        kids.push(Value::Object(c));
    }
    if let Some(raw) = text(sub, "child.new.person") {
        let pid = forms::resolve_person(raw, flat).map_err(|e| e.key())?;
        if !kids
            .iter()
            .any(|c| c.get("person_id").and_then(Value::as_str) == Some(pid.as_str()))
        {
            let mut c = Map::new();
            c.insert("person_id".into(), json!(pid));
            set_or_remove(&mut c, "lineage", lineage_of(sub, "child.new.lineage"));
            set_or_remove(
                &mut c,
                "confidence",
                confidence_of(sub, "child.new.confidence"),
            );
            kids.push(Value::Object(c));
        }
    }
    if kids.is_empty() {
        fam.as_object_mut().map(|o| o.remove("children"));
    } else {
        fam["children"] = json!(kids);
    }
    Ok(())
}

/// The family this person is a child of, as its block changed it: their own
/// child entry, and nothing else.
fn apply_parents(fam: &mut Value, sub: &HashMap<String, String>, me: &str) {
    let Some(kids) = fam.get_mut("children").and_then(Value::as_array_mut) else {
        return;
    };
    let mine = |c: &Value| c.get("person_id").and_then(Value::as_str) == Some(me);
    if on(sub, "leave") {
        kids.retain(|c| !mine(c));
        if kids.is_empty() {
            fam.as_object_mut().map(|o| o.remove("children"));
        }
        return;
    }
    for c in kids.iter_mut().filter(|c| mine(c)) {
        if let Some(o) = c.as_object_mut() {
            set_or_remove(o, "lineage", lineage_of(sub, "lineage"));
            set_or_remove(o, "confidence", confidence_of(sub, "confidence"));
        }
    }
}

// ---------------------------------------------------------------------------
// handlers
// ---------------------------------------------------------------------------

/// `GET /admin/person/:id/edit`
pub async fn edit(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(id): Path<String>,
    axum::extract::Query(q): axum::extract::Query<HashMap<String, String>>,
) -> Response {
    let (viewer, chrome) = match require(&state, &headers, Need::Write) {
        Ok(v) => v,
        Err(r) => return r,
    };
    let Some(stored) = state.read(|f| f.get("persons").and_then(|c| c.get(&id)).cloned()) else {
        return no_person(&chrome);
    };
    let saved = q.get("saved").and_then(|n| n.parse::<usize>().ok());
    render_editor(&state, &chrome, &viewer, &id, &stored, Refused::None, saved)
}

/// Why the editor is being drawn again.
pub enum Refused<'a> {
    /// Not refused: drawn from the bundle.
    None,
    /// The save was refused; draw what was sent.
    With {
        body: &'a Body,
        /// The Fiche's fields as sent (the raw document and the fields).
        entity: Value,
        raw: String,
        problems: &'a crate::profile::form::Problems,
        message: String,
        tab: &'static str,
    },
}

/// Draw the editor.
pub fn render_editor(
    state: &Shared,
    chrome: &render::Chrome,
    viewer: &Viewer,
    id: &str,
    stored: &Value,
    refused: Refused,
    saved: Option<usize>,
) -> Response {
    let k = EntityKind::Person;
    let readable = crate::access::readable_for(k, stored, viewer.ceiling());
    let shown = super::admin::shown_entity(stored, readable).into_owned();
    let pickers = super::admin::picker_lists(state, viewer, k, chrome.lang);
    let mut event_lists = super::admin::picker_lists(state, viewer, EntityKind::Event, chrome.lang);
    event_lists.extend(pickers.clone());
    let mut model = model(state, viewer, chrome.lang, id, stored, &event_lists);
    let (fiche_entity, raw, body, problems, error, error_tab) = match &refused {
        Refused::None => (
            shown.clone(),
            serde_json::to_string_pretty(&shown).unwrap_or_else(|_| "{}".into()),
            None,
            crate::profile::form::Problems::new(),
            None,
            None,
        ),
        Refused::With {
            body,
            entity,
            raw,
            problems,
            message,
            tab,
        } => (
            entity.clone(),
            raw.clone(),
            Some(*body),
            (*problems).clone(),
            Some(message.clone()),
            Some(*tab),
        ),
    };
    if let Some(b) = body {
        refill(&mut model, b);
    }
    let fiche = generic_fields(
        &super::admin::field_views(k, &fiche_entity, chrome.lang, &pickers),
        "",
    );
    let p = state.read_as(viewer.ceiling(), |flat, lens| {
        crate::person::build_in(flat, id, lens, chrome.lang)
    });
    let relatives = p
        .as_ref()
        .map(|p| crate::person::RecordTab::Family.count(p))
        .unwrap_or_default();
    let profile = super::profile::profile_form(
        state,
        chrome.lang,
        viewer,
        id,
        stored,
        relatives,
        body,
        &problems,
    );
    let profile_changed = body.is_some_and(|b| {
        profile.sections.iter().any(|s| {
            b.contains_key(&super::profile::group_marker(s.group.key))
                && s.group.attributes.iter().any(|a| !a.problems.is_empty())
        })
    }) || !problems.is_empty();
    if let Some(t) = model.tabs.iter_mut().find(|t| t.key == "profil") {
        t.changed |= profile_changed;
    }
    if let (Some(b), Some(t)) = (body, model.tabs.iter_mut().find(|t| t.key == "fiche")) {
        // Against the Fiche as stored, not as sent: what differs is unsaved.
        let mut drawn = Body::new();
        for f in generic_fields(
            &super::admin::field_views(k, &shown, chrome.lang, &pickers),
            "",
        ) {
            f.posted(&mut drawn);
        }
        t.changed |= drawn
            .iter()
            .any(|(name, v)| v.trim() != b.get(name).map(|s| s.trim()).unwrap_or(""))
            || b.get("raw_json")
                .is_some_and(|r| serde_json::from_str::<Value>(r).ok().as_ref() != Some(&shown));
    }
    let mut status = StatusCode::OK;
    if matches!(refused, Refused::With { .. }) {
        status = StatusCode::UNPROCESSABLE_ENTITY;
    }
    let tab = error_tab
        .or_else(|| body.and_then(|b| b.get("tab")).and_then(|t| known_tab(t)))
        .unwrap_or("fiche");
    let mut resp = render::page_with(
        chrome,
        "admin_person.html",
        context! {
            nav => "admin",
            id,
            person_name => crate::view::person_display_name(stored),
            tabs => model.tabs,
            tab,
            datalists => model.lists,
            pickers,
            event_pickers => event_lists,
            fiche,
            raw,
            error,
            saved,
            unsaved => body.is_some(),
            base_version => body
                .and_then(|b| b.get("base_version").cloned())
                .unwrap_or_else(|| crate::state::version_of(stored).to_string()),
            profile_sections => profile.sections,
            profile_sources => profile.sources,
            precisions => profile.precisions,
            is_admin => viewer.is_admin(),
            history => super::admin::history_json(state, "person", id, viewer.ceiling(), chrome.lang),
            max_upload_mb => crate::documents::MAX_UPLOAD / (1024 * 1024),
        },
    );
    *resp.status_mut() = status;
    resp
}

fn no_person(chrome: &render::Chrome) -> Response {
    render::error_page_in(
        chrome,
        StatusCode::NOT_FOUND,
        "error-no-such-person-title",
        "error-no-such-person-detail",
    )
}

/// Read the save's body: a form, or — when the Médias tab carries a file — a
/// multipart form with the file in it.
async fn read_body(req: Request) -> (Body, Option<Upload>) {
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
        return (form.into_iter().collect(), None);
    }
    let mut body = Body::new();
    let mut upload = Upload::default();
    let Ok(mut mp) = axum::extract::Multipart::from_request(req, &()).await else {
        return (body, None);
    };
    loop {
        let field = match mp.next_field().await {
            Ok(Some(f)) => f,
            Ok(None) => break,
            Err(e) => {
                upload.too_large = e.status() == StatusCode::PAYLOAD_TOO_LARGE;
                break;
            }
        };
        let name = field.name().unwrap_or_default().to_string();
        if name == "new.doc.file" {
            upload.filename = field.file_name().unwrap_or("upload").to_string();
            match field.bytes().await {
                Ok(b) => upload.bytes = b.to_vec(),
                Err(_) => upload.too_large = true,
            }
        } else {
            body.insert(name, field.text().await.unwrap_or_default());
        }
    }
    (body, Some(upload))
}

/// `POST /admin/person/:id` — the one save of every tab.
pub async fn save(
    State(state): State<Shared>,
    headers: HeaderMap,
    Path(id): Path<String>,
    req: Request,
) -> Response {
    let (viewer, chrome) = match require(&state, &headers, Need::Write) {
        Ok(v) => v,
        Err(r) => return r,
    };
    let k = EntityKind::Person;
    let Some(stored) = state.read(|f| f.get("persons").and_then(|c| c.get(&id)).cloned()) else {
        return no_person(&chrome);
    };
    let (body, upload) = read_body(req).await;
    if upload.as_ref().is_some_and(|u| u.too_large)
        && upload.as_ref().is_some_and(|u| u.bytes.is_empty())
    {
        // The body limit cut the request short: nothing that came with it
        // can be trusted to be whole.
        return render::error_page_args(
            &chrome,
            StatusCode::PAYLOAD_TOO_LARGE,
            "error-upload-title",
            "error-upload-too-large",
            &[(
                "mb",
                ((crate::documents::MAX_UPLOAD / (1024 * 1024)) as i64).into(),
            )],
        );
    }
    let mut form: HashMap<String, String> =
        body.iter().map(|(a, b)| (a.clone(), b.clone())).collect();
    let raw_sent = form.get("raw_json").cloned().unwrap_or_else(|| "{}".into());
    let no_problems = crate::profile::form::Problems::new();
    let refused =
        |entity: Value, problems: &crate::profile::form::Problems, message: String, tab| {
            render_editor(
                &state,
                &chrome,
                &viewer,
                &id,
                &stored,
                Refused::With {
                    body: &body,
                    entity,
                    raw: raw_sent.clone(),
                    problems,
                    message,
                    tab,
                },
                None,
            )
        };
    let fiche_as_sent = |form: &HashMap<String, String>| {
        let base = serde_json::from_str::<Value>(&raw_sent).unwrap_or(json!({}));
        crate::admin::apply_form(base, k, form)
    };

    // The Fiche: the generic person form, as it always was.
    if let Err(msg) = super::admin::resolve_pickers(&state, k, &mut form, chrome.lang) {
        return refused(fiche_as_sent(&form), &no_problems, msg, "fiche");
    }
    let base = match super::admin::base_from_raw(&form, chrome.lang) {
        Ok(v) => v,
        Err(msg) => return refused(fiche_as_sent(&form), &no_problems, msg, "fiche"),
    };
    let mut entity = crate::admin::apply_form(base, k, &form);
    entity["id"] = Value::String(id.clone());

    // The Profil: every group the editor changed.
    entity = match super::profile::apply_profile(
        &state,
        chrome.lang,
        &viewer,
        &id,
        &stored,
        entity,
        &body,
    ) {
        Ok(e) => e,
        Err(problems) => {
            return refused(
                fiche_as_sent(&form),
                &problems,
                chrome.t("profile-problems"),
                "profil",
            )
        }
    };
    entity["id"] = Value::String(id.clone());

    // Famille, Événements, Sources, Médias: every block that changed.
    let lists = super::admin::picker_lists(&state, &viewer, EntityKind::Event, chrome.lang);
    let model = model(&state, &viewer, chrome.lang, &id, &stored, &lists);
    let mut work = Working::default();
    if let Err(r) = read_back(
        &state,
        chrome.lang,
        &model,
        &body,
        upload,
        &id,
        &mut entity,
        &mut work,
    ) {
        return refused(fiche_as_sent(&form), &no_problems, r.message, r.tab);
    }

    // What this editor may not read, they did not see and cannot have meant
    // to change: put back as stored.
    let readable = crate::access::readable_for(k, &stored, viewer.ceiling());
    entity = crate::sensitive::restore(&entity, &stored, readable);

    // The ops: what is new first, so what follows may name it; then every
    // record that changed; then what goes.
    let mut ops: Vec<Op> = work.creates.clone();
    let pending: Vec<String> = ops
        .iter()
        .filter_map(|op| match op {
            Op::Create { entity, .. } | Op::AddDocument { entity, .. } => {
                entity.get("id").and_then(Value::as_str).map(str::to_string)
            }
            _ => None,
        })
        .collect();
    let person_version = super::admin::submitted_version(&body);
    if entity != stored {
        work.records.insert(
            ("persons".into(), id.clone()),
            (k, stored.clone(), entity.clone(), person_version),
        );
    }
    let mut checks: Vec<(EntityKind, Value, Option<Value>)> = Vec::new();
    for op in &work.creates {
        match op {
            Op::Create { kind, entity } => checks.push((*kind, entity.clone(), None)),
            Op::AddDocument { entity, .. } => {
                checks.push((EntityKind::Document, entity.clone(), None))
            }
            _ => {}
        }
    }
    for ((_, rid), (kind, before, after, expected)) in &work.records {
        if after == before {
            continue;
        }
        let mut e = after.clone();
        e["id"] = json!(rid);
        checks.push((*kind, e.clone(), Some(before.clone())));
        ops.push(Op::Update {
            kind: *kind,
            id: rid.clone(),
            expected_version: *expected,
            entity: e,
        });
    }
    for op in &work.deletes {
        if let Op::Delete { kind, id: did, .. } = op {
            if let Some(before) = state.read(|f| {
                f.get(crate::admin::collection_for(*kind))
                    .and_then(|c| c.get(did))
                    .cloned()
            }) {
                checks.push((*kind, before.clone(), Some(before)));
            }
        }
    }
    ops.extend(work.deletes.iter().cloned());
    for (kind, e, before) in &checks {
        if let Err(r) =
            super::admin::check_scope(&state, &chrome, &viewer, *kind, e, before.as_ref())
        {
            return r;
        }
        if let Err(msg) =
            super::admin::refuse_new_dangling(&state, e, before.as_ref(), chrome.lang, &pending)
        {
            return refused(fiche_as_sent(&form), &no_problems, msg, tab_of(&body));
        }
    }

    if ops.is_empty() {
        return axum::response::Redirect::to(&format!(
            "/admin/person/{id}/edit?saved=0#{}",
            tab_of(&body)
        ))
        .into_response();
    }
    let outcome = match state.commit(ops, viewer.name(), |kind, e| {
        super::admin::label_for(kind, e)
    }) {
        Ok(o) => o,
        Err(e) => return super::admin::io_error(&chrome, &e),
    };
    match outcome {
        crate::state::BatchOutcome::Applied { records } => axum::response::Redirect::to(&format!(
            "/admin/person/{id}/edit?saved={records}#{}",
            tab_of(&body)
        ))
        .into_response(),
        crate::state::BatchOutcome::Conflict { ids } => {
            // The person themselves: the real conflict screen, which shows
            // what the other editor changed beside what this one did.
            if ids.contains(&id) {
                let current = state
                    .read(|f| f.get("persons").and_then(|c| c.get(&id)).cloned())
                    .unwrap_or(Value::Null);
                return super::admin::conflict_page(
                    &state,
                    &chrome,
                    "person",
                    k,
                    &id,
                    Some(&stored),
                    &current,
                    &entity,
                    crate::state::version_of(&current),
                    person_version,
                    viewer.ceiling(),
                );
            }
            let names: Vec<String> = ids
                .iter()
                .map(|i| format!("#{}", forms::short_id(i)))
                .collect();
            refused(
                fiche_as_sent(&form),
                &no_problems,
                chrome.t_args("edit-stale", &[("what", names.join(", ").into())]),
                tab_of(&body),
            )
        }
        crate::state::BatchOutcome::Refused {
            id: rid,
            diagnostics,
        } => refused(
            fiche_as_sent(&form),
            &no_problems,
            chrome.t_args(
                "edit-refused",
                &[(
                    "reason",
                    format!(
                        "#{} — {}",
                        forms::short_id(&rid),
                        crate::state::format_diagnostics(&diagnostics)
                    )
                    .into(),
                )],
            ),
            tab_of(&body),
        ),
    }
}

/// The tab the editor was on when they saved (`tab`, kept by editor.js).
fn tab_of(body: &Body) -> &'static str {
    body.get("tab")
        .and_then(|t| known_tab(t))
        .unwrap_or("fiche")
}

/// `GET /admin/person/new`. The editor's `/admin/person/:id` would otherwise
/// take the literal `new` for an id; it is the generic form's, as for every
/// kind.
pub async fn new_form(state: State<Shared>, headers: HeaderMap) -> Response {
    super::admin::new_form(state, headers, Path("person".to_string())).await
}

/// `POST /admin/person/bulk`, for the same reason: the listing's bulk bar.
pub async fn bulk(
    state: State<Shared>,
    headers: HeaderMap,
    form: axum::extract::Form<Vec<(String, String)>>,
) -> Response {
    super::admin::bulk(state, headers, Path("person".to_string()), form).await
}
