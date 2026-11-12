//! Mail (docs/adr/0048): the forgotten password, the welcome of an account an
//! administrator makes, the notice that a password changed, the relay's
//! settings, and the way back to the page a sign-in interrupted.
//!
//! The relay is a recorder in most of these, which keeps every message as it
//! would go on the wire; the last ones speak SMTP to a relay of the suite's
//! own on loopback, so `lettre`'s path is tried as well.

mod common;

use std::os::unix::fs::PermissionsExt as _;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use axgf_cms::mail::{Failure, Outgoing, Recorder, Relay, Transport};
use axum::body::Body;
use axum::http::{header, Response, StatusCode};
use common::*;

const PUBLIC: &str = "https://family.example";
const SENDER: &str = "family@family.example";
const RELAY_SECRET: &str = "relay-app-secret-4417";
const LANG: &str = "axgf_lang=en";

fn enc(v: &str) -> String {
    v.bytes()
        .map(|b| match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                (b as char).to_string()
            }
            _ => format!("%{b:02X}"),
        })
        .collect()
}

fn form(pairs: &[(&str, &str)]) -> String {
    pairs
        .iter()
        .map(|(k, v)| format!("{k}={}", enc(v)))
        .collect::<Vec<_>>()
        .join("&")
}

/// A page's text with the template's escapes undone: `/` is `&#x2f;` there.
fn unescaped(html: &str) -> String {
    html.replace("&#x2f;", "/")
        .replace("&#x27;", "'")
        .replace("&quot;", "\"")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
}

/// A site with mail, as a test drives it.
struct Site {
    app: axum::Router,
    state: Arc<axgf_cms::AppState>,
    /// An administrator's session, reading English.
    admin: String,
    bundle: PathBuf,
    _dir: Scratch,
}

impl Site {
    async fn open(tag: &str, transport: Arc<dyn Transport>) -> Site {
        let dir = scratch(tag);
        let bundle = dir.join("family.axgf");
        let (app, state) = build_app_with_mailer(&bundle, transport).expect("build app");
        let admin = format!("{}; {LANG}", admin_cookie(&app).await);
        Site {
            app,
            state,
            admin,
            bundle,
            _dir: dir,
        }
    }

    /// The relay and the public address, as an administrator sets them.
    async fn with_relay(self) -> Site {
        self.set_relay(&[]).await;
        self.set_public().await;
        self
    }

    /// Save the relay: a complete one, with `extra` in place of its fields.
    async fn set_relay(&self, extra: &[(&str, &str)]) -> String {
        let r = self.save_relay(extra).await;
        expect_status(r, StatusCode::OK, "the relay is saved").await
    }

    async fn save_relay(&self, extra: &[(&str, &str)]) -> Response<Body> {
        let mut pairs = vec![
            ("host", "relay.test"),
            ("port", "587"),
            ("security", "starttls"),
            ("username", SENDER),
            ("password", RELAY_SECRET),
            ("from_address", SENDER),
            ("from_name", "Famille Ndiaye-Lefèvre"),
        ];
        for &(k, v) in extra {
            pairs.retain(|&(pk, _)| pk != k);
            pairs.push((k, v));
        }
        self.admin_post("/admin/connectivity/mail", &pairs).await
    }

    async fn set_public(&self) {
        let r = self
            .admin_post("/admin/connectivity/address", &[("public_url", PUBLIC)])
            .await;
        expect_status(r, StatusCode::OK, "the public address is saved").await;
    }

    async fn admin_post(&self, uri: &str, pairs: &[(&str, &str)]) -> Response<Body> {
        post_form_as(&self.app, &self.admin, uri, &form(pairs)).await
    }

    async fn send_test(&self, to: &str) -> Response<Body> {
        self.admin_post("/admin/connectivity/mail/test", &[("to", to)])
            .await
    }

    async fn admin_page(&self, uri: &str) -> String {
        let r = get_with_cookie(&self.app, uri, &self.admin).await;
        expect_status(r, StatusCode::OK, uri).await
    }

    async fn visitor_post(&self, uri: &str, pairs: &[(&str, &str)]) -> Response<Body> {
        post_form_as(&self.app, LANG, uri, &form(pairs)).await
    }

    async fn visit(&self, uri: &str) -> Response<Body> {
        get_with_cookie(&self.app, uri, LANG).await
    }

    /// A member's account, written straight to the accounts: no welcome goes.
    fn account(&self, username: &str, password: &str, email: Option<&str>) -> String {
        let mut u = axgf_cms::acl::new_user(username, password, axgf_cms::acl::Role::Viewer)
            .expect("an account");
        u.email = email.map(str::to_string);
        let id = u.id.clone();
        self.state
            .acl_mutate(|acl| acl.users.push(u))
            .expect("saved");
        id
    }

    async fn sign_in(&self, username: &str, password: &str, next: &str) -> Response<Body> {
        self.visitor_post(
            "/admin/login",
            &[
                ("username", username),
                ("password", password),
                ("next", next),
            ],
        )
        .await
    }

