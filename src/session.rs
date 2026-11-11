//! Sessions and login throttling.
//!
//! # Kept beside the bundle, so a restart signs nobody out
//!
//! Sessions live in a `RwLock<HashMap>`, and every account's session is also
//! written to `family.sessions` beside the bundle, mode 600, with the secret
//! that signs the cookies (docs/adr/0028). They used to die with the process,
//! "a mild inconvenience at this scale" — until upgrades and settings changes
//! restarted the service ten times in an evening and the family was signed
//! out at every reload.
//!
//! - **Not in the bundle.** It is transient state, not the family's; the file
//!   people mail to each other never holds it.
//! - **Disposable.** A missing, unreadable or corrupt file only means signing
//!   in again: the store starts empty with a fresh secret. Deleting it and
//!   restarting is how an operator signs everyone out.
//! - **The emergency token's session is never written.** It ends with the
//!   process, as it always did.
//!
//! # How long a session lives
//!
//! Thirty days without being used ([`SESSION_TTL`]), and each use moves that
//! on: someone who comes back every week is never asked to sign in again. The
//! move is written to disk at most once an hour per session
//! ([`PERSIST_EVERY`]), not on every page. The browser keeps the cookie for as
//! long as browsers allow ([`COOKIE_MAX_AGE`]); the session's own expiry, on
//! the server, is what ends it.
//!
//! # The cookie
//!
//! `<session-id>.<hmac>`, where the id is 244 bits from the OS random source
//! and the signature is HMAC-SHA256 under a secret generated at startup. The
//! id alone is already unguessable, so the signature is not what makes the
//! session secure; it is what makes a forged or truncated cookie fail at the
//! door instead of turning into a map lookup miss, and it means a cookie from
//! a previous run of the process is rejected outright rather than by accident.
//!
//! `HttpOnly` keeps it away from script. `SameSite=Strict` is chosen over
//! `Lax` because every mutating route here is a form POST and there is no
//! third-party flow that needs the cookie to survive a cross-site navigation.
//! `Secure` is added only when the request actually arrived over TLS: setting
//! it unconditionally would make the cookie undeliverable on the documented
//! `http://localhost` deployment, and browsers would silently drop it.

use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock, RwLock};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use axum::http::header::{HeaderMap, COOKIE};
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// Name of the session cookie, before this installation's own mark
/// ([`cookie_name`]).
pub const COOKIE_NAME: &str = "axgf_session";

/// How long a session lives without being used. Each use moves it on.
pub const SESSION_TTL: Duration = Duration::from_secs(30 * 24 * 60 * 60);

/// How long the browser keeps the cookie: the longest browsers allow (400
/// days). The session's expiry on the server is what ends it.
pub const COOKIE_MAX_AGE: Duration = Duration::from_secs(400 * 24 * 60 * 60);

/// How far a session's expiry must move before the move is written to disk.
pub const PERSIST_EVERY: Duration = Duration::from_secs(60 * 60);

static INSTANCE_COOKIE: OnceLock<String> = OnceLock::new();

/// The session cookie's name: [`COOKIE_NAME`], followed by this
/// installation's mark once [`set_instance`] has named it.
///
/// A browser keeps one cookie per name and per host, whatever the port, so
/// two installations on one machine (a production and a preview, say) used to
/// overwrite each other's cookie: signing in to one signed you out of the
/// other.
pub fn cookie_name() -> &'static str {
    INSTANCE_COOKIE
        .get()
        .map(String::as_str)
        .unwrap_or(COOKIE_NAME)
}

/// The cookie name an installation over `bundle` takes: the first eight
/// hexadecimal digits of the SHA-256 of the bundle's path, which no other
/// installation can share.
pub fn instance_cookie_name(bundle: &Path) -> String {
    let digest = Sha256::digest(bundle.to_string_lossy().as_bytes());
    let mark: String = digest.iter().take(4).map(|b| format!("{b:02x}")).collect();
    format!("{COOKIE_NAME}_{mark}")
}

/// Give this process's cookie its installation's name. Once, at startup;
/// a later call changes nothing.
pub fn set_instance(bundle: &Path) {
    let _ = INSTANCE_COOKIE.set(instance_cookie_name(bundle));
}

