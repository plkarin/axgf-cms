//! What an operator needs to know without being asked.
//!
//! # One status, four questions
//!
//! `/health` used to answer "the process is up and here are some counts", which
//! a monitor can only turn into "up" or "not answering". Neither of the two
//! ways this application actually fails in a family's house — the disk filled
//! last Tuesday, the backup timer has not fired since the upgrade — moves that
//! answer at all.
//!
//! So the endpoint reports four checks, and the HTTP status is the worst of
//! them. A monitor that knows nothing about AXGF and watches only for a
//! non-200 catches every one. That is the whole design: the alerting lives in
//! whatever the household already uses, and this end of it is a status code.
//!
//! # Warn, and fail
//!
//! Two levels below `Ok`. `Warn` is "somebody should look at this in the next
//! few days" — a backup that is three days old, a disk at 12 % free. `Fail` is
//! "it is broken or about to be" — the bundle does not validate, the disk is
//! nearly full. A `Warn` answers 200 so that a monitor does not page a family
//! at midnight over a stale backup, and it is shown loudly in the dashboard
//! where somebody who can act on it will see it. A `Fail` answers 503.

use std::path::Path;

use serde::Serialize;
use serde_json::{json, Value};

/// Free-space fraction below which the disk is a warning.
pub const DISK_WARN_FRACTION: f64 = 0.10;
/// Free-space fraction below which the disk is a failure. Below this there is
/// not enough room to rebuild a bundle beside itself, so the next save fails.
pub const DISK_FAIL_FRACTION: f64 = 0.03;
/// Age past which the last backup is a warning, in hours.
pub const BACKUP_WARN_HOURS: i64 = 48;
/// Age past which it is a failure.
pub const BACKUP_FAIL_HOURS: i64 = 24 * 14;

/// How bad a check is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Level {
    Ok,
    Warn,
    Fail,
}

impl Level {
    pub fn as_str(self) -> &'static str {
        match self {
            Level::Ok => "ok",
            Level::Warn => "warn",
            Level::Fail => "fail",
        }
    }
}

/// One thing that was checked.
#[derive(Debug, Clone, Serialize)]
pub struct Check {
    /// Stable machine name: `bundle`, `disk`, `backup`, `cache`.
    pub name: &'static str,
    pub level: Level,
    /// One sentence, in English, for a human reading `curl /health`.
    pub detail: String,
}

/// Every check, and the worst level among them.
#[derive(Debug, Clone, Serialize)]
pub struct Report {
    pub checks: Vec<Check>,
}

impl Report {
    /// The worst level reported.
    pub fn level(&self) -> Level {
        self.checks
            .iter()
            .map(|c| c.level)
            .max()
            .unwrap_or(Level::Ok)
    }

    /// One check by name.
    pub fn get(&self, name: &str) -> Option<&Check> {
        self.checks.iter().find(|c| c.name == name)
    }

    /// The checks that are not `Ok`, for a banner.
    pub fn problems(&self) -> Vec<&Check> {
        self.checks
            .iter()
            .filter(|c| c.level != Level::Ok)
            .collect()
    }

    /// The HTTP status a monitor should see.
    ///
    /// 503 for a failure, because that is what every uptime monitor already
    /// treats as down. 200 for a warning: a stale backup is not a reason to
    /// wake anyone, and a monitor that pages on warnings gets muted.
    pub fn http_status(&self) -> axum::http::StatusCode {
        match self.level() {
            Level::Fail => axum::http::StatusCode::SERVICE_UNAVAILABLE,
            _ => axum::http::StatusCode::OK,
        }
    }

    /// The JSON body, checks and all.
    pub fn to_json(&self) -> Value {
        let mut checks = serde_json::Map::new();
        for c in &self.checks {
            checks.insert(
                c.name.to_string(),
                json!({ "status": c.level.as_str(), "detail": c.detail }),
            );
        }
        json!({
            "status": self.level().as_str(),
            "checks": Value::Object(checks),
        })
    }
}

/// Run every check against a live application state.
///
/// `backup_dir` is where archives are expected; `None` means the operator has
/// not configured one and the backup check reports that rather than claiming a
/// failure. An installation with no backups configured is a real risk and says
/// so — as a warning, because it is a decision somebody may have made.
pub fn report(state: &crate::state::AppState, backup_dir: Option<&Path>) -> Report {
    Report {
        checks: vec![
            bundle_check(state),
            disk_check(state.bundle_path()),
            backup_check(backup_dir),
            cache_check(state),
        ],
    }
}

