# ADR 0044 — Genealogy and Clarity: softer colours, a wider walk

**Status:** accepted, v1.13.0 (2026-11-09); §2 and §3 withdrawn the same evening (see the end) · **Amends:** docs/adr/0013 (Clarity's
palette and its prism), docs/adr/0016 (Genealogy's palette and its tree) ·
**Applies to:** `static/css/tokens.css`, `static/app.css`, `static/river.js`
(`frame`, the camera, `meta.wide`, the names' default), `static/genealogy.js`,
`static/clarity.js`, `templates/river.html`, `locales/*.ftl`

## Context

On 9 November 2026 the operator asked for the next step of the concept:
Genealogy and Clarity made prettier, because "the colour contrasts are too
violent" — the colours softened, the texts legible with contrasts that suit
the reading sections — and "the two navigation meshes optimised along the
horizontal axis: more length across, to favour reading and the genealogical
walk".

What they saw: Genealogy's near-black green page (`#142210`) carrying bright
cream cards (`#f4ecda`, 14:1 off the page), saturated pink and gold vines,
words at 14:1; Clarity's page and funnel on a loud prism of glass, half
veiled on the pages and not at all behind the funnel, its category inks at
full saturation and its words at 16:1. And in both drawings, a crowded
generation squeezed into the window, its people a few dozen pixels apart,
named only for the generations either side of the fixed point.

## Decision

### 1. Softer palettes, still measured

Both palettes keep every pair `every_theme_reads` and `genealogy_cards_read`
measure (text 4.5:1, control edges 3:1, on every surface and wash), and
come down from the extremes:

- **Genealogy**: the page a moss (`#283a2e`) rather than a near-black, the
  cards a quieter parchment (`#ebe3d0`) with a softer sepia, the gold an old
  brass; the words at 8–10:1, the cards 9.5:1 off the page (14:1 before).
  The vines are faded into the page's moss, their strips' own dark ground
  blended away (`background-blend-mode: lighten`), and the cards' shadow is
  lighter. The tree's stage, knots, cards and mounts follow.
- **Clarity**: no pure white and no near-black (`#fbfcfd`, `#25323f`), the
  words at 11–13:1, the accent a terracotta, the five category inks quieter
  (each still 4.5:1 or more on its tint); the prism four fifths veiled on
  the pages and veiled behind the funnel, the frosting without its extra
  saturation, the funnel's glass paler.

### 2. A wider walk

A view may ask for more length across than the window: `meta.wide`, a number
of windows. `river.js` then frames the drawing at the window's own scale and
lets the camera travel along the drawing at any zoom — a drag, a trackpad's
sideways swipe or Shift with the wheel, a finger — while the wheel alone
still scrolls the page; after a travel the camera comes back to the person
travelled to. A line under the drawing says it can be walked sideways, only
when it can.

- **Genealogy's tree** spreads a level's people round the trunk so that
  neighbours stand a card's width apart (up to six windows), the cards may
  use the whole width, and the levels' numbers follow the camera at the
  window's edge.
- **Clarity's funnel** widens its rings until each person has a name's width
  (up to six windows), centred.

### 3. Every name, there being room

The two views name everyone by default (`namesAll`): the room is there now.
"All names" shows as pressed, and the reader's own choice, once made, is
kept as before (docs/adr/0004's names, 2026-10-23).

## Consequences

- The two themes read calmer, and a crowded generation can be read name by
  name, walked sideways rather than squeezed.
- The river under the other themes is unchanged: no view but these two asks
  for `meta.wide`, and without it `river.js` frames as it did.
- A new text, `river-wide-hint`, in the fourteen languages (machine
  translations, to be reread).

## Withdrawn: the wider walk and every name (9 November 2026, evening)

Having seen v1.13.0, the operator found the drawings too wide: "it must be
able to stay within a max width, not overflow sideways, for both". The two
drawings keep within their window again, as before v1.13.0 (§2 withdrawn:
no `meta.wide`, no sideways walk at rest, no line saying so), and with no
more room than the window, the near generations are named by default again,
as before (§3 withdrawn; "All names" still names everyone). The softer
palettes of §1 stay.
