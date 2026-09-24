//! Field-by-field differences between two entities.
//!
//! # Why a hand-written walk rather than a JSON-patch crate
//!
//! What this produces is not for a machine to apply — it is for a person to
//! read in the seconds after being told their edit was refused. That makes the
//! requirements the opposite of a patch format's. A patch wants the smallest
//! correct set of operations; this wants the *most legible* account of what
//! differs, which means stable dotted paths a reader can find in the form
//! above, arrays compared by position rather than by a longest-common-
//! subsequence that would report a shift as an unrelated add and remove, and
//! `null` treated as absence so a field cleared by one editor and never set by
//! another do not read as a disagreement.

use std::fmt::Write as _;

use serde_json::Value;

/// One field that differs.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Change {
    /// Dotted path, e.g. `identity.name.display` or `children.0.person_id`.
    pub path: String,
    /// The value on the left, rendered for reading. `None` means absent.
    pub from: Option<String>,
    /// The value on the right. `None` means absent.
    pub to: Option<String>,
}

/// Fields whose difference is never worth showing a human.
///
/// `version_num` and `updated_at` change on *every* save by definition, so
/// listing them would put two lines of noise at the top of every diff and
/// bury the one line that matters.
const NOISE: [&str; 2] = ["version_num", "updated_at"];

/// Every field that differs between `a` and `b`, in path order.
pub fn diff(a: &Value, b: &Value) -> Vec<Change> {
    let mut out = Vec::new();
    walk("", a, b, &mut out);
    out.retain(|c| !NOISE.contains(&c.path.as_str()));
    out.sort_by(|x, y| path_order(&x.path, &y.path));
    out
}

/// Dotted paths in reading order: `names.2` before `names.10`.
///
/// Plain string order put the tenth entry of a list before the second, which
/// reads oddly in a table and is wrong for [`crate::journal::rewind`], which
/// undoes removals from the highest index down.
pub fn path_order(a: &str, b: &str) -> std::cmp::Ordering {
    let mut x = a.split('.');
    let mut y = b.split('.');
    loop {
        match (x.next(), y.next()) {
            (None, None) => return std::cmp::Ordering::Equal,
            (None, Some(_)) => return std::cmp::Ordering::Less,
            (Some(_), None) => return std::cmp::Ordering::Greater,
            (Some(p), Some(q)) => {
                let o = match (p.parse::<usize>(), q.parse::<usize>()) {
                    (Ok(i), Ok(j)) => i.cmp(&j),
                    _ => p.cmp(q),
                };
                if o != std::cmp::Ordering::Equal {
                    return o;
                }
            }
        }
    }
}

/// Journal entries written before blocks were diffed field by field.
///
/// Those recorded a block that appeared or disappeared whole, as one change
/// whose value was the block's JSON. The ones short enough to have been kept
/// intact are expanded here into the changes the current diff would have
/// recorded, so the history reads the same for old entries as for new ones and
/// a rewind across one puts back an object rather than a string of JSON.
///
/// The ones that were cut off at two hundred characters cannot be: the rest of
/// the value was never written down. They are left exactly as they are.
pub fn expand_legacy(changes: Vec<Change>) -> Vec<Change> {
    if !changes.iter().any(is_legacy_block) {
        return changes;
    }
    let mut out = Vec::new();
    for c in changes {
        if !is_legacy_block(&c) {
            out.push(c);
            continue;
        }
        let parse = |v: &Option<String>| {
            v.as_deref()
                .and_then(|s| serde_json::from_str::<Value>(s).ok())
                .unwrap_or(Value::Null)
        };
        let (from, to) = (parse(&c.from), parse(&c.to));
        let mut expanded = Vec::new();
        walk(&c.path, &from, &to, &mut expanded);
        out.extend(expanded);
    }
    out.sort_by(|x, y| path_order(&x.path, &y.path));
    out
}

