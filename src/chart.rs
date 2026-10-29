//! The family around one person as a classic chart (the operator's reviews of
//! 27 and 28 October 2026, docs/adr/0019).
//!
//! The tree page used to lay out the family around a person by the bundle's
//! own generation numbers ([`crate::tree`]): everyone of one generation on one
//! row, ordered to cross as little as possible. Around a real person those
//! rows mixed an ancestor's other partners with the lines of the person's own
//! children, and the connectors still crossed and ran over one another. The
//! operator asked for the classic family chart instead: ordered, its lines
//! clean, never crossing.
//!
//! A family around one person is two trees joined at that person, drawn
//! across the page, one column per generation, the oldest on the left:
//!
//! * **Ancestors**, to their left: beside each person their father and
//!   mother, the father above, each beside their own parents further left.
//!   An ancestor's other partners stand with them in their column, on the
//!   outer side.
//! * **Descendants**, to their right: the person and their partners one above
//!   the other in their column, each couple's children in the next column,
//!   the first level with the couple and the others below, each child with
//!   their own partners and children, and so on.
//!
//! A generation is a column, so the chart is as wide as the generations it
//! shows and never as wide as its largest family: it is read down the page,
//! never across a scrollbar that hides people (2026-10-28; until then the
//! generations were rows, and ten grandchildren side by side ran off the
//! screen). Every person's subtree has a band of the page to itself and the
//! bands of two people never overlap, so a couple's bracket or a family's
//! comb never crosses another. A person the chart reaches a second time —
//! cousins who married, an ancestor on two lines — is drawn again where the
//! line needs them, marked as the same person, and not followed further.
//!
//! Each line to a child says who the child is to the family: its colour the
//! child's side (a son, a daughter), its pattern how they belong to it when
//! it is not by birth (adoption, fostering, guardianship, a step-child).
//!
//! The chart reads the structure the band layout reads and returns the same
//! [`TreeLayout`], so the page, its filter and its highlight keep working; the
//! cards' own dress ([`dress`]) is shared by both.

use std::collections::{BTreeMap, BTreeSet};

use serde_json::Value;

use crate::tree::{Band, Card, Edge, FamilyEdges, TreeLayout};
use crate::view::{self, Confidence};

/// A card, folded: one line, the first given name and an initial — lower on
/// 2026-10-28 ("more compact still"), and as wide as the whole family's
/// cards: narrower, one name in fifteen of a real family was cut.
pub const CARD_W: f64 = 120.0;
pub const CARD_H: f64 = 30.0;
/// How far a card reaches down its column.
const SPAN: f64 = CARD_H;
/// Between brothers and sisters, one above the other.
const GAP: f64 = 8.0;
/// Between partners: their bracket stands beside them, so they stand close.
const PARTNER_GAP: f64 = 8.0;
/// Between two families' children, and between the two sides of a couple's
/// ancestry.
const GROUP: f64 = 20.0;
/// From one generation's column to the next: a couple's bracket and its
/// knot, the family's comb, and each child's own coloured branch.
const LINK_W: f64 = 40.0;
/// How far a couple's bracket stands out from their cards.
const BRACKET: f64 = 11.0;
const MARGIN: f64 = 10.0;
/// Room above the columns for each one's name ("Grandparents").
pub const HEAD: f64 = 56.0;

/// One card in a block, where it stands down its column relative to the
/// block.
#[derive(Debug, Clone)]
struct Slot {
    key: usize,
    id: String,
    at: f64,
    level: i64,
    dup: bool,
}

/// Where a line to a child leaves from, in the parents' column: a couple's
/// knot or one parent's card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum From {
    Card(usize),
    Union(usize, usize),
}

#[derive(Debug, Clone)]
enum Wire {
    /// Two partners in one column, joined by a bracket.
    Union {
        a: usize,
        b: usize,
        conf: Option<f64>,
    },
    /// A parentage: from the parents' column to the child's, the next one
    /// to the right.
    Parent {
        from: From,
        /// The parents' ids, space-separated: the highlight lights the line
        /// with either of them.
        parent: String,
        child: usize,
        conf: Option<f64>,
        lineage: Option<String>,
    },
}

/// A laid-out subtree: its cards and lines, and the band of the page it
/// takes down every column.
#[derive(Debug, Clone, Default)]
struct Block {
    slots: Vec<Slot>,
    wires: Vec<Wire>,
    lo: f64,
    hi: f64,
}

impl Block {
    fn line(slots: Vec<Slot>) -> Block {
        let lo = slots.iter().map(|s| s.at).fold(f64::INFINITY, f64::min);
        let hi = slots
            .iter()
            .map(|s| s.at + SPAN)
            .fold(f64::NEG_INFINITY, f64::max);
        Block {
            slots,
            wires: Vec::new(),
            lo,
            hi,
        }
    }
    fn shift(&mut self, d: f64) {
        for s in &mut self.slots {
            s.at += d;
        }
        self.lo += d;
        self.hi += d;
    }
    fn absorb(&mut self, other: Block) {
        if other.slots.is_empty() {
            self.wires.extend(other.wires);
            return;
        }
        if self.slots.is_empty() {
            self.lo = other.lo;
            self.hi = other.hi;
        } else {
            self.lo = self.lo.min(other.lo);
            self.hi = self.hi.max(other.hi);
        }
        self.slots.extend(other.slots);
        self.wires.extend(other.wires);
    }
    fn at_of(&self, key: usize) -> f64 {
        self.slots
            .iter()
            .find(|s| s.key == key)
            .map(|s| s.at)
            .unwrap_or(0.0)
    }
    fn centre_of(&self, key: usize) -> f64 {
        self.at_of(key) + SPAN / 2.0
    }
    fn id_of(&self, key: usize) -> String {
        self.slots
            .iter()
            .find(|s| s.key == key)
            .map(|s| s.id.clone())
            .unwrap_or_default()
    }
}

