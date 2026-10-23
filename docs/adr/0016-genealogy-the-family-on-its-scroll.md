# ADR 0016 — Genealogy: dark green and cream, the family on a tree the reader climbs

**Status:** accepted, v0.5.0 (2026-10-23) ·
**Builds on:** [ADR 0007](0007-a-theme-may-bring-its-own-view.md),
[ADR 0013](0013-clarity-the-family-as-a-funnel.md) (a light palette),
[ADR 0015](0015-the-tree-in-its-glass-and-the-funnels-laboratory.md) (the
tree in tiers) · **Applies to:** `static/genealogy.js`,
`static/css/tokens.css`, `static/app.css`, `static/ui.js`,
`static/river.js`, `static/img/genealogy-*.jpg`,
`static/img/notes-genealogy.jpg`, `src/theme.rs`, `src/render.rs`,
`src/routes/mod.rs`, `src/routes/public.rs`, `templates/river.html`,
`locales/*.ftl`, `tests/views.rs`, `tests/themes.rs`,
`tests/design_tokens.rs`, `tests/controls.rs`, `tests/masthead.rs`,
`tests/brand.rs`

## Context

On 23 October 2026 the operator asked for a sixth theme, "Genealogy", and
gave its frame pack as two screens:

1. a tree growing out of an old parchment scroll, sepia portraits on the
   parchment joined to it by thin gold lines — "exactly the navigation
   mesh", the colours and the navigation to keep, the building in the trunk
   to take out;
2. a dark green interface: cream cards with rounded corners, oval portraits,
   a table in a cream card, titles in a book face, antique floral vines
   climbing both edges of the page.

The first screen carries a third party's watermark, and the repository is
public, so it was not edited: new pictures were made in Canva in its
spirit.

The first version followed the first screen literally: the family laid out
on a picture of a tree growing from a scroll, joined by gold lines. The
operator's review, the same day, on their own family: the lines were the
colour of the picture under them and could not be read, the picture was
ugly, and what they wanted was "a tree in 3D that one climbs as one moves
through it, or goes down". The vines were pixelated on their screen.

The second version drew that tree: a trunk through the whole window, the
person at the fixed point on it, a level per generation round it and thin
branches out to each person. The operator's next review, the same evening:
it "does not look like a tree at all, more like an ugly stem". A third,
drawn as a tree made of the family's own wood (a trunk, limbs as thick as
what they bore, a dome of leaves, roots under the ground), looked "very
cartoon". The operator asked for the reference's tree itself, through
Canva, as "a serious 3D tree".

The reference's tree belongs to its author, and the operator holds no
licence to it. The operator chose (2026-10-23) to follow it faithfully
without copying it: a new picture made in Canva from a description of
it, never from the picture itself. And, between a picture in relief and a
true 3D model (Blender, WebGL with three.js), the picture in relief: no
new dependency, and hours rather than days.

## Decision

### 1. The view: the reference's tree, in relief, which the reader climbs

`static/genealogy.js` registers `window.axgfView` (name `scroll`, kept from
the first version) under ADR 0007's contract, from the river's own layout.

- **The tree is a picture**, made in the operator's Canva (§3): an old tree
  with a twisted trunk and a crown of gold leaves, growing out of a
  parchment scroll whose roots spill over its lower roll, in sepia, cream
  and gold. It stands on the stage's own green, its wall cut away, as an
  exhibit lit in the dark: the page's green beyond its edges is the
  picture's own.
- **In relief.** A depth map of the picture (Depth Anything V2, §3) tells,
  for each of its pixels, how near it is. The picture is drawn by the
  graphics card, each pixel looked up where the camera's shift, scaled by
  how near that pixel is, takes it: the crown and the roots stand off the
  parchment, and the parchment off the stage. The pointer leans it a little;
  a slow sway keeps it alive while the scene moves; without WebGL it is
  drawn flat.
