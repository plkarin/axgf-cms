//! Canopée's world (docs/adr/0043): the family's land, built once for each
//! shape of the family.
//!
//! The family's **shape** is who is there and who is whose parent and
//! partner — no name, no date. Its **fingerprint** names it, and the same
//! shape always builds the same world: the land's relief, its water, its
//! trees, rocks and herds are drawn from a random seeded by the
//! fingerprint. So the world is built once per shape and kept; a person
//! added or removed, a link made or unmade, makes a new shape and a new
//! world, whole. A name or a date changed does not.
//!
//! The land: the whole family's layout (`tree::layout`) laid on a valley, the
//! oldest generation up in the mountains, each younger one further down, the
//! youngest by the lake at its foot. The family carves it — a valley from
//! each parent to the couple's confluence and on to their children, deepest
//! along the line that carries most descendants, a hollow where each person
//! stands — and then the water does what water does: each person is a
//! spring, the rain gathers, every drop runs to its lowest neighbour. So the
//! streams join and never cross, lie in their beds, and grow on their way
//! down to the lake (the hollows filled to their brims become pools).
//!
//! Served as one binary ([`World::encode`]), read by `static/canopee-world.js`.

use std::collections::{BTreeMap, BTreeSet};

use serde_json::Value;
use sha2::{Digest, Sha256};

/// The family's shape: who is there, who is whose parent, who is whose
/// partner. Sorted, so that two bundles with the same family have the same
/// shape whatever order their records come in.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Shape {
    pub persons: Vec<String>,
    /// (parent, child)
    pub children: Vec<(String, String)>,
    /// (a, b), a < b
    pub unions: Vec<(String, String)>,
}

/// The shape of the family in `flat`.
pub fn shape_of(flat: &Value) -> Shape {
    let persons: Vec<String> = flat
        .get("persons")
        .and_then(Value::as_object)
        .map(|m| m.keys().cloned().collect())
        .unwrap_or_default();
    let known: BTreeSet<&str> = persons.iter().map(String::as_str).collect();
    let mut children: BTreeSet<(String, String)> = BTreeSet::new();
    let mut unions: BTreeSet<(String, String)> = BTreeSet::new();
    for f in crate::tree::family_edges(flat) {
        let parents: Vec<&String> = f
            .parents
            .iter()
            .filter(|p| known.contains(p.as_str()))
            .collect();
        for (c, _) in &f.children {
            if !known.contains(c.as_str()) {
                continue;
            }
            for p in &parents {
                children.insert(((*p).clone(), c.clone()));
            }
        }
        for (i, a) in parents.iter().enumerate() {
            for b in &parents[i + 1..] {
                let (x, y) = if a < b { (a, b) } else { (b, a) };
                unions.insert(((*x).clone(), (*y).clone()));
            }
        }
    }
    let mut persons = persons;
    persons.sort();
    Shape {
        persons,
        children: children.into_iter().collect(),
        unions: unions.into_iter().collect(),
    }
}

/// The name of a shape: the SHA-256 of its sorted parts, 32 hex digits.
pub fn fingerprint(shape: &Shape) -> String {
    let mut h = Sha256::new();
    h.update(b"axgf-world/1\n");
    for p in &shape.persons {
        h.update(b"p ");
        h.update(p.as_bytes());
        h.update(b"\n");
    }
    for (a, b) in &shape.children {
        h.update(format!("c {a} {b}\n").as_bytes());
    }
    for (a, b) in &shape.unions {
        h.update(format!("u {a} {b}\n").as_bytes());
    }
    let digest = h.finalize();
    digest.iter().take(16).map(|b| format!("{b:02x}")).collect()
}

/// A person's place in the world.
#[derive(Debug, Clone, PartialEq)]
pub struct Spot {
    pub id: String,
    pub x: f32,
    pub z: f32,
    pub y: f32,
    /// The river's discharge class, 1–5 (docs/adr/0004).
    pub class: u8,
    /// The generation's rank in the layout, 0 the oldest.
    pub generation: u16,
}

/// Water as it runs: down the land's own slope, from the springs where the
/// people stand to the lake, joining and never crossing.
#[derive(Debug, Clone, PartialEq)]
pub struct River {
    /// x, z, the water's surface, half its width, its foam (0 still, 1 a
    /// confluence or a rapid).
    pub points: Vec<[f32; 5]>,
}

/// Still water held in a hollow: the pool where a person stands, or a tarn.
#[derive(Debug, Clone, PartialEq)]
pub struct Pond {
    pub x: f32,
    pub z: f32,
    pub level: f32,
    pub radius: f32,
    /// The cells it covers, row by row: the row, the first and the last
    /// column — its own shape, so that it fills its hollow and no other.
    pub spans: Vec<[u16; 3]>,
}

/// The world, as the page draws it.
#[derive(Debug, Clone, PartialEq)]
pub struct World {
    pub fingerprint: String,
    /// Grid points per side.
    pub n: usize,
    pub x0: f32,
    pub z0: f32,
    pub size_x: f32,
    pub size_z: f32,
    /// n × n, row by row along z: the ground, beds carved.
    pub heights: Vec<f32>,
    /// n × n: how thick the forest is, 0–255.
    pub forest: Vec<u8>,
    /// The lake's level.
    pub water: f32,
    pub persons: Vec<Spot>,
    pub rivers: Vec<River>,
    pub ponds: Vec<Pond>,
    /// Footpaths: x, z, half their width.
    pub paths: Vec<Vec<[f32; 3]>>,
    /// x, z, y, scale, kind (0 a conifer, 1 a broadleaf), tint (0–1).
    pub trees: Vec<[f32; 6]>,
    /// x, z, y, scale.
    pub rocks: Vec<[f32; 4]>,
    /// x, z, y, how many animals.
    pub herds: Vec<[f32; 4]>,
}

/// Grid points per side of the heightfield, at the least; a wide family's
/// land has more, up to [`GRID_MAX`], so that its streams stay fine.
pub const GRID: usize = 513;
/// Grid points per side of the heightfield, at the most.
pub const GRID_MAX: usize = 769;
/// Across the valley, between two people of one generation, in the world's
/// units: room enough that their streams run apart before they join.
const SPACING: f32 = 12.0;
/// Down the valley, between two generations.
const GENERATION: f32 = 88.0;
/// How far the ground falls from one generation to the next.
const FALL: f32 = 10.5;

