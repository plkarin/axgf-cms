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
/// Where the river is served. Every person in the SVG links here.
pub const RIVER_PATH: &str = "/river";
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

/// One family: its partners in slot order and its children with the
/// confidence of each child's claim.
#[derive(Debug, Clone)]
struct Fam {
    id: String,
    /// Partners, father first where the records say which is which.
    parents: Vec<usize>,
    kids: Vec<(usize, Conf, Option<f64>)>,
}

/// Every person and family the river can reach, indexed.
pub struct Graph {
    ids: Vec<String>,
    index: HashMap<String, usize>,
    shape: Vec<RiverShape>,
    fams: Vec<Fam>,
    /// The family each person is a child of, if any.
    born_in: Vec<Option<usize>>,
    /// Children, oldest first.
    kids_of: Vec<Vec<usize>>,
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
                let kids = f
                    .get("children")
                    .and_then(Value::as_array)
                    .map(|cs| {
                        cs.iter()
                            .filter_map(|c| {
                                let id = c.get("person_id").and_then(Value::as_str)?;
                                let i = *index.get(id)?;
                                let conf = c.get("confidence").and_then(Value::as_f64);
                                Some((i, Conf::of(conf), conf))
                            })
                            .collect()
                    })
                    .unwrap_or_default();
                fams.push(Fam {
                    id: (*fid).clone(),
                    parents,
                    kids,
                });
            }
        }

        // A child recorded in two families (an adoption beside a birth) is
        // drawn from the one whose claim is strongest; ties go to the first.
        let mut born_in: Vec<Option<usize>> = vec![None; n];
        let mut born_conf: Vec<f64> = vec![f64::NEG_INFINITY; n];
        let mut kid_sets: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); n];
        for (fi, f) in fams.iter().enumerate() {
            for &(k, _, raw) in &f.kids {
                let c = raw.unwrap_or(UNRATED);
                if c > born_conf[k] {
                    born_conf[k] = c;
                    born_in[k] = Some(fi);
                }
                for &p in &f.parents {
                    if p != k {
                        kid_sets[p].insert(k);
                    }
                }
            }
        }
        let by_birth = |a: &usize, b: &usize| {
            let ya = shape[*a].birth_year.unwrap_or(i64::MAX);
            let yb = shape[*b].birth_year.unwrap_or(i64::MAX);
            ya.cmp(&yb).then_with(|| ids[*a].cmp(&ids[*b]))
        };
        let kids_of: Vec<Vec<usize>> = kid_sets
            .into_iter()
            .map(|s| {
                let mut v: Vec<usize> = s.into_iter().collect();
                v.sort_by(by_birth);
                v
            })
            .collect();

        let mut g = Graph {
            ids,
            index,
            shape,
            fams,
            born_in,
            kids_of,
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

    /// Partners in families that have children, in the order those children
    /// were born.
    fn spouses_of(&self, i: usize) -> Vec<usize> {
        let mut out = Vec::new();
        for &k in &self.kids_of[i] {
            if let Some(f) = self.born_in[k] {
                for &p in &self.fams[f].parents {
                    if p != i && !out.contains(&p) && self.fams[f].parents.contains(&i) {
                        out.push(p);
                    }
                }
            }
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
    pub x: f64,
    pub y: f64,
    pub role: Role,
    /// Label tier: 0 none, 1 given name, 2 given name and initial, 3 full.
    pub lab: u8,
    /// The smaller gap to a neighbour in the row: the room a label has.
    pub room: f64,
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
    pub fn person(&self, id: &str) -> Option<&PNode> {
        self.persons.iter().find(|p| p.id == id)
    }
}

struct Frame<'g> {
    g: &'g Graph,
    n: usize,
    step: f64,
    /// Row y per signed generation offset.
    row_y: BTreeMap<i64, f64>,
    persons: Vec<PNode>,
    at: HashMap<usize, usize>,
}

impl Frame<'_> {
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

    fn put(&mut self, idx: usize, x: f64, role: Role, gen: i64) {
        let node = PNode {
            id: self.g.ids[idx].clone(),
            x,
            y: self.y(gen),
            role,
            lab: 0,
            room: 999.0,
            idx,
            gen,
            d: self.g.desc[idx] + 1,
        };
        match self.at.get(&idx) {
            Some(&slot) => self.persons[slot] = node,
            None => {
                self.at.insert(idx, self.persons.len());
                self.persons.push(node);
            }
        }
    }

    fn pos(&self, idx: usize) -> Option<&PNode> {
        self.at.get(&idx).map(|&s| &self.persons[s])
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

struct ANode {
    idx: usize,
    depth: usize,
    par: Vec<ANode>,
    x: f64,
}

struct DNode {
    idx: usize,
    depth: usize,
    kids: Vec<usize>,
    sp: Vec<usize>,
    u: f64,
    spu: Option<f64>,
    x: f64,
    spx: Option<f64>,
}

/// Lay out the river around `centre` with `n` generations each way.
pub fn layout(g: &Graph, centre: &str, n: usize, shape: Shape) -> Option<Layout> {
    let c = g.index_of(centre)?;
    let step = 96f64
        .min((CY - 46.0) / n as f64)
        .min((H - 64.0 - CY) / n as f64);

    // ---- ancestors: walk [father, mother] to depth n
    fn walk(g: &Graph, idx: usize, depth: usize, n: usize, leaves: &mut usize) -> ANode {
        let par: Vec<ANode> = if depth < n {
            g.parents_of(idx)
                .into_iter()
                .map(|p| walk(g, p, depth + 1, n, leaves))
                .collect()
        } else {
            Vec::new()
        };
        let mut node = ANode {
            idx,
            depth,
            par,
            x: 0.0,
        };
        if node.par.is_empty() {
            node.x = *leaves as f64;
            *leaves += 1;
        }
        node
    }
    let mut leaves = 0usize;
    let mut root = walk(g, c, 0, n, &mut leaves);
    let sa = 150f64.min((W - 150.0) / leaves.max(1) as f64);
    fn set_x(nd: &mut ANode, sa: f64) {
        if nd.par.is_empty() {
            nd.x *= sa;
        } else {
            for p in nd.par.iter_mut() {
                set_x(p, sa);
            }
            nd.x = nd.par.iter().map(|p| p.x).sum::<f64>() / nd.par.len() as f64;
        }
    }
    set_x(&mut root, sa);
    let off = CX - root.x;
    fn shift(nd: &mut ANode, off: f64) {
        nd.x += off;
        for p in nd.par.iter_mut() {
            shift(p, off);
        }
    }
    shift(&mut root, off);
    fn collect<'a>(nd: &'a mut ANode, out: &mut Vec<&'a mut f64>) {
        out.push(&mut nd.x);
        for p in nd.par.iter_mut() {
            collect(p, out);
        }
    }
    {
        let mut xs = Vec::new();
        collect(&mut root, &mut xs);
        fit(&mut xs, CX, 70.0, W - 44.0);
    }

    // ---- descendants: a unit-grid walk
    let mut dn: Vec<DNode> = Vec::new();
    let mut cur = 0f64;
    fn lay(
        g: &Graph,
        idx: usize,
        depth: usize,
        n: usize,
        cur: &mut f64,
        dn: &mut Vec<DNode>,
    ) -> usize {
        let ks: Vec<usize> = if depth < n {
            g.kids_of[idx].clone()
        } else {
            Vec::new()
        };
        let slot = dn.len();
        dn.push(DNode {
            idx,
            depth,
            kids: Vec::new(),
            sp: Vec::new(),
            u: 0.0,
            spu: None,
            x: 0.0,
            spx: None,
        });
        if ks.is_empty() {
            dn[slot].u = *cur;
            *cur += 1.0;
            return slot;
        }
        let kids: Vec<usize> = ks
            .iter()
            .map(|&k| lay(g, k, depth + 1, n, cur, dn))
            .collect();
        let sp = g.spouses_of(idx);
        if !sp.is_empty() && kids.len() == 1 {
            *cur += 1.0;
        }
        let m = kids.iter().map(|&k| dn[k].u).sum::<f64>() / kids.len() as f64;
        let node = &mut dn[slot];
        node.u = if sp.is_empty() { m } else { m - 0.5 };
        node.spu = (!sp.is_empty()).then_some(m + 0.5);
        node.kids = kids;
        node.sp = sp;
        slot
    }
    let top: Vec<usize> = if n > 0 {
        g.kids_of[c]
            .iter()
            .map(|&k| lay(g, k, 1, n, &mut cur, &mut dn))
            .collect()
    } else {
        Vec::new()
    };
    let sd = 130f64.min((W - 150.0) / cur.max(1.0));
    let csp = g.spouses_of(c);
    let sp_x = CX + 200f64.max(230f64.min(sd * 1.4));
    let c0x = if csp.is_empty() {
        CX
    } else {
        (CX + sp_x) / 2.0
    };
    let m_top = if top.is_empty() {
        0.0
    } else {
        top.iter().map(|&k| dn[k].u).sum::<f64>() / top.len() as f64
    };
    for nd in dn.iter_mut() {
        nd.x = (nd.u - m_top) * sd + c0x;
        nd.spx = nd.spu.map(|u| (u - m_top) * sd + c0x);
    }
    {
        let mut xs: Vec<&mut f64> = Vec::new();
        for nd in dn.iter_mut() {
            xs.push(&mut nd.x);
            if let Some(s) = nd.spx.as_mut() {
                xs.push(s);
            }
        }
        fit(&mut xs, c0x, 70.0, W - 44.0);
    }

    // ---- rows: head counts, the vertical scale, the hourglass budget
    let mut frame = Frame {
        g,
        n,
        step,
        row_y: BTreeMap::new(),
        persons: Vec::new(),
        at: HashMap::new(),
    };
    let mut heads: BTreeMap<i64, BTreeSet<usize>> = BTreeMap::new();
    fn count_anc(nd: &ANode, heads: &mut BTreeMap<i64, BTreeSet<usize>>) {
        heads.entry(-(nd.depth as i64)).or_default().insert(nd.idx);
        for p in &nd.par {
            count_anc(p, heads);
        }
    }
    count_anc(&root, &mut heads);
    for nd in &dn {
        let e = heads.entry(nd.depth as i64).or_default();
        e.insert(nd.idx);
        if nd.spx.is_some() {
            if let Some(&s) = nd.sp.first() {
                e.insert(s);
            }
        }
    }
    frame.row_y = rows_y(&heads, n, step, shape.log_spacing);

    // Place everybody, in the reference's order.
    fn put_anc(frame: &mut Frame<'_>, nd: &ANode, c: usize) {
        let role = if nd.idx == c { Role::Centre } else { Role::Anc };
        frame.put(nd.idx, nd.x, role, -(nd.depth as i64));
        for p in &nd.par {
            put_anc(frame, p, c);
        }
    }
    put_anc(&mut frame, &root, c);
    if let Some(&s) = csp.first() {
        frame.put(s, sp_x, Role::Spouse, 0);
    }
    if let Some(&s) = csp.get(1) {
        frame.put(s, CX - (sp_x - CX), Role::Spouse, 0);
    }
    for nd in &dn {
        frame.put(nd.idx, nd.x, Role::Desc, nd.depth as i64);
        if let (Some(&s), Some(x)) = (nd.sp.first(), nd.spx) {
            frame.put(s, x, Role::Dspouse, nd.depth as i64);
        }
    }

    if shape.hourglass {
        hourglass(&mut frame, 130.0, c0x);
    }

    // ---- couples and tails, from the final positions
    let mut couples: Vec<Couple> = Vec::new();
    let mut couple_at: HashMap<String, usize> = HashMap::new();
    let mut tails: Vec<Tail> = Vec::new();
    let year_est = estimate_years(g, c, &frame);
    let yr = |i: usize| -> f64 { year_est.get(&i).copied().unwrap_or(1900.0) };
    let mut add_couple = |couples: &mut Vec<Couple>, cp: Couple| match couple_at.get(&cp.key) {
        Some(&s) => couples[s] = cp,
        None => {
            couple_at.insert(cp.key.clone(), couples.len());
            couples.push(cp);
        }
    };

    fn anc_nodes<'a>(nd: &'a ANode, out: &mut Vec<&'a ANode>) {
        out.push(nd);
        for p in &nd.par {
            anc_nodes(p, out);
        }
    }
    let mut all_anc = Vec::new();
    anc_nodes(&root, &mut all_anc);
    for nd in all_anc {
        let i = nd.idx;
        let gen = -(nd.depth as i64);
        let Some(me) = frame.pos(i).map(|p| (p.x, p.y)) else {
            continue;
        };
        let parents = g.parents_of(i);
        if !nd.par.is_empty() {
            let f = g.born_in[i].expect("drawn parents come from a family");
            let fam = &g.fams[f];
            let pxs: Vec<f64> = nd
                .par
                .iter()
                .filter_map(|p| frame.pos(p.idx).map(|q| q.x))
                .collect();
            let pm = pxs.iter().sum::<f64>() / pxs.len().max(1) as f64;
            let cx = pm * 0.5 + me.0 * 0.5;
            let cy = me.1 + 0.62 * frame.gap_down(gen);
            let k = g.kid_conf(f, i);
            let d = g.family_discharge(f);
            let drawn = g.desc[i] + 1;
            add_couple(
                &mut couples,
                Couple {
                    key: fam.id.clone(),
                    x: cx,
                    y: cy,
                    parents: parents.iter().map(|&p| g.ids[p].clone()).collect(),
                    kids: vec![g.ids[i].clone()],
                    d,
                    stub: fam.kids.iter().filter(|kk| kk.0 != i).count(),
                    pconf: parents.iter().map(|&p| (g.ids[p].clone(), k)).collect(),
                    kconf: [(g.ids[i].clone(), k)].into_iter().collect(),
                    stub_d: d.saturating_sub(drawn).max(1),
                    stub_year: 1850.0,
                },
            );
            if parents.len() < 2 {
                // Which parent is missing decides the side the lost line
                // leaves on: a missing father to the left.
                let side = match parents.first().and_then(|&p| g.shape[p].slot) {
                    Some(1) => -1.0,
                    _ => 1.0,
                };
                tails.push(Tail {
                    key: format!("m{}", fam.id),
                    x1: cx,
                    y1: cy,
                    x2: cx + side * 26f64.max(sa * 0.42),
                    y2: frame.y(gen - 1) + 14.0,
                    d,
                    kind: TailKind::Lost,
                    dir: 1,
                    count: 0,
                    year: yr(i) - 28.0,
                    who: i,
                    offset: -28.0,
                });
            }
        } else if !parents.is_empty() {
            tails.push(Tail {
                key: format!("c{}", g.ids[i]),
                x1: me.0,
                y1: me.1,
                x2: me.0,
                y2: me.1 + 0.58 * frame.gap_down(gen),
                d: g.desc[i] + 1,
                kind: TailKind::Cont,
                dir: 1,
                count: g.anc[i],
                year: yr(i) - 20.0,
                who: i,
                offset: -20.0,
            });
        } else {
            tails.push(Tail {
                key: format!("l{}", g.ids[i]),
                x1: me.0,
                y1: me.1,
                x2: me.0,
                y2: me.1 + 0.62 * frame.gap_down(gen),
                d: g.desc[i] + 1,
                kind: TailKind::Lost,
                dir: 1,
                count: 0,
                year: yr(i) - 20.0,
                who: i,
                offset: -20.0,
            });
        }
    }

    // Descendant families, grouped under each drawn parent.
    let mut add_desc = |pid: usize, couples: &mut Vec<Couple>, tails: &mut Vec<Tail>| {
        let Some(pp) = frame.pos(pid).map(|p| (p.y, p.gen)) else {
            return;
        };
        let mut groups: Vec<(usize, Vec<usize>)> = Vec::new();
        for &k in &g.kids_of[pid] {
            if frame.pos(k).is_none() {
                continue;
            }
            let Some(f) = g.born_in[k] else { continue };
            if !g.fams[f].parents.contains(&pid) {
                continue;
            }
            match groups.iter_mut().find(|(gf, _)| *gf == f) {
                Some((_, ks)) => ks.push(k),
                None => groups.push((f, vec![k])),
            }
        }
        for (f, ks) in groups {
            let fam = &g.fams[f];
            let placed: Vec<f64> = fam
                .parents
                .iter()
                .filter_map(|&p| frame.pos(p).map(|q| q.x))
                .collect();
            let pm = placed.iter().sum::<f64>() / placed.len().max(1) as f64;
            let km = ks
                .iter()
                .filter_map(|&k| frame.pos(k).map(|q| q.x))
                .sum::<f64>()
                / ks.len() as f64;
            let cx = pm * 0.5 + km * 0.5;
            let cy = pp.0 - 0.38 * frame.gap_up(pp.1);
            let d = g.family_discharge(f);
            // The parent's line into the family is as certain as its
            // best-attested child.
            let best = ks
                .iter()
                .map(|&k| g.kid_conf(f, k))
                .min()
                .unwrap_or(Conf::Attested);
            add_couple(
                couples,
                Couple {
                    key: fam.id.clone(),
                    x: cx,
                    y: cy,
                    parents: fam.parents.iter().map(|&p| g.ids[p].clone()).collect(),
                    kids: ks.iter().map(|&k| g.ids[k].clone()).collect(),
                    d,
                    stub: 0,
                    pconf: fam
                        .parents
                        .iter()
                        .map(|&p| (g.ids[p].clone(), best))
                        .collect(),
                    kconf: ks
                        .iter()
                        .map(|&k| (g.ids[k].clone(), g.kid_conf(f, k)))
                        .collect(),
                    stub_d: 1,
                    stub_year: 1850.0,
                },
            );
            if fam.parents.len() < 2 {
                tails.push(Tail {
                    key: format!("m{}", fam.id),
                    x1: cx,
                    y1: cy,
                    x2: cx - 30.0,
                    y2: pp.0 + 10.0,
                    d,
                    kind: TailKind::Lost,
                    dir: 1,
                    count: 0,
                    year: yr(pid),
                    who: pid,
                    offset: 0.0,
                });
            }
        }
    };
    add_desc(c, &mut couples, &mut tails);
    for nd in &dn {
        if !nd.kids.is_empty() {
            add_desc(nd.idx, &mut couples, &mut tails);
        }
    }
    for nd in &dn {
        if !nd.kids.is_empty() {
            continue;
        }
        let Some(me) = frame.pos(nd.idx).map(|p| (p.x, p.y, p.gen)) else {
            continue;
        };
        let up = 0.55 * frame.gap_up(me.2);
        if !g.kids_of[nd.idx].is_empty() {
            tails.push(Tail {
                key: format!("c{}", g.ids[nd.idx]),
                x1: me.0,
                y1: me.1,
                x2: me.0,
                y2: me.1 - up,
                d: g.desc[nd.idx] + 1,
                kind: TailKind::Cont,
                dir: -1,
                count: g.desc[nd.idx],
                year: yr(nd.idx) + 20.0,
                who: nd.idx,
                offset: 20.0,
            });
        } else {
            // Every childless line ends in the same ring, living or not.
            // Drawing it only for the dead told a reader of a redacted,
            // childless person whether they were alive.
            tails.push(Tail {
                key: format!("e{}", g.ids[nd.idx]),
                x1: me.0,
                y1: me.1,
                x2: me.0,
                y2: me.1 - up,
                d: 1,
                kind: TailKind::Lost,
                dir: -1,
                count: 0,
                year: yr(nd.idx),
                who: nd.idx,
                offset: 0.0,
            });
        }
    }
    if g.kids_of[c].is_empty() {
        tails.push(Tail {
            key: format!("e{}", g.ids[c]),
            x1: CX,
            y1: CY,
            x2: CX,
            y2: CY - 0.55 * step,
            d: 1,
            kind: TailKind::Lost,
            dir: -1,
            count: 0,
            year: yr(c),
            who: c,
            offset: 0.0,
        });
    }
    let spouses: Vec<(usize, f64, f64, i64)> = frame
        .persons
        .iter()
        .filter(|p| matches!(p.role, Role::Spouse | Role::Dspouse))
        .map(|p| (p.idx, p.x, p.y, p.gen))
        .collect();
    for (i, x, y, gen) in spouses {
        let down = frame.gap_down(gen);
        if !g.parents_of(i).is_empty() {
            tails.push(Tail {
                key: format!("c{}", g.ids[i]),
                x1: x,
                y1: y,
                x2: x,
                y2: y + 0.5 * down,
                d: g.desc[i] + 1,
                kind: TailKind::Cont,
                dir: 1,
                count: g.anc[i],
                year: yr(i) - 20.0,
                who: i,
                offset: -20.0,
            });
        } else {
            tails.push(Tail {
                key: format!("l{}", g.ids[i]),
                x1: x,
                y1: y,
                x2: x,
                y2: y + 0.55 * down,
                d: g.desc[i] + 1,
                kind: TailKind::Lost,
                dir: 1,
                count: 0,
                year: yr(i) - 20.0,
                who: i,
                offset: -20.0,
            });
        }
    }

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
        },
        years,
    })
}

