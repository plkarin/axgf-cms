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
// Tall enough for a two-line name plus the dates line. Polish given-name
// chains run long ("Alfons Władysław Antoni Wierzbięta"); at 58px the clamped
// second line, ellipsis and all, was clipped by the card's own overflow, so a
// name read as a truncated fragment. The connector geometry and row pitch are
// both derived from this, so raising it keeps cards and wires aligned.
const CARD_H: f64 = 66.0;
const H_GAP: f64 = 14.0;
/// Gap between one generation and the next. A band's own rows sit `ROW_GAP`
/// apart, which is a third of this — the difference is what tells a reader
/// that a wrapped generation is one band rather than several.
const V_GAP: f64 = 78.0;
const MARGIN: f64 = 24.0;

/// A person the reader may not read, drawn as a marker rather than a card.
///
/// The genealogical fact that survives redaction is "somebody is here", and
/// that needs a place in the row, not a card's width. On the operator's bundle
/// a signed-out visitor may read nobody at all, so every one of the 866 cards
/// is a redaction: at full width that is a 24,124px canvas of the word
/// "Private". Wide enough to stay an obvious, clickable-looking object and to
/// hold the dash the card draws in place of dates.
const MARK_W: f64 = 34.0;

/// Vertical gap between the wrapped rows of one generation.
///
/// Deliberately much smaller than `V_GAP`, which separates one generation from
/// the next. The two gaps are what tell a reader that four rows are one band
/// and the next band is a different generation, so they must not be similar.
const ROW_GAP: f64 = 26.0;

/// A corridor kept clear at each side of the canvas.
///
/// When a generation wraps, an edge leaving a card on an inner row has to get
/// past its own generation's other rows to reach the one above. It climbs a
/// lane — a vertical strip with no card in it. Lanes between cards are used
/// first because they are close to the edge's own path; these two are the
/// fallback that always exists, whatever the rows happen to contain.
const LANE: f64 = 22.0;

/// A generation larger than this folds into a thicket rather than a band.
///
/// The number that matters for folding is not how many rows it takes but how
/// many people are in them. Fourteen people folded onto fourteen rows of one
/// card is a column, and on a phone it is the only thing that fits. A hundred
/// and sixty-five people folded onto fourteen rows is a hundred and sixty-five
/// connectors climbing past each other, and the crossings cost more than the
/// width saved. So the row cap below applies to the second and not the first.
const THICKET: usize = 48;

/// The most rows a generation over `THICKET` will fold onto.
///
/// Past this the band stops reading as one generation — the label is at its
/// top, the reader is somewhere in the middle, and there is more band above
/// and below than there is screen. Wrapping also stops paying for itself
/// around here: every extra row is another set of connectors that has to climb
/// past it, and on the operator's widest generation the crossings grow faster
/// than the width falls.
///
/// A generation too wide to fold into this many rows is drawn wider than the
/// target instead, and the horizontal scrollbar comes back for that view. That
/// is the honest outcome: 165 people in one generation cannot be made legible
/// by folding them into a column, and pretending otherwise trades a scrollbar
/// for a thicket.
const MAX_BAND_ROWS: usize = 6;

/// How wide a row may be when the request does not say.
///
/// The layout is computed in Rust and shipped as absolute coordinates, so the
/// server has to choose a width before it can know the reader's. This is the
/// no-JavaScript answer; `tree.js` measures the column it actually got and
/// stores it, so the next navigation is laid out to the real one.
pub const DEFAULT_WRAP_W: f64 = 1200.0;
/// One card, plus the margins and lanes around it.
///
/// Asking for less than this cannot be honoured — a card is 132px and does not
/// shrink — so it is the floor. On a phone the tree does become a column, and
/// a column is the only thing that fits a phone.
pub const MIN_WRAP_W: f64 = CARD_W + 2.0 * (MARGIN + LANE);
/// Wider than this is not a screen anybody has; it is a way of asking the
/// server to build an enormous canvas.
pub const MAX_WRAP_W: f64 = 4000.0;

/// How a tree should be laid out, beyond which people are in it.
#[derive(Debug, Clone, Copy)]
pub struct LayoutOpts<'a> {
    /// Widest a single row of cards may be, in canvas pixels.
    pub wrap_w: f64,
    /// The people this reader may read. `None` means "everyone" — an
    /// administrator, or a bundle with nothing withheld. A person outside the
    /// set keeps their place in the row but takes a marker's width.
    pub visible: Option<&'a BTreeSet<String>>,
}

impl Default for LayoutOpts<'_> {
    fn default() -> Self {
        Self {
            wrap_w: DEFAULT_WRAP_W,
            visible: None,
        }
    }
}

impl<'a> LayoutOpts<'a> {
    /// Clamp a requested width into the range the layout will honour.
    pub fn with_width(mut self, w: f64) -> Self {
        self.wrap_w = w.clamp(MIN_WRAP_W, MAX_WRAP_W);
        self
    }

    /// Restrict to what this reader may read.
    pub fn seeing(mut self, visible: Option<&'a BTreeSet<String>>) -> Self {
        self.visible = visible;
        self
    }

    /// Is this person drawn as a marker rather than a card?
    fn hidden(&self, id: &str) -> bool {
        self.visible.is_some_and(|set| !set.contains(id))
    }

    /// The width one person's box takes in a row.
    fn box_w(&self, id: &str) -> f64 {
        if self.hidden(id) {
            MARK_W
        } else {
            CARD_W
        }
    }
}

/// Where every person sits, by generation.
#[derive(Debug, Default, PartialEq)]
pub struct Generations {
    /// Generation number per placed person. 0 is the oldest.
    pub gen: BTreeMap<String, i64>,
    /// Persons who appear in no family at all — neither as a partner nor as a
    /// child. They are shown in their own band rather than dropped.
    pub unplaced: Vec<String>,
    /// True when the bundle states something no assignment can satisfy — a
    /// parentage loop, or a union between two people on one line of descent.
    /// The rows are still drawn; some of them are knowingly wrong.
    pub truncated: bool,
    /// The parent-child pairs that could not be honoured, in the order they
    /// were found. Naming them is the difference between telling a reader
    /// something is wrong and telling them where: "run the validator" is not
    /// an instruction a signed-out visitor can follow.
    pub contradictions: Vec<(String, String)>,
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
/// The rules, as specified:
///
/// * a child is one generation deeper than the **deepest** parent in their
///   family;
/// * **two spouses share a generation**, however much of each side's ancestry
///   happens to be written down;
/// * anyone who appears in no family at all is unplaced, not omitted.
///
/// # Why the union edge has to be a constraint
///
/// Deriving depth from parent-child edges alone put the operator in generation
/// 14 — he descends from a deeply documented line — and his wife in generation
/// 1, because her mother is where her recorded ancestry stops. Both numbers
/// were individually correct and the pair was wrong: a generation is a social
/// position, not a count of how much research has been done on each side.
/// Across the whole file that reading split 236 of 287 couples onto different
/// rows and stretched their connectors up to 17,500px.
///
/// # How both constraints are satisfied at once
///
/// "Spouses are equal" is not something to iterate towards; it is an identity.
/// So the pass **contracts each union into one node** ([`Couples`]) and solves
/// the parent-child constraint on the resulting quotient graph, where it is a
/// plain longest-path problem over a DAG. Spouses cannot come out on different
/// rows because they are no longer separate vertices.
///
/// Alternating a levelling pass with a relaxation pass — the obvious reading —
/// does not work on real data and the failure is worth recording: this file
/// contains two unions between people already related by descent, and each one
/// makes the two passes chase each other. Levelling raises a spouse, relaxing
/// pushes their in-law deeper, which makes the couple unlevel again. Run to a
/// fixed point it never converges; run to a round cap it produced generation
/// numbers in the millions and left 203 couples still split.
///
/// # Pulling a married-in line down to meet its descendant
///
/// Contraction alone leaves the *other* half of the operator's complaint. His
/// wife is now in generation 14, but her mother, whose ancestry is recorded no
/// further, stays at 0 with a fourteen-row connector between them. So a second
/// pass ([`lift_free_ancestry`]) slides such a line down until it sits directly
/// above the descendant that anchors it. It moves a set only when that set's
/// *only* connection to the rest of the graph is the one node being anchored
/// to, which is what makes it a single sweep rather than another fixed point:
/// a lift can never disturb anything it did not include. On this bundle 29
/// lifts fire, the mother lands on 13, and her son — Laura's brother, who has
/// no recorded descendants of his own to hold him up — travels with her.
pub fn assign_generations(flat: &Value) -> Generations {
    let families = family_edges(flat);
    let all_persons: BTreeSet<String> = flat
        .get("persons")
        .and_then(Value::as_object)
        .map(|m| m.keys().cloned().collect())
        .unwrap_or_default();

    // Everyone a family mentions, whether or not the bundle holds them.
    let mut in_a_family: BTreeSet<String> = BTreeSet::new();
    let mut couples = Couples::default();
    for f in &families {
        for p in &f.parents {
            in_a_family.insert(p.clone());
        }
        for (c, _) in &f.children {
            in_a_family.insert(c.clone());
        }
        // Every union member is one node. Chaining consecutive partners is
        // enough to merge all of them, including a polygamous union's three.
        for pair in f.parents.windows(2) {
            couples.merge(&pair[0], &pair[1]);
        }
    }

    let quotient = Quotient::contract(&families, &in_a_family, &mut couples);
    let (mut level, cyclic) = quotient.longest_path();
    lift_free_ancestry(&quotient, &mut level);

    let gen: BTreeMap<String, i64> = in_a_family
        .iter()
        .map(|p| {
            let g = level.get(&couples.find(p)).copied().unwrap_or(0);
            (p.clone(), g)
        })
        .collect();

    // Unplaced: present in the bundle but in no family whatsoever.
    let unplaced: Vec<String> = all_persons
        .iter()
        .filter(|p| !in_a_family.contains(*p))
        .cloned()
        .collect();

    Generations {
        gen,
        unplaced,
        truncated: cyclic || !quotient.contradictions.is_empty(),
        contradictions: quotient.contradictions,
    }
}

/// Union-find over spouses: every person maps to the couple they belong to.
///
/// A person in no union is their own couple, so the quotient graph covers
/// everybody and no caller needs a special case for the unmarried.
#[derive(Default)]
struct Couples {
    parent: BTreeMap<String, String>,
}

impl Couples {
    /// The representative of `id`'s couple, with path compression.
    fn find(&mut self, id: &str) -> String {
        let mut root = id.to_string();
        while let Some(next) = self.parent.get(&root) {
            if next == &root {
                break;
            }
            root = next.clone();
        }
        // Compress, so a long chain of merges is walked once rather than once
        // per lookup — `find` is called for every person and every edge.
        let mut cur = id.to_string();
        while let Some(next) = self.parent.get(&cur).cloned() {
            if next == cur {
                break;
            }
            self.parent.insert(cur, root.clone());
            cur = next;
        }
        root
    }

