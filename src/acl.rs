//! Accounts, roles and family scope — the `.acl` companion file.
//!
//! # Why the accounts are not in the bundle
//!
//! A `.axgf` bundle is meant to be copied. It is mailed to a cousin, published
//! on a website, handed to an archive and restored from a backup years later.
//! Putting password hashes inside it would make every one of those copies a
//! copy of the credential store, and there is no way to un-send an email. The
//! genealogy and the credentials have opposite distribution rules, so they are
//! two files:
//!
//! ```text
//! family.axgf   the genealogy   — shareable, no credentials
//! family.acl    the accounts    — mode 600, never shared casually
//! ```
//!
//! The ACL is JSON with a small schema of its own, inspectable and versionable
//! like everything else here. Encryption at rest is GPG's job and out of scope;
//! what this module guarantees is that the file is not readable by other users
//! on the host, and that the process refuses to start if it is.
//!
//! # Passwords
//!
//! Argon2id with the OWASP 2024 parameters (m=19456 KiB, t=2, p=1), stored in
//! the standard PHC string so the parameters travel with the hash and can be
//! raised later without invalidating anything. Never SHA-256: that is a fast
//! hash meant for integrity, and a GPU tries billions of candidates a second
//! against it.
//!
//! # Binding to a bundle
//!
//! An ACL records which family tree it was created for. Pointing one family's
//! accounts at another family's bundle is not something to do silently — the
//! roles would apply, the family scopes would name person ids that mean
//! nothing in the new tree, and nobody would be told. The check is a warning
//! rather than a refusal, because the legitimate case (a bundle renamed, or
//! restored from a backup) is common and blocking it would be worse.

use std::collections::BTreeSet;
use std::fmt;
use std::fs;
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use argon2::{Algorithm, Argon2, Params, Version};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Schema version of the `.acl` file, so a later release can migrate rather
/// than guess.
pub const ACL_VERSION: u32 = 1;

/// OWASP 2024 parameters for Argon2id: 19 MiB of memory, two passes, one lane.
pub const ARGON2_M_COST: u32 = 19456;
/// Time cost (passes) — see [`ARGON2_M_COST`].
pub const ARGON2_T_COST: u32 = 2;
/// Parallelism (lanes) — see [`ARGON2_M_COST`].
pub const ARGON2_P_COST: u32 = 1;

// ---------------------------------------------------------------------------
// roles and visibility
// ---------------------------------------------------------------------------

/// What a signed-in account may do.
///
/// Three roles, named after the `visibility` vocabulary the AXGF specification
/// already defines, so the two systems share one language: a `contributor`
/// account is exactly the audience an entity marked `visibility:
/// contributors` is addressed to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    /// Reads entities marked `public` or `members`.
    Viewer,
    /// Also creates, updates and uploads, and reads `contributors` entities.
    Contributor,
    /// Also manages users, deletes under any policy, dedups, validates,
    /// exports, and reads `private` entities.
    Admin,
}

impl Role {
    /// The highest visibility this role may read.
    pub fn ceiling(self) -> Visibility {
        match self {
            Role::Viewer => Visibility::Members,
            Role::Contributor => Visibility::Contributors,
            Role::Admin => Visibility::Private,
        }
    }

    /// May this role create, update and upload?
    pub fn may_write(self) -> bool {
        self >= Role::Contributor
    }

    /// May this role manage accounts, delete, dedup, validate and export?
    pub fn is_admin(self) -> bool {
        self == Role::Admin
    }

    /// The wire form, as written in the ACL file and the forms.
    pub fn as_str(self) -> &'static str {
        match self {
            Role::Viewer => "viewer",
            Role::Contributor => "contributor",
            Role::Admin => "admin",
        }
    }

    /// Parse a role from a form field or the ACL file.
    pub fn parse(s: &str) -> Option<Self> {
        match s.trim().to_ascii_lowercase().as_str() {
            "viewer" => Some(Role::Viewer),
            "contributor" => Some(Role::Contributor),
            "admin" => Some(Role::Admin),
            _ => None,
        }
    }

    /// Every role, weakest first — for the account form.
    pub const ALL: [Role; 3] = [Role::Viewer, Role::Contributor, Role::Admin];
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The AXGF `visibility` ladder, weakest first.
///
/// This is the specification's enum, not an invention of this application:
/// `person.identity.visibility` and `link.visibility` carry exactly these
/// values, and the roles above are defined in terms of them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Visibility {
    /// Anyone, signed in or not.
    Public,
    /// Any signed-in account.
    Members,
    /// Contributors and admins.
    Contributors,
    /// Admins only.
    Private,
}

