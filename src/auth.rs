//! Resolving who is asking.
//!
//! The shared token is on its way out: it still grants admin, but it is now
//! the *emergency* path rather than the authentication system, and this module
//! is what turns a request's headers into an [`Viewer`](crate::access::Viewer)
//! that the read paths can measure entities against. Accounts and sessions
//! replace it in [`crate::session`].

use axum::http::header::{HeaderMap, COOKIE};

use crate::access::Viewer;

/// Who is asking, from the request's cookies.
///
/// Resolved once at the top of a handler and passed down; nothing below this
/// looks at headers again. Today the only credential is the shared token,
/// which grants admin for the request; every other request is anonymous and
/// reads at the `public` ceiling.
pub fn viewer(headers: &HeaderMap, admin_token: &str) -> Viewer {
    if is_admin(headers, admin_token) {
        Viewer::emergency_admin()
    } else {
        Viewer::anonymous()
    }
}

/// Name of the cookie carrying the admin token.
pub const COOKIE_NAME: &str = "axgf_admin";

/// Whether the request carries a valid admin token.
///
/// Comparison is constant-time over the token bytes so a caller cannot recover
/// the token by timing repeated guesses.
pub fn is_admin(headers: &HeaderMap, expected: &str) -> bool {
    // A blank expected token would otherwise match a blank cookie and grant
    // everyone admin. Config refuses blank tokens; this is the second lock.
    if expected.is_empty() {
        return false;
    }
    match token_from_cookies(headers) {
        Some(got) => constant_time_eq(got.as_bytes(), expected.as_bytes()),
        None => false,
    }
}

/// Extract the admin cookie value, if present.
fn token_from_cookies(headers: &HeaderMap) -> Option<String> {
    for value in headers.get_all(COOKIE).iter() {
        let Ok(raw) = value.to_str() else { continue };
        for pair in raw.split(';') {
            let pair = pair.trim();
            if let Some(v) = pair.strip_prefix(concat!("axgf_admin", "=")) {
                return Some(v.to_string());
            }
        }
    }
    None
}

/// `Set-Cookie` value that establishes the session.
///
/// `HttpOnly` keeps it away from scripts and `SameSite=Lax` blocks
/// cross-site form posts. `Secure` is intentionally **not** set: V1's
/// documented deployment is plain HTTP on localhost, and a `Secure` cookie
/// would simply never be stored there. Behind the TLS reverse proxy the README
/// recommends, the proxy is the component that should add it.
pub fn set_cookie(token: &str) -> String {
    format!("{COOKIE_NAME}={token}; Path=/; HttpOnly; SameSite=Lax; Max-Age=604800")
}

/// `Set-Cookie` value that clears the session.
pub fn clear_cookie() -> String {
    format!("{COOKIE_NAME}=; Path=/; HttpOnly; SameSite=Lax; Max-Age=0")
}

/// Compare two byte strings without leaking their contents through timing.
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
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

    fn headers(cookie: &str) -> HeaderMap {
        let mut h = HeaderMap::new();
        h.insert(COOKIE, HeaderValue::from_str(cookie).unwrap());
        h
    }

    #[test]
    fn matching_cookie_authenticates() {
        assert!(is_admin(&headers("axgf_admin=secret"), "secret"));
    }

    #[test]
    fn wrong_or_missing_cookie_does_not() {
        assert!(!is_admin(&headers("axgf_admin=nope"), "secret"));
        assert!(!is_admin(&headers("other=secret"), "secret"));
        assert!(!is_admin(&HeaderMap::new(), "secret"));
    }

    #[test]
    fn token_is_found_among_other_cookies() {
        assert!(is_admin(
            &headers("theme=dark; axgf_admin=secret; lang=en"),
            "secret"
        ));
    }

    #[test]
    fn an_empty_expected_token_does_not_authenticate_everyone() {
        // Defence in depth: config already refuses blank tokens, but a blank
        // token reaching this far must not turn into "no password required".
        assert!(!is_admin(&headers("axgf_admin="), ""));
    }

    #[test]
    fn constant_time_eq_matches_normal_equality() {
        assert!(constant_time_eq(b"abc", b"abc"));
        assert!(!constant_time_eq(b"abc", b"abd"));
        assert!(!constant_time_eq(b"abc", b"ab"));
    }

    #[test]
    fn cookies_are_http_only_and_same_site() {
        let c = set_cookie("t");
        assert!(c.contains("HttpOnly"));
        assert!(c.contains("SameSite=Lax"));
        assert!(clear_cookie().contains("Max-Age=0"));
    }
}