/// A small deterministic random, seeded by the fingerprint.
struct Rng(u64);

impl Rng {
    fn new(fingerprint: &str) -> Self {
        let mut seed = 0x9e37_79b9_7f4a_7c15u64;
        for b in fingerprint.bytes() {
            seed = seed.rotate_left(5) ^ u64::from(b);
            seed = seed.wrapping_mul(0x1000_0000_01b3);
        }
        Rng(seed | 1)
    }

    fn next(&mut self) -> u64 {
        // xorshift64*
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_f491_4f6c_dd1d)
    }

    /// In [0, 1).
    fn f(&mut self) -> f32 {
        (self.next() >> 40) as f32 / (1u64 << 24) as f32
    }

    fn range(&mut self, a: f32, b: f32) -> f32 {
        a + (b - a) * self.f()
    }
}

/// Value noise on a lattice, smoothed: the relief's grain.
fn lattice(seed: u64, xi: i64, zi: i64) -> f32 {
    let mut h = seed
        ^ (xi as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15)
        ^ (zi as u64).wrapping_mul(0xc2b2_ae3d_27d4_eb4f);
    h ^= h >> 33;
    h = h.wrapping_mul(0xff51_afd7_ed55_8ccd);
    h ^= h >> 33;
    (h >> 40) as f32 / (1u64 << 24) as f32 * 2.0 - 1.0
}

fn noise(seed: u64, x: f32, z: f32) -> f32 {
    let (xi, zi) = (x.floor(), z.floor());
    let (fx, fz) = (x - xi, z - zi);
    let s = |t: f32| t * t * (3.0 - 2.0 * t);
    let (sx, sz) = (s(fx), s(fz));
    let (xi, zi) = (xi as i64, zi as i64);
    let a = lattice(seed, xi, zi);
    let b = lattice(seed, xi + 1, zi);
    let c = lattice(seed, xi, zi + 1);
    let d = lattice(seed, xi + 1, zi + 1);
    let top = a + (b - a) * sx;
    let bottom = c + (d - c) * sx;
    top + (bottom - top) * sz
}

/// Fractal noise: five octaves, each half as strong and twice as fine.
fn fbm(seed: u64, x: f32, z: f32) -> f32 {
    let (mut sum, mut amp, mut freq) = (0.0, 1.0, 1.0);
    for o in 0..5 {
        sum += amp * noise(seed.wrapping_add(o * 7919), x * freq, z * freq);
        amp *= 0.5;
        freq *= 2.03;
    }
    sum
}

/// How many distinct descendants each person has, in the shape.
fn descendants(shape: &Shape) -> BTreeMap<&str, usize> {
    let mut kids: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for (p, c) in &shape.children {
        kids.entry(p.as_str()).or_default().push(c.as_str());
    }
    let mut out = BTreeMap::new();
    for p in &shape.persons {
        let mut seen: BTreeSet<&str> = BTreeSet::new();
        let mut stack: Vec<&str> = kids.get(p.as_str()).cloned().unwrap_or_default();
        while let Some(c) = stack.pop() {
            if seen.insert(c) {
                if let Some(more) = kids.get(c) {
                    stack.extend(more.iter().copied());
                }
            }
        }
        out.insert(p.as_str(), seen.len());
    }
    out
}

/// A height in the priority queue of the flood: ordered, lowest first.
#[derive(Clone, Copy, PartialEq)]
struct Low(f32, u32);
impl Eq for Low {}
impl PartialOrd for Low {
    fn partial_cmp(&self, o: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(o))
    }
}
impl Ord for Low {
    fn cmp(&self, o: &Self) -> std::cmp::Ordering {
        o.0.total_cmp(&self.0).then(o.1.cmp(&self.1))
    }
}

/// A ridged noise: sharp crests, for the mountains.
fn ridged(seed: u64, x: f32, z: f32) -> f32 {
    let (mut sum, mut amp, mut freq) = (0.0, 1.0, 1.0);
    for o in 0..5 {
        let v = 1.0 - noise(seed.wrapping_add(31 + o * 101), x * freq, z * freq).abs();
        sum += amp * v * v;
        amp *= 0.5;
        freq *= 2.1;
    }
    sum
}

/// Smooth a polyline (Chaikin's corner cutting), its ends kept.
fn chaikin(points: &[[f32; 5]], rounds: usize) -> Vec<[f32; 5]> {
    let mut p = points.to_vec();
    for _ in 0..rounds {
        if p.len() < 3 {
            break;
        }
        let mut q = Vec::with_capacity(p.len() * 2);
        q.push(p[0]);
        for w in p.windows(2) {
            let (a, b) = (w[0], w[1]);
            let mut r = [0.0f32; 5];
            let mut s = [0.0f32; 5];
            for k in 0..5 {
                r[k] = a[k] * 0.75 + b[k] * 0.25;
                s[k] = a[k] * 0.25 + b[k] * 0.75;
            }
            q.push(r);
            q.push(s);
        }
        q.push(p[p.len() - 1]);
        p = q;
    }
    p
}

/// A line thinned to points `gap` apart at least (its ends kept), each kept
/// point carrying the most foam of those it stands for.
fn thin(points: &[[f32; 5]], gap: f32) -> Vec<[f32; 5]> {
    let mut out: Vec<[f32; 5]> = Vec::with_capacity(points.len() / 3 + 2);
    let mut foam = 0.0f32;
    for (q, p) in points.iter().enumerate() {
        foam = foam.max(p[4]);
        let keep = match out.last() {
            None => true,
            Some(o) => {
                q + 1 == points.len()
                    || ((p[0] - o[0]).powi(2) + (p[1] - o[1]).powi(2)).sqrt() >= gap
            }
        };
        if keep {
            out.push([p[0], p[1], p[2], p[3], foam]);
            foam = 0.0;
        }
    }
    out
}

