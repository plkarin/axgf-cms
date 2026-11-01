//! How a family's union is read: a couple, or a polygamous union read as one
//! couple per spouse (AXGF 1.0 §4.2.1–§4.2.2, axgf-spec proposal 0003,
//! docs/adr/0021).
//!
//! A polygamous union (`union.type: "polygamous"`) joins one person, its
//! primary person, to each of the others. The others are not married to one
//! another: they are co-spouses. Each spouse's own marriage — its start, its
//! end, its status — is the `unions[]` entry that names them, and each child's
//! parents are the ones `children[].parent_ids` names. A child whose other
//! parent the record does not name stays with the primary person alone, in an
//! *open* couple: nobody says whether it is a full or a half sibling of the
//! others.
//!
//! Every view reads a family through [`couples`], so the record, the tree, the
//! river and kinship see the same marriages and the same parents. A union
//! that is not polygamous is one couple, exactly as it was always read.

use serde_json::{Map, Value};

/// `union.type` of a polygamous union.
pub const POLYGAMOUS: &str = "polygamous";

/// Whether a family's union is polygamous.
pub fn is_polygamous(family: &Value) -> bool {
    family.pointer("/union/type").and_then(Value::as_str) == Some(POLYGAMOUS)
}

/// Everyone in a family's union: `union.persons`, then any spouse only
/// `union.unions[]` names, each once, in the record's order.
pub fn members(family: &Value) -> Vec<&str> {
    let union = family.get("union");
    let listed = union
        .and_then(|u| u.get("persons"))
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|p| p.get("person_id").and_then(Value::as_str));
    let spouses = union
        .and_then(|u| u.get("unions"))
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|s| s.get("spouse_id").and_then(Value::as_str));
    let mut out: Vec<&str> = Vec::new();
    for id in listed.chain(spouses) {
        if !id.is_empty() && !out.contains(&id) {
            out.push(id);
        }
    }
    out
}

/// The role `id` is recorded with in `union.persons`.
pub fn role_of<'a>(family: &'a Value, id: &str) -> Option<&'a str> {
    family
        .pointer("/union/persons")
        .and_then(Value::as_array)?
        .iter()
        .find(|p| p.get("person_id").and_then(Value::as_str) == Some(id))
        .and_then(|p| p.get("role"))
        .and_then(Value::as_str)
}

/// The one member whose value differs from every other member's, all the
/// others sharing one: the husband among wives, by role or by sex.
fn odd_one_out<'a, T: PartialEq>(
    members: &[&'a str],
    value: impl Fn(&str) -> Option<T>,
) -> Option<&'a str> {
    let values: Vec<Option<T>> = members.iter().map(|m| value(m)).collect();
    if values.iter().any(Option::is_none) {
        return None;
    }
    let differs = |i: usize| {
        values
            .iter()
            .enumerate()
            .filter(|(j, _)| *j != i)
            .all(|(_, v)| *v != values[i])
    };
    let rest_agree = |i: usize| {
        let mut others = values.iter().enumerate().filter(|(j, _)| *j != i);
        let first = others.next().map(|(_, v)| v);
        others.all(|(_, v)| Some(v) == first)
    };
    let found: Vec<usize> = (0..members.len())
        .filter(|&i| differs(i) && rest_agree(i))
        .collect();
    match found.as_slice() {
        [i] => Some(members[*i]),
        _ => None,
    }
}