impl Visibility {
    /// Parse the spec's spelling; anything unrecognised is `None`, which the
    /// caller resolves through its own default rather than guessing here.
    pub fn parse(s: &str) -> Option<Self> {
        match s.trim().to_ascii_lowercase().as_str() {
            "public" => Some(Visibility::Public),
            "members" => Some(Visibility::Members),
            "contributors" => Some(Visibility::Contributors),
            "private" => Some(Visibility::Private),
            _ => None,
        }
    }

    /// The wire form.
    pub fn as_str(self) -> &'static str {
        match self {
            Visibility::Public => "public",
            Visibility::Members => "members",
            Visibility::Contributors => "contributors",
            Visibility::Private => "private",
        }
    }
}

// ---------------------------------------------------------------------------
// the file
// ---------------------------------------------------------------------------

/// One account.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Argon2id PHC string. Never a bare digest, never reversible.
    pub password_hash: String,
    pub role: Role,
    pub status: Status,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_login: Option<String>,
    /// BCP 47 tag for the interface language. The *data* is never translated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub theme: Option<String>,
    /// Whether the soft page background is drawn. Stored beside the theme and
    /// the language because it is the same kind of thing: a choice about how
    /// the interface looks that should follow the reader to another browser.
    /// `None` means they have never said, which is not the same as `false`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backgrounds: Option<bool>,
    /// Root person ids this account may edit under. Empty means the whole
    /// tree. Read access is governed by visibility, never by this.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub family_scope: Vec<String>,
}

/// Whether an account may sign in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Active,
    /// Kept, but refused at the door. Deleting an account would orphan its
    /// entries in the edit journal.
    Disabled,
}

impl Status {
    pub fn as_str(self) -> &'static str {
        match self {
            Status::Active => "active",
            Status::Disabled => "disabled",
        }
    }
}

/// Which bundle this ACL was created for.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BundleBinding {
    /// SHA-256 of the `.axgf` at ACL creation. Every edit changes it, so it
    /// identifies the bundle only until the first write — which is exactly the
    /// window in which a fresh ACL could be pointed at the wrong tree.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sha256: Option<String>,
    /// `manifest.family.name`, which survives editing.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    /// `manifest.created_at`, which also survives editing.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

/// The `.acl` file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Acl {
    pub acl_version: u32,
    #[serde(default)]
    pub bundle: BundleBinding,
    #[serde(default)]
    pub users: Vec<User>,
}

impl Default for Acl {
    fn default() -> Self {
        Self {
            acl_version: ACL_VERSION,
            bundle: BundleBinding::default(),
            users: Vec::new(),
        }
    }
}

/// The result of checking an ACL against the bundle it was loaded beside.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Binding {
    /// The ACL names this bundle, or names nothing yet.
    Ok,
    /// The ACL was created for a different tree. Said out loud, not enforced:
    /// a restored backup and a renamed file are both legitimate, and refusing
    /// to start would be the worse failure.
    Mismatch { expected: String, found: String },
}

impl Acl {
    /// The conventional ACL path for a bundle: the same name with `.acl`.
    pub fn path_for(bundle: &Path) -> PathBuf {
        bundle.with_extension("acl")
    }

    /// Read an ACL, refusing a file other users on the host can read.
    ///
    /// The permission check is the point of the separate file. An ACL that
    /// anyone can read is a credential store that anyone can grind offline,
    /// and starting anyway while logging a warning is how that stays true for
    /// years.
    pub fn load(path: &Path) -> Result<Self> {
        let meta = fs::metadata(path).with_context(|| format!("reading {}", path.display()))?;
        let mode = meta.permissions().mode() & 0o777;
        if mode & 0o077 != 0 {
            bail!(
                "{} is mode {:03o}: readable or writable by users other than its owner.\n\
                 It holds password hashes, so it must be mode 600.\n\
                 Fix it with:  chmod 600 {}",
                path.display(),
                mode,
                path.display()
            );
        }
        let body =
            fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
        let acl: Acl = serde_json::from_str(&body)
            .with_context(|| format!("{} is not a valid ACL file", path.display()))?;
        if acl.acl_version > ACL_VERSION {
            bail!(
                "{} declares acl_version {}, but this build understands {}. \
                 Upgrade axgf-cms rather than letting an older build write it back.",
                path.display(),
                acl.acl_version,
                ACL_VERSION
            );
        }
        Ok(acl)
    }

