//! Turning a structured place into a position, through Nominatim.
//!
//! # The usage policy is binding
//!
//! Nominatim runs on donated infrastructure and is free. Its policy caps
//! automated use at one request per second, requires a User-Agent that
//! identifies the application and how to reach whoever runs it, and forbids
//! bulk geocoding. Instances are blocked for abuse regularly, and the block
//! lands on everyone behind the address.
//!
//! Four things here follow from that, and none of them are configurable in the
//! direction of doing more:
//!
//! * **One request per second, enforced here.** The gate is a single mutex
//!   inside one [`Geocoder`], which is held by the application state, so the
//!   limit is shared by every reader of the instance rather than being per
//!   session. Two administrators searching at once queue behind each other.
//! * **A User-Agent naming the product, its version and a contact.** Without a
//!   contact there is no geocoder at all — [`Geocoder::new`] returns `None` —
//!   because an anonymous automated caller is exactly what the policy asks
//!   operators not to be. The manual path still works, so the feature degrades
//!   rather than disappearing.
//! * **Through the server.** A browser calling Nominatim directly would put
//!   every visitor's address into someone else's logs and would put the rate
//!   limit somewhere it cannot be enforced.
//! * **No bulk button.** 123 places at one a second is two minutes of
//!   hammering a donated service to produce 123 unreviewed guesses. One place
//!   at a time with a human confirming the match is both compliant and better
//!   genealogy: see the module tests for what the service actually returns for
//!   a nineteenth-century Polish village.

use std::sync::Arc;
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use ureq::Agent;

/// The public service, used when an operator names no other.
pub const DEFAULT_ENDPOINT: &str = "https://nominatim.openstreetmap.org/search";

/// The floor between two outbound requests.
const MIN_INTERVAL: Duration = Duration::from_secs(1);

/// How long to wait on the service before giving up and offering the manual
/// path. Nominatim is usually fast; a reader looking at a spinner is not.
const TIMEOUT: Duration = Duration::from_secs(10);

/// One candidate position.
///
/// The field names are `jsonv2`'s, which are not `json`'s. In particular the
/// OSM key category arrives as **`category`**; the v1 format calls it `class`.
/// Reading it under the wrong name is quiet rather than loud — the field is
/// simply always empty — and it takes [`Candidate::precision`] with it, so
/// every result, a bus stop included, would be recorded as a city centre.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candidate {
    /// The full name Nominatim gives, which is what lets an administrator tell
    /// one Springfield from another.
    pub display_name: String,
    /// The short name, for the button.
    #[serde(default)]
    pub name: String,
    pub lat: String,
    pub lon: String,
    /// `village`, `town`, `administrative`, `bus_stop`… Used to suggest a
    /// precision rather than to assert one.
    #[serde(default, rename = "type")]
    pub kind: String,
    /// The OSM key: `place`, `boundary`, `highway`, `amenity`, `railway`…
    /// This is what separates a village from a bus stop named after one.
    #[serde(default)]
    pub category: String,
    /// Nominatim's own summary of what the object is addressed as — `town`,
    /// `village`, `road`. Present more often than a useful `type`.
    #[serde(default)]
    pub addresstype: String,
    /// 0–30, coarse to fine. 16 is a town, 30 is a building.
    #[serde(default)]
    pub place_rank: u16,
    /// Present when the query asked for it; the country is the field that most
    /// often separates two identical names.
    #[serde(default)]
    pub address: Address,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Address {
    #[serde(default)]
    pub country: String,
    #[serde(default)]
    pub country_code: String,
}

