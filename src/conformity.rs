//! Is the bundle what it should be? (v0.5.0, docs/adr/0011 §4)
//!
//! The AXGF bundle page answers one question in nine parts: the format, the
//! manifest, the validator, the attached files, the accounts' binding, the
//! living who are public, the special categories, the data quality and the
//! size. Each part is an [`Item`]: green, a warning or critical, what was
//! found and what to do. Nothing here changes the bundle.

use serde_json::Value;

use crate::findings::{Item, Level};

/// The AXGF version this build reads and writes.
pub const SPEC: &str = "1.1";

/// The order the page lists the areas in.
pub const AREAS: [&str; 4] = ["format", "integrity", "privacy", "content"];

/// What is known about the bundle file itself, for the page's particulars.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct Facts {
    pub spec: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub generator: Option<String>,
    pub gedcom_source: Option<String>,
    /// The archive carries the JSON schema it was written against.
    pub schema_embedded: Option<String>,
    pub members: usize,
}

/// Read the particulars from the manifest in memory and the archive's
/// directory on disk (names only: nothing is decompressed).
pub fn facts(flat: &Value, bundle: &std::path::Path) -> Facts {
    let m = flat.get("manifest");
    let s = |path: &[&str]| -> Option<String> {
        let mut v = m?;
        for p in path {
            v = v.get(*p)?;
        }
        v.as_str().map(str::to_string)
    };
    let generator = match (s(&["generator", "name"]), s(&["generator", "version"])) {
        (Some(n), Some(v)) => Some(format!("{n} {v}")),
        (Some(n), None) => Some(n),
        _ => None,
    };
    let (schema_embedded, members) = std::fs::File::open(bundle)
        .ok()
        .and_then(|f| zip::ZipArchive::new(std::io::BufReader::new(f)).ok())
        .map(|z| {
            let schema = z
                .file_names()
                .find(|n| n.starts_with("schema/") && n.ends_with(".json"))
                .map(str::to_string);
            (schema, z.len())
        })
        .unwrap_or((None, 0));
    Facts {
        spec: s(&["axgf"]),
        created_at: s(&["created_at"]),
        updated_at: s(&["updated_at"]),
        generator,
        gedcom_source: s(&["compatibility", "gedcom_source"]),
        schema_embedded,
        members,
    }
}

/// What the page needs measured beyond the bundle itself.
pub struct Context<'a> {
    pub validation: &'a axgf_rs::boundary::envelope::Envelope,
    pub missing_payloads: usize,
    pub declared_payloads: usize,
    pub binding: crate::acl::Binding,
    pub size: u64,
    pub size_warn: u64,
    pub lang: &'a str,
}