struct Chart<'a> {
    persons: &'a serde_json::Map<String, Value>,
    families: Vec<FamilyEdges>,
    parent_of: BTreeMap<String, Vec<usize>>,
    child_in: BTreeMap<String, Vec<usize>>,
    drawn: BTreeSet<String>,
    next: usize,
    up: i64,
    down: i64,
}

impl Chart<'_> {
    /// A card for `id`; a person already drawn is drawn again, marked.
    fn slot(&mut self, id: &str, level: i64) -> Slot {
        let dup = !self.drawn.insert(id.to_string());
        self.next += 1;
        Slot {
            key: self.next,
            id: id.to_string(),
            at: 0.0,
            level,
            dup,
        }
    }

    /// When a person was born, for ordering brothers and sisters: the
    /// earliest the record allows, the undated last, then by name.
    fn born(&self, id: &str) -> (i64, String) {
        let p = self.persons.get(id);
        let k = p
            .and_then(|p| {
                view::render_date_field(p.get("birth").unwrap_or(&Value::Null), "date").sort
            })
            .unwrap_or(i64::MAX);
        (k, p.map(view::person_display_name).unwrap_or_default())
    }

    /// Father first, then anyone unstated, then mother: the father above,
    /// as charts are drawn.
    fn side(&self, id: &str) -> u8 {
        match crate::tree::tone_of(self.persons.get(id)) {
            "a" => 0,
            "b" => 2,
            _ => 1,
        }
    }

    /// The unions a person is a parent in, the earliest children first.
    fn unions(&self, id: &str) -> Vec<usize> {
        let mut fs: Vec<usize> = self.parent_of.get(id).cloned().unwrap_or_default();
        fs.sort_by_key(|&f| {
            let first = self.families[f]
                .children
                .iter()
                .map(|(c, _)| self.born(c).0)
                .min()
                .unwrap_or(i64::MAX);
            (first, f)
        });
        fs.dedup();
        fs
    }

    fn partner_in(&self, f: usize, id: &str) -> Option<String> {
        self.families[f]
            .parents
            .iter()
            .find(|p| p.as_str() != id)
            .cloned()
    }

    fn children(&self, f: usize) -> Vec<(String, Option<f64>)> {
        let mut cs = self.families[f].children.clone();
        cs.sort_by_key(|(c, _)| self.born(c));
        cs
    }

    /// `id` in column `level`, their partners with them, and in the next
    /// column each couple's children, as far as `self.down`. Returns the
    /// block and the key of `id`'s own card.
    fn descend(&mut self, id: &str, level: i64) -> (Block, usize) {
        let me = self.slot(id, level);
        let key = me.key;
        if me.dup {
            return (Block::line(vec![me]), key);
        }
        let fams = self.unions(id);
        // The column: the person, and the partner of each union — the first
        // union's above when there are several, the others below, in order.
        let mut above: Vec<(usize, Slot)> = Vec::new();
        let mut below: Vec<(usize, Slot)> = Vec::new();
        let mut alone: Vec<usize> = Vec::new();
        for (i, &f) in fams.iter().enumerate() {
            match self.partner_in(f, id) {
                Some(q) => {
                    let s = self.slot(&q, level);
                    if i == 0 && fams.len() > 1 {
                        above.push((i, s));
                    } else {
                        below.push((i, s));
                    }
                }
                None => alone.push(i),
            }
        }
        let mut col: Vec<Slot> = Vec::new();
        let mut partner: Vec<Option<usize>> = vec![None; fams.len()];
        for (i, s) in above.into_iter().rev() {
            partner[i] = Some(s.key);
            col.push(s);
        }
        col.push(me);
        for (i, s) in below {
            partner[i] = Some(s.key);
            col.push(s);
        }
        for (j, s) in col.iter_mut().enumerate() {
            s.at = j as f64 * (SPAN + PARTNER_GAP);
        }
        let mut block = Block::line(col);
        let me_at = block.at_of(key);
        let mut wires: Vec<Wire> = Vec::new();
        for (i, p) in partner.iter().enumerate() {
            if let Some(q) = p {
                wires.push(Wire::Union {
                    a: key,
                    b: *q,
                    conf: self.families[fams[i]].union_confidence,
                });
            }
        }

        // Each union's children, beside the couple: in the order of the
        // couples' brackets, top to bottom, each family starting level with
        // its own couple where the families before it allow.
        let knot = |block: &Block, i: usize| match partner[i] {
            Some(q) => union_at(me_at, block.at_of(q)),
            None => {
                let nth = alone.iter().position(|&a| a == i).unwrap_or(0);
                me_at + SPAN / 2.0 + 6.0 * nth as f64
            }
        };
        let mut groups: Vec<(f64, usize)> = (0..fams.len()).map(|i| (knot(&block, i), i)).collect();
        groups.sort_by(|a, b| a.0.total_cmp(&b.0));

        let mut boundary = f64::NEG_INFINITY;
        if level < self.down {
            for &(_, i) in &groups {
                let (from, parents) = match partner[i] {
                    Some(q) => (From::Union(key, q), format!("{id} {}", block.id_of(q))),
                    None => (From::Card(key), id.to_string()),
                };
                let mut group = Block::default();
                let mut at = 0.0;
                let mut kids: Vec<usize> = Vec::new();
                for (c, conf) in self.children(fams[i]) {
                    let lineage = self.families[fams[i]].lineage.get(&c).cloned();
                    let (mut b, ck) = self.descend(&c, level + 1);
                    b.shift(at - b.lo);
                    at = b.hi + GAP;
                    kids.push(ck);
                    group.absorb(b);
                    wires.push(Wire::Parent {
                        from,
                        parent: parents.clone(),
                        child: ck,
                        conf,
                        lineage,
                    });
                }
                let Some(&k0) = kids.first() else {
                    continue;
                };
                // The first child level with the couple's knot and the others
                // below (2026-10-28): the chart is read down the page, and a
                // person stays at the top of their own family rather than in
                // the middle of it — with seventeen children, the person in
                // the middle of the page had sunk below the first screen.
                let mut d = knot(&block, i) - group.centre_of(k0);
                // A family after a taller one starts below it, so that its
                // comb never meets that family's...
                if group.lo + d < boundary + GROUP {
                    d = boundary + GROUP - group.lo;
                }
                group.shift(d);
                // ...and a later partner moves down with their children, level
                // with the first, the knot beside them: never twice as far, as
                // the middle of a bracket stretched past that family would put
                // them (`chart` sets each knot level with the first child).
                if let Some(q) = partner[i] {
                    let qa = block.at_of(q);
                    let m = group.centre_of(k0) - (qa + SPAN / 2.0);
                    if qa > me_at && m > 0.0 {
                        let mut hi = block.hi;
                        for s in block.slots.iter_mut() {
                            if s.level == level && s.at >= qa {
                                s.at += m;
                                hi = hi.max(s.at + SPAN);
                            }
                        }
                        block.hi = hi;
                    }
                }
                boundary = group.hi;
                block.absorb(group);
            }
        }
        block.wires.extend(wires);
        (block, key)
    }

    /// The parents of `child` (their first family) in column `level`, one
    /// above the other, each beside their own ancestry, as far as
    /// `-self.up`. Returns the block, the parents' card keys, how sure the
    /// child's parentage is, and how the child belongs to the family.
    #[allow(clippy::type_complexity)]
    fn parents_of(
        &mut self,
        child: &str,
        level: i64,
    ) -> Option<(Block, Vec<usize>, Option<f64>, Option<String>)> {
        if level < -self.up {
            return None;
        }
        let f = *self.child_in.get(child)?.first()?;
        let conf = self.families[f]
            .children
            .iter()
            .find(|(c, _)| c == child)
            .and_then(|(_, c)| *c);
        let lineage = self.families[f].lineage.get(child).cloned();
        let mut parents = self.families[f].parents.clone();
        if parents.is_empty() {
            return None;
        }
        parents.sort_by_key(|p| self.side(p));
        let n = parents.len();
        let mut block = Block::default();
        let mut keys = Vec::new();
        let mut at = 0.0;
        for (i, p) in parents.iter().enumerate() {
            let outer = if n == 1 || i > 0 { 1 } else { -1 };
            let (mut b, k) = self.ancestor(p, level, f, outer);
            b.shift(at - b.lo);
            at = b.hi + GROUP;
            keys.push(k);
            block.absorb(b);
        }
        for w in keys.windows(2) {
            block.wires.push(Wire::Union {
                a: w[0],
                b: w[1],
                conf: self.families[f].union_confidence,
            });
        }
        Some((block, keys, conf, lineage))
    }

    /// The line from `child`'s parents to `child`: from the couple's knot,
    /// or from a lone parent's card.
    fn to_child(
        pb: &Block,
        pkeys: &[usize],
        child: usize,
        conf: Option<f64>,
        lineage: Option<String>,
    ) -> Wire {
        let parent = pkeys
            .iter()
            .map(|&k| pb.id_of(k))
            .collect::<Vec<_>>()
            .join(" ");
        let from = match pkeys {
            [a, b, ..] => From::Union(*a, *b),
            [a] => From::Card(*a),
            [] => From::Card(child),
        };
        Wire::Parent {
            from,
            parent,
            child,
            conf,
            lineage,
        }
    }

    /// One ancestor in column `level`: their card, their other partners with
    /// them on the `outer` side, and their own parents to the left.
    fn ancestor(&mut self, id: &str, level: i64, via: usize, outer: i32) -> (Block, usize) {
        let me = self.slot(id, level);
        let key = me.key;
        if me.dup {
            return (Block::line(vec![me]), key);
        }
        let others: Vec<(String, Option<f64>)> = self
            .unions(id)
            .into_iter()
            .filter(|&f| f != via)
            .filter_map(|f| {
                self.partner_in(f, id)
                    .map(|q| (q, self.families[f].union_confidence))
            })
            .collect();
        let mut unions: Vec<(usize, Option<f64>)> = Vec::new();
        let mut beside: Vec<Slot> = Vec::new();
        for (q, conf) in &others {
            let s = self.slot(q, level);
            unions.push((s.key, *conf));
            beside.push(s);
        }
        let mut col: Vec<Slot> = Vec::new();
        if outer < 0 {
            col.extend(beside.into_iter().rev());
            col.push(me);
        } else {
            col.push(me);
            col.extend(beside);
        }
        for (j, s) in col.iter_mut().enumerate() {
            s.at = j as f64 * (SPAN + PARTNER_GAP);
        }
        let mut block = Block::line(col);
        for (q, conf) in unions {
            block.wires.push(Wire::Union { a: key, b: q, conf });
        }
        if let Some((mut pb, pkeys, conf, lineage)) = self.parents_of(id, level - 1) {
            let (Some(&p0), Some(&p1)) = (pkeys.first(), pkeys.last()) else {
                return (block, key);
            };
            let mid = parents_mid(&pb, p0, p1);
            pb.shift(block.centre_of(key) - mid);
            block
                .wires
                .push(Self::to_child(&pb, &pkeys, key, conf, lineage));
            block.absorb(pb);
        }
        (block, key)
    }
}

