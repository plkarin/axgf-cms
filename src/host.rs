//! The machine the site runs on: what is stored, how much room is left, and
//! what the process and the system are using (v0.5.0, docs/adr/0011 §6).
//!
//! # Volumes
//!
//! Each piece of the installation is measured on its own — the binary, the
//! bundle, the payload cache, the backups, the journal, the accounts — beside
//! the filesystem that holds the bundle: what is free, what is used, and how
//! long the free part lasts at the rate the bundle has been growing, read from
//! the backups' own sizes over time.
//!
//! # Computing
//!
//! Read from `/proc` on Linux: the CPU, the load, the memory, the process's
//! own footprint and the cgroup's limit, and whether the process runs under
//! the systemd sandbox bootstrap.sh writes. The unit sets `ProcSubset=pid`,
//! which hides the system-wide files (`/proc/meminfo`, `/proc/loadavg`) from
//! the service on purpose; the page says so rather than showing zeros, and
//! the process's own figures, which stay visible, are still shown. Every
//! other platform reports what the standard library knows and says the rest
//! is not measured there.

use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

use serde::Serialize;

/// A filesystem's room.
#[derive(Debug, Clone, Copy, Serialize)]
pub struct Room {
    pub free: u64,
    pub total: u64,
}

impl Room {
    pub fn used(&self) -> u64 {
        self.total.saturating_sub(self.free)
    }
    pub fn used_fraction(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            self.used() as f64 / self.total as f64
        }
    }
}

/// The backups, as files on disk.
#[derive(Debug, Clone, Default, Serialize)]
pub struct Backups {
    pub dir: String,
    pub count: usize,
    pub total: u64,
    pub newest: Option<String>,
    pub newest_size: u64,
    pub newest_at: Option<String>,
    pub oldest_at: Option<String>,
}

/// Everything the installation stores.
#[derive(Debug, Clone, Default, Serialize)]
pub struct Volumes {
    pub binary_path: Option<String>,
    pub binary: u64,
    pub bundle: u64,
    pub cache: u64,
    pub cache_files: usize,
    pub journal: u64,
    pub accounts: u64,
    pub backups: Option<Backups>,
    /// The filesystem holding the bundle.
    pub room: Option<Room>,
    /// The filesystem holding the backups, when it is another one.
    pub backup_room: Option<Room>,
    /// How fast the bundle has been growing, from the backups' sizes.
    pub growth_per_day: Option<f64>,
    /// How many days the free room lasts at that rate, the bundle and every
    /// kept backup growing together.
    pub days_left: Option<f64>,
}

impl Volumes {
    /// Everything this installation itself occupies.
    pub fn ours(&self) -> u64 {
        self.binary
            + self.bundle
            + self.cache
            + self.journal
            + self.accounts
            + self.backups.as_ref().map(|b| b.total).unwrap_or(0)
    }
}

/// Measure the installation around `bundle`.
pub fn volumes(
    bundle: &Path,
    cache_dir: &Path,
    backup_dir: Option<&Path>,
    space: &dyn crate::space::Probe,
) -> Volumes {
    let size = |p: &Path| std::fs::metadata(p).map(|m| m.len()).unwrap_or(0);
    let (binary_path, binary) = std::env::current_exe()
        .ok()
        .map(|p| {
            let n = size(&p);
            (Some(p.display().to_string()), n)
        })
        .unwrap_or((None, 0));
    let (cache, cache_files) = dir_size(cache_dir);
    let journal = crate::journal::Journal::new(crate::journal::Journal::path_for(bundle))
        .segments()
        .iter()
        .map(|p| size(p))
        .sum::<u64>();
    let accounts = size(&crate::acl::Acl::path_for(bundle));
    let dir = bundle.parent().unwrap_or(Path::new("."));
    let room = match (space.available(dir), space.total(dir)) {
        (Some(free), Some(total)) => Some(Room { free, total }),
        _ => None,
    };
    let (backups, history) = match backup_dir {
        Some(d) => {
            let (b, h) = backups(d);
            (Some(b), h)
        }
        None => (None, Vec::new()),
    };
    let backup_room = backup_dir.and_then(|d| {
        if same_filesystem(d, dir) {
            return None;
        }
        match (space.available(d), space.total(d)) {
            (Some(free), Some(total)) => Some(Room { free, total }),
            _ => None,
        }
    });
    let growth_per_day = growth(&history);
    let kept = backups.as_ref().map(|b| b.count).unwrap_or(0) as f64;
    let days_left = match (growth_per_day, room) {
        (Some(g), Some(r)) if g > 0.0 => Some(r.free as f64 / (g * (1.0 + kept))),
        _ => None,
    };
    Volumes {
        binary_path,
        binary,
        bundle: size(bundle),
        cache,
        cache_files,
        journal,
        accounts,
        backups,
        room,
        backup_room,
        growth_per_day,
        days_left,
    }
}

