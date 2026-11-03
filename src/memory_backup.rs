//! The AFG's memory, backed up apart from the family's archive (the
//! operator's decision of 3 November 2026, docs/adr/0027).
//!
//! # What it holds
//!
//! The two folders the AFG keeps beside the bundle, and nothing else:
//!
//! * `family.brains/`    — each account's brain (docs/adr/0024)
//! * `family.assistant/` — each account's personal assistant: its projects,
//!   conversations, files and second brains (docs/adr/0026)
//!
//! # Not the bundle's backup
//!
//! The bundle's archive ([`crate::backup`]) keeps the family's state and
//! nothing else: the bundle, the accounts and the journal. The AFG's memory is
//! about the people who talk to it, not the genealogy, so it has an archive of
//! its own, `axgf-memory-<stamp>.zip`, in the same directory. The same nightly
//! run writes it, after the bundle's; the same daily, weekly and monthly
//! selection keeps it, as a set of its own. Neither set ever counts against the
//! other, and the bundle's archive is byte for byte what it was before.
//!
//! # The same discipline
//!
//! Written as a `.part` claimed with `flock`, mode 600 from the moment it
//! exists. Read back, member by member, against the SHA-256 the manifest
//! recorded, before it is given its name. A `.part` left by an interrupted run
//! is reclaimed by the next. A ZIP, so `unzip` restores it without this
//! program; [`restore`] is the convenient path.
//!
//! # Taken while the service runs
//!
//! The service writes these folders while the backup reads them:
//!
//! * a file the service rewrites is replaced whole, by a rename, so the
//!   archive holds the old version or the new one, never half of each;
//! * a conversation's lines are appended, so one being written at that very
//!   second may be cut; the application skips a line it cannot read;
//! * a file still being written (`*.tmp`) is left out.

use std::collections::BTreeMap;
use std::fs;
use std::io::{Read, Write};
use std::path::{Component, Path, PathBuf};

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};

use crate::backup::{self, Retention};
use crate::space::{self, Need};

/// Prefix every archive of the AFG's memory carries.
pub const PREFIX: &str = "axgf-memory-";

/// Name of the manifest inside the archive. Not the bundle archive's
/// `backup.json`: which manifest an archive carries says which kind it is.
pub const MEMBER_MANIFEST: &str = "memory.json";

/// The two kinds of folder an archive may hold, by the ending of their names.
const BRAINS: &str = ".brains";
const ASSISTANT: &str = ".assistant";

/// The manifest written into every archive.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    /// Format of the archive itself, so a future change is detectable.
    pub format: u32,
    /// RFC 3339, UTC.
    pub created_at: String,
    /// Version of the binary that wrote it.
    pub written_by: String,
    /// The bundle the folders were beside.
    pub source_bundle: String,
    /// The folders it holds, by their names beside the bundle
    /// (`family.brains`, `family.assistant`).
    pub folders: Vec<String>,
    /// For each folder, how many accounts have one.
    pub accounts: BTreeMap<String, usize>,
    /// SHA-256 of each file, keyed by its path inside the archive.
    pub sha256: BTreeMap<String, String>,
    /// Bytes of each file, uncompressed.
    pub bytes: BTreeMap<String, u64>,
}

/// What a run did.
#[derive(Debug, Clone)]
pub struct Report {
    pub archive: PathBuf,
    /// Its size on disk.
    pub bytes: u64,
    /// Archives of this set removed by retention.
    pub pruned: Vec<PathBuf>,
    pub manifest: Manifest,
    /// Part-written archives from interrupted runs that this run reclaimed.
    pub swept_parts: usize,
}

/// What an archive holds, once it has been proven readable.
#[derive(Debug, Clone)]
pub struct Verified {
    pub manifest: Manifest,
    pub files: usize,
    /// Bytes of all its files, uncompressed.
    pub bytes: u64,
}

/// The two folders beside `bundle`, whether or not they exist.
pub fn folders(bundle: &Path) -> [PathBuf; 2] {
    [
        crate::brain::Brains::beside(bundle).root().to_path_buf(),
        crate::assistant::Spaces::beside(bundle)
            .root()
            .to_path_buf(),
    ]
}

