//! The infrastructure checks the Infrastructure page runs on every visit
//! (v0.5.0, docs/adr/0011 §3).
//!
//! These are the questions a review of an installation asks — the October
//! 2026 QA pass asked most of them by hand — turned into code that asks them
//! every time an administrator opens the page: how the site is reached, who
//! holds the keys, whether the data is safe on disk, whether it will still fit
//! next month, whether the version is current and whether the process is
//! still in its sandbox. Each answer is green, a warning or critical, with
//! what was found and, when it is not green, the plan to put it right.
//!
//! Nothing here acts. A check that could fix what it found would be a check
//! that changes the installation whenever somebody looks at it.

use std::time::Duration;

use crate::findings::{Item, Level};

/// The order the page lists the areas in.
pub const AREAS: [&str; 6] = [
    "security", "access", "data", "capacity", "release", "platform",
];

/// Failed sign-ins in a day past which somebody is guessing.
pub const FAILED_SIGN_INS_WARN: usize = 20;
/// Memory used against the service's ceiling: a warning, then critical.
pub const MEMORY_WARN: f64 = 0.80;
pub const MEMORY_FAIL: f64 = 0.95;
/// Days of room left, at the current growth: a warning, then critical.
pub const ROOM_WARN_DAYS: f64 = 60.0;
pub const ROOM_FAIL_DAYS: f64 = 14.0;

/// What the checks are run against.
pub struct Inputs<'a> {
    pub state: &'a crate::state::AppState,
    /// The request that opened the page arrived over TLS.
    pub request_tls: bool,
    pub volumes: &'a crate::host::Volumes,
    pub compute: &'a crate::host::Compute,
    pub channel: Option<&'a crate::releases::Channel>,
    pub helper: bool,
    pub lang: &'a str,
}