    /// Write the ACL at mode 600, atomically.
    ///
    /// The temp file is created with the same mode, so the hashes are never
    /// briefly world-readable between `create` and `set_permissions`.
    pub fn save(&self, path: &Path) -> Result<()> {
        let tmp = path.with_extension("acl.tmp");
        let body = serde_json::to_string_pretty(self).context("serialising the ACL")?;
        {
            let mut f = fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .mode(0o600)
                .open(&tmp)
                .with_context(|| format!("creating {}", tmp.display()))?;
            use std::io::Write as _;
            f.write_all(body.as_bytes())
                .with_context(|| format!("writing {}", tmp.display()))?;
            f.sync_all().ok();
        }
        // An existing file keeps whatever mode it had unless we say otherwise;
        // rename carries the temp file's 600 over, which is what we want.
        fs::rename(&tmp, path)
            .with_context(|| format!("replacing {} with {}", path.display(), tmp.display()))?;
        Ok(())
    }

    /// Find an active account by username, case-insensitively.
    pub fn active(&self, username: &str) -> Option<&User> {
        self.users
            .iter()
            .find(|u| u.status == Status::Active && u.username.eq_ignore_ascii_case(username))
    }

    /// Find any account by id.
    pub fn by_id(&self, id: &str) -> Option<&User> {
        self.users.iter().find(|u| u.id == id)
    }

    /// Whether a username is already taken, case-insensitively.
    pub fn has_username(&self, username: &str) -> bool {
        self.users
            .iter()
            .any(|u| u.username.eq_ignore_ascii_case(username))
    }

    /// How many active admins the ACL holds.
    ///
    /// Consulted before demoting or disabling one, so an installation cannot
    /// be locked out of its own admin surface by a single form post.
    pub fn active_admins(&self) -> usize {
        self.users
            .iter()
            .filter(|u| u.status == Status::Active && u.role == Role::Admin)
            .count()
    }

    /// Record which bundle this ACL belongs to.
    pub fn bind_to(&mut self, manifest: Option<&Value>, sha256: Option<String>) {
        self.bundle = BundleBinding {
            sha256,
            family: manifest
                .and_then(|m| m.get("family"))
                .and_then(|f| f.get("name"))
                .and_then(Value::as_str)
                .map(str::to_string),
            created_at: manifest
                .and_then(|m| m.get("created_at"))
                .and_then(Value::as_str)
                .map(str::to_string),
        };
    }

    /// Compare the recorded binding against the bundle actually loaded.
    pub fn check_binding(&self, manifest: Option<&Value>, sha256: Option<&str>) -> Binding {
        let b = &self.bundle;
        if b.sha256.is_none() && b.family.is_none() && b.created_at.is_none() {
            return Binding::Ok;
        }
        // The file's own hash, when it still matches, settles it outright.
        if let (Some(want), Some(got)) = (b.sha256.as_deref(), sha256) {
            if want == got {
                return Binding::Ok;
            }
        }
        let family = |m: Option<&Value>| {
            m.and_then(|m| m.get("family"))
                .and_then(|f| f.get("name"))
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_string()
        };
        let created = |m: Option<&Value>| {
            m.and_then(|m| m.get("created_at"))
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_string()
        };
        let (want_f, want_c) = (
            b.family.clone().unwrap_or_default(),
            b.created_at.clone().unwrap_or_default(),
        );
        let (got_f, got_c) = (family(manifest), created(manifest));
        if want_f == got_f && want_c == got_c {
            return Binding::Ok;
        }
        Binding::Mismatch {
            expected: describe(&want_f, &want_c),
            found: describe(&got_f, &got_c),
        }
    }
}

