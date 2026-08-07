//! `deploy/bootstrap.sh` must be idempotent.
//!
//! The requirement is explicit: running it twice must not destroy an existing
//! bundle and must not regenerate the admin token. Both would be silent
//! disasters — the first loses the entire database, the second locks the
//! operator out of a working install.
//!
//! The script installs into `$AXGF_CMS_PREFIX`, so these tests run it twice
//! against a scratch directory with no root and no systemd.

mod common;

use std::path::{Path, PathBuf};
use std::process::Command;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// Run bootstrap.sh into `prefix`, returning its stdout.
fn run_bootstrap(prefix: &Path, extra: &[&str]) -> String {
    let script = repo_root().join("deploy/bootstrap.sh");
    // Any executable file will do: these tests exercise the script's logic,
    // not the binary it installs.
    let stand_in = prefix.join("stand-in-binary");
    if !stand_in.exists() {
        std::fs::write(&stand_in, "#!/bin/sh\nexit 0\n").expect("write stand-in");
    }

    let out = Command::new("bash")
        .arg(&script)
        .args(extra)
        .env("AXGF_CMS_PREFIX", prefix)
        .env("AXGF_CMS_SKIP_PRIVILEGED", "1")
        .env("AXGF_CMS_LOCAL_BINARY", &stand_in)
        .output()
        .expect("run bootstrap.sh");

    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    assert!(
        out.status.success(),
        "bootstrap.sh failed:\n--- stdout ---\n{stdout}\n--- stderr ---\n{stderr}"
    );
    stdout
}

fn token_in(prefix: &Path) -> String {
    let env_file = prefix.join("etc/axgf-cms/env");
    let text = std::fs::read_to_string(&env_file)
        .unwrap_or_else(|e| panic!("read {}: {e}", env_file.display()));
    text.lines()
        .find_map(|l| l.strip_prefix("AXGF_CMS_ADMIN_TOKEN="))
        .expect("env file should define the token")
        .trim()
        .to_string()
}

#[test]
fn bootstrap_dry_run_changes_nothing() {
    let prefix = common::scratch("boot-dry");
    let out = run_bootstrap(&prefix, &["--dry-run", "--with-sample"]);

    assert!(out.contains("[dry-run]"), "a dry run should say so");
    assert!(
        !prefix.join("etc/axgf-cms/env").exists(),
        "no env file written"
    );
    assert!(
        !prefix.join("var/lib/axgf-cms/family.axgf").exists(),
        "no bundle written"
    );
    assert!(
        !prefix.join("etc/systemd/system/axgf-cms.service").exists(),
        "no unit written"
    );
}

#[test]
fn running_bootstrap_twice_keeps_the_token_and_the_bundle() {
    let prefix = common::scratch("boot-twice");

    // --- first run -------------------------------------------------------
    let first = run_bootstrap(&prefix, &["--with-sample"]);
    assert!(first.contains("generated a new admin token"));
    assert!(first.contains("seeded with the demonstration family"));

    let token1 = token_in(&prefix);
    assert_eq!(token1.len(), 64, "32 random bytes as hex");

    let unit = prefix.join("etc/systemd/system/axgf-cms.service");
    assert!(unit.exists(), "the unit should be installed");
    let unit_text = std::fs::read_to_string(&unit).expect("read unit");
    assert!(
        unit_text.contains("--seed-sample"),
        "the seed flag is passed"
    );
    assert!(unit_text.contains("Restart=on-failure"));
    assert!(unit_text.contains("127.0.0.1:8080"), "binds to localhost");

    // The service would create the bundle on start; simulate that, then check
    // the second run leaves it alone.
    let bundle = prefix.join("var/lib/axgf-cms/family.axgf");
    std::fs::create_dir_all(bundle.parent().unwrap()).expect("mkdir");
    std::fs::write(&bundle, b"PK\x03\x04 pretend bundle contents").expect("write bundle");
    let bundle_before = std::fs::read(&bundle).expect("read bundle");

    // --- second run ------------------------------------------------------
    let second = run_bootstrap(&prefix, &["--with-sample"]);

    assert!(
        second.contains("keeping the existing token"),
        "a second run must not regenerate the token: {second}"
    );
    assert_eq!(
        token_in(&prefix),
        token1,
        "the admin token must survive a re-run"
    );

    assert!(
        second.contains("leaving it untouched"),
        "a second run must not touch the bundle: {second}"
    );
    assert_eq!(
        std::fs::read(&bundle).expect("read bundle after"),
        bundle_before,
        "the bundle is the entire database and must survive byte-identical"
    );

    // With a bundle present, seeding must be switched off so the running
    // service cannot be pointed at sample data.
    let unit_text2 = std::fs::read_to_string(&unit).expect("read unit again");
    assert!(
        !unit_text2.contains("--seed-sample"),
        "--with-sample must be ignored once a bundle exists: {unit_text2}"
    );
    assert!(
        second.contains("ignored"),
        "and the script should say it is being ignored"
    );
}

#[test]
fn bootstrap_reports_the_token_and_the_security_position() {
    let prefix = common::scratch("boot-report");
    let out = run_bootstrap(&prefix, &[]);
    let token = token_in(&prefix);

    assert!(out.contains(&token), "the token is printed once at the end");
    assert!(out.contains("/admin/login"), "the admin URL is printed");
    assert!(
        out.contains("no user accounts"),
        "the V1 limitation must be stated at install time"
    );
    assert!(
        out.contains("back it up"),
        "the operator is told what the bundle is"
    );
}

#[test]
fn a_custom_bind_address_reaches_the_unit() {
    let prefix = common::scratch("boot-bind");
    run_bootstrap(&prefix, &["--bind", "127.0.0.1:9999"]);
    let unit = std::fs::read_to_string(prefix.join("etc/systemd/system/axgf-cms.service"))
        .expect("read unit");
    assert!(unit.contains("--bind 127.0.0.1:9999"), "{unit}");
}

#[test]
fn the_unit_confines_the_service_to_its_data_directory() {
    let prefix = common::scratch("boot-harden");
    run_bootstrap(&prefix, &[]);
    let unit = std::fs::read_to_string(prefix.join("etc/systemd/system/axgf-cms.service"))
        .expect("read unit");

    for directive in [
        "User=axgf-cms",
        "NoNewPrivileges=yes",
        "ProtectSystem=strict",
        "ProtectHome=yes",
        "ReadWritePaths=",
    ] {
        assert!(
            unit.contains(directive),
            "unit is missing {directive}:\n{unit}"
        );
    }
}
