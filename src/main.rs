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
    let state = Arc::new(
        AppState::load_or_seed(&cfg.bundle, token.clone(), seed)
            .context("initialising application state")?,
    );

    let total: usize = state.counts().iter().map(|(_, n)| n).sum();
    tracing::info!(bundle = %cfg.bundle.display(), entities = total, "bundle loaded");

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
            "WARNING: bound to {}, which is not localhost. V1 has no user \
             accounts — anyone who reaches this port and holds the token has \
             full edit rights. Put it behind a reverse proxy with TLS.",
            cfg.bind
        );
    }

    eprintln!("axgf-cms listening on http://{}", cfg.bind);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("server error")?;

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
