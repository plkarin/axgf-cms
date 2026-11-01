//! How two people of the family are related, and every relative of one (the
//! operator's request of 1 November 2026, docs/adr/0020).
//!
//! The family's records state only near facts: a child of a union, a partner
//! in one, a godmother. Everything else — a third cousin, a great-niece, a
//! sister-in-law, a stepfather, the godson of an uncle — is a path through
//! those facts. This module finds the paths and says what each one is.
//!
//! * **By descent.** Two people share an ancestor (or one descends from the
//!   other): A is `up` generations below it and B `down`. (1, 1) is a
//!   brother or sister, (2, 1) an uncle or aunt, (3, 3) a second cousin,
//!   (3, 2) a first cousin one generation older. Descent counts birth and
//!   adoption; a path through an adoption says so. Two children of one union
//!   are full siblings; children of two unions sharing one parent are half
//!   siblings, and so on up any line. A family married into twice gives two
//!   paths, and both are kept.
//! * **By marriage.** A's partner, a relative of A's partner, the partner of
//!   A's relative: a mother-in-law, a brother-in-law, a stepson, the wife of a
//!   cousin.
//! * **By care.** A foster parent, a guardian, a stepfather the family's
//!   records name as such (`lineage`).
//! * **By rite or custom** ([`crate::rites`]): a godmother, a milk sibling, a
//!   namesake.
//! * **Otherwise**, if the records connect them at all, the chain of steps.
//!
//! The paths run through the structure of the family whatever a reader may
//! see, as the tree does: a person this reader may not read keeps their place
//! in a path and is named by nobody ([`crate::access`]). A tie of rite is a
//! Link, and a Link this reader may not see is not followed.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use serde_json::Value;

use crate::access::Lens;
use crate::rites::{Rite, Role};

/// A person's sex as the record states it: what a kinship word agrees with.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sex {
    M,
    F,
    U,
}

impl Sex {
    /// The sex a person's record states.
    pub fn of_person(p: Option<&Value>) -> Sex {
        Sex::of(p)
    }
    fn of(p: Option<&Value>) -> Sex {
        match crate::tree::tone_of(p) {
            "a" => Sex::M,
            "b" => Sex::F,
            _ => Sex::U,
        }
    }
    /// The Fluent selector value.
    pub fn as_str(self) -> &'static str {
        match self {
            Sex::M => "M",
            Sex::F => "F",
            Sex::U => "U",
        }
    }
}

/// How a child belongs to a parent (AXGF 1.1 `lineage`), the strongest
/// first: a mother who is also the partner of a second union the child is a
/// step-child of is still the mother.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Line {
    /// Biological, or unstated (the 1.0 reading).
    Birth,
    Adoption,
    Foster,
    Guardianship,
    Step,
}

impl Line {
    fn of(term: Option<&str>) -> Line {
        match term {
            Some("adoptive") => Line::Adoption,
            Some("foster") => Line::Foster,
            Some("guardianship") => Line::Guardianship,
            Some("step") => Line::Step,
            _ => Line::Birth,
        }
    }
    /// Descent: birth and adoption make a child of the family's blood line;
    /// fostering, guardianship and a step-parent raise a child without it.
    pub fn is_descent(self) -> bool {
        matches!(self, Line::Birth | Line::Adoption)
    }
    /// The Fluent selector value.
    pub fn as_str(self) -> &'static str {
        match self {
            Line::Birth => "birth",
            Line::Adoption => "adoption",
            Line::Foster => "foster",
            Line::Guardianship => "guardianship",
            Line::Step => "step",
        }
    }
}

/// The parent that stands for the unrecorded parents of a sibling group
/// (AXGF 1.0 §4.2.3: a family with children and no union). Its children are
/// brothers and sisters through it. It is named by nobody and listed
/// nowhere, and a way through it is shown as the siblings' own step.
const UNRECORDED: &str = "unrecorded-parents:";

/// Whether `id` is the unrecorded parent of a sibling group.
pub fn is_unrecorded(id: &str) -> bool {
    id.starts_with(UNRECORDED)
}

#[derive(Debug, Clone, Copy)]
struct Up {
    parent: usize,
    line: Line,
    family: usize,
}

#[derive(Debug, Clone, Copy)]
struct Partner {
    to: usize,
    married: bool,
    ended: bool,
}

#[derive(Debug, Clone, Copy)]
struct Tie {
    to: usize,
    rite: &'static Rite,
    /// This person's role.
    role: Role,
}

/// One step of a path: who it reaches, and what they are to the person
/// before them.
#[derive(Debug, Clone, PartialEq)]
pub enum Hop {
    /// The parent of the one before.
    Parent(Line),
    /// The child of the one before.
    Child(Line),
    /// The partner of the one before.
    Partner { married: bool, ended: bool },
    /// Tied to the one before by a rite, in this role.
    Rite(&'static Rite, Role),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Step {
    pub person: String,
    pub hop: Hop,
}

/// What a relationship is, before it is put in words.
#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
    /// By descent: A is `up` generations below the ancestor they share with
    /// B, and B `down`. (n, 0): B is A's ancestor; (0, n): A's descendant.
    Blood {
        up: u32,
        down: u32,
        /// Through one ancestor of a couple only: a half-brother, a half
        /// first cousin.
        half: bool,
        /// Some step of the way is an adoption.
        adoptive: bool,
    },
    /// Partners in a union.
    Partner { married: bool, ended: bool },
    /// By marriage: the relationship by descent between A, or A's partner
    /// (`a_side`), and B, or B's partner (`b_side`).
    Affinity {
        core: Box<Kind>,
        a_side: bool,
        b_side: bool,
    },
    /// Raised by, or raising, without descent: a step-parent, a foster
    /// parent, a guardian the family's records name (`lineage`). `up`: B
    /// raised A.
    Care { line: Line, up: bool },
    /// A rite or a custom, B in `role`.
    Rite { rite: &'static Rite, role: Role },
    /// Connected by the records, in no way with a name of its own.
    Chain,
}

/// The facts a kinship word may agree with besides B's own sex: the side of
/// the family, an elder or a younger, a son's line or a daughter's.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Context {
    /// The sex of the person next to A on the way: A's parent the path climbs
    /// through (the father's side or the mother's), or A's child it goes down
    /// through, or A's partner.
    pub a_next: Option<Sex>,
    /// The sex of the person next to B on the way: B's parent, B's child, or
    /// B's partner.
    pub b_next: Option<Sex>,
    /// The two children of the shared ancestor the paths go down through: on
    /// A's side and on B's. For an uncle, A's parent and the uncle; for a
    /// nephew, A and A's brother or sister.
    pub turn_a: Option<Sex>,
    pub turn_b: Option<Sex>,
    /// Every person between A and the shared ancestor is a man, and so on
    /// B's side: one male line, which Chinese names apart (堂 against 表).
    pub male_line: bool,
    /// B is older than A, when both births are recorded.
    pub b_elder: Option<bool>,
    /// B's side's child of the shared ancestor is older than A's: an uncle
    /// older than A's father (伯父 against 叔父).
    pub turn_elder: Option<bool>,
    /// Between cousins of two generations, the one a word names besides B:
    /// when B is of an older generation, A's forebear at B's generation (B is
    /// "a first cousin of A's father"); when younger, B's forebear at A's
    /// generation (B is "the son of A's first cousin").
    pub relay: Option<Sex>,
}

