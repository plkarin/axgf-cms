//! Mail: the relay, the message as it leaves, and its delivery
//! (docs/adr/0048).
//!
//! - **One relay**, set in the administration (`Site::mail`, below) or in the
//!   configuration (`AXGF_CMS_SMTP_URL`, `AXGF_CMS_MAIL_FROM`), which wins.
//! - **Built whole, sent beside the request.** A handler hands an
//!   [`Outgoing`] to the [`Mailer`] and goes on; the mailer sends it in a task
//!   of its own, two at a time, three tries over six minutes. Nothing waits on
//!   SMTP but the administration's test button.
//! - **The transport is a seam.** [`Smtp`] is `lettre`'s blocking client over
//!   rustls, in `spawn_blocking`; [`Recorder`] keeps the messages instead, for
//!   the tests, which can then read what would have gone.

use std::collections::BTreeMap;
use std::fmt;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use serde::{Deserialize, Serialize};

/// How the connection to the relay is secured.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Security {
    /// Plain, then upgraded with STARTTLS before anything is said: port 587,
    /// what Gmail and most providers expect.
    #[default]
    Starttls,
    /// TLS from the first byte: port 465.
    Tls,
    /// None at all — for a relay on this machine only, and refused otherwise.
    None,
}

impl Security {
    pub fn as_str(self) -> &'static str {
        match self {
            Security::Starttls => "starttls",
            Security::Tls => "tls",
            Security::None => "none",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s.trim() {
            "starttls" => Some(Security::Starttls),
            "tls" => Some(Security::Tls),
            "none" => Some(Security::None),
            _ => None,
        }
    }

    /// The port this security usually goes with.
    pub fn default_port(self) -> u16 {
        match self {
            Security::Starttls => 587,
            Security::Tls => 465,
            Security::None => 25,
        }
    }
}

fn is_zero(port: &u16) -> bool {
    *port == 0
}

fn is_starttls(security: &Security) -> bool {
    *security == Security::Starttls
}

/// The relay as the administration stores it, under `site.mail` in the
/// `.acl`. Every field optional on the wire, so an accounts file written
/// before mail reads as no relay at all.
#[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub host: String,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub port: u16,
    #[serde(default, skip_serializing_if = "is_starttls")]
    pub security: Security,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub username: String,
    /// The relay's password — an app password, for Gmail. A secret of the
    /// same kind as a sign-in service's: shown as set or unset, never sent
    /// back to a page, never written to the journal.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub password: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub from_address: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub from_name: String,
    /// Where replies go, if not to the sender: the site never reads them.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reply_to: String,
    /// What mail is written in for an account with no language of its own.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub language: String,
    /// Messages handed to the relay in a day, at most; 0 means the default.
    #[serde(default, skip_serializing_if = "is_zero_u32")]
    pub daily_limit: u32,
}

fn is_zero_u32(n: &u32) -> bool {
    *n == 0
}

impl Settings {
    /// The day's ceiling in force.
    pub fn ceiling(&self) -> u32 {
        if self.daily_limit == 0 {
            DEFAULT_CEILING
        } else {
            self.daily_limit
        }
    }

    pub fn is_default(&self) -> bool {
        *self == Self::default()
    }
}

/// `Site` derives `Debug`; the password must not ride along with it.
impl fmt::Debug for Settings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Settings")
            .field("host", &self.host)
            .field("port", &self.port)
            .field("security", &self.security)
            .field("username", &self.username)
            .field("password_set", &!self.password.is_empty())
            .field("from_address", &self.from_address)
            .field("from_name", &self.from_name)
            .field("reply_to", &self.reply_to)
            .field("language", &self.language)
            .field("daily_limit", &self.daily_limit)
            .finish()
    }
}

/// The relay a message goes through, wherever it was set.
#[derive(Clone, PartialEq)]
pub struct Relay {
    pub host: String,
    pub port: u16,
    pub security: Security,
    pub username: String,
    pub password: String,
    pub from_address: String,
    pub from_name: String,
    /// Where replies go, if anywhere but the sender.
    pub reply_to: String,
    /// Set by the server's configuration rather than the administration.
    pub from_env: bool,
}

impl fmt::Debug for Relay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Relay")
            .field("host", &self.host)
            .field("port", &self.port)
            .field("security", &self.security)
            .field("username", &self.username)
            .field("password_set", &!self.password.is_empty())
            .field("from_address", &self.from_address)
            .field("from_env", &self.from_env)
            .finish()
    }
}

/// Why a relay cannot be used.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Refused {
    /// No host.
    NoHost,
    /// No sender, or one that is not an address.
    NoSender,
    /// No security, to a relay that is not this machine.
    PlainElsewhere,
    /// The configuration's address does not read.
    BadUrl,
    /// 465 with STARTTLS, or 587 with TLS: each port speaks the other.
    PortMismatch,
}

impl Refused {
    /// The catalogue key that says so.
    pub fn key(self) -> &'static str {
        match self {
            Refused::NoHost => "mail-refused-host",
            Refused::NoSender => "mail-refused-sender",
            Refused::PlainElsewhere => "mail-refused-plain",
            Refused::BadUrl => "mail-refused-url",
            Refused::PortMismatch => "mail-port-mismatch",
        }
    }
}

/// Whether `host` is this machine, where a relay without TLS is acceptable.
fn is_local(host: &str) -> bool {
    let h = host.trim().trim_start_matches('[').trim_end_matches(']');
    h.eq_ignore_ascii_case("localhost")
        || h.parse::<std::net::IpAddr>()
            .is_ok_and(|ip| ip.is_loopback())
}

impl Relay {
    /// The administration's relay, if it is complete enough to try.
    pub fn from_settings(s: &Settings) -> Result<Relay, Refused> {
        let relay = Relay {
            host: s.host.trim().to_string(),
            port: if s.port == 0 {
                s.security.default_port()
            } else {
                s.port
            },
            security: s.security,
            username: s.username.trim().to_string(),
            password: s.password.clone(),
            from_address: s.from_address.trim().to_string(),
            from_name: s.from_name.trim().to_string(),
            reply_to: s.reply_to.trim().to_string(),
            from_env: false,
        };
        relay.check()?;
        Ok(relay)
    }

