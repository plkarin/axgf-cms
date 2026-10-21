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

/// An upgrade takes the installation's settings from its env file, not from
/// the script's defaults: on the operator's server the service listens on
/// 8088 and something else answers on 8080, so an upgrade that asked 8080
/// for health would have rolled back a healthy release.
#[test]
fn an_upgrade_asks_for_health_where_the_installation_listens() {
    let prefix = common::scratch("boot-upgrade-bind");
    let elsewhere = prefix.join("srv/elsewhere");
    run_bootstrap(
        &prefix,
        &[
            "--bind",
            "127.0.0.1:8099",
            "--backup-dir",
            elsewhere.to_str().unwrap(),
        ],
    );
    // The stand-in binary writes no bundle; an upgrade wants one to exist.
    std::fs::write(prefix.join("var/lib/axgf-cms/family.axgf"), b"stand-in").unwrap();
    let out = run_bootstrap(&prefix, &["--upgrade"]);
    assert!(
        out.contains("health at http://127.0.0.1:8099/health"),
        "{out}"
    );
    assert!(!out.contains("127.0.0.1:8080"), "{out}");
    assert!(
        out.contains("srv/elsewhere"),
        "the backup goes where the installation keeps them: {out}"
    );

    // A --bind given on the command line still wins.
    let out = run_bootstrap(&prefix, &["--upgrade", "--bind", "127.0.0.1:8100"]);
    assert!(
        out.contains("health at http://127.0.0.1:8100/health"),
        "{out}"
    );
}

/// Upgrading from a binary that predates pre-upgrade archives (0.2.0 and
/// before): its backup would be one more daily and push the oldest out, so
/// the script runs it with pruning off and moves the archive into the
/// pre-upgrade set, 0600. This stand-in behaves like such a binary: no
/// `--pre-upgrade` in its help, and an `axgf-backup-*.zip` written 0644.
#[test]
fn upgrading_from_an_older_binary_keeps_the_dailies_and_files_its_archive_apart() {
    use std::os::unix::fs::PermissionsExt;
    let prefix = common::scratch("boot-upgrade-old");
    std::fs::write(
        prefix.join("stand-in-binary"),
        r#"#!/bin/sh
case "$1" in
  --version) echo "axgf-cms 0.2.0"; exit 0 ;;
  backup)
    for a in "$@"; do
      [ "$a" = --help ] && { echo "Usage: axgf-cms backup [--dest DIR] [--keep-daily N]"; exit 0; }
    done
    dest=""; prev=""
    for a in "$@"; do [ "$prev" = --dest ] && dest="$a"; prev="$a"; done
    # Like the real 0.2.0: write the archive 0644 under sudo -u's umask,
    # then keep one archive a day — which removes that day's daily.
    day=$(date -u +%Y%m%d)
    new="axgf-backup-$(date -u +%Y%m%dT%H%M%SZ).zip"
    ( umask 022; echo archive > "$dest/$new" )
    for old in "$dest"/axgf-backup-"$day"T*.zip; do
      [ -e "$old" ] && [ "$(basename "$old")" != "$new" ] && rm -f "$old"
    done
    exit 0 ;;
