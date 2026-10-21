//! The encrypted export: the bundle as an OpenPGP message to one recipient
//! (v0.5.0, docs/adr/0011 §4).
//!
//! # What comes out
//!
//! `family.axgf.gpg`, which `gpg --decrypt family.axgf.gpg > family.axgf`
//! opens with the recipient's private key and nothing else: a public-key
//! encrypted session key, and the bundle inside a symmetrically encrypted,
//! integrity-protected packet (SEIPD v1), with AES-256. Version 1 because it
//! is the one every GnuPG in service reads; RFC 9580's version 2 (AES-256 in
//! GCM) is not read by GnuPG 2.4.
//!
//! # Which keys, and the FIPS question
//!
//! The recipient is an RSA key of 2048 bits or more, or an elliptic-curve
//! (ECDH) key. Smaller RSA, ElGamal and keys that cannot encrypt are refused;
//! an expired or revoked key is refused; a key whose self-signatures do not
//! verify is refused, because a substituted subkey is how an export would be
//! sent to someone else.
//!
//! The page says whether the recipient's algorithms are in the FIPS-approved
//! set: AES-256, and RSA of 3072 bits or more or ECDH on NIST P-256, P-384 or
//! P-521. It never says the export is "FIPS compliant": that is a property of
//! a validated cryptographic module, and the one here — RustCrypto, through
//! rpgp — is not one. OpenPGP's RSA key transport is also PKCS #1 v1.5, which
//! NIST SP 800-131A no longer approves for key transport; a NIST curve does not
//! have that problem, which is why the page recommends one.
//!
//! # Memory
//!
//! Streamed: the bundle is read through a buffer and written through another.
//! A 435 MB archive costs two buffers, not 435 MB.

use std::fs::File;
use std::io::{BufReader, BufWriter, Write as _};
use std::path::Path;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use pgp::composed::{Deserializable, MessageBuilder, SignedPublicKey, SignedPublicSubKey};
use pgp::crypto::public_key::PublicKeyAlgorithm;
use pgp::crypto::sym::SymmetricKeyAlgorithm;
use pgp::types::{KeyDetails, PublicParams};
use rsa::traits::PublicKeyParts as _;

/// The smallest RSA key an export is made for.
pub const MIN_RSA_BITS: usize = 2048;
/// The smallest RSA key counted as in the FIPS-approved set for new work.
pub const FIPS_RSA_BITS: usize = 3072;
/// The longest public key a form may carry: a large RSA key with many
/// signatures is a few tens of kilobytes; this is generous.
pub const MAX_KEY_TEXT: usize = 256 * 1024;

/// Why a key will not do.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SealError {
    /// Not an OpenPGP public key, or not one this library can read.
    NotAKey,
    /// Its self-signatures do not verify.
    BadSignatures,
    /// It, or every key in it that can encrypt, has expired.
    Expired,
    /// Its owner revoked it.
    Revoked,
    /// It holds no key allowed to encrypt.
    NoEncryptionKey,
    /// An RSA key smaller than [`MIN_RSA_BITS`].
    TooWeak { bits: usize },
    /// An algorithm this export does not use (ElGamal, a post-quantum hybrid).
    Unsupported(String),
    /// Reading the bundle or writing the message failed.
    Io(String),
}

impl SealError {
    /// The catalogue key the page shows.
    pub fn key(&self) -> &'static str {
        match self {
            SealError::NotAKey => "seal-error-not-a-key",
            SealError::BadSignatures => "seal-error-bad-signatures",
            SealError::Expired => "seal-error-expired",
            SealError::Revoked => "seal-error-revoked",
            SealError::NoEncryptionKey => "seal-error-no-encryption-key",
            SealError::TooWeak { .. } => "seal-error-too-weak",
            SealError::Unsupported(_) => "seal-error-unsupported",
            SealError::Io(_) => "seal-error-io",
        }
    }
}

/// The recipient an export will be encrypted to, as the page describes it.
#[derive(Debug, Clone)]
pub struct Recipient {
    /// The primary key's fingerprint, in groups of four.
    pub fingerprint: String,
    /// The key that will actually encrypt: a subkey, usually.
    pub encryption_fingerprint: String,
    pub user_ids: Vec<String>,
    /// "RSA 3072", "ECDH P-384", "ECDH Curve25519"…
    pub algorithm: String,
    /// In the FIPS-approved set (see the module notes).
    pub fips_approved: bool,
    /// When the encrypting key expires, if it does: `YYYY-MM-DD`.
    pub expires: Option<String>,
    key: SignedPublicKey,
    /// `None`: the primary key encrypts. `Some(i)`: subkey `i` does.
    subkey: Option<usize>,
}

