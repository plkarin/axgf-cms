//! The identity view: everything known about one person, assembled for
//! rendering.
//!
//! This module resolves references — a `place_id` into a place name, a
//! `source_id` into a graded source — and groups the results the way the page
//! shows them. It interprets nothing: confidences, dates and relationships are
//! read as `axgf-rs` wrote them.
//!
//! A referenced person who is not in the bundle becomes `[Unknown]` with no
//! link, never a broken page.

use serde::Serialize;
use serde_json::Value;

use crate::view::{self, Confidence, DateDisplay};

/// A reference to another person, resolved as far as the bundle allows.
#[derive(Debug, Clone, Serialize)]
pub struct PersonRef {
    pub id: String,
    pub name: String,
    /// False when the id is referenced but absent from the bundle.
    pub known: bool,
    pub confidence: Option<Confidence>,
    /// Extra context: a role, a birth order, a note.
    pub detail: Option<String>,
}

/// A place, resolved with its historical country context.
#[derive(Debug, Clone, Serialize)]
pub struct PlaceView {
    pub name: String,
    pub known: bool,
    pub place_type: Option<String>,
    pub country_current: Option<String>,
    /// Which countries this place belonged to, and when. GEDCOM has no way to
    /// say that a town changed hands.
    pub country_history: Vec<String>,
    pub note: Option<String>,
}

/// A dated fact such as birth or death.
#[derive(Debug, Clone, Serialize)]
pub struct FactView {
    pub label: &'static str,
    pub date: DateDisplay,
    pub place: Option<PlaceView>,
    pub confidence: Option<Confidence>,
    pub source: Option<SourceView>,
    pub cause: Option<String>,
    pub present: bool,
}

/// A source with its reliability grade.
#[derive(Debug, Clone, Serialize)]
pub struct SourceView {
    pub id: String,
    pub title: String,
    pub source_type: Option<String>,
    pub reliability: String,
    pub reliability_label: &'static str,
    pub reliability_rank: u8,
    pub confidence: Option<Confidence>,
    pub status: Option<String>,
    pub repository: Option<String>,
    pub known: bool,
}

/// An alternative name with its own validity period and source.
#[derive(Debug, Clone, Serialize)]
pub struct NameView {
    pub display: String,
    pub kind: String,
    pub valid_from: Option<String>,
    pub valid_until: Option<String>,
    pub confidence: Option<Confidence>,
    pub source: Option<SourceView>,
    pub note: Option<String>,
}

/// A non-family link: godparent, employer, witness, mentor.
///
/// This is the clearest thing AXGF expresses that GEDCOM cannot, so it gets
/// its own prominent section rather than being folded into the family lists.
#[derive(Debug, Clone, Serialize)]
pub struct LinkView {
    /// The label as it reads from this person's point of view.
    pub label: String,
    pub other: PersonRef,
    /// `outgoing` when this person is the `from` end, `incoming` otherwise.
    pub direction: &'static str,
    pub category: Option<String>,
    pub from_date: Option<DateDisplay>,
    pub until_date: Option<DateDisplay>,
    pub confidence: Option<Confidence>,
    pub source: Option<SourceView>,
    pub note: Option<String>,
}

/// An occupation, which is a span with a duration rather than an event.
#[derive(Debug, Clone, Serialize)]
pub struct OccupationView {
    pub title: String,
    pub employer: Option<String>,
    pub place: Option<PlaceView>,
    pub from: Option<DateDisplay>,
    pub until: Option<DateDisplay>,
    /// Human span, e.g. "1948–1978".
    pub span: String,
    pub confidence: Option<Confidence>,
    pub source: Option<SourceView>,
    pub note: Option<String>,
    /// Bar geometry as percentages of the shared timeline, so several
    /// occupations can be compared at a glance.
    pub bar_left: f64,
    pub bar_width: f64,
    /// True when a bound is missing and the bar is therefore open-ended.
    pub open_start: bool,
    pub open_end: bool,
}

/// A document attached to this person.
#[derive(Debug, Clone, Serialize)]
pub struct DocumentView {
    pub id: String,
    pub filename: String,
    pub document_type: String,
    pub status: String,
    pub caption: Option<String>,
    pub role: Option<String>,
    pub known: bool,
}

/// An event this person took part in.
#[derive(Debug, Clone, Serialize)]
pub struct EventView {
    pub category: String,
    pub subcategory: Option<String>,
    pub date: DateDisplay,
    pub place: Option<PlaceView>,
    pub role: Option<String>,
    pub description: Option<String>,
    pub confidence: Option<Confidence>,
    pub source: Option<SourceView>,
}

