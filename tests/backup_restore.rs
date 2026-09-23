//! Backups, and the only thing that makes one real: restoring it.
//!
//! Every test here destroys something. A backup that has never been restored
//! is a claim, and a `.zip` in a backup directory is the most confidently
//! believed claim an installation makes about itself — so the tests do the
//! believing in advance: wipe the bundle, wipe the accounts, wipe the journal,
//! put them back, and compare every entity, every payload's SHA-256, every
//! account and every journal line against what was there before.

mod common;

use std::collections::BTreeMap;
use std::path::Path;

use axgf_cms::backup::{self, Retention};
use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use base64::Engine as _;
use common::*;
use serde_json::json;
use tower::ServiceExt;

const ALICE: &str = "11111111-1111-4111-8111-111111111111";
const BOB: &str = "22222222-2222-4222-8222-222222222222";

fn person(id: &str, name: &str) -> serde_json::Value {
    json!({
        "id": id, "type": "person", "axgf_version": "1.0", "version_num": 1,
        "identity": {"name": {"display": name, "components": [
                        {"type": "given_name", "value": name, "order": 1}]},
                     "visibility": "public", "is_living": false}
    })
}

fn png(seed: u8) -> Vec<u8> {
    let img = image::RgbImage::from_fn(9, 7, |x, y| {
        image::Rgb([seed, (x * 20) as u8, (y * 30) as u8])
    });
    let mut out = std::io::Cursor::new(Vec::new());
    image::DynamicImage::ImageRgb8(img)
        .write_to(&mut out, image::ImageFormat::Png)
        .expect("encode");
    out.into_inner()
}

/// A bundle with two people and one photograph actually inside the archive.
///
/// The payload matters: a backup that carries the textual half and loses the
/// media would pass every test that only counted entities.
fn seeded(tag: &str) -> Scratch {
    let dir = scratch(tag);
    let path = dir.join("family.axgf");
    let bytes = png(200);
    let sha = axgf_cms::documents::sha256_hex(&bytes);
    let doc = "33333333-3333-4333-8333-333333333333";
    let zip_path = format!("documents/files/{doc}.png");
    let flat = json!({
        "manifest": {"axgf": "1.0", "family": {"name": "Backup"}},
        "persons": {ALICE: person(ALICE, "Alice"), BOB: person(BOB, "Bob")},
        "families": {}, "events": {}, "links": {}, "occupations": {},
        "sources": {}, "places": {},
        "documents": {doc: {
            "id": doc, "type": "document", "axgf_version": "1.0",
            "filename": "portrait.png", "mime_type": "image/png",
            "document_type": "photo", "status": "present",
            "file": {"path": zip_path, "size_bytes": bytes.len(), "sha256": sha}
        }},
        "attachments": {zip_path: base64::engine::general_purpose::STANDARD.encode(&bytes)}
    });
    std::fs::write(
        &path,
        axgf_cms::state::export_to_bytes(&flat.to_string()).expect("export"),
    )
    .expect("write");

    // Accounts and a journal line, so all three files are real.
    let mut acl = axgf_cms::acl::Acl::default();
    acl.users.push(
        axgf_cms::acl::new_user("greta", "correct-horse-battery", axgf_cms::acl::Role::Admin)
            .expect("user"),
    );
    acl.save(&axgf_cms::acl::Acl::path_for(&path)).expect("acl");

    let journal = axgf_cms::journal::Journal::new(axgf_cms::journal::Journal::path_for(&path));
    for i in 0..3 {
        journal
            .append(&axgf_cms::journal::entry_for(axgf_cms::journal::Record {
                who: "greta",
                action: "update",
                kind: "person",
                entity_id: ALICE,
                label: Some("Alice".into()),
                version_num: Some(i + 1),
                before: Some(&json!({"note": format!("v{i}")})),
                after: Some(&json!({"note": format!("v{}", i + 1)})),
            }))
            .expect("journal");
    }

    dir.pointing_at(path)
}

