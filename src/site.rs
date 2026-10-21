//! The rules of entry: who may open an account, and how anyone signs in
//! (v0.5.0, docs/adr/0012).
//!
//! # Where they are kept
//!
//! In the accounts file, `family.acl`, under `site`. They are the same kind of
//! thing as the accounts — who gets in — and they have the same distribution
//! rule: an identity provider's client secret and the family's registration
//! secret must never travel with the bundle that is mailed to a cousin, and
//! they must travel with every backup, because a restored installation that
//! has its accounts and has forgotten how they sign in is half restored. The
//! `.acl` is already mode 600, already written atomically under the write lock,
//! and already in every archive.
//!
//! An older build reading the file ignores this block, and drops it on its
//! next write of the accounts. Downgrading therefore forgets the providers'
//! settings and the registration policy, which closes registration again —
//! the safe direction — and leaves every account able to sign in with its
//! password.
//!
//! # Secrets
//!
//! The client secrets and the registration secret are stored as typed. They
//! have to be: a client secret is presented to the provider on every sign-in,
//! and the registration secret is something an administrator reads out to a
//! cousin. They never reach a page: a client secret is shown as set or unset,
//! and a form that leaves its field blank keeps the stored one.

use serde::{Deserialize, Serialize};

/// Who may open an account without an administrator (docs/adr/0012 §1).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RegistrationMode {
    /// Only an administrator creates accounts. The default, and what every
    /// installation did before 0.5.0.
    #[default]
    Closed,
    /// A visitor may open a member account if they know the family's secret.
    Secret,
    /// Any visitor may open a member account.
    Open,
}

impl RegistrationMode {
    pub const ALL: [RegistrationMode; 3] = [
        RegistrationMode::Closed,
        RegistrationMode::Secret,
        RegistrationMode::Open,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            RegistrationMode::Closed => "closed",
            RegistrationMode::Secret => "secret",
            RegistrationMode::Open => "open",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|m| m.as_str() == s.trim())
    }
}

/// Who may sign in with a username and a password (docs/adr/0012 §3).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PasswordPolicy {
    /// Every account.
    #[default]
    Everyone,
    /// Every account but the administrators', who sign in through a provider.
    NonAdmins,
    /// Nobody: every sign-in goes through a provider. The emergency token is
    /// still accepted, because it is how an installation is rescued when the
    /// provider is the thing that broke.
    Nobody,
}

impl PasswordPolicy {
    pub const ALL: [PasswordPolicy; 3] = [
        PasswordPolicy::Everyone,
        PasswordPolicy::NonAdmins,
        PasswordPolicy::Nobody,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            PasswordPolicy::Everyone => "everyone",
            PasswordPolicy::NonAdmins => "non-admins",
            PasswordPolicy::Nobody => "nobody",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|m| m.as_str() == s.trim())
    }

    /// May an account of this role sign in with its password?
    pub fn allows(self, role: crate::acl::Role) -> bool {
        match self {
            PasswordPolicy::Everyone => true,
            PasswordPolicy::NonAdmins => !role.is_admin(),
            PasswordPolicy::Nobody => false,
        }
    }
}

/// The registration policy and, in secret mode, the secret.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Registration {
    #[serde(default)]
    pub mode: RegistrationMode,
    /// The family's secret, as the administrator set it. Only read in
    /// [`RegistrationMode::Secret`].
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub secret: String,
    /// When and by whom the policy last changed, for the accounts page.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub changed_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub changed_by: Option<String>,
}

impl Registration {
    /// Does `offered` open the door in secret mode? Compared in constant time,
    /// and an unset secret opens nothing.
    pub fn secret_matches(&self, offered: &str) -> bool {
        let offered = offered.trim();
        !self.secret.is_empty()
            && !offered.is_empty()
            && crate::session::constant_time_eq(offered.as_bytes(), self.secret.as_bytes())
    }

    /// Whether a visitor may open an account at all, secret or not.
    pub fn is_open_to_visitors(&self) -> bool {
        match self.mode {
            RegistrationMode::Closed => false,
            RegistrationMode::Secret => !self.secret.is_empty(),
            RegistrationMode::Open => true,
        }
    }
}

