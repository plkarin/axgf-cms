//! Binary payloads on disk, so a media-heavy bundle is not held in RAM.
//!
//! A bundle's media never belongs in the process. `axgf-rs` 0.3's streaming
//! boundary is what makes that literal: [`axgf_rs::import_bundle_streaming`]
//! hands over one payload at a time as a live reader, so each one goes from
//! the archive to a file in this cache through a fixed 64 KiB buffer and is
//! never a `Vec` at all. What stays resident is the textual data — persons,
//! families, document *metadata*, the manifest — which is under a megabyte and
//! is what every page render touches.
//!
//! The reverse direction matches: [`axgf_rs::export_bundle_streaming`] asks
//! for one payload at a time and writes it straight into the open ZIP entry,
//! so writing the bundle back costs a file handle rather than a copy of the
//! media. Nothing here ever base64-encodes a payload.
//!
//! # Cache location and lifecycle
//!
//! Default `<bundle_dir>/.axgf-cms-cache/<bundle-sha>/`, overridable with
//! `--cache-dir`. Keying the directory by a hash of the bundle means a
//! different bundle can never read another's payloads, and an unchanged bundle
//! restarts against an already-populated cache without rewriting 420 MiB. The
//! cache is derived data — the `.axgf` holds the authoritative copy — so it
//! never needs backing up, it is never written inside the bundle file itself,
//! and anything missing from it can be rebuilt from the bundle.
//!
//! # What proves a cached payload is still the right bytes
//!
//! The ZIP central directory carries each entry's uncompressed size and CRC-32,
//! and a streaming import reports both *before* decompressing anything. So a
//! warm restart re-reads the cached file, computes its CRC-32, and reuses it
//! only when that matches the archive — which is a direct proof that the cache
//! holds the bundle's bytes, not merely an intact file. The sha256 recorded in
//! the index is what the document metadata is checked against, and a
//! disagreement between the two is reported rather than served silently.

use std::collections::BTreeMap;
use std::fs;
use std::io::{self, Read as _, Write as _};
use std::path::{Path, PathBuf};
use std::sync::RwLock;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};

/// Name of the manifest kept inside a cache directory.
const INDEX_FILE: &str = "index.json";

/// Copy buffer for moving a payload between the archive and the cache. Matches
/// the size `axgf-rs` uses internally, so neither side is the bottleneck.
const COPY_BUF: usize = 64 * 1024;

