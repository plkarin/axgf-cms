//! Whether to show somebody as living, and the line this rule does not cross.
//!
//! # What is wrong with the file
//!
//! GEDCOM has no way to record "died, date unknown". A person whose death was
//! never written down leaves the format with nothing to say, so a converter
//! writes `is_living: true` and the receiving application repeats it. Pelagia
//! Simla, born 1898, arrives in the bundle marked living and this application
//! rendered her as `1898 – living`, which in 2026 is a claim that somebody is
//! 128 years old. The converter is not at fault and cannot be fixed: the
//! answer is not in the file.
//!
//! # The rule
//!
//! A person whose birth is more than [`DEFAULT_MAX_AGE_YEARS`] years ago is
//! *presumed* deceased, whatever the record says. Presumed is the whole of it:
//!
//! * **The bundle is never touched.** This is a display rule. The record still
//!   says what the source said, the raw JSON still shows `is_living: true`,
//!   and an export carries the file the operator was given.
//! * **The interface says which it is.** "Presumed deceased", not a dash where
//!   a death date would go and not a silent blank. A reader must not be able
//!   to mistake an inference for a record, because the difference between what
//!   is written down and what is worked out is the whole argument of this
//!   product.
//! * **No age is stated.** A presumed-deceased person has no death year, so
//!   there is no span to give. The figure beside the record disappears with
//!   it, for the same reason: nobody knows what age to draw.
//!
//! # Permissions do not use this, and must not
//!
//! Read this before routing anything else through here.
//!
//! A living person's health data is withheld from everybody but an
//! administrator — see [`crate::access::health_visibility`]. That rule keys on
//! living status, so a presumption that flips somebody to deceased would
//! *publish* their conditions, their cause of death and their religion to
//! every signed-in relative. The same is true of
//! [`crate::access::person_visibility`], where an absent visibility on a
//! living person defaults to `members` and on a deceased one to `public`.
//!
//! So the two questions are kept apart by name and by module:
//!
//! | question | answer from | changes with this rule |
//! |---|---|---|
//! | how is this person *shown* | [`status`] | yes |
//! | what may this reader *read* | [`crate::access`] | no |
//!
//! `access` reads `identity.is_living` directly and deliberately, and there is
//! a test whose entire job is to fail if that ever stops being true. A
//! plausibility rule that widens access is not a plausibility rule, it is a
//! privacy hole with a date arithmetic in front of it.

use std::sync::OnceLock;

use serde::Serialize;
use serde_json::Value;

/// Years after a birth beyond which a person is presumed deceased.
///
/// 120 rather than 115 or 125 because the oldest verified human lifespan is
/// 122 and the number should sit just under the limit of the possible rather
/// than at the edge of the probable: the rule exists to catch a converter's
/// silence, not to declare that a very old person has died. At 120 the false
/// positives are people who would be the oldest in the world, and the false
/// negatives are a few records that stay wrong for another decade.
pub const DEFAULT_MAX_AGE_YEARS: i64 = 120;

/// The operator's own figure, set once at startup.
static MAX_AGE: OnceLock<i64> = OnceLock::new();

/// The configured limit, or [`DEFAULT_MAX_AGE_YEARS`] when nothing set one.
pub fn max_age_years() -> i64 {
    *MAX_AGE.get_or_init(|| DEFAULT_MAX_AGE_YEARS)
}

/// Set the limit. Called once, from the binary, before anything renders.
///
/// Zero switches the presumption off entirely, which is a supported way to run
/// rather than a missing feature: an operator whose bundle records deaths
/// properly does not need it, and one who disagrees with the arithmetic should
/// be able to say so without editing the source.
pub fn set_max_age_years(years: i64) {
    let _ = MAX_AGE.set(years.max(0));
}

/// How a person's living status is shown.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Status {
    /// The record says living and the dates do not contradict it.
    Living,
    /// The record says deceased.
    Deceased,
    /// The record says living and the arithmetic says otherwise.
    PresumedDeceased,
}

impl Status {
    /// Whether to *show* this person as living. Never ask this about
    /// permissions; see the module docs.
    pub fn shown_as_living(self) -> bool {
        matches!(self, Status::Living)
    }

    /// True only for the inferred case, so the interface can mark it as one.
    pub fn is_presumed(self) -> bool {
        matches!(self, Status::PresumedDeceased)
    }

    /// The catalogue key for the status said out loud.
    pub fn key(self) -> &'static str {
        match self {
            Status::Living => "record-living-yes",
            Status::Deceased => "record-deceased",
            Status::PresumedDeceased => "record-presumed-deceased",
        }
    }
}

/// The status to show for one stored person.
pub fn status(person: &Value) -> Status {
    status_at(person, max_age_years(), current_year())
}