/// Read a recipient from an ASCII-armored public key, and check it is one an
/// export may be made for, now.
pub fn read_recipient(armored: &str) -> Result<Recipient, SealError> {
    read_recipient_at(armored, SystemTime::now())
}

/// [`read_recipient`], at a given moment (for the tests' expired key).
pub fn read_recipient_at(armored: &str, now: SystemTime) -> Result<Recipient, SealError> {
    let text = armored.trim();
    if text.is_empty() || text.len() > MAX_KEY_TEXT {
        return Err(SealError::NotAKey);
    }
    let (key, _) = SignedPublicKey::from_string(text).map_err(|_| SealError::NotAKey)?;
    key.verify_bindings()
        .map_err(|_| SealError::BadSignatures)?;
    if !key.details.revocation_signatures.is_empty() {
        return Err(SealError::Revoked);
    }
    let now_s = now
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let created = |c: pgp::types::Timestamp| u64::from(c.as_secs());
    // The primary key's lifetime: the newest self-signature that states one.
    let primary_sigs = key
        .details
        .direct_signatures
        .iter()
        .chain(key.details.users.iter().flat_map(|u| u.signatures.iter()));
    let newest_primary = primary_sigs
        .clone()
        .max_by_key(|s| s.created().map(|t| t.as_secs()).unwrap_or(0));
    let primary_expiry = newest_primary
        .and_then(|s| s.key_expiration_time())
        // A lifetime of zero is the wire's way of saying "none".
        .filter(|d| d.as_secs() > 0)
        .map(|d| created(key.primary_key.created_at()) + u64::from(d.as_secs()));
    if primary_expiry.is_some_and(|e| e <= now_s) {
        return Err(SealError::Expired);
    }

    // The newest subkey bound for encryption, alive, and not revoked.
    let mut best: Option<(usize, u64, Option<u64>)> = None;
    let mut any_encrypting = false;
    for (i, sub) in key.public_subkeys.iter().enumerate() {
        if !sub.key.algorithm().can_encrypt() {
            continue;
        }
        let revoked = sub
            .signatures
            .iter()
            .any(|s| s.typ() == Some(pgp::packet::SignatureType::SubkeyRevocation));
        let Some(binding) = sub
            .signatures
            .iter()
            .filter(|s| s.typ() == Some(pgp::packet::SignatureType::SubkeyBinding))
            .max_by_key(|s| s.created().map(|t| t.as_secs()).unwrap_or(0))
        else {
            continue;
        };
        let flags = binding.key_flags();
        if !(flags.encrypt_comms() || flags.encrypt_storage()) {
            continue;
        }
        any_encrypting = true;
        if revoked {
            continue;
        }
        let born = created(sub.key.created_at());
        let expiry = binding
            .key_expiration_time()
            .filter(|d| d.as_secs() > 0)
            .map(|d| born + u64::from(d.as_secs()));
        if expiry.is_some_and(|e| e <= now_s) {
            continue;
        }
        if best.is_none_or(|(_, b, _)| born > b) {
            best = Some((i, born, expiry.or(primary_expiry)));
        }
    }
    let (subkey, expiry, params, algorithm) = match best {
        Some((i, _, expiry)) => {
            let k = &key.public_subkeys[i].key;
            (Some(i), expiry, k.public_params().clone(), k.algorithm())
        }
        None => {
            // A primary key may encrypt itself, if its self-signature allows.
            let flags = newest_primary.map(|s| s.key_flags());
            let allowed = flags.is_some_and(|f| f.encrypt_comms() || f.encrypt_storage());
            if allowed && key.primary_key.algorithm().can_encrypt() {
                (
                    None,
                    primary_expiry,
                    key.primary_key.public_params().clone(),
                    key.primary_key.algorithm(),
                )
            } else if any_encrypting {
                return Err(SealError::Expired);
            } else {
                return Err(SealError::NoEncryptionKey);
            }
        }
    };
    let (name, fips) = describe(&params, algorithm)?;
    let encryption_fingerprint = match subkey {
        Some(i) => group(&key.public_subkeys[i].key.fingerprint().to_string()),
        None => group(&key.primary_key.fingerprint().to_string()),
    };
    let user_ids = key
        .details
        .users
        .iter()
        .filter_map(|u| u.id.as_str().map(str::to_string))
        .collect();
    Ok(Recipient {
        fingerprint: group(&key.primary_key.fingerprint().to_string()),
        encryption_fingerprint,
        user_ids,
        algorithm: name,
        fips_approved: fips,
        expires: expiry.map(|e| day(UNIX_EPOCH + Duration::from_secs(e))),
        key,
        subkey,
    })
}

