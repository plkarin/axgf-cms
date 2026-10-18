# ADR 0004 — Canopée is the reference theme, and the river's ground is a landscape

**Status:** accepted · **Applies to:** `static/css/tokens.css`,
`static/app.css`, `src/theme.rs`, `templates/home.html`,
`templates/_icons.html`, `templates/river.html`, `static/river.js`,
`static/ui.js`, `static/img/`, `tests/controls.rs`, `tests/masthead.rs`,
`tests/themes.rs`, `tests/design_tokens.rs`

On 18 October 2026 the operator asked for a reference theme built on three
references, cut apart and reworked in Canva:

- **Zenze Agri Solutions**, a dark forest-green landing page: lime pill
  buttons with an arrow, a photograph behind the headline, a strip of figures,
  a glass card of live bars, cards with coloured icon discs;
- **Selva Verde**, an architect's site plan: a glowing path winding through a
  forest seen from above, translucent terrain zones with contour lines,
  numbered stations with leader lines, a compass and a scale bar;
- an **animated loader set** (Pinterest): rings, dots, ripples, an hourglass.

He asked for the river to be reworked after Selva Verde as well. This records
what was built, where it departs from ADR 0002 and Study 06, and why.

---

## 1. Canopée is the default, and every other theme stays

**Decision.** `theme::DEFAULT` is `canopee`. Canopée is listed first in the
selector. Nocturne (Study 06), Craie, Gravure and the others are unchanged and
still on offer; a reader who chose one keeps it. The new default reaches only
readers who never chose.

**Supersedes** ADR 0002 §1 on which theme is the default. The rest of ADR 0002
stands: the river keeps its own palette under every theme, and every theme is
still measured by `every_theme_reads` with no exception.

A page with no `data-theme` at all still falls back to Study 06 in the
stylesheet. The server always writes the attribute, so this is the fallback
of a fallback, and moving it would have touched the one `prefers-color-scheme`
rule for nothing.

## 2. A theme that carries its own furniture

**Decision.** Every other theme is a palette and nothing else. Canopée also
carries a shape and a tempo:

- pill buttons, softened fields (`--btn-radius`, `--field-radius`) and frames
  (`--radius`, `--radius-sm`, `--radius-lg`);
- a quicker ease (`--tempo`, `--ease`);
- a lime glow under the primary action and under the focus ring;
- a leaf beside the wordmark, the section you are in written in lime, and
  the sign-in filled;
- segmented-pill record tabs, notices whose shape also carries its sign (a
  ring with *i*, a disc with ✓, a diamond with !, a square with ×);
- cards with a sheen and a deeper shadow;
- coloured icon discs on the overview page.

All of it is written under `[data-theme="canopee"]`. Choosing any other theme
gives Study 06's furniture back exactly.

**What does not change, here or anywhere:**

- the four button variants, 36 px or 28 px tall;
- the focus ring, 2 px at 2 px on every control: Canopée adds a glow under
  it, never in place of it;
- labels above fields;
- the notice shapes;
- every chrome budget;
- reading and editing as two modes.

**What the tests now say.**

- `tests/controls.rs`: "radius 2 nowhere else" became "one radius per
  theme". Every button rule takes `var(--btn-radius)`. `--btn-radius` is
  defined on `:root` (2 px, Study 06) and in Canopée's block (a pill), and
  nowhere else.
- `tests/masthead.rs`: "nothing in the bar is coloured" now reads the
  stylesheet rule by rule. It used to read it in chunks split at `\n.`,
  which would have let a rule beginning with anything other than a class
  through unread. The accent may reach the masthead only under Canopée's own
  selector.
- `tests/themes.rs`: Canopée joins the theme list. `--btn-radius`,
  `--field-radius` and `--type-hero` join the names that are not colours.

## 3. Pictures: made, not copied

The references' pixels belong to their authors, and this repository is
public. Nothing was taken from them. What the site carries was made for it:

| file | made by | drawn |
|---|---|---|
| `static/img/canopee-hero.jpg` | Canva, generated (terraced hills at golden hour) | behind the overview's hero, Canopée only |
| `static/img/canopee-cta.jpg` | Canva, generated (an elder's and a child's hands on a family album) | beside the overview's last band, Canopée only |
| `static/img/river-ground.jpg` | Canva, generated (a dark forest from above, in mist) | the river's ground, every theme but Gravure |
| `static/img/river-contours.svg` | a script: isolines of a fixed-seed relief, used as a mask | over the river's ground |

They come to 0.3 MB in the binary. They are served by
`/static/img/…` with a day of cache, and the Content-Security-Policy already
allowed same-origin images.

**Text never sits on a bare photograph.** A shade made from the theme's own
ground lies between the two, so text keeps the contrast its palette was
measured for. No picture is ever drawn on a control, a field or a button.
Under Gravure (high contrast) and in print, the river's ground goes back to
plain ink.

## 4. Loaders beside the words, never in place of them

