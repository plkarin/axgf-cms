# ADR 0019 — The tree page as a classic family chart

**Status:** accepted (2026-10-27), amended 2026-10-28 · **Builds on:** the
band layout of `src/tree.rs`, the side panel of the tree page · **Applies
to:** `src/chart.rs`, `src/tree.rs`, `src/person.rs`,
`src/routes/public.rs`, `src/routes/mod.rs`, `src/render.rs`,
`templates/_tree_canvas.html`, `templates/tree.html`, `static/tree.js`,
`static/app.css`, `static/css/tokens.css`, `static/img/laurel-ring.png`,
`locales/*.ftl`

## Context

The tree page is for the reader who wants a family chart and not one of the
themes' scenes. It drew the family around a person by the bundle's own
generation numbers: everyone of a generation on one row, ordered to cross as
little as possible. Around a real person those rows mixed an ancestor's other
partners with the person's children, and the lines still crossed and ran over
one another. Each card carried a full name and two dates in one line, so a
row of cards was wide and hard to read.

On 27 October 2026 the operator asked for a classic layout with ordered lines
that never cross. They also asked for compact cards: "First name L.", which
open on the pointer or on a click to show the dates and the person's
portrait. The kit was to be drawn in Canva first.

The next day they asked for the page to be read down the page, with no
horizontal scrollbar hiding people, and for the cards to be more compact
still. They wanted lines that show the relationship: a son blue, a daughter
red, an adoption dotted. They wanted the whole page clear and well
contrasted for readers of every age, clean and compact in every theme.

## Decision

1. **Two trees joined at one person, one column per generation**
   (`src/chart.rs`).
   - **Direction.** The oldest are on the left and the newest on the right,
     mirrored in a right-to-left language.
   - **Ancestors** are to the left of the person, each person beside their
     father and mother (the father above), and each parent beside their own
     parents. An ancestor's other partners stand with them in their column,
     on the outer side.
   - **Descendants** are to the right. The person and their partners are
     one above the other in their column, each couple's children in the
     next column, and so on. The first child stands level with the couple's
     knot and the others below, so a person stays at the top of their own
     family: centred on seventeen children, a person had sunk below the
     first screen. A later partner stands beside their own children, below
     the families before theirs, their knot level with them.
   - **No crossings.** Every subtree has a band of the page to itself, and
     two bands never overlap. No couple's bracket or family's comb can
     cross another.
   - **Repeated people.** A person the chart reaches a second time (cousins
     who married, an ancestor on two lines) is drawn again where the line
     needs them. The card is dashed, marked ↺ and not followed further.
   - **Shared output.** The chart returns the band layout's `TreeLayout`, so
     the filter, the highlight and the side panel are unchanged, and so is
     the whole-family view (`?all=1`, which keeps the band layout).
   - **Rows to columns.** Until 2026-10-28 generations were rows. Ten
     grandchildren side by side ran past the screen, and a horizontal
     scrollbar hid them.
