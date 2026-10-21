//! What happened at the door, recently (v0.5.0, docs/adr/0011 §3).
//!
//! The Infrastructure page and the accounts page say how many sign-ins
//! failed, how many accounts were opened and how many cross-site posts were
//! refused. Those are counts over the last day, kept in memory: they die with
//! the process, like the sessions, and the journal (`journalctl`) is where the
//! durable record of each event already is, one line per event.

use std::collections::VecDeque;
use std::sync::Mutex;
use std::time::{Duration, Instant};

/// The longest window a count is asked for.
pub const WINDOW: Duration = Duration::from_secs(24 * 60 * 60);
/// The most events kept. A flood of failed sign-ins is counted up to here and
/// then the oldest are forgotten, so the count is "at least".
const CAP: usize = 20_000;

/// The kinds of event counted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// A password, a token or a provider's answer was refused.
    SignInFailed,
    /// A sign-in succeeded, by any means.
    SignedIn,
    /// A visitor opened an account themselves.
    Registered,
    /// A form posted from another site, refused (docs/adr/0011 §2).
    CrossSiteRefused,
}

/// The events of the last day.
pub struct Events {
    started: Instant,
    log: Mutex<VecDeque<(Instant, Kind)>>,
}

impl Default for Events {
    fn default() -> Self {
        Self::new()
    }
}

impl Events {
    pub fn new() -> Self {
        Self {
            started: Instant::now(),
            log: Mutex::new(VecDeque::new()),
        }
    }

    /// Note one event, now.
    pub fn record(&self, kind: Kind) {
        let mut log = self.log.lock().unwrap_or_else(|e| e.into_inner());
        let now = Instant::now();
        while log
            .front()
            .is_some_and(|(at, _)| now.duration_since(*at) > WINDOW)
        {
            log.pop_front();
        }
        if log.len() >= CAP {
            log.pop_front();
        }
        log.push_back((now, kind));
    }

    /// How many of `kind` in the last `within`.
    pub fn count(&self, kind: Kind, within: Duration) -> usize {
        let log = self.log.lock().unwrap_or_else(|e| e.into_inner());
        let now = Instant::now();
        log.iter()
            .filter(|(at, k)| *k == kind && now.duration_since(*at) <= within)
            .count()
    }

    /// How long this process has been counting: a count "in the last day"
    /// from a process started an hour ago covers an hour.
    pub fn counting_for(&self) -> Duration {
        self.started.elapsed()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn events_are_counted_by_kind() {
        let e = Events::new();
        e.record(Kind::SignInFailed);
        e.record(Kind::SignInFailed);
        e.record(Kind::SignedIn);
        assert_eq!(e.count(Kind::SignInFailed, WINDOW), 2);
        assert_eq!(e.count(Kind::SignedIn, WINDOW), 1);
        assert_eq!(e.count(Kind::Registered, WINDOW), 0);
    }
}