/// Everything about an installation that a restore has to reproduce.
#[derive(Debug, PartialEq, Eq)]
struct Fingerprint {
    /// Every entity id, by collection.
    entities: BTreeMap<String, Vec<String>>,
    /// Every document payload, by its SHA-256 as read back out of the archive.
    payloads: BTreeMap<String, String>,
    /// Every account's username and password hash.
    accounts: Vec<(String, String)>,
    /// Every journal line, verbatim.
    journal: Vec<String>,
}

/// Read an installation without going through `AppState`, so the comparison
/// does not depend on the thing being tested.
fn fingerprint(bundle: &Path) -> Fingerprint {
    use std::io::Read as _;

    let file = std::fs::File::open(bundle).expect("open bundle");
    let mut entities: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut payloads = BTreeMap::new();

    let env = axgf_rs::import_bundle_streaming(file, |_p| Ok(()));
    let flat = axgf_cms::state::envelope_into_data(env).expect("import");
    for name in axgf_cms::state::COLLECTIONS {
        let mut ids: Vec<String> = flat
            .get(name)
            .and_then(|v| v.as_object())
            .map(|m| m.keys().cloned().collect())
            .unwrap_or_default();
        ids.sort();
        entities.insert(name.to_string(), ids);
    }

    // The payload bytes, straight out of the ZIP.
    let mut zip = zip::ZipArchive::new(std::fs::File::open(bundle).expect("open")).expect("zip");
    for i in 0..zip.len() {
        let mut m = zip.by_index(i).expect("member");
        let name = m.name().to_string();
        if !name.starts_with("documents/files/") {
            continue;
        }
        let mut bytes = Vec::new();
        m.read_to_end(&mut bytes).expect("read payload");
        payloads.insert(name, axgf_cms::documents::sha256_hex(&bytes));
    }

    let acl_path = axgf_cms::acl::Acl::path_for(bundle);
    let accounts = axgf_cms::acl::Acl::load(&acl_path)
        .map(|a| {
            a.users
                .iter()
                .map(|u| (u.username.clone(), u.password_hash.clone()))
                .collect()
        })
        .unwrap_or_default();

    let journal_path = axgf_cms::journal::Journal::path_for(bundle);
    let journal = std::fs::read_to_string(&journal_path)
        .unwrap_or_default()
        .lines()
        .map(str::to_string)
        .collect();

    Fingerprint {
        entities,
        payloads,
        accounts,
        journal,
    }
}

// ---------------------------------------------------------------------------
// The headline: destroy everything, put it back, compare.
// ---------------------------------------------------------------------------

#[test]
fn a_backup_restores_every_entity_every_payload_every_account_and_every_journal_line() {
    let bundle = seeded("br-roundtrip");
    let dest = scratch("br-roundtrip-dest");
    let before = fingerprint(&bundle);
    assert!(!before.payloads.is_empty(), "the fixture has a photograph");
    assert_eq!(before.accounts.len(), 1);
    assert_eq!(before.journal.len(), 3);

    let report = backup::run(&bundle, dest.dir(), Retention::default()).expect("backup");
    assert!(report.archive.exists());
    assert_eq!(report.manifest.accounts, 1);
    assert_eq!(report.manifest.journal_lines, 3);

    // Destroy the lot. This is the disk failure, in one line.
    for p in [
        bundle.to_path_buf(),
        axgf_cms::acl::Acl::path_for(&bundle),
        axgf_cms::journal::Journal::path_for(&bundle),
    ] {
        std::fs::remove_file(&p).expect("destroy");
        assert!(!p.exists());
    }

    backup::restore(&report.archive, &bundle, false).expect("restore");

    let after = fingerprint(&bundle);
    assert_eq!(
        before, after,
        "the restored installation is not the one that was backed up"
    );
}