/// Label tier from a person's smaller neighbour gap.
pub fn label_tier(gap: f64) -> u8 {
    if gap < 46.0 {
        0
    } else if gap < 76.0 {
        1
    } else if gap < 138.0 {
        2
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
fn hourglass(frame: &mut Frame<'_>, sd_cap: f64, c0x: f64) {
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
fn estimate_years(g: &Graph, c: usize, frame: &Frame<'_>) -> HashMap<usize, f64> {
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
fn year_scale(frame: &Frame<'_>, year: f64) -> YearScale {
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
        .map(|(p, &y)| (p.id.as_str(), y))
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
    let mut layout = layout(g, centre, n, shape)?;
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
    let up = g.kids_of[c].first().map(|&k| g.ids[k].clone());
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
    let by_id: HashMap<&str, usize> = l
        .persons
        .iter()
        .enumerate()
        .map(|(i, p)| (p.id.as_str(), i))
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
    let mut living_named = false;
    // No bands without an era, nor for a signed-out reader (`Meta::bands`).
    let mut y0 = if l.meta.bands { 1600 } else { 2050 };
    while y0 < 2050 {
        let ya = l.meta.scale.y_of(y0 as f64);
        let yb = l.meta.scale.y_of((y0 + 50) as f64);
        if !(yb > H || ya < 0.0) {
            let live = y0 >= 1950;
            let t = yb.max(0.0);
            let b = ya.min(H);
            let fill = if live {
                BAND_LIVE
            } else if (y0 / 50) % 2 == 1 {
                BAND_ODD
            } else {
                BG
            };
            let _ = write!(
                out,
                r#"<rect x="0" y="{}" width="{}" height="{}" fill="{fill}"/>"#,
                num(t),
                num(W),
                num(b - t)
            );
            if ya <= H {
                let _ = write!(
                    out,
                    r#"<line x1="0" y1="{y}" x2="{w}" y2="{y}" stroke="{BAND_RULE}"/><text x="10" y="{ty}" font-family="{MONO}" font-size="10" fill="{tf}">{y0}</text>"#,
                    y = num(ya),
                    w = num(W),
                    ty = num(ya - 6.0),
                    tf = if live { BAND_TEXT_LIVE } else { BAND_TEXT }
                );
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
                    r#"<text x="10" y="{}" font-family="{MONO}" font-size="10" letter-spacing="1" fill="{BAND_TEXT_LIVE}">{}</text>"#,
                    num(label_y),
                    html_escape(&words.living_band)
                );
            }
        }
        y0 += 50;
    }

    // ---- right rail: generations relative to the centre
    for &(k, y) in &l.meta.rail {
        if !(8.0..=H - 4.0).contains(&y) {
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
            num(W - 10.0),
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
                    r#"<g class="rv-tail"><path d="{d}" fill="none" stroke="{c}" stroke-width="{w}"/><polygon points="{},{} {},{} {},{}" fill="{c}"/><text x="{}" y="{}" font-family="{MONO}" font-size="9.5" fill="{DATA_TEXT}">+{}</text></g>"#,
                    num(t.x2 - 4.0),
                    num(t.y2),
                    num(t.x2 + 4.0),
                    num(t.y2),
                    num(t.x2),
                    num(t.y2 + dir * 6.0),
                    num(t.x2 + 8.0),
                    num(t.y2 + if t.dir > 0 { 4.0 } else { 2.0 }),
                    t.count
                );
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
            let side = if c.x < 150.0 { 1.0 } else { -1.0 };
            let (x2, y2) = (c.x + side * 36.0, c.y - 32.0);
            let base = c.stub_year;
            let _ = write!(
                out,
                r#"<g class="rv-stub"><path d="{}" fill="none" stroke="{}" stroke-opacity="0.45" stroke-width="{}"/><text x="{}" y="{}" text-anchor="{}" font-family="{MONO}" font-size="10" fill="{DATA_TEXT}">+{}</text></g>"#,
                curve(c.x, c.y, x2, y2),
                col(base),
                num(width_of(c.stub_d)),
                num(x2 + side * 5.0),
                num(y2 - 3.0),
                if side < 0.0 { "end" } else { "start" },
                c.stub
            );
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
    for (p, s) in l.persons.iter().zip(shown) {
        let is_c = p.role == Role::Centre;
        let r = if is_c { 7.5 } else { 4.8 };
        let (x, y) = (num(p.x), num(p.y));
        // A link, so that the river travels without JavaScript too: the page
        // re-centred on this person, at the same range.
        let _ = write!(
            out,
            r#"<a class="rv-p" data-id="{}" href="{RIVER_PATH}?p={}&amp;n={}"><circle cx="{x}" cy="{y}" r="20" fill="transparent"/>"#,
            html_escape(&p.id),
            url_component(&p.id),
            l.meta.n
        );
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
        let tier = if s.redacted {
            0
        } else if is_c {
            3
        } else {
            p.lab
        };
        let tier = fit_edge(fit_tier(tier, s, p.room, is_c), s, p.x, r, is_c);
        if tier > 0 {
            let name = &s.names[(tier as usize).clamp(1, 3) - 1];
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
        r#"<svg class="rv-svg" viewBox="0 0 {w} {h}" width="100%" role="img" aria-label="{a}" xmlns="http://www.w3.org/2000/svg"><defs>{defs}</defs><rect width="{w}" height="{h}" fill="{BG}"/>{out}</svg>"#,
        w = num(W),
        h = num(H),
        a = html_escape(aria_label)
    )
}

/// Step a label down a tier while it would run past the right rail.
///
/// Near the right edge the frame used to cut names off ("Jarosław Medy…" on
/// the operator's bundle). Drawing those labels to the left of the dot
/// instead ran them into their left neighbour's, so a label keeps to the
/// right, as the reference draws it, and shortens instead: there is never a
/// neighbour further right for it to meet. The centre stands at the fixed
/// point and never reaches the edge.
pub fn fit_edge(tier: u8, s: &Shown, x: f64, r: f64, centre: bool) -> u8 {
    if centre {
        return tier;
    }
    let start = x + r + 6.0;
    let years = s.years.chars().count() as f64 * 5.8;
    let mut t = tier;
    while t > 0 && start + label_width(&s.names[t as usize - 1]).max(years) > RIGHT_EDGE {
        t -= 1;
    }
    t
}

/// Labels stop short of the right rail's generation numbers.
const RIGHT_EDGE: f64 = W - 34.0;

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

/// Roughly how wide a 12-unit serif label is.
fn label_width(text: &str) -> f64 {
    text.chars().count() as f64 * 6.3
}

/// Step a label down a tier while it would run into its neighbour.
///
/// The tier rule reads the gap only, so a long full name in a 140-unit gap
/// overprinted the next person's. The centre always keeps its full name. A
/// given name that does not fit either is dropped rather than overprinted:
/// the person still labels on hover, which is where a reader looks for it.
pub fn fit_tier(tier: u8, s: &Shown, room: f64, centre: bool) -> u8 {
    if centre {
        return tier;
    }
    let mut t = tier;
    while t > 0 && label_width(&s.names[t as usize - 1]) > room - 12.0 {
        t -= 1;
    }
    t
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
        assert_eq!(label_tier(46.0), 1);
        assert_eq!(label_tier(75.9), 1);
        assert_eq!(label_tier(137.9), 2);
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
        assert!(w.x - CX >= 200.0 && w.x - CX <= 230.0);
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
        assert_eq!(fit_tier(3, &s, 10.0, true), 3);
    }
}
