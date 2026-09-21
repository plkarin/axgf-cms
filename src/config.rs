//! Command-line configuration.

use std::net::SocketAddr;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// Environment variable consulted when `--admin-token` is absent.
pub const ADMIN_TOKEN_ENV: &str = "AXGF_CMS_ADMIN_TOKEN";

/// Environment variable naming the backup directory.
pub const BACKUP_DIR_ENV: &str = "AXGF_CMS_BACKUP_DIR";

/// What to do instead of serving.
///
/// Serving stays the default with no subcommand, so every existing invocation
/// — the systemd unit's, the bootstrap script's, an operator's shell history —
/// keeps working unchanged.
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Write one verified archive of the bundle, the accounts and the journal.
    ///
    /// Safe to run against a live installation: it takes the same write lock a
    /// save takes, so the three files in the archive agree with each other.
    Backup {
        /// Directory to write the archive into. Created if absent. Defaults to
        /// --backup-dir, then to AXGF_CMS_BACKUP_DIR.
        #[arg(long, value_name = "DIR")]
        dest: Option<PathBuf>,
        /// Daily archives to keep.
        #[arg(long, value_name = "N", default_value_t = 7)]
        keep_daily: usize,
        /// Weekly archives to keep.
        #[arg(long, value_name = "N", default_value_t = 4)]
        keep_weekly: usize,
        /// Monthly archives to keep.
        #[arg(long, value_name = "N", default_value_t = 12)]
        keep_monthly: usize,
    },
    /// Put a backup archive back, keeping the current state aside first.
    ///
    /// Refuses to run while an instance is serving the bundle. Stop the
    /// service, restore, start it again.
    Restore {
        /// The archive to restore.
        #[arg(value_name = "ARCHIVE")]
        archive: PathBuf,
        /// Restore even though something appears to be running against the
        /// bundle. Almost never right; the running process would write its own
        /// copy back over the restored one at its next save.
        #[arg(long)]
        force: bool,
    },
    /// Read an archive and say what is in it, changing nothing.
    Verify {
        /// The archive to check.
        #[arg(value_name = "ARCHIVE")]
        archive: PathBuf,
    },
}

/// Browse and edit one AXGF bundle.
#[derive(Debug, Parser)]
#[command(name = "axgf-cms", version, about, long_about = None)]
pub struct Config {
    /// What to do. Absent means serve.
    #[command(subcommand)]
    pub command: Option<Command>,

    /// Path to the .axgf bundle. Created empty if it does not exist.
    ///
    /// Global, so it reads the same before or after a subcommand:
    /// `axgf-cms --bundle X backup --dest Y` and
    /// `axgf-cms backup --bundle X --dest Y` are the same command.
    #[arg(long, value_name = "PATH", global = true)]
    pub bundle: Option<PathBuf>,

    /// Address to bind. Defaults to localhost, and should stay there: this
    /// process speaks plain HTTP, so bound anywhere else it sends passwords
    /// across the network in clear text. TLS is the reverse proxy's job.
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

    /// Years after a birth beyond which a person marked living is presumed
    /// deceased, whatever the record says.
    ///
    /// GEDCOM cannot record "died, date unknown", so a converter marks
    /// everybody whose death was never written down as alive. Without this,
    /// the operator's own bundle renders a woman born in 1898 as living, which
    /// is a claim that she is 128.
    ///
    /// It is a *display* rule and it never touches the bundle. It also never
    /// touches permissions: a living person's health data stays withheld on
    /// the recorded flag, not the presumed one, because a plausibility rule
    /// that widens access is a privacy hole with arithmetic in front of it.
    ///
    /// `0` switches it off, for an operator whose bundle records deaths
    /// properly or who disagrees with the number.
    #[arg(long, value_name = "YEARS", default_value_t = crate::living::DEFAULT_MAX_AGE_YEARS)]
    pub presume_deceased_after: i64,

    /// Create an administrator account with this username, print a generated
    /// password once to stderr, and exit without serving.
    ///
    /// This is how an installation gets its first account. There is no web
    /// setup page on purpose: the window between deploying and the first login
    /// is exactly when an installation is unprotected, and a setup page is a
    /// door standing open for the length of it. Running this needs shell
    /// access to the host, which the attacker on the other side of that window
    /// does not have.
    ///
    /// Safe to re-run: an existing username is refused rather than
    /// overwritten, so a bootstrap script can call it unconditionally.
    #[arg(long, value_name = "USERNAME")]
    pub create_admin: Option<String>,

    /// Directory for the binary-payload cache. Defaults to
    /// `<bundle_dir>/.axgf-cms-cache`. Override it when the bundle sits on slow
    /// or read-only storage. It is derived data — the `.axgf` is authoritative
    /// — and never needs backing up.
    #[arg(long, value_name = "PATH")]
    pub cache_dir: Option<PathBuf>,

    /// Contact address to send to the geocoder, which turns place-name lookup
    /// on.
    ///
    /// Nominatim's usage policy requires a User-Agent identifying the
    /// application *and how to reach whoever runs it*. Without this there is no
    /// lookup button at all — an anonymous automated caller is exactly what the
    /// policy asks operators not to be, and the manual coordinate fields work
    /// with no third party involved. An email address or a URL both do.
    #[arg(long, value_name = "EMAIL_OR_URL")]
    pub geocoder_contact: Option<String>,

    /// A Nominatim-compatible `/search` endpoint to use instead of the public
    /// service. Point this at a self-hosted instance to lift the one-per-second
    /// pacing, which exists to be kind to donated infrastructure.
    #[arg(long, value_name = "URL")]
    pub geocoder_endpoint: Option<String>,

