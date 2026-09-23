//! Never lose a write.
//!
//! Four failures an installation in a family's house actually meets — a full
//! disk, a wrong owner, a bundle that has drifted onto a second filesystem,
//! and a process killed part-way through a save — and what the operator is
//! told about each. The value being tested is not that the program survives
//! them: it is that the previous bundle is still byte-identical afterwards and
//! that the sentence on the screen names the remedy.

mod common;

use std::path::Path;

use common::*;
use serde_json::json;

const ALICE: &str = "11111111-1111-4111-8111-111111111111";

fn bundle_with_one_person(tag: &str) -> Scratch {
    let dir = scratch(tag);
    let path = dir.join("family.axgf");
    let flat = json!({
        "manifest": {"axgf": "1.0", "family": {"name": "Durability"}},
        "persons": {ALICE: {
            "id": ALICE, "type": "person", "axgf_version": "1.0", "version_num": 1,
            "identity": {"name": {"display": "Alice", "components": []},
                         "visibility": "public", "is_living": false}}},
        "families": {}, "events": {}, "links": {}, "occupations": {},
        "sources": {}, "places": {}, "documents": {}
    });
    std::fs::write(
        &path,
        axgf_cms::state::export_to_bytes(&flat.to_string()).expect("export"),
    )
    .expect("write");
    dir.pointing_at(path)
}

// ---------------------------------------------------------------------------
// The messages. One per remedy, because they are different remedies.
// ---------------------------------------------------------------------------

#[test]
fn a_full_disk_says_the_disk_is_full_and_that_nothing_was_lost() {
    // The two questions an operator has, in that order: did I lose anything,
    // and what do I do. The old message — "renaming /tmp/wt-full.axgf.tmp over
    // /tmp/wt-full.axgf" — answered neither.
    let msg = axgf_cms::space::explain_write_failure(
        &std::io::Error::from_raw_os_error(libc_enospc()),
        Path::new("/var/lib/axgf-cms/family.axgf"),
        Some(Path::new("/var/lib/axgf-cms/family.axgf.tmp")),
    );
    assert!(msg.contains("full"), "{msg}");
    assert!(msg.contains("nothing has been lost"), "{msg}");
    assert!(
        msg.contains("payload cache"),
        "and it names something safe to delete: {msg}"
    );
    assert!(
        !msg.contains(".tmp over"),
        "the old message must not come back: {msg}"
    );
}

#[test]
fn a_cross_device_rename_is_told_apart_from_a_full_disk() {
    let full = axgf_cms::space::explain_write_failure(
        &std::io::Error::from_raw_os_error(libc_enospc()),
        Path::new("/srv/family.axgf"),
        None,
    );
    let xdev = axgf_cms::space::explain_write_failure(
        &std::io::Error::from_raw_os_error(18), // EXDEV
        Path::new("/srv/family.axgf"),
        Some(Path::new("/tmp/family.axgf.tmp")),
    );
    assert!(xdev.contains("two different filesystems"), "{xdev}");
    assert!(
        xdev.contains("configuration problem"),
        "freeing space will not help here, and it says so: {xdev}"
    );
    assert_ne!(full, xdev);
}

#[test]
fn a_sticky_shared_directory_is_named_rather_than_left_as_permission_denied() {
    // /tmp is 1777: the directory is writable and the rename still fails,
    // which makes the ordinary "check the owner of the directory" advice a
    // dead end. This is the case the operator's own `/tmp/wt-full.axgf` hit.
    let msg = axgf_cms::space::explain_write_failure(
        &std::io::Error::from_raw_os_error(1), // EPERM
        Path::new("/tmp/wt-full.axgf"),
        Some(Path::new("/tmp/wt-full.axgf.tmp")),
    );
    assert!(msg.contains("sticky bit"), "{msg}");
    assert!(msg.contains("/var/lib/axgf-cms"), "{msg}");
}

fn libc_enospc() -> i32 {
    28
}

// ---------------------------------------------------------------------------
// The guard that runs before the first byte.
// ---------------------------------------------------------------------------

#[test]
fn a_write_that_cannot_fit_is_refused_before_it_starts() {
    let dir = scratch("dur-space");
    let err = axgf_cms::space::ensure_room_for(
        dir.dir(),
        &axgf_cms::space::Need {
            bytes: u64::MAX / 4,
            what: "the bundle",
        },
    )
    .expect_err("four exabytes do not fit anywhere");
    let msg = err.to_string();
    assert!(msg.contains("short by"), "{msg}");
    assert!(msg.contains("existing data is untouched"), "{msg}");
    assert!(
        msg.contains("built beside the old one"),
        "the figure is larger than the file, and that needs explaining: {msg}"
    );
}

