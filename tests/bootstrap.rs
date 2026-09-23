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
    assert!(first.contains("with a new admin token"));
    assert!(first.contains("seeded with the demonstration family"));

    let token1 = token_in(&prefix);
    assert_eq!(token1.len(), 64, "32 random bytes as hex");

    let unit = prefix.join("etc/systemd/system/axgf-cms.service");
    assert!(unit.exists(), "the unit should be installed");
    let unit_text = std::fs::read_to_string(&unit).expect("read unit");
    // The unit must NOT carry --seed-sample. Seeding is a one-off that happens
    // before anything is running; leaving the flag on a long-lived service is
    // what let the service and the create-admin step race for the same file,
    // and the empty bundle that came out of that race is the reason this
    // assertion is inverted from what it used to say.
    assert!(
        !unit_text.contains("--seed-sample"),
        "seeding is done before the service starts, not by it"
    );
    assert!(unit_text.contains("Restart=on-failure"));
    // The bind address is in the environment file, not in the unit — but it
    // still has to be localhost by default, which is where that now reads.
    let env_text = std::fs::read_to_string(prefix.join("etc/axgf-cms/env")).expect("read env");
    assert!(
        env_text.contains("AXGF_CMS_BIND=127.0.0.1:8080"),
        "binds to localhost by default:\n{env_text}"
    );

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
fn bootstrap_reports_the_accounts_and_the_security_position() {
    let prefix = common::scratch("boot-report");
    let out = run_bootstrap(&prefix, &[]);
    let token = token_in(&prefix);

    assert!(out.contains(&token), "the token is printed once at the end");
    assert!(out.contains("/admin/login"), "the sign-in URL is printed");
    assert!(
        out.contains("the genealogy; share it freely"),
        "the operator is told what the bundle is"
    );
    // Backups are no longer an instruction in a one-line summary but an
    // installed timer, and the summary has to say where the archives land and
    // that leaving them there is not enough.
    assert!(
        out.contains("Backups"),
        "the backup directory is named: {out}"
    );
    assert!(
        out.contains("does not survive that disk"),
        "an archive beside the bundle is not a backup, and this has to say so: {out}"
    );
    assert!(
        out.contains("rsync -a"),
        "with a command the operator can actually copy: {out}"
    );

    // The two files, and which of them may be shared. Getting this the wrong
    // way round is how a credential store ends up mailed to a relative.
    assert!(out.contains(".acl"), "the accounts file is named: {out}");
    assert!(
        out.contains("share it with nobody"),
        "and the operator is told not to share it: {out}"
    );

    // The emergency token is no longer *the* authentication system, and the
    // summary has to stop describing it as one.
    assert!(
        !out.contains("no user accounts"),
        "the V1 limitation is no longer true and must not still be claimed"
    );
    assert!(
        out.contains("not an account"),
        "the token's new role is stated: {out}"
    );

    // The default that will surprise an operator publishing a converted
    // GEDCOM: it carries no visibility at all, so the rule that fills the gap
    // has to be stated where they will read it.
    assert!(
        out.contains("carries no visibility"),
        "the GEDCOM visibility default is stated at install time: {out}"
    );
}

#[test]
fn every_setting_is_in_the_environment_file_and_none_is_on_the_command_line() {
    // The whole contract of this install: one file holds the configuration,
    // and `ExecStart` is the binary. An operator who moves the bundle, changes
    // the port or turns the logging up edits one line and restarts — there is
    // no unit to keep in step, and no flag that can disagree with the file.
    let prefix = common::scratch("boot-envfile");
    run_bootstrap(&prefix, &["--bind", "127.0.0.1:9999"]);

    let env = std::fs::read_to_string(prefix.join("etc/axgf-cms/env")).expect("read env file");
    for key in [
        "AXGF_CMS_BUNDLE=",
        "AXGF_CMS_BIND=127.0.0.1:9999",
        "AXGF_CMS_BACKUP_DIR=",
        "AXGF_CMS_CACHE_DIR=",
        "RUST_LOG=",
        "AXGF_CMS_ADMIN_TOKEN=",
    ] {
        assert!(env.contains(key), "the env file is missing {key}:\n{env}");
    }

    let unit = std::fs::read_to_string(prefix.join("etc/systemd/system/axgf-cms.service"))
        .expect("read unit");
    let exec = unit
        .lines()
        .find(|l| l.starts_with("ExecStart="))
        .expect("the unit starts something");
    assert!(
        !exec.contains("--"),
        "ExecStart must carry no flags at all, got: {exec}"
    );
    assert!(
        unit.contains("EnvironmentFile="),
        "and it must read the environment file:\n{unit}"
    );
}