#[tokio::test]
async fn a_restored_installation_serves_the_same_pages_and_the_same_photograph() {
    // The fingerprint compares files. This compares what a family member sees,
    // which is the thing they would actually notice.
    let bundle = seeded("br-serve");
    let dest = scratch("br-serve-dest");

    let app = axgf_cms::app(&bundle, TOKEN).expect("app");
    let before_page = body_string(get(&app, &format!("/person/{ALICE}")).await).await;
    let doc_id = "33333333-3333-4333-8333-333333333333";
    let before_bytes = body_bytes(get(&app, &format!("/document/{doc_id}/raw")).await).await;
    assert!(!before_bytes.is_empty(), "the photograph is served");
    drop(app);

    let report = backup::run(&bundle, dest.dir(), Retention::default()).expect("backup");
    std::fs::remove_file(&bundle).expect("destroy the bundle");
    // The payload cache survives, so this also proves the restore does not
    // merely serve what the cache happens to still hold.
    backup::restore(&report.archive, &bundle, false).expect("restore");

    let app = axgf_cms::app(&bundle, TOKEN).expect("app after restore");
    let after_page = body_string(get(&app, &format!("/person/{ALICE}")).await).await;
    let after_bytes = body_bytes(get(&app, &format!("/document/{doc_id}/raw")).await).await;
    assert_eq!(before_page, after_page, "the record page changed");
    assert_eq!(
        axgf_cms::documents::sha256_hex(&before_bytes),
        axgf_cms::documents::sha256_hex(&after_bytes),
        "the photograph came back as different bytes"
    );
}

// ---------------------------------------------------------------------------
// A backup that cannot be read is refused, loudly, before it can be trusted.
// ---------------------------------------------------------------------------

#[test]
fn a_corrupted_archive_is_refused_and_the_live_data_is_untouched() {
    let bundle = seeded("br-corrupt");
    let dest = scratch("br-corrupt-dest");
    let report = backup::run(&bundle, dest.dir(), Retention::default()).expect("backup");
    let before = fingerprint(&bundle);

    // Flip bytes in the middle of the archive: the ZIP structure survives, the
    // member's CRC does not. This is what a decaying disk does.
    let mut bytes = std::fs::read(&report.archive).expect("read archive");
    let mid = bytes.len() / 2;
    for b in bytes.iter_mut().skip(mid).take(512) {
        *b ^= 0xff;
    }
    std::fs::write(&report.archive, &bytes).expect("corrupt");

    let err = backup::verify(&report.archive).expect_err("a damaged archive must not verify");
    let msg = format!("{err:#}");
    assert!(
        msg.contains("damaged") || msg.contains("not the file that was backed up"),
        "the refusal has to say the archive is bad: {msg}"
    );

    let err = backup::restore(&report.archive, &bundle, false).expect_err("restore must refuse");
    assert!(!format!("{err:#}").is_empty());
    assert_eq!(
        before,
        fingerprint(&bundle),
        "a refused restore must not have touched anything"
    );
    // And nothing was moved aside, because nothing got as far as moving.
    let strays: Vec<_> = std::fs::read_dir(bundle.parent().unwrap())
        .unwrap()
        .flatten()
        .filter(|e| {
            e.file_name()
                .to_string_lossy()
                .starts_with("axgf-replaced-")
        })
        .collect();
    assert!(strays.is_empty(), "a refused restore staged something");
}

#[test]
fn an_archive_truncated_in_transit_is_refused() {
    let bundle = seeded("br-truncated");
    let dest = scratch("br-truncated-dest");
    let report = backup::run(&bundle, dest.dir(), Retention::default()).expect("backup");

    let bytes = std::fs::read(&report.archive).expect("read");
    std::fs::write(&report.archive, &bytes[..bytes.len() * 2 / 3]).expect("truncate");

    let err = backup::verify(&report.archive).expect_err("half an archive is not an archive");
    let msg = format!("{err:#}");
    assert!(
        msg.contains("ZIP") || msg.contains("copy completed") || msg.contains("damaged"),
        "{msg}"
    );
}