#[test]
fn the_requirement_counts_both_copies_because_both_exist_at_once() {
    // The margin is what makes the difference between "there is exactly enough
    // room for one more copy" and "the save succeeds".
    assert_eq!(axgf_cms::space::margin_for(0), 16 * 1024 * 1024);
    assert_eq!(
        axgf_cms::space::margin_for(1000 * 1024 * 1024),
        100 * 1024 * 1024
    );
}

// ---------------------------------------------------------------------------
// Wreckage from an interrupted write.
// ---------------------------------------------------------------------------

#[tokio::test]
async fn a_temp_file_left_by_a_crash_is_swept_at_startup_and_the_bundle_is_untouched() {
    let path = bundle_with_one_person("dur-sweep");
    let before = std::fs::read(&path).expect("read bundle");

    // Exactly what a kill -9 during an export leaves: a sibling of the bundle,
    // with the bundle's name and something ending .tmp appended. Several,
    // because a crash loop leaves several.
    let dir = path.parent().unwrap();
    let strays = [
        dir.join("family.axgf.tmp"),
        dir.join("family.axgf.download-7c9e6679.tmp"),
        dir.join("family.axgf.export-f47ac10b.tmp"),
    ];
    for s in &strays {
        std::fs::write(s, vec![0u8; 512 * 1024]).expect("write stray");
    }
    // And something that is not ours, in the same directory.
    let innocent = dir.join("family.axgf.notes");
    std::fs::write(&innocent, b"keep me").expect("write");

    let app = axgf_cms::app(&path, TOKEN).expect("start over the wreckage");

    for s in &strays {
        assert!(!s.exists(), "{} survived startup", s.display());
    }
    assert!(innocent.exists(), "the sweep took a file that was not ours");
    assert_eq!(
        before,
        std::fs::read(&path).expect("read bundle"),
        "the bundle itself is the one thing a crashed write never touches"
    );
    // And the tree is still served.
    let body = body_string(get(&app, &format!("/person/{ALICE}")).await).await;
    assert!(body.contains("Alice"), "the record is still there");
}

#[test]
fn a_temp_file_is_never_swept_while_another_process_is_writing_one() {
    // The sweep runs under the write lock precisely so that a `.tmp` being
    // written *this instant* by a save in another process is not deleted out
    // from under it.
    let path = bundle_with_one_person("dur-sweep-locked");
    let dir = path.parent().unwrap();
    let in_flight = dir.join("family.axgf.download-inflight.tmp");
    std::fs::write(&in_flight, b"being written right now").expect("write");

    let held = axgf_cms::lockfile::WriteLock::acquire(&path).expect("hold the lock");
    axgf_cms::state::sweep_stale_temp_files(&path);
    assert!(
        in_flight.exists(),
        "the sweep deleted a temp file belonging to a live write"
    );
    drop(held);

    axgf_cms::state::sweep_stale_temp_files(&path);
    assert!(
        !in_flight.exists(),
        "and removes it once nothing is writing"
    );
}

// ---------------------------------------------------------------------------
// The journal, which is history and must not be pruned to bound its size.
// ---------------------------------------------------------------------------

fn journal_entry(v: u64) -> axgf_cms::journal::Entry {
    axgf_cms::journal::entry_for(axgf_cms::journal::Record {
        who: "greta",
        action: "update",
        kind: "person",
        entity_id: ALICE,
        label: Some("Alice".into()),
        version_num: Some(v),
        before: Some(&json!({"notes": "x"})),
        after: Some(&json!({"notes": "y"})),
    })
}

#[test]
fn a_rotated_journal_still_reads_as_one_history() {
    let dir = scratch("dur-rotate");
    let path = dir.join("family.journal");
    let j = axgf_cms::journal::Journal::new(path.clone());

    // Fat entries, so this is a hundred and thirty appends rather than two
    // thousand: the threshold is a production figure, not a test fixture.
    let filler = "x".repeat(64 * 1024);
    let mut written = 0usize;
    while j.segments().len() == 1 {
        let mut e = journal_entry(written as u64 + 1);
        e.label = Some(filler.clone());
        j.append(&e).expect("append");
        written += 1;
        assert!(written < 1000, "rotation never happened");
    }
    assert_eq!(j.segments().len(), 2, "one old segment and the live one");
    assert!(path.with_extension("journal.1").exists());

    assert_eq!(
        j.len(),
        written,
        "rotation must not lose a single line — the conflict screen replays \
         this backwards and a restore is only as good as the history with it"
    );
    let all = j.read_all();
    assert_eq!(all.len(), written);
    assert_eq!(all[0].version_num, Some(1), "oldest first, across segments");
    assert_eq!(all[written - 1].version_num, Some(written as u64));

    // And the cheap reads still answer correctly.
    let recent = j.recent(3);
    assert_eq!(recent.len(), 3);
    assert_eq!(recent[0].version_num, Some(written as u64), "newest first");
    assert_eq!(
        j.for_entity("person", ALICE).len(),
        written,
        "per-entity history spans segments too"
    );
}

