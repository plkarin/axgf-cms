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
it "does not look like a tree at all, more like an ugly stem".

## Decision

### 1. The view: a tree the reader climbs

`static/genealogy.js` registers `window.axgfView` (name `scroll`, kept from
the first version) under ADR 0007's contract, from the river's own layout.
The family's own wood makes the tree:

- the person at the fixed point is set into a broad trunk, a little over
  the ground. Each family forks on the wood of the person it grows from,
  the partner's twig joining it there; the fixed point's own families fork
  at the top of the trunk, so that their children's limbs leave the trunk
  itself. Below the ground, each root parts toward the two parents;
- every stem is about as thick as all it bears — a twig for one person, a
  limb for a family, the trunk for everyone (`wood(load)`, the load counted
  up the crown or down the roots as far as the drawing reaches) — and
  grows as wood does: it leaves its fork at a slant and bends up toward
  the light, or, a root, down toward the deep (`bend`, which the lit route
  and the glints follow too). A stem is lit wood edged in dark bark, lit
  on its upper left, shaded on its lower right, its grain along it; the
  trunk's bark is fissured. Every stem stands out at 3:1 or more from
  whatever lies behind it, as WCAG asks of a graphic: its lit side on the
  dark leaves, the soil and the woods, its dark edge on the lit leaves.
  The cream cards hold their words at 7:1 or more: nothing is the colour
  of what it lies on;
- a crown of leaves stands behind the descendants, as wide as they reach
  and over the highest of them: a dome of foliage, darker inside and lit
  along its upper left, with clusters round each person up the tree. With
  no one up the tree the crown is the tree's own; a crown with fewer than
  three limbs gets the trunk's own, bearing no one;
- the ground lies the whole width of the window: soil from a little under
  the fixed point down, with its stones, rising in a mound round the trunk,
  moss and grass along its edge. The roots and the ancestors are under it;
- a generation further up or down is further from the camera, so it is
  drawn closer to the last and smaller, and each level's people spread
  round the trunk in the river's order, the nearest at the front and those
  round the side smaller, by the same tidy hierarchy as Fusion's tiers
  (ADR 0015);
- each person is a portrait in an oval gold frame — their initials, since
  the payload carries no picture (§4) — with a cream card beside it,
  outward from the trunk; the person at the fixed point has a larger frame,
  set into the bark, a halo and their card under it; a card that would
  cover another is left out, as everywhere, and the pointer or "All names"
  (ADR 0015 §4) names whoever has none. A parentage not attested runs
  along its stem as a seam: broken for an inferred one, a line of knots
  for a speculative one;
- travelling, the reader climbs: the camera rises to the person travelled
  to, or goes down, the tree sliding past and that person growing as they
  come level with it. The blend is done in the tree's own space — each
  person's place round the trunk and height on it — rather than between
  two pictures (`window.axgfView.tween`). The ground holds still, the tree
  coming up out of it or going down into it, and the woods move with the
  camera, slowly, as distant things do.

Its living scene: under the drawing, the woods (a picture, §3), the
ground and the crown's leaves, painted once into a still layer for each
drawing of the tree; over them, a band of mist, gold motes rising, leaves
falling, and a glint run up every attested stem. Painting the leaves
there, rather than as the drawing's own elements, keeps a climb at the
screen's own rate: the second version's drawing would have held some 1,300
elements, redrawn at every frame of a climb. It holds still on the river's
terms: reduced motion, "Cut", the pause, a hidden tab, a tree off screen, a
machine too slow for it.

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
| `genealogy-woods.jpg` | an ancient forest at dusk, light falling from the upper left, an empty clearing | behind the tree, 3,840 × 2,160 (ADR 0017) |

The photographs in `genealogy-cta.jpg` are generated: nobody in them
exists. The tree on its scroll and the wide vine ground of the first
version are gone.

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
- The view adds two hooks to `river.js` that any view may use: a card
  (`rect.gn-card`) counts as a placed word for "All names", and a
  portrait's initials (`text.gn-initials`) do not count as a person's name.
- Its words in fourteen languages.