/// Attempts to register allowed per client address in a window. A client
/// address that keeps failing to sign in is jailed instead ([`crate::jail`]).
pub const MAX_ATTEMPTS: u32 = 8;
/// Failed sign-ins allowed per account in a window, from every address
/// together. Three addresses' worth: one address is jailed long before it
/// could shut the account against its owner, and guesses spread over many
/// addresses still run out.
pub const MAX_ACCOUNT_FAILURES: u32 = 3 * crate::jail::MAX_FAILURES;
/// The window failed logins are counted over.
pub const ATTEMPT_WINDOW: Duration = Duration::from_secs(15 * 60);

/// What a live session grants.
#[derive(Debug, Clone)]
pub struct Session {
    /// The account id, or `None` for a session opened with `--admin-token`.
    pub user_id: Option<String>,
    /// True when this session came from the emergency token.
    pub emergency: bool,
    expires: SystemTime,
}

/// Live sessions, the login throttle and the sign-in jail.
pub struct SessionStore {
    secret: [u8; 32],
    sessions: RwLock<HashMap<String, Session>>,
    attempts: RwLock<HashMap<String, Attempts>>,
    /// The size of `attempts` at which its expired entries are next swept.
    attempts_sweep: std::sync::atomic::AtomicUsize,
    jail: crate::jail::Jail,
    /// Where the accounts' sessions are kept between runs. `None` keeps them
    /// in memory only.
    file: Option<PathBuf>,
    /// One write of the file at a time.
    writing: Mutex<()>,
}

/// The file's form.
#[derive(Serialize, Deserialize)]
struct Saved {
    format: u32,
    /// The signing secret, in hexadecimal.
    secret: String,
    /// Each account's session, by its id: the account and when it expires,
    /// in seconds since 1970.
    sessions: BTreeMap<String, SavedSession>,
}

#[derive(Serialize, Deserialize)]
struct SavedSession {
    user_id: String,
    expires: u64,
}

#[derive(Debug, Clone, Copy)]
struct Attempts {
    count: u32,
    window_start: Instant,
}

impl Default for SessionStore {
    fn default() -> Self {
        Self::new()
    }
}

impl SessionStore {
    /// A store with a fresh signing secret.
    pub fn new() -> Self {
        let a = *uuid::Uuid::new_v4().as_bytes();
        let b = *uuid::Uuid::new_v4().as_bytes();
        let mut secret = [0u8; 32];
        secret[..16].copy_from_slice(&a);
        secret[16..].copy_from_slice(&b);
        Self {
            secret,
            sessions: RwLock::new(HashMap::new()),
            attempts: RwLock::new(HashMap::new()),
            attempts_sweep: std::sync::atomic::AtomicUsize::new(1024),
            jail: crate::jail::Jail::new(),
            file: None,
            writing: Mutex::new(()),
        }
    }

    /// The file the sessions of the installation over `bundle` are kept in:
    /// `family.sessions` for `family.axgf`.
    pub fn path_for(bundle: &Path) -> PathBuf {
        bundle.with_extension("sessions")
    }

    /// The sessions kept beside `bundle`, those that have not expired, with
    /// the secret that signed their cookies. Nothing there, or nothing
    /// readable, is a fresh store: everyone signs in again, as a restart used
    /// to make them.
    pub fn beside(bundle: &Path) -> Self {
        let path = Self::path_for(bundle);
        let mut store = Self::new();
        match std::fs::read(&path) {
            Ok(bytes) => match serde_json::from_slice::<Saved>(&bytes) {
                Ok(saved) if saved.format == 1 => match unhex32(&saved.secret) {
                    Some(secret) => {
                        store.secret = secret;
                        let now = SystemTime::now();
                        let map = store.sessions.get_mut().unwrap_or_else(|e| e.into_inner());
                        for (id, s) in saved.sessions {
                            let expires = UNIX_EPOCH + Duration::from_secs(s.expires);
                            if expires > now {
                                map.insert(
                                    id,
                                    Session {
                                        user_id: Some(s.user_id),
                                        emergency: false,
                                        expires,
                                    },
                                );
                            }
                        }
                    }
                    None => tracing::warn!(
                        path = %path.display(),
                        "the sessions file holds no usable secret; everyone signs in again"
                    ),
                },
                _ => tracing::warn!(
                    path = %path.display(),
                    "the sessions file cannot be read; everyone signs in again"
                ),
            },
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => tracing::warn!(
                path = %path.display(),
                error = %e,
                "the sessions file cannot be opened; everyone signs in again"
            ),
        }
        store.file = Some(path);
        store
    }

