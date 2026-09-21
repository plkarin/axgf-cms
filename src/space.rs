//! How much room is left, and what to say when a write runs out of it.
//!
//! # Why this exists
//!
//! Every write that grows the bundle builds the whole archive in a temp file
//! and then renames it over the live one. That is what makes a crash safe, and
//! it is also what makes a full disk expensive: for the length of the write the
//! filesystem holds *two* copies of the bundle, and on a 435 MB archive that is
//! 870 MB of headroom needed to save a corrected birth year.
//!
//! Finding that out halfway through costs the operator a failed save and a
//! stray temp file. Finding it out before the first byte costs them a sentence
//! telling them how much to free. [`ensure_room_for`] is that sentence.
//!
//! # The one `unsafe` in the crate
//!
//! There is no free-space call in `std`, and no pure-Rust way to ask the
//! kernel. The alternatives were shelling out to `df` — which quietly does
//! nothing on a host that has no `df`, so the guard would be absent exactly
//! where it is least expected — or one `statvfs`. The call below is the whole
//! of it: a zeroed struct, one FFI call, a read of two integers, nothing
//! borrowed and nothing kept. The crate is `deny(unsafe_code)` rather than
//! `forbid` for this function alone.

use std::io;
use std::path::{Path, PathBuf};

/// Bytes the unprivileged caller may still write to the filesystem holding
/// `path`, or `None` when the question cannot be answered.
///
/// `f_bavail`, not `f_bfree`: the blocks reserved for root are not room this
/// process has, and counting them would make the guard pass right up to the
/// point where the write fails.
///
/// A path that does not exist yet is normal here — the temp file has not been
/// created — so the nearest existing ancestor is measured instead. That is the
/// same filesystem unless the missing component is itself a mount point, which
/// it cannot be, because a mount point exists.
pub fn available_bytes(path: &Path) -> Option<u64> {
    let probe = nearest_existing(path)?;
    let c = std::ffi::CString::new(probe.as_os_str().as_encoded_bytes()).ok()?;
    #[allow(unsafe_code)]
    let stat = unsafe {
        let mut stat: libc::statvfs = std::mem::zeroed();
        if libc::statvfs(c.as_ptr(), &mut stat) != 0 {
            return None;
        }
        stat
    };
    // f_frsize is the fragment size, which is what f_bavail counts in. On
    // Linux it equals f_bsize; POSIX says to use f_frsize and that is free.
    let unit = if stat.f_frsize > 0 {
        stat.f_frsize as u64
    } else {
        stat.f_bsize as u64
    };
    Some((stat.f_bavail as u64).saturating_mul(unit))
}

/// Total bytes of the filesystem holding `path`, for reporting a percentage.
pub fn total_bytes(path: &Path) -> Option<u64> {
    let probe = nearest_existing(path)?;
    let c = std::ffi::CString::new(probe.as_os_str().as_encoded_bytes()).ok()?;
    #[allow(unsafe_code)]
    let stat = unsafe {
        let mut stat: libc::statvfs = std::mem::zeroed();
        if libc::statvfs(c.as_ptr(), &mut stat) != 0 {
            return None;
        }
        stat
    };
    let unit = if stat.f_frsize > 0 {
        stat.f_frsize as u64
    } else {
        stat.f_bsize as u64
    };
    Some((stat.f_blocks as u64).saturating_mul(unit))
}

/// Free space as a fraction of the whole, 0.0–1.0.
pub fn free_fraction(path: &Path) -> Option<f64> {
    let total = total_bytes(path)?;
    if total == 0 {
        return None;
    }
    Some(available_bytes(path)? as f64 / total as f64)
}

/// Walk up until something exists, so an unborn temp file can still be measured.
fn nearest_existing(path: &Path) -> Option<PathBuf> {
    let mut p: &Path = path;
    loop {
        if p.exists() {
            return Some(p.to_path_buf());
        }
        p = p.parent()?;
        if p.as_os_str().is_empty() {
            return Some(PathBuf::from("."));
        }
    }
}

/// A slack margin on top of the bytes a write is known to need.
///
/// The known part is the bundle that will be rebuilt; the slack covers the ZIP
/// central directory, the journal line, the filesystem's own metadata, and the
/// fact that the new archive is usually slightly larger than the old one
/// because something was added to it. Ten per cent with a 16 MiB floor: a
/// proportion alone is nothing on a small bundle, and a flat figure alone is
/// nothing on a large one.
pub fn margin_for(bytes: u64) -> u64 {
    (bytes / 10).max(16 * 1024 * 1024)
}

/// What a write is about to need, and on which filesystem.
#[derive(Debug, Clone)]
pub struct Need {
    /// Bytes the finished artefact is expected to occupy.
    pub bytes: u64,
    /// Human name for what is being written, for the refusal message.
    pub what: &'static str,
}

