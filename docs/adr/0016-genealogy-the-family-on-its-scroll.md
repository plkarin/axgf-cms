# ADR 0016 — Genealogy: dark green and cream, the family on its scroll

**Status:** proposed, v0.5.0 (2026-10-23), until the operator's review ·
**Builds on:** [ADR 0007](0007-a-theme-may-bring-its-own-view.md),
[ADR 0013](0013-clarity-the-family-as-a-funnel.md) (a light palette) ·
**Applies to:** `static/genealogy.js`, `static/css/tokens.css`,
`static/app.css`, `static/ui.js`, `static/river.js`,
`static/img/genealogy-*.jpg`, `static/img/notes-genealogy.jpg`,
`src/theme.rs`, `src/render.rs`, `src/routes/mod.rs`,
`src/routes/public.rs`, `templates/river.html`, `locales/*.ftl`,
`tests/views.rs`, `tests/themes.rs`, `tests/design_tokens.rs`,
`tests/controls.rs`, `tests/masthead.rs`, `tests/brand.rs`

## Context

On 23 October 2026 the operator asked for a sixth theme, "Genealogy", and
gave its frame pack as two screens:

1. a tree growing out of an old parchment scroll, sepia portraits on the
   parchment joined to it by thin gold lines with square corners, on a warm
   grey ground — "exactly the navigation mesh", the colours and the
   navigation to keep, the building in the trunk to take out;
2. a dark green interface: cream cards with rounded corners, oval portraits,
   a table in a cream card, titles in a book face, antique floral vines
   climbing both edges of the page.

Their words: rework the same pictures in Canva and build the frame pack
accordingly.

The first screen carries a third party's watermark, and the repository is
public, so it was not edited: five new pictures were made in Canva in its
spirit, the tree without any building (§3).

## Decision

### 1. The view: the family on its scroll

`static/genealogy.js` registers `window.axgfView` (name `scroll`) under
ADR 0007's contract, from the river's own layout:

- the tree on its scroll fills the stage, under a veil, its trunk under the
  person at the fixed point, the row of that person where the trunk leaves
  the scroll;
- a row per generation, ruled faintly, numbered at the right as the river's
  rail: descendants above, toward the crown, each family's children side by
  side over it with the parents centred under them; ancestors below, toward
  the roots, each person's father and mother under them. The river's order
  from left to right decides who comes first; the layout is the same tidy
  hierarchy as Fusion's (ADR 0015), flat;
- each person a portrait in an oval gold frame — their initials, since the
  payload carries no picture (§4) — with a cream card beside it, outward
  from the trunk, that names them and gives their years; the person at the
  fixed point a larger frame with a halo and their card under it;
- the lines thin and gold, with square corners, as the reference's and as
  an engraved family tree's: an attested parentage a whole line, an
  inferred one a broken line, a speculative one a trace of dots; each family
  a gold knot where its lines meet;
- a card that would cover another is left out, as everywhere; the pointer
  names whoever has no card, and so does "All names" (ADR 0015 §4). While
  only the near generations are named, a far person keeps their portrait
  and its initials.

Its living scene: a few gold leaves falling from the crown, and a glint run
along every attested line, older to newer. It holds still on the river's
terms: reduced motion, "Cut", the pause, a hidden tab, a scroll off screen,
a machine too slow for it.

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

A card carries a double rule, as old card stock is printed; titles are in a
book face; portraits are oval. A notice is an opaque card tinted by its kind (the
kit's veil reads as grey over this green). The overview's picture is light
and gold where the other themes' are dark, so its hero holds the green
further across it, and the title keeps to the side the green covers.

The page's ground is the vines picture: antique vines along both edges, the
middle plain and the page's own green. It is a frame, as Canopée's
photographs are, not a texture under the wash's switch (v0.4.3): nothing is
written over the vines, and on a narrow screen, where only the middle shows,
they are left out.

### 3. The pictures

Made in the operator's Canva (generated, then exported at full size through
the working design), none edited from the reference:

| File | What it shows | Where |
|---|---|---|
| `genealogy-tree.jpg` | a golden tree growing from an unrolled parchment scroll, roots on its lower roll, no building | the view's stage |
| `genealogy-hero.jpg` | a golden tree, a scroll and empty oval frames, light on the left | the overview's hero |
| `genealogy-cta.jpg` | old letters, sepia photographs, a magnifying glass, a pen | the overview's last band |
| `genealogy-ground.jpg` | dark green with antique vines along both edges | the page's ground |
| `notes-genealogy.jpg` | aged cream parchment with faint writing | behind the family's notes |

The photographs in `genealogy-cta.jpg` are generated: nobody in them exists.

### 4. Not done here: real portraits on the scroll

The reference's cards hold photographs. The river's payload carries a
person's names, years and state, not their picture, and ADR 0007 keeps a
view to that payload: a view changes the drawing, never what the server
sends. Showing each person's portrait on the scroll would add the avatar's
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