    /// Write the accounts' live sessions to the file, when there is one. A
    /// failure is logged, never fatal: the cost is signing in again after the
    /// next restart.
    fn persist(&self) {
        let Some(path) = &self.file else { return };
        let _one = self.writing.lock().unwrap_or_else(|e| e.into_inner());
        let now = SystemTime::now();
        let sessions: BTreeMap<String, SavedSession> = {
            let map = self.sessions.read().unwrap_or_else(|e| e.into_inner());
            map.iter()
                .filter(|(_, s)| !s.emergency && s.expires > now)
                .filter_map(|(id, s)| {
                    Some((
                        id.clone(),
                        SavedSession {
                            user_id: s.user_id.clone()?,
                            expires: s.expires.duration_since(UNIX_EPOCH).ok()?.as_secs(),
                        },
                    ))
                })
                .collect()
        };
        let saved = Saved {
            format: 1,
            secret: self.secret.iter().map(|b| format!("{b:02x}")).collect(),
            sessions,
        };
        let written = serde_json::to_vec(&saved)
            .map_err(std::io::Error::other)
            .and_then(|bytes| crate::brain::write_private(path, &bytes));
        if let Err(e) = written {
            tracing::warn!(
                path = %path.display(),
                error = %e,
                "the sessions could not be written; they will not survive a restart"
            );
        }
    }

    /// Open a session for an account, returning the cookie value.
    pub fn open(&self, user_id: Option<String>, emergency: bool) -> String {
        let id = format!(
            "{}{}",
            uuid::Uuid::new_v4().simple(),
            uuid::Uuid::new_v4().simple()
        );
        let session = Session {
            user_id,
            emergency,
            expires: SystemTime::now() + SESSION_TTL,
        };
        {
            let mut map = self.sessions.write().unwrap_or_else(|e| e.into_inner());
            // Opportunistic sweep: expired entries are dropped whenever
            // somebody signs in, so an abandoned process does not accumulate
            // them forever without a background task.
            let now = SystemTime::now();
            map.retain(|_, s| s.expires > now);
            map.insert(id.clone(), session);
        }
        if !emergency {
            self.persist();
        }
        let sig = self.sign(&id);
        format!("{id}.{sig}")
    }

    /// Resolve a cookie value to a live session, refusing a bad signature.
    pub fn resolve(&self, cookie: &str) -> Option<Session> {
        let (id, sig) = cookie.split_once('.')?;
        if !constant_time_eq(sig.as_bytes(), self.sign(id).as_bytes()) {
            return None;
        }
        let now = SystemTime::now();
        let s = {
            let map = self.sessions.read().unwrap_or_else(|e| e.into_inner());
            map.get(id)?.clone()
        };
        if s.expires <= now {
            return None;
        }
        // Each use moves the expiry on, in steps of at least an hour, so a
        // session in use is written to disk at most once an hour.
        let fresh = now + SESSION_TTL;
        if fresh.duration_since(s.expires).unwrap_or_default() >= PERSIST_EVERY {
            {
                let mut map = self.sessions.write().unwrap_or_else(|e| e.into_inner());
                if let Some(live) = map.get_mut(id) {
                    live.expires = fresh;
                }
            }
            if !s.emergency {
                self.persist();
            }
        }
        Some(s)
    }

    /// Close one session.
    pub fn close(&self, cookie: &str) {
        let Some((id, _)) = cookie.split_once('.') else {
            return;
        };
        let removed = {
            let mut map = self.sessions.write().unwrap_or_else(|e| e.into_inner());
            map.remove(id)
        };
        if removed.is_some_and(|s| !s.emergency) {
            self.persist();
        }
    }