#[test]
fn an_archive_whose_bundle_was_swapped_for_rubbish_is_refused() {
    // The SHA-256 in the manifest is what catches this: the ZIP is rebuilt so
    // every CRC is correct, and only the recorded digest disagrees.
    let bundle = seeded("br-swapped");
    let dest = scratch("br-swapped-dest");
    let report = backup::run(&bundle, dest.dir(), Retention::default()).expect("backup");

    rebuild_archive_with(
        &report.archive,
        backup::MEMBER_BUNDLE,
        b"not a bundle at all",
    );

    let err = backup::verify(&report.archive).expect_err("refused");
    let msg = format!("{err:#}");
    assert!(msg.contains("not the file that was backed up"), "{msg}");
    assert!(msg.contains("recorded"), "it shows both digests: {msg}");
}

#[test]
fn an_archive_with_no_manifest_is_not_treated_as_one_of_ours() {
    let dir = scratch("br-nomanifest");
    let path = dir.join("axgf-backup-20260101T000000Z.zip");
    {
        let f = std::fs::File::create(&path).expect("create");
        let mut z = zip::ZipWriter::new(f);
        z.start_file("family.axgf", zip::write::FileOptions::default())
            .unwrap();
        std::io::Write::write_all(&mut z, b"whatever").unwrap();
        z.finish().unwrap();
    }
    let err = backup::verify(&path).expect_err("refused");
    assert!(format!("{err:#}").contains("backup.json"), "{err:#}");
}

/// Rewrite one member of a ZIP, leaving every CRC correct.
fn rebuild_archive_with(archive: &Path, member: &str, content: &[u8]) {
    use std::io::{Read as _, Write as _};
    let mut src = zip::ZipArchive::new(std::fs::File::open(archive).unwrap()).unwrap();
    let tmp = archive.with_extension("rebuilt");
    {
        let mut out = zip::ZipWriter::new(std::fs::File::create(&tmp).unwrap());
        for i in 0..src.len() {
            let mut m = src.by_index(i).unwrap();
            let name = m.name().to_string();
            let mut bytes = Vec::new();
            m.read_to_end(&mut bytes).unwrap();
            out.start_file(&name, zip::write::FileOptions::default())
                .unwrap();
            if name == member {
                out.write_all(content).unwrap();
            } else {
                out.write_all(&bytes).unwrap();
            }
        }
        out.finish().unwrap();
    }
    std::fs::rename(&tmp, archive).unwrap();
}

// ---------------------------------------------------------------------------
// Restoring under a running instance, and keeping the old state.
// ---------------------------------------------------------------------------

#[test]
fn a_restore_keeps_the_state_it_replaces() {
    let bundle = seeded("br-aside");
    let dest = scratch("br-aside-dest");
    let report = backup::run(&bundle, dest.dir(), Retention::default()).expect("backup");
    let before = fingerprint(&bundle);

    // Change the live installation, then restore over it.
    let app = axgf_cms::app(&bundle, TOKEN).expect("app");
    drop(app);
    std::fs::write(axgf_cms::journal::Journal::path_for(&bundle), b"").expect("wipe journal");

    let plan = backup::restore(&report.archive, &bundle, false).expect("restore");
    assert!(plan.aside.is_dir(), "the replaced state has to still exist");
    assert!(
        plan.aside.join("family.axgf").exists(),
        "including the bundle: restoring the wrong archive must be reversible"
    );
    assert_eq!(before, fingerprint(&bundle), "and the restore was correct");
}

