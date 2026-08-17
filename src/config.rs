//! Command-line configuration.

use std::net::SocketAddr;
use std::path::PathBuf;

use clap::Parser;

/// Environment variable consulted when `--admin-token` is absent.
pub const ADMIN_TOKEN_ENV: &str = "AXGF_CMS_ADMIN_TOKEN";

/// Browse and edit one AXGF bundle.
#[derive(Debug, Parser)]
#[command(name = "axgf-cms", version, about, long_about = None)]
pub struct Config {
    /// Path to the .axgf bundle. Created empty if it does not exist.
    #[arg(long, value_name = "PATH")]
    pub bundle: PathBuf,

    /// Address to bind. Defaults to localhost: V1 has no user accounts, so the
    /// admin surface must not reach the network by accident.
    #[arg(long, value_name = "ADDR", default_value = "127.0.0.1:8080")]
    pub bind: SocketAddr,

    /// Shared admin token. Falls back to AXGF_CMS_ADMIN_TOKEN, then to a
    /// random token printed once to stderr at startup.
    #[arg(long, value_name = "TOKEN", env = ADMIN_TOKEN_ENV)]
    pub admin_token: Option<String>,

    /// Seed a *new* bundle with the built-in demonstration family instead of
    /// creating an empty one. Ignored when the bundle already exists, so this
    /// is safe to leave set permanently and safe to run twice.
    #[arg(long)]
    pub seed_sample: bool,

    /// Bundle size, in megabytes, past which the admin panel starts warning
    /// that the archive is getting heavy. Textual data is memory-resident and
    /// bounded by the tree's size; only that is measured against this. Not a
    /// limit — the archive is the operator's.
    #[arg(long, value_name = "MB", default_value_t = crate::documents::DEFAULT_SIZE_WARN / (1024 * 1024))]
    pub size_warn_mb: u64,

    /// Directory for the binary-payload cache. Defaults to
    /// `<bundle_dir>/.axgf-cms-cache`. Override it when the bundle sits on slow
    /// or read-only storage. It is derived data — the `.axgf` is authoritative
    /// — and never needs backing up.
    #[arg(long, value_name = "PATH")]
    pub cache_dir: Option<PathBuf>,
}

impl Config {
    /// Resolve the admin token, generating one when none was supplied.
    ///
    /// Returns the token and whether it was generated, so the caller can print
    /// a generated token exactly once.
    pub fn resolve_admin_token(&self) -> (String, bool) {
        match self.admin_token.as_deref() {
            Some(t) if !t.trim().is_empty() => (t.to_string(), false),
            _ => (generate_token(), true),
        }
    }
}

/// Generate a random hex token.
///
/// Two v4 UUIDs from the OS random source give 244 bits of entropy, which is
/// ample for a bearer token and avoids pulling in a separate RNG stack.
fn generate_token() -> String {
    let a = uuid::Uuid::new_v4().simple().to_string();
    let b = uuid::Uuid::new_v4().simple().to_string();
    format!("{a}{b}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg(token: Option<&str>) -> Config {
        Config {
            bundle: PathBuf::from("/tmp/x.axgf"),
            bind: "127.0.0.1:8080".parse().unwrap(),
            admin_token: token.map(str::to_string),
            seed_sample: false,
            size_warn_mb: 200,
            cache_dir: None,
        }
    }

    #[test]
    fn explicit_token_is_used_verbatim() {
        let (t, generated) = cfg(Some("hunter2")).resolve_admin_token();
        assert_eq!(t, "hunter2");
        assert!(!generated);
    }

    #[test]
    fn absent_token_is_generated() {
        let (t, generated) = cfg(None).resolve_admin_token();
        assert!(generated);
        assert_eq!(t.len(), 64, "two v4 UUIDs, dashes stripped");
    }

    #[test]
    fn blank_token_is_treated_as_absent() {
        // An empty AXGF_CMS_ADMIN_TOKEN in a systemd env file must not become
        // an empty password that every request satisfies.
        let (t, generated) = cfg(Some("   ")).resolve_admin_token();
        assert!(generated);
        assert_ne!(t.trim(), "");
    }

    #[test]
    fn generated_tokens_differ() {
        let (a, _) = cfg(None).resolve_admin_token();
        let (b, _) = cfg(None).resolve_admin_token();
        assert_ne!(a, b);
    }

    #[test]
    fn default_bind_is_localhost() {
        let c = Config::parse_from(["axgf-cms", "--bundle", "/tmp/x.axgf"]);
        assert_eq!(c.bind.to_string(), "127.0.0.1:8080");
        assert!(c.bind.ip().is_loopback());
    }

    #[test]
    fn the_size_warning_threshold_defaults_to_200_mb_and_is_settable() {
        let c = Config::parse_from(["axgf-cms", "--bundle", "/tmp/x.axgf"]);
        assert_eq!(c.size_warn_mb, 200);
        let c = Config::parse_from([
            "axgf-cms",
            "--bundle",
            "/tmp/x.axgf",
            "--size-warn-mb",
            "50",
        ]);
        assert_eq!(c.size_warn_mb, 50);
    }
}