/// A whole block recorded as one value: one side absent, the other a JSON
/// object or array that parses. A free-text field that happens to begin with a
/// brace is never one — those are prose, whatever they look like.
fn is_legacy_block(c: &Change) -> bool {
    const PROSE: [&str; 6] = [
        "bio",
        "notes",
        "note",
        "description",
        "caption",
        "transcription",
    ];
    if PROSE.contains(&c.path.rsplit('.').next().unwrap_or("")) {
        return false;
    }
    let whole = match (&c.from, &c.to) {
        (None, Some(v)) | (Some(v), None) => v,
        _ => return false,
    };
    matches!(
        serde_json::from_str::<Value>(whole),
        Ok(Value::Object(_) | Value::Array(_))
    )
}

/// Whether two entities differ in any way a reader would care about.
pub fn differs(a: &Value, b: &Value) -> bool {
    !diff(a, b).is_empty()
}

fn walk(prefix: &str, a: &Value, b: &Value, out: &mut Vec<Change>) {
    // `null` and absent are the same statement: the record does not say.
    // Treating them differently would report "cleared" against "never set".
    if a.is_null() && b.is_null() {
        return;
    }
    match (a, b) {
        (Value::Object(x), Value::Object(y)) => {
            let mut keys: Vec<&String> = x.keys().chain(y.keys()).collect();
            keys.sort();
            keys.dedup();
            for k in keys {
                let child = if prefix.is_empty() {
                    k.clone()
                } else {
                    format!("{prefix}.{k}")
                };
                walk(
                    &child,
                    x.get(k).unwrap_or(&Value::Null),
                    y.get(k).unwrap_or(&Value::Null),
                    out,
                );
            }
        }
        // Compared by position. A longest-common-subsequence would describe a
        // child inserted at the front as every later child having changed, and
        // "children.0 changed, children.1 changed, …" is a worse answer for a
        // reader than "children.0 was inserted".
        (Value::Array(x), Value::Array(y)) => {
            for i in 0..x.len().max(y.len()) {
                let child = format!("{prefix}.{i}");
                walk(
                    &child,
                    x.get(i).unwrap_or(&Value::Null),
                    y.get(i).unwrap_or(&Value::Null),
                    out,
                );
            }
        }
        // A block on one side only — a 1.1 profile group added to a person,
        // say — is walked against an empty one, so it is recorded field by
        // field like every other change. Rendered whole it reached the history
        // as a line of raw JSON, cut off at two hundred characters: unreadable,
        // and, cut off, not something the journal could ever replay.
        (Value::Null, Value::Object(_)) => walk(prefix, &Value::Object(Default::default()), b, out),
        (Value::Object(_), Value::Null) => walk(prefix, a, &Value::Object(Default::default()), out),
        (Value::Null, Value::Array(_)) => walk(prefix, &Value::Array(Vec::new()), b, out),
        (Value::Array(_), Value::Null) => walk(prefix, a, &Value::Array(Vec::new()), out),
        _ => {
            if a != b {
                out.push(Change {
                    path: prefix.to_string(),
                    from: render(a),
                    to: render(b),
                });
            }
        }
    }
}

/// A scalar as a reader should see it. `None` for absent.
fn render(v: &Value) -> Option<String> {
    match v {
        Value::Null => None,
        Value::String(s) => Some(s.clone()),
        Value::Bool(b) => Some(b.to_string()),
        Value::Number(n) => Some(n.to_string()),
        // A whole object or array landing here means one side had a scalar and
        // the other a structure — rare, and worth showing verbatim rather than
        // as "[object]".
        other => Some(compact(other)),
    }
}

fn compact(v: &Value) -> String {
    let mut s = String::new();
    let _ = write!(s, "{v}");
    if s.chars().count() > 200 {
        s = s.chars().take(197).collect::<String>() + "…";
    }
    s
}

/// A one-line summary, for the journal listing and the dashboard, in English.
pub fn summarise(changes: &[Change], kind: &str) -> String {
    summarise_in(changes, kind, crate::i18n::DEFAULT)
}

