//! Application state: one `.axgf` bundle, held in memory, written atomically.
//!
//! The bundle file *is* the database. There is no SQL, no cache server, no
//! migration story — the process owns one path and one `RwLock`.
//!
//! # The mutation contract
//!
//! Every mutating handler goes through [`AppState::mutate`], which enforces the
//! ordering the project requires:
//!
//! 1. take the write lock
//! 2. call the `axgf-rs` function
//! 3. if the envelope status is `error`, release and return the diagnostics —
//!    memory and file both unchanged
//! 4. on success, replace the in-memory flat JSON
//! 5. `export_bundle`, then write atomically (tmp + fsync + rename)
//! 6. release the lock
//!
//! Step 5 is the reason a handler must never write the file itself: a partial
//! write would leave the showcase's only database truncated.

use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::sync::RwLock;

use anyhow::{Context, Result};
use axgf_rs::boundary::envelope::{Diagnostic, Envelope, Status};
use base64::Engine as _;
use serde_json::Value;

/// The eight collection names in a flat bundle, in the order the UI lists them.
pub const COLLECTIONS: [&str; 8] = [
    "persons",
    "families",
    "events",
    "links",
    "occupations",
    "sources",
    "places",
    "documents",
];

/// One `.axgf` bundle plus the path it is persisted to.
pub struct AppState {
    /// Absolute path of the live bundle. Never written non-atomically.
    bundle_path: PathBuf,
    /// The flat-bundle JSON, as returned by `import_bundle`/`create_bundle`.
    inner: RwLock<Value>,
    /// Shared admin token. V1 auth is a single token, not a user system.
    admin_token: String,
    /// Converted bundles awaiting download. Deliberately separate from the
    /// served bundle: conversion is a utility and must never write over it.
    conversions: crate::convert::ConversionCache,
    /// Rendered thumbnails, bounded. Decoding a photograph per request is a
    /// cost that only becomes visible once a gallery has a dozen pictures.
    thumbs: crate::documents::ThumbCache,
    /// Bundle size past which the admin panel warns. Not a limit.
    size_warn: u64,
}

/// Outcome of a mutation attempt.
///
/// A refused mutation is not an application error — the library declined and
/// said why, and the caller renders those diagnostics. Only an I/O failure
/// while persisting is an `Err`.
pub struct MutationOutcome {
    /// Whether the library accepted the mutation and the file was rewritten.
    pub applied: bool,
    /// Diagnostics from the library call, always surfaced, never swallowed.
    pub diagnostics: Vec<Diagnostic>,
    /// The envelope's `data`, so callers can read the extras that accompany
    /// the new bundle — `id` after a create, `merged_persons` after a dedup.
    pub data: Value,
}

/// Pull the new flat bundle out of an envelope's `data`.
///
/// The library is not uniform here, and the difference matters:
///
/// | function | `data` |
/// |---|---|
/// | `create_bundle`, `import_bundle` | the flat bundle itself |
/// | `add_entity`, `update_entity`, `delete_entity` | `{"id", "bundle"}` |
/// | `deduplicate` | `{"bundle", "merged_persons", …}` |
/// | `convert_gedcom` | `{"bundle"}` |
///
/// A flat bundle has no top-level `bundle` key of its own — its keys are
/// `manifest` and the eight collections — so preferring `data.bundle` when
/// present and falling back to `data` covers every case unambiguously.
fn bundle_from_data(data: &Value) -> Option<&Value> {
    match data.get("bundle") {
        Some(b) if b.is_object() => Some(b),
        _ if data.get("manifest").is_some() => Some(data),
        _ => None,
    }
}

impl AppState {
    /// Load the bundle at `path`, creating an empty one if the file is absent.
    pub fn load_or_create(path: &Path, admin_token: String) -> Result<Self> {
        Self::load_or_seed(path, admin_token, None)
    }

