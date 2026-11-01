//! Ties of rite and custom: godparents and the other bonds a culture makes
//! between people who are not born to one another (the operator's request of
//! 1 November 2026, docs/adr/0020).
//!
//! A godmother links a child to another family; so does a milk mother, a
//! namesake, a compadre, a sworn brother. None of them is a parent, and the
//! drawings show them apart from the family itself, in a dotted line of their
//! own colour. This module names them.
//!
//! # Stored in AXGF 1.1, without leaving it
//!
//! AXGF 1.1 records such a tie as a Link (§5.13): `relation` from a closed
//! vocabulary (`godparent`, `godchild`, `guardian`, `ward`, `mentor`,
//! `apprentice`, `witness`, `other`), the record's own words in `label` and
//! `label_reverse`. The vocabulary is frozen with 1.1.0, and it has no word
//! for a milk sibling or a namesake. So each rite here is stored as the 1.1
//! relation closest to it, and the rite itself is kept where the format puts
//! a vendor's data (1.0 §10.4), namespaced and versioned like this
//! application's other extensions:
//!
//! ```json
//! "relation": "other", "category": "spiritual",
//! "label": "mère de lait", "label_reverse": "enfant de lait",
//! "extensions": {"axgf-cms:kinship/v1": {"rite": "milk_parent", "sponsor": "from"}}
//! ```
//!
//! Any other reader still sees a valid Link with its words. A Link that
//! carries no extension is read by its `relation` alone: `godparent` and
//! `godchild` are godparenthood whatever the tradition, `guardian` and
//! `ward` a guardianship. The proposal to name these rites in the format
//! itself is axgf-spec's proposals/0002.

use serde_json::{json, Value};

/// Where this application keeps a Link's rite.
pub const EXT_KEY: &str = "axgf-cms:kinship/v1";

/// One tie of rite or custom.
#[derive(Debug, PartialEq, Eq)]
pub struct Rite {
    /// The stable key: `kin-rite-{key}` names it, `kin-rite-{key}-note`
    /// says what it is.
    pub key: &'static str,
    /// The tradition it belongs to, for grouping: `kin-tradition-{key}`.
    pub tradition: &'static str,
    /// The AXGF 1.1 `relation` it is stored as, from the sponsor's end, and
    /// from the other end. A mutual tie stores the same word both ways.
    pub relation: &'static str,
    pub reverse: &'static str,
    /// The AXGF `category` it is stored with.
    pub category: &'static str,
    /// The same tie both ways: milk siblings, namesakes, sworn siblings.
    pub mutual: bool,
    /// The word the tradition itself uses, shown as it is, untranslated:
    /// "кум · кума", "compadre · comadre". Empty where the rite has no word
    /// of its own beyond the reader's language.
    pub native: &'static str,
}

/// The traditions, in the order an editor lists them.
pub const TRADITIONS: &[&str] = &[
    "christian",
    "orthodox",
    "hispanic",
    "jewish",
    "islamic",
    "south-asian",
    "east-asian",
    "african",
    "indigenous",
    "civil",
];

macro_rules! rite {
    ($key:literal, $tradition:literal, $relation:literal, $reverse:literal, $category:literal, $mutual:literal, $native:literal) => {
        Rite {
            key: $key,
            tradition: $tradition,
            relation: $relation,
            reverse: $reverse,
            category: $category,
            mutual: $mutual,
            native: $native,
        }
    };
}

