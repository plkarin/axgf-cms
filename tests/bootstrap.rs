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
fn bootstrap_reports_the_accounts_and_the_security_position() {
    let prefix = common::scratch("boot-report");
    let out = run_bootstrap(&prefix, &[]);
    let token = token_in(&prefix);

    assert!(out.contains(&token), "the token is printed once at the end");
    assert!(out.contains("/admin/login"), "the sign-in URL is printed");
    assert!(
        out.contains("back it up"),
        "the operator is told what the bundle is"
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