/// One way B is related to A.
#[derive(Debug, Clone, PartialEq)]
pub struct Relation {
    pub kind: Kind,
    /// From A to B, one step at a time; A is not in it.
    pub path: Vec<Step>,
    /// The ancestors A and B share, for a relationship by descent: a couple,
    /// or one person.
    pub common: Vec<String>,
    pub context: Context,
}

impl Relation {
    /// How close: fewer steps first, a step of care, marriage or rite
    /// counting more than one by descent; then descent before care before
    /// marriage before rite; then the nearer generation. A tie of rite
    /// weighs as much as four steps of descent, the way to a first cousin:
    /// a grandmother, an aunt or a first cousin who is also a godmother is
    /// first a grandmother, an aunt or a cousin, and a second cousin is first
    /// a godmother.
    pub fn rank(&self) -> (u32, u32, u32) {
        let class = match &self.kind {
            Kind::Blood { .. } => 0,
            Kind::Partner { .. } => 0,
            Kind::Care { .. } => 1,
            Kind::Affinity { .. } => 2,
            Kind::Rite { .. } => 3,
            Kind::Chain => 4,
        };
        let steps = self.path.len() as u32;
        let spread = match &self.kind {
            Kind::Blood { up, down, .. } => up.abs_diff(*down),
            _ => 0,
        };
        (steps + class, class, spread)
    }
}

/// The deepest an ancestry is followed, in generations.
const MAX_UP: usize = 24;
/// How many ways to one ancestor are kept: a family that married into itself
/// reaches the same forebear several ways, and each is a relationship.
const MAX_WAYS: usize = 6;
/// The longest chain looked for between two people related in no named way.
const MAX_CHAIN: usize = 10;

/// The family's structure, read once per request.
pub struct Kin<'a> {
    lens: &'a Lens,
    ids: Vec<String>,
    index: BTreeMap<String, usize>,
    /// Held by the bundle, rather than only named by a family.
    known: Vec<bool>,
    sex: Vec<Sex>,
    born: Vec<Option<i64>>,
    /// Each person's parents, by their strongest tie.
    ups: Vec<Vec<Up>>,
    /// Each person's children, by their strongest tie.
    downs: Vec<Vec<Up>>,
    partners: Vec<Vec<Partner>>,
    ties: Vec<Vec<Tie>>,
}

/// One way from a person up to an ancestor: the people on it, the person
/// first, the ancestor last, and the family each step climbs through.
#[derive(Debug, Clone)]
struct Way {
    people: Vec<usize>,
    families: Vec<usize>,
    adoptive: bool,
}

