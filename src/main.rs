//! Binary entry point: parse flags, load the bundle, serve the router.

use anyhow::{Context, Result};
use clap::Parser;

use axgf_cms::config::{self, Command, Config};
use axgf_cms::state::AppState;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    init_logging();

    let cfg = Config::parse();

    // The three commands that are not "serve" run and exit. Each of them is a
    // thing an operator does at a shell or a timer does at four in the
    // morning, and none of them needs a listening socket.
    match &cfg.command {
        Some(Command::Backup {
            dest,
            keep_daily,
            keep_weekly,
            keep_monthly,
        }) => {
            return run_backup(
                &cfg,
                dest.as_deref(),
                *keep_daily,
                *keep_weekly,
                *keep_monthly,
            )
        }
        Some(Command::Restore { archive, force }) => return run_restore(&cfg, archive, *force),
        Some(Command::Verify { archive }) => {
            return match archive {
                Some(a) => run_verify(a),
                None => run_self_check(&cfg),
            }
        }
        None => {}
    }
    // Before anything renders: the plausibility limit is fixed for the life of
    // the process, and every page asks for it.
    axgf_cms::living::set_max_age_years(cfg.presume_deceased_after);
    let (token, generated) = cfg.resolve_admin_token();

    let bundle = cfg.bundle()?.clone();

    // Claim the bundle for the life of this process, before it is read.
    //
    // Two things depend on this. A second server over the same bundle is
    // refused rather than allowed to take turns overwriting the first one's
    // edits; and `axgf-cms restore` can tell that an instance is attached and
    // refuse to swap the files out from under it. The lock is released by the
    // kernel when this process ends, however it ends.
    //
    // Held in `main` rather than in `AppState`, because a library caller — the
    // test suite, chiefly — opens the same bundle more than once on purpose,
    // and the claim being made is specifically "a server is serving this".
    let _instance = axgf_cms::lockfile::InstanceLock::acquire(&bundle)?;

    let seed = cfg.seed_sample.then_some(axgf_cms::SAMPLE_BUNDLE);
    let (state, payloads) = AppState::load(&bundle, token.clone(), seed, cfg.cache_dir.as_deref())
        .context("initialising application state")?;
    // No contact address means no geocoder, which is a supported way to run
    // rather than a missing feature: the coordinate fields are typed by hand
    // either way, and an installation that will not identify itself does not
    // make automated calls to someone else's donated service.
    let geocoder = axgf_cms::geocode::Geocoder::new(
        cfg.geocoder_endpoint.as_deref(),
        cfg.geocoder_contact.as_deref(),
    );
    if let Some(g) = geocoder.as_ref() {
        tracing::info!(
            endpoint = g.endpoint(),
            user_agent = g.user_agent(),
            "geocoder ready"
        );
    }
    let state = Arc::new(
        state
            .with_size_warn(cfg.size_warn_mb.saturating_mul(1024 * 1024))
            .with_geocoder(geocoder)
            .with_map(axgf_cms::state::MapTiles::new(
                cfg.map_tiles.as_deref(),
                cfg.map_attribution.as_deref(),
            ))
            .with_backup_dir(cfg.backup_dir.clone())
            // `generated` means a fresh token each boot, printed once and gone on
            // restart; the opposite is one the operator set, which lasts.
            .with_standing_admin_token(!generated),
    );

    // --create-admin runs against the loaded state and then exits. It happens
    // after the bundle is open, because the ACL binds to the bundle it is
    // created beside — that binding is what detects one family's accounts
    // being applied to another family's tree later.
    if let Some(username) = cfg.create_admin.as_deref() {
        return create_first_admin(&state, username);
    }

    let total: usize = state.counts().iter().map(|(_, n)| n).sum();
    tracing::info!(bundle = %bundle.display(), entities = total, "bundle loaded");

    // State plainly what happened to the media: an operator should see at a
    // glance that the payloads are on disk, not in RAM.
    let textual_bytes = state.textual_bundle_bytes();
    eprintln!("─────────────────────────────────────────────────────────");
    if payloads.extracted == 0 && payloads.reused == 0 {
        eprintln!("  payloads:  none in this bundle");
    } else {
        eprintln!(
            "  payloads:  {} extracted, {} reused from cache{}",
            payloads.extracted,
            payloads.reused,
            if payloads.mismatches > 0 {
                format!(
                    ", {} SHA-256 MISMATCH(ES) — see warnings above",
                    payloads.mismatches
                )
            } else {
                String::new()
            }
        );
        eprintln!("  cache:     {}", payloads.cache_dir.display());
        eprintln!(
            "  on disk:   {} of media",
            axgf_cms::documents::human_size(payloads.bytes_on_disk)
        );
    }
    eprintln!(
        "  in RAM:    {} of textual data (persons, families, metadata)",
        axgf_cms::documents::human_size(textual_bytes)
    );
    eprintln!("─────────────────────────────────────────────────────────");

    let app = axgf_cms::router(Arc::clone(&state));

    let listener = tokio::net::TcpListener::bind(cfg.bind)
        .await
        .with_context(|| format!("binding {}", cfg.bind))?;

    if generated {
        // Printed once, to stderr, so it shows up in `journalctl` on first boot
        // but never lands in the HTTP logs.
        eprintln!("─────────────────────────────────────────────────────────");
        eprintln!("  axgf-cms generated an admin token for this run:");
        eprintln!();
        eprintln!("      {token}");
        eprintln!();
        eprintln!(
            "  Set --admin-token or {} to keep it stable",
            config::ADMIN_TOKEN_ENV
        );
        eprintln!("  across restarts.");
        eprintln!("─────────────────────────────────────────────────────────");
    }
    if !cfg.bind.ip().is_loopback() {
        eprintln!(
            "WARNING: bound to {}, which is not localhost. This process \
             speaks plain HTTP, so every password reaching it does so in \
             clear text, and the session cookie is issued without `Secure` \
             unless a proxy sets X-Forwarded-Proto. Put it behind a reverse \
             proxy with TLS.",
            cfg.bind
        );
    }

    // Say out loud, once, what the health endpoint would say — an operator
    // reading `journalctl -u axgf-cms` after a restart should not have to curl
    // anything to find out that the backups stopped a fortnight ago.
    for check in state.health().problems() {
        tracing::warn!(check = check.name, "{}", check.detail);
    }

    eprintln!("axgf-cms listening on http://{}", cfg.bind);

    // Ready means ready: the bundle is loaded, validated and listening. Until
    // this datagram, `systemctl start` is still waiting — which on a 435 MB
    // archive is several seconds during which a Type=exec unit would already
    // have claimed success and a proxy in front of it would be serving 502.
    axgf_cms::notify::ready(&status_line(&state));

    // And the line `systemctl status` shows, kept current. A minute is often
    // enough: the three facts on it — the bundle, how many people, how old the
    // last backup is — change on the scale of an edit and a nightly timer, and
    // an operator reading `status` wants today's answer, not the one from
    // whenever the service last restarted.
    {
        let state = state.clone();
        tokio::spawn(async move {
            let mut tick = tokio::time::interval(std::time::Duration::from_secs(60));
            tick.tick().await; // the immediate one; `ready` just sent it
            loop {
                tick.tick().await;
                axgf_cms::notify::status(&status_line(&state));
            }
        });
    }

    // `into_make_service_with_connect_info` is what makes the peer address
    // reachable from a handler, and the login throttle's per-address bucket is
    // useless without it: an absent `ConnectInfo` would silently collapse
    // every anonymous attempt into one bucket keyed "unknown". Behind the
    // documented reverse proxy the peer is the proxy, which is why
    // `X-Forwarded-For` is preferred over it — but the direct-bind case has to
    // work too.
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal(state.clone()))
    .await
    .context("server error")?;

    // Every request that was in flight has returned by here — that is what
    // `with_graceful_shutdown` waits for. What it does not wait for is a save
    // that a *different* process started, and the one that matters is the
    // backup timer: it holds the write lock across three files. Taking the
    // lock is how this process says "whatever was being written has finished",
    // and dropping it immediately afterwards is the point — the claim is the
    // wait, not the holding.
    match axgf_cms::lockfile::WriteLock::acquire_for(&bundle, std::time::Duration::from_secs(30)) {
        Ok(lock) => drop(lock),
        Err(e) => tracing::warn!(error = %e, "stopped while another process was writing"),
    }
    tracing::info!("stopped cleanly");
    Ok(())
}