/// Refuse up front when `dir` cannot hold `need` plus a margin.
///
/// The count is deliberately the *whole* artefact rather than the difference
/// between old and new: during the write both exist, the temp file beside the
/// live one, and it is the sum that has to fit.
///
/// An unanswerable `statvfs` is not a refusal. A host whose filesystem will
/// not report itself is not a host where every save should stop.
pub fn ensure_room_for(dir: &Path, need: &Need) -> Result<(), NotEnoughRoom> {
    let Some(available) = available_bytes(dir) else {
        tracing::debug!(
            path = %dir.display(),
            "could not read free space; proceeding without the check"
        );
        return Ok(());
    };
    let required = need.bytes.saturating_add(margin_for(need.bytes));
    if available >= required {
        return Ok(());
    }
    Err(NotEnoughRoom {
        what: need.what,
        required,
        available,
        path: dir.to_path_buf(),
    })
}

/// A write refused before it started.
#[derive(Debug, Clone)]
pub struct NotEnoughRoom {
    pub what: &'static str,
    pub required: u64,
    pub available: u64,
    pub path: PathBuf,
}

impl std::fmt::Display for NotEnoughRoom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let short = self.required.saturating_sub(self.available);
        write!(
            f,
            "not enough free space to write {what}, so nothing was written \
             and the existing data is untouched.\n\
             \n\
             \x20 needs      {required}\n\
             \x20 free       {available} on the filesystem holding {path}\n\
             \x20 short by   {short}\n\
             \n\
             The figure is larger than the file itself because the new copy is \
             built beside the old one and only replaces it once it is complete \
             — both exist at the same moment. Free at least {short} and try \
             again.",
            what = self.what,
            required = crate::documents::human_size(self.required),
            available = crate::documents::human_size(self.available),
            path = self.path.display(),
            short = crate::documents::human_size(short),
        )
    }
}

impl std::error::Error for NotEnoughRoom {}

/// Rewrite an `io::Error` from a write, a create or a rename into something an
/// operator can act on.
///
/// The message this replaces was `renaming /var/lib/axgf-cms/family.axgf.tmp
/// over /var/lib/axgf-cms/family.axgf`, which says what the program was doing
/// and nothing about what went wrong or what to do next. Each case below has a
/// different remedy, and telling them apart is the whole value.
///
/// `dest` is the file being written or replaced; `tmp` the temp file, when
/// there was one.
pub fn explain_write_failure(e: &io::Error, dest: &Path, tmp: Option<&Path>) -> String {
    let dir = dest.parent().unwrap_or(Path::new("."));
    let code = e.raw_os_error();
    let free = available_bytes(dir)
        .map(crate::documents::human_size)
        .unwrap_or_else(|| "unknown".into());

    match code {
        Some(libc::ENOSPC) => format!(
            "the disk holding {dir} is full, so {dest} was left exactly as it \
             was — nothing has been lost.\n\
             \n\
             Free space now: {free}.\n\
             \n\
             Free some room and repeat the change. `axgf-cms backup --dest` \
             archives elsewhere are the usual thing to move off first; the \
             payload cache under the bundle directory rebuilds itself and can \
             be deleted whole.",
            dir = dir.display(),
            dest = dest.display(),
        ),
        Some(libc::EDQUOT) => format!(
            "the disk quota for this account is exhausted, so {dest} was left \
             exactly as it was.\n\
             \n\
             The filesystem itself reports {free} free — the limit is on the \
             user the service runs as, not on the disk. Raise the quota, or \
             move the bundle to a filesystem without one.",
            dest = dest.display(),
        ),
        Some(libc::EROFS) => format!(
            "{dir} is mounted read-only, so {dest} could not be written and \
             was left exactly as it was.\n\
             \n\
             Check `mount | grep {dir}`. A filesystem remounts itself \
             read-only after an I/O error, so look in `dmesg` for the disk \
             before remounting it read-write.",
            dir = dir.display(),
            dest = dest.display(),
        ),
        Some(libc::EXDEV) => format!(
            "{tmpdesc} and {dest} are on two different filesystems, so the \
             final rename — the step that makes the new file replace the old \
             one in a single instant — is not possible.\n\
             \n\
             This is a configuration problem rather than a fault: the temp \
             file has to be a sibling of the bundle. If the bundle directory \
             is a mount point with a symlink in it, or the temp file has been \
             redirected elsewhere, put them back on one filesystem. Nothing \
             was changed.",
            tmpdesc = tmp
                .map(|t| t.display().to_string())
                .unwrap_or_else(|| "the temporary file".into()),
            dest = dest.display(),
        ),
        Some(libc::EACCES) | Some(libc::EPERM) => {
            let sticky = sticky_bit_set(dir);
            let mut msg = format!(
                "permission denied writing {dest}, so it was left exactly as \
                 it was.\n\
                 \n\
                 The service runs as its own user and needs to own both \
                 {dir} and the files in it. Check with `ls -ld {dir}` and \
                 `ls -l {dest}`.",
                dest = dest.display(),
                dir = dir.display(),
            );
            if sticky {
                msg.push_str(&format!(
                    "\n\n{dir} has the sticky bit set (`drwxrwxrwt`), which is \
                     how a shared directory such as /tmp is marked: in one, \
                     only the owner of a file may replace or remove it, so the \
                     rename fails even though the directory is writable. A \
                     bundle does not belong in a shared directory — move it to \
                     a directory the service user owns, such as \
                     /var/lib/axgf-cms.",
                    dir = dir.display()
                ));
            }
            msg
        }
        Some(libc::EISDIR) => format!(
            "{dest} is a directory, not a file. Point --bundle at the .axgf \
             file itself. Nothing was changed.",
            dest = dest.display()
        ),
        Some(libc::ENOENT) => format!(
            "{dir} does not exist, so {dest} could not be written. Create the \
             directory — the service user must own it — and try again. \
             Nothing was changed.",
            dir = dir.display(),
            dest = dest.display(),
        ),
        _ => format!(
            "writing {dest} failed: {e}. Nothing was changed; the previous \
             file is intact. Free space on {dir} is {free}.",
            dest = dest.display(),
            dir = dir.display(),
        ),
    }
}

