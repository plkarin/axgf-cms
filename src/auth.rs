//! Resolving who is asking.
//!
//! The shared token is on its way out: it still grants admin, but it is now
//! the *emergency* path rather than the authentication system, and this module
//! is what turns a request's headers into an [`Viewer`](crate::access::Viewer)
//! that the read paths can measure entities against. Accounts and sessions
//! replace it in [`crate::session`].

use axum::http::header::HeaderMap;

use crate::access::Viewer;

/// Who is asking, from the request's cookies.
///
/// Resolved once at the top of a handler and passed down; nothing below this
/// looks at a header again.
///
/// The session cookie is tried first and the emergency token second, so an
/// operator who has signed in normally is not silently upgraded to admin by a
/// stale `--admin-token` cookie left in the same browser.
///
/// An account that has been disabled, deleted or had its role changed since
/// the cookie was issued is refused here and its session closed, so the
/// change takes effect on the next request rather than whenever the cookie
/// happens to expire. The account is re-read from the ACL on every request
/// for exactly that reason: caching the `User` inside the session would mean
/// caching the role, and a revoked admin would keep their rights until they
/// chose to sign out.
pub fn viewer(state: &crate::state::AppState, headers: &HeaderMap) -> Viewer {
    if let Some(cookie) = crate::session::cookie_value(headers) {
        if let Some(session) = state.sessions().resolve(&cookie) {
            if session.emergency {
                return Viewer::emergency_admin();
            }
            if let Some(id) = session.user_id.as_deref() {
                match state.acl_read(|acl| acl.by_id(id).cloned()) {
                    Some(user) if user.status == crate::acl::Status::Active => {
                        return Viewer {
                            user: Some(user),
                            emergency: false,
                        };
                    }
                    // Disabled or gone. Close it rather than merely refusing
                    // it, so the cookie stops costing a lookup per request.
                    _ => state.sessions().close(&cookie),
                }
            }
        }
    }

    // The emergency token is **not** consulted here. It is a way in, through
    // `POST /admin/login`, where it is throttled like any other credential,
    // logged loudly, and exchanged for an ordinary session that a logout can
    // close. Accepting it as a cookie on every request — which is what this
    // used to do — meant a credential with no attempt limit, no log line and
    // no server-side revocation: a token an operator had chosen by hand could
    // be ground through at the speed of the network, silently. Nothing set
    // that cookie any more; this is the other half of removing it.
    Viewer::anonymous()
}

/// Name of the cookie that used to carry the emergency admin token.
///
/// Nothing issues it and nothing accepts it. The name survives for one
/// purpose: an installation upgraded while somebody's browser still holds the
/// old cookie should have it cleared on sign-out rather than left to sit there
/// for the seven days its `Max-Age` promised.
pub const LEGACY_COOKIE_NAME: &str = "axgf_admin";

/// `Set-Cookie` value that clears the legacy admin cookie.
pub fn clear_cookie() -> String {
    format!("{LEGACY_COOKIE_NAME}=; Path=/; HttpOnly; SameSite=Strict; Max-Age=0")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_legacy_cookie_is_only_ever_cleared() {
        // The value that used to be a credential. Nothing in this module can
        // issue it any more, and clearing it is expressed as an expiry rather
        // than as a value somebody might mistake for a way back in.
        let c = clear_cookie();
        assert!(c.starts_with("axgf_admin="), "{c}");
        assert!(c.contains("Max-Age=0"), "{c}");
        assert!(c.contains("HttpOnly"), "{c}");
    }
}
