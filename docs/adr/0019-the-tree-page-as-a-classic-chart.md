# ADR 0019 — The tree page as a classic family chart

**Status:** accepted (2026-10-27) · **Builds on:** the band layout of
`src/tree.rs`, the side panel of the tree page · **Applies to:**
`src/chart.rs`, `src/tree.rs`, `src/person.rs`, `src/routes/public.rs`,
`src/routes/mod.rs`, `src/render.rs`, `templates/_tree_canvas.html`,
`templates/tree.html`, `static/tree.js`, `static/app.css`,
`static/img/laurel-ring.png`, `locales/*.ftl`

## Context

The tree page is for the reader who wants a family chart and not one of the
themes' scenes. It drew the family around a person by the bundle's own
generation numbers: everyone of a generation on one row, ordered to cross as
little as possible. Around a real person those rows mixed an ancestor's other
partners with the person's children, and the lines still crossed and ran over
one another. Each card carried a full name and two dates in one line, so a
row of cards was wide and hard to read.

The operator asked for a classic layout with ordered lines that never cross,
and for compact cards: "First name L.", which open on the pointer or on a
click to show the dates and the person's portrait. They also asked for the
kit to be drawn in Canva first.

## Decision

1. **Two trees joined at one person** (`src/chart.rs`).
   - The ancestors are drawn below the person, the oldest at the bottom, as
     everywhere in this application. Each person stands over their father
     and mother, and each parent over their own parents. An ancestor's other
     partners stand on the outer side.
   - The descendants are drawn above. The person and their partners share a
     row, each couple's children sit over the couple, and so on upward.
   - Every subtree has a horizontal band of its own, and two bands never
     overlap. As a result no couple's line or family's bus can cross another.
   - A person the chart reaches a second time (cousins who married, an
     ancestor on two lines) is drawn again where the line needs them. The
     card is dashed, marked ↺ and not followed further.
   - The chart returns the band layout's `TreeLayout`, so the filter, the
     highlight, the side panel and the whole-family view (`?all=1`, which
     keeps the band layout) are unchanged.
2. **Rows named from the person in the middle.** The rows are named
   "Parents", "Grandparents", "Children", "Grandchildren" and so on, with
   their count. The names stay at the start of the column while a wide
   chart scrolls under them. On a phone they become small tags above their
   rows.
3. **Right-angle lines in the accent.**
   - A couple's line carries a union node of two rings.
   - Children hang from the middle of that line, or from the card of a
     single parent, through one bus each.
   - The lines are drawn at right angles, like an inked family chart.
4. **Compact cards that open.**
   - A folded card is 140 × 40 px. It shows the first given name and the
     family name's initial, the person's hue as a slim strip and the
     certainty dot.
   - The card opens on the pointer after a short wait, so that moving across
     the chart opens nothing. It opens at once on focus, and a click pins it
     open.
   - An open card grows over its neighbours. It shows the portrait in a
     laurel ring (the kit's, a mask in the theme's accent), the full name,
     the dates with ✦ and †, and two actions: centre the tree on the
     person, or open their record.
   - A click also shows the record in the side panel without moving the
     tree.
   - Near the chart's edges the open card grows the other way. Its portrait
     is fetched the first time it opens.
   - Without scripts, the card's face is a link that centres the tree on
     the person.
5. **The chart's size is the reader's.** Smaller, larger or fitted to the
   column, applied with CSS `zoom` so the cards stay on their lines, and
   remembered from one visit to the next.
6. **On a phone, a column of generations** (the operator's review the same
   evening: the tree ran off the screen sideways). Under 640 px the chart
   has no lines, zones or name column. Each generation is a block with its
   name and its count, its cards two to a line, the newest at the top and
   the oldest at the foot, the person's own block outlined in the accent.
   Nothing scrolls sideways. An open card is a sheet at the foot of the
   screen, and the zoom is hidden. The markup is the same: each
   generation's cards sit in a `.tree-row`, which on a wide screen is no
   box at all (`display: contents`), so every card stands where the layout
   put it.
7. **The kit was drawn first.** The card, folded and open, the laurel ring,
   the union node and the row labels were drawn in Canva (2026-10-27). The
   laurel ring is served as `static/img/laurel-ring.png`, white on
   transparent at 256 px, and used as a mask.

## Consequences

- A person's portrait is chosen as their record's masthead chooses it. For a
  whole drawing it is read through `person::Portraits`, which reads the file
  once: which documents this reader may not see, and which documents name
  each person. Asked once per card, those two questions took the whole
  family of the full-size fixture to 1.8 s.
- The cards no longer show dates when folded. Dates live in the open card
  and in the side panel. The filter still matches the full name, so typing
  a family name finds "Marie D.".
- The script delegates on a card's face and on its "centre" link, never
  on the card itself, which holds both and the record's link: no control
  inside a card is swallowed (`tests/interactive_walk.rs`). A click
  elsewhere folds an open card by containment, not by claiming the card.
- The focused view is one layout and the whole-family view is another. A
  bundle too large for a chart around one person is still drawn whole by
  generation.