/// How signing in works.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignIn {
    #[serde(default)]
    pub password: PasswordPolicy,
    /// Sign a provider's user into the existing account holding the same
    /// address, when the provider says it verified that address. How an
    /// administrator gives a relative a Google sign-in without a registration
    /// door: create the account with their address, and their first sign-in
    /// links it.
    #[serde(default = "yes")]
    pub link_by_email: bool,
    /// Domains a provider's user may open an account from, when the policy
    /// lets visitors open accounts. Empty means any.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_domains: Vec<String>,
}

fn yes() -> bool {
    true
}

impl Default for SignIn {
    fn default() -> Self {
        Self {
            password: PasswordPolicy::Everyone,
            link_by_email: true,
            allowed_domains: Vec::new(),
        }
    }
}

impl SignIn {
    /// Is `email` from a domain the policy accepts for a new account?
    pub fn domain_allowed(&self, email: &str) -> bool {
        if self.allowed_domains.is_empty() {
            return true;
        }
        let Some((_, domain)) = email.trim().rsplit_once('@') else {
            return false;
        };
        let domain = domain.to_ascii_lowercase();
        self.allowed_domains.iter().any(|d| {
            d.trim()
                .trim_start_matches('@')
                .eq_ignore_ascii_case(&domain)
        })
    }
}

/// One identity provider's settings, as the operator registered the
/// application with it.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Provider {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub client_id: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub client_secret: String,
    /// Microsoft: the directory (`common`, `organizations`, `consumers` or a
    /// tenant id). Unused elsewhere.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub tenant: String,
    /// A generic OpenID Connect provider: its issuer, from which everything
    /// else is discovered.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub issuer: String,
    /// A generic provider: the name its button carries.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub label: String,
}

impl Provider {
    /// Enabled, and carrying what a sign-in needs.
    pub fn is_ready(&self, kind: crate::sso::Kind) -> bool {
        self.enabled
            && !self.client_id.trim().is_empty()
            && !self.client_secret.trim().is_empty()
            && (kind != crate::sso::Kind::Oidc || !self.issuer.trim().is_empty())
    }
}

/// Every provider this build knows how to speak to.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Providers {
    #[serde(default)]
    pub google: Provider,
    #[serde(default)]
    pub microsoft: Provider,
    #[serde(default)]
    pub github: Provider,
    #[serde(default)]
    pub oidc: Provider,
}

impl Providers {
    pub fn get(&self, kind: crate::sso::Kind) -> &Provider {
        match kind {
            crate::sso::Kind::Google => &self.google,
            crate::sso::Kind::Microsoft => &self.microsoft,
            crate::sso::Kind::Github => &self.github,
            crate::sso::Kind::Oidc => &self.oidc,
        }
    }

    pub fn get_mut(&mut self, kind: crate::sso::Kind) -> &mut Provider {
        match kind {
            crate::sso::Kind::Google => &mut self.google,
            crate::sso::Kind::Microsoft => &mut self.microsoft,
            crate::sso::Kind::Github => &mut self.github,
            crate::sso::Kind::Oidc => &mut self.oidc,
        }
    }

    /// The providers a sign-in page offers, in a fixed order.
    pub fn ready(&self) -> Vec<crate::sso::Kind> {
        crate::sso::Kind::ALL
            .into_iter()
            .filter(|k| self.get(*k).is_ready(*k))
            .collect()
    }
}

/// The site's rules of entry, as stored under `site` in the `.acl`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Site {
    #[serde(default)]
    pub registration: Registration,
    #[serde(default)]
    pub sign_in: SignIn,
    #[serde(default)]
    pub providers: Providers,
    /// The address readers reach the site at, `https://family.example`. What
    /// an identity provider sends a reader back to, so it has to be the public
    /// one, not the address this process listens on.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub public_url: String,
    /// The public key an encrypted export was last made for, kept so the next
    /// one does not need it pasted again. A public key: nothing secret.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub export_recipient: String,
}

impl Site {
    /// The public address without a trailing slash, if one is set and is a
    /// plausible absolute http(s) URL.
    pub fn public_base(&self) -> Option<String> {
        normalise_base(&self.public_url)
    }

    /// Whether the public address is served over TLS — what an identity
    /// provider insists on, and what makes the session cookie `Secure`.
    pub fn public_is_https(&self) -> bool {
        self.public_base()
            .is_some_and(|b| b.starts_with("https://"))
    }
}

