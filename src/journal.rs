//! The edit journal: who changed what, when, and to what.
//!
//! # Beside the bundle, not inside it
//!
//! For the same reason the accounts are: a `.axgf` is copied, mailed and
//! published, and the journal names people and records every value they
//! corrected. "Grandmother's birth year was wrong for six months and Anna
//! fixed it" is a fact about the family's *editors*, not about the family, and
//! it should not travel with a file meant to be shared.
//!
//! # JSON Lines, appended, never rewritten
//!
//! One object per line. Appending is a single `write` on an `O_APPEND` handle,
//! so a crash can lose the last line but cannot corrupt the ones before it —
//! which is the property a re-serialised JSON array would not have. It also
//! means `tail`, `grep` and `wc -l` work on it, which matters for a file whose
//! whole job is to be inspectable when something has gone wrong.
//!
//! Nothing prunes it. At a family's edit rate — a few hundred changes a year —
//! it stays smaller than one photograph for a lifetime, and a journal that
//! silently drops history is not one.

use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::diff::Change;

/// One recorded mutation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    /// RFC 3339, UTC.
    pub at: String,
    /// The username, or `emergency-token`. Never an id: the journal has to
    /// stay readable after an account is deleted.
    pub who: String,
    /// `create`, `update`, `delete` or `upload`.
    pub action: String,
    /// `person`, `family`, …
    pub kind: String,
    pub entity_id: String,
    /// A display name for the entity at the time, so a listing does not have
    /// to resolve ids against a bundle that may since have lost them.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// The version this edit produced.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_num: Option<u64>,
    #[serde(default)]
    pub changes: Vec<Change>,
}

impl Entry {
    /// A one-line summary for a listing.
    pub fn summary(&self) -> String {
        match self.action.as_str() {
            "create" => "created".to_string(),
            "delete" => "deleted".to_string(),
            "upload" => "attached a file".to_string(),
            _ => crate::diff::summarise(&self.changes),
        }
    }
}

/// The append-only journal file.
pub struct Journal {
    path: PathBuf,
}

impl Journal {
    /// The journal beside `bundle`: `family.axgf` → `family.journal`.
    pub fn path_for(bundle: &Path) -> PathBuf {
        bundle.with_extension("journal")
    }

    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Append one entry.
    ///
    /// Mode 600 like the ACL: the journal names people and quotes the values
    /// they changed, which is not less sensitive than the accounts are.
    pub fn append(&self, entry: &Entry) -> Result<()> {
        let line = serde_json::to_string(entry).context("serialising a journal entry")?;
        let mut opts = OpenOptions::new();
        opts.create(true).append(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt as _;
            opts.mode(0o600);
        }
        let mut f = opts
            .open(&self.path)
            .with_context(|| format!("opening {}", self.path.display()))?;
        // One write, one line. A partially written line is the failure mode
        // this cannot fully prevent; putting the newline in the same buffer
        // makes it as close to atomic as a file append gets.
        f.write_all(format!("{line}\n").as_bytes())
            .with_context(|| format!("appending to {}", self.path.display()))?;
        Ok(())
    }

    /// Read the whole journal, oldest first.
    ///
    /// A line that does not parse is skipped rather than failing the read: a
    /// truncated last line from a crash must not make the history page
    /// unreachable, which is exactly when somebody needs it.
    pub fn read_all(&self) -> Vec<Entry> {
        let Ok(f) = std::fs::File::open(&self.path) else {
            return Vec::new();
        };
        let mut out = Vec::new();
        let mut skipped = 0usize;
        for line in BufReader::new(f).lines().map_while(Result::ok) {
            if line.trim().is_empty() {
                continue;
            }
            match serde_json::from_str::<Entry>(&line) {
                Ok(e) => out.push(e),
                Err(_) => skipped += 1,
            }
        }
        if skipped > 0 {
            tracing::warn!(
                journal = %self.path.display(),
                skipped,
                "skipped unparseable journal lines"
            );
        }
        out
    }

    /// The most recent `n` entries, newest first.
    pub fn recent(&self, n: usize) -> Vec<Entry> {
        let mut all = self.read_all();
        all.reverse();
        all.truncate(n);
        all
    }

    /// Every entry for one entity, newest first.
    pub fn for_entity(&self, kind: &str, id: &str) -> Vec<Entry> {
        let mut all: Vec<Entry> = self
            .read_all()
            .into_iter()
            .filter(|e| e.kind == kind && e.entity_id == id)
            .collect();
        all.reverse();
        all
    }

    /// Who last changed this entity, and when.
    ///
    /// What the conflict page needs in order to name a person rather than
    /// saying "somebody". `None` when the journal has no record of it — the
    /// edit predates journalling, or another tool wrote the bundle — and the
    /// caller falls back to the entity's own `updated_at`.
    pub fn last_touched(&self, kind: &str, id: &str) -> Option<Entry> {
        self.for_entity(kind, id).into_iter().next()
    }