    /// The configuration's relay: `AXGF_CMS_SMTP_URL` —
    /// `smtp://user:password@host:587` for STARTTLS, `smtps://…:465` for TLS,
    /// `…?security=none` for a relay on this machine — and
    /// `AXGF_CMS_MAIL_FROM`, `Name <address>` or a bare address, defaulting to
    /// the user name when that is an address (as Gmail's is). A user name with
    /// an `@` in it is written `%40` in the URL.
    pub fn from_env(url: &str, from: Option<&str>) -> Result<Relay, Refused> {
        let url = url.trim();
        let (scheme, rest) = url.split_once("://").ok_or(Refused::BadUrl)?;
        let mut security = match scheme.to_ascii_lowercase().as_str() {
            "smtp" => Security::Starttls,
            "smtps" => Security::Tls,
            _ => return Err(Refused::BadUrl),
        };
        let (rest, query) = match rest.split_once('?') {
            Some((r, q)) => (r, Some(q)),
            None => (rest, None),
        };
        if let Some(q) = query {
            for pair in q.split('&') {
                match pair.split_once('=') {
                    Some(("security", v)) => {
                        security = Security::parse(v).ok_or(Refused::BadUrl)?;
                    }
                    _ => return Err(Refused::BadUrl),
                }
            }
        }
        let rest = rest.trim_end_matches('/');
        let (userinfo, hostport) = match rest.rsplit_once('@') {
            Some((u, h)) => (Some(u), h),
            None => (None, rest),
        };
        let (username, password) = match userinfo {
            Some(u) => match u.split_once(':') {
                Some((n, p)) => (decode(n)?, decode(p)?),
                None => (decode(u)?, String::new()),
            },
            None => (String::new(), String::new()),
        };
        let (host, port) = if let Some(h) = hostport.strip_prefix('[') {
            // [::1]:25
            let (h, after) = h.split_once(']').ok_or(Refused::BadUrl)?;
            let port = match after.strip_prefix(':') {
                Some(p) => p.parse().map_err(|_| Refused::BadUrl)?,
                None => security.default_port(),
            };
            (h.to_string(), port)
        } else {
            match hostport.rsplit_once(':') {
                Some((h, p)) => (h.to_string(), p.parse().map_err(|_| Refused::BadUrl)?),
                None => (hostport.to_string(), security.default_port()),
            }
        };
        let (from_name, from_address) = match from.map(str::trim).filter(|f| !f.is_empty()) {
            Some(f) => split_mailbox(f),
            None => (String::new(), username.clone()),
        };
        let relay = Relay {
            host,
            port,
            security,
            username,
            password: password.split_whitespace().collect(),
            from_address,
            from_name,
            reply_to: String::new(),
            from_env: true,
        };
        relay.check()?;
        Ok(relay)
    }

    /// What may be tried: a host, a sender that reads as an address, and TLS
    /// unless the relay is this machine.
    pub fn check(&self) -> Result<(), Refused> {
        if self.host.is_empty() || self.host.contains(char::is_whitespace) {
            return Err(Refused::NoHost);
        }
        if !valid_address(&self.from_address) {
            return Err(Refused::NoSender);
        }
        if !self.reply_to.is_empty() && !valid_address(&self.reply_to) {
            return Err(Refused::NoSender);
        }
        if self.security == Security::None && !is_local(&self.host) {
            return Err(Refused::PlainElsewhere);
        }
        if (self.port == 465 && self.security == Security::Starttls)
            || (self.port == 587 && self.security == Security::Tls)
        {
            return Err(Refused::PortMismatch);
        }
        Ok(())
    }
}

/// `%40` and friends, in a URL's user name or password.
fn decode(s: &str) -> Result<String, Refused> {
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' {
            let hex = s.get(i + 1..i + 3).ok_or(Refused::BadUrl)?;
            out.push(u8::from_str_radix(hex, 16).map_err(|_| Refused::BadUrl)?);
            i += 3;
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    String::from_utf8(out).map_err(|_| Refused::BadUrl)
}

/// `Name <address>` into its two halves; a bare address has no name.
fn split_mailbox(s: &str) -> (String, String) {
    match s.rsplit_once('<') {
        Some((name, addr)) => (
            name.trim().trim_matches('"').trim().to_string(),
            addr.trim_end_matches('>').trim().to_string(),
        ),
        None => (String::new(), s.trim().to_string()),
    }
}

/// Whether `s` reads as one mail address — and only that: no name, no
/// spaces, no line breaks to smuggle a header in.
pub fn valid_address(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 254
        && !s.chars().any(|c| c.is_whitespace() || c.is_control())
        && s.parse::<lettre::Address>().is_ok()
}

/// An address as the journal may write it: `a…@example.org`.
pub fn mask(address: &str) -> String {
    match address.split_once('@') {
        Some((local, domain)) => {
            let first: String = local.chars().take(1).collect();
            format!("{first}…@{domain}")
        }
        None => "…".to_string(),
    }
}

/// A message, whole, before it is handed to the relay.
#[derive(Debug, Clone)]
pub struct Outgoing {
    /// One address.
    pub to: String,
    pub subject: String,
    pub text: String,
    pub html: String,
    /// What kind of message, for the journal: `reset`, `welcome`, …
    pub kind: &'static str,
    /// Where one click stops messages of this kind: `List-Unsubscribe`.
    pub unsubscribe: Option<String>,
    /// The language it was written in, for the tests.
    pub lang: String,
}

/// What went wrong on the way to the relay, sorted by what to do about it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Failure {
    /// Worth trying again: a 4xx reply, a timeout, a refused connection.
    Transient(String),
    /// The relay refused the credentials. Nothing more is tried until the
    /// relay is saved again or the test gets through.
    Auth(String),
    /// The recipient was refused for good (550 5.1.x).
    Recipient(String),
    /// The relay's own daily limit (Gmail's 550 5.4.5): nothing more today.
    Quota(String),
    /// The service's sandbox refused the connection: the drop-in that opens
    /// outgoing connections is missing.
    Sandbox(String),
    /// TLS did not come up: 465 goes with TLS, 587 with STARTTLS.
    Tls(String),
    /// Anything else, not tried again.
    Other(String),
}