#[test]
fn an_older_installation_gains_the_settings_it_never_had() {
    // An env file written before the settings moved into it holds only the
    // token. Re-running must add what is missing and keep the token, rather
    // than rewrite the file over somebody's own edits.
    let prefix = common::scratch("boot-envfile-old");
    std::fs::create_dir_all(prefix.join("etc/axgf-cms")).expect("mkdir");
    std::fs::write(
        prefix.join("etc/axgf-cms/env"),
        "AXGF_CMS_ADMIN_TOKEN=an-old-token\n# an operator's own note\n",
    )
    .expect("write old env file");

    run_bootstrap(&prefix, &[]);

    let env = std::fs::read_to_string(prefix.join("etc/axgf-cms/env")).expect("read env file");
    assert!(
        env.contains("AXGF_CMS_ADMIN_TOKEN=an-old-token"),
        "the token must survive:\n{env}"
    );
    assert!(
        env.contains("an operator's own note"),
        "and so must their comments:\n{env}"
    );
    for key in ["AXGF_CMS_BUNDLE=", "AXGF_CMS_BIND=", "RUST_LOG="] {
        assert!(env.contains(key), "missing {key}:\n{env}");
    }
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
        "PrivateTmp=yes",
        "PrivateDevices=yes",
        "PrivateUsers=yes",
        "ProtectSystem=strict",
        "ProtectHome=yes",
        "ProtectKernelTunables=yes",
        "ProtectKernelModules=yes",
        "ProtectKernelLogs=yes",
        "ProtectControlGroups=yes",
        "ProtectClock=yes",
        "ProtectHostname=yes",
        "ProtectProc=invisible",
        "ProcSubset=pid",
        "RestrictNamespaces=yes",
        "RestrictRealtime=yes",
        "RestrictSUIDSGID=yes",
        "RemoveIPC=yes",
        "LockPersonality=yes",
        "MemoryDenyWriteExecute=yes",
        "SystemCallArchitectures=native",
        "SystemCallFilter=@system-service",
        "CapabilityBoundingSet=",
        "UMask=0077",
        "ReadWritePaths=",
    ] {
        assert!(
            unit.contains(directive),
            "unit is missing {directive}:\n{unit}"
        );
    }

    // AF_UNIX is the one that cannot be dropped: journald's socket and
    // systemd's own readiness protocol both go through it. A unit that lists
    // only the INET families starts, logs nothing, and hangs `systemctl start`
    // until the timeout.
    let families = unit
        .lines()
        .find(|l| l.starts_with("RestrictAddressFamilies="))
        .expect("the unit restricts address families");
    for family in ["AF_UNIX", "AF_INET", "AF_INET6"] {
        assert!(families.contains(family), "{families}");
    }

    // Readiness, and a restart that backs off rather than hammering a disk
    // that is already full.
    // Tightened past @system-service, and refused any address but the
    // loopback the proxy talks to it over. Both were tested by exercising the
    // product under them, which is the only way to know a filter is not
    // quietly breaking a save.
    assert!(unit.contains("SystemCallFilter=~@privileged"), "{unit}");
    assert!(unit.contains("IPAddressDeny=any"), "{unit}");
    assert!(unit.contains("IPAddressAllow=localhost"), "{unit}");

    assert!(unit.contains("Type=notify"), "{unit}");
    assert!(unit.contains("Restart=on-failure"), "{unit}");
    assert!(unit.contains("RestartSteps="), "{unit}");
    assert!(unit.contains("RestartMaxDelaySec="), "{unit}");
}

