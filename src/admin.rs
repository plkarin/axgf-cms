//! Admin-side form model.
//!
//! # Why a field table rather than eight HTML files
//!
//! The fields below are chosen by hand, per entity kind, for what someone
//! realistically edits. They are **not** derived from the JSON Schema — no
//! introspection, no `$ref` walking, no generated widgets. The table is
//! explicit source you can read and diff.
//!
//! It is rendered by one loop instead of being copied into eight near-identical
//! HTML blocks, which would be several hundred lines of duplicated markup with
//! nothing to stop them drifting apart.
//!
//! Anything not in the table stays editable through the raw JSON textarea that
//! every form carries, so no part of an entity is ever unreachable.

use axgf_rs::EntityKind;
use serde::Serialize;
use serde_json::{Map, Value};

/// How a field is presented and parsed.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FieldKind {
    Text,
    LongText,
    /// A 0.0–1.0 confidence, rendered as a slider.
    Confidence,
    Bool,
    Select,
    /// An entity reference: a search over the bundle, not a UUID typed by
    /// hand. [`Field::picks`] says over what.
    Picker,
}

/// One editable field.
///
/// Every word the form shows comes from the catalogue: `label` and `hint` are
/// message keys, and each option of a select is named by its vocabulary's own
/// message (`vocab` is the family, so `union-type` names `marriage` through
/// `union-type-marriage`). The table used to carry the English, and a Polish
/// reader editing a record met a form in English with raw schema values in
/// its selects — invisible to the template linter, because the words never
/// passed through a template.
#[derive(Debug, Clone, Serialize)]
pub struct Field {
    /// Form input name, and the dotted path into the entity JSON.
    pub path: &'static str,
    pub kind: FieldKind,
    /// The label's message key.
    pub label: &'static str,
    /// The hint's message key, for a field that has one.
    pub hint: Option<&'static str>,
    pub options: &'static [&'static str],
    /// The vocabulary family naming the options; empty for anything else.
    pub vocab: &'static str,
    /// What a [`FieldKind::Picker`] picks: `person`, `place`, `source`, or
    /// `linkable` for the far end of a link, which may be any of three kinds.
    /// Empty for every other kind of field.
    ///
    /// These were plain text inputs until this existed. A form that asks an
    /// administrator to type `3ca35cb9-a8d6-4943-9ad3-d1af333e3a50` is not
    /// asking a question anybody can answer; the structured editors had grown
    /// pickers for exactly this and the generic ones had not.
    pub picks: &'static str,
}

impl Field {
    /// The message naming one option of a select.
    ///
    /// A 1.1 vocabulary is named the way the profile names it, with its terms'
    /// underscores opened into dashes (`pv-link-relation-close-friend`); the
    /// 1.0 families keep the term as the schema spells it.
    pub fn option_key(&self, option: &str) -> String {
        if self.vocab.starts_with("pv-") {
            format!("{}-{}", self.vocab, option.replace('_', "-"))
        } else {
            format!("{}-{option}", self.vocab)
        }
    }
}

const NO_OPTS: &[&str] = &[];

/// Fields shown for a person.
const PERSON_FIELDS: &[Field] = &[
    Field {
        path: "identity.name.display",
        kind: FieldKind::Text,
        label: "field-person-display-name",
        hint: Some("field-person-display-name-hint"),
        options: NO_OPTS,
        vocab: "",
        picks: "",
    },
    Field {
        path: "identity.gender.value",
        kind: FieldKind::Select,
        label: "field-person-gender",
        hint: None,
        options: &["", "M", "F", "NB", "U"],
        vocab: "gender",
        picks: "",
    },
    Field {
        path: "identity.is_living",
        kind: FieldKind::Bool,
        label: "field-person-living",
        hint: None,
        options: NO_OPTS,
        vocab: "",
        picks: "",
    },
    Field {
        path: "birth.date.value",
        kind: FieldKind::Text,
        label: "field-person-birth-date",
        hint: Some("field-date-value-hint"),
        options: NO_OPTS,
        vocab: "",
        picks: "",
    },
    Field {
        path: "birth.date.precision",
        kind: FieldKind::Select,
        label: "field-person-birth-precision",
        hint: Some("field-precision-hint"),
        options: &[
            "",
            "exact",
            "month",
            "year",
            "decade",
            "quarter_century",
            "century",
            "unknown",
        ],
        vocab: "precision",
        picks: "",
    },
    Field {
        path: "birth.date.circa",
        kind: FieldKind::Bool,
        label: "field-person-birth-circa",
        hint: Some("field-circa-hint"),
        options: NO_OPTS,
        vocab: "",
        picks: "",
    },
    Field {
        path: "birth.place_id",
        kind: FieldKind::Picker,
        label: "field-person-birth-place",
        hint: None,
        options: NO_OPTS,
        vocab: "",
        picks: "place",
    },
    Field {
        path: "birth.confidence",
        kind: FieldKind::Confidence,
        label: "field-person-birth-confidence",
        hint: Some("field-person-confidence-hint"),
        options: NO_OPTS,
        vocab: "",
        picks: "",
    },
    Field {
        path: "death.date.value",
        kind: FieldKind::Text,
        label: "field-person-death-date",
        hint: None,
        options: NO_OPTS,
        vocab: "",
        picks: "",
    },
    Field {
        path: "death.date.precision",
        kind: FieldKind::Select,
        label: "field-person-death-precision",
        hint: None,
        options: &[
            "",
            "exact",
            "month",
            "year",
            "decade",
            "quarter_century",
            "century",
            "unknown",
        ],
        vocab: "precision",
        picks: "",
    },
    Field {
        path: "death.date.circa",
        kind: FieldKind::Bool,
        label: "field-person-death-circa",
        hint: None,
        options: NO_OPTS,
        vocab: "",
        picks: "",
    },
    Field {
        path: "death.place_id",
        kind: FieldKind::Picker,
        label: "field-person-death-place",
        hint: None,
        options: NO_OPTS,
        vocab: "",
        picks: "place",
    },
    Field {
        path: "death.confidence",
        kind: FieldKind::Confidence,
        label: "field-person-death-confidence",
        hint: None,
        options: NO_OPTS,
        vocab: "",
        picks: "",
    },
    Field {
        path: "death.cause",
        kind: FieldKind::Text,
        label: "field-person-death-cause",
        hint: None,
        options: NO_OPTS,
        vocab: "",
        picks: "",
    },
    Field {
        path: "bio",
        kind: FieldKind::LongText,
        label: "field-person-bio",
        hint: None,
        options: NO_OPTS,
        vocab: "",
        picks: "",
    },
    Field {
        path: "notes",
        kind: FieldKind::LongText,
        label: "field-notes",
        hint: None,
        options: NO_OPTS,
        vocab: "",
        picks: "",
    },
];

