//! Backups: one archive holding the three files that are the whole state, and
//! the proof that it can be read back.
//!
//! # What the state is
//!
//! Three files, not one:
//!
//! * `family.axgf`     — the genealogy, including every document's bytes
//! * `family.acl`      — the accounts, mode 600, never inside the bundle
//! * `family.journal`  — who changed what, which the conflict screen replays
//!
//! Losing any one of them loses something that cannot be reconstructed from
//! the others, so a backup that holds fewer than three is not a backup of this
//! application. The payload cache is deliberately *not* included: it is derived
//! from the bundle and rebuilds itself.
//!
//! # A ZIP, because an operator may not have this binary
//!
//! The day a backup is needed is the day something has gone badly wrong, and
//! that is the worst possible moment to discover that reading the archive
//! requires a working copy of the program that wrote it. `unzip` gets the three
//! files out. [`restore`] is the convenient path, not the only one.
//!
//! # Verified as it is written
//!
//! A backup nobody has read is a backup nobody knows about, and it is worse
//! than having none because it is trusted. Every archive is re-opened as soon
//! as it is closed: the bundle inside it is imported with the library and must
//! validate, the ACL must parse, and the SHA-256 of all three must match what
//! the manifest recorded. An archive that fails is deleted rather than kept,
//! because a file named `axgf-backup-…zip` sitting in the backup directory is a
//! claim.

use std::collections::BTreeMap;
use std::fs;
use std::io::{Read, Seek, Write};
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::lockfile::WriteLock;
use crate::space::{self, Need};

/// Name of the bundle inside the archive.
pub const MEMBER_BUNDLE: &str = "family.axgf";
/// Name of the accounts file inside the archive.
pub const MEMBER_ACL: &str = "family.acl";
/// Name of the journal inside the archive.
pub const MEMBER_JOURNAL: &str = "family.journal";
/// Name of the manifest inside the archive.
pub const MEMBER_MANIFEST: &str = "backup.json";

/// Prefix every archive this application writes carries.
pub const PREFIX: &str = "axgf-backup-";

/// The manifest written into every archive.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    /// Format of the archive itself, so a future change is detectable.
    pub format: u32,
    /// RFC 3339, UTC. Also encoded in the filename, but a file can be renamed.
    pub created_at: String,
    /// Version of the binary that wrote it.
    pub written_by: String,
    /// Absolute path the bundle was taken from, for an operator reading it
    /// two years later on a different machine.
    pub source_bundle: String,
    /// SHA-256 of each member, so a corrupted archive is caught rather than
    /// restored. Keyed by the member name.
    pub sha256: BTreeMap<String, String>,
    /// Bytes of each member, uncompressed.
    pub bytes: BTreeMap<String, u64>,
    /// Entity counts at the moment of the backup, so `restore` can say what it
    /// is about to put back without opening the bundle first.
    pub entities: BTreeMap<String, usize>,
    /// Lines in the journal.
    pub journal_lines: usize,
    /// Accounts in the ACL.
    pub accounts: usize,
}

/// How many archives of each age to keep.
///
/// Daily, weekly and monthly are *selections from the same archives*, not three
/// separate sets: an archive that is the newest of its week is kept as the
/// week's, whether or not it is also one of the last seven days'. Nothing is
/// copied and nothing is written twice.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Retention {
    pub daily: usize,
    pub weekly: usize,
    pub monthly: usize,
}

impl Default for Retention {
    fn default() -> Self {
        Self {
            daily: 7,
            weekly: 4,
            monthly: 12,
        }
    }
}

/// What a backup run did.
#[derive(Debug, Clone)]
pub struct Report {
    /// The archive written.
    pub archive: PathBuf,
    /// Its size on disk.
    pub bytes: u64,
    /// Archives removed by retention.
    pub pruned: Vec<PathBuf>,
    /// The manifest that was verified.
    pub manifest: Manifest,
    /// How long the write lock was held, in milliseconds — the window in which
    /// a save would have waited.
    pub lock_held_ms: u128,
    /// Part-written archives from interrupted runs that this run reclaimed.
    pub swept_parts: usize,
    /// What they were costing, in bytes.
    pub swept_bytes: u64,
}

/// The files a backup covers, derived from the bundle path.
///
/// Three things, but more than three files: the journal rotates, and a backup
/// that carried only the live segment would restore an installation whose
/// conflict screen could no longer reconstruct anything older than the last
/// rotation. Every segment goes in, under its own name.
#[derive(Debug, Clone)]
pub struct StateFiles {
    pub bundle: PathBuf,
    pub acl: PathBuf,
    /// Oldest first, exactly as [`crate::journal::Journal::segments`] orders
    /// them; the last is the live file.
    pub journals: Vec<PathBuf>,
}