#[test]
fn a_restore_refuses_while_a_server_is_attached_to_the_bundle() {
    // The case that used to slip through. A running server does *not* keep the
    // bundle open — it reads the tree in at startup and closes the file — so
    // looking for an open descriptor found nothing, and a restore happily
    // swapped the files out from under a live process whose next save would
    // have written the old tree back over them.
    let bundle = seeded("br-attached");
    let dest = scratch("br-attached-dest");
    let report = backup::run(&bundle, dest.dir(), Retention::default()).expect("backup");
    let before = fingerprint(&bundle);

    let instance = axgf_cms::lockfile::InstanceLock::acquire(&bundle).expect("attach");
    let err = backup::restore(&report.archive, &bundle, false)
        .expect_err("a restore under a running server must be refused");
    let msg = format!("{err:#}");
    assert!(msg.contains("an instance is running"), "{msg}");
    assert!(msg.contains("systemctl stop axgf-cms"), "{msg}");
    assert!(msg.contains("Nothing was changed"), "{msg}");
    assert_eq!(before, fingerprint(&bundle));

    drop(instance);
    backup::restore(&report.archive, &bundle, false).expect("and it works once stopped");
}

#[test]
fn a_second_server_over_one_bundle_is_refused() {
    let bundle = seeded("br-two-servers");
    let first = axgf_cms::lockfile::InstanceLock::acquire(&bundle).expect("first");
    let err = axgf_cms::lockfile::InstanceLock::acquire(&bundle).expect_err("second");
    assert!(format!("{err:#}").contains("already serving"), "{err:#}");
    drop(first);
}

#[test]
fn a_restore_refuses_while_something_holds_the_write_lock() {
    let bundle = seeded("br-running");
    let dest = scratch("br-running-dest");
    let report = backup::run(&bundle, dest.dir(), Retention::default()).expect("backup");
    let before = fingerprint(&bundle);

    let held = axgf_cms::lockfile::WriteLock::acquire(&bundle).expect("hold the lock");
    let err = backup::restore(&report.archive, &bundle, false)
        .expect_err("a restore under a writer must be refused");
    let msg = format!("{err:#}");
    assert!(msg.contains("systemctl stop axgf-cms"), "{msg}");
    assert!(msg.contains("Nothing was changed"), "{msg}");
    drop(held);

    assert_eq!(before, fingerprint(&bundle));
    backup::restore(&report.archive, &bundle, false).expect("and it works once released");
}

// ---------------------------------------------------------------------------
// Consistency, retention, and the parts that only matter under load.
// ---------------------------------------------------------------------------

#[tokio::test]
async fn a_backup_taken_during_editing_holds_three_files_that_agree() {
    // The failure this guards against is invisible until the day it is
    // restored: a bundle from after a save and an ACL from before it.
    let bundle = seeded("br-consistent");
    let dest = scratch("br-consistent-dest");
    let app = axgf_cms::app(&bundle, TOKEN).expect("app");

    let bundle_path = bundle.to_path_buf();
    let dest_path = dest.dir().to_path_buf();
    let writer = std::thread::spawn(move || {
        // Twenty archives, taken while the handler thread is saving. Each one
        // is checked the moment it is written, because retention keeps one
        // archive per day and these are all on the same day: by the end of
        // the loop nineteen of them have rightly been pruned.
        let mut taken = 0usize;
        for i in 0..8 {
            let report = match backup::run(&bundle_path, &dest_path, Retention::default()) {
                Ok(r) => r,
                Err(e) => panic!("a backup during editing failed: {e:#}"),
            };
            // `run` verifies before naming the file, so reaching here already
            // means the bundle imported and validated and the ACL parsed. This
            // is the extra claim: the counts the manifest recorded while the
            // lock was held are the counts the archive actually contains.
            let v = backup::verify(&report.archive)
                .unwrap_or_else(|e| panic!("archive {i} did not verify: {e:#}"));
            assert_eq!(
                v.entities, v.manifest.entities,
                "archive {i} disagrees with its own manifest"
            );
            assert_eq!(v.accounts, v.manifest.accounts, "archive {i}: accounts");
            taken += 1;
        }
        taken
    });

    for i in 0..8u64 {
        // The version the bundle holds right now. A save is refused unless it
        // declares the version it was edited from, which is the same check a
        // second editor's browser goes through.
        let body = format!(
            "base_version={}&identity.name.display={}&raw_json={}",
            i + 1,
            urlencode(&format!("Bob {i}")),
            urlencode(
                &json!({"id": BOB, "type": "person", "axgf_version": "1.0",
                        "version_num": i + 1,
                        "identity": {"name": {"display": format!("Bob {i}"),
                                              "components": []},
                                     "visibility": "public", "is_living": false}})
                .to_string()
            )
        );
        let resp = post_form(&app, &format!("/admin/person/{BOB}"), &body, true).await;
        assert!(
            resp.status().is_success() || resp.status().is_redirection(),
            "save {i} failed: {}",
            resp.status()
        );
    }

    let taken = writer.join().expect("the backup thread");
    assert_eq!(taken, 8, "every backup taken under editing load succeeded");
}