/// Fields shown for a family.
const FAMILY_FIELDS: &[Field] = &[
    Field {
        path: "name",
        kind: FieldKind::Text,
        label: "field-family-name",
        hint: None,
        options: NO_OPTS,
        vocab: "",
        picks: "",
    },
    Field {
        path: "description",
        kind: FieldKind::LongText,
        label: "field-description",
        hint: None,
        options: NO_OPTS,
        vocab: "",
        picks: "",
    },
    Field {
        path: "union.type",
        kind: FieldKind::Select,
        label: "field-family-union-type",
        hint: None,
        options: &[
            "",
            "marriage",
            "civil_union",
            "cohabitation",
            "religious_only",
            "polygamous",
            "unknown",
        ],
        vocab: "union-type",
        picks: "",
    },
    Field {
        path: "union.status",
        kind: FieldKind::Select,
        label: "field-family-union-status",
        hint: None,
        options: &[
            "",
            "active",
            "ended_by_death",
            "ended_by_divorce",
            "ended_by_separation",
            "annulled",
            "unknown",
        ],
        vocab: "union-status",
        picks: "",
    },
    Field {
        path: "union.confidence",
        kind: FieldKind::Confidence,
        label: "field-family-union-confidence",
        hint: Some("field-family-union-confidence-hint"),
        options: NO_OPTS,
        vocab: "",
        picks: "",
    },
    Field {
        path: "union.start.date.value",
        kind: FieldKind::Text,
        label: "field-family-union-start",
        hint: None,
        options: NO_OPTS,
        vocab: "",
        picks: "",
    },
    Field {
        path: "union.end.date.value",
        kind: FieldKind::Text,
        label: "field-family-union-end",
        hint: None,
        options: NO_OPTS,
        vocab: "",
        picks: "",
    },
    Field {
        path: "notes",
        kind: FieldKind::LongText,
        label: "field-notes",
        hint: Some("field-family-notes-hint"),
        options: NO_OPTS,
        vocab: "",
        picks: "",
    },
];

/// Fields shown for a event.
const EVENT_FIELDS: &[Field] = &[
    Field {
        path: "category",
        kind: FieldKind::Select,
        label: "field-category",
        hint: Some("field-required-hint"),
        options: &[
            "",
            "birth",
            "death",
            "marriage",
            "divorce",
            "adoption",
            "migration",
            "naturalization",
            "military",
            "incarceration",
            "name_change",
            "census",
            "legal",
            "religious",
            "social",
            "historical",
            "other",
        ],
        vocab: "event-category",
        picks: "",
    },
    Field {
        path: "subcategory",
        kind: FieldKind::Text,
        label: "field-event-subcategory",
        hint: None,
        options: NO_OPTS,
        vocab: "",
        picks: "",
    },
    Field {
        path: "date.value",
        kind: FieldKind::Text,
        label: "field-date",
        hint: Some("field-event-date-hint"),
        options: NO_OPTS,
        vocab: "",
        picks: "",
    },
    Field {
        path: "date.precision",
        kind: FieldKind::Select,
        label: "field-precision",
        hint: None,
        options: &[
            "",
            "exact",
            "month",
            "year",
            "decade",
            "quarter_century",
            "century",
            "unknown",
        ],
        vocab: "precision",
        picks: "",
    },
    Field {
        path: "date.circa",
        kind: FieldKind::Bool,
        label: "field-circa",
        hint: None,
        options: NO_OPTS,
        vocab: "",
        picks: "",
    },
    Field {
        path: "place_id",
        kind: FieldKind::Picker,
        label: "field-place-id",
        hint: None,
        options: NO_OPTS,
        vocab: "",
        picks: "place",
    },
    Field {
        path: "description",
        kind: FieldKind::LongText,
        label: "field-description",
        hint: None,
        options: NO_OPTS,
        vocab: "",
        picks: "",
    },
    Field {
        path: "confidence",
        kind: FieldKind::Confidence,
        label: "field-confidence",
        hint: None,
        options: NO_OPTS,
        vocab: "",
        picks: "",
    },
    Field {
        path: "source_id",
        kind: FieldKind::Picker,
        label: "field-source-id",
        hint: None,
        options: NO_OPTS,
        vocab: "",
        picks: "source",
    },
];