impl Candidate {
    /// The precision this result honestly supports.
    ///
    /// A geocoder that returns the administrative centre of a commune has not
    /// found a house, and recording `exact` for it would be a claim nobody
    /// made. Mapped from what Nominatim says the object is, preferring
    /// `place_rank` — which is a number on one scale — over the category
    /// vocabulary, which is large and open-ended.
    pub fn precision(&self) -> &'static str {
        match self.category.as_str() {
            "building" => return "building",
            "highway" => return "street",
            _ => {}
        }
        match self.kind.as_str() {
            "house" | "building" => return "building",
            "country" => return "country_center",
            "state" | "region" | "county" | "province" => return "region_center",
            "road" | "street" => return "street",
            _ => {}
        }
        // Nominatim's own scale, when the vocabulary did not settle it.
        match self.place_rank {
            0..=3 => "approximate",
            4..=5 => "country_center",
            6..=12 => "region_center",
            13..=19 => "city_center",
            20..=27 => "street",
            _ => "building",
        }
    }

    /// Whether this result is the kind of thing a birth record means by a
    /// place: a settlement, an administrative area, or a named locality.
    ///
    /// Not a filter. It drives a warning, because the failure this catches is
    /// the one that actually happened when the operator's places were put
    /// through the service: "Bałtów Magonie" returned a **bus stop** called
    /// Magonie, and "Litwa Kowieńska" — a historical region — returned
    /// *Kauno gatvė*, a street in Vilnius. Both are confident top results,
    /// both sit at the head of the list, and both are wrong in a way that a
    /// display name alone does not make obvious. A miss sends the reader to
    /// the manual fields; a false hit invites a click.
    pub fn is_settlement(&self) -> bool {
        matches!(self.category.as_str(), "place" | "boundary")
            && !matches!(self.kind.as_str(), "house" | "houses")
    }
}

/// A rate-limited, identified client for one Nominatim instance.
///
/// The agent is built once and kept. It holds a connection pool, so a second
/// lookup reuses the first one's TLS session instead of shaking hands again —
/// which matters more here than it usually would, because the whole point of
/// this module is to be light on someone else's donated hardware.
pub struct Geocoder {
    endpoint: String,
    user_agent: String,
    agent: Agent,
    /// When the last request went out. One gate for the whole process, which
    /// is what makes the limit a property of the instance rather than of a
    /// session.
    gate: Arc<Mutex<Option<Instant>>>,
}

// `ureq::Agent` is `Clone` but not `Debug`, and `Geocoder` sits inside the
// application state, which is. Written out rather than derived so the state
// stays printable, and so a log line shows the identity this instance sends
// rather than the innards of a connection pool.
impl std::fmt::Debug for Geocoder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Geocoder")
            .field("endpoint", &self.endpoint)
            .field("user_agent", &self.user_agent)
            .finish_non_exhaustive()
    }
}

impl Geocoder {
    /// Build a geocoder, or `None` when the operator has given no contact.
    ///
    /// The contact is not optional politeness. Nominatim's policy asks for a
    /// User-Agent that identifies the application *and how to reach whoever is
    /// running it*, so that a misbehaving deployment can be told rather than
    /// blocked. An installation that will not say who it is does not get to
    /// make automated calls from this application; it gets the manual path,
    /// which needs no third party at all.
    pub fn new(endpoint: Option<&str>, contact: Option<&str>) -> Option<Self> {
        let contact = contact.map(str::trim).filter(|c| !c.is_empty())?;
        let user_agent = format!(
            "{}/{} ({contact})",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        );
        let agent: Agent = Agent::new_with_config(
            Agent::config_builder()
                // One budget for the whole exchange — connect, TLS, request,
                // response. A per-stage timeout can still add up to a reader
                // waiting far longer than any single number suggests.
                .timeout_global(Some(TIMEOUT))
                .user_agent(user_agent.as_str())
                .build(),
        );
        Some(Self {
            endpoint: endpoint
                .map(str::trim)
                .filter(|e| !e.is_empty())
                .unwrap_or(DEFAULT_ENDPOINT)
                .to_string(),
            user_agent,
            agent,
            gate: Arc::new(Mutex::new(None)),
        })
    }