/// True when `dir` is a directory with the sticky bit set.
#[cfg(unix)]
fn sticky_bit_set(dir: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt as _;
    std::fs::metadata(dir)
        .map(|m| m.permissions().mode() & 0o1000 != 0)
        .unwrap_or(false)
}

#[cfg(not(unix))]
fn sticky_bit_set(_dir: &Path) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn free_space_on_a_real_directory_is_reported() {
        let here = std::env::current_dir().unwrap();
        let free = available_bytes(&here).expect("a real filesystem answers");
        assert!(free > 0, "a filesystem with a checkout on it is not full");
        assert!(total_bytes(&here).unwrap() >= free);
        let f = free_fraction(&here).unwrap();
        assert!((0.0..=1.0).contains(&f));
    }

    #[test]
    fn a_file_that_does_not_exist_yet_measures_its_directory() {
        let here = std::env::current_dir().unwrap();
        let unborn = here.join("no-such-dir").join("no-such-file.tmp");
        assert!(
            available_bytes(&unborn).is_some(),
            "the temp file has not been created when the check runs"
        );
    }

    #[test]
    fn the_margin_has_both_a_floor_and_a_proportion() {
        assert_eq!(margin_for(0), 16 * 1024 * 1024, "a floor, on a tiny bundle");
        assert_eq!(
            margin_for(1024 * 1024 * 1024),
            1024 * 1024 * 1024 / 10,
            "a proportion, on a large one"
        );
    }

    #[test]
    fn an_impossible_requirement_is_refused_with_the_shortfall_named() {
        let here = std::env::current_dir().unwrap();
        let err = ensure_room_for(
            &here,
            &Need {
                bytes: u64::MAX / 4,
                what: "the bundle",
            },
        )
        .expect_err("nothing has four exabytes free");
        let msg = err.to_string();
        assert!(msg.contains("short by"), "{msg}");
        assert!(msg.contains("the bundle"), "{msg}");
        assert!(
            msg.contains("existing data is untouched"),
            "the operator's first question is whether they lost anything: {msg}"
        );
    }

    #[test]
    fn an_ordinary_write_is_not_refused() {
        let here = std::env::current_dir().unwrap();
        assert!(ensure_room_for(
            &here,
            &Need {
                bytes: 1024,
                what: "the bundle"
            }
        )
        .is_ok());
    }

    #[test]
    fn each_failure_is_named_in_the_operators_language() {
        let dest = Path::new("/var/lib/axgf-cms/family.axgf");
        let tmp = Path::new("/var/lib/axgf-cms/family.axgf.tmp");

        let full =
            explain_write_failure(&io::Error::from_raw_os_error(libc::ENOSPC), dest, Some(tmp));
        assert!(full.contains("disk holding"), "{full}");
        assert!(full.contains("nothing has been lost"), "{full}");

        let xdev =
            explain_write_failure(&io::Error::from_raw_os_error(libc::EXDEV), dest, Some(tmp));
        assert!(xdev.contains("two different filesystems"), "{xdev}");

        let perm =
            explain_write_failure(&io::Error::from_raw_os_error(libc::EACCES), dest, Some(tmp));
        assert!(perm.contains("permission denied"), "{perm}");

        let ro = explain_write_failure(&io::Error::from_raw_os_error(libc::EROFS), dest, Some(tmp));
        assert!(ro.contains("read-only"), "{ro}");

        // And they are genuinely different messages, not one message with a
        // code substituted into it.
        assert_ne!(full, xdev);
        assert_ne!(xdev, perm);
    }

    #[cfg(unix)]
    #[test]
    fn a_sticky_directory_is_called_out_by_name() {
        // /tmp is the case this exists for: mode 1777, so a rename over
        // somebody else's file fails with EPERM and the ordinary permission
        // advice — "check the owner of the directory" — is a dead end.
        assert!(sticky_bit_set(Path::new("/tmp")), "/tmp is 1777");
        let msg = explain_write_failure(
            &io::Error::from_raw_os_error(libc::EPERM),
            Path::new("/tmp/family.axgf"),
            None,
        );
        assert!(msg.contains("sticky bit"), "{msg}");
        assert!(
            msg.contains("/var/lib/axgf-cms"),
            "it says where to go: {msg}"
        );
    }
}