    /// Close every session belonging to an account.
    ///
    /// Used when an account is disabled, has its role lowered, or has its
    /// password changed: a live cookie that outlived the change would keep
    /// granting what was just taken away.
    pub fn close_all_for(&self, user_id: &str) {
        {
            let mut map = self.sessions.write().unwrap_or_else(|e| e.into_inner());
            map.retain(|_, s| s.user_id.as_deref() != Some(user_id));
        }
        self.persist();
    }

    /// How many sessions are live. For the admin dashboard.
    pub fn live(&self) -> usize {
        let now = SystemTime::now();
        let map = self.sessions.read().unwrap_or_else(|e| e.into_inner());
        map.values().filter(|s| s.expires > now).count()
    }

    /// HMAC-SHA256 of a session id under the process secret, hex-encoded.
    fn sign(&self, id: &str) -> String {
        let mut mac =
            Hmac::<Sha256>::new_from_slice(&self.secret).expect("HMAC accepts a key of any length");
        mac.update(id.as_bytes());
        let bytes = mac.finalize().into_bytes();
        let mut out = String::with_capacity(bytes.len() * 2);
        for b in bytes {
            use std::fmt::Write as _;
            let _ = write!(out, "{b:02x}");
        }
        out
    }

    // -- login throttling ---------------------------------------------------

    /// The sign-in jail, where client addresses are counted.
    pub fn jail(&self) -> &crate::jail::Jail {
        &self.jail
    }

    /// Whether `key` — a client's registrations — is out of attempts.
    pub fn is_throttled(&self, key: &str) -> bool {
        self.count_reaches(key, MAX_ATTEMPTS)
    }

    /// Charge a sign-in attempt to the account `username` before its password
    /// is compared; false, and nothing charged, when the account is out of
    /// attempts. Checked and counted at once, as the jail counts addresses:
    /// checked before the hash and counted after it, a burst from many
    /// addresses at once was compared in full. A success clears the count
    /// ([`Self::clear_account`]); a failure stays counted.
    pub fn charge_account(&self, username: &str) -> bool {
        let mut map = self.attempts.write().unwrap_or_else(|e| e.into_inner());
        let now = Instant::now();
        self.sweep_attempts(&mut map);
        let e = map.entry(account_key(username)).or_insert(Attempts {
            count: 0,
            window_start: now,
        });
        if e.window_start.elapsed() >= ATTEMPT_WINDOW {
            e.count = 0;
            e.window_start = now;
        }
        if e.count >= MAX_ACCOUNT_FAILURES {
            return false;
        }
        e.count += 1;
        true
    }

    /// The expired counts out, now and then rather than at every attempt:
    /// each sweep sets the next at twice what it kept.
    fn sweep_attempts(&self, map: &mut HashMap<String, Attempts>) {
        use std::sync::atomic::Ordering::Relaxed;
        if map.len() >= self.attempts_sweep.load(Relaxed) {
            map.retain(|_, a| a.window_start.elapsed() < ATTEMPT_WINDOW);
            self.attempts_sweep
                .store((map.len() * 2).max(1024), Relaxed);
        }
    }

    /// Forget the failed sign-ins against the account `username`: its owner
    /// got in.
    pub fn clear_account(&self, username: &str) {
        let mut map = self.attempts.write().unwrap_or_else(|e| e.into_inner());
        map.remove(&account_key(username));
    }

    fn count_reaches(&self, key: &str, limit: u32) -> bool {
        let map = self.attempts.read().unwrap_or_else(|e| e.into_inner());
        match map.get(key) {
            Some(a) if a.window_start.elapsed() < ATTEMPT_WINDOW => a.count >= limit,
            _ => false,
        }
    }

    /// Record a failed attempt against `key`.
    pub fn record_failure(&self, key: &str) {
        let mut map = self.attempts.write().unwrap_or_else(|e| e.into_inner());
        let now = Instant::now();
        self.sweep_attempts(&mut map);
        let e = map.entry(key.to_string()).or_insert(Attempts {
            count: 0,
            window_start: now,
        });
        if e.window_start.elapsed() >= ATTEMPT_WINDOW {
            e.count = 0;
            e.window_start = now;
        }
        e.count += 1;
    }

    /// Forget the failures against `key` after a success.
    pub fn clear_failures(&self, key: &str) {
        let mut map = self.attempts.write().unwrap_or_else(|e| e.into_inner());
        map.remove(key);
    }
}