/// Every check.
pub fn run(x: &Inputs) -> Vec<Item> {
    let mut out = Vec::new();
    let state = x.state;
    let site = state.site();
    let bind = state.bind();
    let exposed = bind.is_some_and(|b| !b.ip().is_loopback());

    // -- security -----------------------------------------------------------
    let item = Item::new("transport", "security", "infra-transport-title").text(
        "bind",
        bind.map(|b| b.to_string()).unwrap_or_else(|| "—".into()),
    );
    out.push(if x.request_tls {
        item.ok("infra-transport-ok")
    } else if exposed {
        item.fail("infra-transport-exposed", "infra-transport-exposed-fix")
    } else {
        item.warn("infra-transport-plain", "infra-transport-plain-fix")
    });

    let item = Item::new("forwarded", "security", "infra-forwarded-title");
    out.push(if exposed {
        item.warn("infra-forwarded-exposed", "infra-forwarded-fix")
    } else {
        item.ok("infra-forwarded-ok")
    });

    let refused = state
        .events()
        .count(crate::events::Kind::CrossSiteRefused, crate::events::WINDOW);
    let item = Item::new("cross-site", "security", "infra-cross-site-title").num("n", refused);
    out.push(if refused == 0 {
        item.ok("infra-cross-site-ok")
    } else {
        item.warn("infra-cross-site-refused", "infra-cross-site-fix")
    });

    let enabled = crate::sso::Kind::ALL
        .into_iter()
        .filter(|k| site.providers.get(*k).enabled)
        .count();
    let item = Item::new("service-transport", "security", "infra-sso-title")
        .num("n", enabled)
        .link("/admin/connectivity");
    out.push(if enabled == 0 {
        item.ok("infra-sso-none")
    } else if site.public_is_https() {
        item.ok("infra-sso-ok")
    } else {
        item.fail("infra-sso-no-https", "infra-sso-fix")
    });

    // -- access -------------------------------------------------------------
    let admins = state.acl_read(|a| a.active_admins());
    let item = Item::new("admins", "access", "infra-admins-title")
        .num("n", admins)
        .link("/admin/users");
    out.push(match admins {
        0 => item.fail("infra-admins-none", "infra-admins-none-fix"),
        1 => item.warn("infra-admins-one", "infra-admins-one-fix"),
        _ => item.ok("infra-admins-ok"),
    });

    let item = Item::new("emergency-token", "access", "infra-token-title");
    out.push(if !state.standing_admin_token() {
        item.ok("infra-token-generated")
    } else if admins == 0 {
        item.ok("infra-token-only-way")
    } else {
        item.warn("infra-token-standing", "infra-token-fix")
    });

    let item = Item::new("registration", "access", "infra-registration-title")
        .link("/admin/users#registration");
    out.push(match site.registration.mode {
        crate::site::RegistrationMode::Closed => item.ok("infra-registration-closed"),
        crate::site::RegistrationMode::Secret => item.ok("infra-registration-secret"),
        crate::site::RegistrationMode::Open => {
            item.warn("infra-registration-open", "infra-registration-open-fix")
        }
    });

    let failed = state
        .events()
        .count(crate::events::Kind::SignInFailed, crate::events::WINDOW);
    let item = Item::new("sign-in-failures", "access", "infra-failures-title").num("n", failed);
    out.push(if failed >= FAILED_SIGN_INS_WARN {
        item.warn("infra-failures-many", "infra-failures-fix")
    } else {
        item.ok("infra-failures-ok")
    });

    // -- data: the operational checks /health already makes ----------------
    let health = state.health();
    for (name, title) in [
        ("bundle", "infra-health-bundle-title"),
        ("cache", "infra-health-cache-title"),
        ("backup", "infra-health-backup-title"),
    ] {
        let Some(c) = health.get(name) else { continue };
        let mut item = Item::new(
            match name {
                "bundle" => "bundle",
                "cache" => "cache",
                _ => "backup",
            },
            "data",
            title,
        );
        item.args = c.args.clone();
        item = match c.level {
            Level::Ok => item.ok(match name {
                "bundle" => "infra-health-bundle-ok",
                "cache" => "infra-health-cache-ok",
                _ => "infra-health-backup-ok",
            }),
            level => {
                let mut i = item;
                i.level = level;
                i.finding = c.key;
                i.fix = Some(match name {
                    "bundle" => "infra-health-bundle-fix",
                    "cache" => "infra-health-cache-fix",
                    _ => "infra-health-backup-fix",
                });
                i
            }
        };
        if name == "backup" {
            if let Some(b) = x.volumes.backups.as_ref() {
                item = item.num("count", b.count);
            }
        }
        if name == "bundle" {
            item = item.link("/admin/bundle");
        }
        out.push(item);
    }

    // -- capacity -----------------------------------------------------------
    if let Some(c) = health.get("disk") {
        let mut item = Item::new("disk", "capacity", "infra-disk-title");
        item.args = c.args.clone();
        if let Some(r) = x.volumes.room {
            // The same figures, written in the reader's language.
            item.args.retain(|(k, _)| !matches!(*k, "free" | "percent"));
            item = item
                .text("free", crate::documents::human_size_in(r.free, x.lang))
                .text("total", crate::documents::human_size_in(r.total, x.lang))
                .num(
                    "percent",
                    ((1.0 - r.used_fraction()) * 100.0).round() as i64,
                );
        }
        item = item.link("#volumes");
        out.push(match c.level {
            Level::Ok => item.ok("infra-disk-ok"),
            level => {
                let mut i = item;
                i.level = level;
                i.finding = c.key;
                i.fix = Some("infra-disk-fix");
                i
            }
        });
    }

    let item = Item::new("growth", "capacity", "infra-growth-title").link("#volumes");
    out.push(match (x.volumes.growth_per_day, x.volumes.days_left) {
        (Some(g), Some(days)) if g > 0.0 => {
            let item = item
                .text(
                    "rate",
                    crate::documents::human_size_in((g * 30.0) as u64, x.lang),
                )
                .num("days", days.round() as i64);
            if days < ROOM_FAIL_DAYS {
                item.fail("infra-growth-soon", "infra-growth-fix")
            } else if days < ROOM_WARN_DAYS {
                item.warn("infra-growth-soon", "infra-growth-fix")
            } else {
                item.ok("infra-growth-ok")
            }
        }
        (Some(_), _) => item.ok("infra-growth-flat"),
        _ => item.ok("infra-growth-unknown"),
    });

    let item = Item::new("memory", "capacity", "infra-memory-title").link("#compute");
    out.push(match (x.compute.cgroup_current, x.compute.cgroup_max) {
        (Some(used), Some(max)) if max > 0 => {
            let share = used as f64 / max as f64;
            let item = item
                .text("used", crate::documents::human_size_in(used, x.lang))
                .text("max", crate::documents::human_size_in(max, x.lang))
                .num("percent", (share * 100.0).round() as i64);
            if share >= MEMORY_FAIL {
                item.fail("infra-memory-high", "infra-memory-fix")
            } else if share >= MEMORY_WARN {
                item.warn("infra-memory-high", "infra-memory-fix")
            } else {
                item.ok("infra-memory-ok")
            }
        }
        _ => item
            .text(
                "used",
                crate::documents::human_size_in(x.compute.rss.unwrap_or(0), x.lang),
            )
            .ok("infra-memory-unlimited"),
    });

    // -- release ------------------------------------------------------------
    let item = Item::new("version", "release", "infra-version-title")
        .text("current", crate::releases::current())
        .link("#version");
    out.push(match x.channel {
        None => item.warn("infra-version-unknown", "infra-version-unknown-fix"),
        Some(ch) => match (ch.newer_stable(), ch.age_days()) {
            (Some(newer), _) => item
                .text("latest", newer.tag.clone())
                .warn("infra-version-behind", "infra-version-behind-fix"),
            (None, Some(age)) if age > crate::releases::STALE_DAYS => item
                .num("days", age)
                .warn("infra-version-stale", "infra-version-unknown-fix"),
            (None, _) if ch.releases.is_empty() => {
                item.warn("infra-version-unknown", "infra-version-unknown-fix")
            }
            (None, _) => item.ok("infra-version-ok"),
        },
    });

    let item = Item::new("upgrades", "release", "infra-upgrades-title").link("#version");
    out.push(if x.helper {
        item.ok("infra-upgrades-ok")
    } else {
        item.warn("infra-upgrades-manual", "infra-upgrades-fix")
    });

    // -- platform -----------------------------------------------------------
    let item = Item::new("sandbox", "platform", "infra-sandbox-title");
    out.push(match (x.compute.no_new_privs, x.compute.seccomp) {
        (Some(true), Some(2)) => item.ok("infra-sandbox-ok"),
        (None, None) => item.ok("infra-sandbox-unknown"),
        _ => item.warn("infra-sandbox-off", "infra-sandbox-fix"),
    });

    let year = crate::living::current_year();
    let item = Item::new("clock", "platform", "infra-clock-title").num("year", year);
    out.push(if year < 2026 {
        item.fail("infra-clock-wrong", "infra-clock-fix")
    } else {
        item.ok("infra-clock-ok")
    });

    out
}

/// How long ago, as whole days, hours or minutes — for "counting for".
pub fn coarse(d: Duration) -> (i64, &'static str) {
    let s = d.as_secs() as i64;
    if s >= 86_400 {
        (s / 86_400, "days")
    } else if s >= 3_600 {
        (s / 3_600, "hours")
    } else {
        // "0 minutes" read as a fault; a process up for seconds has been up
        // for a minute, near enough.
        ((s / 60).max(1), "minutes")
    }
}