/// Down the column, the middle of the bracket between two partners.
fn union_at(a: f64, b: f64) -> f64 {
    (a.min(b) + a.max(b) + SPAN) / 2.0
}

/// Where a child stands level with their parents: the middle of the couple's
/// bracket, or of a lone parent's card.
fn parents_mid(pb: &Block, p0: usize, p1: usize) -> f64 {
    if p0 == p1 {
        pb.centre_of(p0)
    } else {
        union_at(pb.at_of(p0), pb.at_of(p1))
    }
}

/// The family around `root`: `up` generations of ancestors in the columns to
/// their left, `down` generations of descendants to their right, everyone's
/// partners in their own column.
pub fn chart(flat: &Value, root: &str, up: usize, down: usize) -> TreeLayout {
    let empty = serde_json::Map::new();
    let persons = flat
        .get("persons")
        .and_then(Value::as_object)
        .unwrap_or(&empty);
    let families = crate::tree::family_edges(flat);
    let mut parent_of: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    let mut child_in: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    for (i, f) in families.iter().enumerate() {
        for p in &f.parents {
            parent_of.entry(p.clone()).or_default().push(i);
        }
        for (c, _) in &f.children {
            child_in.entry(c.clone()).or_default().push(i);
        }
    }
    let mut c = Chart {
        persons,
        families,
        parent_of,
        child_in,
        drawn: BTreeSet::new(),
        next: 0,
        up: up as i64,
        down: down as i64,
    };
    let (mut block, rk) = c.descend(root, 0);
    if let Some((mut pb, pkeys, conf, lineage)) = c.parents_of(root, -1) {
        if let (Some(&p0), Some(&p1)) = (pkeys.first(), pkeys.last()) {
            let mid = parents_mid(&pb, p0, p1);
            pb.shift(block.centre_of(rk) - mid);
            block
                .wires
                .push(Chart::to_child(&pb, &pkeys, rk, conf, lineage));
            block.absorb(pb);
        }
    }
    block.shift(-block.lo);

    let newest = block.slots.iter().map(|s| s.level).max().unwrap_or(0);
    let oldest = block.slots.iter().map(|s| s.level).min().unwrap_or(0);
    let x_of = |level: i64| MARGIN + (level - oldest) as f64 * (CARD_W + LINK_W);
    let top = HEAD + MARGIN;
    let body = block.hi - block.lo;
    let pos: BTreeMap<usize, (f64, f64, i64, String)> = block
        .slots
        .iter()
        .map(|s| (s.key, (x_of(s.level), top + s.at, s.level, s.id.clone())))
        .collect();

    // The columns, the oldest first, as the page reads them.
    let mut bands: Vec<Band> = Vec::new();
    for level in oldest..=newest {
        let mut cards: Vec<Card> = block
            .slots
            .iter()
            .filter(|s| s.level == level)
            .map(|s| {
                let mut card = crate::tree::card_for(
                    &s.id,
                    persons.get(&s.id),
                    x_of(level),
                    top + s.at,
                    s.id == root && !s.dup,
                );
                card.w = CARD_W;
                card.dup = s.dup;
                card
            })
            .collect();
        cards.sort_by(|a, b| a.y.total_cmp(&b.y));
        bands.push(Band {
            label: String::new(),
            sublabel: String::new(),
            count: cards.len(),
            generation: None,
            cards,
            y: HEAD,
            rows: 1,
            height: body + 2.0 * MARGIN,
            unplaced: false,
            relative: Some(level),
            x: x_of(level),
            w: CARD_W,
        });
    }

    let mut edges: Vec<Edge> = Vec::new();
    let edge = |kind: &'static str, d: String, c: &Confidence, from: String, to: String| Edge {
        kind,
        d,
        opacity: line_opacity(c),
        band: c.band,
        pct: c.percent,
        title: String::new(),
        from,
        to,
        hue: None,
        tone: None,
        line: None,
        kids: None,
        node: None,
    };
    // A couple's bracket, beside their cards, and the knot on it.
    let bracket = |a: &(f64, f64, i64, String), b: &(f64, f64, i64, String)| {
        let (y1, y2) = (a.1.min(b.1) + CARD_H / 2.0, a.1.max(b.1) + CARD_H / 2.0);
        let x0 = a.0 + CARD_W;
        (x0, x0 + BRACKET, y1, y2)
    };
    let mut unions: Vec<(usize, usize, Option<f64>)> = Vec::new();
    let mut combs: BTreeMap<From, Vec<Kid>> = BTreeMap::new();
    for w in &block.wires {
        match w {
            Wire::Union { a, b, conf } => unions.push((*a, *b, *conf)),
            Wire::Parent {
                from,
                parent,
                child,
                conf,
                lineage,
            } => combs.entry(*from).or_default().push(Kid {
                key: *child,
                parents: parent.clone(),
                conf: *conf,
                lineage: lineage.clone(),
            }),
        }
    }
    // A couple's knot stands on their bracket level with their first child:
    // in its middle above the ancestors, whose child stands there, and lower
    // for a later couple whose family starts below a taller one.
    let knot = |a: usize, b: usize| {
        let (x0, xb, y1, y2) = bracket(pos.get(&a)?, pos.get(&b)?);
        let first = combs
            .get(&From::Union(a, b))
            .into_iter()
            .flatten()
            .filter_map(|kid| pos.get(&kid.key))
            .map(|p| p.1 + CARD_H / 2.0)
            .reduce(f64::min);
        let at = first.map_or((y1 + y2) / 2.0, |y| y.clamp(y1, y2));
        Some((x0, xb, y1, y2, at))
    };
    for &(a, b, conf) in &unions {
        let Some((x0, xb, y1, y2, at)) = knot(a, b) else {
            continue;
        };
        let c = Confidence::new(conf.unwrap_or(0.8));
        let mut e = edge(
            "spouse",
            format!("M {x0:.1} {y1:.1} H {xb:.1} V {y2:.1} H {x0:.1}"),
            &c,
            pos[&a].3.clone(),
            pos[&b].3.clone(),
        );
        e.node = Some([xb, at]);
        edges.push(e);
    }
    // Each family's lines: from the couple's knot (or a lone parent's card)
    // to a comb, and from the comb to each child, in the child's own colour
    // and pattern. An only child's line is theirs all the way.
    for (from, kids) in &combs {
        let source = match from {
            From::Card(k) => pos.get(k).map(|p| (p.0 + CARD_W, p.1 + CARD_H / 2.0)),
            From::Union(a, b) => knot(*a, *b).map(|(_, xb, _, _, at)| (xb, at)),
        };
        let Some((sx, sy)) = source else { continue };
        let ends: Vec<End> = kids
            .iter()
            .filter_map(|kid| {
                pos.get(&kid.key).map(|p| End {
                    x: p.0,
                    y: p.1 + CARD_H / 2.0,
                    id: p.3.clone(),
                    kid,
                })
            })
            .collect();
        let Some(cx) = ends.first().map(|e| e.x) else {
            continue;
        };
        let bx = sx + (cx - sx) * 0.45;
        let branch = |d: String, end: &End| {
            let c = Confidence::new(end.kid.conf.unwrap_or(0.8));
            let mut e = edge("parent", d, &c, end.kid.parents.clone(), end.id.clone());
            e.tone = Some(crate::tree::tone_of(persons.get(&end.id)));
            e.line = end.kid.lineage.clone();
            e
        };
        if let [only] = ends.as_slice() {
            let cy = only.y;
            edges.push(branch(
                format!("M {sx:.1} {sy:.1} H {bx:.1} V {cy:.1} H {cx:.1}"),
                only,
            ));
            continue;
        }
        let y0 = ends.iter().map(|e| e.y).fold(sy, f64::min);
        let y1 = ends.iter().map(|e| e.y).fold(sy, f64::max);
        let mut stem = edge(
            "stem",
            format!("M {sx:.1} {sy:.1} H {bx:.1} V {y0:.1} V {y1:.1}"),
            &Confidence::new(1.0),
            ends[0].kid.parents.clone(),
            String::new(),
        );
        stem.kids = Some(
            ends.iter()
                .map(|e| e.id.clone())
                .collect::<Vec<_>>()
                .join(" "),
        );
        edges.push(stem);
        for e in &ends {
            let cy = e.y;
            edges.push(branch(format!("M {bx:.1} {cy:.1} H {cx:.1}"), e));
        }
    }

    let generations = crate::tree::assign_generations(flat);
    let drawn = block.slots.iter().filter(|s| !s.dup).count();
    TreeLayout {
        // The newest column's couples keep their brackets and knots, beside
        // their cards, inside the drawing.
        width: x_of(newest) + CARD_W + LINK_W / 2.0 + MARGIN,
        height: top + body + MARGIN,
        bands,
        edges,
        person_count: drawn,
        total_person_count: persons.len(),
        generation_count: (newest - oldest + 1) as usize,
        unplaced_count: 0,
        truncated: generations.truncated,
        contradictions: generations.contradictions,
    }
}