#[test]
fn rotation_never_overwrites_an_older_segment() {
    let dir = scratch("dur-rotate-twice");
    let path = dir.join("family.journal");
    let j = axgf_cms::journal::Journal::new(path.clone());
    let filler = "x".repeat(64 * 1024);

    let mut written = 0usize;
    while j.segments().len() < 3 {
        let mut e = journal_entry(written as u64 + 1);
        e.label = Some(filler.clone());
        j.append(&e).expect("append");
        written += 1;
        assert!(written < 2000, "the second rotation never happened");
    }

    assert_eq!(j.segments().len(), 3, "two rotations, three segments");
    assert!(path.with_extension("journal.2").exists());
    assert_eq!(j.len(), written, "and still every line");
    let all = j.read_all();
    for (i, e) in all.iter().enumerate() {
        assert_eq!(
            e.version_num,
            Some(i as u64 + 1),
            "the order across three segments is the order they were written"
        );
    }
}

#[tokio::test]
async fn the_conflict_screen_can_still_reconstruct_across_a_rotation() {
    // The one thing rotation could actually break: `rewind` replays a
    // journal backwards, and a gap makes it refuse to guess. A rotation must
    // not look like a gap.
    let dir = scratch("dur-rotate-rewind");
    let path = dir.join("family.journal");
    let j = axgf_cms::journal::Journal::new(path.clone());

    // Version 1 -> 2, then a great deal of unrelated noise that forces a
    // rotation, then version 2 -> 3.
    j.append(&change(1, "first", "second")).expect("append");
    let filler = "x".repeat(64 * 1024);
    let mut guard = 0;
    while j.segments().len() == 1 {
        let mut e = journal_entry(0);
        e.entity_id = "somebody-else".into();
        e.label = Some(filler.clone());
        j.append(&e).expect("append");
        guard += 1;
        assert!(guard < 1000, "rotation never happened");
    }
    j.append(&change(2, "second", "third")).expect("append");
    assert!(j.segments().len() >= 2, "the history spans a rotation");

    let current = json!({"notes": "third"});
    let entries = j.for_entity("person", ALICE);
    let base = axgf_cms::journal::rewind(&current, &entries, 0, 2)
        .expect("the version the losing editor started from is still reachable");
    assert_eq!(base["notes"], "first");
}

fn change(version: u64, from: &str, to: &str) -> axgf_cms::journal::Entry {
    axgf_cms::journal::Entry {
        at: format!("2026-01-0{version}T00:00:00Z"),
        who: "greta".into(),
        action: "update".into(),
        kind: "person".into(),
        entity_id: ALICE.into(),
        label: None,
        version_num: Some(version),
        changes: vec![axgf_cms::diff::Change {
            path: "notes".into(),
            from: Some(from.into()),
            to: Some(to.into()),
        }],
    }
}

// ---------------------------------------------------------------------------
// The payload cache, which is derived data and used to grow without bound.
// ---------------------------------------------------------------------------

#[tokio::test]
async fn stale_payload_cache_generations_are_swept_at_startup() {
    // Every save produces a bundle with a new SHA-256, which opens a new cache
    // generation and orphans the last one — complete, with every photograph in
    // it. Nothing removed them: on the machine this was written for, five
    // generations of one bundle had reached 2.1 GB, on the disk the bundle is
    // saved to.
    let path = bundle_with_one_person("dur-cache-sweep");
    let dir = path.parent().unwrap();
    let cache_base = dir.join(".axgf-cms-cache");
    std::fs::create_dir_all(&cache_base).expect("create");

    // Two generations from older bundles, named as the cache names them.
    let stale: Vec<_> = ["a".repeat(64), "b".repeat(64)]
        .iter()
        .map(|n| {
            let d = cache_base.join(n);
            std::fs::create_dir_all(&d).unwrap();
            std::fs::write(d.join("photo.bin"), vec![0u8; 256 * 1024]).unwrap();
            d
        })
        .collect();
    // And a directory that is not a generation.
    let innocent = cache_base.join("notes");
    std::fs::create_dir_all(&innocent).unwrap();

    let _app = axgf_cms::app(&path, TOKEN).expect("start");

    for d in &stale {
        assert!(!d.exists(), "{} was not swept", d.display());
    }
    assert!(
        innocent.exists(),
        "the sweep took something that was not a generation"
    );
    // The live generation is of course still there.
    let live: Vec<_> = std::fs::read_dir(&cache_base)
        .unwrap()
        .flatten()
        .filter(|e| e.path().is_dir())
        .collect();
    assert_eq!(
        live.len(),
        2,
        "the live generation and the innocent directory"
    );
}
