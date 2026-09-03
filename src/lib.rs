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

// Nothing in this crate needs `unsafe`, and `forbid` is what says so in a way
// that cannot be locally waived. The one exception this file used to carry —
// a `malloc_trim` call to hand back the heap `import_bundle`'s base64 forced
// into being — went away with the streaming boundary: with no payload ever
// materialised, there is no transient heap to return.
#![forbid(unsafe_code)]

pub mod access;
pub mod acl;
pub mod admin;
pub mod auth;
pub mod completeness;
pub mod config;
pub mod convert;
pub mod diff;
pub mod documents;
pub mod i18n;
pub mod journal;
pub mod payloads;
pub mod person;
pub mod place;
pub mod render;
pub mod routes;
pub mod session;
pub mod state;
pub mod theme;
pub mod tree;
pub mod view;

use std::path::Path;
use std::sync::Arc;

use anyhow::Result;

pub use routes::router;
pub use state::AppState;

/// The demonstration bundle written by `--seed-sample`.
///
/// Embedded so a binary downloaded from GitHub Releases can seed itself with
/// no extra files. It is the converted `deploy/sample.ged` plus the
/// AXGF-native facts GEDCOM cannot carry — see `tests/sample_bundle.rs`.
pub const SAMPLE_BUNDLE: &[u8] = include_bytes!("../deploy/sample.axgf");

/// Build a router over the bundle at `path`, creating it if absent.
///
/// This is the single wiring point shared by the binary and the tests.
pub fn app(path: &Path, admin_token: &str) -> Result<axum::Router> {
    let state = Arc::new(AppState::load_or_create(path, admin_token.to_string())?);
    Ok(router(state))
}