/// The one line `systemctl status` prints under the unit.
///
/// Three facts, because they are the three questions somebody opens `status`
/// to answer: which bundle is loaded, how much is in it, and whether the
/// backups are still happening.
fn status_line(state: &AppState) -> String {
    let people = state
        .counts()
        .iter()
        .find(|(k, _)| *k == "persons")
        .map(|(_, n)| *n)
        .unwrap_or(0);
    let health = state.health();
    let backup = health
        .get("backup")
        .map(|c| c.detail.clone())
        .unwrap_or_default();
    // The check's own sentence, shortened to the clause that carries the fact.
    let backup = backup.split(" — ").next().unwrap_or(&backup).to_string();
    format!(
        "{} · {people} people · {}",
        state.bundle_path().display(),
        backup
    )
}

/// Structured logs, to journald when systemd is running this process.
///
/// Under systemd, stderr goes to the journal already, so what matters is not
/// *where* the lines go but what they look like when they get there: one field
/// per fact rather than a sentence with values inlined, so `journalctl -o json`
/// and anything reading it can filter on them.
///
/// `info` by default and not `debug`: a family's installation writes a handful
/// of lines a day at `info` and several per request at `debug`, and a journal
/// that has rotated away the week something went wrong is no journal. `RUST_LOG`
/// still overrides it for the afternoon somebody is debugging.
fn init_logging() {
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "axgf_cms=info,tower_http=warn".into());
    let under_systemd = std::env::var_os("INVOCATION_ID").is_some();

    if under_systemd {
        // Straight to the journald socket, one journal field per tracing
        // field. Stderr would have reached the journal too, but as a line of
        // text: `journalctl -o json` would hand back the whole sentence as
        // MESSAGE and nothing to filter on. This way a save is a record with
        // BYTES and PATH on it, and an operator can ask for the record rather
        // than grep for the wording.
        //
        // If the socket is not there — a container without journald, a
        // sandbox that took AF_UNIX away — falling back to stderr is right:
        // logs that go somewhere plain beat a process that will not start.
        match tracing_journald::layer() {
            Ok(journal) => {
                use tracing_subscriber::layer::SubscriberExt as _;
                use tracing_subscriber::util::SubscriberInitExt as _;
                tracing_subscriber::registry()
                    .with(filter)
                    .with(journal)
                    .init();
                return;
            }
            Err(e) => {
                tracing_subscriber::fmt()
                    .with_env_filter(filter)
                    .with_target(true)
                    .with_ansi(false)
                    .without_time()
                    .init();
                tracing::warn!(error = %e, "no journald socket; logging to stderr instead");
                return;
            }
        }
    }
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(true)
        .init();
}

