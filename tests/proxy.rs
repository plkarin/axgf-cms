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
//! Where nginx is installed, `nginx -t` runs over the real file.

mod common;

use std::path::{Path, PathBuf};
use std::process::Command;

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

    // Above the product's own 10 MB limit, so an oversized upload is refused
    // by the page that explains itself rather than by a bare 413.
    let limit = conf
        .lines()
        .find_map(|l| l.trim().strip_prefix("client_max_body_size "))
        .expect("a body limit is set");
    assert_eq!(limit.trim_end_matches(';'), "12m");

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

#[test]
fn the_caddyfile_says_the_same_things() {
    let conf = read("deploy/proxy/Caddyfile");
    assert!(conf.contains("reverse_proxy 127.0.0.1:8080"));
    assert!(conf.contains("max_size 12MB"), "the same 12 MB ceiling");
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
