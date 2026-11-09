# ADR 0043 — Canopée: a world built from the family

**Status:** withdrawn, v1.12.1 (2026-11-09; released in v1.12.0) · **Builds on:** docs/adr/0004 (the
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
A person is a point on the ground, twelve units from the next one across
the valley and eighty-eight from the next generation down: room enough for
their streams to run apart before they join (the operator's review of
9 November 2026: the rivers were too close).

### 4. The family carves the land; the water does what water does

The family does not draw its water: it carves the valleys, and the water
finds them. A valley runs from each parent down to the couple's confluence
and on to their children, deepest toward the child who carries most of the
line (the river's five discharge classes, ADR 0004); a hollow where each
person stands. Then, as water does: the hollows are filled to their brims
(a priority flood from the lake and the world's edge), each point gives its
water to its lowest neighbour, the rain gathers, and each person is a
spring: a line that goes on runs on as a stream, a line that ends (no
descendant) rests in its pool. A river is what the springs feed, or a
large mountain catchment's own torrent; it is traced from its source down to where it joins another or
reaches the lake, smoothed, its surface never climbing downstream, white
where it meets another or falls. **So the streams join and never cross**,
and grow on their way down. Their beds are carved under them — at least a
cell of the ground wide, their banks rising to meet the water's edge — so
the water lies in its bed and nowhere else; the hollows the flood filled
are pools, sent with their own cells, so that a pool fills its hollow and no
other.

The land: a heightfield of 513² points (up to 769² for a wide family, so
that its streams stay fine), a slope from the springs to the lake,
mountains on either side with sharp crests, relief from seeded noise; the
forest thick on the slopes and in clumps, a clearing where each person
stands, in a gentle bowl that holds the person's pool; footpaths between
partners. Beyond the heightfield the mountains go
on and the lake reaches the horizon, so the world has no edge to see.

### 5. Its look and its life

The ground is shaded in its own shader: meadows of two greens and dry
patches with their flowers, the forest's floor under the trees, scree then
rock where it is steep, snow on the summits, sand on the lake's shore, the
dark wet line just above the water and the stones of the bed under it. The
water, drawn where it lies (the rivers at their own level, the pools, the
lake as one surface), knows how deep it is at every point: clear and brown
over the shallows, dark where it is deep, its waves carried by its flow,
the sky and the sun in it, foam where it meets and falls. Grass grows thick
around what the camera looks at, grown by the graphics card on the meadows
only and bending in the wind. The trees are trunks, limbs and foliage: a
fir is whorls of needle sprays, narrowing to its top; a broadleaf is clumps
of leaves over a few rounded masses with the light between them, each tuft
facing the camera and cut leaf by leaf in the shader (simpler shapes far
away and for the shadows); they sway in gusts, with the sun through their
leaves. Boulders have their grain and lichen; deer, rounded, wander a
little and graze; flocks of birds cross the valley and an eagle circles
over the person the camera looks at. The water mirrors the sky above and,
low down, its banks and their trees, and the sun's ripples dance on the
stones of its bed (the operator's review of 9 November 2026: the shapes
were to look true).

The light is the reader's hour: the sun rises, crosses and sets, the hours
around are gold, the night is blue under the moon and its stars; the sun's
shadows fall around the camera's target; mist on the distance and in the
sun's direction. Reduced motion, the river's pause and "Cut" hold everything
still.

### 6. Moving in it

The camera flies high over the valley, looking down at its target (the
operator's review of 9 November 2026: the navigation higher, the controls
simpler). One moves as on a map: drag the land, the wheel or two fingers
apart to come closer, the right button, Ctrl or two fingers turning to turn
and to look from higher or lower, a double click to go to a place; the
arrows walk (with Shift they turn), Home comes back to the person. A pad in
the scene's corner does the same with a click, or as long as a button is
held: forward, back, left, right, turn either way, look from higher or
lower, closer, farther, and back to the person; the first times, a line
says how to move. Choosing a person — in the scene, in the itinerary, in
the search — flies the camera to them and opens their record beside the
river, as the river does today. The camera rises round its target rather
than go into the ground, the trees or the water, and a tree between it and
that person steps aside. Names are drawn by the page over the scene, for the people near the
camera, none written over another.

### 7. As fast as the reader's machine allows

The trees, boulders and herds are kept in chunks of the land: what the
camera does not see is not drawn, what is far is drawn simpler. The page
measures how fast its frames come and lowers its own detail — resolution,
shadows, grass, the reach of the detailed trees, over four levels — when
they come slowly, raising it again when there is room. `?world=fps` shows
the frame time, the script's share of it, the level, the resolution and the
graphics card; `?world=q0`…`q3` holds a level and `?world=h<hour>` an hour,
to compare.

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
  river travels to; a name or a ring chosen travels the river.
  **Stage 2 (9 November 2026)**, after the operator found the water and its
  crossings unnatural: the hydrology of §4 (the water runs where the family
  carved, joins, never crosses; pools by their cells; 513² ground at the least), the look
  and life of §5, the camera of §6 and the measured detail of §7; then,
  after the operator's second review the same day, the people spaced, the
  lines that end resting in their pools, the trees, deer, stones and water
  made to look true, the camera higher and the pad. Measured on fictitious
  families: the world is about 1.3 MB for 213 people and 2.5 MB for 492
  (769² points), built once per shape; the script spends about a
  millisecond a frame, so the frame rate is the graphics card's — in a
  software renderer on two cores, 18 frames a second at full detail and 39
  at the third level; a real card does far better, and `?world=fps` shows it
  on the reader's own machine.

## Withdrawn

The same day, having seen the world on the family's own land, the operator
judged it ugly and asked for the state before it: the river under Canopée
is again the family's water drawn by `river.js` over the photograph of a
valley (`river-scene.js`, docs/adr/0007), as in v1.11.0. The world's code
(`src/world.rs`, `src/routes/world.rs`, `static/canopee-world.js`,
`/canopee/world.bin`, `/canopee/words.json`) is removed; this record stays,
with what was tried and measured, for whoever takes the idea up again.