/// Files to archive, as (name inside the archive, path).
type Files = Vec<(String, PathBuf)>;

/// How many accounts each folder holds, by the folder's name.
type Accounts = BTreeMap<String, usize>;

/// Every file to archive, sorted, and how many accounts each folder holds.
fn members(bundle: &Path) -> Result<(Files, Accounts)> {
    let mut files = Vec::new();
    let mut accounts = BTreeMap::new();
    for root in folders(bundle) {
        if !root.is_dir() {
            continue;
        }
        let top = root
            .file_name()
            .and_then(|n| n.to_str())
            .with_context(|| format!("{} has no name", root.display()))?
            .to_string();
        let n = fs::read_dir(&root)
            .with_context(|| format!("listing {}", root.display()))?
            .flatten()
            .filter(|e| e.file_type().is_ok_and(|t| t.is_dir()))
            .count();
        accounts.insert(top.clone(), n);
        walk(&root, &top, &mut files)?;
    }
    files.sort_by(|a, b| a.0.cmp(&b.0));
    Ok((files, accounts))
}

fn walk(dir: &Path, prefix: &str, out: &mut Vec<(String, PathBuf)>) -> Result<()> {
    let mut entries: Vec<_> = fs::read_dir(dir)
        .with_context(|| format!("listing {}", dir.display()))?
        .flatten()
        .collect();
    entries.sort_by_key(|e| e.file_name());
    for e in entries {
        let Ok(kind) = e.file_type() else { continue };
        // The application names every file in plain ASCII; anything else was
        // not written by it.
        let Some(name) = e.file_name().to_str().map(str::to_string) else {
            continue;
        };
        let member = format!("{prefix}/{name}");
        if kind.is_dir() {
            walk(&e.path(), &member, out)?;
        } else if kind.is_file() && !name.ends_with(".tmp") {
            out.push((member, e.path()));
        }
        // A symbolic link, or a file still being written: the application
        // writes no links, and a `.tmp` becomes a file of its own when done.
    }
    Ok(())
}

/// Write one verified archive of the AFG's memory into `dest`, then prune
/// this set to `retention`. `None` when neither folder exists yet: an empty
/// archive would be a file that says "backup" about nothing.
pub fn run(bundle: &Path, dest: &Path, retention: Retention) -> Result<Option<Report>> {
    let (files, accounts) = members(bundle)?;
    if accounts.is_empty() {
        return Ok(None);
    }
    fs::create_dir_all(dest)
        .with_context(|| format!("creating the backup directory {}", dest.display()))?;
    let swept_parts = backup::sweep_parts(dest, &[PREFIX]).0;

    let need: u64 = files
        .iter()
        .filter_map(|(_, p)| fs::metadata(p).ok())
        .map(|m| m.len())
        .sum();
    space::ensure_room_for(
        dest,
        &Need {
            bytes: need,
            what: "an archive of the AFG's memory",
        },
    )?;

    let stamp = backup::stamp();
    let tmp = dest.join(format!("{PREFIX}{stamp}.zip.part"));
    let final_path = dest.join(format!("{PREFIX}{stamp}.zip"));
    // 0600 from the moment it exists, and claimed for as long as it is being
    // written, exactly as the bundle's archive is: see `backup::run_as`.
    let part = {
        use std::os::unix::fs::OpenOptionsExt;
        fs::OpenOptions::new()
            .write(true)
            .read(true)
            .create_new(true)
            .mode(0o600)
            .open(&tmp)
    }
    .map_err(|e| anyhow::anyhow!("{}", space::explain_write_failure(&e, &tmp, None)))?;
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = part.set_permissions(fs::Permissions::from_mode(0o600));
    }
    if let Err(e) = part.try_lock() {
        let _ = fs::remove_file(&tmp);
        bail!("another backup is already writing {}: {e}", tmp.display());
    }
    let writing = part
        .try_clone()
        .with_context(|| format!("duplicating the handle on {}", tmp.display()))?;

    let manifest = write_archive(bundle, &files, accounts, writing).inspect_err(|_| {
        let _ = fs::remove_file(&tmp);
    })?;

    if let Err(e) = verify(&tmp) {
        let _ = fs::remove_file(&tmp);
        return Err(e).context(
            "the archive of the AFG's memory that was just written could not be read \
             back, so it has been deleted rather than left in place.",
        );
    }
    fs::rename(&tmp, &final_path).map_err(|e| {
        let _ = fs::remove_file(&tmp);
        anyhow::anyhow!(
            "{}",
            space::explain_write_failure(&e, &final_path, Some(&tmp))
        )
    })?;
    drop(part);
    let bytes = fs::metadata(&final_path).map(|m| m.len()).unwrap_or(0);
    let pruned = prune(dest, retention)?;

    tracing::info!(
        archive = %final_path.display(),
        bytes,
        files = manifest.sha256.len(),
        pruned = pruned.len(),
        "archive of the AFG's memory written and verified"
    );
    Ok(Some(Report {
        archive: final_path,
        bytes,
        pruned,
        manifest,
        swept_parts,
    }))
}