impl Failure {
    /// What the relay or the system said, addresses taken out.
    pub fn detail(&self) -> String {
        let raw = match self {
            Failure::Transient(d)
            | Failure::Auth(d)
            | Failure::Recipient(d)
            | Failure::Quota(d)
            | Failure::Sandbox(d)
            | Failure::Tls(d)
            | Failure::Other(d) => d,
        };
        scrub(raw)
    }

    /// Which kind, as the ledger and the catalogue name it.
    pub fn kind(&self) -> &'static str {
        match self {
            Failure::Transient(_) => "transient",
            Failure::Auth(_) => "auth",
            Failure::Recipient(_) => "recipient",
            Failure::Quota(_) => "quota",
            Failure::Sandbox(_) => "sandbox",
            Failure::Tls(_) => "tls",
            Failure::Other(_) => "other",
        }
    }

    fn worth_retrying(&self) -> bool {
        matches!(self, Failure::Transient(_))
    }
}

/// A relay's reply or an error, with anything that looks like an address
/// replaced: the service log keeps what went wrong, not who it went to.
pub fn scrub(s: &str) -> String {
    s.split_whitespace()
        .map(|w| if w.contains('@') { "…" } else { w })
        .collect::<Vec<_>>()
        .join(" ")
}

/// The enhanced status code a relay's reply opens with — `5.4.5` in
/// "permanent error (550): 5.4.5 Daily user sending quota exceeded" — or
/// nothing. Only that code is read: the free text after it may quote
/// anything, an address included.
fn enhanced(text: &str) -> &str {
    text.split_once("): ")
        .map_or("", |(_, rest)| rest)
        .split_whitespace()
        .next()
        .unwrap_or("")
}

/// Sorts `lettre`'s error into a [`Failure`].
fn classify(e: &lettre::transport::smtp::Error) -> Failure {
    use std::io::ErrorKind as K;
    let text = e.to_string();
    let mut source: Option<&(dyn std::error::Error + 'static)> = std::error::Error::source(e);
    while let Some(err) = source {
        if let Some(io) = err.downcast_ref::<std::io::Error>() {
            match io.kind() {
                // The system's own refusal: the unit's IPAddressDeny answers
                // EPERM.
                K::PermissionDenied => return Failure::Sandbox(text),
                // rustls says a handshake failed — a certificate, or TLS
                // spoken to a port that waits for STARTTLS — as data it could
                // not read.
                K::InvalidData => return Failure::Tls(text),
                // The network, for now. A read past lettre's timeout is
                // EAGAIN on Linux, which reads as WouldBlock, not TimedOut.
                K::ConnectionRefused
                | K::ConnectionReset
                | K::ConnectionAborted
                | K::NotConnected
                | K::BrokenPipe
                | K::UnexpectedEof
                | K::TimedOut
                | K::WouldBlock
                | K::Interrupted
                | K::NetworkUnreachable
                | K::NetworkDown
                | K::HostUnreachable
                | K::AddrNotAvailable => return Failure::Transient(text),
                _ => {}
            }
        }
        source = err.source();
    }
    if e.is_timeout() || e.is_transient() {
        return Failure::Transient(text);
    }
    if e.is_tls() {
        return Failure::Tls(text);
    }
    let code = e.status().map(|c| c.to_string()).unwrap_or_default();
    if e.is_permanent() {
        if matches!(code.as_str(), "530" | "534" | "535") {
            return Failure::Auth(text);
        }
        let enhanced = enhanced(&text);
        if enhanced.starts_with("5.4.5") {
            return Failure::Quota(text);
        }
        if enhanced.starts_with("5.1.") {
            return Failure::Recipient(text);
        }
        return Failure::Other(text);
    }
    if text.contains("Operation not permitted") {
        return Failure::Sandbox(text);
    }
    // The conversation cut short ("incomplete response"), or a name that did
    // not resolve this time: worth another try.
    if e.is_response() || text.starts_with("Connection error") {
        return Failure::Transient(text);
    }
    Failure::Other(text)
}

/// Where a message goes. `Ok` carries the relay's reply. `timeout` bounds
/// each wait on the relay: the queue's is long, the test's short enough that
/// the relay's real answer comes back within the page's wait.
pub trait Transport: Send + Sync {
    fn send(&self, relay: &Relay, message: &Outgoing, timeout: Duration)
        -> Result<String, Failure>;
}

/// `List-Unsubscribe: <url>`.
#[derive(Clone)]
struct ListUnsubscribe(String);

impl lettre::message::header::Header for ListUnsubscribe {
    fn name() -> lettre::message::header::HeaderName {
        lettre::message::header::HeaderName::new_from_ascii_str("List-Unsubscribe")
    }
    fn parse(s: &str) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        Ok(Self(s.to_string()))
    }
    fn display(&self) -> lettre::message::header::HeaderValue {
        lettre::message::header::HeaderValue::new(Self::name(), format!("<{}>", self.0))
    }
}

/// `Auto-Submitted: auto-generated` (RFC 3834): a machine wrote this, so a
/// machine should not answer it.
#[derive(Clone)]
struct AutoSubmitted;

impl lettre::message::header::Header for AutoSubmitted {
    fn name() -> lettre::message::header::HeaderName {
        lettre::message::header::HeaderName::new_from_ascii_str("Auto-Submitted")
    }
    fn parse(_: &str) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        Ok(Self)
    }
    fn display(&self) -> lettre::message::header::HeaderValue {
        lettre::message::header::HeaderValue::new(Self::name(), "auto-generated".to_string())
    }
}

/// `List-Unsubscribe-Post: List-Unsubscribe=One-Click` (RFC 8058).
#[derive(Clone)]
struct ListUnsubscribePost;

impl lettre::message::header::Header for ListUnsubscribePost {
    fn name() -> lettre::message::header::HeaderName {
        lettre::message::header::HeaderName::new_from_ascii_str("List-Unsubscribe-Post")
    }
    fn parse(_: &str) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        Ok(Self)
    }
    fn display(&self) -> lettre::message::header::HeaderValue {
        lettre::message::header::HeaderValue::new(
            Self::name(),
            "List-Unsubscribe=One-Click".to_string(),
        )
    }
}