/// The person a polygamous union joins to each of the others.
///
/// The one `primary_person_id` names. Else the one the record makes plain:
/// the only member `unions[]` does not name as a spouse, or the only one
/// whose recorded role, or sex, differs from all the others'. Else the first
/// listed, the place every converter and the specification's own example
/// put them. `None` for a union that is not polygamous, and for a
/// polygamous couple of two that names no primary person and no `unions[]`,
/// which is one marriage of a polygamous household recorded on its own.
pub fn primary<'a>(family: &'a Value, persons: Option<&Map<String, Value>>) -> Option<&'a str> {
    if !is_polygamous(family) {
        return None;
    }
    let members = members(family);
    let union = family.get("union")?;
    if let Some(p) = union
        .get("primary_person_id")
        .and_then(Value::as_str)
        .filter(|p| members.contains(p))
    {
        return Some(p);
    }
    let spouses: Vec<&str> = union
        .get("unions")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|s| s.get("spouse_id").and_then(Value::as_str))
        .collect();
    if spouses.is_empty() && members.len() < 3 {
        return None;
    }
    if !spouses.is_empty() {
        let unnamed: Vec<&str> = members
            .iter()
            .copied()
            .filter(|m| !spouses.contains(m))
            .collect();
        if let [p] = unnamed.as_slice() {
            return Some(p);
        }
    }
    odd_one_out(&members, |m| {
        role_of(family, m).map(|r| r.trim().to_lowercase())
    })
    .or_else(|| {
        odd_one_out(&members, |m| {
            match crate::kinship::Sex::of_person(persons.and_then(|ps| ps.get(m))) {
                crate::kinship::Sex::U => None,
                s => Some(s),
            }
        })
    })
    .or_else(|| members.first().copied())
}

/// One couple of a family, as every view reads it.
#[derive(Debug, Clone)]
pub struct Couple<'a> {
    /// The partners: two, or one — a lone parent, or a polygamous union's
    /// primary person with the children whose other parent is not recorded.
    /// A polygamous union's primary person comes first.
    pub partners: Vec<&'a str>,
    /// The children's entries, in the family's order.
    pub children: Vec<&'a Value>,
    /// Part of a polygamous union.
    pub polygamous: bool,
    /// A polygamous union's spouse: that spouse's own marriage, the
    /// `unions[]` entry that names them.
    pub own: Option<&'a Value>,
    /// The children's other parent is not recorded.
    pub open: bool,
}

impl<'a> Couple<'a> {
    /// A field of this couple's marriage: the spouse's own (`unions[]`) where
    /// it says, else the union's.
    pub fn field(&self, family: &'a Value, key: &str) -> Option<&'a Value> {
        self.own
            .and_then(|o| o.get(key))
            .filter(|v| !v.is_null())
            .or_else(|| family.get("union").and_then(|u| u.get(key)))
            .filter(|v| !v.is_null())
    }

    /// The marriage's status term.
    pub fn status(&self, family: &'a Value) -> Option<&'a str> {
        self.field(family, "status").and_then(Value::as_str)
    }
}

/// The persons of the union a child entry names as its parents.
pub fn parent_ids(child: &Value) -> Vec<&str> {
    child
        .get("parent_ids")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .collect()
}

/// The `unions[]` entry of one spouse.
fn own_entry<'a>(family: &'a Value, spouse: &str) -> Option<&'a Value> {
    family
        .pointer("/union/unions")
        .and_then(Value::as_array)?
        .iter()
        .find(|s| s.get("spouse_id").and_then(Value::as_str) == Some(spouse))
}

