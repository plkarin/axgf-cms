# ADR 0043 — Canopée: a world built from the family

**Status:** proposed (in progress) · **Builds on:** docs/adr/0004 (the
river, §11–§17), docs/adr/0007 (the themes' living scenes) · **Applies to:**
`src/world.rs`, `src/routes/world.rs`, `src/state.rs`,
`static/canopee-world.js`, `templates/index.html` (the river), `static/app.css`

## Context

On 9 November 2026 the operator asked for Canopée to become **one 3D
scene**. Until now the river is two layers: the family's water drawn by
`river.js` over a photograph of a valley (`river-scene.js`) that a depth
map moves in relief. When the reader travels in the river, the photograph
moves too, then drifts down on its own: the water and the land are not the
same world.

What was asked:

- a **terrain and a river built from scratch from the family**: the land
  and its water *are* the family, a stream added for each person;
- trees, mountains, animals, all in 3D, as in a video game;
- the reader then only **moves within that scene**;
- **no regeneration on every click**: the world is generated once, and
  generated again, whole, when a person is added or removed.

## Decision

### 1. One scene, one coordinate space

The ground, its water, its trees, rocks and animals, the people's marks and
the sky are one WebGL scene. The camera moves; nothing else does. The river
page under Canopée shows it where WebGL 2 is available; elsewhere, and in
every other theme, the river stays as it is.

### 2. The family's shape, and its fingerprint

The world is built from the family's **shape** only: who is there, and
who is whose parent and partner — never a name or a date. A **fingerprint**
(SHA-256 over the sorted person ids and the sorted parent→child and union
links) names a shape. The same shape always builds the same world (its
random choices are seeded by the fingerprint), so the world is **built once
per shape**: kept in memory, served at `/canopee/world.bin?v=<fingerprint>`
with a year's `immutable` cache, and built again only when the fingerprint
changes — a person added or removed, a link made or unmade. A name or a
date changed does not touch it.

What changes more often — names, years, the era's colour of each person's
water — is a small separate payload, fetched with the page, laid over the
world without rebuilding it. The privacy of the river applies: the shape is
the same for every reader (as `access::river_shape`), the words only for
those the reader's lens admits (ADR 0001).

### 3. The family laid on the land

The whole family's layout (`tree::layout`: generations, and an order across
each that keeps families together and crossings few) is laid on the land:
the oldest generation up in the mountains, where the springs are; each
younger one further down the valley; the youngest by the lake at its foot.
A person is a point on the ground; a parent's water runs down to each child,
so streams join where two parents meet in their children, and grow with the
descendants they carry (the river's five discharge classes, ADR 0004).

### 4. The land

A heightfield: a slope from the springs down to the lake, mountains on
either side, relief from seeded fractal noise, and the beds of the family's
streams and rivers carved into it, wider and deeper downstream; a pool where
a person stands. Meadows on the flat, forest on the slopes, rock and scree
where it is steep, snow on the highest summits — by height and slope, in the
ground's own shader.

### 5. Its life

Trees (conifers high, broadleaves low), rocks and boulders, herds grazing in
the meadows, an eagle, birds: instances placed once, from the seeded random
of the shape, where the ground allows (not in water, not on cliffs). The
water flows downstream in its shader; mist lies on the lake; the sun is low
on the left, as in the photograph it replaces. Reduced motion, the river's
pause and "Cut" hold everything still.

### 6. Moving in it

Drag to look around, wheel or pinch to come closer, the arrows to walk;
choosing a person — in the scene, in the itinerary, in the search — flies
the camera to them and opens their record beside the river, as the river
does today. Names are drawn by the page over the scene, for the people near
the camera.

## Consequences

- The river under Canopée becomes a place: the reader travels through the
  family's land instead of over a picture of a valley.
- The world is rebuilt only by what changes its shape; the page builds its
  buffers once and then only moves its camera.
- The photograph, its maps and `river-scene.js` stay for the browsers
  without WebGL 2 and for the other ways of reading the river.
- Work in stages. **Stage 1 (9 November 2026)**: the shape, the
  fingerprint and the world served (`src/world.rs`, a 257² heightfield, the
  streams, ~5,000 trees, rocks and herds, ~400 KB for 213 people, built in
  a tenth of a second and kept); the scene (`static/canopee-world.js`, WebGL
  2, no library): the ground shaded by height and slope, the water flowing,
  instanced conifers, broadleaves, boulders and grazing animals, the sky and
  its mist, the people's rings and their names placed without covering one
  another; the camera turned, brought closer, walked, flown to whoever the
  river travels to; a name or a ring chosen travels the river. To come: the
  ground's detail (grass, paths, scree), water that follows its bed, birds
  and the eagle, the light of the hour, and the performance measured on the
  operator's family (~600 people).