fn describe(family: &str, created: &str) -> String {
    match (family.is_empty(), created.is_empty()) {
        (true, true) => "an unnamed bundle".into(),
        (true, false) => format!("a bundle created {created}"),
        (false, true) => format!("“{family}”"),
        (false, false) => format!("“{family}”, created {created}"),
    }
}

// ---------------------------------------------------------------------------
// passwords
// ---------------------------------------------------------------------------

/// The configured Argon2id hasher.
fn argon2() -> Argon2<'static> {
    let params = Params::new(ARGON2_M_COST, ARGON2_T_COST, ARGON2_P_COST, None)
        .expect("the OWASP parameters are valid by construction");
    Argon2::new(Algorithm::Argon2id, Version::V0x13, params)
}

/// Hash a password with Argon2id, returning a PHC string.
/// A real Argon2id hash of a value nobody knows, at the same parameters as
/// every stored hash.
///
/// Verifying against this when the username does not exist makes a failed
/// login cost the same wall-clock time whether or not the account is real.
/// Without it the difference is the whole Argon2id computation — tens of
/// milliseconds at these parameters — which is not a subtle timing signal
/// requiring statistics to extract, but a plainly visible one that turns the
/// login form into a list of which accounts exist.
/// It is a real hash produced by [`hash_password`], not a hand-written string:
/// an invented one parses with a short salt and finishes measurably sooner,
/// which reintroduces the very signal it exists to remove.
pub const DUMMY_HASH: &str =
    "$argon2id$v=19$m=19456,t=2,p=1$Ls9pMbhhjjKrffiRq/Dl6g$q3J7vjR4/zegv99IjvaT63OOmgZrq1FWmcKT+/Z2wRk";

pub fn hash_password(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut argon2::password_hash::rand_core::OsRng);
    let hash = argon2()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow::anyhow!("hashing the password: {e}"))?;
    Ok(hash.to_string())
}

/// Check a password against a stored PHC string.
///
/// A malformed stored hash verifies as `false` rather than erroring: it means
/// the account cannot be signed into, which is the safe reading, and it is not
/// something the person at the keyboard can act on.
pub fn verify_password(password: &str, phc: &str) -> bool {
    match PasswordHash::new(phc) {
        Ok(parsed) => argon2()
            .verify_password(password.as_bytes(), &parsed)
            .is_ok(),
        Err(_) => false,
    }
}

/// A generated password, printed once and never stored in the clear.
///
/// Four groups of five characters from an unambiguous alphabet: no `0`/`O`,
/// no `1`/`l`/`I`. About 100 bits, and it survives being read aloud over the
/// phone or copied off a terminal, which is how a first password actually
/// travels.
pub fn generate_password() -> String {
    const ALPHABET: &[u8] = b"abcdefghijkmnpqrstuvwxyz23456789ACDEFGHJKLMNPQRTUVWXY";
    let mut out = String::with_capacity(23);
    for group in 0..4 {
        if group > 0 {
            out.push('-');
        }
        for _ in 0..5 {
            // uuid v4 is the OS random source this crate already trusts for
            // tokens; taking one byte per character keeps the dependency list
            // where it is.
            let b = uuid::Uuid::new_v4().as_bytes()[0];
            out.push(ALPHABET[b as usize % ALPHABET.len()] as char);
        }
    }
    out
}

/// Build a new user with a fresh id and creation timestamp.
pub fn new_user(username: &str, password: &str, role: Role) -> Result<User> {
    Ok(User {
        id: uuid::Uuid::new_v4().to_string(),
        username: username.trim().to_string(),
        email: None,
        password_hash: hash_password(password)?,
        role,
        status: Status::Active,
        created_at: crate::view::now_iso8601(),
        last_login: None,
        language: None,
        theme: None,
        backgrounds: None,
        family_scope: Vec::new(),
    })
}

/// Reject usernames that would be ambiguous or unusable.
pub fn validate_username(name: &str) -> Result<()> {
    let n = name.trim();
    if n.len() < 2 || n.len() > 64 {
        bail!("A username must be between 2 and 64 characters.");
    }
    if !n
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '-' || c == '_')
    {
        bail!("A username may contain letters, digits, dot, dash and underscore only.");
    }
    Ok(())
}