esac
exit 0
"#,
    )
    .unwrap();
    std::fs::set_permissions(
        prefix.join("stand-in-binary"),
        std::fs::Permissions::from_mode(0o755),
    )
    .unwrap();
    let backups = prefix.join("srv/backups");
    run_bootstrap(&prefix, &["--backup-dir", backups.to_str().unwrap()]);
    std::fs::write(prefix.join("var/lib/axgf-cms/family.axgf"), b"stand-in").unwrap();
    // Two older dailies, and this morning's — the one a same-day backup
    // beside it used to replace.
    let now = time::OffsetDateTime::now_utc();
    let today = format!(
        "axgf-backup-{:04}{:02}{:02}T000100Z.zip",
        now.year(),
        u8::from(now.month()),
        now.day()
    );
    let dailies = [
        "axgf-backup-20261001T031500Z.zip".to_string(),
        "axgf-backup-20261002T031500Z.zip".to_string(),
        today,
    ];
    for d in &dailies {
        std::fs::write(backups.join(d), b"daily").unwrap();
    }
    // Four earlier pre-upgrade archives: with the new one, the set is kept
    // to its last three.
    for d in ["20260901", "20260902", "20260903", "20260904"] {
        std::fs::write(
            backups.join(format!("axgf-pre-upgrade-{d}T120000Z.zip")),
            b"earlier",
        )
        .unwrap();
    }

    let out = run_bootstrap(&prefix, &["--upgrade"]);

    for d in &dailies {
        assert!(backups.join(d).exists(), "the upgrade evicted {d}");
    }
    let names: Vec<String> = std::fs::read_dir(&backups)
        .unwrap()
        .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
        .collect();
    let pre: Vec<&String> = names
        .iter()
        .filter(|n| n.starts_with("axgf-pre-upgrade-"))
        .collect();
    assert_eq!(
        pre.len(),
        3,
        "the last three pre-upgrade archives: {names:?}\n{out}"
    );
    assert!(
        !names.iter().any(|n| n.contains("20260901")),
        "the oldest went"
    );
    assert_eq!(
        names
            .iter()
            .filter(|n| n.starts_with("axgf-backup-"))
            .count(),
        3,
        "only the three dailies remain under the daily name: {names:?}"
    );
    assert!(
        !names.iter().any(|n| n.starts_with(".pre-upgrade")),
        "its working directory is gone: {names:?}"
    );
    let newest = pre.iter().max().unwrap();
    let mode = std::fs::metadata(backups.join(newest))
        .unwrap()
        .permissions()
        .mode()
        & 0o777;
    assert_eq!(mode, 0o600, "{newest} is {mode:o}");
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
        .env("TMPDIR", script_tmp(prefix))
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
    stage_script(&dir, tag, None);
    mirror
}

/// Stage the installer a release ships: this repository's bootstrap.sh,
/// stamped with `tag` exactly as the release workflow stamps it, with its
/// `.sha256` beside it. `extra` is a line added after the stamp, so a test
/// can see that this copy is the one that ran.
fn stage_script(dir: &Path, tag: &str, extra: Option<&str>) {
    let src = std::fs::read_to_string(repo_root().join("deploy/bootstrap.sh"))
        .expect("read bootstrap.sh");
    let stamp_from = "SCRIPT_VERSION=\"__SCRIPT_VERSION__\"";
    assert!(
        src.contains(stamp_from),
        "the script carries the placeholder the workflow stamps"
    );
    let mut stamped = src.replacen(stamp_from, &format!("SCRIPT_VERSION=\"{tag}\""), 1);
    if let Some(line) = extra {
        stamped = stamped.replacen(
            &format!("SCRIPT_VERSION=\"{tag}\""),
            &format!("SCRIPT_VERSION=\"{tag}\"\n{line}"),
            1,
        );
    }
    std::fs::write(dir.join("bootstrap.sh"), stamped).expect("write staged script");
    let st = Command::new("bash")
        .arg("-c")
        .arg(format!(
            "cd {} && sha256sum bootstrap.sh > bootstrap.sh.sha256",
            dir.display()
        ))
        .status()
        .expect("checksum the staged script");
    assert!(st.success());
}

/// A dry run with no locally supplied binary, so the download branch is the
/// one that reports what it would fetch.
/// It runs the installer as a release ships it, stamped `v0.1.0-rc1`.
fn dry_run_download(prefix: &Path, extra: &[&str]) -> String {
    dry_run_as(prefix, Some("v0.1.0-rc1"), extra)
}