/// [`summarise`] in `lang`, as it reads after the editor's name in a history:
/// "Anna changed Height and Weight".
///
/// Every name in it is the reader's. This used to put the dotted path into the
/// sentence on the grounds that a path is an identifier, and so a Polish
/// history read "zmienił(a) extensions": a Polish sentence around an English
/// word that no Polish reader had any reason to know. The path is still in the
/// table under the summary for whoever needs the exact location; the sentence
/// names what the form calls it.
pub fn summarise_in(changes: &[Change], kind: &str, lang: &str) -> String {
    summarise_as(changes, kind, lang, "diff-summary")
}

/// The same summary standing on its own, after "Saved as version 3 —".
///
/// Two families rather than one because a sentence that follows a name is not
/// the same sentence as one that follows a dash in most of these languages: a
/// Polish or Russian past tense agrees with the person who acted, and on the
/// saved banner there is no person in the sentence to agree with.
pub fn saved_in(changes: &[Change], kind: &str, lang: &str) -> String {
    summarise_as(changes, kind, lang, "diff-saved")
}

fn summarise_as(changes: &[Change], kind: &str, lang: &str, family: &str) -> String {
    use fluent::{FluentArgs, FluentValue};
    // One name per thing changed, not per leaf: twelve measurements in one
    // profile group are "Height, Weight and 10 more", and three leaves of one
    // date are the date once.
    let mut names: Vec<String> = Vec::new();
    for c in changes {
        let name = label(kind, &c.path, lang);
        if !names.contains(&name) {
            names.push(name);
        }
    }
    let name = |i: usize| FluentValue::from(names[i].clone());
    let (suffix, args) = match names.len() {
        0 => ("none", FluentArgs::new()),
        1 => ("one", FluentArgs::from_iter([("a", name(0))])),
        2 => (
            "two",
            FluentArgs::from_iter([("a", name(0)), ("b", name(1))]),
        ),
        n => (
            "many",
            FluentArgs::from_iter([
                ("a", name(0)),
                ("b", name(1)),
                ("n", FluentValue::from((n - 2) as i64)),
            ]),
        ),
    };
    crate::i18n::translate(lang, &format!("{family}-{suffix}"), Some(&args))
}

/// What a reader calls the field at `path` of a `kind`, in `lang`.
///
/// The most specific name the catalogues have, in this order:
///
/// 1. a 1.1 profile attribute (`morphology.height.0.value` → "Height");
/// 2. a field of the generic editor (`birth.date.value` → "Birth date");
/// 3. the profile group a block belongs to (`morphology.x` → "Morphology");
/// 4. the section it sits in (`union.persons.0.person_id` → "Partners");
/// 5. "another field", rather than the path. A path is a fact about the file
///    format; the summary is a sentence for the family.
pub fn label(kind: &str, path: &str, lang: &str) -> String {
    let t = |key: &str| crate::i18n::translate(lang, key, None);
    let within = |prefix: &str| path == prefix || path.starts_with(&format!("{prefix}."));
    let top = path.split('.').next().unwrap_or(path);

    if kind == "person" {
        use axgf_rs::model::profile::registry::GROUPS;
        for g in GROUPS {
            for a in g.attributes {
                if within(a.path) {
                    return t(&crate::profile::attribute_key(a));
                }
            }
        }
    }
    if let Some(k) = crate::admin::kind_from_str(kind) {
        // Longest match first: `birth.date.value` over a section `birth`.
        let best = crate::admin::fields_for(k)
            .iter()
            .filter(|f| within(f.path))
            .max_by_key(|f| f.path.len());
        if let Some(f) = best {
            return t(f.label);
        }
    }
    if kind == "person" {
        use axgf_rs::model::profile::registry::GROUPS;
        if let Some(g) = GROUPS
            .iter()
            .find(|g| g.key == top || g.attributes.iter().any(|a| a.block == top))
        {
            return t(&crate::profile::group_key(g));
        }
    }
    for (prefix, key) in SECTIONS {
        if within(prefix) {
            return t(key);
        }
    }
    t("diff-section-other")
}