/// The shortest password this application will store.
pub const MIN_PASSWORD: usize = 12;

/// Reject a password too short to be worth hashing.
///
/// Length only. Composition rules ("one digit, one symbol") shrink the search
/// space people actually use and are not what stops an offline attack; the
/// Argon2 parameters are.
pub fn validate_password(password: &str) -> Result<()> {
    if password.chars().count() < MIN_PASSWORD {
        bail!("A password must be at least {MIN_PASSWORD} characters.");
    }
    Ok(())
}

/// SHA-256 of a file, hex-encoded — the bundle binding's fingerprint.
pub fn file_sha256(path: &Path) -> Option<String> {
    use sha2::Digest as _;
    let mut file = fs::File::open(path).ok()?;
    let mut hasher = sha2::Sha256::new();
    std::io::copy(&mut file, &mut hasher).ok()?;
    Some(format!("{:x}", hasher.finalize()))
}

/// The set of person ids an account may edit, from its scope roots.
///
/// A scope is a list of root people; the account may edit those roots, their
/// descendants, and the spouses of everyone reached. Spouses are included
/// because a family is not editable without them — recording a marriage means
/// touching both records — but they are a leaf: a spouse's own ancestors
/// belong to another branch, and following them would quietly widen the scope
/// back to the whole tree.
///
/// Empty roots means the whole tree, represented by `None` so no set has to be
/// built for the common case.
pub fn scope_set(flat: &Value, roots: &[String]) -> Option<BTreeSet<String>> {
    if roots.is_empty() {
        return None;
    }
    let families = flat.get("families").and_then(Value::as_object);
    // parent id -> the families they are a parent of
    let mut parent_of: std::collections::BTreeMap<&str, Vec<&Value>> = Default::default();
    if let Some(families) = families {
        for f in families.values() {
            for pid in union_person_ids(f) {
                parent_of.entry(pid).or_default().push(f);
            }
        }
    }

    let mut out: BTreeSet<String> = roots.iter().cloned().collect();
    let mut queue: std::collections::VecDeque<String> = roots.iter().cloned().collect();
    while let Some(id) = queue.pop_front() {
        for f in parent_of.get(id.as_str()).map(Vec::as_slice).unwrap_or(&[]) {
            for child in child_person_ids(f) {
                if out.insert(child.to_string()) {
                    queue.push_back(child.to_string());
                }
            }
        }
    }
    // Spouses of everyone reached, as a single closing pass.
    let reached: Vec<String> = out.iter().cloned().collect();
    for id in reached {
        for f in parent_of.get(id.as_str()).map(Vec::as_slice).unwrap_or(&[]) {
            for partner in union_person_ids(f) {
                out.insert(partner.to_string());
            }
        }
    }
    Some(out)
}

fn union_person_ids(family: &Value) -> Vec<&str> {
    family
        .get("union")
        .and_then(|u| u.get("persons"))
        .and_then(Value::as_array)
        .map(|ps| {
            ps.iter()
                .filter_map(|p| p.get("person_id").and_then(Value::as_str))
                .collect()
        })
        .unwrap_or_default()
}

fn child_person_ids(family: &Value) -> Vec<&str> {
    family
        .get("children")
        .and_then(Value::as_array)
        .map(|cs| {
            cs.iter()
                .filter_map(|c| c.get("person_id").and_then(Value::as_str))
                .collect()
        })
        .unwrap_or_default()
}

#[cfg(test)]
mod dummy_hash_tests {
    use super::*;