/// Build the world of the family in `flat`.
pub fn build(flat: &Value) -> World {
    let shape = shape_of(flat);
    let fp = fingerprint(&shape);
    let seed = Rng::new(&fp).next();
    let mut rng = Rng::new(&fp);

    // The family laid out as the tree lays it: generations, and an order
    // across each that keeps families together. The tree draws the
    // youngest at the top; the valley starts with the oldest, where the
    // springs are.
    let layout = crate::tree::layout(flat);
    let mut placed: Vec<&crate::tree::Band> = layout
        .bands
        .iter()
        .filter(|b| !b.unplaced && !b.cards.is_empty() && b.generation.is_some())
        .collect();
    placed.sort_by_key(|b| b.generation);
    let widest = placed
        .iter()
        .map(|b| b.cards.len())
        .max()
        .unwrap_or(1)
        .max(1);
    let (min_x, max_x) = placed
        .iter()
        .flat_map(|b| b.cards.iter())
        .fold((f64::MAX, f64::MIN), |(lo, hi), c| {
            (lo.min(c.x + c.w / 2.0), hi.max(c.x + c.w / 2.0))
        });
    let span = (widest as f32 * SPACING).max(60.0);
    let half = span / 2.0;
    let gens = placed.len().max(1);
    let to_x = |cx: f64| -> f32 {
        if max_x > min_x {
            (((cx - min_x) / (max_x - min_x)) as f32 - 0.5) * span
        } else {
            0.0
        }
    };

    // The land around them: mountains beyond the family on either side and
    // above the springs, the lake below the youngest; wide enough that the
    // mist hides the world's edge from wherever the camera may stand.
    let margin_x = (span * 0.45).clamp(260.0, 320.0);
    let size_x = span + 2.0 * margin_x;
    let top = -(gens as f32 - 1.0) * GENERATION / 2.0;
    let bottom = (gens as f32 - 1.0) * GENERATION / 2.0;
    let z0 = top - 260.0;
    let size_z = (bottom + 300.0) - z0;
    let x0 = -size_x / 2.0;
    let n = ((size_x.max(size_z) / 2.4).ceil() as usize).clamp(GRID, GRID_MAX);
    let cell_x = size_x / (n - 1) as f32;
    let cell_z = size_z / (n - 1) as f32;
    let cell = cell_x.min(cell_z);
    // The ground's fall down the valley: 0 at the lake's shore, FALL a
    // generation above it.
    let base = |z: f32| -> f32 { (bottom + 40.0 - z) / GENERATION * FALL };
    let water = base(bottom + 120.0) - 1.5;
    let gx = |i: usize| x0 + i as f32 * cell_x;
    let gz = |j: usize| z0 + j as f32 * cell_z;
    let at = |x: f32, z: f32| -> (usize, usize) {
        (
            (((x - x0) / cell_x).round().clamp(0.0, (n - 1) as f32)) as usize,
            (((z - z0) / cell_z).round().clamp(0.0, (n - 1) as f32)) as usize,
        )
    };

    let mut heights = vec![0.0f32; n * n];
    for j in 0..n {
        let z = gz(j);
        for i in 0..n {
            let x = gx(i);
            let mut h = base(z);
            // The valley's walls, rising beyond the family into mountains
            // with sharp crests.
            let out = (x.abs() - half - 25.0).max(0.0);
            let above = (top - 50.0 - z).max(0.0);
            let rise = out + above * 0.8;
            h += rise * rise * 0.0014 + rise * 0.12;
            h += ridged(seed, x / 90.0, z / 90.0) * (rise * 0.22).min(60.0);
            // The valley floor's own gentle relief.
            h += fbm(seed, x / 60.0, z / 60.0) * (2.6 + rise * 0.05);
            // The lake's basin, below the youngest generation.
            let lake = (z - (bottom + 70.0)).max(0.0);
            if lake > 0.0 {
                let shore = (lake / 70.0).min(1.0);
                let shore = shore * shore * (3.0 - 2.0 * shore);
                h = h * (1.0 - shore) + (water - 5.0) * shore;
            }
            heights[j * n + i] = h;
        }
    }

    // The people, where the layout puts them.
    let desc = descendants(&shape);
    let mut persons: Vec<Spot> = Vec::new();
    let mut index: BTreeMap<&str, u16> = BTreeMap::new();
    for (g, band) in placed.iter().enumerate() {
        let z = top + g as f32 * GENERATION;
        for c in &band.cards {
            if persons.len() >= u16::MAX as usize - 1 {
                break;
            }
            let x = to_x(c.x + c.w / 2.0);
            let d = desc.get(c.id.as_str()).copied().unwrap_or(0);
            index.insert(c.id.as_str(), persons.len() as u16);
            persons.push(Spot {
                id: c.id.clone(),
                x,
                z,
                y: 0.0,
                class: crate::river::class_of(d + 1) as u8,
                generation: g as u16,
            });
        }
    }

    // The family carves the land: a valley from each parent down to the
    // confluence below the couple, and from it down to their children —
    // deepest toward the child who carries most of the line, so that the
    // water, which goes where the land is lowest, follows the line; the
    // other children's valleys are gentler, and their own springs feed
    // them. A hollow where each person stands.
    let mut parents_of: BTreeMap<u16, Vec<u16>> = BTreeMap::new();
    for (p, c) in &shape.children {
        if let (Some(&a), Some(&b)) = (index.get(p.as_str()), index.get(c.as_str())) {
            if persons[b as usize].z > persons[a as usize].z {
                parents_of.entry(b).or_default().push(a);
            }
        }
    }
    let mut by_parents: BTreeMap<Vec<u16>, Vec<u16>> = BTreeMap::new();
    for (child, mut ps) in parents_of {
        ps.sort_unstable();
        ps.dedup();
        ps.truncate(2);
        by_parents.entry(ps).or_default().push(child);
    }
    let mut valleys: Vec<([f32; 2], [f32; 2], f32, f32)> = Vec::new();
    for (ps, kids) in &by_parents {
        let main = kids
            .iter()
            .copied()
            .max_by_key(|k| (persons[*k as usize].class, u16::MAX - *k))
            .unwrap_or(kids[0]);
        let lowest = kids
            .iter()
            .map(|k| persons[*k as usize].z)
            .fold(f32::MAX, f32::min);
        let tops: Vec<&Spot> = ps.iter().map(|p| &persons[*p as usize]).collect();
        let top_z = tops.iter().map(|p| p.z).fold(f32::MIN, f32::max);
        let meet = if tops.len() == 2 {
            [
                (tops[0].x + tops[1].x) / 2.0,
                top_z + (lowest - top_z) * 0.42,
            ]
        } else {
            [tops[0].x, top_z + (lowest - top_z) * 0.25]
        };
        let class = persons[main as usize].class as f32;
        for p in &tops {
            valleys.push((
                [p.x, p.z],
                meet,
                2.4 + p.class as f32 * 0.5,
                7.0 + p.class as f32 * 1.2,
            ));
        }
        for k in kids {
            let c = &persons[*k as usize];
            let deep = if *k == main { 3.2 + class * 0.6 } else { 1.3 };
            let wide = if *k == main { 8.0 + class * 1.4 } else { 5.0 };
            valleys.push((meet, [c.x, c.z], deep, wide));
        }
    }
    let carve_valley = |heights: &mut Vec<f32>,
                        a: [f32; 2],
                        b: [f32; 2],
                        depth: f32,
                        width: f32,
                        rng: &mut Rng| {
        // A gentle curve between the two, meandering a little.
        let wiggle = rng.range(-1.0, 1.0) * (b[1] - a[1]).abs().max(8.0) * 0.18;
        let (c1, c2) = (
            [a[0] + wiggle, a[1] + (b[1] - a[1]) * 0.4],
            [b[0] - wiggle * 0.6, b[1] - (b[1] - a[1]) * 0.4],
        );
        let len = ((b[0] - a[0]).powi(2) + (b[1] - a[1]).powi(2)).sqrt();
        let steps = ((len / (cell * 0.6)).ceil() as usize).max(4);
        for s in 0..=steps {
            let t = s as f32 / steps as f32;
            let u = 1.0 - t;
            let x = u * u * u * a[0]
                + 3.0 * u * u * t * c1[0]
                + 3.0 * u * t * t * c2[0]
                + t * t * t * b[0];
            let z = u * u * u * a[1]
                + 3.0 * u * u * t * c1[1]
                + 3.0 * u * t * t * c2[1]
                + t * t * t * b[1];
            let floor = base(z) - depth;
            let (i0, j0) = at(x - width, z - width);
            let (i1, j1) = at(x + width, z + width);
            for j in j0..=j1 {
                for i in i0..=i1 {
                    let d = ((gx(i) - x).powi(2) + (gz(j) - z).powi(2)).sqrt();
                    if d >= width {
                        continue;
                    }
                    let q = d / width;
                    let target = floor + depth * q * q * (3.0 - 2.0 * q);
                    let k = j * n + i;
                    if target < heights[k] {
                        heights[k] = target;
                    }
                }
            }
        }
    };
    for (a, b, depth, width) in &valleys {
        carve_valley(&mut heights, *a, *b, *depth, *width, &mut rng);
    }
    // A hollow where each person stands: a gentle bowl in the ground as it
    // is there, closed enough below to hold a pool.
    for p in &persons {
        let (ci, cj) = at(p.x, p.z);
        let centre = heights[cj * n + ci];
        let depth = 2.0 + p.class as f32 * 0.3;
        let r = 7.0 + p.class as f32 * 0.6;
        let (i0, j0) = at(p.x - r, p.z - r);
        let (i1, j1) = at(p.x + r, p.z + r);
        for j in j0..=j1 {
            for i in i0..=i1 {
                let d = ((gx(i) - p.x).powi(2) + (gz(j) - p.z).powi(2)).sqrt() / r;
                if d < 1.0 {
                    let k = j * n + i;
                    let target = centre - depth * (1.0 - d * d).powi(2);
                    if target < heights[k] {
                        heights[k] = target;
                    }
                }
            }
        }
    }
    // Smoothed, so that no carving leaves a step.
    for _ in 0..2 {
        let src = heights.clone();
        for j in 1..n - 1 {
            for i in 1..n - 1 {
                let k = j * n + i;
                heights[k] =
                    (src[k] * 4.0 + src[k - 1] + src[k + 1] + src[k - n] + src[k + n]) / 8.0;
            }
        }
    }

    // Where the water goes. The hollows are filled to their brims (the
    // priority flood, from the world's edge and the lake), so that every
    // drop finds its way down; each cell gives its water to its lowest
    // neighbour; the rain that falls and the springs where the people
    // stand gather downstream.
    let mut filled = heights.clone();
    let mut seen = vec![false; n * n];
    let mut queue = std::collections::BinaryHeap::new();
    for j in 0..n {
        for i in 0..n {
            let k = j * n + i;
            if i == 0 || j == 0 || i == n - 1 || j == n - 1 || heights[k] < water {
                seen[k] = true;
                queue.push(Low(filled[k], k as u32));
            }
        }
    }
    let steps8: [(i32, i32, f32); 8] = [
        (-1, 0, 1.0),
        (1, 0, 1.0),
        (0, -1, 1.0),
        (0, 1, 1.0),
        (-1, -1, std::f32::consts::SQRT_2),
        (1, -1, std::f32::consts::SQRT_2),
        (-1, 1, std::f32::consts::SQRT_2),
        (1, 1, std::f32::consts::SQRT_2),
    ];
    while let Some(Low(h, k)) = queue.pop() {
        let (i, j) = ((k as usize) % n, (k as usize) / n);
        for (di, dj, _) in steps8 {
            let (ni, nj) = (i as i32 + di, j as i32 + dj);
            if ni < 0 || nj < 0 || ni >= n as i32 || nj >= n as i32 {
                continue;
            }
            let m = nj as usize * n + ni as usize;
            if seen[m] {
                continue;
            }
            seen[m] = true;
            // A hollow is filled just above its rim, so that it still runs.
            filled[m] = filled[m].max(h + 1e-3);
            queue.push(Low(filled[m], m as u32));
        }
    }
    // The world's edge and the lake, where the water leaves the land.
    let sinks: Vec<bool> = (0..n * n)
        .map(|k| {
            let (i, j) = (k % n, k / n);
            i == 0 || j == 0 || i == n - 1 || j == n - 1 || heights[k] < water
        })
        .collect();
    let sink = |k: usize| -> bool { sinks[k] };
    let mut receiver = vec![u32::MAX; n * n];
    for k in 0..n * n {
        if sink(k) {
            continue;
        }
        let (i, j) = (k % n, k / n);
        let (mut best, mut drop) = (u32::MAX, 0.0f32);
        for (di, dj, dist) in steps8 {
            let (ni, nj) = (i as i32 + di, j as i32 + dj);
            if ni < 0 || nj < 0 || ni >= n as i32 || nj >= n as i32 {
                continue;
            }
            let m = nj as usize * n + ni as usize;
            let d = (filled[k] - filled[m]) / dist;
            if d > drop {
                drop = d;
                best = m as u32;
            }
        }
        receiver[k] = best;
    }
    let mut order: Vec<u32> = (0..(n * n) as u32).collect();
    order.sort_unstable_by(|a, b| filled[*b as usize].total_cmp(&filled[*a as usize]));
    let mut flow = vec![0.012f32; n * n];
    let mut springs = vec![false; n * n];
    for p in &persons {
        let (i, j) = at(p.x, p.z);
        let k = j * n + i;
        // A line that goes on runs on as a stream; a line that ends (class 1:
        // no descendant) rests in its pool, and what overflows is the rain's.
        if p.class > 1 {
            flow[k] += 5.0 + p.class as f32 * 3.0;
            springs[k] = true;
        } else {
            flow[k] += 1.2;
        }
    }
    for &k in &order {
        let r = receiver[k as usize];
        if r != u32::MAX {
            let f = flow[k as usize];
            flow[r as usize] += f;
        }
    }

    // The water the people's springs feed, all the way down.
    let mut spring_fed = springs.clone();
    for &k in &order {
        let r = receiver[k as usize];
        if r != u32::MAX && spring_fed[k as usize] {
            spring_fed[r as usize] = true;
        }
    }

    // The rivers: what the springs feed, and rain gathered enough to make a
    // stream of its own — the mountains have their torrents, not a comb of
    // them — traced from their sources down to where they join a river
    // already traced, or the lake.
    const ENOUGH: f32 = 3.0;
    const RAIN: f32 = 30.0;
    let river = |k: usize| !sink(k) && flow[k] >= if spring_fed[k] { ENOUGH } else { RAIN };
    let mut fed = vec![0u8; n * n];
    for (k, &r) in receiver.iter().enumerate() {
        if river(k) && r != u32::MAX {
            fed[r as usize] = fed[r as usize].saturating_add(1);
        }
    }
    let mut traced = vec![false; n * n];
    let mut heads: Vec<usize> = (0..n * n).filter(|&k| river(k) && fed[k] == 0).collect();
    // The longest first, so that a main river is one line.
    heads.sort_unstable_by(|a, b| filled[*b].total_cmp(&filled[*a]));
    let mut rivers: Vec<River> = Vec::new();
    let half_width = |f: f32| (0.34 * f.sqrt()).clamp(0.45, 5.5);
    for h in heads {
        let mut line: Vec<[f32; 5]> = Vec::new();
        let mut k = h;
        loop {
            let (i, j) = (k % n, k / n);
            let joined = traced[k];
            let foam = if fed[k] > 1 { 1.0 } else { 0.0 };
            line.push([gx(i), gz(j), filled[k], half_width(flow[k]), foam]);
            if joined {
                break;
            }
            traced[k] = true;
            let r = receiver[k];
            if r == u32::MAX {
                break;
            }
            let rk = r as usize;
            if sink(rk) {
                // Into the lake at its level; over the world's edge at the
                // edge's own height.
                let (ri, rj) = (rk % n, rk / n);
                let end = if heights[rk] < water {
                    water
                } else {
                    filled[rk].min(filled[k])
                };
                line.push([gx(ri), gz(rj), end, half_width(flow[k]) * 1.2, 0.3]);
                break;
            }
            k = rk;
        }
        if line.len() < 2 {
            continue;
        }
        let mut pts = chaikin(&line, 3);
        // The surface never climbs downstream, and a rapid foams.
        for q in 1..pts.len() {
            if pts[q][2] > pts[q - 1][2] {
                pts[q][2] = pts[q - 1][2];
            }
        }
        for q in 1..pts.len() {
            let run = ((pts[q][0] - pts[q - 1][0]).powi(2) + (pts[q][1] - pts[q - 1][1]).powi(2))
                .sqrt()
                .max(1e-3);
            let steep = ((pts[q - 1][2] - pts[q][2]) / run * 2.0).clamp(0.0, 0.45);
            pts[q][4] = pts[q][4].max(steep);
        }
        rivers.push(River {
            points: thin(&pts, cell * 0.6),
        });
    }

    // The beds: the ground under each river lowered, its banks rising to
    // meet the water's edge, so that the water lies in its bed and nowhere
    // else.
    // The land as it was before the rivers cut their beds: the hollows the
    // flood filled are measured on it, not on the beds.
    let uncut = heights.clone();
    let mut wet = vec![false; n * n];
    for r in &rivers {
        for w in r.points.windows(2) {
            let (a, b) = (w[0], w[1]);
            let len = ((b[0] - a[0]).powi(2) + (b[1] - a[1]).powi(2)).sqrt();
            let k = (len / (cell * 0.5)).ceil().max(1.0) as usize;
            for q in 0..=k {
                let t = q as f32 / k as f32;
                let (x, z) = (a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t);
                let surface = a[2] + (b[2] - a[2]) * t;
                // A bed at least a cell wide, so that the smallest brook shows.
                let hw = (a[3] + (b[3] - a[3]) * t).max(cell * 0.8);
                let reach = hw + 2.5 * cell;
                let (i0, j0) = at(x - reach, z - reach);
                let (i1, j1) = at(x + reach, z + reach);
                for j in j0..=j1 {
                    for i in i0..=i1 {
                        let d = ((gx(i) - x).powi(2) + (gz(j) - z).powi(2)).sqrt();
                        if d > reach {
                            continue;
                        }
                        let target = if d < hw {
                            surface - (0.35 + hw * 0.22) * (1.0 - (d / hw).powi(2)) - 0.05
                        } else {
                            surface - 0.05 + (d - hw) / (reach - hw) * 1.6
                        };
                        let m = j * n + i;
                        if target < heights[m] {
                            heights[m] = target;
                        }
                        if d < hw + cell {
                            wet[m] = true;
                        }
                    }
                }
            }
        }
    }

    // Still water: the hollows the flood filled, the pools where the people
    // stand first among them.
    let mut ponds: Vec<Pond> = Vec::new();
    let mut taken = vec![false; n * n];
    for k in 0..n * n {
        if taken[k] || sink(k) || filled[k] - uncut[k] < 0.6 {
            continue;
        }
        let mut stack = vec![k];
        taken[k] = true;
        let (mut sx, mut sz, mut count, mut level) = (0.0f32, 0.0f32, 0usize, f32::MIN);
        let mut cells = Vec::new();
        while let Some(c) = stack.pop() {
            let (i, j) = (c % n, c / n);
            sx += gx(i);
            sz += gz(j);
            count += 1;
            level = level.max(filled[c]);
            cells.push(c);
            for (di, dj, _) in steps8.iter().take(4) {
                let (ni, nj) = (i as i32 + di, j as i32 + dj);
                if ni < 0 || nj < 0 || ni >= n as i32 || nj >= n as i32 {
                    continue;
                }
                let m = nj as usize * n + ni as usize;
                if !taken[m] && !sink(m) && filled[m] - uncut[m] >= 0.12 {
                    taken[m] = true;
                    stack.push(m);
                }
            }
        }
        if count < 2 {
            continue;
        }
        let (cx, cz) = (sx / count as f32, sz / count as f32);
        let radius = cells
            .iter()
            .map(|c| ((gx(c % n) - cx).powi(2) + (gz(c / n) - cz).powi(2)).sqrt())
            .fold(0.0f32, f32::max)
            + cell;
        // Too shallow to be a pool (a film over the grass): left dry.
        let lowest = cells.iter().map(|c| heights[*c]).fold(f32::MAX, f32::min);
        if level - lowest < 0.7 {
            continue;
        }
        for c in &cells {
            wet[*c] = true;
        }
        cells.sort_unstable_by_key(|c| (c / n, c % n));
        let mut spans: Vec<[u16; 3]> = Vec::new();
        for c in &cells {
            let (i, j) = ((c % n) as u16, (c / n) as u16);
            match spans.last_mut() {
                Some(s) if s[0] == j && s[2] + 1 == i => s[2] = i,
                _ => spans.push([j, i, i]),
            }
        }
        ponds.push(Pond {
            x: cx,
            z: cz,
            level: level - 0.08,
            radius,
            spans,
        });
    }

    let height_at = |x: f32, z: f32| -> f32 {
        let fx = ((x - x0) / cell_x).clamp(0.0, (n - 1) as f32 - 0.001);
        let fz = ((z - z0) / cell_z).clamp(0.0, (n - 1) as f32 - 0.001);
        let (i, j) = (fx.floor() as usize, fz.floor() as usize);
        let (tx, tz) = (fx - i as f32, fz - j as f32);
        let h = |i: usize, j: usize| heights[j * n + i];
        let a = h(i, j) + (h(i + 1, j) - h(i, j)) * tx;
        let b = h(i, j + 1) + (h(i + 1, j + 1) - h(i, j + 1)) * tx;
        a + (b - a) * tz
    };
    let slope_at = |x: f32, z: f32| -> f32 {
        let dx = height_at(x + cell_x, z) - height_at(x - cell_x, z);
        let dz = height_at(x, z + cell_z) - height_at(x, z - cell_z);
        ((dx / (2.0 * cell_x)).powi(2) + (dz / (2.0 * cell_z)).powi(2)).sqrt()
    };
    let wet_near = |x: f32, z: f32, r: f32| -> bool {
        let (i0, j0) = at(x - r, z - r);
        let (i1, j1) = at(x + r, z + r);
        (j0..=j1).any(|j| (i0..=i1).any(|i| wet[j * n + i]))
    };
    for p in &mut persons {
        p.y = height_at(p.x, p.z);
    }

    // Footpaths: between partners, and from each person down to the water
    // nearest them, worn into the meadow.
    let mut paths: Vec<Vec<[f32; 3]>> = Vec::new();
    for (a, b) in &shape.unions {
        let (Some(&ia), Some(&ib)) = (index.get(a.as_str()), index.get(b.as_str())) else {
            continue;
        };
        let (pa, pb) = (&persons[ia as usize], &persons[ib as usize]);
        let len = ((pb.x - pa.x).powi(2) + (pb.z - pa.z).powi(2)).sqrt();
        if !(2.0..=SPACING * 14.0).contains(&len) {
            continue;
        }
        let bow = rng.range(-1.0, 1.0) * len * 0.12 + len * 0.08;
        let steps = ((len / 2.0).ceil() as usize).max(3);
        let path: Vec<[f32; 3]> = (0..=steps)
            .map(|s| {
                let t = s as f32 / steps as f32;
                let lift = 4.0 * t * (1.0 - t) * bow;
                [
                    pa.x + (pb.x - pa.x) * t,
                    pa.z + (pb.z - pa.z) * t - lift,
                    0.55,
                ]
            })
            .collect();
        paths.push(path);
    }

    // The forest, thick on the slopes and in clumps, thin on the valley
    // floor, none in the water, on the paths or above the trees' line.
    let treeline = base(top) + 70.0;
    let mut forest = vec![0u8; n * n];
    for j in 0..n {
        for i in 0..n {
            let (x, z) = (gx(i), gz(j));
            let k = j * n + i;
            let h = heights[k];
            if wet[k] || h < water + 0.6 || h > treeline {
                continue;
            }
            let clump = fbm(seed ^ 0x51ed, x / 70.0, z / 70.0) * 0.6
                + noise(seed ^ 0x77, x / 18.0, z / 18.0) * 0.4;
            let slope = slope_at(x, z);
            let rise = ((x.abs() - half).max(0.0) / 120.0).min(1.0);
            let d = (clump + 0.15 + rise * 0.5 + (slope - 0.15).clamp(0.0, 0.5) * 0.8
                - (slope - 1.0).max(0.0) * 2.0)
                .clamp(0.0, 1.0);
            forest[k] = (d * 255.0) as u8;
        }
    }
    // A clearing where each person stands.
    for p in &persons {
        let (i0, j0) = at(p.x - 16.0, p.z - 16.0);
        let (i1, j1) = at(p.x + 16.0, p.z + 16.0);
        for j in j0..=j1 {
            for i in i0..=i1 {
                let d = ((gx(i) - p.x).powi(2) + (gz(j) - p.z).powi(2)).sqrt();
                let k = j * n + i;
                forest[k] = (forest[k] as f32 * ((d - 9.0) / 7.0).clamp(0.0, 1.0)) as u8;
            }
        }
    }
    let forest_at = |x: f32, z: f32| -> f32 {
        let (i, j) = at(x, z);
        forest[j * n + i] as f32 / 255.0
    };
    let near_person = |x: f32, z: f32, r: f32| {
        persons
            .iter()
            .any(|p| (p.x - x).powi(2) + (p.z - z).powi(2) < r * r)
    };
    let on_path = |x: f32, z: f32, r: f32| {
        paths.iter().any(|path| {
            path.iter()
                .any(|q| (q[0] - x).powi(2) + (q[1] - z).powi(2) < r * r)
        })
    };

    let mut trees = Vec::new();
    for _ in 0..60_000 {
        if trees.len() >= 11_000 {
            break;
        }
        let x = rng.range(x0 + 4.0, x0 + size_x - 4.0);
        let z = rng.range(z0 + 4.0, z0 + size_z - 4.0);
        let f = forest_at(x, z);
        // Thick forest, and a lone tree now and then in the meadow.
        if rng.f() > f * f * 1.1 + 0.015 {
            continue;
        }
        let h = height_at(x, z);
        if h < water + 0.6 || h > treeline || slope_at(x, z) > 1.25 {
            continue;
        }
        if wet_near(x, z, 2.0) || near_person(x, z, 8.0) || on_path(x, z, 2.2) {
            continue;
        }
        let high = ((h - base(0.0)) / 70.0).clamp(0.0, 1.0);
        let kind = if rng.f() < 0.35 + high * 0.6 {
            0.0
        } else {
            1.0
        };
        let scale = rng.range(0.75, 1.35) * (1.15 - high * 0.3);
        trees.push([x, z, h, scale, kind, rng.f()]);
    }
    let mut rocks = Vec::new();
    for _ in 0..9_000 {
        if rocks.len() >= 3_200 {
            break;
        }
        let x = rng.range(x0 + 2.0, x0 + size_x - 2.0);
        let z = rng.range(z0 + 2.0, z0 + size_z - 2.0);
        let h = height_at(x, z);
        if h < water {
            continue;
        }
        let s = slope_at(x, z);
        // Boulders and scree where the ground is steep; stones by the water.
        if s > 0.7
            || (s > 0.45 && rng.f() < 0.4)
            || (wet_near(x, z, 3.0) && !wet_near(x, z, 0.8) && rng.f() < 0.12)
        {
            let big = if s > 0.9 {
                rng.range(0.8, 3.2)
            } else {
                rng.range(0.3, 1.4)
            };
            rocks.push([x, z, h, big]);
        }
    }
    let mut herds = Vec::new();
    for _ in 0..1_200 {
        if herds.len() >= 18 {
            break;
        }
        let x = rng.range(-half - 30.0, half + 30.0);
        let z = rng.range(top, bottom + 40.0);
        let h = height_at(x, z);
        if h < water + 0.5
            || slope_at(x, z) > 0.18
            || wet_near(x, z, 6.0)
            || forest_at(x, z) > 0.3
            || near_person(x, z, 10.0)
        {
            continue;
        }
        herds.push([x, z, h, (3.0 + rng.f() * 5.0).floor()]);
    }

    World {
        fingerprint: fp,
        n,
        x0,
        z0,
        size_x,
        size_z,
        heights,
        forest,
        water,
        persons,
        rivers,
        ponds,
        paths,
        trees,
        rocks,
        herds,
    }
}