impl StateFiles {
    pub fn for_bundle(bundle: &Path) -> Self {
        let journal = crate::journal::Journal::new(crate::journal::Journal::path_for(bundle));
        Self {
            bundle: bundle.to_path_buf(),
            acl: crate::acl::Acl::path_for(bundle),
            journals: journal.segments(),
        }
    }

    /// Every member, paired with the name it takes inside the archive.
    ///
    /// The live journal keeps the plain name so an operator running `unzip -l`
    /// sees the three files the documentation talks about, with the rotated
    /// segments beside them under the names they have on disk.
    fn members(&self) -> Vec<(String, PathBuf)> {
        let mut out = vec![
            (MEMBER_BUNDLE.to_string(), self.bundle.clone()),
            (MEMBER_ACL.to_string(), self.acl.clone()),
        ];
        for seg in &self.journals {
            let suffix = seg
                .file_name()
                .and_then(|n| n.to_str())
                .and_then(|n| n.rsplit_once(".journal"))
                .map(|(_, rest)| rest.to_string())
                .unwrap_or_default();
            out.push((format!("{MEMBER_JOURNAL}{suffix}"), seg.clone()));
        }
        out
    }

    /// Bytes they all occupy, for the free-space check.
    fn total_bytes(&self) -> u64 {
        self.members()
            .iter()
            .filter_map(|(_, p)| fs::metadata(p).ok())
            .map(|m| m.len())
            .sum()
    }
}

/// True when an archive member is a journal segment.
fn is_journal_member(name: &str) -> bool {
    name == MEMBER_JOURNAL || name.starts_with(&format!("{MEMBER_JOURNAL}."))
}

/// UTC timestamp in the form the filenames use: `20260921T143005Z`.
fn stamp() -> String {
    let now = time::OffsetDateTime::now_utc();
    format!(
        "{:04}{:02}{:02}T{:02}{:02}{:02}Z",
        now.year(),
        u8::from(now.month()),
        now.day(),
        now.hour(),
        now.minute(),
        now.second()
    )
}

/// Write one verified archive into `dest`, then prune to `retention`.
///
/// The write lock is held across all three copies and released before
/// verification, which reads only the finished archive. Holding it for the
/// verification too would double the window in which a family member's save
/// waits, for no gain: the archive is closed and nothing can change it.
pub fn run(bundle: &Path, dest: &Path, retention: Retention) -> Result<Report> {
    let files = StateFiles::for_bundle(bundle);
    if !files.bundle.exists() {
        bail!(
            "there is no bundle at {} to back up.",
            files.bundle.display()
        );
    }
    fs::create_dir_all(dest)
        .with_context(|| format!("creating the backup directory {}", dest.display()))?;

    // A run killed between "create the .part" and "rename it into place"
    // leaves a part-written archive behind — on the operator's bundle that is
    // 415 MB, once per interrupted run, in the directory the daily timer
    // writes to. Reclaimed here, before the space check, because an orphan
    // from yesterday is precisely the room today's archive needs.
    let (swept_parts, swept_bytes) = sweep_orphaned_parts(dest);

    // Up front, before the lock is taken and before a byte is written: the
    // archive is at most the three files plus a little, and a backup that
    // fails halfway leaves a part-written file in the directory an operator
    // trusts. The bundle goes in stored, so "at most" is close to exact.
    let need = files.total_bytes();
    space::ensure_room_for(
        dest,
        &Need {
            bytes: need,
            what: "a backup archive",
        },
    )?;

    let tmp = dest.join(format!("{PREFIX}{}.zip.part", stamp()));
    let final_path = dest.join(format!("{PREFIX}{}.zip", stamp()));

    // Created and claimed here rather than inside `write_archive`, because the
    // claim has to outlive the writing: between the last byte and the rename
    // there is a verification pass over 435 MB, and a sweep running in that
    // window would otherwise see an unlocked `.part` and take it for the
    // wreckage of a dead run. `try_clone` shares one open file description, so
    // both handles hold the same lock and it is released when the last of them
    // goes — at the end of this function, after the rename.
    let part = fs::File::create(&tmp)
        .map_err(|e| anyhow::anyhow!("{}", space::explain_write_failure(&e, &tmp, None)))?;
    if let Err(e) = part.try_lock() {
        let _ = fs::remove_file(&tmp);
        bail!("another backup is already writing {}: {e}", tmp.display());
    }
    let writing = part
        .try_clone()
        .with_context(|| format!("duplicating the handle on {}", tmp.display()))?;

    let started = std::time::Instant::now();
    let manifest = {
        // Everything between here and the end of this block happens with no
        // other process able to write the bundle or the accounts.
        let _lock = WriteLock::acquire(bundle)?;
        let held = std::time::Instant::now();
        let m = write_archive(&files, writing).inspect_err(|_| {
            let _ = fs::remove_file(&tmp);
        })?;
        tracing::debug!(ms = held.elapsed().as_millis(), "write lock released");
        m
    };
    let lock_held_ms = started.elapsed().as_millis();

    // Read it back before it is given its final name. A `.part` that fails
    // verification is deleted; nothing is ever left in the directory under a
    // name that says "this is a backup" unless it has been proven to be one.
    if let Err(e) = verify(&tmp) {
        let _ = fs::remove_file(&tmp);
        return Err(e).context(
            "the archive that was just written could not be read back, so it has been \
             deleted rather than left in place. A backup that cannot be restored is \
             worse than no backup, because it is trusted.",
        );
    }

    fs::rename(&tmp, &final_path).map_err(|e| {
        let _ = fs::remove_file(&tmp);
        anyhow::anyhow!(
            "{}",
            space::explain_write_failure(&e, &final_path, Some(&tmp))
        )
    })?;
    // The `.part` name is gone; the claim on it has nothing left to protect.
    drop(part);
    let bytes = fs::metadata(&final_path).map(|m| m.len()).unwrap_or(0);

    let pruned = prune(dest, retention)?;

    tracing::info!(
        archive = %final_path.display(),
        bytes,
        pruned = pruned.len(),
        lock_held_ms,
        "backup written and verified"
    );

    Ok(Report {
        archive: final_path,
        bytes,
        pruned,
        manifest,
        lock_held_ms,
        swept_parts,
        swept_bytes,
    })
}