Study 06 (B2) says a busy button changes its label and shows no spinner. The
label change stays, everywhere. Under Canopée, a loader from the reference
set moves beside the words, hidden from assistive technology:

- three dots by default;
- the river on the record editor's save, which rewrites the whole archive
  (12 to 17 s);
- the hourglass on an import;
- ripples on the person a reader is travelling to in the river, while that
  river is fetched (every theme: it belongs to the river, like the travel it
  announces).

The other themes keep the words alone. `prefers-reduced-motion` stills every
loader, like everything else.

## 5. The river's ground is a landscape (after Selva Verde)

**Decision.** The river keeps its geometry, its widths, its dash patterns,
its bands and every one of ADR 0001's invariants. What changes is the ground
it runs on and the light on the water:

- **Ground.** The forest photograph under a shade, contour lines in
  `--rv-contour` over it, and the drawing's own ground and even bands made
  translucent (structural selectors, so `src/river.rs` is unchanged).
- **Terrain.** Every line is a valley two soft bands wide, in its own water's
  colour, so the old generations sit in teal ground and the recent ones in
  ochre, and the terrain deepens where lines gather.
- **Cores.** Attested water carries a thread of light down its middle. Solid
  and lit, dashed and dull, dotted and faint: confidence now also reads as
  brightness.
- **The lit route.** A halo, a glow, the light, a warm core, and a thread of
  light running from the sources toward the living, three times, then still.
  Every person on the route is ringed as a station of the path.
- **The legend.** It reads like a cartographer's: a title in spaced capitals,
  a compass for the direction of time, and the scale of a generation (29
  years). The scale is shown only for a river with an era: ADR 0001 §3 says a
  river without a visible anchor has no time of any kind in the page.
- **The itinerary.** Each person visited is a numbered stop on one line, 01
  to 07, as Selva Verde numbers its stations.
- **Motion.** An entrance once per page (the water and terrain come up, then
  the people), and the fixed point's ring breathing twice on arrival.

**Why all of it is drawn by the client.** The terrain, the cores, the lights
and the scale are computed in `river.js` from the payload the page already
holds: the same edges, at the same places, in the same water colours. The
server's SVG and payload are byte for byte what they were. ADR 0001's
invariant, that "two archives that differ only by hidden people's dates
render byte-identically", therefore holds of everything the server sends,
untouched, and nothing drawn in the browser carries information the page did
not already have. A page without scripts gets the ground and the river as
before, without the light.

**The rendering contract holds.** Plain strokes only: no filter, no
clip-path. `gradientUnits="userSpaceOnUse"` is unchanged. Tap targets are
unchanged. "Cut" in the settings and `prefers-reduced-motion` stop every
light that moves.

## 6. Motion that answers the reader

Under Canopée, these move once and stop:

- a section rises into place the first time it scrolls into view;
- a figure counts up to its number the first time it is seen;
- the primary's arrow nudges under the pointer;
- the scroll cue's wheel turns three times.

Nothing loops except a loader while work is out. `ui.js` marks the root
`js-motion` only when `prefers-reduced-motion` allows it. A page without
scripts, or a reader who asked for stillness, therefore has every section in
place and every number whole from the first paint.

## 7. The overview page

The overview stays the family's page, not the product's: `tests/routes.rs`
still keeps the pitches out. It has:

- **the hero**: the family's name in a chip, a two-line headline (*Every life
  is a source. Every family, a river.*), what the family has recorded, and
  two ways in, the river and the tree;
- **a composition card**: each kind of record as a bar, as long as its share
  of the largest;
- **the strip of figures**;
- **the showcase cards**, each with its coloured disc;
- **a last band**: the only edit control, *Add a person*, shown to editors
  only.

Chrome: 2 controls of the page's own for a reader, 3 for an editor, under
the read-only cap of 5.

## 8. What was measured

- `every_theme_reads` passes for Canopée, washes and contour texture
  included. The tightest pair is `--border-strong` on `--wash-1` under the
  texture, at 3.25:1 against 3:1.
- The icon inks on their discs measure 5.75 to 6.87:1.
- Tree-card outlines are at 3:1 or more on the page, and the crossing hues
  at 3:1 or more composited at 75 %, as `tokens.css` promises of every
  theme.

## 9. Where the Canva work is

It is in the operator's Canva, folder **AXGF · Thème Canopée**:

- the two references, each separated into editable layers;
- **Kit UI Canopée**: seven boards generated from the brief (palette and
  type, buttons and their states, fields, cards and figures, notices and
  tabs, loaders, the river's legend), plus the three pages the photographs
  were exported from;
- **Page d'accueil web ax·genealogy**: the overview reworked close to the
  Zenze reference;
- **Canopée — implémentation 1 to 5**: the theme as built, rendered from
  this stylesheet and separated into editable objects (palette, buttons and
  tabs, fields, cards and figures, notices and loaders). Where these and the
  generated boards disagree, these are right.

The boards are the reference for anyone changing this theme. The code is the
theme.
