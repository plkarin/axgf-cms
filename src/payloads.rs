//! Binary payloads on disk, so a media-heavy bundle is not held in RAM.
//!
//! `import_bundle` decodes every attachment under `documents/files/**` into the
//! flat JSON's `attachments` map as base64. On the operator's 420 MiB archive
//! that is roughly 560 MiB resident before a single request is served — the
//! "held entirely in memory" design has crossed the line it was warned about.
//!
//! So at load time this module writes each attachment out to a disk cache as a
//! real file and drops it from the in-memory flat JSON. What stays resident is
//! the textual data — persons, families, document *metadata*, the manifest —
//! which is under a megabyte and is what every page render touches. Payloads
//! are streamed from disk on `GET /document/:id/raw` and re-materialised, only
//! for the duration of one call, when a `.axgf` has to be exported.
//!
//! # Cache location and lifecycle
//!
//! Default `<bundle_dir>/.axgf-cms-cache/<bundle-sha>/`, overridable with
//! `--cache-dir`. Keying the directory by a hash of the bundle means a
//! different bundle can never read another's payloads, and an unchanged bundle
//! restarts against an already-populated cache without rewriting 420 MiB. Every
//! extraction is verified against the sha256 in the document metadata; a
//! mismatch is reported loudly rather than served silently. The cache is
//! derived data — the `.axgf` holds the authoritative copy — so it never needs
//! backing up, and it is never written inside the bundle file itself.

use std::collections::BTreeMap;
use std::fs;
use std::io::{Read as _, Write as _};
use std::path::{Path, PathBuf};
use std::sync::RwLock;

use anyhow::{Context, Result};
use base64::Engine as _;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::documents::sha256_hex;

/// Name of the manifest kept inside a cache directory.
const INDEX_FILE: &str = "index.json";

/// One extracted payload: the cache filename plus what verifies it.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Entry {
    /// Filename within the cache directory.
    file: String,
    sha256: String,
    size: u64,
}

/// What one populate pass did, for the startup report.
#[derive(Debug, Default, Clone)]
pub struct PopulateReport {
    /// Payloads written to disk this time.
    pub extracted: usize,
    /// Payloads found already cached and verified, so not rewritten.
    pub reused: usize,
    /// Payloads whose bytes did not match the sha256 in the metadata.
    pub mismatches: usize,
    /// Total bytes now held on disk.
    pub bytes_on_disk: u64,
    /// The cache directory these payloads live in.
    pub cache_dir: PathBuf,
}

/// A disk-backed store of one bundle's binary payloads, keyed by ZIP path.
pub struct PayloadCache {
    dir: PathBuf,
    index: RwLock<BTreeMap<String, Entry>>,
}

impl PayloadCache {
    /// The default cache base directory beside a bundle.
    pub fn default_base(bundle_path: &Path) -> PathBuf {
        bundle_path
            .parent()
            .filter(|p| !p.as_os_str().is_empty())
            .unwrap_or_else(|| Path::new("."))
            .join(".axgf-cms-cache")
    }

    /// Open (creating if needed) the cache directory for one bundle.
    ///
    /// `base` is `<bundle_dir>/.axgf-cms-cache` by default, or the `--cache-dir`
    /// override; `bundle_sha` names a subdirectory within it.
    pub fn open(base: &Path, bundle_sha: &str) -> Result<Self> {
        let dir = base.join(bundle_sha);
        fs::create_dir_all(&dir)
            .with_context(|| format!("creating cache directory {}", dir.display()))?;
        let index = read_index(&dir).unwrap_or_default();
        Ok(Self {
            dir,
            index: RwLock::new(index),
        })
    }

    /// The cache directory in use.
    pub fn dir(&self) -> &Path {
        &self.dir
    }