    /// Load the bundle at `path`, seeding it from `seed` when the file does
    /// not exist.
    ///
    /// Seeding only ever happens when there is no bundle, which is what makes
    /// `bootstrap.sh --with-sample` safe to run twice: an existing bundle is
    /// loaded untouched and the seed is ignored.
    pub fn load_or_seed(path: &Path, admin_token: String, seed: Option<&[u8]>) -> Result<Self> {
        let flat = if path.exists() {
            let bytes =
                fs::read(path).with_context(|| format!("reading bundle {}", path.display()))?;
            let env = axgf_rs::import_bundle(&bytes);
            envelope_into_data(env)
                .with_context(|| format!("importing bundle {}", path.display()))?
        } else {
            let flat = match seed {
                Some(bytes) => {
                    tracing::info!(path = %path.display(), "seeding a new bundle");
                    let env = axgf_rs::import_bundle(bytes);
                    envelope_into_data(env).context("importing the seed bundle")?
                }
                None => {
                    tracing::info!(
                        path = %path.display(),
                        "no bundle found; creating an empty one"
                    );
                    let env = axgf_rs::create_bundle(None);
                    envelope_into_data(env).context("creating an empty bundle")?
                }
            };
            if let Some(parent) = path.parent() {
                if !parent.as_os_str().is_empty() {
                    fs::create_dir_all(parent).with_context(|| {
                        format!("creating bundle directory {}", parent.display())
                    })?;
                }
            }
            write_bundle(path, &flat).context("writing the initial bundle")?;
            flat
        };

        Ok(Self {
            bundle_path: path.to_path_buf(),
            inner: RwLock::new(flat),
            admin_token,
            conversions: crate::convert::ConversionCache::default(),
            thumbs: crate::documents::ThumbCache::default(),
            size_warn: crate::documents::DEFAULT_SIZE_WARN,
        })
    }

    /// The configured admin token.
    pub fn admin_token(&self) -> &str {
        &self.admin_token
    }

    /// Short-lived store of converted bundles awaiting download.
    pub fn conversions(&self) -> &crate::convert::ConversionCache {
        &self.conversions
    }

    /// The bounded thumbnail cache.
    pub fn thumbs(&self) -> &crate::documents::ThumbCache {
        &self.thumbs
    }

    /// Set the size past which the admin panel warns, before the state is
    /// shared. Consuming `self` keeps the setting immutable once it is behind
    /// an `Arc`, which is where every handler sees it from.
    pub fn with_size_warn(mut self, bytes: u64) -> Self {
        self.size_warn = bytes;
        self
    }

    /// The configured size warning threshold, in bytes.
    pub fn size_warn(&self) -> u64 {
        self.size_warn
    }

    /// Path of the live bundle.
    pub fn bundle_path(&self) -> &Path {
        &self.bundle_path
    }

    /// Run `f` against the current flat bundle under the read lock.
    ///
    /// Reads never touch the disk; the in-memory copy is authoritative between
    /// mutations because nothing else is allowed to write the file.
    pub fn read<T>(&self, f: impl FnOnce(&Value) -> T) -> T {
        let guard = self.inner.read().unwrap_or_else(|e| e.into_inner());
        f(&guard)
    }

    /// Serialize the current flat bundle to a JSON string.
    pub fn flat_json(&self) -> String {
        self.read(|v| v.to_string())
    }

    /// Export the current bundle to `.axgf` bytes without mutating anything.
    pub fn export_bytes(&self) -> Result<Vec<u8>> {
        let flat = self.flat_json();
        export_to_bytes(&flat)
    }

    /// Apply a mutation under the write lock, persisting atomically on success.
    ///
    /// `op` receives the current flat JSON and returns the library's envelope.
    /// It must not write to disk or take the lock itself.
    pub fn mutate(&self, op: impl FnOnce(&str) -> Envelope) -> Result<MutationOutcome> {
        self.mutate_and_adjust(op, |_, _| {})
    }