/// Everything the identity page shows.
#[derive(Debug, Clone, Serialize)]
pub struct PersonView {
    pub id: String,
    pub name: String,
    pub names: Vec<NameView>,
    pub gender: Option<String>,
    pub gender_note: Option<String>,
    pub is_living: bool,
    pub birth: FactView,
    pub death: FactView,
    pub bio: Option<String>,
    pub notes: Option<String>,
    pub parents: Vec<PersonRef>,
    pub spouses: Vec<PersonRef>,
    pub siblings: Vec<PersonRef>,
    pub children: Vec<PersonRef>,
    pub links: Vec<LinkView>,
    pub occupations: Vec<OccupationView>,
    pub events: Vec<EventView>,
    pub sources: Vec<SourceView>,
    pub documents: Vec<DocumentView>,
    /// Timeline axis labels for the occupation chart.
    pub timeline_from: i64,
    pub timeline_to: i64,
    /// False when no occupation carries a single usable year — a GEDCOM
    /// import records occupation titles with no dates at all. Drawing an axis
    /// then would invent a scale the data does not support.
    pub has_timeline: bool,
    /// Which GEDCOM-impossible features this person actually demonstrates.
    pub showcase_notes: Vec<String>,
}

/// Build the identity view for `id`, or `None` when no such person exists.
pub fn build(flat: &Value, id: &str) -> Option<PersonView> {
    let person = flat.get("persons")?.get(id)?;
    let ctx = Ctx { flat };

    let identity = person.get("identity");
    let name = view::person_display_name(person);

    let gender = identity
        .and_then(|i| i.get("gender"))
        .and_then(|g| g.get("value"))
        .and_then(Value::as_str)
        .map(|g| {
            match g {
                "M" => "Male",
                "F" => "Female",
                "NB" => "Non-binary",
                _ => "Unrecorded",
            }
            .to_string()
        });
    let gender_note = identity
        .and_then(|i| i.get("gender"))
        .and_then(|g| g.get("note"))
        .and_then(Value::as_str)
        .map(str::to_string);

    let is_living = identity
        .and_then(|i| i.get("is_living"))
        .and_then(Value::as_bool)
        .unwrap_or(false);

    // Alternative names, each with its own validity window and source.
    let names: Vec<NameView> = identity
        .and_then(|i| i.get("names"))
        .and_then(Value::as_array)
        .map(|arr| {
            arr.iter()
                .map(|n| NameView {
                    display: n
                        .get("display")
                        .and_then(Value::as_str)
                        .unwrap_or("[Unnamed]")
                        .to_string(),
                    kind: n
                        .get("type")
                        .and_then(Value::as_str)
                        .unwrap_or("other")
                        .replace('_', " "),
                    valid_from: str_field(n, "valid_from"),
                    valid_until: str_field(n, "valid_until"),
                    confidence: Confidence::from_field(n, "confidence"),
                    source: ctx.source_of(n),
                    note: str_field(n, "note"),
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let birth = ctx.fact(person, "birth", "Born");
    let death = ctx.fact(person, "death", "Died");

    let (parents, spouses, siblings, children) = ctx.relations(id);
    let links = ctx.links_for(id);
    let (occupations, timeline_from, timeline_to, has_timeline) = ctx.occupations_for(id);
    let events = ctx.events_for(id);
    let documents = ctx.documents_for(id, person);
    let sources = ctx.sources_for(&birth, &death, &names, &links, &occupations, &events);

    let mut showcase_notes = Vec::new();
    if !links.is_empty() {
        showcase_notes.push(format!(
            "{} non-family relationship{} with their own dates, sources and confidence",
            links.len(),
            if links.len() == 1 { "" } else { "s" }
        ));
    }
    if !occupations.is_empty() {
        let n = occupations.len();
        showcase_notes.push(if n == 1 {
            "an occupation recorded as a span rather than an event".to_string()
        } else {
            format!("{n} occupations recorded as spans rather than events")
        });
    }
    for (noun, f) in [("birth", &birth), ("death", &death)] {
        if f.present && matches!(f.date.kind, "range" | "approximate" | "preserved") {
            showcase_notes.push(format!(
                "a {noun} date the source could not pin down, shown as recorded"
            ));
        }
    }
    if names.len() > 1 {
        showcase_notes.push(format!("{} recorded names", names.len() + 1));
    }

    Some(PersonView {
        id: id.to_string(),
        name,
        names,
        gender,
        gender_note,
        is_living,
        birth,
        death,
        bio: str_field(person, "bio"),
        notes: str_field(person, "notes"),
        parents,
        spouses,
        siblings,
        children,
        links,
        occupations,
        events,
        sources,
        documents,
        timeline_from,
        timeline_to,
        has_timeline,
        showcase_notes,
    })
}

/// Read a non-empty string field.
fn str_field(v: &Value, key: &str) -> Option<String> {
    v.get(key)
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

/// Lookup helpers bound to one bundle.
struct Ctx<'a> {
    flat: &'a Value,
}

impl Ctx<'_> {
    fn collection(&self, name: &str) -> Option<&serde_json::Map<String, Value>> {
        self.flat.get(name).and_then(Value::as_object)
    }

    /// Resolve a person id to a reference, tolerating absence.
    fn person_ref(&self, id: &str, confidence: Option<f64>, detail: Option<String>) -> PersonRef {
        let found = self.collection("persons").and_then(|m| m.get(id));
        PersonRef {
            id: id.to_string(),
            name: found
                .map(view::person_display_name)
                .unwrap_or_else(|| "[Unknown]".into()),
            known: found.is_some(),
            confidence: confidence.map(Confidence::new),
            detail,
        }
    }

    /// Resolve a place id, including the country history that makes a place
    /// meaningful across border changes.
    fn place(&self, id: Option<&str>) -> Option<PlaceView> {
        let id = id?;
        let found = self.collection("places").and_then(|m| m.get(id));
        let Some(p) = found else {
            return Some(PlaceView {
                name: "[Unknown place]".into(),
                known: false,
                place_type: None,
                country_current: None,
                country_history: Vec::new(),
                note: None,
            });
        };
        let history = p
            .get("country_history")
            .and_then(Value::as_array)
            .map(|arr| {
                arr.iter()
                    .map(|h| {
                        let country = h
                            .get("country")
                            .and_then(Value::as_str)
                            .unwrap_or("unknown");
                        let from = h.get("from").and_then(Value::as_str);
                        let until = h.get("until").and_then(Value::as_str);
                        match (from, until) {
                            (Some(f), Some(u)) => format!("{country} ({f}–{u})"),
                            (Some(f), None) => format!("{country} (from {f})"),
                            (None, Some(u)) => format!("{country} (until {u})"),
                            (None, None) => country.to_string(),
                        }
                    })
                    .collect()
            })
            .unwrap_or_default();

        Some(PlaceView {
            name: view::place_name(p),
            known: true,
            place_type: str_field(p, "place_type").map(|t| t.replace('_', " ")),
            country_current: str_field(p, "country_current"),
            country_history: history,
            note: str_field(p, "note"),
        })
    }

    /// Resolve the `source_id` of an object, if it has one.
    fn source_of(&self, holder: &Value) -> Option<SourceView> {
        let id = holder.get("source_id").and_then(Value::as_str)?;
        Some(self.source(id))
    }

    fn source(&self, id: &str) -> SourceView {
        let found = self.collection("sources").and_then(|m| m.get(id));
        let Some(s) = found else {
            return SourceView {
                id: id.to_string(),
                title: "[Unknown source]".into(),
                source_type: None,
                reliability: "unknown".into(),
                reliability_label: view::reliability_label("unknown"),
                reliability_rank: 0,
                confidence: None,
                status: None,
                repository: None,
                known: false,
            };
        };
        let reliability = s
            .get("reliability")
            .and_then(Value::as_str)
            .unwrap_or("unknown")
            .to_string();
        SourceView {
            id: id.to_string(),
            title: str_field(s, "title").unwrap_or_else(|| "[Untitled source]".into()),
            source_type: str_field(s, "source_type").map(|t| t.replace('_', " ")),
            reliability_label: view::reliability_label(&reliability),
            reliability_rank: view::reliability_rank(&reliability),
            reliability,
            confidence: Confidence::from_field(s, "confidence"),
            status: str_field(s, "status").map(|t| t.replace('_', " ")),
            repository: s
                .get("repository")
                .and_then(|r| r.get("name"))
                .and_then(Value::as_str)
                .map(str::to_string),
            known: true,
        }
    }

    /// Build a birth/death fact.
    fn fact(&self, person: &Value, key: &str, label: &'static str) -> FactView {
        let holder = person.get(key);
        let Some(h) = holder.filter(|v| !v.is_null()) else {
            return FactView {
                label,
                date: DateDisplay::absent(),
                place: None,
                confidence: None,
                source: None,
                cause: None,
                present: false,
            };
        };
        FactView {
            label,
            date: view::render_date_field(h, "date"),
            place: self.place(h.get("place_id").and_then(Value::as_str)),
            confidence: Confidence::from_field(h, "confidence"),
            source: self.source_of(h),
            cause: str_field(h, "cause"),
            present: true,
        }
    }

    /// Parents, spouses, siblings and children, each carrying its confidence.
    fn relations(
        &self,
        id: &str,
    ) -> (
        Vec<PersonRef>,
        Vec<PersonRef>,
        Vec<PersonRef>,
        Vec<PersonRef>,
    ) {
        let mut parents = Vec::new();
        let mut spouses = Vec::new();
        let mut siblings = Vec::new();
        let mut children = Vec::new();

        let Some(families) = self.collection("families") else {
            return (parents, spouses, siblings, children);
        };

        for fam in families.values() {
            let partners: Vec<(&str, Option<&str>)> = fam
                .get("union")
                .and_then(|u| u.get("persons"))
                .and_then(Value::as_array)
                .map(|ps| {
                    ps.iter()
                        .filter_map(|p| {
                            let pid = p.get("person_id").and_then(Value::as_str)?;
                            Some((pid, p.get("role").and_then(Value::as_str)))
                        })
                        .collect()
                })
                .unwrap_or_default();

            let kids: Vec<(&str, Option<f64>, Option<String>)> = fam
                .get("children")
                .and_then(Value::as_array)
                .map(|cs| {
                    cs.iter()
                        .filter_map(|c| {
                            let cid = c.get("person_id").and_then(Value::as_str)?;
                            let conf = c.get("confidence").and_then(Value::as_f64);
                            let note = str_field(c, "note");
                            Some((cid, conf, note))
                        })
                        .collect()
                })
                .unwrap_or_default();

            let union_conf = fam
                .get("union")
                .and_then(|u| u.get("confidence"))
                .and_then(Value::as_f64);

            let is_partner = partners.iter().any(|(p, _)| *p == id);
            let as_child = kids.iter().find(|(c, _, _)| *c == id);

            if let Some((_, conf, _)) = as_child {
                // Parents, and the confidence of *this* person's parentage.
                for (p, role) in &partners {
                    parents.push(self.person_ref(p, *conf, role.map(|r| r.replace('_', " "))));
                }
                for (c, cconf, note) in &kids {
                    if *c != id {
                        siblings.push(self.person_ref(c, *cconf, note.clone()));
                    }
                }
            }

            if is_partner {
                for (p, role) in &partners {
                    if *p != id {
                        spouses.push(self.person_ref(
                            p,
                            union_conf,
                            union_type(fam).or_else(|| role.map(|r| r.replace('_', " "))),
                        ));
                    }
                }
                for (c, cconf, note) in &kids {
                    children.push(self.person_ref(c, *cconf, note.clone()));
                }
            }
        }

        dedup_refs(&mut parents);
        dedup_refs(&mut spouses);
        dedup_refs(&mut siblings);
        dedup_refs(&mut children);
        (parents, spouses, siblings, children)
    }

    /// Non-family links, read from this person's point of view.
    fn links_for(&self, id: &str) -> Vec<LinkView> {
        let Some(links) = self.collection("links") else {
            return Vec::new();
        };
        let mut out = Vec::new();
        for l in links.values() {
            let from_id = l
                .get("from")
                .filter(|f| f.get("entity_type").and_then(Value::as_str) == Some("person"))
                .and_then(|f| f.get("entity_id"))
                .and_then(Value::as_str);
            let to_id = l
                .get("to")
                .filter(|t| t.get("entity_type").and_then(Value::as_str) == Some("person"))
                .and_then(|t| t.get("entity_id"))
                .and_then(Value::as_str);

            let label = l
                .get("label")
                .and_then(Value::as_str)
                .unwrap_or("linked to");
            let reverse = l.get("label_reverse").and_then(Value::as_str);

            // Read the link from this person's side: outgoing uses `label`,
            // incoming prefers `label_reverse` so the sentence stays true.
            let (direction, other_id, shown_label) = if from_id == Some(id) {
                ("outgoing", to_id, label.to_string())
            } else if to_id == Some(id) {
                (
                    "incoming",
                    from_id,
                    reverse
                        .map(str::to_string)
                        .unwrap_or_else(|| format!("{label} (of)")),
                )
            } else {
                continue;
            };
            let Some(other_id) = other_id else { continue };

            out.push(LinkView {
                label: shown_label,
                other: self.person_ref(other_id, None, None),
                direction,
                category: str_field(l, "category").map(|c| c.replace('_', " ")),
                from_date: l
                    .get("valid_from")
                    .filter(|v| !v.is_null())
                    .map(|v| view::render_date_field(v, "date")),
                until_date: l
                    .get("valid_until")
                    .filter(|v| !v.is_null())
                    .map(|v| view::render_date_field(v, "date")),
                confidence: Confidence::from_field(l, "confidence"),
                source: self.source_of(l),
                note: str_field(l, "note"),
            });
        }
        out.sort_by(|a, b| a.label.cmp(&b.label).then(a.other.name.cmp(&b.other.name)));
        out
    }

    /// Occupations as spans on a shared timeline.
    fn occupations_for(&self, id: &str) -> (Vec<OccupationView>, i64, i64, bool) {
        let Some(occs) = self.collection("occupations") else {
            return (Vec::new(), 0, 0, false);
        };
        let mine: Vec<&Value> = occs
            .values()
            .filter(|o| o.get("person_id").and_then(Value::as_str) == Some(id))
            .collect();
        if mine.is_empty() {
            return (Vec::new(), 0, 0, false);
        }

        // A shared axis, so two occupations can be compared by eye.
        let mut years: Vec<i64> = Vec::new();
        for o in &mine {
            for key in ["valid_from", "valid_until"] {
                if let Some(y) = year_of_bound(o.get(key)) {
                    years.push(y);
                }
            }
        }
        // A GEDCOM import gives occupation titles with no dates whatsoever.
        // There is no scale to draw then, and inventing one would imply the
        // record says something it does not.
        let has_timeline = !years.is_empty();

        let lo = years.iter().copied().min().unwrap_or(0);
        let hi = years.iter().copied().max().unwrap_or(0);
        // Pad so a bar never starts flush against the edge, and never divide
        // by zero when every occupation is a single year.
        let (axis_lo, axis_hi) = if years.is_empty() {
            (0, 1)
        } else if hi == lo {
            (lo - 1, hi + 1)
        } else {
            let pad = ((hi - lo) as f64 * 0.05).ceil() as i64;
            (lo - pad.max(1), hi + pad.max(1))
        };
        let span = (axis_hi - axis_lo).max(1) as f64;

        let mut out: Vec<OccupationView> = mine
            .iter()
            .map(|o| {
                let from = o
                    .get("valid_from")
                    .filter(|v| !v.is_null())
                    .map(|v| view::render_date_field(v, "date"));
                let until = o
                    .get("valid_until")
                    .filter(|v| !v.is_null())
                    .map(|v| view::render_date_field(v, "date"));

                let y0 = year_of_bound(o.get("valid_from"));
                let y1 = year_of_bound(o.get("valid_until"));
                let open_start = y0.is_none();
                let open_end = y1.is_none();
                let b0 = y0.unwrap_or(axis_lo);
                let b1 = y1.unwrap_or(axis_hi);
                let left = ((b0 - axis_lo) as f64 / span * 100.0).clamp(0.0, 100.0);
                let width = (((b1 - b0) as f64 / span) * 100.0).clamp(1.5, 100.0 - left);

                let span_text = match (
                    from.as_ref()
                        .map(|d| d.short.clone())
                        .filter(|s| !s.is_empty()),
                    until
                        .as_ref()
                        .map(|d| d.short.clone())
                        .filter(|s| !s.is_empty()),
                ) {
                    (Some(a), Some(b)) => format!("{a}–{b}"),
                    (Some(a), None) => format!("from {a}"),
                    (None, Some(b)) => format!("until {b}"),
                    (None, None) => "dates unrecorded".into(),
                };

                OccupationView {
                    title: str_field(o, "title").unwrap_or_else(|| "[Untitled]".into()),
                    employer: o
                        .get("employer")
                        .and_then(|e| e.get("name"))
                        .and_then(Value::as_str)
                        .map(str::to_string),
                    place: self.place(o.get("place_id").and_then(Value::as_str)),
                    from,
                    until,
                    span: span_text,
                    confidence: Confidence::from_field(o, "confidence"),
                    source: self.source_of(o),
                    note: str_field(o, "note"),
                    bar_left: left,
                    bar_width: width,
                    open_start,
                    open_end,
                }
            })
            .collect();

        out.sort_by(|a, b| {
            a.bar_left
                .partial_cmp(&b.bar_left)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.title.cmp(&b.title))
        });
        (out, axis_lo, axis_hi, has_timeline)
    }

    /// Events this person participated in.
    fn events_for(&self, id: &str) -> Vec<EventView> {
        let Some(events) = self.collection("events") else {
            return Vec::new();
        };
        let mut out: Vec<EventView> = events
            .values()
            .filter_map(|e| {
                let role = e
                    .get("participants")
                    .and_then(Value::as_array)?
                    .iter()
                    .find(|p| p.get("entity_id").and_then(Value::as_str) == Some(id))
                    .map(|p| {
                        p.get("role")
                            .and_then(Value::as_str)
                            .unwrap_or("participant")
                            .replace('_', " ")
                    })?;
                Some(EventView {
                    category: e
                        .get("category")
                        .and_then(Value::as_str)
                        .unwrap_or("other")
                        .replace('_', " "),
                    subcategory: str_field(e, "subcategory"),
                    date: view::render_date_field(e, "date"),
                    place: self.place(e.get("place_id").and_then(Value::as_str)),
                    role: Some(role),
                    description: str_field(e, "description"),
                    confidence: Confidence::from_field(e, "confidence"),
                    source: self.source_of(e),
                })
            })
            .collect();
        out.sort_by(|a, b| a.date.short.cmp(&b.date.short));
        out
    }

    /// Documents attached to this person, from either direction.
    fn documents_for(&self, id: &str, person: &Value) -> Vec<DocumentView> {
        let Some(docs) = self.collection("documents") else {
            return Vec::new();
        };
        let mut out: Vec<DocumentView> = Vec::new();
        let mut seen: Vec<String> = Vec::new();

        let mut push = |doc_id: &str, role: Option<String>| {
            if seen.iter().any(|s| s == doc_id) {
                return;
            }
            seen.push(doc_id.to_string());
            let found = docs.get(doc_id);
            out.push(match found {
                Some(d) => DocumentView {
                    id: doc_id.to_string(),
                    filename: str_field(d, "filename").unwrap_or_else(|| doc_id.to_string()),
                    document_type: str_field(d, "document_type")
                        .unwrap_or_else(|| "other".into())
                        .replace('_', " "),
                    status: str_field(d, "status")
                        .unwrap_or_else(|| "unknown".into())
                        .replace('_', " "),
                    caption: str_field(d, "caption"),
                    role,
                    known: true,
                },
                None => DocumentView {
                    id: doc_id.to_string(),
                    filename: "[Missing document]".into(),
                    document_type: "unknown".into(),
                    status: "referenced".into(),
                    caption: None,
                    role,
                    known: false,
                },
            });
        };

        if let Some(arr) = person.get("documents").and_then(Value::as_array) {
            for dl in arr {
                if let Some(did) = dl.get("document_id").and_then(Value::as_str) {
                    push(did, str_field(dl, "role"));
                }
            }
        }
        for (did, d) in docs.iter() {
            let linked = d
                .get("linked_to")
                .and_then(Value::as_array)
                .map(|arr| {
                    arr.iter().any(|l| {
                        l.get("entity_id").and_then(Value::as_str) == Some(id)
                            && l.get("entity_type").and_then(Value::as_str) == Some("person")
                    })
                })
                .unwrap_or(false);
            if linked {
                push(did, None);
            }
        }
        out
    }

    /// Every source any of this person's facts rests on, strongest first.
    fn sources_for(
        &self,
        birth: &FactView,
        death: &FactView,
        names: &[NameView],
        links: &[LinkView],
        occupations: &[OccupationView],
        events: &[EventView],
    ) -> Vec<SourceView> {
        let mut out: Vec<SourceView> = Vec::new();
        let mut add = |s: &Option<SourceView>| {
            if let Some(s) = s {
                if !out.iter().any(|e| e.id == s.id) {
                    out.push(s.clone());
                }
            }
        };
        add(&birth.source);
        add(&death.source);
        for n in names {
            add(&n.source);
        }
        for l in links {
            add(&l.source);
        }
        for o in occupations {
            add(&o.source);
        }
        for e in events {
            add(&e.source);
        }
        out.sort_by(|a, b| {
            b.reliability_rank
                .cmp(&a.reliability_rank)
                .then(a.title.cmp(&b.title))
        });
        out
    }
}

/// The union type of a family, as a readable phrase.
fn union_type(fam: &Value) -> Option<String> {
    let t = fam.get("union")?.get("type")?.as_str()?;
    Some(match t {
        "marriage" => "married".into(),
        "civil_union" => "civil union".into(),
        "cohabitation" => "cohabited".into(),
        "religious_only" => "religious union".into(),
        other => other.replace('_', " "),
    })
}

/// The first year mentioned by a `{date: …}` bound, for timeline geometry.
fn year_of_bound(bound: Option<&Value>) -> Option<i64> {
    let b = bound?;
    if b.is_null() {
        return None;
    }
    let d = view::render_date_field(b, "date");
    d.short
        .trim_start_matches(['<', '>'])
        .split('–')
        .next()
        .and_then(|s| s.parse::<i64>().ok())
}

/// Drop repeats while keeping the first occurrence, which carries the richest
/// detail. The same person can appear through more than one family.
fn dedup_refs(v: &mut Vec<PersonRef>) {
    let mut seen: Vec<String> = Vec::new();
    v.retain(|r| {
        if seen.contains(&r.id) {
            false
        } else {
            seen.push(r.id.clone());
            true
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn bundle() -> Value {
        json!({
            "manifest": {"axgf": "1.0"},
            "persons": {
                "p-jules": {
                    "id": "p-jules", "type": "person", "axgf_version": "1.0",
                    "identity": {
                        "name": {"display": "Jules Meunier"},
                        "gender": {"value": "M"},
                        "is_living": false,
                        "names": [{"type": "birth", "display": "Jules Meunier",
                                   "valid_from": "1920", "confidence": 0.9,
                                   "source_id": "s-reg"}]
                    },
                    "birth": {"date": {"value": "1920", "precision": "year"},
                              "place_id": "pl-lyon", "confidence": 0.95,
                              "source_id": "s-reg"},
                    "death": {"date": {"precision": "unknown", "note": "sometime after the war"},
                              "confidence": 0.3},
                    "documents": [{"document_id": "d-photo", "role": "portrait"}]
                },
                "p-jean": {
                    "id": "p-jean", "type": "person", "axgf_version": "1.0",
                    "identity": {"name": {"display": "Jean Boucher"}}},
                "p-dad": {
                    "id": "p-dad", "type": "person", "axgf_version": "1.0",
                    "identity": {"name": {"display": "Henri Meunier"}}},
                "p-sib": {
                    "id": "p-sib", "type": "person", "axgf_version": "1.0",
                    "identity": {"name": {"display": "Marie Meunier"}}}
            },
            "families": {
                "f1": {"id": "f1", "type": "family", "axgf_version": "1.0",
                       "union": {"type": "marriage", "confidence": 0.88,
                                 "persons": [{"person_id": "p-dad", "role": "spouse"}]},
                       "children": [
                           {"person_id": "p-jules", "confidence": 0.35},
                           {"person_id": "p-sib", "confidence": 0.9},
                           {"person_id": "p-ghost", "confidence": 0.5}]}
            },
            "links": {
                "l1": {"id": "l1", "type": "link", "axgf_version": "1.0",
                       "from": {"entity_type": "person", "entity_id": "p-jean"},
                       "to": {"entity_type": "person", "entity_id": "p-jules"},
                       "label": "godfather", "label_reverse": "godson",
                       "category": "spiritual",
                       "valid_from": {"date": {"value": "1950", "precision": "year"}},
                       "confidence": 0.85, "source_id": "s-letter",
                       "note": "per a family letter"}
            },
            "occupations": {
                "o1": {"id": "o1", "type": "occupation", "axgf_version": "1.0",
                       "person_id": "p-jules", "title": "Schoolteacher",
                       "employer": {"name": "École Normale"},
                       "place_id": "pl-lyon",
                       "valid_from": {"date": {"value": "1948", "precision": "year"}},
                       "valid_until": {"date": {"value": "1978", "precision": "year"}},
                       "confidence": 0.8, "source_id": "s-reg"},
                "o2": {"id": "o2", "type": "occupation", "axgf_version": "1.0",
                       "person_id": "p-jules", "title": "Clerk",
                       "valid_from": {"date": {"value": "1940", "precision": "year"}},
                       "confidence": 0.5}
            },
            "sources": {
                "s-reg": {"id": "s-reg", "type": "source", "axgf_version": "1.0",
                          "title": "Lyon civil register", "source_type": "birth_certificate",
                          "reliability": "primary", "confidence": 0.95},
                "s-letter": {"id": "s-letter", "type": "source", "axgf_version": "1.0",
                             "title": "Family letter, 1954", "source_type": "letter",
                             "reliability": "oral", "confidence": 0.6}
            },
            "places": {
                "pl-lyon": {"id": "pl-lyon", "type": "place", "axgf_version": "1.0",
                            "names": [{"lang": "fr", "value": "Lyon", "is_primary": true}],
                            "place_type": "city", "country_current": "France",
                            "country_history": [
                                {"country": "Kingdom of France", "until": "1792"},
                                {"country": "France", "from": "1792"}]}
            },
            "events": {}, "documents": {
                "d-photo": {"id": "d-photo", "type": "document", "axgf_version": "1.0",
                            "filename": "jules.jpg", "mime_type": "image/jpeg",
                            "document_type": "photo", "status": "present"}
            }
        })
    }

    fn jules() -> PersonView {
        build(&bundle(), "p-jules").expect("Jules exists")
    }

    #[test]
    fn missing_person_returns_none_rather_than_a_broken_page() {
        assert!(build(&bundle(), "nobody").is_none());
    }

    #[test]
    fn birth_resolves_place_confidence_and_source() {
        let v = jules();
        assert_eq!(v.birth.date.text, "1920");
        assert_eq!(v.birth.place.as_ref().unwrap().name, "Lyon");
        assert_eq!(v.birth.confidence.as_ref().unwrap().band, "certain");
        assert_eq!(
            v.birth.source.as_ref().unwrap().reliability_label,
            "Primary source"
        );
    }

    #[test]
    fn an_unparseable_death_date_is_shown_not_hidden() {
        let v = jules();
        assert_eq!(v.death.date.kind, "preserved");
        assert!(v.death.date.text.contains("sometime after the war"));
        assert_eq!(v.death.confidence.as_ref().unwrap().band, "low");
    }

    #[test]
    fn place_carries_its_border_history() {
        let v = jules();
        let p = v.birth.place.as_ref().unwrap();
        assert_eq!(p.country_history.len(), 2);
        assert!(p.country_history[0].contains("Kingdom of France"));
        assert!(p.country_history[0].contains("1792"));
    }

    #[test]
    fn relations_carry_their_own_confidence() {
        let v = jules();
        // Jules' parentage is only 35% confident; his sibling's is 90%.
        let dad = v.parents.iter().find(|p| p.id == "p-dad").unwrap();
        assert_eq!(dad.confidence.as_ref().unwrap().band, "low");
        let sib = v.siblings.iter().find(|p| p.id == "p-sib").unwrap();
        assert_eq!(sib.confidence.as_ref().unwrap().band, "certain");
    }

    #[test]
    fn a_referenced_but_absent_relative_is_unknown_and_unlinked() {
        let v = build(&bundle(), "p-sib").expect("sibling exists");
        let ghost = v.siblings.iter().find(|s| s.id == "p-ghost").unwrap();
        assert_eq!(ghost.name, "[Unknown]");
        assert!(!ghost.known, "the template must not link an absent person");
    }

    #[test]
    fn an_incoming_link_reads_from_this_persons_side() {
        let v = jules();
        assert_eq!(v.links.len(), 1);
        let l = &v.links[0];
        // Jean is the godfather, so from Jules' side the label is "godson".
        assert_eq!(l.label, "godson");
        assert_eq!(l.direction, "incoming");
        assert_eq!(l.other.name, "Jean Boucher");
        assert_eq!(l.confidence.as_ref().unwrap().percent, 85);
        assert_eq!(l.from_date.as_ref().unwrap().text, "1950");
        assert_eq!(l.source.as_ref().unwrap().reliability, "oral");
    }

    #[test]
    fn the_outgoing_side_of_a_link_uses_the_forward_label() {
        let v = build(&bundle(), "p-jean").expect("Jean exists");
        assert_eq!(v.links[0].label, "godfather");
        assert_eq!(v.links[0].direction, "outgoing");
    }

    #[test]
    fn occupations_are_spans_with_comparable_bars() {
        let v = jules();
        assert_eq!(v.occupations.len(), 2);
        let teacher = v
            .occupations
            .iter()
            .find(|o| o.title == "Schoolteacher")
            .unwrap();
        assert_eq!(teacher.span, "1948–1978");
        assert_eq!(teacher.employer.as_deref(), Some("École Normale"));
        assert!(teacher.bar_width > 0.0);
        // The clerk post has no end date, so its bar runs open to the axis end.
        let clerk = v.occupations.iter().find(|o| o.title == "Clerk").unwrap();
        assert!(clerk.open_end);
        assert_eq!(clerk.span, "from 1940");
        // Both bars sit on one axis so they can be compared by eye.
        assert!(v.has_timeline);
        assert!(v.timeline_from <= 1940 && v.timeline_to >= 1978);
    }

    #[test]
    fn undated_occupations_do_not_get_an_invented_axis() {
        // A GEDCOM import records occupation titles with no dates at all.
        // Drawing a scale then would assert something the record does not say.
        let mut b = bundle();
        b["occupations"] = json!({
            "o9": {"id": "o9", "type": "occupation", "axgf_version": "1.0",
                   "person_id": "p-jules", "title": "Farmaceuta", "confidence": 0.8}
        });
        let v = build(&b, "p-jules").expect("builds");
        assert_eq!(v.occupations.len(), 1);
        assert!(
            !v.has_timeline,
            "with no dates anywhere there is no scale to draw"
        );
        assert_eq!(v.occupations[0].span, "dates unrecorded");
    }

    #[test]
    fn bars_stay_within_the_track() {
        let v = jules();
        for o in &v.occupations {
            assert!(o.bar_left >= 0.0 && o.bar_left <= 100.0, "{o:?}");
            assert!(o.bar_left + o.bar_width <= 100.01, "{o:?}");
        }
    }

    #[test]
    fn sources_are_listed_strongest_evidence_first() {
        let v = jules();
        assert!(v.sources.len() >= 2);
        assert_eq!(v.sources[0].reliability, "primary");
        assert!(v.sources.iter().any(|s| s.reliability == "oral"));
    }

    #[test]
    fn documents_attached_by_the_person_are_found() {
        let v = jules();
        assert_eq!(v.documents.len(), 1);
        assert_eq!(v.documents[0].filename, "jules.jpg");
        assert_eq!(v.documents[0].role.as_deref(), Some("portrait"));
    }

    #[test]
    fn showcase_notes_name_what_this_person_demonstrates() {
        let v = jules();
        let joined = v.showcase_notes.join(" | ");
        assert!(joined.contains("non-family relationship"), "{joined}");
        assert!(joined.contains("occupation"), "{joined}");
    }

    #[test]
    fn a_person_with_nothing_recorded_still_builds() {
        let b = json!({"manifest": {"axgf": "1.0"},
                       "persons": {"bare": {"id": "bare", "type": "person",
                                            "axgf_version": "1.0",
                                            "identity": {"name": {"display": "Bare"}}}}});
        let v = build(&b, "bare").expect("builds");
        assert!(!v.birth.present);
        assert_eq!(v.birth.date.text, "Not recorded");
        assert!(v.parents.is_empty() && v.links.is_empty() && v.occupations.is_empty());
    }
}