/// The archives in a backup directory, and their (time, size) history.
fn backups(dir: &Path) -> (Backups, Vec<(SystemTime, u64)>) {
    let mut out = Backups {
        dir: dir.display().to_string(),
        ..Default::default()
    };
    let mut history = Vec::new();
    let Ok(entries) = std::fs::read_dir(dir) else {
        return (out, history);
    };
    let mut newest: Option<(SystemTime, String, u64)> = None;
    let mut oldest: Option<SystemTime> = None;
    for e in entries.flatten() {
        let name = e.file_name().to_string_lossy().into_owned();
        if !name.starts_with("axgf-") || !name.ends_with(".zip") {
            continue;
        }
        let Ok(meta) = e.metadata() else { continue };
        let at = meta.modified().unwrap_or(SystemTime::UNIX_EPOCH);
        out.count += 1;
        out.total += meta.len();
        // Only the regular archives say how the bundle grows: a pre-upgrade
        // archive is a copy taken at a moment somebody chose.
        if name.starts_with("axgf-backup-") {
            history.push((at, meta.len()));
        }
        if newest.as_ref().is_none_or(|(t, _, _)| at > *t) {
            newest = Some((at, name.clone(), meta.len()));
        }
        if oldest.is_none_or(|t| at < t) {
            oldest = Some(at);
        }
    }
    if let Some((at, name, size)) = newest {
        out.newest = Some(name);
        out.newest_size = size;
        out.newest_at = Some(iso(at));
    }
    out.oldest_at = oldest.map(iso);
    (out, history)
}

/// Bytes a day, the least-squares slope through (time, size). Needs two
/// points at least a day apart; anything less is not a rate.
fn growth(points: &[(SystemTime, u64)]) -> Option<f64> {
    if points.len() < 2 {
        return None;
    }
    let base = points.iter().map(|(t, _)| *t).min()?;
    let xs: Vec<f64> = points
        .iter()
        .map(|(t, _)| t.duration_since(base).unwrap_or_default().as_secs_f64() / 86_400.0)
        .collect();
    let span =
        xs.iter().cloned().fold(f64::MIN, f64::max) - xs.iter().cloned().fold(f64::MAX, f64::min);
    if span < 1.0 {
        return None;
    }
    let ys: Vec<f64> = points.iter().map(|(_, s)| *s as f64).collect();
    let n = xs.len() as f64;
    let (mx, my) = (xs.iter().sum::<f64>() / n, ys.iter().sum::<f64>() / n);
    let num: f64 = xs.iter().zip(&ys).map(|(x, y)| (x - mx) * (y - my)).sum();
    let den: f64 = xs.iter().map(|x| (x - mx).powi(2)).sum();
    (den > 0.0).then(|| num / den)
}

/// The total size of the files under `dir`, and how many there are.
pub fn dir_size(dir: &Path) -> (u64, usize) {
    let mut total = 0;
    let mut files = 0;
    let mut stack: Vec<PathBuf> = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&d) else {
            continue;
        };
        for e in entries.flatten() {
            let Ok(ft) = e.file_type() else { continue };
            if ft.is_dir() {
                stack.push(e.path());
            } else if ft.is_file() {
                total += e.metadata().map(|m| m.len()).unwrap_or(0);
                files += 1;
            }
        }
    }
    (total, files)
}

#[cfg(unix)]
fn same_filesystem(a: &Path, b: &Path) -> bool {
    use std::os::unix::fs::MetadataExt;
    match (std::fs::metadata(a), std::fs::metadata(b)) {
        (Ok(x), Ok(y)) => x.dev() == y.dev(),
        _ => true,
    }
}

#[cfg(not(unix))]
fn same_filesystem(_a: &Path, _b: &Path) -> bool {
    true
}

fn iso(t: SystemTime) -> String {
    let secs = t
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0) as i64;
    time::OffsetDateTime::from_unix_timestamp(secs)
        .ok()
        .and_then(|d| {
            d.format(&time::format_description::well_known::Rfc3339)
                .ok()
        })
        .unwrap_or_default()
}

// ---------------------------------------------------------------------------
// Computing
// ---------------------------------------------------------------------------

/// What the process and the machine are using.
#[derive(Debug, Clone, Default, Serialize)]
pub struct Compute {
    pub os: Option<String>,
    pub kernel: Option<String>,
    pub arch: &'static str,
    pub cpus: usize,
    pub cpu_model: Option<String>,
    pub load: Option<[f64; 3]>,
    pub mem_total: Option<u64>,
    pub mem_available: Option<u64>,
    pub rss: Option<u64>,
    pub threads: Option<u64>,
    pub open_files: Option<usize>,
    /// The service's cgroup: what it uses, and its ceiling if it has one.
    pub cgroup_current: Option<u64>,
    pub cgroup_max: Option<u64>,
    pub process_uptime_secs: u64,
    pub system_uptime_secs: Option<u64>,
    /// `NoNewPrivs` and the seccomp mode from `/proc/self/status`.
    pub no_new_privs: Option<bool>,
    pub seccomp: Option<u8>,
    /// The system-wide figures exist but this process may not read them —
    /// `ProcSubset=pid` — which is the sandbox working, not a fault.
    pub system_hidden: bool,
}