/// `axgf-cms backup --dest <dir>`
fn run_backup(
    cfg: &Config,
    dest: Option<&std::path::Path>,
    daily: usize,
    weekly: usize,
    monthly: usize,
) -> Result<()> {
    let bundle = cfg.bundle()?;
    let dest = dest
        .or(cfg.backup_dir.as_deref())
        .ok_or_else(|| {
            anyhow::anyhow!(
                "no destination. Say where the archive goes:\n\n\
                 \x20 axgf-cms backup --bundle {} --dest /srv/backups\n\n\
                 or set {} for this machine.",
                bundle.display(),
                config::BACKUP_DIR_ENV
            )
        })?
        .to_path_buf();

    let retention = axgf_cms::backup::Retention {
        daily,
        weekly,
        monthly,
    };
    let report = axgf_cms::backup::run(bundle, &dest, retention)?;

    let m = &report.manifest;
    eprintln!("─────────────────────────────────────────────────────────");
    eprintln!("  backup written and verified");
    eprintln!();
    eprintln!("    archive    {}", report.archive.display());
    eprintln!(
        "    size       {}",
        axgf_cms::documents::human_size(report.bytes)
    );
    eprintln!("    entities   {}", m.entities.values().sum::<usize>());
    eprintln!("    accounts   {}", m.accounts);
    eprintln!("    journal    {} lines", m.journal_lines);
    eprintln!("    saves held {} ms", report.lock_held_ms);
    if report.swept_parts > 0 {
        eprintln!();
        eprintln!(
            "    reclaimed  {} from {} part-written archive(s) left by an",
            axgf_cms::documents::human_size(report.swept_bytes),
            report.swept_parts
        );
        eprintln!("               interrupted run. They were never backups.");
    }
    if !report.pruned.is_empty() {
        eprintln!();
        eprintln!(
            "    retention removed {} older archive(s):",
            report.pruned.len()
        );
        for p in &report.pruned {
            eprintln!(
                "      {}",
                p.file_name().unwrap_or_default().to_string_lossy()
            );
        }
    }
    eprintln!();
    eprintln!("  An archive on the same disk as the bundle does not survive that");
    eprintln!("  disk. Copy it off the machine — see docs/OPERATOR.md.");
    eprintln!("─────────────────────────────────────────────────────────");
    Ok(())
}