    /// What this geocoder will send as its identity, for the admin screen.
    pub fn user_agent(&self) -> &str {
        &self.user_agent
    }

    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    /// Look a place up.
    ///
    /// Holds the shared gate for the whole request rather than only for the
    /// wait, so two callers cannot both observe "a second has passed" and then
    /// both fire. That makes concurrent searches serialise, which is the
    /// intended behaviour: the limit is one request per second for the
    /// instance, not one per reader.
    pub async fn search(&self, query: &str) -> Result<Vec<Candidate>, GeocodeError> {
        let query = query.trim();
        if query.is_empty() {
            return Ok(Vec::new());
        }

        let mut gate = self.gate.lock().await;
        if let Some(last) = *gate {
            let since = last.elapsed();
            if since < MIN_INTERVAL {
                tokio::time::sleep(MIN_INTERVAL - since).await;
            }
        }

        let url = format!(
            "{}?q={}&format=jsonv2&addressdetails=1&limit=8",
            self.endpoint,
            urlencode(query)
        );
        let agent = self.agent.clone();

        // `ureq` is blocking, and this is a rare call behind a one-second gate,
        // so it goes to a blocking thread rather than pulling an async HTTP
        // stack into a binary that otherwise makes no outbound calls at all.
        // Cloning the agent shares the pool rather than copying it.
        let body = tokio::task::spawn_blocking(move || -> Result<String, String> {
            agent
                .get(&url)
                .call()
                .map_err(|e| e.to_string())?
                .body_mut()
                .read_to_string()
                .map_err(|e| e.to_string())
        })
        .await;

        *gate = Some(Instant::now());
        drop(gate);

        let body = match body {
            Ok(Ok(b)) => b,
            Ok(Err(e)) => return Err(GeocodeError::Upstream(e)),
            Err(e) => return Err(GeocodeError::Upstream(e.to_string())),
        };

        serde_json::from_str::<Vec<Candidate>>(&body)
            .map_err(|e| GeocodeError::Malformed(e.to_string()))
    }
}

/// Why a lookup produced nothing.
///
/// Both variants end at the same place in the interface — the manual path —
/// because from the reader's side "the service is down" and "the service has
/// never heard of your village" need the same next action.
#[derive(Debug)]
pub enum GeocodeError {
    Upstream(String),
    Malformed(String),
}

impl std::fmt::Display for GeocodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Upstream(e) => write!(f, "{e}"),
            Self::Malformed(e) => write!(f, "{e}"),
        }
    }
}

/// Percent-encode a query string.
///
/// Hand-rolled because the alternative is a crate, and this is one rule:
/// everything that is not unreserved gets escaped. Non-ASCII is escaped byte
/// by byte, which is what UTF-8 in a query string requires — the operator's
/// place names are full of Polish diacritics and Cyrillic.
fn urlencode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.as_bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(*b as char)
            }
            b' ' => out.push('+'),
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