/// The history table's rows for a reader: values withheld where the reader's
/// scopes stop, and every enumerated value named as its form names it.
///
/// `year`, `F` or `ended_by_death` in a *what changed* table are the file's
/// codes, not words; the `<select>` that set them showed the reader a label,
/// and the record of the change shows the same one.
pub fn for_reader(
    changes: &[Change],
    readable: crate::sensitive::Scopes,
    kind: &str,
    lang: &str,
) -> Vec<Value> {
    let select = |path: &str| {
        crate::admin::kind_from_str(kind).and_then(|k| {
            crate::admin::fields_for(k)
                .iter()
                .find(|f| f.path == path && f.kind == crate::admin::FieldKind::Select)
        })
    };
    let mut rows = crate::sensitive::changes_for_reader(changes, readable);
    for (row, c) in rows.iter_mut().zip(changes) {
        if let Some(f) = select(&c.path) {
            for side in ["from", "to"] {
                if let Some(v) = row.get(side).and_then(Value::as_str).map(str::to_string) {
                    // A value outside the vocabulary — imported data can hold
                    // anything — stays as it was recorded, not as a message id.
                    let key = f.option_key(&v);
                    let label = crate::i18n::translate(lang, &key, None);
                    if label != key {
                        row[side] = Value::String(label);
                    }
                }
            }
        }
    }
    rows
}