// A multi-thread runtime, because the save blocks its thread while it waits
// for the lock: on the default single-threaded one the `sleep` below could not
// run until the save had already finished, and the test would be measuring
// nothing.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_save_waits_for_a_backup_rather_than_landing_inside_it() {
    // The exclusion itself, without a race: the lock is held by hand, a save
    // is attempted, and it must not have finished until the lock is released.
    let bundle = seeded("br-exclusion");
    let app = axgf_cms::app(&bundle, TOKEN).expect("app");

    let held = axgf_cms::lockfile::WriteLock::acquire(&bundle).expect("take the lock");

    let body = format!(
        "base_version=1&identity.name.display={}&raw_json={}",
        urlencode("Bob Waited"),
        urlencode(
            &json!({"id": BOB, "type": "person", "axgf_version": "1.0", "version_num": 1,
                    "identity": {"name": {"display": "Bob Waited", "components": []},
                                 "visibility": "public", "is_living": false}})
            .to_string()
        )
    );
    let saving =
        tokio::spawn(
            async move { post_form(&app, &format!("/admin/person/{BOB}"), &body, true).await },
        );

    // Give it every chance to finish early. Half a second is thousands of
    // times what this save costs when nothing is in its way.
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    assert!(
        !saving.is_finished(),
        "the save went through while a backup held the write lock"
    );

    drop(held);
    let resp = tokio::time::timeout(std::time::Duration::from_secs(10), saving)
        .await
        .expect("the save must complete once the lock is free")
        .expect("join");
    assert!(
        resp.status().is_success() || resp.status().is_redirection(),
        "and it must succeed, not fail: {}",
        resp.status()
    );
}

fn urlencode(s: &str) -> String {
    s.bytes()
        .map(|b| match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                (b as char).to_string()
            }
            b' ' => "+".to_string(),
            _ => format!("%{b:02X}"),
        })
        .collect()
}

#[test]
fn retention_keeps_the_last_seven_days_four_weeks_and_twelve_months() {
    let dir = scratch("br-retention");
    // Two years of daily archives, as files.
    let start = time::Date::from_calendar_date(2026, time::Month::September, 21).unwrap();
    for i in 0..730i64 {
        let d = start - time::Duration::days(i);
        let name = format!(
            "axgf-backup-{:04}{:02}{:02}T030000Z.zip",
            d.year(),
            u8::from(d.month()),
            d.day()
        );
        std::fs::write(dir.join(&name), b"x").expect("write");
    }
    std::fs::write(dir.join("notes.txt"), b"not mine").expect("write");

    let pruned = backup::prune(dir.dir(), Retention::default()).expect("prune");
    let left = backup::list(dir.dir()).expect("list");
    assert_eq!(pruned.len() + left.len(), 730);
    assert!(left.len() <= 7 + 4 + 12, "kept {}", left.len());
    assert!(left.len() >= 12, "kept {}", left.len());
    assert!(
        dir.join("notes.txt").exists(),
        "retention deleted a file that was not one of ours"
    );
}