    /// Put `a` and `b` in the same couple.
    fn merge(&mut self, a: &str, b: &str) {
        let (ra, rb) = (self.find(a), self.find(b));
        if ra != rb {
            self.parent.insert(ra, rb);
        }
    }
}

/// The parent-child graph with every union contracted to a single node.
struct Quotient {
    nodes: BTreeSet<String>,
    /// node -> the couples it parents.
    kids: BTreeMap<String, BTreeSet<String>>,
    /// node -> the couples that parent it.
    parents: BTreeMap<String, BTreeSet<String>>,
    /// Parent-child edges that landed inside a single couple. The bundle is
    /// claiming someone is both spouse and descendant on one line; the edge is
    /// dropped because no row assignment can honour it. The pair is kept, not
    /// just counted, so the page can name the two people.
    contradictions: Vec<(String, String)>,
}

impl Quotient {
    fn contract(
        families: &[FamilyEdges],
        in_a_family: &BTreeSet<String>,
        couples: &mut Couples,
    ) -> Self {
        let mut q = Quotient {
            nodes: in_a_family.iter().map(|p| couples.find(p)).collect(),
            kids: BTreeMap::new(),
            parents: BTreeMap::new(),
            contradictions: Vec::new(),
        };
        for f in families {
            for p in &f.parents {
                for (c, _) in &f.children {
                    let (a, b) = (couples.find(p), couples.find(c));
                    if a == b {
                        q.contradictions.push((p.clone(), c.clone()));
                        continue;
                    }
                    q.kids.entry(a.clone()).or_default().insert(b.clone());
                    q.parents.entry(b).or_default().insert(a);
                }
            }
        }
        q
    }

    fn kids_of(&self, n: &str) -> impl Iterator<Item = &String> {
        self.kids.get(n).into_iter().flatten()
    }

    fn parents_of(&self, n: &str) -> impl Iterator<Item = &String> {
        self.parents.get(n).into_iter().flatten()
    }

    /// Longest path from the roots, by Kahn's algorithm.
    ///
    /// Returns the level of every node and whether any node was left in a
    /// cycle. A cycle cannot be scheduled at all, so those nodes are placed
    /// one below whichever parents *were* scheduled — an arbitrary but
    /// bounded answer, which is the point: the page renders and says so,
    /// instead of looping.
    fn longest_path(&self) -> (BTreeMap<String, i64>, bool) {
        let mut level: BTreeMap<String, i64> = self.nodes.iter().map(|n| (n.clone(), 0)).collect();
        let mut indegree: BTreeMap<&String, usize> = self
            .nodes
            .iter()
            .map(|n| (n, self.parents_of(n).count()))
            .collect();

        let mut queue: VecDeque<&String> = self
            .nodes
            .iter()
            .filter(|n| indegree[*n] == 0)
            .collect::<VecDeque<_>>();

        let mut scheduled = 0usize;
        while let Some(n) = queue.pop_front() {
            scheduled += 1;
            let here = level.get(n).copied().unwrap_or(0);
            for c in self.kids_of(n) {
                let want = here + 1;
                if level.get(c).copied().unwrap_or(0) < want {
                    level.insert(c.clone(), want);
                }
                if let Some(d) = indegree.get_mut(c) {
                    *d -= 1;
                    if *d == 0 {
                        queue.push_back(c);
                    }
                }
            }
        }

        let cyclic = scheduled < self.nodes.len();
        if cyclic {
            for n in &self.nodes {
                if indegree[n] == 0 {
                    continue;
                }
                let deepest = self
                    .parents_of(n)
                    .filter_map(|p| level.get(p).copied())
                    .max()
                    .unwrap_or(-1);
                level.insert(n.clone(), deepest + 1);
            }
        }
        (level, cyclic)
    }
}

/// Slide an ancestral line down until it sits directly above the descendant
/// that anchors it.
///
/// For each node `x` and each of its parents `p` sitting more than one row
/// above it, this finds the set `p` belongs to — everything reachable from `p`
/// both upward and downward without passing through `x` — and moves the whole
/// set down as one rigid block. Two properties make a single sweep enough:
///
/// * the set is closed under both edge directions, so the only edges crossing
///   its boundary end at `x`, and moving it cannot disturb anything else;
/// * the move is refused outright if the set touches a node already at or
///   below `x`'s row, which is exactly the case where the line is anchored by
///   something other than `x` — an intermarriage, or a shared ancestor.
///
/// The distance moved is bounded by the deepest member of the set that is
/// itself a parent of `x`, so no member is ever pushed level with or below the
/// child it parents. Nodes are visited deepest-first so that a line is pulled
/// down to its lowest anchor rather than an intermediate one.
fn lift_free_ancestry(q: &Quotient, level: &mut BTreeMap<String, i64>) {
    let mut order: Vec<&String> = q.nodes.iter().collect();
    order.sort_by_key(|n| (-level.get(*n).copied().unwrap_or(0), (*n).clone()));

    for x in order {
        let anchor = level.get(x).copied().unwrap_or(0);
        for p in q.parents_of(x) {
            if level.get(p).copied().unwrap_or(0) >= anchor - 1 {
                continue; // already sitting directly above
            }
            let Some(block) = free_block(q, level, p, x, anchor) else {
                continue;
            };
            // The deepest member that parents `x` is what sets the distance:
            // moving further would put it level with `x`.
            let Some(hinge) = block
                .iter()
                .filter(|n| q.parents.get(x).is_some_and(|ps| ps.contains(*n)))
                .filter_map(|n| level.get(n).copied())
                .max()
            else {
                continue;
            };
            let delta = anchor - 1 - hinge;
            if delta <= 0 {
                continue;
            }
            for n in &block {
                *level.entry(n.clone()).or_insert(0) += delta;
            }
        }
    }
}

/// Everything reachable from `start` without passing through `anchor`, or
/// `None` when that set reaches a node at or below `ceiling` — the signal that
/// something other than `anchor` already holds the line in place.
fn free_block(
    q: &Quotient,
    level: &BTreeMap<String, i64>,
    start: &str,
    anchor: &str,
    ceiling: i64,
) -> Option<BTreeSet<String>> {
    let mut block: BTreeSet<String> = BTreeSet::new();
    let mut stack: Vec<String> = vec![start.to_string()];
    while let Some(n) = stack.pop() {
        if n == anchor || block.contains(&n) {
            continue;
        }
        if level.get(&n).copied().unwrap_or(0) >= ceiling {
            return None;
        }
        block.insert(n.clone());
        for next in q.parents_of(&n).chain(q.kids_of(&n)) {
            if next != anchor && !block.contains(next) {
                stack.push(next.clone());
            }
        }
    }
    (!block.is_empty()).then_some(block)
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
    /// How wide this box is. A card is `CARD_W`; a person the reader may not
    /// read is a `MARK_W` marker, which is the whole of the second saving.
    pub w: f64,
    /// Which row of its own band this card sits on, counting from the band's
    /// top. Zero for every card in a generation that did not have to wrap.
    pub row: usize,
    /// Confidence band of the birth fact, shown as a dot on the card.
    pub conf_band: Option<&'static str>,
    pub conf_label: Option<String>,
    /// The confidence as a whole percentage, so the card's dot can be filled
    /// to the value rather than merely tinted by its band.
    pub conf_pct: Option<u8>,
    /// True for the person the focused view is centred on.
    pub is_root: bool,
    /// True when this person exists but the reader may not read them. The card
    /// keeps its place and its id — the tree's shape is the same for everyone —
    /// and carries no name, no dates, no gender and no link.
    pub restricted: bool,
}

/// A connector between two cards.
#[derive(Debug, Clone, Serialize)]
pub struct Edge {
    /// `parent` or `spouse`.
    pub kind: &'static str,
    /// SVG path data.
    pub d: String,
    /// Opacity, driven by the relationship's confidence. Encodes *certainty*,
    /// and nothing else — a faint edge is an uncertain claim.
    pub opacity: f64,
    pub band: &'static str,
    pub title: String,
    /// The two person ids the edge connects: for a parent edge the anchoring
    /// parent and the child, for a spouse edge the two partners. Drives the
    /// hover highlight, which raises a person's own edges and dims the rest.
    pub from: String,
    pub to: String,
    /// Which crossing marker this edge carries, 1..=8, set only on the descent
    /// edges that actually cross another. It encodes *crossing*, a different
    /// thing from opacity's certainty, so the eye can follow one line through
    /// an intersection. `None` leaves the edge the default ink — a marker is
    /// only meaningful because it is rare.
    ///
    /// An index rather than a colour, because the stylesheet pairs each index
    /// with a dash pattern as well as a hue: under a colour-blind theme the
    /// dash is what carries the distinction.
    pub hue: Option<u8>,
}

/// One generation, on one row or several.
///
/// A generation wider than the row it is given wraps onto as many rows as it
/// needs, and the band is what keeps those rows one thing. Vertical position
/// still carries generation — every card in a band belongs to the same one —
/// so the band is drawn as a single tinted zone `rows` high, and the reader
/// reads a band, not a row.
#[derive(Debug, Clone, Serialize)]
pub struct Band {
    /// Filled by [`localise`]; empty as the layout leaves it.
    pub label: String,
    pub sublabel: String,
    /// How many people this band holds, so the label can be built later.
    pub count: usize,
    pub generation: Option<i64>,
    pub cards: Vec<Card>,
    pub y: f64,
    /// How many rows the generation wrapped onto. One for a band that fitted.
    pub rows: usize,
    /// Top to bottom, so the zone behind the cards can be drawn as one shape.
    pub height: f64,
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
    /// The offending parent-child pairs, so the banner can name them.
    pub contradictions: Vec<(String, String)>,
}

// ---------------------------------------------------------------------------
// Choosing what to draw
// ---------------------------------------------------------------------------

/// The people around one person, and how they were reached.
///
/// The full tree is laid out correctly but is not usable at the operator's
/// scale: its widest generation holds 161 people, which is a canvas over
/// 23,000px wide. No one scrolls that far to find an ancestor. A generation
/// that size cannot be made legible by shrinking cards either — the fix is to
/// draw fewer people, not smaller ones.
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
    best_root_among(flat, depth, None)
}