/// Every rite this application names. The first, `godparent`, is the plain
/// AXGF 1.1 godparenthood, which every reader of the format understands: a
/// Link with `relation: godparent` and no extension is this one.
pub const RITES: &[Rite] = &[
    rite!(
        "godparent",
        "christian",
        "godparent",
        "godchild",
        "spiritual",
        false,
        ""
    ),
    rite!(
        "baptism",
        "christian",
        "godparent",
        "godchild",
        "spiritual",
        false,
        ""
    ),
    rite!(
        "confirmation",
        "christian",
        "godparent",
        "godchild",
        "spiritual",
        false,
        ""
    ),
    rite!(
        "wedding-sponsor",
        "christian",
        "witness",
        "other",
        "spiritual",
        false,
        ""
    ),
    rite!(
        "kum",
        "orthodox",
        "godparent",
        "godchild",
        "spiritual",
        false,
        "кум · кума"
    ),
    rite!(
        "koumbaros",
        "orthodox",
        "godparent",
        "godchild",
        "spiritual",
        false,
        "κουμπάρος · κουμπάρα"
    ),
    rite!(
        "padrino",
        "hispanic",
        "godparent",
        "godchild",
        "spiritual",
        false,
        "padrino · madrina"
    ),
    rite!(
        "compadre",
        "hispanic",
        "other",
        "other",
        "spiritual",
        true,
        "compadre · comadre"
    ),
    rite!(
        "ninong",
        "hispanic",
        "godparent",
        "godchild",
        "spiritual",
        false,
        "ninong · ninang"
    ),
    rite!(
        "sandek",
        "jewish",
        "godparent",
        "godchild",
        "spiritual",
        false,
        "סנדק"
    ),
    rite!(
        "kvatter",
        "jewish",
        "godparent",
        "godchild",
        "spiritual",
        false,
        "קוואטער · קוואטערין"
    ),
    rite!(
        "milk-parent",
        "islamic",
        "other",
        "other",
        "spiritual",
        false,
        "أم من الرضاعة"
    ),
    rite!(
        "milk-sibling",
        "islamic",
        "other",
        "other",
        "spiritual",
        true,
        "أخ من الرضاعة · أخت من الرضاعة"
    ),
    rite!(
        "kafala",
        "islamic",
        "guardian",
        "ward",
        "legal",
        false,
        "كفالة"
    ),
    rite!(
        "rakhi",
        "south-asian",
        "other",
        "other",
        "spiritual",
        true,
        "राखी"
    ),
    rite!(
        "guru",
        "south-asian",
        "mentor",
        "apprentice",
        "spiritual",
        false,
        "गुरु · शिष्य"
    ),
    rite!(
        "gan-parent",
        "east-asian",
        "godparent",
        "godchild",
        "social",
        false,
        "干爹 · 干妈"
    ),
    rite!(
        "sworn-sibling",
        "east-asian",
        "other",
        "other",
        "social",
        true,
        "结拜"
    ),
    rite!(
        "name-giver",
        "east-asian",
        "other",
        "other",
        "social",
        false,
        "名付け親"
    ),
    rite!(
        "nakodo",
        "east-asian",
        "witness",
        "other",
        "social",
        false,
        "仲人"
    ),
    rite!(
        "namesake",
        "african",
        "other",
        "other",
        "social",
        true,
        "ndoyi · somo"
    ),
    rite!(
        "initiation",
        "african",
        "mentor",
        "apprentice",
        "spiritual",
        false,
        ""
    ),
    rite!(
        "blood-pact",
        "african",
        "other",
        "other",
        "social",
        true,
        ""
    ),
    rite!(
        "hanai",
        "indigenous",
        "guardian",
        "ward",
        "social",
        false,
        "hānai"
    ),
    rite!(
        "hunka",
        "indigenous",
        "other",
        "other",
        "spiritual",
        false,
        "huŋká"
    ),
    rite!(
        "civil-sponsor",
        "civil",
        "godparent",
        "godchild",
        "social",
        false,
        ""
    ),
    rite!("guardian", "civil", "guardian", "ward", "legal", false, ""),
];

/// A rite by key.
pub fn find(key: &str) -> Option<&'static Rite> {
    RITES.iter().find(|r| r.key == key)
}

/// Which end of a tie a person stands at.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Role {
    /// The godparent, the milk mother, the guru, the guardian.
    Sponsor,
    /// The godchild, the milk child, the disciple, the ward.
    Sponsored,
    /// Both ends alike: milk siblings, namesakes, compadres.
    Mutual,
}

impl Role {
    /// The Fluent selector value.
    pub fn as_str(self) -> &'static str {
        match self {
            Role::Sponsor => "sponsor",
            Role::Sponsored => "sponsored",
            Role::Mutual => "mutual",
        }
    }
    /// The other end's role.
    pub fn other(self) -> Role {
        match self {
            Role::Sponsor => Role::Sponsored,
            Role::Sponsored => Role::Sponsor,
            Role::Mutual => Role::Mutual,
        }
    }
}