/// Fields shown for a link.
const LINK_FIELDS: &[Field] = &[
    Field {
        path: "from.entity_type",
        kind: FieldKind::Select,
        label: "field-link-from-type",
        hint: None,
        options: &["person", "family", "event"],
        vocab: "kind",
        picks: "",
    },
    Field {
        path: "from.entity_id",
        kind: FieldKind::Picker,
        label: "field-link-from-id",
        hint: Some("field-required-hint"),
        options: NO_OPTS,
        vocab: "",
        picks: "linkable",
    },
    Field {
        path: "to.entity_type",
        kind: FieldKind::Select,
        label: "field-link-to-type",
        hint: None,
        options: &["person", "family", "event"],
        vocab: "kind",
        picks: "",
    },
    Field {
        path: "to.entity_id",
        kind: FieldKind::Picker,
        label: "field-link-to-id",
        hint: Some("field-required-hint"),
        options: NO_OPTS,
        vocab: "",
        picks: "linkable",
    },
    Field {
        path: "label",
        kind: FieldKind::Text,
        label: "field-link-label",
        hint: Some("field-link-label-hint"),
        options: NO_OPTS,
        vocab: "",
        picks: "",
    },
    Field {
        path: "label_reverse",
        kind: FieldKind::Text,
        label: "field-link-label-reverse",
        hint: Some("field-link-label-reverse-hint"),
        options: NO_OPTS,
        vocab: "",
        picks: "",
    },
    Field {
        path: "category",
        kind: FieldKind::Select,
        label: "field-category",
        hint: None,
        options: &[
            "",
            "spiritual",
            "professional",
            "social",
            "legal",
            "medical",
            "educational",
            "conflict",
            "other",
        ],
        vocab: "link-category",
        picks: "",
    },
    Field {
        path: "relation",
        kind: FieldKind::Select,
        label: "field-link-relation",
        hint: Some("field-link-relation-hint"),
        options: &[
            "",
            "godparent",
            "godchild",
            "witness",
            "officiant",
            "business_partner",
            "employer",
            "employee",
            "mentor",
            "apprentice",
            "close_friend",
            "neighbour",
            "guardian",
            "ward",
            "other",
        ],
        vocab: "pv-link-relation",
        picks: "",
    },
    Field {
        path: "bidirectional",
        kind: FieldKind::Bool,
        label: "field-link-bidirectional",
        hint: None,
        options: NO_OPTS,
        vocab: "",
        picks: "",
    },
    Field {
        path: "valid_from.date.value",
        kind: FieldKind::Text,
        label: "field-valid-from",
        hint: Some("field-link-valid-from-hint"),
        options: NO_OPTS,
        vocab: "",
        picks: "",
    },
    Field {
        path: "valid_until.date.value",
        kind: FieldKind::Text,
        label: "field-valid-until",
        hint: None,
        options: NO_OPTS,
        vocab: "",
        picks: "",
    },
    Field {
        path: "confidence",
        kind: FieldKind::Confidence,
        label: "field-confidence",
        hint: Some("field-link-confidence-hint"),
        options: NO_OPTS,
        vocab: "",
        picks: "",
    },
    Field {
        path: "source_id",
        kind: FieldKind::Picker,
        label: "field-source-id",
        hint: None,
        options: NO_OPTS,
        vocab: "",
        picks: "source",
    },
    Field {
        path: "note",
        kind: FieldKind::LongText,
        label: "field-note",
        hint: None,
        options: NO_OPTS,
        vocab: "",
        picks: "",
    },
];