fn write_archive(
    bundle: &Path,
    files: &[(String, PathBuf)],
    accounts: Accounts,
    file: fs::File,
) -> Result<Manifest> {
    let mut zip = zip::ZipWriter::new(std::io::BufWriter::new(file));
    let deflated = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .large_file(true);
    let mut sha256 = BTreeMap::new();
    let mut bytes = BTreeMap::new();
    for (name, path) in files {
        // A file the service removed between the listing and now (a
        // conversation deleted, a fact forgotten) is simply not in this
        // archive.
        if !path.exists() {
            continue;
        }
        zip.start_file(name.as_str(), deflated)
            .with_context(|| format!("starting {name} in the archive"))?;
        let (n, digest) = backup::copy_hashing(path, &mut zip)
            .with_context(|| format!("copying {} into the archive", path.display()))?;
        bytes.insert(name.clone(), n);
        sha256.insert(name.clone(), digest);
    }
    let manifest = Manifest {
        format: 1,
        created_at: crate::view::now_iso8601(),
        written_by: format!("axgf-cms {}", env!("CARGO_PKG_VERSION")),
        source_bundle: bundle.display().to_string(),
        folders: accounts.keys().cloned().collect(),
        accounts,
        sha256,
        bytes,
    };
    zip.start_file(
        MEMBER_MANIFEST,
        zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated),
    )
    .context("starting the manifest")?;
    zip.write_all(serde_json::to_string_pretty(&manifest)?.as_bytes())
        .context("writing the manifest")?;
    let mut inner = zip.finish().context("closing the archive")?;
    inner.flush().context("flushing the archive")?;
    inner
        .into_inner()
        .map_err(|e| anyhow::anyhow!("flushing the archive: {e}"))?
        .sync_all()
        .context("fsyncing the archive")?;
    Ok(manifest)
}

/// True when `archive` is an archive of the AFG's memory rather than of the
/// bundle: it carries [`MEMBER_MANIFEST`]. Read from the archive itself, so
/// a renamed file is still told apart.
pub fn is_memory_archive(archive: &Path) -> bool {
    fs::File::open(archive)
        .ok()
        .and_then(|f| zip::ZipArchive::new(std::io::BufReader::new(f)).ok())
        .is_some_and(|mut z| z.by_name(MEMBER_MANIFEST).is_ok())
}

/// A folder name an archive may carry: one plain name, ending in `.brains`
/// or `.assistant`.
fn known_folder(name: &str) -> bool {
    let plain = !name.is_empty() && !name.contains('/') && name != "." && name != "..";
    plain && (name.ends_with(BRAINS) || name.ends_with(ASSISTANT))
}

/// A member's path stays inside one of the archive's folders: no `..`, no
/// root, nothing but plain names.
fn check_member(name: &str, folders: &[String]) -> Result<()> {
    let mut parts = Path::new(name).components();
    let inside = match parts.next() {
        Some(Component::Normal(first)) => folders.iter().any(|f| first == f.as_str()),
        _ => false,
    };
    if !inside || !parts.all(|c| matches!(c, Component::Normal(_))) {
        bail!("{name} is not a path inside the AFG's memory. Do not restore from this archive.");
    }
    Ok(())
}