/// One extracted payload: the cache filename plus what verifies it.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Entry {
    /// Filename within the cache directory.
    file: String,
    sha256: String,
    size: u64,
    /// CRC-32 of the bytes, as the source archive declared them. `0` means an
    /// index written before this was recorded; such an entry cannot be proven
    /// to match the archive, so it is re-extracted once and then carries it.
    #[serde(default)]
    crc32: u32,
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

    /// Take one payload straight from a streaming import into the cache.
    ///
    /// Called once per payload by [`crate::state::AppState::load`], from inside
    /// `import_bundle_streaming`'s callback. The bytes move through a fixed
    /// buffer, so the cost of the largest photograph in the bundle is the
    /// buffer, not the photograph.
    ///
    /// A cached file whose CRC-32 already matches the archive's is left alone
    /// and the payload is not read at all — `axgf-rs` skips the entry when the
    /// callback returns without consuming it, so a warm restart never
    /// decompresses anything.
    pub fn take_payload(
        &self,
        payload: &mut axgf_rs::boundary::stream::Payload<'_>,
        report: &mut PopulateReport,
    ) -> io::Result<()> {
        let zip_path = payload.path().to_string();
        let mut index = self.index.write().unwrap_or_else(|e| e.into_inner());

        if let Some(entry) = index.get(&zip_path) {
            match self.verify_cached(entry, payload.size(), payload.crc32()) {
                Verdict::Good => {
                    report.reused += 1;
                    report.bytes_on_disk += entry.size;
                    return Ok(());
                }
                Verdict::Absent => {}
                Verdict::Stale => {
                    tracing::warn!(path = %zip_path,
                        "cached payload does not match the bundle — re-extracting");
                    report.mismatches += 1;
                }
            }
        }

        // Cold, or a cache entry that could not be proven: stream the archive
        // entry into a temp file, hashing as it passes, then rename into place.
        let file = cache_filename(&zip_path);
        let dest = self.dir.join(&file);
        let (sha, size) = copy_to_file_hashing(payload, &dest)?;
        index.insert(
            zip_path,
            Entry {
                file,
                sha256: sha,
                size,
                crc32: payload.crc32(),
            },
        );
        report.extracted += 1;
        report.bytes_on_disk += size;
        Ok(())
    }

    /// Whether a cached file can be proven to be the archive entry described by
    /// `size` and `crc32`.
    fn verify_cached(&self, entry: &Entry, size: u64, crc32: u32) -> Verdict {
        let cached = self.dir.join(&entry.file);
        let Ok(meta) = fs::metadata(&cached) else {
            return Verdict::Absent;
        };
        // An entry written before CRC-32 was recorded cannot be tied to this
        // archive, so it is re-extracted once and carries the proof afterwards.
        if entry.crc32 == 0 || entry.crc32 != crc32 || entry.size != size || meta.len() != size {
            return Verdict::Stale;
        }
        match crc32_of_file(&cached) {
            Ok(actual) if actual == crc32 => Verdict::Good,
            _ => Verdict::Stale,
        }
    }

    /// Persist the index after a streaming import has filled it.
    pub fn flush_index(&self) -> Result<()> {
        let index = self.index.read().unwrap_or_else(|e| e.into_inner());
        write_index(&self.dir, &index)
    }

    /// Check every cached payload against the sha256 the document metadata
    /// records for it, counting and logging each disagreement.
    ///
    /// Run after a streaming import, because the metadata arrives with the
    /// textual half — that is, after the payloads have already gone past. The
    /// bytes are the archive's either way; what this reports is a bundle whose
    /// file and whose record of that file do not agree.
    pub fn verify_against_metadata(&self, flat: &Value, report: &mut PopulateReport) {
        let index = self.index.read().unwrap_or_else(|e| e.into_inner());
        for (zip_path, expected) in expected_shas(flat) {
            let Some(entry) = index.get(&zip_path) else {
                continue;
            };
            if entry.sha256 != expected {
                report.mismatches += 1;
                tracing::warn!(path = %zip_path, expected = %expected, actual = %entry.sha256,
                    "payload sha256 does not match the bundle metadata — \
                     the file and its record disagree");
            }
        }
    }

    /// Open one payload for reading, for a streaming export to pull from.
    ///
    /// An `Err` here is what turns into `PAYLOAD_SOURCE_FAILED`: `axgf-rs`
    /// refuses the export rather than writing a bundle with that file missing.
    pub fn open_for_read(&self, zip_path: &str) -> io::Result<fs::File> {
        let entry = self
            .index
            .read()
            .unwrap_or_else(|e| e.into_inner())
            .get(zip_path)
            .cloned()
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("payload {zip_path} is not in the cache index"),
                )
            })?;
        fs::File::open(self.dir.join(&entry.file))
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
    ///
    /// Returns the `external_payloads` entry the bundle must declare for this
    /// path, so the caller can record it in the same write that adds the
    /// Document. A payload the cache holds but the bundle does not declare
    /// would be silently absent from the next export.
    pub fn put(&self, zip_path: &str, bytes: &[u8]) -> Result<Value> {
        let file = cache_filename(zip_path);
        write_atomic(&self.dir.join(&file), bytes)?;
        let crc32 = crc32fast::hash(bytes);
        let size = bytes.len() as u64;
        let mut index = self.index.write().unwrap_or_else(|e| e.into_inner());
        index.insert(
            zip_path.to_string(),
            Entry {
                file,
                sha256: crate::documents::sha256_hex(bytes),
                size,
                crc32,
            },
        );
        write_index(&self.dir, &index)?;
        Ok(external_payload_value(size, crc32))
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

/// What a cached file is worth against the archive entry it claims to be.
enum Verdict {
    /// Provably the same bytes: reuse without decompressing anything.
    Good,
    /// The cache file is gone; extract it.
    Absent,
    /// Present but unproven or contradicted; extract over it.
    Stale,
}

/// One `external_payloads` entry as the flat bundle carries it.
fn external_payload_value(size: u64, crc32: u32) -> Value {
    serde_json::json!({ "size_bytes": size, "crc32": crc32 })
}

/// Copy a reader into `dest` atomically, hashing as the bytes pass.
///
/// Nothing is buffered beyond [`COPY_BUF`], so a 200 MiB photograph costs 64
/// KiB. Returns the sha256 and the byte count. Written to a temp sibling and
/// renamed, so an interrupted copy never leaves a half-file the next run would
/// trust.
fn copy_to_file_hashing(src: &mut impl io::Read, dest: &Path) -> io::Result<(String, u64)> {
    let tmp = tmp_sibling(dest);
    let mut hasher = Sha256::new();
    let mut size = 0u64;
    {
        let mut out = fs::File::create(&tmp)?;
        let mut buf = vec![0u8; COPY_BUF];
        loop {
            let n = src.read(&mut buf)?;
            if n == 0 {
                break;
            }
            hasher.update(&buf[..n]);
            out.write_all(&buf[..n])?;
            size += n as u64;
        }
        out.sync_all()?;
    }
    fs::rename(&tmp, dest)?;
    Ok((hex(&hasher.finalize()), size))
}

/// CRC-32 of a file's contents, read through a fixed buffer.
fn crc32_of_file(path: &Path) -> io::Result<u32> {
    let mut f = fs::File::open(path)?;
    let mut hasher = crc32fast::Hasher::new();
    let mut buf = vec![0u8; COPY_BUF];
    loop {
        let n = f.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Ok(hasher.finalize())
}

/// Stream-hash a file to a lowercase hex sha256, without reading it all into
/// memory — the bundle can be hundreds of megabytes.
pub fn hash_file(path: &Path) -> Result<String> {
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
    Ok(hex(&hasher.finalize()))
}

/// Lowercase hex of a digest.
fn hex(bytes: &[u8]) -> String {
    use std::fmt::Write as _;
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        let _ = write!(out, "{b:02x}");
    }
    out
}

fn read_index(dir: &Path) -> Option<BTreeMap<String, Entry>> {
    let bytes = fs::read(dir.join(INDEX_FILE)).ok()?;
    serde_json::from_slice(&bytes).ok()
}

fn write_index(dir: &Path, index: &BTreeMap<String, Entry>) -> Result<()> {
    let bytes = serde_json::to_vec(index).context("serialising cache index")?;
    write_atomic(&dir.join(INDEX_FILE), &bytes)
}

/// The `.tmp` sibling a file is staged at before being renamed into place.
/// Same directory, so the rename stays on one filesystem and stays atomic.
fn tmp_sibling(path: &Path) -> PathBuf {
    let mut tmp = path.as_os_str().to_os_string();
    tmp.push(".tmp");
    PathBuf::from(tmp)
}

/// Write a file atomically: temp sibling, then rename.
fn write_atomic(path: &Path, bytes: &[u8]) -> Result<()> {
    let tmp = tmp_sibling(path);
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
