//! The Place entity as a structured record, rather than a string a converter
//! happened to split on a comma.
//!
//! # Why this exists
//!
//! §5.3 of the specification models a place with everything a genealogist
//! needs: several names, each in its own language, one of them primary; a type
//! from a fixed vocabulary; a region and a present-day country; the states
//! that held it over time; identifiers into Wikidata and GeoNames; and
//! coordinates. Almost none of it survives a GEDCOM import, because GEDCOM has
//! one field. `PLAC Słomniki, Pologne` becomes `names[0].value` and nothing
//! else — on the operator's file, 123 places with one Polish-tagged name
//! apiece, 26 of which carry a `region` the converter guessed by splitting on
//! the comma and got wrong ("Pologne", "France", "dom opieki").
//!
//! That matters beyond tidiness. A geocoder handed "Słomniki, Pologne" with no
//! structure does badly on a nineteenth-century Polish village; handed a name,
//! a region and a country as separate fields, it does far better. The
//! structure is what makes [`crate::geocode`] worth attempting at all.
//!
//! # Every field is optional
//!
//! A place with one name and nothing else is valid, and is what the whole
//! bundle currently looks like. Nothing here demands more than the record has.

use std::collections::HashMap;

use serde::Serialize;
use serde_json::{json, Map, Value};

/// The vocabulary from §5.3.1, in the order the specification lists it.
pub const PLACE_TYPES: &[&str] = &[
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
];

/// How precisely the coordinates locate the place.
///
/// The specification leaves `coordinates.precision` a free string, so this is
/// a vocabulary rather than a constraint. It exists because a village centre
/// and a house are different claims, and a geocoder that returns the middle of
/// a commune should not be recorded as though somebody surveyed a doorstep.
pub const PRECISIONS: &[&str] = &[
    "exact",
    "building",
    "street",
    "city_center",
    "region_center",
    "country_center",
    "approximate",
];

/// One of a place's names.
#[derive(Debug, Clone, Serialize, Default)]
pub struct PlaceName {
    pub lang: String,
    pub value: String,
    pub is_primary: bool,
}

/// One period during which a state held this place.
#[derive(Debug, Clone, Serialize, Default)]
pub struct CountryPeriod {
    pub country: String,
    pub from: String,
    pub until: String,
    pub note: String,
}

/// A position on the earth.
#[derive(Debug, Clone, Serialize, Default)]
pub struct Coordinates {
    pub lat: String,
    pub lon: String,
    pub precision: String,
}

/// A place, in the shape the editor renders and parses.
///
/// Strings throughout, including the coordinates: this is what a form holds,
/// and a half-typed latitude has to survive being re-rendered with an error
/// beside it rather than being silently dropped by a parse.
#[derive(Debug, Clone, Serialize, Default)]
pub struct PlaceForm {
    pub id: String,
    pub names: Vec<PlaceName>,
    pub place_type: String,
    pub region: String,
    pub country_current: String,
    pub history: Vec<CountryPeriod>,
    pub wikidata: String,
    pub geonames: String,
    pub coordinates: Coordinates,
    pub note: String,
    pub version: u64,
    /// The primary name, for headings. Empty for a place with no names, which
    /// the schema forbids but a hand-edited bundle can still contain.
    pub display: String,
}

fn s(v: &Value, key: &str) -> String {
    v.get(key)
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string()
}

fn num(v: &Value, key: &str) -> String {
    match v.get(key) {
        Some(Value::Number(n)) => n.to_string(),
        Some(Value::String(t)) => t.clone(),
        _ => String::new(),
    }
}

impl PlaceForm {
    /// Read a stored place into the form.
    pub fn from_entity(entity: &Value) -> Self {
        let names: Vec<PlaceName> = entity
            .get("names")
            .and_then(Value::as_array)
            .map(|a| {
                a.iter()
                    .map(|n| PlaceName {
                        lang: s(n, "lang"),
                        value: s(n, "value"),
                        is_primary: n
                            .get("is_primary")
                            .and_then(Value::as_bool)
                            .unwrap_or(false),
                    })
                    .collect()
            })
            .unwrap_or_default();

        let history: Vec<CountryPeriod> = entity
            .get("country_history")
            .and_then(Value::as_array)
            .map(|a| {
                a.iter()
                    .map(|h| CountryPeriod {
                        country: s(h, "country"),
                        from: s(h, "from"),
                        until: s(h, "until"),
                        note: s(h, "note"),
                    })
                    .collect()
            })
            .unwrap_or_default();

        let ids = entity.get("identifiers").cloned().unwrap_or(Value::Null);
        let coords = entity.get("coordinates").cloned().unwrap_or(Value::Null);

        let display = names
            .iter()
            .find(|n| n.is_primary)
            .or_else(|| names.first())
            .map(|n| n.value.clone())
            .unwrap_or_default();

        Self {
            id: s(entity, "id"),
            names,
            place_type: s(entity, "place_type"),
            region: s(entity, "region"),
            country_current: s(entity, "country_current"),
            history,
            wikidata: s(&ids, "wikidata"),
            geonames: s(&ids, "geonames"),
            coordinates: Coordinates {
                lat: num(&coords, "lat"),
                lon: num(&coords, "lon"),
                precision: s(&coords, "precision"),
            },
            note: s(entity, "note"),
            version: entity
                .get("version_num")
                .and_then(Value::as_u64)
                .unwrap_or(0),
            display,
        }
    }