/// Open an archive of the AFG's memory and prove every file in it can be
/// read back as it was written.
///
/// 1. the ZIP is intact and every member's CRC matches;
/// 2. every file's SHA-256 and size are the ones the manifest recorded;
/// 3. every path stays inside a folder of the AFG's memory;
/// 4. nothing is in it that the manifest does not name.
pub fn verify(archive: &Path) -> Result<Verified> {
    let file = fs::File::open(archive).with_context(|| format!("opening {}", archive.display()))?;
    let mut zip = zip::ZipArchive::new(std::io::BufReader::new(file))
        .with_context(|| format!("{} is not a readable ZIP archive", archive.display()))?;
    let manifest: Manifest = {
        let mut m = zip.by_name(MEMBER_MANIFEST).with_context(|| {
            format!(
                "{} carries no {MEMBER_MANIFEST}, so it is not an archive of the AFG's memory",
                archive.display()
            )
        })?;
        let mut s = String::new();
        m.read_to_string(&mut s).context("reading the manifest")?;
        serde_json::from_str(&s).context("parsing the manifest")?
    };
    if let Some(f) = manifest.folders.iter().find(|f| !known_folder(f)) {
        bail!(
            "{} names a folder, {f}, that is not one of the AFG's memory",
            archive.display()
        );
    }
    let mut total = 0u64;
    for (name, expected) in &manifest.sha256 {
        check_member(name, &manifest.folders)?;
        let mut member = zip.by_name(name).with_context(|| {
            format!(
                "{name} is in the manifest of {} but not in the archive",
                archive.display()
            )
        })?;
        let (n, digest) = backup::copy_hashing_reader(&mut member, &mut std::io::sink())
            .with_context(|| format!("{name} could not be read out of {}", archive.display()))?;
        if &digest != expected || manifest.bytes.get(name) != Some(&n) {
            bail!(
                "{name} in {} is not the file that was backed up. The archive has been \
                 altered or has decayed on disk; use an older one.",
                archive.display()
            );
        }
        total += n;
    }
    let names: Vec<String> = zip
        .file_names()
        .map(|n| n.map(|n| n.into_owned()))
        .collect::<std::result::Result<_, _>>()
        .with_context(|| format!("listing what {} holds", archive.display()))?;
    for name in names {
        if name != MEMBER_MANIFEST && !name.ends_with('/') && !manifest.sha256.contains_key(&name) {
            bail!(
                "{name} is in {} but not in its manifest. Do not restore from it.",
                archive.display()
            );
        }
    }
    Ok(Verified {
        files: manifest.sha256.len(),
        bytes: total,
        manifest,
    })
}

/// Every archive of this set in `dir`, newest first.
pub fn list(dir: &Path) -> Result<Vec<backup::Archive>> {
    backup::list_prefixed(dir, PREFIX)
}

/// The newest archive of this set, or `None`.
pub fn latest(dir: &Path) -> Option<backup::Archive> {
    list(dir).ok().and_then(|v| v.into_iter().next())
}

/// Keep what the daily, weekly and monthly selection keeps of this set, and
/// remove the rest. The bundle's archives are never looked at.
pub fn prune(dir: &Path, retention: Retention) -> Result<Vec<PathBuf>> {
    let all = list(dir)?;
    let keep = backup::select_keepers(&all, retention);
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
                "could not remove an expired archive of the AFG's memory"
            ),
        }
    }
    Ok(removed)
}

/// What a restore did.
#[derive(Debug)]
pub struct RestorePlan {
    pub verified: Verified,
    /// The folders put back.
    pub restored: Vec<PathBuf>,
    /// Where the folders that were there were moved, when there were any.
    pub aside: Option<PathBuf>,
}

