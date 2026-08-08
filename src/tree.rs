//! Layout for the global tree view.
//!
//! This module answers two questions and nothing else: *which row does each
//! person belong in*, and *where on the canvas does their card sit*. It reads
//! only the structure `axgf-rs` produced — it does not interpret genealogy, it
//! arranges it.
//!
//! # Reading direction
//!
//! The page reads bottom-to-top: the oldest generation sits at the bottom and
//! the user scrolls upward through time. Generation 0 is the oldest, so the
//! rows are emitted highest-generation-first in the DOM and normal document
//! flow puts the youngest at the top.
//!
//! One consequence is worth stating, because it inverts the usual convention:
//! a parent is *below* its child on screen. Connectors therefore run from the
//! parent's **top** edge to the child's **bottom** edge. Drawing them from the
//! parent's bottom edge — the phrasing that fits a conventional top-down tree —
//! would send every line the wrong way around the cards.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use serde::Serialize;
use serde_json::Value;

use crate::view::{self, Confidence};

// Card geometry. Positions are computed server-side because the SVG
// connectors must line up with the cards without a layout pass in the browser.
const CARD_W: f64 = 132.0;
const CARD_H: f64 = 58.0;
const H_GAP: f64 = 14.0;
const V_GAP: f64 = 78.0;
const ROW_PITCH: f64 = CARD_H + V_GAP;
const MARGIN: f64 = 24.0;

/// Where every person sits, by generation.
#[derive(Debug, Default, PartialEq)]
pub struct Generations {
    /// Generation number per placed person. 0 is the oldest.
    pub gen: BTreeMap<String, i64>,
    /// Persons who appear in no family at all — neither as a partner nor as a
    /// child. They are shown in their own band rather than dropped.
    pub unplaced: Vec<String>,
    /// True when parentage cycles forced the relaxation pass to stop early.
    pub truncated: bool,
}

/// One family reduced to the two lists the layout cares about.
struct FamilyEdges {
    parents: Vec<String>,
    children: Vec<(String, Option<f64>)>,
    union_confidence: Option<f64>,
}

/// Read the families of a flat bundle into parent/child lists.
fn family_edges(flat: &Value) -> Vec<FamilyEdges> {
    let Some(families) = flat.get("families").and_then(Value::as_object) else {
        return Vec::new();
    };
    families
        .values()
        .map(|f| {
            let parents = f
                .get("union")
                .and_then(|u| u.get("persons"))
                .and_then(Value::as_array)
                .map(|ps| {
                    ps.iter()
                        .filter_map(|p| p.get("person_id").and_then(Value::as_str))
                        .map(str::to_string)
                        .collect()
                })
                .unwrap_or_default();
            let children = f
                .get("children")
                .and_then(Value::as_array)
                .map(|cs| {
                    cs.iter()
                        .filter_map(|c| {
                            let id = c.get("person_id").and_then(Value::as_str)?;
                            let conf = c.get("confidence").and_then(Value::as_f64);
                            Some((id.to_string(), conf))
                        })
                        .collect()
                })
                .unwrap_or_default();
            let union_confidence = f
                .get("union")
                .and_then(|u| u.get("confidence"))
                .and_then(Value::as_f64);
            FamilyEdges {
                parents,
                children,
                union_confidence,
            }
        })
        .collect()
}

/// Assign a generation to every person in the bundle.
///
/// The rule, as specified:
///
/// * a person in no family's children list is generation 0;
/// * a child is one generation deeper than the **deepest** parent in their
///   family;
/// * anyone who appears in no family at all is unplaced, not omitted.
///
/// "Deepest parent" makes this a longest-path problem, so the breadth-first
/// pass relaxes each child upward and re-queues it whenever a deeper parent is
/// discovered. A parentage cycle would otherwise relax forever; the pass is
/// capped and reports `truncated` rather than hanging the page.
///
/// # Partner alignment
///
/// Those rules alone place every *married-in* spouse at generation 0, because
/// someone whose own parents were never recorded is in nobody's children list.
/// On the operator's file that split 236 of 287 couples across different rows
/// and stretched their connectors up to 17,500px — which contradicts the other
/// half of the specification, that partners are joined by a *horizontal*
/// connector.
///
/// The two requirements can only both hold if partners share a row, so after
/// the initial pass a partner who is nobody's child is pulled down to the
/// deepest generation among their partners. Only such roots move: a person
/// with recorded parents keeps the generation their ancestry earned, so no
/// descendant is ever shifted out from under its parent. Moving a root can
/// deepen the children of another family it parents, so the relaxation is
/// re-run until the assignment stops changing.
pub fn assign_generations(flat: &Value) -> Generations {
    let families = family_edges(flat);
    let all_persons: BTreeSet<String> = flat
        .get("persons")
        .and_then(Value::as_object)
        .map(|m| m.keys().cloned().collect())
        .unwrap_or_default();

    // Who is a child somewhere, and who parents whom.
    let mut is_child: BTreeSet<String> = BTreeSet::new();
    let mut in_a_family: BTreeSet<String> = BTreeSet::new();
    // parent -> families they are a parent in (by index)
    let mut parent_of: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    // child -> families they are a child in (by index)
    let mut child_in: BTreeMap<String, Vec<usize>> = BTreeMap::new();

    for (i, f) in families.iter().enumerate() {
        for p in &f.parents {
            in_a_family.insert(p.clone());
            parent_of.entry(p.clone()).or_default().push(i);
        }
        for (c, _) in &f.children {
            in_a_family.insert(c.clone());
            is_child.insert(c.clone());
            child_in.entry(c.clone()).or_default().push(i);
        }
    }

    // Roots: known to the bundle (or referenced by a family) but not a child.
    let mut gen: BTreeMap<String, i64> = BTreeMap::new();
    let mut queue: VecDeque<String> = VecDeque::new();
    let candidates: BTreeSet<&String> = all_persons.iter().chain(in_a_family.iter()).collect();
    for p in candidates {
        if !is_child.contains(p) && in_a_family.contains(p) {
            gen.insert(p.clone(), 0);
            queue.push_back(p.clone());
        }
    }

    // Relaxation: a child sits one below its deepest known parent.
    let cap = (families.len().max(1) * 64).max(10_000);
    let mut steps = 0usize;
    let mut truncated = false;

    // A closure would need to borrow `gen` and `queue` mutably alongside the
    // alignment pass, so the relaxation is written out per round instead.
    for _round in 0..8 {
        while let Some(p) = queue.pop_front() {
            steps += 1;
            if steps > cap {
                truncated = true;
                break;
            }
            let Some(fam_ids) = parent_of.get(&p) else {
                continue;
            };
            let pg = gen.get(&p).copied().unwrap_or(0);
            for &fi in fam_ids {
                for (c, _) in &families[fi].children {
                    let want = pg + 1;
                    let cur = gen.get(c).copied();
                    if cur.is_none_or(|g| g < want) {
                        gen.insert(c.clone(), want);
                        queue.push_back(c.clone());
                    }
                }
            }
        }
        if truncated {
            break;
        }

        // Pull root partners down to their deepest partner's row. Only people
        // with no recorded parents move, so nobody is lifted above an ancestor.
        let mut changed = false;
        for f in &families {
            let deepest = f
                .parents
                .iter()
                .filter_map(|p| gen.get(p).copied())
                .max()
                .unwrap_or(0);
            for p in &f.parents {
                if is_child.contains(p) {
                    continue; // their ancestry fixes them; leave them be
                }
                if gen.get(p).copied().unwrap_or(0) < deepest {
                    gen.insert(p.clone(), deepest);
                    queue.push_back(p.clone());
                    changed = true;
                }
            }
        }
        if !changed {
            break;
        }
    }

    // A child whose parents are themselves unknown to the bundle never gets
    // relaxed. Place it at least one below zero-depth rather than dropping it.
    for c in child_in.keys() {
        gen.entry(c.clone()).or_insert(1);
    }

    // Unplaced: present in the bundle but in no family whatsoever.
    let unplaced: Vec<String> = all_persons
        .iter()
        .filter(|p| !in_a_family.contains(*p))
        .cloned()
        .collect();

    Generations {
        gen,
        unplaced,
        truncated,
    }
}