    /// Read the form back out of a POST body.
    ///
    /// Rows are numbered in the field name — `names.0.value`, `names.1.value`
    /// — and the numbering is allowed to be sparse, because the form always
    /// renders spare blank rows and a reader may fill the second and leave the
    /// first alone. Rows with no name are dropped rather than saved as empty.
    pub fn from_post(form: &HashMap<String, String>) -> Self {
        let get = |k: &str| {
            form.get(k)
                .map(|v| v.trim().to_string())
                .unwrap_or_default()
        };

        let primary: Option<usize> = form.get("primary").and_then(|v| v.parse().ok());
        let mut names = Vec::new();
        for i in indices(form, "names") {
            let value = get(&format!("names.{i}.value"));
            if value.is_empty() {
                continue;
            }
            names.push(PlaceName {
                lang: get(&format!("names.{i}.lang")),
                value,
                is_primary: primary == Some(i),
            });
        }
        // Exactly one primary. If the chosen row was left blank, or nothing was
        // chosen, the first surviving name takes it: the schema wants a name
        // and the rest of the application wants to know which one to show.
        if !names.iter().any(|n| n.is_primary) {
            if let Some(first) = names.first_mut() {
                first.is_primary = true;
            }
        }

        let mut history = Vec::new();
        for i in indices(form, "history") {
            let country = get(&format!("history.{i}.country"));
            if country.is_empty() {
                continue;
            }
            history.push(CountryPeriod {
                country,
                from: get(&format!("history.{i}.from")),
                until: get(&format!("history.{i}.until")),
                note: get(&format!("history.{i}.note")),
            });
        }

        let display = names
            .iter()
            .find(|n| n.is_primary)
            .map(|n| n.value.clone())
            .unwrap_or_default();

        Self {
            id: get("id"),
            names,
            place_type: get("place_type"),
            region: get("region"),
            country_current: get("country_current"),
            history,
            wikidata: get("identifiers.wikidata"),
            geonames: get("identifiers.geonames"),
            coordinates: Coordinates {
                lat: get("coordinates.lat"),
                lon: get("coordinates.lon"),
                precision: get("coordinates.precision"),
            },
            note: get("note"),
            version: form
                .get("base_version")
                .and_then(|v| v.trim().parse().ok())
                .unwrap_or(u64::MAX),
            display,
        }
    }