/// [`status`] with the limit and the year given, which is what the tests use:
/// a rule about how long ago something was cannot be tested against a clock
/// that moves.
pub fn status_at(person: &Value, max_age_years: i64, now_year: i64) -> Status {
    let recorded_living = person
        .get("identity")
        .and_then(|i| i.get("is_living"))
        .and_then(Value::as_bool)
        .unwrap_or(false);
    if !recorded_living {
        return Status::Deceased;
    }
    // No birth year is no evidence, and a presumption needs evidence. A record
    // that says living and says nothing else stays living.
    let Some(born) = crate::view::latest_year_of_field(person, "birth") else {
        return Status::Living;
    };
    if max_age_years > 0 && now_year - born > max_age_years {
        Status::PresumedDeceased
    } else {
        Status::Living
    }
}

/// This year.
///
/// Days since the epoch over the mean Gregorian year. Good to the day, which
/// is far more precision than a rule measured in decades needs.
pub fn current_year() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    1970 + (secs / 86_400) * 400 / 146_097
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn person(living: bool, birth: Option<&str>) -> Value {
        let mut p = json!({"identity": {"is_living": living}});
        if let Some(b) = birth {
            p["birth"] = json!({"date": {"value": b, "precision": "year"}});
        }
        p
    }

    #[test]
    fn a_record_that_says_deceased_is_deceased() {
        assert_eq!(
            status_at(&person(false, Some("1898")), 120, 2026),
            Status::Deceased
        );
        // And an absent flag is the same answer, which is what every other
        // reader of this field already assumes.
        assert_eq!(status_at(&json!({}), 120, 2026), Status::Deceased);
    }

    #[test]
    fn the_operators_own_case_is_the_one_this_exists_for() {
        // Pelagia Simla, born 1898, marked living by the converter because
        // GEDCOM had no way to say her death was never written down.
        assert_eq!(
            status_at(&person(true, Some("1898")), 120, 2026),
            Status::PresumedDeceased
        );
    }

    #[test]
    fn the_boundary_is_more_than_rather_than_at_least() {
        // Exactly 120 is somebody's 120th year, which happens. 121 is past
        // every verified lifespan but one.
        let born = 2026 - 120;
        assert_eq!(
            status_at(&person(true, Some(&born.to_string())), 120, 2026),
            Status::Living
        );
        let born = 2026 - 121;
        assert_eq!(
            status_at(&person(true, Some(&born.to_string())), 120, 2026),
            Status::PresumedDeceased
        );
    }

    #[test]
    fn no_birth_year_is_no_presumption() {
        // A presumption needs something to presume from. Nearly half the
        // operator's bundle has no dates at all, and inventing a status for
        // them would be the same defect in the other direction.
        assert_eq!(status_at(&person(true, None), 120, 2026), Status::Living);
        assert_eq!(
            status_at(&person(true, Some("not a date")), 120, 2026),
            Status::Living
        );
    }

    #[test]
    fn a_ranged_birth_is_read_from_its_latest_bound() {
        // The converter emits ranged dates with no top-level value, and the
        // operator's bundle is full of them. Reading only `value` would leave
        // those people living for ever.
        let bounded = json!({
            "identity": {"is_living": true},
            "birth": {"date": {"precision": "unknown",
                "range": {"latest": {"value": "1885", "precision": "year"}}}}
        });
        assert_eq!(status_at(&bounded, 120, 2026), Status::PresumedDeceased);
    }

    #[test]
    fn a_birth_with_only_a_lower_bound_presumes_nothing() {
        // "No earlier than 1880" bounds nothing: they could have been born in
        // 1990. Reading the earliest bound here would declare a thirty-six
        // year old dead, which is the failure this rule exists to avoid, in
        // the other direction.
        let open_ended = json!({
            "identity": {"is_living": true},
            "birth": {"date": {"precision": "unknown",
                "range": {"earliest": {"value": "1880", "precision": "year"}}}}
        });
        assert_eq!(status_at(&open_ended, 120, 2026), Status::Living);
    }

    #[test]
    fn zero_switches_the_rule_off() {
        assert_eq!(
            status_at(&person(true, Some("1700")), 0, 2026),
            Status::Living
        );
    }

    #[test]
    fn every_status_names_a_distinct_key() {
        let all = [Status::Living, Status::Deceased, Status::PresumedDeceased];
        let keys: std::collections::BTreeSet<_> = all.iter().map(|s| s.key()).collect();
        assert_eq!(keys.len(), all.len());
        assert!(Status::Living.shown_as_living());
        assert!(!Status::PresumedDeceased.shown_as_living());
        assert!(!Status::Deceased.shown_as_living());
        assert!(Status::PresumedDeceased.is_presumed());
        assert!(!Status::Deceased.is_presumed());
    }

    #[test]
    fn the_clock_is_this_century() {
        // A sanity check on the arithmetic rather than on the clock: if this
        // ever returns 1970 the epoch read failed and every presumption in the
        // application silently stops firing.
        let y = current_year();
        assert!((2020..2200).contains(&y), "current_year said {y}");
    }
}