/// Measure, now.
pub fn compute(process_uptime: Duration) -> Compute {
    let mut c = Compute {
        arch: std::env::consts::ARCH,
        cpus: std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1),
        process_uptime_secs: process_uptime.as_secs(),
        os: os_release(),
        ..Default::default()
    };
    linux(&mut c);
    c
}

/// `PRETTY_NAME` from os-release, which the sandbox leaves readable.
fn os_release() -> Option<String> {
    let text = std::fs::read_to_string("/etc/os-release")
        .or_else(|_| std::fs::read_to_string("/usr/lib/os-release"))
        .ok()?;
    text.lines()
        .find_map(|l| l.strip_prefix("PRETTY_NAME="))
        .map(|v| v.trim_matches('"').to_string())
}

#[cfg(target_os = "linux")]
fn linux(c: &mut Compute) {
    let read = |p: &str| std::fs::read_to_string(p).ok();
    let field = |text: &str, name: &str| -> Option<u64> {
        text.lines()
            .find_map(|l| l.strip_prefix(name))
            .and_then(|v| v.split_whitespace().next())
            .and_then(|v| v.parse().ok())
    };
    if let Some(status) = read("/proc/self/status") {
        c.rss = field(&status, "VmRSS:").map(|kb| kb * 1024);
        c.threads = field(&status, "Threads:");
        c.no_new_privs = field(&status, "NoNewPrivs:").map(|v| v == 1);
        c.seccomp = field(&status, "Seccomp:").map(|v| v as u8);
    }
    c.open_files = std::fs::read_dir("/proc/self/fd").ok().map(|d| d.count());
    match read("/proc/meminfo") {
        Some(mem) => {
            c.mem_total = field(&mem, "MemTotal:").map(|kb| kb * 1024);
            c.mem_available = field(&mem, "MemAvailable:").map(|kb| kb * 1024);
        }
        None => c.system_hidden = c.rss.is_some(),
    }
    c.load = read("/proc/loadavg").and_then(|l| {
        let v: Vec<f64> = l
            .split_whitespace()
            .take(3)
            .filter_map(|x| x.parse().ok())
            .collect();
        (v.len() == 3).then(|| [v[0], v[1], v[2]])
    });
    c.system_uptime_secs = read("/proc/uptime")
        .and_then(|u| u.split_whitespace().next()?.parse::<f64>().ok())
        .map(|s| s as u64);
    c.cpu_model = read("/proc/cpuinfo").and_then(|i| {
        i.lines()
            .find_map(|l| l.strip_prefix("model name"))
            .map(|v| v.trim_start_matches([' ', '\t', ':']).trim().to_string())
    });
    c.kernel = read("/proc/sys/kernel/osrelease").map(|s| s.trim().to_string());
    // The cgroup v2 this process is in: `0::/system.slice/axgf-cms.service`.
    if let Some(cg) = read("/proc/self/cgroup") {
        if let Some(path) = cg.lines().find_map(|l| l.strip_prefix("0::")) {
            let base = Path::new("/sys/fs/cgroup").join(path.trim_start_matches('/'));
            c.cgroup_current = read(&base.join("memory.current").to_string_lossy())
                .and_then(|v| v.trim().parse().ok());
            c.cgroup_max = read(&base.join("memory.max").to_string_lossy())
                .and_then(|v| v.trim().parse().ok());
        }
    }
}

#[cfg(not(target_os = "linux"))]
fn linux(_c: &mut Compute) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn growth_is_a_slope_over_at_least_a_day() {
        let t0 = SystemTime::UNIX_EPOCH + Duration::from_secs(1_700_000_000);
        let day = Duration::from_secs(86_400);
        assert_eq!(growth(&[(t0, 100)]), None);
        assert_eq!(
            growth(&[(t0, 100), (t0 + Duration::from_secs(60), 200)]),
            None
        );
        let g = growth(&[(t0, 1000), (t0 + day, 1100), (t0 + day * 2, 1200)]).unwrap();
        assert!((g - 100.0).abs() < 1e-6, "{g}");
    }

    #[test]
    fn a_directory_is_measured_file_by_file() {
        let dir = crate::scratch::Dir::new("host");
        std::fs::create_dir_all(dir.join("a/b")).unwrap();
        std::fs::write(dir.join("one"), [0u8; 10]).unwrap();
        std::fs::write(dir.join("a/b/two"), [0u8; 32]).unwrap();
        assert_eq!(dir_size(&dir), (42, 2));
    }

    #[test]
    fn the_process_can_always_measure_itself() {
        let c = compute(Duration::from_secs(5));
        assert!(c.cpus >= 1);
        assert_eq!(c.process_uptime_secs, 5);
        if cfg!(target_os = "linux") {
            assert!(c.rss.unwrap_or(0) > 0);
        }
    }
}
