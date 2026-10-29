# ADR 0007 — A theme may bring its own view

**Status:** accepted, v0.5.0 (2026-10-19) · **Applies to:** `static/fusion.js`,
`static/galaxy.js`, `static/river.js`, `templates/river.html`,
`templates/home.html`, `templates/person.html`, `templates/base.html`,
`templates/merged.html`, `templates/admin_setup_result.html`,
`static/css/tokens.css`, `static/app.css`, `static/ui.js`, `src/theme.rs`,
`src/render.rs`, `src/routes/public.rs`, `src/routes/mod.rs`,
`locales/*.ftl`, `tests/views.rs`, `tests/themes.rs`,
`tests/design_tokens.rs`, `tests/masthead.rs`, `tests/controls.rs`,
`tests/brand.rs`

## Context

On 19 October 2026, before 0.5.0 shipped, the operator asked for two more
themes, each from a pair of reference images:

- **Fusion.** A dashboard at night: charcoal tiles, neon red and orange,
  gauges and bar charts. Beside it, a tree whose crown is a dome of boughs
  with red veins running along them, green and red lights at their tips, and
  roots that sink into a histogram. "Instead of the river, a tree — living,
  showing from root to leaf the whole path of the ancestors and the family.
  Animated, explorable, and with a view of the whole as well."
- **Galaxy.** Deep space: navy, violet, cyan, magenta; planets on orbits,
  joined by links. "A journey through a solar system, with its
  dependencies and its links."

They looked at the first tree and asked for two things more. The tree should
look like the reference's: it was a family chart in rows with thick ribbons.
And no page should speak of a river where a tree or a sky is drawn.

Until then:

- a theme was a palette and, for Canopée, its furniture (ADR 0004);
- the landing page was always the river (ADR 0001).

## Decision

### 1. A view is another drawing of the river's layout, not a second layout

**The server is unchanged.** `/` renders the river's SVG, the page as it is
without scripts, and `/river/data` sends the river's payload. Neither
depends on the theme, and `render_svg` is byte for byte what it was.
ADR 0001's invariants stand.

**The theme decides the drawing.** `theme::view_of` returns `river`, `tree`
(Fusion) or `galaxy` (Galaxy). The page:

- says which in `data-view`;
- loads the view's script before `river.js`, so the view has registered
  when `river.js` looks for it;
- does not load the river's living ground (`river-scene.js`).

**A view registers `window.axgfView`.** It provides:

- **`shape(frame)`**: the same people, unions and lines at new places. The
  keys do not change, so travel tweens between two shaped frames exactly as
  the river tweens between two of its own.
- **`edge(x1, y1, x2, y2)`**: how a line bends. The lit route under the
  pointer uses it, so the light runs along the bough or the filament that is
  drawn.
- **`paint(frame, api, atRest)`**: the SVG, at rest and in travel.
- **`rest(svg, frame, api)`**: the dashboard, at rest.
- **`named()`**: the people the drawing named. The pointer names the ones it
  left out, and only those.

**Everything else stays `river.js`'s:** travel, the trail, the panel, the
keyboard, the camera (zoom 1–3 and drag), touch, the range and "Cut".

**What a reader may see is decided before the drawing**, by the lens, as
for the river. A view draws only what the payload holds.

### 2. Fusion: a living tree

**Where everyone stands.**

- **The fixed point** stands on the soil line, at the foot of the trunk.
- **The trunk** rises to a fork: the union of the central couple. The river
  places a union 38 % of the way from the parents' row to the children's.
- **The crown is the descendants.** It is a dome of rings around the fork,
  one ring per generation. The first ring is at 2/(n+1) of the crown's
  reach and the last at its rim. The dome runs a little past level on each
  side, as boughs droop.
- **The roots are the ancestors.** They are a flat fan of rings around the
  foot of the taproot, which is the parents' union, 62 % of the way.
- **Order.** Each side keeps the river's order from left to right, over its
  own breadth rather than the window's. A crown bough crosses another no
  more often than the river's streams cross.
- **Partners** stand on the soil line beside the trunk, and their grafts
  join it.
- **The soil line** sits where each side gets room for the generations it
  has: between 40 % and 80 % of the height.

**How a bough is drawn.**

- **Its shape.** A bough leaves its inner end the way the tree grows there:
  up the trunk, outward from the fork through the crown, down the taproot,
  outward from its foot through the roots. It reaches its outer end the way
  the tree grows there too. Crown boughs rise and then droop; roots dive and
  then spread.
- **Its thickness.** Bark is 22 units wide at the trunk and loses about a
  third with each generation, never thinner than a hairline. Roots are
  finer, and a partner's graft finer still. The trunk is thicker, and a
  heavier line of descent makes a heavier bough.
- **Its veins** carry what the parentage is worth. An attested link is a
  vein of light, an inferred one a broken vein, and a speculative one a
  dotted trace on faded bark.
- **Leaves.** The living are green, the dead an ember, and a person this
  reader may not see an outline only. Foliage sits around each tip and
  along the crown's outer boughs. It is seeded by who it belongs to, so it
  is the same in every frame.
- **Growth rings.** Faint dotted arcs mark each generation. Their numbers
  are written where the names leave room.

**Names.** A name stands outward, the way the tree grows where it is. The
nearest generations are named first. A name that would cover another is
left out, and the pointer brings it back. A count of people beyond the
frame is never left out; it steps aside instead.

**Life.** Sap is drawn up from the root tips, through the trunk, to the
leaves, along every attested bough. The leaves breathe and spores rise. It
holds still exactly as the river's ground does: reduced motion, "Cut", the
pause, a hidden tab, a tree off screen, a machine too slow for it.

