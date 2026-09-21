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
        Some(Command::Verify { archive }) => return run_verify(archive),
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
            .with_backup_dir(cfg.backup_dir.clone()),
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
    .with_graceful_shutdown(shutdown_signal())
    .await
    .context("server error")?;

    Ok(())
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
    let builder = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(true);
    if under_systemd {
        // systemd stamps every line itself, so a second timestamp is noise;
        // ANSI colour in a journal is worse than noise.
        builder.with_ansi(false).without_time().init();
    } else {
        builder.init();
    }
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
async fn shutdown_signal() {
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
    tracing::info!("shutting down");
}
