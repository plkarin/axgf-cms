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
    out.sort_by(|x, y| x.path.cmp(&y.path));
    out
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

/// A one-line summary, for the journal listing and the dashboard.
pub fn summarise(changes: &[Change]) -> String {
    match changes.len() {
        0 => "no field changed".to_string(),
        1 => format!("changed {}", changes[0].path),
        2 => format!("changed {} and {}", changes[0].path, changes[1].path),
        n => format!(
            "changed {}, {} and {} more",
            changes[0].path,
            changes[1].path,
            n - 2
        ),
    }
}

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
        assert_eq!(summarise(&[]), "no field changed");
        assert_eq!(summarise(&[c("a")]), "changed a");
        assert_eq!(summarise(&[c("a"), c("b")]), "changed a and b");
        assert_eq!(
            summarise(&[c("a"), c("b"), c("c"), c("d")]),
            "changed a, b and 2 more"
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
}