    /// Move every attachment in `flat` to disk and drop the `attachments` map.
    ///
    /// A payload already cached with a matching sha256 is reused rather than
    /// rewritten, so a restart on an unchanged bundle is fast. Each extraction
    /// is verified against the document metadata's sha256, and a mismatch is
    /// counted and logged loudly.
    pub fn populate_from_flat(&self, flat: &mut Value) -> Result<PopulateReport> {
        let mut report = PopulateReport {
            cache_dir: self.dir.clone(),
            ..Default::default()
        };
        let expected = expected_shas(flat);
        // Take the attachments map out of the bundle by value rather than
        // cloning it: on a 420 MiB archive the clone alone would be another
        // ~560 MiB of transient base64. Removing it here is also what drops the
        // payloads from the resident bundle — what stays in RAM is the textual
        // data, not the media.
        let attachments = flat
            .as_object_mut()
            .and_then(|o| o.remove("attachments"))
            .and_then(|v| match v {
                Value::Object(m) => Some(m),
                _ => None,
            });

        let mut index = self.index.write().unwrap_or_else(|e| e.into_inner());
        if let Some(map) = attachments {
            for (zip_path, b64v) in map {
                let Some(b64) = b64v.as_str() else { continue };

                // Warm reuse: an existing cache file that still verifies, and
                // agrees with the metadata, is not decoded or rewritten.
                if let Some(entry) = index.get(&zip_path) {
                    let cached = self.dir.join(&entry.file);
                    if let Ok(bytes) = fs::read(&cached) {
                        let ok_self = sha256_hex(&bytes) == entry.sha256;
                        let ok_meta = expected.get(&zip_path).is_none_or(|s| s == &entry.sha256);
                        if ok_self && ok_meta {
                            report.reused += 1;
                            report.bytes_on_disk += entry.size;
                            continue;
                        }
                        tracing::warn!(path = %zip_path,
                            "cached payload failed verification — re-extracting");
                        report.mismatches += 1;
                    }
                }

                // Cold, or an invalid cache entry: decode, verify, write.
                let bytes = base64::engine::general_purpose::STANDARD
                    .decode(b64)
                    .with_context(|| format!("decoding attachment {zip_path}"))?;
                let sha = sha256_hex(&bytes);
                if let Some(exp) = expected.get(&zip_path) {
                    if exp != &sha {
                        report.mismatches += 1;
                        tracing::warn!(path = %zip_path, expected = %exp, actual = %sha,
                            "payload sha256 does not match the bundle metadata — \
                             the file and its record disagree");
                    }
                }
                let file = cache_filename(&zip_path);
                write_atomic(&self.dir.join(&file), &bytes)?;
                let size = bytes.len() as u64;
                index.insert(
                    zip_path,
                    Entry {
                        file,
                        sha256: sha,
                        size,
                    },
                );
                report.extracted += 1;
                report.bytes_on_disk += size;
            }
        }

        write_index(&self.dir, &index)?;
        drop(index);
        Ok(report)
    }

    /// Read one payload's bytes from disk.
    pub fn read(&self, zip_path: &str) -> Option<Vec<u8>> {
        let entry = self
            .index
            .read()
            .unwrap_or_else(|e| e.into_inner())
            .get(zip_path)
            .cloned()?;
        fs::read(self.dir.join(entry.file)).ok()
    }

    /// The on-disk path of a payload, for streaming a response body.
    pub fn path_of(&self, zip_path: &str) -> Option<PathBuf> {
        let entry = self
            .index
            .read()
            .unwrap_or_else(|e| e.into_inner())
            .get(zip_path)
            .cloned()?;
        let p = self.dir.join(entry.file);
        p.exists().then_some(p)
    }

    /// Store a payload straight to the cache. Used by uploads, so a new file
    /// never transits through the in-memory bundle.
    pub fn put(&self, zip_path: &str, bytes: &[u8]) -> Result<()> {
        let file = cache_filename(zip_path);
        write_atomic(&self.dir.join(&file), bytes)?;
        let mut index = self.index.write().unwrap_or_else(|e| e.into_inner());
        index.insert(
            zip_path.to_string(),
            Entry {
                file,
                sha256: sha256_hex(bytes),
                size: bytes.len() as u64,
            },
        );
        write_index(&self.dir, &index)
    }

    /// Build the `attachments` map for one export call: ZIP path -> base64,
    /// read from disk. Meant to be spliced into a flat clone for the export and
    /// dropped immediately after — never kept resident between requests.
    pub fn attachments_value(&self) -> Value {
        let index = self.index.read().unwrap_or_else(|e| e.into_inner());
        let mut map = serde_json::Map::new();
        for (zip_path, entry) in index.iter() {
            if let Ok(bytes) = fs::read(self.dir.join(&entry.file)) {
                map.insert(
                    zip_path.clone(),
                    Value::String(base64::engine::general_purpose::STANDARD.encode(&bytes)),
                );
            }
        }
        Value::Object(map)
    }

    /// True when no payloads are cached.
    pub fn is_empty(&self) -> bool {
        self.index.read().map(|i| i.is_empty()).unwrap_or(true)
    }

    /// How many payloads are cached.
    pub fn len(&self) -> usize {
        self.index.read().map(|i| i.len()).unwrap_or(0)
    }
}