/// Fields shown for a occupation.
const OCCUPATION_FIELDS: &[Field] = &[
    Field {
        path: "person_id",
        kind: FieldKind::Picker,
        label: "field-occupation-person-id",
        hint: Some("field-required-hint"),
        options: NO_OPTS,
        vocab: "",
        picks: "person",
    },
    Field {
        path: "title",
        kind: FieldKind::Text,
        label: "field-occupation-title",
        hint: Some("field-occupation-title-hint"),
        options: NO_OPTS,
        vocab: "",
        picks: "",
    },
    Field {
        path: "title_latin",
        kind: FieldKind::Text,
        label: "field-occupation-title-latin",
        hint: None,
        options: NO_OPTS,
        vocab: "",
        picks: "",
    },
    Field {
        path: "position",
        kind: FieldKind::Text,
        label: "field-occupation-position",
        hint: Some("field-occupation-position-hint"),
        options: NO_OPTS,
        vocab: "",
        picks: "",
    },
    Field {
        path: "employer.name",
        kind: FieldKind::Text,
        label: "field-occupation-employer",
        hint: None,
        options: NO_OPTS,
        vocab: "",
        picks: "",
    },
    Field {
        path: "place_id",
        kind: FieldKind::Picker,
        label: "field-place-id",
        hint: None,
        options: NO_OPTS,
        vocab: "",
        picks: "place",
    },
    Field {
        path: "valid_from.date.value",
        kind: FieldKind::Text,
        label: "field-occupation-from",
        hint: Some("field-occupation-from-hint"),
        options: NO_OPTS,
        vocab: "",
        picks: "",
    },
    Field {
        path: "valid_until.date.value",
        kind: FieldKind::Text,
        label: "field-occupation-until",
        hint: None,
        options: NO_OPTS,
        vocab: "",
        picks: "",
    },
    Field {
        path: "confidence",
        kind: FieldKind::Confidence,
        label: "field-confidence",
        hint: None,
        options: NO_OPTS,
        vocab: "",
        picks: "",
    },
    Field {
        path: "source_id",
        kind: FieldKind::Picker,
        label: "field-source-id",
        hint: None,
        options: NO_OPTS,
        vocab: "",
        picks: "source",
    },
    Field {
        path: "note",
        kind: FieldKind::LongText,
        label: "field-note",
        hint: None,
        options: NO_OPTS,
        vocab: "",
        picks: "",
    },
];

/// Fields shown for a source.
const SOURCE_FIELDS: &[Field] = &[
    Field {
        path: "title",
        kind: FieldKind::Text,
        label: "field-source-title",
        hint: Some("field-required-hint"),
        options: NO_OPTS,
        vocab: "",
        picks: "",
    },
    Field {
        path: "source_type",
        kind: FieldKind::Select,
        label: "field-source-type",
        hint: Some("field-required-hint"),
        options: &[
            "birth_certificate",
            "death_certificate",
            "marriage_certificate",
            "census",
            "baptism_record",
            "burial_record",
            "will",
            "land_record",
            "military_record",
            "immigration_record",
            "naturalization",
            "passport",
            "photograph",
            "letter",
            "diary",
            "newspaper",
            "oral_tradition",
            "dna",
            "family_bible",
            "gravestone",
            "published_genealogy",
            "other",
        ],
        vocab: "source-type",
        picks: "",
    },
    Field {
        path: "reliability",
        kind: FieldKind::Select,
        label: "field-source-reliability",
        hint: Some("field-source-reliability-hint"),
        options: &[
            "primary",
            "secondary",
            "derivative",
            "authored",
            "oral",
            "unknown",
        ],
        vocab: "reliability",
        picks: "",
    },
    Field {
        path: "status",
        kind: FieldKind::Select,
        label: "field-source-status",
        hint: None,
        options: &["", "verified", "unverified", "lost", "known_missing"],
        vocab: "source-status",
        picks: "",
    },
    Field {
        path: "confidence",
        kind: FieldKind::Confidence,
        label: "field-confidence",
        hint: None,
        options: NO_OPTS,
        vocab: "",
        picks: "",
    },
    Field {
        path: "repository.name",
        kind: FieldKind::Text,
        label: "field-source-repository",
        hint: None,
        options: NO_OPTS,
        vocab: "",
        picks: "",
    },
    Field {
        path: "repository.reference",
        kind: FieldKind::Text,
        label: "field-source-repository-reference",
        hint: None,
        options: NO_OPTS,
        vocab: "",
        picks: "",
    },
    Field {
        path: "transcription",
        kind: FieldKind::LongText,
        label: "field-source-transcription",
        hint: None,
        options: NO_OPTS,
        vocab: "",
        picks: "",
    },
    Field {
        path: "note",
        kind: FieldKind::LongText,
        label: "field-note",
        hint: None,
        options: NO_OPTS,
        vocab: "",
        picks: "",
    },
];

/// Fields shown for a place.
const PLACE_FIELDS: &[Field] = &[
    Field {
        path: "names.0.value",
        kind: FieldKind::Text,
        label: "field-place-name",
        hint: Some("field-required-hint"),
        options: NO_OPTS,
        vocab: "",
        picks: "",
    },
    Field {
        path: "names.0.lang",
        kind: FieldKind::Text,
        label: "field-place-name-lang",
        hint: Some("field-place-name-lang-hint"),
        options: NO_OPTS,
        vocab: "",
        picks: "",
    },
    Field {
        path: "place_type",
        kind: FieldKind::Select,
        label: "field-place-type",
        hint: None,
        options: &[
            "",
            "continent",
            "country",
            "region",
            "department",
            "city",
            "village",
            "district",
            "street",
            "building",
            "farm",
            "island",
            "historical",
            "unknown",
        ],
        vocab: "place-type",
        picks: "",
    },
    Field {
        path: "region",
        kind: FieldKind::Text,
        label: "field-place-region",
        hint: None,
        options: NO_OPTS,
        vocab: "",
        picks: "",
    },
    Field {
        path: "country_current",
        kind: FieldKind::Text,
        label: "field-place-country-current",
        hint: Some("field-place-country-current-hint"),
        options: NO_OPTS,
        vocab: "",
        picks: "",
    },
    Field {
        path: "note",
        kind: FieldKind::LongText,
        label: "field-note",
        hint: None,
        options: NO_OPTS,
        vocab: "",
        picks: "",
    },
];

