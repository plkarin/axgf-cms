//! Which version runs, which ones are published, and asking for another one
//! (v0.5.0, docs/adr/0011 §5).
//!
//! # Why the web process never upgrades itself
//!
//! Replacing the binary and restarting the service needs root, and this
//! process runs as an unprivileged user in a sandbox that cannot write
//! `/usr/local/bin` and cannot reach the internet. That is the point of the
//! sandbox, and a button that worked around it would undo it. So the button
//! *asks*: it writes a request into the data directory, and a small root
//! helper that bootstrap.sh installs (`axgf-cms-upgrade.path` and its
//! service) does the upgrade with bootstrap.sh's own `--upgrade` — the
//! pre-upgrade backup, the checksum, the health check and the automatic
//! rollback — and writes back how it went. The request names a tag and
//! nothing else, and the helper accepts only a published release's tag: the
//! most a compromised web process could ask for is another official release.
//!
//! # Where the list of releases comes from
//!
//! `releases.json` in the data directory. A helper timer writes it daily; a
//! process that can reach GitHub itself (a development machine, an
//! installation whose operator allowed it the network) may refresh it
//! directly. A private repository needs a token: `AXGF_CMS_GITHUB_TOKEN` in
//! the environment, read when it is used and never shown or stored here.
//!
//! # Files
//!
//! * `releases.json` — the published releases, newest first, and when.
//! * `releases.request` — asks the helper for a fresh list.
//! * `upgrade.request` — asks the helper for a version.
//! * `upgrade.status` — what the helper did with it.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

/// Where the releases are published.
pub const REPO: &str = "plkarin/axgf-cms";
/// The environment variable holding a token for a private repository.
pub const TOKEN_ENV: &str = "AXGF_CMS_GITHUB_TOKEN";
/// Where the upgrade helper reads a token from: root's alone, never the web
/// service's.
pub const TOKEN_FILE: &str = "/etc/axgf-cms/github-token";
/// The helper's request unit, as bootstrap.sh writes it.
pub const HELPER_UNIT: &str = "/etc/systemd/system/axgf-cms-upgrade.path";
/// How old a list may be before the page says it cannot tell.
pub const STALE_DAYS: i64 = 7;

/// This build's version.
pub fn current() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// One published release.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Release {
    pub tag: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub published_at: String,
    #[serde(default)]
    pub prerelease: bool,
    #[serde(default)]
    pub url: String,
}

/// The list, as last fetched.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Channel {
    #[serde(default)]
    pub checked_at: Option<String>,
    /// `helper` or `direct`: who fetched it.
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub releases: Vec<Release>,
    /// Why the last attempt failed, when it did.
    #[serde(default)]
    pub error: Option<String>,
    /// Whether the fetch carried a token: the page's answer to "can a private
    /// repository be reached", since the token itself lives with the helper.
    #[serde(default)]
    pub authenticated: bool,
}

impl Channel {
    /// The newest release that is not a release candidate.
    pub fn latest_stable(&self) -> Option<&Release> {
        self.releases
            .iter()
            .filter(|r| !r.prerelease)
            .max_by(|a, b| compare(&a.tag, &b.tag))
    }

    /// Is a stable release newer than this build published?
    pub fn newer_stable(&self) -> Option<&Release> {
        self.latest_stable()
            .filter(|r| compare(&r.tag, current()) == std::cmp::Ordering::Greater)
    }

    /// Is `tag` one of the published releases?
    pub fn has(&self, tag: &str) -> bool {
        self.releases.iter().any(|r| r.tag == tag)
    }

    /// Days since the list was fetched; `None` if it never was.
    pub fn age_days(&self) -> Option<i64> {
        let at = self.checked_at.as_deref()?;
        let then =
            time::OffsetDateTime::parse(at, &time::format_description::well_known::Rfc3339).ok()?;
        Some((time::OffsetDateTime::now_utc() - then).whole_days())
    }
}

