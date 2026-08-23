//! Sessions and login throttling.
//!
//! # In memory, on purpose
//!
//! Sessions live in a `RwLock<HashMap>` and die with the process. A restart
//! signs everyone out, which at this scale — one family, a handful of accounts
//! — is a mild inconvenience, and the alternative is a second persistence
//! layer to back up, migrate and corrupt. The bundle is the database; adding a
//! session table to it would put transient state in the file people mail to
//! each other.
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

use std::collections::HashMap;
use std::sync::RwLock;
use std::time::{Duration, Instant};

use axum::http::header::{HeaderMap, COOKIE};
use hmac::{Hmac, Mac};
use sha2::Sha256;

/// Name of the session cookie.
pub const COOKIE_NAME: &str = "axgf_session";

/// How long a session lives without being used.
pub const SESSION_TTL: Duration = Duration::from_secs(12 * 60 * 60);

/// Failed logins allowed per username, and per client address, in a window.
pub const MAX_ATTEMPTS: u32 = 8;
/// The window failed logins are counted over.
pub const ATTEMPT_WINDOW: Duration = Duration::from_secs(15 * 60);

/// What a live session grants.
#[derive(Debug, Clone)]
pub struct Session {
    /// The account id, or `None` for a session opened with `--admin-token`.
    pub user_id: Option<String>,
    /// True when this session came from the emergency token.
    pub emergency: bool,
    expires: Instant,
}

/// Live sessions and the login throttle.
pub struct SessionStore {
    secret: [u8; 32],
    sessions: RwLock<HashMap<String, Session>>,
    attempts: RwLock<HashMap<String, Attempts>>,
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
            expires: Instant::now() + SESSION_TTL,
        };
        {
            let mut map = self.sessions.write().unwrap_or_else(|e| e.into_inner());
            // Opportunistic sweep: expired entries are dropped whenever
            // somebody signs in, so an abandoned process does not accumulate
            // them forever without a background task.
            let now = Instant::now();
            map.retain(|_, s| s.expires > now);
            map.insert(id.clone(), session);
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
        let map = self.sessions.read().unwrap_or_else(|e| e.into_inner());
        let s = map.get(id)?;
        if s.expires <= Instant::now() {
            return None;
        }
        Some(s.clone())
    }

    /// Close one session.
    pub fn close(&self, cookie: &str) {
        let Some((id, _)) = cookie.split_once('.') else {
            return;
        };
        let mut map = self.sessions.write().unwrap_or_else(|e| e.into_inner());
        map.remove(id);
    }

    /// Close every session belonging to an account.
    ///
    /// Used when an account is disabled, has its role lowered, or has its
    /// password changed: a live cookie that outlived the change would keep
    /// granting what was just taken away.
    pub fn close_all_for(&self, user_id: &str) {
        let mut map = self.sessions.write().unwrap_or_else(|e| e.into_inner());
        map.retain(|_, s| s.user_id.as_deref() != Some(user_id));
    }

    /// How many sessions are live. For the admin dashboard.
    pub fn live(&self) -> usize {
        let now = Instant::now();
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

    /// Whether `key` — a username or a client address — is out of attempts.
    pub fn is_throttled(&self, key: &str) -> bool {
        let map = self.attempts.read().unwrap_or_else(|e| e.into_inner());
        match map.get(key) {
            Some(a) if a.window_start.elapsed() < ATTEMPT_WINDOW => a.count >= MAX_ATTEMPTS,
            _ => false,
        }
    }

    /// Record a failed attempt against `key`.
    pub fn record_failure(&self, key: &str) {
        let mut map = self.attempts.write().unwrap_or_else(|e| e.into_inner());
        let now = Instant::now();
        map.retain(|_, a| a.window_start.elapsed() < ATTEMPT_WINDOW);
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

/// The session cookie's value, if the request carries one.
pub fn cookie_value(headers: &HeaderMap) -> Option<String> {
    for value in headers.get_all(COOKIE).iter() {
        let Ok(raw) = value.to_str() else { continue };
        for pair in raw.split(';') {
            if let Some(v) = pair.trim().strip_prefix(&format!("{COOKIE_NAME}=")) {
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

/// The client's address, for rate limiting.
///
/// `X-Forwarded-For`'s first entry when a proxy set one, else the peer address
/// axum recorded. Absent both, every anonymous attempt shares one bucket,
/// which throttles harder rather than less.
pub fn client_key(headers: &HeaderMap, peer: Option<std::net::SocketAddr>) -> String {
    if let Some(xff) = headers.get("x-forwarded-for").and_then(|v| v.to_str().ok()) {
        if let Some(first) = xff.split(',').next() {
            let first = first.trim();
            if !first.is_empty() {
                return first.to_string();
            }
        }
    }
    match peer {
        Some(a) => a.ip().to_string(),
        None => "unknown".into(),
    }
}

/// `Set-Cookie` establishing the session.
pub fn set_cookie(value: &str, secure: bool) -> String {
    let mut s = format!(
        "{COOKIE_NAME}={value}; Path=/; HttpOnly; SameSite=Strict; Max-Age={}",
        SESSION_TTL.as_secs()
    );
    if secure {
        s.push_str("; Secure");
    }
    s
}

/// `Set-Cookie` clearing the session.
pub fn clear_cookie() -> String {
    format!("{COOKIE_NAME}=; Path=/; HttpOnly; SameSite=Strict; Max-Age=0")
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
        // Restarting signs everyone out. The secret is per-process, so an old
        // cookie fails the signature rather than reaching a lookup.
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
        let peer: std::net::SocketAddr = "10.0.0.9:5555".parse().unwrap();
        assert_eq!(client_key(&h, Some(peer)), "10.0.0.9");
        assert_eq!(client_key(&h, None), "unknown");
        h.insert(
            "x-forwarded-for",
            HeaderValue::from_static("203.0.113.7, 10.0.0.1"),
        );
        assert_eq!(client_key(&h, Some(peer)), "203.0.113.7");
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
}