#[test]
fn a_daily_backup_timer_is_installed_beside_the_service() {
    // An installation with no backups is one disk failure from losing years of
    // somebody's research, and "remember to run the backup command" is not a
    // backup strategy. The timer is part of the install, not an extra step.
    let prefix = common::scratch("boot-timer");
    run_bootstrap(&prefix, &[]);

    let timer = std::fs::read_to_string(prefix.join("etc/systemd/system/axgf-cms-backup.timer"))
        .expect("the timer is installed");
    assert!(timer.contains("OnCalendar=03:30"), "{timer}");
    assert!(
        timer.contains("Persistent=true"),
        "a machine switched off at half past three must catch up: {timer}"
    );
    assert!(
        timer.contains("RandomizedDelaySec="),
        "and not all wake at the same instant: {timer}"
    );

    let service =
        std::fs::read_to_string(prefix.join("etc/systemd/system/axgf-cms-backup.service"))
            .expect("the timer's service is installed");
    assert!(service.contains("Type=oneshot"), "{service}");
    // The same one file as the service, and the same bare command: the bundle
    // and the destination come from the environment, so an operator who moves
    // either does not have to remember that a second unit names them too.
    assert!(service.contains("EnvironmentFile="), "{service}");
    assert!(
        service.lines().any(|l| l.trim()
            == format!(
                "ExecStart={}/usr/local/bin/axgf-cms backup",
                prefix.display()
            )),
        "the backup command takes no flags:\n{service}"
    );
    assert!(
        service.contains("User=axgf-cms"),
        "the archive holds the ACL; it is written by the service user: {service}"
    );
    // Confined the way the server is. A backup job has even less business
    // reaching the rest of the filesystem than the server does.
    for directive in [
        "NoNewPrivileges=yes",
        "ProtectSystem=strict",
        "ProtectHome=yes",
        "RestrictAddressFamilies=",
    ] {
        assert!(
            service.contains(directive),
            "missing {directive}:\n{service}"
        );
    }
}

#[test]
fn the_service_is_told_where_the_backups_are_so_health_can_say_how_old_they_are() {
    let prefix = common::scratch("boot-backupdir");
    // Somewhere other than the default, and writable by this test: an operator
    // would give an absolute path on another volume, which is the whole point
    // of the option.
    let elsewhere = prefix.join("srv-elsewhere");
    run_bootstrap(&prefix, &["--backup-dir", elsewhere.to_str().unwrap()]);
    let env = std::fs::read_to_string(prefix.join("etc/axgf-cms/env")).expect("read env file");
    assert!(
        env.contains(&format!("AXGF_CMS_BACKUP_DIR={}", elsewhere.display())),
        "an installation whose timer stopped firing must not look identical to \
         one whose timer is working:\n{env}"
    );
    let unit = std::fs::read_to_string(prefix.join("etc/systemd/system/axgf-cms.service"))
        .expect("read unit");
    assert!(
        unit.lines()
            .any(|l| l.starts_with("ReadWritePaths=") && l.contains("srv-elsewhere")),
        "and the sandbox has to let it write them:\n{unit}"
    );
    assert!(
        elsewhere.is_dir(),
        "the directory is created by the install"
    );
}

#[test]
fn the_backup_hour_is_the_operators_to_choose() {
    let prefix = common::scratch("boot-at");
    run_bootstrap(&prefix, &["--backup-at", "Mon *-*-* 04:15:00"]);
    let timer = std::fs::read_to_string(prefix.join("etc/systemd/system/axgf-cms-backup.timer"))
        .expect("read timer");
    assert!(timer.contains("OnCalendar=Mon *-*-* 04:15:00"), "{timer}");
}