### 3. Galaxy: a solar system

- **The sun** is the person at the fixed point, and every generation is an
  orbit.
- **The two halves of the sky.** Descendants are in the upper half and
  ancestors in the lower. Each half takes the river's order from left to
  right over its own breadth, so the planets use the whole sky.
- **Partners** are on the innermost orbit, beside the sun.
- **Links** bend a little toward the sun and carry the parentage's worth,
  as the river's lines do.
- **Names** are placed outward, nearest orbits first, and left out rather
  than written over another. Counts step aside. Each orbit's number goes
  where the names leave room: right, else left, else further from the axis.
- **Life.** Stars twinkle, the sun breathes, a comet runs on each orbit and
  pulses travel along the links. It holds still as the tree does.

### 4. The dashboard and the whole family

**The dashboard.** Under a view, a strip between the controls and the
drawing gives:

- the ancestors, the descendants, the generations and the living;
- the share of attested links, as a gauge;
- people per generation, as bars.

It is read from the drawing on screen, through the reader's lens. It never
covers a person.

**"Whole family"** travels to the widest range, 5, and back to the range the
reader had.

### 5. No river where a tree or a sky is drawn

**Every visible word that names the river has a variant for the tree and
one for the galaxy**, in all eleven languages. Each is chosen by `view`, which
every template receives through the chrome:

- the overview's two lines ("Every life is a leaf. Every family, a tree." /
  "Every life is a star. Every family, a galaxy.") and its last band;
- the call to action and the navigation;
- the legend's title and its compass;
- the pause;
- the record's "See in…" link;
- what Enter does in the masthead's search;
- the error page's way back;
- the setup's last button.

**The legend's keys to width and height belong to the river.** In the river,
width is discharge and height is time. A view draws neither, so it shows
neither key.

**Three things still name the river, and are right to:**

- the server's SVG, which is the page without scripts, and that page is a
  river;
- class names;
- storage keys. The pause is remembered under the river's key, so it holds
  across the three drawings.

### 6. The kit

Fusion and Galaxy carry Canopée's furniture through one selector,
`:is([data-theme="canopee"], [data-theme="fusion"], [data-theme="galaxy"])`.
Each theme then has its own:

- round: 8 px for Fusion, a pill for Galaxy;
- tempo;
- logo colours;
- two photographs, the overview's hero and its last band, made in Canva
  from the references.

**Contrast** is measured by the same rules as every theme
(`every_theme_reads`), under each theme's own texture at its strongest
pixel. The tightest pair in both is a control's edge (`--border-strong`) on
the third wash (`--wash-3`): 3.6:1 for Fusion and 4.0:1 for Galaxy.

### 7. Galaxy, amended (the operator's review, 2026-10-27)

The operator asked for the flows up and down to be told apart, in violet
and white, for a mesh with more style, a background made in Canva, more
design for the stars, and a gravitational field round each star that
grows with its descendants.

- **Two sides of the sun.** A filament on the upper sky, toward the
  descendants, is violet (`--gx-up`). One on the lower sky, toward the
  ancestors, is white (`--gx-down`). Each has a halo of its colour. An
  attested one also carries a fine thread of light. A couple's meeting
  point is a small four-pointed spark of its side's colour. The light
  running along the attested filaments takes their colour. The legend and
  the dashboard's bars show the two colours.
- **A field round every star.** A disc of faint light with a dotted edge,
  wider as more descendants come from the star. Its radius is 9 + 4·√n
  units for n descendants (the river's own count, `d`, less the person),
  never more than 0.56 of an orbit's step. A second ring shows inside it
  from six descendants on. Ancestors, from whom the whole visible family
  comes, hold the widest fields.
- **Stars.** A living person is a lit sphere with a glow and four fine
  rays. A person who is gone is a cooler sphere with a tilted ring. The
  sun has a corona, eight rays, a lit disc and its heart.
- **The sky** is a spiral galaxy made in Canva, its core behind the sun
  (`static/img/galaxy-sky.jpg`, enlarged to 3,840 × 2,160 as ADR 0017
  says). It is a little darkened, and a dark well round the sun keeps the
  drawing readable over the galaxy's bright core. The orbits are drawn
  lighter over it. The stars still twinkle over the picture.

## Consequences

- **A view needs scripts.** Without them the page is the river. It waits
  unseen for 1.6 s for a view that may never come, then shows; with reduced
  motion it shows at once.
- **The river stays the reference drawing.** Its layout carries the
  generation and the order that a view needs. A layout of its own for each
  view would mean two more layouts to keep correct for privacy, for
  pedigree collapse and for a person drawn again beside a later partner.
- **Adding a view** takes four things:
  - an id in `view_of`;
  - a script that registers `window.axgfView` and holds still on the
    river's terms;
  - its words in eleven languages;
  - its tokens, in its theme's block.
- **Tests.**
  - `tests/views.rs`: each theme loads its own view and no other; no river
    is named where a tree or a sky is drawn; the views' scripts and
    pictures are served and name no colour; each view gives `river.js`
    everything it asks for.
  - `tests/themes.rs` and `tests/design_tokens.rs`: thirteen palettes
    measured.
  - `tests/masthead.rs`, `tests/controls.rs` and `tests/brand.rs`: the kit
    is the three themes'.

## Alternatives considered

- **Hiding the river's words with CSS.** Rejected: a screen reader, a search
  engine and a printed page would still read them.
- **The tree in rows, as the river draws its generations.** This was the
  first attempt. It read as a family chart with thick ribbons, not as a
  tree, and the operator asked for the reference's silhouette.