impl<'a> Kin<'a> {
    pub fn new(flat: &Value, lens: &'a Lens) -> Kin<'a> {
        let empty = serde_json::Map::new();
        let persons = flat
            .get("persons")
            .and_then(Value::as_object)
            .unwrap_or(&empty);
        let mut ids: Vec<String> = persons.keys().cloned().collect();
        let mut index: BTreeMap<String, usize> = BTreeMap::new();
        for (i, id) in ids.iter().enumerate() {
            index.insert(id.clone(), i);
        }
        // A family names somebody the bundle does not hold: they still stand
        // in the structure, as the tree draws them.
        let add = |id: &str, ids: &mut Vec<String>, index: &mut BTreeMap<String, usize>| {
            if let Some(&i) = index.get(id) {
                return i;
            }
            ids.push(id.to_string());
            index.insert(id.to_string(), ids.len() - 1);
            ids.len() - 1
        };

        let mut parent_links: Vec<(usize, usize, Line, usize)> = Vec::new();
        let mut unions: Vec<(usize, usize, bool, bool)> = Vec::new();
        let families = flat.get("families").and_then(Value::as_object);
        for (fi, f) in families.into_iter().flat_map(|m| m.values()).enumerate() {
            let union = f.get("union");
            let parents: Vec<usize> = union
                .and_then(|u| u.get("persons"))
                .and_then(Value::as_array)
                .map(|ps| {
                    ps.iter()
                        .filter_map(|p| p.get("person_id").and_then(Value::as_str))
                        .map(|id| add(id, &mut ids, &mut index))
                        .collect()
                })
                .unwrap_or_default();
            // Children whose parents nobody recorded are still brothers and
            // sisters: a family with no union at all (1.0 §4.2.3).
            let children = f.get("children").and_then(Value::as_array).map_or(0, |cs| {
                cs.iter()
                    .filter(|c| c.get("person_id").and_then(Value::as_str).is_some())
                    .count()
            });
            let parents = if union.is_none() && children >= 2 {
                vec![add(&format!("{UNRECORDED}{fi}"), &mut ids, &mut index)]
            } else {
                parents
            };
            let kind = union.and_then(|u| u.get("type")).and_then(Value::as_str);
            let status = union.and_then(|u| u.get("status")).and_then(Value::as_str);
            let married = matches!(kind, Some("marriage" | "religious_only" | "polygamous"));
            let ended = matches!(
                status,
                Some("ended_by_divorce" | "ended_by_separation" | "annulled")
            );
            for (i, &a) in parents.iter().enumerate() {
                for &b in parents.iter().skip(i + 1) {
                    if a != b {
                        unions.push((a, b, married, ended));
                    }
                }
            }
            for c in f
                .get("children")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
            {
                let Some(cid) = c.get("person_id").and_then(Value::as_str) else {
                    continue;
                };
                let child = add(cid, &mut ids, &mut index);
                let line = Line::of(c.get("lineage").and_then(Value::as_str));
                for &p in &parents {
                    if p != child {
                        parent_links.push((child, p, line, fi));
                    }
                }
            }
        }

        let n = ids.len();
        let mut ups: Vec<Vec<Up>> = vec![Vec::new(); n];
        for (child, parent, line, family) in parent_links {
            let list = &mut ups[child];
            match list.iter_mut().find(|u| u.parent == parent) {
                Some(u) if line < u.line => {
                    u.line = line;
                    u.family = family;
                }
                Some(_) => {}
                None => list.push(Up {
                    parent,
                    line,
                    family,
                }),
            }
        }
        let mut downs: Vec<Vec<Up>> = vec![Vec::new(); n];
        for (child, list) in ups.iter().enumerate() {
            for u in list {
                downs[u.parent].push(Up {
                    parent: child,
                    line: u.line,
                    family: u.family,
                });
            }
        }
        let mut partners: Vec<Vec<Partner>> = vec![Vec::new(); n];
        for (a, b, married, ended) in unions {
            for (x, y) in [(a, b), (b, a)] {
                if !partners[x].iter().any(|p| p.to == y) {
                    partners[x].push(Partner {
                        to: y,
                        married,
                        ended,
                    });
                }
            }
        }

        let mut ties: Vec<Vec<Tie>> = vec![Vec::new(); n];
        for l in flat
            .get("links")
            .and_then(Value::as_object)
            .into_iter()
            .flat_map(|m| m.values())
        {
            if !lens.sees_entity(l) {
                continue;
            }
            let Some((rite, role)) = crate::rites::read(l) else {
                continue;
            };
            let end = |k: &str| -> Vec<usize> {
                let Some(e) = l.get(k) else { return Vec::new() };
                let id = e.get("entity_id").and_then(Value::as_str).unwrap_or("");
                match e.get("entity_type").and_then(Value::as_str) {
                    Some("person") => index.get(id).copied().into_iter().collect(),
                    // A tie to a couple — the sponsors of a wedding — ties
                    // to each partner of it.
                    Some("family") => families
                        .and_then(|m| m.get(id))
                        .and_then(|f| f.pointer("/union/persons"))
                        .and_then(Value::as_array)
                        .into_iter()
                        .flatten()
                        .filter_map(|p| p.get("person_id").and_then(Value::as_str))
                        .filter_map(|id| index.get(id).copied())
                        .collect(),
                    _ => Vec::new(),
                }
            };
            for &x in &end("from") {
                for &y in &end("to") {
                    if x == y {
                        continue;
                    }
                    ties[x].push(Tie { to: y, rite, role });
                    ties[y].push(Tie {
                        to: x,
                        rite,
                        role: role.other(),
                    });
                }
            }
        }

        let known = ids.iter().map(|id| persons.contains_key(id)).collect();
        let sex = ids.iter().map(|id| Sex::of(persons.get(id))).collect();
        let born = ids
            .iter()
            .map(|id| {
                persons.get(id).and_then(|p| {
                    crate::view::render_date_field(p.get("birth").unwrap_or(&Value::Null), "date")
                        .sort
                })
            })
            .collect();
        Kin {
            lens,
            ids,
            index,
            known,
            sex,
            born,
            ups,
            downs,
            partners,
            ties,
        }
    }

    /// The sex the record gives `id`.
    pub fn sex_of(&self, id: &str) -> Sex {
        self.index.get(id).map_or(Sex::U, |&i| self.sex[i])
    }

    /// Whether this reader may name `id`.
    pub fn sees(&self, id: &str) -> bool {
        self.lens.sees_person(id)
    }

    /// `id`'s ties of rite and custom this reader may see: who, which rite,
    /// and what they are to `id`.
    pub fn ties_of(&self, id: &str) -> Vec<(String, &'static Rite, Role)> {
        let Some(&i) = self.index.get(id) else {
            return Vec::new();
        };
        self.ties[i]
            .iter()
            .map(|t| (self.ids[t.to].clone(), t.rite, t.role.other()))
            .collect()
    }

    /// Every way up from `a`: each ancestor by descent, with the ways that
    /// reach it, `a` itself at the bottom.
    fn ancestry(&self, a: usize) -> BTreeMap<usize, Vec<Way>> {
        let mut out: BTreeMap<usize, Vec<Way>> = BTreeMap::new();
        out.insert(
            a,
            vec![Way {
                people: vec![a],
                families: Vec::new(),
                adoptive: false,
            }],
        );
        let mut queue: VecDeque<Way> = VecDeque::new();
        queue.push_back(Way {
            people: vec![a],
            families: Vec::new(),
            adoptive: false,
        });
        while let Some(way) = queue.pop_front() {
            if way.people.len() > MAX_UP {
                continue;
            }
            let top = *way.people.last().expect("a way is never empty");
            for u in &self.ups[top] {
                if !u.line.is_descent() || way.people.contains(&u.parent) {
                    continue;
                }
                let mut next = way.clone();
                next.people.push(u.parent);
                next.families.push(u.family);
                next.adoptive |= u.line == Line::Adoption;
                let ways = out.entry(u.parent).or_default();
                if ways.len() >= MAX_WAYS {
                    continue;
                }
                ways.push(next.clone());
                queue.push_back(next);
            }
        }
        out
    }

    /// The ways A and B are related by descent.
    fn blood(
        &self,
        a: usize,
        b: usize,
        anc_a: &BTreeMap<usize, Vec<Way>>,
        anc_b: &BTreeMap<usize, Vec<Way>>,
    ) -> Vec<Relation> {
        let mut out = Vec::new();
        let mut seen: BTreeSet<(Vec<usize>, Vec<usize>, Option<usize>)> = BTreeSet::new();
        for (c, ways_a) in anc_a {
            let Some(ways_b) = anc_b.get(c) else { continue };
            for wa in ways_a {
                for wb in ways_b {
                    // The paths meet at the ancestor and nowhere below it:
                    // anywhere lower, that lower person is the one they share.
                    let below_a = &wa.people[..wa.people.len() - 1];
                    let below_b = &wb.people[..wb.people.len() - 1];
                    if below_a.iter().any(|p| below_b.contains(p)) {
                        continue;
                    }
                    if a == b {
                        continue;
                    }
                    // Through a couple when both paths come down through one
                    // of its unions: the couple is shared, and the relationship
                    // is full. Through two unions of one ancestor, it is half.
                    let fa = wa.families.last().copied();
                    let fb = wb.families.last().copied();
                    let (half, shared_family) = match (fa, fb) {
                        (Some(x), Some(y)) => (x != y, (x == y).then_some(x)),
                        _ => (false, None),
                    };
                    let key = (below_a.to_vec(), below_b.to_vec(), shared_family);
                    if !seen.insert(key) {
                        continue;
                    }
                    let up = (wa.people.len() - 1) as u32;
                    let down = (wb.people.len() - 1) as u32;
                    let common = match shared_family {
                        Some(f) => self.couple_of(f, below_a.last().copied(), *c),
                        None => vec![*c],
                    };
                    out.push(Relation {
                        kind: Kind::Blood {
                            up,
                            down,
                            half,
                            adoptive: wa.adoptive || wb.adoptive,
                        },
                        path: self.blood_path(wa, wb),
                        common: common.iter().map(|&i| self.ids[i].clone()).collect(),
                        context: self.blood_context(a, b, wa, wb),
                    });
                }
            }
        }
        out
    }

    /// The parents of family `f` who are parents by descent of `child`: the
    /// couple both paths come down from. `c` is always one of them.
    fn couple_of(&self, f: usize, child: Option<usize>, c: usize) -> Vec<usize> {
        let Some(child) = child else { return vec![c] };
        let mut out: Vec<usize> = self.ups[child]
            .iter()
            .filter(|u| u.family == f && u.line.is_descent())
            .map(|u| u.parent)
            .collect();
        if !out.contains(&c) {
            out.push(c);
        }
        // Father first, as charts name a couple.
        out.sort_by_key(|&p| match self.sex[p] {
            Sex::M => 0,
            Sex::U => 1,
            Sex::F => 2,
        });
        out
    }

    fn blood_path(&self, wa: &Way, wb: &Way) -> Vec<Step> {
        let mut path = Vec::new();
        for (i, &p) in wa.people.iter().enumerate().skip(1) {
            let line = self.line_between(wa.people[i - 1], p);
            path.push(Step {
                person: self.ids[p].clone(),
                hop: Hop::Parent(line),
            });
        }
        // Down B's side, from the ancestor's child to B.
        let down: Vec<usize> = wb.people.iter().rev().copied().collect();
        for (i, &p) in down.iter().enumerate().skip(1) {
            let line = self.line_between(p, down[i - 1]);
            path.push(Step {
                person: self.ids[p].clone(),
                hop: Hop::Child(line),
            });
        }
        path
    }

    /// How `child` belongs to `parent`.
    fn line_between(&self, child: usize, parent: usize) -> Line {
        self.ups[child]
            .iter()
            .find(|u| u.parent == parent)
            .map_or(Line::Birth, |u| u.line)
    }

    fn blood_context(&self, a: usize, b: usize, wa: &Way, wb: &Way) -> Context {
        let sex = |i: usize| self.sex[i];
        let older = |x: usize, y: usize| match (self.born[x], self.born[y]) {
            (Some(p), Some(q)) if p != q => Some(p < q),
            _ => None,
        };
        let pa = &wa.people;
        let pb = &wb.people;
        // A's neighbour: up through A's parent, or, when A is the ancestor,
        // down through A's child.
        let a_next = if pa.len() > 1 {
            Some(sex(pa[1]))
        } else {
            pb.get(pb.len().wrapping_sub(2)).map(|&i| sex(i))
        };
        let b_next = if pb.len() > 1 {
            Some(sex(pb[1]))
        } else {
            pa.get(pa.len().wrapping_sub(2)).map(|&i| sex(i))
        };
        let turn_a = pa.len().checked_sub(2).map(|i| pa[i]);
        let turn_b = pb.len().checked_sub(2).map(|i| pb[i]);
        let male =
            |w: &[usize]| w.len() <= 2 || w[1..w.len() - 1].iter().all(|&p| self.sex[p] == Sex::M);
        // Generations apart: the forebear on the nearer side at the farther
        // one's generation.
        let (ua, ub) = (pa.len() - 1, pb.len() - 1);
        let relay = match ua.cmp(&ub) {
            std::cmp::Ordering::Greater => pa.get(ua - ub).map(|&i| sex(i)),
            std::cmp::Ordering::Less => pb.get(ub - ua).map(|&i| sex(i)),
            std::cmp::Ordering::Equal => None,
        };
        Context {
            a_next,
            b_next,
            turn_a: turn_a.map(sex),
            turn_b: turn_b.map(sex),
            relay,
            male_line: male(pa) && male(pb),
            b_elder: older(b, a),
            turn_elder: match (turn_a, turn_b) {
                (Some(x), Some(y)) => older(y, x),
                _ => None,
            },
        }
    }

    /// Every way B is related to A, the closest first.
    pub fn between(&self, a: &str, b: &str) -> Vec<Relation> {
        let (Some(&ai), Some(&bi)) = (self.index.get(a), self.index.get(b)) else {
            return Vec::new();
        };
        if ai == bi {
            return Vec::new();
        }
        let mut memo: BTreeMap<usize, BTreeMap<usize, Vec<Way>>> = BTreeMap::new();
        self.between_idx(ai, bi, &mut memo)
    }

    fn between_idx(
        &self,
        a: usize,
        b: usize,
        memo: &mut BTreeMap<usize, BTreeMap<usize, Vec<Way>>>,
    ) -> Vec<Relation> {
        let anc = |x: usize, memo: &mut BTreeMap<usize, BTreeMap<usize, Vec<Way>>>| {
            memo.entry(x).or_insert_with(|| self.ancestry(x));
        };
        anc(a, memo);
        anc(b, memo);
        let mut out = self.blood(a, b, &memo[&a], &memo[&b]);
        // A sibling group often repeats children whom a family with parents
        // already holds (a GEDCOM import keeps both). A way through the
        // unrecorded parent then says nothing the recorded parents do not.
        let unrecorded = |r: &Relation| r.common.iter().any(|c| is_unrecorded(c));
        let recorded: Vec<(u32, u32)> = out
            .iter()
            .filter(|r| !unrecorded(r))
            .filter_map(|r| match &r.kind {
                Kind::Blood { up, down, .. } => Some((*up, *down)),
                _ => None,
            })
            .collect();
        out.retain(|r| match &r.kind {
            Kind::Blood { up, down, .. } => !unrecorded(r) || !recorded.contains(&(*up, *down)),
            _ => true,
        });

        // Partners.
        for p in self.partners[a].iter().filter(|p| p.to == b) {
            out.push(Relation {
                kind: Kind::Partner {
                    married: p.married,
                    ended: p.ended,
                },
                path: vec![Step {
                    person: self.ids[b].clone(),
                    hop: Hop::Partner {
                        married: p.married,
                        ended: p.ended,
                    },
                }],
                common: Vec::new(),
                context: Context {
                    b_elder: self.older(b, a),
                    ..Context::default()
                },
            });
        }

        // Care: a parent who raised a child without descent, either way.
        for u in self.ups[a]
            .iter()
            .filter(|u| u.parent == b && !u.line.is_descent())
        {
            out.push(self.care(b, u.line, true));
        }
        for u in self.ups[b]
            .iter()
            .filter(|u| u.parent == a && !u.line.is_descent())
        {
            out.push(self.care(b, u.line, false));
        }

        // Rites.
        for t in self.ties[a].iter().filter(|t| t.to == b) {
            out.push(Relation {
                kind: Kind::Rite {
                    rite: t.rite,
                    role: t.role.other(),
                },
                path: vec![Step {
                    person: self.ids[b].clone(),
                    hop: Hop::Rite(t.rite, t.role.other()),
                }],
                common: Vec::new(),
                context: Context {
                    b_elder: self.older(b, a),
                    ..Context::default()
                },
            });
        }

        // By marriage: a relative of A's partner, the partner of A's
        // relative, or both. A child of A's partner who is A's own child is
        // A's child, and the partner of A's parent who is A's parent is a
        // parent: neither is said twice.
        let own_blood: Vec<(u32, u32)> = out
            .iter()
            .filter_map(|r| match r.kind {
                Kind::Blood { up, down, .. } => Some((up, down)),
                _ => None,
            })
            .collect();
        let partners_a: Vec<Partner> = self.partners[a].clone();
        let partners_b: Vec<Partner> = self.partners[b].clone();
        // A partner through whom a nearer way by marriage was found is not
        // walked again for a farther one: the wife of ego's son is his
        // daughter-in-law, and not also the wife of his wife's son.
        let mut near: BTreeSet<usize> = BTreeSet::new();
        for p in &partners_a {
            if p.to == b {
                continue;
            }
            anc(p.to, memo);
            for core in self.blood(p.to, b, &memo[&p.to], &memo[&b]) {
                near.insert(p.to);
                if let Kind::Blood { up: 0, down, .. } = core.kind {
                    if own_blood.contains(&(0, down)) {
                        continue;
                    }
                }
                out.push(self.affine(a, b, Some(p), None, core));
            }
        }
        for q in &partners_b {
            if q.to == a {
                continue;
            }
            anc(q.to, memo);
            for core in self.blood(a, q.to, &memo[&a], &memo[&q.to]) {
                near.insert(q.to);
                if let Kind::Blood { up, down: 0, .. } = core.kind {
                    if own_blood.contains(&(up, 0)) {
                        continue;
                    }
                }
                out.push(self.affine(a, b, None, Some((q, q.to)), core));
            }
        }
        for p in &partners_a {
            for q in &partners_b {
                if p.to == b
                    || q.to == a
                    || p.to == q.to
                    || near.contains(&p.to)
                    || near.contains(&q.to)
                {
                    continue;
                }
                anc(p.to, memo);
                anc(q.to, memo);
                for core in self.blood(p.to, q.to, &memo[&p.to], &memo[&q.to]) {
                    out.push(self.affine(a, b, Some(p), Some((q, q.to)), core));
                }
            }
        }

        // A stepfather the records name as such is not said twice, once as
        // the one who raised A and once as the partner of A's mother.
        let raised = |up: bool| {
            out.iter().any(|r| {
                r.kind
                    == Kind::Care {
                        line: Line::Step,
                        up,
                    }
            })
        };
        let (by_b, by_a) = (raised(true), raised(false));
        out.retain(|r| match &r.kind {
            Kind::Affinity {
                core,
                a_side,
                b_side,
            } => match **core {
                Kind::Blood { up: 1, down: 0, .. } => !(by_b && *b_side && !*a_side),
                Kind::Blood { up: 0, down: 1, .. } => !(by_a && *a_side && !*b_side),
                _ => true,
            },
            _ => true,
        });

        if out.is_empty() {
            if let Some(path) = self.chain(a, b) {
                out.push(Relation {
                    kind: Kind::Chain,
                    path,
                    common: Vec::new(),
                    context: Context::default(),
                });
            }
        }
        out.sort_by_key(|r| r.rank());
        out
    }

    fn older(&self, x: usize, y: usize) -> Option<bool> {
        match (self.born[x], self.born[y]) {
            (Some(p), Some(q)) if p != q => Some(p < q),
            _ => None,
        }
    }

    fn care(&self, b: usize, line: Line, up: bool) -> Relation {
        Relation {
            kind: Kind::Care { line, up },
            path: vec![Step {
                person: self.ids[b].clone(),
                hop: if up {
                    Hop::Parent(line)
                } else {
                    Hop::Child(line)
                },
            }],
            common: Vec::new(),
            context: Context::default(),
        }
    }

    /// A relationship by marriage around `core`, the relationship by descent
    /// between A (or A's partner `pa`) and B (or B's partner `pb`).
    fn affine(
        &self,
        a: usize,
        b: usize,
        pa: Option<&Partner>,
        pb: Option<(&Partner, usize)>,
        core: Relation,
    ) -> Relation {
        let mut path = Vec::new();
        if let Some(p) = pa {
            path.push(Step {
                person: self.ids[p.to].clone(),
                hop: Hop::Partner {
                    married: p.married,
                    ended: p.ended,
                },
            });
        }
        path.extend(core.path.iter().cloned());
        if let Some((q, _)) = pb {
            path.push(Step {
                person: self.ids[b].clone(),
                hop: Hop::Partner {
                    married: q.married,
                    ended: q.ended,
                },
            });
        }
        let mut context = core.context.clone();
        // Who the word is about besides B: A's partner, or B's.
        context.a_next = pa.map(|p| self.sex[p.to]).or(context.a_next);
        context.b_next = pb.map(|(_, q)| self.sex[q]).or(context.b_next);
        context.b_elder = self.older(b, a);
        Relation {
            kind: Kind::Affinity {
                core: Box::new(core.kind),
                a_side: pa.is_some(),
                b_side: pb.is_some(),
            },
            path,
            common: core.common,
            context,
        }
    }

    /// The shortest chain of facts between A and B, by any tie at all.
    fn chain(&self, a: usize, b: usize) -> Option<Vec<Step>> {
        let mut prev: BTreeMap<usize, (usize, Hop)> = BTreeMap::new();
        let mut depth: BTreeMap<usize, usize> = BTreeMap::new();
        depth.insert(a, 0);
        let mut queue = VecDeque::from([a]);
        while let Some(x) = queue.pop_front() {
            if x == b {
                break;
            }
            let d = depth[&x];
            if d >= MAX_CHAIN {
                continue;
            }
            let mut next: Vec<(usize, Hop)> = Vec::new();
            next.extend(self.ups[x].iter().map(|u| (u.parent, Hop::Parent(u.line))));
            next.extend(self.downs[x].iter().map(|u| (u.parent, Hop::Child(u.line))));
            next.extend(self.partners[x].iter().map(|p| {
                (
                    p.to,
                    Hop::Partner {
                        married: p.married,
                        ended: p.ended,
                    },
                )
            }));
            next.extend(
                self.ties[x]
                    .iter()
                    .map(|t| (t.to, Hop::Rite(t.rite, t.role.other()))),
            );
            for (y, hop) in next {
                if depth.contains_key(&y) {
                    continue;
                }
                depth.insert(y, d + 1);
                prev.insert(y, (x, hop));
                queue.push_back(y);
            }
        }
        if !prev.contains_key(&b) {
            return None;
        }
        let mut steps = Vec::new();
        let mut at = b;
        while at != a {
            let (from, hop) = prev[&at].clone();
            steps.push(Step {
                person: self.ids[at].clone(),
                hop,
            });
            at = from;
        }
        steps.reverse();
        Some(steps)
    }

    /// Everyone the records connect to `a` whom this reader may see, each
    /// with every way they are related, the closest relatives first.
    pub fn relatives(&self, a: &str) -> Vec<(String, Vec<Relation>)> {
        let Some(&ai) = self.index.get(a) else {
            return Vec::new();
        };
        // Only the people connected to A at all are worth asking about.
        let mut reach: BTreeSet<usize> = BTreeSet::from([ai]);
        let mut queue = VecDeque::from([ai]);
        while let Some(x) = queue.pop_front() {
            let next = self.ups[x]
                .iter()
                .map(|u| u.parent)
                .chain(self.downs[x].iter().map(|u| u.parent))
                .chain(self.partners[x].iter().map(|p| p.to))
                .chain(self.ties[x].iter().map(|t| t.to));
            for y in next {
                if reach.insert(y) {
                    queue.push_back(y);
                }
            }
        }
        let mut memo: BTreeMap<usize, BTreeMap<usize, Vec<Way>>> = BTreeMap::new();
        let mut out: Vec<(String, Vec<Relation>)> = Vec::new();
        for &b in &reach {
            if b == ai || !self.known[b] || !self.lens.sees_person(&self.ids[b]) {
                continue;
            }
            let rels = self.between_idx(ai, b, &mut memo);
            // A chain longer than a few steps names nothing a reader can use
            // in a list of relatives: it stays for the question "how is X
            // related to Y", where it is the answer.
            let rels: Vec<Relation> = rels
                .into_iter()
                .filter(|r| r.kind != Kind::Chain || r.path.len() <= 4)
                .collect();
            if !rels.is_empty() {
                out.push((self.ids[b].clone(), rels));
            }
        }
        // Equally close: men first, as charts name a couple, then the elder.
        let order = |id: &str| {
            let i = self.index[id];
            let sex = match self.sex[i] {
                Sex::M => 0,
                Sex::U => 1,
                Sex::F => 2,
            };
            (sex, self.born[i].unwrap_or(i64::MAX), i)
        };
        out.sort_by(|x, y| {
            x.1[0]
                .rank()
                .cmp(&y.1[0].rank())
                .then_with(|| order(&x.0).cmp(&order(&y.0)))
        });
        out
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use serde_json::json;

    pub(crate) fn person(id: &str, sex: &str, born: &str) -> (String, Value) {
        (
            id.to_string(),
            json!({"id": id, "type": "person", "axgf_version": "1.1",
                   "identity": {"name": {"display": id}, "gender": {"value": sex},
                                "visibility": "public"},
                   "birth": {"date": {"value": born}}}),
        )
    }

    /// A fictional family over five generations: two couples at the top, a
    /// second marriage, an adoption, a stepfather, a godmother, a milk sister.
    pub(crate) fn family() -> Value {
        let mut ps = serde_json::Map::new();
        for (id, s, b) in [
            ("gg", "M", "1850"),
            ("ggw", "F", "1852"),
            ("g1", "M", "1880"),
            ("g1w", "F", "1882"),
            ("g2", "F", "1884"),
            ("g2h", "M", "1880"),
            ("g3", "M", "1890"),
            ("p1", "M", "1910"),
            ("p1w", "F", "1912"),
            ("p2", "F", "1913"),
            ("c1", "F", "1914"),
            ("c1h", "M", "1911"),
            ("h1", "M", "1920"),
            ("ego", "M", "1940"),
            ("sis", "F", "1942"),
            ("sish", "M", "1940"),
            ("half", "M", "1950"),
            ("p1w2", "F", "1925"),
            ("adop", "F", "1945"),
            ("step", "M", "1915"),
            ("cous", "F", "1938"),
            ("cous2", "M", "1965"),
            ("kid", "M", "1965"),
            ("kidw", "F", "1966"),
            ("gkid", "F", "1990"),
            ("neph", "M", "1968"),
            ("wifeb", "M", "1936"),
            ("wife", "F", "1941"),
            ("god", "F", "1915"),
            ("milk", "F", "1940"),
        ] {
            let (k, v) = person(id, s, b);
            ps.insert(k, v);
        }
        let fam = |id: &str, parents: &[&str], kids: &[(&str, &str)], kind: &str| {
            (
                id.to_string(),
                json!({"id": id, "type": "family", "axgf_version": "1.1",
                       "union": {"type": kind,
                                 "persons": parents.iter().map(|p| json!({"person_id": p, "role": "partner"})).collect::<Vec<_>>()},
                       "children": kids.iter().map(|(c, l)| {
                           if l.is_empty() { json!({"person_id": c}) } else { json!({"person_id": c, "lineage": l}) }
                       }).collect::<Vec<_>>()}),
            )
        };
        let mut fs = serde_json::Map::new();
        for (k, v) in [
            fam(
                "f0",
                &["gg", "ggw"],
                &[("g1", ""), ("g2", ""), ("g3", "")],
                "marriage",
            ),
            fam("f1", &["g1", "g1w"], &[("p1", ""), ("p2", "")], "marriage"),
            fam("f2", &["g2h", "g2"], &[("c1", "")], "marriage"),
            fam("f3", &["c1h", "c1"], &[("cous", "")], "marriage"),
            fam(
                "f4",
                &["p1", "p1w"],
                &[("ego", ""), ("sis", ""), ("adop", "adoptive")],
                "marriage",
            ),
            fam("f5", &["p1", "p1w2"], &[("half", "")], "marriage"),
            fam("f6", &["step", "p1w"], &[("ego", "step")], "cohabitation"),
            fam("f7", &["ego", "wife"], &[("kid", "")], "marriage"),
            fam("f8", &["kid", "kidw"], &[("gkid", "")], "marriage"),
            fam("f9", &["sish", "sis"], &[("neph", "")], "marriage"),
            fam("f10", &["g3"], &[("h1", "")], "unknown"),
            fam(
                "f11",
                &["wifeb_parent"],
                &[("wife", ""), ("wifeb", "")],
                "unknown",
            ),
            fam("f12", &["cous"], &[("cous2", "")], "unknown"),
        ] {
            fs.insert(k, v);
        }
        let mut ls = serde_json::Map::new();
        ls.insert(
            "l1".into(),
            json!({"id": "l1", "type": "link", "axgf_version": "1.1",
                   "from": {"entity_type": "person", "entity_id": "god"},
                   "to": {"entity_type": "person", "entity_id": "ego"},
                   "label": "marraine", "relation": "godparent", "category": "spiritual"}),
        );
        ls.insert(
            "l2".into(),
            json!({"id": "l2", "type": "link", "axgf_version": "1.1",
                   "from": {"entity_type": "person", "entity_id": "milk"},
                   "to": {"entity_type": "person", "entity_id": "ego"},
                   "label": "sœur de lait", "relation": "other", "category": "spiritual",
                   "extensions": {"axgf-cms:kinship/v1": {"rite": "milk-sibling"}}}),
        );
        json!({"persons": ps, "families": fs, "links": ls})
    }

    fn all() -> Lens {
        Lens::unrestricted()
    }

    fn kinds(k: &Kin, a: &str, b: &str) -> Vec<Kind> {
        k.between(a, b).into_iter().map(|r| r.kind).collect()
    }

    fn blood(up: u32, down: u32) -> Kind {
        Kind::Blood {
            up,
            down,
            half: false,
            adoptive: false,
        }
    }

    #[test]
    fn descent_reads_as_generations_up_and_down() {
        let f = family();
        let lens = all();
        let k = Kin::new(&f, &lens);
        assert_eq!(kinds(&k, "ego", "p1"), vec![blood(1, 0)]);
        assert_eq!(kinds(&k, "ego", "gg"), vec![blood(3, 0)]);
        assert_eq!(kinds(&k, "ego", "kid"), vec![blood(0, 1)]);
        assert_eq!(kinds(&k, "ego", "gkid"), vec![blood(0, 2)]);
        // A sister, an aunt, a great-uncle, a nephew.
        assert_eq!(kinds(&k, "ego", "sis"), vec![blood(1, 1)]);
        assert_eq!(kinds(&k, "ego", "p2"), vec![blood(2, 1)]);
        assert_eq!(kinds(&k, "ego", "g3"), vec![blood(3, 1)]);
        assert_eq!(kinds(&k, "ego", "neph"), vec![blood(1, 2)]);
        // Cousins: first cousin of ego's father, second cousin, and the second
        // cousin's son, once removed the younger way.
        assert_eq!(kinds(&k, "ego", "c1"), vec![blood(3, 2)]);
        assert_eq!(kinds(&k, "ego", "cous"), vec![blood(3, 3)]);
        assert_eq!(kinds(&k, "ego", "cous2"), vec![blood(3, 4)]);
        // h1 is the son of a great-uncle: a first cousin once removed.
        assert_eq!(kinds(&k, "ego", "h1"), vec![blood(3, 2)]);
    }

    #[test]
    fn a_shared_couple_is_full_and_a_shared_parent_is_half() {
        let f = family();
        let lens = all();
        let k = Kin::new(&f, &lens);
        let r = k.between("ego", "sis");
        assert_eq!(r.len(), 1);
        assert_eq!(r[0].common, vec!["p1".to_string(), "p1w".to_string()]);
        assert_eq!(
            kinds(&k, "ego", "half"),
            vec![Kind::Blood {
                up: 1,
                down: 1,
                half: true,
                adoptive: false
            }]
        );
        assert_eq!(k.between("ego", "half")[0].common, vec!["p1".to_string()]);
        // An adopted sister is a sister, by adoption.
        assert_eq!(
            kinds(&k, "ego", "adop"),
            vec![Kind::Blood {
                up: 1,
                down: 1,
                half: false,
                adoptive: true
            }]
        );
    }

    #[test]
    fn marriage_care_and_rite_are_relationships_too() {
        let f = family();
        let lens = all();
        let k = Kin::new(&f, &lens);
        assert_eq!(
            kinds(&k, "ego", "wife"),
            vec![Kind::Partner {
                married: true,
                ended: false
            }]
        );
        // The wife's brother, the sister's husband, the son's wife.
        let in_law = |up, down, a_side, b_side| Kind::Affinity {
            core: Box::new(blood(up, down)),
            a_side,
            b_side,
        };
        assert_eq!(kinds(&k, "ego", "wifeb"), vec![in_law(1, 1, true, false)]);
        assert_eq!(kinds(&k, "ego", "sish"), vec![in_law(1, 1, false, true)]);
        assert_eq!(kinds(&k, "ego", "kidw"), vec![in_law(0, 1, false, true)]);
        // The father's second wife is a stepmother; the mother's partner who
        // raised ego, a stepfather the records name as such.
        assert_eq!(kinds(&k, "ego", "p1w2"), vec![in_law(1, 0, false, true)]);
        assert!(kinds(&k, "ego", "step").contains(&Kind::Care {
            line: Line::Step,
            up: true
        }));
        let god = crate::rites::find("godparent").unwrap();
        assert_eq!(
            kinds(&k, "ego", "god"),
            vec![Kind::Rite {
                rite: god,
                role: Role::Sponsor
            }]
        );
        assert_eq!(
            kinds(&k, "god", "ego"),
            vec![Kind::Rite {
                rite: god,
                role: Role::Sponsored
            }]
        );
        let milk = crate::rites::find("milk-sibling").unwrap();
        assert_eq!(
            kinds(&k, "ego", "milk"),
            vec![Kind::Rite {
                rite: milk,
                role: Role::Mutual
            }]
        );
    }

    /// Godparents are most often aunts and uncles: the family comes first,
    /// out to a first cousin, and the rite after it.
    #[test]
    fn an_aunt_who_is_a_godmother_is_first_an_aunt() {
        let mut f = family();
        for (id, from) in [("l3", "p2"), ("l4", "cous")] {
            f["links"][id] = serde_json::json!({
                "id": id, "type": "link", "axgf_version": "1.1",
                "from": {"entity_type": "person", "entity_id": from},
                "to": {"entity_type": "person", "entity_id": "ego"},
                "label": "marraine", "relation": "godparent", "category": "spiritual"});
        }
        let lens = all();
        let k = Kin::new(&f, &lens);
        let aunt = kinds(&k, "ego", "p2");
        assert_eq!(aunt[0], blood(2, 1), "{aunt:?}");
        assert!(matches!(aunt[1], Kind::Rite { .. }), "{aunt:?}");
        // A second cousin is six steps away: the godmother comes first.
        let cousin = kinds(&k, "ego", "cous");
        assert!(matches!(cousin[0], Kind::Rite { .. }), "{cousin:?}");
        assert_eq!(cousin[1], blood(3, 3));
    }

    #[test]
    fn the_sides_and_ages_a_word_may_need_are_kept() {
        let f = family();
        let lens = all();
        let k = Kin::new(&f, &lens);
        // The aunt is on the father's side, and older than nobody: born
        // after him.
        let aunt = &k.between("ego", "p2")[0];
        assert_eq!(aunt.context.a_next, Some(Sex::M));
        assert_eq!(aunt.context.turn_elder, Some(false));
        // The second cousin comes down the grandfather's sister's line: not
        // one male line.
        assert!(!k.between("ego", "cous")[0].context.male_line);
        // The great-uncle's son comes down the great-uncle's.
        let h1 = &k.between("ego", "h1")[0];
        assert!(h1.context.male_line);
        // The sister is younger.
        assert_eq!(k.between("ego", "sis")[0].context.b_elder, Some(false));
    }

    #[test]
    fn every_relative_is_listed_closest_first() {
        let f = family();
        let lens = all();
        let k = Kin::new(&f, &lens);
        let rel = k.relatives("ego");
        let ids: Vec<&str> = rel.iter().map(|(id, _)| id.as_str()).collect();
        for id in [
            "p1", "p1w", "sis", "kid", "wife", "gkid", "neph", "cous", "cous2", "god", "milk",
            "wifeb", "sish", "kidw", "half", "adop", "gg",
        ] {
            assert!(ids.contains(&id), "{id} missing from {ids:?}");
        }
        // A parent before a second cousin.
        let at = |id: &str| ids.iter().position(|x| *x == id).unwrap();
        assert!(at("p1") < at("cous"));
        assert!(at("sis") < at("cous2"));
        // Nobody is their own relative, and a person outside the family is
        // not one either.
        assert!(!ids.contains(&"ego"));
    }

    #[test]
    fn people_the_records_connect_in_no_named_way_get_their_chain() {
        let f = family();
        let lens = all();
        let k = Kin::new(&f, &lens);
        // The godmother's... nothing, beyond ego. The milk sister and the
        // godmother are connected through ego only.
        let r = k.between("milk", "god");
        assert_eq!(r.len(), 1);
        assert_eq!(r[0].kind, Kind::Chain);
        assert_eq!(r[0].path.len(), 2);
        assert_eq!(r[0].path.last().unwrap().person, "god");
    }
}
