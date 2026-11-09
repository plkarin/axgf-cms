//! Canopée's world (docs/adr/0043): the family's land, built once for each
//! shape of the family.
//!
//! The family's **shape** is who is there and who is whose parent and
//! partner — no name, no date. Its **fingerprint** names it, and the same
//! shape always builds the same world: the land's relief, its streams, its
//! trees, rocks and herds are drawn from a random seeded by the fingerprint.
//! So the world is built once per shape and kept; a person added or removed,
//! a link made or unmade, makes a new shape and a new world, whole. A name or
//! a date changed does not.
//!
//! The land: the whole family's layout (`tree::layout`) laid on a valley, the
//! oldest generation up in the mountains where the springs are, each younger
//! one further down, the youngest by the lake at its foot. A parent's water
//! runs down to each child; the beds are carved into the ground, wider and
//! deeper as they carry more descendants; a pool where each person stands.
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

/// A parent's water running down to a child.
#[derive(Debug, Clone, PartialEq)]
pub struct Stream {
    pub from: u16,
    pub to: u16,
    /// x, z, the water's height, its width.
    pub points: Vec<[f32; 4]>,
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
    /// n × n, row by row along z.
    pub heights: Vec<f32>,
    /// The lake's level.
    pub water: f32,
    pub persons: Vec<Spot>,
    pub streams: Vec<Stream>,
    /// x, z, y, scale, kind (0 a conifer, 1 a broadleaf).
    pub trees: Vec<[f32; 5]>,
    /// x, z, y, scale.
    pub rocks: Vec<[f32; 4]>,
    /// x, z, y, how many animals.
    pub herds: Vec<[f32; 4]>,
}

/// Grid points per side of the heightfield.
pub const GRID: usize = 257;
/// Across the valley, between two people of one generation, in the world's
/// units.
const SPACING: f32 = 9.0;
/// Down the valley, between two generations.
const GENERATION: f32 = 70.0;
/// How far the ground falls from one generation to the next.
const FALL: f32 = 9.0;

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