impl World {
    /// The world as the page reads it, little-endian:
    ///
    /// ```text
    /// "AXW2"  u32 n  f32 x0 z0 size_x size_z  f32 water  f32 hmin hmax
    /// u16 × n·n           heights, hmin..hmax
    /// u8 × n·n            the forest's thickness
    /// u32 persons         per person: u8 id length, id, f32 x z y, u8 class, u16 generation
    /// u32 rivers          per river: u32 points, per point f32 x z y half-width foam
    /// u32 ponds           per pond: f32 x z level radius, u32 spans,
    ///                     per span u16 row first last
    /// u32 paths           per path: u32 points, per point f32 x z half-width
    /// u32 trees           per tree: f32 x z y scale kind tint
    /// u32 rocks           per rock: f32 x z y scale
    /// u32 herds           per herd: f32 x z y count
    /// ```
    pub fn encode(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(self.heights.len() * 3 + self.trees.len() * 24 + 65_536);
        fn f(out: &mut Vec<u8>, v: f32) {
            out.extend_from_slice(&v.to_le_bytes());
        }
        fn u(out: &mut Vec<u8>, v: usize) {
            out.extend_from_slice(&(v as u32).to_le_bytes());
        }
        out.extend_from_slice(b"AXW2");
        u(&mut out, self.n);
        for v in [self.x0, self.z0, self.size_x, self.size_z, self.water] {
            f(&mut out, v);
        }
        let (lo, hi) = self
            .heights
            .iter()
            .fold((f32::MAX, f32::MIN), |(a, b), &h| (a.min(h), b.max(h)));
        let (lo, hi) = if hi > lo { (lo, hi) } else { (lo, lo + 1.0) };
        f(&mut out, lo);
        f(&mut out, hi);
        for &h in &self.heights {
            let q = ((h - lo) / (hi - lo) * 65535.0).round().clamp(0.0, 65535.0) as u16;
            out.extend_from_slice(&q.to_le_bytes());
        }
        out.extend_from_slice(&self.forest);
        u(&mut out, self.persons.len());
        for p in &self.persons {
            let id = p.id.as_bytes();
            let len = id.len().min(255);
            out.push(len as u8);
            out.extend_from_slice(&id[..len]);
            for v in [p.x, p.z, p.y] {
                f(&mut out, v);
            }
            out.push(p.class);
            out.extend_from_slice(&p.generation.to_le_bytes());
        }
        u(&mut out, self.rivers.len());
        for r in &self.rivers {
            u(&mut out, r.points.len());
            for p in &r.points {
                for v in p {
                    f(&mut out, *v);
                }
            }
        }
        u(&mut out, self.ponds.len());
        for p in &self.ponds {
            for v in [p.x, p.z, p.level, p.radius] {
                f(&mut out, v);
            }
            u(&mut out, p.spans.len());
            for s in &p.spans {
                for v in s {
                    out.extend_from_slice(&v.to_le_bytes());
                }
            }
        }
        u(&mut out, self.paths.len());
        for path in &self.paths {
            u(&mut out, path.len());
            for q in path {
                for v in q {
                    f(&mut out, *v);
                }
            }
        }
        u(&mut out, self.trees.len());
        for t in &self.trees {
            for v in t {
                f(&mut out, *v);
            }
        }
        for list in [&self.rocks, &self.herds] {
            u(&mut out, list.len());
            for r in list.iter() {
                for v in r {
                    f(&mut out, *v);
                }
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn person(id: &str) -> Value {
        json!({"id": id, "type": "person", "identity": {"name": {"display": id}}})
    }

    /// Two grandparents' couples, their children married, a grandchild.
    fn family() -> Value {
        json!({
            "persons": {
                "a": person("a"), "b": person("b"), "c": person("c"), "d": person("d"),
                "e": person("e"), "f": person("f"), "g": person("g")
            },
            "families": {
                "f1": {"id": "f1", "union": {"persons": [
                    {"person_id": "a", "role": "spouse"}, {"person_id": "b", "role": "spouse"}]},
                    "children": [{"person_id": "e"}]},
                "f2": {"id": "f2", "union": {"persons": [
                    {"person_id": "c", "role": "spouse"}, {"person_id": "d", "role": "spouse"}]},
                    "children": [{"person_id": "f"}]},
                "f3": {"id": "f3", "union": {"persons": [
                    {"person_id": "e", "role": "spouse"}, {"person_id": "f", "role": "spouse"}]},
                    "children": [{"person_id": "g"}]}
            }
        })
    }

    #[test]
    fn the_shape_is_who_is_there_and_who_is_whose() {
        let s = shape_of(&family());
        assert_eq!(s.persons, ["a", "b", "c", "d", "e", "f", "g"]);
        assert!(s.children.contains(&("a".into(), "e".into())));
        assert!(s.children.contains(&("f".into(), "g".into())));
        assert_eq!(s.children.len(), 6);
        assert_eq!(
            s.unions,
            [
                ("a".into(), "b".into()),
                ("c".into(), "d".into()),
                ("e".into(), "f".into())
            ]
        );
    }

    #[test]
    fn a_name_changed_keeps_the_fingerprint_a_person_added_does_not() {
        let mut f = family();
        let before = fingerprint(&shape_of(&f));
        f["persons"]["a"]["identity"]["name"]["display"] = json!("Another name");
        f["persons"]["a"]["birth"] = json!({"date": {"value": "1890"}});
        assert_eq!(
            fingerprint(&shape_of(&f)),
            before,
            "words are not the shape"
        );
        f["persons"]["h"] = person("h");
        let added = fingerprint(&shape_of(&f));
        assert_ne!(added, before, "a person added");
        f["families"]["f3"]["children"]
            .as_array_mut()
            .unwrap()
            .push(json!({"person_id": "h"}));
        assert_ne!(fingerprint(&shape_of(&f)), added, "a link made");
        assert_eq!(before.len(), 32);
    }

    #[test]
    fn the_same_shape_builds_the_same_world() {
        let a = build(&family());
        let b = build(&family());
        assert_eq!(a, b, "seeded by the shape");
        assert_eq!(a.encode(), b.encode());
        assert_eq!(a.heights.len(), a.n * a.n);
        assert_eq!(a.forest.len(), a.n * a.n);
        assert_eq!(a.persons.len(), 7);
        assert!(!a.rivers.is_empty(), "water runs");
        assert!(!a.trees.is_empty() && !a.rocks.is_empty());
        assert_eq!(a.paths.len(), 3, "a path between each pair of partners");
    }

    #[test]
    fn the_people_stand_down_the_valley_generation_by_generation() {
        let w = build(&family());
        let spot = |id: &str| w.persons.iter().find(|p| p.id == id).unwrap().clone();
        let (a, e, g) = (spot("a"), spot("e"), spot("g"));
        assert!(a.z < e.z && e.z < g.z, "down the valley");
        assert!(a.y > e.y && e.y > g.y, "and down the slope");
        assert_eq!(g.class, 1);
        assert_eq!(a.class, crate::river::class_of(3) as u8);
    }

    #[test]
    fn water_runs_downhill_joins_and_never_crosses() {
        let w = build(&family());
        for r in &w.rivers {
            for p in r.points.windows(2) {
                assert!(p[1][2] <= p[0][2] + 1e-4, "no water runs uphill");
            }
            assert!(r.points.iter().all(|p| p[3] > 0.0));
        }
        // Each river ends on another one, in the lake, or over the world's
        // edge: a tree of water.
        let edge = |x: f32, z: f32| {
            x < w.x0 + 8.0
                || z < w.z0 + 8.0
                || x > w.x0 + w.size_x - 8.0
                || z > w.z0 + w.size_z - 8.0
        };
        let ends_somewhere = w.rivers.iter().all(|r| {
            let end = r.points[r.points.len() - 1];
            end[2] <= w.water + 1e-3
                || edge(end[0], end[1])
                || w.rivers.iter().any(|o| {
                    !std::ptr::eq(o, r)
                        && o.points
                            .iter()
                            .any(|q| (q[0] - end[0]).abs() < 4.0 && (q[1] - end[1]).abs() < 4.0)
                })
        });
        assert!(
            ends_somewhere,
            "every river joins another or reaches the lake"
        );
        // A spring where each person stands: water near every one.
        for p in &w.persons {
            let near = w.rivers.iter().any(|r| {
                r.points
                    .iter()
                    .any(|q| (q[0] - p.x).powi(2) + (q[1] - p.z).powi(2) < 100.0)
            }) || w
                .ponds
                .iter()
                .any(|o| (o.x - p.x).powi(2) + (o.z - p.z).powi(2) < (o.radius + 6.0).powi(2));
            assert!(near, "{} has water by them", p.id);
        }
    }

    #[test]
    fn the_world_reads_back_from_its_bytes() {
        let w = build(&family());
        let bytes = w.encode();
        assert_eq!(&bytes[..4], b"AXW2");
        let n = u32::from_le_bytes(bytes[4..8].try_into().unwrap()) as usize;
        assert!((GRID..=GRID_MAX).contains(&n));
        // header 8 + 7 floats, then the heights, the forest, the persons' count.
        let at = 8 + 7 * 4 + n * n * 2 + n * n;
        let persons = u32::from_le_bytes(bytes[at..at + 4].try_into().unwrap());
        assert_eq!(persons, 7);
        assert_eq!(bytes[at + 4], 1, "ids carried whole");
    }
}