/// A Link read as a tie of rite or custom: which rite, and the role of its
/// `from` end. `None` for every other Link — an employer, a neighbour, a
/// witness to a deed are relations, not kin.
pub fn read(link: &Value) -> Option<(&'static Rite, Role)> {
    let ext = link.get("extensions").and_then(|e| e.get(EXT_KEY));
    if let Some(rite) = ext
        .and_then(|e| e.get("rite"))
        .and_then(Value::as_str)
        .and_then(find)
    {
        let role = if rite.mutual {
            Role::Mutual
        } else {
            match ext.and_then(|e| e.get("sponsor")).and_then(Value::as_str) {
                Some("to") => Role::Sponsored,
                Some("from") => Role::Sponsor,
                // No side stated: the relation's own direction says it.
                _ => match link.get("relation").and_then(Value::as_str) {
                    Some(r) if r == rite.reverse && r != rite.relation => Role::Sponsored,
                    _ => Role::Sponsor,
                },
            }
        };
        return Some((rite, role));
    }
    let godparent = find("godparent")?;
    let guardian = find("guardian")?;
    match link.get("relation").and_then(Value::as_str)? {
        "godparent" => Some((godparent, Role::Sponsor)),
        "godchild" => Some((godparent, Role::Sponsored)),
        "guardian" => Some((guardian, Role::Sponsor)),
        "ward" => Some((guardian, Role::Sponsored)),
        _ => None,
    }
}

/// What a Link stores for `rite`, seen from its `from` end in `role`: the
/// 1.1 `relation`, its `category`, and the extension that keeps the rite.
pub fn stored(rite: &Rite, role: Role) -> (&'static str, &'static str, Value) {
    let relation = match role {
        Role::Sponsored => rite.reverse,
        Role::Sponsor | Role::Mutual => rite.relation,
    };
    let mut ext = json!({"rite": rite.key});
    if !rite.mutual {
        ext["sponsor"] = json!(if role == Role::Sponsored {
            "to"
        } else {
            "from"
        });
    }
    (relation, rite.category, ext)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_rite_is_stored_as_a_word_the_format_knows() {
        // AXGF 1.1's closed vocabularies, as the schema enumerates them.
        let relations = [
            "godparent",
            "godchild",
            "witness",
            "officiant",
            "business_partner",
            "employer",
            "employee",
            "mentor",
            "apprentice",
            "close_friend",
            "neighbour",
            "guardian",
            "ward",
            "other",
        ];
        let categories = [
            "spiritual",
            "professional",
            "social",
            "legal",
            "medical",
            "educational",
            "conflict",
            "other",
        ];
        for r in RITES {
            assert!(relations.contains(&r.relation), "{}", r.key);
            assert!(relations.contains(&r.reverse), "{}", r.key);
            assert!(categories.contains(&r.category), "{}", r.key);
            assert!(TRADITIONS.contains(&r.tradition), "{}", r.key);
            assert!(r.key.chars().all(|c| c.is_ascii_lowercase() || c == '-'));
        }
        let mut keys: Vec<&str> = RITES.iter().map(|r| r.key).collect();
        keys.sort_unstable();
        keys.dedup();
        assert_eq!(keys.len(), RITES.len(), "a key twice");
    }

    #[test]
    fn a_rite_reads_back_as_it_was_stored_from_either_end() {
        for rite in RITES {
            let roles: &[Role] = if rite.mutual {
                &[Role::Mutual]
            } else {
                &[Role::Sponsor, Role::Sponsored]
            };
            for &role in roles {
                let (relation, category, ext) = stored(rite, role);
                let link = json!({"relation": relation, "category": category,
                                  "extensions": {EXT_KEY: ext}});
                assert_eq!(read(&link), Some((rite, role)), "{} {role:?}", rite.key);
            }
        }
    }

    #[test]
    fn a_plain_godparent_link_is_godparenthood() {
        let g = find("godparent").unwrap();
        assert_eq!(
            read(&json!({"relation": "godparent", "label": "matka chrzestna"})),
            Some((g, Role::Sponsor))
        );
        assert_eq!(
            read(&json!({"relation": "godchild"})),
            Some((g, Role::Sponsored))
        );
        // A witness to a deed, an employer, a friend: relations, not kin.
        for r in ["witness", "employer", "close_friend", "mentor", "other"] {
            assert_eq!(read(&json!({"relation": r})), None, "{r}");
        }
        assert_eq!(read(&json!({"label": "voisin"})), None);
    }
}
