//! The sign-in jail: an address that keeps failing to get in is shut out of
//! every way in for a while, and for longer each time (docs/adr/0047).
//!
//! The login throttle it replaces counted failures per address, eight in a
//! quarter of an hour, and refused the ninth until the quarter was out. An
//! address could come back every fifteen minutes for ever, a hundred attempts
//! sent at once all passed the count together, and nothing in the journal
//! said who it was. Asked for on 10 November 2026, the day the operator put
//! the sign-in page of ax-genealogy.com in front of the internet:
//!
//! - **Counted before it is compared.** An attempt is charged when it begins
//!   and given back if it succeeds, so attempts sent at once cannot pass the
//!   count together: the ninth waits for no hash, it is refused.
//! - **A term, longer each time.** [`MAX_FAILURES`] failed sign-ins in
//!   [`WINDOW`], or [`MAX_TOKEN_FAILURES`] wrong emergency tokens, and the
//!   address is shut out for the first of [`TERMS`]: fifteen minutes, then an
//!   hour, six hours, a day, and a day again after that. [`MEMORY`] without a
//!   term and the next one is the first again.
//! - **Every way in.** The password, the emergency token and the registration
//!   secret ask the same jail, and a term closes all three.
//! - **The emergency token, for everyone.** It opens an administrator's
//!   session and has no account to lock, so wrong tokens are also counted from
//!   every address together: [`MAX_TOKEN_FAILURES_ANYWHERE`] in an hour, and
//!   the token is refused to all until the hour is out.
//! - **In the journal, once a term**, with the address in the sentence —
//!   `sign-in jail: 203.0.113.7 shut out for 900 s` — which is the line a
//!   fail2ban filter bans on (README, "Rate limiting and jail"). What happens
//!   during a term is not written down: an address that keeps knocking would
//!   otherwise fill the journal for it.
//!
//! The jail lives in memory, like the throttle did. Restarting the service
//! empties it, which is also how an operator who shut themselves out gets
//! back in.

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

/// Failed sign-ins an address may make in [`WINDOW`] before a term.
pub const MAX_FAILURES: u32 = 8;

/// Wrong emergency tokens an address may offer in [`WINDOW`] before a term.
/// Fewer than passwords: the token opens an administrator's session, and the
/// one person who has it pastes it rather than types it.
pub const MAX_TOKEN_FAILURES: u32 = 3;

/// The window failures are counted over, from the first.
pub const WINDOW: Duration = Duration::from_secs(15 * 60);

/// The terms, the first time and each time after. The last one repeats.
pub const TERMS: [Duration; 4] = [
    Duration::from_secs(15 * 60),
    Duration::from_secs(60 * 60),
    Duration::from_secs(6 * 60 * 60),
    Duration::from_secs(24 * 60 * 60),
];

/// How long after its last term an address is remembered. Past it, the next
/// term is the first again.
pub const MEMORY: Duration = Duration::from_secs(24 * 60 * 60);

/// Wrong emergency tokens from every address together, in
/// [`TOKEN_HOUR`], before the token is refused to all.
pub const MAX_TOKEN_FAILURES_ANYWHERE: u32 = 20;

/// The period [`MAX_TOKEN_FAILURES_ANYWHERE`] is counted over.
pub const TOKEN_HOUR: Duration = Duration::from_secs(60 * 60);

/// Addresses kept before the forgotten ones are swept out.
const SWEEP_ABOVE: usize = 1024;

/// A way in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Way {
    /// A username and a password — or a registration secret, which is a
    /// guess of the same kind.
    Password,
    /// The emergency token.
    Token,
}

/// Why an attempt was refused before it was compared.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Refusal {
    /// The address is serving a term, for this long yet.
    Jailed(Duration),
    /// The emergency token is refused to everyone, for this long yet.
    TokenClosed(Duration),
}

impl Refusal {
    /// How long until an attempt can be made again.
    pub fn wait(self) -> Duration {
        match self {
            Refusal::Jailed(d) | Refusal::TokenClosed(d) => d,
        }
    }
}