// ---------------------------------------------------------------------------
// Card layout
// ---------------------------------------------------------------------------

/// A person card, positioned on the canvas.
#[derive(Debug, Clone, Serialize)]
pub struct Card {
    pub id: String,
    pub name: String,
    /// Lowercased name, for the client-side filter.
    pub search: String,
    pub birth: String,
    pub death: String,
    /// `a`, `b` or `u` — two neutral hues plus unknown, never pink and blue.
    pub sex: &'static str,
    pub x: f64,
    pub y: f64,
    /// Confidence band of the birth fact, shown as a dot on the card.
    pub conf_band: Option<&'static str>,
    pub conf_label: Option<String>,
    /// True for the person the focused view is centred on.
    pub is_root: bool,
}

/// A connector between two cards.
#[derive(Debug, Clone, Serialize)]
pub struct Edge {
    /// `parent` or `spouse`.
    pub kind: &'static str,
    /// SVG path data.
    pub d: String,
    /// Opacity, driven by the relationship's confidence.
    pub opacity: f64,
    pub band: &'static str,
    pub title: String,
}

/// One horizontal row of cards.
#[derive(Debug, Clone, Serialize)]
pub struct Band {
    pub label: String,
    pub sublabel: String,
    pub generation: Option<i64>,
    pub cards: Vec<Card>,
    pub y: f64,
    pub unplaced: bool,
}

/// The complete laid-out tree.
#[derive(Debug, Clone, Serialize)]
pub struct TreeLayout {
    pub bands: Vec<Band>,
    pub edges: Vec<Edge>,
    pub width: f64,
    pub height: f64,
    /// How many cards this layout actually drew.
    pub person_count: usize,
    /// How many people the bundle holds, whether drawn or not.
    pub total_person_count: usize,
    pub generation_count: usize,
    pub unplaced_count: usize,
    pub truncated: bool,
}

// ---------------------------------------------------------------------------
// Choosing what to draw
// ---------------------------------------------------------------------------

/// The people around one person, and how they were reached.
///
/// The full tree is laid out correctly but is not usable at the operator's
/// scale: 283 people share generation 0, which is a canvas nearly 18,000px
/// wide. No one scrolls that far to find an ancestor. A generation that size
/// cannot be made legible by shrinking cards either — the fix is to draw
/// fewer people, not smaller ones.
#[derive(Debug, Clone, PartialEq)]
pub struct Subtree {
    /// Everyone to lay out, including the root.
    pub ids: BTreeSet<String>,
    pub root: String,
    /// Generations of ancestors requested.
    pub up: usize,
    /// Generations of descendants requested.
    pub down: usize,
    /// How many were reached going up, down, and sideways to partners.
    pub ancestor_count: usize,
    pub descendant_count: usize,
    pub spouse_count: usize,
}

/// Map each person to the families they parent, and the families they are a
/// child of. Both directions are needed to walk a subtree.
fn family_index(
    families: &[FamilyEdges],
) -> (BTreeMap<String, Vec<usize>>, BTreeMap<String, Vec<usize>>) {
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
    (parent_of, child_in)
}

/// Select the people within `up` generations above and `down` below `root`,
/// plus the partners of everyone selected.
///
/// Partners are added in a single pass at the end rather than being followed
/// recursively: a partner's own ancestors are a different family's tree, and
/// pulling them in is how a focused view turns back into the whole bundle.
pub fn select_subtree(flat: &Value, root: &str, up: usize, down: usize) -> Subtree {
    let families = family_edges(flat);
    let index = family_index(&families);
    select_with(&families, &index, root, up, down)
}

type FamilyIndex = (BTreeMap<String, Vec<usize>>, BTreeMap<String, Vec<usize>>);

/// The body of [`select_subtree`], against a prebuilt index.
///
/// Split out because choosing a default root evaluates every person in the
/// bundle: re-reading the families out of JSON each time turned a scan into
/// hundreds of redundant parses.
fn select_with(
    families: &[FamilyEdges],
    (parent_of, child_in): &FamilyIndex,
    root: &str,
    up: usize,
    down: usize,
) -> Subtree {
    let mut ids: BTreeSet<String> = BTreeSet::new();
    ids.insert(root.to_string());

    // Upward: the parents of every family this generation is a child of.
    let mut frontier: Vec<String> = vec![root.to_string()];
    let mut ancestor_count = 0usize;
    for _ in 0..up {
        let mut next: Vec<String> = Vec::new();
        for p in &frontier {
            for &fi in child_in.get(p).map(Vec::as_slice).unwrap_or(&[]) {
                for parent in &families[fi].parents {
                    if ids.insert(parent.clone()) {
                        ancestor_count += 1;
                        next.push(parent.clone());
                    }
                }
            }
        }
        if next.is_empty() {
            break;
        }
        frontier = next;
    }

    // Downward: the children of every family this generation parents.
    let mut frontier: Vec<String> = vec![root.to_string()];
    let mut descendant_count = 0usize;
    for _ in 0..down {
        let mut next: Vec<String> = Vec::new();
        for p in &frontier {
            for &fi in parent_of.get(p).map(Vec::as_slice).unwrap_or(&[]) {
                for (child, _) in &families[fi].children {
                    if ids.insert(child.clone()) {
                        descendant_count += 1;
                        next.push(child.clone());
                    }
                }
            }
        }
        if next.is_empty() {
            break;
        }
        frontier = next;
    }

    // Sideways: partners of everyone selected, so couples read as couples.
    let selected: Vec<String> = ids.iter().cloned().collect();
    let mut spouse_count = 0usize;
    for p in selected {
        for &fi in parent_of.get(&p).map(Vec::as_slice).unwrap_or(&[]) {
            for partner in &families[fi].parents {
                if partner != &p && ids.insert(partner.clone()) {
                    spouse_count += 1;
                }
            }
        }
    }

    Subtree {
        ids,
        root: root.to_string(),
        up,
        down,
        ancestor_count,
        descendant_count,
        spouse_count,
    }
}