/// The message as it goes on the wire: a text part, and an HTML part with
/// the logo beside it.
pub fn to_mime(relay: &Relay, m: &Outgoing) -> Result<lettre::Message, String> {
    use lettre::message::header::ContentType;
    use lettre::message::{Attachment, Mailbox, MultiPart, SinglePart};
    let from_addr: lettre::Address = relay
        .from_address
        .parse()
        .map_err(|e: lettre::address::AddressError| e.to_string())?;
    let from = Mailbox::new(
        Some(relay.from_name.clone()).filter(|n| !n.is_empty()),
        from_addr,
    );
    let to: Mailbox =
        m.to.parse::<lettre::Address>()
            .map(|a| Mailbox::new(None, a))
            .map_err(|e| e.to_string())?;
    // A Message-ID of the sender's domain: lettre adds none of itself, and a
    // relay that does not either has the message refused by Gmail's servers
    // ("missing a valid Message-ID") or taken for spam.
    let domain = relay
        .from_address
        .rsplit_once('@')
        .map_or("localhost", |(_, d)| d);
    let mut builder = lettre::Message::builder()
        .message_id(Some(format!(
            "<{}@{domain}>",
            uuid::Uuid::new_v4().simple()
        )))
        .from(from)
        .to(to)
        .subject(m.subject.clone())
        .header(AutoSubmitted);
    if !relay.reply_to.is_empty() {
        let reply: lettre::Address = relay
            .reply_to
            .parse()
            .map_err(|e: lettre::address::AddressError| e.to_string())?;
        builder = builder.reply_to(Mailbox::new(None, reply));
    }
    if let Some(url) = &m.unsubscribe {
        builder = builder
            .header(ListUnsubscribe(url.clone()))
            .header(ListUnsubscribePost);
    }
    let png = ContentType::parse("image/png").map_err(|e| e.to_string())?;
    let body = MultiPart::alternative()
        .singlepart(SinglePart::plain(m.text.clone()))
        .multipart(
            MultiPart::related()
                .singlepart(SinglePart::html(m.html.clone()))
                .singlepart(
                    Attachment::new_inline("logo".to_string())
                        .body(crate::render::ICON_192_PNG.to_vec(), png),
                ),
        );
    builder.multipart(body).map_err(|e| e.to_string())
}

/// The real thing: `lettre`'s SMTP client, blocking — call it off the async
/// workers. `starttls_relay` requires STARTTLS; the relay's certificate is
/// checked against webpki-roots and its name.
pub struct Smtp;

impl Transport for Smtp {
    fn send(&self, relay: &Relay, m: &Outgoing, timeout: Duration) -> Result<String, Failure> {
        use lettre::transport::smtp::authentication::Credentials;
        use lettre::{SmtpTransport, Transport as _};
        relay
            .check()
            .map_err(|r| Failure::Other(format!("{r:?}")))?;
        let message = to_mime(relay, m).map_err(Failure::Other)?;
        let builder = match relay.security {
            Security::Starttls => {
                SmtpTransport::starttls_relay(&relay.host).map_err(|e| classify(&e))?
            }
            Security::Tls => SmtpTransport::relay(&relay.host).map_err(|e| classify(&e))?,
            Security::None => SmtpTransport::builder_dangerous(&relay.host),
        };
        let mut builder = builder.port(relay.port).timeout(Some(timeout));
        if !relay.username.is_empty() {
            builder = builder.credentials(Credentials::new(
                relay.username.clone(),
                relay.password.clone(),
            ));
        }
        builder
            .build()
            .send(&message)
            .map(|r| {
                let words: Vec<String> = r.message().map(str::to_string).collect();
                format!("{} {}", r.code(), words.join(" "))
            })
            .map_err(|e| classify(&e))
    }
}

/// For the tests: keeps every message, sends none.
#[derive(Clone, Default)]
pub struct Recorder {
    sent: Arc<Mutex<Vec<Outgoing>>>,
}

impl Recorder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Everything handed over so far.
    pub fn sent(&self) -> Vec<Outgoing> {
        self.sent.lock().unwrap_or_else(|e| e.into_inner()).clone()
    }
}

impl Transport for Recorder {
    fn send(&self, relay: &Relay, m: &Outgoing, _: Duration) -> Result<String, Failure> {
        // Built as it would go, so a message the wire would refuse fails here.
        to_mime(relay, m).map_err(Failure::Other)?;
        self.sent
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .push(m.clone());
        Ok("250 2.0.0 kept by the test recorder".to_string())
    }
}

/// One attempt, as the administration sees it.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Attempt {
    /// RFC 3339.
    pub at: String,
    pub ok: bool,
    /// What kind of message: `reset`, `welcome`, `test`, …
    pub message: String,
    /// The failure's kind (`auth`, `sandbox`, …), empty when it went.
    #[serde(default)]
    pub failure: String,
    /// The relay's reply or the error, addresses taken out.
    pub detail: String,
}

/// What the mailer remembers between restarts, in `family.mailstate`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ledger {
    /// The UTC day `today` counts.
    #[serde(default)]
    pub day: String,
    /// Messages handed to the relay that day.
    #[serde(default)]
    pub today: u32,
    #[serde(default)]
    pub last_ok: Option<Attempt>,
    #[serde(default)]
    pub last_failure: Option<Attempt>,
    /// Each attempt's time and outcome over the last day, for the counts.
    #[serde(default)]
    pub recent: Vec<(String, bool)>,
    /// Why nothing is sent, if nothing is: `auth` until the relay is saved
    /// again or the test gets through, `quota` until the next day.
    #[serde(default)]
    pub paused: Option<String>,
    /// The UTC day a `quota` pause ends.
    #[serde(default)]
    pub paused_until: String,
    #[serde(default)]
    pub failures_in_a_row: u32,
    /// Each account's last message, by account id.
    #[serde(default)]
    pub accounts: BTreeMap<String, Attempt>,
}

