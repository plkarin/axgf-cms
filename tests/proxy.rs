//! The shipped reverse-proxy configurations.
//!
//! Both were tested by running them — nginx 1.24 and Caddy 2.6 in front of a
//! live instance, over TLS, with a 9 MB upload, an 11 MB refusal and a
//! thirteen-second save. What a test in this repository can do afterwards is
//! stop the parts that made those pass from quietly going away: the header
//! that makes the session cookie `Secure`, the body limit that lets the
//! product refuse an oversized upload in its own words, and the read timeout
//! without which a long save reaches the reader as a 504.
//!
//! Their body limit went from 12 MB to 210 MB in v1.10.1 (docs/adr/0037),
//! with the product's own limit from 10 MB to 50 MB, without running them
//! again.
//!
//! In v1.15.1 both gained a way through for the import of a whole family
//! (docs/adr/0042): `/admin/setup` and the addresses under it, up to just
//! above the archive's 4 GB, where the 210 MB answered a few hundred
//! megabytes with a bare 413 — found on ax-genealogy.com with a 428 MB
//! archive. The nginx location has no trailing slash: with one, nginx
//! answered `/admin/setup` with a 301 to `/admin/setup/`, which the
//! application does not serve.
//!
//! The nginx file also counts sign-in attempts per address at the door, the
//! posts only, beside the application's own jail (docs/adr/0047), and sets
//! the headers the application reads once for every location: nginx gives a
//! location the server's `proxy_set_header` lines only if it sets none.
//!
//! Where nginx is installed, `nginx -t` runs over the real file.

mod common;

use std::path::{Path, PathBuf};
use std::process::Command;

/// The proxies' 210 MB hold the largest request the application reads.
const _: () = assert!(axgf_cms::documents::MAX_REQUEST <= 210 * 1024 * 1024);
/// Their 4200 MiB on the import hold an archive at the application's default
/// limit and the megabyte of form around it: that is, the default is under
/// 4200 MiB.
const _: () = assert!(axgf_cms::limits::ARCHIVE.default_mb < 4200);

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read(rel: &str) -> String {
    let p = repo_root().join(rel);
    std::fs::read_to_string(&p).unwrap_or_else(|e| panic!("read {}: {e}", p.display()))
}

#[test]
fn the_nginx_config_forwards_what_the_application_reads() {
    let conf = read("deploy/proxy/nginx-axgf-cms.conf");

    // Without this the session cookie is issued without `Secure`, over TLS,
    // and nothing anywhere says so.
    assert!(
        conf.contains("proxy_set_header X-Forwarded-Proto $scheme;"),
        "the proxy must tell the application it is behind TLS"
    );
    // Set, not appended: an appended one is client-supplied, and a client that
    // picks its own value picks its own login-throttle bucket.
    assert!(
        conf.contains("proxy_set_header X-Forwarded-For   $remote_addr;"),
        "X-Forwarded-For must be set from the peer, not carried through"
    );
    assert!(conf.contains("proxy_set_header Host              $host;"));

    // Above the 200 MB a request with a file may be (each file at most 50 MB,
    // a larger one read to its end), so an oversized upload is refused by the
    // page that explains itself rather than by a bare 413.
    let limit = conf
        .lines()
        .find_map(|l| l.trim().strip_prefix("client_max_body_size "))
        .expect("a body limit is set");
    assert_eq!(limit.trim_end_matches(';'), "210m");

    // A save on a 435 MB bundle is seconds; the default 60 s read timeout has
    // no margin over a slow disk. Cutting this to 5 s turns the same save into
    // a 504, which is how it was tested.
    assert!(
        conf.contains("proxy_read_timeout    300s;"),
        "a long save must not time out at the proxy"
    );

    // The application sets these on every response. Two CSP headers are
    // enforced as the intersection of the two, which breaks the page in a way
    // nothing reports.
    for header in [
        "Content-Security-Policy",
        "X-Frame-Options",
        "Referrer-Policy",
    ] {
        assert!(
            !conf.contains(&format!("add_header {header}")),
            "{header} is the application's to set; adding it here duplicates it"
        );
    }
    // Except the one the application cannot know about.
    assert!(conf.contains("add_header Strict-Transport-Security"));
}