#[test]
fn upgrading_something_that_is_not_installed_is_refused_rather_than_installed() {
    let prefix = common::scratch("boot-upgrade-none");
    let script = repo_root().join("deploy/bootstrap.sh");
    let out = Command::new("bash")
        .arg(&script)
        .arg("--upgrade")
        .env("AXGF_CMS_PREFIX", &prefix)
        .env("AXGF_CMS_SKIP_PRIVILEGED", "1")
        .output()
        .expect("run bootstrap.sh");
    let text = String::from_utf8_lossy(&out.stderr);
    assert!(!out.status.success(), "there is nothing to upgrade");
    assert!(
        text.contains("nothing to upgrade") || text.contains("nothing is installed"),
        "{text}"
    );
}

/// Run bootstrap.sh with the *real* binary rather than the stand-in, so the
/// steps that actually invoke it are exercised.
fn run_bootstrap_for_real(prefix: &Path, extra: &[&str]) -> String {
    let script = repo_root().join("deploy/bootstrap.sh");
    let out = Command::new("bash")
        .arg(&script)
        .args(extra)
        .env("AXGF_CMS_PREFIX", prefix)
        .env("AXGF_CMS_SKIP_PRIVILEGED", "1")
        .env("AXGF_CMS_LOCAL_BINARY", env!("CARGO_BIN_EXE_axgf-cms"))
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

#[test]
fn a_fresh_install_gets_an_administrator_and_a_second_run_does_not_rotate_it() {
    // The whole point of creating the first account from the shell rather than
    // from a web setup page: the window between deploying and the first login
    // is exactly when an installation is unprotected. So a fresh install must
    // come up *with* an administrator, not waiting for someone to make one.
    let prefix = common::scratch("boot-admin");
    let out = run_bootstrap_for_real(&prefix, &["--admin-user", "karin"]);

    let acl = prefix.join("var/lib/axgf-cms/family.acl");
    assert!(acl.exists(), "a fresh install creates the .acl: {out}");

    // Mode 600. A credential store any local user can read is not a
    // credential store.
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        let mode = std::fs::metadata(&acl)
            .expect("stat acl")
            .permissions()
            .mode()
            & 0o777;
        assert_eq!(mode, 0o600, "the .acl must not be readable by anyone else");
    }

    let text = std::fs::read_to_string(&acl).expect("read acl");
    let parsed: serde_json::Value = serde_json::from_str(&text).expect("the .acl is JSON");
    assert_eq!(parsed["users"][0]["username"], "karin");
    assert_eq!(parsed["users"][0]["role"], "admin");
    assert!(
        parsed["users"][0]["password_hash"]
            .as_str()
            .is_some_and(|h| h.starts_with("$argon2id$")),
        "the password is stored as an Argon2id hash, never reversibly"
    );

    // The password is printed once, in the summary, and is not in the file.
    let password = out
        .lines()
        .find_map(|l| l.trim().strip_prefix("password     "))
        .expect("the generated password is printed once")
        .trim()
        .to_string();
    assert!(password.len() >= 12);
    assert!(
        !text.contains(&password),
        "the plaintext password must not be written anywhere"
    );

    // Re-running must not rotate it: a bootstrap script gets run again on
    // every deploy, and silently changing a working account's password would
    // lock the family out.
    let again = run_bootstrap_for_real(&prefix, &["--admin-user", "karin"]);
    let text2 = std::fs::read_to_string(&acl).expect("read acl again");
    assert_eq!(text, text2, "the accounts file is left alone on a re-run");
    assert!(
        again.contains("leaving the existing accounts alone"),
        "and the script says so: {again}"
    );
    assert!(
        !again.contains("password     "),
        "a re-run prints no password, because it created no account"
    );
}