/// `https://Family.example/` → `https://family.example`; anything that is not
/// an absolute http(s) address with a host → `None`.
pub fn normalise_base(raw: &str) -> Option<String> {
    let raw = raw.trim().trim_end_matches('/');
    let (scheme, rest) = raw.split_once("://")?;
    let scheme = scheme.to_ascii_lowercase();
    if scheme != "https" && scheme != "http" {
        return None;
    }
    let (host, path) = match rest.split_once('/') {
        Some((h, p)) => (h, format!("/{p}")),
        None => (rest, String::new()),
    };
    if host.is_empty()
        || host.contains(['@', ' ', '?', '#', '\\'])
        || path.contains(['?', '#', ' ', '\\'])
    {
        return None;
    }
    Some(format!("{scheme}://{}{path}", host.to_ascii_lowercase()))
}

/// A registration secret nobody has to invent: five groups of four from an
/// unambiguous alphabet, about 100 bits, readable aloud over the telephone.
pub fn generate_secret() -> String {
    const ALPHABET: &[u8] = b"abcdefghijkmnpqrstuvwxyz23456789";
    let mut out = String::with_capacity(24);
    let bytes: Vec<u8> = (0..2)
        .flat_map(|_| uuid::Uuid::new_v4().into_bytes())
        .collect();
    for (i, b) in bytes.iter().take(20).enumerate() {
        if i > 0 && i % 4 == 0 {
            out.push('-');
        }
        out.push(ALPHABET[*b as usize % ALPHABET.len()] as char);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_closed_door_is_the_default_and_an_empty_secret_opens_nothing() {
        let site = Site::default();
        assert_eq!(site.registration.mode, RegistrationMode::Closed);
        assert!(!site.registration.is_open_to_visitors());
        let mut r = Registration {
            mode: RegistrationMode::Secret,
            ..Default::default()
        };
        assert!(!r.is_open_to_visitors(), "secret mode with no secret");
        assert!(!r.secret_matches(""));
        r.secret = "abcd-efgh".into();
        assert!(r.is_open_to_visitors());
        assert!(r.secret_matches(" abcd-efgh "));
        assert!(!r.secret_matches("abcd-efgi"));
    }

    #[test]
    fn the_password_policy_says_who_signs_in_with_a_password() {
        use crate::acl::Role;
        assert!(PasswordPolicy::Everyone.allows(Role::Admin));
        assert!(!PasswordPolicy::NonAdmins.allows(Role::Admin));
        assert!(PasswordPolicy::NonAdmins.allows(Role::Viewer));
        assert!(!PasswordPolicy::Nobody.allows(Role::Viewer));
    }

    #[test]
    fn a_domain_list_restricts_new_accounts_to_its_domains() {
        let mut s = SignIn::default();
        assert!(s.domain_allowed("anyone@example.org"));
        s.allowed_domains = vec!["@Example.org".into(), "kowalski.pl".into()];
        assert!(s.domain_allowed("a@example.org"));
        assert!(s.domain_allowed("b@KOWALSKI.PL"));
        assert!(!s.domain_allowed("c@example.org.evil.test"));
        assert!(!s.domain_allowed("no-at-sign"));
    }

    #[test]
    fn a_public_address_is_normalised_or_refused() {
        assert_eq!(
            normalise_base(" https://Family.Example/ ").as_deref(),
            Some("https://family.example")
        );
        assert_eq!(
            normalise_base("https://example.org/tree").as_deref(),
            Some("https://example.org/tree")
        );
        assert_eq!(normalise_base("ftp://example.org"), None);
        assert_eq!(normalise_base("https://"), None);
        assert_eq!(normalise_base("https://user@example.org"), None);
        assert_eq!(normalise_base("example.org"), None);
    }

    #[test]
    fn a_generated_secret_is_five_groups_of_four() {
        let s = generate_secret();
        let groups: Vec<&str> = s.split('-').collect();
        assert_eq!(groups.len(), 5, "{s}");
        assert!(groups.iter().all(|g| g.len() == 4), "{s}");
        assert_ne!(s, generate_secret());
    }

    #[test]
    fn an_older_acl_without_a_site_block_reads_as_the_defaults() {
        let acl: crate::acl::Acl = serde_json::from_str(r#"{"acl_version":1,"users":[]}"#).unwrap();
        assert_eq!(acl.site.registration.mode, RegistrationMode::Closed);
        assert!(acl.site.sign_in.link_by_email);
        assert!(acl.site.providers.ready().is_empty());
    }
}
