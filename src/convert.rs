//! GEDCOM → AXGF conversion.
//!
//! This page is a standalone utility. It never touches the served bundle:
//! replacing the live database is a separate, explicit admin action, so a
//! visitor experimenting with their own GEDCOM cannot overwrite what the site
//! is showing.
//!
//! Conversion in the other direction is deliberately absent. The point of the
//! page is to show what AXGF records that GEDCOM discards; offering a lossy
//! round trip back would undercut that.

use std::sync::Mutex;
use std::time::{Duration, Instant};

use serde_json::Value;

/// How long a converted bundle stays available for download.
const TTL: Duration = Duration::from_secs(15 * 60);

/// How many conversions are held at once. Small: the download follows the
/// conversion immediately, and this is a demonstration utility, not storage.
const CAPACITY: usize = 8;

/// Upload ceiling. A 767-person GEDCOM is about 320 KB, so 10 MB is generous
/// while still bounding what one request can allocate.
pub const MAX_UPLOAD: usize = 10 * 1024 * 1024;

struct Entry {
    id: String,
    filename: String,
    bytes: Vec<u8>,
    created: Instant,
}

/// Short-lived store of converted bundles, so the results page can show what
/// the conversion produced *before* the download link, and the link still
/// works when clicked.
#[derive(Default)]
pub struct ConversionCache {
    entries: Mutex<Vec<Entry>>,
}

impl ConversionCache {
    /// Store converted bytes and return the download id.
    pub fn put(&self, filename: String, bytes: Vec<u8>) -> String {
        let id = uuid::Uuid::new_v4().simple().to_string();
        let mut guard = self.entries.lock().unwrap_or_else(|e| e.into_inner());
        guard.retain(|e| e.created.elapsed() < TTL);
        while guard.len() >= CAPACITY {
            guard.remove(0);
        }
        guard.push(Entry {
            id: id.clone(),
            filename,
            bytes,
            created: Instant::now(),
        });
        id
    }

    /// Fetch converted bytes by id, if they have not expired.
    pub fn get(&self, id: &str) -> Option<(String, Vec<u8>)> {
        let mut guard = self.entries.lock().unwrap_or_else(|e| e.into_inner());
        guard.retain(|e| e.created.elapsed() < TTL);
        guard
            .iter()
            .find(|e| e.id == id)
            .map(|e| (e.filename.clone(), e.bytes.clone()))
    }
}

/// Per-collection counts of what a conversion produced.
pub fn counts_of(bundle: &Value) -> Vec<(&'static str, usize)> {
    crate::state::COLLECTIONS
        .iter()
        .map(|&name| {
            let n = bundle
                .get(name)
                .and_then(Value::as_object)
                .map(|m| m.len())
                .unwrap_or(0);
            (name, n)
        })
        .collect()
}

/// Turn an uploaded filename into the matching `.axgf` name.
pub fn axgf_name_for(uploaded: &str) -> String {
    let stem = uploaded
        .rsplit('/')
        .next()
        .unwrap_or(uploaded)
        .trim_end_matches(".ged")
        .trim_end_matches(".GED")
        .trim();
    let stem = if stem.is_empty() { "converted" } else { stem };
    format!("{stem}.axgf")
}

/// A quick shape check so obviously wrong uploads get a clear message rather
/// than a wall of parser diagnostics.
///
/// This is not a parser — `axgf-rs` owns that. It only looks for the `0 HEAD`
/// line every GEDCOM 5.5.1 file starts with, tolerating a UTF-8 BOM and
/// leading blank lines.
pub fn looks_like_gedcom(bytes: &[u8]) -> bool {
    let head = &bytes[..bytes.len().min(4096)];
    let text = String::from_utf8_lossy(head);
    let text = text.trim_start_matches('\u{feff}');
    text.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .take(5)
        .any(|l| {
            let mut parts = l.split_whitespace();
            parts.next() == Some("0")
                && parts.next().is_some_and(|t| t.eq_ignore_ascii_case("HEAD"))
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_real_gedcom_header_is_recognised() {
        assert!(looks_like_gedcom(b"0 HEAD\n1 SOUR test\n"));
        // A BOM and leading blank lines are common and harmless.
        assert!(looks_like_gedcom("\u{feff}\n\n0 HEAD\n".as_bytes()));
        assert!(looks_like_gedcom(b"0 HEAD\r\n1 CHAR UTF-8\r\n"));
    }

    #[test]
    fn other_formats_are_rejected_before_the_parser_sees_them() {
        assert!(!looks_like_gedcom(b"{\"json\": true}"));
        assert!(!looks_like_gedcom(b"PK\x03\x04binary zip"));
        assert!(!looks_like_gedcom(b""));
        assert!(!looks_like_gedcom(b"<?xml version=\"1.0\"?>"));
    }

    #[test]
    fn download_name_follows_the_upload() {
        assert_eq!(axgf_name_for("family.ged"), "family.axgf");
        assert_eq!(axgf_name_for("FAMILY.GED"), "FAMILY.axgf");
        assert_eq!(axgf_name_for("tree"), "tree.axgf");
        // A path-ish name is reduced to its last segment.
        assert_eq!(axgf_name_for("/tmp/x/tree.ged"), "tree.axgf");
        assert_eq!(axgf_name_for(""), "converted.axgf");
    }

    #[test]
    fn cache_round_trips_and_expires_by_capacity() {
        let c = ConversionCache::default();
        let id = c.put("a.axgf".into(), vec![1, 2, 3]);
        let (name, bytes) = c.get(&id).expect("still cached");
        assert_eq!(name, "a.axgf");
        assert_eq!(bytes, vec![1, 2, 3]);

        // Filling past capacity evicts the oldest.
        for i in 0..CAPACITY {
            c.put(format!("{i}.axgf"), vec![i as u8]);
        }
        assert!(c.get(&id).is_none(), "the oldest entry should be gone");
    }

    #[test]
    fn unknown_download_id_is_simply_absent() {
        let c = ConversionCache::default();
        assert!(c.get("nope").is_none());
    }

    #[test]
    fn counts_cover_every_collection() {
        let b = serde_json::json!({"persons": {"a": {}, "b": {}}, "families": {}});
        let counts = counts_of(&b);
        assert_eq!(counts.len(), crate::state::COLLECTIONS.len());
        assert_eq!(counts.iter().find(|(k, _)| *k == "persons").unwrap().1, 2);
        assert_eq!(counts.iter().find(|(k, _)| *k == "links").unwrap().1, 0);
    }
}