/// The widest a generation row may be before the focused view stops being
/// readable. Fourteen cards is about 2,000px — one wide screen, or a short
/// scroll on a laptop.
const LEGIBLE_ROW: usize = 14;

/// The person whose surroundings make the best landing page.
///
/// Two earlier readings of "the one with the most descendants" both produced
/// bad first screens on the operator's bundle, and the measurements are worth
/// recording because they are not obvious:
///
/// * **Most descendants overall** picks someone with 118 of them — spread over
///   thirteen further generations, so at depth 3 the view is nine cards out of
///   767. Correct, and useless as an introduction.
/// * **Most people shown at the depth** picks someone with a 69-child
///   generation: 142 cards, but a 10,108px canvas. That is the same
///   horizontal-scrolling problem the focused view exists to solve, just
///   smaller.
///
/// Legibility is governed by the *widest row*, not the total, so the root is
/// the one showing the most people while keeping every row inside
/// [`LEGIBLE_ROW`]. If no one qualifies — every candidate has an enormous
/// sibling group — the narrowest available view wins instead, because a
/// cramped view still beats an unreadable one.
///
/// Ties break on total descendants, then name, then id, so the landing page is
/// stable across restarts rather than depending on map iteration order.
pub fn best_root(flat: &Value, depth: usize) -> Option<String> {
    let families = family_edges(flat);
    let index = family_index(&families);
    let (parent_of, _) = &index;
    let persons = flat.get("persons").and_then(Value::as_object)?;
    if persons.is_empty() {
        return None;
    }
    // Generation numbers come from the same pass the layout uses, so "widest
    // row" here means exactly what it will mean on the rendered page.
    let generations = assign_generations(flat);

    let total_descendants = |id: &str| -> usize {
        let mut seen: BTreeSet<&str> = [id].into();
        let mut queue: VecDeque<&str> = [id].into();
        let mut n = 0usize;
        while let Some(p) = queue.pop_front() {
            for &fi in parent_of.get(p).map(Vec::as_slice).unwrap_or(&[]) {
                for (child, _) in &families[fi].children {
                    if seen.insert(child.as_str()) {
                        n += 1;
                        queue.push_back(child.as_str());
                    }
                }
            }
        }
        n
    };

    /// How a candidate root scores. Ordered best-first by `is_better`.
    struct Score {
        legible: bool,
        shown: usize,
        widest: usize,
        total: usize,
        name: String,
        id: String,
    }

    let is_better = |a: &Score, b: &Score| -> bool {
        // A view that fits wins outright over one that does not.
        match (a.legible, b.legible) {
            (true, false) => return true,
            (false, true) => return false,
            _ => {}
        }
        if a.legible {
            // Both fit: show as many people as possible.
            match a.shown.cmp(&b.shown) {
                std::cmp::Ordering::Greater => return true,
                std::cmp::Ordering::Less => return false,
                std::cmp::Ordering::Equal => {}
            }
        } else {
            // Neither fits: take the narrowest.
            match a.widest.cmp(&b.widest) {
                std::cmp::Ordering::Less => return true,
                std::cmp::Ordering::Greater => return false,
                std::cmp::Ordering::Equal => {}
            }
        }
        match a.total.cmp(&b.total) {
            std::cmp::Ordering::Greater => true,
            std::cmp::Ordering::Less => false,
            std::cmp::Ordering::Equal => a.name < b.name || (a.name == b.name && a.id < b.id),
        }
    };

    let mut best: Option<Score> = None;
    for id in persons.keys() {
        let sub = select_with(&families, &index, id, depth, depth);

        // Bucket the subtree by the generation the layout will put it in.
        let mut per_row: BTreeMap<i64, usize> = BTreeMap::new();
        let mut unplaced = 0usize;
        for pid in &sub.ids {
            if !persons.contains_key(pid) {
                continue;
            }
            match generations.gen.get(pid) {
                Some(g) => *per_row.entry(*g).or_default() += 1,
                None => unplaced += 1,
            }
        }
        let widest = per_row.values().copied().max().unwrap_or(0).max(unplaced);
        let shown = per_row.values().sum::<usize>() + unplaced;

        let candidate = Score {
            legible: widest <= LEGIBLE_ROW,
            shown,
            widest,
            total: total_descendants(id),
            name: persons
                .get(id)
                .map(view::person_display_name)
                .unwrap_or_default(),
            id: id.clone(),
        };
        if best.as_ref().is_none_or(|b| is_better(&candidate, b)) {
            best = Some(candidate);
        }
    }
    best.map(|s| s.id)
}

/// Lay the whole bundle out for `/tree?all=1`.
pub fn layout(flat: &Value) -> TreeLayout {
    layout_subset(flat, None, None)
}

/// Lay out only `subtree`, for the focused default view.
pub fn layout_focused(flat: &Value, subtree: &Subtree) -> TreeLayout {
    layout_subset(flat, Some(&subtree.ids), Some(&subtree.root))
}