/// Why a message was not handed over.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Held {
    /// The day's ceiling is reached.
    Ceiling,
    /// The mailer is paused (credentials refused, or the relay's own limit).
    Paused,
    /// Too many messages are waiting already.
    Full,
}

/// Messages sent at once, at most.
const SLOTS: usize = 2;

/// Messages waiting at most; past it, a new one is dropped with a log line.
const QUEUE: usize = 200;

/// How long to wait before each try of a temporary failure: at once, after a
/// minute, after five.
const TRIES: [Duration; 3] = [
    Duration::ZERO,
    Duration::from_secs(60),
    Duration::from_secs(300),
];

/// Messages handed to the relay in a day, unless the section says otherwise:
/// under consumer Gmail's few hundred.
pub const DEFAULT_CEILING: u32 = 300;

/// The UTC day of an RFC 3339 time.
fn day_of(at: &str) -> String {
    at.get(..10).unwrap_or_default().to_string()
}

/// How long one wait on the relay may last, in the queue and in the test.
const QUEUE_TIMEOUT: Duration = Duration::from_secs(30);
const TEST_TIMEOUT: Duration = Duration::from_secs(10);

/// A limit: a key — an account's, an address's, a client's — and how many
/// it may have in a window.
pub type Limit = (String, usize, Duration);

/// Counted things that are not worth a file: messages per account and per
/// recipient, asks per client, over a window.
#[derive(Default)]
struct Counts {
    hits: std::collections::HashMap<String, Vec<std::time::Instant>>,
}

impl Counts {
    fn sweep(&mut self, now: std::time::Instant) {
        let day = Duration::from_secs(86_400);
        if self.hits.len() > 4096 {
            self.hits
                .retain(|_, v| v.iter().any(|t| now.duration_since(*t) < day));
        }
    }

    fn count(&self, key: &str, window: Duration, now: std::time::Instant) -> usize {
        self.hits.get(key).map_or(0, |v| {
            v.iter()
                .filter(|t| now.duration_since(**t) < window)
                .count()
        })
    }

    /// Whether every limit has room for one more; if so, all are counted at
    /// once, and none if not.
    fn take_all(&mut self, limits: &[Limit]) -> bool {
        let now = std::time::Instant::now();
        self.sweep(now);
        if limits
            .iter()
            .any(|(k, max, window)| self.count(k, *window, now) >= *max)
        {
            return false;
        }
        for (k, ..) in limits {
            let v = self.hits.entry(k.clone()).or_default();
            v.retain(|t| now.duration_since(*t) < Duration::from_secs(86_400));
            v.push(now);
        }
        true
    }

    /// Give back what [`Self::take_all`] counted, for a message that was not
    /// handed over after all.
    fn refund(&mut self, limits: &[Limit]) {
        for (k, ..) in limits {
            if let Some(v) = self.hits.get_mut(k) {
                v.pop();
            }
        }
    }
}

/// The ledger and the file it is kept in, shared with the tasks that send.
#[derive(Default)]
struct Book {
    ledger: Mutex<Ledger>,
    file: Option<std::path::PathBuf>,
    /// Each change's number, taken under the ledger's lock…
    changes: std::sync::atomic::AtomicU64,
    /// …and the last one written, so that an older snapshot never replaces a
    /// newer one on disk.
    written: Mutex<u64>,
}

impl Book {
    fn snapshot(&self) -> Ledger {
        self.ledger
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .clone()
    }

    fn paused(&self) -> bool {
        let today = day_of(&crate::view::now_iso8601());
        let l = self.ledger.lock().unwrap_or_else(|e| e.into_inner());
        l.paused.is_some() && !quota_over(&l, &today)
    }

    /// Change the ledger with `f`, which says whether it changed anything,
    /// and keep it if it did.
    fn change<T>(&self, f: impl FnOnce(&mut Ledger) -> (T, bool)) -> T {
        let (out, kept) = {
            let mut l = self.ledger.lock().unwrap_or_else(|e| e.into_inner());
            let (out, changed) = f(&mut l);
            let kept = changed.then(|| {
                let n = self
                    .changes
                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
                    + 1;
                (n, serde_json::to_vec_pretty(&*l).unwrap_or_default())
            });
            (out, kept)
        };
        if let (Some((n, bytes)), Some(path)) = (kept, &self.file) {
            let mut last = self.written.lock().unwrap_or_else(|e| e.into_inner());
            if n > *last {
                match crate::brain::write_private(path, &bytes) {
                    Ok(()) => *last = n,
                    Err(e) => tracing::warn!(error = %e, "could not keep the mail ledger"),
                }
            }
        }
        out
    }

    /// Write an attempt down, pausing on a refusal of the credentials or the
    /// relay's own limit.
    fn settle(&self, account: Option<&str>, kind: &str, result: &Result<String, Failure>) {
        let at = crate::view::now_iso8601();
        let attempt = Attempt {
            at: at.clone(),
            ok: result.is_ok(),
            message: kind.to_string(),
            failure: result
                .as_ref()
                .err()
                .map(|f| f.kind().to_string())
                .unwrap_or_default(),
            detail: match result {
                Ok(reply) => scrub(reply),
                Err(f) => f.detail(),
            },
        };
        self.change(|l| {
            l.recent.retain(|(t, _)| within_a_day(t, &at));
            l.recent.push((at.clone(), attempt.ok));
            if l.recent.len() > 2000 {
                let excess = l.recent.len() - 2000;
                l.recent.drain(..excess);
            }
            match result {
                Ok(_) => {
                    l.failures_in_a_row = 0;
                    l.last_ok = Some(attempt.clone());
                }
                Err(f) => {
                    l.failures_in_a_row += 1;
                    l.last_failure = Some(attempt.clone());
                    match f {
                        Failure::Auth(_) => l.paused = Some("auth".to_string()),
                        Failure::Quota(_) => {
                            l.paused = Some("quota".to_string());
                            l.paused_until = next_day(&at);
                        }
                        _ => {}
                    }
                }
            }
            if let Some(id) = account {
                l.accounts.insert(id.to_string(), attempt);
            }
            ((), true)
        });
    }
}