/// What a key's algorithm is called on the page, and whether it is in the
/// FIPS-approved set; or why it is refused.
fn describe(params: &PublicParams, alg: PublicKeyAlgorithm) -> Result<(String, bool), SealError> {
    use pgp::crypto::ecc_curve::ECCCurve;
    match params {
        PublicParams::RSA(rsa) => {
            let bits = rsa.key.n().bits();
            if bits < MIN_RSA_BITS {
                return Err(SealError::TooWeak { bits });
            }
            Ok((format!("RSA {bits}"), bits >= FIPS_RSA_BITS))
        }
        PublicParams::ECDH(ecdh) => {
            let curve = ecdh.curve();
            let fips = matches!(curve, ECCCurve::P256 | ECCCurve::P384 | ECCCurve::P521);
            let name = match curve {
                ECCCurve::P256 => "P-256".to_string(),
                ECCCurve::P384 => "P-384".to_string(),
                ECCCurve::P521 => "P-521".to_string(),
                ECCCurve::Curve25519Legacy => "Curve25519".to_string(),
                other => other.to_string(),
            };
            Ok((format!("ECDH {name}"), fips))
        }
        PublicParams::X25519(_) => Ok(("X25519".to_string(), false)),
        PublicParams::X448(_) => Ok(("X448".to_string(), false)),
        _ => Err(SealError::Unsupported(format!("{alg:?}"))),
    }
}

/// `ABCD1234…` → `ABCD 1234 …`, uppercase.
fn group(hex: &str) -> String {
    let up: Vec<char> = hex
        .chars()
        .filter(|c| c.is_ascii_hexdigit())
        .map(|c| c.to_ascii_uppercase())
        .collect();
    up.chunks(4)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join(" ")
}

fn day(t: SystemTime) -> String {
    let secs = t
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0) as i64;
    time::OffsetDateTime::from_unix_timestamp(secs)
        .map(|d| format!("{:04}-{:02}-{:02}", d.year(), u8::from(d.month()), d.day()))
        .unwrap_or_default()
}