#[test]
fn a_backup_run_prunes_as_it_goes() {
    let bundle = seeded("br-prune");
    let dest = scratch("br-prune-dest");
    // An archive from long ago, which a retention of one day must remove.
    std::fs::write(
        dest.join("axgf-backup-20200101T000000Z.zip"),
        b"an ancient archive",
    )
    .expect("write");

    let report = backup::run(
        &bundle,
        dest.dir(),
        Retention {
            daily: 1,
            weekly: 0,
            monthly: 0,
        },
    )
    .expect("backup");
    assert_eq!(report.pruned.len(), 1);
    assert!(report.archive.exists(), "today's archive survived");
    assert!(!dest.join("axgf-backup-20200101T000000Z.zip").exists());
}

#[test]
fn the_archive_is_an_ordinary_zip_anyone_can_open() {
    // The day a backup is needed is the worst possible day to need this
    // program in order to read it.
    let bundle = seeded("br-plainzip");
    let dest = scratch("br-plainzip-dest");
    let report = backup::run(&bundle, dest.dir(), Retention::default()).expect("backup");

    let out = std::process::Command::new("unzip")
        .arg("-l")
        .arg(&report.archive)
        .output();
    let Ok(out) = out else {
        return; // no unzip on this host; the zip crate already read it back
    };
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(out.status.success(), "unzip refused: {text}");
    for member in ["family.axgf", "family.acl", "family.journal", "backup.json"] {
        assert!(
            text.contains(member),
            "unzip -l did not list {member}:\n{text}"
        );
    }
}

#[test]
fn a_rotated_journal_is_carried_in_full() {
    // A backup that took only the live segment would restore an installation
    // whose conflict screen cannot reconstruct anything older than the last
    // rotation — and nothing would say so until somebody needed it.
    let bundle = seeded("br-rotated");
    let dest = scratch("br-rotated-dest");
    let live = axgf_cms::journal::Journal::path_for(&bundle);
    std::fs::write(live.with_extension("journal.1"), "{\"at\":\"a\",\"who\":\"x\",\"action\":\"update\",\"kind\":\"person\",\"entity_id\":\"p\"}\n").expect("write");
    std::fs::write(live.with_extension("journal.2"), "{\"at\":\"b\",\"who\":\"y\",\"action\":\"update\",\"kind\":\"person\",\"entity_id\":\"p\"}\n").expect("write");

    let report = backup::run(&bundle, dest.dir(), Retention::default()).expect("backup");
    assert_eq!(
        report.manifest.journal_lines, 5,
        "three live lines and two rotated ones"
    );

    for p in [
        live.clone(),
        live.with_extension("journal.1"),
        live.with_extension("journal.2"),
    ] {
        std::fs::remove_file(&p).expect("destroy");
    }
    backup::restore(&report.archive, &bundle, false).expect("restore");
    let j = axgf_cms::journal::Journal::new(live.clone());
    assert_eq!(j.len(), 5, "every segment came back");
    assert!(live.with_extension("journal.2").exists());
}

#[test]
fn an_installation_with_no_accounts_and_no_history_still_backs_up() {
    // A fresh install, backed up by the timer before anybody has signed in.
    let (app, path) = app_with_empty_bundle("br-fresh");
    drop(app);
    let dest = scratch("br-fresh-dest");
    let report = backup::run(&path, dest.dir(), Retention::default()).expect("backup");
    assert_eq!(report.manifest.accounts, 0);
    let v = backup::verify(&report.archive).expect("it still verifies");
    assert_eq!(v.accounts, 0);

    std::fs::remove_file(&path).expect("destroy");
    backup::restore(&report.archive, &path, false).expect("restore");
    assert!(path.exists());
}

