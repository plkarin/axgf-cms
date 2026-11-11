//! The journal lines the operator's tools read (docs/adr/0047 §7), through
//! the router: `sign-in failed from <address>`, which the README's check
//! greps for, and `sign-in jail: <address> shut out for N s`, which its
//! fail2ban filter bans on.
//!
//! A test binary of its own, with one test: tracing caches whether a call
//! site is listened to across every thread of the process, so a subscriber
//! set for one test thread hears nothing reliable while others run beside it.

mod common;

use axum::body::Body;
use axum::http::{header, Request};
use tower::ServiceExt as _;

/// What the subscriber wrote.
#[derive(Clone, Default)]
struct Captured(std::sync::Arc<std::sync::Mutex<Vec<u8>>>);

impl std::io::Write for Captured {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl<'a> tracing_subscriber::fmt::MakeWriter<'a> for Captured {
    type Writer = Captured;
    fn make_writer(&'a self) -> Self::Writer {
        self.clone()
    }
}

#[tokio::test]
async fn the_journal_names_the_address_of_a_failure_and_of_a_term() {
    let lines = Captured::default();
    tracing::subscriber::set_global_default(
        tracing_subscriber::fmt()
            .with_writer(lines.clone())
            .with_ansi(false)
            .without_time()
            .finish(),
    )
    .expect("the only subscriber of this binary");

    let (app, _dir) = common::app_with_empty_bundle("journal-lines");
    // Three more than it takes: those are refused during the term, and a
    // term is written once, not once an attempt.
    for i in 0..axgf_cms::jail::MAX_FAILURES + 3 {
        let req = Request::builder()
            .uri("/admin/login")
            .method("POST")
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .header("x-forwarded-for", "203.0.113.11")
            .body(Body::from(format!("username=nobody&password=wrong{i}")))
            .unwrap();
        app.clone().oneshot(req).await.expect("request");
    }
    let out = String::from_utf8_lossy(&lines.0.lock().unwrap()).into_owned();
    let failed = out
        .lines()
        .filter(|l| l.contains("a sign-in failed from 203.0.113.11"))
        .count();
    assert_eq!(failed, axgf_cms::jail::MAX_FAILURES as usize, "{out}");
    let terms: Vec<&str> = out
        .lines()
        .filter(|l| l.contains("sign-in jail:"))
        .collect();
    assert_eq!(terms.len(), 1, "{out}");
    assert!(
        terms[0].contains(
            "sign-in jail: 203.0.113.11 shut out for 900 s, after 8 failed sign-ins (term 1)"
        ),
        "{out}"
    );
}