/// The location that opens with `head`, from its first line to its closing
/// brace.
fn location<'a>(conf: &'a str, head: &str) -> &'a str {
    let start = conf.find(head).unwrap_or_else(|| panic!("no {head}"));
    let end = conf[start..]
        .find("\n    }")
        .expect("the location is closed");
    &conf[start..start + end]
}

#[test]
fn every_location_inherits_the_headers_the_application_reads() {
    // A location that set a proxy_set_header of its own would lose the
    // server's four — the cookie's `Secure` and the per-client counting with
    // them — and nothing would say so.
    let conf = read("deploy/proxy/nginx-axgf-cms.conf");
    let tls = &conf[conf.find("listen 443").expect("the TLS server")..];
    let heads: Vec<&str> = tls
        .lines()
        .map(str::trim)
        .filter(|l| l.starts_with("location ") && l.ends_with('{'))
        .collect();
    for head in &heads {
        let block = location(tls, head);
        assert!(
            !block.contains("proxy_set_header"),
            "this location would drop the server's headers:\n{block}"
        );
        assert!(
            block.contains("proxy_pass http://127.0.0.1:8080;"),
            "{block}"
        );
    }
    let locations = heads.len();
    assert_eq!(locations, 4, "/, the sign-in, the registration, the import");
}

#[test]
fn the_nginx_config_counts_sign_in_attempts_at_the_door() {
    let conf = read("deploy/proxy/nginx-axgf-cms.conf");
    // The posts only: an empty key is not limited, so the page is never
    // refused, and the key is the address nginx sees.
    assert!(conf.contains(
        "map $request_method $axgf_sign_in {\n    POST    $binary_remote_addr;\n    default \"\";\n}"
    ));
    assert!(conf.contains("limit_req_zone $axgf_sign_in zone=axgf_sign_in:10m rate=10r/m;"));
    for head in ["location = /admin/login {", "location ^~ /register {"] {
        let block = location(&conf, head);
        assert!(
            block.contains("limit_req zone=axgf_sign_in burst=10 nodelay;"),
            "{block}"
        );
        assert!(block.contains("limit_req_status 429;"), "{block}");
    }
}

#[test]
fn the_nginx_config_lets_a_whole_family_through_to_its_import() {
    let conf = read("deploy/proxy/nginx-axgf-cms.conf");
    // With a trailing slash, nginx answers /admin/setup — the import page —
    // with a 301 to /admin/setup/, which the application does not serve.
    assert!(
        !conf.contains("location ^~ /admin/setup/"),
        "the import's prefix must not end in a slash"
    );
    let import = location(&conf, "location ^~ /admin/setup {");
    assert!(
        import.contains("client_max_body_size 4200m;"),
        "an archive of the application's default size must get through"
    );
    assert!(
        import.contains("proxy_read_timeout    900s;"),
        "an import must have the time to be read"
    );
    // Streamed: nginx writes nothing to disk for a visitor the application
    // turns away at the first bytes.
    assert!(import.contains("proxy_request_buffering off;"), "{import}");
}

#[tokio::test]
async fn the_application_serves_the_import_page_without_a_trailing_slash() {
    // Why the proxies' prefix has none: the page is /admin/setup, and
    // /admin/setup/ is nothing.
    let (app, _dir) = common::app_with_empty_bundle("proxy-setup-path");
    let page = common::get(&app, "/admin/setup").await;
    assert_ne!(page.status(), axum::http::StatusCode::NOT_FOUND);
    let slashed = common::get(&app, "/admin/setup/").await;
    assert_eq!(slashed.status(), axum::http::StatusCode::NOT_FOUND);
}

