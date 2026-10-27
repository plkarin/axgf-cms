//! The family around one person as a classic chart (the operator's review of
//! 27 October 2026, docs/adr/0019).
//!
//! The tree page used to lay out the family around a person by the bundle's
//! own generation numbers ([`crate::tree`]): everyone of one generation on one
//! row, ordered to cross as little as possible. Around a real person those
//! rows mixed an ancestor's other partners with the lines of the person's own
//! children, and the connectors still crossed and ran over one another. The
//! operator asked for the classic family chart instead: ordered, its lines
//! clean, never crossing.
//!
//! A family around one person is two trees joined at that person, and that is
//! how it is drawn here:
//!
//! * **Ancestors**, below them — the oldest at the bottom, as everywhere in
//!   this application: under each person their father and mother side by
//!   side, each over their own parents. An ancestor's other partners stand
//!   beside them, on the outer side.
//! * **Descendants**, above them: the person and their partners on one row,
//!   each couple's children over the couple, each child with their own
//!   partners and children, and so on.
//!
//! Every person's subtree has a band of the page to itself and the bands of
//! two people never overlap, so a couple's line or a family's bus never
//! crosses another. A person the chart reaches a second time — cousins who
//! married, an ancestor on two lines — is drawn again where the line needs
//! them, marked as the same person, and not followed further.
//!
//! The chart reads the structure the band layout reads and returns the same
//! [`TreeLayout`], so the page, its filter and its highlight keep working; the
//! cards' own dress ([`dress`]) is shared by both.

use std::collections::{BTreeMap, BTreeSet};

use serde_json::Value;

use crate::tree::{Band, Card, Edge, FamilyEdges, TreeLayout};
use crate::view::{self, Confidence};

/// A card, folded: one line, the first given name and an initial.
pub const CARD_W: f64 = 140.0;
pub const CARD_H: f64 = 40.0;
/// Between brothers and sisters.
const GAP: f64 = 16.0;
/// Between partners: room for the couple's line and its union node.
const PARTNER_GAP: f64 = 36.0;
/// Between two families' children, and between the two sides of a couple's
/// ancestry.
const GROUP: f64 = 36.0;
/// From one generation's row to the next: room for a family's bus between
/// them, well clear of both rows.
const PITCH: f64 = CARD_H + 84.0;
const MARGIN: f64 = 28.0;
/// Room at the start of each row for its name ("Grandparents").
pub const GUTTER: f64 = 116.0;

/// One card in a block, its x (left edge) relative to the block.
#[derive(Debug, Clone)]
struct Slot {
    key: usize,
    id: String,
    x: f64,
    level: i64,
    dup: bool,
}

/// Where a line to a child starts, on the parents' row: a couple's line (its
/// middle) or one parent's card (its top).
#[derive(Debug, Clone, Copy)]
enum From {
    Card(usize),
    Union(usize, usize),
}

#[derive(Debug, Clone)]
enum Wire {
    /// Two partners on one row.
    Union {
        a: usize,
        b: usize,
        conf: Option<f64>,
    },
    /// A parentage: from the parents' row, up to the child's row.
    Parent {
        from: From,
        parent: String,
        child: usize,
        conf: Option<f64>,
    },
}

/// A laid-out subtree: its cards and lines, and the band of the page it
/// takes, every level included.
#[derive(Debug, Clone, Default)]
struct Block {
    slots: Vec<Slot>,
    wires: Vec<Wire>,
    left: f64,
    right: f64,
}

