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
//!
//! # What the page is for
//!
//! The identity page is the argument for the format, so it shows the whole
//! record rather than a summary of it: every name with the period it was used
//! and the source behind it, every event the person took part in — including
//! the ones they did not own, because standing witness at a marriage is a fact
//! about you — every relationship in both directions, every place, every
//! source, and the entity's own JSON at the bottom. A section with nothing in
//! it is omitted rather than shown empty, so the shape of the page is itself a
//! readout of what the bundle carries.

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
    /// Extra context: a role, a union type, a note.
    pub detail: Option<String>,
    /// Birth order within the family, when the record states one.
    pub birth_order: Option<i64>,
    /// Lifespan in brief, e.g. "1881–1962", for the family lists.
    pub lifespan: Option<String>,
}

/// A place, resolved with its historical country context.
#[derive(Debug, Clone, Serialize)]
pub struct PlaceView {
    pub id: String,
    pub name: String,
    pub known: bool,
    pub place_type: Option<String>,
    pub country_current: Option<String>,
    /// Which countries this place belonged to, and when. GEDCOM has no way to
    /// say that a town changed hands.
    pub country_history: Vec<String>,
    pub note: Option<String>,
}

/// A place together with everything on this page that happened there.
#[derive(Debug, Clone, Serialize)]
pub struct PlaceUse {
    pub place: PlaceView,
    /// "Born", "Married", "Occupation: Schoolteacher" — one line per use.
    pub uses: Vec<String>,
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
    pub note: Option<String>,
    pub known: bool,
    /// Which facts on this page rest on this source. Filled in only for the
    /// evidence section; empty on the chips shown next to individual facts.
    pub used_for: Vec<String>,
}

/// One recorded name, with the period it was used and where it came from.
#[derive(Debug, Clone, Serialize)]
pub struct NameView {
    pub display: String,
    /// The Latin transliteration, when the record carries one that differs
    /// from `display`. Showing both side by side is the clearest thing AXGF
    /// does that GEDCOM cannot: one field, one script, take it or leave it.
    pub latin: Option<String>,
    pub kind: String,
    /// True for `identity.name` — the name the rest of the site uses.
    pub is_primary: bool,
    pub culture: Option<String>,
    pub direction: Option<String>,
    pub reading: Option<String>,
    pub reading_system: Option<String>,
    /// "1920–1945", "from 1920", "until 1945" — empty when always applicable.
    pub period: Option<String>,
    pub confidence: Option<Confidence>,
    pub source: Option<SourceView>,
    pub note: Option<String>,
    /// "given name: Laura", in the order the record states.
    pub components: Vec<String>,
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
    pub mime_type: Option<String>,
    pub document_type: String,
    pub status: String,
    pub caption: Option<String>,
    pub note: Option<String>,
    pub role: Option<String>,
    pub known: bool,
    /// True when the bundle actually carries the bytes, so the page may offer
    /// a download. A `referenced` document names a file that lives elsewhere.
    pub has_payload: bool,
    /// True when the payload is an image and can be shown in the gallery.
    pub is_image: bool,
    pub size_bytes: Option<u64>,
    /// "1.4 MB", or `None` when the record does not state a size.
    pub size_human: Option<String>,
}

/// One entry on the life timeline: a vital fact or an event.
#[derive(Debug, Clone, Serialize)]
pub struct TimelineEntry {
    /// "Born", "Died", "Marriage", "Baptism".
    pub label: String,
    /// `birth`, `death` or `event`, for styling the marker.
    pub kind: &'static str,
    pub date: DateDisplay,
    pub place: Option<PlaceView>,
    /// This person's part in it: spouse, witness, subject.
    pub role: Option<String>,
    pub description: Option<String>,
    pub cause: Option<String>,
    pub confidence: Option<Confidence>,
    pub source: Option<SourceView>,
    /// Sort position; `None` sorts last, after everything dated.
    pub sort: Option<i64>,
}

/// One union this person was part of, with its own children.
#[derive(Debug, Clone, Serialize)]
pub struct UnionView {
    /// `None` when the bundle records only one partner in the union.
    pub spouse: Option<PersonRef>,
    /// "married", "civil union", "cohabited".
    pub kind: Option<String>,
    /// "ongoing", "ended by divorce", "ended by the death of a spouse".
    pub status: String,
    pub start: Option<DateDisplay>,
    pub start_place: Option<PlaceView>,
    pub end: Option<DateDisplay>,
    pub end_note: Option<String>,
    pub confidence: Option<Confidence>,
    pub source: Option<SourceView>,
    /// Children of this union, in birth order where the record states one.
    pub children: Vec<PersonRef>,
}

/// A block of free text.
#[derive(Debug, Clone, Serialize)]
pub struct NoteView {
    pub label: String,
    pub text: String,
    /// True when this is text a converter could not interpret and kept
    /// verbatim rather than dropping.
    pub verbatim: bool,
}

