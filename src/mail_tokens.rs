//! Links that hold and need nothing stored (docs/adr/0048 §4).
//!
//! A token is an account id, a purpose and an expiry, with two HMAC-SHA256
//! under a key of the site's own (`family.mailkey`, mode 600 beside the
//! bundle, never in a backup):
//!
//! - one over those three, which says the token was made here — a token that
//!   fails it is a forgery, and counts in the jail when posted;
//! - one over them and digests of the account as it was — its password hash,
//!   role, status and address, a count of the changes that end its links
//!   for good, and for a welcome its last sign-in — which says nothing has
//!   changed since. Setting a password by any way, a new address, a new role,
//!   a disabled account, or for a welcome a first sign-in, each end every
//!   link the account had, and undoing the change does not bring them back:
//!   nothing is stored but that count, and asking for a link writes nothing.

use std::path::Path;

use base64::Engine as _;
use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};

use crate::acl::{Acl, Status, User};

type HmacSha256 = Hmac<Sha256>;

/// What a link is for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Purpose {
    /// A forgotten password: an hour.
    Reset,
    /// An account an administrator made: choose its first password. Seven
    /// days, one for an administrator's account, and only until the first
    /// sign-in.
    Welcome,
}

impl Purpose {
    fn tag(self) -> &'static str {
        match self {
            Purpose::Reset => "reset",
            Purpose::Welcome => "welcome",
        }
    }

    fn parse(s: &str) -> Option<Self> {
        match s {
            "reset" => Some(Purpose::Reset),
            "welcome" => Some(Purpose::Welcome),
            _ => None,
        }
    }

    /// How long a link of this purpose holds, for `user`.
    pub fn lifetime(self, user: &User) -> time::Duration {
        match self {
            Purpose::Reset => time::Duration::hours(1),
            Purpose::Welcome if user.role.is_admin() => time::Duration::days(1),
            Purpose::Welcome => time::Duration::days(7),
        }
    }
}

/// The key links are signed with.
pub struct Key([u8; 32]);

impl Key {
    /// The key kept beside `bundle` (`family.mailkey`), made on first use. A
    /// key that cannot be kept still works until the next restart, after
    /// which its links stop: the failure is logged.
    pub fn beside(bundle: &Path) -> Key {
        let path = bundle.with_extension("mailkey");
        if let Some(key) = std::fs::read_to_string(&path)
            .ok()
            .and_then(|s| hex32(s.trim()))
        {
            return Key(key);
        }
        let mut key = [0u8; 32];
        use rand::RngCore as _;
        rand::rngs::OsRng.fill_bytes(&mut key);
        let hex: String = key.iter().map(|b| format!("{b:02x}")).collect();
        if let Err(e) = crate::brain::write_private(&path, hex.as_bytes()) {
            tracing::warn!(error = %e, "could not keep the key mail links are signed with");
        }
        Key(key)
    }

    /// A key for tests.
    pub fn fixed(bytes: [u8; 32]) -> Key {
        Key(bytes)
    }

    fn mac(&self, domain: &[u8], parts: &[&[u8]]) -> HmacSha256 {
        let mut m = HmacSha256::new_from_slice(&self.0).expect("any key length");
        m.update(domain);
        for p in parts {
            m.update(&(p.len() as u64).to_be_bytes());
            m.update(p);
        }
        m
    }
}

fn hex32(s: &str) -> Option<[u8; 32]> {
    if s.len() != 64 {
        return None;
    }
    let mut out = [0u8; 32];
    for (i, b) in out.iter_mut().enumerate() {
        *b = u8::from_str_radix(s.get(2 * i..2 * i + 2)?, 16).ok()?;
    }
    Some(out)
}