impl Block {
    fn row(slots: Vec<Slot>) -> Block {
        let left = slots.iter().map(|s| s.x).fold(f64::INFINITY, f64::min);
        let right = slots
            .iter()
            .map(|s| s.x + CARD_W)
            .fold(f64::NEG_INFINITY, f64::max);
        Block {
            slots,
            wires: Vec::new(),
            left,
            right,
        }
    }
    fn shift(&mut self, dx: f64) {
        for s in &mut self.slots {
            s.x += dx;
        }
        self.left += dx;
        self.right += dx;
    }
    fn absorb(&mut self, other: Block) {
        if other.slots.is_empty() {
            self.wires.extend(other.wires);
            return;
        }
        if self.slots.is_empty() {
            self.left = other.left;
            self.right = other.right;
        } else {
            self.left = self.left.min(other.left);
            self.right = self.right.max(other.right);
        }
        self.slots.extend(other.slots);
        self.wires.extend(other.wires);
    }
    fn x_of(&self, key: usize) -> f64 {
        self.slots
            .iter()
            .find(|s| s.key == key)
            .map(|s| s.x)
            .unwrap_or(0.0)
    }
    fn centre_of(&self, key: usize) -> f64 {
        self.x_of(key) + CARD_W / 2.0
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
            x: 0.0,
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

    /// Father first, then anyone unstated, then mother: the father on the
    /// left, as charts are drawn.
    fn side(&self, id: &str) -> u8 {
        match self
            .persons
            .get(id)
            .and_then(|p| p.get("identity"))
            .and_then(|i| i.get("gender"))
            .and_then(|g| g.get("value"))
            .and_then(Value::as_str)
        {
            Some("M") => 0,
            Some("F") => 2,
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

    /// `id` on `level`, their partners beside them, and above them each
    /// couple's children, as far as `self.down`. Returns the block and the
    /// key of `id`'s own card.
    fn descend(&mut self, id: &str, level: i64) -> (Block, usize) {
        let me = self.slot(id, level);
        let key = me.key;
        if me.dup {
            return (Block::row(vec![me]), key);
        }
        let fams = self.unions(id);
        // The row: the person, and the partner of each union — the first
        // union's on the left when there are several, the others on the
        // right, in order.
        let mut left: Vec<(usize, Slot)> = Vec::new();
        let mut right: Vec<(usize, Slot)> = Vec::new();
        let mut alone: Vec<usize> = Vec::new();
        for (i, &f) in fams.iter().enumerate() {
            match self.partner_in(f, id) {
                Some(q) => {
                    let s = self.slot(&q, level);
                    if i == 0 && fams.len() > 1 {
                        left.push((i, s));
                    } else {
                        right.push((i, s));
                    }
                }
                None => alone.push(i),
            }
        }
        let mut row: Vec<Slot> = Vec::new();
        let mut partner: Vec<Option<usize>> = vec![None; fams.len()];
        for (i, s) in left.into_iter().rev() {
            partner[i] = Some(s.key);
            row.push(s);
        }
        row.push(me);
        for (i, s) in right {
            partner[i] = Some(s.key);
            row.push(s);
        }
        for (j, s) in row.iter_mut().enumerate() {
            s.x = j as f64 * (CARD_W + PARTNER_GAP);
        }
        let mut block = Block::row(row);
        let me_x = block.x_of(key);
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

        // Each union's children, over the couple: in the order of the
        // couples' lines, left to right, each group centred over its own
        // couple where the groups before it allow.
        let mut groups: Vec<(f64, usize)> = Vec::new();
        for (i, p) in partner.iter().enumerate() {
            let ux = match p {
                Some(q) => union_x(me_x, block.x_of(*q)),
                None => {
                    let nth = alone.iter().position(|&a| a == i).unwrap_or(0);
                    me_x + CARD_W / 2.0 + 10.0 * nth as f64
                }
            };
            groups.push((ux, i));
        }
        groups.sort_by(|a, b| a.0.total_cmp(&b.0));

        let mut boundary = f64::NEG_INFINITY;
        if level < self.down {
            for &(ux0, i) in &groups {
                // A couple whose line comes after a wider group moves along,
                // so that its own line to its children never meets that
                // group's bus.
                let mut ux = ux0;
                if let Some(q) = partner[i] {
                    let qx = block.x_of(q);
                    let need = boundary + GROUP / 2.0;
                    if ux < need && qx > me_x {
                        let d = 2.0 * (need - ux);
                        for s in &mut block.slots {
                            if s.level == level && s.x >= qx {
                                s.x += d;
                            }
                        }
                        block.right += d;
                        ux += d / 2.0;
                    }
                }
                let from = match partner[i] {
                    Some(q) => From::Union(key, q),
                    None => From::Card(key),
                };
                let mut group = Block::default();
                let mut x = 0.0;
                let mut kids: Vec<usize> = Vec::new();
                for (c, conf) in self.children(fams[i]) {
                    let (mut b, ck) = self.descend(&c, level + 1);
                    b.shift(x - b.left);
                    x = b.right + GAP;
                    kids.push(ck);
                    group.absorb(b);
                    wires.push(Wire::Parent {
                        from,
                        parent: id.to_string(),
                        child: ck,
                        conf,
                    });
                }
                let (Some(&k0), Some(&k1)) = (kids.first(), kids.last()) else {
                    continue;
                };
                let mid = (group.centre_of(k0) + group.centre_of(k1)) / 2.0;
                let mut dx = ux - mid;
                if group.left + dx < boundary + GROUP {
                    dx = boundary + GROUP - group.left;
                }
                group.shift(dx);
                boundary = group.right;
                block.absorb(group);
            }
        }
        block.wires.extend(wires);
        (block, key)
    }

    /// The parents of `child` (their first family) on `level`, side by side,
    /// each over their own ancestry, as far as `-self.up`. Returns the block,
    /// the parents' card keys, and how sure the child's parentage is.
    fn parents_of(&mut self, child: &str, level: i64) -> Option<(Block, Vec<usize>, Option<f64>)> {
        if level < -self.up {
            return None;
        }
        let f = *self.child_in.get(child)?.first()?;
        let conf = self.families[f]
            .children
            .iter()
            .find(|(c, _)| c == child)
            .and_then(|(_, c)| *c);
        let mut parents = self.families[f].parents.clone();
        if parents.is_empty() {
            return None;
        }
        parents.sort_by_key(|p| self.side(p));
        let n = parents.len();
        let mut block = Block::default();
        let mut keys = Vec::new();
        let mut x = 0.0;
        for (i, p) in parents.iter().enumerate() {
            let outer = if n == 1 || i > 0 { 1 } else { -1 };
            let (mut b, k) = self.ancestor(p, level, f, outer);
            b.shift(x - b.left);
            x = b.right + GROUP;
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
        Some((block, keys, conf))
    }

    /// One ancestor on `level`: their card, their other partners beside
    /// them on the `outer` side, and their own parents below.
    fn ancestor(&mut self, id: &str, level: i64, via: usize, outer: i32) -> (Block, usize) {
        let me = self.slot(id, level);
        let key = me.key;
        if me.dup {
            return (Block::row(vec![me]), key);
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
        let mut row: Vec<Slot> = Vec::new();
        if outer < 0 {
            row.extend(beside.into_iter().rev());
            row.push(me);
        } else {
            row.push(me);
            row.extend(beside);
        }
        for (j, s) in row.iter_mut().enumerate() {
            s.x = j as f64 * (CARD_W + PARTNER_GAP);
        }
        let mut block = Block::row(row);
        for (q, conf) in unions {
            block.wires.push(Wire::Union { a: key, b: q, conf });
        }
        if let Some((mut pb, pkeys, conf)) = self.parents_of(id, level - 1) {
            let (Some(&p0), Some(&p1)) = (pkeys.first(), pkeys.last()) else {
                return (block, key);
            };
            let mid = (pb.centre_of(p0) + pb.centre_of(p1)) / 2.0;
            pb.shift(block.centre_of(key) - mid);
            for &pk in &pkeys {
                block.wires.push(Wire::Parent {
                    from: From::Card(pk),
                    parent: pb.id_of(pk),
                    child: key,
                    conf,
                });
            }
            block.absorb(pb);
        }
        (block, key)
    }
}

/// The middle of the line between two partners side by side.
fn union_x(a: f64, b: f64) -> f64 {
    if a <= b {
        (a + CARD_W + b) / 2.0
    } else {
        (b + CARD_W + a) / 2.0
    }
}

/// The family around `root`: `up` generations of ancestors under them, `down`
/// generations of descendants over them, everyone's partners beside them.
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
    if let Some((mut pb, pkeys, conf)) = c.parents_of(root, -1) {
        if let (Some(&p0), Some(&p1)) = (pkeys.first(), pkeys.last()) {
            let mid = (pb.centre_of(p0) + pb.centre_of(p1)) / 2.0;
            pb.shift(block.centre_of(rk) - mid);
            for &pk in &pkeys {
                block.wires.push(Wire::Parent {
                    from: From::Card(pk),
                    parent: pb.id_of(pk),
                    child: rk,
                    conf,
                });
            }
            block.absorb(pb);
        }
    }
    block.shift(GUTTER + MARGIN - block.left);

    let top = block.slots.iter().map(|s| s.level).max().unwrap_or(0);
    let bottom = block.slots.iter().map(|s| s.level).min().unwrap_or(0);
    let y_of = |level: i64| MARGIN + (top - level) as f64 * PITCH;
    let pos: BTreeMap<usize, (f64, i64, String)> = block
        .slots
        .iter()
        .map(|s| (s.key, (s.x, s.level, s.id.clone())))
        .collect();

    let mut bands: Vec<Band> = Vec::new();
    for level in (bottom..=top).rev() {
        let mut cards: Vec<Card> = block
            .slots
            .iter()
            .filter(|s| s.level == level)
            .map(|s| {
                let mut card = crate::tree::card_for(
                    &s.id,
                    persons.get(&s.id),
                    s.x,
                    y_of(level),
                    s.id == root && !s.dup,
                );
                card.w = CARD_W;
                card.dup = s.dup;
                card
            })
            .collect();
        cards.sort_by(|a, b| a.x.total_cmp(&b.x));
        bands.push(Band {
            label: String::new(),
            sublabel: String::new(),
            count: cards.len(),
            generation: None,
            cards,
            y: y_of(level),
            rows: 1,
            height: CARD_H,
            unplaced: false,
            relative: Some(level),
        });
    }

    let mut edges: Vec<Edge> = Vec::new();
    for w in &block.wires {
        match w {
            Wire::Union { a, b, conf } => {
                let (Some(pa), Some(pb)) = (pos.get(a), pos.get(b)) else {
                    continue;
                };
                let (x1, x2) = if pa.0 <= pb.0 {
                    (pa.0 + CARD_W, pb.0)
                } else {
                    (pb.0 + CARD_W, pa.0)
                };
                let y = y_of(pa.1) + CARD_H / 2.0;
                let c = Confidence::new(conf.unwrap_or(0.8));
                // The union's node, the two rings, where the line has room
                // for it.
                let node = (x2 - x1 >= PARTNER_GAP - 0.5).then_some([(x1 + x2) / 2.0, y]);
                edges.push(Edge {
                    kind: "spouse",
                    d: format!("M {x1:.1} {y:.1} L {x2:.1} {y:.1}"),
                    opacity: crate::tree::opacity_for(&c),
                    band: c.band,
                    pct: c.percent,
                    title: String::new(),
                    from: pa.2.clone(),
                    to: pb.2.clone(),
                    hue: None,
                    node,
                });
            }
            Wire::Parent {
                from,
                parent,
                child,
                conf,
            } => {
                let Some(pc) = pos.get(child) else { continue };
                let (sx, sy, level) = match from {
                    From::Card(k) => {
                        let Some(p) = pos.get(k) else { continue };
                        (p.0 + CARD_W / 2.0, y_of(p.1), p.1)
                    }
                    From::Union(a, b) => {
                        let (Some(pa), Some(pb)) = (pos.get(a), pos.get(b)) else {
                            continue;
                        };
                        (union_x(pa.0, pb.0), y_of(pa.1) + CARD_H / 2.0, pa.1)
                    }
                };
                let cx = pc.0 + CARD_W / 2.0;
                let cy = y_of(pc.1) + CARD_H;
                let bus = (cy + y_of(level)) / 2.0;
                let c = Confidence::new(conf.unwrap_or(0.8));
                edges.push(Edge {
                    kind: "parent",
                    d: format!("M {sx:.1} {sy:.1} V {bus:.1} H {cx:.1} V {cy:.1}"),
                    opacity: crate::tree::opacity_for(&c),
                    band: c.band,
                    pct: c.percent,
                    title: String::new(),
                    from: parent.clone(),
                    to: pc.2.clone(),
                    hue: None,
                    node: None,
                });
            }
        }
    }

    let generations = crate::tree::assign_generations(flat);
    let drawn = block.slots.iter().filter(|s| !s.dup).count();
    TreeLayout {
        width: block.right + MARGIN,
        height: y_of(bottom) + CARD_H + MARGIN,
        bands,
        edges,
        person_count: drawn,
        total_person_count: persons.len(),
        generation_count: (top - bottom + 1) as usize,
        unplaced_count: 0,
        truncated: generations.truncated,
        contradictions: generations.contradictions,
    }
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

    fn at(l: &TreeLayout, id: &str) -> (f64, f64) {
        l.bands
            .iter()
            .flat_map(|b| &b.cards)
            .find(|c| c.id == id && !c.dup)
            .map(|c| (c.x + CARD_W / 2.0, c.y))
            .unwrap()
    }

    #[test]
    fn the_chart_puts_ancestors_under_and_descendants_over_the_person() {
        let l = chart(&family(), "root", 3, 3);
        assert!(at(&l, "dad").1 > at(&l, "root").1 && at(&l, "gd1").1 > at(&l, "dad").1);
        assert!(at(&l, "k1").1 < at(&l, "root").1 && at(&l, "g1").1 < at(&l, "k1").1);
        assert_eq!(at(&l, "wife").1, at(&l, "root").1, "a partner beside");
        assert_eq!(
            at(&l, "gm1b").1,
            at(&l, "gd1").1,
            "an ancestor's other partner beside them"
        );
        assert_eq!(l.bands.first().unwrap().relative, Some(2));
        assert_eq!(l.bands.last().unwrap().relative, Some(-2));
        // A child stands between their father and mother.
        assert!(at(&l, "dad").0 < at(&l, "root").0 && at(&l, "root").0 < at(&l, "mum").0);
    }

    #[test]
    fn no_two_cards_overlap_and_no_two_lines_cross() {
        let l = chart(&family(), "root", 3, 3);
        for b in &l.bands {
            let mut xs: Vec<f64> = b.cards.iter().map(|c| c.x).collect();
            xs.sort_by(f64::total_cmp);
            for w in xs.windows(2) {
                assert!(
                    w[1] - w[0] >= CARD_W + GAP - 0.01,
                    "cards overlap on row {:?}",
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