/// Encrypt the file at `input` to `recipient`, into `output`.
///
/// `name` is the file name the message carries inside it, which is what
/// `gpg --decrypt` offers when asked to write the original back.
pub fn seal_file(
    input: &Path,
    output: &Path,
    recipient: &Recipient,
    name: &str,
) -> Result<(), SealError> {
    let reader = BufReader::with_capacity(
        256 * 1024,
        File::open(input).map_err(|e| SealError::Io(e.to_string()))?,
    );
    let out = File::create(output).map_err(|e| SealError::Io(e.to_string()))?;
    let mut writer = BufWriter::with_capacity(256 * 1024, out);
    let mut rng = rand::rngs::OsRng;
    let mut builder = MessageBuilder::from_reader(name.to_string(), reader)
        .seipd_v1(&mut rng, SymmetricKeyAlgorithm::AES256);
    let encrypted = match recipient.subkey {
        Some(i) => {
            let sub: &SignedPublicSubKey = &recipient.key.public_subkeys[i];
            builder.encrypt_to_key(&mut rng, sub)
        }
        None => builder.encrypt_to_key(&mut rng, &recipient.key.primary_key),
    };
    encrypted.map_err(|e| SealError::Io(e.to_string()))?;
    builder
        .to_writer(rng, &mut writer)
        .map_err(|e| SealError::Io(e.to_string()))?;
    writer.flush().map_err(|e| SealError::Io(e.to_string()))?;
    writer
        .into_inner()
        .map_err(|e| SealError::Io(e.to_string()))?
        .sync_all()
        .map_err(|e| SealError::Io(e.to_string()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture(name: &str) -> String {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        std::fs::read_to_string(root.join("tests/fixtures/pgp").join(name)).unwrap()
    }

    #[test]
    fn an_rsa_3072_key_is_read_and_its_encryption_subkey_chosen() {
        let r = read_recipient(&fixture("rsa3072.pub.asc")).unwrap();
        assert_eq!(r.algorithm, "RSA 3072");
        assert!(r.fips_approved);
        assert!(r.subkey.is_some(), "the [E] subkey, not the [SC] primary");
        assert_ne!(r.fingerprint, r.encryption_fingerprint);
        assert_eq!(r.user_ids, vec!["Test RSA 3072 <rsa3072@example.org>"]);
        assert_eq!(r.fingerprint.split(' ').count(), 10, "{}", r.fingerprint);
    }

    #[test]
    fn a_nist_curve_is_in_the_approved_set_and_curve25519_is_not() {
        let p = read_recipient(&fixture("p384.pub.asc")).unwrap();
        assert_eq!(p.algorithm, "ECDH P-384");
        assert!(p.fips_approved);
        let x = read_recipient(&fixture("x25519.pub.asc")).unwrap();
        assert!(x.algorithm.contains("25519"), "{}", x.algorithm);
        assert!(!x.fips_approved);
    }

    #[test]
    fn a_weak_an_expired_and_a_signing_only_key_are_refused() {
        assert!(matches!(
            read_recipient(&fixture("rsa1024.pub.asc")),
            Err(SealError::TooWeak { bits: 1024 })
        ));
        assert_eq!(
            read_recipient(&fixture("expired.pub.asc")).unwrap_err(),
            SealError::Expired
        );
        assert_eq!(
            read_recipient(&fixture("signonly.pub.asc")).unwrap_err(),
            SealError::NoEncryptionKey
        );
        assert_eq!(read_recipient("hello").unwrap_err(), SealError::NotAKey);
        assert_eq!(read_recipient("").unwrap_err(), SealError::NotAKey);
    }

    #[test]
    fn a_secret_key_pasted_by_mistake_is_not_taken_for_a_public_one() {
        assert_eq!(
            read_recipient(&fixture("p384.sec.asc")).unwrap_err(),
            SealError::NotAKey
        );
    }

    #[test]
    fn what_is_sealed_opens_with_the_private_key_and_is_the_same_bytes() {
        use pgp::composed::{Message, SignedSecretKey};
        let dir = crate::scratch::Dir::new("seal");
        std::fs::create_dir_all(&dir).unwrap();
        let plain = dir.join("family.axgf");
        let sealed = dir.join("family.axgf.gpg");
        let body: Vec<u8> = (0..300_000u32).map(|i| (i * 7 % 251) as u8).collect();
        std::fs::write(&plain, &body).unwrap();
        for name in ["p384", "rsa3072"] {
            let r = read_recipient(&fixture(&format!("{name}.pub.asc"))).unwrap();
            seal_file(&plain, &sealed, &r, "family.axgf").unwrap();
            let bytes = std::fs::read(&sealed).unwrap();
            assert_ne!(&bytes[..64], &body[..64]);
            let (secret, _) =
                SignedSecretKey::from_string(&fixture(&format!("{name}.sec.asc"))).unwrap();
            let mut msg = Message::from_bytes(&bytes[..])
                .unwrap()
                .decrypt(&"".into(), &secret)
                .unwrap();
            assert_eq!(msg.as_data_vec().unwrap(), body, "{name}");
        }
    }

    /// GnuPG itself opens what this writes. Run by hand with
    /// `AXGF_SEAL_OUT=<dir> cargo test --lib -- --ignored seal`, then
    /// `gpg --decrypt <dir>/rsa3072.axgf.gpg` against the fixture's secret key.
    #[test]
    #[ignore]
    fn writes_messages_for_gnupg_to_open() {
        let Ok(out) = std::env::var("AXGF_SEAL_OUT") else {
            return;
        };
        let dir = std::path::PathBuf::from(out);
        std::fs::create_dir_all(&dir).unwrap();
        let plain = dir.join("family.axgf");
        std::fs::write(&plain, b"ax-genealogy interop check\n").unwrap();
        for name in ["rsa3072", "p384", "x25519"] {
            let r = read_recipient(&fixture(&format!("{name}.pub.asc"))).unwrap();
            seal_file(
                &plain,
                &dir.join(format!("{name}.axgf.gpg")),
                &r,
                "family.axgf",
            )
            .unwrap();
        }
    }
}