/// [`best_root`], restricted to candidates the reader may actually read.
///
/// `only` filters the *candidates*, not the subtree each one would draw: the
/// landing view still shows the hidden people around the root, redacted, so
/// the tree does not change shape according to who is looking. Without this
/// the default root on a real family bundle is almost always someone living —
/// and therefore `members` — so every signed-out visitor would land on a
/// panel reading "Private".
pub fn best_root_among(
    flat: &Value,
    depth: usize,
    only: Option<&BTreeSet<String>>,
) -> Option<String> {
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
        if only.is_some_and(|set| !set.contains(id)) {
            continue;
        }
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
    layout_with(flat, LayoutOpts::default())
}

/// [`layout`], to a stated row width and reader.
pub fn layout_with(flat: &Value, opts: LayoutOpts<'_>) -> TreeLayout {
    layout_subset(flat, None, None, opts)
}

/// Lay out only `subtree`, for the focused default view.
pub fn layout_focused(flat: &Value, subtree: &Subtree) -> TreeLayout {
    layout_focused_with(flat, subtree, LayoutOpts::default())
}

/// [`layout_focused`], to a stated row width and reader.
pub fn layout_focused_with(flat: &Value, subtree: &Subtree, opts: LayoutOpts<'_>) -> TreeLayout {
    layout_subset(flat, Some(&subtree.ids), Some(&subtree.root), opts)
}

// ---------------------------------------------------------------------------
// Within-row ordering: barycentre with crossing minimisation
// ---------------------------------------------------------------------------

/// Hues for the residual crossings. The Okabe–Ito qualitative set: eight
/// colours chosen to stay distinct under the common forms of colour blindness.
/// Small on purpose — colour is a signal here, not decoration, so it has to be
/// rare to mean anything.
/// How many distinct crossing markers exist.
///
/// The marker is an *index*, not a colour. The stylesheet turns each index
/// into both a hue and a dash pattern, which is what lets the colour-blind
/// themes keep the distinction: two lines that look identical in hue are still
/// one dashed and one dotted. Hardcoding hex here would have made that
/// impossible without a second set of numbers in Rust.
const CROSSING_MARKERS: u8 = 8;

/// A unit the ordering permutes as one: a contracted couple, or a lone person.
/// Keeping partners inside a single unit is what guarantees they stay adjacent
/// through every sweep.
type Unit = Vec<String>;

/// Everything the ordering pass needs about the graph, in the terms it works
/// in: rows of units, and the drawn descent edges as (parent, child) pairs.
struct Ordered {
    /// Generation -> people, left to right, after minimisation.
    rows: BTreeMap<i64, Vec<String>>,
    /// Crossings remaining in `rows` — reported, and the test oracle.
    crossings: usize,
    /// The descent edges that still cross something, each given a hue so one
    /// line can be traced through an intersection.
    hues: BTreeMap<(String, String), u8>,
}

/// The anchoring parent of `child`: the parent that sits directly below it and
/// therefore carries the drawn connector. That is the *deepest* parent present
/// (a married-in spouse with shallower ancestry shares the couple's row, so
/// both are equally deep; the tie breaks on id so the choice is deterministic
/// and identical everywhere it is recomputed).
fn anchor_parent<'a>(
    parents: &'a [String],
    gen: &BTreeMap<String, i64>,
    present: &impl Fn(&str) -> bool,
) -> Option<&'a String> {
    parents.iter().filter(|p| present(p)).max_by(|a, b| {
        gen.get(*a)
            .copied()
            .unwrap_or(0)
            .cmp(&gen.get(*b).copied().unwrap_or(0))
            // Equal generation: prefer the smaller id, so pick the one that
            // sorts *greater* under reversed id comparison.
            .then_with(|| b.cmp(a))
    })
}

/// Column index of every person in `rows`, i.e. its left-to-right position
/// within its own row. This is the coordinate barycentres and crossings are
/// both measured in.
fn column_index(rows: &BTreeMap<i64, Vec<String>>) -> BTreeMap<String, usize> {
    let mut col = BTreeMap::new();
    for people in rows.values() {
        for (i, id) in people.iter().enumerate() {
            col.insert(id.clone(), i);
        }
    }
    col
}

/// Count crossings between the drawn descent edges.
///
/// For each pair of adjacent layers it counts *inversions*: two edges whose
/// parents are in one horizontal order while their children are in the
/// opposite one must cross somewhere between the rows. Summed over every
/// adjacent pair, that is the number of intersections on the canvas — the
/// quantity the sweeps try to reduce, and the oracle the tests assert on.
fn count_crossings(
    rows: &BTreeMap<i64, Vec<String>>,
    anchor_of: &BTreeMap<String, String>,
) -> usize {
    let col = column_index(rows);
    let mut total = 0usize;
    // Bucket edges by the child's layer, so each bucket is one adjacent pair.
    let mut by_layer: BTreeMap<i64, Vec<(usize, usize)>> = BTreeMap::new();
    for (&g, people) in rows {
        for child in people {
            let Some(parent) = anchor_of.get(child) else {
                continue;
            };
            let (Some(&pc), Some(&cc)) = (col.get(parent), col.get(child)) else {
                continue;
            };
            by_layer.entry(g).or_default().push((pc, cc));
        }
    }
    for edges in by_layer.values_mut() {
        // Sort by parent column, then child column; the number of crossings is
        // then the number of inversions in the child-column sequence. A Fenwick
        // tree counts them in O(e log e), so the widest generation (161 people)
        // does not turn every sweep into quadratic work.
        edges.sort_unstable();
        let width = rows.values().map(Vec::len).max().unwrap_or(0).max(1);
        let mut seen = vec![0u32; width + 1];
        for (placed, &(_, cc)) in edges.iter().enumerate() {
            // Children already placed that sit to the right of this one cross it.
            total += placed - fenwick_prefix(&seen, cc + 1);
            fenwick_add(&mut seen, cc + 1);
        }
    }
    total
}

/// Fenwick (binary indexed) tree: sum over `[1, i]`, 1-based.
fn fenwick_prefix(tree: &[u32], mut i: usize) -> usize {
    let mut sum = 0usize;
    while i > 0 {
        sum += tree[i] as usize;
        i -= i & i.wrapping_neg();
    }
    sum
}

/// Fenwick (binary indexed) tree: add one at index `i`, 1-based.
fn fenwick_add(tree: &mut [u32], mut i: usize) {
    while i < tree.len() {
        tree[i] += 1;
        i += i & i.wrapping_neg();
    }
}

/// Sort each layer once, by the mean column of its neighbours in the adjacent
/// layer. `neighbours` yields the neighbour ids of a person on the side being
/// swept towards (parents for a downward sweep, children for an upward one).
/// Layers are visited in `gen_order` so the side already fixed this sweep is
/// the side the barycentre reads.
fn sweep(
    rows: &mut BTreeMap<i64, Vec<String>>,
    units_by_gen: &BTreeMap<i64, Vec<Unit>>,
    gen_order: &[i64],
    neighbours: &BTreeMap<String, Vec<String>>,
    birth_key: &impl Fn(&str) -> (i64, String),
) {
    let mut col = column_index(rows);
    for &g in gen_order {
        let Some(units) = units_by_gen.get(&g) else {
            continue;
        };
        let mut order = units.clone();
        let barycentre = |unit: &Unit| -> f64 {
            let cols: Vec<f64> = unit
                .iter()
                .filter_map(|id| neighbours.get(id))
                .flatten()
                .filter_map(|n| col.get(n))
                .map(|c| *c as f64)
                .collect();
            if cols.is_empty() {
                // No neighbour on this side: leave it where it was. f64::MAX
                // sends it to the end, and a stable sort keeps such units in
                // their current relative order.
                f64::MAX
            } else {
                cols.iter().sum::<f64>() / cols.len() as f64
            }
        };
        let unit_key = |unit: &Unit| -> (i64, String) {
            unit.iter()
                .map(|id| birth_key(id))
                .min()
                .unwrap_or((i64::MAX, String::new()))
        };
        order.sort_by(|a, b| {
            barycentre(a)
                .partial_cmp(&barycentre(b))
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| unit_key(a).cmp(&unit_key(b)))
        });
        let people: Vec<String> = order.into_iter().flatten().collect();
        for (i, id) in people.iter().enumerate() {
            col.insert(id.clone(), i);
        }
        rows.insert(g, people);
    }
}

/// Give every descent edge involved in a crossing a hue, so the two lines
/// through an intersection are told apart. This is a graph colouring: build the
/// "these two edges cross" graph and greedily assign each edge the lowest
/// palette colour none of its already-coloured crossing partners uses, which
/// guarantees the pair at any single intersection differ.
fn assign_hues(
    rows: &BTreeMap<i64, Vec<String>>,
    anchor_of: &BTreeMap<String, String>,
) -> BTreeMap<(String, String), u8> {
    // One drawn edge, as (parent column, child column, (parent id, child id)).
    type LayerEdge = (usize, usize, (String, String));
    let col = column_index(rows);
    // Edges, bucketed by layer as in `count_crossings`.
    let mut by_layer: BTreeMap<i64, Vec<LayerEdge>> = BTreeMap::new();
    for (&g, people) in rows {
        for child in people {
            let Some(parent) = anchor_of.get(child) else {
                continue;
            };
            let (Some(&pc), Some(&cc)) = (col.get(parent), col.get(child)) else {
                continue;
            };
            by_layer
                .entry(g)
                .or_default()
                .push((pc, cc, (parent.clone(), child.clone())));
        }
    }

    // Which edges cross which. Keyed by (parent, child), deterministically.
    let mut crosses: BTreeMap<(String, String), BTreeSet<(String, String)>> = BTreeMap::new();
    for edges in by_layer.values() {
        for i in 0..edges.len() {
            for j in (i + 1)..edges.len() {
                let (a, b) = (&edges[i], &edges[j]);
                // A crossing is a pair ordered one way by parent and the other
                // by child.
                let inverted = (a.0 < b.0 && a.1 > b.1) || (a.0 > b.0 && a.1 < b.1);
                if inverted {
                    crosses.entry(a.2.clone()).or_default().insert(b.2.clone());
                    crosses.entry(b.2.clone()).or_default().insert(a.2.clone());
                }
            }
        }
    }

    let mut hues: BTreeMap<(String, String), u8> = BTreeMap::new();
    for edge in crosses.keys() {
        let used: BTreeSet<u8> = crosses[edge]
            .iter()
            .filter_map(|other| hues.get(other).copied())
            .collect();
        let marker = (1..=CROSSING_MARKERS)
            .find(|m| !used.contains(m))
            .unwrap_or(1);
        hues.insert(edge.clone(), marker);
    }
    hues
}