/// `axgf-cms restore <archive>`
fn run_restore(cfg: &Config, archive: &std::path::Path, force: bool) -> Result<()> {
    let bundle = cfg.bundle()?;
    let plan = axgf_cms::backup::restore(archive, bundle, force)?;
    let v = &plan.verified;
    eprintln!("─────────────────────────────────────────────────────────");
    eprintln!("  restored {}", archive.display());
    eprintln!();
    eprintln!("    taken      {}", v.manifest.created_at);
    eprintln!("    bundle     {}", plan.files.bundle.display());
    for (kind, n) in &v.entities {
        if *n > 0 {
            eprintln!("    {kind:<10} {n}");
        }
    }
    eprintln!("    accounts   {}", v.accounts);
    eprintln!("    journal    {} lines", v.journal_lines);
    eprintln!();
    eprintln!("  The state that was there is not deleted. It is in");
    eprintln!("    {}", plan.aside.display());
    eprintln!("  If this was the wrong archive, move those files back.");
    eprintln!("─────────────────────────────────────────────────────────");
    Ok(())
}

/// `axgf-cms verify <archive>`
fn run_verify(archive: &std::path::Path) -> Result<()> {
    let v = axgf_cms::backup::verify(archive)?;
    eprintln!("─────────────────────────────────────────────────────────");
    eprintln!("  {} is a readable backup", archive.display());
    eprintln!();
    eprintln!("    taken      {}", v.manifest.created_at);
    eprintln!("    written by {}", v.manifest.written_by);
    eprintln!("    of         {}", v.manifest.source_bundle);
    for (kind, n) in &v.entities {
        if *n > 0 {
            eprintln!("    {kind:<10} {n}");
        }
    }
    eprintln!("    accounts   {}", v.accounts);
    eprintln!("    journal    {} lines", v.journal_lines);
    eprintln!("─────────────────────────────────────────────────────────");
    Ok(())
}

/// `axgf-cms verify` with no archive: check this installation.
///
/// # Why a separate unit and not a thread in the server
///
/// Reading a 435 MB archive back is minutes of I/O once a week, and the
/// failures it finds — a bundle that no longer validates, a cache that was
/// cleaned out, an archive that will not open — are exactly the failures that
/// must not be able to take the web service down with them. A separate unit
/// gets its own status, its own log and its own failure; `systemctl status
/// axgf-cms-verify` answers "did the weekly check pass" without reference to
/// whether anybody can reach the site.
///
/// Exit status is the point: non-zero puts the unit in `failed`, which is what
/// `systemctl is-failed` and every monitor already understand.
fn run_self_check(cfg: &Config) -> Result<()> {
    let bundle = cfg.bundle()?;
    // Deliberately not `AppState::load`: that opens the payload cache and
    // rewrites its index, and this runs from a unit with no writable path at
    // all. A job that reads the data back must not change it while looking —
    // which is not a theory: the first run of the real unit failed with
    // "Read-only file system" on the cache index, and that is the failure this
    // shape prevents. No instance lock either, for the same reason it takes no
    // locks at all: the server is running and this is a reader.
    let report = axgf_cms::health::report_offline(
        bundle,
        cfg.cache_dir.as_deref(),
        cfg.backup_dir.as_deref(),
    );

    eprintln!("─────────────────────────────────────────────────────────");
    eprintln!("  axgf-cms verify — {}", bundle.display());
    eprintln!();
    let mut worst = axgf_cms::health::Level::Ok;
    for c in &report.checks {
        eprintln!("    {:<8} {:<5} {}", c.name, c.level.as_str(), c.detail);
        worst = worst.max(c.level);
    }

    // The newest archive, read back in full: CRCs, the SHA-256 of every
    // member against the manifest, the bundle re-imported and validated.
    if let Some(dir) = cfg.backup_dir.as_deref() {
        match axgf_cms::backup::latest(dir) {
            Some(a) => match axgf_cms::backup::verify(&a.path) {
                Ok(v) => {
                    eprintln!();
                    eprintln!(
                        "    newest archive {} — readable, {} entities, {} accounts",
                        a.path.file_name().unwrap_or_default().to_string_lossy(),
                        v.entities.values().sum::<usize>(),
                        v.accounts
                    );
                }
                Err(e) => {
                    eprintln!();
                    eprintln!("    newest archive {} — UNREADABLE", a.path.display());
                    eprintln!("    {e:#}");
                    worst = axgf_cms::health::Level::Fail;
                }
            },
            None => eprintln!("\n    no archive in {} to read back", dir.display()),
        }
    }
    eprintln!("─────────────────────────────────────────────────────────");

    match worst {
        axgf_cms::health::Level::Fail => {
            anyhow::bail!("this installation has a failing check; see above")
        }
        axgf_cms::health::Level::Warn => {
            tracing::warn!("verify finished with warnings");
            Ok(())
        }
        axgf_cms::health::Level::Ok => {
            tracing::info!("verify finished: everything readable");
            Ok(())
        }
    }
}