/// Does the bundle in memory still validate?
///
/// Not "did it load" — it loaded, or there would be no process to ask. This is
/// the library's own validation over the current tree, which is what catches an
/// edit that produced something structurally wrong.
fn bundle_check(state: &crate::state::AppState) -> Check {
    let env = state.inspect_with(axgf_rs::validate);
    let errors = env
        .diagnostics
        .iter()
        .filter(|d| d.severity == axgf_rs::boundary::envelope::Severity::Error)
        .count();
    let total: usize = state.counts().iter().map(|(_, n)| n).sum();
    if env.status == axgf_rs::boundary::envelope::Status::Error || errors > 0 {
        return Check {
            name: "bundle",
            level: Level::Fail,
            detail: format!(
                "the bundle is loaded but does not validate: {errors} error(s). \
                 See the admin dashboard for the diagnostics."
            ),
        };
    }
    Check {
        name: "bundle",
        level: Level::Ok,
        detail: format!("loaded and valid, {total} entities"),
    }
}

/// Is there room to save?
fn disk_check(bundle: &Path) -> Check {
    let dir = bundle.parent().unwrap_or(Path::new("."));
    let (Some(free), Some(total)) = (
        crate::space::available_bytes(dir),
        crate::space::total_bytes(dir),
    ) else {
        return Check {
            name: "disk",
            level: Level::Warn,
            detail: format!(
                "could not read free space on the filesystem holding {}",
                dir.display()
            ),
        };
    };
    let fraction = if total == 0 {
        0.0
    } else {
        free as f64 / total as f64
    };
    let human = format!(
        "{} free of {} ({:.0}%) on {}",
        crate::documents::human_size(free),
        crate::documents::human_size(total),
        fraction * 100.0,
        dir.display()
    );
    // The bundle has to be rebuilt beside itself on every save, so "enough
    // room" is measured against the bundle's own size as well as against a
    // percentage: a 90 %-full 2 TB disk has plenty, a 20 %-full 1 GB disk
    // holding a 400 MB bundle does not.
    let bundle_bytes = std::fs::metadata(bundle).map(|m| m.len()).unwrap_or(0);
    let need = bundle_bytes.saturating_add(crate::space::margin_for(bundle_bytes));
    if free < need {
        return Check {
            name: "disk",
            level: Level::Fail,
            detail: format!(
                "{human} — less than the {} the next save needs to rebuild the \
                 bundle beside itself. Saves will be refused.",
                crate::documents::human_size(need)
            ),
        };
    }
    if fraction < DISK_FAIL_FRACTION {
        return Check {
            name: "disk",
            level: Level::Fail,
            detail: format!("{human} — below {:.0}%", DISK_FAIL_FRACTION * 100.0),
        };
    }
    if fraction < DISK_WARN_FRACTION {
        return Check {
            name: "disk",
            level: Level::Warn,
            detail: format!("{human} — below {:.0}%", DISK_WARN_FRACTION * 100.0),
        };
    }
    Check {
        name: "disk",
        level: Level::Ok,
        detail: human,
    }
}

/// How old is the newest verified archive?
fn backup_check(dir: Option<&Path>) -> Check {
    let Some(dir) = dir else {
        return Check {
            name: "backup",
            level: Level::Warn,
            detail: "no backup directory is configured, so this installation is \
                     one disk failure from losing everything. Set --backup-dir."
                .into(),
        };
    };
    let Some(latest) = crate::backup::latest(dir) else {
        return Check {
            name: "backup",
            level: Level::Fail,
            detail: format!(
                "no backup has ever been written to {}. Run `axgf-cms backup --dest {}`.",
                dir.display(),
                dir.display()
            ),
        };
    };
    let age = time::OffsetDateTime::now_utc() - latest.taken;
    let hours = age.whole_hours();
    let human = format!(
        "newest archive {} is {}",
        latest
            .path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy(),
        describe_age(hours)
    );
    let level = if hours >= BACKUP_FAIL_HOURS {
        Level::Fail
    } else if hours >= BACKUP_WARN_HOURS {
        Level::Warn
    } else {
        Level::Ok
    };
    let detail = match level {
        Level::Ok => human,
        _ => format!(
            "{human} — the timer should write one a day. \
             Check `systemctl status axgf-cms-backup.timer`."
        ),
    };
    Check {
        name: "backup",
        level,
        detail,
    }
}

