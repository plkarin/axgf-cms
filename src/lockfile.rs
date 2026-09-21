//! One writer at a time, across processes.
//!
//! # Why an in-process lock was not enough
//!
//! [`crate::state::AppState`] holds an `RwLock` and every mutation goes through
//! it, which orders the handlers of one server against each other. It says
//! nothing at all about a *second* process touching the same bundle — and the
//! backup command is exactly that: a separate `axgf-cms backup` run, started by
//! a systemd timer, reading the bundle, the accounts and the journal while the
//! server is up and someone is editing.
//!
//! A backup taken across a save is the failure this prevents. Not a torn
//! `.axgf` — the save is a rename, so the bundle file is whole at every instant
//! — but a *set* that does not agree with itself: the bundle from after the
//! save and the `.acl` from before it, or the other way round. That kind of
//! damage is invisible until the day it is restored.
//!
//! # flock, because the kernel releases it
//!
//! `std::fs::File::lock` is `flock(2)` underneath. The property that matters is
//! that the lock belongs to the open file description, so it goes away when the
//! process does — including when it is killed, which a lock file holding a PID
//! would not manage. There is no stale lock to clean up after a crash, and
//! [`crate::backup`] and the server can therefore both simply ask for it.
//!
//! # Not re-entrant
//!
//! flock is per-open-file-description, so a second [`WriteLock::acquire`] in
//! the same process on a lock this process already holds blocks against itself.
//! Every call site below takes it for one short operation and drops it; none of
//! them nests, and none of them may start to.

use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use anyhow::{Context, Result};

/// How long to wait for another process to finish writing before giving up.
///
/// Long enough to sit out a save of a large bundle — a 435 MB archive is
/// rebuilt in a few seconds — and short enough that a wedged process shows up
/// as an error rather than a backup that never returns.
pub const WAIT: Duration = Duration::from_secs(120);

/// How long a *request* waits.
///
/// Shorter than [`WAIT`], because on the other end of it is a person who has
/// just pressed Save and is looking at a spinner. Long enough to sit out a
/// backup of a large bundle — the archive is copied, not recompressed, so even
/// 435 MB is seconds — and short enough that a wedged backup produces a page
/// saying so rather than a request that never returns.
///
/// A save that waits this long and then fails has changed nothing: the lock is
/// taken before the first byte of the new archive is written.
pub const SERVER_WAIT: Duration = Duration::from_secs(45);

/// An exclusive claim on one bundle, held for as long as the value lives.
#[derive(Debug)]
pub struct WriteLock {
    _file: File,
    path: PathBuf,
}

impl WriteLock {
    /// The lock file beside `bundle`: `family.axgf` → `family.axgf.lock`.
    ///
    /// A sibling rather than a fixed location, because the lock has to be on
    /// the same filesystem as the thing it guards and because two bundles on
    /// one host are two independent things.
    pub fn path_for(bundle: &Path) -> PathBuf {
        let mut name = bundle.file_name().unwrap_or_default().to_os_string();
        name.push(".lock");
        bundle.with_file_name(name)
    }

    /// Take the lock, waiting up to [`WAIT`] for whoever holds it.
    ///
    /// Polls rather than blocking in the kernel so that the wait has a bound
    /// and the error can say who to look for. The poll interval is irrelevant
    /// to throughput: contention here is a backup meeting a save, which happens
    /// once a day at most.
    pub fn acquire(bundle: &Path) -> Result<Self> {
        Self::acquire_for(bundle, WAIT)
    }

    /// [`WriteLock::acquire`] with an explicit deadline, for the tests.
    pub fn acquire_for(bundle: &Path, wait: Duration) -> Result<Self> {
        let path = Self::path_for(bundle);
        let mut opts = OpenOptions::new();
        opts.create(true).read(true).write(true).truncate(false);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt as _;
            // The lock file's existence is not a secret, but it sits beside a
            // mode-600 ACL in a directory only the service user should be in.
            opts.mode(0o600);
        }
        let file = opts
            .open(&path)
            .with_context(|| format!("opening the write lock {}", path.display()))?;