/// A child on a family's comb: their card, their parents' ids, how sure the
/// parentage is, and how they belong to the family.
struct Kid {
    key: usize,
    parents: String,
    conf: Option<f64>,
    lineage: Option<String>,
}

/// Where a child's branch ends: their card's start across the page and its
/// middle down it.
struct End<'a> {
    x: f64,
    y: f64,
    id: String,
    kid: &'a Kid,
}

/// A line's opacity in the chart: as sure as the record is, never so faint
/// that its colour cannot be told.
fn line_opacity(c: &Confidence) -> f64 {
    0.5 + 0.5 * c.value
}

/// The cards' own dress, for the chart and the whole family alike (the
/// operator's review, 2026-10-27): the folded name ("Marie D."), the dates in
/// words, and the portrait the record would show, among the pictures of
/// theirs this reader may see. A person this reader may not see gets none of
/// it; [`crate::tree::redact_in`] blanks the rest.
pub fn dress(layout: &mut TreeLayout, flat: &Value, lens: &crate::access::Lens, lang: &str) {
    let empty = serde_json::Map::new();
    let persons = flat
        .get("persons")
        .and_then(Value::as_object)
        .unwrap_or(&empty);
    let portraits = crate::person::Portraits::new(flat, lens);
    for card in layout.bands.iter_mut().flat_map(|b| b.cards.iter_mut()) {
        if !card.known || !lens.sees_person(&card.id) {
            continue;
        }
        let label = crate::access::river_label(flat, lens, &card.id);
        card.short = short_name(&label.given, &label.surname, &card.name);
        card.initials = crate::person::initials_of(&card.name);
        if let Some(p) = persons.get(&card.id) {
            let text = |key: &str| {
                let d =
                    view::render_date_field_in(p.get(key).unwrap_or(&Value::Null), "date", lang);
                if d.kind == "unknown" || d.short.is_empty() {
                    String::new()
                } else {
                    d.text
                }
            };
            card.birth_text = text("birth");
            if card.death != "—" {
                card.death_text = text("death");
            }
        }
        if let Some((src, focal)) = portraits.of(&card.id) {
            card.portrait = Some(src);
            card.focal = focal;
        }
    }
}