/// Lay out the bundle, optionally restricted to a set of people.
///
/// Restricting here rather than in a separate code path means the focused view
/// and the full view share one implementation: generation numbers, ordering
/// and connector geometry are computed exactly the same way, so a person sits
/// in the same generation whichever view you reach them through.
pub fn layout_subset(
    flat: &Value,
    only: Option<&BTreeSet<String>>,
    root: Option<&str>,
) -> TreeLayout {
    let generations = assign_generations(flat);
    let empty = serde_json::Map::new();
    let persons = flat
        .get("persons")
        .and_then(Value::as_object)
        .unwrap_or(&empty);

    let wanted = |id: &String| only.is_none_or(|set| set.contains(id));

    // Group by generation.
    let mut by_gen: BTreeMap<i64, Vec<String>> = BTreeMap::new();
    for (id, g) in &generations.gen {
        // Only lay out persons the bundle actually contains; a family may
        // reference an id that was never imported.
        if persons.contains_key(id) && wanted(id) {
            by_gen.entry(*g).or_default().push(id.clone());
        }
    }

    let max_gen = by_gen.keys().copied().max().unwrap_or(0);

    // Order within a row.
    //
    // Two things make a row readable. Partners must sit next to each other, or
    // their connector stretches across a canvas that is tens of thousands of
    // pixels wide — on the operator's file, generation 0 alone is 283 people.
    // And siblings must sit under their parents, or the parent connectors
    // cross into a thicket.
    //
    // So each row is ordered as *couples first, then barycentre*: persons are
    // grouped into units by shared partnership, and the units are sorted by
    // the mean position of their parents in the row below. One pass, cheap,
    // and enough to make 767 people legible.
    let families = family_edges(flat);
    let mut parents_of_child: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for f in &families {
        for (c, _) in &f.children {
            parents_of_child
                .entry(c.clone())
                .or_default()
                .extend(f.parents.iter().cloned());
        }
    }

    let name_of = |id: &str| -> String {
        persons
            .get(id)
            .map(view::person_display_name)
            .unwrap_or_else(|| "[Unknown]".into())
    };

    // Partners, for grouping. A person who partners in several families joins
    // the first one encountered; splitting them across units is impossible in
    // a single row anyway.
    let mut partners: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for f in &families {
        for a in &f.parents {
            for b in &f.parents {
                if a != b {
                    partners.entry(a.clone()).or_default().push(b.clone());
                }
            }
        }
    }

    let mut order_index: BTreeMap<String, usize> = BTreeMap::new();
    for g in 0..=max_gen {
        let Some(row) = by_gen.get_mut(&g) else {
            continue;
        };
        let members: BTreeSet<&String> = row.iter().collect();

        // Build units of partners who share this row.
        let mut seen: BTreeSet<String> = BTreeSet::new();
        let mut units: Vec<Vec<String>> = Vec::new();
        let mut ordered: Vec<String> = row.clone();
        ordered.sort_by_key(|id| (name_of(id), id.clone()));
        for id in &ordered {
            if seen.contains(id) {
                continue;
            }
            let mut unit = vec![id.clone()];
            seen.insert(id.clone());
            if let Some(ps) = partners.get(id) {
                for p in ps {
                    if members.contains(p) && !seen.contains(p) {
                        unit.push(p.clone());
                        seen.insert(p.clone());
                    }
                }
            }
            units.push(unit);
        }

        // Sort units by the mean position of their members' parents in the
        // row below. Units with no placed parents sort last, by name.
        let barycentre = |unit: &Vec<String>| -> f64 {
            let idxs: Vec<f64> = unit
                .iter()
                .filter_map(|id| parents_of_child.get(id))
                .flatten()
                .filter_map(|p| order_index.get(p))
                .map(|i| *i as f64)
                .collect();
            if idxs.is_empty() {
                f64::MAX
            } else {
                idxs.iter().sum::<f64>() / idxs.len() as f64
            }
        };
        units.sort_by(|a, b| {
            barycentre(a)
                .partial_cmp(&barycentre(b))
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| name_of(&a[0]).cmp(&name_of(&b[0])))
                .then_with(|| a[0].cmp(&b[0]))
        });

        *row = units.into_iter().flatten().collect();
        for (i, id) in row.iter().enumerate() {
            order_index.insert(id.clone(), i);
        }
    }

    // Unplaced people appear in no family, so a subtree walk can never reach
    // one — unless the root *is* unplaced, which is exactly the case this
    // filter keeps working.
    let unplaced: Vec<String> = generations
        .unplaced
        .iter()
        .filter(|id| wanted(id))
        .cloned()
        .collect();

    // Canvas width is set by the widest row.
    let widest = by_gen
        .values()
        .map(|r| r.len())
        .chain(std::iter::once(unplaced.len()))
        .max()
        .unwrap_or(0);
    let width = MARGIN * 2.0 + (widest as f64) * (CARD_W + H_GAP) - H_GAP;
    let width = width.max(320.0);

    // DOM order: unplaced band first (top), then youngest generation down to
    // the oldest.
    let mut bands: Vec<Band> = Vec::new();
    let mut row_y = MARGIN;

    if !unplaced.is_empty() {
        let mut ids = unplaced.clone();
        ids.sort_by_key(|id| (name_of(id), id.clone()));
        let cards = place_row(&ids, persons, row_y, width, root);
        bands.push(Band {
            label: "Unplaced".into(),
            sublabel: format!(
                "{} {} in no family — shown rather than omitted",
                ids.len(),
                if ids.len() == 1 { "person" } else { "people" }
            ),
            generation: None,
            cards,
            y: row_y,
            unplaced: true,
        });
        row_y += ROW_PITCH;
    }

    let mut positions: BTreeMap<String, (f64, f64)> = BTreeMap::new();
    for g in (0..=max_gen).rev() {
        let Some(ids) = by_gen.get(&g) else { continue };
        if ids.is_empty() {
            continue;
        }
        let cards = place_row(ids, persons, row_y, width, root);
        for c in &cards {
            positions.insert(c.id.clone(), (c.x, c.y));
        }
        bands.push(Band {
            label: format!("Generation {g}"),
            sublabel: format!(
                "{} {}",
                ids.len(),
                if ids.len() == 1 { "person" } else { "people" }
            ),
            generation: Some(g),
            cards,
            y: row_y,
            unplaced: false,
        });
        row_y += ROW_PITCH;
    }

    let height = row_y + CARD_H + MARGIN;
    // Connectors are derived from `positions`, which only holds the people
    // that were drawn, so an edge to someone outside the subtree is dropped
    // rather than dangling off the canvas.
    let edges = build_edges(&families, &positions, &generations.gen, persons);

    let drawn = by_gen.values().map(Vec::len).sum::<usize>() + unplaced.len();

    TreeLayout {
        bands,
        edges,
        width,
        height,
        person_count: drawn,
        total_person_count: persons.len(),
        generation_count: by_gen.len(),
        unplaced_count: unplaced.len(),
        truncated: generations.truncated,
    }
}

/// Position one row of cards, centred on the canvas.
fn place_row(
    ids: &[String],
    persons: &serde_json::Map<String, Value>,
    y: f64,
    canvas_w: f64,
    root: Option<&str>,
) -> Vec<Card> {
    let row_w = (ids.len() as f64) * (CARD_W + H_GAP) - H_GAP;
    let x0 = ((canvas_w - row_w) / 2.0).max(MARGIN);
    ids.iter()
        .enumerate()
        .map(|(i, id)| {
            let x = x0 + (i as f64) * (CARD_W + H_GAP);
            card_for(id, persons.get(id), x, y, root == Some(id.as_str()))
        })
        .collect()
}

/// Build one card. A referenced-but-absent person still gets a card so the
/// tree never silently loses someone.
fn card_for(id: &str, person: Option<&Value>, x: f64, y: f64, is_root: bool) -> Card {
    let Some(p) = person else {
        return Card {
            id: id.to_string(),
            name: "[Unknown]".into(),
            search: "[unknown]".into(),
            birth: String::new(),
            death: String::new(),
            sex: "u",
            x,
            y,
            conf_band: None,
            conf_label: None,
            is_root,
        };
    };

    let name = view::person_display_name(p);
    let birth = view::render_date_field(p.get("birth").unwrap_or(&Value::Null), "date");
    let death = view::render_date_field(p.get("death").unwrap_or(&Value::Null), "date");

    let is_living = p
        .get("identity")
        .and_then(|i| i.get("is_living"))
        .and_then(Value::as_bool)
        .unwrap_or(false);

    let sex = match p
        .get("identity")
        .and_then(|i| i.get("gender"))
        .and_then(|g| g.get("value"))
        .and_then(Value::as_str)
    {
        Some("M") => "a",
        Some("F") => "b",
        _ => "u",
    };

    // The card shows the birth fact's confidence, which is the one most
    // readers scan for.
    let conf = p
        .get("birth")
        .and_then(|b| Confidence::from_field(b, "confidence"))
        .or_else(|| birth.confidence.clone());

    Card {
        id: id.to_string(),
        name: name.clone(),
        search: name.to_lowercase(),
        birth: birth.short,
        death: if is_living { "—".into() } else { death.short },
        sex,
        x,
        y,
        conf_band: conf.as_ref().map(|c| c.band),
        conf_label: conf.map(|c| c.description),
        is_root,
    }
}