#[test]
fn with_sample_seeds_a_family_a_visitor_can_actually_see() {
    // The bug this exists for: bootstrap started the service, which began
    // seeding the sample, then stopped it a fraction of a second later so the
    // ACL could be written. The create-admin invocation found no bundle yet
    // and made an empty one, so a fresh --with-sample install served a
    // signed-out visitor "0 of 0 people". Everything is created before
    // anything runs now, and the assertion is on the bundle's contents rather
    // than on the ordering, so a future reshuffle has to keep the result.
    let prefix = common::scratch("boot-sample");
    let out = run_bootstrap_for_real(&prefix, &["--with-sample"]);

    let bundle = prefix.join("var/lib/axgf-cms/family.axgf");
    assert!(bundle.exists(), "a bundle is created: {out}");

    let bytes = std::fs::read(&bundle).expect("read seeded bundle");
    let env = axgf_rs::import_bundle(&bytes);
    let persons = env
        .data
        .get("persons")
        .and_then(|p| p.as_object())
        .map(|p| p.len())
        .unwrap_or(0);
    assert!(
        persons >= 10,
        "--with-sample must seed the demonstration family, found {persons} persons"
    );

    // And they must be visible to somebody who is not signed in, or the first
    // impression of a fresh install is a blank page.
    let public = env
        .data
        .get("persons")
        .and_then(|p| p.as_object())
        .map(|p| {
            p.values()
                .filter(|v| {
                    axgf_cms::access::person_visibility(v) == axgf_cms::acl::Visibility::Public
                })
                .count()
        })
        .unwrap_or(0);
    assert_eq!(
        public, persons,
        "every person in the sample is explicitly public; a visitor sees a tree"
    );
}

// ---------------------------------------------------------------------------
// The download path.
//
// Until now nothing exercised it: every test above hands the script a binary
// through AXGF_CMS_LOCAL_BINARY, so the branch that fetches a release had only
// ever been read, never run. These tests point AXGF_CMS_RELEASE_BASE and
// AXGF_CMS_API_BASE at a staged file:// mirror and run it for real.
// ---------------------------------------------------------------------------

/// The release target triple the script derives from `uname -m`.
fn target_triple() -> &'static str {
    match std::env::consts::ARCH {
        "x86_64" => "x86_64-unknown-linux-musl",
        "aarch64" => "aarch64-unknown-linux-gnu",
        other => panic!("unsupported test architecture {other}"),
    }
}

struct Outcome {
    ok: bool,
    out: String,
}

impl Outcome {
    fn says(&self, needle: &str) -> bool {
        self.out.contains(needle)
    }
}

/// Run bootstrap.sh against a staged mirror, letting it fail.
fn run_against_mirror(prefix: &Path, mirror: &Path, api: &Path, extra: &[&str]) -> Outcome {
    let out = Command::new("bash")
        .arg(repo_root().join("deploy/bootstrap.sh"))
        .args(extra)
        .env("AXGF_CMS_PREFIX", prefix)
        .env("AXGF_CMS_SKIP_PRIVILEGED", "1")
        .env(
            "AXGF_CMS_RELEASE_BASE",
            format!("file://{}", mirror.display()),
        )
        .env("AXGF_CMS_API_BASE", format!("file://{}", api.display()))
        // Explicitly empty: this is the branch under test.
        .env("AXGF_CMS_LOCAL_BINARY", "")
        .output()
        .expect("run bootstrap.sh");
    Outcome {
        ok: out.status.success(),
        out: String::from_utf8_lossy(&out.stdout).into_owned()
            + &String::from_utf8_lossy(&out.stderr),
    }
}

/// Stage `api/releases` holding `json`, the way the GitHub API would answer.
fn stage_api(prefix: &Path, json: &str) -> PathBuf {
    let api = prefix.join("api");
    std::fs::create_dir_all(&api).expect("mkdir api");
    std::fs::write(api.join("releases"), json).expect("write releases");
    api
}

/// Stage a release asset for `tag`, packaged exactly as the workflow packages
/// it: `axgf-cms-<tag>-<target>.tar.gz` with a `.sha256` sidecar beside it.
fn stage_release(prefix: &Path, tag: &str) -> PathBuf {
    let mirror = prefix.join("mirror");
    let dir = mirror.join("releases/download").join(tag);
    std::fs::create_dir_all(&dir).expect("mkdir release dir");

    let stage = prefix.join(format!("axgf-cms-{tag}-{}", target_triple()));
    std::fs::create_dir_all(&stage).expect("mkdir stage");
    std::fs::write(stage.join("axgf-cms"), "#!/bin/sh\nexit 0\n").expect("write binary");

    let name = format!("axgf-cms-{tag}-{}.tar.gz", target_triple());
    let sh = format!(
        "set -eu; cd {p}; tar -czf {d}/{n} {s}; cd {d}; sha256sum {n} > {n}.sha256",
        p = prefix.display(),
        d = dir.display(),
        n = name,
        s = stage.file_name().unwrap().to_string_lossy(),
    );
    let st = Command::new("bash")
        .arg("-c")
        .arg(&sh)
        .status()
        .expect("package the staged release");
    assert!(st.success(), "packaging failed");
    mirror
}