/// Build the world of the family in `flat`.
pub fn build(flat: &Value) -> World {
    let shape = shape_of(flat);
    let fp = fingerprint(&shape);
    let seed = Rng::new(&fp).next();
    let mut rng = Rng::new(&fp);

    // The family laid out as the tree lays it: generations, and an order
    // across each that keeps families together.
    let layout = crate::tree::layout(flat);
    // The tree draws the youngest at the top; the valley starts with the
    // oldest, where the springs are.
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

    // The land around them: mountains beyond the family on either side, the
    // springs above the oldest generation, the lake below the youngest.
    // Wide enough that the mist hides the world's edge from wherever the
    // camera may stand.
    let margin_x = 260.0f32.max(span * 0.45);
    let size_x = span + 2.0 * margin_x;
    let top = -(gens as f32 - 1.0) * GENERATION / 2.0;
    let bottom = (gens as f32 - 1.0) * GENERATION / 2.0;
    let z0 = top - 260.0;
    let size_z = (bottom + 300.0) - z0;
    let x0 = -size_x / 2.0;
    let n = GRID;
    let cell_x = size_x / (n - 1) as f32;
    let cell_z = size_z / (n - 1) as f32;
    // The ground's fall down the valley: 0 at the lake's shore, FALL a
    // generation above it.
    let base = |z: f32| -> f32 { (bottom + 40.0 - z) / GENERATION * FALL };
    let water = base(bottom + 120.0) - 1.5;

    let mut heights = vec![0.0f32; n * n];
    for j in 0..n {
        let z = z0 + j as f32 * cell_z;
        for i in 0..n {
            let x = x0 + i as f32 * cell_x;
            let mut h = base(z);
            // The valley's walls, rising beyond the family.
            let out = (x.abs() - half - 20.0).max(0.0);
            h += out * out * 0.0016 + out * 0.15;
            // Above the springs, the mountains close the valley.
            let above = (top - 40.0 - z).max(0.0);
            h += above * above * 0.004;
            // The relief: gentle on the valley floor, strong in the
            // mountains.
            let rough = 3.0 + (out + above) * 0.12;
            h += fbm(seed, x / 55.0, z / 55.0) * rough;
            // The lake's basin, below the youngest generation.
            let lake = (z - (bottom + 70.0)).max(0.0);
            if lake > 0.0 {
                let shore = (lake / 60.0).min(1.0);
                h = h * (1.0 - shore) + (water - 4.0) * shore;
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
            if persons.len() >= u16::MAX as usize {
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

    // The family's water. A child's parents' streams meet first, at a
    // confluence below the couple, and one river runs on from it, dividing
    // toward each of their children: the parents' water becomes the
    // children's. A child with one parent recorded is fed by that parent's
    // stream alone. Each stream is a curve down the valley, meandering a
    // little, as wide as the descendants it carries.
    let mut streams: Vec<Stream> = Vec::new();
    // Half the water's width: a brook for one person, a river for a great
    // line, always narrower than the space between two people.
    let half_width = |class: u8| 0.3 + class as f32 * 0.18;
    let curve = |rng: &mut Rng, a: [f32; 2], b: [f32; 2], w0: f32, w1: f32| -> Vec<[f32; 4]> {
        let wiggle = rng.range(-1.0, 1.0) * (b[1] - a[1]) * 0.12;
        let (c1, c2) = (
            [a[0] + wiggle, a[1] + (b[1] - a[1]) * 0.45],
            [b[0] - wiggle, b[1] - (b[1] - a[1]) * 0.45],
        );
        let steps = 24;
        (0..=steps)
            .map(|s| {
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
                [x, z, 0.0, w0 + (w1 - w0) * t]
            })
            .collect()
    };
    // Each child's parents, as the world has placed them.
    let mut parents_of: BTreeMap<u16, Vec<u16>> = BTreeMap::new();
    for (p, c) in &shape.children {
        if let (Some(&a), Some(&b)) = (index.get(p.as_str()), index.get(c.as_str())) {
            // No water runs uphill: a child laid beside or above a parent (a
            // contradiction the tree reports) is not fed by them.
            if persons[b as usize].z > persons[a as usize].z {
                parents_of.entry(b).or_default().push(a);
            }
        }
    }
    // The children of each pair of parents, and of each lone parent.
    let mut by_parents: BTreeMap<Vec<u16>, Vec<u16>> = BTreeMap::new();
    for (child, mut ps) in parents_of {
        ps.sort_unstable();
        ps.dedup();
        ps.truncate(2);
        by_parents.entry(ps).or_default().push(child);
    }
    for (ps, kids) in &by_parents {
        let class = kids
            .iter()
            .map(|k| persons[*k as usize].class)
            .max()
            .unwrap_or(1)
            .min(5)
            + 1;
        let class = class.min(5);
        if ps.len() == 2 {
            let (a, b) = (&persons[ps[0] as usize], &persons[ps[1] as usize]);
            let lowest = kids
                .iter()
                .map(|k| persons[*k as usize].z)
                .fold(f32::MAX, f32::min);
            let top = a.z.max(b.z);
            let meet = [(a.x + b.x) / 2.0, top + (lowest - top) * 0.38];
            for parent in [a, b] {
                let points = curve(
                    &mut rng,
                    [parent.x, parent.z],
                    meet,
                    half_width(parent.class.min(3)),
                    half_width(class),
                );
                streams.push(Stream {
                    from: ps[0],
                    to: u16::MAX,
                    points,
                });
            }
            for k in kids {
                let child = &persons[*k as usize];
                let points = curve(
                    &mut rng,
                    meet,
                    [child.x, child.z],
                    half_width(class),
                    half_width(child.class),
                );
                streams.push(Stream {
                    from: u16::MAX,
                    to: *k,
                    points,
                });
            }
        } else if let Some(&p) = ps.first() {
            let parent = &persons[p as usize];
            for k in kids {
                let child = &persons[*k as usize];
                let points = curve(
                    &mut rng,
                    [parent.x, parent.z],
                    [child.x, child.z],
                    half_width(parent.class.min(3)),
                    half_width(child.class),
                );
                streams.push(Stream {
                    from: p,
                    to: *k,
                    points,
                });
            }
        }
    }

    // The beds carved into the ground, and the pools where the people stand.
    let mut wet = vec![false; n * n];
    let at = |x: f32, z: f32| -> (usize, usize) {
        (
            (((x - x0) / cell_x).round().clamp(0.0, (n - 1) as f32)) as usize,
            (((z - z0) / cell_z).round().clamp(0.0, (n - 1) as f32)) as usize,
        )
    };
    let carve =
        |heights: &mut Vec<f32>, wet: &mut Vec<bool>, x: f32, z: f32, radius: f32, level: f32| {
            let reach = radius * 2.5;
            let (i0, j0) = at(x - reach, z - reach);
            let (i1, j1) = at(x + reach, z + reach);
            for j in j0..=j1 {
                for i in i0..=i1 {
                    let (gx, gz) = (x0 + i as f32 * cell_x, z0 + j as f32 * cell_z);
                    let d = ((gx - x).powi(2) + (gz - z).powi(2)).sqrt();
                    if d > reach {
                        continue;
                    }
                    // The bed, then its banks rising to the ground around.
                    let target = if d < radius {
                        level - 0.9 * (1.0 - (d / radius).powi(2))
                    } else {
                        level + (d - radius) / (reach - radius) * 3.0
                    };
                    let k = j * n + i;
                    if target < heights[k] {
                        heights[k] = target;
                    }
                    if d < radius {
                        wet[k] = true;
                    }
                }
            }
        };
    for s in &mut streams {
        let (za, zb) = (s.points[0][1], s.points[s.points.len() - 1][1]);
        let (la, lb) = (base(za) - 0.6, base(zb) - 0.6);
        for p in &mut s.points {
            let t = if zb > za {
                (p[1] - za) / (zb - za)
            } else {
                0.0
            };
            p[2] = la + (lb - la) * t.clamp(0.0, 1.0);
        }
        // Carved between the samples too, so that the bed has no gaps.
        for w in s.points.windows(2) {
            let (a, b) = (w[0], w[1]);
            let len = ((b[0] - a[0]).powi(2) + (b[1] - a[1]).powi(2)).sqrt();
            let k = (len / (cell_x.min(cell_z) * 0.5)).ceil().max(1.0) as usize;
            for q in 0..=k {
                let t = q as f32 / k as f32;
                carve(
                    &mut heights,
                    &mut wet,
                    a[0] + (b[0] - a[0]) * t,
                    a[1] + (b[1] - a[1]) * t,
                    a[3] + (b[3] - a[3]) * t,
                    a[2] + (b[2] - a[2]) * t,
                );
            }
        }
    }
    for p in &persons {
        carve(
            &mut heights,
            &mut wet,
            p.x,
            p.z,
            2.6 + p.class as f32 * 0.4,
            base(p.z) - 0.9,
        );
    }

    // One pass of smoothing, so that the carving's edges are not steps.
    let mut smooth = heights.clone();
    for j in 1..n - 1 {
        for i in 1..n - 1 {
            let k = j * n + i;
            let sum = heights[k] * 4.0
                + heights[k - 1]
                + heights[k + 1]
                + heights[k - n]
                + heights[k + n];
            smooth[k] = sum / 8.0;
        }
    }
    let heights = smooth;
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
        p.y = height_at(p.x, p.z).max(base(p.z) - 1.4);
    }

    // Its life: trees where the ground holds them, rocks where it is steep,
    // herds in the meadows. The same shape, the same places.
    let treeline = base(top) + 55.0;
    let mut trees = Vec::new();
    for _ in 0..14_000 {
        let x = rng.range(x0 + 4.0, x0 + size_x - 4.0);
        let z = rng.range(z0 + 4.0, z0 + size_z - 4.0);
        let h = height_at(x, z);
        if h < water + 0.5 || h > treeline {
            continue;
        }
        let s = slope_at(x, z);
        // Forest gathers on the slopes and in clumps; the flat floor of the
        // valley is meadow.
        let clump = noise(seed ^ 0x51ed, x / 30.0, z / 30.0);
        if s > 1.1 || (s < 0.12 && clump < 0.25) || clump < -0.35 {
            continue;
        }
        if wet_near(x, z, 3.0) {
            continue;
        }
        // A clearing where each person stands, so that their mark and their
        // name are seen.
        if persons
            .iter()
            .any(|p| (p.x - x).powi(2) + (p.z - z).powi(2) < 49.0)
        {
            continue;
        }
        let high = (h - base(0.0)) / 60.0;
        let kind = if high > 0.25 || rng.f() < 0.45 {
            0.0
        } else {
            1.0
        };
        trees.push([x, z, h, rng.range(0.75, 1.5), kind]);
    }
    let mut rocks = Vec::new();
    for _ in 0..2_400 {
        let x = rng.range(x0 + 2.0, x0 + size_x - 2.0);
        let z = rng.range(z0 + 2.0, z0 + size_z - 2.0);
        let h = height_at(x, z);
        if h < water {
            continue;
        }
        let s = slope_at(x, z);
        if s > 0.55 || (wet_near(x, z, 4.0) && rng.f() < 0.25) {
            rocks.push([x, z, h, rng.range(0.6, 2.6) * (1.0 + s.min(2.0) * 0.6)]);
        }
    }
    let mut herds = Vec::new();
    for _ in 0..600 {
        if herds.len() >= 24 {
            break;
        }
        let x = rng.range(-half - 10.0, half + 10.0);
        let z = rng.range(top, bottom + 40.0);
        let h = height_at(x, z);
        if h < water + 0.5 || slope_at(x, z) > 0.14 || wet_near(x, z, 6.0) {
            continue;
        }
        herds.push([x, z, h, (3.0 + rng.f() * 6.0).floor()]);
    }

    World {
        fingerprint: fp,
        n,
        x0,
        z0,
        size_x,
        size_z,
        heights,
        water,
        persons,
        streams,
        trees,
        rocks,
        herds,
    }
}

impl World {
    /// The world as the page reads it, little-endian:
    ///
    /// ```text
    /// "AXW1"  u32 n  f32 x0 z0 size_x size_z  f32 water  f32 hmin hmax
    /// u16 × n·n           heights, hmin..hmax
    /// u32 persons         per person: u8 id length, id, f32 x z y, u8 class, u16 generation
    /// u32 streams         per stream: u16 from to, u16 points, per point f32 x z y width
    /// u32 trees           per tree: f32 x z y scale kind
    /// u32 rocks           per rock: f32 x z y scale
    /// u32 herds           per herd: f32 x z y count
    /// ```
    pub fn encode(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(self.heights.len() * 2 + self.trees.len() * 20 + 4096);
        let f = |out: &mut Vec<u8>, v: f32| out.extend_from_slice(&v.to_le_bytes());
        out.extend_from_slice(b"AXW1");
        out.extend_from_slice(&(self.n as u32).to_le_bytes());
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
        out.extend_from_slice(&(self.persons.len() as u32).to_le_bytes());
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
        out.extend_from_slice(&(self.streams.len() as u32).to_le_bytes());
        for s in &self.streams {
            out.extend_from_slice(&s.from.to_le_bytes());
            out.extend_from_slice(&s.to.to_le_bytes());
            out.extend_from_slice(&(s.points.len() as u16).to_le_bytes());
            for p in &s.points {
                for v in p {
                    f(&mut out, *v);
                }
            }
        }
        out.extend_from_slice(&(self.trees.len() as u32).to_le_bytes());
        for t in &self.trees {
            for v in t {
                f(&mut out, *v);
            }
        }
        out.extend_from_slice(&(self.rocks.len() as u32).to_le_bytes());
        for r in &self.rocks {
            for v in r {
                f(&mut out, *v);
            }
        }
        out.extend_from_slice(&(self.herds.len() as u32).to_le_bytes());
        for h in &self.herds {
            for v in h {
                f(&mut out, *v);
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
        assert_eq!(a.heights.len(), GRID * GRID);
        assert_eq!(a.persons.len(), 7);
        // Each couple: its two streams to their confluence, and one on to
        // their child.
        assert_eq!(
            a.streams.len(),
            9,
            "two tributaries and a river for each couple"
        );
        assert!(!a.trees.is_empty() && !a.rocks.is_empty());
    }

    #[test]
    fn water_runs_down_from_the_oldest_to_the_youngest() {
        let w = build(&family());
        let spot = |id: &str| w.persons.iter().find(|p| p.id == id).unwrap().clone();
        let (a, e, g) = (spot("a"), spot("e"), spot("g"));
        assert!(
            a.z < e.z && e.z < g.z,
            "down the valley, generation by generation"
        );
        assert!(a.y > e.y && e.y > g.y, "and down the slope");
        for s in &w.streams {
            for p in s.points.windows(2) {
                assert!(p[1][2] <= p[0][2] + 1e-4, "no water runs uphill");
            }
        }
        // The grandchild carries nobody: the smallest class; the
        // grandparents carry two.
        assert_eq!(g.class, 1);
        assert_eq!(a.class, crate::river::class_of(3) as u8);
    }

    #[test]
    fn the_world_reads_back_from_its_bytes() {
        let w = build(&family());
        let bytes = w.encode();
        assert_eq!(&bytes[..4], b"AXW1");
        let n = u32::from_le_bytes(bytes[4..8].try_into().unwrap()) as usize;
        assert_eq!(n, GRID);
        // header 8 + 7 floats, then the heights, then the persons' count.
        let at = 8 + 7 * 4 + n * n * 2;
        let persons = u32::from_le_bytes(bytes[at..at + 4].try_into().unwrap());
        assert_eq!(persons, 7);
        let first = bytes[at + 4] as usize;
        assert_eq!(first, 1, "ids carried whole");
    }
}