/// Everything the identity page shows.
#[derive(Debug, Clone, Serialize)]
pub struct PersonView {
    pub id: String,
    pub name: String,
    /// The primary name first, then every alternative.
    pub names: Vec<NameView>,
    pub gender: Option<String>,
    pub gender_note: Option<String>,
    pub is_living: bool,
    pub visibility: Option<String>,
    pub birth: FactView,
    pub death: FactView,
    /// Birth and death merged with every event, in date order.
    pub timeline: Vec<TimelineEntry>,
    pub parents: Vec<PersonRef>,
    pub siblings: Vec<PersonRef>,
    pub unions: Vec<UnionView>,
    pub links: Vec<LinkView>,
    pub occupations: Vec<OccupationView>,
    pub places: Vec<PlaceUse>,
    pub sources: Vec<SourceView>,
    pub documents: Vec<DocumentView>,
    /// Images among `documents`, for the gallery.
    pub images: Vec<DocumentView>,
    pub notes: Vec<NoteView>,
    /// The entity exactly as the bundle holds it, pretty-printed.
    pub raw_json: String,
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

    let names = ctx.names_of(identity);

    let birth = ctx.fact(person, "birth", "Born");
    let death = ctx.fact(person, "death", "Died");

    let (parents, siblings, unions) = ctx.relations(id);
    let links = ctx.links_for(id);
    let (occupations, timeline_from, timeline_to, has_timeline) = ctx.occupations_for(id);
    let events = ctx.events_for(id);
    let timeline = build_timeline(&birth, &death, events);
    let documents = ctx.documents_for(id, person);
    let images: Vec<DocumentView> = documents
        .iter()
        .filter(|d| d.is_image && d.has_payload)
        .cloned()
        .collect();
    let places = ctx.places_for(&birth, &death, &timeline, &occupations, &unions);
    let sources = ctx.sources_for(&timeline, &names, &links, &occupations, &unions);
    let notes = collect_notes(person, &birth, &death, &timeline);