    /// Apply a mutation, then adjust the resulting bundle before it is written.
    ///
    /// `adjust` receives the new flat bundle and the envelope's `data`, and
    /// runs *inside* the same write lock and before the atomic write, so what
    /// it changes lands on disk in the same rename as the library's own
    /// change. This exists for the one thing `axgf-rs` has no CRUD call for:
    /// a Document entity and its bytes are two writes to two parts of the
    /// bundle, and splitting them across two `mutate` calls would leave a
    /// window where a Document claims a file the bundle does not carry.
    ///
    /// `adjust` must not fail: everything it needs was validated before the
    /// lock was taken.
    pub fn mutate_and_adjust(
        &self,
        op: impl FnOnce(&str) -> Envelope,
        adjust: impl FnOnce(&mut Value, &Value),
    ) -> Result<MutationOutcome> {
        // 1. take the write lock
        let mut guard = self.inner.write().unwrap_or_else(|e| e.into_inner());

        // 2. call the library
        let env = op(&guard.to_string());
        let diagnostics = env.diagnostics.clone();

        // 3. refusal leaves memory and file untouched
        if env.status == Status::Error || env.data.is_null() {
            return Ok(MutationOutcome {
                applied: false,
                diagnostics,
                data: Value::Null,
            });
        }

        // An `ok` envelope that carries no bundle is not something to persist.
        // Treating it as a refusal is the safe reading: better to report "not
        // applied" than to write a bundle-shaped fragment over the database.
        let Some(new_bundle) = bundle_from_data(&env.data) else {
            tracing::error!("mutation returned ok but carried no bundle; not persisting");
            return Ok(MutationOutcome {
                applied: false,
                diagnostics,
                data: env.data,
            });
        };
        let mut new_bundle = new_bundle.clone();
        adjust(&mut new_bundle, &env.data);

        // 4/5. persist first, then swap memory in. Writing before the swap means
        // an I/O failure leaves the in-memory bundle matching what is on disk,
        // so a failed write cannot strand the process with unsaved state.
        write_bundle(&self.bundle_path, &new_bundle)?;
        *guard = new_bundle;

        // 6. lock released on drop
        Ok(MutationOutcome {
            applied: true,
            diagnostics,
            data: env.data,
        })
    }

    /// Run a read-only library call against the current bundle.
    ///
    /// Used for `validate` and `inspect`, which report on the bundle without
    /// producing a new one and therefore must never take the write lock.
    pub fn inspect_with(&self, op: impl FnOnce(&str) -> Envelope) -> Envelope {
        let flat = self.flat_json();
        op(&flat)
    }

    /// Size of the bundle on disk, in bytes.
    ///
    /// Read from the filesystem rather than tracked, so it is the number the
    /// operator would see in a file listing. Zero when the file has gone
    /// missing underneath us, which is worth reporting as "unknown" rather
    /// than failing a page render.
    pub fn bundle_size(&self) -> u64 {
        fs::metadata(&self.bundle_path)
            .map(|m| m.len())
            .unwrap_or(0)
    }

    /// The bytes of an attachment stored in the bundle, decoded from base64.
    pub fn attachment(&self, path: &str) -> Option<Vec<u8>> {
        self.read(|flat| {
            let b64 = flat.get("attachments")?.get(path)?.as_str()?;
            base64::engine::general_purpose::STANDARD.decode(b64).ok()
        })
    }

    /// Per-collection entity counts, cheap enough to recompute on each request.
    pub fn counts(&self) -> Vec<(&'static str, usize)> {
        self.read(|flat| {
            COLLECTIONS
                .iter()
                .map(|&name| {
                    let n = flat
                        .get(name)
                        .and_then(Value::as_object)
                        .map(|m| m.len())
                        .unwrap_or(0);
                    (name, n)
                })
                .collect()
        })
    }
}

/// Unwrap an envelope into its `data`, turning a refusal into an error that
/// carries the library's own diagnostic text.
///
/// `data` is a `Value`, never an `Option`, so this checks `is_null()` rather
/// than unwrapping.
pub fn envelope_into_data(env: Envelope) -> Result<Value> {
    if env.status == Status::Error || env.data.is_null() {
        anyhow::bail!("{}", format_diagnostics(&env.diagnostics));
    }
    Ok(env.data)
}