2. **Read down the page, never across.**
   - A column is a generation, so the chart is as wide as the generations it
     shows and never as wide as its largest family. Seven generations
     (three each way) are about 1,120 px; a family of twelve children makes
     the chart taller, not wider (`tests` in `src/chart.rs`).
   - **Nobody is ever cut off.** The chart's column is clipped across and
     never scrolls. A chart still wider than its column (many generations,
     a narrow window) is scaled down to fit by `tree.js` with CSS `zoom`,
     which scales the lines with the cards. It is never scaled up, and
     never below three quarters, where a name is still easy to read.
   - **Past that scale**, the record moves under the chart, which takes the
     page's width. If even that is not enough, the chart is drawn as the
     phone's column of generations (§5), its cards as many to a line as
     fit and a card opening over its neighbours.
   - **The whole-family view** fits the same way. Its largest generations
     are folded wider than any column (a real family's is about 4,000 px),
     so it is mostly the column of generations, and its warning that it is
     several screens wide is taken away.
   - **Without scripts** nothing measures the chart, and its column
     scrolls rather than hiding anyone.
   - The record beside it is narrower than on other pages (at most 380 px)
     and grows with the page as it always has: one page scroll, not two
     (`tests/panel_layout.rs`). Under 1,200 px the chart takes the page's
     width and the record follows it, below.
   - Each column's name ("Parents", "Grandchildren", with the count) heads
     it and stays at the top of the window while the page scrolls down.
3. **Lines that say who the child is.**
   - **Couples.** A couple is joined by a bracket beside their cards, with a
     knot of two rings.
   - **Combs.** From the knot (or a lone parent's card) a comb runs to the
     next column. Brackets and combs are in the page's grey, never in an
     accent that could pass for a son's blue or a daughter's red (Galaxy's
     violet, Fusion's red).
   - **Colour by the child's side.** The line from the comb to each child
     is a son blue, a daughter red, anyone else grey (`--kin-boy`,
     `--kin-girl`, `--kin-other`, bright on every dark ground, deeper in
     Clarity). An only child's line is theirs from the knot.
   - **Pattern by lineage.** When a child belongs to the family other than
     by birth (AXGF 1.1's `lineage`), the line has a pattern: an adoption
     dashed, a fostering or a guardianship dotted, a step-child dash and
     dot.
   - **Confidence.** A line's opacity still says how sure the parentage is,
     never so faint that its colour cannot be told.
   - **Legend.** It draws the same lines with the same classes and shares
     the controls' bar: one band above the chart.
   - **Privacy.** A hidden child's line is grey and plain, as their card
     says no more (`tree::redact_in`).
4. **Compact cards that open.**
   - **Folded.** A card is 120 × 30 px: the first given name and the family
     name's initial in the strongest ink on a plain ground, and a strip of
     the person's colour (blue, red or grey, as their lines). At 112 px,
     one name in fifteen of a real family was cut. A name still too long
     for its card is a size smaller, never cut (`tree.js` measures it).
   - **Opening.** The card opens on the pointer after a short wait, so that
     moving across the chart opens nothing. It opens at once on focus, and a
     click pins it open.
   - **Open.** An open card grows over its neighbours, at its own size
     however far the chart is scaled. It shows the portrait
     in a laurel ring (the kit's, a mask in the theme's accent), the full
     name, the dates with ✦ and †, how sure the record is of the birth (the
     certainty dot and its words, moved here so that the folded card keeps
     its width for the name), and two actions: centre the tree on the
     person, or open their record.
   - **The record.** A click also shows the record in the side panel without
     moving the tree.
   - **Placement.** Near the window's edges the open card grows the other
     way. Its portrait is fetched the first time it opens.
   - **Without scripts**, the card's face is a link that centres the tree on
     the person.
5. **On a phone, a column of generations.** Under 640 px the chart has no
   lines, zones or column heads. Each generation is a block with its name
   and its count, its cards two to a line, in the order the columns read
   (the oldest first), the person's own block outlined in the accent.
   Nothing scrolls sideways. An open card is a sheet at the foot of the
   screen. The markup is the same: each generation's cards sit in a
   `.tree-row`, which on a wide screen is no box at all
   (`display: contents`), so every card stands where the layout put it.
6. **The kit was drawn first.** The card, folded and open, the laurel ring,
   the union knot and the generations' names were drawn in Canva
   (2026-10-27). The laurel ring is served as `static/img/laurel-ring.png`,
   white on transparent at 256 px, and used as a mask.

## Consequences

- **Portraits.** A person's portrait is chosen as their record's masthead
  chooses it. For a whole drawing it is read through `person::Portraits`,
  which reads the file once: which documents this reader may not see, and
  which documents name each person. Asked once per card, those two
  questions took the whole family of the full-size fixture to 1.8 s.
- **Folded cards** show neither dates nor certainty. Both live in the open
  card and in the side panel. The filter still matches the full name, so
  typing a family name finds "Marie D.".
- **Script delegation.** The script delegates on a card's face and on its
  "centre" link, never on the card itself, which holds both and the
  record's link: no control inside a card is swallowed
  (`tests/interactive_walk.rs`). A click elsewhere folds an open card by
  containment, not by claiming the card.
- **Highlight.** A line from a couple names both parents, and a family's
  comb every child it leads to (`data-from`, `data-kids`). The highlight
  lights it with any of them.
- **Two layouts.** The focused view is one layout and the whole-family view
  is another. A bundle too large for a chart around one person is still
  drawn whole by generation, folded to the column. Its lines take the same
  colours and patterns. Its warning about width shows only when it still
  came out wider than the column.
- **The zoom buttons are gone**, and their words with them: the chart fits
  by itself.