/// Create an administrator account and print its generated password once.
///
/// The password is generated rather than taken as an argument, and printed to
/// stderr rather than stdout: an argument would sit in the shell history and
/// in `ps` output for as long as the process ran, and stdout is what a
/// bootstrap script is most likely to be piping somewhere.
///
/// Re-running with an existing username is an error rather than a reset, so a
/// bootstrap script can call this unconditionally on every deploy without
/// silently rotating a working account's password.
fn create_first_admin(state: &AppState, username: &str) -> Result<()> {
    use axgf_cms::acl::{self, Role};

    acl::validate_username(username)?;
    if state.acl_read(|a| a.has_username(&username.to_ascii_lowercase())) {
        anyhow::bail!(
            "an account named {username:?} already exists in {}. \n\
             This command creates accounts; it does not reset them. Sign in as \
             another administrator to change a password, or delete the account \
             from the .acl file if you have locked yourself out.",
            state.acl_path().display()
        );
    }

    let password = acl::generate_password();
    let user = acl::new_user(username, &password, Role::Admin)?;
    let name = user.username.clone();

    // Bind the ACL to this bundle the first time an account exists, so that
    // applying one family's accounts to another family's tree is detectable
    // later. The manifest fields survive editing; the SHA-256 does not, which
    // is why both are recorded.
    let manifest = state.read(|flat| flat.get("manifest").cloned());
    let sha = acl::file_sha256(state.bundle_path());

    state.acl_mutate(|acl| {
        if acl.users.is_empty() {
            acl.bind_to(manifest.as_ref(), sha);
        }
        acl.users.push(user);
    })?;

    eprintln!("─────────────────────────────────────────────────────────");
    eprintln!("  Administrator account created in");
    eprintln!("  {}", state.acl_path().display());
    eprintln!();
    eprintln!("      username:  {name}");
    eprintln!("      password:  {password}");
    eprintln!();
    eprintln!("  This password is shown once and is not recoverable — it is");
    eprintln!("  stored only as an Argon2id hash. Write it down now.");
    eprintln!("─────────────────────────────────────────────────────────");
    Ok(())
}

/// Resolve on Ctrl-C or SIGTERM so systemd restarts are clean.
async fn shutdown_signal(state: std::sync::Arc<AppState>) {
    let ctrl_c = async {
        let _ = tokio::signal::ctrl_c().await;
    };
    #[cfg(unix)]
    let term = async {
        if let Ok(mut s) = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
        {
            s.recv().await;
        }
    };
    #[cfg(not(unix))]
    let term = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {}
        _ = term => {}
    }
    // Before anything else: tell systemd this is a shutdown in progress, not a
    // unit that has stopped answering. `systemctl stop` then prints
    // "deactivating" with a reason rather than sitting silent until the
    // timeout.
    axgf_cms::notify::stopping("finishing the write in flight");
    tracing::info!("shutting down; finishing requests in flight");
    let _ = state;
}