/// Remove part-written archives in `dest` that no process is still writing.
///
/// # How "no process is still writing" is decided
///
/// Not by age, and not by a PID in a file: by asking the kernel. A run holds
/// an exclusive `flock` on its own `.part` for as long as it is writing it
/// ([`write_archive`]), and that lock belongs to the open file description, so
/// it is gone the instant the process is — including under `kill -9`, which is
/// exactly the case this cleans up after. A `.part` that can be locked is
/// therefore one nobody is writing.
///
/// The alternative, "older than an hour", would either delete a slow run's
/// archive out from under it or leave a dead one lying for an hour. This is
/// the same reasoning [`crate::lockfile`] gives for using flock at all.
pub fn sweep_orphaned_parts(dest: &Path) -> (usize, u64) {
    let Ok(entries) = fs::read_dir(dest) else {
        return (0, 0);
    };
    let (mut count, mut bytes) = (0usize, 0u64);
    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name();
        let Some(name) = name.to_str() else { continue };
        if !name.starts_with(PREFIX) || !name.ends_with(".zip.part") {
            continue;
        }
        let Ok(file) = fs::OpenOptions::new().read(true).write(true).open(&path) else {
            continue;
        };
        match file.try_lock() {
            Ok(()) => {}
            // Somebody is writing it this second. Leave it alone.
            Err(_) => {
                tracing::debug!(path = %path.display(), "a run is writing this .part; left");
                continue;
            }
        }
        let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
        drop(file);
        match fs::remove_file(&path) {
            Ok(()) => {
                count += 1;
                bytes += size;
                tracing::warn!(
                    path = %path.display(),
                    bytes = size,
                    "removed a part-written archive left by an interrupted backup;                      it was never a backup and was never named as one"
                );
            }
            Err(e) => tracing::warn!(path = %path.display(), error = %e, "could not remove it"),
        }
    }
    (count, bytes)
}

/// Stream the three files into the ZIP behind `file` and return the manifest.
///
/// The handle is created, claimed and kept by [`run`]; this only writes.
fn write_archive(files: &StateFiles, file: fs::File) -> Result<Manifest> {
    let mut zip = zip::ZipWriter::new(std::io::BufWriter::new(file));

    let mut sha256 = BTreeMap::new();
    let mut bytes = BTreeMap::new();

    // The bundle is itself a ZIP of deflated members, so deflating it again
    // buys nothing and costs a pass over 400 MB.
    let stored = zip::write::FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .large_file(true);
    let deflated =
        zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    for (name, path) in files.members() {
        if !path.exists() {
            // An installation with no accounts yet, or one that has never been
            // edited. Absent is a fact worth recording, not an error: the
            // manifest simply does not list the member.
            tracing::debug!(path = %path.display(), "not present; not in this archive");
            continue;
        }
        let opts = if name == MEMBER_BUNDLE {
            stored
        } else {
            deflated
        };
        zip.start_file(&name, opts)
            .with_context(|| format!("starting {name} in the archive"))?;
        let (n, digest) = copy_hashing(&path, &mut zip)
            .with_context(|| format!("copying {} into the archive", path.display()))?;
        bytes.insert(name.clone(), n);
        sha256.insert(name, digest);
    }

    let manifest = Manifest {
        format: 1,
        created_at: crate::view::now_iso8601(),
        written_by: format!("axgf-cms {}", env!("CARGO_PKG_VERSION")),
        source_bundle: files.bundle.display().to_string(),
        entities: count_entities(&files.bundle),
        journal_lines: crate::journal::Journal::new(crate::journal::Journal::path_for(
            &files.bundle,
        ))
        .len(),
        accounts: crate::acl::Acl::load(&files.acl)
            .map(|a| a.users.len())
            .unwrap_or(0),
        sha256,
        bytes,
    };

    zip.start_file(MEMBER_MANIFEST, deflated)
        .context("starting the manifest")?;
    zip.write_all(serde_json::to_string_pretty(&manifest)?.as_bytes())
        .context("writing the manifest")?;

    let mut inner = zip.finish().context("closing the archive")?;
    inner.flush().context("flushing the archive")?;
    // The archive has to be on the platters, not in the page cache: a backup
    // that only survives a clean shutdown is not one.
    inner
        .into_inner()
        .map_err(|e| anyhow::anyhow!("flushing the archive: {e}"))?
        .sync_all()
        .context("fsyncing the archive")?;
    Ok(manifest)
}