/// Put an archive of the AFG's memory back beside `bundle`.
///
/// The order is the bundle's restore's ([`backup::restore`]):
///
/// 1. refuse if the service is running against this bundle;
/// 2. verify the archive completely — a bad archive stops here, having
///    touched nothing;
/// 3. extract its folders to a staging directory beside the bundle;
/// 4. move the folders that are there now aside, into
///    `axgf-memory-replaced-<stamp>/`, which this never deletes;
/// 5. rename the staged folders into place.
///
/// Only the folders the archive holds are replaced: an archive taken before
/// any assistant existed leaves today's assistants as they are.
pub fn restore(archive: &Path, bundle: &Path, force: bool) -> Result<RestorePlan> {
    if !force {
        backup::refuse_if_running(bundle)?;
    }
    let verified = verify(archive)?;
    let [brains, assistant] = folders(bundle);
    let mut targets = Vec::new();
    for f in &verified.manifest.folders {
        let target = if f.ends_with(BRAINS) {
            brains.clone()
        } else {
            assistant.clone()
        };
        targets.push((f.clone(), target));
    }
    let parent = bundle
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));

    let staging = parent.join(format!(
        ".axgf-memory-restore-{}",
        uuid::Uuid::new_v4().simple()
    ));
    crate::brain::make_dir(&staging).with_context(|| format!("creating {}", staging.display()))?;
    let outcome = put_back(archive, &verified, &staging, parent, &targets);
    let _ = fs::remove_dir_all(&staging);
    let aside = outcome?;
    Ok(RestorePlan {
        verified,
        restored: targets.into_iter().map(|(_, t)| t).collect(),
        aside,
    })
}

fn put_back(
    archive: &Path,
    verified: &Verified,
    staging: &Path,
    parent: &Path,
    targets: &[(String, PathBuf)],
) -> Result<Option<PathBuf>> {
    // 3. Out of the archive, every file checked again as it lands.
    let file = fs::File::open(archive)?;
    let mut zip = zip::ZipArchive::new(std::io::BufReader::new(file))?;
    for (f, _) in targets {
        crate::brain::make_dir(&staging.join(f))?;
    }
    for (name, expected) in &verified.manifest.sha256 {
        let out = staging.join(name);
        if let Some(dir) = out.parent() {
            make_dirs(staging, dir)?;
        }
        let mut member = zip.by_name(name)?;
        let mut f = {
            use std::os::unix::fs::OpenOptionsExt;
            fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .mode(0o600)
                .open(&out)
        }
        .with_context(|| format!("creating {}", out.display()))?;
        let (_, digest) = backup::copy_hashing_reader(&mut member, &mut f)?;
        f.sync_all().ok();
        if &digest != expected {
            bail!("{name} changed while it was being restored. Nothing was replaced.");
        }
    }

    // 4. What is there now, aside.
    let mut aside = None;
    for (_, target) in targets {
        if !target.exists() {
            continue;
        }
        let dir = aside.get_or_insert_with(|| {
            parent.join(format!("axgf-memory-replaced-{}", backup::stamp()))
        });
        crate::brain::make_dir(dir)?;
        let name = target.file_name().context("a folder with no name")?;
        fs::rename(target, dir.join(name))
            .with_context(|| format!("moving {} aside", target.display()))?;
    }

    // 5. The staged folders, into place.
    for (f, target) in targets {
        fs::rename(staging.join(f), target)
            .with_context(|| format!("putting {} in place", target.display()))?;
    }
    Ok(aside)
}