/// A dry run with no locally supplied binary, so the download branch is the
/// one that reports what it would fetch.
fn dry_run_download(prefix: &Path, extra: &[&str]) -> String {
    let mut args = vec!["--dry-run"];
    args.extend_from_slice(extra);
    let out = Command::new("bash")
        .arg(repo_root().join("deploy/bootstrap.sh"))
        .args(&args)
        .env("AXGF_CMS_PREFIX", prefix)
        .env("AXGF_CMS_SKIP_PRIVILEGED", "1")
        .env("AXGF_CMS_LOCAL_BINARY", "")
        .output()
        .expect("run bootstrap.sh");
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8_lossy(&out.stdout).into_owned()
}

#[test]
fn the_default_asks_for_latest_and_a_tag_asks_for_that_tag() {
    // The URL is the whole of the fix: /releases/latest resolves stable
    // releases only, so a release candidate is unreachable without --version.
    let prefix = common::scratch("boot-url");
    let out = dry_run_download(&prefix, &[]);
    assert!(
        out.contains(&format!(
            "releases/latest/download/axgf-cms-{}.tar.gz",
            target_triple()
        )),
        "the default is still latest: {out}"
    );

    let prefix = common::scratch("boot-url-tag");
    let out = dry_run_download(&prefix, &["--version", "v0.1.0-rc1"]);
    assert!(
        out.contains(&format!(
            "releases/download/v0.1.0-rc1/axgf-cms-v0.1.0-rc1-{}.tar.gz",
            target_triple()
        )),
        "--version addresses the tagged asset: {out}"
    );
}

#[test]
fn help_lists_every_option_the_script_accepts() {
    // --help printed a fixed line range of the header comment, so growing the
    // header truncated it. Every option the parser accepts has to appear.
    let out = Command::new("bash")
        .arg(repo_root().join("deploy/bootstrap.sh"))
        .arg("--help")
        .output()
        .expect("run bootstrap.sh --help");
    assert!(out.status.success());
    let help = String::from_utf8_lossy(&out.stdout);
    for option in [
        "--with-sample",
        "--from-source",
        "--version",
        "--bind",
        "--admin-user",
        "--dry-run",
    ] {
        assert!(
            help.contains(option),
            "--help does not mention {option}:\n{help}"
        );
    }
    assert!(
        help.contains("IDEMPOTENT"),
        "the help runs to the end of the header:\n{help}"
    );
}

#[test]
fn a_version_with_no_tag_is_refused_rather_than_silently_meaning_latest() {
    let prefix = common::scratch("boot-noval");
    let out = Command::new("bash")
        .arg(repo_root().join("deploy/bootstrap.sh"))
        .args(["--dry-run", "--version"])
        .env("AXGF_CMS_PREFIX", &prefix)
        .env("AXGF_CMS_SKIP_PRIVILEGED", "1")
        .output()
        .expect("run bootstrap.sh");
    let text = String::from_utf8_lossy(&out.stderr);
    assert!(!out.status.success(), "an empty tag must not be accepted");
    assert!(text.contains("--version needs a value"), "{text}");
}