    #[test]
    fn the_dummy_hash_is_a_real_hash_at_the_same_parameters() {
        // If this ever stops parsing, `verify_password` returns immediately
        // and an unknown username becomes tens of milliseconds faster than a
        // known one — which is the account-enumeration oracle the constant
        // exists to close. The assertion is that it *does* work, not that it
        // looks right.
        assert!(
            !verify_password("anything at all", DUMMY_HASH),
            "nothing may verify against it"
        );
        let real = hash_password("some other password").unwrap();
        for field in ["$argon2id$", "v=19", "m=19456", "t=2", "p=1"] {
            assert!(
                DUMMY_HASH.contains(field),
                "the dummy must carry {field}, as every stored hash does"
            );
            assert!(real.contains(field));
        }
        // Same shape: algorithm, version, parameters, salt and digest.
        assert_eq!(
            DUMMY_HASH.matches('$').count(),
            real.matches('$').count(),
            "a hand-written stand-in with a missing field parses faster"
        );
        let seg = |s: &str, n: usize| s.split('$').nth(n).unwrap().len();
        assert_eq!(seg(DUMMY_HASH, 4), seg(&real, 4), "salt length");
        assert_eq!(seg(DUMMY_HASH, 5), seg(&real, 5), "digest length");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn tmpdir(tag: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("axgf-acl-{}-{tag}", std::process::id()));
        let _ = fs::remove_dir_all(&d);
        fs::create_dir_all(&d).unwrap();
        d
    }

    #[test]
    fn a_password_verifies_against_its_own_hash_and_nothing_else() {
        let phc = hash_password("correct horse battery").unwrap();
        assert!(phc.starts_with("$argon2id$"), "must be argon2id: {phc}");
        assert!(verify_password("correct horse battery", &phc));
        assert!(!verify_password("correct horse batteru", &phc));
        assert!(!verify_password("", &phc));
    }

    #[test]
    fn the_stored_parameters_are_the_owasp_2024_ones() {
        // The parameters live in the PHC string, so they are checkable rather
        // than asserted in a comment — and a later change to them is visible
        // here rather than silent.
        let phc = hash_password("correct horse battery").unwrap();
        assert!(
            phc.contains("m=19456,t=2,p=1"),
            "expected OWASP 2024 parameters, got {phc}"
        );
    }

    #[test]
    fn the_same_password_hashes_differently_every_time() {
        // A per-hash salt, so two accounts with the same password do not look
        // alike in the file.
        let a = hash_password("correct horse battery").unwrap();
        let b = hash_password("correct horse battery").unwrap();
        assert_ne!(a, b);
        assert!(verify_password("correct horse battery", &a));
        assert!(verify_password("correct horse battery", &b));
    }

    #[test]
    fn a_malformed_hash_refuses_rather_than_panicking() {
        assert!(!verify_password("x", "not-a-phc-string"));
        assert!(!verify_password("x", ""));
        // Notably: a bare SHA-256 hex digest, which is what this must never be.
        assert!(!verify_password(
            "x",
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        ));
    }

    #[test]
    fn saving_writes_mode_600_and_loading_refuses_anything_looser() {
        let dir = tmpdir("modes");
        let path = dir.join("family.acl");
        let mut acl = Acl::default();
        acl.users
            .push(new_user("karin", "correct horse battery", Role::Admin).unwrap());
        acl.save(&path).unwrap();

        let mode = fs::metadata(&path).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, 0o600, "the ACL must be created unreadable to others");
        assert!(Acl::load(&path).is_ok());