/// A dry run of the installer stamped with `stamp`, or of the unstamped copy
/// on `main` when it is `None`.
fn dry_run_as(prefix: &Path, stamp: Option<&str>, extra: &[&str]) -> String {
    let mut args = vec!["--dry-run"];
    args.extend_from_slice(extra);
    let source = std::fs::read_to_string(repo_root().join("deploy/bootstrap.sh")).unwrap();
    let script = match stamp {
        Some(tag) => source.replace(
            "SCRIPT_VERSION=\"__SCRIPT_VERSION__\"",
            &format!("SCRIPT_VERSION=\"{tag}\""),
        ),
        None => source,
    };
    std::fs::create_dir_all(prefix).unwrap();
    let path = prefix.join("bootstrap-dry-run.sh");
    std::fs::write(&path, script).unwrap();
    let out = Command::new("bash")
        .arg(&path)
        .args(&args)
        .env("AXGF_CMS_PREFIX", prefix)
        .env("AXGF_CMS_SKIP_PRIVILEGED", "1")
        .env("AXGF_CMS_LOCAL_BINARY", "")
        .env("TMPDIR", script_tmp(prefix))
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
fn a_dry_run_describes_the_release_that_would_be_installed() {
    // A release's installer installs its own release, so that is the one
    // whose asset the dry run names, with or without --version.
    let asset = format!(
        "releases/download/v0.1.0-rc1/axgf-cms-v0.1.0-rc1-{}.tar.gz",
        target_triple()
    );
    let out = dry_run_download(&common::scratch("boot-url"), &[]);
    assert!(out.contains(&asset), "its own release: {out}");
    let out = dry_run_download(
        &common::scratch("boot-url-tag"),
        &["--version", "v0.1.0-rc1"],
    );
    assert!(out.contains(&asset), "--version naming itself: {out}");
}

#[test]
fn a_dry_run_that_would_hand_over_stops_there() {
    // It used to go on and describe its own release: asked for v0.3.0, the
    // v0.3.1 installer said it would download v0.3.1.
    let out = dry_run_download(&common::scratch("boot-dry-other"), &["--version", "v0.3.0"]);
    assert!(
        out.contains("releases/download/v0.3.0/bootstrap.sh"),
        "it names the installer it would hand over to: {out}"
    );
    assert!(out.contains("the rest is up to that installer"), "{out}");
    assert!(
        !out.contains("tar.gz"),
        "and describes no download of its own: {out}"
    );

    // The copy on main always hands over, to the latest release's installer.
    let out = dry_run_as(&common::scratch("boot-dry-main"), None, &[]);
    assert!(
        out.contains("releases/latest/download/bootstrap.sh"),
        "{out}"
    );
    assert!(out.contains("the rest is up to that installer"), "{out}");
    assert!(!out.contains("tar.gz"), "{out}");
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

/// Where the script's `mktemp` lands during a test: inside the test's own
/// scratch directory, so the guard removes whatever the script left, and so a
/// test can look at what it left.
fn script_tmp(prefix: &Path) -> PathBuf {
    let d = prefix.join("script-tmp");
    std::fs::create_dir_all(&d).expect("mkdir script tmp");
    d
}

fn leftovers(prefix: &Path) -> Vec<String> {
    std::fs::read_dir(script_tmp(prefix))
        .map(|r| {
            r.flatten()
                .map(|e| e.file_name().to_string_lossy().into_owned())
                .collect()
        })
        .unwrap_or_default()
}

/// The installer that runs is the one the release shipped: the copy from
/// `main` hands over to it, checked against its checksum, and that copy
/// installs exactly its own version's binary.
#[test]
fn the_installer_that_runs_is_the_one_the_release_shipped() {
    let prefix = common::scratch("boot-handover");
    let mirror = stage_release(&prefix, "v0.9.0");
    stage_script(
        &mirror.join("releases/download/v0.9.0"),
        "v0.9.0",
        Some("echo SHIPPED-WITH-v0.9.0"),
    );
    let api = stage_api(&prefix, r#"[{"tag_name": "v0.9.0", "prerelease": false}]"#);
    let r = run_against_mirror(&prefix, &mirror, &api, &["--version", "v0.9.0"]);
    assert!(r.ok, "{}", r.out);
    assert!(
        r.says("handing over to the installer shipped with v0.9.0 (checksum verified)"),
        "{}",
        r.out
    );
    assert!(
        r.says("SHIPPED-WITH-v0.9.0"),
        "the shipped copy ran: {}",
        r.out
    );
    assert!(
        r.says("releases/download/v0.9.0/axgf-cms-v0.9.0-"),
        "and fetched its own version by tag: {}",
        r.out
    );
    assert!(prefix.join("usr/local/bin/axgf-cms").exists(), "{}", r.out);
    assert_eq!(
        leftovers(&prefix),
        Vec::<String>::new(),
        "the handed-over copy cleans up after both"
    );
}

/// An installer that does not match its checksum, that says it belongs to
/// another release, or that a release never shipped, is not run.
#[test]
fn an_installer_that_is_not_the_releases_own_is_refused() {
    // Tampered with after its checksum was taken.
    let prefix = common::scratch("boot-handover-bad");
    let mirror = stage_release(&prefix, "v0.9.0");
    let api = stage_api(&prefix, r#"[{"tag_name": "v0.9.0", "prerelease": false}]"#);
    let script = mirror.join("releases/download/v0.9.0/bootstrap.sh");
    let mut s = std::fs::read_to_string(&script).unwrap();
    s.push_str("\necho TAMPERED\n");
    std::fs::write(&script, s).unwrap();
    let r = run_against_mirror(&prefix, &mirror, &api, &["--version", "v0.9.0"]);
    assert!(!r.ok && r.says("does not match its checksum"), "{}", r.out);
    assert!(
        !r.says("TAMPERED") && !prefix.join("usr/local/bin/axgf-cms").exists(),
        "{}",
        r.out
    );

    // Stamped for a different release than the one it sits in.
    let prefix = common::scratch("boot-handover-other");
    let mirror = stage_release(&prefix, "v0.9.0");
    let api = stage_api(&prefix, r#"[{"tag_name": "v0.9.0", "prerelease": false}]"#);
    stage_script(&mirror.join("releases/download/v0.9.0"), "v0.8.0", None);
    let r = run_against_mirror(&prefix, &mirror, &api, &["--version", "v0.9.0"]);
    assert!(
        !r.ok && r.says("says it is the installer for v0.8.0, not v0.9.0"),
        "{}",
        r.out
    );

    // A release from before installers were shipped.
    let prefix = common::scratch("boot-handover-old");
    let mirror = stage_release(&prefix, "v0.3.0");
    for f in ["bootstrap.sh", "bootstrap.sh.sha256"] {
        std::fs::remove_file(mirror.join("releases/download/v0.3.0").join(f)).unwrap();
    }
    let api = stage_api(&prefix, r#"[{"tag_name": "v0.3.0", "prerelease": false}]"#);
    let r = run_against_mirror(&prefix, &mirror, &api, &["--version", "v0.3.0"]);
    assert!(
        !r.ok && r.says("publishes no bootstrap.sh") && r.says("v0.3.1 or later"),
        "{}",
        r.out
    );
    assert!(!prefix.join("usr/local/bin/axgf-cms").exists());
}

/// The script removes its temporary directories however it exits.
///
/// They were removed on the success path only. A full test run left eighteen
/// in /tmp; on an operator's machine a failed download or checksum left one
/// each time, and `--from-source` left a clone and a release build.
#[test]
fn bootstrap_leaves_no_temporary_directory_however_it_exits() {
    // Installed from a release.
    let ok = common::scratch("boot-tmp-ok");
    let mirror = stage_release(&ok, "v0.1.0-rc1");
    let api = stage_api(&ok, r#"[{"tag_name": "v0.1.0-rc1", "prerelease": true}]"#);
    let r = run_against_mirror(&ok, &mirror, &api, &["--version", "v0.1.0-rc1"]);
    assert!(r.ok, "the install itself should succeed:\n{}", r.out);
    assert_eq!(
        leftovers(&ok),
        Vec::<String>::new(),
        "after a successful install"
    );

    // Refused on a checksum mismatch — `die`, mid-download.
    let bad = common::scratch("boot-tmp-bad");
    let mirror = stage_release(&bad, "v0.1.0-rc1");
    let api = stage_api(&bad, r#"[{"tag_name": "v0.1.0-rc1", "prerelease": true}]"#);
    let asset = mirror
        .join("releases/download/v0.1.0-rc1")
        .join(format!("axgf-cms-v0.1.0-rc1-{}.tar.gz", target_triple()));
    std::fs::write(&asset, b"not the bytes that were signed for").expect("corrupt asset");
    let r = run_against_mirror(&bad, &mirror, &api, &["--version", "v0.1.0-rc1"]);
    assert!(!r.ok, "a corrupt asset must be refused");
    assert_eq!(
        leftovers(&bad),
        Vec::<String>::new(),
        "after a checksum refusal"
    );

    // Nothing to download at all.
    let none = common::scratch("boot-tmp-none");
    let mirror = none.join("mirror");
    std::fs::create_dir_all(&mirror).expect("mkdir mirror");
    let api = stage_api(&none, "[]");
    let r = run_against_mirror(&none, &mirror, &api, &[]);
    assert!(!r.ok);
    assert_eq!(
        leftovers(&none),
        Vec::<String>::new(),
        "after a failed download"
    );

    // A dry run, which never downloads and used to leave an empty one.
    let dry = common::scratch("boot-tmp-dry");
    dry_run_download(&dry, &[]);
    assert_eq!(leftovers(&dry), Vec::<String>::new(), "after a dry run");
}

/// The rollback rehearsal every deploy runs first (docs/DEPLOY.md) parses,
/// explains itself, and will not run without the release it rehearses —
/// rehearsing with any other installer would prove nothing about this one.
/// It needs root and a live installation to run for real; the runbook is
/// where it is run.
#[test]
fn the_rollback_rehearsal_is_shipped_and_insists_on_a_version() {
    let script = repo_root().join("deploy/rehearse-rollback.sh");
    let syntax = Command::new("bash")
        .arg("-n")
        .arg(&script)
        .status()
        .unwrap();
    assert!(syntax.success());
    let help = Command::new("bash")
        .arg(&script)
        .arg("--help")
        .output()
        .unwrap();
    assert!(help.status.success());
    assert!(String::from_utf8_lossy(&help.stdout).contains("REQUIRED"));
    let bare = Command::new("bash").arg(&script).output().unwrap();
    assert!(!bare.status.success());
    assert!(String::from_utf8_lossy(&bare.stderr).contains("--version is required"));
    let workflow =
        std::fs::read_to_string(repo_root().join(".github/workflows/release.yml")).unwrap();
    assert!(
        workflow.contains("cp deploy/rehearse-rollback.sh dist/rehearse-rollback.sh"),
        "shipped with each release"
    );
    assert!(
        workflow.contains("cp deploy/bootstrap.sh dist/bootstrap.sh"),
        "and so is the installer"
    );
}

// ---------------------------------------------------------------------------
// The upgrade helper (docs/adr/0011 §5)
// ---------------------------------------------------------------------------

/// Run the installed helper the way its units do.
fn run_helper(prefix: &Path, mode: &str) -> std::process::Output {
    Command::new("bash")
        .arg(prefix.join("usr/local/lib/axgf-cms/upgrade-helper"))
        .arg(mode)
        .output()
        .expect("run the upgrade helper")
}

fn helper_status(prefix: &Path) -> serde_json::Value {
    let text = std::fs::read_to_string(prefix.join("var/lib/axgf-cms/upgrade.status"))
        .expect("the helper wrote upgrade.status");
    serde_json::from_str(&text)
        .unwrap_or_else(|e| panic!("upgrade.status is not JSON ({e}):\n{text}"))
}

fn ask_for(prefix: &Path, version: &str) {
    let body = serde_json::json!({ "version": version, "requested_by": "ada", "requested_at": "2026-10-21T09:00:00Z", "id": "x" });
    std::fs::write(
        prefix.join("var/lib/axgf-cms/upgrade.request"),
        body.to_string(),
    )
    .unwrap();
}

#[test]
fn the_upgrade_helper_is_installed_and_acts_on_a_release_tag_only() {
    let prefix = common::scratch("boot-helper");
    run_bootstrap(&prefix, &[]);
    let helper = prefix.join("usr/local/lib/axgf-cms/upgrade-helper");
    let installer = prefix.join("usr/local/lib/axgf-cms/bootstrap.sh");
    assert!(helper.exists() && installer.exists());
    use std::os::unix::fs::PermissionsExt;
    assert_eq!(
        std::fs::metadata(&helper).unwrap().permissions().mode() & 0o777,
        0o700,
        "root's alone"
    );
    assert!(Command::new("bash")
        .arg("-n")
        .arg(&helper)
        .status()
        .unwrap()
        .success());
    let unit = |name: &str| {
        std::fs::read_to_string(prefix.join("etc/systemd/system").join(name))
            .unwrap_or_else(|e| panic!("{name}: {e}"))
    };
    let data = prefix.join("var/lib/axgf-cms");
    assert!(unit("axgf-cms-upgrade.path")
        .contains(&format!("PathExists={}/upgrade.request", data.display())));
    assert!(unit("axgf-cms-upgrade.service")
        .contains(&format!("ExecStart={} upgrade", helper.display())));
    assert!(unit("axgf-cms-releases.path")
        .contains(&format!("PathExists={}/releases.request", data.display())));
    assert!(unit("axgf-cms-releases.service")
        .contains(&format!("ExecStart={} releases", helper.display())));
    assert!(unit("axgf-cms-releases.timer").contains("OnCalendar=daily"));

    // Anything but a release tag is refused, and the request is taken away.
    ask_for(&prefix, "main; rm -rf /");
    let out = run_helper(&prefix, "upgrade");
    assert!(out.status.success());
    assert_eq!(helper_status(&prefix)["state"], "refused");
    assert!(!data.join("upgrade.request").exists());

    // A tag goes to the installer, whose output comes back as the status's
    // log — colours stripped, quotes kept, valid JSON throughout.
    std::fs::write(
        &installer,
        "#!/bin/bash\necho \"stub $*\"\nprintf '\\033[1m==> a step\\033[0m\\n'\necho 'a \"quoted\" \\\\ line'\nexit 0\n",
    )
    .unwrap();
    ask_for(&prefix, "v9.9.9");
    assert!(run_helper(&prefix, "upgrade").status.success());
    let st = helper_status(&prefix);
    assert_eq!(st["state"], "succeeded");
    assert_eq!(st["to"], "v9.9.9");
    let log: Vec<String> = st["log"]
        .as_array()
        .unwrap()
        .iter()
        .map(|l| l.as_str().unwrap().to_string())
        .collect();
    assert!(
        log.contains(&"stub --upgrade --version v9.9.9".to_string()),
        "{log:?}"
    );
    assert!(log.contains(&"==> a step".to_string()), "{log:?}");
    // The stub's single quotes keep both backslashes; so does the status.
    assert!(
        log.contains(&r#"a "quoted" \\ line"#.to_string()),
        "{log:?}"
    );
    assert!(!data.join("upgrade.request").exists());

    // A failing installer is reported as failed.
    std::fs::write(
        &installer,
        "#!/bin/bash\necho 'health check failed'\nexit 1\n",
    )
    .unwrap();
    ask_for(&prefix, "v9.9.8");
    run_helper(&prefix, "upgrade");
    assert_eq!(helper_status(&prefix)["state"], "failed");

    // Nothing asked, nothing done.
    assert!(run_helper(&prefix, "upgrade").status.success());

    // And an uninstall takes it all away, data aside.
    run_bootstrap(&prefix, &["--uninstall"]);
    assert!(!helper.exists() && !installer.exists());
    for name in [
        "axgf-cms-upgrade.path",
        "axgf-cms-upgrade.service",
        "axgf-cms-releases.path",
        "axgf-cms-releases.service",
        "axgf-cms-releases.timer",
    ] {
        assert!(
            !prefix.join("etc/systemd/system").join(name).exists(),
            "{name} left behind"
        );
    }
    assert!(
        data.join("upgrade.status").exists(),
        "the data directory is not touched"
    );
}

#[test]
fn a_private_repository_is_reached_through_the_api_with_a_token() {
    let prefix = common::scratch("boot-private");
    let tag = "v0.1.0-rc1";
    let staged = stage_release(&prefix, tag);
    let assets = staged.join("releases/download").join(tag);
    // What the API holds: the release by tag, naming its assets by id, and
    // each asset's bytes at its id. The public download URLs answer nothing,
    // as a private repository's do.
    let api = prefix.join("api");
    std::fs::create_dir_all(api.join("releases/tags")).unwrap();
    std::fs::create_dir_all(api.join("releases/assets")).unwrap();
    let mut listed = Vec::new();
    for (i, entry) in std::fs::read_dir(&assets).unwrap().enumerate() {
        let entry = entry.unwrap();
        let id = 7000 + i;
        let name = entry.file_name().to_string_lossy().into_owned();
        std::fs::copy(
            entry.path(),
            api.join("releases/assets").join(id.to_string()),
        )
        .unwrap();
        listed.push(serde_json::json!({
            "url": format!("https://api.github.com/repos/plkarin/axgf-cms/releases/assets/{id}"),
            "id": id,
            "node_id": "RA_x",
            "name": name,
            "uploader": { "login": "someone", "url": "https://api.github.com/users/someone" },
        }));
    }
    let release = serde_json::json!({
        "url": "https://api.github.com/repos/plkarin/axgf-cms/releases/1",
        "assets_url": "https://api.github.com/repos/plkarin/axgf-cms/releases/1/assets",
        "tag_name": tag,
        "name": format!("axgf-cms {tag}"),
        "assets": listed,
    });
    std::fs::write(
        api.join("releases/tags").join(tag),
        serde_json::to_string_pretty(&release).unwrap(),
    )
    .unwrap();
    let nowhere = prefix.join("no-public-downloads");
    std::fs::create_dir_all(&nowhere).unwrap();

    let secret = "ghp_not-a-real-token-but-secret";
    let out = Command::new("bash")
        .arg(repo_root().join("deploy/bootstrap.sh"))
        .args(["--version", tag])
        .env("AXGF_CMS_PREFIX", &prefix)
        .env("AXGF_CMS_SKIP_PRIVILEGED", "1")
        .env(
            "AXGF_CMS_RELEASE_BASE",
            format!("file://{}", nowhere.display()),
        )
        .env("AXGF_CMS_API_BASE", format!("file://{}", api.display()))
        .env("AXGF_CMS_LOCAL_BINARY", "")
        .env("AXGF_CMS_GITHUB_TOKEN", secret)
        .env("TMPDIR", script_tmp(&prefix))
        .output()
        .expect("run bootstrap.sh");
    let text =
        String::from_utf8_lossy(&out.stdout).into_owned() + &String::from_utf8_lossy(&out.stderr);
    assert!(out.status.success(), "{text}");
    assert!(text.contains("checksum verified"), "{text}");
    assert!(
        prefix.join("usr/local/bin/axgf-cms").exists(),
        "installed from the API's copy"
    );
    assert!(!text.contains(secret), "the token is never printed");

    // Without the token, the same repository is a failed download.
    let prefix2 = common::scratch("boot-private-no-token");
    let out = Command::new("bash")
        .arg(repo_root().join("deploy/bootstrap.sh"))
        .args(["--version", tag])
        .env("AXGF_CMS_PREFIX", &prefix2)
        .env("AXGF_CMS_SKIP_PRIVILEGED", "1")
        .env(
            "AXGF_CMS_RELEASE_BASE",
            format!("file://{}", nowhere.display()),
        )
        .env("AXGF_CMS_API_BASE", format!("file://{}", api.display()))
        .env("AXGF_CMS_LOCAL_BINARY", "")
        .env_remove("AXGF_CMS_GITHUB_TOKEN")
        .env("TMPDIR", script_tmp(&prefix2))
        .output()
        .expect("run bootstrap.sh");
    assert!(!out.status.success());
}
