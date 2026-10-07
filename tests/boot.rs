//! What the binary does before it opens its listener.

/// The translation catalogue is parsed, and the templates compiled, at boot —
/// not by the first visitor.
///
/// All eleven catalogues are parsed together on first use, 16–24 ms, which a
/// first request to the river measured as 15.7 ms against 3.2 ms for every
/// request after it; the templates were most of what remained. `main` warms
/// both before binding, so no visitor pays.
#[test]
fn the_catalogue_and_templates_are_warmed_before_the_listener_opens() {
    let main = include_str!("../src/main.rs");
    let bind = main
        .find("TcpListener::bind")
        .expect("main binds a listener");
    for warm in ["axgf_cms::i18n::warm()", "axgf_cms::render::warm()"] {
        let at = main
            .find(warm)
            .unwrap_or_else(|| panic!("main does not call {warm}"));
        assert!(at < bind, "{warm} must run before the listener opens");
    }
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

#[test]
fn warming_compiles_every_template() {
    axgf_cms::render::warm();
    assert!(axgf_cms::render::env().get_template("river.html").is_ok());
}
