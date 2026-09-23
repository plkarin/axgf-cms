//! Shared test harness: an app instance over a throwaway bundle.
//!
//! `axgf-cms` is a binary crate, so integration tests drive it the way a
//! browser would — through the router, with axum's `oneshot`.

#![allow(dead_code)]

use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use axum::body::Body;
use axum::http::{header, Request, Response, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

pub const TOKEN: &str = "test-token-abc123";

static COUNTER: AtomicUsize = AtomicUsize::new(0);

/// The base every test directory is made under.
///
/// `CARGO_TARGET_TMPDIR` is `target/tmp`, and cargo never touches it again:
/// whatever a test leaves there stays there until somebody notices. On the
/// machine this was written for that was 8.6 GB of extracted photographs, on
/// the same filesystem the application saves its bundle to — so the test suite
/// was, slowly, arranging for the product to run out of disk.
pub fn scratch_base() -> PathBuf {
    option_env!("CARGO_TARGET_TMPDIR")
        .map(PathBuf::from)
        .unwrap_or_else(std::env::temp_dir)
}

/// A scratch directory that deletes itself.
///
/// Dropped at the end of the test that made it, including when that test
/// panics — the unwind runs destructors, which is exactly the case the old
/// `remove_dir_all` at the end of a test body did not cover, and a failing
/// test is the one most likely to have written a lot.
///
/// It dereferences to a path so that call sites read as they did when this was
/// a `PathBuf`: `dir.join("x")`, `fs::read(&path)`, `path.display()`.
#[derive(Debug)]
pub struct Scratch {
    dir: PathBuf,
    /// What the value points at: the directory itself, or the bundle inside it.
    subject: PathBuf,
}

impl Scratch {
    /// The directory, whatever this value points at.
    pub fn dir(&self) -> &Path {
        &self.dir
    }

    /// The same scratch, now pointing at `subject`.
    ///
    /// What a fixture helper returns: the guard keeps the directory alive for
    /// as long as the caller holds it, and the value reads as the bundle path.
    pub fn pointing_at(mut self, subject: PathBuf) -> Self {
        // Taking the directory out of the old value is what stops its `Drop`
        // from deleting the very directory being handed on: `Drop` returns
        // early on an empty one.
        let dir = std::mem::take(&mut self.dir);
        Self { dir, subject }
    }

    /// A path this harness did not create and must never delete.
    ///
    /// For the benchmark that runs against the operator's real bundle when
    /// `AXGF_CMS_BENCH_BUNDLE` points at one: the same code path has to accept
    /// both a throwaway fixture and a file worth 435 MB of somebody's family.
    pub fn external(path: &Path) -> Self {
        Self {
            dir: PathBuf::new(),
            subject: path.to_path_buf(),
        }
    }

    /// Give up ownership: the directory is no longer removed on drop.
    ///
    /// For the two tests that deliberately outlive their files — a restore
    /// that has to still be there afterwards, a leak check that inspects the
    /// wreckage.
    pub fn keep(mut self) -> PathBuf {
        let dir = std::mem::take(&mut self.dir);
        std::mem::forget(self);
        dir
    }
}

impl Deref for Scratch {
    type Target = Path;
    fn deref(&self) -> &Path {
        &self.subject
    }
}

impl AsRef<Path> for Scratch {
    fn as_ref(&self) -> &Path {
        &self.subject
    }
}

/// So a scratch path can be handed straight to `Command::env`, which wants an
/// `OsStr` and does not see through `Deref`.
impl AsRef<std::ffi::OsStr> for Scratch {
    fn as_ref(&self) -> &std::ffi::OsStr {
        self.subject.as_os_str()
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        if self.dir.as_os_str().is_empty() {
            return;
        }
        // Best-effort: a test that has arranged for a directory to be
        // unremovable has bigger news than this.
        let _ = std::fs::remove_dir_all(&self.dir);
    }
}

/// A unique scratch directory for one test, removed when the value is dropped.
pub fn scratch(tag: &str) -> Scratch {
    let n = COUNTER.fetch_add(1, Ordering::SeqCst);
    let dir = scratch_base().join(format!("axgf-cms-it-{}-{}-{}", std::process::id(), tag, n));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("create scratch dir");
    Scratch {
        subject: dir.clone(),
        dir,
    }
}

/// Build an app over a fresh empty bundle, returning the router and the path.
///
/// The second value owns the directory: hold it for as long as the app is
/// used, which every caller does anyway because it is the bundle path.
pub fn app_with_empty_bundle(tag: &str) -> (axum::Router, Scratch) {
    let dir = scratch(tag);
    let path = dir.join("family.axgf");
    let app = axgf_cms::app(&path, TOKEN).expect("build app");
    (app, dir.pointing_at(path))
}

/// Build an app over a copy of an existing bundle file.
pub fn app_with_bundle(tag: &str, source: &Path) -> (axum::Router, Scratch) {
    let dir = scratch(tag);
    let path = dir.join("family.axgf");
    std::fs::copy(source, &path).expect("copy source bundle");
    let app = axgf_cms::app(&path, TOKEN).expect("build app");
    (app, dir.pointing_at(path))
}

/// Issue a GET and return the response.
pub async fn get(app: &axum::Router, uri: &str) -> Response<Body> {
    app.clone()
        .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
        .await
        .expect("request")
}

/// Sign in with the emergency token and return the session cookie.
///
/// The token used to be replayed as a cookie on every request, which is how
/// every test here used to authenticate. It is no longer a credential — it
/// buys a session through the login form, throttled and logged like any other
/// sign-in — so the harness does what a browser does: one POST, then the
/// cookie that came back.
pub async fn admin_cookie(app: &axum::Router) -> String {
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/admin/login")
                .method("POST")
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .body(Body::from(format!("token={TOKEN}")))
                .unwrap(),
        )
        .await
        .expect("login request");
    assert!(
        resp.status().is_redirection(),
        "the emergency token should open a session, got {}",
        resp.status()
    );
    let raw = resp
        .headers()
        .get(header::SET_COOKIE)
        .and_then(|v| v.to_str().ok())
        .expect("a session cookie");
    raw.split(';').next().unwrap_or("").to_string()
}