/// `v0.5.0` against `0.4.3`, numerically; a release candidate (`-rc1`) sorts
/// before its release.
pub fn compare(a: &str, b: &str) -> std::cmp::Ordering {
    let split = |v: &str| -> (Vec<u64>, Option<String>) {
        let v = v.trim().trim_start_matches('v');
        let (core, pre) = match v.split_once('-') {
            Some((c, p)) => (c, Some(p.to_string())),
            None => (v, None),
        };
        (
            core.split('.').map(|p| p.parse().unwrap_or(0)).collect(),
            pre,
        )
    };
    let (ca, pa) = split(a);
    let (cb, pb) = split(b);
    for i in 0..ca.len().max(cb.len()) {
        let (x, y) = (
            ca.get(i).copied().unwrap_or(0),
            cb.get(i).copied().unwrap_or(0),
        );
        if x != y {
            return x.cmp(&y);
        }
    }
    match (pa, pb) {
        (None, None) => std::cmp::Ordering::Equal,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (Some(_), None) => std::cmp::Ordering::Less,
        (Some(x), Some(y)) => x.cmp(&y),
    }
}

/// A tag the helper would accept: `v` and a semantic version.
pub fn is_tag(tag: &str) -> bool {
    let Some(rest) = tag.strip_prefix('v') else {
        return false;
    };
    let (core, pre) = match rest.split_once('-') {
        Some((c, p)) => (c, Some(p)),
        None => (rest, None),
    };
    let parts: Vec<&str> = core.split('.').collect();
    parts.len() == 3
        && parts
            .iter()
            .all(|p| !p.is_empty() && p.chars().all(|c| c.is_ascii_digit()))
        && pre.is_none_or(|p| {
            !p.is_empty() && p.chars().all(|c| c.is_ascii_alphanumeric() || c == '.')
        })
        && tag.len() <= 40
}

fn path(dir: &Path, name: &str) -> PathBuf {
    dir.join(name)
}

/// The list in `dir`, if one was ever written.
pub fn read(dir: &Path) -> Option<Channel> {
    let text = std::fs::read_to_string(path(dir, "releases.json")).ok()?;
    serde_json::from_str(&text).ok()
}

/// Write a file atomically, readable by its owner and root only.
fn write_atomic(target: &Path, body: &str) -> std::io::Result<()> {
    let tmp = target.with_extension(format!("tmp-{}", uuid::Uuid::new_v4()));
    {
        let mut opts = std::fs::OpenOptions::new();
        opts.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            opts.mode(0o600);
        }
        let mut f = opts.open(&tmp)?;
        use std::io::Write as _;
        f.write_all(body.as_bytes())?;
        f.sync_all()?;
    }
    std::fs::rename(&tmp, target)
}

/// Whether the root helper is installed *for this installation*, so the
/// button can work: bootstrap.sh's unit, watching this data directory's
/// `upgrade.request`, or `helper.json` beside it (a hand install, the
/// tests). A unit watching another directory is another installation's
/// helper: the preview, the demo or a test suite on the same host would
/// otherwise offer a button nothing answers.
pub fn helper_installed(dir: &Path) -> bool {
    path(dir, "helper.json").exists() || unit_watches(Path::new(HELPER_UNIT), dir)
}

/// Whether the path unit at `unit` watches `dir`'s request file.
fn unit_watches(unit: &Path, dir: &Path) -> bool {
    let Ok(text) = std::fs::read_to_string(unit) else {
        return false;
    };
    let want = path(dir, "upgrade.request");
    text.lines()
        .filter_map(|l| l.trim().strip_prefix("PathExists="))
        .any(|p| Path::new(p.trim()) == want)
}

/// Ask the helper for a fresh list.
pub fn request_list(dir: &Path) -> std::io::Result<()> {
    write_atomic(
        &path(dir, "releases.request"),
        &serde_json::json!({ "requested_at": crate::view::now_iso8601() }).to_string(),
    )
}

/// Fetch the list from GitHub from this process, and write it. Blocking:
/// call it from `spawn_blocking`.
pub fn fetch_direct(dir: &Path) -> Channel {
    fetch_into(dir, "direct")
}

