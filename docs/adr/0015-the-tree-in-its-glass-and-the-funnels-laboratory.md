# ADR 0015 — Fusion's tree in its glass; Clarity's funnel in its laboratory

**Status:** accepted, v0.5.0 (2026-10-23) · **Amends:**
[ADR 0007](0007-a-theme-may-bring-its-own-view.md) (Fusion's living tree) and
[ADR 0013](0013-clarity-the-family-as-a-funnel.md) (Clarity's funnel) ·
**Applies to:** `static/fusion.js`, `static/clarity.js`, `static/river.js`,
`static/app.css`, `static/css/tokens.css`, `static/img/fusion-lab.jpg`,
`src/render.rs`, `src/routes/public.rs`, `src/routes/mod.rs`,
`tests/views.rs`

## Context

On 23 October 2026 the operator reviewed both views on the family's own
archive:

- **Fusion**: the tree was unreadable because every frame was the same
  trunk under the person at the centre. They asked for a navigation and a
  background that look more like a tree, simple to travel, after a
  reference picture — a glass cylinder in a dark room, a glowing trunk
  rising through it with branches spreading in depth, floating blue screens
  around — and described the movement: "if a person has several children
  from right to left, the centre continues the path toward the top centre,
  while the left or the right make the display pivot slightly left or right
  to continue the upward progression".
- **Clarity**: the funnel's indicators were unreadable, "melted in the same
  colour as the bucket", and the whole too simple; they asked for an
  animated background in the theme's mood, "something next-generation
  science", in its colours.

## Decision

### 1. Fusion: a tree in a glass cylinder

From the river's own layout — the same people, links, order and lens —
`fusion.js` stands the family in a glass cylinder seen from the front and a
little above:

- the person at the fixed point on the axis, in a column of light;
- a floor of the glass per generation: descendants above, ancestors below
  as roots (the legend's words do not change);
- on each floor, the river's order from left to right turned into an angle
  round the axis: the child in the middle continues the column straight up,
  a child to the left stands round to the left, one to the right round to
  the right; a floor of many goes round to the back of the glass;
- the front is near — larger, lit, its people named on small glass panels
  with a leader line, as the reference's floating screens; the back is far,
  dimmed, named by the pointer only;
- branches climb from floor to floor, upright at each end, with the light
  of their parentage (attested, inferred, speculative) as before.

**Travel turns the glass.** A view may now blend two frames itself
(`window.axgfView.tween`, called by river.js with the river's own blend):
every person in both frames turns round the axis — the shorter way — and
climbs or sinks to where the next frame has them, over 0.9 s instead of the
river's 0.56 s. The person travelled to comes round to the front and up
onto the axis: the glass seems to pivot toward the side they were on and
the climb goes on upward.

**The room.** The glass stands on the projector of a laboratory made in the
operator's Canva (`static/img/fusion-lab.jpg`, 1680 px, 88 KB: a dark room,
blue screens at its sides, a projector glowing red on the floor), drawn in
the SVG so that the projector is under the glass at any size, with a veil
of the page's colour over it. Under the drawing, on its own canvas: motes
turning round the glass, a scanner's arc going round each floor, embers
rising from the projector up the column; over it, the light drawn up every
attested branch.

### 2. Clarity: counts over the glass, a laboratory under it

- What the record continues past the last slice, and every count of the
  people beyond, is drawn over the glass after the mesh — counts with the
  page's ground around them, tips ringed in white — instead of under the
  slices, where it took the glass's colour.
- The cone stands on a projector: a platform under the narrow end with its
  rings, a beam up the axis to the mouth, a gauge's ticks beside each
  opening.
- Under it, on its own canvas: a floor of light lines running to a horizon
  behind the projector, rings rising from the projector, a network of data
  points drifting up the sheet in the slices' colours, a scanner's band now
  and then.

### 3. On the river's terms

Both scenes hold still as the river's ground does — reduced motion, "Cut",
the pause, a hidden tab, a stage off screen, a machine too slow — and
are then drawn once, unmoving. No colour is named in either script; Fusion
gains three tokens (`--tv-glass`, `--tv-panel`, `--tv-panel-line`). The
server's river, its SVG and its payload, are unchanged (ADR 0001).

## Consequences

- Fusion names fewer people at once than the river (the back of the glass
  is named by the pointer only), but every name it shows stands on its own
  panel, clear of the branches.
- The tween hook is open to every view; only Fusion uses it.
- Fusion's frames carry each person's place in the glass (`w`: angle,
  floor, distance from the axis) beside their place on the page.