/// Every conformity item, for a bundle that has a family in it.
pub fn check(flat: &Value, cx: &Context) -> Vec<Item> {
    use axgf_rs::boundary::envelope::Severity;
    let mut out = Vec::new();

    // The format version the manifest declares.
    let spec = flat
        .get("manifest")
        .and_then(|m| m.get("axgf"))
        .and_then(Value::as_str)
        .unwrap_or("");
    let item = Item::new("format", "format", "bundle-check-format-title").text("version", spec);
    out.push(if spec == SPEC {
        item.ok("bundle-check-format-ok")
    } else if spec.is_empty() {
        item.fail(
            "bundle-check-format-missing",
            "bundle-check-format-missing-fix",
        )
    } else if version_lt(spec, SPEC) {
        item.text("current", SPEC)
            .warn("bundle-check-format-older", "bundle-check-format-older-fix")
    } else {
        item.text("current", SPEC)
            .fail("bundle-check-format-newer", "bundle-check-format-newer-fix")
            .link("/admin/infrastructure#version")
    });

    // The manifest's counts against what the bundle holds.
    let declared = flat
        .get("manifest")
        .and_then(|m| m.get("stats"))
        .and_then(Value::as_object);
    let drift = crate::state::COLLECTIONS.iter().find_map(|c| {
        let actual = flat
            .get(*c)
            .and_then(Value::as_object)
            .map(|m| m.len())
            .unwrap_or(0) as i64;
        let said = declared?.get(*c)?.as_i64()?;
        (said != actual).then_some((*c, said, actual))
    });
    let item = Item::new("manifest", "format", "bundle-check-manifest-title");
    out.push(match drift {
        None => item.ok("bundle-check-manifest-ok"),
        Some((c, said, actual)) => item
            .text(
                "collection",
                crate::i18n::translate(
                    cx.lang,
                    &format!("kind-{}-plural", singular(c)),
                    Some(&crate::i18n::args(&[(
                        "n",
                        fluent::FluentValue::from(said),
                    )])),
                ),
            )
            .num("declared", said)
            .num("actual", actual)
            .warn("bundle-check-manifest-drift", "bundle-check-manifest-fix"),
    });

    // The library's own validator.
    let errors = cx
        .validation
        .diagnostics
        .iter()
        .filter(|d| d.severity == Severity::Error)
        .count();
    let warnings = cx
        .validation
        .diagnostics
        .iter()
        .filter(|d| d.severity == Severity::Warning)
        .count();
    let item = Item::new("validation", "integrity", "bundle-check-validation-title")
        .num("errors", errors)
        .num("warnings", warnings)
        .link("#diagnostics");
    out.push(if errors > 0 {
        item.fail(
            "bundle-check-validation-errors",
            "bundle-check-validation-errors-fix",
        )
    } else if warnings > 0 {
        item.warn(
            "bundle-check-validation-warnings",
            "bundle-check-validation-warnings-fix",
        )
    } else {
        item.ok("bundle-check-validation-ok")
    });

    // The attached files.
    let item = Item::new("payloads", "integrity", "bundle-check-payloads-title")
        .num("declared", cx.declared_payloads)
        .num("missing", cx.missing_payloads);
    out.push(if cx.declared_payloads == 0 {
        item.ok("bundle-check-payloads-none")
    } else if cx.missing_payloads == 0 {
        item.ok("bundle-check-payloads-ok")
    } else {
        item.warn("bundle-check-payloads-missing", "bundle-check-payloads-fix")
    });

    // The accounts' binding.
    let item = Item::new("binding", "integrity", "bundle-check-binding-title");
    out.push(match &cx.binding {
        crate::acl::Binding::Ok => item.ok("bundle-check-binding-ok"),
        crate::acl::Binding::Mismatch { want, got, .. } => item
            .text("expected", described(cx.lang, want))
            .text("found", described(cx.lang, got))
            .warn("bundle-check-binding-mismatch", "bundle-check-binding-fix"),
    });

    // The living, readable by any visitor.
    let living_public = flat
        .get("persons")
        .and_then(Value::as_object)
        .map(|ps| {
            ps.values()
                .filter(|p| {
                    let id = p.get("identity");
                    id.and_then(|i| i.get("is_living")).and_then(Value::as_bool) == Some(true)
                        && crate::access::person_visibility(p) == crate::acl::Visibility::Public
                })
                .count()
        })
        .unwrap_or(0);
    let item =
        Item::new("living-public", "privacy", "bundle-check-living-title").num("n", living_public);
    out.push(if living_public == 0 {
        item.ok("bundle-check-living-ok")
    } else {
        item.warn("bundle-check-living-public", "bundle-check-living-fix")
            .link("/admin/person")
    });

    // The special categories.
    let special = flat
        .get("persons")
        .and_then(Value::as_object)
        .map(|ps| {
            ps.values()
                .filter(|p| !crate::sensitive::present(p).is_empty())
                .count()
        })
        .unwrap_or(0);
    let item = Item::new("special", "privacy", "bundle-check-special-title").num("n", special);
    out.push(if special == 0 {
        item.ok("bundle-check-special-none")
    } else {
        item.ok("bundle-check-special-some")
    });

    // The data quality page's findings that nobody has marked checked.
    let findings = crate::quality::detect(flat);
    let open: Vec<_> = findings.iter().filter(|f| f.dismissed.is_none()).collect();
    let groups: std::collections::BTreeSet<&str> = open.iter().map(|f| f.rule.group()).collect();
    let item = Item::new("quality", "content", "bundle-check-quality-title")
        .num("n", open.len())
        .num("groups", groups.len())
        .num("checked", findings.len() - open.len())
        .link("/quality");
    out.push(if open.is_empty() {
        item.ok("bundle-check-quality-ok")
    } else {
        item.warn("bundle-check-quality-open", "bundle-check-quality-fix")
    });

    // The size.
    let item = Item::new("size", "content", "bundle-check-size-title")
        .text("size", crate::documents::human_size_in(cx.size, cx.lang))
        .text(
            "warn",
            crate::documents::human_size_in(cx.size_warn, cx.lang),
        )
        .link("/admin/infrastructure#volumes");
    out.push(if cx.size > cx.size_warn {
        item.warn("bundle-check-size-heavy", "bundle-check-size-fix")
    } else {
        item.ok("bundle-check-size-ok")
    });

    out
}