/// Whether a pause for the relay's own daily limit is over by `today`.
fn quota_over(l: &Ledger, today: &str) -> bool {
    l.paused.as_deref() == Some("quota")
        && !l.paused_until.is_empty()
        && l.paused_until.as_str() <= today
}

/// The mailer the server holds.
pub struct Mailer {
    transport: Arc<dyn Transport>,
    /// The configuration's relay, which wins over the administration's.
    env: Option<Result<Relay, Refused>>,
    /// `AXGF_CMS_MAIL_FROM` was set without `AXGF_CMS_SMTP_URL`.
    from_without_url: bool,
    slots: Arc<tokio::sync::Semaphore>,
    book: Arc<Book>,
    counts: Mutex<Counts>,
    waiting: Arc<std::sync::atomic::AtomicUsize>,
}

impl Default for Mailer {
    fn default() -> Self {
        Self::new(Arc::new(Smtp))
    }
}

impl Mailer {
    pub fn new(transport: Arc<dyn Transport>) -> Self {
        Mailer {
            transport,
            env: None,
            from_without_url: false,
            slots: Arc::new(tokio::sync::Semaphore::new(SLOTS)),
            book: Arc::new(Book::default()),
            counts: Mutex::new(Counts::default()),
            waiting: Arc::new(std::sync::atomic::AtomicUsize::new(0)),
        }
    }

    /// Keep the ledger in `path` (`family.mailstate`), reading what is there.
    /// A missing or unreadable file is a fresh ledger.
    pub fn with_ledger_file(mut self, path: std::path::PathBuf) -> Self {
        let read = std::fs::read(&path)
            .ok()
            .and_then(|b| serde_json::from_slice::<Ledger>(&b).ok())
            .unwrap_or_default();
        self.book = Arc::new(Book {
            ledger: Mutex::new(read),
            file: Some(path),
            ..Book::default()
        });
        self
    }

    /// The same mailer with another transport, for tests.
    pub fn with_transport(mut self, transport: Arc<dyn Transport>) -> Self {
        self.transport = transport;
        self
    }

    /// The relay the configuration sets, if it sets one: `url` from
    /// `AXGF_CMS_SMTP_URL`, `from` from `AXGF_CMS_MAIL_FROM`.
    pub fn with_env(mut self, url: Option<&str>, from: Option<&str>) -> Self {
        let url = url.map(str::trim).filter(|u| !u.is_empty());
        self.from_without_url = url.is_none() && from.is_some_and(|f| !f.trim().is_empty());
        self.env = url.map(|u| Relay::from_env(u, from));
        if let Some(Err(r)) = &self.env {
            tracing::warn!(refused = ?r, "AXGF_CMS_SMTP_URL does not read; mail stays off");
        }
        self
    }

    /// Whether the configuration sets the relay — the administration's
    /// relay fields are then for show.
    pub fn set_by_env(&self) -> bool {
        self.env.is_some()
    }

    /// `AXGF_CMS_MAIL_FROM` set alone, and so ignored.
    pub fn from_without_url(&self) -> bool {
        self.from_without_url
    }

    /// Why the configuration's relay cannot be used, if it is set and cannot.
    pub fn env_refused(&self) -> Option<Refused> {
        match &self.env {
            Some(Err(r)) => Some(*r),
            _ => None,
        }
    }

    /// The configuration's relay as it reads, for the administration to show
    /// without its password.
    pub fn env_relay(&self) -> Option<&Relay> {
        match &self.env {
            Some(Ok(r)) => Some(r),
            _ => None,
        }
    }

    /// The relay in force: the configuration's — with the section's reply
    /// address, which the configuration does not set — else the
    /// administration's.
    pub fn relay(&self, site: &crate::site::Site) -> Option<Relay> {
        match &self.env {
            Some(r) => r.clone().ok().map(|mut r| {
                if valid_address(site.mail.reply_to.trim()) {
                    r.reply_to = site.mail.reply_to.trim().to_string();
                }
                r
            }),
            None => Relay::from_settings(&site.mail).ok(),
        }
    }

    /// What the mailer remembers, as it stands now: a pause for the relay's
    /// own limit is over once its day has come, whether or not a message has
    /// been asked for since.
    pub fn ledger(&self) -> Ledger {
        let mut l = self.book.snapshot();
        let now = crate::view::now_iso8601();
        if quota_over(&l, &day_of(&now)) {
            l.paused = None;
        }
        l.recent.retain(|(at, _)| within_a_day(at, &now));
        l
    }

    /// Messages handed over and not yet settled.
    pub fn waiting(&self) -> usize {
        self.waiting.load(std::sync::atomic::Ordering::Relaxed)
    }

    /// Lift a pause for refused credentials — the relay was saved again —
    /// and forget the failures in a row, which were the old settings'. A
    /// pause for the relay's own daily limit stays: saving changes nothing
    /// for it.
    pub fn resume(&self) {
        self.book.change(|l| {
            let auth = l.paused.as_deref() == Some("auth");
            let changed = auth || l.failures_in_a_row > 0;
            if auth {
                l.paused = None;
            }
            l.failures_in_a_row = 0;
            ((), changed)
        });
    }

    /// Lift any pause: a test got through, or the relay is another one.
    pub fn resume_all(&self) {
        self.book.change(|l| {
            let changed = l.paused.is_some() || l.failures_in_a_row > 0;
            l.paused = None;
            l.paused_until.clear();
            l.failures_in_a_row = 0;
            ((), changed)
        });
    }

    /// Whether every limit — an account's, an address's, a client's — has
    /// room for one more; if so all are counted, at once, and none if not.
    pub fn take_all(&self, limits: &[Limit]) -> bool {
        self.counts
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .take_all(limits)
    }

    /// Give back what [`Self::take_all`] counted, for a message that was not
    /// handed over after all.
    pub fn refund(&self, limits: &[Limit]) {
        self.counts
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .refund(limits);
    }