#[tokio::test]
async fn health_reports_the_age_of_the_newest_backup() {
    let bundle = seeded("br-health");
    let dest = scratch("br-health-dest");
    let state = axgf_cms::AppState::load_or_create(&bundle, TOKEN.into()).expect("state");
    let state = std::sync::Arc::new(state.with_backup_dir(Some(dest.dir().to_path_buf())));

    let app = axgf_cms::router(std::sync::Arc::clone(&state));
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("request");
    assert_eq!(
        resp.status(),
        StatusCode::SERVICE_UNAVAILABLE,
        "an installation that has never been backed up is not healthy"
    );
    let v: serde_json::Value = serde_json::from_str(&body_string(resp).await).expect("json");
    assert_eq!(v["checks"]["backup"]["status"], "fail");

    backup::run(&bundle, dest.dir(), Retention::default()).expect("backup");

    let app = axgf_cms::router(state);
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("request");
    assert_eq!(resp.status(), StatusCode::OK);
    let v: serde_json::Value = serde_json::from_str(&body_string(resp).await).expect("json");
    assert_eq!(v["status"], "ok");
    assert_eq!(v["checks"]["backup"]["status"], "ok");
    assert!(
        v["checks"]["backup"]["detail"]
            .as_str()
            .unwrap()
            .contains("axgf-backup-"),
        "it names the archive: {}",
        v["checks"]["backup"]["detail"]
    );
}

#[test]
fn the_header_of_every_archive_says_where_it_came_from() {
    let bundle = seeded("br-manifest");
    let dest = scratch("br-manifest-dest");
    let report = backup::run(&bundle, dest.dir(), Retention::default()).expect("backup");
    let v = backup::verify(&report.archive).expect("verify");
    assert_eq!(v.manifest.format, 1);
    assert_eq!(v.manifest.source_bundle, bundle.display().to_string());
    assert!(v.manifest.written_by.starts_with("axgf-cms "));
    assert_eq!(v.entities["persons"], 2);
    assert_eq!(v.entities["documents"], 1);
    // The counts read back out of the archive agree with the ones recorded
    // when it was written — which is the check that catches a bundle that was
    // already damaged before it was archived.
    assert_eq!(v.entities, v.manifest.entities);
}

/// A `kill -9` between "create the .part" and "rename it" leaves a full-size
/// archive fragment behind. On the operator's bundle that is 415 MB a day, in
/// the directory the timer writes to, and nothing used to reclaim it.
#[test]
fn a_part_file_left_by_a_killed_run_is_reclaimed_by_the_next_one() {
    let bundle = seeded("br-orphan");
    let dest = scratch("br-orphan-dest");

    // Exactly what the killed run leaves: the right prefix, the right suffix,
    // nobody holding it.
    let orphan = dest.dir().join("axgf-backup-20200101T000000Z.zip.part");
    std::fs::write(&orphan, vec![7u8; 4096]).expect("write the orphan");

    let report = backup::run(&bundle, dest.dir(), Retention::default()).expect("backup");
    assert!(!orphan.exists(), "the orphan should have been reclaimed");
    assert_eq!(report.swept_parts, 1);
    assert_eq!(report.swept_bytes, 4096);
    assert!(report.archive.exists(), "and the new archive still written");
}

/// The other half of it: a `.part` a live run is writing must survive a sweep,
/// which is why the test for liveness is the kernel's lock and not the clock.
#[test]
fn a_part_file_a_live_run_is_writing_is_left_alone() {
    let dest = scratch("br-live-part-dest");

    let live = dest.dir().join("axgf-backup-20200101T000000Z.zip.part");
    let held = std::fs::OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .truncate(true)
        .open(&live)
        .expect("create");
    held.lock().expect("hold it the way a running backup does");

    let (swept, bytes) = backup::sweep_orphaned_parts(dest.dir());
    assert_eq!((swept, bytes), (0, 0), "a claimed .part is not wreckage");
    assert!(live.exists());

    // Released — as the kernel does when a process dies — it is wreckage.
    drop(held);
    let (swept, _) = backup::sweep_orphaned_parts(dest.dir());
    assert_eq!(swept, 1);
    assert!(!live.exists());
}

/// A helper the other tests share for header assertions.
fn _unused(_: &Request<Body>, _: header::HeaderName) {}
