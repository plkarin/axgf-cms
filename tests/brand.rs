//! ax·genealogy's logo (docs/adr/0006): one drawing, shown wherever the site
//! names itself. In the masthead under every theme, coloured under Canopée
//! only. On the sign-in page. As the tab's icon, the phone's home-screen icon
//! and the manifest's.

mod common;

use axum::http::{header, StatusCode};
use common::*;

/// Every `d="…"` in a piece of SVG, in order.
fn paths(svg: &str) -> Vec<&str> {
    svg.match_indices(" d=\"")
        .map(|(i, _)| {
            let rest = &svg[i + 4..];
            &rest[..rest.find('"').expect("a closing quote")]
        })
        .collect()
}

#[tokio::test]
async fn the_icons_a_browser_and_a_phone_ask_for_are_the_logo() {
    let (app, _s) = app_with_empty_bundle("brand-icons");
    for (uri, kind) in [
        ("/favicon.svg", "image/svg+xml"),
        ("/favicon.ico", "image/x-icon"),
        ("/apple-touch-icon.png", "image/png"),
        ("/static/img/logo-mark.svg", "image/svg+xml"),
        ("/static/img/icon-192.png", "image/png"),
        ("/static/img/icon-512.png", "image/png"),
        ("/site.webmanifest", "application/manifest+json"),
    ] {
        let resp = get(&app, uri).await;
        assert_eq!(resp.status(), StatusCode::OK, "{uri}");
        assert_eq!(resp.headers()[header::CONTENT_TYPE], kind, "{uri}");
    }
    // The page names them, and the emoji that stood in for a logo is gone.
    let page = body_string(get(&app, "/about").await).await;
    for link in [
        r#"<link rel="icon" href="/favicon.ico" sizes="32x32">"#,
        r#"<link rel="icon" href="/favicon.svg" type="image/svg+xml">"#,
        r#"<link rel="apple-touch-icon" href="/apple-touch-icon.png">"#,
        r#"<link rel="manifest" href="/site.webmanifest">"#,
    ] {
        assert!(page.contains(link), "{link}");
    }
    assert!(
        !page.contains("&#127795;"),
        "the tree emoji is not the icon"
    );
    // Every icon the manifest names is served.
    let manifest: serde_json::Value =
        serde_json::from_slice(&body_bytes(get(&app, "/site.webmanifest").await).await)
            .expect("the manifest is JSON");
    assert_eq!(manifest["name"], "ax·genealogy");
    let icons = manifest["icons"].as_array().expect("icons");
    assert!(!icons.is_empty());
    for icon in icons {
        let src = icon["src"].as_str().expect("src");
        let png = body_bytes(get(&app, src).await).await;
        assert!(png.starts_with(b"\x89PNG"), "{src} is a PNG");
    }
}

/// The masthead draws the same logo in every theme, each in its own colours;
/// a page that names no theme draws it in ink, like the wordmark beside it.
#[tokio::test]
async fn the_masthead_carries_the_logo_under_every_theme() {
    let (app, _s) = app_with_empty_bundle("brand-masthead");
    for theme in axgf_cms::theme::THEMES.iter().map(|t| t.id) {
        let page =
            body_string(get_with_cookie(&app, "/about", &format!("axgf_theme={theme}")).await)
                .await;
        let bar = &page[page
            .find(r#"<header class="masthead">"#)
            .expect("a masthead")..];
        let bar = &bar[..bar.find("</header>").expect("its end")];
        assert!(
            bar.contains(r#"<svg class="logo-mark brand-mark""#),
            "{theme}: the logo is in the masthead"
        );
        assert!(bar.contains(r#"aria-hidden="true""#), "{theme}: decorative");
    }
    let css = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("static/app.css"),
    )
    .expect("app.css");
    // The kit's three themes colour it, each from its own `--logo-*`; the
    // ink is the floor under them.
    for line in css.lines().filter(|l| l.contains("var(--logo-")) {
        assert!(
            line.trim_start().starts_with(
                ":is([data-theme=\"canopee\"], [data-theme=\"fusion\"], [data-theme=\"galaxy\"], [data-theme=\"summit\"], [data-theme=\"clarity\"], [data-theme=\"genealogy\"]) .logo-mark"
            ),
            "the logo's colours are the kit's alone: {line}"
        );
    }
    assert!(css.contains(".logo-mark stop { stop-color: currentColor; }"));
}

/// The logo drawn inline is the logo in the file: the same strokes, leaf for
/// leaf, so the two copies cannot drift apart.
#[test]
fn the_inline_logo_is_the_drawing_in_the_file() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let file = std::fs::read_to_string(root.join("static/img/logo-mark.svg")).expect("the logo");
    let inline = std::fs::read_to_string(root.join("templates/_logo.html")).expect("the macro");
    assert_eq!(paths(&inline), paths(&file));
    assert_eq!(
        inline.matches("<ellipse").count(),
        file.matches("<ellipse").count()
    );
    for leaf in file.split("<ellipse ").skip(1) {
        let geometry = &leaf[..leaf.find(" fill=").expect("a fill")];
        assert!(inline.contains(geometry), "{geometry}");
    }
    // What a browser fetches carries no metadata: the manifest that came with
    // the drawing is in the operator's original, not on every page.
    assert!(!file.contains("<metadata"));
}

#[tokio::test]
async fn the_sign_in_page_shows_the_logo_with_ids_of_its_own() {
    let (app, _s) = app_with_empty_bundle("brand-login");
    let page = body_string(get(&app, "/admin/login").await).await;
    assert!(page.contains(r#"<div class="login-mark"><svg class="logo-mark login-logo""#));
    // Two logos on one page: each gradient keeps its own id.
    for id in ["mh-lo", "mh-up", "lg-lo", "lg-up"] {
        assert_eq!(page.matches(&format!(r#"id="{id}""#)).count(), 1, "{id}");
    }
}