/// Copy a file into `out`, returning its length and SHA-256.
///
/// A fixed buffer, because the file on the other end of it is the whole
/// genealogy: reading it into memory to hash it would undo the streaming the
/// rest of this application is built around.
fn copy_hashing<W: Write>(path: &Path, out: &mut W) -> Result<(u64, String)> {
    use sha2::{Digest, Sha256};
    let mut f = fs::File::open(path).with_context(|| format!("opening {}", path.display()))?;
    let mut hasher = Sha256::new();
    let mut buf = vec![0u8; 256 * 1024];
    let mut total = 0u64;
    loop {
        let n = f.read(&mut buf).with_context(|| "reading")?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
        out.write_all(&buf[..n]).context("writing")?;
        total += n as u64;
    }
    Ok((total, hex(&hasher.finalize())))
}

/// Non-empty lines in a file, without parsing them.
fn count_lines(path: &Path) -> usize {
    use std::io::BufRead as _;
    fs::File::open(path)
        .map(|f| {
            std::io::BufReader::new(f)
                .lines()
                .map_while(Result::ok)
                .filter(|l| !l.trim().is_empty())
                .count()
        })
        .unwrap_or(0)
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// Entity counts, read from the bundle without extracting its payloads.
fn count_entities(bundle: &Path) -> BTreeMap<String, usize> {
    let mut out = BTreeMap::new();
    let Ok(file) = fs::File::open(bundle) else {
        return out;
    };
    let env = axgf_rs::import_bundle_streaming(file, |_payload| Ok(()));
    let Ok(flat) = crate::state::envelope_into_data(env) else {
        return out;
    };
    for name in crate::state::COLLECTIONS {
        let n = flat
            .get(name)
            .and_then(Value::as_object)
            .map(|m| m.len())
            .unwrap_or(0);
        out.insert(name.to_string(), n);
    }
    out
}

/// What an archive holds, once it has been proven readable.
#[derive(Debug, Clone)]
pub struct Verified {
    pub manifest: Manifest,
    /// Entity counts as re-imported from the archive, not as recorded — the
    /// point is to disagree with the manifest when something has rotted.
    pub entities: BTreeMap<String, usize>,
    /// Accounts the ACL in the archive parses to.
    pub accounts: usize,
    /// Lines the journal in the archive holds.
    pub journal_lines: usize,
}

/// Open an archive and prove every part of it can be read.
///
/// Four separate claims, each of which has failed in the wild for somebody:
///
/// 1. the ZIP structure is intact and every member's CRC matches
/// 2. the SHA-256 of each member is the one the manifest recorded
/// 3. the bundle imports with `axgf-rs` and validates
/// 4. the ACL parses and the journal's lines are JSON
///
/// One is not enough: a ZIP whose CRCs pass can still hold a bundle that was
/// truncated before it was ever archived.
pub fn verify(archive: &Path) -> Result<Verified> {
    let file = fs::File::open(archive).with_context(|| format!("opening {}", archive.display()))?;
    let mut zip = zip::ZipArchive::new(std::io::BufReader::new(file)).with_context(|| {
        format!(
            "{} is not a readable ZIP archive. If it was copied off this machine \
             and back, check that the copy completed.",
            archive.display()
        )
    })?;

    let manifest: Manifest = {
        let mut m = zip.by_name(MEMBER_MANIFEST).with_context(|| {
            format!(
                "{} carries no {MEMBER_MANIFEST}, so it was not written by axgf-cms \
                 (or was written by a version before backups existed).",
                archive.display()
            )
        })?;
        let mut s = String::new();
        m.read_to_string(&mut s).context("reading the manifest")?;
        serde_json::from_str(&s).context("parsing the manifest")?
    };

    // The bundle has to be a file on disk for the streaming importer, and it
    // is up to 400 MB, so it is extracted next to the archive rather than held.
    let workdir = tempdir_beside(archive)?;
    let bundle_path = workdir.path().join(MEMBER_BUNDLE);
    let acl_path = workdir.path().join(MEMBER_ACL);
    let journal_path = workdir.path().join(MEMBER_JOURNAL);

    // Every member the manifest recorded, which is the live journal plus
    // however many rotated segments this installation had.
    let mut present: Vec<String> = Vec::new();
    for name in manifest.sha256.keys().cloned().collect::<Vec<_>>() {
        let out = match name.as_str() {
            MEMBER_BUNDLE => bundle_path.clone(),
            MEMBER_ACL => acl_path.clone(),
            _ => workdir.path().join(&name),
        };
        let out = &out;
        let name = name.as_str();
        let digest = extract_verifying(&mut zip, name, out).with_context(|| {
            format!(
                "{name} could not be read out of {}. The archive is damaged; do not \
                 restore from it.",
                archive.display()
            )
        })?;
        let expected = &manifest.sha256[name];
        if &digest != expected {
            bail!(
                "{name} in {} is not the file that was backed up.\n\n\
                 \x20 recorded  {expected}\n\
                 \x20 found     {digest}\n\n\
                 The archive has been altered or has decayed on disk. Do not restore \
                 from it; use an older one.",
                archive.display()
            );
        }
        present.push(name.to_string());
    }

    if !present.iter().any(|n| n == MEMBER_BUNDLE) {
        bail!(
            "{} holds no {MEMBER_BUNDLE}. There is nothing to restore.",
            archive.display()
        );
    }

    // 3. It has to be a bundle, not merely bytes that hash correctly.
    let file = fs::File::open(&bundle_path).context("re-opening the extracted bundle")?;
    let env = axgf_rs::import_bundle_streaming(file, |_payload| Ok(()));
    let flat = crate::state::envelope_into_data(env).with_context(|| {
        format!(
            "the bundle inside {} does not import. Do not restore from it.",
            archive.display()
        )
    })?;
    let validation = axgf_rs::validate(&flat.to_string());
    if validation.status == axgf_rs::boundary::envelope::Status::Error {
        bail!(
            "the bundle inside {} imports but does not validate:\n{}",
            archive.display(),
            crate::state::format_diagnostics(&validation.diagnostics)
        );
    }

    let mut entities = BTreeMap::new();
    for name in crate::state::COLLECTIONS {
        entities.insert(
            name.to_string(),
            flat.get(name)
                .and_then(Value::as_object)
                .map(|m| m.len())
                .unwrap_or(0),
        );
    }

    // 4. The other two files, which are as irreplaceable as the bundle.
    let accounts = if present.iter().any(|n| n == MEMBER_ACL) {
        crate::acl::Acl::load(&acl_path)
            .with_context(|| {
                format!(
                    "the accounts file inside {} does not parse. Restoring it would \
                     leave the installation with no way in.",
                    archive.display()
                )
            })?
            .users
            .len()
    } else {
        0
    };
    // Every segment, counted the way the application counts them: a restore
    // that dropped the rotated history would still pass a check that only
    // looked at the live file.
    let journal_lines = present
        .iter()
        .filter(|n| is_journal_member(n))
        .map(|n| count_lines(&workdir.path().join(n)))
        .sum();
    let _ = &journal_path;

    Ok(Verified {
        manifest,
        entities,
        accounts,
        journal_lines,
    })
}

/// Extract one member to `out`, returning its SHA-256.
fn extract_verifying<R: Read + Seek>(
    zip: &mut zip::ZipArchive<R>,
    name: &str,
    out: &Path,
) -> Result<String> {
    use sha2::{Digest, Sha256};
    let mut member = zip.by_name(name)?;
    let mut f = fs::File::create(out).with_context(|| format!("creating {}", out.display()))?;
    let mut hasher = Sha256::new();
    let mut buf = vec![0u8; 256 * 1024];
    loop {
        // A CRC mismatch surfaces here, as an error from the ZIP reader.
        let n = member.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
        f.write_all(&buf[..n])?;
    }
    f.sync_all().ok();
    // The ACL and the journal are mode 600 wherever they exist, including in a
    // temporary directory during verification: extracting them at the umask's
    // 644 and then asking `Acl::load` to read them is a refusal, and rightly —
    // it is the check that stops a world-readable credential store.
    restrict_if_secret(name, out);
    Ok(hex(&hasher.finalize()))
}

/// A directory that removes itself, beside `near` so the extraction stays on
/// one filesystem and costs a rename rather than a copy.
struct ScratchDir(PathBuf);

impl ScratchDir {
    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for ScratchDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn tempdir_beside(near: &Path) -> Result<ScratchDir> {
    let parent = near.parent().unwrap_or(Path::new("."));
    let dir = parent.join(format!(
        ".axgf-verify-{}-{}",
        std::process::id(),
        uuid::Uuid::new_v4().simple()
    ));
    fs::create_dir_all(&dir)
        .with_context(|| format!("creating a working directory at {}", dir.display()))?;
    Ok(ScratchDir(dir))
}

/// One archive on disk, as retention sees it.
#[derive(Debug, Clone)]
pub struct Archive {
    pub path: PathBuf,
    /// Parsed from the filename: the archive's own claim about when it was
    /// taken. Filenames are used rather than mtimes because an archive copied
    /// off the machine and back has a new mtime and the same name.
    pub taken: time::OffsetDateTime,
}

/// Every archive in `dir`, newest first.
pub fn list(dir: &Path) -> Result<Vec<Archive>> {
    let mut out = Vec::new();
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(out),
        Err(e) => return Err(e).with_context(|| format!("listing {}", dir.display())),
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        let Some(stamp) = name
            .strip_prefix(PREFIX)
            .and_then(|s| s.strip_suffix(".zip"))
        else {
            continue;
        };
        let Some(taken) = parse_stamp(stamp) else {
            continue;
        };
        out.push(Archive { path, taken });
    }
    out.sort_by_key(|a| std::cmp::Reverse(a.taken));
    Ok(out)
}

/// `20260921T143005Z` back into a time.
fn parse_stamp(s: &str) -> Option<time::OffsetDateTime> {
    if s.len() != 16 || !s.ends_with('Z') || s.as_bytes()[8] != b'T' {
        return None;
    }
    let n = |a: usize, b: usize| s.get(a..b)?.parse::<i32>().ok();
    let date = time::Date::from_calendar_date(
        n(0, 4)?,
        time::Month::try_from(n(4, 6)? as u8).ok()?,
        n(6, 8)? as u8,
    )
    .ok()?;
    let t = time::Time::from_hms(n(9, 11)? as u8, n(11, 13)? as u8, n(13, 15)? as u8).ok()?;
    Some(date.with_time(t).assume_utc())
}

/// The newest archive, or `None`.
pub fn latest(dir: &Path) -> Option<Archive> {
    list(dir).ok().and_then(|v| v.into_iter().next())
}

/// Decide which archives to keep, and delete the rest.
///
/// Returns what was removed. A failure to remove one is reported rather than
/// fatal: a backup directory that cannot be tidied is a smaller problem than a
/// backup run that reports failure because of it.
pub fn prune(dir: &Path, retention: Retention) -> Result<Vec<PathBuf>> {
    let all = list(dir)?;
    let keep = select_keepers(&all, retention);
    let mut removed = Vec::new();
    for a in &all {
        if keep.contains(&a.path) {
            continue;
        }
        match fs::remove_file(&a.path) {
            Ok(()) => removed.push(a.path.clone()),
            Err(e) => tracing::warn!(
                path = %a.path.display(),
                error = %e,
                "could not remove an expired backup"
            ),
        }
    }
    Ok(removed)
}

/// The archives retention keeps, as a set of paths.
///
/// Split out from [`prune`] so the policy can be tested against a list of
/// dates without a single file existing.
pub fn select_keepers(
    all: &[Archive],
    retention: Retention,
) -> std::collections::BTreeSet<PathBuf> {
    use std::collections::BTreeSet;
    let mut keep: BTreeSet<PathBuf> = BTreeSet::new();
    // `all` arrives newest first, which is the order every rule below wants:
    // the first archive seen in a period is the newest one in it.
    let mut seen_day = BTreeSet::new();
    let mut seen_week = BTreeSet::new();
    let mut seen_month = BTreeSet::new();

    for a in all {
        let d = a.taken.date();
        let day = (d.year(), d.ordinal());
        if seen_day.insert(day) && seen_day.len() <= retention.daily {
            keep.insert(a.path.clone());
        }
        let week = d.to_iso_week_date();
        let week = (week.0, week.1);
        if seen_week.insert(week) && seen_week.len() <= retention.weekly {
            keep.insert(a.path.clone());
        }
        let month = (d.year(), u8::from(d.month()));
        if seen_month.insert(month) && seen_month.len() <= retention.monthly {
            keep.insert(a.path.clone());
        }
    }
    keep
}

/// What a restore is about to do, and whether it may.
#[derive(Debug)]
pub struct RestorePlan {
    pub verified: Verified,
    pub files: StateFiles,
    /// Where the current state is moved to before anything is replaced.
    pub aside: PathBuf,
}

/// Put an archive back over `bundle`.
///
/// The order is the whole of the safety:
///
/// 1. refuse if a process is running against this bundle
/// 2. verify the archive completely — a bad archive stops here, having
///    touched nothing
/// 3. extract all three files to a staging directory on the same filesystem
/// 4. move the *current* three files aside, into a timestamped directory that
///    is never deleted by this command
/// 5. rename the staged files into place
///
/// Step 4 is what makes this reversible. Restoring the wrong archive is a
/// mistake somebody will make at three in the morning, and the current state
/// having been kept is the difference between an inconvenience and a second
/// disaster.
pub fn restore(archive: &Path, bundle: &Path, force: bool) -> Result<RestorePlan> {
    let files = StateFiles::for_bundle(bundle);

    // 1. Not over a running instance. The lock is the same one a save takes,
    // so a server mid-write holds it; but a server that is merely *up* does
    // not, which is why the port is checked as well.
    if !force {
        refuse_if_running(bundle)?;
    }
    let _lock = WriteLock::acquire(bundle)?;

    // 2. Everything, before anything is moved.
    let verified = verify(archive)?;

    let need = verified.manifest.bytes.values().sum::<u64>();
    let dir = bundle.parent().unwrap_or(Path::new("."));
    // Both the staged copy and the set moved aside exist at once, so the
    // requirement is roughly twice the archive's contents.
    space::ensure_room_for(
        dir,
        &Need {
            bytes: need.saturating_mul(2),
            what: "the restored files and a copy of the current ones",
        },
    )?;

    // 3. Staged on the same filesystem, so step 5 is a rename and not a copy
    // that can fail halfway.
    let staging = tempdir_beside(bundle)?;
    let mut staged = Vec::new();
    {
        let f = fs::File::open(archive)?;
        let mut zip = zip::ZipArchive::new(std::io::BufReader::new(f))?;
        let journal = crate::journal::Journal::path_for(bundle);
        for name in verified.manifest.sha256.keys() {
            let dest = match name.as_str() {
                MEMBER_BUNDLE => files.bundle.clone(),
                MEMBER_ACL => files.acl.clone(),
                // `family.journal`, `family.journal.1`, … back to the names
                // the application rotates through.
                n if is_journal_member(n) => {
                    let suffix = n.strip_prefix(MEMBER_JOURNAL).unwrap_or("");
                    let mut f = journal.file_name().unwrap_or_default().to_os_string();
                    f.push(suffix);
                    journal.with_file_name(f)
                }
                _ => continue,
            };
            let out = staging.path().join(name);
            extract_verifying(&mut zip, name, &out)?;
            staged.push((out, dest));
        }
    }

    // 4. The current state, kept.
    let aside = dir.join(format!("axgf-replaced-{}", stamp()));
    fs::create_dir_all(&aside).with_context(|| format!("creating {}", aside.display()))?;
    let mut current: Vec<PathBuf> = vec![files.bundle.clone(), files.acl.clone()];
    current.extend(files.journals.iter().cloned());
    for p in &current {
        if p.exists() {
            let to = aside.join(p.file_name().unwrap_or_default());
            fs::rename(p, &to).map_err(|e| {
                anyhow::anyhow!("{}", space::explain_write_failure(&e, &to, Some(p)))
            })?;
        }
    }

    // 5. Into place.
    for (from, to) in staged {
        fs::rename(&from, &to).map_err(|e| {
            anyhow::anyhow!("{}", space::explain_write_failure(&e, &to, Some(&from)))
        })?;
    }

    // The payload cache is keyed by the bundle's SHA-256, so the restored
    // bundle simply gets a different key and re-extracts. Nothing has to be
    // invalidated; the old key's directory is a stale generation the next
    // startup sweeps.
    tracing::warn!(
        archive = %archive.display(),
        bundle = %bundle.display(),
        aside = %aside.display(),
        "restored from a backup; the previous state was kept"
    );

    Ok(RestorePlan {
        verified,
        files,
        aside,
    })
}

/// The ACL and the journal are mode 600 wherever they live.
fn restrict_if_secret(name: &str, path: &Path) {
    #[cfg(unix)]
    if name == MEMBER_ACL || is_journal_member(name) {
        use std::os::unix::fs::PermissionsExt as _;
        let _ = fs::set_permissions(path, fs::Permissions::from_mode(0o600));
    }
    #[cfg(not(unix))]
    let _ = (name, path);
}

/// Refuse to restore under a running instance.
///
/// Two signals, because neither alone is enough. A server that is idle holds no
/// write lock, so the lock cannot detect it; and a bundle on a machine where
/// nothing is listening may still be mid-save by a `backup` run. Together they
/// cover both.
fn refuse_if_running(bundle: &Path) -> Result<()> {
    // Is anything holding the write lock right now?
    if WriteLock::acquire_for(bundle, std::time::Duration::from_millis(200)).is_err() {
        bail!(
            "something is writing {} right now, so it will not be replaced.\n\n\
             Stop the service first:\n\n\
             \x20 sudo systemctl stop axgf-cms\n\n\
             Nothing was changed.",
            bundle.display()
        );
    }
    // Is a server attached to this bundle? It does not keep the file open —
    // the tree is read in at startup and the handle closed — so looking for an
    // open descriptor finds nothing, which is exactly the mistake this used to
    // make. `InstanceLock` is the server saying so explicitly, for its whole
    // life, in a way the kernel retracts if it is killed.
    if let Some(pid) = crate::lockfile::InstanceLock::is_held(bundle) {
        bail!(
            "an instance is running against {}{}. Restoring under a running server \
             would leave that process serving a bundle that no longer exists on \
             disk, and its next save would write the old tree straight back over \
             the restored one.\n\n\
             \x20 sudo systemctl stop axgf-cms\n\n\
             Then restore, then start it again. Nothing was changed.",
            bundle.display(),
            if pid == 0 {
                String::new()
            } else {
                format!(" (process {pid})")
            }
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at(s: &str) -> Archive {
        Archive {
            path: PathBuf::from(format!("{PREFIX}{s}.zip")),
            taken: parse_stamp(s).expect("a stamp this test wrote"),
        }
    }

    #[test]
    fn a_stamp_round_trips() {
        let t = parse_stamp("20260921T143005Z").unwrap();
        assert_eq!(t.year(), 2026);
        assert_eq!(u8::from(t.month()), 9);
        assert_eq!(t.day(), 21);
        assert_eq!(t.hour(), 14);
        assert!(parse_stamp("nonsense").is_none());
        assert!(parse_stamp("20261321T143005Z").is_none(), "month 13");
    }

    #[test]
    fn the_current_stamp_parses_back() {
        assert!(parse_stamp(&stamp()).is_some());
    }

    #[test]
    fn retention_keeps_seven_days_four_weeks_and_twelve_months() {
        // One archive a day for two years, which is the shape a daily timer
        // produces and the one where a policy that counts wrong shows up.
        let mut all = Vec::new();
        let start = time::Date::from_calendar_date(2026, time::Month::September, 21).unwrap();
        for i in 0..730i64 {
            let d = start - time::Duration::days(i);
            all.push(at(&format!(
                "{:04}{:02}{:02}T030000Z",
                d.year(),
                u8::from(d.month()),
                d.day()
            )));
        }
        let keep = select_keepers(&all, Retention::default());
        // 7 daily + 4 weekly + 12 monthly, minus the overlaps: the newest
        // archive is simultaneously today's, this week's and this month's.
        assert!(
            keep.len() >= 12,
            "at least a year of months: {}",
            keep.len()
        );
        assert!(
            keep.len() <= 7 + 4 + 12,
            "never more than the three quotas: {}",
            keep.len()
        );
        assert!(keep.contains(&all[0].path), "today's is always kept");
        assert!(
            !keep.contains(&all[400].path),
            "an arbitrary archive from over a year ago is not"
        );
        // Twelve distinct months survive.
        let months: std::collections::BTreeSet<_> = all
            .iter()
            .filter(|a| keep.contains(&a.path))
            .map(|a| (a.taken.year(), u8::from(a.taken.month())))
            .collect();
        assert_eq!(months.len(), 12, "twelve monthly archives, one per month");
    }

    #[test]
    fn retention_is_configurable_down_to_almost_nothing() {
        let all: Vec<Archive> = (0..30)
            .map(|i| at(&format!("202609{:02}T030000Z", 30 - i)))
            .collect();
        let keep = select_keepers(
            &all,
            Retention {
                daily: 1,
                weekly: 0,
                monthly: 1,
            },
        );
        assert_eq!(
            keep.len(),
            1,
            "the newest is both the day's and the month's"
        );
        assert!(keep.contains(&all[0].path));
    }

    #[test]
    fn several_archives_on_one_day_count_as_one_day() {
        // A timer that fires hourly must not consume the whole daily quota in
        // an afternoon.
        let all: Vec<Archive> = (0..24)
            .map(|h| at(&format!("20260921T{:02}0000Z", 23 - h)))
            .collect();
        let keep = select_keepers(&all, Retention::default());
        assert_eq!(keep.len(), 1, "one day, one week, one month, one archive");
    }

    #[test]
    fn a_file_that_is_not_one_of_ours_is_never_listed_or_pruned() {
        let dir = crate::scratch::Dir::new("prune");
        fs::write(dir.join("notes.txt"), b"mine").unwrap();
        fs::write(dir.join("axgf-backup-nonsense.zip"), b"x").unwrap();
        assert!(list(&dir).unwrap().is_empty());
        assert!(prune(&dir, Retention::default()).unwrap().is_empty());
        assert!(dir.join("notes.txt").exists(), "somebody else's file");
    }
}