    /// Whether a message may be handed over now, counting it if so.
    fn admit(&self, ceiling: u32) -> Result<(), Held> {
        let today = day_of(&crate::view::now_iso8601());
        let full = self.waiting.load(std::sync::atomic::Ordering::Relaxed) >= QUEUE;
        self.book.change(|l| {
            let mut changed = false;
            if quota_over(l, &today) {
                l.paused = None;
                l.paused_until.clear();
                changed = true;
            }
            if l.paused.is_some() {
                return (Err(Held::Paused), changed);
            }
            if l.day != today {
                l.day = today.clone();
                l.today = 0;
                changed = true;
            }
            if l.today >= ceiling.max(1) {
                return (Err(Held::Ceiling), changed);
            }
            if full {
                return (Err(Held::Full), changed);
            }
            l.today += 1;
            (Ok(()), true)
        })
    }

    async fn attempt(
        transport: Arc<dyn Transport>,
        slots: Option<Arc<tokio::sync::Semaphore>>,
        relay: Relay,
        message: Outgoing,
        timeout: Duration,
    ) -> Result<String, Failure> {
        let _slot = match slots {
            Some(s) => Some(
                s.acquire_owned()
                    .await
                    .map_err(|e| Failure::Other(e.to_string()))?,
            ),
            None => None,
        };
        tokio::task::spawn_blocking(move || transport.send(&relay, &message, timeout))
            .await
            .unwrap_or_else(|e| Err(Failure::Other(e.to_string())))
    }

    /// Hand a message over for `account` (an id, for the accounts page): sent
    /// in a task of its own, a temporary failure tried again after one and
    /// five minutes — unless mail has been paused meanwhile — the outcome
    /// written to the ledger and to the service log with the address masked.
    /// `ceiling` is the section's daily ceiling.
    pub fn send_later(
        &self,
        relay: Relay,
        message: Outgoing,
        account: Option<String>,
        ceiling: u32,
    ) -> Result<(), Held> {
        if let Err(held) = self.admit(ceiling) {
            tracing::warn!(kind = message.kind, account = account.as_deref().unwrap_or("-"), held = ?held, "mail not handed to the relay");
            return Err(held);
        }
        let transport = Arc::clone(&self.transport);
        let slots = Arc::clone(&self.slots);
        let book = Arc::clone(&self.book);
        let waiting = Arc::clone(&self.waiting);
        waiting.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        tokio::spawn(async move {
            let to = mask(&message.to);
            let who = account.clone().unwrap_or_else(|| "-".to_string());
            for (i, wait) in TRIES.iter().enumerate() {
                if !wait.is_zero() {
                    tokio::time::sleep(*wait).await;
                }
                // Paused while it waited: another try would be refused the
                // same way, and counted against the account at Gmail.
                if book.paused() {
                    tracing::warn!(kind = message.kind, account = %who, to = %to, "mail to {to} not sent: mail is paused");
                    break;
                }
                let result = Self::attempt(
                    Arc::clone(&transport),
                    Some(Arc::clone(&slots)),
                    relay.clone(),
                    message.clone(),
                    QUEUE_TIMEOUT,
                )
                .await;
                book.settle(account.as_deref(), message.kind, &result);
                match &result {
                    Ok(_) => {
                        tracing::info!(kind = message.kind, account = %who, to = %to, "mail accepted by the relay");
                        break;
                    }
                    Err(f) if f.worth_retrying() && i + 1 < TRIES.len() => {
                        tracing::info!(kind = message.kind, account = %who, to = %to, failure = f.kind(), detail = %f.detail(), "mail not accepted yet; will try again");
                    }
                    Err(f) => {
                        tracing::warn!(kind = message.kind, account = %who, to = %to, failure = f.kind(), detail = %f.detail(), "mail to {to} not sent ({})", f.kind());
                        break;
                    }
                }
            }
            waiting.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
        });
        Ok(())
    }

    /// Send now, outside the queue, and say what came back — the
    /// administration's test. Each wait on the relay is bounded below the
    /// page's own fifteen seconds, so the relay's real answer, or its
    /// failure, is what comes back. A success lifts a pause.
    pub async fn send_now(&self, relay: Relay, message: Outgoing) -> Result<String, Failure> {
        let kind = message.kind;
        let result = tokio::time::timeout(
            Duration::from_secs(15),
            Self::attempt(
                Arc::clone(&self.transport),
                None,
                relay,
                message,
                TEST_TIMEOUT,
            ),
        )
        .await
        .unwrap_or_else(|_| {
            Err(Failure::Transient(
                "no answer in fifteen seconds".to_string(),
            ))
        });
        self.book.settle(None, kind, &result);
        if result.is_ok() {
            self.resume_all();
        }
        result
    }
}

/// Whether `at` is within a day before `now`, both RFC 3339.
fn within_a_day(at: &str, now: &str) -> bool {
    use time::format_description::well_known::Rfc3339;
    match (
        time::OffsetDateTime::parse(at, &Rfc3339),
        time::OffsetDateTime::parse(now, &Rfc3339),
    ) {
        (Ok(a), Ok(n)) => n - a < time::Duration::days(1),
        _ => false,
    }
}