/// One address's record.
#[derive(Debug, Clone, Copy)]
struct Cell {
    /// Failed sign-ins in the window, attempts under way included.
    failures: u32,
    /// Wrong emergency tokens in the window, attempts under way included.
    token_failures: u32,
    /// When the window began; `None` when nothing is counted.
    window: Option<Instant>,
    /// Terms served while remembered.
    terms: u32,
    /// When the current or last term ends.
    until: Option<Instant>,
}

impl Cell {
    const fn new() -> Self {
        Cell {
            failures: 0,
            token_failures: 0,
            window: None,
            terms: 0,
            until: None,
        }
    }

    fn term_left(&self, now: Instant) -> Option<Duration> {
        self.until
            .and_then(|until| until.checked_duration_since(now))
            .filter(|left| !left.is_zero())
    }

    /// Start counting afresh once the window is over.
    fn roll(&mut self, now: Instant) {
        if self
            .window
            .is_some_and(|start| now.saturating_duration_since(start) >= WINDOW)
        {
            self.failures = 0;
            self.token_failures = 0;
            self.window = None;
        }
    }

    fn used_up(&self, way: Way) -> bool {
        match way {
            Way::Password => self.failures >= MAX_FAILURES,
            Way::Token => self.token_failures >= MAX_TOKEN_FAILURES,
        }
    }

    /// Begin a term now. Returns its length: the first of [`TERMS`] when
    /// `escalates` is false, whatever came before.
    fn lock(&mut self, now: Instant, escalates: bool) -> Duration {
        let forgotten = self
            .until
            .is_some_and(|until| now.saturating_duration_since(until) >= MEMORY);
        if forgotten {
            self.terms = 0;
        }
        let served = if escalates { self.terms as usize } else { 0 };
        let term = TERMS[served.min(TERMS.len() - 1)];
        self.terms = self.terms.saturating_add(1);
        self.until = Some(now + term);
        self.failures = 0;
        self.token_failures = 0;
        self.window = None;
        term
    }

    /// Nothing left worth keeping: no count, no term, nothing remembered.
    fn forgotten(&self, now: Instant) -> bool {
        let counting = self
            .window
            .is_some_and(|start| now.saturating_duration_since(start) < WINDOW);
        let remembered = self
            .until
            .is_some_and(|until| now.saturating_duration_since(until) < MEMORY);
        !counting && !remembered
    }
}

#[derive(Debug, Default)]
struct Inner {
    cells: HashMap<String, Cell>,
    /// Wrong emergency tokens from every address, and when the hour began.
    tokens_anywhere: u32,
    tokens_since: Option<Instant>,
    /// When the token was refused to everyone, until.
    tokens_closed_until: Option<Instant>,
}

/// The sign-in jail. One per process, in [`crate::session::SessionStore`].
#[derive(Debug, Default)]
pub struct Jail {
    inner: Mutex<Inner>,
}

/// An attempt that has been charged. Say how it went: dropped without a
/// word, it stays counted as a failure.
#[must_use = "say whether the attempt succeeded, or it counts as a failure"]
pub struct Charge<'a> {
    jail: &'a Jail,
    key: String,
    addr: String,
    way: Way,
}

impl Charge<'_> {
    /// The attempt got in: the address's failures are forgotten. Its terms
    /// are not — a success does not wipe a record.
    pub fn succeeded(self) {
        self.jail.succeeded_at(&self.key);
    }

    /// The attempt was not a guess after all (an account created with the
    /// right secret, say): give back its charge and nothing else.
    pub fn refund(self) {
        self.jail.refund_at(&self.key, self.way);
    }

    /// The attempt failed. It was counted when it began; if it was the last
    /// one allowed, the term starts now, so the next is refused at once.
    pub fn failed(self) {
        self.jail
            .failed_at(&self.key, &self.addr, self.way, Instant::now());
    }
}

