//! Access tokens (docs/adr/0045 §3): what lets an AI assistant elsewhere —
//! ChatGPT, Claude, any MCP client — act on the archive as one account,
//! through the MCP.
//!
//! A token is `axgf_<id>_<secret>`: its id finds it, its secret proves it.
//! Only the SHA-256 of the whole token is kept, in the accounts file beside
//! the password hashes (mode 600); the token itself is shown once, when it is
//! made, and never again. It acts with its account's role and branch and
//! never more, and one made for reading cannot write, whatever its account
//! may. It stops working when it is revoked, when its account is disabled,
//! and while an administrator keeps the MCP closed.

use base64::Engine as _;
use rand::RngCore as _;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::acl::{Acl, Status, User};

/// What every token begins with: what a secret scanner looks for.
pub const PREFIX: &str = "axgf_";

/// The most tokens an account may hold.
pub const MOST: usize = 10;

/// The longest a token's name may be, in characters.
pub const LABEL_MAX: usize = 40;

/// One token, as the accounts file keeps it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApiToken {
    /// Eight hexadecimal digits, the token's second part: what finds it and
    /// what revokes it. Not a secret.
    pub id: String,
    /// What the account called it: "ChatGPT", "Claude on the laptop".
    pub label: String,
    /// The SHA-256 of the whole token, in hexadecimal.
    pub sha256: String,
    /// Whether it may write. A token that may not reads and nothing else.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub write: bool,
    pub created_at: String,
    /// The day it was last used, `YYYY-MM-DD`: a day and not an instant, so
    /// that using a token writes the accounts file once a day at most.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_used: Option<String>,
}

/// The SHA-256 of a token, in hexadecimal.
pub fn digest(token: &str) -> String {
    Sha256::digest(token.as_bytes())
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

/// A token's name, cleaned: no control characters, spaces collapsed, at most
/// [`LABEL_MAX`] characters, `MCP` if nothing is left.
pub fn clean_label(label: &str) -> String {
    let cleaned: String = label
        .chars()
        .map(|c| if c.is_control() { ' ' } else { c })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .chars()
        .take(LABEL_MAX)
        .collect();
    let cleaned = cleaned.trim().to_string();
    if cleaned.is_empty() {
        "MCP".into()
    } else {
        cleaned
    }
}

/// A new token: what the accounts file keeps, and the token itself, to be
/// shown once.
pub fn mint(label: &str, write: bool, now: &str) -> (ApiToken, String) {
    let mut rng = rand::thread_rng();
    let mut id = [0u8; 4];
    rng.fill_bytes(&mut id);
    let mut secret = [0u8; 32];
    rng.fill_bytes(&mut secret);
    let id: String = id.iter().map(|b| format!("{b:02x}")).collect();
    let token = format!(
        "{PREFIX}{id}_{}",
        base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(secret)
    );
    (
        ApiToken {
            id,
            label: clean_label(label),
            sha256: digest(&token),
            write,
            created_at: now.to_string(),
            last_used: None,
        },
        token,
    )
}

/// The id a token names, if it has a token's shape.
pub fn id_of(token: &str) -> Option<&str> {
    let rest = token.trim().strip_prefix(PREFIX)?;
    let (id, secret) = rest.split_once('_')?;
    (id.len() == 8 && id.bytes().all(|b| b.is_ascii_hexdigit()) && secret.len() >= 32).then_some(id)
}

/// Two digests compared in a time that does not depend on where they differ.
fn same(a: &str, b: &str) -> bool {
    a.len() == b.len()
        && a.bytes()
            .zip(b.bytes())
            .fold(0u8, |acc, (x, y)| acc | (x ^ y))
            == 0
}

/// The active account a token belongs to, and the token as it is kept.
pub fn find<'a>(acl: &'a Acl, token: &str) -> Option<(&'a User, &'a ApiToken)> {
    let id = id_of(token)?;
    let wanted = digest(token.trim());
    acl.users
        .iter()
        .filter(|u| u.status == Status::Active)
        .find_map(|u| {
            u.api_tokens
                .iter()
                .find(|t| t.id == id && same(&t.sha256, &wanted))
                .map(|t| (u, t))
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_token_is_found_by_its_id_and_proved_by_its_secret() {
        let (kept, token) = mint(
            "  Chat\u{7}GPT   on   the laptop ",
            true,
            "2026-11-09T00:00:00Z",
        );
        assert!(token.starts_with("axgf_"));
        assert_eq!(id_of(&token), Some(kept.id.as_str()));
        assert_eq!(kept.label, "Chat GPT on the laptop");
        assert_ne!(kept.sha256, token, "the token itself is never kept");
        let mut acl = Acl::default();
        let mut user: User = serde_json::from_value(serde_json::json!({
            "id": "u1", "username": "karin", "password_hash": "", "role": "contributor",
            "status": "active", "created_at": "2026-11-09T00:00:00Z"
        }))
        .unwrap();
        user.api_tokens.push(kept.clone());
        acl.users.push(user);
        assert_eq!(
            find(&acl, &token).map(|(u, _)| u.username.as_str()),
            Some("karin")
        );
        // The right id with another secret is nobody.
        let forged = format!("axgf_{}_{}", kept.id, "A".repeat(43));
        assert!(find(&acl, &forged).is_none());
        // A disabled account's tokens stop working.
        acl.users[0].status = Status::Disabled;
        assert!(find(&acl, &token).is_none());
    }

    #[test]
    fn a_name_is_never_empty_nor_long() {
        assert_eq!(clean_label("   "), "MCP");
        assert_eq!(clean_label(&"x".repeat(90)).chars().count(), LABEL_MAX);
        assert!(id_of("axgf_zz_secret").is_none());
        assert!(id_of("bearer something").is_none());
    }
}