/// Every directory from `base` down to `dir`, each readable by its owner
/// only, as the application makes them.
fn make_dirs(base: &Path, dir: &Path) -> Result<()> {
    let rel = dir.strip_prefix(base).unwrap_or(dir);
    let mut at = base.to_path_buf();
    for part in rel.components() {
        at.push(part);
        if !at.is_dir() {
            crate::brain::make_dir(&at).with_context(|| format!("creating {}", at.display()))?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch(tag: &str) -> crate::scratch::Dir {
        crate::scratch::Dir::new(&format!("memory-{tag}"))
    }

    /// A bundle path with a brain for `vera` and an assistant project for her.
    fn memory(dir: &Path) -> PathBuf {
        let bundle = dir.join("family.axgf");
        fs::write(&bundle, b"not read here").unwrap();
        let brain = dir.join("family.brains").join("vera");
        fs::create_dir_all(&brain).unwrap();
        fs::write(brain.join("memory.md"), "[2026-11-03] likes roses\n").unwrap();
        fs::write(brain.join("conversations.jsonl"), "{\"q\":\"hi\"}\n").unwrap();
        // Being written: left out.
        fs::write(brain.join("memory.tmp"), "half").unwrap();
        let project = dir
            .join("family.assistant")
            .join("vera")
            .join("projects")
            .join("0123456789abcdef");
        fs::create_dir_all(project.join("files")).unwrap();
        fs::write(project.join("project.json"), "{\"name\":\"Santé\"}").unwrap();
        fs::write(
            project.join("files").join("fedcba9876543210"),
            [0u8, 159, 146, 150],
        )
        .unwrap();
        bundle
    }

    #[test]
    fn nothing_to_back_up_writes_nothing() {
        let dir = scratch("none");
        let bundle = dir.join("family.axgf");
        fs::write(&bundle, b"x").unwrap();
        let dest = dir.join("backups");
        assert!(run(&bundle, &dest, Retention::default()).unwrap().is_none());
        assert!(!dest.exists() || fs::read_dir(&dest).unwrap().next().is_none());
    }

    #[test]
    fn the_brains_and_the_assistants_go_in_an_archive_of_their_own() {
        use std::os::unix::fs::PermissionsExt;
        let dir = scratch("write");
        let bundle = memory(&dir);
        let dest = dir.join("backups");
        let report = run(&bundle, &dest, Retention::default()).unwrap().unwrap();
        let name = report.archive.file_name().unwrap().to_str().unwrap();
        assert!(name.starts_with(PREFIX) && name.ends_with(".zip"), "{name}");
        let mode = fs::metadata(&report.archive).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, 0o600);
        let m = &report.manifest;
        assert_eq!(m.folders, ["family.assistant", "family.brains"]);
        assert_eq!(m.accounts["family.brains"], 1);
        let names: Vec<&str> = m.sha256.keys().map(String::as_str).collect();
        assert!(names.contains(&"family.brains/vera/memory.md"), "{names:?}");
        assert!(
            names.contains(
                &"family.assistant/vera/projects/0123456789abcdef/files/fedcba9876543210"
            ),
            "{names:?}"
        );
        assert!(!names.iter().any(|n| n.ends_with(".tmp")), "{names:?}");
        assert!(is_memory_archive(&report.archive));
        // No part left, and the bundle's own set untouched.
        let left: Vec<String> = fs::read_dir(&dest)
            .unwrap()
            .flatten()
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .collect();
        assert_eq!(left, [name.to_string()]);
        let v = verify(&report.archive).unwrap();
        assert_eq!(v.files, 4);
    }

    #[test]
    fn a_damaged_or_foreign_archive_is_refused() {
        let dir = scratch("damaged");
        let bundle = memory(&dir);
        let dest = dir.join("backups");
        let report = run(&bundle, &dest, Retention::default()).unwrap().unwrap();
        let good = verify(&report.archive).unwrap().manifest;

        // A file altered after the backup.
        let altered = dir.join("altered.zip");
        let mut zip = zip::ZipWriter::new(fs::File::create(&altered).unwrap());
        let opts = zip::write::SimpleFileOptions::default();
        for name in good.sha256.keys() {
            zip.start_file(name.as_str(), opts).unwrap();
            zip.write_all(b"not what was backed up").unwrap();
        }
        zip.start_file(MEMBER_MANIFEST, opts).unwrap();
        zip.write_all(serde_json::to_string(&good).unwrap().as_bytes())
            .unwrap();
        zip.finish().unwrap();
        assert!(verify(&altered).is_err());

        // A path that climbs out of the memory, with a manifest that agrees.
        let climbing = dir.join("climbing.zip");
        let mut evil = good.clone();
        let body = b"overwrite";
        let digest = {
            let mut sink = std::io::sink();
            backup::copy_hashing_reader(&mut &body[..], &mut sink)
                .unwrap()
                .1
        };
        evil.sha256 = BTreeMap::from([("family.brains/../family.axgf".to_string(), digest)]);
        evil.bytes = BTreeMap::from([("family.brains/../family.axgf".to_string(), 9)]);
        let mut zip = zip::ZipWriter::new(fs::File::create(&climbing).unwrap());
        zip.start_file("family.brains/../family.axgf", opts)
            .unwrap();
        zip.write_all(body).unwrap();
        zip.start_file(MEMBER_MANIFEST, opts).unwrap();
        zip.write_all(serde_json::to_string(&evil).unwrap().as_bytes())
            .unwrap();
        zip.finish().unwrap();
        assert!(verify(&climbing).is_err());

        // A folder that is not the AFG's memory.
        let mut foreign = good.clone();
        foreign.folders = vec!["family.axgf".into()];
        let other = dir.join("foreign.zip");
        let mut zip = zip::ZipWriter::new(fs::File::create(&other).unwrap());
        zip.start_file(MEMBER_MANIFEST, opts).unwrap();
        zip.write_all(serde_json::to_string(&foreign).unwrap().as_bytes())
            .unwrap();
        zip.finish().unwrap();
        assert!(verify(&other).is_err());
    }

    #[test]
    fn a_restore_puts_the_memory_back_and_keeps_what_was_there() {
        let dir = scratch("restore");
        let bundle = memory(&dir);
        let dest = dir.join("backups");
        let report = run(&bundle, &dest, Retention::default()).unwrap().unwrap();

        // Then a fact is forgotten and a new one learned.
        let brain = dir.join("family.brains").join("vera");
        fs::write(brain.join("memory.md"), "[2026-11-04] something else\n").unwrap();

        let plan = restore(&report.archive, &bundle, true).unwrap();
        assert_eq!(
            fs::read_to_string(brain.join("memory.md")).unwrap(),
            "[2026-11-03] likes roses\n"
        );
        assert!(!brain.join("memory.tmp").exists(), "a .tmp is not restored");
        let aside = plan.aside.expect("what was there is kept");
        assert_eq!(
            fs::read_to_string(aside.join("family.brains/vera/memory.md")).unwrap(),
            "[2026-11-04] something else\n"
        );
        assert!(dir
            .join("family.assistant/vera/projects/0123456789abcdef/project.json")
            .exists());
        // No staging directory left behind.
        assert!(!fs::read_dir(&dir).unwrap().flatten().any(|e| e
            .file_name()
            .to_string_lossy()
            .starts_with(".axgf-memory-restore-")));
    }

    #[test]
    fn retention_and_sweeping_touch_this_set_only() {
        let dir = scratch("retention");
        let dest = dir.join("backups");
        fs::create_dir_all(&dest).unwrap();
        for day in 1..=9 {
            fs::write(
                dest.join(format!("{PREFIX}202610{day:02}T030000Z.zip")),
                b"old",
            )
            .unwrap();
            fs::write(
                dest.join(format!("axgf-backup-202610{day:02}T030000Z.zip")),
                b"bundle",
            )
            .unwrap();
        }
        fs::write(
            dest.join(format!("{PREFIX}20261010T030000Z.zip.part")),
            b"dead",
        )
        .unwrap();
        fs::write(dest.join("axgf-backup-20261010T030000Z.zip.part"), b"dead").unwrap();
        let retention = Retention {
            daily: 3,
            weekly: 0,
            monthly: 0,
            pre_upgrade: 3,
        };
        let removed = prune(&dest, retention).unwrap();
        assert_eq!(removed.len(), 6);
        assert_eq!(list(&dest).unwrap().len(), 3);
        assert_eq!(
            backup::list(&dest).unwrap().len(),
            9,
            "the bundle's untouched"
        );
        let (swept, _) = backup::sweep_parts(&dest, &[PREFIX]);
        assert_eq!(swept, 1);
        assert!(dest.join("axgf-backup-20261010T030000Z.zip.part").exists());
    }
}
