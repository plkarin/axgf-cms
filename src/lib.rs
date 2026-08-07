//! axgf-cms — the reference showcase application for the AXGF genealogy format.
//!
//! This library holds the whole application: state, routing, rendering. The
//! binary in `main.rs` only parses flags and serves the router, which lets the
//! integration tests drive the real application rather than a stand-in.
//!
//! # Where the genealogy lives
//!
//! Not here. Parsing dates, merging duplicates, validating structure and
//! converting GEDCOM are all `axgf-rs`'s job. This crate reads the bundle,
//! calls a library function, writes the bundle back and renders HTML. The one
//! deliberate exception is *presentation*: [`view`] decides how a date the
//! library already parsed should read in prose.

#![forbid(unsafe_code)]

pub mod auth;
pub mod config;
pub mod person;
pub mod render;
pub mod routes;
pub mod state;
pub mod tree;
pub mod view;

use std::path::Path;
use std::sync::Arc;

use anyhow::Result;

pub use routes::router;
pub use state::AppState;

/// Build a router over the bundle at `path`, creating it if absent.
///
/// This is the single wiring point shared by the binary and the tests.
pub fn app(path: &Path, admin_token: &str) -> Result<axum::Router> {
    let state = Arc::new(AppState::load_or_create(path, admin_token.to_string())?);
    Ok(router(state))
}