/// Order the people of every generation to minimise descent-edge crossings.
///
/// The layer assignment is a given (the union-contracted longest-path pass);
/// this permutes *within* each layer. It contracts each couple into one unit so
/// spouses cannot be separated, seeds a deterministic order (birth date, then
/// id), then alternates downward and upward barycentre sweeps — order by the
/// mean position of parents, then of children — keeping the best ordering seen
/// rather than the last, because the crossing count can oscillate. Ten sweeps
/// is the cap; in practice it settles in two or three.
fn compute_ordering(
    persons: &serde_json::Map<String, Value>,
    families: &[FamilyEdges],
    gen: &BTreeMap<String, i64>,
    only: Option<&BTreeSet<String>>,
) -> Ordered {
    let wanted = |id: &String| only.is_none_or(|set| set.contains(id));

    // Drawn people, grouped by layer.
    let mut by_gen: BTreeMap<i64, Vec<String>> = BTreeMap::new();
    for (id, g) in gen {
        if persons.contains_key(id) && wanted(id) {
            by_gen.entry(*g).or_default().push(id.clone());
        }
    }
    let present = |id: &str| {
        gen.contains_key(id) && persons.contains_key(id) && only.is_none_or(|s| s.contains(id))
    };

    // Birth-date sort key, for a stable tie-break. A missing or unparseable
    // year sorts last, so dated people anchor the order.
    let birth_key = |id: &str| -> (i64, String) {
        let year = persons
            .get(id)
            .and_then(|p| p.get("birth"))
            .and_then(|b| b.get("date"))
            .and_then(|d| d.get("value"))
            .and_then(Value::as_str)
            .and_then(parse_year)
            .unwrap_or(i64::MAX);
        (year, id.to_string())
    };

    // Partners, for grouping into units. A person who partners in several
    // families joins the first unit encountered; a single row cannot split
    // them across units anyway.
    let mut partners: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for f in families {
        for a in &f.parents {
            for b in &f.parents {
                if a != b {
                    partners.entry(a.clone()).or_default().push(b.clone());
                }
            }
        }
    }

    // The drawn descent edges, and both neighbour maps derived from them, so
    // barycentres and crossings measure exactly the connectors on screen.
    let mut anchor_of: BTreeMap<String, String> = BTreeMap::new();
    let mut parents_side: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut children_side: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for f in families {
        for (child, _) in &f.children {
            if !present(child) {
                continue;
            }
            let Some(parent) = anchor_parent(&f.parents, gen, &present) else {
                continue;
            };
            anchor_of.insert(child.clone(), parent.clone());
            parents_side
                .entry(child.clone())
                .or_default()
                .push(parent.clone());
            children_side
                .entry(parent.clone())
                .or_default()
                .push(child.clone());
        }
    }

    // Build the units of each layer, in a deterministic seed order.
    let mut units_by_gen: BTreeMap<i64, Vec<Unit>> = BTreeMap::new();
    for (&g, row) in &by_gen {
        let members: BTreeSet<&String> = row.iter().collect();
        let mut seed = row.clone();
        seed.sort_by_key(|a| birth_key(a));
        let mut seen: BTreeSet<String> = BTreeSet::new();
        let mut units: Vec<Unit> = Vec::new();
        for id in &seed {
            if seen.contains(id) {
                continue;
            }
            let mut unit = vec![id.clone()];
            seen.insert(id.clone());
            if let Some(ps) = partners.get(id) {
                for p in ps {
                    if members.contains(p) && seen.insert(p.clone()) {
                        unit.push(p.clone());
                    }
                }
            }
            // Deterministic order within a couple.
            unit.sort_by_key(|a| birth_key(a));
            units.push(unit);
        }
        units_by_gen.insert(g, units);
    }

    // Seed rows from the units in their deterministic order.
    let mut rows: BTreeMap<i64, Vec<String>> = units_by_gen
        .iter()
        .map(|(&g, units)| (g, units.iter().flatten().cloned().collect()))
        .collect();

    let gens_up: Vec<i64> = rows.keys().copied().collect();
    let gens_down: Vec<i64> = gens_up.iter().rev().copied().collect();

    let mut best = rows.clone();
    let mut best_c = count_crossings(&rows, &anchor_of);
    let mut stale = 0;
    for it in 0..10 {
        if it % 2 == 0 {
            // Downward: order each layer by the mean position of its parents.
            sweep(
                &mut rows,
                &units_by_gen,
                &gens_up,
                &parents_side,
                &birth_key,
            );
        } else {
            // Upward: order each layer by the mean position of its children.
            sweep(
                &mut rows,
                &units_by_gen,
                &gens_down,
                &children_side,
                &birth_key,
            );
        }
        let c = count_crossings(&rows, &anchor_of);
        if c < best_c {
            best = rows.clone();
            best_c = c;
            stale = 0;
        } else {
            stale += 1;
        }
        if best_c == 0 || stale >= 2 {
            break;
        }
    }

    let hues = assign_hues(&best, &anchor_of);
    Ordered {
        rows: best,
        crossings: best_c,
        hues,
    }
}

/// The leading four-digit year of a date value like `1923`, `1923-04-12` or
/// `-0044-03-15`, for ordering only.
fn parse_year(value: &str) -> Option<i64> {
    let neg = value.starts_with('-');
    let digits: String = value
        .trim_start_matches('-')
        .chars()
        .take_while(char::is_ascii_digit)
        .take(4)
        .collect();
    if digits.is_empty() {
        return None;
    }
    let y: i64 = digits.parse().ok()?;
    Some(if neg { -y } else { y })
}

/// Crossing counts before and after minimisation, for reporting. "Before" is
/// the deterministic seed order (couples grouped, birth-date sorted, no
/// sweeps); "after" is the minimised ordering.
pub fn crossings_before_after(flat: &Value) -> (usize, usize) {
    let families = family_edges(flat);
    let empty = serde_json::Map::new();
    let persons = flat
        .get("persons")
        .and_then(Value::as_object)
        .unwrap_or(&empty);
    let gen = assign_generations(flat).gen;

    // "Before": seed order only — build the ordering but read the crossing
    // count of the seed rather than the swept result.
    let mut anchor_of: BTreeMap<String, String> = BTreeMap::new();
    let present = |id: &str| gen.contains_key(id) && persons.contains_key(id);
    for f in &families {
        for (child, _) in &f.children {
            if !present(child) {
                continue;
            }
            if let Some(parent) = anchor_parent(&f.parents, &gen, &present) {
                anchor_of.insert(child.clone(), parent.clone());
            }
        }
    }
    let birth_key = |id: &str| -> (i64, String) {
        let year = persons
            .get(id)
            .and_then(|p| p.get("birth"))
            .and_then(|b| b.get("date"))
            .and_then(|d| d.get("value"))
            .and_then(Value::as_str)
            .and_then(parse_year)
            .unwrap_or(i64::MAX);
        (year, id.to_string())
    };
    let mut seed: BTreeMap<i64, Vec<String>> = BTreeMap::new();
    for (id, g) in &gen {
        if persons.contains_key(id) {
            seed.entry(*g).or_default().push(id.clone());
        }
    }
    for row in seed.values_mut() {
        row.sort_by_key(|a| birth_key(a));
    }
    let before = count_crossings(&seed, &anchor_of);

    let after = compute_ordering(persons, &families, &gen, None).crossings;
    (before, after)
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
    opts: LayoutOpts<'_>,
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

    // Order within each row to minimise crossings.
    //
    // Two things make a row readable. Partners must sit next to each other, or
    // their connector stretches across a canvas that is tens of thousands of
    // pixels wide — the operator's widest generation is 161 people. And
    // children must sit under their parents, or the descent connectors cross
    // into a thicket.
    //
    // Both fall out of barycentre ordering ([`compute_ordering`]): couples are
    // contracted into units that move together, and the units are ordered by
    // the mean position of their neighbours in the adjacent layer, sweeping
    // down then up until the crossing count stops falling. It replaces the
    // single downward pass that left the operator's parents' edges crossed.
    let families = family_edges(flat);
    let ordering = compute_ordering(persons, &families, &generations.gen, only);
    for (g, people) in &ordering.rows {
        by_gen.insert(*g, people.clone());
    }
    let max_gen = by_gen.keys().copied().max().unwrap_or(0);
    let name_of = |id: &str| -> String {
        persons
            .get(id)
            .map(view::person_display_name)
            .unwrap_or_else(|| "[Unknown]".into())
    };

    // Unplaced people appear in no family, so a subtree walk can never reach
    // one — unless the root *is* unplaced, which is exactly the case this
    // filter keeps working.
    let unplaced: Vec<String> = generations
        .unplaced
        .iter()
        .filter(|id| wanted(id))
        .cloned()
        .collect();

    // Partners, so a couple is never split across a row break.
    let mut partners: BTreeSet<(String, String)> = BTreeSet::new();
    for f in &families {
        for pair in f.parents.windows(2) {
            partners.insert((pair[0].clone(), pair[1].clone()));
            partners.insert((pair[1].clone(), pair[0].clone()));
        }
    }

    // Wrap first, then size the canvas to what the wrapping produced.
    //
    // Two lanes are kept clear inside the margins, one at each side, for edges
    // that have to get past their own generation's other rows. They are part
    // of the canvas rather than of any band, so a lane is in the same place
    // whichever band an edge is leaving.
    let content_w = (opts.wrap_w - 2.0 * (MARGIN + LANE)).max(CARD_W);

    let mut unplaced_sorted = unplaced.clone();
    unplaced_sorted.sort_by_key(|id| (name_of(id), id.clone()));

    let mut plan: Vec<(Option<i64>, Vec<Vec<String>>)> = Vec::new();
    if !unplaced_sorted.is_empty() {
        plan.push((
            None,
            wrap_rows(&unplaced_sorted, &opts, &partners, content_w),
        ));
    }
    for g in (0..=max_gen).rev() {
        let Some(ids) = by_gen.get(&g) else { continue };
        if ids.is_empty() {
            continue;
        }
        plan.push((Some(g), wrap_rows(ids, &opts, &partners, content_w)));
    }

    let widest_row = plan
        .iter()
        .flat_map(|(_, rows)| rows.iter())
        .map(|r| row_width(r, &opts))
        .fold(0.0f64, f64::max);
    let width = (widest_row + 2.0 * (MARGIN + LANE)).max(320.0);

    // DOM order: unplaced band first (top), then youngest generation down to
    // the oldest.
    let mut bands: Vec<Band> = Vec::new();
    let mut geom: Vec<BandGeom> = Vec::new();
    let mut row_y = MARGIN;

    for (generation, rows) in &plan {
        let band_widest = rows
            .iter()
            .map(|r| row_width(r, &opts))
            .fold(0.0f64, f64::max);
        let x0 = MARGIN + LANE + (widest_row - band_widest) / 2.0;
        let cards = place_band(rows, persons, &opts, row_y, x0, root);
        let n_rows = rows.len().max(1);
        let height = (n_rows as f64) * CARD_H + (n_rows as f64 - 1.0) * ROW_GAP;

        let mut row_spans: Vec<(f64, Vec<(f64, f64)>)> = Vec::new();
        for r in 0..n_rows {
            let y = row_y + (r as f64) * (CARD_H + ROW_GAP);
            let spans = cards
                .iter()
                .filter(|c| c.row == r)
                .map(|c| (c.x, c.x + c.w))
                .collect();
            row_spans.push((y, spans));
        }
        geom.push(BandGeom {
            top: row_y,
            bottom: row_y + height,
            rows: row_spans,
        });

        bands.push(Band {
            // Left blank; `localise` fills both in the reader's language.
            // The layout itself stays language-neutral so that geometry — what
            // the tests and the cache care about — cannot vary with a header.
            label: String::new(),
            sublabel: String::new(),
            count: rows.iter().map(Vec::len).sum(),
            generation: *generation,
            cards,
            y: row_y,
            rows: n_rows,
            height,
            unplaced: generation.is_none(),
        });
        row_y += height + V_GAP;
    }

    let mut placed: BTreeMap<String, Placement> = BTreeMap::new();
    for (b, band) in bands.iter().enumerate() {
        for c in &band.cards {
            placed.insert(
                c.id.clone(),
                Placement {
                    x: c.x,
                    y: c.y,
                    w: c.w,
                    band: b,
                    row: c.row,
                },
            );
        }
    }

    let height = row_y - V_GAP + MARGIN;
    // Connectors are derived from `placed`, which only holds the people that
    // were drawn, so an edge to someone outside the subtree is dropped rather
    // than dangling off the canvas.
    let edges = build_edges(
        &families,
        &placed,
        &geom,
        width,
        &generations.gen,
        persons,
        &ordering.hues,
    );

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
        contradictions: generations.contradictions.clone(),
    }
}