#[test]
fn the_caddyfile_says_the_same_things() {
    let conf = read("deploy/proxy/Caddyfile");
    assert!(conf.contains("reverse_proxy 127.0.0.1:8080"));
    assert!(conf.contains("max_size 210MB"), "the same 210 MB ceiling");
    // The import in a `handle` of its own: two `request_body` on one request
    // would both apply, and the 210 MB would win. In MiB, as the application
    // counts: 4200MB is less than its 4 GB.
    let import = &conf[conf
        .find("handle /admin/setup* {")
        .expect("a handle for the import of a whole family")..];
    let import = &import[..import.find("\n\t}").expect("the handle is closed")];
    assert!(import.contains("max_size 4200MiB"), "{import}");
    assert!(import.contains("response_header_timeout 900s"), "{import}");
    assert!(
        conf.contains("\thandle {"),
        "everything else in a handle of its own, exclusive of the import's"
    );
    assert!(
        conf.contains("header Strict-Transport-Security"),
        "HSTS is the proxy's to add"
    );
    for header in ["Content-Security-Policy", "X-Frame-Options"] {
        assert!(
            !conf.contains(&format!("header {header}")),
            "{header} is the application's to set"
        );
    }
    // Caddy sets the forwarding headers itself; the config must not be relying
    // on a `header_up X-Forwarded-Proto` that was never needed, nor on nothing
    // at all — so the comment that records *why* there is no directive here is
    // load-bearing and checked.
    assert!(
        conf.contains("X-Forwarded-Proto by itself"),
        "say why no header_up is needed, or somebody will add a wrong one"
    );
}

#[test]
fn nginx_accepts_the_file_where_nginx_is_installed() {
    let Some(nginx) = which("nginx") else {
        eprintln!("nginx is not installed here; skipping the syntax check");
        return;
    };
    // The real file, with only what a test cannot supply changed: the name,
    // the certificate, and privileged ports.
    // A guard, not a bare path: the early returns below and a failing
    // `assert!` both leave before any cleanup at the bottom would run.
    let dir = common::scratch("proxy");
    let certs = dir.join("certs");
    std::fs::create_dir_all(&certs).expect("mkdir");
    if !make_self_signed(&certs) {
        eprintln!("no openssl here; skipping the syntax check");
        return;
    }
    let body = read("deploy/proxy/nginx-axgf-cms.conf")
        .replace("server_name tree.example.org;", "server_name 127.0.0.1;")
        .replace(
            "/etc/letsencrypt/live/tree.example.org/fullchain.pem",
            certs.join("cert.pem").to_str().unwrap(),
        )
        .replace(
            "/etc/letsencrypt/live/tree.example.org/privkey.pem",
            certs.join("key.pem").to_str().unwrap(),
        )
        .replace("listen 80;", "listen 18080;")
        .replace("listen [::]:80;", "listen [::]:18080;")
        .replace("listen 443 ssl http2;", "listen 18443 ssl http2;")
        .replace("listen [::]:443 ssl http2;", "listen [::]:18443 ssl http2;");
    let conf = format!(
        "worker_processes 1;\npid {pid};\nerror_log {err};\nevents {{ worker_connections 8; }}\n\
         http {{\n  access_log off;\n  client_body_temp_path {tmp};\n  proxy_temp_path {tmp2};\n\
         {body}\n}}\n",
        pid = dir.join("nginx.pid").display(),
        err = dir.join("error.log").display(),
        tmp = dir.join("body").display(),
        tmp2 = dir.join("proxy").display(),
    );
    let path = dir.join("nginx.conf");
    std::fs::write(&path, conf).expect("write conf");

    let out = Command::new(nginx)
        .args(["-t", "-c"])
        .arg(&path)
        .arg("-p")
        .arg(&dir)
        .output()
        .expect("run nginx -t");
    let text = String::from_utf8_lossy(&out.stderr).into_owned();
    assert!(out.status.success(), "nginx rejected the config:\n{text}");
}

fn which(bin: &str) -> Option<PathBuf> {
    std::env::var_os("PATH").and_then(|paths| {
        std::env::split_paths(&paths)
            .map(|d| d.join(bin))
            .find(|p| p.is_file())
    })
}

fn make_self_signed(dir: &Path) -> bool {
    let Some(openssl) = which("openssl") else {
        return false;
    };
    Command::new(openssl)
        .args([
            "req", "-x509", "-newkey", "rsa:2048", "-nodes", "-days", "1", "-subj", "/CN=test",
        ])
        .arg("-keyout")
        .arg(dir.join("key.pem"))
        .arg("-out")
        .arg(dir.join("cert.pem"))
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}