/// The couples of one family. A union that is not polygamous, a family with
/// no union (its partners empty) and a polygamous couple of two are one
/// couple each; a polygamous union of more is one couple per spouse, and one
/// open couple for the children whose other parent is not recorded.
pub fn couples<'a>(family: &'a Value, persons: Option<&Map<String, Value>>) -> Vec<Couple<'a>> {
    let children: Vec<&Value> = family
        .get("children")
        .and_then(Value::as_array)
        .map(|a| a.iter().collect())
        .unwrap_or_default();
    let members = members(family);
    let polygamous = is_polygamous(family);
    let Some(p) = primary(family, persons) else {
        let own = if polygamous && members.len() == 2 {
            own_entry(family, members[1]).or_else(|| own_entry(family, members[0]))
        } else {
            None
        };
        return vec![Couple {
            partners: members,
            children,
            polygamous,
            own,
            open: false,
        }];
    };
    let spouses: Vec<&str> = members.iter().copied().filter(|m| *m != p).collect();
    if spouses.len() <= 1 {
        let own = spouses.first().and_then(|s| own_entry(family, s));
        let mut partners = vec![p];
        partners.extend(spouses);
        return vec![Couple {
            partners,
            children,
            polygamous,
            own,
            open: false,
        }];
    }
    let mut out: Vec<Couple> = spouses
        .iter()
        .map(|s| Couple {
            partners: vec![p, s],
            children: Vec::new(),
            polygamous,
            own: own_entry(family, s),
            open: false,
        })
        .collect();
    let mut open = Vec::new();
    for c in children {
        let named = parent_ids(c);
        match spouses.iter().position(|s| named.contains(s)) {
            Some(i) => out[i].children.push(c),
            None => open.push(c),
        }
    }
    if !open.is_empty() {
        out.push(Couple {
            partners: vec![p],
            children: open,
            polygamous,
            own: None,
            open: true,
        });
    }
    out
}

/// Remove `person` from a family's polygamous fields: their `unions[]`
/// entry, `primary_person_id` when it names them, and every child's
/// `parent_ids`. For an editor that has just taken them out of the union, so
/// that nothing is left pointing at somebody who is no longer in it.
pub fn forget(family: &mut Value, person: &str) {
    if let Some(union) = family.get_mut("union").and_then(Value::as_object_mut) {
        if union.get("primary_person_id").and_then(Value::as_str) == Some(person) {
            union.remove("primary_person_id");
        }
        if let Some(list) = union.get_mut("unions").and_then(Value::as_array_mut) {
            list.retain(|s| s.get("spouse_id").and_then(Value::as_str) != Some(person));
            if list.is_empty() {
                union.remove("unions");
            }
        }
    }
    if let Some(kids) = family.get_mut("children").and_then(Value::as_array_mut) {
        for c in kids.iter_mut().filter_map(Value::as_object_mut) {
            if let Some(ids) = c.get_mut("parent_ids").and_then(Value::as_array_mut) {
                ids.retain(|v| v.as_str() != Some(person));
                if ids.is_empty() {
                    c.remove("parent_ids");
                }
            }
        }
    }
}

