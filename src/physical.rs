//! The two extension objects this application wrote before AXGF 1.1.
//!
//! # What they were
//!
//! AXGF 1.0 had no field for how tall somebody was, what colour their eyes
//! were, or what they died of, and the right response to that was not to
//! invent one. So this application recorded them under `extensions`, in two
//! namespaced objects rather than one:
//!
//! * [`TRAITS_KEY`] held what a passport or a conscription register records —
//!   height, weight, eye and hair colour, build, handedness, distinguishing
//!   features, military service, languages. None of it a medical fact.
//! * [`HEALTH_KEY`] held what is a special category of personal data —
//!   conditions, operations, blood group, cause of death, and religion, which
//!   article 9 of the GDPR lists beside health.
//!
//! Every entry was a dated, sourced, rated claim, in exactly the shape AXGF 1.1
//! later gave its own attributes.
//!
//! # What they are now
//!
//! AXGF 1.1 gives nearly all of it a first-class home, and
//! [`crate::profile::lift`] moves each entry there — on the page as soon as
//! this version reads the bundle, and in the bundle the first time the profile
//! editor saves the person. What has no home stays in its extension, and is
//! still shown and still editable under its old label. The health object is
//! governed as the `health` class by [`crate::sensitive`], like everything else
//! of that class.
//!
//! Nothing here writes either object any more; it names them.

/// Namespaced key for the non-sensitive half.
pub const TRAITS_KEY: &str = "axgf-cms:traits/v1";

/// Namespaced key for the special-category half.
pub const HEALTH_KEY: &str = "axgf-cms:health/v1";

/// Every field either object ever held, so that each one — lifted or not — has
/// a label a reader can see it under.
pub const LEGACY_FIELDS: &[&str] = &[
    "height_cm",
    "weight_kg",
    "eye_colour",
    "hair_colour",
    "build",
    "handedness",
    "features",
    "military",
    "languages",
    "blood_group",
    "conditions",
    "operations",
    "cause_of_death",
    "religion",
    "health_notes",
];

/// The catalogue key for a legacy field's label.
pub fn label_key(field: &str) -> String {
    format!("phys-field-{}", field.replace('_', "-"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_field_has_a_distinct_name_and_a_dashed_label_key() {
        let names: std::collections::BTreeSet<_> = LEGACY_FIELDS.iter().collect();
        assert_eq!(names.len(), LEGACY_FIELDS.len());
        for f in LEGACY_FIELDS {
            assert!(!label_key(f).contains('_'));
        }
    }
}