/// The account as a link was made for it.
fn state(user: &User, purpose: Purpose) -> [u8; 32] {
    let mut h = Sha256::new();
    for part in [
        user.password_hash.as_str(),
        user.role.as_str(),
        user.status.as_str(),
        &user.email.as_deref().unwrap_or("").trim().to_lowercase(),
    ] {
        h.update((part.len() as u64).to_be_bytes());
        h.update(part.as_bytes());
    }
    h.update(user.link_epoch.to_be_bytes());
    if purpose == Purpose::Welcome {
        let seen = user.last_login.as_deref().unwrap_or("");
        h.update((seen.len() as u64).to_be_bytes());
        h.update(seen.as_bytes());
    }
    h.finalize().into()
}

const B64: base64::engine::GeneralPurpose = base64::engine::general_purpose::URL_SAFE_NO_PAD;

/// Half a SHA-256: 128 bits, ample against forgery.
const TAG: usize = 16;

/// A link for `user`, good for `purpose` from `now`.
pub fn mint(key: &Key, user: &User, purpose: Purpose, now: time::OffsetDateTime) -> String {
    let expires = (now + purpose.lifetime(user)).unix_timestamp();
    let payload = format!("{}.{}.{}", user.id, purpose.tag(), expires);
    let made = key
        .mac(b"axgf-mail-made", &[payload.as_bytes()])
        .finalize()
        .into_bytes();
    let held = key
        .mac(
            b"axgf-mail-held",
            &[payload.as_bytes(), &state(user, purpose)],
        )
        .finalize()
        .into_bytes();
    format!(
        "{}.{}.{}",
        B64.encode(payload.as_bytes()),
        B64.encode(&made[..TAG]),
        B64.encode(&held[..TAG])
    )
}

/// What a link turned out to be.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verdict {
    /// Good for `purpose`, for the account `id`.
    Good { id: String, purpose: Purpose },
    /// Never made here: a forgery or a mangled copy.
    Forged,
    /// Made here and good once; the account has changed since, or is gone.
    Stale,
    /// Made here, and past its time.
    Expired,
}