        let deadline = Instant::now() + wait;
        loop {
            match file.try_lock() {
                Ok(()) => return Ok(Self { _file: file, path }),
                // Held by somebody else: wait and ask again.
                Err(std::fs::TryLockError::WouldBlock) => {}
                Err(std::fs::TryLockError::Error(e)) => {
                    return Err(anyhow::Error::new(e))
                        .with_context(|| format!("locking {}", path.display()))
                }
            }
            if Instant::now() >= deadline {
                anyhow::bail!(
                    "another process has been writing {} for more than {} seconds \
                     and still holds the lock {}.\n\n\
                     That is either a save of a very large bundle still running, \
                     or a process that has wedged. `fuser {}` names the process \
                     holding it. Nothing was changed.",
                    bundle.display(),
                    wait.as_secs(),
                    path.display(),
                    path.display(),
                );
            }
            std::thread::sleep(Duration::from_millis(50));
        }
    }

    /// Where the lock file is, for a message.
    pub fn path(&self) -> &Path {
        &self.path
    }
}

/// A running server's claim on one bundle, held for the life of the process.
///
/// # What it is for
///
/// `restore` used to look for a process with the bundle *open* in `/proc`, and
/// found nothing — because the server does not keep it open. It streams the
/// archive in at startup and closes the file; from then on the tree lives in
/// memory and the file is only reopened for the seconds a save takes. So a
/// restore ran happily underneath a live server, moved its three files aside,
/// put older ones in their place, and left a process serving data that no
/// longer existed on disk — whose next save would write the old tree straight
/// back over the restored one.
///
/// This is the missing statement: "an instance is attached to this bundle". It
/// is a second flock, on its own file, taken once at startup and released only
/// when the process ends — by exiting, by `systemctl stop`, or by being
/// killed, because the kernel releases it either way.
///
/// It also refuses to start a *second* server on one bundle, which is worth
/// having on its own: two processes each holding their own copy of the tree in
/// memory would take turns overwriting each other's edits, silently.
#[derive(Debug)]
pub struct InstanceLock {
    _file: File,
    path: PathBuf,
}

impl InstanceLock {
    /// `family.axgf` → `family.axgf.instance`.
    pub fn path_for(bundle: &Path) -> PathBuf {
        let mut name = bundle.file_name().unwrap_or_default().to_os_string();
        name.push(".instance");
        bundle.with_file_name(name)
    }