/// Build the SVG connectors.
///
/// One SVG covers the whole canvas. At the operator's scale — 767 persons,
/// 295 families, on the order of a thousand paths — a single element measures
/// well under the budget, so the added complexity of splitting the drawing per
/// generation gap buys nothing. If a far larger bundle ever made this heavy,
/// the split would go here: each gap already has its own y band.
fn build_edges(
    families: &[FamilyEdges],
    pos: &BTreeMap<String, (f64, f64)>,
    gen: &BTreeMap<String, i64>,
    persons: &serde_json::Map<String, Value>,
) -> Vec<Edge> {
    let mut edges = Vec::new();
    let name = |id: &str| -> String {
        persons
            .get(id)
            .map(view::person_display_name)
            .unwrap_or_else(|| "[Unknown]".into())
    };

    for f in families {
        // Spouse connector: a horizontal line between partners in the same row.
        if f.parents.len() >= 2 {
            for pair in f.parents.windows(2) {
                let (Some(&(ax, ay)), Some(&(bx, by))) = (pos.get(&pair[0]), pos.get(&pair[1]))
                else {
                    continue;
                };
                let c = Confidence::new(f.union_confidence.unwrap_or(0.8));
                let (x1, x2) = if ax <= bx {
                    (ax + CARD_W, bx)
                } else {
                    (bx + CARD_W, ax)
                };
                let y1 = ay + CARD_H / 2.0;
                let y2 = by + CARD_H / 2.0;
                let d = if (ay - by).abs() < f64::EPSILON {
                    format!("M {x1:.1} {y1:.1} L {x2:.1} {y2:.1}")
                } else {
                    // Partners landed in different rows; a straight run would
                    // cut through the cards between them, so bow it out.
                    let mx = (x1 + x2) / 2.0;
                    format!("M {x1:.1} {y1:.1} C {mx:.1} {y1:.1} {mx:.1} {y2:.1} {x2:.1} {y2:.1}")
                };
                edges.push(Edge {
                    kind: "spouse",
                    d,
                    opacity: opacity_for(&c),
                    band: c.band,
                    title: format!(
                        "{} and {} — {}",
                        name(&pair[0]),
                        name(&pair[1]),
                        c.description
                    ),
                });
            }
        }

        // Parent-to-child connectors. The parent sits *below* the child in this
        // inverted layout, so the line leaves the parent's top edge and arrives
        // at the child's bottom edge.
        for (child, conf) in &f.children {
            let Some(&(cx, cy)) = pos.get(child) else {
                continue;
            };
            // Anchor on the deepest parent, which is the one that set the
            // child's generation.
            let anchor = f
                .parents
                .iter()
                .filter(|p| pos.contains_key(*p))
                .max_by_key(|p| gen.get(*p).copied().unwrap_or(0));
            let Some(parent) = anchor else { continue };
            let Some(&(px, py)) = pos.get(parent) else {
                continue;
            };

            let c = Confidence::new(conf.unwrap_or(0.8));
            let x1 = px + CARD_W / 2.0;
            let y1 = py; // parent's top edge
            let x2 = cx + CARD_W / 2.0;
            let y2 = cy + CARD_H; // child's bottom edge
            let mid = (y1 + y2) / 2.0;
            let d = format!("M {x1:.1} {y1:.1} V {mid:.1} H {x2:.1} V {y2:.1}");

            edges.push(Edge {
                kind: "parent",
                d,
                opacity: opacity_for(&c),
                band: c.band,
                title: format!("{} → {} — {}", name(parent), name(child), c.description),
            });
        }
    }
    edges
}