/// Break one generation into rows no wider than `content_w`.
///
/// Greedy left to right, which is what keeps the ordering pass's work: the ids
/// arrive already sorted by barycentre, so filling rows in that order puts the
/// people whose children sit at the left of the generation below on the
/// earlier row, at the left. Reading the band is reading its rows top to
/// bottom, left to right, which is the same sequence the single wide row had.
///
/// A couple is never split across a break. Partners are adjacent in the
/// ordering because the ordering contracts them into one unit, and separating
/// them here would undo that and leave their connector bowing between rows.
fn wrap_rows(
    ids: &[String],
    opts: &LayoutOpts<'_>,
    partners: &BTreeSet<(String, String)>,
    content_w: f64,
) -> Vec<Vec<String>> {
    let mut rows: Vec<Vec<String>> = Vec::new();
    let mut row: Vec<String> = Vec::new();
    let mut used = 0.0f64;

    for (i, id) in ids.iter().enumerate() {
        let w = opts.box_w(id);
        let need = if row.is_empty() { w } else { used + H_GAP + w };
        if !row.is_empty() && need > content_w {
            // Breaking here would separate this person from the partner just
            // before them; move that partner down too, unless doing so would
            // empty the row.
            // …unless the two of them do not fit a row on their own, which on
            // a phone is any two cards at all. A couple kept together off the
            // side of the screen is not kept together.
            let prev = &row[row.len() - 1];
            let pair_fits = opts.box_w(prev) + H_GAP + w <= content_w;
            let splits_couple =
                row.len() > 1 && pair_fits && partners.contains(&(prev.clone(), id.clone()));
            let carried = if splits_couple { row.pop() } else { None };
            rows.push(std::mem::take(&mut row));
            if let Some(c) = carried {
                used = opts.box_w(&c);
                row.push(c);
                row.push(id.clone());
                used += H_GAP + w;
                continue;
            }
            used = w;
            row.push(id.clone());
            continue;
        }
        used = need;
        row.push(id.clone());
        let _ = i;
    }
    if !row.is_empty() {
        rows.push(row);
    }
    if rows.is_empty() {
        rows.push(Vec::new());
    }

    // Too many rows to still read as one generation, and enough people in them
    // for the connectors to matter: fold it into `MAX_BAND_ROWS` of equal
    // length instead, and let the canvas be wider than the target. A smaller
    // generation folds as far as it needs to, down to one card a row.
    if ids.len() > THICKET && rows.len() > MAX_BAND_ROWS {
        let per = ids.len().div_ceil(MAX_BAND_ROWS);
        rows = ids.chunks(per).map(<[String]>::to_vec).collect();
        // Equal chunks land wherever they land, so mend the same seam the
        // greedy pass mends: a couple across a break moves down together.
        for i in 0..rows.len().saturating_sub(1) {
            let split = match (rows[i].last(), rows[i + 1].first()) {
                (Some(a), Some(b)) => {
                    rows[i].len() > 1 && partners.contains(&(a.clone(), b.clone()))
                }
                _ => false,
            };
            if split {
                let carried = rows[i].pop().expect("checked non-empty");
                rows[i + 1].insert(0, carried);
            }
        }
    }
    rows
}

/// Width of one row of boxes.
fn row_width(ids: &[String], opts: &LayoutOpts<'_>) -> f64 {
    if ids.is_empty() {
        return 0.0;
    }
    ids.iter().map(|id| opts.box_w(id)).sum::<f64>() + (ids.len() as f64 - 1.0) * H_GAP
}

/// Position one band's rows, all sharing a left edge.
///
/// Every row of a band starts at the same x. That is what makes the gaps
/// between cards line up vertically down the whole band, and a gap that lines
/// up is a lane an edge can climb without crossing a card — see [`free_lane`].
/// The band as a block is centred on the canvas; its rows are ragged on the
/// right, the way a paragraph is.
fn place_band(
    rows: &[Vec<String>],
    persons: &serde_json::Map<String, Value>,
    opts: &LayoutOpts<'_>,
    y_top: f64,
    x0: f64,
    root: Option<&str>,
) -> Vec<Card> {
    let mut cards = Vec::new();
    for (r, ids) in rows.iter().enumerate() {
        let y = y_top + (r as f64) * (CARD_H + ROW_GAP);
        let mut x = x0;
        for id in ids {
            let w = opts.box_w(id);
            let mut card = card_for(id, persons.get(id), x, y, root == Some(id.as_str()));
            card.w = w;
            card.row = r;
            card.restricted = opts.hidden(id);
            cards.push(card);
            x += w + H_GAP;
        }
    }
    cards
}

/// Where one person ended up, and in which row of which band.
#[derive(Debug, Clone, Copy)]
struct Placement {
    x: f64,
    y: f64,
    w: f64,
    band: usize,
    row: usize,
}

/// The horizontal extent of every box in a band, row by row.
///
/// Used to find a lane: a vertical strip an edge can travel through without
/// crossing a card.
#[derive(Debug, Clone)]
struct BandGeom {
    /// Top of the band's first row.
    top: f64,
    /// Bottom of the band's last row.
    bottom: f64,
    /// Per row: its top y, and the occupied x spans left to right.
    rows: Vec<(f64, Vec<(f64, f64)>)>,
}

impl BandGeom {
    fn row_top(&self, r: usize) -> f64 {
        self.rows[r].0
    }

    /// Is `x` clear of every box in rows `range`?
    ///
    /// Spans arrive sorted left to right — they are built in placement order —
    /// so the row is searched rather than scanned. On the operator's widest
    /// generation this runs about a hundred and fifty thousand times per
    /// render, which is the difference between the search being free and being
    /// the most expensive thing on the page.
    fn clear(&self, range: std::ops::Range<usize>, x: f64) -> bool {
        const CLEARANCE: f64 = 3.0;
        for r in range {
            let Some((_, spans)) = self.rows.get(r) else {
                continue;
            };
            // The last span whose left edge is at or before x.
            let i = spans.partition_point(|(a, _)| a - CLEARANCE <= x);
            if i > 0 {
                let (a, b) = spans[i - 1];
                if x > a - CLEARANCE && x < b + CLEARANCE {
                    return false;
                }
            }
        }
        true
    }