    let raw_json =
        serde_json::to_string_pretty(person).unwrap_or_else(|_| "<unserializable>".into());

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
        showcase_notes.push(format!("{} recorded names", names.len()));
    }
    if names.iter().any(|n| n.latin.is_some()) {
        showcase_notes
            .push("a name in its own script beside its Latin transliteration".to_string());
    }
    let witnessed = timeline
        .iter()
        .filter(|t| t.kind == "event" && t.role.as_deref() == Some("witness"))
        .count();
    if witnessed > 0 {
        showcase_notes.push(format!(
            "{witnessed} event{} they witnessed rather than owned",
            if witnessed == 1 { "" } else { "s" }
        ));
    }

    Some(PersonView {
        id: id.to_string(),
        name,
        names,
        gender,
        gender_note,
        is_living,
        visibility: identity
            .and_then(|i| str_field(i, "visibility"))
            .map(|v| v.replace('_', " ")),
        birth,
        death,
        timeline,
        parents,
        siblings,
        unions,
        links,
        occupations,
        places,
        sources,
        documents,
        images,
        notes,
        raw_json,
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

/// Merge the vitals and the events into one chronological list.
///
/// Undated entries sort last rather than first: a fact with no date is not a
/// fact that happened before everything else.
fn build_timeline(
    birth: &FactView,
    death: &FactView,
    events: Vec<TimelineEntry>,
) -> Vec<TimelineEntry> {
    let mut out: Vec<TimelineEntry> = Vec::new();
    for (fact, kind) in [(birth, "birth"), (death, "death")] {
        if !fact.present {
            continue;
        }
        out.push(TimelineEntry {
            label: fact.label.to_string(),
            kind,
            date: fact.date.clone(),
            place: fact.place.clone(),
            role: None,
            description: None,
            cause: fact.cause.clone(),
            confidence: fact.confidence.clone(),
            source: fact.source.clone(),
            sort: fact.date.sort,
        });
    }
    out.extend(events);
    out.sort_by(|a, b| match (a.sort, b.sort) {
        (Some(x), Some(y)) => x.cmp(&y).then_with(|| a.label.cmp(&b.label)),
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => a.label.cmp(&b.label),
    });
    out
}

/// Gather every piece of free text, marking the ones a converter preserved
/// because it could not parse them.
fn collect_notes(
    person: &Value,
    birth: &FactView,
    death: &FactView,
    timeline: &[TimelineEntry],
) -> Vec<NoteView> {
    let mut out = Vec::new();
    if let Some(bio) = str_field(person, "bio") {
        out.push(NoteView {
            label: "Biography".into(),
            text: bio,
            verbatim: false,
        });
    }
    if let Some(n) = str_field(person, "notes") {
        out.push(NoteView {
            label: "Notes".into(),
            text: n,
            verbatim: false,
        });
    }
    // A date the converter could not interpret is kept as text on the date
    // itself. It belongs on the page: it is the evidence that the conversion
    // dropped nothing, and it is often the only thing the record says.
    for (label, fact) in [
        ("Birth date, as recorded", birth),
        ("Death date, as recorded", death),
    ] {
        if let Some(note) = fact.date.note.clone() {
            out.push(NoteView {
                label: label.into(),
                text: note,
                verbatim: true,
            });
        }
    }
    for entry in timeline {
        if entry.kind == "event" {
            if let Some(note) = entry.date.note.clone() {
                out.push(NoteView {
                    label: format!("{} date, as recorded", entry.label),
                    text: note,
                    verbatim: true,
                });
            }
        }
    }
    out
}

/// Format a byte count the way a file listing would.
fn human_size(bytes: u64) -> String {
    const UNITS: [(&str, u64); 3] = [("MB", 1024 * 1024), ("KB", 1024), ("bytes", 1)];
    for (unit, scale) in UNITS {
        if bytes >= scale {
            return if scale == 1 {
                format!("{bytes} bytes")
            } else {
                format!("{:.1} {unit}", bytes as f64 / scale as f64)
            };
        }
    }
    "0 bytes".into()
}

/// One entry of a family's `children[]`, read out before it is resolved.
struct ChildEntry<'a> {
    id: &'a str,
    confidence: Option<f64>,
    note: Option<String>,
    birth_order: Option<i64>,
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
            birth_order: None,
            lifespan: found.and_then(lifespan_of),
        }
    }

    /// Resolve a place id, including the country history that makes a place
    /// meaningful across border changes.
    fn place(&self, id: Option<&str>) -> Option<PlaceView> {
        let id = id?;
        let found = self.collection("places").and_then(|m| m.get(id));
        let Some(p) = found else {
            return Some(PlaceView {
                id: id.to_string(),
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
            id: id.to_string(),
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
                note: None,
                known: false,
                used_for: Vec::new(),
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
            note: str_field(s, "note"),
            known: true,
            used_for: Vec::new(),
        }
    }

    /// The primary name followed by every alternative, each with its period
    /// and provenance.
    fn names_of(&self, identity: Option<&Value>) -> Vec<NameView> {
        let mut out = Vec::new();
        let Some(identity) = identity else {
            return out;
        };
        if let Some(primary) = identity.get("name") {
            out.push(self.name_view(primary, true));
        }
        if let Some(arr) = identity.get("names").and_then(Value::as_array) {
            for n in arr {
                let v = self.name_view(n, false);
                // The converter often repeats the display name in `names[]`.
                // Listing it twice says nothing; a differing script or period
                // does, so only exact duplicates are folded away.
                let dup = out.iter().any(|e: &NameView| {
                    e.display == v.display
                        && e.latin == v.latin
                        && e.period.is_none()
                        && v.period.is_none()
                });
                if !dup {
                    out.push(v);
                }
            }
        }
        out
    }

    fn name_view(&self, n: &Value, is_primary: bool) -> NameView {
        let display = str_field(n, "display").unwrap_or_else(|| "[Unnamed]".into());
        // Only a transliteration that actually differs is worth two columns.
        let latin = str_field(n, "display_latin").filter(|l| l != &display);
        let from = str_field(n, "valid_from");
        let until = str_field(n, "valid_until");
        let period = match (from.as_deref(), until.as_deref()) {
            (Some(a), Some(b)) => Some(format!("{a}–{b}")),
            (Some(a), None) => Some(format!("from {a}")),
            (None, Some(b)) => Some(format!("until {b}")),
            (None, None) => None,
        };
        let components = n
            .get("components")
            .and_then(Value::as_array)
            .map(|arr| {
                let mut parts: Vec<(i64, String)> = arr
                    .iter()
                    .filter_map(|c| {
                        let value = str_field(c, "value")?;
                        let kind = str_field(c, "type")
                            .unwrap_or_else(|| "part".into())
                            .replace('_', " ");
                        let order = c.get("order").and_then(Value::as_i64).unwrap_or(i64::MAX);
                        Some((order, format!("{kind}: {value}")))
                    })
                    .collect();
                parts.sort_by_key(|(order, _)| *order);
                parts.into_iter().map(|(_, s)| s).collect()
            })
            .unwrap_or_default();

        NameView {
            display,
            latin,
            kind: str_field(n, "type")
                .unwrap_or_else(|| {
                    if is_primary {
                        "primary".into()
                    } else {
                        "other".into()
                    }
                })
                .replace('_', " "),
            is_primary,
            culture: str_field(n, "culture"),
            direction: str_field(n, "direction"),
            reading: str_field(n, "reading"),
            reading_system: str_field(n, "reading_system").map(|r| r.replace('_', " ")),
            period,
            confidence: Confidence::from_field(n, "confidence"),
            source: self.source_of(n),
            note: str_field(n, "note"),
            components,
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

    /// Resolve one `family.children[]` entry, carrying its birth order over.
    fn child_ref(&self, c: &ChildEntry) -> PersonRef {
        let mut r = self.person_ref(c.id, c.confidence, c.note.clone());
        r.birth_order = c.birth_order;
        r
    }

    /// Parents, siblings and unions, each carrying its own confidence.
    fn relations(&self, id: &str) -> (Vec<PersonRef>, Vec<PersonRef>, Vec<UnionView>) {
        let mut parents = Vec::new();
        let mut siblings = Vec::new();
        let mut unions: Vec<UnionView> = Vec::new();

        let Some(families) = self.collection("families") else {
            return (parents, siblings, unions);
        };

        for fam in families.values() {
            let union = fam.get("union");
            let partners: Vec<(&str, Option<&str>)> = union
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

            let kids: Vec<ChildEntry> = fam
                .get("children")
                .and_then(Value::as_array)
                .map(|cs| {
                    cs.iter()
                        .filter_map(|c| {
                            Some(ChildEntry {
                                id: c.get("person_id").and_then(Value::as_str)?,
                                confidence: c.get("confidence").and_then(Value::as_f64),
                                note: str_field(c, "note"),
                                birth_order: c.get("birth_order").and_then(Value::as_i64),
                            })
                        })
                        .collect()
                })
                .unwrap_or_default();

            let is_partner = partners.iter().any(|(p, _)| *p == id);
            let as_child = kids.iter().find(|c| c.id == id);

            if let Some(me) = as_child {
                // Parents, and the confidence of *this* person's parentage.
                for (p, role) in &partners {
                    parents.push(self.person_ref(
                        p,
                        me.confidence,
                        role.map(|r| r.replace('_', " ")),
                    ));
                }
                for c in &kids {
                    if c.id != id {
                        siblings.push(self.child_ref(c));
                    }
                }
            }

            if is_partner {
                let spouse = partners.iter().find(|(p, _)| *p != id).map(|(p, role)| {
                    self.person_ref(
                        p,
                        union
                            .and_then(|u| u.get("confidence"))
                            .and_then(Value::as_f64),
                        role.map(|r| r.replace('_', " ")),
                    )
                });
                let mut children: Vec<PersonRef> = kids.iter().map(|c| self.child_ref(c)).collect();
                // Birth order first where the record states one; the rest keep
                // the order the family lists them in.
                children.sort_by_key(|c| c.birth_order.unwrap_or(i64::MAX));

                unions.push(UnionView {
                    spouse,
                    kind: union.and_then(union_type),
                    status: union_status(union),
                    start: union
                        .and_then(|u| u.get("start"))
                        .filter(|v| !v.is_null())
                        .map(|s| view::render_date_field(s, "date")),
                    start_place: union
                        .and_then(|u| u.get("start"))
                        .and_then(|s| s.get("place_id"))
                        .and_then(Value::as_str)
                        .and_then(|p| self.place(Some(p))),
                    end: union
                        .and_then(|u| u.get("end"))
                        .filter(|v| !v.is_null())
                        .map(|e| view::render_date_field(e, "date")),
                    end_note: union
                        .and_then(|u| u.get("end"))
                        .and_then(|e| str_field(e, "note")),
                    confidence: union.and_then(|u| Confidence::from_field(u, "confidence")),
                    source: union.and_then(|u| self.source_of(u)),
                    children,
                });
            }
        }

        dedup_refs(&mut parents);
        dedup_refs(&mut siblings);
        (parents, siblings, unions)
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

    /// Events this person took part in, in any role.
    ///
    /// Being a witness at someone else's marriage is a fact about you, so the
    /// filter is participation, not ownership.
    fn events_for(&self, id: &str) -> Vec<TimelineEntry> {
        let Some(events) = self.collection("events") else {
            return Vec::new();
        };
        events
            .values()
            .filter_map(|e| {
                let role = e
                    .get("participants")
                    .and_then(Value::as_array)?
                    .iter()
                    .find(|p| {
                        p.get("entity_id").and_then(Value::as_str) == Some(id)
                            && p.get("entity_type").and_then(Value::as_str) != Some("family")
                    })
                    .map(|p| {
                        p.get("role")
                            .and_then(Value::as_str)
                            .unwrap_or("participant")
                            .replace('_', " ")
                    })?;
                let category = e
                    .get("category")
                    .and_then(Value::as_str)
                    .unwrap_or("other")
                    .replace('_', " ");
                let label = match str_field(e, "subcategory") {
                    Some(sub) => format!("{category} — {sub}"),
                    None => capitalise(&category),
                };
                let date = view::render_date_field(e, "date");
                Some(TimelineEntry {
                    label,
                    kind: "event",
                    sort: date.sort,
                    date,
                    place: self.place(e.get("place_id").and_then(Value::as_str)),
                    role: Some(role),
                    description: str_field(e, "description"),
                    cause: None,
                    confidence: Confidence::from_field(e, "confidence"),
                    source: self.source_of(e),
                })
            })
            .collect()
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
            let Some(d) = docs.get(doc_id) else {
                out.push(DocumentView {
                    id: doc_id.to_string(),
                    filename: "[Missing document]".into(),
                    mime_type: None,
                    document_type: "unknown".into(),
                    status: "referenced".into(),
                    caption: None,
                    note: None,
                    role,
                    known: false,
                    has_payload: false,
                    is_image: false,
                    size_bytes: None,
                    size_human: None,
                });
                return;
            };
            let mime = str_field(d, "mime_type");
            let status = str_field(d, "status").unwrap_or_else(|| "unknown".into());
            let size = d
                .get("file")
                .and_then(|f| f.get("size_bytes"))
                .and_then(Value::as_u64);
            out.push(DocumentView {
                id: doc_id.to_string(),
                filename: str_field(d, "filename").unwrap_or_else(|| doc_id.to_string()),
                is_image: mime.as_deref().is_some_and(|m| m.starts_with("image/")),
                mime_type: mime,
                document_type: str_field(d, "document_type")
                    .unwrap_or_else(|| "other".into())
                    .replace('_', " "),
                // Only a `present` document has bytes in the bundle; anything
                // else names a file that lives somewhere else entirely.
                has_payload: status == "present"
                    && d.get("file")
                        .and_then(|f| f.get("path"))
                        .and_then(Value::as_str)
                        .is_some(),
                status: status.replace('_', " "),
                caption: str_field(d, "caption"),
                note: str_field(d, "note"),
                role,
                known: true,
                size_bytes: size,
                size_human: size.map(human_size),
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

    /// Every place this person's record touches, and what happened at each.
    fn places_for(
        &self,
        birth: &FactView,
        death: &FactView,
        timeline: &[TimelineEntry],
        occupations: &[OccupationView],
        unions: &[UnionView],
    ) -> Vec<PlaceUse> {
        let mut out: Vec<PlaceUse> = Vec::new();
        let mut add = |place: &Option<PlaceView>, use_of: String| {
            let Some(p) = place else { return };
            match out.iter_mut().find(|e| e.place.id == p.id) {
                Some(existing) => {
                    if !existing.uses.contains(&use_of) {
                        existing.uses.push(use_of);
                    }
                }
                None => out.push(PlaceUse {
                    place: p.clone(),
                    uses: vec![use_of],
                }),
            }
        };

        add(&birth.place, "Born".into());
        add(&death.place, "Died".into());
        for e in timeline {
            if e.kind == "event" {
                add(&e.place, e.label.clone());
            }
        }
        for o in occupations {
            add(&o.place, format!("Worked as {}", o.title));
        }
        for u in unions {
            let who = u
                .spouse
                .as_ref()
                .map(|s| format!("Married {}", s.name))
                .unwrap_or_else(|| "Married".into());
            add(&u.start_place, who);
        }
        out.sort_by(|a, b| a.place.name.cmp(&b.place.name));
        out
    }

    /// Every source any of this person's facts rests on, strongest first, each
    /// carrying the list of facts that depend on it.
    fn sources_for(
        &self,
        timeline: &[TimelineEntry],
        names: &[NameView],
        links: &[LinkView],
        occupations: &[OccupationView],
        unions: &[UnionView],
    ) -> Vec<SourceView> {
        let mut out: Vec<SourceView> = Vec::new();
        let mut add = |s: &Option<SourceView>, used_for: String| {
            let Some(s) = s else { return };
            match out.iter_mut().find(|e| e.id == s.id) {
                Some(existing) => {
                    if !existing.used_for.contains(&used_for) {
                        existing.used_for.push(used_for);
                    }
                }
                None => {
                    let mut copy = s.clone();
                    copy.used_for = vec![used_for];
                    out.push(copy);
                }
            }
        };

        for e in timeline {
            add(&e.source, e.label.clone());
        }
        for n in names {
            add(&n.source, format!("the name “{}”", n.display));
        }
        for l in links {
            add(&l.source, format!("{} {}", l.label, l.other.name));
        }
        for o in occupations {
            add(&o.source, format!("working as {}", o.title));
        }
        for u in unions {
            let who = u
                .spouse
                .as_ref()
                .map(|s| format!("the union with {}", s.name))
                .unwrap_or_else(|| "the union".into());
            add(&u.source, who);
        }
        out.sort_by(|a, b| {
            b.reliability_rank
                .cmp(&a.reliability_rank)
                .then(a.title.cmp(&b.title))
        });
        out
    }
}

/// "1881–1962", for the family lists, or `None` when neither date is recorded.
fn lifespan_of(person: &Value) -> Option<String> {
    let b = view::render_date_field(person.get("birth").unwrap_or(&Value::Null), "date").short;
    let living = person
        .get("identity")
        .and_then(|i| i.get("is_living"))
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let d = if living {
        String::new()
    } else {
        view::render_date_field(person.get("death").unwrap_or(&Value::Null), "date").short
    };
    match (b.is_empty(), d.is_empty()) {
        (true, true) => None,
        (false, true) if living => Some(format!("b. {b}")),
        (false, true) => Some(format!("{b}–")),
        (true, false) => Some(format!("d. {d}")),
        (false, false) => Some(format!("{b}–{d}")),
    }
}

/// The union type of a family, as a readable phrase.
fn union_type(union: &Value) -> Option<String> {
    let t = union.get("type")?.as_str()?;
    Some(match t {
        "marriage" => "married".into(),
        "civil_union" => "civil union".into(),
        "cohabitation" => "cohabited".into(),
        "religious_only" => "religious union".into(),
        other => other.replace('_', " "),
    })
}

/// How a union stands: still running, or ended, and how.
///
/// `union.status` is preferred when present; otherwise an `end` block with a
/// reason says the same thing. A union with neither is reported as unrecorded
/// rather than assumed to be ongoing — the record does not say.
fn union_status(union: Option<&Value>) -> String {
    let Some(u) = union else {
        return "not recorded".into();
    };
    if let Some(s) = str_field(u, "status") {
        return match s.as_str() {
            "active" => "ongoing".into(),
            other => other.replace('_', " "),
        };
    }
    let end = u.get("end").filter(|e| !e.is_null());
    match end.and_then(|e| str_field(e, "reason")) {
        Some(r) => match r.as_str() {
            "death_of_spouse" => "ended by the death of a spouse".into(),
            other => format!("ended by {}", other.replace('_', " ")),
        },
        None if end.is_some() => "ended".into(),
        None => "not recorded".into(),
    }
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

/// Upper-case the first character, leaving the rest alone.
fn capitalise(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
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
                        "visibility": "public",
                        "names": [{"type": "birth", "display": "Jules Meunier",
                                   "valid_from": "1920", "confidence": 0.9,
                                   "source_id": "s-reg"},
                                  {"type": "transliteration",
                                   "display": "Юлий Мёнье",
                                   "display_latin": "Yuliy Myonye",
                                   "culture": "ru"}]
                    },
                    "birth": {"date": {"value": "1920", "precision": "year"},
                              "place_id": "pl-lyon", "confidence": 0.95,
                              "source_id": "s-reg"},
                    "death": {"date": {"precision": "unknown", "note": "sometime after the war"},
                              "confidence": 0.3},
                    "notes": "Left the village in 1946.",
                    "documents": [{"document_id": "d-photo", "role": "portrait"}]
                },
                "p-jean": {
                    "id": "p-jean", "type": "person", "axgf_version": "1.0",
                    "identity": {"name": {"display": "Jean Boucher"}}},
                "p-dad": {
                    "id": "p-dad", "type": "person", "axgf_version": "1.0",
                    "identity": {"name": {"display": "Henri Meunier"}}},
                "p-wife": {
                    "id": "p-wife", "type": "person", "axgf_version": "1.0",
                    "identity": {"name": {"display": "Adèle Roux"}},
                    "birth": {"date": {"value": "1922", "precision": "year"}},
                    "death": {"date": {"value": "1999", "precision": "year"}}},
                "p-kid1": {
                    "id": "p-kid1", "type": "person", "axgf_version": "1.0",
                    "identity": {"name": {"display": "Second Child"}}},
                "p-kid2": {
                    "id": "p-kid2", "type": "person", "axgf_version": "1.0",
                    "identity": {"name": {"display": "First Child"}}},
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
                           {"person_id": "p-ghost", "confidence": 0.5}]},
                "f2": {"id": "f2", "type": "family", "axgf_version": "1.0",
                       "union": {"type": "marriage", "confidence": 0.9,
                                 "persons": [{"person_id": "p-jules", "role": "spouse"},
                                             {"person_id": "p-wife", "role": "spouse"}],
                                 "start": {"date": {"value": "1946-05-04", "precision": "exact"},
                                           "place_id": "pl-lyon"},
                                 "end": {"date": {"value": "1971", "precision": "year"},
                                         "reason": "divorce"},
                                 "source_id": "s-reg"},
                       "children": [
                           {"person_id": "p-kid1", "birth_order": 2, "confidence": 0.9},
                           {"person_id": "p-kid2", "birth_order": 1, "confidence": 0.9}]}
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
                                {"country": "France", "from": "1792"}]},
                "pl-paris": {"id": "pl-paris", "type": "place", "axgf_version": "1.0",
                             "names": [{"lang": "fr", "value": "Paris", "is_primary": true}]}
            },
            "events": {
                "e-wed": {"id": "e-wed", "type": "event", "axgf_version": "1.0",
                          "category": "marriage",
                          "date": {"value": "1946-05-04", "precision": "exact"},
                          "place_id": "pl-lyon",
                          "participants": [
                              {"entity_type": "person", "entity_id": "p-jules", "role": "spouse"},
                              {"entity_type": "person", "entity_id": "p-wife", "role": "spouse"}],
                          "confidence": 0.9, "source_id": "s-reg"},
                "e-wit": {"id": "e-wit", "type": "event", "axgf_version": "1.0",
                          "category": "marriage", "subcategory": "civil ceremony",
                          "date": {"value": "1953-08-11", "precision": "exact"},
                          "place_id": "pl-paris",
                          "participants": [
                              {"entity_type": "person", "entity_id": "p-jean", "role": "spouse"},
                              {"entity_type": "person", "entity_id": "p-jules", "role": "witness"}],
                          "confidence": 0.7}
            },
            "documents": {
                "d-photo": {"id": "d-photo", "type": "document", "axgf_version": "1.0",
                            "filename": "jules.jpg", "mime_type": "image/jpeg",
                            "document_type": "photo", "status": "present",
                            "file": {"path": "documents/files/d-photo.jpg",
                                     "size_bytes": 20480, "sha256": "ab"}},
                "d-ref": {"id": "d-ref", "type": "document", "axgf_version": "1.0",
                          "filename": "parish book", "mime_type": "application/pdf",
                          "document_type": "certificate", "status": "referenced",
                          "linked_to": [{"entity_type": "person", "entity_id": "p-jules"}]}
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

    // -- identity ----------------------------------------------------------

    #[test]
    fn the_primary_name_leads_the_name_list() {
        let v = jules();
        assert!(v.names[0].is_primary);
        assert_eq!(v.names[0].display, "Jules Meunier");
    }

    #[test]
    fn a_transliteration_keeps_both_scripts() {
        let v = jules();
        let t = v
            .names
            .iter()
            .find(|n| n.kind == "transliteration")
            .expect("the transliterated name");
        assert_eq!(t.display, "Юлий Мёнье");
        assert_eq!(t.latin.as_deref(), Some("Yuliy Myonye"));
        assert_eq!(t.culture.as_deref(), Some("ru"));
    }

    #[test]
    fn a_name_carries_its_period_and_provenance() {
        let v = jules();
        let birth_name = v.names.iter().find(|n| n.kind == "birth").unwrap();
        assert_eq!(birth_name.period.as_deref(), Some("from 1920"));
        assert_eq!(birth_name.confidence.as_ref().unwrap().percent, 90);
        assert_eq!(
            birth_name.source.as_ref().unwrap().title,
            "Lyon civil register"
        );
    }

    #[test]
    fn visibility_and_living_status_are_surfaced() {
        let v = jules();
        assert_eq!(v.visibility.as_deref(), Some("public"));
        assert!(!v.is_living);
    }

    // -- timeline ----------------------------------------------------------

    #[test]
    fn the_timeline_merges_vitals_and_events_in_date_order() {
        let v = jules();
        let labels: Vec<&str> = v.timeline.iter().map(|t| t.label.as_str()).collect();
        assert_eq!(labels[0], "Born", "1920 comes first: {labels:?}");
        // 1946 marriage, then the 1953 one he witnessed, then the undated
        // death, which must sort last rather than pretending to be at year 0.
        assert_eq!(v.timeline.last().unwrap().label, "Died");
        let years: Vec<Option<i64>> = v.timeline.iter().map(|t| t.sort).collect();
        assert_eq!(
            years,
            vec![Some(19200000), Some(19460504), Some(19530811), None]
        );
    }

    #[test]
    fn an_event_this_person_only_witnessed_is_still_theirs() {
        let v = jules();
        let e = v
            .timeline
            .iter()
            .find(|t| t.role.as_deref() == Some("witness"))
            .expect("the marriage he witnessed");
        assert!(e.label.contains("civil ceremony"));
        assert_eq!(e.place.as_ref().unwrap().name, "Paris");
    }

    #[test]
    fn a_family_role_in_an_event_is_not_mistaken_for_a_person() {
        // Events name the family they created as a participant. Reading that
        // as a person would put the family's id on someone's timeline.
        let mut b = bundle();
        b["events"]["e-wed"]["participants"]
            .as_array_mut()
            .unwrap()
            .push(json!({"entity_type": "family", "entity_id": "p-jules", "role": "created"}));
        let v = build(&b, "p-jules").expect("builds");
        let roles: Vec<&str> = v
            .timeline
            .iter()
            .filter_map(|t| t.role.as_deref())
            .collect();
        assert!(!roles.contains(&"created"), "{roles:?}");
    }

    // -- family ------------------------------------------------------------

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
    fn a_union_states_its_type_dates_and_how_it_ended() {
        let v = jules();
        let u = v.unions.first().expect("one union");
        assert_eq!(u.spouse.as_ref().unwrap().name, "Adèle Roux");
        assert_eq!(u.kind.as_deref(), Some("married"));
        assert_eq!(u.status, "ended by divorce");
        assert_eq!(u.start.as_ref().unwrap().text, "4 May 1946");
        assert_eq!(u.end.as_ref().unwrap().text, "1971");
        assert_eq!(u.start_place.as_ref().unwrap().name, "Lyon");
        assert_eq!(u.confidence.as_ref().unwrap().percent, 90);
    }

    #[test]
    fn children_are_listed_in_birth_order_not_alphabetically() {
        let v = jules();
        let kids: Vec<&str> = v.unions[0]
            .children
            .iter()
            .map(|c| c.name.as_str())
            .collect();
        assert_eq!(kids, vec!["First Child", "Second Child"]);
        assert_eq!(v.unions[0].children[0].birth_order, Some(1));
    }

    #[test]
    fn a_spouse_shows_their_lifespan() {
        let v = jules();
        assert_eq!(
            v.unions[0].spouse.as_ref().unwrap().lifespan.as_deref(),
            Some("1922–1999")
        );
    }

    #[test]
    fn a_union_with_one_recorded_partner_has_no_spouse() {
        let v = build(&bundle(), "p-dad").expect("Henri exists");
        assert_eq!(v.unions.len(), 1);
        assert!(v.unions[0].spouse.is_none());
        assert_eq!(v.unions[0].children.len(), 3);
    }

    // -- links, occupations ------------------------------------------------

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

    // -- places, sources, documents, notes ---------------------------------

    #[test]
    fn places_are_gathered_with_what_happened_at_each() {
        let v = jules();
        let lyon = v.places.iter().find(|p| p.place.name == "Lyon").unwrap();
        assert!(lyon.uses.contains(&"Born".to_string()), "{:?}", lyon.uses);
        assert!(lyon
            .uses
            .iter()
            .any(|u| u.starts_with("Worked as Schoolteacher")));
        assert!(lyon.uses.iter().any(|u| u.starts_with("Married")));
        assert!(!lyon.place.country_history.is_empty());
        // Paris comes only from the marriage he witnessed.
        assert!(v.places.iter().any(|p| p.place.name == "Paris"));
    }

    #[test]
    fn sources_are_listed_strongest_evidence_first_and_name_what_rests_on_them() {
        let v = jules();
        assert!(v.sources.len() >= 2);
        assert_eq!(v.sources[0].reliability, "primary");
        let reg = v
            .sources
            .iter()
            .find(|s| s.title == "Lyon civil register")
            .unwrap();
        assert!(
            reg.used_for.contains(&"Born".to_string()),
            "{:?}",
            reg.used_for
        );
        assert!(reg.used_for.iter().any(|u| u.contains("Schoolteacher")));
        let letter = v.sources.iter().find(|s| s.reliability == "oral").unwrap();
        assert!(letter.used_for.iter().any(|u| u.contains("Jean Boucher")));
    }

    #[test]
    fn documents_attached_by_the_person_are_found() {
        let v = jules();
        assert_eq!(v.documents.len(), 2);
        let photo = v
            .documents
            .iter()
            .find(|d| d.filename == "jules.jpg")
            .unwrap();
        assert_eq!(photo.role.as_deref(), Some("portrait"));
        assert!(photo.is_image && photo.has_payload);
        assert_eq!(photo.size_human.as_deref(), Some("20.0 KB"));
        assert_eq!(v.images.len(), 1, "only the image goes in the gallery");
    }

    #[test]
    fn a_referenced_document_carries_no_payload_and_no_size() {
        let v = jules();
        let d = v
            .documents
            .iter()
            .find(|d| d.status == "referenced")
            .unwrap();
        assert!(!d.has_payload, "a referenced document has no bytes here");
        assert!(d.size_human.is_none());
        assert!(!v.images.iter().any(|i| i.id == d.id));
    }

    #[test]
    fn unparseable_text_is_kept_as_a_verbatim_note() {
        let v = jules();
        let verbatim = v.notes.iter().find(|n| n.verbatim).expect("a kept note");
        assert!(verbatim.text.contains("sometime after the war"));
        assert!(v.notes.iter().any(|n| n.label == "Notes" && !n.verbatim));
    }

    #[test]
    fn the_raw_entity_is_available_verbatim() {
        let v = jules();
        let parsed: Value = serde_json::from_str(&v.raw_json).expect("valid JSON");
        assert_eq!(parsed["id"], "p-jules");
        assert!(v.raw_json.contains('\n'), "pretty-printed, not one line");
    }

    #[test]
    fn showcase_notes_name_what_this_person_demonstrates() {
        let v = jules();
        let joined = v.showcase_notes.join(" | ");
        assert!(joined.contains("non-family relationship"), "{joined}");
        assert!(joined.contains("occupation"), "{joined}");
        assert!(joined.contains("transliteration"), "{joined}");
        assert!(joined.contains("witnessed"), "{joined}");
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
        // Every section is empty, so the page will show none of them.
        assert!(v.parents.is_empty() && v.links.is_empty() && v.occupations.is_empty());
        assert!(v.timeline.is_empty() && v.places.is_empty() && v.unions.is_empty());
        assert!(v.notes.is_empty() && v.sources.is_empty() && v.documents.is_empty());
        // Except the raw block, which always has something to say.
        assert!(v.raw_json.contains("Bare"));
    }

    #[test]
    fn human_size_reads_like_a_file_listing() {
        assert_eq!(human_size(0), "0 bytes");
        assert_eq!(human_size(512), "512 bytes");
        assert_eq!(human_size(2048), "2.0 KB");
        assert_eq!(human_size(3 * 1024 * 1024 + 512 * 1024), "3.5 MB");
    }
}