/// Fetch the list and write it, saying who fetched it: `direct` from the web
/// process, `helper` from `axgf-cms releases` run by the upgrade helper.
pub fn fetch_into(dir: &Path, source: &str) -> Channel {
    let mut channel = read(dir).unwrap_or_default();
    channel.source = source.into();
    channel.checked_at = Some(crate::view::now_iso8601());
    let token = std::env::var(TOKEN_ENV)
        .ok()
        .filter(|t| !t.trim().is_empty());
    channel.authenticated = token.is_some();
    match fetch(REPO, token) {
        Ok(list) => {
            channel.releases = list;
            channel.error = None;
        }
        Err(e) => channel.error = Some(e),
    }
    if let Ok(body) = serde_json::to_string_pretty(&channel) {
        if let Err(e) = write_atomic(&path(dir, "releases.json"), &body) {
            tracing::warn!(error = %e, "could not write releases.json");
        }
    }
    channel
}

fn fetch(repo: &str, token: Option<String>) -> Result<Vec<Release>, String> {
    let agent = ureq::Agent::new_with_config(
        ureq::Agent::config_builder()
            .timeout_global(Some(std::time::Duration::from_secs(10)))
            .user_agent(concat!(
                env!("CARGO_PKG_NAME"),
                "/",
                env!("CARGO_PKG_VERSION")
            ))
            .http_status_as_error(false)
            .build(),
    );
    let url = format!("https://api.github.com/repos/{repo}/releases?per_page=50");
    let mut req = agent
        .get(&url)
        .header("Accept", "application/vnd.github+json");
    if let Some(t) = token.as_deref() {
        req = req.header("Authorization", &format!("Bearer {}", t.trim()));
    }
    let mut resp = req.call().map_err(|e| e.to_string())?;
    let status = resp.status().as_u16();
    let body = resp
        .body_mut()
        .read_to_string()
        .map_err(|e| e.to_string())?;
    if status == 404 && token.is_none() {
        return Err(format!(
            "{url} answered 404: the repository is private or does not exist. A private one needs a token: {TOKEN_ENV}, which the upgrade helper reads from {TOKEN_FILE}."
        ));
    }
    if !(200..300).contains(&status) {
        return Err(format!("{url} answered {status}"));
    }
    parse_github(&body)
}

/// GitHub's release list, as this module keeps it.
pub fn parse_github(body: &str) -> Result<Vec<Release>, String> {
    let list: Vec<serde_json::Value> = serde_json::from_str(body).map_err(|e| e.to_string())?;
    let mut out: Vec<Release> = list
        .iter()
        .filter(|r| r.get("draft").and_then(|d| d.as_bool()) != Some(true))
        .filter_map(|r| {
            let s = |k: &str| r.get(k).and_then(|v| v.as_str()).unwrap_or("").to_string();
            let tag = s("tag_name");
            is_tag(&tag).then(|| Release {
                name: s("name"),
                published_at: s("published_at"),
                prerelease: r
                    .get("prerelease")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false),
                url: s("html_url"),
                tag,
            })
        })
        .collect();
    out.sort_by(|a, b| compare(&b.tag, &a.tag));
    Ok(out)
}

/// An upgrade asked for.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    pub version: String,
    pub requested_by: String,
    pub requested_at: String,
    pub id: String,
}

/// Ask the helper to install `version`. The tag is checked here and again by
/// the helper, which also checks it is published.
pub fn request_upgrade(dir: &Path, version: &str, by: &str) -> Result<Request, String> {
    if !is_tag(version) {
        return Err(format!("{version} is not a release tag"));
    }
    let req = Request {
        version: version.to_string(),
        requested_by: by.to_string(),
        requested_at: crate::view::now_iso8601(),
        id: uuid::Uuid::new_v4().to_string(),
    };
    let body = serde_json::to_string_pretty(&req).map_err(|e| e.to_string())?;
    write_atomic(&path(dir, "upgrade.request"), &body).map_err(|e| e.to_string())?;
    Ok(req)
}

/// The request waiting for the helper, if any.
pub fn pending(dir: &Path) -> Option<Request> {
    let text = std::fs::read_to_string(path(dir, "upgrade.request")).ok()?;
    serde_json::from_str(&text).ok()
}