/// Fields shown for a document.
const DOCUMENT_FIELDS: &[Field] = &[
    Field {
        path: "filename",
        kind: FieldKind::Text,
        label: "field-document-filename",
        hint: Some("field-required-hint"),
        options: NO_OPTS,
        vocab: "",
        picks: "",
    },
    Field {
        path: "mime_type",
        kind: FieldKind::Text,
        label: "field-document-mime-type",
        hint: Some("field-document-mime-type-hint"),
        options: NO_OPTS,
        vocab: "",
        picks: "",
    },
    Field {
        path: "document_type",
        kind: FieldKind::Select,
        label: "field-document-type",
        hint: Some("field-required-hint"),
        options: &[
            "photo",
            "birth_certificate",
            "death_certificate",
            "marriage_certificate",
            "census_page",
            "baptism_record",
            "military_record",
            "will",
            "land_record",
            "letter",
            "diary",
            "newspaper_clipping",
            "gravestone_photo",
            "family_tree_drawing",
            "audio",
            "video",
            "other",
        ],
        vocab: "document-type",
        picks: "",
    },
    Field {
        path: "status",
        kind: FieldKind::Select,
        label: "field-document-status",
        hint: Some("field-required-hint"),
        options: &["present", "referenced", "known_missing", "lost", "unknown"],
        vocab: "document-status",
        picks: "",
    },
    Field {
        path: "url",
        kind: FieldKind::Text,
        label: "field-document-url",
        hint: None,
        options: NO_OPTS,
        vocab: "",
        picks: "",
    },
    Field {
        path: "caption",
        kind: FieldKind::Text,
        label: "field-document-caption",
        hint: None,
        options: NO_OPTS,
        vocab: "",
        picks: "",
    },
    Field {
        path: "note",
        kind: FieldKind::LongText,
        label: "field-note",
        hint: None,
        options: NO_OPTS,
        vocab: "",
        picks: "",
    },
];

/// Every message key the generic form can ask for: each label, each hint,
/// and each option of each select through its vocabulary family.
pub fn every_key() -> Vec<String> {
    let kinds = [
        EntityKind::Person,
        EntityKind::Family,
        EntityKind::Event,
        EntityKind::Link,
        EntityKind::Occupation,
        EntityKind::Source,
        EntityKind::Place,
        EntityKind::Document,
    ];
    let mut out: Vec<String> = Vec::new();
    for kind in kinds {
        for f in fields_for(kind) {
            out.push(f.label.to_string());
            out.extend(f.hint.map(str::to_string));
            out.extend(
                f.options
                    .iter()
                    .filter(|o| !o.is_empty())
                    .map(|o| f.option_key(o)),
            );
        }
    }
    out.sort();
    out.dedup();
    out
}

/// The fields shown for each entity kind.
pub fn fields_for(kind: EntityKind) -> &'static [Field] {
    match kind {
        EntityKind::Person => PERSON_FIELDS,
        EntityKind::Family => FAMILY_FIELDS,
        EntityKind::Event => EVENT_FIELDS,
        EntityKind::Link => LINK_FIELDS,
        EntityKind::Occupation => OCCUPATION_FIELDS,
        EntityKind::Source => SOURCE_FIELDS,
        EntityKind::Place => PLACE_FIELDS,
        EntityKind::Document => DOCUMENT_FIELDS,
    }
}

/// Parse the `:kind` path segment.
pub fn kind_from_str(s: &str) -> Option<EntityKind> {
    Some(match s {
        "person" => EntityKind::Person,
        "family" => EntityKind::Family,
        "event" => EntityKind::Event,
        "link" => EntityKind::Link,
        "occupation" => EntityKind::Occupation,
        "source" => EntityKind::Source,
        "place" => EntityKind::Place,
        "document" => EntityKind::Document,
        _ => return None,
    })
}

/// The flat-bundle collection an entity kind lives in.
///
/// The inverse of [`kind_from_str`], and the one place that mapping is
/// written down: a write path that needs the entity the bundle currently
/// holds has to know where to look for it.
pub fn collection_for(kind: EntityKind) -> &'static str {
    match kind {
        EntityKind::Person => "persons",
        EntityKind::Family => "families",
        EntityKind::Event => "events",
        EntityKind::Link => "links",
        EntityKind::Occupation => "occupations",
        EntityKind::Source => "sources",
        EntityKind::Place => "places",
        EntityKind::Document => "documents",
    }
}

/// Every kind, for navigation.
pub const KINDS: [&str; 8] = [
    "person",
    "family",
    "event",
    "link",
    "occupation",
    "source",
    "place",
    "document",
];

/// Parse the delete policy chosen on the form. Reject is the default because
/// it is the only one that cannot lose data.
pub fn policy_from_str(s: &str) -> axgf_rs::DeletePolicy {
    match s {
        "cascade" => axgf_rs::DeletePolicy::Cascade,
        "orphan" => axgf_rs::DeletePolicy::Orphan,
        _ => axgf_rs::DeletePolicy::Reject,
    }
}

