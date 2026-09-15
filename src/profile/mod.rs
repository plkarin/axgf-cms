//! The AXGF 1.1 person profile, as this application draws and edits it.
//!
//! # Nothing here lists an attribute
//!
//! Fourteen groups, 132 attributes and 102 vocabularies are a lot to keep in
//! step with a specification, and this module does not try. It reads the
//! library's registry (`axgf_rs::model::profile::registry`) for every group,
//! attribute, value shape, vocabulary and class, and derives the rest: the
//! catalogue key of every label, the input every closed vocabulary becomes,
//! the way a claim is parsed back out of a form. An attribute the library
//! gains is on the record page and in the editor the day the dependency is
//! bumped, and a test fails until its label is in the catalogues.
//!
//! * [`view`] — a group as a reader sees it, withheld classes and all.
//! * [`form`] — a group as an editor edits it, and the claims read back.
//! * [`lift`] — the values this application recorded in its own extensions
//!   before 1.1 gave them first-class homes.
//!
//! # Catalogue keys
//!
//! | what              | key                                  |
//! |-------------------|--------------------------------------|
//! | group             | `pg-<group>`, `pg-<group>-intro`     |
//! | attribute         | `pa-<block>-<key>`                   |
//! | field of a value  | `pf-<block>-<key>-<field>`, else `pf-<field>` |
//! | vocabulary term   | `pv-<vocabulary>-<term>`             |
//! | number with unit  | `pu-<unit>`, with `$n`               |
//!
//! Underscores become dashes, as everywhere else in the catalogues.
//!
//! # What is shown untranslated, and why
//!
//! Five vocabularies are notation rather than language — ABO blood groups,
//! Y-DNA and mitochondrial haplogroups, UCUM units and MBTI types — and are
//! shown as written: nobody translates `R1b`, `mmol/L` or `INTJ`. A military
//! rank is shown by the title its own army gives it, taken from the library:
//! a Polish *kapral* is *kapral* whatever language the reader uses, for the
//! same reason a place is named as the record names it.

pub mod form;
pub mod lift;
pub mod view;

/// Whether an entity carries anything AXGF 1.1 defines, which obliges it to
/// declare `"1.1"` (SPEC_1.1 §2.1).
///
/// The library answers the same question when it validates, and reports a
/// mismatch as `SPEC_VERSION_MISMATCH`; it only stamps a version on *create*,
/// though, and the editors here mostly *update*. So a person given a blood
/// group by the profile editor kept saying `"1.0"` until this existed. The
/// test `every_write_declares_the_version_its_content_needs` saves one of
/// each shape through the real write path and holds this to the library's
/// own answer.
pub fn uses_1_1(kind: axgf_rs::EntityKind, entity: &serde_json::Value) -> bool {
    use axgf_rs::EntityKind as K;
    use serde_json::Value;
    match kind {
        K::Person => {
            // Presence, not non-null, exactly as the library tests it: a
            // `"health": null` is still a 1.1 key, and a copy of the rule
            // that disagreed with the validator would stamp one version and
            // be warned about the other.
            registry::PROFILE_BLOCKS
                .iter()
                .any(|b| entity.get(*b).is_some())
                || registry::attributes()
                    .filter(|a| matches!(a.block, "identity" | "birth" | "death"))
                    .any(|a| entity.get(a.block).and_then(|b| b.get(a.key)).is_some())
                || entity.pointer("/identity/class_visibility").is_some()
        }
        K::Family => entity
            .get("children")
            .and_then(Value::as_array)
            .is_some_and(|cs| cs.iter().any(|c| c.get("lineage").is_some())),
        K::Link => entity.get("relation").is_some(),
        K::Occupation => entity.get("position").is_some(),
        _ => false,
    }
}