/// Render diagnostics as one human-readable line.
pub fn format_diagnostics(diags: &[Diagnostic]) -> String {
    if diags.is_empty() {
        return "no diagnostics reported".to_string();
    }
    diags
        .iter()
        .map(|d| format!("{}: {}", d.code.as_str(), d.message))
        .collect::<Vec<_>>()
        .join("; ")
}

/// Export flat JSON to `.axgf` ZIP bytes.
///
/// `export_bundle` returns the archive as base64 inside `{"zip_base64": ...}`,
/// so the bytes have to be decoded back out.
pub fn export_to_bytes(flat_json: &str) -> Result<Vec<u8>> {
    let env = axgf_rs::export_bundle(flat_json);
    let data = envelope_into_data(env).context("exporting bundle")?;
    let b64 = data
        .get("zip_base64")
        .and_then(Value::as_str)
        .context("export envelope carried no zip_base64 string")?;
    base64::engine::general_purpose::STANDARD
        .decode(b64)
        .context("decoding exported bundle base64")
}

/// Write a flat bundle to `path` atomically.
///
/// Writes `path.tmp`, fsyncs it, then renames over `path`. The live file is
/// never truncated, so a crash mid-write leaves the previous bundle intact.
/// The export is performed *before* the temp file is created: if the library
/// refuses to export, the original file is never even opened.
pub fn write_bundle(path: &Path, flat: &Value) -> Result<()> {
    let bytes = export_to_bytes(&flat.to_string())?;

    let tmp = tmp_path_for(path);
    {
        let mut f = fs::File::create(&tmp)
            .with_context(|| format!("creating temp bundle {}", tmp.display()))?;
        f.write_all(&bytes)
            .with_context(|| format!("writing temp bundle {}", tmp.display()))?;
        f.sync_all()
            .with_context(|| format!("fsyncing temp bundle {}", tmp.display()))?;
    }

    fs::rename(&tmp, path)
        .with_context(|| format!("renaming {} over {}", tmp.display(), path.display()))?;

    // Best-effort: fsync the directory so the rename itself is durable. A
    // failure here does not corrupt anything, so it is logged, not fatal.
    if let Some(dir) = path.parent() {
        if let Ok(d) = fs::File::open(dir) {
            if let Err(e) = d.sync_all() {
                tracing::debug!(error = %e, "could not fsync bundle directory");
            }
        }
    }
    Ok(())
}