#[test]
fn a_tagged_release_is_downloaded_and_its_checksum_verified() {
    let prefix = common::scratch("boot-dl");
    let mirror = stage_release(&prefix, "v0.1.0-rc1");
    let api = stage_api(
        &prefix,
        r#"[{"tag_name": "v0.1.0-rc1", "prerelease": true}]"#,
    );

    let r = run_against_mirror(&prefix, &mirror, &api, &["--version", "v0.1.0-rc1"]);
    assert!(r.ok, "install should succeed:\n{}", r.out);
    assert!(r.says("checksum verified"), "{}", r.out);
    assert!(
        prefix.join("usr/local/bin/axgf-cms").exists(),
        "the downloaded binary is installed:\n{}",
        r.out
    );
}

#[test]
fn a_download_that_does_not_match_its_checksum_is_refused() {
    let prefix = common::scratch("boot-badsum");
    let mirror = stage_release(&prefix, "v0.1.0-rc1");
    let api = stage_api(
        &prefix,
        r#"[{"tag_name": "v0.1.0-rc1", "prerelease": true}]"#,
    );

    // Corrupt the archive, leaving the sidecar describing what it used to be.
    let asset = mirror
        .join("releases/download/v0.1.0-rc1")
        .join(format!("axgf-cms-v0.1.0-rc1-{}.tar.gz", target_triple()));
    std::fs::write(&asset, b"not the bytes that were signed for").expect("corrupt asset");

    let r = run_against_mirror(&prefix, &mirror, &api, &["--version", "v0.1.0-rc1"]);
    assert!(
        !r.ok,
        "a mismatched checksum must stop the install:\n{}",
        r.out
    );
    assert!(r.says("checksum mismatch"), "{}", r.out);
    assert!(
        !prefix.join("usr/local/bin/axgf-cms").exists(),
        "and nothing is installed:\n{}",
        r.out
    );
}