/// The query to send for a place, built from its structured fields.
///
/// This is why the structured editor comes first. "Słomniki, Pologne" is what
/// the converter left behind and is a poor query: the country is in French,
/// the string is one blob, and Nominatim has to guess what part of it is a
/// name. Name, region and country as separate terms is what the service is
/// built to take.
///
/// The primary name alone is the base, with the region and the country added
/// when the record has them and they are not already inside the name — the
/// converted names very often end in ", Pologne" already, and repeating it
/// makes the query worse rather than better.
pub fn query_for(name: &str, region: Option<&str>, country: Option<&str>) -> String {
    let mut parts: Vec<String> = vec![name.trim().to_string()];
    for extra in [region, country].into_iter().flatten() {
        let extra = extra.trim();
        if extra.is_empty() {
            continue;
        }
        // Recomputed each time rather than closed over, because the haystack
        // has to include the terms added on earlier turns of this loop: a
        // record whose region and country are both "Pologne" should still
        // produce it once.
        let hay = parts.join(" ").to_lowercase();
        if hay.contains(&extra.to_lowercase()) {
            continue;
        }
        parts.push(extra.to_string());
    }
    parts.join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_query_is_built_from_the_structured_fields() {
        assert_eq!(
            query_for("Słomniki", Some("Lesser Poland"), Some("PL")),
            "Słomniki, Lesser Poland, PL"
        );
    }

    #[test]
    fn a_term_already_inside_the_name_is_not_repeated() {
        // What the converter leaves behind: the country is already in the
        // name, and "Słomniki, Pologne, Pologne" is a worse query than either.
        assert_eq!(
            query_for("Słomniki, Pologne", None, Some("Pologne")),
            "Słomniki, Pologne"
        );
        assert_eq!(query_for("France", None, Some("France")), "France");
    }

    #[test]
    fn no_contact_means_no_geocoder() {
        // Nominatim's policy asks for a User-Agent that says who is running
        // the application. An installation that will not say does not make
        // automated calls from here.
        assert!(Geocoder::new(Some(DEFAULT_ENDPOINT), None).is_none());
        assert!(Geocoder::new(Some(DEFAULT_ENDPOINT), Some("   ")).is_none());
        let g = Geocoder::new(None, Some("ops@example.org")).expect("a contact is enough");
        assert_eq!(g.endpoint(), DEFAULT_ENDPOINT);
        assert!(g.user_agent().contains("axgf-cms"), "{}", g.user_agent());
        assert!(g.user_agent().contains("ops@example.org"));
    }

    #[test]
    fn the_endpoint_is_configurable_for_a_self_hosted_instance() {
        let g = Geocoder::new(Some("http://nominatim.internal/search"), Some("ops@x")).unwrap();
        assert_eq!(g.endpoint(), "http://nominatim.internal/search");
    }

    #[test]
    fn a_query_is_encoded_bytewise_so_diacritics_survive() {
        assert_eq!(urlencode("Słomniki"), "S%C5%82omniki");
        assert_eq!(urlencode("a b"), "a+b");
        assert_eq!(urlencode("Сломники").matches('%').count(), 16);
    }

    /// Built from a real `jsonv2` response, field for field.
    fn karczew() -> Candidate {
        // Captured from nominatim.openstreetmap.org, 2026-09-03.
        serde_json::from_str(
            r#"{"place_id":177395908,"osm_type":"relation","osm_id":336208,
                "lat":"52.0782795","lon":"21.2508068","category":"boundary",
                "type":"administrative","place_rank":16,"addresstype":"town",
                "name":"Karczew",
                "display_name":"Karczew, gmina Karczew, powiat otwocki, województwo mazowieckie, Polska",
                "address":{"town":"Karczew","country":"Polska","country_code":"pl"}}"#,
        )
        .expect("a real response parses")
    }

    #[test]
    fn the_jsonv2_category_key_is_read_under_its_own_name() {
        // `jsonv2` says `category`; only the v1 format says `class`. Reading
        // the wrong one leaves the field empty and every result silently
        // becomes a city centre.
        let c = karczew();
        assert_eq!(c.category, "boundary");
        assert_eq!(c.addresstype, "town");
        assert_eq!(c.place_rank, 16);
        assert_eq!(c.address.country_code, "pl");
        assert_eq!(c.precision(), "city_center");
        assert!(c.is_settlement());
    }

    #[test]
    fn precision_is_taken_from_what_the_result_actually_is() {
        let mut c = karczew();
        assert_eq!(c.precision(), "city_center");
        c.kind = "house".into();
        assert_eq!(c.precision(), "building");
        c.kind = "country".into();
        assert_eq!(c.precision(), "country_center");
        // Nothing in the vocabulary, so the rank decides.
        c.kind = "locality".into();
        c.category = "place".into();
        c.place_rank = 30;
        assert_eq!(c.precision(), "building");
    }

    #[test]
    fn the_false_hits_the_operators_places_actually_produce_are_flagged() {
        // Both of these are real top results for real names in the bundle,
        // and both are wrong. The list has to be able to say so.
        let bus_stop = Candidate {
            display_name: "Magonie/01/690, Sudół, gmina Bodzechów, Polska".into(),
            name: "Magonie/01".into(),
            lat: "50.9".into(),
            lon: "21.4".into(),
            kind: "bus_stop".into(),
            category: "highway".into(),
            addresstype: "bus_stop".into(),
            place_rank: 30,
            address: Address::default(),
        };
        // Searched for "Bałtów Magonie", a hamlet.
        assert!(!bus_stop.is_settlement());
        assert_eq!(bus_stop.precision(), "street");

        let street = Candidate {
            display_name: "Kauno g., Naujamiestis, Vilnius, Lietuva".into(),
            name: "Kauno g.".into(),
            lat: "54.6".into(),
            lon: "25.2".into(),
            kind: "primary".into(),
            category: "highway".into(),
            addresstype: "road".into(),
            place_rank: 26,
            address: Address::default(),
        };
        // Searched for "Litwa Kowieńska", a historical region.
        assert!(!street.is_settlement());
        assert_eq!(street.precision(), "street");
    }
}