    /// What is wrong with this form, in the reader's terms.
    ///
    /// Returns locale keys rather than sentences: the caller renders them.
    pub fn problems(&self) -> Vec<&'static str> {
        let mut out = Vec::new();
        if self.names.is_empty() {
            out.push("place-error-no-name");
        }
        match (
            self.coordinates.lat.is_empty(),
            self.coordinates.lon.is_empty(),
        ) {
            (true, true) => {}
            (false, false) => {
                let lat = self.coordinates.lat.parse::<f64>();
                let lon = self.coordinates.lon.parse::<f64>();
                match (lat, lon) {
                    (Ok(a), Ok(o)) => {
                        if !(-90.0..=90.0).contains(&a) || !(-180.0..=180.0).contains(&o) {
                            out.push("place-error-coords-range");
                        }
                    }
                    _ => out.push("place-error-coords-number"),
                }
            }
            // One without the other is not half a position, it is no position.
            _ => out.push("place-error-coords-pair"),
        }
        out
    }

    /// Write the form onto the stored entity.
    ///
    /// Starts from what the bundle holds and overwrites only the fields this
    /// form owns, so anything the editor does not model — a field a future
    /// version of the specification adds, an `insee` identifier the converter
    /// wrote — survives being edited by a form that has never heard of it.
    pub fn apply(&self, stored: &Value) -> Value {
        let mut out = stored.clone();
        let obj = match out.as_object_mut() {
            Some(o) => o,
            None => {
                out = json!({});
                out.as_object_mut().expect("just made an object")
            }
        };

        obj.insert("type".into(), json!("place"));
        obj.insert(
            "names".into(),
            Value::Array(
                self.names
                    .iter()
                    .map(|n| {
                        let mut m = Map::new();
                        // `lang` is required by the schema. An unstated
                        // language is recorded as undetermined rather than as
                        // an empty string that fails validation.
                        m.insert(
                            "lang".into(),
                            json!(if n.lang.is_empty() { "und" } else { &n.lang }),
                        );
                        m.insert("value".into(), json!(n.value));
                        m.insert("is_primary".into(), json!(n.is_primary));
                        Value::Object(m)
                    })
                    .collect(),
            ),
        );

        set_or_clear(obj, "place_type", &self.place_type);
        set_or_clear(obj, "region", &self.region);
        set_or_clear(obj, "country_current", &self.country_current);
        set_or_clear(obj, "note", &self.note);

        if self.history.is_empty() {
            obj.remove("country_history");
        } else {
            obj.insert(
                "country_history".into(),
                Value::Array(
                    self.history
                        .iter()
                        .map(|h| {
                            let mut m = Map::new();
                            m.insert("country".into(), json!(h.country));
                            for (k, v) in
                                [("from", &h.from), ("until", &h.until), ("note", &h.note)]
                            {
                                if !v.is_empty() {
                                    m.insert(k.into(), json!(v));
                                }
                            }
                            Value::Object(m)
                        })
                        .collect(),
                ),
            );
        }

        // Identifiers keep whatever else was there — `insee`, say — and gain or
        // lose only the two this form edits.
        let mut ids = stored
            .get("identifiers")
            .and_then(Value::as_object)
            .cloned()
            .unwrap_or_default();
        for (k, v) in [("wikidata", &self.wikidata), ("geonames", &self.geonames)] {
            if v.is_empty() {
                ids.remove(k);
            } else {
                ids.insert(k.into(), json!(v));
            }
        }
        if ids.is_empty() {
            obj.remove("identifiers");
        } else {
            obj.insert("identifiers".into(), Value::Object(ids));
        }

        match (
            self.coordinates.lat.parse::<f64>(),
            self.coordinates.lon.parse::<f64>(),
        ) {
            (Ok(lat), Ok(lon)) => {
                let mut c = Map::new();
                c.insert("lat".into(), json!(lat));
                c.insert("lon".into(), json!(lon));
                if !self.coordinates.precision.is_empty() {
                    c.insert("precision".into(), json!(self.coordinates.precision));
                }
                obj.insert("coordinates".into(), Value::Object(c));
            }
            // Both blank means the reader cleared the position, which is a
            // legitimate edit: a coordinate somebody now believes is wrong is
            // worse than none.
            _ => {
                obj.remove("coordinates");
            }
        }

        out
    }
}

fn set_or_clear(obj: &mut Map<String, Value>, key: &str, value: &str) {
    if value.is_empty() {
        obj.remove(key);
    } else {
        obj.insert(key.into(), json!(value));
    }
}

/// Every row index present in the form under `prefix`, in order.
fn indices(form: &HashMap<String, String>, prefix: &str) -> Vec<usize> {
    let mut out: Vec<usize> = form
        .keys()
        .filter_map(|k| {
            let rest = k.strip_prefix(prefix)?.strip_prefix('.')?;
            rest.split('.').next()?.parse::<usize>().ok()
        })
        .collect();
    out.sort_unstable();
    out.dedup();
    out
}