/// Read a dotted path out of an entity, as a string for form pre-fill.
pub fn get_path(entity: &Value, path: &str) -> String {
    let mut cur = entity;
    for seg in path.split('.') {
        cur = match seg.parse::<usize>() {
            Ok(i) => match cur.get(i) {
                Some(v) => v,
                None => return String::new(),
            },
            Err(_) => match cur.get(seg) {
                Some(v) => v,
                None => return String::new(),
            },
        };
    }
    match cur {
        Value::String(s) => s.clone(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        Value::Null => String::new(),
        other => other.to_string(),
    }
}

/// Write a dotted path into an entity, creating containers as needed.
///
/// A `None` value removes the key instead of writing a null or an empty
/// string: an absent field and a field explicitly set to "" are different
/// things to a JSON Schema, and the empty one is usually invalid.
pub fn set_path(entity: &mut Value, path: &str, value: Option<Value>) {
    let segs: Vec<&str> = path.split('.').collect();
    set_path_inner(entity, &segs, value);
}

fn set_path_inner(cur: &mut Value, segs: &[&str], value: Option<Value>) {
    let Some((head, rest)) = segs.split_first() else {
        return;
    };

    if let Ok(idx) = head.parse::<usize>() {
        if !cur.is_array() {
            if value.is_none() {
                return;
            }
            *cur = Value::Array(Vec::new());
        }
        let arr = cur.as_array_mut().expect("just ensured array");
        if rest.is_empty() {
            match value {
                Some(v) => {
                    while arr.len() <= idx {
                        arr.push(Value::Null);
                    }
                    arr[idx] = v;
                }
                None => {
                    if idx < arr.len() {
                        arr.remove(idx);
                    }
                }
            }
            return;
        }
        if arr.len() <= idx {
            if value.is_none() {
                return;
            }
            while arr.len() <= idx {
                arr.push(Value::Object(Map::new()));
            }
        }
        set_path_inner(&mut arr[idx], rest, value);
        return;
    }

    if !cur.is_object() {
        if value.is_none() {
            return;
        }
        *cur = Value::Object(Map::new());
    }
    let obj = cur.as_object_mut().expect("just ensured object");

    if rest.is_empty() {
        match value {
            Some(v) => {
                obj.insert((*head).to_string(), v);
            }
            None => {
                obj.remove(*head);
            }
        }
        return;
    }

    if !obj.contains_key(*head) {
        if value.is_none() {
            return;
        }
        obj.insert((*head).to_string(), Value::Object(Map::new()));
    }
    let removing = value.is_none();
    let child = obj.get_mut(*head).expect("just ensured present");
    set_path_inner(child, rest, value);

    // Drop containers the removal emptied, so a cleared date does not leave
    // `{"date": {}}` behind to fail validation.
    if removing {
        let empty = match obj.get(*head) {
            Some(Value::Object(m)) => m.is_empty(),
            _ => false,
        };
        if empty {
            obj.remove(*head);
        }
    }
}

/// Apply submitted form values onto a base entity.
///
/// `base` is the raw-JSON textarea when the user supplied one, otherwise the
/// entity being edited (or `{}` for a create). The typed fields are then
/// written over it, so a field covered by the form always reflects what the
/// form shows.
pub fn apply_form(
    base: Value,
    kind: EntityKind,
    form: &std::collections::HashMap<String, String>,
) -> Value {
    let mut entity = if base.is_object() {
        base
    } else {
        Value::Object(Map::new())
    };

    for field in fields_for(kind) {
        let raw = form.get(field.path).map(String::as_str).unwrap_or("");
        let trimmed = raw.trim();

        let value = match field.kind {
            // An unchecked checkbox is simply absent from the submission.
            FieldKind::Bool => {
                let on = matches!(trimmed, "on" | "true" | "1" | "yes");
                if on {
                    Some(Value::Bool(true))
                } else {
                    None
                }
            }
            FieldKind::Confidence => match trimmed.parse::<f64>() {
                Ok(v) if !trimmed.is_empty() => {
                    serde_json::Number::from_f64(v.clamp(0.0, 1.0)).map(Value::Number)
                }
                _ => None,
            },
            _ => {
                if trimmed.is_empty() {
                    None
                } else {
                    Some(Value::String(trimmed.to_string()))
                }
            }
        };

        set_path(&mut entity, field.path, value);
    }

    ensure_required(kind, &mut entity);
    entity
}

/// Fill in the schema-required pieces a form cannot sensibly ask for.
///
/// The schema requires `identity.gender`, `identity.is_living` and
/// `name.components` on every person. Leaving them out produces an entity the
/// library accepts — validation is non-blocking — but flags with
/// `SCHEMA_VALIDATION_FAILED` warnings on every subsequent validate. Filling
/// them with honest defaults ("U" for unrecorded gender, an empty component
/// list) means the admin panel creates valid entities rather than immediately
/// dirtying the bundle.
///
/// Only structural requirements are filled. Nothing here invents a fact: an
/// unrecorded gender becomes the schema's own "unknown", not a guess.
fn ensure_required(kind: EntityKind, entity: &mut Value) {
    match kind {
        EntityKind::Person => {
            // Only complete an identity that exists; conjuring one for an
            // empty submission would hide the real problem.
            if entity.get("identity").map(Value::is_object) != Some(true) {
                return;
            }
            if get_path(entity, "identity.name.display").is_empty() {
                return;
            }
            if entity
                .pointer("/identity/name/components")
                .map(Value::is_array)
                != Some(true)
            {
                set_path(
                    entity,
                    "identity.name.components",
                    Some(Value::Array(vec![])),
                );
            }
            if get_path(entity, "identity.gender.value").is_empty() {
                set_path(
                    entity,
                    "identity.gender.value",
                    Some(Value::String("U".into())),
                );
            }
            if entity.pointer("/identity/is_living").map(Value::is_boolean) != Some(true) {
                set_path(entity, "identity.is_living", Some(Value::Bool(false)));
            }
        }
        // A place name needs a language tag alongside its value.
        EntityKind::Place
            if !get_path(entity, "names.0.value").is_empty()
                && get_path(entity, "names.0.lang").is_empty() =>
        {
            set_path(entity, "names.0.lang", Some(Value::String("en".into())));
        }
        _ => {}
    }
}

/// One page of a listing.
pub struct Page<T> {
    pub items: Vec<T>,
    pub page: usize,
    pub pages: usize,
    pub total: usize,
    pub per_page: usize,
}

/// Entities per admin listing page.
pub const PER_PAGE: usize = 50;

/// Slice `items` into the requested page, clamping out-of-range requests.
pub fn paginate<T>(items: Vec<T>, page: usize) -> Page<T> {
    let total = items.len();
    let pages = total.div_ceil(PER_PAGE).max(1);
    let page = page.clamp(1, pages);
    let start = (page - 1) * PER_PAGE;
    let items = items.into_iter().skip(start).take(PER_PAGE).collect();
    Page {
        items,
        page,
        pages,
        total,
        per_page: PER_PAGE,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::collections::HashMap;

    fn form(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs
            .iter()
            .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
            .collect()
    }

    #[test]
    fn every_kind_parses_and_has_fields() {
        for k in KINDS {
            let kind = kind_from_str(k).unwrap_or_else(|| panic!("{k} should parse"));
            assert!(!fields_for(kind).is_empty(), "{k} has no fields");
        }
        assert!(kind_from_str("nonsense").is_none());
        assert!(kind_from_str("persons").is_none(), "plural is not a kind");
    }

    #[test]
    fn set_path_creates_nested_objects() {
        let mut v = json!({});
        set_path(&mut v, "identity.name.display", Some(json!("Ada")));
        assert_eq!(v, json!({"identity": {"name": {"display": "Ada"}}}));
    }

    #[test]
    fn set_path_with_none_prunes_every_container_it_empties() {
        // Clearing the only value leaves no empty husks: `{"date": {}}` would
        // fail schema validation, and an empty `birth` asserts nothing either.
        let mut v = json!({"birth": {"date": {"value": "1923"}}});
        set_path(&mut v, "birth.date.value", None);
        assert_eq!(v, json!({}), "got {v}");

        // A sibling with content stops the cascade at the right level.
        let mut v = json!({"birth": {"date": {"value": "1923"}, "confidence": 0.9}});
        set_path(&mut v, "birth.date.value", None);
        assert_eq!(v, json!({"birth": {"confidence": 0.9}}), "got {v}");
    }

    #[test]
    fn set_path_handles_array_indices() {
        let mut v = json!({});
        set_path(&mut v, "names.0.value", Some(json!("Warsaw")));
        set_path(&mut v, "names.0.lang", Some(json!("en")));
        assert_eq!(v, json!({"names": [{"value": "Warsaw", "lang": "en"}]}));
    }

    #[test]
    fn get_path_reads_back_what_set_path_wrote() {
        let mut v = json!({});
        set_path(&mut v, "a.b.c", Some(json!("x")));
        assert_eq!(get_path(&v, "a.b.c"), "x");
        assert_eq!(get_path(&v, "a.b.missing"), "");
        assert_eq!(get_path(&v, "nope.nope"), "");
    }

    #[test]
    fn get_path_stringifies_scalars_for_prefill() {
        let v = json!({"n": 0.85, "b": true, "s": "t", "z": null});
        assert_eq!(get_path(&v, "n"), "0.85");
        assert_eq!(get_path(&v, "b"), "true");
        assert_eq!(get_path(&v, "s"), "t");
        assert_eq!(get_path(&v, "z"), "");
    }

    #[test]
    fn empty_form_fields_are_omitted_not_written_as_empty_strings() {
        // A schema with minLength would reject "", and an absent field is what
        // "not recorded" actually means.
        let e = apply_form(
            json!({}),
            EntityKind::Person,
            &form(&[
                ("identity.name.display", "Ada Lovelace"),
                ("birth.date.value", "   "),
                ("bio", ""),
            ]),
        );
        assert_eq!(e["identity"]["name"]["display"], "Ada Lovelace");
        assert!(e.get("bio").is_none(), "empty bio must not be written");
        assert!(
            e.get("birth").is_none() || e["birth"].get("date").is_none(),
            "a blank date must not create an empty date object: {e}"
        );
    }

    #[test]
    fn an_unchecked_checkbox_clears_the_flag() {
        let base = json!({"identity": {"is_living": true, "name": {"display": "X"}}});
        let e = apply_form(
            base,
            EntityKind::Person,
            &form(&[("identity.name.display", "X")]),
        );
        // is_living is schema-required, so it becomes false rather than absent.
        assert_eq!(
            e["identity"]["is_living"],
            json!(false),
            "an unchecked box means false, got {e}"
        );
    }

    #[test]
    fn a_created_person_satisfies_the_schemas_required_fields() {
        // Without this the library accepts the entity but flags
        // SCHEMA_VALIDATION_FAILED warnings on every later validate.
        let e = apply_form(
            json!({}),
            EntityKind::Person,
            &form(&[("identity.name.display", "Ada Lovelace")]),
        );
        assert!(e["identity"]["name"]["components"].is_array());
        assert_eq!(e["identity"]["gender"]["value"], json!("U"));
        assert_eq!(e["identity"]["is_living"], json!(false));
    }

    #[test]
    fn an_empty_person_submission_is_not_papered_over() {
        // Nothing typed means nothing to complete; the library should get the
        // empty entity and say what is missing.
        let e = apply_form(json!({}), EntityKind::Person, &form(&[]));
        assert!(e.get("identity").is_none(), "got {e}");
    }

    #[test]
    fn a_place_name_gains_a_language_tag() {
        let e = apply_form(
            json!({}),
            EntityKind::Place,
            &form(&[("names.0.value", "Warszawa")]),
        );
        assert_eq!(e["names"][0]["lang"], json!("en"));
        // An explicit tag is respected.
        let e = apply_form(
            json!({}),
            EntityKind::Place,
            &form(&[("names.0.value", "Warszawa"), ("names.0.lang", "pl")]),
        );
        assert_eq!(e["names"][0]["lang"], json!("pl"));
    }

    #[test]
    fn a_checked_checkbox_sets_true() {
        let e = apply_form(
            json!({}),
            EntityKind::Person,
            &form(&[("identity.name.display", "X"), ("identity.is_living", "on")]),
        );
        assert_eq!(e["identity"]["is_living"], json!(true));
    }

    #[test]
    fn confidence_is_parsed_and_clamped() {
        let e = apply_form(
            json!({}),
            EntityKind::Person,
            &form(&[
                ("identity.name.display", "X"),
                ("birth.confidence", "0.35"),
                ("death.confidence", "5"),
            ]),
        );
        assert_eq!(e["birth"]["confidence"], json!(0.35));
        assert_eq!(e["death"]["confidence"], json!(1.0));
    }

    #[test]
    fn typed_fields_are_written_over_the_raw_json_base() {
        // The raw textarea supplies fields the form does not cover; the typed
        // fields own the paths they show.
        let base = json!({
            "identity": {"name": {"display": "From raw"}},
            "tags": ["kept-from-raw"]
        });
        let e = apply_form(
            base,
            EntityKind::Person,
            &form(&[("identity.name.display", "From the form")]),
        );
        assert_eq!(e["identity"]["name"]["display"], "From the form");
        assert_eq!(
            e["tags"],
            json!(["kept-from-raw"]),
            "uncovered fields survive"
        );
    }

    #[test]
    fn a_non_object_base_is_replaced_rather_than_crashing() {
        let e = apply_form(
            json!("not an object"),
            EntityKind::Person,
            &form(&[("identity.name.display", "X")]),
        );
        assert!(e.is_object());
        assert_eq!(e["identity"]["name"]["display"], "X");
    }

    #[test]
    fn delete_policy_defaults_to_reject() {
        assert_eq!(policy_from_str("cascade"), axgf_rs::DeletePolicy::Cascade);
        assert_eq!(policy_from_str("orphan"), axgf_rs::DeletePolicy::Orphan);
        assert_eq!(policy_from_str("reject"), axgf_rs::DeletePolicy::Reject);
        // Anything unexpected must fall back to the policy that cannot lose data.
        assert_eq!(policy_from_str(""), axgf_rs::DeletePolicy::Reject);
        assert_eq!(policy_from_str("garbage"), axgf_rs::DeletePolicy::Reject);
    }

    #[test]
    fn pagination_clamps_and_reports_totals() {
        let items: Vec<usize> = (0..120).collect();
        let p = paginate(items.clone(), 1);
        assert_eq!(p.items.len(), 50);
        assert_eq!((p.page, p.pages, p.total), (1, 3, 120));

        let p = paginate(items.clone(), 3);
        assert_eq!(p.items.len(), 20);
        assert_eq!(p.items[0], 100);

        // Out of range in both directions clamps into the valid range.
        assert_eq!(paginate(items.clone(), 99).page, 3);
        assert_eq!(paginate(items.clone(), 0).page, 1);

        let empty: Vec<usize> = Vec::new();
        let p = paginate(empty, 1);
        assert_eq!((p.page, p.pages, p.total), (1, 1, 0));
    }
}
