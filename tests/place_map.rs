//! The place-editor map, and the decision not to load one by default.
//!
//! Tiles are fetched by the reader's browser, not by this process. That is a
//! request to a third party made on the reader's behalf, and it is exactly the
//! thing the geocoder goes through the server to avoid — so it is off unless
//! the operator turned it on, and these tests hold both halves of that.

mod common;

use axum::http::StatusCode;
use common::*;

fn sample() -> &'static std::path::Path {
    std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/deploy/sample.axgf"))
}

async fn a_place_id(app: &axum::Router) -> String {
    let body = body_string(get_admin(app, "/admin/place").await).await;
    let marker = "/admin/place/";
    let at = body
        .match_indices(marker)
        .map(|(i, _)| i + marker.len())
        .find(|i| {
            let rest: String = body[*i..].chars().take(48).collect();
            rest.contains("/edit")
        })
        .expect("a place row with an edit link");
    body[at..]
        .chars()
        .take_while(|c| c.is_ascii_alphanumeric() || *c == '-')
        .collect::<String>()
}

#[tokio::test]
async fn without_a_tile_source_the_editor_asks_for_no_map_at_all() {
    let dir = scratch("map-off");
    let path = dir.join("family.axgf");
    std::fs::copy(sample(), &path).expect("copy");
    let app = axgf_cms::app(&path, TOKEN).expect("app");
    let id = a_place_id(&app).await;

    let body = body_string(get_admin(&app, &format!("/admin/place/{id}/edit")).await).await;
    assert!(
        !body.contains("leaflet"),
        "no library, no stylesheet, no map div: {body}"
    );
    assert!(
        !body.contains("place-map"),
        "and no empty box where one would go"
    );
    // The manual path is untouched by the decision.
    assert!(body.contains(r#"name="coordinates.lat""#));
    assert!(
        body.contains(r#"name="paste""#),
        "and the paste box is there"
    );
    // The template escapes `/` as `&#x2f;`, which the HTML parser turns back
    // into a slash before the browser ever sees the URL. Assert on what
    // survives that rather than on the raw string.
    assert!(
        body.contains("openstreetmap.org") && body.contains("search?query="),
        "the out-link is the workflow when there is no basemap: {body}"
    );
}

#[tokio::test]
async fn with_a_tile_source_it_loads_exactly_the_two_assets_this_binary_serves() {
    let dir = scratch("map-on");
    let path = dir.join("family.axgf");
    std::fs::copy(sample(), &path).expect("copy");
    let tiles = axgf_cms::state::MapTiles::new(
        Some("https://tile.example.org/{z}/{x}/{y}.png"),
        Some("© Example"),
    )
    .expect("a url is enough");
    let app = axgf_cms::app_with_map(&path, TOKEN, tiles).expect("app");
    let id = a_place_id(&app).await;

    let body = body_string(get_admin(&app, &format!("/admin/place/{id}/edit")).await).await;
    assert!(
        body.contains(r#"<script src="/static/vendor/leaflet.js"></script>"#),
        "the library comes from this binary, not a CDN: {body}"
    );
    assert!(body.contains(r#"href="/static/vendor/leaflet.css""#));
    assert!(body.contains(r#"<script src="/static/map.js"></script>"#));
    assert!(
        body.contains("tile.example.org")
            && body.contains("{z}")
            && body.contains("{x}")
            && body.contains("{y}"),
        "the tile template reaches the page with its placeholders intact — \
         escaping the slashes is fine, eating the braces would not be: {body}"
    );
    assert!(
        body.contains("© Example") || body.contains("&#169; Example"),
        "and so does the attribution, which is a licence condition"
    );
    assert!(
        body.contains("id=\"place-map\" hidden"),
        "the box stays hidden until the script has something to draw in it"
    );

    // No third-party origin is named anywhere in the markup except the tile
    // template the operator chose themselves.
    for host in ["unpkg.com", "cdn.jsdelivr.net", "cdnjs.cloudflare.com"] {
        assert!(!body.contains(host), "{host} must not appear in the page");
    }
}

#[tokio::test]
async fn the_vendored_library_is_served_by_this_binary() {
    let dir = scratch("map-assets");
    let path = dir.join("family.axgf");
    std::fs::copy(sample(), &path).expect("copy");
    let app = axgf_cms::app(&path, TOKEN).expect("app");

    for (uri, needle) in [
        ("/static/vendor/leaflet.js", "Leaflet"),
        ("/static/vendor/leaflet.css", "leaflet-container"),
        ("/static/map.js", "place-map"),
    ] {
        let resp = get(&app, uri).await;
        assert_eq!(resp.status(), StatusCode::OK, "{uri}");
        let body = body_string(resp).await;
        assert!(body.contains(needle), "{uri} served the wrong thing");
    }
}

/// The vendored stylesheet points at no file this binary does not carry.
///
/// Upstream's CSS references three PNGs under `images/` — the default marker
/// and the layers-control sprites. Nothing here uses either, so they were
/// removed when the file was vendored; if an update quietly puts them back,
/// the page starts making requests for assets that 404 and nobody notices,
/// because a missing background image fails silently.
#[test]
fn the_vendored_stylesheet_fetches_nothing() {
    let css = axgf_cms::render::LEAFLET_CSS;
    assert!(
        !css.contains("url(images/"),
        "the vendored stylesheet points at an images/ directory this binary \
         does not serve — re-apply the modification described in its header"
    );
    assert!(
        css.contains("leaflet-container"),
        "…and it is still Leaflet's stylesheet"
    );
}