/// Issue a GET as a signed-in administrator.
pub async fn get_admin(app: &axum::Router, uri: &str) -> Response<Body> {
    let cookie = admin_cookie(app).await;
    get_with_cookie(app, uri, &cookie).await
}

/// Issue a GET carrying an arbitrary cookie — a theme, a language, a
/// preference — so a test can ask for a page the way a reader would.
pub async fn get_with_cookie(app: &axum::Router, uri: &str, cookie: &str) -> Response<Body> {
    app.clone()
        .oneshot(
            Request::builder()
                .uri(uri)
                .header(header::COOKIE, cookie)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("request")
}

/// Issue a form POST, optionally authenticated.
pub async fn post_form(app: &axum::Router, uri: &str, body: &str, admin: bool) -> Response<Body> {
    if admin {
        let cookie = admin_cookie(app).await;
        return post_form_as(app, &cookie, uri, body).await;
    }
    let b = Request::builder()
        .uri(uri)
        .method("POST")
        .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded");
    app.clone()
        .oneshot(b.body(Body::from(body.to_string())).unwrap())
        .await
        .expect("request")
}

/// Issue a form POST carrying an arbitrary cookie — a session, a preference —
/// so a test can act as somebody other than the emergency token.
pub async fn post_form_as(
    app: &axum::Router,
    cookie: &str,
    uri: &str,
    body: &str,
) -> Response<Body> {
    app.clone()
        .oneshot(
            Request::builder()
                .uri(uri)
                .method("POST")
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .header(header::COOKIE, cookie)
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .expect("request")
}

/// Collect a response body into a string.
pub async fn body_string(resp: Response<Body>) -> String {
    let bytes = resp.into_body().collect().await.expect("body").to_bytes();
    String::from_utf8_lossy(&bytes).into_owned()
}

/// Collect a response body into bytes.
pub async fn body_bytes(resp: Response<Body>) -> Vec<u8> {
    resp.into_body()
        .collect()
        .await
        .expect("body")
        .to_bytes()
        .to_vec()
}

/// Assert a status, printing the body when it does not match.
pub async fn expect_status(resp: Response<Body>, want: StatusCode, what: &str) -> String {
    let got = resp.status();
    let body = body_string(resp).await;
    assert_eq!(
        got,
        want,
        "{what}: expected {want}, got {got}\n{}",
        body.chars().take(600).collect::<String>()
    );
    body
}