/// Sections no generic field or profile group names, most specific first.
///
/// `tests/i18n.rs` holds every key here to every catalogue, so a section added
/// without its translations fails there rather than printing a key.
pub const SECTIONS: &[(&str, &str)] = &[
    ("extensions.axgf-cms:avatar/v1", "diff-section-avatar"),
    ("union.persons", "diff-section-partners"),
    ("ai", "diff-section-ai"),
    ("birth", "diff-section-birth"),
    ("civil_status", "diff-section-civil-status"),
    ("documents", "diff-section-documents"),
    ("extensions", "diff-section-extensions"),
    ("children", "diff-section-children"),
    ("union", "diff-section-union"),
    ("date", "diff-section-date"),
    ("participants", "diff-section-participants"),
    ("from", "diff-section-from"),
    ("to", "diff-section-to"),
    ("valid_from", "diff-section-valid-from"),
    ("valid_until", "diff-section-valid-until"),
    ("visibility", "diff-section-visibility"),
    ("employer", "diff-section-employer"),
    ("title_normalized", "diff-section-title-normalized"),
    ("conflicts", "diff-section-conflicts"),
    ("dna", "diff-section-dna"),
    ("document_id", "diff-section-document"),
    ("language", "diff-section-language"),
    ("place_id", "diff-section-place"),
    ("repository", "diff-section-repository"),
    ("script", "diff-section-script"),
    ("coordinates", "diff-section-coordinates"),
    ("country_history", "diff-section-country-history"),
    ("identifiers", "diff-section-identifiers"),
    ("names", "diff-section-names"),
    ("file", "diff-section-file"),
    ("linked_to", "diff-section-linked-to"),
    ("ocr", "diff-section-ocr"),
    ("axgf_version", "diff-section-format-version"),
];

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn a_changed_scalar_is_reported_with_both_values() {
        let a = json!({"identity": {"name": {"display": "Laura"}}});
        let b = json!({"identity": {"name": {"display": "Laura Karin"}}});
        assert_eq!(
            diff(&a, &b),
            vec![Change {
                path: "identity.name.display".into(),
                from: Some("Laura".into()),
                to: Some("Laura Karin".into()),
            }]
        );
    }

    #[test]
    fn an_added_and_a_removed_field_read_as_absence_on_one_side() {
        let a = json!({"note": "kept", "gone": "was here"});
        let b = json!({"note": "kept", "added": "now here"});
        let d = diff(&a, &b);
        assert_eq!(d.len(), 2);
        assert_eq!(d[0].path, "added");
        assert_eq!(d[0].from, None);
        assert_eq!(d[1].path, "gone");
        assert_eq!(d[1].to, None);
    }

    #[test]
    fn null_and_absent_are_the_same_statement() {
        // A field one editor cleared and another never set must not read as a
        // disagreement — the record says nothing in both cases.
        let a = json!({"death": null, "birth": {"date": null}});
        let b = json!({"birth": {}});
        assert_eq!(diff(&a, &b), vec![], "neither side says anything");
    }

    #[test]
    fn version_and_timestamp_churn_is_not_shown() {
        // They change on every save by definition. Listing them would put two
        // lines of noise above the one line that matters.
        let a = json!({"version_num": 1, "updated_at": "a", "note": "x"});
        let b = json!({"version_num": 2, "updated_at": "b", "note": "y"});
        assert_eq!(
            diff(&a, &b),
            vec![Change {
                path: "note".into(),
                from: Some("x".into()),
                to: Some("y".into()),
            }]
        );
    }

    #[test]
    fn arrays_are_compared_by_position() {
        let a = json!({"children": [{"person_id": "a"}, {"person_id": "b"}]});
        let b = json!({"children": [{"person_id": "a"}, {"person_id": "c"}]});
        let d = diff(&a, &b);
        assert_eq!(d.len(), 1);
        assert_eq!(d[0].path, "children.1.person_id");
    }

    #[test]
    fn a_shortened_array_reports_the_dropped_entry_as_absent() {
        let a = json!({"names": ["one", "two"]});
        let b = json!({"names": ["one"]});
        let d = diff(&a, &b);
        assert_eq!(d.len(), 1);
        assert_eq!(d[0].path, "names.1");
        assert_eq!(d[0].to, None);
    }

    #[test]
    fn an_identical_entity_has_no_diff() {
        let a = json!({"identity": {"name": {"display": "Laura"}}, "n": 1});
        assert_eq!(diff(&a, &a.clone()), vec![]);
        assert!(!differs(&a, &a.clone()));
    }

    #[test]
    fn a_summary_names_the_fields_and_then_counts_them() {
        let c = |p: &str| Change {
            path: p.into(),
            from: None,
            to: None,
        };
        assert_eq!(summarise(&[], "person"), "no field changed");
        assert_eq!(summarise(&[c("notes")], "person"), "changed Notes");
        assert_eq!(
            summarise(&[c("notes"), c("bio")], "person"),
            "changed Notes and Biography"
        );
        assert_eq!(
            summarise(
                &[
                    c("notes"),
                    c("bio"),
                    c("birth.date.value"),
                    c("death.cause")
                ],
                "person"
            ),
            "changed Notes, Biography and 2 more"
        );
    }

    #[test]
    fn a_long_value_is_truncated_rather_than_flooding_the_page() {
        let a = json!({"x": 1});
        let b = json!({"x": {"deep": "y".repeat(500)}});
        let d = diff(&a, &b);
        assert_eq!(d.len(), 1);
        assert!(d[0].to.as_ref().unwrap().chars().count() <= 200);
    }

    /// The shape the operator's history carried: a 1.1 profile group added to
    /// a person, recorded as one line of JSON.
    fn with_morphology() -> (Value, Value) {
        let before = json!({"identity": {"name": {"display": "Laura"}}});
        let mut after = before.clone();
        after["morphology"] = json!({
            "height": [{"value": 193}],
            "weight": [{"value": 105}]
        });
        (before, after)
    }

    #[test]
    fn a_profile_group_added_whole_is_recorded_field_by_field() {
        let (before, after) = with_morphology();
        let d = diff(&before, &after);
        let paths: Vec<&str> = d.iter().map(|c| c.path.as_str()).collect();
        assert_eq!(
            paths,
            ["morphology.height.0.value", "morphology.weight.0.value"]
        );
        assert!(
            d.iter()
                .all(|c| !c.to.as_deref().unwrap_or("").starts_with('{')),
            "no change carries a block as JSON: {d:?}"
        );
        // And removed whole, the same the other way round.
        let back = diff(&after, &before);
        assert_eq!(back.len(), 2);
        assert!(back.iter().all(|c| c.to.is_none() && c.from.is_some()));
    }

    #[test]
    fn a_summary_names_the_field_in_the_readers_language() {
        // "zmienił(a) extensions" was a Polish sentence around an English
        // identifier. Every name in the sentence is the reader's now.
        let (before, after) = with_morphology();
        let d = diff(&before, &after);
        let en = summarise_in(&d, "person", "en");
        assert_eq!(en, "changed Height and Weight");
        let pl = summarise_in(&d, "person", "pl");
        assert!(!pl.contains("morphology") && !pl.contains("height"), "{pl}");
        assert!(pl.starts_with("zmienił(a) "), "{pl}");

        let avatar = [Change {
            path: "extensions.axgf-cms:avatar/v1.document_id".into(),
            from: None,
            to: Some("f7f4d05f-feee-49b2-b726-63e918b25e3f".into()),
        }];
        assert_eq!(
            summarise_in(&avatar, "person", "en"),
            "changed Portrait photo"
        );
        assert!(!summarise_in(&avatar, "person", "pl").contains("extensions"));
    }

    #[test]
    fn many_leaves_of_one_thing_are_named_once() {
        let c = |p: &str| Change {
            path: p.into(),
            from: None,
            to: Some("x".into()),
        };
        let d = [
            c("union.persons.0.person_id"),
            c("union.persons.0.role"),
            c("union.persons.1.person_id"),
        ];
        assert_eq!(summarise_in(&d, "family", "en"), "changed Partners");
    }

    #[test]
    fn a_path_nothing_names_reads_as_another_field_not_as_the_path() {
        let d = [Change {
            path: "some_future_block.x".into(),
            from: None,
            to: Some("1".into()),
        }];
        assert_eq!(summarise_in(&d, "person", "en"), "changed another field");
    }

    #[test]
    fn a_legacy_block_that_survived_intact_is_expanded_on_read() {
        // Verbatim from the operator's journal, version 3 of a person.
        let legacy = vec![Change {
            path: "extensions".into(),
            from: None,
            to: Some(
                r#"{"axgf-cms:avatar/v1":{"document_id":"f7f4d05f-feee-49b2-b726-63e918b25e3f"}}"#
                    .into(),
            ),
        }];
        let e = expand_legacy(legacy);
        assert_eq!(e.len(), 1);
        assert_eq!(e[0].path, "extensions.axgf-cms:avatar/v1.document_id");
        assert_eq!(
            e[0].to.as_deref(),
            Some("f7f4d05f-feee-49b2-b726-63e918b25e3f")
        );
    }

    #[test]
    fn a_legacy_block_that_was_cut_off_is_left_as_it_was() {
        // Also verbatim in shape: 197 characters and an ellipsis. The rest was
        // never written down, so there is nothing to expand it into.
        let cut = format!(
            "{}…",
            &r#"{"height":[{"value":193}],"weight":[{"value":105}],"bmi":[{"value":22}],"build":[{"value":"#
                [..90]
        );
        let legacy = vec![Change {
            path: "morphology".into(),
            from: None,
            to: Some(cut.clone()),
        }];
        assert_eq!(expand_legacy(legacy.clone()), legacy);
    }

    #[test]
    fn prose_that_begins_with_a_brace_is_not_mistaken_for_a_block() {
        let note = vec![Change {
            path: "notes".into(),
            from: None,
            to: Some(r#"{"quoted": "from a letter"}"#.into()),
        }];
        assert_eq!(expand_legacy(note.clone()), note);
    }

    #[test]
    fn paths_sort_in_reading_order() {
        let mut v = vec!["names.10", "names.2", "names.1.x", "birth"];
        v.sort_by(|a, b| path_order(a, b));
        assert_eq!(v, ["birth", "names.1.x", "names.2", "names.10"]);
    }
}