    /// A vertical strip through rows `range` with no card in it, as near to
    /// `prefer` as one can be found.
    ///
    /// Candidates are the gaps between cards — which line up down the band,
    /// because every row shares a left edge — and, always, the two lanes kept
    /// clear at the canvas edges. The edge lanes are the reason this cannot
    /// fail: if a band's rows leave no gap in common, the edge goes around the
    /// band rather than through it.
    fn free_lane(&self, range: std::ops::Range<usize>, prefer: f64, canvas_w: f64) -> f64 {
        let mut candidates: Vec<f64> = vec![MARGIN + LANE / 2.0, canvas_w - MARGIN - LANE / 2.0];
        for r in range.clone() {
            let Some((_, spans)) = self.rows.get(r) else {
                continue;
            };
            for pair in spans.windows(2) {
                candidates.push((pair[0].1 + pair[1].0) / 2.0);
            }
            if let Some(last) = spans.last() {
                candidates.push(last.1 + H_GAP / 2.0);
            }
            if let Some(first) = spans.first() {
                candidates.push(first.0 - H_GAP / 2.0);
            }
        }
        // Rows of a band share a left edge, so most of these candidates are
        // the same x proposed by several rows. Collapsing them first is what
        // keeps the search below proportional to the width of a row rather
        // than to the area of the band.
        candidates.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        candidates.dedup_by(|a, b| (*a - *b).abs() < 1.0);
        candidates.sort_by(|a, b| {
            (a - prefer)
                .abs()
                .partial_cmp(&(b - prefer).abs())
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        candidates
            .into_iter()
            .find(|x| self.clear(range.clone(), *x))
            .unwrap_or(MARGIN + LANE / 2.0)
    }
}

/// Emit an orthogonal path through `points` as SVG path data.
///
/// Every segment is axis-aligned, so `V` and `H` say it in a third of the
/// characters `L` would take. On `?all=1` that is a thousand paths, and the
/// page is already a megabyte.
fn ortho_path(points: &[(f64, f64)]) -> String {
    let mut out = String::new();
    let (mut cx, mut cy) = points[0];
    out.push_str(&format!("M {cx:.1} {cy:.1}"));
    for &(x, y) in &points[1..] {
        if (x - cx).abs() > 0.05 {
            out.push_str(&format!(" H {x:.1}"));
            cx = x;
        }
        if (y - cy).abs() > 0.05 {
            out.push_str(&format!(" V {y:.1}"));
            cy = y;
        }
    }
    out
}

/// Write each band's label in the reader's language.
///
/// Separate from the layout because a band label is interface text and the
/// layout is geometry. Keeping them apart means the expensive part does not
/// vary with an `Accept-Language` header, and the tests that assert on
/// positions do not have to care what language they are running in.
pub fn localise(layout: &mut TreeLayout, lang: &str) {
    let n_arg =
        |n: usize| fluent::FluentArgs::from_iter([("n", fluent::FluentValue::from(n as i64))]);
    for band in &mut layout.bands {
        if band.unplaced {
            band.label = crate::i18n::translate(lang, "tree-band-unplaced", None);
            band.sublabel =
                crate::i18n::translate(lang, "tree-band-unplaced-note", Some(&n_arg(band.count)));
        } else {
            let g = band.generation.unwrap_or(0);
            let args = fluent::FluentArgs::from_iter([("g", fluent::FluentValue::from(g))]);
            band.label = crate::i18n::translate(lang, "tree-band-generation", Some(&args));
            band.sublabel =
                crate::i18n::translate(lang, "tree-band-people", Some(&n_arg(band.count)));
        }
    }
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
            w: CARD_W,
            row: 0,
            conf_band: None,
            conf_label: None,
            conf_pct: None,
            is_root,
            restricted: false,
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
        w: CARD_W,
        row: 0,
        conf_band: conf.as_ref().map(|c| c.band),
        conf_pct: conf.as_ref().map(|c| c.percent),
        conf_label: conf.map(|c| c.description),
        is_root,
        restricted: false,
    }
}

/// Blank everything about the people `visible` does not admit.
///
/// Run *after* layout rather than folded into it, and that is the point: the
/// geometry is identical whether or not the reader may read the names. A tree
/// that changed shape according to who was looking would leak the very thing
/// this is meant to withhold — the difference between two layouts is a signal,
/// and a reader who could see both would read the hidden people out of it.
/// Here the only difference between an admin's tree and an anonymous
/// visitor's is which cards carry text.
///
/// `None` admits everyone and costs nothing, which is the common case: a
/// bundle with nothing hidden, or an admin.
pub fn redact(layout: &mut TreeLayout, visible: Option<&BTreeSet<String>>) {
    redact_in(layout, visible, crate::i18n::DEFAULT)
}

/// [`redact`], labelling the withheld cards in the reader's language.
pub fn redact_in(layout: &mut TreeLayout, visible: Option<&BTreeSet<String>>, lang: &str) {
    let Some(visible) = visible else { return };
    let mut hidden: BTreeSet<&str> = BTreeSet::new();
    for band in &mut layout.bands {
        for card in &mut band.cards {
            if visible.contains(&card.id) {
                continue;
            }
            hidden.insert(card.id.as_str());
            card.name = crate::i18n::translate(lang, crate::person::RESTRICTED_KEY, None);
            // Emptied, not redacted-in-place: `search` drives the client-side
            // filter, and a filter that still matched the real name would hand
            // back every name in the bundle one keystroke at a time.
            card.search = String::new();
            card.birth = String::new();
            card.death = String::new();
            // "a"/"b" encode recorded sex on the card; "u" is the same thing
            // an unrecorded gender gets, so a redacted card is not
            // distinguishable from an incomplete one.
            card.sex = "u";
            // The confidence of a birth fact is a statement about a fact the
            // reader may not see.
            card.conf_band = None;
            card.conf_label = None;
            card.conf_pct = None;
            card.restricted = true;
        }
    }
    if hidden.is_empty() {
        return;
    }
    // An edge title names both ends. The edge itself stays — it is the shape,
    // and the shape is not what is being withheld — but it stops saying who.
    for edge in &mut layout.edges {
        let a = hidden.contains(edge.from.as_str());
        let b = hidden.contains(edge.to.as_str());
        if !a && !b {
            continue;
        }
        edge.title = crate::i18n::translate(
            lang,
            match edge.kind {
                "spouse" => "tree-edge-union",
                _ => "tree-edge-parentage",
            },
            None,
        );
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
    pos: &BTreeMap<String, Placement>,
    geom: &[BandGeom],
    canvas_w: f64,
    gen: &BTreeMap<String, i64>,
    persons: &serde_json::Map<String, Value>,
    hues: &BTreeMap<(String, String), u8>,
) -> Vec<Edge> {
    let mut edges = Vec::new();
    let name = |id: &str| -> String {
        persons
            .get(id)
            .map(view::person_display_name)
            .unwrap_or_else(|| "[Unknown]".into())
    };
    let present = |id: &str| pos.contains_key(id);

    for f in families {
        // Spouse connector: a horizontal line between partners in the same row.
        if f.parents.len() >= 2 {
            for pair in f.parents.windows(2) {
                let (Some(&a), Some(&b)) = (pos.get(&pair[0]), pos.get(&pair[1])) else {
                    continue;
                };
                let (ax, ay, bx, by) = (a.x, a.y, b.x, b.y);
                let c = Confidence::new(f.union_confidence.unwrap_or(0.8));
                let (x1, x2) = if ax <= bx {
                    (ax + a.w, bx)
                } else {
                    (bx + b.w, ax)
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
                    from: pair[0].clone(),
                    to: pair[1].clone(),
                    hue: None,
                });
            }
        }

        // Parent-to-child connectors. The parent sits *below* the child in this
        // inverted layout, so the line leaves the parent's top edge and arrives
        // at the child's bottom edge.
        for (child, conf) in &f.children {
            let Some(&cp) = pos.get(child) else {
                continue;
            };
            // Anchor on the deepest parent — the same choice the ordering pass
            // made, so the coloured crossings line up with the drawn ones.
            let Some(parent) = anchor_parent(&f.parents, gen, &present) else {
                continue;
            };
            let Some(&pp) = pos.get(parent) else {
                continue;
            };

            let c = Confidence::new(conf.unwrap_or(0.8));
            let x1 = pp.x + pp.w / 2.0;
            let y1 = pp.y; // parent's top edge
            let x2 = cp.x + cp.w / 2.0;
            let y2 = cp.y + CARD_H; // child's bottom edge

            // The parent is below the child, so the line leaves the parent's
            // band through its top and enters the child's band through its
            // bottom. Where a generation wrapped, the rows in between are in
            // the way, and the line climbs a lane rather than crossing them.
            let (pg, cg) = (geom.get(pp.band), geom.get(cp.band));
            let (band_top, band_bottom) = match (pg, cg) {
                (Some(a), Some(b)) => (a.top, b.bottom),
                _ => (y1, y2),
            };
            let mid = (band_top + band_bottom) / 2.0;

            let mut points: Vec<(f64, f64)> = vec![(x1, y1)];

            // Out of the parent's band. Rows 0..pp.row sit above the parent.
            let exit_x = if pp.row > 0 {
                if let Some(g) = pg {
                    // The strip directly above the parent's row holds no card.
                    let lane_y = g.row_top(pp.row) - ROW_GAP / 2.0;
                    let lane_x = g.free_lane(0..pp.row, x1, canvas_w);
                    points.push((x1, lane_y));
                    points.push((lane_x, lane_y));
                    lane_x
                } else {
                    x1
                }
            } else {
                x1
            };

            // Across the gap between the two generations.
            points.push((exit_x, mid));

            // Into the child's band. Rows cp.row+1.. sit below the child.
            let last = cg.map(|g| g.rows.len()).unwrap_or(1);
            if cp.row + 1 < last {
                if let Some(g) = cg {
                    let lane_x = g.free_lane(cp.row + 1..last, x2, canvas_w);
                    let lane_y = g.row_top(cp.row) + CARD_H + ROW_GAP / 2.0;
                    points.push((lane_x, mid));
                    points.push((lane_x, lane_y));
                    points.push((x2, lane_y));
                }
            } else {
                points.push((x2, mid));
            }
            points.push((x2, y2));
            let d = ortho_path(&points);

            edges.push(Edge {
                kind: "parent",
                d,
                opacity: opacity_for(&c),
                band: c.band,
                title: format!("{} → {} — {}", name(parent), name(child), c.description),
                from: parent.clone(),
                to: child.clone(),
                hue: hues.get(&(parent.clone(), child.clone())).copied(),
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

    // -- spouse levelling --------------------------------------------------

    #[test]
    fn two_spouses_at_different_depths_converge_on_one_generation() {
        // This is the operator's own case in miniature: "b" has one recorded
        // parent and lands at 1, "d" descends from a longer line and lands at
        // 2. They are married, so they belong in the same row — and it is the
        // deeper number that survives, because that side's ancestors are all
        // occupying rows of their own above it.
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
        assert_eq!(g(&gens, "b"), g(&gens, "d"), "spouses share a generation");
        assert_eq!(g(&gens, "b"), 2, "the deeper ancestry sets the row");
        assert!(!gens.truncated, "nothing here is contradictory");
    }

    #[test]
    fn a_relevelled_spouses_parents_shift_with_them() {
        // "b" moved from 1 to 2, so the parents whose only descendant is "b"
        // move too. Leaving them at 0 would draw a two-row connector where the
        // record says "mother and daughter".
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
        assert_eq!(g(&gens, "p1"), 1, "the mother followed her daughter down");
        assert_eq!(g(&gens, "p2"), 1);
        assert_eq!(
            g(&gens, "b") - g(&gens, "p1"),
            1,
            "and lands exactly one row above her"
        );
        // The line that anchored the couple is untouched.
        assert_eq!(g(&gens, "g1"), 0);
        assert_eq!(g(&gens, "c"), 1);
    }

    #[test]
    fn a_relevelled_spouses_children_stay_below_them() {
        // "b" has a child from an earlier union who would otherwise sit at
        // generation 2 — level with the re-levelled "b" itself.
        let bd = bundle(
            &["p1", "b", "g1", "c", "d", "kid", "elder"],
            &[
                (&["p1"], &["b"]),
                (&["g1"], &["c"]),
                (&["c"], &["d"]),
                (&["b", "d"], &["kid"]),
                (&["b"], &["elder"]),
            ],
        );
        let gens = assign_generations(&bd);
        assert_eq!(g(&gens, "b"), 2);
        assert!(
            g(&gens, "elder") > g(&gens, "b"),
            "a child of a re-levelled parent must be pushed below them, got \
             elder={} b={}",
            g(&gens, "elder"),
            g(&gens, "b")
        );
        assert!(g(&gens, "kid") > g(&gens, "b"));
    }

    #[test]
    fn a_person_with_no_spouse_is_left_where_their_ancestry_puts_them() {
        // "solo" parents a child alone, in a bundle where levelling is
        // happening elsewhere. Nothing about that may move them.
        let b = bundle(
            &["gp", "solo", "child", "x", "y", "z", "w"],
            &[
                (&["gp"], &["solo"]),
                (&["solo"], &["child"]),
                (&["x"], &["y"]),
                (&["y", "z"], &["w"]),
            ],
        );
        let gens = assign_generations(&b);
        assert_eq!(g(&gens, "gp"), 0);
        assert_eq!(g(&gens, "solo"), 1);
        assert_eq!(g(&gens, "child"), 2);
    }

    #[test]
    fn a_union_between_two_people_on_one_line_of_descent_does_not_hang() {
        // The operator's file contains two of these — a father and son written
        // into a union by the GEDCOM converter. Levelling them would put a
        // person level with their own parent, so the parent-child edge is
        // dropped, reported, and the page still renders.
        let b = bundle(
            &["gp", "dad", "son", "mum"],
            &[
                (&["gp"], &["dad"]),
                (&["dad", "mum"], &["son"]),
                (&["dad", "son"], &[]),
            ],
        );
        let gens = assign_generations(&b);
        for id in ["gp", "dad", "son", "mum"] {
            assert!(gens.gen.contains_key(id), "{id} vanished");
        }
        assert!(
            gens.truncated,
            "a union along a line of descent cannot be honoured and must say so"
        );
        // And the layout still draws everyone.
        let l = layout(&b);
        assert_eq!(l.person_count, 4);
    }

    #[test]
    fn a_marriage_cycle_terminates_rather_than_looping() {
        // Two people who are each other's ancestor, and married as well: every
        // constraint contradicts another. The pass must return.
        let b = bundle(
            &["a", "z"],
            &[(&["a"], &["z"]), (&["z"], &["a"]), (&["a", "z"], &[])],
        );
        let gens = assign_generations(&b);
        assert!(gens.gen.contains_key("a") && gens.gen.contains_key("z"));
        assert!(gens.truncated);
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

        // The 60-card row is no longer 8,794px of canvas. Sixty is over
        // `THICKET`, so it folds to `MAX_BAND_ROWS` rows rather than as far as
        // the target width would take it — ten cards a row, not seven — and
        // the canvas is wider than the target by exactly that much. Still a
        // fifth of what one row cost.
        assert!(
            full.width < 8794.0 / 4.0,
            "wrapping must collapse the canvas: {}",
            full.width
        );
        let kids = full.bands.iter().find(|b| b.count == 60).unwrap();
        assert_eq!(
            kids.rows, MAX_BAND_ROWS,
            "a generation this size stops at the row cap"
        );
        assert!(
            full.height > 2.0 * (CARD_H + V_GAP),
            "…by becoming taller, which is where the width went: {}",
            full.height
        );
        // Focusing still narrows it further — it draws three people, and three
        // people need one short row.
        assert!(
            focused.width < full.width / 2.0,
            "focusing must still collapse the canvas: {} vs {}",
            focused.width,
            full.width
        );
    }

    #[test]
    fn a_wide_generation_wraps_instead_of_widening_the_canvas() {
        // 60 siblings in one generation, laid out to a row that holds seven.
        let mut people: Vec<String> = vec!["ma".into(), "pa".into()];
        for i in 0..60 {
            people.push(format!("kid{i:02}"));
        }
        let refs: Vec<&str> = people.iter().map(String::as_str).collect();
        let kids: Vec<&str> = refs[2..].to_vec();
        let b = bundle(&refs, &[(&["ma", "pa"], &kids)]);

        let narrow = layout_with(&b, LayoutOpts::default().with_width(1200.0));
        let wide = layout_with(&b, LayoutOpts::default().with_width(4000.0));

        // Same people either way; only the shape of the canvas differs.
        assert_eq!(narrow.person_count, wide.person_count);
        assert!(
            narrow.width < wide.width,
            "a narrower row means a narrower canvas"
        );
        assert!(narrow.height > wide.height, "and a taller one");

        // The band is still one band: every card in it carries the same
        // generation, and the band knows how many rows it took.
        let kids_band = narrow
            .bands
            .iter()
            .find(|b| b.count == 60)
            .expect("the sixty siblings are one band");
        assert!(kids_band.rows > 1, "it wrapped");
        assert_eq!(
            kids_band.cards.len(),
            60,
            "and holds all of them, on {} rows",
            kids_band.rows
        );
        let rows: BTreeSet<usize> = kids_band.cards.iter().map(|c| c.row).collect();
        assert_eq!(
            rows.len(),
            kids_band.rows,
            "every row of the band actually holds cards"
        );
        // The band's own height covers every row it drew.
        let lowest = kids_band.cards.iter().map(|c| c.y).fold(f64::MIN, f64::max);
        assert!(
            lowest + CARD_H <= kids_band.y + kids_band.height + 0.1,
            "the band's zone contains its own cards"
        );
    }

    #[test]
    fn a_generation_under_the_thicket_folds_as_far_as_it_needs_to() {
        // Twenty siblings on a phone-width row: two cards each, ten rows. The
        // row cap is for generations big enough to turn into a thicket, and
        // folding a small one into a near-column is the only thing that fits a
        // phone.
        let mut people: Vec<String> = vec!["ma".into(), "pa".into()];
        for i in 0..20 {
            people.push(format!("kid{i:02}"));
        }
        let refs: Vec<&str> = people.iter().map(String::as_str).collect();
        let kids: Vec<&str> = refs[2..].to_vec();
        let b = bundle(&refs, &[(&["ma", "pa"], &kids)]);

        let phone = layout_with(&b, LayoutOpts::default().with_width(MIN_WRAP_W));
        let band = phone.bands.iter().find(|b| b.count == 20).unwrap();
        assert!(
            band.rows > MAX_BAND_ROWS,
            "a small generation folds past the cap: {} rows",
            band.rows
        );
        // One card a row, so the canvas is at its floor — narrow enough for
        // the tree column of a 390px phone.
        assert!(
            phone.width <= 320.0,
            "…which is what lets it fit the screen: {}",
            phone.width
        );
    }

    #[test]
    fn a_person_the_reader_may_not_read_takes_a_marker_not_a_card() {
        // The shape of the family is not what is being withheld, so the person
        // keeps their place in the row. What they do not keep is a card's
        // width: on a bundle where a visitor may read nobody, that is the
        // difference between a canvas of the word "Private" and a legible one.
        let people: Vec<String> = (0..20).map(|i| format!("p{i:02}")).collect();
        let refs: Vec<&str> = people.iter().map(String::as_str).collect();
        let kids: Vec<&str> = refs[2..].to_vec();
        let b = bundle(&refs, &[(&[refs[0], refs[1]], &kids)]);

        let everyone = layout_with(&b, LayoutOpts::default().with_width(MAX_WRAP_W));

        // Only the parents are readable.
        let visible: BTreeSet<String> = ["p00".to_string(), "p01".to_string()].into();
        let redacted = layout_with(
            &b,
            LayoutOpts::default()
                .with_width(MAX_WRAP_W)
                .seeing(Some(&visible)),
        );

        assert_eq!(
            everyone.person_count, redacted.person_count,
            "compressed, not omitted: a hidden person is still drawn"
        );
        assert!(
            redacted.width < everyone.width / 2.0,
            "and takes far less width doing it: {} vs {}",
            redacted.width,
            everyone.width
        );

        let marker_widths: Vec<f64> = redacted
            .bands
            .iter()
            .flat_map(|b| b.cards.iter())
            .filter(|c| c.restricted)
            .map(|c| c.w)
            .collect();
        assert_eq!(marker_widths.len(), 18, "eighteen people are withheld");
        assert!(
            marker_widths.iter().all(|w| (*w - MARK_W).abs() < 0.01),
            "each of them a marker's width"
        );
        // The two readable parents keep a card.
        assert!(redacted
            .bands
            .iter()
            .flat_map(|b| b.cards.iter())
            .any(|c| !c.restricted && (c.w - CARD_W).abs() < 0.01));
    }

    #[test]
    fn a_connector_out_of_a_wrapped_band_climbs_a_lane_rather_than_a_card() {
        // The constraint the wrapping has to respect: an edge leaving a card
        // on an inner row must get past its own generation's other rows to
        // reach the one above, and it must go around them rather than through
        // them.
        let mut people: Vec<String> = vec!["ma".into(), "pa".into()];
        for i in 0..24 {
            people.push(format!("kid{i:02}"));
        }
        let refs: Vec<&str> = people.iter().map(String::as_str).collect();
        let kids: Vec<&str> = refs[2..].to_vec();
        // One family: ma and pa below, twenty-four children in the band above,
        // so every child has a connector that must enter its wrapped band from
        // underneath and get past the rows below it.
        let b = bundle(&refs, &[(&refs[0..2], &kids[..])]);

        let l = layout_with(&b, LayoutOpts::default().with_width(900.0));
        let band = l.bands.iter().find(|b| b.count == 24).unwrap();
        assert!(band.rows > 1, "the generation wrapped");

        // Rectangles of every card, and every segment of every drawn edge.
        let rects: Vec<(f64, f64, f64, f64)> = l
            .bands
            .iter()
            .flat_map(|b| b.cards.iter())
            .map(|c| (c.x, c.y, c.x + c.w, c.y + CARD_H))
            .collect();

        let mut through = 0;
        for e in l.edges.iter().filter(|e| e.kind == "parent") {
            for (p, q) in path_segments(&e.d) {
                for r in &rects {
                    if segment_enters(p, q, *r) {
                        through += 1;
                    }
                }
            }
        }
        assert_eq!(
            through, 0,
            "no descent connector may run across a card in a wrapped band"
        );
    }

    /// Parse the orthogonal path data back into segments.
    fn path_segments(d: &str) -> Vec<((f64, f64), (f64, f64))> {
        let mut pts = Vec::new();
        let mut cur = (0.0f64, 0.0f64);
        let mut it = d.split_whitespace().peekable();
        while let Some(tok) = it.next() {
            match tok {
                "M" => {
                    let x: f64 = it.next().unwrap().parse().unwrap();
                    let y: f64 = it.next().unwrap().parse().unwrap();
                    cur = (x, y);
                    pts.push(cur);
                }
                "H" => {
                    cur.0 = it.next().unwrap().parse().unwrap();
                    pts.push(cur);
                }
                "V" => {
                    cur.1 = it.next().unwrap().parse().unwrap();
                    pts.push(cur);
                }
                _ => {}
            }
        }
        pts.windows(2).map(|w| (w[0], w[1])).collect()
    }

    /// Does an axis-aligned segment pass through a rectangle's interior?
    fn segment_enters(p: (f64, f64), q: (f64, f64), r: (f64, f64, f64, f64)) -> bool {
        let pad = 1.5;
        let (x0, y0, x1, y1) = (r.0 + pad, r.1 + pad, r.2 - pad, r.3 - pad);
        if (p.0 - q.0).abs() < 1e-6 {
            let (lo, hi) = (p.1.min(q.1), p.1.max(q.1));
            return p.0 > x0 && p.0 < x1 && hi > y0 && lo < y1;
        }
        if (p.1 - q.1).abs() < 1e-6 {
            let (lo, hi) = (p.0.min(q.0), p.0.max(q.0));
            return p.1 > y0 && p.1 < y1 && hi > x0 && lo < x1;
        }
        false
    }

    #[test]
    fn wrapping_keeps_the_barycentre_order_reading_left_to_right_then_down() {
        let mut people: Vec<String> = vec!["ma".into(), "pa".into()];
        for i in 0..30 {
            people.push(format!("kid{i:02}"));
        }
        let refs: Vec<&str> = people.iter().map(String::as_str).collect();
        let kids: Vec<&str> = refs[2..].to_vec();
        let b = bundle(&refs, &[(&["ma", "pa"], &kids)]);

        let wide = layout_with(&b, LayoutOpts::default().with_width(4000.0));
        let narrow = layout_with(&b, LayoutOpts::default().with_width(1200.0));

        let seq = |l: &TreeLayout| -> Vec<String> {
            let band = l.bands.iter().find(|b| b.count == 30).unwrap();
            let mut cards: Vec<&Card> = band.cards.iter().collect();
            // Reading order: down the rows, left to right within each.
            cards.sort_by(|a, b| a.row.cmp(&b.row).then(a.x.partial_cmp(&b.x).unwrap()));
            cards.iter().map(|c| c.id.clone()).collect()
        };
        assert_eq!(
            seq(&wide),
            seq(&narrow),
            "wrapping must not reorder the generation, only fold it"
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

    // -- barycentre ordering / crossing minimisation -----------------------

    /// Run the ordering pass over a whole bundle.
    fn ordered(b: &Value) -> Ordered {
        compute_ordering(
            b["persons"].as_object().unwrap(),
            &family_edges(b),
            &assign_generations(b).gen,
            None,
        )
    }

    /// Position of `id` within its own row.
    fn col(rows: &BTreeMap<i64, Vec<String>>, g: i64, id: &str) -> usize {
        rows[&g].iter().position(|x| x == id).expect("id in row")
    }

    #[test]
    fn the_operator_case_is_clean_after_ordering() {
        // The operator's own complaint, in miniature: two couples in the
        // parents' generation, whose children are a couple, entered so the
        // parent edges cross. Saturnina (his mother) and Jacqueline (his
        // wife's mother) sit in generation 0; he and his wife in generation 1.
        let b = bundle(
            &["saturnina", "sat_sp", "jacqueline", "jac_sp", "him", "her"],
            &[
                (&["saturnina", "sat_sp"], &["him"]),
                (&["jacqueline", "jac_sp"], &["her"]),
                (&["him", "her"], &[]),
            ],
        );
        let ord = ordered(&b);
        assert_eq!(
            ord.crossings, 0,
            "the two parent edges must not cross once ordered"
        );

        // Each parent sits on the same side as their own child: the affiliation
        // is legible at a glance because the lines run straight.
        let him_left = col(&ord.rows, 1, "him") < col(&ord.rows, 1, "her");
        let sat_left = col(&ord.rows, 0, "saturnina") < col(&ord.rows, 0, "jacqueline");
        assert_eq!(
            him_left, sat_left,
            "his mother must sit on his side, and his wife's mother on hers"
        );
    }

    #[test]
    fn an_unavoidable_crossing_is_kept_minimal_and_coloured() {
        // Two couples whose children cross-marry: A's children pair with B's
        // children in two separate unions. A is left of B, yet each has a child
        // in the other's union — one crossing is forced and cannot be ordered
        // away.
        let b = bundle(
            &["a1", "a2", "b1", "b2", "ax", "ay", "bx", "by"],
            &[
                (&["a1", "a2"], &["ax", "ay"]),
                (&["b1", "b2"], &["bx", "by"]),
                (&["ax", "bx"], &[]),
                (&["ay", "by"], &[]),
            ],
        );
        let ord = ordered(&b);
        assert_eq!(
            ord.crossings, 1,
            "exactly the one forced crossing survives, not zero and not more"
        );

        // The residual crossing's two edges each get a marker, and they differ
        // so the eye can follow one line through the intersection. The marker
        // is an index: the stylesheet gives it both a hue and a dash pattern,
        // so the two lines stay distinguishable under a colour-blind theme.
        let markers: Vec<u8> = ord.hues.values().copied().collect();
        assert_eq!(
            markers.len(),
            2,
            "only the crossing pair is marked: {:?}",
            ord.hues
        );
        assert_ne!(
            markers[0], markers[1],
            "the two crossing lines get distinct markers"
        );
        assert!(
            markers.iter().all(|m| (1..=CROSSING_MARKERS).contains(m)),
            "every marker has a stylesheet rule: {markers:?}"
        );

        // And a marker is rare: no other edge carries one.
        let l = layout(&b);
        let marked = l.edges.iter().filter(|e| e.hue.is_some()).count();
        assert_eq!(marked, 2, "a marker must stay rare to stay meaningful");
    }

    #[test]
    fn spouses_stay_adjacent_through_every_sweep() {
        // A five-deep line with a married-in spouse at each level, plus a
        // sibling group, so several sweeps actually fire.
        let b = bundle(
            &[
                "g0", "g0s", "g1", "g1s", "g2", "g2s", "g3", "k1", "k2", "k3",
            ],
            &[
                (&["g0", "g0s"], &["g1"]),
                (&["g1", "g1s"], &["g2"]),
                (&["g2", "g2s"], &["g3", "k1", "k2", "k3"]),
            ],
        );
        let ord = ordered(&b);
        for (a, s) in [("g0", "g0s"), ("g1", "g1s"), ("g2", "g2s")] {
            let g = assign_generations(&b).gen[a];
            let da = col(&ord.rows, g, a) as i64;
            let db = col(&ord.rows, g, s) as i64;
            assert_eq!(
                (da - db).abs(),
                1,
                "{a} and {s} are a couple and must stay adjacent"
            );
        }
    }

    #[test]
    fn the_layout_is_deterministic_across_runs() {
        // Same bundle, twice: every card must land on exactly the same spot,
        // or the tree shifts under the reader between page loads.
        let b = bundle(
            &["a1", "a2", "b1", "b2", "c", "d", "e", "f"],
            &[
                (&["a1", "a2"], &["c", "d"]),
                (&["b1", "b2"], &["e", "f"]),
                (&["c", "e"], &[]),
            ],
        );
        let first = layout(&b);
        let second = layout(&b);
        let pos = |l: &TreeLayout| -> Vec<(String, i64, i64)> {
            let mut v: Vec<(String, i64, i64)> = l
                .bands
                .iter()
                .flat_map(|band| &band.cards)
                .map(|c| (c.id.clone(), c.x as i64, c.y as i64))
                .collect();
            v.sort();
            v
        };
        assert_eq!(pos(&first), pos(&second), "layout must be reproducible");
    }

    #[test]
    fn ordering_never_increases_crossings_over_the_seed() {
        // The pass keeps the best ordering it sees, so its result can only be
        // as good as, or better than, the deterministic seed order.
        let b = bundle(
            &["p1", "p2", "q1", "q2", "r1", "r2", "a", "b", "c"],
            &[
                (&["p1", "p2"], &["a"]),
                (&["q1", "q2"], &["b"]),
                (&["r1", "r2"], &["c"]),
                (&["a", "b"], &[]),
                (&["b", "c"], &[]),
            ],
        );
        let (before, after) = crossings_before_after(&b);
        assert!(
            after <= before,
            "sweeps must not make it worse: {before} -> {after}"
        );
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