/// Map a confidence to line opacity.
///
/// This is the showcase requirement applied to the tree itself: a 0.4
/// parentage must be visibly fainter than a 0.99 one. The floor keeps a
/// speculative link visible — faint is the point, invisible is a lie.
fn opacity_for(c: &Confidence) -> f64 {
    0.15 + 0.75 * c.value
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// Build a flat bundle from person ids and (parents, children) families.
    fn bundle(persons: &[&str], families: &[(&[&str], &[&str])]) -> Value {
        let mut ps = serde_json::Map::new();
        for p in persons {
            ps.insert(
                (*p).to_string(),
                json!({"id": p, "type":"person", "axgf_version":"1.0",
                       "identity":{"name":{"display": p}}}),
            );
        }
        let mut fs = serde_json::Map::new();
        for (i, (parents, children)) in families.iter().enumerate() {
            let id = format!("f{i}");
            fs.insert(
                id.clone(),
                json!({
                    "id": id, "type":"family", "axgf_version":"1.0",
                    "union": {"persons": parents.iter()
                        .map(|p| json!({"person_id": p, "role":"spouse"}))
                        .collect::<Vec<_>>()},
                    "children": children.iter()
                        .map(|c| json!({"person_id": c}))
                        .collect::<Vec<_>>(),
                }),
            );
        }
        json!({"manifest":{"axgf":"1.0"}, "persons": ps, "families": fs})
    }

    fn g(gens: &Generations, id: &str) -> i64 {
        *gens
            .gen
            .get(id)
            .unwrap_or_else(|| panic!("{id} was not placed"))
    }

    #[test]
    fn three_generation_tree_is_numbered_oldest_first() {
        // grandparents -> parent -> child
        let b = bundle(
            &["gp1", "gp2", "par", "spouse", "kid"],
            &[(&["gp1", "gp2"], &["par"]), (&["par", "spouse"], &["kid"])],
        );
        let gens = assign_generations(&b);
        assert_eq!(g(&gens, "gp1"), 0);
        assert_eq!(g(&gens, "gp2"), 0);
        assert_eq!(g(&gens, "par"), 1);
        assert_eq!(g(&gens, "kid"), 2);
        // The married-in spouse has no recorded parents, so the base rule
        // would leave them at 0. Partner alignment pulls them alongside the
        // person they married, which is what makes their connector horizontal.
        assert_eq!(g(&gens, "spouse"), 1);
        assert!(gens.unplaced.is_empty());
    }

    #[test]
    fn a_person_with_no_parents_is_generation_zero() {
        let b = bundle(&["a", "b", "c"], &[(&["a", "b"], &["c"])]);
        let gens = assign_generations(&b);
        assert_eq!(g(&gens, "a"), 0);
        assert_eq!(g(&gens, "b"), 0);
    }

    #[test]
    fn a_sibling_group_with_no_parents_shares_one_generation() {
        // A family with children but no recorded union members.
        let b = bundle(&["s1", "s2", "s3"], &[(&[], &["s1", "s2", "s3"])]);
        let gens = assign_generations(&b);
        assert_eq!(g(&gens, "s1"), g(&gens, "s2"));
        assert_eq!(g(&gens, "s2"), g(&gens, "s3"));
        assert!(gens.unplaced.is_empty(), "siblings are in a family");
    }

    #[test]
    fn a_person_in_no_family_is_unplaced_not_omitted() {
        let b = bundle(&["a", "b", "c", "loner"], &[(&["a", "b"], &["c"])]);
        let gens = assign_generations(&b);
        assert_eq!(gens.unplaced, vec!["loner".to_string()]);
        assert!(!gens.gen.contains_key("loner"));

        // And the layout must still show them.
        let l = layout(&b);
        assert_eq!(l.unplaced_count, 1);
        let band = l.bands.first().expect("a band");
        assert!(band.unplaced, "the unplaced band comes first in the DOM");
        assert!(band.cards.iter().any(|c| c.id == "loner"));
    }

    #[test]
    fn a_child_sits_below_its_deepest_parent() {
        // "shallow" is a root; "deep" is two generations down. Their child
        // must land below the deeper of the two, not the shallower.
        let b = bundle(
            &["r1", "r2", "mid", "deep", "shallow", "kid"],
            &[
                (&["r1", "r2"], &["mid"]),
                (&["mid"], &["deep"]),
                (&["deep", "shallow"], &["kid"]),
            ],
        );
        let gens = assign_generations(&b);
        assert_eq!(g(&gens, "mid"), 1);
        assert_eq!(g(&gens, "deep"), 2);
        // "shallow" married in with no recorded parents, so alignment brings
        // them alongside "deep" rather than stranding them at generation 0.
        assert_eq!(g(&gens, "shallow"), 2);
        assert_eq!(
            g(&gens, "kid"),
            3,
            "deepest parent wins, not the shallowest"
        );
    }

    #[test]
    fn partner_alignment_never_lifts_someone_above_their_own_parents() {
        // "b" has recorded parents putting them at generation 1, and marries
        // "d" who sits at generation 2. Alignment must not move "b" — doing so
        // would place them level with or above their own parent.
        let bd = bundle(
            &["p1", "p2", "b", "g1", "c", "d", "kid"],
            &[
                (&["p1", "p2"], &["b"]),
                (&["g1"], &["c"]),
                (&["c"], &["d"]),
                (&["b", "d"], &["kid"]),
            ],
        );
        let gens = assign_generations(&bd);
        assert_eq!(g(&gens, "b"), 1, "a person with parents keeps their row");
        assert_eq!(g(&gens, "d"), 2);
        assert!(
            g(&gens, "b") > g(&gens, "p1"),
            "a child must stay below its parent"
        );
        assert_eq!(g(&gens, "kid"), 3, "still one below the deepest parent");
    }

    #[test]
    fn a_closed_parentage_cycle_still_places_everyone() {
        // a is b's parent and b is a's parent. Nonsense data. Neither is a
        // root, so the relaxation pass never starts — but both people must
        // still land somewhere rather than vanishing from the page.
        let b = bundle(&["a", "b"], &[(&["a"], &["b"]), (&["b"], &["a"])]);
        let gens = assign_generations(&b);
        assert!(gens.gen.contains_key("a"));
        assert!(gens.gen.contains_key("b"));
        assert!(gens.unplaced.is_empty());
    }

    #[test]
    fn a_cycle_reachable_from_a_root_is_capped_and_reported() {
        // r is a genuine root feeding into an a<->b loop, so relaxation would
        // otherwise deepen the pair forever.
        let b = bundle(
            &["r", "a", "b"],
            &[(&["r"], &["a"]), (&["a"], &["b"]), (&["b"], &["a"])],
        );
        let gens = assign_generations(&b);
        assert!(
            gens.truncated,
            "an unbounded cycle should be reported, not absorbed silently"
        );
        // And the page still renders.
        let l = layout(&b);
        assert!(l.truncated);
        assert_eq!(l.person_count, 3);
    }

    #[test]
    fn empty_bundle_lays_out_without_panicking() {
        let b = json!({"manifest":{"axgf":"1.0"},"persons":{},"families":{}});
        let l = layout(&b);
        assert_eq!(l.person_count, 0);
        assert!(l.bands.is_empty());
        assert!(l.edges.is_empty());
        assert!(l.width >= 320.0);
    }

    // -- layout ------------------------------------------------------------

    #[test]
    fn youngest_generation_is_first_in_the_dom() {
        let b = bundle(
            &["gp1", "gp2", "par", "kid"],
            &[(&["gp1", "gp2"], &["par"]), (&["par"], &["kid"])],
        );
        let l = layout(&b);
        let gens: Vec<i64> = l.bands.iter().filter_map(|b| b.generation).collect();
        assert_eq!(gens, vec![2, 1, 0], "highest generation first");
        // And the youngest row must sit above the oldest on the canvas.
        let y_young = l.bands.iter().find(|b| b.generation == Some(2)).unwrap().y;
        let y_old = l.bands.iter().find(|b| b.generation == Some(0)).unwrap().y;
        assert!(y_young < y_old, "youngest at the top");
    }

    #[test]
    fn parent_connectors_run_from_parent_top_to_child_bottom() {
        let b = bundle(&["p", "k"], &[(&["p"], &["k"])]);
        let l = layout(&b);
        let e = l
            .edges
            .iter()
            .find(|e| e.kind == "parent")
            .expect("a parent edge");
        // In this inverted layout the parent is lower on screen, so the path
        // must start below where it ends.
        let nums: Vec<f64> =
            e.d.split_whitespace()
                .filter_map(|t| t.parse::<f64>().ok())
                .collect();
        let (start_y, end_y) = (nums[1], *nums.last().unwrap());
        assert!(
            start_y > end_y,
            "line should travel upward from parent to child: {}",
            e.d
        );
    }

    #[test]
    fn connector_opacity_tracks_confidence() {
        let mut b = bundle(&["p", "k1", "k2"], &[(&["p"], &["k1", "k2"])]);
        // Give the two children very different parentage confidence.
        b["families"]["f0"]["children"][0]["confidence"] = json!(0.99);
        b["families"]["f0"]["children"][1]["confidence"] = json!(0.4);
        let l = layout(&b);
        let mut ops: Vec<f64> = l
            .edges
            .iter()
            .filter(|e| e.kind == "parent")
            .map(|e| e.opacity)
            .collect();
        ops.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(ops.len(), 2);
        assert!(
            ops[1] - ops[0] > 0.3,
            "a 0.4 parentage must look clearly fainter than a 0.99 one: {ops:?}"
        );
        assert!(ops[0] > 0.0, "faint, but never invisible");
    }

    #[test]
    fn cards_carry_the_search_key_and_confidence_band() {
        let mut b = bundle(&["a", "x"], &[(&["a"], &["x"])]);
        b["persons"]["a"]["identity"]["name"]["display"] = json!("Ada Lovelace");
        b["persons"]["a"]["birth"] = json!({"date":{"value":"1815","precision":"year"},
                                            "confidence": 0.95});
        let l = layout(&b);
        let card = l
            .bands
            .iter()
            .flat_map(|b| &b.cards)
            .find(|c| c.id == "a")
            .unwrap();
        assert_eq!(card.name, "Ada Lovelace");
        assert_eq!(card.search, "ada lovelace");
        assert_eq!(card.birth, "1815");
        assert_eq!(card.conf_band, Some("certain"));
    }

    #[test]
    fn partners_are_laid_out_next_to_each_other() {
        // Names chosen so a plain alphabetical sort would separate the couple:
        // "aaa" and "zzz" are partners, "mmm" would land between them.
        let mut b = bundle(
            &["aaa", "zzz", "mmm", "kid", "kid2"],
            &[(&["aaa", "zzz"], &["kid"]), (&["mmm"], &["kid2"])],
        );
        for p in ["aaa", "zzz", "mmm"] {
            b["persons"][p]["identity"]["name"]["display"] = json!(p);
        }
        let l = layout(&b);
        let row = &l
            .bands
            .iter()
            .find(|band| band.generation == Some(0))
            .expect("generation 0")
            .cards;
        let idx = |id: &str| row.iter().position(|c| c.id == id).unwrap() as i64;
        assert_eq!(
            (idx("aaa") - idx("zzz")).abs(),
            1,
            "partners must be adjacent, or their connector spans the canvas"
        );
    }

    #[test]
    fn siblings_are_grouped_under_their_parents() {
        // Two couples, each with two children. The children of one couple must
        // not be interleaved with the children of the other.
        let b = bundle(
            &["p1", "p2", "q1", "q2", "a1", "a2", "b1", "b2"],
            &[
                (&["p1", "p2"], &["a1", "a2"]),
                (&["q1", "q2"], &["b1", "b2"]),
            ],
        );
        let l = layout(&b);
        let row = &l
            .bands
            .iter()
            .find(|band| band.generation == Some(1))
            .expect("generation 1")
            .cards;
        let idx = |id: &str| row.iter().position(|c| c.id == id).unwrap() as i64;
        assert_eq!((idx("a1") - idx("a2")).abs(), 1, "siblings stay together");
        assert_eq!((idx("b1") - idx("b2")).abs(), 1, "siblings stay together");
    }

    // -- subtree selection -------------------------------------------------

    /// A five-generation line with a married-in spouse at each level, plus an
    /// unrelated person in no family at all.
    fn deep_bundle() -> Value {
        bundle(
            &[
                "g0", "g0s", "g1", "g1s", "g2", "g2s", "g3", "g3s", "g4", "loner",
            ],
            &[
                (&["g0", "g0s"], &["g1"]),
                (&["g1", "g1s"], &["g2"]),
                (&["g2", "g2s"], &["g3"]),
                (&["g3", "g3s"], &["g4"]),
            ],
        )
    }

    fn ids(s: &Subtree) -> Vec<&str> {
        let mut v: Vec<&str> = s.ids.iter().map(String::as_str).collect();
        v.sort_unstable();
        v
    }

    #[test]
    fn a_subtree_reaches_the_requested_depth_in_both_directions() {
        let b = deep_bundle();
        let s = select_subtree(&b, "g2", 1, 1);
        // One up: g1 and (via the partner pass) g1s. One down: g3, plus g3's
        // partner g3s. And g2's own partner g2s.
        assert_eq!(ids(&s), vec!["g1", "g1s", "g2", "g2s", "g3", "g3s"]);
        assert!(!s.ids.contains("g0"), "two generations up is out of range");
        assert!(
            !s.ids.contains("g4"),
            "two generations down is out of range"
        );
    }

    #[test]
    fn depth_three_each_way_is_the_default_shape() {
        let b = deep_bundle();
        let s = select_subtree(&b, "g2", 3, 3);
        assert!(s.ids.contains("g0"), "reaches the top of a 5-deep line");
        assert!(s.ids.contains("g4"), "reaches the bottom");
        assert!(!s.ids.contains("loner"), "never pulls in unrelated people");
    }

    #[test]
    fn a_root_with_no_ancestors_still_selects_its_descendants() {
        let b = deep_bundle();
        let s = select_subtree(&b, "g0", 3, 2);
        assert_eq!(s.ancestor_count, 0, "g0 is the top of the line");
        assert!(s.ids.contains("g1"));
        assert!(s.ids.contains("g2"));
        assert!(!s.ids.contains("g3"), "only two generations down");
        assert!(s.ids.contains("g0"), "the root is always included");
    }

    #[test]
    fn a_root_with_no_descendants_still_selects_its_ancestors() {
        let b = deep_bundle();
        let s = select_subtree(&b, "g4", 2, 3);
        assert_eq!(s.descendant_count, 0, "g4 has no children");
        assert!(s.ids.contains("g3"));
        assert!(s.ids.contains("g2"));
        assert!(!s.ids.contains("g1"), "only two generations up");
    }

    #[test]
    fn depth_zero_shows_the_root_and_their_partners_only() {
        let b = deep_bundle();
        let s = select_subtree(&b, "g2", 0, 0);
        assert_eq!(s.ancestor_count, 0);
        assert_eq!(s.descendant_count, 0);
        // The partner pass still runs, because a couple is one unit even at
        // depth zero.
        assert_eq!(ids(&s), vec!["g2", "g2s"]);
    }

    #[test]
    fn depth_zero_on_a_person_with_no_family_is_just_that_person() {
        let b = deep_bundle();
        let s = select_subtree(&b, "loner", 0, 0);
        assert_eq!(ids(&s), vec!["loner"]);
    }

    #[test]
    fn a_root_who_is_unplaced_still_lays_out() {
        // Someone in no family at all can never be reached from another root,
        // but they can be the root themselves — and the unplaced band is the
        // only place they can be drawn.
        let b = deep_bundle();
        let s = select_subtree(&b, "loner", 3, 3);
        assert_eq!(ids(&s), vec!["loner"]);

        let l = layout_focused(&b, &s);
        assert_eq!(l.person_count, 1, "the root is drawn");
        assert_eq!(l.unplaced_count, 1);
        let band = l.bands.first().expect("a band");
        assert!(band.unplaced);
        assert!(band.cards.iter().any(|c| c.id == "loner" && c.is_root));
    }

    #[test]
    fn an_unknown_root_selects_only_itself_and_renders_empty() {
        let b = deep_bundle();
        let s = select_subtree(&b, "nobody-at-all", 3, 3);
        assert_eq!(ids(&s), vec!["nobody-at-all"]);
        // It is not in `persons`, so nothing is drawn — but nothing panics.
        let l = layout_focused(&b, &s);
        assert_eq!(l.person_count, 0);
        assert!(l.bands.is_empty());
    }

    #[test]
    fn a_focused_layout_draws_far_fewer_cards_than_the_full_one() {
        let b = deep_bundle();
        let full = layout(&b);
        let s = select_subtree(&b, "g2", 1, 1);
        let focused = layout_focused(&b, &s);

        assert_eq!(full.person_count, 10, "everyone, including the loner");
        assert_eq!(focused.person_count, 6);
        assert_eq!(
            focused.total_person_count, 10,
            "the bundle size is still reported"
        );
        // Every row in this toy line is two people wide, so the canvas cannot
        // narrow here — see the wide-generation test for that.
        assert!(focused.width <= full.width);
    }

    #[test]
    fn focusing_collapses_the_canvas_when_a_generation_is_wide() {
        // This is the shape that made the full view unusable: one enormous
        // generation. 60 siblings is a 60-card row; focusing on one of them
        // draws that one, their partner and their parent.
        let mut people: Vec<String> = vec!["ma".into(), "pa".into()];
        for i in 0..60 {
            people.push(format!("kid{i:02}"));
        }
        let people_refs: Vec<&str> = people.iter().map(String::as_str).collect();
        let kids: Vec<&str> = people_refs[2..].to_vec();
        let b = bundle(&people_refs, &[(&["ma", "pa"], &kids)]);

        let full = layout(&b);
        let s = select_subtree(&b, "kid00", 1, 1);
        let focused = layout_focused(&b, &s);

        assert_eq!(full.person_count, 62);
        assert_eq!(focused.person_count, 3, "the kid and both parents");
        assert!(
            focused.width < full.width / 10.0,
            "focusing must collapse a wide canvas: {} vs {}",
            focused.width,
            full.width
        );
    }

    #[test]
    fn focused_connectors_never_dangle_outside_the_subtree() {
        let b = deep_bundle();
        let s = select_subtree(&b, "g2", 1, 1);
        let l = layout_focused(&b, &s);

        // Every drawn card position must be inside the canvas, and every edge
        // must have been built from two drawn cards.
        for band in &l.bands {
            for c in &band.cards {
                assert!(c.x >= 0.0 && c.x <= l.width, "{c:?}");
            }
        }
        // g4 is outside the subtree, so no connector may mention it.
        assert!(
            !l.edges.iter().any(|e| e.title.contains("g4")),
            "an edge escaped the subtree: {:?}",
            l.edges
        );
    }

    #[test]
    fn generation_numbers_are_the_same_in_both_views() {
        // Focusing changes which people are drawn, never where they belong.
        let b = deep_bundle();
        let full = layout(&b);
        let s = select_subtree(&b, "g2", 1, 1);
        let focused = layout_focused(&b, &s);

        let gen_of = |l: &TreeLayout, id: &str| -> Option<i64> {
            l.bands
                .iter()
                .find(|band| band.cards.iter().any(|c| c.id == id))
                .and_then(|band| band.generation)
        };
        for id in ["g1", "g2", "g3"] {
            assert_eq!(
                gen_of(&full, id),
                gen_of(&focused, id),
                "{id} moved between views"
            );
        }
    }

    // -- root selection ----------------------------------------------------

    #[test]
    fn the_default_root_maximises_what_the_first_screen_shows() {
        // g2 sits in the middle of the five-deep line, so at depth 1 it sees
        // both a parent and a child — more than an end of the line can.
        let b = deep_bundle();
        let root = best_root(&b, 1).expect("a root");
        let shown = select_subtree(&b, &root, 1, 1).ids.len();
        for other in ["g0", "g1", "g2", "g3", "g4"] {
            let n = select_subtree(&b, other, 1, 1).ids.len();
            assert!(
                shown >= n,
                "{root} shows {shown} but {other} would show {n}"
            );
        }
    }

    #[test]
    fn the_default_root_beats_a_long_thin_line() {
        // This is the shape that made total-descendant-count the wrong metric:
        // "trunk" has the most descendants overall, but they are single file,
        // so at depth 1 it shows fewer people than a parent of six.
        let mut people = vec!["trunk".to_string()];
        for i in 0..8 {
            people.push(format!("thin{i}"));
        }
        people.push("wide".into());
        for i in 0..6 {
            people.push(format!("broad{i}"));
        }
        let refs: Vec<&str> = people.iter().map(String::as_str).collect();

        let mut fams: Vec<(Vec<&str>, Vec<&str>)> = vec![(vec!["trunk"], vec!["thin0"])];
        for i in 0..7 {
            fams.push((vec![&refs[1 + i]], vec![&refs[2 + i]]));
        }
        fams.push((vec!["wide"], (0..6).map(|i| refs[10 + i]).collect()));
        let fam_refs: Vec<(&[&str], &[&str])> = fams
            .iter()
            .map(|(a, b)| (a.as_slice(), b.as_slice()))
            .collect();
        let b = bundle(&refs, &fam_refs);

        assert_eq!(
            best_root(&b, 1).as_deref(),
            Some("wide"),
            "the fullest first screen wins over the longest line"
        );
    }

    #[test]
    fn best_root_prefers_a_real_line_over_an_isolated_person() {
        let b = bundle(
            &["parent", "kid1", "kid2", "alone"],
            &[(&["parent"], &["kid1", "kid2"])],
        );
        assert_eq!(best_root(&b, 3).as_deref(), Some("parent"));
    }

    #[test]
    fn best_root_is_none_for_an_empty_bundle() {
        let b = json!({"manifest": {"axgf": "1.0"}, "persons": {}, "families": {}});
        assert_eq!(best_root(&b, 3), None);
    }

    #[test]
    fn best_root_terminates_on_a_parentage_cycle() {
        let b = bundle(&["a", "b"], &[(&["a"], &["b"]), (&["b"], &["a"])]);
        // Nonsense data, but it must return rather than spin.
        assert!(best_root(&b, 3).is_some());
    }

    #[test]
    fn a_subtree_walk_terminates_on_a_parentage_cycle() {
        let b = bundle(&["a", "b"], &[(&["a"], &["b"]), (&["b"], &["a"])]);
        let s = select_subtree(&b, "a", 5, 5);
        assert!(s.ids.contains("a") && s.ids.contains("b"));
    }

    #[test]
    fn a_living_person_shows_a_dash_for_death() {
        let mut b = bundle(&["a", "x"], &[(&["a"], &["x"])]);
        b["persons"]["a"]["identity"]["is_living"] = json!(true);
        let l = layout(&b);
        let card = l
            .bands
            .iter()
            .flat_map(|b| &b.cards)
            .find(|c| c.id == "a")
            .unwrap();
        assert_eq!(card.death, "—");
    }
}