#[test]
fn no_release_at_all_and_a_prerelease_only_repository_are_told_apart() {
    // The defect: both answered "No release published yet? Use --from-source."
    // The second is not that, and --from-source is not the shortest way out of
    // it — the release is there, it just is not the latest stable one.
    let empty = common::scratch("boot-none");
    let mirror = empty.join("mirror");
    std::fs::create_dir_all(&mirror).expect("mkdir mirror");
    let api = stage_api(&empty, "[]");
    let r = run_against_mirror(&empty, &mirror, &api, &[]);
    assert!(!r.ok);
    assert!(
        r.says("no release has been published"),
        "an empty repository is named as such: {}",
        r.out
    );
    assert!(r.says("--from-source"), "{}", r.out);
    assert!(
        !r.says("--version"),
        "there is no tag to suggest when nothing is published: {}",
        r.out
    );

    let pre = common::scratch("boot-pre");
    let mirror = pre.join("mirror");
    std::fs::create_dir_all(&mirror).expect("mkdir mirror");
    let api = stage_api(&pre, r#"[{"tag_name": "v0.1.0-rc1", "prerelease": true}]"#);
    let r = run_against_mirror(&pre, &mirror, &api, &[]);
    assert!(!r.ok);
    assert!(
        !r.says("no release has been published"),
        "a release WAS published; saying otherwise is the misdiagnosis: {}",
        r.out
    );
    assert!(
        r.says("--version v0.1.0-rc1"),
        "the way in is named, with the tag: {}",
        r.out
    );
}

#[test]
fn an_unknown_tag_says_what_is_published_instead() {
    let prefix = common::scratch("boot-badtag");
    let mirror = stage_release(&prefix, "v0.1.0-rc1");
    let api = stage_api(
        &prefix,
        r#"[{"tag_name": "v0.1.0-rc1", "prerelease": true}]"#,
    );

    let r = run_against_mirror(&prefix, &mirror, &api, &["--version", "v9.9.9"]);
    assert!(!r.ok);
    assert!(r.says("no release is tagged v9.9.9"), "{}", r.out);
    assert!(
        r.says("v0.1.0-rc1"),
        "and lists what there is instead: {}",
        r.out
    );
}

#[test]
fn a_weekly_verification_timer_is_installed_too() {
    // A backup nobody has read is a claim. This is the unit that turns it into
    // a fact, and it is its own unit on purpose: reading 435 MB back once a
    // week must never be able to take the web service down with it.
    let prefix = common::scratch("boot-verify-timer");
    run_bootstrap(&prefix, &[]);

    let timer = std::fs::read_to_string(prefix.join("etc/systemd/system/axgf-cms-verify.timer"))
        .expect("the verify timer is installed");
    assert!(timer.contains("OnCalendar=Sun"), "weekly: {timer}");
    assert!(timer.contains("Persistent=true"), "{timer}");

    let service =
        std::fs::read_to_string(prefix.join("etc/systemd/system/axgf-cms-verify.service"))
            .expect("the verify service is installed");
    assert!(
        service.lines().any(|l| l.trim()
            == format!(
                "ExecStart={}/usr/local/bin/axgf-cms verify",
                prefix.display()
            )),
        "it verifies the installation, with no argument:\n{service}"
    );
    // It reads everything and writes nothing, so it names no writable path at
    // all — the one directive the other two units need and this one must not.
    assert!(
        !service.contains("ReadWritePaths="),
        "a read-only job should have nothing writable:\n{service}"
    );
    assert!(service.contains("PrivateNetwork=yes"), "{service}");
    // Reading a 415 MB archive back has to extract it somewhere, and that
    // somewhere must not be the data directory. systemd's own facility, on
    // disk rather than in RAM, cleaned up when the unit stops.
    assert!(
        service.contains("CacheDirectory=axgf-cms-verify"),
        "{service}"
    );
}

#[test]
fn uninstall_removes_the_service_and_the_binary_and_nothing_else() {
    // The failure this guards against is somebody tidying up and deleting a
    // genealogy. Every path that holds data must still be there afterwards,
    // and the script has to say where.
    let prefix = common::scratch("boot-uninstall");
    run_bootstrap(&prefix, &["--with-sample"]);

    // What an installation looks like once it has run: a bundle, accounts, a
    // journal and an archive. The stand-in binary creates none of them, so
    // they are staged here the way the service would have left them.
    let data = prefix.join("var/lib/axgf-cms");
    let bundle = data.join("family.axgf");
    std::fs::write(&bundle, b"PK\x03\x04 the family").expect("bundle");
    std::fs::write(data.join("family.acl"), b"accounts").expect("acl");
    std::fs::write(data.join("family.axgf.journal"), b"{}\n").expect("journal");
    std::fs::create_dir_all(data.join("backups")).expect("mkdir");
    let archive = data.join("backups/axgf-backup-20260101T000000Z.zip");
    std::fs::write(&archive, b"an archive").expect("archive");

    std::fs::write(
        prefix.join("etc/systemd/system/axgf-cms.service.previous"),
        b"[Service]\n",
    )
    .expect("stage a rollback copy");

    let out = run_bootstrap(&prefix, &["--uninstall"]);

    // Gone — including the copies an upgrade keeps for its rollback.
    for gone in [
        "etc/systemd/system/axgf-cms.service",
        "etc/systemd/system/axgf-cms-backup.service",
        "etc/systemd/system/axgf-cms-backup.timer",
        "etc/systemd/system/axgf-cms-verify.service",
        "etc/systemd/system/axgf-cms-verify.timer",
        "usr/local/bin/axgf-cms",
        "etc/systemd/system/axgf-cms.service.previous",
    ] {
        assert!(
            !prefix.join(gone).exists(),
            "{gone} should have been removed"
        );
    }

    // Still there, every one of them.
    for kept in [
        &bundle,
        &data.join("family.acl"),
        &data.join("family.axgf.journal"),
        &archive,
        &prefix.join("etc/axgf-cms/env"),
    ] {
        assert!(
            kept.exists(),
            "{} must survive an uninstall",
            kept.display()
        );
    }

    // And it says where they are, because an operator who uninstalled by
    // mistake needs to know nothing was lost.
    assert!(out.contains("Nothing else is."), "{out}");
    assert!(out.contains(&bundle.display().to_string()), "{out}");
    assert!(
        out.contains("This script will not do that for you"),
        "it offers the commands to delete the data and refuses to run them: {out}"
    );
}