/// What the helper last did.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Status {
    /// `running`, `succeeded`, `failed` or `refused`.
    #[serde(default)]
    pub state: String,
    #[serde(default)]
    pub from: String,
    #[serde(default)]
    pub to: String,
    #[serde(default)]
    pub started_at: Option<String>,
    #[serde(default)]
    pub finished_at: Option<String>,
    #[serde(default)]
    pub message: String,
    /// The last lines of bootstrap.sh's output.
    #[serde(default)]
    pub log: Vec<String>,
}

/// The helper's last report.
pub fn status(dir: &Path) -> Option<Status> {
    let text = std::fs::read_to_string(path(dir, "upgrade.status")).ok()?;
    serde_json::from_str(&text).ok()
}

#[cfg(test)]
mod tests {

    #[test]
    fn a_helper_unit_serves_the_directory_it_watches_and_no_other() {
        let tmp = crate::scratch::Dir::new("helper-unit");
        let unit = tmp.join("axgf-cms-upgrade.path");
        let ours = tmp.join("data");
        std::fs::write(
            &unit,
            format!(
                "[Path]\nPathExists={}\nUnit=axgf-cms-upgrade.service\n",
                ours.join("upgrade.request").display()
            ),
        )
        .unwrap();
        assert!(unit_watches(&unit, &ours));
        assert!(!unit_watches(&unit, &tmp.join("preview")));
        assert!(!unit_watches(&tmp.join("absent.path"), &ours));
    }

    use super::*;
    use std::cmp::Ordering;

    #[test]
    fn versions_compare_as_releases_do() {
        assert_eq!(compare("v0.5.0", "0.4.3"), Ordering::Greater);
        assert_eq!(compare("v0.4.10", "v0.4.9"), Ordering::Greater);
        assert_eq!(compare("v0.5.0-rc1", "v0.5.0"), Ordering::Less);
        assert_eq!(compare("v0.5.0", "0.5.0"), Ordering::Equal);
    }

    #[test]
    fn only_a_release_tag_is_a_tag() {
        for ok in ["v0.4.3", "v1.0.0-rc1", "v10.20.30"] {
            assert!(is_tag(ok), "{ok}");
        }
        for bad in [
            "0.4.3",
            "v0.4",
            "v0.4.3;rm -rf /",
            "v0.4.3 ",
            "v0.4.3-",
            "main",
            "v0.4.3-rc/1",
            "",
        ] {
            assert!(!is_tag(bad), "{bad}");
        }
    }

    #[test]
    fn githubs_list_keeps_releases_and_drops_drafts() {
        let body = r#"[
          {"tag_name":"v0.4.3","name":"0.4.3","published_at":"2026-10-14T00:00:00Z","prerelease":false,"draft":false,"html_url":"u"},
          {"tag_name":"v0.5.0-rc1","prerelease":true,"draft":false},
          {"tag_name":"v9.9.9","draft":true},
          {"tag_name":"nightly","draft":false}
        ]"#;
        let list = parse_github(body).unwrap();
        let tags: Vec<&str> = list.iter().map(|r| r.tag.as_str()).collect();
        assert_eq!(tags, vec!["v0.5.0-rc1", "v0.4.3"]);
        let ch = Channel {
            releases: list,
            ..Default::default()
        };
        assert_eq!(ch.latest_stable().map(|r| r.tag.as_str()), Some("v0.4.3"));
        assert!(
            ch.newer_stable().is_none(),
            "0.4.3 is older than this build"
        );
    }

    #[test]
    fn a_request_names_a_tag_and_nothing_else() {
        let dir = crate::scratch::Dir::new("releases");
        std::fs::create_dir_all(&dir).unwrap();
        assert!(request_upgrade(&dir, "v0.4.3; reboot", "anna").is_err());
        let r = request_upgrade(&dir, "v0.4.3", "anna").unwrap();
        assert_eq!(pending(&dir).unwrap().id, r.id);
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = std::fs::metadata(dir.join("upgrade.request"))
                .unwrap()
                .permissions()
                .mode();
            assert_eq!(mode & 0o077, 0, "{mode:o}");
        }
    }
}