/// Sibling temp path used by [`write_bundle`]. Kept in the same directory so
/// the rename stays on one filesystem and therefore stays atomic.
fn tmp_path_for(path: &Path) -> PathBuf {
    let mut name = path.file_name().unwrap_or_default().to_os_string();
    name.push(".tmp");
    path.with_file_name(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmpdir() -> PathBuf {
        let base = std::env::var("CARGO_TARGET_TMPDIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| std::env::temp_dir());
        let unique = format!(
            "axgf-cms-state-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        );
        let dir = base.join(unique);
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("create temp dir");
        dir
    }

    #[test]
    fn missing_bundle_is_created_on_startup() {
        let dir = tmpdir();
        let path = dir.join("family.axgf");
        let state = AppState::load_or_create(&path, "t".into()).expect("load");
        assert!(path.exists(), "startup must write the new bundle to disk");
        assert_eq!(state.counts().iter().map(|(_, n)| n).sum::<usize>(), 0);
    }

    #[test]
    fn created_bundle_reimports_cleanly() {
        let dir = tmpdir();
        let path = dir.join("reimport.axgf");
        AppState::load_or_create(&path, "t".into()).expect("first load");
        // Reopening the file exercises export -> import round-tripping.
        let again = AppState::load_or_create(&path, "t".into()).expect("second load");
        assert_eq!(again.counts().len(), COLLECTIONS.len());
    }

    #[test]
    fn add_person_persists_and_survives_reload() {
        let dir = tmpdir();
        let path = dir.join("add.axgf");
        let state = AppState::load_or_create(&path, "t".into()).expect("load");

        let person = r#"{"identity":{"name":{"display":"Ada Lovelace"}}}"#;
        let out = state
            .mutate(|flat| axgf_rs::add_entity(flat, axgf_rs::EntityKind::Person, person))
            .expect("mutate");
        assert!(out.applied, "add should be accepted: {:?}", out.diagnostics);

        let reloaded = AppState::load_or_create(&path, "t".into()).expect("reload");
        let n = reloaded
            .counts()
            .iter()
            .find(|(k, _)| *k == "persons")
            .map(|(_, n)| *n)
            .unwrap();
        assert_eq!(n, 1, "the added person must be on disk, not just in memory");
    }

    #[test]
    fn refused_mutation_leaves_file_byte_identical() {
        let dir = tmpdir();
        let path = dir.join("refuse.axgf");
        let state = AppState::load_or_create(&path, "t".into()).expect("load");
        let before = fs::read(&path).expect("read before");

        // An entity that is not valid JSON must be refused by the library.
        let out = state
            .mutate(|flat| axgf_rs::add_entity(flat, axgf_rs::EntityKind::Person, "{not json"))
            .expect("mutate should not be an I/O error");

        assert!(!out.applied, "invalid JSON must not be applied");
        assert!(
            !out.diagnostics.is_empty(),
            "a refusal must explain itself with diagnostics"
        );
        let after = fs::read(&path).expect("read after");
        assert_eq!(before, after, "a refused mutation must not touch the file");
    }

    #[test]
    fn failed_export_leaves_the_original_file_intact() {
        let dir = tmpdir();
        let path = dir.join("intact.axgf");
        AppState::load_or_create(&path, "t".into()).expect("load");
        let before = fs::read(&path).expect("read before");

        // A structurally invalid flat bundle cannot be exported. write_bundle
        // must fail before it creates or renames anything.
        let bogus = serde_json::json!({ "manifest": "not-an-object" });
        let err = write_bundle(&path, &bogus);
        assert!(err.is_err(), "exporting a broken bundle must fail");

        let after = fs::read(&path).expect("read after");
        assert_eq!(before, after, "a failed export must not damage the bundle");
        assert!(
            !tmp_path_for(&path).exists(),
            "a failed export must not leave a temp file behind"
        );
    }

    #[test]
    fn bundle_is_found_in_both_envelope_shapes() {
        // CRUD and convert wrap the bundle; create and import return it bare.
        let wrapped = serde_json::json!({"id":"x","bundle":{"manifest":{"axgf":"1.0"}}});
        assert!(bundle_from_data(&wrapped)
            .unwrap()
            .get("manifest")
            .is_some());

        let bare = serde_json::json!({"manifest":{"axgf":"1.0"},"persons":{}});
        assert!(bundle_from_data(&bare).unwrap().get("manifest").is_some());

        // validate returns counts and no bundle; that must not be persisted.
        let counts = serde_json::json!({"errors":0,"warnings":2,"total":2});
        assert!(bundle_from_data(&counts).is_none());
    }

    #[test]
    fn an_envelope_without_a_bundle_is_not_persisted() {
        let dir = tmpdir();
        let path = dir.join("nobundle.axgf");
        let state = AppState::load_or_create(&path, "t".into()).expect("load");
        let before = fs::read(&path).expect("read before");

        // `validate` succeeds but returns counts, not a bundle. Routing it
        // through `mutate` by mistake must not damage the file.
        let out = state.mutate(axgf_rs::validate).expect("mutate");
        assert!(!out.applied);
        assert_eq!(fs::read(&path).expect("read after"), before);
    }

    #[test]
    fn temp_path_is_a_sibling_so_rename_stays_atomic() {
        let p = Path::new("/var/lib/axgf-cms/family.axgf");
        let t = tmp_path_for(p);
        assert_eq!(t.parent(), p.parent());
        assert_eq!(t.file_name().unwrap(), "family.axgf.tmp");
    }
}