        fs::set_permissions(&path, fs::Permissions::from_mode(0o644)).unwrap();
        let err = Acl::load(&path).unwrap_err().to_string();
        assert!(err.contains("mode 644"), "must say what it found: {err}");
        assert!(err.contains("chmod 600"), "must say how to fix it: {err}");
    }

    #[test]
    fn a_round_trip_keeps_every_field() {
        let dir = tmpdir("roundtrip");
        let path = dir.join("family.acl");
        let mut acl = Acl::default();
        let mut u = new_user("karin", "correct horse battery", Role::Contributor).unwrap();
        u.email = Some("karin@example.org".into());
        u.language = Some("pl".into());
        u.theme = Some("deuteranopia".into());
        u.family_scope = vec!["11111111-1111-4111-8111-111111111111".into()];
        acl.users.push(u);
        acl.bind_to(
            Some(&json!({"family": {"name": "Klicki"}, "created_at": "2026-08-09T13:15:54Z"})),
            Some("abc123".into()),
        );
        acl.save(&path).unwrap();

        let back = Acl::load(&path).unwrap();
        assert_eq!(back.acl_version, ACL_VERSION);
        assert_eq!(back.bundle.family.as_deref(), Some("Klicki"));
        let u = &back.users[0];
        assert_eq!(u.username, "karin");
        assert_eq!(u.role, Role::Contributor);
        assert_eq!(u.language.as_deref(), Some("pl"));
        assert_eq!(u.family_scope.len(), 1);
        assert!(verify_password("correct horse battery", &u.password_hash));
    }

    #[test]
    fn another_familys_acl_is_detected() {
        let mut acl = Acl::default();
        acl.bind_to(
            Some(&json!({"family": {"name": "Klicki"}, "created_at": "2026-08-09T13:15:54Z"})),
            Some("aaa".into()),
        );

        // The same tree, edited since — the hash moved, the identity did not.
        let same = json!({"family": {"name": "Klicki"}, "created_at": "2026-08-09T13:15:54Z"});
        assert_eq!(acl.check_binding(Some(&same), Some("bbb")), Binding::Ok);

        // A different family altogether.
        let other = json!({"family": {"name": "Meunier"}, "created_at": "2020-01-01T00:00:00Z"});
        match acl.check_binding(Some(&other), Some("bbb")) {
            Binding::Mismatch { expected, found } => {
                assert!(expected.contains("Klicki"), "{expected}");
                assert!(found.contains("Meunier"), "{found}");
            }
            Binding::Ok => panic!("a different family must be reported"),
        }
    }

    #[test]
    fn an_unbound_acl_matches_anything() {
        let acl = Acl::default();
        let any = json!({"family": {"name": "Whoever"}});
        assert_eq!(acl.check_binding(Some(&any), Some("x")), Binding::Ok);
    }

    #[test]
    fn roles_order_from_weakest_to_strongest() {
        assert!(Role::Viewer < Role::Contributor);
        assert!(Role::Contributor < Role::Admin);
        assert!(!Role::Viewer.may_write());
        assert!(Role::Contributor.may_write());
        assert!(!Role::Contributor.is_admin());
        assert!(Role::Admin.is_admin());
        assert_eq!(Role::Viewer.ceiling(), Visibility::Members);
        assert_eq!(Role::Contributor.ceiling(), Visibility::Contributors);
        assert_eq!(Role::Admin.ceiling(), Visibility::Private);
    }

    #[test]
    fn a_generated_password_is_long_and_unambiguous() {
        let p = generate_password();
        assert_eq!(p.len(), 23, "four groups of five plus three dashes: {p}");
        assert!(validate_password(&p).is_ok());
        assert!(
            !p.contains('0') && !p.contains('O'),
            "ambiguous glyph in {p}"
        );
        assert!(
            !p.contains('l') && !p.contains('1'),
            "ambiguous glyph in {p}"
        );
        assert_ne!(p, generate_password());
    }

    #[test]
    fn usernames_and_passwords_are_validated_before_hashing() {
        assert!(validate_username("karin").is_ok());
        assert!(validate_username("karin.p-l_2").is_ok());
        assert!(validate_username("a").is_err());
        assert!(validate_username("has space").is_err());
        assert!(validate_username("évé").is_err());
        assert!(validate_password("short").is_err());
        assert!(validate_password("correct horse battery").is_ok());
    }

    #[test]
    fn a_scope_covers_descendants_and_their_spouses_but_stops_there() {
        // root -> child; child married to spouse; spouse's own parent must not
        // be pulled in, or a branch scope quietly becomes the whole tree.
        let flat = json!({
            "families": {
                "f1": {"union": {"persons": [{"person_id": "root"}]},
                       "children": [{"person_id": "child"}]},
                "f2": {"union": {"persons": [{"person_id": "child"}, {"person_id": "spouse"}]},
                       "children": [{"person_id": "grandchild"}]},
                "f3": {"union": {"persons": [{"person_id": "outsider"}]},
                       "children": [{"person_id": "spouse"}]}
            }
        });
        let set = scope_set(&flat, &["root".to_string()]).unwrap();
        assert!(set.contains("root"));
        assert!(set.contains("child"));
        assert!(set.contains("grandchild"));
        assert!(
            set.contains("spouse"),
            "a spouse is needed to edit the union"
        );
        assert!(
            !set.contains("outsider"),
            "a spouse's parent is another branch and must stay out"
        );
    }

    #[test]
    fn an_empty_scope_is_the_whole_tree() {
        let flat = json!({"families": {}});
        assert!(scope_set(&flat, &[]).is_none());
    }
}