/// The key an account's sign-in attempts are counted under: a digest of the
/// name, so that a name of two megabytes, sent before any password is
/// compared, costs what any other does; and in a namespace of its own, so that
/// no name can be a registration's bucket (`register:<address>`).
fn account_key(username: &str) -> String {
    let digest = Sha256::digest(username.as_bytes());
    let hex: String = digest.iter().map(|b| format!("{b:02x}")).collect();
    format!("account:{hex}")
}

/// The session cookie's value, if the request carries one.
pub fn cookie_value(headers: &HeaderMap) -> Option<String> {
    for value in headers.get_all(COOKIE).iter() {
        let Ok(raw) = value.to_str() else { continue };
        for pair in raw.split(';') {
            if let Some(v) = pair.trim().strip_prefix(&format!("{}=", cookie_name())) {
                return Some(v.to_string());
            }
        }
    }
    None
}

/// Any cookie's value, by name — used for the language and theme cookies.
pub fn named_cookie(headers: &HeaderMap, name: &str) -> Option<String> {
    for value in headers.get_all(COOKIE).iter() {
        let Ok(raw) = value.to_str() else { continue };
        for pair in raw.split(';') {
            if let Some(v) = pair.trim().strip_prefix(&format!("{name}=")) {
                return Some(v.to_string());
            }
        }
    }
    None
}

/// Whether the request reached us over TLS.
///
/// Direct TLS is not something this process terminates, so in practice this is
/// the reverse proxy's `X-Forwarded-Proto`. Trusting a header is only sound
/// because the documented deployment puts this behind a proxy that sets it;
/// the failure mode if it is absent is a cookie without `Secure`, which is
/// what plain HTTP needs anyway.
pub fn is_tls(headers: &HeaderMap) -> bool {
    headers
        .get("x-forwarded-proto")
        .and_then(|v| v.to_str().ok())
        .is_some_and(|v| {
            v.split(',')
                .next()
                .unwrap_or("")
                .trim()
                .eq_ignore_ascii_case("https")
        })
}

/// The client's address.
///
/// `X-Forwarded-For`'s first entry when a proxy on this machine set one and
/// it reads as an address, else the peer address axum recorded.
///
/// - **Only from this machine.** The documented proxy runs beside the
///   application, which binds 127.0.0.1; a peer elsewhere — the application
///   bound wider, with no proxy — is a client, and a client's header used to
///   be believed, so it could name any address: its own cell anew at every
///   attempt, or somebody else's to have it jailed. Without a peer at all
///   (the tests, which call the router directly) the header is read.
/// - **Only an address.** A value that is not one used to be taken whole,
///   and written into the journal line that names the client.
pub fn client_addr(
    headers: &HeaderMap,
    peer: Option<std::net::SocketAddr>,
) -> Option<std::net::IpAddr> {
    use std::net::{IpAddr, SocketAddr};
    let canonical = |ip: IpAddr| match ip {
        IpAddr::V6(v6) => v6.to_ipv4_mapped().map_or(IpAddr::V6(v6), IpAddr::V4),
        v4 => v4,
    };
    let peer = peer.map(|a| canonical(a.ip()));
    let from_here = peer.is_none_or(|ip| ip.is_loopback());
    let forwarded = headers
        .get("x-forwarded-for")
        .filter(|_| from_here)
        .and_then(|v| v.to_str().ok())
        .and_then(|xff| xff.split(',').next())
        .map(str::trim)
        .and_then(|first| {
            first
                .parse::<IpAddr>()
                .ok()
                .or_else(|| first.parse::<SocketAddr>().ok().map(|a| a.ip()))
        });
    forwarded.map(canonical).or(peer)
}

/// The bucket a client's attempts count against: its address, or for IPv6
/// the /64 it is in, since an IPv6 client can take any address of its /64 at
/// will. Without an address, every anonymous attempt shares one bucket, which
/// throttles harder rather than less.
pub fn client_key(headers: &HeaderMap, peer: Option<std::net::SocketAddr>) -> String {
    use std::net::{IpAddr, Ipv6Addr};
    match client_addr(headers, peer) {
        Some(IpAddr::V6(v6)) => {
            let s = v6.segments();
            format!("{}/64", Ipv6Addr::new(s[0], s[1], s[2], s[3], 0, 0, 0, 0))
        }
        Some(ip) => ip.to_string(),
        None => "unknown".into(),
    }
}