    /// How many entries the journal holds.
    pub fn len(&self) -> usize {
        self.read_all().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Reconstruct the version of an entity that an editor started from.
///
/// The bundle holds one version of each entity — the current one — so after a
/// conflict the version the losing editor opened is simply gone. But every
/// change since then is in the journal as a `from`/`to` pair, so replaying
/// those backwards from the current document reproduces it exactly.
///
/// That is what turns the conflict screen from "here is theirs and here is
/// yours" into "here is what you started from, here is what they made of it,
/// and here is what you made of it" — which is the difference between guessing
/// who overwrote what and being shown it.
///
/// `None` when the journal cannot account for every version between `to` and
/// the present: an edit made before journalling, by another tool, or lost to a
/// torn write. Reconstructing from an incomplete history would produce a
/// confident and wrong answer, and the caller falls back to a two-way
/// comparison instead.
pub fn rewind(
    current: &Value,
    entries: &[Entry],
    to_version: u64,
    from_version: u64,
) -> Option<Value> {
    if to_version >= from_version {
        return Some(current.clone());
    }
    let mut doc = current.clone();
    // Newest first: entries arrive that way from `for_entity`.
    let mut expected = from_version;
    for e in entries {
        let Some(v) = e.version_num else { continue };
        if v <= to_version {
            break;
        }
        if v != expected {
            // A gap. Anything reconstructed past it would be a guess.
            return None;
        }
        for c in &e.changes {
            let _ = set_path(&mut doc, &c.path, c.from.as_deref());
        }
        expected = v - 1;
    }
    (expected == to_version).then_some(doc)
}

/// Set a dotted path to a string value, or remove it when `value` is `None`.
///
/// Only ever used to undo a change this application itself recorded, so the
/// path is one [`crate::diff`] produced and the shape it names already exists.
fn set_path(doc: &mut Value, path: &str, value: Option<&str>) -> Option<()> {
    let segments: Vec<&str> = path.split('.').collect();
    let (last, parents) = segments.split_last()?;
    let mut cur = doc;
    for seg in parents {
        // A numeric segment addresses an array, anything else an object.
        cur = match seg.parse::<usize>() {
            Ok(i) => {
                if !cur.is_array() {
                    *cur = Value::Array(Vec::new());
                }
                let arr = cur.as_array_mut()?;
                while arr.len() <= i {
                    arr.push(Value::Null);
                }
                &mut arr[i]
            }
            Err(_) => {
                if !cur.is_object() {
                    *cur = Value::Object(serde_json::Map::new());
                }
                cur.as_object_mut()?.entry(*seg).or_insert(Value::Null)
            }
        };
    }
    match (last.parse::<usize>(), value) {
        (Ok(i), Some(v)) => {
            if let Some(arr) = cur.as_array_mut() {
                while arr.len() <= i {
                    arr.push(Value::Null);
                }
                arr[i] = restore(v);
            }
        }
        (Ok(i), None) => {
            if let Some(arr) = cur.as_array_mut() {
                if i < arr.len() {
                    arr.remove(i);
                }
            }
        }
        (Err(_), Some(v)) => {
            if let Some(obj) = cur.as_object_mut() {
                obj.insert((*last).to_string(), restore(v));
            }
        }
        (Err(_), None) => {
            if let Some(obj) = cur.as_object_mut() {
                obj.remove(*last);
            }
        }
    }
    Some(())
}

/// Restore a rendered scalar to the JSON shape it had.
///
/// The diff rendered every scalar as a string. Coming back, a reverted boolean
/// must not return as `"true"`, or the next diff would report it as a change
/// nobody made.
fn restore(v: &str) -> Value {
    // The diff rendered scalars as strings. Restore the JSON shape where
    // it is unambiguous, so a reverted boolean does not come back as the
    // string "true" and then read as a change on the next diff.
    match v {
        "true" => Value::Bool(true),
        "false" => Value::Bool(false),
        _ => match v.parse::<i64>() {
            Ok(n) => Value::from(n),
            Err(_) => match v.parse::<f64>() {
                Ok(n) if v.contains('.') => Value::from(n),
                _ => Value::String(v.to_string()),
            },
        },
    }
}

/// A mutation that has just been applied, ready to be journalled.
///
/// A struct rather than eight positional arguments: at that width the call
/// sites stop being readable, and two `Option<&Value>` in a row are exactly
/// the pair somebody eventually passes the wrong way round.
pub struct Record<'a> {
    /// The username, or `emergency-token`.
    pub who: &'a str,
    /// `create`, `update`, `delete` or `upload`.
    pub action: &'a str,
    pub kind: &'a str,
    pub entity_id: &'a str,
    pub label: Option<String>,
    pub version_num: Option<u64>,
    /// The entity before and after. Both present produces the field-by-field
    /// diff; anything else records the action without one, which is right for
    /// a create or a delete.
    pub before: Option<&'a Value>,
    pub after: Option<&'a Value>,
}

/// Build an entry for a mutation that has already been applied.
pub fn entry_for(r: Record<'_>) -> Entry {
    let changes = match (r.before, r.after) {
        (Some(b), Some(a)) => crate::diff::diff(b, a),
        _ => Vec::new(),
    };
    Entry {
        at: crate::view::now_iso8601(),
        who: r.who.to_string(),
        action: r.action.to_string(),
        kind: r.kind.to_string(),
        entity_id: r.entity_id.to_string(),
        label: r.label,
        version_num: r.version_num,
        changes,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn scratch(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "axgf-journal-{}-{}-{}",
            std::process::id(),
            tag,
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        dir.join("family.journal")
    }

    fn entry(who: &str, id: &str) -> Entry {
        entry_for(Record {
            who,
            action: "update",
            kind: "person",
            entity_id: id,
            label: Some("Laura Karin".into()),
            version_num: Some(2),
            before: Some(&json!({"note": "old"})),
            after: Some(&json!({"note": "new"})),
        })
    }

    #[test]
    fn the_journal_sits_beside_the_bundle_and_not_inside_it() {
        assert_eq!(
            Journal::path_for(Path::new("/srv/family.axgf")),
            PathBuf::from("/srv/family.journal")
        );
    }

    #[test]
    fn entries_round_trip_in_the_order_they_were_written() {
        let j = Journal::new(scratch("order"));
        for i in 0..5 {
            j.append(&entry("ada", &format!("p{i}"))).unwrap();
        }
        let all = j.read_all();
        assert_eq!(all.len(), 5);
        assert_eq!(all[0].entity_id, "p0", "oldest first");
        assert_eq!(j.recent(2)[0].entity_id, "p4", "newest first");
        assert_eq!(j.recent(2).len(), 2);
    }

    #[test]
    fn an_entry_records_who_what_and_the_field_that_changed() {
        let j = Journal::new(scratch("fields"));
        j.append(&entry("karin", "p1")).unwrap();
        let e = &j.read_all()[0];
        assert_eq!(e.who, "karin");
        assert_eq!(e.kind, "person");
        assert_eq!(e.entity_id, "p1");
        assert_eq!(e.version_num, Some(2));
        assert_eq!(e.changes.len(), 1);
        assert_eq!(e.changes[0].path, "note");
        assert_eq!(e.changes[0].from.as_deref(), Some("old"));
        assert_eq!(e.changes[0].to.as_deref(), Some("new"));
        assert_eq!(e.summary(), "changed note");
        assert!(!e.at.is_empty());
    }

    #[test]
    fn a_torn_last_line_does_not_make_the_history_unreadable() {
        // A crash mid-append is exactly when somebody wants to read this.
        let path = scratch("torn");
        let j = Journal::new(path.clone());
        j.append(&entry("ada", "p1")).unwrap();
        j.append(&entry("ada", "p2")).unwrap();
        {
            use std::io::Write as _;
            let mut f = OpenOptions::new().append(true).open(&path).unwrap();
            f.write_all(b"{\"at\":\"2026-01-01T00:00:00Z\",\"who\":\"tru")
                .unwrap();
        }
        let all = j.read_all();
        assert_eq!(all.len(), 2, "the intact lines are still readable");
    }

    #[test]
    fn history_is_filtered_per_entity_newest_first() {
        let j = Journal::new(scratch("per-entity"));
        j.append(&entry("ada", "p1")).unwrap();
        j.append(&entry("bob", "p2")).unwrap();
        let mut third = entry("cleo", "p1");
        third.version_num = Some(3);
        j.append(&third).unwrap();

        let h = j.for_entity("person", "p1");
        assert_eq!(h.len(), 2);
        assert_eq!(h[0].who, "cleo", "newest first");
        assert_eq!(j.last_touched("person", "p1").unwrap().who, "cleo");
        assert_eq!(j.last_touched("person", "p2").unwrap().who, "bob");
        assert!(j.last_touched("person", "nobody").is_none());
    }

    #[test]
    fn an_absent_journal_reads_as_empty_rather_than_failing() {
        // A fresh installation has never written one, and every page that
        // shows history has to work on the day it is installed.
        let j = Journal::new(scratch("absent").with_file_name("never-written.journal"));
        assert!(j.read_all().is_empty());
        assert!(j.is_empty());
        assert!(j.last_touched("person", "p1").is_none());
    }

    #[cfg(unix)]
    #[test]
    fn the_journal_is_created_at_mode_600() {
        use std::os::unix::fs::PermissionsExt as _;
        let path = scratch("mode");
        let j = Journal::new(path.clone());
        j.append(&entry("ada", "p1")).unwrap();
        let mode = std::fs::metadata(&path).unwrap().permissions().mode() & 0o777;
        assert_eq!(
            mode, 0o600,
            "it names people and quotes what they changed; that is not less \
             sensitive than the accounts"
        );
    }

    fn versioned(v: u64, path: &str, from: &str, to: &str) -> Entry {
        Entry {
            at: format!("2026-01-0{v}T00:00:00Z"),
            who: "ada".into(),
            action: "update".into(),
            kind: "person".into(),
            entity_id: "p1".into(),
            label: None,
            version_num: Some(v),
            changes: vec![crate::diff::Change {
                path: path.into(),
                from: Some(from.into()),
                to: Some(to.into()),
            }],
        }
    }

    #[test]
    fn rewinding_reproduces_the_version_an_editor_started_from() {
        // The bundle holds only the current version, so after a conflict the
        // one the losing editor opened is gone. Replaying the journal
        // backwards is what brings it back.
        let current = json!({"notes": "third", "version_num": 3});
        // Newest first, as `for_entity` returns them.
        let entries = vec![
            versioned(3, "notes", "second", "third"),
            versioned(2, "notes", "first", "second"),
        ];
        let base = rewind(&current, &entries, 1, 3).expect("reconstructable");
        assert_eq!(base["notes"], "first");
    }

    #[test]
    fn rewinding_to_the_current_version_is_the_current_document() {
        let current = json!({"notes": "now"});
        assert_eq!(rewind(&current, &[], 4, 4).unwrap(), current);
    }

    #[test]
    fn a_gap_in_the_journal_refuses_to_guess() {
        // An edit made before journalling, by another tool, or lost to a torn
        // write. Reconstructing across it would produce a confident and wrong
        // answer, and the conflict page falls back to a two-way comparison.
        let current = json!({"notes": "third"});
        let entries = vec![versioned(3, "notes", "second", "third")];
        assert!(
            rewind(&current, &entries, 1, 3).is_none(),
            "version 2 is missing, so version 1 cannot be reproduced"
        );
    }

    #[test]
    fn rewinding_restores_an_added_field_to_absence() {
        let current = json!({"notes": "added later", "name": "kept"});
        let entries = vec![Entry {
            at: "2026-01-02T00:00:00Z".into(),
            who: "ada".into(),
            action: "update".into(),
            kind: "person".into(),
            entity_id: "p1".into(),
            label: None,
            version_num: Some(2),
            changes: vec![crate::diff::Change {
                path: "notes".into(),
                from: None,
                to: Some("added later".into()),
            }],
        }];
        let base = rewind(&current, &entries, 1, 2).expect("reconstructable");
        assert!(base.get("notes").is_none(), "it was not there before");
        assert_eq!(base["name"], "kept");
    }

    #[test]
    fn a_reverted_boolean_comes_back_as_a_boolean() {
        // The diff rendered it as a string. Restoring it as one would make the
        // next diff report a change nobody made.
        let current = json!({"identity": {"is_living": false}});
        let entries = vec![Entry {
            at: "2026-01-02T00:00:00Z".into(),
            who: "ada".into(),
            action: "update".into(),
            kind: "person".into(),
            entity_id: "p1".into(),
            label: None,
            version_num: Some(2),
            changes: vec![crate::diff::Change {
                path: "identity.is_living".into(),
                from: Some("true".into()),
                to: Some("false".into()),
            }],
        }];
        let base = rewind(&current, &entries, 1, 2).expect("reconstructable");
        assert_eq!(base["identity"]["is_living"], Value::Bool(true));
        assert!(
            crate::diff::diff(&base, &base.clone()).is_empty(),
            "and the reconstruction is stable under its own diff"
        );
    }

    #[test]
    fn rewinding_walks_a_nested_path() {
        let current = json!({"identity": {"name": {"display": "Karin"}}});
        let entries = vec![versioned(2, "identity.name.display", "Kowalski", "Karin")];
        let base = rewind(&current, &entries, 1, 2).expect("reconstructable");
        assert_eq!(base["identity"]["name"]["display"], "Kowalski");
    }

    #[test]
    fn a_create_and_a_delete_read_as_themselves() {
        let base = Record {
            who: "ada",
            action: "create",
            kind: "person",
            entity_id: "p1",
            label: None,
            version_num: Some(1),
            before: None,
            after: None,
        };
        assert_eq!(entry_for(base).summary(), "created");
        assert_eq!(
            entry_for(Record {
                action: "delete",
                version_num: None,
                who: "ada",
                kind: "person",
                entity_id: "p1",
                label: None,
                before: None,
                after: None,
            })
            .summary(),
            "deleted"
        );
    }
}