/// Raise an entity's declared `axgf_version` to what its content needs.
///
/// Only ever raised. An entity that stops carrying 1.1 content keeps saying
/// `"1.1"`, which is still true of it and costs nothing: the manifest a
/// version was once raised for is never lowered either. A version this build
/// does not know is left alone — it is newer than anything this could write.
pub fn declare_version(kind: axgf_rs::EntityKind, entity: &mut serde_json::Value) {
    if !uses_1_1(kind, entity) {
        return;
    }
    let declared = entity
        .get("axgf_version")
        .and_then(serde_json::Value::as_str);
    let rank = |v: &str| {
        axgf_rs::SUPPORTED_SPEC_VERSIONS
            .iter()
            .position(|s| *s == v)
    };
    let needs = rank("1.1");
    let raise = match declared.map(|d| (d, rank(d))) {
        None => true,
        Some((_, None)) => false,
        Some((_, known)) => known < needs,
    };
    if raise {
        entity["axgf_version"] = serde_json::Value::from("1.1");
    }
}

use axgf_rs::model::profile::registry::{self, Attribute, Group};
use axgf_rs::model::profile::vocab::{self, Vocabulary};

use crate::sensitive::Scopes;

/// The vocabularies shown as written. See the module documentation.
pub const NOTATION_VOCABULARIES: &[&str] = &[
    "blood_group",
    "y_haplogroup",
    "mt_haplogroup",
    "lab_unit",
    "mbti",
];

/// Whether a vocabulary's terms are shown as written rather than translated.
pub fn is_notation(vocabulary: &Vocabulary) -> bool {
    NOTATION_VOCABULARIES.contains(&vocabulary.name)
        || vocabulary.name.starts_with("military_rank_")
}

fn dashed(s: &str) -> String {
    s.replace('_', "-")
}

/// `pg-<group>`.
pub fn group_key(g: &Group) -> String {
    format!("pg-{}", dashed(g.key))
}

/// `pg-<group>-intro`.
pub fn group_intro_key(g: &Group) -> String {
    format!("pg-{}-intro", dashed(g.key))
}

/// `pa-<block>-<key>`.
pub fn attribute_key(a: &Attribute) -> String {
    format!("pa-{}-{}", dashed(a.block), dashed(a.key))
}

/// The specific key for one field of an attribute's value.
pub fn field_key_specific(a: &Attribute, field: &str) -> String {
    format!("pf-{}-{}-{}", dashed(a.block), dashed(a.key), dashed(field))
}

/// The shared key for a field, used when no specific one exists.
pub fn field_key_shared(field: &str) -> String {
    format!("pf-{}", dashed(field))
}

/// `pv-<vocabulary>-<term>`.
pub fn term_key(vocabulary: &str, term: &str) -> String {
    format!("pv-{}-{}", dashed(vocabulary), dashed(term))
}

/// `pu-<unit>`, for a unit as the registry writes it.
pub fn unit_key(unit: &str) -> Option<String> {
    let slug = match unit {
        "" => return None,
        "cm" => "cm",
        "kg" => "kg",
        "kg/m²" => "kg-m2",
        "%" => "percent",
        "mm" => "mm",
        "Hz" => "hz",
        "words/min" => "words-min",
        "dB HL" => "db-hl",
        "mmHg" => "mmhg",
        "bpm" => "bpm",
        "L" => "litres",
        "x" => "coverage",
        "years" => "years",
        "t CO2e/yr" => "t-co2e-yr",
        other => return Some(format!("pu-{}", dashed(&other.to_ascii_lowercase()))),
    };
    Some(format!("pu-{slug}"))
}

/// A group's title in the reader's language.
pub fn group_title(lang: &str, g: &Group) -> String {
    crate::i18n::translate(lang, &group_key(g), None)
}

/// An attribute's label in the reader's language.
pub fn attribute_label(lang: &str, a: &Attribute) -> String {
    crate::i18n::translate(lang, &attribute_key(a), None)
}

/// A field's label: the attribute's own wording when it has one.
pub fn field_label(lang: &str, a: &Attribute, field: &str) -> String {
    let specific = field_key_specific(a, field);
    if crate::i18n::has_message(crate::i18n::DEFAULT, &specific) {
        crate::i18n::translate(lang, &specific, None)
    } else {
        crate::i18n::translate(lang, &field_key_shared(field), None)
    }
}