/// Expected sha256 per ZIP path, taken from the document metadata, which is the
/// authority the payload files are checked against.
fn expected_shas(flat: &Value) -> BTreeMap<String, String> {
    let mut m = BTreeMap::new();
    if let Some(docs) = flat.get("documents").and_then(Value::as_object) {
        for d in docs.values() {
            let file = d.get("file");
            let path = file.and_then(|f| f.get("path")).and_then(Value::as_str);
            let sha = file.and_then(|f| f.get("sha256")).and_then(Value::as_str);
            if let (Some(p), Some(s)) = (path, sha) {
                m.insert(p.to_string(), s.to_string());
            }
        }
    }
    m
}

/// A filesystem-safe filename for a ZIP path. Document ids are UUIDs, so the
/// flattened path is unique; any character that is not plainly safe becomes an
/// underscore.
fn cache_filename(zip_path: &str) -> String {
    zip_path
        .chars()
        .map(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '.' | '-' | '_' => c,
            _ => '_',
        })
        .collect()
}

/// Return freed heap memory to the operating system.
///
/// `import_bundle` materialises every payload as base64 (~560 MiB on the
/// operator's archive) before this module writes them to disk and drops them.
/// The Rust allocations are freed, but glibc's allocator keeps that freed heap
/// mapped by default, so the process RSS stays high even though the live data
/// is about a megabyte. `malloc_trim` hands the top of the heap back to the
/// kernel, so RSS reflects what is actually resident. A no-op off glibc.
#[allow(unsafe_code)]
pub fn release_freed_memory() {
    #[cfg(target_env = "gnu")]
    // SAFETY: malloc_trim takes an integer pad and only releases already-free
    // memory back to the OS; it cannot invalidate or move any live allocation,
    // so no Rust reference is affected.
    unsafe {
        libc::malloc_trim(0);
    }
}

/// Stream-hash a file to a lowercase hex sha256, without reading it all into
/// memory — the bundle can be hundreds of megabytes.
pub fn hash_file(path: &Path) -> Result<String> {
    use sha2::{Digest, Sha256};
    let mut f = fs::File::open(path).with_context(|| format!("opening {}", path.display()))?;
    let mut hasher = Sha256::new();
    let mut buf = vec![0u8; 1 << 20];
    loop {
        let n = f
            .read(&mut buf)
            .with_context(|| format!("reading {}", path.display()))?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    let digest = hasher.finalize();
    let mut out = String::with_capacity(64);
    for b in digest {
        use std::fmt::Write as _;
        let _ = write!(out, "{b:02x}");
    }
    Ok(out)
}

fn read_index(dir: &Path) -> Option<BTreeMap<String, Entry>> {
    let bytes = fs::read(dir.join(INDEX_FILE)).ok()?;
    serde_json::from_slice(&bytes).ok()
}

fn write_index(dir: &Path, index: &BTreeMap<String, Entry>) -> Result<()> {
    let bytes = serde_json::to_vec(index).context("serialising cache index")?;
    write_atomic(&dir.join(INDEX_FILE), &bytes)
}

/// Write a file atomically: temp sibling, then rename.
fn write_atomic(path: &Path, bytes: &[u8]) -> Result<()> {
    let mut tmp = path.as_os_str().to_os_string();
    tmp.push(".tmp");
    let tmp = PathBuf::from(tmp);
    {
        let mut f =
            fs::File::create(&tmp).with_context(|| format!("creating {}", tmp.display()))?;
        f.write_all(bytes)
            .with_context(|| format!("writing {}", tmp.display()))?;
        f.sync_all()
            .with_context(|| format!("fsyncing {}", tmp.display()))?;
    }
    fs::rename(&tmp, path)
        .with_context(|| format!("renaming {} to {}", tmp.display(), path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cache_filename_is_filesystem_safe_and_unique() {
        assert_eq!(
            cache_filename("documents/files/abc-123.jpg"),
            "documents_files_abc-123.jpg"
        );
        // Two different ids stay different.
        assert_ne!(
            cache_filename("documents/files/a.jpg"),
            cache_filename("documents/files/b.jpg")
        );
    }

    #[test]
    fn round_trips_a_payload_through_the_cache() {
        let base = std::env::temp_dir().join(format!("axgf-pc-{}", std::process::id()));
        let cache = PayloadCache::open(&base, "deadbeef").expect("open");
        cache.put("documents/files/x.txt", b"hello").expect("put");
        assert_eq!(
            cache.read("documents/files/x.txt").as_deref(),
            Some(&b"hello"[..])
        );
        assert!(cache.path_of("documents/files/x.txt").is_some());
        let _ = fs::remove_dir_all(&base);
    }
}