/// What `token` is, against the accounts as they are at `now`.
pub fn verify(key: &Key, token: &str, acl: &Acl, now: time::OffsetDateTime) -> Verdict {
    let mut parts = token.trim().split('.');
    let (Some(p), Some(m), Some(h), None) =
        (parts.next(), parts.next(), parts.next(), parts.next())
    else {
        return Verdict::Forged;
    };
    let (Ok(payload), Ok(made), Ok(held)) = (B64.decode(p), B64.decode(m), B64.decode(h)) else {
        return Verdict::Forged;
    };
    if key
        .mac(b"axgf-mail-made", &[&payload])
        .verify_truncated_left(&made)
        .is_err()
        || made.len() != TAG
    {
        return Verdict::Forged;
    }
    let Ok(payload) = String::from_utf8(payload) else {
        return Verdict::Forged;
    };
    let mut fields = payload.split('.');
    let (Some(id), Some(purpose), Some(expires)) = (fields.next(), fields.next(), fields.next())
    else {
        return Verdict::Forged;
    };
    let (Some(purpose), Ok(expires)) = (Purpose::parse(purpose), expires.parse::<i64>()) else {
        return Verdict::Forged;
    };
    if now.unix_timestamp() > expires {
        return Verdict::Expired;
    }
    let Some(user) = acl.by_id(id).filter(|u| u.status == Status::Active) else {
        return Verdict::Stale;
    };
    let fresh = key
        .mac(
            b"axgf-mail-held",
            &[payload.as_bytes(), &state(user, purpose)],
        )
        .verify_truncated_left(&held)
        .is_ok()
        && held.len() == TAG;
    if !fresh {
        return Verdict::Stale;
    }
    Verdict::Good {
        id: id.to_string(),
        purpose,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::acl::{new_user, Role};

    fn key() -> Key {
        Key::fixed([7u8; 32])
    }

    fn acl_with(u: &User) -> Acl {
        let mut acl = Acl::default();
        acl.users.push(u.clone());
        acl
    }

    #[test]
    fn a_link_is_good_for_its_purpose_until_its_time() {
        let mut u = new_user("ada", "correct horse battery staple", Role::Viewer).unwrap();
        u.email = Some("ada@example.org".into());
        let now = time::OffsetDateTime::now_utc();
        let t = mint(&key(), &u, Purpose::Reset, now);
        let acl = acl_with(&u);
        assert_eq!(
            verify(&key(), &t, &acl, now),
            Verdict::Good {
                id: u.id.clone(),
                purpose: Purpose::Reset
            }
        );
        assert_eq!(
            verify(&key(), &t, &acl, now + time::Duration::minutes(61)),
            Verdict::Expired
        );
    }

    #[test]
    fn any_change_to_the_account_ends_its_links() {
        let mut u = new_user("ada", "correct horse battery staple", Role::Viewer).unwrap();
        u.email = Some("ada@example.org".into());
        let now = time::OffsetDateTime::now_utc();
        let t = mint(&key(), &u, Purpose::Reset, now);
        for change in [
            |u: &mut User| {
                u.password_hash = crate::acl::hash_password("another one entirely").unwrap()
            },
            |u: &mut User| u.email = Some("someone@example.org".into()),
            |u: &mut User| u.role = Role::Contributor,
            |u: &mut User| u.status = Status::Disabled,
        ] {
            let mut changed = u.clone();
            change(&mut changed);
            assert_eq!(verify(&key(), &t, &acl_with(&changed), now), Verdict::Stale);
        }
        // A welcome also ends at the first sign-in; a reset does not care.
        let w = mint(&key(), &u, Purpose::Welcome, now);
        let mut signed_in = u.clone();
        signed_in.last_login = Some("2026-11-11T10:00:00Z".into());
        assert_eq!(
            verify(&key(), &w, &acl_with(&signed_in), now),
            Verdict::Stale
        );
        assert!(matches!(
            verify(&key(), &t, &acl_with(&signed_in), now),
            Verdict::Good { .. }
        ));
    }

    #[test]
    fn a_link_made_elsewhere_or_mangled_is_a_forgery() {
        let u = new_user("ada", "correct horse battery staple", Role::Viewer).unwrap();
        let now = time::OffsetDateTime::now_utc();
        let t = mint(&key(), &u, Purpose::Reset, now);
        let acl = acl_with(&u);
        assert_eq!(
            verify(&Key::fixed([8u8; 32]), &t, &acl, now),
            Verdict::Forged
        );
        let mut chars: Vec<char> = t.chars().collect();
        chars[3] = if chars[3] == 'A' { 'B' } else { 'A' };
        let mangled: String = chars.into_iter().collect();
        assert_eq!(verify(&key(), &mangled, &acl, now), Verdict::Forged);
        assert_eq!(verify(&key(), "nonsense", &acl, now), Verdict::Forged);
        assert_eq!(verify(&key(), "", &acl, now), Verdict::Forged);
    }

    #[test]
    fn an_administrators_welcome_holds_a_day_and_a_members_seven() {
        let admin = new_user("root", "correct horse battery staple", Role::Admin).unwrap();
        let viewer = new_user("vera", "correct horse battery staple", Role::Viewer).unwrap();
        assert_eq!(Purpose::Welcome.lifetime(&admin), time::Duration::days(1));
        assert_eq!(Purpose::Welcome.lifetime(&viewer), time::Duration::days(7));
        assert_eq!(Purpose::Reset.lifetime(&admin), time::Duration::hours(1));
    }

    #[test]
    fn the_key_is_kept_and_read_back() {
        let dir = crate::scratch::Dir::new("mailkey");
        let bundle = dir.join("family.axgf");
        let u = new_user("ada", "correct horse battery staple", Role::Viewer).unwrap();
        let now = time::OffsetDateTime::now_utc();
        let t = mint(&Key::beside(&bundle), &u, Purpose::Reset, now);
        assert!(matches!(
            verify(&Key::beside(&bundle), &t, &acl_with(&u), now),
            Verdict::Good { .. }
        ));
    }
}
