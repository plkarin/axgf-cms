//! The river: one person at a fixed point, ancestors flowing in from below,
//! descendants flowing out above.
//!
//! This module lays the river out and writes it as SVG, server-side, so the
//! first paint is correct with JavaScript disabled. It is pure: functions over
//! the loaded bundle and a [`Lens`], no disk, no globals. The client
//! (`static/river.js`) only re-renders frames *between* two server layouts
//! while it travels.
//!
//! # What the drawing means
//!
//! * **Width is recorded descendants, and nothing else.** Everyone has two
//!   parents, so going upstream a current forks rather than gathering
//!   tributaries, and the same descendants flow down the father's line and the
//!   mother's at full width. Widths therefore do not sum at a confluence. That
//!   is correct, and it is why width cannot mean anything else.
//! * **Five discharge classes from a lookup** ([`class_of`]), never `sqrt`:
//!   quantised widths are testable and stay comparable between two
//!   screenshots taken a year apart.
//! * **Confidence is the stroke**: solid for attested, dashed for inferred, a
//!   faint ghost under a dotted line for speculative ([`Conf`]). The bands are
//!   the record page's own ([`crate::view::Confidence`]), so the river and the
//!   record agree about every claim.
//! * **Fade is atmosphere, the ring is information.** A lost line fades and
//!   ends in an open ring; a line that only leaves the frame is solid and ends
//!   in an arrowhead and a count.
//! * **Time runs on two channels**: colour along the water, and half-century
//!   bands behind it that survive colour-blind themes and print.
//!
//! # Privacy
//!
//! Nothing here reads a person field. The geometry comes from
//! [`access::river_shape`], which is the same for every reader, and the text
//! from [`access::river_label`], which is blank for whoever the lens does not
//! admit. A redacted person keeps their node, their edges and their width —
//! the descendant count is already visible in the grid view — and has no
//! name, no years, no label, and a colour taken from their generation rather
//! than their birth year.
//!
//! # SVG budget
//!
//! Plain `path`, `circle`, `polygon`, `rect`, `line` and `text`, and
//! `linearGradient` for the fades. No filters and no clip-paths: one
//! `feGaussianBlur` froze rendering in the designer's testing.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt::Write as _;

use serde::Serialize;
use serde_json::Value;

use crate::access::{self, Lens, RiverLabel, RiverShape};
use crate::render::html_escape;

// ---------------------------------------------------------------------------
// Geometry contract
// ---------------------------------------------------------------------------

/// The viewBox is `0 0 W H`.
pub const W: f64 = 900.0;
pub const H: f64 = 640.0;
/// The selected person's fixed point.
pub const CX: f64 = 470.0;
pub const CY: f64 = 318.0;
/// Years per generation, for the year scale.
pub const YEARS_PER_GEN: f64 = 29.0;
/// Where the river is served: the landing page. Every person in the SVG
/// links here.
pub const RIVER_PATH: &str = "/";
/// The ranges a reader can choose.
pub const RANGES: [usize; 3] = [2, 3, 5];
pub const DEFAULT_RANGE: usize = 3;

/// Stroke width per discharge class, at the focused scale. Index 0 is unused.
pub const WIDTHS: [f64; 6] = [0.0, 1.6, 3.0, 5.0, 8.0, 12.0];

/// The colour ramp, on `t = (birth_year - 1690) / (1985 - 1690)`.
const RAMP: [(f64, [u8; 3]); 5] = [
    (0.0, [0x2b, 0x5a, 0x52]),
    (0.40, [0x3f, 0x80, 0x72]),
    (0.68, [0x86, 0xb0, 0x8f]),
    (0.88, [0xd3, 0xcf, 0x9e]),
    (1.0, [0xe6, 0xb0, 0x62]),
];
/// The one colour of a river with no era: the legend's water ink.
pub const NEUTRAL: &str = "#86b08f";
const RAMP_FROM: f64 = 1690.0;
const RAMP_TO: f64 = 1985.0;

/// The river canvas colour: the fill of open rings and of the label halo.
const BG: &str = "#0c1310";
const ACCENT: &str = "#dcae64";
const LIVING_GLOW: &str = "#e6b062";
const BAND_ODD: &str = "#0e1714";
const BAND_LIVE: &str = "#15150f";
const BAND_RULE: &str = "#18241f";
const BAND_TEXT: &str = "#4f6158";
const BAND_TEXT_LIVE: &str = "#8a7a52";
const DATA_TEXT: &str = "#8d9c92";
const NAME_TEXT: &str = "#e2dccb";
const CENTRE_TEXT: &str = "#f6efdc";
const MONO: &str = "ui-monospace,Menlo,monospace";
const SERIF: &str = "'Iowan Old Style',Palatino,Georgia,serif";

/// Discharge class for `d`, which is descendants + 1 for a person and the
/// count of distinct descendants for a couple.
pub fn class_of(d: usize) -> usize {
    match d {
        0..=1 => 1,
        2..=5 => 2,
        6..=20 => 3,
        21..=80 => 4,
        _ => 5,
    }
}

/// Stroke width for `d`.
pub fn width_of(d: usize) -> f64 {
    WIDTHS[class_of(d)]
}

/// The water's colour for a birth year, as `#rrggbb`.
pub fn colour_at(year: f64) -> String {
    let t = ((year - RAMP_FROM) / (RAMP_TO - RAMP_FROM)).clamp(0.0, 1.0);
    let t = if t.is_nan() { 0.0 } else { t };
    for i in 1..RAMP.len() {
        if t <= RAMP[i].0 {
            let (t0, a) = RAMP[i - 1];
            let (t1, b) = RAMP[i];
            let k = if t1 > t0 { (t - t0) / (t1 - t0) } else { 0.0 };
            let mix = |j: usize| {
                let v = f64::from(a[j]) + (f64::from(b[j]) - f64::from(a[j])) * k;
                // JavaScript's Math.round: halves go up.
                (v + 0.5).floor() as u8
            };
            return format!("#{:02x}{:02x}{:02x}", mix(0), mix(1), mix(2));
        }
    }
    let c = RAMP[RAMP.len() - 1].1;
    format!("#{:02x}{:02x}{:02x}", c[0], c[1], c[2])
}

/// Round to one decimal.
pub fn r1(v: f64) -> f64 {
    let r = (v * 10.0).round() / 10.0;
    if r == 0.0 {
        0.0
    } else {
        r
    }
}

/// A coordinate as the SVG carries it: one decimal at most, no trailing `.0`,
/// which is also how the client's `Math.round(n * 10) / 10` prints.
pub fn num(v: f64) -> String {
    let r = r1(v);
    if r.fract() == 0.0 {
        format!("{}", r as i64)
    } else {
        format!("{r:.1}")
    }
}

/// The vertical-tangent cubic: both control points on the midline.
pub fn curve(x1: f64, y1: f64, x2: f64, y2: f64) -> String {
    let m = (y1 + y2) / 2.0;
    format!(
        "M{} {}C{} {} {} {} {} {}",
        num(x1),
        num(y1),
        num(x1),
        num(m),
        num(x2),
        num(m),
        num(x2),
        num(y2)
    )
}

// ---------------------------------------------------------------------------
// Confidence
// ---------------------------------------------------------------------------

/// How a parent-child claim is drawn.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub enum Conf {
    /// Solid, round caps.
    #[serde(rename = "a")]
    Attested,
    /// Dashed.
    #[serde(rename = "d")]
    Inferred,
    /// A faint ghost under a dotted line.
    #[serde(rename = "s")]
    Speculative,
}

/// The confidence a claim carries when it states none: the grid view's
/// default, so the two views agree about an unrated claim too.
const UNRATED: f64 = 0.8;

impl Conf {
    /// From the AXGF float, through the record page's bands: `certain` and
    /// `high` are attested, `medium` inferred, `low` speculative.
    pub fn of(value: Option<f64>) -> Self {
        match crate::view::Confidence::new(value.unwrap_or(UNRATED)).band {
            "certain" | "high" => Conf::Attested,
            "medium" => Conf::Inferred,
            _ => Conf::Speculative,
        }
    }
}

// ---------------------------------------------------------------------------
// The family graph
// ---------------------------------------------------------------------------

/// Where a child stands in its family's own record: `birth_order`, then its
/// position in the list.
type RecordOrder = (i64, usize);

/// One family: its partners in slot order and its children with the
/// confidence of each child's claim.
#[derive(Debug, Clone)]
struct Fam {
    id: String,
    /// Partners, father first where the records say which is which.
    parents: Vec<usize>,
    kids: Vec<(usize, Conf, Option<f64>)>,
    /// The union's start date as a `YYYYMMDD` key: orders a person's unions,
    /// and only when the reader may see every member of them.
    start_sort: Option<i64>,
}

/// Every person and family the river can reach, indexed.
pub struct Graph {
    ids: Vec<String>,
    index: HashMap<String, usize>,
    shape: Vec<RiverShape>,
    fams: Vec<Fam>,
    /// The family each person is a child of, if any.
    born_in: Vec<Option<usize>>,
    /// Children, in the order their families record them.
    kids_of: Vec<Vec<usize>>,
    /// Families each person is a partner in, in the order families are read.
    partner_in: Vec<Vec<usize>>,
    desc: Vec<usize>,
    anc: Vec<usize>,
}

impl Graph {
    /// Read the families of `flat`. Persons come through
    /// [`access::river_shape`] only.
    pub fn build(flat: &Value) -> Self {
        let mut ids: Vec<String> = flat
            .get("persons")
            .and_then(Value::as_object)
            .map(|m| m.keys().cloned().collect())
            .unwrap_or_default();
        ids.sort();
        let index: HashMap<String, usize> = ids
            .iter()
            .enumerate()
            .map(|(i, id)| (id.clone(), i))
            .collect();
        let shape: Vec<RiverShape> = ids.iter().map(|id| access::river_shape(flat, id)).collect();
        let n = ids.len();

        let mut fams = Vec::new();
        let families = flat.get("families").and_then(Value::as_object);
        if let Some(families) = families {
            let mut fam_ids: Vec<&String> = families.keys().collect();
            fam_ids.sort();
            for fid in &fam_ids {
                let f = &families[fid.as_str()];
                let mut parents: Vec<usize> = f
                    .get("union")
                    .and_then(|u| u.get("persons"))
                    .and_then(Value::as_array)
                    .map(|ps| {
                        ps.iter()
                            .filter_map(|p| p.get("person_id").and_then(Value::as_str))
                            .filter_map(|id| index.get(id).copied())
                            .collect()
                    })
                    .unwrap_or_default();
                parents.dedup();
                // Father first. A stable sort on the slot keeps the union's own
                // order wherever the records say nothing.
                parents.sort_by_key(|&p| shape[p].slot.unwrap_or(0));
                // Children in the order the family records them: its own
                // `birth_order`, then the order of the list. Never by birth
                // date: a hidden child's date would then move a visible
                // sibling, and the order of two hidden siblings would state
                // which is older (tests/river.rs, the hidden-dates invariant).
                let mut kids: Vec<(usize, Conf, Option<f64>, RecordOrder)> = f
                    .get("children")
                    .and_then(Value::as_array)
                    .map(|cs| {
                        cs.iter()
                            .enumerate()
                            .filter_map(|(pos, c)| {
                                let id = c.get("person_id").and_then(Value::as_str)?;
                                let i = *index.get(id)?;
                                let conf = c.get("confidence").and_then(Value::as_f64);
                                let order = c
                                    .get("birth_order")
                                    .and_then(Value::as_i64)
                                    .unwrap_or(i64::MAX);
                                Some((i, Conf::of(conf), conf, (order, pos)))
                            })
                            .collect()
                    })
                    .unwrap_or_default();
                kids.sort_by_key(|k| k.3);
                let kids = kids.into_iter().map(|(i, c, raw, _)| (i, c, raw)).collect();
                let start_sort = f
                    .get("union")
                    .and_then(|u| u.get("start"))
                    .map(|s| crate::view::render_date_field(s, "date"))
                    .and_then(|d| d.sort);
                fams.push(Fam {
                    id: (*fid).clone(),
                    parents,
                    kids,
                    start_sort,
                });
            }
        }

        // A child recorded in two families (an adoption beside a birth) is
        // drawn from the one whose claim is strongest; ties go to the first.
        let mut born_in: Vec<Option<usize>> = vec![None; n];
        let mut born_conf: Vec<f64> = vec![f64::NEG_INFINITY; n];
        let mut kids_of: Vec<Vec<usize>> = vec![Vec::new(); n];
        let mut partner_in: Vec<Vec<usize>> = vec![Vec::new(); n];
        for (fi, f) in fams.iter().enumerate() {
            for &p in &f.parents {
                partner_in[p].push(fi);
            }
        }
        for (fi, f) in fams.iter().enumerate() {
            for &(k, _, raw) in &f.kids {
                let c = raw.unwrap_or(UNRATED);
                if c > born_conf[k] {
                    born_conf[k] = c;
                    born_in[k] = Some(fi);
                }
                for &p in &f.parents {
                    if p != k && !kids_of[p].contains(&k) {
                        kids_of[p].push(k);
                    }
                }
            }
        }
        // A parent's children: family by family in the order families are
        // read (by id), each family's in its own recorded order.

        let mut g = Graph {
            ids,
            index,
            shape,
            fams,
            born_in,
            kids_of,
            partner_in,
            desc: Vec::new(),
            anc: Vec::new(),
        };
        g.desc = (0..n).map(|i| g.descendants(&[i]).len()).collect();
        g.anc = (0..n).map(|i| g.ancestors(i)).collect();
        g
    }

    /// Distinct people descended from any of `roots`, the roots excluded.
    fn descendants(&self, roots: &[usize]) -> BTreeSet<usize> {
        let mut seen = BTreeSet::new();
        let mut stack: Vec<usize> = roots.to_vec();
        while let Some(x) = stack.pop() {
            for &k in &self.kids_of[x] {
                if !roots.contains(&k) && seen.insert(k) {
                    stack.push(k);
                }
            }
        }
        seen
    }

    fn ancestors(&self, root: usize) -> usize {
        let mut seen = BTreeSet::new();
        let mut stack = vec![root];
        while let Some(x) = stack.pop() {
            for p in self.parents_of(x) {
                if p != root && seen.insert(p) {
                    stack.push(p);
                }
            }
        }
        seen.len()
    }