/// Every spouse `unions[]` names is also listed in `union.persons`, as the
/// specification asks, so that a reader that knows only `persons` still
/// finds everyone in the union.
pub fn list_every_spouse(family: &mut Value) {
    let spouses: Vec<String> = family
        .pointer("/union/unions")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|s| s.get("spouse_id").and_then(Value::as_str))
        .map(str::to_string)
        .collect();
    let Some(persons) = family
        .pointer_mut("/union/persons")
        .and_then(Value::as_array_mut)
    else {
        return;
    };
    for s in spouses {
        if !persons
            .iter()
            .any(|p| p.get("person_id").and_then(Value::as_str) == Some(s.as_str()))
        {
            persons.push(serde_json::json!({"person_id": s, "role": "spouse"}));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn persons() -> Map<String, Value> {
        let mut m = Map::new();
        for (id, sex) in [("h", "M"), ("w1", "F"), ("w2", "F"), ("w3", "F")] {
            m.insert(
                id.into(),
                json!({"id": id, "identity": {"gender": {"value": sex}}}),
            );
        }
        m
    }

    #[test]
    fn a_couple_is_one_couple() {
        let f = json!({"union": {"type": "marriage", "persons": [
            {"person_id": "h", "role": "spouse"}, {"person_id": "w1", "role": "spouse"}]},
            "children": [{"person_id": "c1"}]});
        let cs = couples(&f, Some(&persons()));
        assert_eq!(cs.len(), 1);
        assert_eq!(cs[0].partners, vec!["h", "w1"]);
        assert!(!cs[0].polygamous && !cs[0].open);
        assert_eq!(cs[0].children.len(), 1);
    }

    #[test]
    fn the_specifications_polygamous_shape_is_one_couple_per_wife() {
        // AXGF 1.0 §4.2.2, as the reference library round-trips it: the
        // primary person in `persons`, each wife in `unions[]`.
        let f = json!({"union": {"type": "polygamous",
            "persons": [{"person_id": "h", "role": "spouse"}],
            "primary_person_id": "h",
            "unions": [
                {"spouse_id": "w1", "start": {"date": {"value": "1890"}}},
                {"spouse_id": "w2", "start": {"date": {"value": "1895"}}, "status": "ended_by_divorce"}]},
            "children": [
                {"person_id": "a", "parent_ids": ["h", "w1"]},
                {"person_id": "b", "parent_ids": ["h", "w2"]},
                {"person_id": "c"}]});
        assert_eq!(members(&f), vec!["h", "w1", "w2"]);
        let cs = couples(&f, Some(&persons()));
        assert_eq!(cs.len(), 3);
        assert_eq!(cs[0].partners, vec!["h", "w1"]);
        assert_eq!(cs[1].partners, vec!["h", "w2"]);
        assert_eq!(cs[1].status(&f), Some("ended_by_divorce"));
        assert_eq!(
            cs[0]
                .field(&f, "start")
                .and_then(|s| s.pointer("/date/value")),
            Some(&json!("1890"))
        );
        // The child whose mother the record does not name stays with the
        // father alone.
        assert_eq!(cs[2].partners, vec!["h"]);
        assert!(cs[2].open);
        assert_eq!(cs[2].children[0]["person_id"], "c");
    }

    #[test]
    fn the_primary_person_is_found_when_nobody_names_them() {
        let three = |roles: [&str; 3]| {
            json!({"union": {"type": "polygamous", "persons": [
                {"person_id": "w1", "role": roles[0]},
                {"person_id": "h", "role": roles[1]},
                {"person_id": "w2", "role": roles[2]}]}})
        };
        // By role: one husband among wives.
        assert_eq!(
            primary(&three(["wife", "husband", "wife"]), None),
            Some("h")
        );
        // By sex, when every role is "spouse".
        assert_eq!(
            primary(&three(["spouse", "spouse", "spouse"]), Some(&persons())),
            Some("h")
        );
        // Else the first listed.
        assert_eq!(
            primary(&three(["spouse", "spouse", "spouse"]), None),
            Some("w1")
        );
        // A polygamous couple of two is a couple.
        let two = json!({"union": {"type": "polygamous", "persons": [
            {"person_id": "h", "role": "spouse"}, {"person_id": "w1", "role": "spouse"}]}});
        assert_eq!(primary(&two, Some(&persons())), None);
        let cs = couples(&two, Some(&persons()));
        assert_eq!(cs.len(), 1);
        assert!(cs[0].polygamous);
    }

    #[test]
    fn a_spouse_taken_out_is_forgotten_everywhere() {
        let mut f = json!({"union": {"type": "polygamous", "primary_person_id": "h",
            "persons": [{"person_id": "h", "role": "husband"}, {"person_id": "w1", "role": "wife"}],
            "unions": [{"spouse_id": "w1"}, {"spouse_id": "w2"}]},
            "children": [{"person_id": "a", "parent_ids": ["h", "w2"]}]});
        list_every_spouse(&mut f);
        assert_eq!(members(&f), vec!["h", "w1", "w2"]);
        assert_eq!(f["union"]["persons"].as_array().unwrap().len(), 3);
        forget(&mut f, "w2");
        assert_eq!(f["union"]["unions"], json!([{"spouse_id": "w1"}]));
        assert_eq!(f["children"][0]["parent_ids"], json!(["h"]));
        forget(&mut f, "h");
        assert!(f["union"].get("primary_person_id").is_none());
    }
}