/// The findings of the data quality page, counted by rule, for the
/// anomalies table: open and checked.
pub fn anomalies(flat: &Value) -> Vec<(crate::quality::Rule, usize, usize)> {
    let findings = crate::quality::detect(flat);
    crate::quality::Rule::ALL
        .iter()
        .filter_map(|r| {
            let mine: Vec<_> = findings.iter().filter(|f| f.rule == *r).collect();
            if mine.is_empty() {
                return None;
            }
            let open = mine.iter().filter(|f| f.dismissed.is_none()).count();
            Some((*r, open, mine.len() - open))
        })
        .collect()
}

/// `persons` → `person`, the singular the catalogue's kind keys use.
fn singular(collection: &str) -> &'static str {
    match collection {
        "persons" => "person",
        "families" => "family",
        "events" => "event",
        "links" => "link",
        "occupations" => "occupation",
        "sources" => "source",
        "places" => "place",
        _ => "document",
    }
}

/// `1.0` < `1.1`, numerically, part by part.
pub fn version_lt(a: &str, b: &str) -> bool {
    let parse = |v: &str| -> Vec<u64> {
        v.trim_start_matches('v')
            .split(['.', '-'])
            .map(|p| p.parse().unwrap_or(0))
            .collect()
    };
    let (a, b) = (parse(a), parse(b));
    for i in 0..a.len().max(b.len()) {
        let (x, y) = (
            a.get(i).copied().unwrap_or(0),
            b.get(i).copied().unwrap_or(0),
        );
        if x != y {
            return x < y;
        }
    }
    false
}

/// The worst level of a bundle's items, as the page's headline says it.
pub fn verdict(items: &[Item]) -> Level {
    crate::findings::worst(items)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn versions_compare_part_by_part() {
        assert!(version_lt("1.0", "1.1"));
        assert!(version_lt("1.9", "1.10"));
        assert!(!version_lt("1.1", "1.1"));
        assert!(!version_lt("2.0", "1.1"));
        assert!(version_lt("v0.4.3", "v0.5.0"));
    }
}

/// A bundle as a finding names it, in the reader's language: its family's
/// name and the day it was created, whichever it carries.
fn described(lang: &str, (family, created): &(String, String)) -> String {
    let date = crate::view::local_datetime(created, lang);
    match (family.is_empty(), created.is_empty()) {
        (true, true) => crate::view::t(lang, "bundle-binding-unnamed", &[]),
        (true, false) => crate::view::t(lang, "bundle-binding-created", &[("date", date.into())]),
        (false, true) => crate::view::t(
            lang,
            "bundle-binding-named",
            &[("family", family.clone().into())],
        ),
        (false, false) => crate::view::t(
            lang,
            "bundle-binding-named-created",
            &[("family", family.clone().into()), ("date", date.into())],
        ),
    }
}

#[cfg(test)]
mod described_tests {
    use super::described;

    #[test]
    fn a_bundle_is_named_in_the_readers_language() {
        let both = (
            "Famille Exemple".to_string(),
            "2026-08-09T13:15:54Z".to_string(),
        );
        let fr = described("fr", &both);
        // French guillemets hold the name with no-break spaces.
        assert!(
            fr.contains("«\u{a0}Famille Exemple\u{a0}»") && fr.contains("créée le"),
            "{fr}"
        );
        assert!(!fr.contains("created") && !fr.contains("T13:15"), "{fr}");
        let en = described("en", &(String::new(), String::new()));
        assert_eq!(en, "an unnamed archive");
        let de = described("de", &(String::new(), "2026-08-24T12:17:36Z".to_string()));
        assert!(
            de.starts_with("ein Archiv, angelegt am") && !de.contains("T12"),
            "{de}"
        );
    }
}