    /// Once the tasks a request left behind have run, and every message in
    /// flight has settled.
    async fn settle(&self) {
        tokio::time::sleep(Duration::from_millis(50)).await;
        for _ in 0..500 {
            if self.state.mailer().waiting() == 0 {
                return;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        panic!("a message is still in flight");
    }
}

/// The first `n` messages handed to the relay, waiting for them.
async fn wait_for(rec: &Recorder, n: usize) -> Vec<Outgoing> {
    for _ in 0..500 {
        let sent = rec.sent();
        if sent.len() >= n {
            return sent;
        }
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    panic!(
        "{n} message(s) expected, {} handed to the relay",
        rec.sent().len()
    );
}

/// The link a message carries to choose a password, as a path of the site —
/// the same in both parts.
fn link_in(m: &Outgoing) -> String {
    let start = format!("{PUBLIC}/mail/set/");
    let at = m
        .text
        .find(&start)
        .unwrap_or_else(|| panic!("no link in:\n{}", m.text));
    let url = m.text[at..].split_whitespace().next().unwrap();
    assert!(
        unescaped(&m.html).contains(&format!("href=\"{url}\"")),
        "the HTML part carries the same link"
    );
    url.strip_prefix(PUBLIC).unwrap().to_string()
}

const NEW: [(&str, &str); 2] = [
    ("password", "a-new-password-long-enough"),
    ("again", "a-new-password-long-enough"),
];

// ---------------------------------------------------------------------------

#[tokio::test]
async fn a_forgotten_password_is_answered_the_same_whoever_is_asked_about() {
    let rec = Recorder::new();
    let site = Site::open("mail-same", Arc::new(rec.clone())).await;
    site.account(
        "ada",
        "an-old-password-long-enough",
        Some("ada@example.org"),
    );
    let site = site.with_relay().await;

    let r = site.visitor_post("/mail/forgot", &[("who", "ada")]).await;
    assert_eq!(r.headers()[header::CACHE_CONTROL], "no-store");
    let known = expect_status(r, StatusCode::OK, "a known account").await;
    let mut others = Vec::new();
    for who in ["nobody", "nobody@example.org", ""] {
        let r = site.visitor_post("/mail/forgot", &[("who", who)]).await;
        others.push(expect_status(r, StatusCode::OK, who).await);
    }
    for other in &others {
        assert_eq!(&known, other, "the answer says nothing of who exists");
    }
    assert!(
        known.contains("If an account matches, a link is on its way"),
        "{known}"
    );

    site.settle().await;
    let sent = rec.sent();
    assert_eq!(sent.len(), 1, "one message, to the account that exists");
    let m = &sent[0];
    assert_eq!((m.kind, m.to.as_str()), ("reset", "ada@example.org"));
    assert_eq!(m.lang, "en", "in the language the page was asked in");
    assert!(
        m.subject.starts_with("Choose a new password"),
        "{}",
        m.subject
    );
    assert!(m.text.contains("ada"), "it names the account");
    link_in(m);
}

#[tokio::test]
async fn a_mailed_link_sets_the_password_once_and_the_account_is_told() {
    let rec = Recorder::new();
    let site = Site::open("mail-reset", Arc::new(rec.clone())).await;
    site.account(
        "ada",
        "an-old-password-long-enough",
        Some("ada@example.org"),
    );
    let site = site.with_relay().await;

    // By its address, in any case.
    site.visitor_post("/mail/forgot", &[("who", "ADA@Example.org")])
        .await;
    let link = link_in(&wait_for(&rec, 1).await[0]);

    // Opening the link spends nothing.
    for _ in 0..2 {
        let r = site.visit(&link).await;
        assert_eq!(r.headers()[header::CACHE_CONTROL], "no-store");
        let html = expect_status(r, StatusCode::OK, "the link's page").await;
        assert!(
            html.contains("Choose a new password for the account ada."),
            "{html}"
        );
        assert!(html.contains(r#"name="again""#));
    }

    let r = site
        .visitor_post(
            &link,
            &[
                ("password", "a-new-password-long-enough"),
                ("again", "another-password-entirely"),
            ],
        )
        .await;
    let html = expect_status(r, StatusCode::UNPROCESSABLE_ENTITY, "two passwords").await;
    assert!(html.contains("The two passwords are not the same."));
    let r = site
        .visitor_post(&link, &[("password", "short"), ("again", "short")])
        .await;
    expect_status(r, StatusCode::UNPROCESSABLE_ENTITY, "a short password").await;

    let r = site.visitor_post(&link, &NEW).await;
    assert_eq!(r.status(), StatusCode::SEE_OTHER);
    assert_eq!(
        r.headers()[header::LOCATION],
        "/",
        "a member goes to the family"
    );
    assert_eq!(r.headers()[header::CACHE_CONTROL], "no-store");
    let raw = r.headers()[header::SET_COOKIE]
        .to_str()
        .unwrap()
        .to_string();
    assert!(raw.contains("Secure"), "the public address is https: {raw}");
    let session = raw.split(';').next().unwrap().to_string();
    let r = get_with_cookie(&site.app, "/account", &session).await;
    assert_eq!(r.status(), StatusCode::OK, "signed in by the link");

    let r = site.sign_in("ada", "a-new-password-long-enough", "").await;
    assert!(r.status().is_redirection(), "the new password works");
    let r = site.sign_in("ada", "an-old-password-long-enough", "").await;
    assert!(!r.status().is_redirection(), "the old one does not");

    // The link is spent.
    let html = body_string(site.visit(&link).await).await;
    assert!(html.contains("This link is no longer good"), "{html}");
    assert!(!html.contains(r#"name="again""#));
    let r = site.visitor_post(&link, &NEW).await;
    expect_status(r, StatusCode::BAD_REQUEST, "a spent link, posted").await;

    // And the account is told, at its address.
    let sent = wait_for(&rec, 2).await;
    let told = &sent[1];
    assert_eq!(
        (told.kind, told.to.as_str()),
        ("changed", "ada@example.org")
    );
    assert!(
        told.text
            .contains("The password of the account ada was changed"),
        "{}",
        told.text
    );
    assert!(!told.text.contains("An administrator"), "by its owner");
    assert!(
        !told.text.contains("/mail/set/"),
        "a notice is not a way in"
    );
    site.settle().await;
    assert_eq!(rec.sent().len(), 2);
}

#[tokio::test]
async fn a_link_never_made_here_counts_in_the_jail_and_a_spent_one_does_not() {
    let rec = Recorder::new();
    let site = Site::open("mail-jail", Arc::new(rec.clone())).await;
    site.account(
        "ada",
        "an-old-password-long-enough",
        Some("ada@example.org"),
    );
    let site = site.with_relay().await;
    site.visitor_post("/mail/forgot", &[("who", "ada")]).await;
    let link = link_in(&wait_for(&rec, 1).await[0]);
    assert!(site
        .visitor_post(&link, &NEW)
        .await
        .status()
        .is_redirection());

    // A link that was good once, posted again and again: somebody with an
    // old message, not a guess.
    for _ in 0..12 {
        let r = site.visitor_post(&link, &NEW).await;
        assert_eq!(r.status(), StatusCode::BAD_REQUEST);
    }
    // Opening a link charges nothing, whatever it is.
    let forged = format!("/mail/set/bm90LWEtdG9rZW4.{0}.{0}", "A".repeat(22));
    for _ in 0..12 {
        assert_eq!(site.visit(&forged).await.status(), StatusCode::OK);
    }
    let r = site.sign_in("ada", "a-new-password-long-enough", "").await;
    assert!(r.status().is_redirection(), "nothing so far was a guess");

    // A link never made here, posted, is a failed sign-in.
    for i in 0..axgf_cms::jail::MAX_FAILURES {
        let uri = if i % 2 == 0 {
            forged.clone()
        } else {
            "/mail/set/not-a-token".to_string()
        };
        let r = site.visitor_post(&uri, &NEW).await;
        assert_eq!(r.status(), StatusCode::BAD_REQUEST);
    }
    let r = site.sign_in("ada", "a-new-password-long-enough", "").await;
    assert_eq!(
        r.status(),
        StatusCode::TOO_MANY_REQUESTS,
        "the guesses shut the address out"
    );
}

#[tokio::test]
async fn an_address_gets_one_link_a_quarter_of_an_hour_and_a_client_eight_tries() {
    let rec = Recorder::new();
    let site = Site::open("mail-quarter", Arc::new(rec.clone())).await;
    site.account(
        "ada",
        "an-old-password-long-enough",
        Some("ada@example.org"),
    );
    site.account(
        "bob",
        "an-old-password-long-enough",
        Some("bob@example.org"),
    );
    let site = site.with_relay().await;

    for who in ["ada", "ada@example.org", "Ada"] {
        site.visitor_post("/mail/forgot", &[("who", who)]).await;
    }
    site.settle().await;
    assert_eq!(rec.sent().len(), 1, "asked three times, one message");

    // Eight asks from one place in a quarter of an hour, then nothing more,
    // for anyone — and the page says the same.
    let before = body_string(site.visitor_post("/mail/forgot", &[("who", "x")]).await).await;
    for _ in 4..axgf_cms::session::MAX_ATTEMPTS {
        site.visitor_post("/mail/forgot", &[("who", "x")]).await;
    }
    let after = body_string(site.visitor_post("/mail/forgot", &[("who", "bob")]).await).await;
    assert_eq!(before, after);
    site.settle().await;
    assert_eq!(rec.sent().len(), 1, "bob's link was not sent");
}

#[tokio::test]
async fn an_account_signing_in_through_a_service_is_told_which() {
    let rec = Recorder::new();
    let site = Site::open("mail-service", Arc::new(rec.clone())).await;
    let identity = axgf_cms::acl::Identity {
        provider: "google".into(),
        issuer: "https://accounts.google.com".into(),
        subject: "g-1".into(),
        email: Some("gaby@example.org".into()),
        linked_at: "2026-11-01T10:00:00Z".into(),
    };
    let user = axgf_cms::acl::new_provider_user("gaby", identity, Some("gaby@example.org".into()));
    site.state.acl_mutate(|acl| acl.users.push(user)).unwrap();
    let site = site.with_relay().await;

    site.visitor_post("/mail/forgot", &[("who", "gaby")]).await;
    let sent = wait_for(&rec, 1).await;
    let m = &sent[0];
    assert_eq!(m.kind, "service");
    assert!(m.text.contains("Google"), "{}", m.text);
    assert!(m.text.contains(&format!("{PUBLIC}/admin/login")));
    assert!(
        !m.text.contains("/mail/set/"),
        "no password to set on an account without one"
    );
}

#[tokio::test]
async fn an_account_made_with_an_address_is_welcomed_in_the_language_chosen() {
    let rec = Recorder::new();
    let site = Site::open("mail-welcome", Arc::new(rec.clone()))
        .await
        .with_relay()
        .await;

    let html = site.admin_page("/admin/users").await;
    assert!(
        html.contains(r#"name="language""#),
        "the welcome's language"
    );
    let r = site
        .admin_post(
            "/admin/users",
            &[
                ("username", "pola"),
                ("password", ""),
                ("email", "pola@example.org"),
                ("role", "viewer"),
                ("language", "pl"),
            ],
        )
        .await;
    let html = expect_status(r, StatusCode::OK, "the account is made").await;
    assert!(html.contains("Account pola created. Its welcome"), "{html}");
    assert!(
        !html.contains("Their password is"),
        "no password shown: the welcome carries the way in"
    );

    let sent = wait_for(&rec, 1).await;
    let m = &sent[0];
    assert_eq!(
        (m.kind, m.to.as_str(), m.lang.as_str()),
        ("welcome", "pola@example.org", "pl")
    );
    let link = link_in(m);

    let html = body_string(site.visit(&link).await).await;
    assert!(html.contains("The account pola is ready"), "{html}");
    let r = site
        .visitor_post(
            &link,
            &[
                ("password", "pola-chose-this-one"),
                ("again", "pola-chose-this-one"),
            ],
        )
        .await;
    assert_eq!(r.status(), StatusCode::SEE_OTHER);
    assert_eq!(r.headers()[header::LOCATION], "/");
    let r = site.sign_in("pola", "pola-chose-this-one", "").await;
    assert!(r.status().is_redirection());

    // Good until the first sign-in, and no further.
    let html = body_string(site.visit(&link).await).await;
    assert!(html.contains("This link is no longer good"), "{html}");
    // A welcome is not followed by a notice that a password changed.
    site.settle().await;
    assert_eq!(rec.sent().len(), 1);

    // The accounts page says what became of the last message.
    let html = site.admin_page("/admin/users").await;
    assert!(
        html.contains("Last message: accepted by the relay"),
        "{html}"
    );
}

#[tokio::test]
async fn an_administrators_welcome_holds_one_day() {
    let rec = Recorder::new();
    let site = Site::open("mail-admin-welcome", Arc::new(rec.clone()))
        .await
        .with_relay()
        .await;
    let r = site
        .admin_post(
            "/admin/users",
            &[
                ("username", "root2"),
                ("password", ""),
                ("email", "root2@example.org"),
                ("role", "admin"),
            ],
        )
        .await;
    expect_status(r, StatusCode::OK, "the account is made").await;
    let m = &wait_for(&rec, 1).await[0];
    assert!(m.text.contains("This link works for one day"), "{}", m.text);
    assert_eq!(m.lang, "en", "the administrator's language, none chosen");
}

#[tokio::test]
async fn the_welcome_goes_again_and_a_member_who_signed_in_gets_a_reset() {
    let rec = Recorder::new();
    let site = Site::open("mail-again", Arc::new(rec.clone())).await;
    let quiet = site.account(
        "quiet",
        "a-password-long-enough-1",
        Some("quiet@example.org"),
    );
    let seen = site.account("seen", "a-password-long-enough-2", Some("seen@example.org"));
    let site = site.with_relay().await;
    let r = site.sign_in("seen", "a-password-long-enough-2", "").await;
    assert!(r.status().is_redirection());

    let html = site.admin_page("/admin/users").await;
    assert!(
        html.contains(&format!(r#"action="/admin/users/{quiet}/welcome""#)),
        "{html}"
    );
    let r = site
        .admin_post(&format!("/admin/users/{quiet}/welcome"), &[])
        .await;
    let html = expect_status(r, StatusCode::OK, "the welcome again").await;
    assert!(
        html.contains("The welcome is handed to the relay again for quiet."),
        "{html}"
    );
    let r = site
        .admin_post(&format!("/admin/users/{seen}/welcome"), &[])
        .await;
    let html = expect_status(r, StatusCode::OK, "a reset instead").await;
    assert!(
        html.contains("seen has signed in before: a link to choose a new password"),
        "{html}"
    );
    site.settle().await;
    let kinds: Vec<_> = rec.sent().iter().map(|m| (m.kind, m.to.clone())).collect();
    assert_eq!(
        kinds,
        [
            ("welcome", "quiet@example.org".to_string()),
            ("reset", "seen@example.org".to_string())
        ]
    );

    // Again at once: the address had one a moment ago.
    let r = site
        .admin_post(&format!("/admin/users/{quiet}/welcome"), &[])
        .await;
    let html = expect_status(r, StatusCode::OK, "too soon").await;
    assert!(html.contains("Nothing sent to quiet"), "{html}");
    site.settle().await;
    assert_eq!(rec.sent().len(), 2);
}

#[tokio::test]
async fn without_a_relay_and_a_public_address_nothing_is_offered_and_the_password_is_shown() {
    let rec = Recorder::new();
    let site = Site::open("mail-off", Arc::new(rec.clone())).await;
    site.account(
        "ada",
        "an-old-password-long-enough",
        Some("ada@example.org"),
    );

    let html = body_string(site.visit("/admin/login").await).await;
    assert!(
        !html.contains("/mail/forgot"),
        "no way to a page that cannot help"
    );
    let html = body_string(site.visit("/mail/forgot").await).await;
    assert!(html.contains("This site does not send mail"), "{html}");
    assert!(!html.contains(r#"name="who""#));
    let html = body_string(site.visitor_post("/mail/forgot", &[("who", "ada")]).await).await;
    assert!(html.contains("This site does not send mail"), "{html}");

    let r = site
        .admin_post(
            "/admin/users",
            &[
                ("username", "bea"),
                ("password", ""),
                ("email", "bea@example.org"),
                ("role", "viewer"),
            ],
        )
        .await;
    let html = expect_status(r, StatusCode::OK, "the account is made").await;
    assert!(html.contains("Their password is"), "{html}");

    // A relay alone is not enough: a link is built on the public address,
    // never on the address the request came to.
    site.set_relay(&[]).await;
    let html = body_string(site.visit("/mail/forgot").await).await;
    assert!(html.contains("This site does not send mail"), "{html}");
    site.set_public().await;
    let html = body_string(site.visit("/mail/forgot").await).await;
    assert!(html.contains(r#"name="who""#), "{html}");
    let html = unescaped(&body_string(site.visit("/admin/login").await).await);
    assert!(html.contains(r#"href="/mail/forgot""#), "{html}");

    site.settle().await;
    assert!(rec.sent().is_empty(), "nothing was sent before");
    let infra = unescaped(&site.admin_page("/admin/infrastructure").await);
    assert!(infra.contains("The relay is set and ready"), "{infra}");
}

#[tokio::test]
async fn without_a_relay_the_checks_say_mail_is_off_and_the_dashboard_says_nothing() {
    let site = Site::open("mail-off-checks", Arc::new(Recorder::new())).await;
    let infra = unescaped(&site.admin_page("/admin/infrastructure").await);
    assert!(
        infra.contains("No mail relay: the site sends no mail"),
        "{infra}"
    );
    let home = unescaped(&site.admin_page("/admin").await);
    assert!(!home.contains("connectivity#mail"), "{home}");
    // A relay without the public address: a warning, with the way to fix it.
    site.set_relay(&[]).await;
    let infra = unescaped(&site.admin_page("/admin/infrastructure").await);
    assert!(
        infra.contains("A relay is set but no public address"),
        "{infra}"
    );
}

#[tokio::test]
async fn the_relay_password_is_kept_and_never_shown() {
    let site = Site::open("mail-secret", Arc::new(Recorder::new())).await;
    let saved = site.set_relay(&[]).await;
    assert!(saved.contains("Saved: the mail relay."), "{saved}");
    assert!(!saved.contains(RELAY_SECRET));
    let page = site.admin_page("/admin/connectivity").await;
    assert!(page.contains("relay.test"), "the relay is named");
    assert!(!page.contains(RELAY_SECRET), "its password is not");
    assert_eq!(site.state.site().mail.password, RELAY_SECRET);

    // Blank keeps it; an app password's spaces go; the box clears it.
    site.set_relay(&[("password", "")]).await;
    assert_eq!(site.state.site().mail.password, RELAY_SECRET);
    site.set_relay(&[("password", "abcd efgh ijkl mnop")]).await;
    assert_eq!(site.state.site().mail.password, "abcdefghijklmnop");
    site.set_relay(&[("password", ""), ("clear_password", "on")])
        .await;
    assert!(site.state.site().mail.password.is_empty());

    // What cannot work is refused before it is kept.
    let kept = site.state.site().mail;
    for refused in [
        vec![("port", "465")],
        vec![("security", "tls"), ("port", "587")],
        vec![("from_address", "not an address")],
        vec![("from_address", "a@b.example\r\nBcc: c@d.example")],
        vec![("security", "none")],
        vec![("reply_to", "nobody")],
        vec![("port", "seventy")],
        vec![("daily_limit", "0")],
        vec![("language", "tlh")],
    ] {
        let r = site.save_relay(&refused).await;
        assert_eq!(r.status(), StatusCode::UNPROCESSABLE_ENTITY, "{refused:?}");
        assert!(
            site.state.site().mail == kept,
            "{refused:?} changed nothing"
        );
    }
    // Without TLS, only this machine.
    site.set_relay(&[("host", "127.0.0.1"), ("security", "none"), ("port", "25")])
        .await;
}

/// A relay that refuses the credentials, and counts how often it was asked.
#[derive(Clone, Default)]
struct Refusing {
    calls: Arc<AtomicUsize>,
}

impl Transport for Refusing {
    fn send(&self, _: &Relay, _: &Outgoing, _: Duration) -> Result<String, Failure> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        Err(Failure::Auth(
            "535 5.7.8 Username and Password not accepted".into(),
        ))
    }
}

#[tokio::test]
async fn a_refused_password_stops_mail_until_the_relay_is_saved_again() {
    let relay = Refusing::default();
    let site = Site::open("mail-auth", Arc::new(relay.clone())).await;
    site.account(
        "ada",
        "an-old-password-long-enough",
        Some("ada@example.org"),
    );
    site.account(
        "bob",
        "an-old-password-long-enough",
        Some("bob@example.org"),
    );
    let site = site.with_relay().await;

    site.visitor_post("/mail/forgot", &[("who", "ada")]).await;
    site.settle().await;
    assert_eq!(
        relay.calls.load(Ordering::SeqCst),
        1,
        "a refusal is not tried again"
    );
    let ledger = site.state.mailer().ledger();
    assert_eq!(ledger.paused.as_deref(), Some("auth"));
    assert_eq!(ledger.last_failure.as_ref().unwrap().failure, "auth");
    let page = unescaped(&site.admin_page("/admin/connectivity").await);
    assert!(
        page.contains("Paused: the relay refused the credentials."),
        "{page}"
    );
    let users = unescaped(&site.admin_page("/admin/users").await);
    assert!(users.contains("Last message: not sent"), "{users}");

    // Said where an administrator looks first, and among the checks.
    let home = unescaped(&site.admin_page("/admin").await);
    assert!(
        home.contains("Neither forgotten passwords nor welcomes go out.")
            && home.contains(r#"href="/admin/connectivity#mail""#),
        "{home}"
    );
    let infra = unescaped(&site.admin_page("/admin/infrastructure").await);
    assert!(
        infra.contains("Neither forgotten passwords nor welcomes go out."),
        "{infra}"
    );

    // Nothing more is tried while it is paused.
    site.visitor_post("/mail/forgot", &[("who", "bob")]).await;
    site.settle().await;
    assert_eq!(relay.calls.load(Ordering::SeqCst), 1);

    // Saving the relay again lifts the pause.
    site.set_relay(&[("password", "another-app-password")])
        .await;
    assert_eq!(site.state.mailer().ledger().paused, None);
}

#[tokio::test]
async fn the_days_ceiling_holds_mail_back_and_says_so() {
    let rec = Recorder::new();
    let site = Site::open("mail-ceiling", Arc::new(rec.clone())).await;
    site.account(
        "ada",
        "an-old-password-long-enough",
        Some("ada@example.org"),
    );
    site.account(
        "bob",
        "an-old-password-long-enough",
        Some("bob@example.org"),
    );
    site.set_relay(&[("daily_limit", "1")]).await;
    site.set_public().await;
    let home = unescaped(&site.admin_page("/admin").await);
    assert!(!home.contains("connectivity#mail"), "nothing to say yet");

    site.visitor_post("/mail/forgot", &[("who", "ada")]).await;
    site.visitor_post("/mail/forgot", &[("who", "bob")]).await;
    site.settle().await;
    let sent = rec.sent();
    assert_eq!(sent.len(), 1, "one a day");
    assert_eq!(sent[0].to, "ada@example.org");
    let infra = unescaped(&site.admin_page("/admin/infrastructure").await);
    assert!(
        infra.contains("The site's daily ceiling is reached (1)"),
        "{infra}"
    );
    let home = unescaped(&site.admin_page("/admin").await);
    assert!(home.contains("connectivity#mail"), "{home}");
}

#[tokio::test]
async fn a_relay_set_by_the_configuration_wins_over_the_administration() {
    let rec = Recorder::new();
    let dir = scratch("mail-env");
    let bundle = dir.join("family.axgf");
    let state = axgf_cms::AppState::load_or_create(&bundle, TOKEN.to_string())
        .unwrap()
        .with_mailer(Arc::new(rec.clone()))
        .with_mail_env(
            Some("smtp://family%40family.example:env-secret-5521@relay.env.test:587"),
            Some("Famille Ndiaye-Lefèvre <family@family.example>"),
        );
    state
        .site_mutate(|s| s.doors = axgf_cms::site::Doors::everyone())
        .unwrap();
    let state = Arc::new(state);
    let app = axgf_cms::router(state.clone());
    let site = Site {
        admin: format!("{}; {LANG}", admin_cookie(&app).await),
        app,
        state,
        bundle,
        _dir: dir,
    };

    // The relay's own fields are for show; what the section adds to any
    // relay is kept.
    let r = site
        .save_relay(&[
            ("reply_to", "cousins@family.example"),
            ("daily_limit", "50"),
        ])
        .await;
    let html = expect_status(r, StatusCode::OK, "the section's own fields").await;
    assert!(html.contains("Saved: the mail relay."), "{html}");
    let kept = site.state.site().mail;
    assert!(
        kept.host.is_empty() && kept.password.is_empty(),
        "no relay field kept"
    );
    assert_eq!(
        (kept.reply_to.as_str(), kept.daily_limit),
        ("cousins@family.example", 50)
    );
    let html = site.admin_page("/admin/connectivity").await;
    assert!(html.contains("The relay is set by the server"), "{html}");
    let page = site.admin_page("/admin/connectivity").await;
    assert!(page.contains("relay.env.test"), "{page}");
    assert!(!page.contains("env-secret-5521"));

    site.set_public().await;
    site.account(
        "ada",
        "an-old-password-long-enough",
        Some("ada@example.org"),
    );
    site.visitor_post("/mail/forgot", &[("who", "ada")]).await;
    assert_eq!(wait_for(&rec, 1).await[0].kind, "reset");
    let relay = site.state.mailer().relay(&site.state.site()).unwrap();
    assert_eq!((relay.host.as_str(), relay.port), ("relay.env.test", 587));
    assert_eq!(relay.from_name, "Famille Ndiaye-Lefèvre");
    assert_eq!(
        relay.reply_to, "cousins@family.example",
        "the section's reply address"
    );
}

#[tokio::test]
async fn the_sign_in_goes_back_to_the_page_asked_for_and_never_elsewhere() {
    let site = Site::open("mail-next", Arc::new(Recorder::new())).await;
    site.account("ada", "a-password-long-enough", None);
    let pw = "a-password-long-enough";

    let html = unescaped(
        &body_string(
            site.visit("/admin/login?next=%2Fperson%2FI1%3Ftab%3Dnotes")
                .await,
        )
        .await,
    );
    assert!(
        html.contains(r#"name="next" value="/person/I1?tab=notes""#),
        "{html}"
    );
    let r = site.sign_in("ada", pw, "/person/I1?tab=notes").await;
    assert_eq!(r.headers()[header::LOCATION], "/person/I1?tab=notes");
    let session = r.headers()[header::SET_COOKIE]
        .to_str()
        .unwrap()
        .split(';')
        .next()
        .unwrap()
        .to_string();

    for elsewhere in [
        "https://elsewhere.example/",
        "//elsewhere.example",
        "/\t/elsewhere.example",
        "/\n/elsewhere.example",
        "/\\elsewhere.example",
        "javascript:alert(1)",
    ] {
        let r = site.sign_in("ada", pw, elsewhere).await;
        assert_eq!(r.status(), StatusCode::SEE_OTHER, "{elsewhere:?}");
        assert_eq!(
            r.headers()[header::LOCATION],
            "/",
            "{elsewhere:?} stays on the site"
        );
    }
    let html = unescaped(
        &body_string(
            site.visit("/admin/login?next=%2F%09%2Felsewhere.example")
                .await,
        )
        .await,
    );
    assert!(
        !html.contains(r#"name="next""#),
        "nowhere to go back to: {html}"
    );

    // Signed in already, the sign-in sends the reader straight on.
    let r = get_with_cookie(&site.app, "/admin/login?next=%2Fperson%2FI1", &session).await;
    assert_eq!(r.headers()[header::LOCATION], "/person/I1");
    let r = get_with_cookie(
        &site.app,
        "/admin/login?next=%2F%09%2Felsewhere.example",
        &session,
    )
    .await;
    assert_eq!(r.headers()[header::LOCATION], "/");
}

#[tokio::test]
async fn an_administrator_who_sets_a_password_tells_the_account() {
    let rec = Recorder::new();
    let site = Site::open("mail-admin-set", Arc::new(rec.clone())).await;
    let id = site.account(
        "ada",
        "an-old-password-long-enough",
        Some("ada@example.org"),
    );
    let site = site.with_relay().await;
    let edit = |password| {
        vec![
            ("role", "viewer"),
            ("status", "active"),
            ("email", "ada@example.org"),
            ("password", password),
        ]
    };

    let r = site
        .admin_post(
            &format!("/admin/users/{id}"),
            &edit("set-by-the-administrator"),
        )
        .await;
    expect_status(r, StatusCode::OK, "the password is set").await;
    let m = &wait_for(&rec, 1).await[0];
    assert_eq!((m.kind, m.to.as_str()), ("changed", "ada@example.org"));
    assert!(
        m.text.contains("An administrator of the site changed it."),
        "{}",
        m.text
    );

    // A change that leaves the password alone says nothing.
    let r = site
        .admin_post(&format!("/admin/users/{id}"), &edit(""))
        .await;
    expect_status(r, StatusCode::OK, "nothing but the same").await;
    site.settle().await;
    assert_eq!(rec.sent().len(), 1);
}

#[tokio::test]
async fn the_mail_pages_answer_on_a_closed_site() {
    let dir = scratch("mail-closed");
    let state = axgf_cms::AppState::load_or_create(&dir.join("family.axgf"), TOKEN.to_string())
        .unwrap()
        .with_mailer(Arc::new(Recorder::new()));
    let app = axgf_cms::router(Arc::new(state));

    let r = get_with_cookie(&app, "/account", LANG).await;
    assert!(r.status().is_redirection(), "the site is closed");
    for uri in ["/mail/forgot", "/mail/set/not-a-token"] {
        let r = get_with_cookie(&app, uri, LANG).await;
        assert_eq!(r.status(), StatusCode::OK, "{uri}");
    }
    let r = post_form_as(&app, LANG, "/mail/forgot", "who=ada").await;
    assert_eq!(r.status(), StatusCode::OK);
}

#[tokio::test]
async fn what_mail_keeps_on_disk_is_private_names_nobody_and_survives_a_restart() {
    let rec = Recorder::new();
    let site = Site::open("mail-disk", Arc::new(rec.clone())).await;
    site.account(
        "ada",
        "an-old-password-long-enough",
        Some("ada@example.org"),
    );
    let site = site.with_relay().await;
    site.visitor_post("/mail/forgot", &[("who", "ada")]).await;
    let link = link_in(&wait_for(&rec, 1).await[0]);
    site.settle().await;

    for ext in ["mailstate", "mailkey"] {
        let meta = std::fs::metadata(site.bundle.with_extension(ext)).unwrap();
        assert_eq!(meta.permissions().mode() & 0o777, 0o600, "{ext}");
    }
    let ledger = std::fs::read_to_string(site.bundle.with_extension("mailstate")).unwrap();
    assert!(!ledger.contains("example.org"), "no address: {ledger}");
    assert!(ledger.contains("\"reset\""), "{ledger}");

    // The same site after a restart: the link still holds, the day's count
    // is still the day's.
    let Site {
        app,
        state,
        bundle,
        _dir,
        ..
    } = site;
    drop(app);
    drop(state);
    let (app, state) = build_app_with_mailer(&bundle, Arc::new(rec.clone())).unwrap();
    assert_eq!(state.mailer().ledger().today, 1);
    let html = body_string(get_with_cookie(&app, &link, LANG).await).await;
    assert!(
        html.contains("Choose a new password for the account ada."),
        "{html}"
    );
}

// ---------------------------------------------------------------------------
// A relay on loopback, spoken to over SMTP.

/// A relay of the suite's own: just enough SMTP to take a message or refuse
/// it, each conversation kept whole. What it answers can change between
/// conversations.
struct Sink {
    port: u16,
    auth: Arc<Mutex<&'static str>>,
    rcpt: Arc<Mutex<&'static str>>,
    talks: Arc<Mutex<Vec<String>>>,
}

impl Sink {
    async fn start() -> Sink {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let sink = Sink {
            port: listener.local_addr().unwrap().port(),
            auth: Arc::new(Mutex::new("235 2.7.0 Accepted")),
            rcpt: Arc::new(Mutex::new("250 2.1.5 OK")),
            talks: Arc::default(),
        };
        let (auth, rcpt, talks) = (sink.auth.clone(), sink.rcpt.clone(), sink.talks.clone());
        tokio::spawn(async move {
            while let Ok((stream, _)) = listener.accept().await {
                let replies = (*auth.lock().unwrap(), *rcpt.lock().unwrap());
                let talks = talks.clone();
                tokio::spawn(async move {
                    let talk = converse(stream, replies.0, replies.1).await;
                    talks.lock().unwrap().push(talk);
                });
            }
        });
        sink
    }

    /// Conversation `n`, once it has ended.
    async fn talk(&self, n: usize) -> String {
        for _ in 0..500 {
            if let Some(t) = self.talks.lock().unwrap().get(n) {
                return t.clone();
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        panic!("conversation {n} never ended");
    }
}

async fn converse(stream: tokio::net::TcpStream, auth: &str, rcpt: &str) -> String {
    use tokio::io::{AsyncBufReadExt as _, AsyncWriteExt as _};
    let (read, mut write) = stream.into_split();
    let mut read = tokio::io::BufReader::new(read);
    let mut talk = String::new();
    let mut line = String::new();
    if write.write_all(b"220 sink.test ESMTP\r\n").await.is_err() {
        return talk;
    }
    loop {
        line.clear();
        match read.read_line(&mut line).await {
            Ok(0) | Err(_) => break,
            Ok(_) => talk.push_str(&line),
        }
        let command = line.trim_end().to_ascii_uppercase();
        let reply = if command.starts_with("EHLO") {
            "250-sink.test\r\n250 AUTH PLAIN LOGIN"
        } else if command.starts_with("AUTH") {
            auth
        } else if command.starts_with("RCPT TO") {
            rcpt
        } else if command == "DATA" {
            if write.write_all(b"354 go on\r\n").await.is_err() {
                break;
            }
            loop {
                line.clear();
                match read.read_line(&mut line).await {
                    Ok(0) | Err(_) => return talk,
                    Ok(_) if line == ".\r\n" => break,
                    Ok(_) => talk.push_str(&line),
                }
            }
            "250 2.0.0 OK queued as T1"
        } else if command == "QUIT" {
            let _ = write.write_all(b"221 2.0.0 bye\r\n").await;
            break;
        } else {
            "250 2.0.0 OK"
        };
        if write
            .write_all(format!("{reply}\r\n").as_bytes())
            .await
            .is_err()
        {
            break;
        }
    }
    talk
}

#[tokio::test(flavor = "multi_thread")]
async fn the_test_message_goes_through_a_real_smtp_conversation() {
    let sink = Sink::start().await;
    let site = Site::open("mail-smtp", Arc::new(axgf_cms::mail::Smtp)).await;
    let port = sink.port.to_string();
    site.set_relay(&[
        ("host", "127.0.0.1"),
        ("port", port.as_str()),
        ("security", "none"),
    ])
    .await;

    // The credentials refused: said so, and mail paused.
    *sink.auth.lock().unwrap() = "535 5.7.8 Username and Password not accepted";
    let html = unescaped(
        &expect_status(
            site.send_test("cousin@example.org").await,
            StatusCode::OK,
            "a refusal",
        )
        .await,
    );
    assert!(
        html.contains("127.0.0.1 refused the user name or the password"),
        "{html}"
    );
    assert!(!html.contains(RELAY_SECRET));
    sink.talk(0).await;
    assert_eq!(site.state.mailer().ledger().paused.as_deref(), Some("auth"));

    // Accepted: the pause lifts, and the message is whole.
    *sink.auth.lock().unwrap() = "235 2.7.0 Accepted";
    let html = unescaped(
        &expect_status(
            site.send_test("cousin@example.org").await,
            StatusCode::OK,
            "accepted",
        )
        .await,
    );
    assert!(html.contains("Accepted by 127.0.0.1"), "{html}");
    assert_eq!(site.state.mailer().ledger().paused, None);
    let talk = sink.talk(1).await;
    for part in [
        "MAIL FROM:<family@family.example>",
        "RCPT TO:<cousin@example.org>",
        "From: =?utf-8?",
        "To: cousin@example.org",
        // Only the word that needs it is encoded: the dash.
        "Subject: Test message =?utf-8?b?4oCU?=",
        "Auto-Submitted: auto-generated",
        "multipart/alternative",
        "multipart/related",
        "Content-Type: text/plain",
        "Content-Type: text/html",
        "Content-ID: <logo>",
        // Of the sender's domain, never of this machine's name.
        "@family.example>",
    ] {
        assert!(talk.contains(part), "{part} in:\n{talk}");
    }
    assert!(
        talk.lines()
            .any(|l| l.starts_with("Message-ID: <") && l.ends_with("@family.example>")),
        "{talk}"
    );
    assert!(!talk.to_ascii_lowercase().contains("\nbcc:"), "{talk}");
    assert_eq!(
        site.state.mailer().ledger().last_ok.unwrap().message,
        "test"
    );

    // The address refused: said so, and nothing paused.
    *sink.rcpt.lock().unwrap() = "550 5.1.1 No such user here";
    let html = unescaped(
        &expect_status(
            site.send_test("nobody@example.org").await,
            StatusCode::OK,
            "no such user",
        )
        .await,
    );
    assert!(html.contains("127.0.0.1 refused that address"), "{html}");
    let ledger = site.state.mailer().ledger();
    assert_eq!(ledger.last_failure.unwrap().failure, "recipient");
    assert_eq!(ledger.paused, None);
    assert!(
        !std::fs::read_to_string(site.bundle.with_extension("mailstate"))
            .unwrap()
            .contains("nobody@"),
        "the relay's reply is kept without the address"
    );
}

// ---------------------------------------------------------------------------
// Every language.

/// The keys of the English catalogue's mail block, which runs to its end.
fn mail_keys() -> Vec<String> {
    let en = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/locales/en.ftl"))
        .expect("the English catalogue");
    let start = en.find("## Mail (docs/adr/0048)").expect("the mail block");
    en[start..]
        .lines()
        .filter_map(|l| l.split_once(" = ").map(|(k, _)| k))
        .filter(|k| {
            k.starts_with(|c: char| c.is_ascii_lowercase())
                && k.chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        })
        .map(str::to_string)
        .collect()
}

#[test]
fn every_text_of_mail_is_written_in_every_language() {
    let keys = mail_keys();
    assert!(keys.len() > 140, "{} keys", keys.len());
    let mut missing = Vec::new();
    for locale in axgf_cms::i18n::LOCALES {
        for key in &keys {
            if axgf_cms::i18n::translate_marked(locale.tag, key, None).1 {
                missing.push(format!("{} {key}", locale.tag));
            }
        }
    }
    assert!(
        missing.is_empty(),
        "not translated:\n{}",
        missing.join("\n")
    );
}

#[test]
fn every_message_reads_whole_in_every_language() {
    use axgf_cms::mail_messages as m;
    let link = format!("{PUBLIC}/mail/set/abc.def.ghi");
    let sign_in = format!("{PUBLIC}/admin/login");
    for locale in axgf_cms::i18n::LOCALES {
        let head = m::Letterhead {
            lang: locale.tag,
            theme: "canopee".into(),
            family: "Ndiaye-Lefèvre".into(),
            site_url: Some(PUBLIC.into()),
        };
        let shapes = [
            m::reset(&head, "ada", &link),
            m::welcome_with_link(&head, "ada", &link, 1),
            m::welcome_with_link(&head, "ada", &link, 7),
            m::welcome_with_services(&head, "ada", "Google", &sign_in),
            m::password_changed(&head, "ada", "2026-11-12 08:00 UTC", true, &sign_in),
            m::password_changed(&head, "ada", "2026-11-12 08:00 UTC", false, &sign_in),
            m::service_only(&head, "ada", "Google", &sign_in),
            m::test(&head),
        ];
        for shape in &shapes {
            let (html, text) = m::lay_out(shape, &head);
            let tag = locale.tag;
            for part in [&shape.subject, &text, &html] {
                assert!(
                    !part.contains("mail-") && !part.contains("{ $"),
                    "{tag}: a key or a placeable left in:\n{part}"
                );
            }
            assert!(
                text.contains("Ndiaye-Lefèvre") || shape.subject.contains("Ndiaye-Lefèvre"),
                "{tag}: the family is named"
            );
            if let Some(action) = &shape.action {
                assert!(text.contains(&action.url), "{tag}: the link written out");
            }
            let rtl = matches!(tag, "ar" | "he");
            assert_eq!(html.contains(r#"dir="rtl""#), rtl, "{tag}: direction");
        }
    }
}

#[tokio::test]
async fn the_pages_of_mail_send_their_address_to_no_one() {
    let site = Site::open("mail-referrer", Arc::new(Recorder::new())).await;
    for uri in ["/mail/forgot", "/mail/set/abc.def.ghi"] {
        let r = site.visit(uri).await;
        assert_eq!(
            r.headers()[header::REFERRER_POLICY],
            "strict-origin",
            "{uri}"
        );
        assert_eq!(
            r.headers().get_all(header::REFERRER_POLICY).iter().count(),
            1
        );
    }
    let r = site.visit("/admin/login").await;
    assert_eq!(r.headers()[header::REFERRER_POLICY], "same-origin");
}

#[tokio::test]
async fn undoing_a_change_does_not_bring_a_link_back() {
    let rec = Recorder::new();
    let site = Site::open("mail-epoch", Arc::new(rec.clone()))
        .await
        .with_relay()
        .await;
    let r = site
        .admin_post(
            "/admin/users",
            &[
                ("username", "lea"),
                ("password", ""),
                ("email", "lea@example.org"),
                ("role", "viewer"),
            ],
        )
        .await;
    expect_status(r, StatusCode::OK, "made").await;
    let link = link_in(&wait_for(&rec, 1).await[0]);
    let id = site
        .state
        .acl_read(|acl| acl.active("lea").map(|u| u.id.clone()))
        .unwrap();
    let edit = |status| {
        vec![
            ("role", "viewer"),
            ("status", status),
            ("email", "lea@example.org"),
            ("password", ""),
        ]
    };
    // Disabled, then enabled again: the welcome stays dead.
    for status in ["disabled", "active"] {
        let r = site
            .admin_post(&format!("/admin/users/{id}"), &edit(status))
            .await;
        expect_status(r, StatusCode::OK, status).await;
        let html = body_string(site.visit(&link).await).await;
        assert!(
            html.contains("This link is no longer good"),
            "{status}: {html}"
        );
    }
    let r = site.visitor_post(&link, &NEW).await;
    assert_eq!(r.status(), StatusCode::BAD_REQUEST);
    // A change that ends nothing — the same address in other letters — keeps
    // a new link good.
    let r = site
        .admin_post(&format!("/admin/users/{id}/welcome"), &[])
        .await;
    expect_status(r, StatusCode::OK, "again").await;
    site.settle().await;
}

#[tokio::test]
async fn a_message_held_back_gives_its_limits_back() {
    let rec = Recorder::new();
    let site = Site::open("mail-refund", Arc::new(rec.clone())).await;
    site.account(
        "ada",
        "an-old-password-long-enough",
        Some("ada@example.org"),
    );
    site.account(
        "bob",
        "an-old-password-long-enough",
        Some("bob@example.org"),
    );
    site.set_relay(&[("daily_limit", "1")]).await;
    site.set_public().await;

    site.visitor_post("/mail/forgot", &[("who", "ada")]).await;
    site.settle().await;
    site.visitor_post("/mail/forgot", &[("who", "bob")]).await;
    site.settle().await;
    assert_eq!(rec.sent().len(), 1, "bob's held back by the ceiling");

    // The ceiling raised: bob asks again at once, and his link goes.
    site.set_relay(&[("daily_limit", "5")]).await;
    site.visitor_post("/mail/forgot", &[("who", "bob")]).await;
    let sent = wait_for(&rec, 2).await;
    assert_eq!(sent[1].to, "bob@example.org");
}

#[tokio::test]
async fn a_new_relay_needs_its_password_typed_again() {
    let site = Site::open("mail-endpoint", Arc::new(Recorder::new())).await;
    site.set_relay(&[]).await;
    for elsewhere in [
        vec![("host", "smtp.elsewhere.example"), ("password", "")],
        vec![("username", "other@family.example"), ("password", "")],
        vec![("security", "tls"), ("port", "465"), ("password", "")],
    ] {
        let r = site.save_relay(&elsewhere).await;
        let html = expect_status(r, StatusCode::UNPROCESSABLE_ENTITY, "kept for its relay").await;
        assert!(html.contains("needs the password typed again"), "{html}");
        let kept = site.state.site().mail;
        assert_eq!(
            (kept.host.as_str(), kept.password.as_str()),
            ("relay.test", RELAY_SECRET)
        );
    }
    // With the password typed again, the new relay is kept.
    site.set_relay(&[
        ("host", "smtp.elsewhere.example"),
        ("password", "another-secret"),
    ])
    .await;
    assert_eq!(site.state.site().mail.host, "smtp.elsewhere.example");
}

#[tokio::test]
async fn a_test_says_where_it_comes_from_and_not_that_an_account_exists() {
    let rec = Recorder::new();
    let site = Site::open("mail-test-footer", Arc::new(rec.clone()))
        .await
        .with_relay()
        .await;
    let r = site.send_test("anyone@example.org").await;
    let html = expect_status(r, StatusCode::OK, "the test").await;
    assert!(html.contains("Accepted by relay.test"), "{html}");
    let m = &rec.sent()[0];
    assert_eq!(m.kind, "test");
    assert!(
        m.text
            .contains("This test was sent from its mail settings."),
        "{}",
        m.text
    );
    assert!(
        !m.text.contains("an account there has this address"),
        "{}",
        m.text
    );
}

#[tokio::test]
async fn a_configured_relay_refused_says_why_in_the_checks() {
    let dir = scratch("mail-env-refused");
    let state = axgf_cms::AppState::load_or_create(&dir.join("family.axgf"), TOKEN.to_string())
        .unwrap()
        .with_mailer(Arc::new(Recorder::new()))
        // An API key for a user name, and no sender: nobody to send as.
        .with_mail_env(Some("smtp://apikey:SECRET-55@smtp.relay.example:587"), None);
    state
        .site_mutate(|s| s.doors = axgf_cms::site::Doors::everyone())
        .unwrap();
    let app = axgf_cms::router(Arc::new(state));
    let admin = format!("{}; {LANG}", admin_cookie(&app).await);
    let html = body_string(get_with_cookie(&app, "/admin/infrastructure", &admin).await).await;
    assert!(
        html.contains(
            "The sender, and a reply address if there is one, must each be one mail address."
        ),
        "{html}"
    );
    assert!(!html.contains("SECRET-55"));
}

#[tokio::test]
async fn the_notice_says_which_welcome_went_or_that_none_did() {
    let rec = Recorder::new();
    let site = Site::open("mail-welcome-kinds", Arc::new(rec.clone()))
        .await
        .with_relay()
        .await;
    site.state
        .site_mutate(|s| {
            s.sign_in.password = axgf_cms::site::PasswordPolicy::NonAdmins;
            s.providers.google = axgf_cms::site::Provider {
                enabled: true,
                client_id: "id.apps.example".into(),
                client_secret: "not-a-real-secret".into(),
                ..Default::default()
            };
        })
        .unwrap();
    let make = |username: &'static str, role: &'static str| {
        vec![
            ("username", username),
            ("password", ""),
            ("email", "family@example.org"),
            ("role", role),
        ]
    };

    // An administrator, who may not use a password here: the services.
    let r = site
        .admin_post("/admin/users", &make("root2", "admin"))
        .await;
    let html = expect_status(r, StatusCode::OK, "an administrator").await;
    assert!(
        html.contains("Its welcome, naming the services to sign in with"),
        "{html}"
    );
    let m = &wait_for(&rec, 1).await[0];
    assert!(
        m.text.contains("Google") && !m.text.contains("/mail/set/"),
        "{}",
        m.text
    );

    // The same address again at once: nothing goes, and the notice says so.
    let r = site
        .admin_post("/admin/users", &make("jean", "viewer"))
        .await;
    let html = expect_status(r, StatusCode::OK, "a second account").await;
    assert!(html.contains("Their password is"), "{html}");
    assert!(html.contains("Nothing sent to jean"), "{html}");
    site.settle().await;
    assert_eq!(rec.sent().len(), 1);
}