/// A vocabulary term in the reader's language, or as written for notation.
///
/// A value that is not a term of its vocabulary is shown as it was recorded:
/// the library reports it, and a record that prints somebody's own word is
/// telling the truth about what was written.
pub fn term_label(lang: &str, vocabulary: &Vocabulary, term: &str) -> String {
    if is_notation(vocabulary) || !vocabulary.contains(term) {
        return term.to_string();
    }
    crate::i18n::translate(lang, &term_key(vocabulary.name, term), None)
}

/// A number with its unit, through the catalogue so that the unit's symbol and
/// its position follow the language.
pub fn with_unit(lang: &str, unit: &str, n: &str) -> String {
    match unit_key(unit) {
        Some(key) => {
            let args = crate::i18n::args(&[("n", fluent::FluentValue::from(n))]);
            crate::i18n::translate(lang, &key, Some(&args))
        }
        None => n.to_string(),
    }
}

/// The title of a registered military rank, or `None` when the country has no
/// registered list or the rank is not in it.
pub fn rank_title(country: &str, term: &str) -> Option<&'static str> {
    vocab::MILITARY_RANKS
        .iter()
        .find(|r| r.country == country)
        .and_then(|r| r.ranks.iter().find(|x| x.term == term))
        .map(|x| x.title)
}

/// The scopes one attribute is governed by, from the same table
/// [`crate::sensitive`] uses for every other surface.
pub fn attribute_scopes(a: &Attribute) -> Scopes {
    crate::sensitive::location_scopes(a.block, a.key)
}

/// The groups, in the specification's order.
pub fn groups() -> &'static [Group] {
    registry::GROUPS
}

/// A group by its key.
pub fn group(key: &str) -> Option<&'static Group> {
    registry::group(key)
}

/// Every catalogue key the profile can ask for, for the test that holds the
/// catalogues to the registry.
pub fn every_key() -> Vec<String> {
    let mut out = Vec::new();
    for g in groups() {
        out.push(group_key(g));
        out.push(group_intro_key(g));
        for a in g.attributes {
            out.push(attribute_key(a));
            collect_shape_keys(a, &a.shape, &mut out);
        }
    }
    out.sort();
    out.dedup();
    out
}

fn collect_shape_keys(
    a: &Attribute,
    shape: &axgf_rs::model::profile::registry::Shape,
    out: &mut Vec<String>,
) {
    use axgf_rs::model::profile::registry::Shape;
    match shape {
        Shape::Vocab(v) => {
            if !is_notation(v) {
                out.extend(v.terms.iter().map(|t| term_key(v.name, t)));
            }
        }
        Shape::Number { unit, .. } | Shape::Integer { unit, .. } => out.extend(unit_key(unit)),
        Shape::Rank => {
            out.extend(
                vocab::RANK_CATEGORY
                    .terms
                    .iter()
                    .map(|t| term_key("rank_category", t)),
            );
        }
        Shape::Artefact(types) => {
            out.extend(types.iter().map(|t| term_key("artefact_type", t)));
            out.extend(vocab::CONSENT.terms.iter().map(|t| term_key("consent", t)));
            for f in [
                "document_id",
                "artefact_type",
                "format",
                "generator",
                "derived_from_id",
                "consent",
            ] {
                out.push(field_key_shared(f));
            }
        }
        Shape::Coordinates => {
            for f in ["lat", "lon", "precision"] {
                out.push(field_key_shared(f));
            }
        }
        Shape::Object { fields, .. } => {
            for f in fields.iter() {
                let specific = field_key_specific(a, f.key);
                // Either key satisfies the field; the test checks English
                // holds one of the two, so only the shared one is listed
                // here when the specific one is absent.
                if crate::i18n::has_message(crate::i18n::DEFAULT, &specific) {
                    out.push(specific);
                } else {
                    out.push(field_key_shared(f.key));
                }
                collect_shape_keys(a, &f.shape, out);
            }
        }
        _ => {}
    }
}
