//! What the first translation in a process, and in each language, costs.

use std::time::Instant;

#[test]
#[ignore = "timing report, run with --nocapture"]
fn first_translation_cost_per_locale() {
    let args = fluent::FluentArgs::from_iter([("n", fluent::FluentValue::from(3))]);
    let t = Instant::now();
    let _ = axgf_cms::i18n::translate("en", "nav-river", None);
    println!(
        "first translate in the process: {:.2} ms",
        t.elapsed().as_secs_f64() * 1000.0
    );
    for l in axgf_cms::i18n::LOCALES {
        let t = Instant::now();
        let _ = axgf_cms::i18n::translate(l.tag, "tree-hidden-notice", Some(&args));
        let first = t.elapsed().as_secs_f64() * 1000.0;
        let t = Instant::now();
        let _ = axgf_cms::i18n::translate(l.tag, "tree-hidden-notice", Some(&args));
        println!(
            "{:8} first plural {first:.3} ms · second {:.3} ms",
            l.tag,
            t.elapsed().as_secs_f64() * 1000.0
        );
    }
}