impl Jail {
    /// An empty jail.
    pub fn new() -> Self {
        Self::default()
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, Inner> {
        self.inner.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// Whether the address under `key` is serving a term, and for how long
    /// yet.
    pub fn term_left(&self, key: &str) -> Option<Duration> {
        self.term_left_at(key, Instant::now())
    }

    fn term_left_at(&self, key: &str, now: Instant) -> Option<Duration> {
        self.lock().cells.get(key).and_then(|c| c.term_left(now))
    }

    /// Charge an attempt by `way` from the address under `key` (written in
    /// the journal as `addr`), before anything is compared.
    ///
    /// Refused while the address serves a term, while the token is refused
    /// to everyone, and when the address has no attempt left — which starts
    /// its term.
    pub fn begin(&self, key: &str, addr: &str, way: Way) -> Result<Charge<'_>, Refusal> {
        self.begin_at(key, addr, way, Instant::now())?;
        Ok(Charge {
            jail: self,
            key: key.to_string(),
            addr: addr.to_string(),
            way,
        })
    }

    fn begin_at(&self, key: &str, addr: &str, way: Way, now: Instant) -> Result<(), Refusal> {
        let mut inner = self.lock();
        if inner.cells.len() > SWEEP_ABOVE {
            inner.cells.retain(|_, c| !c.forgotten(now));
        }
        if way == Way::Token {
            if let Some(left) = inner
                .tokens_closed_until
                .and_then(|until| until.checked_duration_since(now))
                .filter(|left| !left.is_zero())
            {
                return Err(Refusal::TokenClosed(left));
            }
        }
        let cell = inner.cells.entry(key.to_string()).or_insert(Cell::new());
        if let Some(left) = cell.term_left(now) {
            return Err(Refusal::Jailed(left));
        }
        cell.roll(now);
        if cell.used_up(way) {
            let term = cell.lock(now, !is_loopback(addr));
            let terms = cell.terms;
            drop(inner);
            log_term(addr, term, terms, way);
            return Err(Refusal::Jailed(term));
        }
        cell.window.get_or_insert(now);
        match way {
            Way::Password => cell.failures += 1,
            Way::Token => cell.token_failures += 1,
        }
        Ok(())
    }

    fn succeeded_at(&self, key: &str) {
        if let Some(cell) = self.lock().cells.get_mut(key) {
            cell.failures = 0;
            cell.token_failures = 0;
            cell.window = None;
        }
    }

    fn refund_at(&self, key: &str, way: Way) {
        if let Some(cell) = self.lock().cells.get_mut(key) {
            match way {
                Way::Password => cell.failures = cell.failures.saturating_sub(1),
                Way::Token => cell.token_failures = cell.token_failures.saturating_sub(1),
            }
        }
    }

    fn failed_at(&self, key: &str, addr: &str, way: Way, now: Instant) {
        let mut inner = self.lock();
        let mut closed = None;
        if way == Way::Token {
            let fresh = inner
                .tokens_since
                .is_none_or(|since| now.saturating_duration_since(since) >= TOKEN_HOUR);
            if fresh {
                inner.tokens_since = Some(now);
                inner.tokens_anywhere = 0;
            }
            inner.tokens_anywhere += 1;
            let already = inner.tokens_closed_until.is_some_and(|until| until > now);
            if inner.tokens_anywhere >= MAX_TOKEN_FAILURES_ANYWHERE && !already {
                let until = inner.tokens_since.unwrap_or(now) + TOKEN_HOUR;
                inner.tokens_closed_until = Some(until);
                closed = Some((inner.tokens_anywhere, until.saturating_duration_since(now)));
            }
        }
        let mut term = None;
        if let Some(cell) = inner.cells.get_mut(key) {
            if cell.term_left(now).is_none() && cell.used_up(way) {
                term = Some((cell.lock(now, !is_loopback(addr)), cell.terms));
            }
        }
        drop(inner);
        if let Some((term, terms)) = term {
            log_term(addr, term, terms, way);
        }
        if let Some((count, left)) = closed {
            tracing::warn!(
                wrong_tokens = count,
                seconds = left.as_secs(),
                "sign-in jail: the emergency token is refused to everyone for {} s, \
                 after {count} wrong tokens in an hour from every address together; \
                 restarting the service opens it again",
                left.as_secs()
            );
        }
    }

    /// Count a failure by `way` from `key` that was not charged beforehand —
    /// a registration secret, compared before anything else is spent.
    pub fn record_failure(&self, key: &str, addr: &str, way: Way) {
        let now = Instant::now();
        if self.begin_at(key, addr, way, now).is_ok() {
            self.failed_at(key, addr, way, now);
        }
    }
}

/// Whether `addr` is this machine.
///
/// Behind the documented proxy every visitor arrives from it, and only the
/// proxy's `X-Forwarded-For` says who they are. An attempt that comes from
/// this machine is therefore most likely every visitor at once, through a
/// proxy that does not say: their terms do not lengthen — the old throttle's
/// quarter of an hour — and the journal says why.
fn is_loopback(addr: &str) -> bool {
    addr.parse::<std::net::IpAddr>()
        .is_ok_and(|ip| ip.is_loopback())
}

/// The one line a term writes, with the address in the sentence for fail2ban.
fn log_term(addr: &str, term: Duration, terms: u32, way: Way) {
    let after = match way {
        Way::Password => format!("{MAX_FAILURES} failed sign-ins"),
        Way::Token => format!("{MAX_TOKEN_FAILURES} wrong emergency tokens"),
    };
    tracing::warn!(
        client = %addr,
        seconds = term.as_secs(),
        term = terms,
        "sign-in jail: {addr} shut out for {} s, after {after} (term {terms})",
        term.as_secs()
    );
    if is_loopback(addr) {
        tracing::warn!(
            "sign-in jail: those attempts came from this machine; if a proxy is in \
             front, it does not pass the visitor's address in X-Forwarded-For, and \
             every visitor shares that term (docs/DEPLOY.md)"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const KEY: &str = "203.0.113.7";

    fn fail(jail: &Jail, key: &str, way: Way, now: Instant) -> Result<(), Refusal> {
        jail.begin_at(key, key, way, now)?;
        jail.failed_at(key, key, way, now);
        Ok(())
    }

    #[test]
    fn eight_failures_start_a_term_and_the_ninth_is_refused() {
        let jail = Jail::new();
        let t = Instant::now();
        for i in 0..MAX_FAILURES {
            assert!(fail(&jail, KEY, Way::Password, t).is_ok(), "attempt {i}");
        }
        assert_eq!(jail.term_left_at(KEY, t), Some(TERMS[0]));
        assert_eq!(
            jail.begin_at(KEY, KEY, Way::Password, t),
            Err(Refusal::Jailed(TERMS[0]))
        );
        assert!(
            jail.begin_at("198.51.100.1", "x", Way::Password, t).is_ok(),
            "an address is jailed alone"
        );
    }

    #[test]
    fn attempts_under_way_count_so_a_burst_cannot_pass_together() {
        // Begun and not finished: the hashes are still running.
        let jail = Jail::new();
        let t = Instant::now();
        for _ in 0..MAX_FAILURES {
            jail.begin_at(KEY, KEY, Way::Password, t).unwrap();
        }
        assert!(matches!(
            jail.begin_at(KEY, KEY, Way::Password, t),
            Err(Refusal::Jailed(_))
        ));
    }

    #[test]
    fn a_success_forgives_the_failures_but_not_the_record() {
        let jail = Jail::new();
        let t = Instant::now();
        for _ in 0..MAX_FAILURES {
            fail(&jail, KEY, Way::Password, t).unwrap();
        }
        let after = t + TERMS[0];
        for _ in 0..MAX_FAILURES - 1 {
            fail(&jail, KEY, Way::Password, after).unwrap();
        }
        jail.begin_at(KEY, KEY, Way::Password, after).unwrap();
        jail.succeeded_at(KEY);
        for _ in 0..MAX_FAILURES {
            fail(&jail, KEY, Way::Password, after).unwrap();
        }
        assert_eq!(
            jail.term_left_at(KEY, after),
            Some(TERMS[1]),
            "the second term, though a success came between"
        );
    }

    #[test]
    fn terms_lengthen_and_a_quiet_day_starts_again() {
        let jail = Jail::new();
        let mut t = Instant::now();
        for (n, term) in TERMS.iter().chain([TERMS[3]].iter()).enumerate() {
            for _ in 0..MAX_FAILURES {
                fail(&jail, KEY, Way::Password, t).unwrap();
            }
            assert_eq!(jail.term_left_at(KEY, t), Some(*term), "term {}", n + 1);
            t += *term;
            assert_eq!(jail.term_left_at(KEY, t), None, "served");
        }
        t += MEMORY;
        for _ in 0..MAX_FAILURES {
            fail(&jail, KEY, Way::Password, t).unwrap();
        }
        assert_eq!(jail.term_left_at(KEY, t), Some(TERMS[0]));
    }

    #[test]
    fn this_machine_never_serves_more_than_the_first_term() {
        // Every visitor at once, most likely, through a proxy that does not
        // say who they are.
        let jail = Jail::new();
        let mut t = Instant::now();
        for addr in ["127.0.0.1", "::1"] {
            for _ in 0..3 {
                for _ in 0..MAX_FAILURES {
                    fail(&jail, addr, Way::Password, t).unwrap();
                }
                assert_eq!(jail.term_left_at(addr, t), Some(TERMS[0]), "{addr}");
                t += TERMS[0];
            }
        }
    }

    #[test]
    fn failures_older_than_the_window_are_not_counted() {
        let jail = Jail::new();
        let t = Instant::now();
        for _ in 0..MAX_FAILURES - 1 {
            fail(&jail, KEY, Way::Password, t).unwrap();
        }
        let later = t + WINDOW;
        for _ in 0..MAX_FAILURES - 1 {
            fail(&jail, KEY, Way::Password, later).unwrap();
        }
        assert_eq!(jail.term_left_at(KEY, later), None);
    }

    #[test]
    fn three_wrong_tokens_are_enough() {
        let jail = Jail::new();
        let t = Instant::now();
        for _ in 0..MAX_TOKEN_FAILURES {
            fail(&jail, KEY, Way::Token, t).unwrap();
        }
        assert_eq!(jail.term_left_at(KEY, t), Some(TERMS[0]));
        assert!(
            matches!(
                jail.begin_at(KEY, KEY, Way::Password, t),
                Err(Refusal::Jailed(_))
            ),
            "a term closes every way in"
        );
    }

    #[test]
    fn wrong_tokens_from_everywhere_close_the_token_for_everyone() {
        let jail = Jail::new();
        let t = Instant::now();
        for i in 0..MAX_TOKEN_FAILURES_ANYWHERE {
            let addr = format!("192.0.2.{i}");
            fail(&jail, &addr, Way::Token, t).unwrap();
        }
        let fresh = "198.51.100.200";
        assert_eq!(
            jail.begin_at(fresh, fresh, Way::Token, t),
            Err(Refusal::TokenClosed(TOKEN_HOUR))
        );
        assert!(
            jail.begin_at(fresh, fresh, Way::Password, t).is_ok(),
            "passwords are not the token"
        );
        assert!(jail
            .begin_at(fresh, fresh, Way::Token, t + TOKEN_HOUR)
            .is_ok());
    }

    #[test]
    fn a_refund_gives_back_one_charge() {
        let jail = Jail::new();
        let t = Instant::now();
        for _ in 0..MAX_FAILURES - 1 {
            fail(&jail, KEY, Way::Password, t).unwrap();
        }
        jail.begin_at(KEY, KEY, Way::Password, t).unwrap();
        jail.refund_at(KEY, Way::Password);
        jail.begin_at(KEY, KEY, Way::Password, t).unwrap();
        jail.failed_at(KEY, KEY, Way::Password, t);
        assert!(jail.term_left_at(KEY, t).is_some());
    }

    #[test]
    fn forgotten_addresses_are_swept() {
        let jail = Jail::new();
        let t = Instant::now();
        for i in 0..=SWEEP_ABOVE {
            let addr = format!("10.0.{}.{}", i / 256, i % 256);
            fail(&jail, &addr, Way::Password, t).unwrap();
        }
        let later = t + WINDOW;
        jail.begin_at(KEY, KEY, Way::Password, later).unwrap();
        assert_eq!(jail.lock().cells.len(), 1, "only the address still counted");
    }
}
