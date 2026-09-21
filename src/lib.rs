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

// This used to be `forbid`, which cannot be locally waived, and that was right
// for as long as nothing here needed `unsafe`. One thing now does: there is no
// free-space call in `std`, and refusing a write before it starts — rather than
// discovering a full disk halfway through rebuilding a 400 MB archive — needs
// `statvfs`. `deny` keeps every other module exactly as strict, and
// `src/space.rs` carries the single `#[allow]`, two of them, around two FFI
// calls that borrow nothing and keep nothing.
#![deny(unsafe_code)]

pub mod access;
pub mod acl;
pub mod admin;
pub mod auth;
pub mod avatar;
pub mod backup;
pub mod completeness;
pub mod config;
pub mod convert;
pub mod coords;
pub mod diff;
pub mod documents;
pub mod forms;
pub mod geocode;
pub mod health;
pub mod i18n;
pub mod journal;
pub mod living;
pub mod lockfile;
pub mod payloads;
pub mod person;
pub mod physical;
pub mod place;
pub mod profile;
pub mod radar;
pub mod render;
pub mod routes;
pub mod sensitive;
pub mod session;
pub mod settings;
pub mod space;
pub mod state;
pub mod style;
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

/// The same, with a basemap configured.
///
/// Exists for the tests, which need to see both sides of a decision the
/// operator makes once at startup: with no tile source the place editor must
/// pull in no map assets at all, and with one it must pull in exactly the two
/// this binary serves itself.
pub fn app_with_map(
    path: &Path,
    admin_token: &str,
    tiles: crate::state::MapTiles,
) -> Result<axum::Router> {
    let state = AppState::load_or_create(path, admin_token.to_string())?;
    Ok(router(Arc::new(state.with_map(Some(tiles)))))
}

/// A throwaway directory for a unit test, removed when the value is dropped.
///
/// The unit tests used to scatter directories through `/tmp` — a hundred and
/// two of them per run on the machine this was written for — and integration
/// tests did the same in `target/tmp`, which is the filesystem the application
/// saves its bundle to. Nothing cleaned either up. Both now hand back a guard
/// instead of a path, so a test's files go away when the test does, including
/// when it panics.
#[cfg(test)]
pub(crate) mod scratch {
    use std::ops::Deref;
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicUsize, Ordering};

    static N: AtomicUsize = AtomicUsize::new(0);

    /// A directory that removes itself.
    #[derive(Debug)]
    pub struct Dir(PathBuf);

    impl Dir {
        /// A fresh directory named after `tag`.
        pub fn new(tag: &str) -> Self {
            let base = std::env::var("CARGO_TARGET_TMPDIR")
                .map(PathBuf::from)
                .unwrap_or_else(|_| std::env::temp_dir());
            let dir = base.join(format!(
                "axgf-unit-{}-{}-{}",
                std::process::id(),
                tag,
                N.fetch_add(1, Ordering::SeqCst)
            ));
            let _ = std::fs::remove_dir_all(&dir);
            std::fs::create_dir_all(&dir).expect("create scratch dir");
            Self(dir)
        }
    }

    impl Deref for Dir {
        type Target = Path;
        fn deref(&self) -> &Path {
            &self.0
        }
    }

    impl AsRef<Path> for Dir {
        fn as_ref(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for Dir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }
}