/// `Set-Cookie` establishing the session.
pub fn set_cookie(value: &str, secure: bool) -> String {
    let mut s = format!(
        "{}={value}; Path=/; HttpOnly; SameSite=Strict; Max-Age={}",
        cookie_name(),
        COOKIE_MAX_AGE.as_secs()
    );
    if secure {
        s.push_str("; Secure");
    }
    s
}

/// `Set-Cookie` clearing the session.
pub fn clear_cookie() -> String {
    format!(
        "{}=; Path=/; HttpOnly; SameSite=Strict; Max-Age=0",
        cookie_name()
    )
}

/// The 32 bytes a 64-digit hexadecimal string spells, or `None`.
fn unhex32(s: &str) -> Option<[u8; 32]> {
    if s.len() != 64 {
        return None;
    }
    let mut out = [0u8; 32];
    for (i, b) in out.iter_mut().enumerate() {
        *b = u8::from_str_radix(s.get(2 * i..2 * i + 2)?, 16).ok()?;
    }
    Some(out)
}

/// Compare two byte strings without leaking their contents through timing.
pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderValue;

    #[test]
    fn a_session_round_trips_and_carries_its_account() {
        let store = SessionStore::new();
        let cookie = store.open(Some("user-1".into()), false);
        let s = store.resolve(&cookie).expect("the session resolves");
        assert_eq!(s.user_id.as_deref(), Some("user-1"));
        assert!(!s.emergency);
    }

    #[test]
    fn a_forged_or_tampered_cookie_is_refused() {
        let store = SessionStore::new();
        let cookie = store.open(Some("user-1".into()), false);
        let (id, sig) = cookie.split_once('.').unwrap();

        assert!(store
            .resolve(&format!("{id}.{}", "0".repeat(sig.len())))
            .is_none());
        assert!(store.resolve(id).is_none(), "no signature at all");
        assert!(store.resolve(&format!("{id}x.{sig}")).is_none());
        assert!(store.resolve("").is_none());
    }

    #[test]
    fn a_cookie_from_another_process_does_not_resolve() {
        // A store with no file beside a bundle keeps nothing between runs.
        // Its secret is its own, so another store's cookie fails the
        // signature rather than reaching a lookup.
        let a = SessionStore::new();
        let cookie = a.open(Some("user-1".into()), false);
        let b = SessionStore::new();
        assert!(b.resolve(&cookie).is_none());
    }

    #[test]
    fn signing_out_ends_the_session() {
        let store = SessionStore::new();
        let cookie = store.open(Some("user-1".into()), false);
        store.close(&cookie);
        assert!(store.resolve(&cookie).is_none());
    }

    #[test]
    fn disabling_an_account_ends_every_session_it_holds() {
        let store = SessionStore::new();
        let one = store.open(Some("user-1".into()), false);
        let two = store.open(Some("user-1".into()), false);
        let other = store.open(Some("user-2".into()), false);
        store.close_all_for("user-1");
        assert!(store.resolve(&one).is_none());
        assert!(store.resolve(&two).is_none());
        assert!(
            store.resolve(&other).is_some(),
            "other accounts are untouched"
        );
    }

    #[test]
    fn failures_throttle_per_key_and_a_success_clears_them() {
        let store = SessionStore::new();
        assert!(!store.is_throttled("karin"));
        for _ in 0..MAX_ATTEMPTS {
            store.record_failure("karin");
        }
        assert!(store.is_throttled("karin"));
        assert!(!store.is_throttled("someone-else"), "buckets are per key");
        store.clear_failures("karin");
        assert!(!store.is_throttled("karin"));
    }

    #[test]
    fn an_account_is_counted_under_a_short_key_of_its_own() {
        let store = SessionStore::new();
        // A name that is a registration's bucket shuts the account of that
        // name, not the registrations of that address.
        let name = "register:203.0.113.7";
        for _ in 0..MAX_ACCOUNT_FAILURES {
            assert!(store.charge_account(name));
        }
        assert!(!store.charge_account(name), "the account is shut");
        assert!(!store.is_throttled(name), "the registrations are not");
        store.clear_account(name);
        assert!(store.charge_account(name), "its owner got in");
        // Two megabytes of name cost a key of fixed size.
        assert!(store.charge_account(&"x".repeat(2 << 20)));
        let map = store.attempts.read().unwrap();
        assert!(map.keys().all(|k| k.len() == "account:".len() + 64));
    }

    #[test]
    fn the_cookie_is_http_only_strict_and_secure_only_over_tls() {
        let plain = set_cookie("abc.def", false);
        assert!(plain.contains("HttpOnly"));
        assert!(plain.contains("SameSite=Strict"));
        assert!(
            !plain.contains("Secure"),
            "a Secure cookie is never stored over plain http, which is the \
             documented localhost deployment"
        );
        assert!(set_cookie("abc.def", true).contains("; Secure"));
        assert!(clear_cookie().contains("Max-Age=0"));
    }

    #[test]
    fn tls_is_read_from_the_proxy_header() {
        let mut h = HeaderMap::new();
        assert!(!is_tls(&h));
        h.insert("x-forwarded-proto", HeaderValue::from_static("https"));
        assert!(is_tls(&h));
        h.insert("x-forwarded-proto", HeaderValue::from_static("https, http"));
        assert!(is_tls(&h), "the first hop is the client's");
        h.insert("x-forwarded-proto", HeaderValue::from_static("http"));
        assert!(!is_tls(&h));
    }

    #[test]
    fn the_client_key_prefers_the_proxys_first_hop() {
        let mut h = HeaderMap::new();
        let proxy: std::net::SocketAddr = "127.0.0.1:5555".parse().unwrap();
        assert_eq!(client_key(&h, Some(proxy)), "127.0.0.1");
        assert_eq!(client_key(&h, None), "unknown");
        h.insert(
            "x-forwarded-for",
            HeaderValue::from_static("203.0.113.7, 10.0.0.1"),
        );
        assert_eq!(client_key(&h, Some(proxy)), "203.0.113.7");
        assert_eq!(client_key(&h, None), "203.0.113.7");
    }

    #[test]
    fn a_client_elsewhere_cannot_name_an_address() {
        // Bound wider than localhost with no proxy: the peer is the client,
        // and what it says of itself is not believed.
        let mut h = HeaderMap::new();
        h.insert("x-forwarded-for", HeaderValue::from_static("192.168.1.50"));
        let client: std::net::SocketAddr = "192.168.1.66:40000".parse().unwrap();
        assert_eq!(client_key(&h, Some(client)), "192.168.1.66");
        let mapped: std::net::SocketAddr = "[::ffff:127.0.0.1]:5555".parse().unwrap();
        assert_eq!(
            client_key(&h, Some(mapped)),
            "192.168.1.50",
            "this machine, however it is written, is the proxy"
        );
    }

    #[test]
    fn the_client_key_is_an_address_or_nothing() {
        let peer: std::net::SocketAddr = "127.0.0.1:5555".parse().unwrap();
        let key = |xff: &'static str| {
            let mut h = HeaderMap::new();
            h.insert("x-forwarded-for", HeaderValue::from_static(xff));
            client_key(&h, Some(peer))
        };
        assert_eq!(key("203.0.113.7:4711"), "203.0.113.7");
        assert_eq!(key("::ffff:203.0.113.7"), "203.0.113.7");
        assert_eq!(
            key("2001:db8:1:2:aaaa:bbbb:cccc:dddd"),
            "2001:db8:1:2::/64",
            "an IPv6 client with its whole /64"
        );
        assert_eq!(key("[2001:db8:1:2::9]:443"), "2001:db8:1:2::/64");
        assert_eq!(
            key("1.2.3.4 shut out for 900 s"),
            "127.0.0.1",
            "not an address: the peer's, and nothing written in the journal"
        );
        assert_eq!(key("unknown"), "127.0.0.1");
    }

    #[test]
    fn the_session_cookie_is_found_among_others() {
        let mut h = HeaderMap::new();
        h.insert(
            COOKIE,
            HeaderValue::from_static("theme=dark; axgf_session=abc.def; lang=pl"),
        );
        assert_eq!(cookie_value(&h).as_deref(), Some("abc.def"));
        assert_eq!(named_cookie(&h, "lang").as_deref(), Some("pl"));
        assert_eq!(named_cookie(&h, "theme").as_deref(), Some("dark"));
        assert_eq!(named_cookie(&h, "absent"), None);
    }

    #[test]
    fn a_session_survives_a_restart_and_so_does_a_sign_out() {
        let dir = crate::scratch::Dir::new("sessions-restart");
        let bundle = dir.join("family.axgf");
        let first = SessionStore::beside(&bundle);
        let kept = first.open(Some("user-1".into()), false);
        let closed = first.open(Some("user-2".into()), false);
        let emergency = first.open(None, true);
        first.close(&closed);
        drop(first);

        let second = SessionStore::beside(&bundle);
        assert_eq!(
            second.resolve(&kept).and_then(|s| s.user_id).as_deref(),
            Some("user-1")
        );
        assert!(second.resolve(&closed).is_none(), "signing out is kept");
        assert!(
            second.resolve(&emergency).is_none(),
            "the emergency token's session ends with the process"
        );

        // Revoked (a password changed, an account disabled): kept too.
        second.close_all_for("user-1");
        let third = SessionStore::beside(&bundle);
        assert!(third.resolve(&kept).is_none());
    }

    #[test]
    fn the_sessions_file_is_private_and_a_broken_one_only_signs_everyone_out() {
        use std::os::unix::fs::PermissionsExt;
        let dir = crate::scratch::Dir::new("sessions-file");
        let bundle = dir.join("family.axgf");
        let store = SessionStore::beside(&bundle);
        let cookie = store.open(Some("user-1".into()), false);
        let path = SessionStore::path_for(&bundle);
        assert_eq!(path.file_name().unwrap(), "family.sessions");
        let mode = std::fs::metadata(&path).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, 0o600);

        std::fs::write(&path, b"{ not json").unwrap();
        let fresh = SessionStore::beside(&bundle);
        assert!(fresh.resolve(&cookie).is_none());
        let again = fresh.open(Some("user-1".into()), false);
        assert!(fresh.resolve(&again).is_some(), "signing in again works");
    }

    #[test]
    fn a_session_in_use_moves_on_and_an_expired_one_is_not_brought_back() {
        let dir = crate::scratch::Dir::new("sessions-sliding");
        let bundle = dir.join("family.axgf");
        let store = SessionStore::beside(&bundle);
        let cookie = store.open(Some("user-1".into()), false);
        let id = cookie.split_once('.').unwrap().0.to_string();
        let two_hours = Duration::from_secs(2 * 60 * 60);

        // Last moved two hours ago: using it moves it on again.
        store
            .sessions
            .write()
            .unwrap()
            .get_mut(&id)
            .unwrap()
            .expires = SystemTime::now() + SESSION_TTL - two_hours;
        assert!(store.resolve(&cookie).is_some());
        let moved = store.sessions.read().unwrap()[&id].expires;
        assert!(moved + Duration::from_secs(60) > SystemTime::now() + SESSION_TTL);

        // Written as expired by an earlier run: not restored.
        let path = SessionStore::path_for(&bundle);
        let mut saved: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
        saved["sessions"][&id]["expires"] = 1.into();
        std::fs::write(&path, serde_json::to_vec(&saved).unwrap()).unwrap();
        assert!(SessionStore::beside(&bundle).resolve(&cookie).is_none());
    }

    #[test]
    fn each_installation_has_its_own_cookie_and_the_browser_keeps_it() {
        let a = instance_cookie_name(Path::new("/var/lib/axgf-cms/family.axgf"));
        let b = instance_cookie_name(Path::new("/home/cbrain/scratch/preview/family.axgf"));
        assert!(
            a.starts_with("axgf_session_") && a.len() == COOKIE_NAME.len() + 9,
            "{a}"
        );
        assert_ne!(a, b);
        assert_eq!(
            a,
            instance_cookie_name(Path::new("/var/lib/axgf-cms/family.axgf"))
        );
        assert!(
            set_cookie("abc.def", false).contains(&format!("Max-Age={}", COOKIE_MAX_AGE.as_secs()))
        );
    }
}