/// The UTC day after the one `at` falls on.
fn next_day(at: &str) -> String {
    use time::format_description::well_known::Rfc3339;
    time::OffsetDateTime::parse(at, &Rfc3339)
        .map(|t| (t + time::Duration::days(1)).date().to_string())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_configurations_relay_reads_like_a_url() {
        let r = Relay::from_env(
            "smtp://famille%40gmail.com:abcd%20efgh%20ijkl%20mnop@smtp.gmail.com:587",
            Some("Famille Ndiaye-Lefèvre <famille.ndiaye@gmail.com>"),
        )
        .unwrap();
        assert_eq!(r.host, "smtp.gmail.com");
        assert_eq!(r.port, 587);
        assert_eq!(r.security, Security::Starttls);
        assert_eq!(r.username, "famille@gmail.com");
        assert_eq!(
            r.password, "abcdefghijklmnop",
            "an app password, spaces out"
        );
        assert_eq!(r.from_address, "famille.ndiaye@gmail.com");
        assert_eq!(r.from_name, "Famille Ndiaye-Lefèvre");
        assert!(r.from_env);

        let tls = Relay::from_env("smtps://u%40example.org:p@mail.example.org", None).unwrap();
        assert_eq!((tls.security, tls.port), (Security::Tls, 465));
        assert_eq!(
            tls.from_address, "u@example.org",
            "the user name, when an address"
        );

        let local = Relay::from_env(
            "smtp://localhost:25?security=none",
            Some("site@family.example"),
        )
        .unwrap();
        assert_eq!(local.security, Security::None);
    }

    #[test]
    fn a_relay_that_cannot_work_is_refused_before_it_is_tried() {
        assert_eq!(Relay::from_env("http://x", None), Err(Refused::BadUrl));
        assert_eq!(
            Relay::from_env("smtp://smtp.example.org:587", None),
            Err(Refused::NoSender),
            "nobody to send as"
        );
        assert_eq!(
            Relay::from_env("smtp://smtp.example.org?security=none", Some("a@b.org")),
            Err(Refused::PlainElsewhere),
            "no TLS, away from this machine"
        );
        let mut s = Settings {
            host: "smtp.example.org".into(),
            from_address: "not an address".into(),
            ..Settings::default()
        };
        assert_eq!(Relay::from_settings(&s), Err(Refused::NoSender));
        s.from_address = "family@example.org".into();
        let r = Relay::from_settings(&s).unwrap();
        assert_eq!(r.port, 587, "STARTTLS's port when none is given");
    }

    #[test]
    fn an_address_is_one_address_and_nothing_more() {
        assert!(valid_address("awa@example.org"));
        assert!(!valid_address("awa@example.org\r\nBcc: x@y.org"));
        assert!(!valid_address("Awa <awa@example.org>"));
        assert!(!valid_address("awa"));
        assert_eq!(mask("awa@example.org"), "a…@example.org");
    }

    #[test]
    fn the_password_never_shows_in_debug() {
        let s = Settings {
            password: "hunter2hunter2".into(),
            ..Settings::default()
        };
        assert!(!format!("{s:?}").contains("hunter2"));
        let r = Relay::from_env("smtp://u%40x.org:hunter2@h.example.org", None).unwrap();
        assert!(!format!("{r:?}").contains("hunter2"));
    }

    #[test]
    fn only_the_relays_own_code_is_read_not_its_words() {
        assert_eq!(
            enhanced("permanent error (550): 5.4.5 Daily user sending quota exceeded."),
            "5.4.5"
        );
        assert_eq!(
            enhanced("permanent error (554): 5.7.1 Client host [45.4.51.20] blocked"),
            "5.7.1",
            "an address that looks like a code is not the code"
        );
        assert_eq!(
            enhanced("network error: Resource temporarily unavailable"),
            ""
        );
    }

    #[test]
    fn a_port_that_speaks_the_other_security_is_refused() {
        let r = |url: &str| Relay::from_env(url, Some("family@family.example")).map(|_| ());
        assert_eq!(
            r("smtp://u:p@smtp.example.org:465"),
            Err(Refused::PortMismatch)
        );
        assert_eq!(
            r("smtps://u:p@smtp.example.org:587"),
            Err(Refused::PortMismatch)
        );
        assert_eq!(r("smtps://u:p@smtp.example.org:465"), Ok(()));
        assert_eq!(r("smtp://u:p@smtp.example.org:2525"), Ok(()));
    }

    #[test]
    fn limits_are_counted_all_at_once_and_given_back() {
        let mut c = Counts::default();
        let hour = Duration::from_secs(3600);
        let both = [("a".to_string(), 1, hour), ("b".to_string(), 2, hour)];
        assert!(c.take_all(&both));
        assert!(!c.take_all(&both), "a is full");
        assert!(
            c.take_all(&[("b".to_string(), 2, hour)]),
            "b was counted once, not twice"
        );
        assert!(!c.take_all(&[("b".to_string(), 2, hour)]));
        c.refund(&both);
        assert!(c.take_all(&[("a".to_string(), 1, hour)]), "a given back");
    }

    #[test]
    fn a_pause_for_the_relays_limit_ends_with_its_day() {
        let m = Mailer::new(Arc::new(Recorder::new()));
        let now = crate::view::now_iso8601();
        m.book.change(|l| {
            l.paused = Some("quota".into());
            l.paused_until = day_of(&now);
            ((), true)
        });
        assert_eq!(m.ledger().paused, None, "its day has come");
        assert!(!m.book.paused());
        m.book.change(|l| {
            l.paused = Some("auth".into());
            l.failures_in_a_row = 4;
            ((), true)
        });
        assert!(m.book.paused());
        m.resume();
        let l = m.ledger();
        assert_eq!((l.paused, l.failures_in_a_row), (None, 0));
        // Saving keeps a pause for the relay's limit; a test that got through
        // lifts it.
        m.book.change(|l| {
            l.paused = Some("quota".into());
            l.paused_until = "2999-01-01".into();
            ((), true)
        });
        m.resume();
        assert_eq!(m.ledger().paused.as_deref(), Some("quota"));
        m.resume_all();
        assert_eq!(m.ledger().paused, None);
    }

    #[test]
    fn a_message_goes_out_in_two_parts_with_its_subject_encoded() {
        let relay = Relay::from_env("smtp://u%40x.org:p@h.example.org", None).unwrap();
        let m = Outgoing {
            to: "cousin@example.org".into(),
            subject: "Добро пожаловать — Famille".into(),
            text: "Bonjour".into(),
            html: "<p>Bonjour</p>".into(),
            kind: "test",
            unsubscribe: Some("https://family.example/mail/stop/abc".into()),
            lang: "ru".into(),
        };
        let wire = String::from_utf8(to_mime(&relay, &m).unwrap().formatted()).unwrap();
        assert!(wire.contains("Subject: =?utf-8?"), "{wire}");
        assert!(wire.contains("multipart/alternative"));
        assert!(wire.contains("text/plain"));
        assert!(wire.contains("text/html"));
        assert!(wire.contains("List-Unsubscribe: <https://family.example/mail/stop/abc>"));
        assert!(wire.contains("List-Unsubscribe-Post: List-Unsubscribe=One-Click"));
        assert!(wire.contains("Content-ID: <logo>"));
        assert!(wire.contains("Auto-Submitted: auto-generated"));
    }
}