    /// The parents a person is drawn from, father first.
    fn parents_of(&self, i: usize) -> Vec<usize> {
        self.born_in[i]
            .map(|f| {
                self.fams[f]
                    .parents
                    .iter()
                    .copied()
                    .filter(|&p| p != i)
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Each person's children, ordered for one reader.
    ///
    /// A sibship — the children of one family — is sorted by birth date only
    /// when the reader may see every child in it; undated children keep their
    /// recorded places. If any child is hidden, the whole sibship stays in the
    /// order the family records it and no date is read at all: sorting it
    /// would move a visible sibling according to a hidden one's birth date,
    /// and put two hidden siblings in the order of their ages. Families are
    /// taken in the order they are read; a child listed in two is placed by
    /// the first.
    ///
    /// The record's order is often not birth order — 57 of the 120 families
    /// with two or more children on the operator's bundle disagree — which is
    /// why the date is used wherever it may be.
    pub fn order_for(&self, sees: &dyn Fn(usize) -> bool) -> Vec<Vec<usize>> {
        (0..self.len())
            .map(|p| {
                let mut out: Vec<usize> = Vec::new();
                for &f in &self.partner_in[p] {
                    for k in self.sibship(f, p, sees) {
                        if !out.contains(&k) {
                            out.push(k);
                        }
                    }
                }
                out
            })
            .collect()
    }

    /// One family's children, `p` excluded, in the reader's order: by birth
    /// date where they may see every child, recorded order otherwise.
    fn sibship(&self, f: usize, p: usize, sees: &dyn Fn(usize) -> bool) -> Vec<usize> {
        let mut kids: Vec<usize> = self.fams[f]
            .kids
            .iter()
            .map(|k| k.0)
            .filter(|&k| k != p)
            .collect();
        if kids.iter().all(|&k| sees(k)) {
            // Dated children, sorted, back into the slots dated children
            // held; undated ones stay where they were.
            let slots: Vec<usize> = (0..kids.len())
                .filter(|&i| self.shape[kids[i]].birth_sort.is_some())
                .collect();
            let mut dated: Vec<usize> = slots.iter().map(|&i| kids[i]).collect();
            dated.sort_by_key(|&k| self.shape[k].birth_sort);
            for (slot, k) in slots.into_iter().zip(dated) {
                kids[slot] = k;
            }
        }
        kids
    }

    /// A person's unions that have children, in the order they are drawn,
    /// each with the spouse drawn beside the person and the children in
    /// sibling order.
    ///
    /// By the union's own start date where the reader may see every member
    /// of every one of them — partners and children — and every one is
    /// dated; otherwise in the order the families are read, and no date is
    /// read at all. The sibling rule (ADR 0001 §2) for unions: a hidden
    /// spouse's marriage date must not move anything a visitor receives.
    pub fn unions_for(
        &self,
        p: usize,
        sees: &dyn Fn(usize) -> bool,
    ) -> Vec<(usize, Option<usize>, Vec<usize>)> {
        let mut out: Vec<(usize, Option<usize>, Vec<usize>)> = Vec::new();
        for &f in &self.partner_in[p] {
            let kids = self.sibship(f, p, sees);
            if kids.is_empty() {
                continue;
            }
            let spouse = self.fams[f].parents.iter().copied().find(|&q| q != p);
            out.push((f, spouse, kids));
        }
        let all_seen = out.iter().all(|(f, _, _)| {
            let fam = &self.fams[*f];
            fam.parents.iter().all(|&q| sees(q)) && fam.kids.iter().all(|k| sees(k.0))
        });
        let all_dated = out
            .iter()
            .all(|(f, _, _)| self.fams[*f].start_sort.is_some());
        if out.len() > 1 && all_seen && all_dated {
            out.sort_by_key(|(f, _, _)| self.fams[*f].start_sort);
        }
        out
    }

    /// Distinct descendants of one family: its children and everything below.
    fn family_discharge(&self, f: usize) -> usize {
        let kids: Vec<usize> = self.fams[f].kids.iter().map(|k| k.0).collect();
        let mut all = self.descendants(&kids);
        all.extend(kids);
        all.len()
    }

    fn kid_conf(&self, f: usize, kid: usize) -> Conf {
        self.fams[f]
            .kids
            .iter()
            .find(|k| k.0 == kid)
            .map(|k| k.1)
            .unwrap_or(Conf::Attested)
    }

    /// Look a person up by id.
    pub fn index_of(&self, id: &str) -> Option<usize> {
        self.index.get(id).copied()
    }

    /// Number of persons.
    pub fn len(&self) -> usize {
        self.ids.len()
    }

    /// Whether the bundle has nobody in it.
    pub fn is_empty(&self) -> bool {
        self.ids.is_empty()
    }

    /// Whom to centre on when the reader names nobody: the person with the
    /// most recorded ancestors and descendants together, among those `lens`
    /// lets the reader see — the fullest first screen. Ties go to the lowest
    /// id, so the landing page is the same on every load. When the reader may
    /// see nobody, the river still draws its shape, redacted, from the same
    /// person an administrator would land on.
    pub fn default_centre(&self, lens: &Lens) -> Option<String> {
        let best = |seen: bool| {
            (0..self.len())
                .filter(|&i| !seen || lens.sees_person(&self.ids[i]))
                .max_by(|&a, &b| {
                    (self.desc[a] + self.anc[a])
                        .cmp(&(self.desc[b] + self.anc[b]))
                        .then_with(|| self.ids[b].cmp(&self.ids[a]))
                })
        };
        best(true)
            .or_else(|| best(false))
            .map(|i| self.ids[i].clone())
    }

    /// Recorded descendants of `id`.
    pub fn descendant_count(&self, id: &str) -> Option<usize> {
        self.index_of(id).map(|i| self.desc[i])
    }
}

// ---------------------------------------------------------------------------
// Layout
// ---------------------------------------------------------------------------

/// How the layout departs from the reference's uniform geometry.
///
/// The designer's notes ask for both (§5 b and c). The reference page draws
/// neither, so each is a switch, and `tests/river_scale.rs` measures what each
/// does to horizontal sweeps on the operator's bundle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Shape {
    /// Give each generation a width budget from its own head count, so sparse
    /// generations pack toward the centre: an hourglass.
    pub hourglass: bool,
    /// Space generations by `log(people per generation)`, so sparse rows get
    /// more height.
    pub log_spacing: bool,
}

/// Both off: the reference geometry. Measured on the operator's 866-person
/// bundle (`tests/river_scale.rs`), neither switch reduced long horizontal
/// runs — the hourglass made ± 5 worse, log spacing was within noise — and log
/// spacing would also move the right rail during travel, which must stay
/// still, and bend the 29-years-per-generation band scale.
impl Default for Shape {
    fn default() -> Self {
        Self {
            hourglass: false,
            log_spacing: false,
        }
    }
}

/// Where a person stands in the frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Centre,
    Anc,
    Desc,
    Spouse,
    Dspouse,
}

#[derive(Debug, Clone, Serialize)]
pub struct PNode {
    pub id: String,
    /// Unique per drawn occurrence: the id for the first, `id~2`, `id~3` for
    /// an ancestor or descendant drawn again under another line. Edges
    /// reference occurrences by key.
    pub key: String,
    pub x: f64,
    pub y: f64,
    pub role: Role,
    /// Label tier: 0 none, 1 given name, 2 given name and initial, 3 full.
    pub lab: u8,
    /// The smaller gap to a neighbour in the row: what chose `lab`.
    pub room: f64,
    /// The gap to the next person to the right in the row: how far this
    /// person's label may run.
    pub right: f64,
    /// How many lines reach this person, when more than one: a person
    /// reached through two lines (pedigree collapse) is drawn under each,
    /// and every occurrence carries the count. 0 otherwise.
    pub repeat: usize,
    /// Drawn again in the same row beside a third or later partner (a
    /// person sits beside two partners at most). Set on every occurrence of
    /// that person, the first included, so the reader can pair them; marked
    /// differently from `repeat`, which is a different story.
    pub again: bool,
    #[serde(skip)]
    idx: usize,
    /// Generations from the centre, signed: negative is upstream.
    pub gen: i64,
    /// Descendants + 1: the class of the union→child edge into this person.
    pub d: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct Couple {
    pub key: String,
    pub x: f64,
    pub y: f64,
    pub parents: Vec<String>,
    pub kids: Vec<String>,
    /// Distinct descendants of the family.
    pub d: usize,
    /// Brothers and sisters of the drawn child, not drawn.
    pub stub: usize,
    pub pconf: BTreeMap<String, Conf>,
    pub kconf: BTreeMap<String, Conf>,
    /// Discharge of the stub's rest-of-family current.
    pub stub_d: usize,
    /// The year the stub is coloured by: its first parent's, plus 28.
    pub stub_year: f64,
    /// A union drawn more than once, under a repeated ancestor.
    pub repeat: bool,
    /// Where the stub ends, from the union: it rises toward the child's row
    /// and stops short of it, so its count never sits on that row's dots.
    pub stub_dx: f64,
    pub stub_dy: f64,
    /// Which side of the stub's end its count is written on: 1 right, -1
    /// left, 0 not written (it would run over a dot). See [`place_counts`].
    pub stub_side: i8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TailKind {
    /// The line continues off-frame.
    Cont,
    /// The record ends.
    Lost,
}

#[derive(Debug, Clone, Serialize)]
pub struct Tail {
    pub key: String,
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
    pub d: usize,
    pub kind: TailKind,
    /// +1 points down (upstream), -1 up.
    pub dir: i8,
    /// The count beside a `cont` arrowhead.
    pub count: usize,
    /// The year the tail is coloured by: its person's year plus `offset`.
    pub year: f64,
    #[serde(skip)]
    who: usize,
    #[serde(skip)]
    offset: f64,
    /// For a line that continues: which side of the arrowhead its count is
    /// written on, 1 right, -1 left, 0 not written. See [`place_counts`].
    pub count_side: i8,
}

/// The year scale: a piecewise-linear map from year to y.
#[derive(Debug, Clone, Serialize)]
pub struct YearScale {
    /// `(year, y)` knots, year ascending (y descending).
    pub knots: Vec<(f64, f64)>,
}

impl YearScale {
    pub fn y_of(&self, year: f64) -> f64 {
        let k = &self.knots;
        let seg = |a: (f64, f64), b: (f64, f64)| a.1 + (year - a.0) * (b.1 - a.1) / (b.0 - a.0);
        if k.len() < 2 {
            return CY;
        }
        if year <= k[0].0 {
            return seg(k[0], k[1]);
        }
        for w in k.windows(2) {
            if year <= w[1].0 {
                return seg(w[0], w[1]);
            }
        }
        seg(k[k.len() - 2], k[k.len() - 1])
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Meta {
    pub centre: String,
    pub n: usize,
    pub step: f64,
    /// The centre's (possibly estimated) birth year.
    pub year: f64,
    pub cy: f64,
    pub scale: YearScale,
    /// `(generation offset, y)` for the right rail.
    pub rail: Vec<(i64, f64)>,
    /// The year scale rests on a birth year this reader may see. Without
    /// one, the river has no era: no bands, and one neutral colour.
    pub era: bool,
    /// Whether to draw the half-century bands: an era, and a signed-in
    /// reader.
    pub bands: bool,
    /// The part of the 900 × 640 frame the drawing occupies: `(top, height)`
    /// in viewBox units. See [`view_window`].
    pub view: (f64, f64),
    /// Its horizontal part, `(left, width)`. See [`h_window`].
    pub hview: (f64, f64),
}

impl Meta {
    /// Where labels stop: the right rail's generation numbers sit beyond.
    pub fn label_edge(&self) -> f64 {
        self.hview.0 + self.hview.1 - 34.0
    }
}

/// One laid-out river.
#[derive(Debug, Clone, Serialize)]
pub struct Layout {
    pub persons: Vec<PNode>,
    pub couples: Vec<Couple>,
    pub tails: Vec<Tail>,
    pub meta: Meta,
    #[serde(skip)]
    years: Vec<f64>,
}

impl Layout {
    /// A person by occurrence key: their id for the first (or only) time
    /// they are drawn, `id~2` and on for a repeat.
    pub fn person(&self, key: &str) -> Option<&PNode> {
        self.persons.iter().find(|p| p.key == key)
    }
}

struct Frame {
    n: usize,
    step: f64,
    /// Row y per signed generation offset.
    row_y: BTreeMap<i64, f64>,
    persons: Vec<PNode>,
}

impl Frame {
    fn y(&self, gen: i64) -> f64 {
        if let Some(y) = self.row_y.get(&gen) {
            return *y;
        }
        CY - gen as f64 * self.step
    }

    /// Height from row `gen` down to the row upstream of it.
    fn gap_down(&self, gen: i64) -> f64 {
        (self.y(gen - 1) - self.y(gen)).abs().max(1.0)
    }

    /// Height from row `gen` up to the row downstream of it.
    fn gap_up(&self, gen: i64) -> f64 {
        (self.y(gen) - self.y(gen + 1)).abs().max(1.0)
    }
}

/// Scale positions about `cx` so they fit `[lo, hi]`.
fn fit(xs: &mut [&mut f64], cx: f64, lo: f64, hi: f64) {
    let (mut mn, mut mx) = (f64::INFINITY, f64::NEG_INFINITY);
    for x in xs.iter() {
        mn = mn.min(**x);
        mx = mx.max(**x);
    }
    let mut s: f64 = 1.0;
    if mn < lo {
        s = s.min((cx - lo) / (cx - mn));
    }
    if mx > hi {
        s = s.min((hi - cx) / (mx - cx));
    }
    if s < 1.0 {
        for x in xs.iter_mut() {
            **x = cx + (**x - cx) * s;
        }
    }
}

/// Lay out the river around `centre` with `n` generations each way.
pub fn layout(g: &Graph, centre: &str, n: usize, shape: Shape) -> Option<Layout> {
    layout_for(g, &|_| true, &|_| DEFAULT_LABEL_W, centre, n, shape)
}

/// A label's room when the layout is asked without a reader: the unit tests,
/// the scale bench. A reader's own labels are measured in [`build`].
pub const DEFAULT_LABEL_W: f64 = 60.0;
const CENTRE_CLEAR: f64 = 28.0;
/// The space kept between two neighbours' extents in a row.
const PACK_GAP: f64 = 10.0;

/// One drawn occurrence of a person.
#[derive(Debug, Clone)]
struct Occ {
    idx: usize,
    gen: i64,
    role: Role,
    x: f64,
    /// Drawn again beside a third or later partner, not reached by a line.
    again: bool,
}

/// A node of a tree being packed: one or more occurrences side by side in one
/// row — a person and the spouses drawn with them — and the subtrees below.
#[derive(Debug, Clone)]
struct Block {
    /// (occurrence, offset from the block's anchor)
    members: Vec<(usize, f64)>,
    left: f64,
    right: f64,
    row: i64,
    kids: Vec<usize>,
    /// A leaf's tail: the row it reaches into and its extent there, held in
    /// the contour so no neighbouring subtree is packed under it.
    tail: Option<(i64, (f64, f64))>,
}

/// A packed subtree: its extent per row and every block's anchor, relative to
/// the subtree root's anchor.
struct Packed {
    contour: BTreeMap<i64, (f64, f64)>,
    anchors: Vec<(usize, f64)>,
}

impl Packed {
    fn shift(&mut self, dx: f64) {
        for v in self.contour.values_mut() {
            v.0 += dx;
            v.1 += dx;
        }
        for a in self.anchors.iter_mut() {
            a.1 += dx;
        }
    }
}

/// Contour packing (Reingold–Tilford): lay each subtree out on its own, then
/// set it beside the subtrees already placed by walking their right contour
/// against its left contour, row by row, and pushing it right by the worst
/// overlap plus [`PACK_GAP`]. The parent sits at the midpoint of its outermost
/// children. Subtrees are therefore disjoint in every row by construction —
/// which is what barycentric placement could not promise, and why it crossed.
///
/// Contours are kept as one interval per row rather than threaded: a river
/// is at most five rows deep on either side, so the walk is O(rows) per merge
/// and the whole pack linear in the number of blocks for any range drawn.
fn pack(blocks: &[Block], i: usize) -> Packed {
    let b = &blocks[i];
    let mut acc: Option<Packed> = None;
    let mut first = 0.0;
    let mut last = 0.0;
    for (n, &k) in b.kids.iter().enumerate() {
        let mut p = pack(blocks, k);
        match acc.as_mut() {
            None => {
                acc = Some(p);
            }
            Some(a) => {
                let shift = p
                    .contour
                    .iter()
                    .filter_map(|(row, l)| a.contour.get(row).map(|r| r.1 - l.0 + PACK_GAP))
                    .fold(f64::NEG_INFINITY, f64::max);
                let shift = if shift.is_finite() {
                    shift
                } else {
                    last + PACK_GAP
                };
                p.shift(shift);
                for (row, v) in p.contour {
                    let e = a.contour.entry(row).or_insert(v);
                    e.0 = e.0.min(v.0);
                    e.1 = e.1.max(v.1);
                }
                a.anchors.extend(p.anchors);
                last = shift;
                if n == 0 {
                    first = shift;
                }
            }
        }
    }
    let mut out = acc.unwrap_or(Packed {
        contour: BTreeMap::new(),
        anchors: Vec::new(),
    });
    let mid = if b.kids.is_empty() {
        0.0
    } else {
        (first + last) / 2.0
    };
    out.contour.insert(b.row, (mid + b.left, mid + b.right));
    if let Some((row, (l, r))) = b.tail {
        let e = out.contour.entry(row).or_insert((mid + l, mid + r));
        e.0 = e.0.min(mid + l);
        e.1 = e.1.max(mid + r);
    }
    out.anchors.push((i, mid));
    out.shift(-mid);
    out
}

/// Which side each lone spouse is drawn on, by descendant occurrence: what a
/// pass asked for, and what the last build used.
#[derive(Default)]
struct Sides {
    want: HashMap<usize, bool>,
    used: HashMap<usize, bool>,
}

/// The polyline [`curve`] draws, in `PIECES` straight pieces.
fn flatten(x1: f64, y1: f64, x2: f64, y2: f64) -> Vec<(f64, f64)> {
    const PIECES: usize = 24;
    let m = (y1 + y2) / 2.0;
    (0..=PIECES)
        .map(|k| {
            let t = k as f64 / PIECES as f64;
            let u = 1.0 - t;
            let (a, b, c, d) = (u * u * u, 3.0 * u * u * t, 3.0 * u * t * t, t * t * t);
            (
                a * x1 + b * x1 + c * x2 + d * x2,
                a * y1 + b * m + c * m + d * y2,
            )
        })
        .collect()
}

fn pieces_cross(p: (f64, f64), q: (f64, f64), r: (f64, f64), s: (f64, f64)) -> bool {
    let o = |a: (f64, f64), b: (f64, f64), c: (f64, f64)| {
        (b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0)
    };
    let (d1, d2, d3, d4) = (o(r, s, p), o(r, s, q), o(p, q, r), o(p, q, s));
    (d1 > 0.0) != (d2 > 0.0) && (d3 > 0.0) != (d4 > 0.0)
}

/// Choose where each count is written — a continuing line's `+n` beside its
/// arrowhead, a family stub's `+n` beside its end — so that none runs over a
/// dot, a repeat ring, a terminator ring, an arrowhead or another count:
/// on its usual side if that is clear, otherwise on the other, otherwise not
/// at all. A count is a hint (the line is drawn either way, and a tighter
/// range can be opened); a count on top of somebody's dot reads as theirs.
fn place_counts(persons: &[PNode], couples: &mut [Couple], tails: &mut [Tail]) {
    type Box4 = (f64, f64, f64, f64);
    let mut circles: Vec<(f64, f64, f64)> = persons
        .iter()
        .map(|p| (p.x, p.y, outer_radius(p) + 0.5))
        .collect();
    for t in tails.iter() {
        match t.kind {
            TailKind::Lost => circles.push((t.x2, t.y2, 4.2)),
            TailKind::Cont => circles.push((t.x2, t.y2 + f64::from(t.dir) * 3.0, 5.0)),
        }
    }
    let mut placed: Vec<Box4> = Vec::new();
    let clear = |b: Box4, circles: &[(f64, f64, f64)], placed: &[Box4]| {
        circles.iter().all(|&(cx, cy, r)| {
            let dx = (b.0 - cx).max(cx - b.1).max(0.0);
            let dy = (b.2 - cy).max(cy - b.3).max(0.0);
            dx.hypot(dy) >= r
        }) && placed
            .iter()
            .all(|o| b.1 <= o.0 || o.1 <= b.0 || b.3 <= o.2 || o.3 <= b.2)
    };
    // A text box: anchored at `x` on `side` (1 starts there, -1 ends there),
    // baseline `y`.
    let text = |x: f64, y: f64, side: f64, w: f64, size: f64| -> Box4 {
        let (a, b) = if side > 0.0 { (x, x + w) } else { (x - w, x) };
        (a, b, y - 0.8 * size, y + 0.2 * size)
    };
    for t in tails.iter_mut().filter(|t| t.kind == TailKind::Cont) {
        let w = text_width(&format!("+{}", t.count), 9.5, true, false);
        let y = t.y2 + if t.dir > 0 { 4.0 } else { 2.0 };
        t.count_side = 0;
        for side in [1.0, -1.0] {
            let b = text(t.x2 + side * 8.0, y, side, w, 9.5);
            if clear(b, &circles, &placed) {
                t.count_side = side as i8;
                placed.push(b);
                break;
            }
        }
    }
    for c in couples.iter_mut().filter(|c| c.stub > 0) {
        let w = text_width(&format!("+{}", c.stub), 10.0, true, false);
        let (x2, y2) = (c.x + c.stub_dx, c.y + c.stub_dy);
        let usual = c.stub_dx.signum();
        c.stub_side = 0;
        for side in [usual, -usual] {
            let b = text(x2 + side * 5.0, y2 - 3.0, side, w, 10.0);
            if clear(b, &circles, &placed) {
                c.stub_side = side as i8;
                placed.push(b);
                break;
            }
        }
    }
}

/// Shorten every tail that would cross a drawn line — a parent's line to a
/// union, a union's to a child, a family stub — until it clears it by a few
/// units. A tail is only a sign that a line goes on or ends; how long it is
/// carries nothing, and a long sweep from a far union can pass just under a
/// spouse's tail where no packing would move it.
fn clear_tails(persons: &[PNode], couples: &[Couple], tails: &mut [Tail]) {
    let at: HashMap<&str, (f64, f64)> = persons
        .iter()
        .map(|p| (p.key.as_str(), (p.x, p.y)))
        .collect();
    let mut lines: Vec<Vec<(f64, f64)>> = Vec::new();
    for c in couples {
        for k in c.parents.iter().chain(&c.kids) {
            if let Some(&(x, y)) = at.get(k.as_str()) {
                lines.push(flatten(x, y, c.x, c.y));
            }
        }
        if c.stub > 0 {
            lines.push(flatten(c.x, c.y, c.x + c.stub_dx, c.y + c.stub_dy));
        }
    }
    let bbox = |l: &[(f64, f64)]| {
        l.iter().fold(
            (
                f64::INFINITY,
                f64::INFINITY,
                f64::NEG_INFINITY,
                f64::NEG_INFINITY,
            ),
            |b, p| (b.0.min(p.0), b.1.min(p.1), b.2.max(p.0), b.3.max(p.1)),
        )
    };
    let mut boxes: Vec<(f64, f64, f64, f64)> = lines.iter().map(|l| bbox(l)).collect();
    const MARGIN: f64 = 4.0;
    for t in tails.iter_mut() {
        let (dx, dy) = (t.x2 - t.x1, t.y2 - t.y1);
        let len = dx.hypot(dy);
        if len < 1.0 {
            continue;
        }
        let mut scale = 1.0;
        for _ in 0..12 {
            // The tail as drawn plus a margin at its end; ignore what touches
            // its own start, where it leaves a node other lines also meet.
            let s = scale + MARGIN / len;
            let pts = flatten(t.x1, t.y1, t.x1 + dx * s, t.y1 + dy * s);
            let near = |p: (f64, f64)| (p.0 - t.x1).hypot(p.1 - t.y1) < 3.0;
            let (bx0, by0, bx1, by1) = pts.iter().fold(
                (
                    f64::INFINITY,
                    f64::INFINITY,
                    f64::NEG_INFINITY,
                    f64::NEG_INFINITY,
                ),
                |b, p| (b.0.min(p.0), b.1.min(p.1), b.2.max(p.0), b.3.max(p.1)),
            );
            let hit = lines.iter().zip(&boxes).any(|(l, b)| {
                b.0 <= bx1 && bx0 <= b.2 && b.1 <= by1 && by0 <= b.3 && {
                    pts.windows(2).any(|a| {
                        l.windows(2).any(|e| {
                            pieces_cross(a[0], a[1], e[0], e[1])
                                && !(near(a[0]) && near(e[0]) || near(a[0]) && near(e[1]))
                        })
                    })
                }
            });
            if !hit {
                break;
            }
            scale *= 0.8;
        }
        t.x2 = t.x1 + dx * scale;
        t.y2 = t.y1 + dy * scale;
        // A tail placed is in the way of the tails after it.
        let drawn = flatten(t.x1, t.y1, t.x2, t.y2);
        boxes.push(bbox(&drawn));
        lines.push(drawn);
    }
}

/// One union drawn under a descendant: whose block, which family, the spouse
/// drawn with them, and the children's occurrences.
struct UnionDrawn {
    person: usize,
    fam: usize,
    spouse: Option<usize>,
    kids: Vec<usize>,
}

/// Lay the river out: the ancestor tree below the centre and the descendant
/// tree above it, each packed by contour ([`pack`]), placed so the centre
/// holds the fixed point, and fitted into the frame.
///
/// `sees` is the reader's lens (sibling and union order may read dates only
/// where it admits everyone involved), and `label_w` how wide this reader's
/// label for a person is — nothing for a person they may not see, so no
/// hidden name's length moves anything.
pub fn layout_for(
    g: &Graph,
    sees: &dyn Fn(usize) -> bool,
    label_w: &dyn Fn(usize) -> f64,
    centre: &str,
    n: usize,
    shape: Shape,
) -> Option<Layout> {
    let c = g.index_of(centre)?;
    let step = 96f64
        .min((CY - 46.0) / n as f64)
        .min((H - 64.0 - CY) / n as f64);
    let mut widths: HashMap<usize, f64> = HashMap::new();
    let mut extent = |i: usize, centre: bool| -> (f64, f64) {
        let w = *widths.entry(i).or_insert_with(|| label_w(i));
        let r = if centre { 7.5 } else { 4.8 };
        // The centre's spouse line rises across the end of the centre's
        // label on its way to their union; the centre keeps CENTRE_CLEAR
        // more room so it rises clear of the name.
        // The centre's ring (r 14) is its left edge.
        (
            if centre { -(14.0 + 4.0) } else { -(r + 4.0) },
            r + if centre { 10.0 + CENTRE_CLEAR } else { 6.0 } + w,
        )
    };
    // What a leaf's tail occupies in the next row: its stroke, and for a line
    // that continues, the arrowhead and its count.
    fn tail_extent(d: usize, count: Option<usize>) -> (f64, f64) {
        let half = width_of(d) / 2.0 + 3.0;
        match count {
            Some(n) => (
                -half.max(6.0),
                8.0 + text_width(&format!("+{n}"), 9.5, true, false),
            ),
            None => (-half, half),
        }
    }

    let mut occs: Vec<Occ> = vec![Occ {
        idx: c,
        gen: 0,
        role: Role::Centre,
        x: 0.0,
        again: false,
    }];
    let mut blocks: Vec<Block> = Vec::new();

    // ---- the ancestor tree: father, mother, to depth n
    let (cl, cr) = extent(c, true);
    #[allow(clippy::too_many_arguments)]
    fn anc(
        g: &Graph,
        i: usize,
        occ: usize,
        depth: usize,
        n: usize,
        occs: &mut Vec<Occ>,
        blocks: &mut Vec<Block>,
        extent: &mut dyn FnMut(usize, bool) -> (f64, f64),
        lr: (f64, f64),
    ) -> usize {
        let slot = blocks.len();
        blocks.push(Block {
            members: vec![(occ, 0.0)],
            left: lr.0,
            right: lr.1,
            row: -(depth as i64),
            kids: Vec::new(),
            tail: None,
        });
        if depth == n || g.parents_of(i).is_empty() {
            let cont = !g.parents_of(i).is_empty();
            let ext = tail_extent(g.desc[i] + 1, cont.then(|| g.anc[i]));
            blocks[slot].tail = Some((-(depth as i64) - 1, ext));
        }
        if depth < n {
            let mut kids = Vec::new();
            for p in g.parents_of(i) {
                let o = occs.len();
                occs.push(Occ {
                    idx: p,
                    gen: -(depth as i64) - 1,
                    role: Role::Anc,
                    x: 0.0,
                    again: false,
                });
                let e = extent(p, false);
                kids.push(anc(g, p, o, depth + 1, n, occs, blocks, extent, e));
            }
            blocks[slot].kids = kids;
        }
        slot
    }
    let anc_root = anc(g, c, 0, 0, n, &mut occs, &mut blocks, &mut extent, (cl, cr));

    // ---- the descendant tree: a person, their spouses, their unions' children
    let mut unions: Vec<UnionDrawn> = Vec::new();
    #[allow(clippy::too_many_arguments)]
    fn desc(
        g: &Graph,
        i: usize,
        occ: usize,
        depth: usize,
        n: usize,
        sees: &dyn Fn(usize) -> bool,
        occs: &mut Vec<Occ>,
        blocks: &mut Vec<Block>,
        unions: &mut Vec<UnionDrawn>,
        extent: &mut dyn FnMut(usize, bool) -> (f64, f64),
        sides: &mut Sides,
        guess_left: bool,
    ) -> usize {
        let drawn = if depth < n {
            g.unions_for(i, sees)
        } else {
            Vec::new()
        };
        let centre = depth == 0;
        let me = extent(i, centre);
        // The spouses of the drawn unions, beside the person: one to the
        // right; two either side; more continuing to the right.
        let mut spouse_occ: Vec<Option<usize>> = Vec::new();
        for (_, sp, _) in &drawn {
            spouse_occ.push(sp.map(|s| {
                let o = occs.len();
                occs.push(Occ {
                    idx: s,
                    gen: depth as i64,
                    role: if centre { Role::Spouse } else { Role::Dspouse },
                    x: 0.0,
                    again: false,
                });
                o
            }));
        }
        let sp_ext: Vec<Option<(f64, f64)>> = drawn
            .iter()
            .map(|(_, sp, _)| sp.map(|s| extent(s, false)))
            .collect();
        let mut members: Vec<(usize, f64, (f64, f64))> = vec![(occ, 0.0, me)];
        let shown: Vec<(usize, (f64, f64))> = spouse_occ
            .iter()
            .zip(&sp_ext)
            .filter_map(|(o, e)| o.zip(*e))
            .collect();
        // Which occurrence of this person each spouse's union hangs from.
        let mut beside: HashMap<usize, usize> = HashMap::new();
        if shown.len() >= 2 {
            let (o1, e1) = shown[0];
            members.insert(0, (o1, -(e1.1 - me.0 + PACK_GAP), e1));
            let mut x = 0.0;
            let mut prev = me;
            for (k, &(o, e)) in shown[1..].iter().enumerate() {
                // A person sits beside two spouses at most; from the third
                // on, they are drawn again, marked as a repeat, beside each —
                // otherwise their line to a later union must cross an
                // earlier spouse's.
                if k > 0 {
                    let again = occs.len();
                    occs.push(Occ {
                        idx: i,
                        gen: depth as i64,
                        role: Role::Desc,
                        x: 0.0,
                        again: true,
                    });
                    let me2 = extent(i, false);
                    x += prev.1 - me2.0 + PACK_GAP;
                    members.push((again, x, me2));
                    beside.insert(o, again);
                    prev = me2;
                }
                x += prev.1 - e.0 + PACK_GAP;
                members.push((o, x, e));
                prev = e;
            }
        } else if let Some(&(o, e)) = shown.first() {
            // One spouse goes on the side away from where this person's line
            // arrives from their parents' union, so that line never has to
            // pass under the spouse's own.
            let left_of = *sides.want.get(&occ).unwrap_or(&guess_left);
            sides.used.insert(occ, left_of);
            if left_of {
                let a = e.1 - me.0 + PACK_GAP;
                members = vec![(o, -a / 2.0, e), (occ, a / 2.0, me)];
            } else {
                let a = me.1 - e.0 + PACK_GAP;
                members = vec![(occ, -a / 2.0, me), (o, a / 2.0, e)];
            }
        }
        let left = members
            .iter()
            .map(|m| m.1 + m.2 .0)
            .fold(f64::INFINITY, f64::min);
        let right = members
            .iter()
            .map(|m| m.1 + m.2 .1)
            .fold(f64::NEG_INFINITY, f64::max);
        let slot = blocks.len();
        blocks.push(Block {
            members: members.iter().map(|m| (m.0, m.1)).collect(),
            left,
            right,
            row: depth as i64,
            kids: Vec::new(),
            tail: None,
        });
        if drawn.is_empty() {
            let cont = !g.kids_of[i].is_empty();
            let ext = if cont {
                tail_extent(g.desc[i] + 1, Some(g.desc[i]))
            } else {
                tail_extent(1, None)
            };
            blocks[slot].tail = Some((depth as i64 + 1, ext));
        }
        // Children's groups go left to right in the order of the partners
        // they were had with — a union with no partner drawn sits at the
        // person — so no parent's line to a union crosses another's.
        let at = |o: Option<usize>| -> f64 {
            o.and_then(|o| members.iter().find(|m| m.0 == o))
                .map_or_else(|| members.iter().find(|m| m.0 == occ).unwrap().1, |m| m.1)
        };
        let mut drawn: Vec<_> = drawn.into_iter().zip(spouse_occ).collect();
        drawn.sort_by(|a, b| at(a.1).total_cmp(&at(b.1)));
        let mut kid_blocks = Vec::new();
        for ((f, _, kids), sp) in drawn {
            let mut kid_occs = Vec::new();
            let m = kids.len();
            for (j, k) in kids.into_iter().enumerate() {
                let o = occs.len();
                occs.push(Occ {
                    idx: k,
                    gen: depth as i64 + 1,
                    role: Role::Desc,
                    x: 0.0,
                    again: false,
                });
                kid_occs.push(o);
                kid_blocks.push(desc(
                    g,
                    k,
                    o,
                    depth + 1,
                    n,
                    sees,
                    occs,
                    blocks,
                    unions,
                    extent,
                    sides,
                    2 * j + 1 < m,
                ));
            }
            unions.push(UnionDrawn {
                person: sp.and_then(|s| beside.get(&s).copied()).unwrap_or(occ),
                fam: f,
                spouse: sp,
                kids: kid_occs,
            });
        }
        blocks[slot].kids = kid_blocks;
        slot
    }

    // ---- place: pack each tree, put the centre on the fixed point, fit
    fn place(blocks: &[Block], root: usize, occs: &mut [Occ]) -> Vec<usize> {
        let packed = pack(blocks, root);
        let mut touched = Vec::new();
        for (b, ax) in packed.anchors {
            for &(o, dx) in &blocks[b].members {
                occs[o].x = ax + dx;
                touched.push(o);
            }
        }
        let off = CX - occs[0].x;
        for &o in &touched {
            occs[o].x += off;
        }
        touched
    }
    let anc_occs = place(&blocks, anc_root, &mut occs);

    // The descendant tree is built, packed, and built again with each lone
    // spouse moved to the side their partner's line does not arrive from,
    // until no side changes (the sides only move blocks within their row,
    // so this settles in a pass or two; three at most are run).
    let (occ_mark, block_mark) = (occs.len(), blocks.len());
    let mut sides = Sides::default();
    let mut desc_occs = Vec::new();
    for _ in 0..3 {
        occs.truncate(occ_mark);
        blocks.truncate(block_mark);
        unions.clear();
        sides.used.clear();
        let root = desc(
            g,
            c,
            0,
            0,
            n,
            sees,
            &mut occs,
            &mut blocks,
            &mut unions,
            &mut extent,
            &mut sides,
            false,
        );
        desc_occs = place(&blocks, root, &mut occs);
        let mut changed = false;
        for u in &unions {
            let mut ps = vec![occs[u.person].x];
            ps.extend(u.spouse.map(|s| occs[s].x));
            let pm = ps.iter().sum::<f64>() / ps.len() as f64;
            let (kl, kr) = u
                .kids
                .iter()
                .fold((f64::INFINITY, f64::NEG_INFINITY), |a, &k| {
                    (a.0.min(occs[k].x), a.1.max(occs[k].x))
                });
            let cx = pm * 0.5 + (kl + kr) / 4.0;
            for &k in &u.kids {
                if let Some(&was) = sides.used.get(&k) {
                    let want = cx > occs[k].x;
                    if want != was {
                        sides.want.insert(k, want);
                        changed = true;
                    }
                }
            }
        }
        if !changed {
            break;
        }
    }
    let hw = h_window(n);
    // Row 0 holds only the centre and their spouses: it keeps its packed
    // spacing through the fit, which would otherwise squeeze a large
    // family's spouse onto the centre's name.
    let row0: Vec<(usize, f64)> = desc_occs
        .iter()
        .filter(|&&o| occs[o].gen == 0)
        .map(|&o| (o, occs[o].x))
        .collect();
    for list in [&anc_occs, &desc_occs] {
        let mut xs: Vec<f64> = list.iter().map(|&o| occs[o].x).collect();
        {
            let mut refs: Vec<&mut f64> = xs.iter_mut().collect();
            fit(&mut refs, CX, hw.0 + 70.0, hw.0 + hw.1 - 44.0);
        }
        for (&o, x) in list.iter().zip(xs) {
            occs[o].x = x;
        }
    }
    for (o, x) in row0 {
        occs[o].x = x.clamp(hw.0 + 70.0, hw.0 + hw.1 - 44.0);
    }
    occs[0].x = CX;

    // ---- rows
    let mut frame = Frame {
        n,
        step,
        row_y: BTreeMap::new(),
        persons: Vec::new(),
    };
    let mut heads: BTreeMap<i64, BTreeSet<usize>> = BTreeMap::new();
    for o in &occs {
        heads.entry(o.gen).or_default().insert(o.idx);
    }
    frame.row_y = rows_y(&heads, n, step, shape.log_spacing);

    // ---- persons, one per occurrence; a person drawn twice is marked twice
    let mut seen: HashMap<usize, usize> = HashMap::new();
    let mut count: HashMap<usize, usize> = HashMap::new();
    let mut again: BTreeSet<usize> = BTreeSet::new();
    for o in &occs {
        if o.again {
            again.insert(o.idx);
        } else {
            *count.entry(o.idx).or_default() += 1;
        }
    }
    let keys: Vec<String> = occs
        .iter()
        .map(|o| {
            let k = seen.entry(o.idx).or_default();
            *k += 1;
            if *k == 1 {
                g.ids[o.idx].clone()
            } else {
                format!("{}~{k}", g.ids[o.idx])
            }
        })
        .collect();
    for (o, key) in occs.iter().zip(&keys) {
        let reps = count.get(&o.idx).copied().unwrap_or(1);
        frame.persons.push(PNode {
            id: g.ids[o.idx].clone(),
            key: key.clone(),
            x: o.x,
            y: frame.y(o.gen),
            role: o.role,
            lab: 0,
            room: 999.0,
            right: 999.0,
            repeat: if reps > 1 { reps } else { 0 },
            again: again.contains(&o.idx),
            idx: o.idx,
            gen: o.gen,
            d: g.desc[o.idx] + 1,
        });
    }
    if shape.hourglass {
        hourglass(&mut frame, 130.0, CX);
    }
    let xy = |o: usize| (frame.persons[o].x, frame.persons[o].y);

    // ---- couples and tails
    let mut couples: Vec<Couple> = Vec::new();
    let mut tails: Vec<Tail> = Vec::new();
    let year_est = estimate_years(g, c, &frame);
    let yr = |i: usize| -> f64 { year_est.get(&i).copied().unwrap_or(1900.0) };
    let mut fam_seen: HashMap<usize, usize> = HashMap::new();
    let mut fam_count: HashMap<usize, usize> = HashMap::new();
    // Ancestor unions: one per drawn child whose parents are drawn.
    let mut anc_unions: Vec<(usize, Vec<usize>)> = Vec::new();
    {
        fn walk(blocks: &[Block], b: usize, out: &mut Vec<(usize, Vec<usize>)>) {
            let child = blocks[b].members[0].0;
            if !blocks[b].kids.is_empty() {
                out.push((
                    child,
                    blocks[b]
                        .kids
                        .iter()
                        .map(|&k| blocks[k].members[0].0)
                        .collect(),
                ));
            }
            for &k in &blocks[b].kids {
                walk(blocks, k, out);
            }
        }
        walk(&blocks, anc_root, &mut anc_unions);
    }
    for (child, _) in &anc_unions {
        if let Some(f) = g.born_in[occs[*child].idx] {
            *fam_count.entry(f).or_default() += 1;
        }
    }
    for u in &unions {
        *fam_count.entry(u.fam).or_default() += 1;
    }
    let mut fam_key = |f: usize| -> (String, bool) {
        let k = fam_seen.entry(f).or_default();
        *k += 1;
        let key = if *k == 1 {
            g.fams[f].id.clone()
        } else {
            format!("{}~{k}", g.fams[f].id)
        };
        (key, fam_count.get(&f).copied().unwrap_or(1) > 1)
    };

    let mut has_parents_drawn: BTreeSet<usize> = BTreeSet::new();
    for (child, parents) in &anc_unions {
        has_parents_drawn.insert(*child);
        let o = &occs[*child];
        let i = o.idx;
        let Some(f) = g.born_in[i] else { continue };
        let fam = &g.fams[f];
        let me = xy(*child);
        let pm = parents.iter().map(|&p| xy(p).0).sum::<f64>() / parents.len() as f64;
        let cx = pm * 0.5 + me.0 * 0.5;
        let cy = me.1 + 0.62 * frame.gap_down(o.gen);
        let k = g.kid_conf(f, i);
        let d = g.family_discharge(f);
        let drawn = g.desc[i] + 1;
        let (key, repeat) = fam_key(f);
        couples.push(Couple {
            key: key.clone(),
            x: cx,
            y: cy,
            parents: parents.iter().map(|&p| keys[p].clone()).collect(),
            kids: vec![keys[*child].clone()],
            d,
            stub: fam.kids.iter().filter(|kk| kk.0 != i).count(),
            pconf: parents.iter().map(|&p| (keys[p].clone(), k)).collect(),
            kconf: [(keys[*child].clone(), k)].into_iter().collect(),
            stub_d: d.saturating_sub(drawn).max(1),
            stub_year: 1850.0,
            repeat,
            // Rises toward the child's row, ending 18 short of it so its
            // count clears the row's dots; to the right near the window's
            // left edge, otherwise to the left.
            stub_dx: if cx - hw.0 < 80.0 { 36.0 } else { -36.0 },
            stub_dy: -(cy - me.1 - 18.0).clamp(8.0, 32.0),
            stub_side: 0,
        });
        if parents.len() < 2 {
            let side = match parents.first().and_then(|&p| g.shape[occs[p].idx].slot) {
                Some(1) => -1.0,
                _ => 1.0,
            };
            tails.push(Tail {
                key: format!("m{key}"),
                x1: cx,
                y1: cy,
                x2: cx + side * 26f64.max(PACK_GAP * 4.0),
                y2: frame.y(o.gen - 1) + 14.0,
                d,
                kind: TailKind::Lost,
                dir: 1,
                count: 0,
                year: yr(i) - 28.0,
                who: i,
                offset: -28.0,
                count_side: 1,
            });
        }
    }
    // Ancestor leaves: the line continues off the frame, or the record ends.
    for &o in &anc_occs {
        if has_parents_drawn.contains(&o) {
            continue;
        }
        let oc = &occs[o];
        let (x, y) = xy(o);
        let i = oc.idx;
        let down = frame.gap_down(oc.gen);
        let cont = !g.parents_of(i).is_empty();
        tails.push(Tail {
            key: format!("{}{}", if cont { "c" } else { "l" }, keys[o]),
            x1: x,
            y1: y,
            x2: x,
            y2: y + if cont { 0.58 } else { 0.62 } * down,
            d: g.desc[i] + 1,
            kind: if cont { TailKind::Cont } else { TailKind::Lost },
            dir: 1,
            count: if cont { g.anc[i] } else { 0 },
            year: yr(i) - 20.0,
            who: i,
            offset: -20.0,
            count_side: 1,
        });
    }
    // Descendant unions.
    let mut with_kids: BTreeSet<usize> = BTreeSet::new();
    for u in &unions {
        with_kids.insert(u.person);
        let po = &occs[u.person];
        let fam = &g.fams[u.fam];
        let mut parents = vec![u.person];
        if let Some(s) = u.spouse {
            parents.push(s);
        }
        let pm = parents.iter().map(|&p| xy(p).0).sum::<f64>() / parents.len() as f64;
        let (kl, kr) = u
            .kids
            .iter()
            .fold((f64::INFINITY, f64::NEG_INFINITY), |a, &k| {
                (a.0.min(xy(k).0), a.1.max(xy(k).0))
            });
        let km = (kl + kr) / 2.0;
        let cx = pm * 0.5 + km * 0.5;
        let py = xy(u.person).1;
        let cy = py - 0.38 * frame.gap_up(po.gen);
        let d = g.family_discharge(u.fam);
        let best = u
            .kids
            .iter()
            .map(|&k| g.kid_conf(u.fam, occs[k].idx))
            .min()
            .unwrap_or(Conf::Attested);
        let (key, repeat) = fam_key(u.fam);
        couples.push(Couple {
            key: key.clone(),
            x: cx,
            y: cy,
            parents: parents.iter().map(|&p| keys[p].clone()).collect(),
            kids: u.kids.iter().map(|&k| keys[k].clone()).collect(),
            d,
            stub: 0,
            pconf: parents.iter().map(|&p| (keys[p].clone(), best)).collect(),
            kconf: u
                .kids
                .iter()
                .map(|&k| (keys[k].clone(), g.kid_conf(u.fam, occs[k].idx)))
                .collect(),
            stub_d: 1,
            stub_year: 1850.0,
            repeat,
            stub_dx: 0.0,
            stub_dy: 0.0,
            stub_side: 0,
        });
        if fam.parents.len() < 2 {
            tails.push(Tail {
                key: format!("m{key}"),
                x1: cx,
                y1: cy,
                x2: cx - 30.0,
                y2: py + 10.0,
                d,
                kind: TailKind::Lost,
                dir: 1,
                count: 0,
                year: yr(po.idx),
                who: po.idx,
                offset: 0.0,
                count_side: 1,
            });
        }
    }
    // Descendant leaves, the centre included: continued, or the record ends —
    // the same ring whether or not the person is living.
    for (o, oc) in occs.iter().enumerate() {
        if !matches!(oc.role, Role::Desc | Role::Centre) || with_kids.contains(&o) {
            continue;
        }
        let (x, y) = xy(o);
        let up = 0.55 * frame.gap_up(oc.gen);
        let i = oc.idx;
        let cont = !g.kids_of[i].is_empty();
        tails.push(Tail {
            key: format!("{}{}", if cont { "c" } else { "e" }, keys[o]),
            x1: x,
            y1: y,
            x2: x,
            y2: y - up,
            d: if cont { g.desc[i] + 1 } else { 1 },
            kind: if cont { TailKind::Cont } else { TailKind::Lost },
            dir: -1,
            count: if cont { g.desc[i] } else { 0 },
            year: yr(i) + if cont { 20.0 } else { 0.0 },
            who: i,
            offset: if cont { 20.0 } else { 0.0 },
            count_side: 1,
        });
    }
    // Spouses: their own line runs upstream, off the frame or to its end.
    for (o, oc) in occs.iter().enumerate() {
        if !matches!(oc.role, Role::Spouse | Role::Dspouse) {
            continue;
        }
        let (x, y) = xy(o);
        let i = oc.idx;
        let down = frame.gap_down(oc.gen);
        let cont = !g.parents_of(i).is_empty();
        tails.push(Tail {
            key: format!("{}{}", if cont { "c" } else { "l" }, keys[o]),
            x1: x,
            y1: y,
            x2: x,
            y2: y + SPOUSE_TAIL * down,
            d: g.desc[i] + 1,
            kind: if cont { TailKind::Cont } else { TailKind::Lost },
            dir: 1,
            count: if cont { g.anc[i] } else { 0 },
            year: yr(i) - 20.0,
            who: i,
            offset: -20.0,
            count_side: 1,
        });
    }

    // ---- tails stop short of any line they would cross
    clear_tails(&frame.persons, &couples, &mut tails);
    place_counts(&frame.persons, &mut couples, &mut tails);

    // ---- label density per row
    let mut rows: BTreeMap<i64, Vec<usize>> = BTreeMap::new();
    for (s, p) in frame.persons.iter().enumerate() {
        rows.entry(p.y.round() as i64).or_default().push(s);
    }
    for (_, mut r) in rows {
        r.sort_by(|a, b| frame.persons[*a].x.total_cmp(&frame.persons[*b].x));
        for (j, &s) in r.iter().enumerate() {
            let x = frame.persons[s].x;
            let left = if j > 0 {
                x - frame.persons[r[j - 1]].x
            } else {
                999.0
            };
            let right = if j + 1 < r.len() {
                frame.persons[r[j + 1]].x - x
            } else {
                999.0
            };
            frame.persons[s].lab = label_tier(left.min(right));
            frame.persons[s].room = left.min(right);
            frame.persons[s].right = right;
        }
    }

    let year = yr(c);
    let scale = year_scale(&frame, year);
    let rail = (-(n as i64)..=n as i64).map(|k| (k, frame.y(k))).collect();
    let years = frame.persons.iter().map(|p| yr(p.idx)).collect();
    Some(Layout {
        persons: frame.persons,
        couples,
        tails,
        meta: Meta {
            centre: centre.to_string(),
            n,
            step,
            year,
            cy: CY,
            scale,
            rail,
            era: true,
            bands: true,
            view: view_window(n, step),
            hview: h_window(n),
        },
        years,
    })
}

/// How far a spouse's own line runs upstream before it ends or leaves the
/// frame, as a fraction of a generation — at most: where a line arriving at
/// the row passes beneath it, [`clear_tails`] stops it short.
const SPOUSE_TAIL: f64 = 0.5;

/// The window of the frame a river at range `n` needs.
///
/// The step is capped at 96, so at ± 2 the rows reach only 192 units either
/// side of the centre, and a full 640-unit frame left about 40% of itself as
/// empty bands above and below. The geometry is unchanged — the centre stays
/// at (470, 318) and every coordinate means what it meant — and the viewBox
/// is cropped to how far the drawing can reach: the outermost row, plus the
/// longest tail that leaves it (0.55 of a step up, 0.62 down), plus 10 for an
/// arrowhead and its count or a ring. It depends on the range and the step
/// only, never on who is drawn, so the fixed point stays put on screen while
/// the river travels within a range. Where the step was not capped this is
/// (nearly) the whole frame.
pub fn view_window(n: usize, step: f64) -> (f64, f64) {
    let span = n as f64 * step;
    let top = (CY - span - 0.55 * step - 10.0).max(0.0);
    let bottom = (CY + span + 0.62 * step + 10.0).min(H);
    (r1(top), r1(bottom - top))
}

/// How far a river at range `n` usually reaches left and right of the fixed
/// point: the 85th percentile of [`drawn_extent`] over every centre of the
/// operator's 866-person bundle, each side on its own (measured by
/// `tests/river_crossings.rs`; rerun it to re-derive these).
const REACH: [(usize, f64, f64); 3] = [(2, 200.0, 389.0), (3, 297.0, 391.0), (5, 395.0, 391.0)];

/// The horizontal window of the frame a river at range `n` is drawn in:
/// `(left, width)`.
///
/// The vertical window ([`view_window`]) removed the empty bands above and
/// below a short river; this removes the empty frame beside a narrow one.
/// Like the vertical window it depends on the range only, never on who is
/// drawn: a crop per drawing would change the scale from one person to the
/// next and move the fixed point while the river travels. A river that
/// reaches further than most at its range is scaled down into the window by
/// [`fit`], as every river was into the whole frame before.
///
/// The window keeps 62 units left of the usual reach (70 from the edge to
/// the nearest dot, as before, less a dot's own 8.8) for the band years and
/// family stubs, and 34 right of it for the rail.
pub fn h_window(n: usize) -> (f64, f64) {
    let (_, l, r) = REACH
        .iter()
        .copied()
        .find(|&(k, _, _)| k >= n)
        .unwrap_or(REACH[REACH.len() - 1]);
    let left = (CX - l - 62.0).max(0.0);
    let right = (CX + r + 34.0).min(W);
    (r1(left), r1(right - left))
}

/// Label tier from a person's smaller neighbour gap: none where dots are too
/// close to label at all, otherwise the full name, which [`label_tiers`]
/// then shortens to what the measured text leaves room for. (Fixed gap
/// thresholds for the middle tiers predate contour packing, which spaces a
/// row by its labels' widths; they shortened names that fitted.)
pub fn label_tier(gap: f64) -> u8 {
    if gap < 46.0 {
        0
    } else {
        3
    }
}

/// Row y for each signed generation offset.
///
/// Uniform spacing is the reference's: `CY - gen * step`. Log spacing keeps
/// the outermost row of each side where uniform spacing puts it, and divides
/// the height between by `1 / ln(2 + people in the inner row)`, so the gap
/// after a sparse row is the taller one.
fn rows_y(
    heads: &BTreeMap<i64, BTreeSet<usize>>,
    n: usize,
    step: f64,
    log: bool,
) -> BTreeMap<i64, f64> {
    let mut out = BTreeMap::new();
    out.insert(0, CY);
    for dir in [1i64, -1] {
        let weights: Vec<f64> = (1..=n as i64)
            .map(|k| {
                let inner = heads
                    .get(&(dir * (k - 1)))
                    .map(BTreeSet::len)
                    .unwrap_or(1)
                    .max(1);
                if log {
                    1.0 / (2.0 + inner as f64).ln()
                } else {
                    1.0
                }
            })
            .collect();
        let total: f64 = weights.iter().sum();
        let span = n as f64 * step;
        let mut y = CY;
        for (k, w) in weights.iter().enumerate() {
            y -= dir as f64 * span * w / total;
            out.insert(dir * (k as i64 + 1), y);
        }
        // One more row beyond the frame, at a uniform step, for tails.
        out.insert(dir * (n as i64 + 1), y - dir as f64 * step);
    }
    out
}

/// Pack each generation toward the centre: its width comes from its own head
/// count, not from the span of what descends from it.
///
/// Descendant rows only. The ancestor tree already narrows toward the centre
/// — each person sits at the mean of their parents — and re-spacing its rows
/// breaks it wherever a pedigree has holes: when the oldest row holds only the
/// parents of the rightmost people in the row below, spacing it by rank drags
/// them to the far left and every line crosses the frame. On the operator's
/// bundle that turned the hourglass from a small gain into a net loss at ± 5.
///
/// Each row keeps its left-to-right order and is re-spaced evenly, `unit`
/// apart (less if the row would not fit), centred on the anchor. Because both
/// trees are drawn in an order every row agrees with, keeping each row's order
/// keeps the drawing free of new crossings; what changes is that a childless
/// first child no longer sits at the far end of a slot grid sized for its
/// siblings' grandchildren.
fn hourglass(frame: &mut Frame, sd_cap: f64, c0x: f64) {
    let mut rows: BTreeMap<i64, Vec<usize>> = BTreeMap::new();
    for (s, p) in frame.persons.iter().enumerate() {
        if p.gen > 0 {
            rows.entry(p.gen).or_default().push(s);
        }
    }
    for (_, mut slots) in rows {
        let (anchor, cap) = (c0x, sd_cap);
        slots.sort_by(|a, b| frame.persons[*a].x.total_cmp(&frame.persons[*b].x));
        let k = slots.len() as f64;
        let unit = cap.min((W - 150.0) / k);
        for (i, s) in slots.into_iter().enumerate() {
            frame.persons[s].x = anchor + (i as f64 - (k - 1.0) / 2.0) * unit;
        }
    }
}

/// A birth year for everyone drawn: the recorded one, or one estimated from
/// the generation. Redacted persons are coloured by the estimate too, which is
/// done at render time; this is the geometry's year.
fn estimate_years(g: &Graph, c: usize, frame: &Frame) -> HashMap<usize, f64> {
    let centre_year = g.shape[c]
        .birth_year
        .map(|y| y as f64)
        .or_else(|| {
            frame.persons.iter().find_map(|p| {
                g.shape[p.idx]
                    .birth_year
                    .map(|y| y as f64 - p.gen as f64 * YEARS_PER_GEN)
            })
        })
        .unwrap_or(1900.0);
    frame
        .persons
        .iter()
        .map(|p| {
            let y = g.shape[p.idx]
                .birth_year
                .map(|y| y as f64)
                .unwrap_or(centre_year + p.gen as f64 * YEARS_PER_GEN);
            (p.idx, y)
        })
        .collect()
}

/// The year scale through the generation rows.
fn year_scale(frame: &Frame, year: f64) -> YearScale {
    let n = frame.n as i64;
    let mut knots: Vec<(f64, f64)> = (-(n + 1)..=(n + 1))
        .map(|k| (year + k as f64 * YEARS_PER_GEN, frame.y(k)))
        .collect();
    knots.sort_by(|a, b| a.0.total_cmp(&b.0));
    YearScale { knots }
}

// ---------------------------------------------------------------------------
// What the reader sees
// ---------------------------------------------------------------------------

/// The words the river draws, in the reader's language.
#[derive(Debug, Clone)]
pub struct Words {
    /// "VIVANTS": the band from 1950 on.
    pub living_band: String,
    /// The circa prefix: "c." / "v.".
    pub circa: String,
}

/// One person, as this reader may see them, ready to draw.
#[derive(Debug, Clone, Serialize)]
pub struct Shown {
    pub id: String,
    /// Label per tier: `[given, given + initial, full]`. Empty when redacted.
    pub names: [String; 3],
    pub years: String,
    pub colour: String,
    pub living: bool,
    pub sparse: bool,
    pub redacted: bool,
}

fn shown_for(label: &RiverLabel, id: &str, year: f64, words: &Words) -> Shown {
    if label.redacted {
        return Shown {
            id: id.to_string(),
            names: Default::default(),
            years: String::new(),
            colour: colour_at(year),
            living: false,
            sparse: false,
            redacted: true,
        };
    }
    let initial = label
        .surname
        .chars()
        .next()
        .map(|c| format!("{} {c}.", label.given))
        .unwrap_or_else(|| label.given.clone());
    let b = label
        .birth_year
        .map(|y| y.to_string())
        .unwrap_or_else(|| "?".into());
    let years = if label.circa {
        format!("{}{b}", words.circa)
    } else {
        let d = match (label.death_year, label.living) {
            (Some(d), _) => d.to_string(),
            (None, true) => String::new(),
            (None, false) => "?".into(),
        };
        format!("{b}–{d}")
    };
    Shown {
        id: id.to_string(),
        names: [label.given.clone(), initial, label.full.clone()],
        years,
        colour: colour_at(year),
        living: label.living,
        sparse: label.sparse,
        redacted: false,
    }
}

/// Take every year a person this reader may not see would otherwise lend the
/// drawing out of it: colours of nodes, tails and stubs, and the band scale,
/// whose half-century lines sit wherever the centre's birth year puts them.
///
/// The scale is anchored on the centre's recorded birth year when the reader
/// may see the centre and it has one; otherwise on the nearest person in the
/// frame the reader may see who has one, shifted by 29 years a generation.
/// Every other person — hidden, or visible but undated — is coloured by that
/// anchor plus their generation. With no anchor at all the river has no era:
/// it draws no bands, every colour is the one neutral ink, and no year,
/// estimated or otherwise, reaches the page. (It used to round the hidden
/// centre's own year to the century, which both drew a wrong era and put that
/// person's century in the payload.)
fn redact_years(l: &mut Layout, flat: &Value, lens: &Lens) {
    let seen: Vec<bool> = l
        .persons
        .iter()
        .map(|p| lens.sees_person(&p.id) && access::river_shape(flat, &p.id).birth_year.is_some())
        .collect();
    let anchor = l
        .persons
        .iter()
        .zip(&l.years)
        .zip(&seen)
        .filter(|(_, &s)| s)
        .min_by_key(|((p, _), _)| (p.gen.abs(), p.role != Role::Centre))
        .map(|((p, &y), _)| y - p.gen as f64 * YEARS_PER_GEN);
    let Some(year) = anchor else {
        l.meta.era = false;
        l.meta.bands = false;
        l.meta.year = 0.0;
        l.meta.scale.knots.clear();
        l.years.iter_mut().for_each(|y| *y = 0.0);
        l.tails.iter_mut().for_each(|t| t.year = 0.0);
        l.couples.iter_mut().for_each(|c| c.stub_year = 0.0);
        return;
    };
    let shift = year - l.meta.year;
    l.meta.year = year;
    for k in l.meta.scale.knots.iter_mut() {
        k.0 += shift;
    }
    let mut by_idx: HashMap<usize, f64> = HashMap::new();
    for (i, p) in l.persons.iter().enumerate() {
        if !seen[i] {
            l.years[i] = year + p.gen as f64 * YEARS_PER_GEN;
        }
        by_idx.insert(p.idx, l.years[i]);
    }
    for t in l.tails.iter_mut() {
        if let Some(y) = by_idx.get(&t.who) {
            t.year = y + t.offset;
        }
    }
    let by_id: HashMap<&str, f64> = l
        .persons
        .iter()
        .zip(&l.years)
        .map(|(p, &y)| (p.key.as_str(), y))
        .collect();
    for c in l.couples.iter_mut() {
        c.stub_year = c
            .parents
            .first()
            .and_then(|p| by_id.get(p.as_str()))
            .map(|y| y + 28.0)
            .unwrap_or(year + 28.0);
    }
}

/// The client's copy of a layout: geometry plus what this reader may see.
#[derive(Debug, Clone, Serialize)]
pub struct Payload {
    pub layout: Layout,
    pub shown: Vec<Shown>,
    /// Keyboard targets: `↓` and `↑`.
    pub down: Option<String>,
    pub up: Option<String>,
    pub words: [String; 2],
}

/// Everything one request needs: the layout, and the people as this reader
/// sees them.
pub struct River {
    pub layout: Layout,
    pub shown: Vec<Shown>,
    pub words: Words,
    pub down: Option<String>,
    pub up: Option<String>,
}

/// Lay out and resolve the river for one reader.
// The reader is three arguments — lens, words, signed in — and the layout
// three more; bundling them would only rename the list.
#[allow(clippy::too_many_arguments)]
pub fn build(
    flat: &Value,
    lens: &Lens,
    g: &Graph,
    centre: &str,
    n: usize,
    shape: Shape,
    words: Words,
    signed_in: bool,
) -> Option<River> {
    // Children ordered for this reader: by date only where they may see the
    // whole sibship.
    let sees = |i: usize| lens.sees_person(&g.ids[i]);
    // Each label's room is what this reader's label for that person needs:
    // nothing for a person they may not see.
    let label_w = |i: usize| -> f64 {
        let l = access::river_label(flat, lens, &g.ids[i]);
        if l.redacted {
            return 0.0;
        }
        let s = shown_for(&l, &g.ids[i], 1900.0, &words);
        if g.ids[i] == centre {
            // The centre is set in its own, larger face and shows its full
            // name where there is room; a name long enough to need more
            // than CENTRE_LABEL_MAX is clipped instead of pushing the
            // spouse across the frame.
            return label_width(&s.names[2], true)
                .min(CENTRE_LABEL_MAX)
                .max(text_width(&s.years, 11.0, true, false));
        }
        // Room for the full name where the frame has it; the fit and the
        // label tiers shorten it where it does not. One record holds a
        // sentence in a name, so no label asks for more than LABEL_MAX.
        label_width(&s.names[2], false)
            .min(LABEL_MAX)
            .max(text_width(&s.years, 9.5, true, false))
    };
    let mut layout = layout_for(g, &sees, &label_w, centre, n, shape)?;
    redact_years(&mut layout, flat, lens);
    // A signed-out visitor gets no bands at all.
    layout.meta.bands = layout.meta.era && signed_in;
    let era = layout.meta.era;
    let shown = layout
        .persons
        .iter()
        .zip(&layout.years)
        .map(|(p, &year)| {
            let label = access::river_label(flat, lens, &p.id);
            // A redacted person's year is already the generation's estimate.
            let mut s = shown_for(&label, &p.id, year, &words);
            if !era {
                s.colour = NEUTRAL.to_string();
            }
            // A person drawn more than once says why, at the end of their
            // years line (the ring on the dot says it where the label is
            // dropped): `×n` for n lines reaching them, `↔` for a copy
            // beside a later partner. See [`repeat_marks`].
            if !s.redacted {
                s.years.push_str(&repeat_marks(p));
            }
            s
        })
        .collect();
    let c = g.index_of(centre)?;
    let parents = g.parents_of(c);
    let down = parents
        .iter()
        .find(|&&p| g.shape[p].slot == Some(0))
        .or_else(|| parents.first())
        .map(|&p| g.ids[p].clone());
    // ↑: the first child of the first union, in the order they are drawn.
    let up = g
        .unions_for(c, &sees)
        .first()
        .and_then(|u| u.2.first())
        .map(|&k| g.ids[k].clone());
    Some(River {
        layout,
        shown,
        words,
        down,
        up,
    })
}

impl River {
    /// The client's copy.
    pub fn payload(&self) -> Payload {
        Payload {
            layout: self.layout.clone(),
            shown: self.shown.clone(),
            down: self.down.clone(),
            up: self.up.clone(),
            words: [self.words.living_band.clone(), self.words.circa.clone()],
        }
    }

    /// The still frame, as SVG.
    pub fn svg(&self, aria_label: &str) -> String {
        render_svg(&self.layout, &self.shown, &self.words, aria_label)
    }
}

/// Write one still frame.
///
/// `client.js` carries a port of this function for the frames between two
/// layouts; a change here is a change there.
pub fn render_svg(l: &Layout, shown: &[Shown], words: &Words, aria_label: &str) -> String {
    let mut defs = String::new();
    let mut out = String::new();
    let mut gi = 0usize;
    // Edges name the occurrence they join, by key: a person drawn twice is
    // two nodes.
    let by_id: HashMap<&str, usize> = l
        .persons
        .iter()
        .enumerate()
        .map(|(i, p)| (p.key.as_str(), i))
        .collect();
    let col = |year: f64| -> String {
        if l.meta.era {
            colour_at(year)
        } else {
            NEUTRAL.to_string()
        }
    };
    let colour_of = |id: &str| -> String {
        by_id
            .get(id)
            .map(|&i| shown[i].colour.clone())
            .unwrap_or_else(|| col(l.meta.year))
    };

    // ---- half-century bands, left rail
    let (v_top, v_h) = l.meta.view;
    let v_bottom = v_top + v_h;
    let (h_left, h_w) = l.meta.hview;
    let mut living_named = false;
    // No bands without an era, nor for a signed-out reader (`Meta::bands`).
    let mut y0 = if l.meta.bands { 1600 } else { 2050 };
    while y0 < 2050 {
        let ya = l.meta.scale.y_of(y0 as f64);
        let yb = l.meta.scale.y_of((y0 + 50) as f64);
        if !(yb > v_bottom || ya < v_top) {
            let live = y0 >= 1950;
            let t = yb.max(v_top);
            let b = ya.min(v_bottom);
            let fill = if live {
                BAND_LIVE
            } else if (y0 / 50) % 2 == 1 {
                BAND_ODD
            } else {
                BG
            };
            let _ = write!(
                out,
                r#"<rect x="{}" y="{}" width="{}" height="{}" fill="{fill}"/>"#,
                num(h_left),
                num(t),
                num(h_w),
                num(b - t)
            );
            if ya <= v_bottom {
                let _ = write!(
                    out,
                    r#"<line x1="{x1}" y1="{y}" x2="{x2}" y2="{y}" stroke="{BAND_RULE}"/>"#,
                    x1 = num(h_left),
                    y = num(ya),
                    x2 = num(h_left + h_w),
                );
                // The year sits above its line; where that is above the
                // window, it would be cut in half, so it is left out.
                if ya - 6.0 - 10.0 >= v_top {
                    let _ = write!(
                        out,
                        r#"<text x="{tx}" y="{ty}" font-family="{MONO}" font-size="10" fill="{tf}">{y0}</text>"#,
                        tx = num(h_left + 10.0),
                        ty = num(ya - 6.0),
                        tf = if live { BAND_TEXT_LIVE } else { BAND_TEXT }
                    );
                }
            }
            // The living band's name, once, just inside the band's top, and
            // only where it clears the band's own year label below it. The
            // reference placed it at max(16, top + 16) for every living band,
            // which on a river whose 2000 line sits near the top stacked two
            // of them on the year.
            let label_y = t + 16.0;
            if live && !living_named && label_y < b - 6.0 - 12.0 {
                living_named = true;
                let _ = write!(
                    out,
                    r#"<text x="{}" y="{}" font-family="{MONO}" font-size="10" letter-spacing="1" fill="{BAND_TEXT_LIVE}">{}</text>"#,
                    num(h_left + 10.0),
                    num(label_y),
                    html_escape(&words.living_band)
                );
            }
        }
        y0 += 50;
    }

    // ---- right rail: generations relative to the centre
    for &(k, y) in &l.meta.rail {
        if !(v_top + 8.0..=v_bottom - 4.0).contains(&y) {
            continue;
        }
        let label = match k.cmp(&0) {
            std::cmp::Ordering::Greater => format!("+{k}"),
            std::cmp::Ordering::Equal => "0 ◂".into(),
            std::cmp::Ordering::Less => format!("−{}", -k),
        };
        let _ = write!(
            out,
            r#"<text x="{}" y="{}" text-anchor="end" font-family="{MONO}" font-size="10" fill="{}">{label}</text>"#,
            num(h_left + h_w - 10.0),
            num(y + 3.5),
            if k == 0 { ACCENT } else { BAND_TEXT }
        );
    }

    // ---- tails
    for t in &l.tails {
        let c = col(t.year);
        let w = num(width_of(t.d));
        let d = curve(t.x1, t.y1, t.x2, t.y2);
        match t.kind {
            TailKind::Cont => {
                let dir = f64::from(t.dir);
                let _ = write!(
                    out,
                    r#"<g class="rv-tail"><path d="{d}" fill="none" stroke="{c}" stroke-width="{w}"/><polygon points="{},{} {},{} {},{}" fill="{c}"/>"#,
                    num(t.x2 - 4.0),
                    num(t.y2),
                    num(t.x2 + 4.0),
                    num(t.y2),
                    num(t.x2),
                    num(t.y2 + dir * 6.0),
                );
                if t.count_side != 0 {
                    let side = f64::from(t.count_side);
                    let _ = write!(
                        out,
                        r#"<text x="{}" y="{}"{} font-family="{MONO}" font-size="9.5" fill="{DATA_TEXT}">+{}</text>"#,
                        num(t.x2 + side * 8.0),
                        num(t.y2 + if t.dir > 0 { 4.0 } else { 2.0 }),
                        if side < 0.0 {
                            r#" text-anchor="end""#
                        } else {
                            ""
                        },
                        t.count
                    );
                }
                out.push_str("</g>");
            }
            TailKind::Lost => {
                let id = format!("g{gi}");
                gi += 1;
                let _ = write!(
                    defs,
                    r#"<linearGradient id="{id}" gradientUnits="userSpaceOnUse" x1="{}" y1="{}" x2="{}" y2="{}"><stop offset="0" stop-color="{c}" stop-opacity="0.85"/><stop offset="1" stop-color="{c}" stop-opacity="0"/></linearGradient>"#,
                    num(t.x1),
                    num(t.y1),
                    num(t.x2),
                    num(t.y2)
                );
                let _ = write!(
                    out,
                    r#"<g class="rv-tail"><path d="{d}" fill="none" stroke="url(#{id})" stroke-width="{w}"/><circle cx="{}" cy="{}" r="3.4" fill="{BG}" stroke="{c}" stroke-width="1.4"/></g>"#,
                    num(t.x2),
                    num(t.y2)
                );
            }
        }
    }

    // ---- edges, widest first
    struct E {
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        d: usize,
        conf: Conf,
        colour: String,
    }
    let mut edges: Vec<E> = Vec::new();
    for c in &l.couples {
        for pid in &c.parents {
            if let Some(&i) = by_id.get(pid.as_str()) {
                let p = &l.persons[i];
                edges.push(E {
                    x1: p.x,
                    y1: p.y,
                    x2: c.x,
                    y2: c.y,
                    d: c.d,
                    conf: c.pconf.get(pid).copied().unwrap_or(Conf::Attested),
                    colour: colour_of(pid),
                });
            }
        }
        for kid in &c.kids {
            if let Some(&i) = by_id.get(kid.as_str()) {
                let k = &l.persons[i];
                edges.push(E {
                    x1: c.x,
                    y1: c.y,
                    x2: k.x,
                    y2: k.y,
                    d: k.d,
                    conf: c.kconf.get(kid).copied().unwrap_or(Conf::Attested),
                    colour: colour_of(kid),
                });
            }
        }
        if c.stub > 0 {
            let (x2, y2) = (c.x + c.stub_dx, c.y + c.stub_dy);
            let base = c.stub_year;
            let _ = write!(
                out,
                r#"<g class="rv-stub"><path d="{}" fill="none" stroke="{}" stroke-opacity="0.45" stroke-width="{}"/>"#,
                curve(c.x, c.y, x2, y2),
                col(base),
                num(width_of(c.stub_d)),
            );
            if c.stub_side != 0 {
                let side = f64::from(c.stub_side);
                let _ = write!(
                    out,
                    r#"<text x="{}" y="{}" text-anchor="{}" font-family="{MONO}" font-size="10" fill="{DATA_TEXT}">+{}</text>"#,
                    num(x2 + side * 5.0),
                    num(y2 - 3.0),
                    if side < 0.0 { "end" } else { "start" },
                    c.stub
                );
            }
            out.push_str("</g>");
        }
    }
    edges.sort_by_key(|e| std::cmp::Reverse(e.d));
    for e in &edges {
        let d = curve(e.x1, e.y1, e.x2, e.y2);
        let wf = width_of(e.d);
        let w = num(wf);
        let c = &e.colour;
        match e.conf {
            Conf::Attested => {
                let _ = write!(
                    out,
                    r#"<path d="{d}" fill="none" stroke="{c}" stroke-width="{w}" stroke-linecap="round"/>"#
                );
            }
            Conf::Inferred => {
                let _ = write!(
                    out,
                    r#"<path d="{d}" fill="none" stroke="{c}" stroke-width="{w}" stroke-dasharray="{} {}"/>"#,
                    num(4f64.max(wf * 1.3)),
                    num(3f64.max(wf * 0.6))
                );
            }
            Conf::Speculative => {
                let _ = write!(
                    out,
                    r#"<g><path d="{d}" fill="none" stroke="{c}" stroke-width="{w}" stroke-opacity="0.14"/><path d="{d}" fill="none" stroke="{c}" stroke-width="2.6" stroke-linecap="round" stroke-dasharray="0.1 5.5"/></g>"#
                );
            }
        }
    }

    // The lit route is drawn by the client, into this empty group.
    out.push_str(r#"<g class="rv-lit"></g>"#);

    // ---- persons
    let tiers = label_tiers(l, shown);
    for (pi, (p, s)) in l.persons.iter().zip(shown).enumerate() {
        let is_c = p.role == Role::Centre;
        let r = if is_c { 7.5 } else { 4.8 };
        let (x, y) = (num(p.x), num(p.y));
        // A link, so that the river travels without JavaScript too: the page
        // re-centred on this person, at the same range.
        let _ = write!(
            out,
            r#"<a class="rv-p" data-id="{}" data-key="{}" href="{RIVER_PATH}?p={}&amp;n={}"><circle cx="{x}" cy="{y}" r="20" fill="transparent"/>"#,
            html_escape(&p.id),
            html_escape(&p.key),
            url_component(&p.id),
            l.meta.n
        );
        // A person drawn more than once is ringed: a solid ring where two
        // or more lines reach them (pedigree collapse), a dashed ring where
        // they are drawn again beside a later partner. The years line says
        // which in words-free marks too (`repeat_marks`), and the legend
        // names both.
        if p.repeat > 1 {
            let _ = write!(
                out,
                r#"<circle class="rv-repeat" cx="{x}" cy="{y}" r="{}" fill="none" stroke="{DATA_TEXT}" stroke-width="1"/>"#,
                num(r + REPEAT_RING)
            );
        }
        if p.again {
            let _ = write!(
                out,
                r#"<circle class="rv-again" cx="{x}" cy="{y}" r="{}" fill="none" stroke="{DATA_TEXT}" stroke-width="1" stroke-dasharray="2.4 1.8"/>"#,
                num(r + REPEAT_RING + if p.repeat > 1 { 2.2 } else { 0.0 })
            );
        }
        if s.living {
            let _ = write!(
                out,
                r#"<circle cx="{x}" cy="{y}" r="{}" fill="{LIVING_GLOW}" fill-opacity="0.16"/>"#,
                num(r + 4.0)
            );
        }
        if is_c {
            let _ = write!(
                out,
                r#"<circle cx="{x}" cy="{y}" r="14" fill="none" stroke="{ACCENT}" stroke-width="1.2"/><circle cx="{x}" cy="{y}" r="{}" fill="{ACCENT}"/>"#,
                num(r)
            );
        } else if s.sparse || s.redacted {
            let _ = write!(
                out,
                r#"<circle cx="{x}" cy="{y}" r="{}" fill="{BG}" stroke="{}" stroke-width="1.6" stroke-dasharray="1.6 1.6"/>"#,
                num(r),
                s.colour
            );
        } else {
            let _ = write!(
                out,
                r#"<circle cx="{x}" cy="{y}" r="{}" fill="{}" stroke="{BG}" stroke-width="1.6"/>"#,
                num(r),
                s.colour
            );
        }
        let tier = tiers[pi];
        if tier > 0 {
            let name = &s.names[(tier as usize).clamp(1, 3) - 1];
            // The centre always labels; when even its given name is longer
            // than the room it has, it is cut to fit.
            let clipped;
            let name = if is_c {
                let start = p.x + r + 10.0;
                clipped = clip_label(
                    name,
                    (p.right - LABEL_START_C - NEIGHBOUR).min(l.meta.label_edge() - start),
                );
                &clipped
            } else {
                name
            };
            let halo = format!(
                r#"paint-order="stroke" stroke="{BG}" stroke-width="4" stroke-linejoin="round""#
            );
            let lx = num(p.x + r + if is_c { 10.0 } else { 6.0 });
            let _ = write!(
                out,
                r#"<text x="{lx}" y="{}" font-family="{SERIF}" font-size="{}" font-weight="{}" fill="{}" {halo}>{}</text><text x="{lx}" y="{}" font-family="{MONO}" font-size="{}" fill="{DATA_TEXT}" {halo}>{}</text>"#,
                num(p.y - 1.0),
                if is_c { 16 } else { 12 },
                if is_c { 600 } else { 400 },
                if is_c { CENTRE_TEXT } else { NAME_TEXT },
                html_escape(name),
                num(p.y + if is_c { 14.0 } else { 11.5 }),
                if is_c { "11" } else { "9.5" },
                html_escape(&s.years)
            );
        }
        out.push_str("</a>");
    }

    format!(
        r#"<svg class="rv-svg" viewBox="{left} {top} {w} {h}" width="100%" role="img" aria-label="{a}" xmlns="http://www.w3.org/2000/svg"><defs>{defs}</defs><rect x="{left}" y="{top}" width="{w}" height="{h}" fill="{BG}"/>{out}</svg>"#,
        left = num(h_left),
        top = num(v_top),
        w = num(h_w),
        h = num(v_h),
        a = html_escape(aria_label)
    )
}

/// Step a label down a tier while it would run past the right rail.
///
/// Near the right edge the frame used to cut names off ("Jarosław Medy…" on
/// the operator's bundle). Drawing those labels to the left of the dot
/// instead ran them into their left neighbour's, so a label keeps to the
/// right, as the reference draws it, and shortens instead: there is never a
/// neighbour further right for it to meet. The centre shortens too — a long
/// enough name reached the rail from the fixed point — but never below its
/// given name.
pub fn fit_edge(tier: u8, s: &Shown, x: f64, r: f64, centre: bool, edge: f64) -> u8 {
    let start = x + r + if centre { 10.0 } else { 6.0 };
    let years = text_width(&s.years, if centre { 11.0 } else { 9.5 }, true, false);
    // The centre always labels: it keeps at least its given name.
    let floor = u8::from(centre);
    let mut t = tier;
    while t > floor && start + label_width(&s.names[t as usize - 1], centre).max(years) > edge {
        t -= 1;
    }
    t
}

/// Percent-encode everything but the unreserved characters.
pub fn url_component(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.bytes() {
        if b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.' | b'~') {
            out.push(b as char);
        } else {
            let _ = write!(out, "%{b:02X}");
        }
    }
    out
}

/// How wide a run of text is, estimated: 0.55 em a character in the serif,
/// 0.6 em bold or monospace, a full em for wide scripts. `tests/river_text.rs`
/// holds every text node to the frame with an estimate of its own.
pub fn text_width(text: &str, size: f64, mono: bool, bold: bool) -> f64 {
    let latin = if mono || bold { 0.6 } else { 0.55 };
    text.chars()
        .map(|c| size * if is_wide(c) { 1.0 } else { latin })
        .sum()
}

/// East Asian wide characters: a full em each.
fn is_wide(c: char) -> bool {
    matches!(c as u32,
        0x1100..=0x115F | 0x2E80..=0xA4CF | 0xAC00..=0xD7A3 | 0xF900..=0xFAFF
        | 0xFE30..=0xFE4F | 0xFF00..=0xFF60 | 0xFFE0..=0xFFE6)
}

/// A name label's width: 12-unit serif, or the centre's 16-unit bold.
fn label_width(text: &str, centre: bool) -> f64 {
    if centre {
        text_width(text, 16.0, false, true)
    } else {
        text_width(text, 12.0, false, false)
    }
}

/// How far outside a dot its repeat ring is drawn.
pub const REPEAT_RING: f64 = 2.6;

/// The outermost circle drawn for a person: the centre's ring, a repeat
/// ring, or the dot itself.
pub fn outer_radius(p: &PNode) -> f64 {
    if p.role == Role::Centre {
        return 14.0;
    }
    let ring = if p.again && p.repeat > 1 {
        REPEAT_RING + 2.2
    } else if p.again || p.repeat > 1 {
        REPEAT_RING
    } else {
        0.0
    };
    4.8 + ring
}

/// What a label must leave before the next dot in its row: that dot's radius
/// and the label's halo.
const NEIGHBOUR: f64 = 4.8 + 2.0;
/// Where a label starts, from its dot's centre: radius plus gap.
const LABEL_START: f64 = 4.8 + 6.0;
const LABEL_START_C: f64 = 7.5 + 10.0;

/// Step a label down a tier while it would run into its neighbour.
///
/// The tier rule reads the gap only, so a long full name in a 140-unit gap
/// overprinted the next person's. `gap` is the distance to the right-hand
/// neighbour's dot, so the label has that less its own start and the
/// neighbour's radius ([`NEIGHBOUR`]) — measured from the dot's centre it
/// once ran into the next dot (`tests/river_text.rs`, text against circles).
/// The years line counts as well as the name. A label that
/// does not fit even as a given name is dropped rather than overprinted: the
/// person still labels on hover, which is where a reader looks for it. The
/// centre always labels, so it stops at its given name.
pub fn fit_tier(tier: u8, s: &Shown, gap: f64, centre: bool) -> u8 {
    let years = text_width(&s.years, if centre { 11.0 } else { 9.5 }, true, false);
    let floor = u8::from(centre);
    let mut t = tier;
    let room = gap - if centre { LABEL_START_C } else { LABEL_START } - NEIGHBOUR;
    while t > floor && label_width(&s.names[t as usize - 1], centre).max(years) > room {
        t -= 1;
    }
    t
}

/// How far the drawing reaches left and right of the fixed point, in viewBox
/// units: dots, tails and their counts, family stubs and their counts, and
/// every label at the tier it is drawn at.
pub fn drawn_extent(l: &Layout, shown: &[Shown]) -> (f64, f64) {
    let (mut lo, mut hi) = (CX, CX);
    let mut take = |a: f64, b: f64| {
        lo = lo.min(a);
        hi = hi.max(b);
    };
    let tiers = label_tiers(l, shown);
    for ((p, s), &t) in l.persons.iter().zip(shown).zip(&tiers) {
        let centre = p.role == Role::Centre;
        let r = if centre { 7.5 } else { 4.8 };
        take(p.x - r - 4.0, p.x + r);
        if t > 0 {
            let years = text_width(&s.years, if centre { 11.0 } else { 9.5 }, true, false);
            let start = p.x + r + if centre { 10.0 } else { 6.0 };
            let end = start + label_width(&s.names[t as usize - 1], centre).max(years);
            // The centre's name is clipped at the label edge when drawn.
            take(
                start,
                if centre {
                    end.min(l.meta.label_edge())
                } else {
                    end
                },
            );
        }
    }
    for t in &l.tails {
        let count = if t.kind == TailKind::Cont && t.count_side != 0 {
            8.0 + text_width(&format!("+{}", t.count), 9.5, true, false)
        } else {
            4.0
        };
        let (l_, r_) = if t.count_side < 0 {
            (count, 4.0)
        } else {
            (4.0, count)
        };
        take(t.x1.min(t.x2 - l_), t.x1.max(t.x2 + r_));
    }
    for c in &l.couples {
        take(c.x, c.x);
        if c.stub > 0 {
            let side = f64::from(c.stub_side);
            let w = 5.0 + text_width(&format!("+{}", c.stub), 10.0, true, false);
            let end = c.x + c.stub_dx;
            take(end.min(end + side * w), end.max(end + side * w));
        }
    }
    (CX - lo, hi - CX)
}

/// The most room the layout keeps for any other label.
const LABEL_MAX: f64 = 100.0;

/// The most room the layout keeps for the centre's label.
const CENTRE_LABEL_MAX: f64 = 200.0;

/// The years-line suffix for a person drawn more than once: ` ×n` when n
/// lines reach them (pedigree collapse), ` ↔` when they are drawn again
/// beside a later partner; both when both.
pub fn repeat_marks(p: &PNode) -> String {
    let mut out = String::new();
    if p.repeat > 1 {
        let _ = write!(out, " ×{}", p.repeat);
    }
    if p.again {
        out.push_str(" ↔");
    }
    out
}

/// The centre's name, cut with an ellipsis to fit `room`.
///
/// Only for the centre, which always labels. A given name is normally short;
/// one record on the operator's bundle holds a sentence in it ("Anna - w
/// dokumentach wystepuje także pod tym imieniem"), which ran 130 units past
/// the frame from the fixed point.
pub fn clip_label(name: &str, room: f64) -> String {
    if label_width(name, true) <= room {
        return name.to_string();
    }
    let mut out = String::new();
    for c in name.chars() {
        let next = format!("{out}{c}…");
        if label_width(&next, true) > room {
            break;
        }
        out.push(c);
    }
    format!("{}…", out.trim_end())
}

/// Every person's label tier, resolved row by row, left to right.
///
/// A label is fitted to the gap to its right-hand neighbour, counting its
/// years line as well as its name, and to the right rail; the centre shortens
/// too but keeps at least its given name. Then a label that would start
/// inside the previous label in its row is dropped — the previous one may be
/// the centre's, which cannot be — so no two labels overlap
/// (`tests/river_text.rs`, the pairwise check).
pub fn label_tiers(l: &Layout, shown: &[Shown]) -> Vec<u8> {
    let mut tiers = vec![0u8; l.persons.len()];
    let mut rows: BTreeMap<i64, Vec<usize>> = BTreeMap::new();
    for (i, p) in l.persons.iter().enumerate() {
        rows.entry(p.y.round() as i64).or_default().push(i);
    }
    for (_, mut row) in rows {
        row.sort_by(|a, b| l.persons[*a].x.total_cmp(&l.persons[*b].x));
        let mut last_end = f64::NEG_INFINITY;
        for (j, &i) in row.iter().enumerate() {
            let (p, s) = (&l.persons[i], &shown[i]);
            // The room runs to the right-hand neighbour's dot; where that dot
            // is ringed (a repeat, or the centre's ring), to its ring.
            let wider = row
                .get(j + 1)
                .map_or(0.0, |&k| outer_radius(&l.persons[k]) - 4.8);
            let centre = p.role == Role::Centre;
            let r = if centre { 7.5 } else { 4.8 };
            let start = p.x + r + if centre { 10.0 } else { 6.0 };
            let mut t = if s.redacted {
                0
            } else if centre {
                3
            } else {
                p.lab
            };
            if t > 0 {
                t = fit_edge(
                    fit_tier(t, s, p.right - wider, centre),
                    s,
                    p.x,
                    r,
                    centre,
                    l.meta.label_edge(),
                );
            }
            if t > 0 && !centre && start < last_end + 4.0 {
                t = 0;
            }
            if t > 0 {
                let years = text_width(&s.years, if centre { 11.0 } else { 9.5 }, true, false);
                last_end = start + label_width(&s.names[t as usize - 1], centre).max(years);
            }
            tiers[i] = t;
        }
    }
    tiers
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// A person: id, given, surname, gender, birth year, living.
    fn person(id: &str, given: &str, surname: &str, g: &str, born: i64, living: bool) -> Value {
        json!({"id": id, "type": "person", "axgf_version": "1.1",
               "identity": {"name": {"display": format!("{given} {surname}"),
                    "components": [{"type": "given_name", "value": given},
                                   {"type": "family_name", "value": surname}]},
                    "gender": {"value": g}, "is_living": living},
               "birth": {"date": {"value": born.to_string(), "precision": "year"}}})
    }

    fn family(id: &str, parents: &[&str], kids: &[(&str, Option<f64>)]) -> Value {
        json!({"id": id, "type": "family", "axgf_version": "1.1",
               "union": {"persons": parents.iter().map(|p| json!({"person_id": p, "role": "spouse"})).collect::<Vec<_>>()},
               "children": kids.iter().map(|(k, c)| match c {
                   Some(c) => json!({"person_id": k, "confidence": c}),
                   None => json!({"person_id": k}),
               }).collect::<Vec<_>>()})
    }

    fn bundle(persons: Vec<Value>, families: Vec<Value>) -> Value {
        let ps: serde_json::Map<String, Value> = persons
            .into_iter()
            .map(|p| (p["id"].as_str().unwrap().to_string(), p))
            .collect();
        let fs: serde_json::Map<String, Value> = families
            .into_iter()
            .map(|f| (f["id"].as_str().unwrap().to_string(), f))
            .collect();
        json!({"persons": ps, "families": fs})
    }

    /// Three generations up and two down from `ego`.
    fn fixture() -> Value {
        bundle(
            vec![
                person("gf", "Jan", "Nowak", "M", 1830, false),
                person("gm", "Anna", "Lis", "F", 1834, false),
                person("dad", "Józef", "Nowak", "M", 1860, false),
                person("mum", "Marie", "Collin", "F", 1863, false),
                person("ego", "Michel", "Nowak", "M", 1890, false),
                person("wife", "Hélène", "Mercier", "F", 1892, false),
                person("k1", "Paul", "Nowak", "M", 1915, false),
                person("k2", "Rose", "Nowak", "F", 1918, true),
                person("gk", "Léa", "Nowak", "F", 1945, true),
            ],
            vec![
                family("f-gp", &["gf", "gm"], &[("dad", Some(0.95))]),
                family("f-p", &["dad", "mum"], &[("ego", Some(0.6))]),
                family(
                    "f-e",
                    &["ego", "wife"],
                    &[("k1", Some(0.98)), ("k2", Some(0.3))],
                ),
                family("f-k", &["k1"], &[("gk", None)]),
            ],
        )
    }

    #[test]
    fn classes_come_from_the_lookup_not_a_square_root() {
        for (d, c) in [
            (0, 1),
            (1, 1),
            (2, 2),
            (5, 2),
            (6, 3),
            (20, 3),
            (21, 4),
            (80, 4),
            (81, 5),
            (5000, 5),
        ] {
            assert_eq!(class_of(d), c, "d={d}");
        }
        assert_eq!(width_of(1), 1.6);
        assert_eq!(width_of(3), 3.0);
        assert_eq!(width_of(7), 5.0);
        assert_eq!(width_of(40), 8.0);
        assert_eq!(width_of(81), 12.0);
    }

    #[test]
    fn the_ramp_hits_its_stops_and_clamps() {
        assert_eq!(colour_at(1690.0), "#2b5a52");
        assert_eq!(colour_at(1500.0), "#2b5a52");
        assert_eq!(colour_at(1985.0), "#e6b062");
        assert_eq!(colour_at(2020.0), "#e6b062");
        // t = 0.68 → the third stop exactly.
        assert_eq!(colour_at(1690.0 + 0.68 * 295.0), "#86b08f");
        // Halfway between the first two stops, rounded the way JavaScript does.
        assert_eq!(colour_at(1690.0 + 0.2 * 295.0), "#356d62");
        assert_eq!(colour_at(f64::NAN), "#2b5a52");
    }

    #[test]
    fn the_curve_has_vertical_tangents_and_one_decimal() {
        assert_eq!(
            curve(470.0, 318.0, 400.04, 250.0),
            "M470 318C470 284 400 284 400 250"
        );
        assert_eq!(curve(1.25, 0.0, 2.0, 10.0), "M1.3 0C1.3 5 2 5 2 10");
        assert_eq!(num(-0.04), "0");
        assert_eq!(num(12.0), "12");
        assert_eq!(num(3.45), "3.5");
    }

    #[test]
    fn label_tiers_follow_the_gap() {
        assert_eq!(label_tier(45.9), 0);
        assert_eq!(label_tier(46.0), 3);
        assert_eq!(label_tier(138.0), 3);
    }

    #[test]
    fn confidence_uses_the_record_pages_bands() {
        assert_eq!(Conf::of(Some(0.95)), Conf::Attested);
        assert_eq!(Conf::of(Some(0.75)), Conf::Attested);
        assert_eq!(Conf::of(Some(0.74)), Conf::Inferred);
        assert_eq!(Conf::of(Some(0.5)), Conf::Inferred);
        assert_eq!(Conf::of(Some(0.49)), Conf::Speculative);
        // Unrated is the grid's 0.8.
        assert_eq!(Conf::of(None), Conf::Attested);
    }

    #[test]
    fn the_centre_holds_the_fixed_point_and_time_runs_up() {
        let flat = fixture();
        let g = Graph::build(&flat);
        let l = layout(&g, "ego", 3, Shape::default()).unwrap();
        let c = l.person("ego").unwrap();
        assert_eq!((c.x, c.y, c.role), (CX, CY, Role::Centre));
        let step = 96f64.min((CY - 46.0) / 3.0).min((H - 64.0 - CY) / 3.0);
        assert_eq!(step, 86.0);
        assert_eq!(l.meta.step, step);
        // Ancestors below, descendants above, one step per generation.
        assert!((l.person("dad").unwrap().y - (CY + step)).abs() < 1e-9);
        assert!((l.person("gf").unwrap().y - (CY + 2.0 * step)).abs() < 1e-9);
        assert!((l.person("k1").unwrap().y - (CY - step)).abs() < 1e-9);
        assert!((l.person("gk").unwrap().y - (CY - 2.0 * step)).abs() < 1e-9);
        // Father left of mother.
        assert!(l.person("dad").unwrap().x < l.person("mum").unwrap().x);
        // The spouse stands to the right at the centre's row.
        let w = l.person("wife").unwrap();
        assert_eq!((w.y, w.role), (CY, Role::Spouse));
        // Packed beside the centre: far enough for the centre's label, no
        // further (the reference's fixed 200–230 left a gap packing removes).
        let room = 7.5 + 10.0 + DEFAULT_LABEL_W + PACK_GAP + 4.8 + 4.0;
        assert!(
            w.x - CX >= room - 1e-9 && w.x - CX < 200.0,
            "spouse at +{}",
            w.x - CX
        );
    }

    #[test]
    fn unions_sit_nearer_the_parent_row() {
        let flat = fixture();
        let g = Graph::build(&flat);
        let l = layout(&g, "ego", 3, Shape::default()).unwrap();
        let step = l.meta.step;
        let up = l.couples.iter().find(|c| c.key == "f-p").unwrap();
        assert!(
            (up.y - (CY + 0.62 * step)).abs() < 1e-9,
            "ancestor union 0.38 step from the parents"
        );
        let down = l.couples.iter().find(|c| c.key == "f-e").unwrap();
        assert!(
            (down.y - (CY - 0.38 * step)).abs() < 1e-9,
            "descendant union 0.38 step from the parents"
        );
        // x is the mean of the parents' midpoint and the children's.
        let pm = (l.person("dad").unwrap().x + l.person("mum").unwrap().x) / 2.0;
        assert!((up.x - (pm + CX) / 2.0).abs() < 1e-9);
    }

    #[test]
    fn width_is_descendants_and_does_not_sum_at_a_confluence() {
        let flat = fixture();
        let g = Graph::build(&flat);
        // ego: k1, k2, gk.
        assert_eq!(g.descendant_count("ego"), Some(3));
        assert_eq!(g.descendant_count("gf"), Some(5));
        let l = layout(&g, "ego", 3, Shape::default()).unwrap();
        let f = l.couples.iter().find(|c| c.key == "f-p").unwrap();
        // The family's discharge is ego and everything below him: 4, and the
        // same 4 flow down the father's line and the mother's.
        assert_eq!(f.d, 4);
        assert_eq!(f.pconf.len(), 2);
        assert_eq!(l.person("ego").unwrap().d, 4);
        assert_eq!(l.person("dad").unwrap().d, 5);
    }

    #[test]
    fn a_childs_claim_decides_both_its_edges() {
        let flat = fixture();
        let g = Graph::build(&flat);
        let l = layout(&g, "ego", 3, Shape::default()).unwrap();
        let f = l.couples.iter().find(|c| c.key == "f-p").unwrap();
        assert_eq!(f.kconf["ego"], Conf::Inferred);
        assert!(f.pconf.values().all(|c| *c == Conf::Inferred));
        let e = l.couples.iter().find(|c| c.key == "f-e").unwrap();
        assert_eq!(e.kconf["k1"], Conf::Attested);
        assert_eq!(e.kconf["k2"], Conf::Speculative);
        // The parents' line into a family is as good as its best child.
        assert!(e.pconf.values().all(|c| *c == Conf::Attested));
    }

    #[test]
    fn tails_say_lost_or_continued() {
        let flat = fixture();
        let g = Graph::build(&flat);
        let l = layout(&g, "ego", 1, Shape::default()).unwrap();
        // At ± 1 dad's line continues off-frame with a count of his ancestors.
        let t = l.tails.iter().find(|t| t.key == "cdad").unwrap();
        assert_eq!((t.kind, t.dir, t.count), (TailKind::Cont, 1, 2));
        // mum has no recorded parents: a lost line.
        let t = l.tails.iter().find(|t| t.key == "lmum").unwrap();
        assert_eq!(t.kind, TailKind::Lost);
        // k1 continues upward with one descendant; k2 is childless, and her
        // line ends in the same ring whether or not she is living.
        let t = l.tails.iter().find(|t| t.key == "ck1").unwrap();
        assert_eq!((t.kind, t.dir, t.count), (TailKind::Cont, -1, 1));
        let t = l.tails.iter().find(|t| t.key == "ek2").unwrap();
        assert_eq!(t.kind, TailKind::Lost);
        // A family with one recorded parent leaves a lost line from its union.
        let l = layout(&g, "k1", 2, Shape::default()).unwrap();
        assert!(l
            .tails
            .iter()
            .any(|t| t.key == "mf-k" && t.kind == TailKind::Lost));
    }

    #[test]
    fn a_father_son_union_draws_without_hanging() {
        // The Klicki record: a son entered as his father's partner.
        let flat = bundle(
            vec![
                person("jakub", "Jakub", "Klicki", "M", 1800, false),
                person("bron", "Bronisław", "Klicki", "M", 1830, false),
                person("kid", "Ewa", "Klicki", "F", 1855, false),
            ],
            vec![
                family("f1", &["jakub"], &[("bron", None)]),
                family("f2", &["jakub", "bron"], &[("kid", None)]),
            ],
        );
        let g = Graph::build(&flat);
        for c in ["jakub", "bron", "kid"] {
            for n in RANGES {
                let l = layout(&g, c, n, Shape::default()).unwrap();
                assert!(l.persons.iter().all(|p| p.x.is_finite() && p.y.is_finite()));
            }
        }
    }

    #[test]
    fn layout_is_deterministic_and_in_frame() {
        let flat = fixture();
        let g = Graph::build(&flat);
        for n in RANGES {
            let a =
                serde_json::to_string(&layout(&g, "ego", n, Shape::default()).unwrap()).unwrap();
            let b =
                serde_json::to_string(&layout(&g, "ego", n, Shape::default()).unwrap()).unwrap();
            assert_eq!(a, b);
            let l = layout(&g, "ego", n, Shape::default()).unwrap();
            for p in &l.persons {
                assert!(
                    p.x >= 70.0 - 1e-9 && p.x <= W - 44.0 + 1e-9,
                    "{} at {}",
                    p.id,
                    p.x
                );
            }
        }
    }

    #[test]
    fn a_long_name_steps_down_a_tier() {
        let s = Shown {
            id: "x".into(),
            names: [
                "Eugeniusz".into(),
                "Eugeniusz G.".into(),
                "Eugeniusz Karol Alojzy Gundelach".into(),
            ],
            years: String::new(),
            colour: String::new(),
            living: false,
            sparse: false,
            redacted: false,
        };
        assert_eq!(fit_tier(3, &s, 140.0, false), 2);
        assert_eq!(fit_tier(3, &s, 400.0, false), 3);
        assert_eq!(fit_tier(2, &s, 80.0, false), 1);
        assert_eq!(fit_tier(1, &s, 60.0, false), 0);
        assert_eq!(
            fit_tier(3, &s, 10.0, true),
            1,
            "the centre keeps its given name"
        );
    }
}
