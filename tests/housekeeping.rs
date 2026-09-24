//! The test suite tidies up after itself.
//!
//! # Why this is a test and not a note in CONTRIBUTING
//!
//! `CARGO_TARGET_TMPDIR` is `target/tmp`, and cargo creates it and then never
//! looks at it again. Every integration test here made a directory in it,
//! wrote a bundle — sometimes with photographs in it — and left. On the
//! machine this was written for that had reached **8.6 GB**, on the same
//! filesystem the application saves its own bundle to. The test suite was
//! quietly arranging for the product to run out of disk.
//!
//! The fix is in `tests/common/mod.rs`: `scratch()` hands back a guard that
//! removes the directory when it is dropped, including on a panic. This file
//! is what stops that quietly coming undone — a helper that goes back to
//! returning a bare `PathBuf` fails here, not in six months on somebody's
//! disk.

mod common;

use std::path::{Path, PathBuf};

use common::*;

#[test]
fn the_harness_removes_the_directory_it_made() {
    let dir;
    {
        let s = scratch("housekeeping-basic");
        dir = s.dir().to_path_buf();
        std::fs::write(dir.join("something.bin"), vec![0u8; 4096]).expect("write");
        assert!(dir.exists());
    }
    assert!(
        !dir.exists(),
        "{} outlived the guard that owns it",
        dir.display()
    );
}

#[test]
fn a_test_that_panics_still_leaves_nothing_behind() {
    // The case the old `remove_dir_all` at the bottom of a test body never
    // covered, and the one that matters most: a failing test is the one most
    // likely to have written a great deal before it failed.
    let recorded = std::sync::Arc::new(std::sync::Mutex::new(PathBuf::new()));
    let sink = std::sync::Arc::clone(&recorded);
    let outcome = std::panic::catch_unwind(move || {
        let s = scratch("housekeeping-panic");
        *sink.lock().unwrap() = s.dir().to_path_buf();
        std::fs::write(s.dir().join("half-written.axgf"), vec![7u8; 8192]).unwrap();
        panic!("as a failing test would");
    });
    assert!(outcome.is_err(), "the closure was supposed to panic");
    let dir = recorded.lock().unwrap().clone();
    assert!(!dir.as_os_str().is_empty());
    assert!(
        !dir.exists(),
        "the unwind did not take {} with it",
        dir.display()
    );
}

#[test]
fn a_fixture_that_hands_its_path_on_keeps_the_directory_alive() {
    // The other half of the contract: a guard is worthless if it deletes the
    // directory the moment a fixture returns the bundle inside it.
    let (app, path) = app_with_empty_bundle("housekeeping-alive");
    assert!(path.exists(), "the bundle a fixture just built");
    let dir = path.parent().unwrap().to_path_buf();
    drop(app);
    assert!(path.exists(), "still there while the test holds the guard");
    drop(path);
    assert!(!dir.exists(), "and gone once it does not");
}

#[test]
fn no_earlier_run_left_anything_behind() {
    // Directories are named `<prefix>-<pid>-…`. Anything whose process is
    // gone is wreckage: the guard did not run, or somebody reintroduced a
    // helper that returns a bare path.
    let mut wreckage = Vec::new();
    let mut bytes = 0u64;
    for base in [scratch_base(), std::env::temp_dir()] {
        let Ok(entries) = std::fs::read_dir(&base) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }
            let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
                continue;
            };
            let Some(pid) = owning_pid(name) else {
                continue;
            };
            if pid == std::process::id() || alive(pid) {
                continue; // ours, or another test binary still running
            }
            bytes += dir_bytes(&path);
            wreckage.push(path);
        }
    }
    assert!(
        wreckage.is_empty(),
        "a previous test run left {} director{} behind ({} bytes). \
         Something stopped holding a `Scratch` guard.\n{}\n\nClear them with:\n  rm -rf {}",
        wreckage.len(),
        if wreckage.len() == 1 { "y" } else { "ies" },
        bytes,
        wreckage
            .iter()
            .take(10)
            .map(|p| format!("  {}", p.display()))
            .collect::<Vec<_>>()
            .join("\n"),
        wreckage
            .iter()
            .map(|p| p.display().to_string())
            .collect::<Vec<_>>()
            .join(" "),
    );
}

/// `axgf-cms-it-12345-tag-0` → `12345`. `None` for anything not ours.
fn owning_pid(name: &str) -> Option<u32> {
    for prefix in ["axgf-cms-it-", "axgf-unit-", "axgf-cms-state-"] {
        if let Some(rest) = name.strip_prefix(prefix) {
            return rest.split('-').next()?.parse().ok();
        }
    }
    None
}

/// Is that process still running? `/proc` is the cheapest answer on the one
/// platform this suite runs on; elsewhere the check simply does not fire.
fn alive(pid: u32) -> bool {
    if cfg!(target_os = "linux") {
        Path::new(&format!("/proc/{pid}")).exists()
    } else {
        true
    }
}

fn dir_bytes(dir: &Path) -> u64 {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return 0;
    };
    entries
        .flatten()
        .map(|e| {
            let p = e.path();
            if p.is_dir() {
                dir_bytes(&p)
            } else {
                e.metadata().map(|m| m.len()).unwrap_or(0)
            }
        })
        .sum()
}

#[test]
fn nothing_makes_a_scratch_directory_except_the_guards() {
    // `no_earlier_run_left_anything_behind` only recognises the prefixes it
    // was told about, and that is exactly how the proxy test's own
    // `axgf-proxy-<pid>` directory went unnoticed: it returned a bare path, an
    // early `return` skipped the `remove_dir_all` at the bottom, and the leak
    // check did not know the name. So the rule is enforced where it is
    // broken — in the source. Only the two guard modules may ask for a
    // temporary directory; everything else goes through them.
    const ALLOWED: [&str; 2] = ["tests/common/mod.rs", "src/lib.rs"];
    const NEEDLES: [&str; 3] = ["CARGO_TARGET_TMPDIR", "temp_dir()", "tempfile::"];
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut offenders = Vec::new();
    for dir in ["src", "tests"] {
        for file in rust_files(&root.join(dir)) {
            let rel = file
                .strip_prefix(root)
                .unwrap()
                .to_string_lossy()
                .replace('\\', "/");
            if ALLOWED.contains(&rel.as_str()) || rel == "tests/housekeeping.rs" {
                continue;
            }
            let text = std::fs::read_to_string(&file).unwrap_or_default();
            for (n, line) in text.lines().enumerate() {
                let code = line.split("//").next().unwrap_or("");
                if NEEDLES.iter().any(|needle| code.contains(needle)) {
                    offenders.push(format!("  {rel}:{}: {}", n + 1, line.trim()));
                }
            }
        }
    }
    assert!(
        offenders.is_empty(),
        "these make a temporary directory without a guard that removes it; \
         use `common::scratch` (integration tests) or `crate::scratch::Dir` \
         (unit tests):\n{}",
        offenders.join("\n")
    );
}

fn rust_files(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let Ok(entries) = std::fs::read_dir(dir) else {
        return out;
    };
    for entry in entries.flatten() {
        let p = entry.path();
        if p.is_dir() {
            out.extend(rust_files(&p));
        } else if p.extension().is_some_and(|e| e == "rs") {
            out.push(p);
        }
    }
    out
}
