//! Binary entry point: parse flags, load the bundle, serve the router.

use anyhow::{Context, Result};
use clap::Parser;

use axgf_cms::config::{self, Config};
use axgf_cms::state::AppState;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "axgf_cms=info,tower_http=warn".into()),
        )
        .init();

    let cfg = Config::parse();
    let (token, generated) = cfg.resolve_admin_token();

    let seed = cfg.seed_sample.then_some(axgf_cms::SAMPLE_BUNDLE);
    let (state, payloads) =
        AppState::load(&cfg.bundle, token.clone(), seed, cfg.cache_dir.as_deref())
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
            .with_geocoder(geocoder),
    );

    // --create-admin runs against the loaded state and then exits. It happens
    // after the bundle is open, because the ACL binds to the bundle it is
    // created beside — that binding is what detects one family's accounts
    // being applied to another family's tree later.
    if let Some(username) = cfg.create_admin.as_deref() {
        return create_first_admin(&state, username);
    }

    let total: usize = state.counts().iter().map(|(_, n)| n).sum();
    tracing::info!(bundle = %cfg.bundle.display(), entities = total, "bundle loaded");

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

    let app = axgf_cms::router(state);

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