    /// Claim the bundle, or say who already has it.
    pub fn acquire(bundle: &Path) -> Result<Self> {
        let path = Self::path_for(bundle);
        let mut opts = OpenOptions::new();
        opts.create(true).read(true).write(true).truncate(false);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt as _;
            opts.mode(0o600);
        }
        let mut file = opts
            .open(&path)
            .with_context(|| format!("opening {}", path.display()))?;
        match file.try_lock() {
            Ok(()) => {}
            Err(std::fs::TryLockError::WouldBlock) => {
                let who = std::fs::read_to_string(&path).unwrap_or_default();
                let who = who.trim();
                anyhow::bail!(
                    "another axgf-cms is already serving {}{}.\n\n\
                     Two processes over one bundle each hold their own copy of \
                     the tree in memory and take turns writing it back, so the \
                     second one silently undoes the first one's edits. Stop the \
                     running instance first:\n\n\
                     \x20 sudo systemctl stop axgf-cms\n\n\
                     Nothing was changed.",
                    bundle.display(),
                    if who.is_empty() {
                        String::new()
                    } else {
                        format!(" (process {who})")
                    }
                );
            }
            Err(std::fs::TryLockError::Error(e)) => {
                return Err(anyhow::Error::new(e))
                    .with_context(|| format!("locking {}", path.display()))
            }
        }
        // Recorded so a refusal can name a process rather than saying
        // "something". Advisory only — the lock is what decides.
        use std::io::{Seek, Write};
        let _ = file.set_len(0);
        let _ = file.rewind();
        let _ = write!(file, "{}", std::process::id());
        let _ = file.flush();
        Ok(Self { _file: file, path })
    }

    /// Is an instance attached to this bundle right now?
    ///
    /// Asked by `restore`, which must not run underneath one. A `true` here is
    /// certain; a `false` means nothing holds the lock this instant, which is
    /// as much as any check of this kind can say.
    pub fn is_held(bundle: &Path) -> Option<u32> {
        let path = Self::path_for(bundle);
        let file = OpenOptions::new().read(true).write(true).open(&path).ok()?;
        match file.try_lock() {
            Ok(()) => {
                let _ = file.unlock();
                None
            }
            Err(std::fs::TryLockError::WouldBlock) => Some(
                std::fs::read_to_string(&path)
                    .ok()
                    .and_then(|s| s.trim().parse().ok())
                    .unwrap_or(0),
            ),
            Err(_) => None,
        }
    }

    /// Where the file is, for a message.
    pub fn path(&self) -> &Path {
        &self.path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch(tag: &str) -> crate::scratch::Dir {
        crate::scratch::Dir::new(tag)
    }

    #[test]
    fn the_lock_file_is_a_sibling_of_the_bundle() {
        assert_eq!(
            WriteLock::path_for(Path::new("/srv/family.axgf")),
            PathBuf::from("/srv/family.axgf.lock")
        );
    }

    #[test]
    fn a_second_claim_waits_and_then_gives_up_naming_the_lock() {
        let dir = scratch("contended");
        let bundle = dir.join("family.axgf");
        let held = WriteLock::acquire(&bundle).expect("first claim");
        let err = WriteLock::acquire_for(&bundle, Duration::from_millis(150))
            .expect_err("the second claim cannot have it");
        let msg = format!("{err:#}");
        assert!(msg.contains("family.axgf.lock"), "{msg}");
        assert!(msg.contains("Nothing was changed"), "{msg}");
        drop(held);
    }

    #[test]
    fn dropping_the_lock_hands_it_on() {
        let dir = scratch("handover");
        let bundle = dir.join("family.axgf");
        {
            let _held = WriteLock::acquire(&bundle).unwrap();
        }
        WriteLock::acquire_for(&bundle, Duration::from_millis(50))
            .expect("released by the drop above");
    }

    #[test]
    fn a_second_instance_over_one_bundle_is_refused_by_name() {
        let dir = scratch("instance");
        let bundle = dir.join("family.axgf");
        let held = InstanceLock::acquire(&bundle).expect("the first instance");
        assert_eq!(
            InstanceLock::is_held(&bundle),
            Some(std::process::id()),
            "and it says which process has it"
        );
        let err = InstanceLock::acquire(&bundle).expect_err("the second must be refused");
        let msg = format!("{err:#}");
        assert!(msg.contains("already serving"), "{msg}");
        assert!(msg.contains("systemctl stop"), "{msg}");
        drop(held);
        assert!(InstanceLock::is_held(&bundle).is_none());
        InstanceLock::acquire(&bundle).expect("free again");
    }

    #[test]
    fn an_unclaimed_bundle_reports_no_instance() {
        let dir = scratch("unclaimed");
        assert!(InstanceLock::is_held(&dir.join("never-served.axgf")).is_none());
    }

    #[test]
    fn a_lock_held_by_a_dead_process_is_not_stale() {
        // The whole reason for flock over a PID file: the kernel drops it when
        // the holder dies, so a killed backup does not wedge every later save.
        let dir = scratch("dead");
        let bundle = dir.join("family.axgf");
        let lock = WriteLock::path_for(&bundle);
        let child = std::process::Command::new("sh")
            .arg("-c")
            .arg(format!(
                // `exec` so the process holding fd 9 is the one this test
                // kills. Without it the shell forks `sleep`, which inherits
                // the descriptor and outlives its parent still holding the
                // lock — which is a true statement about flock, and not the
                // one being tested here.
                "exec 9>{} && flock -x 9 && echo ready && exec sleep 30",
                lock.display()
            ))
            .stdout(std::process::Stdio::piped())
            .spawn();
        let Ok(mut child) = child else {
            return; // no shell: nothing to prove here
        };
        {
            use std::io::Read as _;
            let mut out = child.stdout.take().unwrap();
            let mut buf = [0u8; 6];
            let _ = out.read_exact(&mut buf);
        }
        assert!(
            WriteLock::acquire_for(&bundle, Duration::from_millis(100)).is_err(),
            "the child holds it"
        );
        let _ = child.kill();
        let _ = child.wait();
        WriteLock::acquire_for(&bundle, Duration::from_secs(2))
            .expect("the kernel released it when the holder died");
    }
}
