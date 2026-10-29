# ADR 0004 — Canopée is the reference theme, and the river's ground is a landscape

**Status:** accepted · **Applies to:** `static/css/tokens.css`,
`static/app.css`, `src/theme.rs`, `templates/home.html`,
`templates/_icons.html`, `templates/river.html`, `static/river.js`,
`static/river-scene.js`, `static/ui.js`, `static/img/`, `tools/pictures/`,
`tests/controls.rs`, `tests/masthead.rs`, `tests/themes.rs`,
`tests/design_tokens.rs`, `tests/river.rs`

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
  the sign-in filled. (Since ADR 0006 the leaf is ax·genealogy's logo and the
  lime is the logo's gold.)
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

They come to 0.86 MB in the binary. The first ones were exported small and
looked soft on a monitor (the operator's review, §10); each is now kept at
the size Canva generates it, 1680 or 1536 pixels wide, and sharpened once.
They are served by `/static/img/…` with a day of cache, so a picture that
changes takes a new `?v=` where the stylesheet names it (the second pass's
are `v=2`): otherwise a reader who saw the old one keeps it for a day. The
Content-Security-Policy already allowed same-origin images.

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
  `--rv-contour` over it, mist in two corners, and the drawing's own ground
  and even bands made translucent (structural selectors, so `src/river.rs`
  is unchanged).
- **Zones.** The ground the lines run through, drawn as a cartographer draws
  terrain: a translucent area with four crisp contour lines nested inside
  it, teal below the fixed point (towards the sources) and ochre above it
  (towards the living). They are the isolines of a field made by the lines
  themselves (marching squares over a blurred raster), so they follow every
  branch and merge where branches gather, and they say nothing the lines do
  not. The first pass drew soft blurred valleys instead; on a monitor they
  read as a smudge, not as Selva Verde's terrain (§10).
- **Banks.** A thin dark edge under every line, so the water stays sharp on
  the forest.
- **Cores and glow.** Attested water carries a thread of light down its
  middle, with a faint warm glow around it in two passes, as Selva Verde's
  path is lit. Solid and lit, dashed and dull, dotted and faint: confidence
  now also reads as brightness.
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

**Why all of it is drawn by the client.** The zones, the banks, the cores,
the glow, the lights and the scale are computed in `river.js` from the
payload the page already
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

Nothing loops except a loader while work is out — and, since the third
pass, the river's ground, which the operator asked to see alive and
which three things hold still (§11). `ui.js` marks the root
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
  generated boards disagree, these are right. They show the first pass: the
  second (§10: section discs, moving buttons, the river's zones and glow) is
  in the stylesheet and `river.js` only.

The boards are the reference for anyone changing this theme. The code is the
theme.

## 10. The second pass: the operator's review

The operator tried the first pass on a monitor, in the compact style, and
reported five things. Each one, and what changed:

- **"The pictures are blurred."** They were. They had been exported small
  and recompressed, and the river's ground was so dark under its shade that
  it read as black. They are re-made at full size (§3), and the river's
  ground is a lighter forest that the shade no longer swallows.
- **"The river does not look like the model."** The soft terrain is
  replaced by crisp zones with nested contour lines, banks under the water,
  a warm glow on attested water and mist in two corners (§5): Selva Verde's
  vocabulary, laid on a family's river rather than on one path.
- **"Every button's animation."** Every button, in every place, now
  answers the pointer under Canopée. A band of light crosses it once, it
  lifts with a soft shadow, and a press shrinks it a little and leaves an ink
  ripple where the pointer went down (`ui.js`, on `pointerdown`). The
  record's way into the river and the edit button beside a name carry their
  own signs (the river, a pen); the edit button is filled, as the kit draws
  a record's one action. `prefers-reduced-motion` stills all of it, as it
  stills everything else; "Cut" in the settings is about the river's travel
  and leaves the buttons alone.
- **"The theme must be on every page; the person page is barely touched."**
  The kit is carried onto every page, still only under
  `[data-theme="canopee"]`. Every section opens with a disc that carries
  its sign, in a colour of its own (a person for identity, two for family,
  a calendar for a life, a pin for places). The person is a card with a
  ringed portrait and dates in a chip. Every page title has a lime rule
  under it. Rows answer the pointer, and figures are tiles. Every
  disclosure turns a chevron, and the counts are pills. The editor's save
  bar glows, and the login page is a glass card over the hero's hills.
  Every other theme is Study 06 exactly, as before.
- **"The name header runs out of its card" and "the language list cannot be
  read".** The identity table bleeds to its section's edges by the
  section's own inline padding, and the compact style padded less than the
  bleed. The bleed now reads `--section-pad-x`, which each style defines
  (`tests/themes.rs` holds it to that). On Windows, an open select drew
  its options on white with the theme's pale text. Options now take the
  theme's raised surface and text, the current one its hover surface, and a
  select its sunken ground.

One more fault was found while checking: the completeness tag ("not yet
recorded") ran past its column in every theme, because the cell does not
wrap and the tag sat beside the count. It now sits under it, as its top
margin always meant.

Measured: every page in the compact style at 390 and 1280 pixels, text at
100 % and 150 %, 56 checks, no horizontal overflow; the open select's
colours read from the browser under Canopée and under the dark and light
themes; the full test suite and Clippy.

## 11. The living river (the operator's second review)

The operator looked again on a 1920 × 1080 monitor. The pictures were still
soft there and the dark backgrounds looked "pixelated"; the river's type was
too big; and they asked for the river to be alive — the water visibly
flowing, the trees moving a little in the wind, something modelled in 3D for
the web and never pixelated — and for its content to be pleasant to move
around in, at the type size of the rest of the site.

**Why it was soft.** The river's ground was a 1680-pixel photograph stretched
to a stage 1466 pixels wide and, on a tall river, 1190 high: scaled up, so
soft. And the drawing scaled with the stage (`width="100%"`): at 1280 pixels
a unit was 1.05 pixels, at 1920 it was 1.87, so a 12-unit name was 22 pixels
and the centre's 30.

**Decision: the ground is computed, not a picture.** `static/river-scene.js`
draws a forest seen from above under the drawing, in WebGL, at the screen's
own resolution (up to two device pixels per CSS pixel):

- crowns lit by a low sun from the upper right, pointed like conifers seen
  from above, with a needle bump and the shadow each casts on the next;
- hills under the forest, whose slopes the same sun lights;
- clearings and wet ground along every line, from a mask drawn from the
  river's own edges;
- mist in two corners, warm on the sun's side.

It is never scaled up, so it is never soft, and it dithers its own output,
so its darkest gradients never show steps. Its colours are tokens:
`--rv-canopy-deep`, `-mid`, `-lit`, `-warm`, `--rv-sun` and `--rv-bank`, with
the existing `--rv-mist-rgb`, `--rv-shade-rgb`, `--rv-core` and
`--rv-lit-halo`. Without WebGL, the photograph is the ground, as before.

**What moves.** Gusts cross the canopy: a band of wind travels over it,
leaning the trees and turning their leaves to the light, and each tree sways
at its own rhythm. The mist drifts. Pulses of light run down every attested
line from the sources toward the living, on a canvas over the drawing that
never covers a person or a word. Thirty frames a second at most; a machine
that cannot keep up gets fewer pixels (never fewer than the screen's own),
then fewer frames. Nothing moves while the river is off screen or the tab is
hidden.

**Who it holds still for.** `prefers-reduced-motion`, "Cut" in the settings,
and the river's own pause, "Figer le paysage" (a toggle with `aria-pressed`,
remembered in this browser), all draw the ground once and hold it. That is
WCAG 2.2.2's way to stop motion that starts by itself. Under Gravure and in
print, the ground is plain ink, as before.

**Type at the site's size.** The drawing is never shown larger than 1.15
pixels to the unit (`KMAX`). A wider stage widens the window instead: the
ground, the half-century bands, the years and the rail run to its edges, and
the drawing stays in the middle. At 1920 pixels the names are 14 pixels and
the centre's 18, as at 1280. The window depends on the range and the stage
alone, so within a range the fixed point still does not move (ADR 0001 §5).

**A map you can move in.** The camera zooms from 1 to 3:

- Ctrl (⌘) with the wheel, which is also what a trackpad pinch sends;
- two fingers;
- the "Zoom" slider;
- `+`, `−` and `0` on the keyboard.

Zoomed in, a drag moves the camera. People and words are map symbols: they
grow with the zoom only up to their own size, which is what makes a phone's
small drawing readable, and keep it after that; only the distances between
them grow. Contour lines and the lit route stay hairlines. Travel keeps the
zoom and glides the camera to the fixed point. At rest, a finger still
scrolls the page. The slider and the pause are hidden until a script shows
them, so a reader without scripts is offered nothing that does nothing.

**The server's river is unchanged.** All of this is `river.js` and
`river-scene.js`, drawn from the payload the page already holds. The SVG and
the payload are byte for byte what they were (ADR 0001), and the drawing has
no filter and no clip-path. `river.js` now replaces the drawing in place
instead of the whole stage, so the scene's two canvases keep their state
through a travel.

**The overview's steps.** The hero's large dark shadow on the dark page
showed as bands, so it is removed. A grey grain finer than a pixel (4.5 %)
over the hero's and the sign-in page's gradients breaks their 8-bit steps.
The hero's JPEG was already quality 88 with full colour resolution.

Measured, in Chromium with WebGL (software rendering):

- the river at 1920 × 1080, 1280 × 900 and 390 × 844;
- zoom 2 and 2.5, a lit route, a travel;
- reduced motion, the pause, Gravure (no scene) and the dark theme (scene);
- no script error.

Two frames 0.9 s apart differ on 7 % of the ground's pixels: the gusts and
the pulses. The software renderer is far slower than a graphics card, so
frame rates were not measured here.

## 12. Selva Verde's own ground (the operator's third review)

On 24 October 2026 the operator set the river beside Selva Verde again and
asked for its background and its mesh to look like the reference's. The
computed forest of §11 was a forest seen straight down, crowns as a
pattern; Selva Verde is a photograph of a forested mountainside seen at an
angle, at golden hour, mist over the valleys at both edges, its spine of
clearings down the middle, the paths a white thread of light through pale,
translucent zones.

**The ground is a photograph again, in relief.** `river-ground.jpg` is made
anew in the operator's Canva, from a description of the reference's
landscape (not from the reference, whose author holds it): a forested
mountainside from the air at golden hour, a spine of small clearings down
the middle, mist at the left and right edges, the sun in the upper right;
no text, buildings or paths. It is enlarged as ADR 0017's pictures are, to
3,840 × 2,160. `river-scene.js` draws it with WebGL, covering the stage and
a tenth more, in relief by its depth map (red in `river-ground-maps.jpg`, Depth
Anything V2, `tools/pictures/depth.py`): the spine and the near trees move
more than the valleys as the pointer leans the ground (only where motion is
allowed) and as it sways while the scene lives. Zoomed, it grows and follows
the camera a third as much as the drawing does, as distant ground would.
Over it the mist still drifts, thickest at both edges and in the lower
corners and warm on the sun's side, the sun's light falls from the upper
right, and the shade every ink was measured on deepens toward the edges.

This undoes §11's "never a picture stretched to fit": the photograph is
four times its Canva size and filtered by the graphics card as it shrinks,
so it is no longer scaled up on any screen up to two device pixels per CSS
pixel. The computed forest, its gusts and its banks' mask are gone; its
tokens with them (`--rv-canopy-*`, `--rv-bank`). The glints along the water
are unchanged.

**The mesh after the reference** (Canopée only, `app.css`):

- the zones are a light wash of teal under the fixed point and ochre over
  it, with fine pale contour lines (`--rv-contour-hi`) rather than lines in
  their own colour, the last one dotted;
- the attested water is a white thread (`--rv-thread`) with a warm glow
  (`--rv-thread-glow`), the dark banks barely there;
- each person is a dot in a ring of light;
- over the photograph the half-century bands are only a veil: their rules
  and years still carry the time, without striping the ground.

The server's river is unchanged (ADR 0001): all of it is the client's, and
without WebGL the photograph is the stage's background, as before.

**The same picture, animated** (the operator's next message, the same day:
"not only the style of the view, but the river, the several rivers and the
animations around it — really the same image, animated"). The ground was
made again, closer to the reference's composition: a valley running down
the middle between dark pine forest, curved terraces on the left, a golden
meadow in the lower left, turquoise streams winding down the right into a
few ponds, a rocky outcrop with a round stone platform at the bottom, mist
at the edges and in the lower corners, the sun in the upper right (again
from a description; no text, no path, no buildings, no people). Beside it,
`river-ground-maps.jpg` carries its depth (red) and its water (green: the
ponds by their turquoise, the streams by their foam, found once with
`tools/pictures/water.py`). What moves:

- the landscape's water: ripples run down the streams and the ponds, the
  photograph trembling under them, and the sun glints on them;
- the family's rivers light the forest around them, as the reference's path
  does, the light coming up them in slow waves from the sources toward the
  living (a soft mask drawn from the river's own edges, the attested ones
  brighter), and pulses of light still run along every attested line;
- each person is a marker of light, as the reference's stations are: a ring
  opens from it and fades every few seconds, the fixed point's wider;
- the sun's rays turn slowly across the ground from the upper right, and
  the mist drifts over the edges.

All of it holds still on the river's terms (reduced motion, "Cut", the
pause, a hidden tab, a river off screen, a machine too slow). The family's
lines keep the river's own geometry: the reference's single path is the
family's many rivers, branching as the family does; the server draws them
and decides where they run.

## 13. The family's lines are the rivers (the operator's fourth review)

The same day, the operator corrected the reading of their request: the
lines between the people are the rivers, and it is their appearance that
should be modelled in 3D after the reference — not only the landscape
under them.

**Each line is a river in the ground.** Under Canopée (`--rv-rivers-3d: 1`),
`river-scene.js` draws every one of the family's lines, and every line the
record continues past the window, as a river carved into the landscape. It
is a strip of triangles along the river's own curve (river.js's `vc`, so
the lit route and the glints follow the same water), drawn by the graphics
card under the drawing:

- its water, clear turquoise (`--rv-river-shallow`), darker toward its banks
  (`--rv-river-deep`), with a little of the line's own colour in it, so the
  ramp of the years still reads;
- ripples running downstream, from the sources toward the living, and the
  sun glinting on them; its wet edge lit on the side that faces the sun
  (upper right), shaded on the other, so that it lies in its bed;
- down its middle, Selva Verde's path: a white and gold ribbon of light
  (`--rv-thread`, `--rv-thread-glow`), its light running toward the living
  in slow pulses, and a warm halo on the forest round it (the ground's own
  glow, so that it never folds at a bend);
- its banks of wet earth (`--rv-river-bank`);
- its width the line's flow (the legend's five widths), the thinner rivers
  drawn first so that a tributary runs into the river it joins, its two
  ends round.

How sure a parentage is still shows: an attested one is a river with its
ribbon of light; an inferred one a stream that runs underground and out
again, its light fainter; a speculative one a dry trail of small pools. A
line the record continues past the window runs on and fades, its arrow
and count kept; a line it ends dries up before its last mark.

**The drawing's own lines step aside** while the scene draws the rivers,
on screen only (the stage's `rv-rivers` class). They come back for a
travel, which the scene does not follow frame by frame, and the rivers
return at rest; in print; and wherever the scene cannot draw (no WebGL).
The server's SVG and payload are unchanged (ADR 0001).

Measured in Chromium with software rendering, on the fictional family at
±2:

- at rest, frames at the screen's rate (median 16.7 ms);
- through a travel, the same median;
- no script error.

## 14. Streams and rivers, each its own (the operator's fifth review)

On 25 October 2026 the operator described the watercourse they meant: the
family's path itself. A parent's line to their family is a small stream
running down steep ground; where the two parents' streams meet to give a
child, the line to the child becomes a river; and so on, generation after
generation, no two streams and no two rivers alike, so that the whole reads
as natural, beautiful and original.

**Which water each line is.** river.js tells the scene, for each line,
whether it is a parent's line to a family (from a parent) or a family's
line to a child, and how many of the family's parents are drawn. A
family's line to a child with both parents drawn is a river; every other
line, and every line the record continues past the window, a stream. A
river is broader (1.25 times the flow's width and more), a stream narrower
(0.55 times); within each, the width still follows the flow, as the legend
says. Streams are drawn first, so that they run into the rivers.

**Its own course.** Each line keeps its two ends and leaves and reaches
them as the river's curve does, but between them it winds (`course`): a
stream in tight, uneven bends, with a little noise in them, as water finds
its way down a slope; a river in broad, calm ones. The bends come from the
line's own ends, so the same family is drawn the same way on every visit,
and no two lines alike. A bend is never tighter than the water and its
banks can follow, so the strip never folds. Its width breathes along its
course.

**How each looks.** A stream runs fast, light and white: its ripples small
and quick, foam on it, and here and there a small fall where the slope
steepens, its banks stony, its ribbon of light thin. A river runs slow and
deep: broad ripples, the sun's sheen on it, Selva Verde's ribbon of light
full down its middle. How sure a parentage is still shows as in §13.

**The lit route follows the course.** While the rivers are drawn, the
scene gives river.js each line's course (`axgfRiverScene.edge`), and the
route the pointer lights runs along it; in a travel, in print and without
WebGL, along the river's curve as before. The glints and the forest's glow
follow the courses too.

## 15. Water that is water (the operator's sixth review)

The same day the operator asked for the rivers to be taken out of the
background, and for the family's streams and rivers to look like rivers —
not only to move like them; everything should rest on the real thing.

**The ground holds no water.** `river-ground.jpg` is made once more, the
same valley (terraces, golden meadow, rocky outcrop, forest, mist) but dry:
no streams, no ponds (again in Canva, from a description). Its map keeps
its depth only; the water mask and `tools/pictures/water.py` are gone. The
family's streams and rivers are its only water, and along them the
vegetation is darker and greener (`--rv-riparian`), as it grows along
water.

**Their water is photographed.** Two pictures made in Canva, seen straight
from above: a clear turquoise mountain stream over pebbles and mossy stones,
with foam (`river-water-stream.jpg`), and a calm, deep green-turquoise
river with its sandy, grassy banks (`river-water-river.jpg`). Each is made
to repeat down its length without a seam (`tools/pictures/tileable.py`:
its end crossfaded into its beginning), at 512 × 1024. The scene lays them
along each course: across the course, the picture's width, its banks at
the course's edges; along it, the picture repeating. So that no two are
alike, each course takes its own stretch of the picture, is mirrored or
not, and has its water a little deeper or lighter.

**What moves.** A stream's water trembles over its stones and runs fast,
foam runs down it, and here and there it falls a step. A river's water
slides slowly downstream while its banks stay where they are, with the
sun's sheen on it. The pulses of light along the lines (§11) are not drawn
where the water is; the rings of light round the people stay.

**Their edges are the ground's.** The water's edge breathes along the
course and frays, never a ruled line, and a shadow of wet earth lies just
outside it. The stylised ribbon of light and the drawn banks of §13 are
gone: the photographs carry the light and the banks.

How sure a parentage is still shows: an inferred one runs underground and
out again, its bed dry where it does; a speculative one is a trail of
pools. The lines toward brothers and sisters not drawn are short streams
that fade.

## 16. The ground follows the water

The operator, the same day: the background must follow the rivers drawn
from the relations, or it looks wrong. A photograph cannot know where a
family's streams will run, so the ground is shaped around them as it is
drawn:

- **Valleys.** The courses, drawn wide and soft into a third field of the
  scene's mask (blue), are a height field: each stream and river runs in a
  valley it has carved. The ground's shader lights that field as the hills
  were lit, with the same sun in the upper right: the slopes facing the sun
  brighter, the others in shade, the floor lower and darker. A river's
  valley is wider than a stream's; a valley is carved however sure the
  parentage is.
- **Wetter ground.** Along the water the vegetation is darker and greener
  (§15).
- **Never apart.** The relief of §12 slides the photograph as the pointer
  leans it. Where the family's water runs, the ground now holds still, so
  that the valleys and the water stay together; the further from the
  water, the more the landscape moves.

All of it is computed in the drawing's own units, so it follows the water
at every zoom and after every travel.

## 17. Seen at 60°, and alive (the operator's seventh review)

On 27 October 2026 the operator found that the rivers were seen from
straight above while the landscape under them was seen at an angle, about
60° from straight down, and asked for the rivers to be seen at the
landscape's angle. Asked how far to go, they chose to tilt everything. They
also asked for falls and other kinds of water where the streams meet, for
water that looks natural — "not framed by straight or parallel lines for
banks", with the colours of foam and of different waters, after an aerial
photograph of a river they showed as a reference (a reference only, not a
picture used) — for wind through the trees, for small herds of animals
moving below, "a whole natural ecosystem, diverse and varied, so as to
forget the static notion of the path", and for rocky and sandy ground.

**The camera.** The people stay where the drawing puts them — the layout,
the names, the bands and every invariant of ADR 0001 are unchanged — and
under them the ground is tilted as the photograph's is: 60° from straight
down, its horizon 0.8 of the drawing's height above its first row. A point
of the screen is a point of that ground; each line's curve is laid back on
the ground, bent there into its stream or river (§14), and seen again
through the camera, on the graphics card, perspective and all
(`river-scene.js`, `toGround`, `toScreen`). A river is wider in the
foreground than in the distance, a reach across the view flatter than one
running away from it, and its water, its stones and its foam are
foreshortened with it; the sky shows more in the water toward the
distance. The valleys the water carved (§16) are carved in the same tilted
ground; the rings of light round the people lie on it as ellipses; the
lit route follows the courses as they are seen. The zones' contour lines,
a map's seen from straight above, step aside under the rivers with the
drawing's own lines; they return for a travel, in print, and without WebGL.

**Water that is water.** Each bank of each course is its own: it wanders,
opens into pools where the water slows, closes in where it races. On the
inside of a bend, and here and there along a bank, the water leaves a bar
of gravel and pale sand, wet and darker near it. Stones lie in the bed and
boulders break the surface, each with its pillow of white water and its
wake; the bed is seen through the shallows at the edge; the water is
deeper and greener in the pools, turquoise where it runs open, and white
where it races. Past the bank there is only the uneven shadow of the bed.
Where a family's streams meet, the river they make starts in one of three
ways, the same at every visit: under a fall, its water churning white and
its spray rising and drifting with the wind; in a still, deep pool; or down
rapids. The stream nearing a fall hurries and breaks; down a stream, steps
of small falls, each with its pool.

**A living valley.** The ground is made anew in Canva, the same mood seen
from the same angle: forest, meadows, larches turning gold, granite
outcrops, a bed of pale sand, mist in the distance, the low sun behind the
valley on the left (`river-ground.jpg`, 3,840 × 2,160, enlarged as ADR 0017
says). Its maps carry its depth and, new, its trees and its meadows
(`tools/pictures/masks.py`). On them:

- the wind comes from the left with the clouds: gusts run through the
  trees, which bend and show the paler side of their leaves, and over the
  meadows in waves;
- clouds cross the valley with their shadows under them, over the forest,
  the family's water and the herds;
- herds keep to their own ground: red deer and wild horses on the meadows,
  chamois and ibex on the rocks and the grass round them, grazing, raising
  their heads, walking on a few steps, each herd drifting across its ground
  over the minutes, each animal with its shade away from the sun, smaller
  and hazier toward the distance; an eagle circles over the far valley,
  its shade far below it; now and then a flock of small birds crosses with
  the wind; here and there a fish rises on a river. The animals were made
  in Canva, seen from above at the landscape's angle, cut out, and packed
  in one atlas (`river-life.png`, `tools/pictures/sprites.py`); the birds
  are drawn.

Nothing of it says anything about the family: the herds are not people,
the falls are where parents' lines meet, as the lines say. All of it holds
still when stillness is asked for, as §11 says, and is drawn once,
unmoving; the clock that moves the herds runs only while the scene moves.

Measured in Chromium with software rendering (no graphics card) at
1,016 × 594 pixels, on the fictional family at ±2: a frame takes about
180 ms, against about 110 ms before, so that such a machine falls back to
a still ground sooner (§11). Most of it is the ground and the water; the
herds take about 2 ms. It was not measured on a graphics card here.

## 18. The water bound into its valley (the operator's eighth review)

The same day the operator found the rest right, and one thing wrong: the
water still lay on the picture rather than in it. It ran through the
trees and over the rocky hills, "a plain sheet of tracing paper with no
relation to the scenery round it". They asked for the river and the
background to be one.

The courses still go where the family's lines go; it is the valley that
makes room for them and the water that belongs to the valley:

- **The water clears its way.** Along every stream and river the ground's
  mask holds a clearing (its near field, wider than the water). Where the
  photograph shows a tree whose foot stands in the clearing, the tree is
  gone: the ground there is grass in the photograph's own meadows' tone,
  moss, and gravel toward the water, in the shade of the trees beyond. A
  tree's foot is under its crown, a third of the tree's height lower on
  the photograph and to either side of it, the tree's height following
  the photograph's distance.
- **The trees of the near bank lean over it.** Where the photograph shows
  a crown whose foot stands outside the clearing, the water there is
  transparent and the crown, drawn by the ground, hides the water's edge.
  The middle of the water always shows, so that the family's line is never
  lost. The ground and the water use the same test, so that the crown kept
  by the one is the crown that hides the other.
- **It takes the colours and the light round it.** The water picks up the
  photograph's own tones around it, blurred, and its light: darker and
  greener under the forest, golden through the meadows. It always holds a
  little of the sky down its middle. The trees beyond it, toward the low
  sun, shade it. Over bare rock it runs white, as a torrent in its gorge,
  and its bars are of that rock. Far off it fades into the haze, as the
  photograph's distance does.
- **The valley's shading is softer** and the water's band of darker,
  greener growth narrower, so that a clearing reads as grass in the light
  and not as a stain.

Measured as in §17: about 150 ms a frame with software rendering, the
costly tests made only where a crown, a clearing or rock can be.
