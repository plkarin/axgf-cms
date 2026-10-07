//! What the binary does before it opens its listener.

/// The translation catalogue is parsed at boot, not by the first visitor.
///
/// All eleven catalogues are parsed together on first use, 16–24 ms, which a
/// first request to the river measured as 15.7 ms against 3.2 ms for every
/// request after it. `main` warms it before binding, so no visitor pays.
#[test]
fn the_catalogue_is_warmed_before_the_listener_opens() {
    let main = include_str!("../src/main.rs");
    let warm = main
        .find("axgf_cms::i18n::warm()")
        .expect("main warms the catalogue");
    let bind = main
        .find("TcpListener::bind")
        .expect("main binds a listener");
    assert!(
        warm < bind,
        "the catalogue must be warm before the listener opens"
    );
}

#[test]
fn warming_builds_every_language() {
    axgf_cms::i18n::warm();
    for l in axgf_cms::i18n::LOCALES {
        assert!(
            !axgf_cms::i18n::translate(l.tag, "nav-river", None).is_empty(),
            "{}",
            l.tag
        );
    }
}