/// Hours as something a person reads without arithmetic.
fn describe_age(hours: i64) -> String {
    match hours {
        h if h < 1 => "under an hour old".to_string(),
        1 => "1 hour old".to_string(),
        h if h < 48 => format!("{h} hours old"),
        h => format!("{} days old", h / 24),
    }
}

/// Does the payload cache still hold every file the bundle declares?
///
/// This is the failure that hides: the bundle is fine, every page renders, and
/// then one photograph 404s because a cache directory was cleaned out from
/// under the process. The export path recovers from it, but only when somebody
/// saves; until then nothing says so.
fn cache_check(state: &crate::state::AppState) -> Check {
    let declared = state.read(|flat| {
        flat.get("external_payloads")
            .and_then(Value::as_object)
            .map(|m| m.keys().cloned().collect::<Vec<_>>())
            .unwrap_or_default()
    });
    if declared.is_empty() {
        return Check {
            name: "cache",
            level: Level::Ok,
            detail: "no payloads in this bundle".into(),
        };
    }
    let missing = state
        .payloads()
        .missing_among(declared.iter().map(String::as_str));
    if missing.is_empty() {
        return Check {
            name: "cache",
            level: Level::Ok,
            detail: format!("{} payloads present", declared.len()),
        };
    }
    Check {
        name: "cache",
        level: Level::Warn,
        detail: format!(
            "{} of {} payloads are missing from {}. They are rebuilt from the \
             bundle on the next save; until then those documents cannot be \
             downloaded.",
            missing.len(),
            declared.len(),
            state.payloads().dir().display()
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(name: &'static str, level: Level) -> Check {
        Check {
            name,
            level,
            detail: String::new(),
        }
    }

    #[test]
    fn the_worst_check_decides_the_status() {
        let r = Report {
            checks: vec![check("bundle", Level::Ok), check("disk", Level::Warn)],
        };
        assert_eq!(r.level(), Level::Warn);
        assert_eq!(
            r.http_status(),
            axum::http::StatusCode::OK,
            "a warning is not an outage"
        );

        let r = Report {
            checks: vec![check("bundle", Level::Ok), check("backup", Level::Fail)],
        };
        assert_eq!(r.level(), Level::Fail);
        assert_eq!(r.http_status(), axum::http::StatusCode::SERVICE_UNAVAILABLE);
    }

    #[test]
    fn every_check_appears_in_the_json_under_its_own_name() {
        let r = Report {
            checks: vec![check("bundle", Level::Ok), check("disk", Level::Fail)],
        };
        let j = r.to_json();
        assert_eq!(j["status"], "fail");
        assert_eq!(j["checks"]["bundle"]["status"], "ok");
        assert_eq!(j["checks"]["disk"]["status"], "fail");
    }

    #[test]
    fn an_unconfigured_backup_directory_warns_rather_than_failing() {
        // It is a decision somebody may have made deliberately — an
        // installation whose bundle is on a replicated volume, say — and a
        // monitor that reports it as an outage would be muted within a week.
        let c = backup_check(None);
        assert_eq!(c.level, Level::Warn);
        assert!(c.detail.contains("--backup-dir"), "{}", c.detail);
    }

    #[test]
    fn a_backup_directory_that_has_never_been_written_to_fails() {
        let dir = crate::scratch::Dir::new("health-empty");
        let c = backup_check(Some(&dir));
        assert_eq!(c.level, Level::Fail, "{}", c.detail);
        assert!(c.detail.contains("axgf-cms backup"), "{}", c.detail);
    }

    #[test]
    fn ages_read_as_english() {
        assert_eq!(describe_age(0), "under an hour old");
        assert_eq!(describe_age(1), "1 hour old");
        assert_eq!(describe_age(30), "30 hours old");
        assert_eq!(describe_age(72), "3 days old");
    }

    #[test]
    fn a_disk_with_room_is_ok() {
        let here = std::env::current_dir().unwrap().join("Cargo.toml");
        let c = disk_check(&here);
        assert_eq!(c.level, Level::Ok, "{}", c.detail);
        assert!(c.detail.contains("free of"), "{}", c.detail);
    }
}