    /// Tile URL template for the place-editor map, which turns the map on.
    ///
    /// Off by default, and deliberately. Tiles are fetched by the reader's
    /// browser, not by this process, so switching them on puts every editor's
    /// address into the tile server's logs — the same objection this
    /// application already refuses to make the reader wear for geocoding, where
    /// the lookup goes through the server precisely so it does not happen. It
    /// is the operator's call to make knowingly, per installation, rather than
    /// a default somebody inherits.
    ///
    /// OpenStreetMap's standard layer is
    /// `https://tile.openstreetmap.org/{z}/{x}/{y}.png`; read their tile usage
    /// policy before pointing at it, and prefer a tile server you run or pay
    /// for. Whatever is set here, set `--map-attribution` to match: it is a
    /// licence condition, not decoration.
    ///
    /// With this unset the editor still offers a link out to a map of your
    /// choosing, and the position pasted back from it is read by the same
    /// parser. Nothing is lost but the click-to-place convenience.
    #[arg(long, value_name = "URL_TEMPLATE")]
    pub map_tiles: Option<String>,

    /// Attribution shown in the map's corner. Required by every tile source
    /// worth using, and empty by default because the correct text depends
    /// entirely on which one `--map-tiles` names.
    #[arg(long, value_name = "TEXT")]
    pub map_attribution: Option<String>,

    /// Directory where backup archives are written and looked for.
    ///
    /// Serving with this set is what lets `/health` and the admin dashboard say
    /// how old the newest backup is — the single most useful thing an operator
    /// can be told, because a backup that silently stopped running looks
    /// exactly like one that is working.
    #[arg(long, value_name = "DIR", env = BACKUP_DIR_ENV, global = true)]
    pub backup_dir: Option<PathBuf>,
}

impl Config {
    /// The bundle path, or an error saying how to supply one.
    ///
    /// `--bundle` is global rather than required because a required global is
    /// reported by clap as a usage error on the *subcommand*, which reads as
    /// though `backup` is at fault. This says the one thing that helps.
    pub fn bundle(&self) -> anyhow::Result<&PathBuf> {
        self.bundle.as_ref().ok_or_else(|| {
            anyhow::anyhow!(
                "no bundle was named. Every command works on one .axgf file:\n\n\
                 \x20 axgf-cms --bundle /var/lib/axgf-cms/family.axgf\n\
                 \x20 axgf-cms backup --bundle /var/lib/axgf-cms/family.axgf --dest /srv/backups"
            )
        })
    }

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
            command: None,
            backup_dir: None,
            bundle: Some(PathBuf::from("/tmp/x.axgf")),
            bind: "127.0.0.1:8080".parse().unwrap(),
            admin_token: token.map(str::to_string),
            seed_sample: false,
            size_warn_mb: 200,
            presume_deceased_after: crate::living::DEFAULT_MAX_AGE_YEARS,
            create_admin: None,
            cache_dir: None,
            geocoder_contact: None,
            geocoder_endpoint: None,
            map_tiles: None,
            map_attribution: None,
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
    fn no_subcommand_still_means_serve() {
        // Every existing invocation — the systemd unit's, the bootstrap
        // script's — must keep working exactly as it did.
        let c = Config::parse_from(["axgf-cms", "--bundle", "/tmp/x.axgf"]);
        assert!(c.command.is_none());
        assert_eq!(c.bundle().unwrap(), &PathBuf::from("/tmp/x.axgf"));
    }

    #[test]
    fn the_bundle_reads_the_same_on_either_side_of_a_subcommand() {
        for args in [
            vec![
                "axgf-cms",
                "--bundle",
                "/srv/f.axgf",
                "backup",
                "--dest",
                "/b",
            ],
            vec![
                "axgf-cms",
                "backup",
                "--bundle",
                "/srv/f.axgf",
                "--dest",
                "/b",
            ],
        ] {
            let c = Config::parse_from(args);
            assert_eq!(c.bundle().unwrap(), &PathBuf::from("/srv/f.axgf"));
            match c.command {
                Some(Command::Backup {
                    dest, keep_daily, ..
                }) => {
                    assert_eq!(dest, Some(PathBuf::from("/b")));
                    assert_eq!(keep_daily, 7);
                }
                other => panic!("expected a backup, got {other:?}"),
            }
        }
    }

    #[test]
    fn a_missing_bundle_says_how_to_supply_one() {
        let c = Config::parse_from(["axgf-cms", "verify", "/b/x.zip"]);
        let err = c.bundle().expect_err("none was given");
        assert!(format!("{err}").contains("--bundle"), "{err}");
    }

    #[test]
    fn retention_is_configurable_from_the_command_line() {
        let c = Config::parse_from([
            "axgf-cms",
            "backup",
            "--bundle",
            "/srv/f.axgf",
            "--dest",
            "/b",
            "--keep-daily",
            "30",
            "--keep-weekly",
            "0",
            "--keep-monthly",
            "24",
        ]);
        match c.command {
            Some(Command::Backup {
                keep_daily,
                keep_weekly,
                keep_monthly,
                ..
            }) => {
                assert_eq!((keep_daily, keep_weekly, keep_monthly), (30, 0, 24));
            }
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn restore_refuses_by_default_and_forces_only_when_asked() {
        let c = Config::parse_from(["axgf-cms", "restore", "/b/a.zip", "--bundle", "/srv/f.axgf"]);
        match c.command {
            Some(Command::Restore { archive, force }) => {
                assert_eq!(archive, PathBuf::from("/b/a.zip"));
                assert!(!force, "forcing is never the default");
            }
            other => panic!("{other:?}"),
        }
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