- **The family is set on it**: the person at the fixed point on the trunk,
  the descendants up in the crown, the ancestors down among the roots. A
  level's height is a share of the picture that closes in as generations
  go further (`lift`), so that any number of them stays on the tree; each
  level's people spread round the trunk in the river's order, the nearest
  at the front and larger, by the same tidy hierarchy as Fusion's tiers
  (ADR 0015). Each family's knot sits on the wood of the person it grows
  from (over them, the partner's line joining it there); below, where the
  line parts toward the two parents.
- **As on the reference**, each person is a sepia print in a cream mount
  with a fine gold edge (their initials, since the payload carries no
  picture, §4) beside a cream card that names them, and a parentage is a
  fine square-cornered line with a knot of gold where a family's lines
  meet. Its ink is dark, with the parchment's light under it, so that it
  reads on the leaves as on the paper (the first version's gold lines were
  the colour of the picture under them): 4.5:1 or more on the parchment,
  3:1 or more on the leaves. An inferred parentage is broken, a speculative
  one a line of knots. The cards hold their words at 7:1 or more.
- **Travelling, the reader climbs**: toward a descendant the camera rises
  into the crown, the picture lifting in relief, and the person travelled
  to comes down to the trunk; toward an ancestor it goes down to the roots.
  The blend is done in the tree's own space — each person's place round
  the trunk and height on it (`window.axgfView.tween`).

Its living scene: gold leaves falling from the crown across the parchment,
a little gold dust in its light, and a glint run along every attested
line. It holds still on the river's terms: reduced motion, "Cut", the
pause, a hidden tab, a tree off screen, a machine too slow for it. Still,
the picture keeps its relief; the pointer leans it only where motion is
allowed.

### 2. Two palettes: a dark page, cream cards

The page is the second screen's dark green, with cream text and a gold
accent, and measured as every theme is (`every_theme_reads`; tightest
pair, a control's edge on the hover surface, 4.3:1). Its cards are cream
with a sepia ink. One set of colour tokens cannot read on both, so the
theme carries a second, complete palette — every colour token's
`--gn-l-` twin in `tokens.css` — and `app.css` gives each token its twin
on a card. Whatever a card holds (text, fields, buttons, chips, a chart)
then reads as it would on a light theme. `genealogy_cards_read` measures
the cards' palette by the same rules (tightest pair, a control's edge on
the sunken surface, 3.7:1).

The record beside the river and beside the grid is not a card: it stays on
the page's palette, where its name and its header read (the first version
put it on the cards' and left its header's dark words on the dark page).
The cards inside it are cream.

A card carries a double rule, as old card stock is printed; titles are in a
book face; portraits are oval. A notice is an opaque card tinted by its
kind (the kit's veil reads as grey over this green). The overview's picture
is light and gold where the other themes' are dark, so its hero holds the
green further across it, and the title keeps to the side the green covers.

The page's frame is an antique vine along each edge, the middle the page's
own green. It is a frame, as Canopée's photographs are, not a texture under
the wash's switch (v0.4.3): nothing is written over the vines, and on a
narrow screen, or a page that takes the whole width, they are left out.
Each vine is a tall strip drawn at a third of its size or less, so that it
stays sharp on a dense screen; the first version stretched one wide
picture over the window, and it blurred.

### 3. The pictures

Made in the operator's Canva (generated, then exported at full size through
the working design), none edited from the reference:

| File | What it shows | Where |
|---|---|---|
| `genealogy-hero.jpg` | a golden tree, a scroll and empty oval frames, light on the left | the overview's hero |
| `genealogy-cta.jpg` | old letters, sepia photographs, a magnifying glass, a pen | the overview's last band |
| `genealogy-vine-l.jpg`, `genealogy-vine-r.jpg` | a tall climbing vine with rose blossoms on dark green, and its mirror | the page's edges |
| `notes-genealogy.jpg` | aged cream parchment with faint writing | behind the family's notes |
| `genealogy-tree.jpg` | an old tree with a twisted trunk and a crown of gold leaves growing out of a parchment scroll, on the stage's green | the climbed tree, 2,392 × 3,200 |
| `genealogy-tree-depth.jpg` | the same, as depth: near light, far dark | the tree's relief, 544 × 728 |

The photographs in `genealogy-cta.jpg` are generated: nobody in them
exists. The tree on its scroll and the wide vine ground of the first
version are gone.

The tree was generated from a description of the reference (an old tree
with a twisted trunk and gold leaves growing out of an upright parchment
scroll, its roots over the bottom roll; sepia, cream, antique gold; no
text, no portraits, no buildings), the first of two candidates. It was
then:

1. enlarged four times as ADR 0017's pictures are;
2. cut out along the mask Canva's "remove background" made of it;
3. set on the stage's green (`--gn-stage`), with a soft light behind the
   scroll and the scroll's shadow (`tools/pictures/compose.py`).

Its depth map comes from Depth Anything V2 (small), smoothed a little
(`tools/pictures/depth.py`). Should the stage's green change, the picture
is composed again on the new one.

### 4. Not done here: real portraits on the tree

The reference's cards hold photographs. The river's payload carries a
person's names, years and state, not their picture, and ADR 0007 keeps a
view to that payload: a view changes the drawing, never what the server
sends. Showing each person's portrait on the tree would add the avatar's
document to the payload, through the same lens that decides who may see
the record — a server change of its own, with its privacy surface, to be
decided separately. Until then the frames hold initials.

## Consequences

- Six themes. Genealogy is the second to change the page's polarity inside
  a component (the cards); a new component that should look like a card
  joins the list in `app.css`, or it stays on the dark page.
- The tree weighs 1.1 MiB, its depth 31 KiB, fetched once a day.
- A wider family than the crown spreads past the scroll onto the stage's
  green, where the cards and the lines (with their light under them) still
  read.
- The view adds two hooks to `river.js` that any view may use: a card
  (`rect.gn-card`) counts as a placed word for "All names", and a
  portrait's initials (`text.gn-initials`) do not count as a person's name.
- Its words in fourteen languages.