/// How many records name this place.
///
/// The point of the count is that a place is shared. Locating Słomniki once
/// serves every person born there, and the editor says so before saving
/// because an edit here is not an edit to one record.
pub fn usage_count(flat: &Value, place_id: &str) -> usize {
    let mut n = 0usize;
    let mut walk = |v: &Value| {
        if let Some(id) = v.get("place_id").and_then(Value::as_str) {
            if id == place_id {
                n += 1;
            }
        }
    };
    for collection in ["persons", "families", "events", "occupations"] {
        let Some(items) = flat.get(collection).and_then(Value::as_object) else {
            continue;
        };
        for entity in items.values() {
            walk(entity);
            for key in ["birth", "death"] {
                if let Some(f) = entity.get(key) {
                    walk(f);
                }
            }
            // Residence history and any other array of dated facts.
            for key in ["residences", "history"] {
                for item in entity
                    .get(key)
                    .and_then(Value::as_array)
                    .into_iter()
                    .flatten()
                {
                    walk(item);
                }
            }
        }
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn form(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs
            .iter()
            .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
            .collect()
    }

    #[test]
    fn a_converted_place_reads_as_one_name_and_nothing_else() {
        // What 123 of the operator's 123 places look like.
        let stored = json!({
            "id": "p1", "type": "place", "axgf_version": "1.0", "version_num": 1,
            "names": [{"lang": "pl", "value": "Słomniki, Pologne", "is_primary": true}],
            "region": "Pologne"
        });
        let f = PlaceForm::from_entity(&stored);
        assert_eq!(f.names.len(), 1);
        assert_eq!(f.display, "Słomniki, Pologne");
        assert_eq!(f.region, "Pologne");
        assert!(f.place_type.is_empty() && f.coordinates.lat.is_empty());
        assert!(f.problems().is_empty(), "a name alone is a valid place");
    }

    #[test]
    fn three_empires_three_names_and_a_border_history() {
        // The case the field exists for: a Polish village recorded in Polish,
        // Russian and German depending on who was administering it.
        let stored = json!({"id": "p1", "type": "place", "version_num": 4,
                            "names": [{"lang": "pl", "value": "Old", "is_primary": true}],
                            "identifiers": {"insee": "97411"}});
        let f = PlaceForm::from_post(&form(&[
            ("names.0.lang", "pl"),
            ("names.0.value", "Słomniki"),
            ("names.1.lang", "ru"),
            ("names.1.value", "Сломники"),
            ("names.2.lang", "de"),
            ("names.2.value", "Slomniki"),
            ("primary", "1"),
            ("place_type", "village"),
            ("country_current", "PL"),
            ("history.0.country", "PL"),
            ("history.0.until", "1795"),
            ("history.1.country", "RU"),
            ("history.1.from", "1815"),
            ("base_version", "4"),
        ]));
        let out = f.apply(&stored);

        let names = out["names"].as_array().unwrap();
        assert_eq!(names.len(), 3);
        assert_eq!(names[1]["value"], "Сломники");
        assert_eq!(
            names[1]["is_primary"],
            json!(true),
            "the chosen row is primary"
        );
        assert_eq!(names[0]["is_primary"], json!(false), "and only that one");

        let hist = out["country_history"].as_array().unwrap();
        assert_eq!(hist.len(), 2);
        assert_eq!(hist[0]["country"], "PL");
        assert!(
            hist[0].get("from").is_none(),
            "an unstated bound is absent, not empty"
        );

        // A field the form has never heard of survives being edited by it.
        assert_eq!(out["identifiers"]["insee"], "97411");
        assert_eq!(
            f.version, 4,
            "the version check gets the version it rendered from"
        );
    }

    #[test]
    fn a_blank_row_is_dropped_and_a_language_defaults_to_undetermined() {
        // The form always renders spare blank rows so it works without
        // scripting; they must not become empty names.
        let f = PlaceForm::from_post(&form(&[
            ("names.0.value", "Lyon"),
            ("names.1.lang", "fr"),
            ("names.1.value", ""),
            ("names.2.value", ""),
        ]));
        assert_eq!(f.names.len(), 1);
        let out = f.apply(&json!({}));
        assert_eq!(
            out["names"][0]["lang"], "und",
            "the schema wants a language"
        );
        assert_eq!(
            out["names"][0]["is_primary"],
            json!(true),
            "something must be primary"
        );
    }

    #[test]
    fn half_a_position_is_no_position() {
        let one = PlaceForm::from_post(&form(&[
            ("names.0.value", "X"),
            ("coordinates.lat", "50.2"),
        ]));
        assert_eq!(one.problems(), vec!["place-error-coords-pair"]);

        let silly = PlaceForm::from_post(&form(&[
            ("names.0.value", "X"),
            ("coordinates.lat", "500"),
            ("coordinates.lon", "20"),
        ]));
        assert_eq!(silly.problems(), vec!["place-error-coords-range"]);

        let words = PlaceForm::from_post(&form(&[
            ("names.0.value", "X"),
            ("coordinates.lat", "north"),
            ("coordinates.lon", "20"),
        ]));
        assert_eq!(words.problems(), vec!["place-error-coords-number"]);
    }

    #[test]
    fn clearing_both_coordinates_removes_the_position() {
        // A coordinate somebody now believes is wrong is worse than none, so
        // emptying both fields is a legitimate edit rather than a no-op.
        let stored = json!({"names": [{"lang":"pl","value":"X","is_primary":true}],
                            "coordinates": {"lat": 1.0, "lon": 2.0}});
        let f = PlaceForm::from_post(&form(&[("names.0.value", "X")]));
        let out = f.apply(&stored);
        assert!(out.get("coordinates").is_none());
    }

    #[test]
    fn a_place_is_counted_wherever_it_is_named() {
        let flat = json!({
            "persons": {
                "a": {"birth": {"place_id": "pl1"}, "death": {"place_id": "pl2"}},
                "b": {"birth": {"place_id": "pl1"}}
            },
            "events": {"e": {"place_id": "pl1"}},
            "occupations": {"o": {"place_id": "pl9"}}
        });
        assert_eq!(usage_count(&flat, "pl1"), 3);
        assert_eq!(usage_count(&flat, "pl2"), 1);
        assert_eq!(usage_count(&flat, "nobody"), 0);
    }
}