/// "Marie D.": the first given name and the family name's initial. A record
/// that does not break its name into parts gives its first and last words;
/// a single word stays whole.
pub fn short_name(given: &str, surname: &str, display: &str) -> String {
    let first = |s: &str| s.split_whitespace().next().unwrap_or("").to_string();
    let initial = |s: &str| {
        s.chars()
            .find(|c| c.is_alphabetic())
            .map(|c| format!("{c}."))
    };
    let given = given.trim();
    let surname = surname.trim();
    if !surname.is_empty() && !given.is_empty() && given != display.trim() {
        return match initial(surname) {
            Some(i) => format!("{} {i}", first(given)),
            None => first(given),
        };
    }
    let words: Vec<&str> = display.split_whitespace().collect();
    match words.as_slice() {
        [] => String::new(),
        [one] => (*one).to_string(),
        [head, .., last] => match initial(last) {
            Some(i) => format!("{head} {i}"),
            None => (*head).to_string(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn person(id: &str, given: &str, surname: &str, g: &str, born: &str) -> (String, Value) {
        (
            id.to_string(),
            json!({"id": id, "type": "person", "axgf_version": "1.0",
                   "identity": {"name": {"display": format!("{given} {surname}"),
                                         "components": {"given": given, "surname": surname}},
                                "gender": {"value": g}},
                   "birth": {"date": {"value": born}}}),
        )
    }

    fn bundle(persons: Vec<(String, Value)>, families: &[(&[&str], &[&str])]) -> Value {
        let ps: serde_json::Map<String, Value> = persons.into_iter().collect();
        let mut fs = serde_json::Map::new();
        for (i, (parents, children)) in families.iter().enumerate() {
            let id = format!("f{i}");
            fs.insert(
                id.clone(),
                json!({"id": id, "type": "family", "axgf_version": "1.0",
                       "union": {"persons": parents.iter().map(|p| json!({"person_id": p})).collect::<Vec<_>>()},
                       "children": children.iter().map(|c| json!({"person_id": c})).collect::<Vec<_>>()}),
            );
        }
        json!({"persons": ps, "families": fs})
    }

    /// A fictional family: a second marriage, a grandparent's second partner,
    /// three generations each way.
    fn family() -> Value {
        let mut ps = Vec::new();
        for (id, g, s, sex, b) in [
            ("root", "Lucien", "Delaunay", "M", "1924"),
            ("wife", "Zofia", "Zajac", "F", "1927"),
            ("wife2", "Ines", "Moreau", "F", "1930"),
            ("dad", "Jean", "Delaunay", "M", "1893"),
            ("mum", "Mathilde", "Hamon", "F", "1892"),
            ("gd1", "Pierre", "Delaunay", "M", "1860"),
            ("gm1", "Anne", "Le Gall", "F", "1862"),
            ("gm1b", "Odile", "Bris", "F", "1870"),
            ("gd2", "Yves", "Hamon", "M", "1858"),
            ("gm2", "Marie", "Guillou", "F", "1861"),
            ("k1", "Helene", "Delaunay", "F", "1952"),
            ("k2", "Michel", "Delaunay", "M", "1955"),
            ("k3", "Elise", "Delaunay", "F", "1958"),
            ("k4", "Anna", "Delaunay", "F", "1960"),
            ("k1h", "Pierre", "Le Bris", "M", "1950"),
            ("g1", "Odette", "Le Bris", "F", "1981"),
            ("g2", "Paul", "Le Bris", "M", "1983"),
            ("k2w", "Claire", "Roux", "F", "1957"),
            ("g3", "Luc", "Delaunay", "M", "1985"),
        ] {
            ps.push(person(id, g, s, sex, b));
        }
        bundle(
            ps,
            &[
                (&["root", "wife"], &["k1", "k2", "k3"]),
                (&["root", "wife2"], &["k4"]),
                (&["dad", "mum"], &["root"]),
                (&["gd1", "gm1"], &["dad"]),
                (&["gd1", "gm1b"], &[]),
                (&["gd2", "gm2"], &["mum"]),
                (&["k1h", "k1"], &["g1", "g2"]),
                (&["k2", "k2w"], &["g3"]),
            ],
        )
    }

    /// A straight run of a line: its two ends and the card it leaves from.
    type Point = (f64, f64);
    type Segment = (Point, Point, String);

    fn segments(layout: &TreeLayout) -> Vec<Segment> {
        let mut out = Vec::new();
        for e in &layout.edges {
            let mut pts: Vec<Point> = Vec::new();
            let mut it = e.d.split_whitespace();
            let (mut x, mut y) = (0.0, 0.0);
            while let Some(cmd) = it.next() {
                match cmd {
                    "M" | "L" => {
                        x = it.next().unwrap().parse().unwrap();
                        y = it.next().unwrap().parse().unwrap();
                    }
                    "H" => x = it.next().unwrap().parse().unwrap(),
                    "V" => y = it.next().unwrap().parse().unwrap(),
                    _ => panic!("unexpected {cmd}"),
                }
                pts.push((x, y));
            }
            for w in pts.windows(2) {
                if w[0] != w[1] {
                    out.push((w[0], w[1], e.from.clone()));
                }
            }
        }
        out
    }

    /// A card's column (its left edge) and its middle down the page.
    fn at(l: &TreeLayout, id: &str) -> (f64, f64) {
        l.bands
            .iter()
            .flat_map(|b| &b.cards)
            .find(|c| c.id == id && !c.dup)
            .map(|c| (c.x, c.y + CARD_H / 2.0))
            .unwrap()
    }

    #[test]
    fn the_chart_puts_ancestors_left_and_descendants_right() {
        let l = chart(&family(), "root", 3, 3);
        assert!(at(&l, "dad").0 < at(&l, "root").0 && at(&l, "gd1").0 < at(&l, "dad").0);
        assert!(at(&l, "k1").0 > at(&l, "root").0 && at(&l, "g1").0 > at(&l, "k1").0);
        assert_eq!(
            at(&l, "wife").0,
            at(&l, "root").0,
            "a partner in their column"
        );
        assert_eq!(
            at(&l, "gm1b").0,
            at(&l, "gd1").0,
            "an ancestor's other partner in their column"
        );
        assert_eq!(l.bands.first().unwrap().relative, Some(-2));
        assert_eq!(l.bands.last().unwrap().relative, Some(2));
        // A child stands level with their parents' knot, the father above.
        let (dad, mum, root) = (at(&l, "dad").1, at(&l, "mum").1, at(&l, "root").1);
        assert!(dad < root && root < mum);
        assert!((root - (dad + mum) / 2.0).abs() < 0.5);
        // A family starts level with its couple's knot, the person at the top
        // of their family: the first child of the first marriage is level
        // with the knot between the wife (above) and the person.
        let (wife, k1) = (at(&l, "wife").1, at(&l, "k1").1);
        assert!(wife < root);
        assert!((k1 - (wife + root) / 2.0).abs() < 0.5);
        assert!(at(&l, "k2").1 > k1 && at(&l, "k3").1 > at(&l, "k2").1);
        // The second wife's family starts below the first's, and she stands
        // beside it, level with her child and their knot: not twice as far
        // down, below her own family.
        let (wife2, k4) = (at(&l, "wife2").1, at(&l, "k4").1);
        assert!(k4 > at(&l, "g3").1 && k4 > at(&l, "k3").1);
        assert!((wife2 - k4).abs() < 0.5, "wife2 {wife2}, k4 {k4}");
        let knot = l
            .edges
            .iter()
            .find(|e| e.kind == "spouse" && e.to == "wife2")
            .and_then(|e| e.node)
            .unwrap();
        assert!((knot[1] - k4).abs() < 0.5);
        assert!(l.height < k4 + CARD_H / 2.0 + MARGIN + 0.5);
    }

    #[test]
    fn no_two_cards_overlap_and_no_two_lines_cross() {
        let l = chart(&family(), "root", 3, 3);
        for b in &l.bands {
            let mut ys: Vec<f64> = b.cards.iter().map(|c| c.y).collect();
            ys.sort_by(f64::total_cmp);
            for w in ys.windows(2) {
                assert!(
                    w[1] - w[0] >= CARD_H + GAP.min(PARTNER_GAP) - 0.01,
                    "cards overlap in column {:?}",
                    b.relative
                );
            }
        }
        let segs = segments(&l);
        for (i, a) in segs.iter().enumerate() {
            for b in segs.iter().skip(i + 1) {
                let (h, v) = if a.0 .1 == a.1 .1 && b.0 .0 == b.1 .0 {
                    (a, b)
                } else if b.0 .1 == b.1 .1 && a.0 .0 == a.1 .0 {
                    (b, a)
                } else {
                    continue;
                };
                let (hx0, hx1) = (h.0 .0.min(h.1 .0), h.0 .0.max(h.1 .0));
                let hy = h.0 .1;
                let vx = v.0 .0;
                let (vy0, vy1) = (v.0 .1.min(v.1 .1), v.0 .1.max(v.1 .1));
                // Strictly inside both is a crossing; meeting at an end, a join.
                let crosses = vx > hx0 + 0.5 && vx < hx1 - 0.5 && hy > vy0 + 0.5 && hy < vy1 - 0.5;
                assert!(!crosses, "a line of {} crosses a line of {}", h.2, v.2);
            }
        }
    }

    /// The operator's review of 2026-10-28: the chart is read down the page.
    /// A family of twelve children takes more of the page's height, never
    /// more of its width.
    #[test]
    fn the_chart_is_as_wide_as_its_generations_not_its_largest_family() {
        let small = chart(&family(), "root", 3, 3);
        let mut b = family();
        let mut kids = vec![
            json!({"person_id": "k1"}),
            json!({"person_id": "k2"}),
            json!({"person_id": "k3"}),
        ];
        for i in 0..9 {
            let id = format!("x{i}");
            b["persons"][&id] = person(&id, "Jules", "Delaunay", "M", "1965").1;
            kids.push(json!({"person_id": id}));
        }
        b["families"]["f0"]["children"] = json!(kids);
        let big = chart(&b, "root", 3, 3);
        assert_eq!(big.width, small.width);
        assert!(big.height > small.height);
    }

    #[test]
    fn a_line_to_a_child_says_their_side_and_how_they_belong() {
        let mut b = family();
        b["families"]["f0"]["children"][0]["lineage"] = json!("adoptive");
        let l = chart(&b, "root", 3, 3);
        let to = |l: &TreeLayout, id: &str| {
            l.edges
                .iter()
                .find(|e| e.kind == "parent" && e.to == id)
                .cloned()
                .unwrap()
        };
        assert_eq!(to(&l, "k1").tone, Some("b"));
        assert_eq!(to(&l, "k1").line.as_deref(), Some("adoptive"));
        assert_eq!(to(&l, "k2").tone, Some("a"));
        assert_eq!(to(&l, "k2").line, None);
        // Three children share one comb, which names both parents and every
        // child, so the highlight finds it from any of them.
        let stem = l
            .edges
            .iter()
            .find(|e| {
                e.kind == "stem"
                    && e.kids
                        .as_deref()
                        .is_some_and(|k| k.split(' ').any(|c| c == "k1"))
            })
            .unwrap();
        assert!(
            stem.from.split(' ').any(|p| p == "root") && stem.from.split(' ').any(|p| p == "wife")
        );
        // A hidden child's line keeps its place and says no more than their
        // card: neither a daughter's colour nor an adoption's pattern.
        let visible: BTreeSet<String> = b["persons"]
            .as_object()
            .unwrap()
            .keys()
            .filter(|id| id.as_str() != "k1")
            .cloned()
            .collect();
        let mut hidden = l.clone();
        crate::tree::redact_in(&mut hidden, Some(&visible), "en");
        assert_eq!(to(&hidden, "k1").tone, Some("u"));
        assert_eq!(to(&hidden, "k1").line, None);
        assert_eq!(to(&hidden, "k2").tone, Some("a"));
    }

    #[test]
    fn a_person_reached_twice_is_drawn_again_and_not_followed() {
        // The root's daughter married her nephew: drawn as her partner, and
        // again as a descendant.
        let mut b = family();
        b["families"]["f6"]["union"]["persons"] = json!([{"person_id": "g3"}, {"person_id": "k1"}]);
        let l = chart(&b, "root", 3, 3);
        let cards: Vec<&Card> = l.bands.iter().flat_map(|b| &b.cards).collect();
        assert!(cards.iter().any(|c| c.dup && c.id == "g3"));
        assert_eq!(l.person_count, cards.iter().filter(|c| !c.dup).count());
    }

    #[test]
    fn a_short_name_is_a_first_name_and_an_initial() {
        assert_eq!(
            short_name("Marie Louise", "Delaunay", "Marie Louise Delaunay"),
            "Marie D."
        );
        assert_eq!(short_name("Jan Nowak", "", "Jan Nowak"), "Jan N.");
        assert_eq!(short_name("", "", "Dobromir"), "Dobromir");
        assert_eq!(
            short_name("Anna", "Ó Súilleabháin", "Anna Ó Súilleabháin"),
            "Anna Ó."
        );
    }
}
