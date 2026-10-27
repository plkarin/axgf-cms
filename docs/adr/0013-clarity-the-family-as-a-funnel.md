# ADR 0013 — Clarity: a light theme, the family as a funnel

**Status:** accepted, v0.5.0 (2026-10-21) · **Amends:**
[ADR 0008](0008-three-themes.md), whose themes were all dark; Clarity is the
first light one · **Builds on:** [ADR 0007](0007-a-theme-may-bring-its-own-view.md)
· **Applies to:** `static/clarity.js`, `static/css/tokens.css`,
`static/app.css`, `static/ui.js`, `static/img/clarity-hero.jpg`,
`static/img/clarity-cta.jpg`, `src/theme.rs`, `src/render.rs`,
`src/routes/mod.rs`, `src/routes/public.rs`, `templates/river.html`,
`locales/*.ftl`, `tests/views.rs`, `tests/themes.rs`,
`tests/design_tokens.rs`, `tests/controls.rs`, `tests/masthead.rs`,
`tests/brand.rs`, `tests/routes.rs`

## Context

On 21 October 2026 the operator asked for "a last theme named Clarity",
taking two screenshots as its frame design pack:

- a glossy funnel chart in blue, violet and pink, its stages labelled with
  their figures (12,480 visitors, 3,920 product views, 1,204 added to cart,
  482 purchases);
- an IT administration dashboard on white: an orange accent, dark teal
  bars, cards, KPI tiles.

Their words for the navigation:

> "A progressive chart that meshes the family from down (oldest) to up
> (newest), the links through an inverted cylindrical pyramidal cone."

## Decision

### 1. The view: an inverted cone of glass slices

`static/clarity.js` registers `window.axgfView` under ADR 0007's contract,
from the river's own layout:

- one slice per generation, its height a generation, its width growing from
  the narrow end (the oldest generation shown) to the mouth (the newest);
- each slice a glass band seen slightly from above — its opening, a gloss —
  in five tones from blue at the mouth through violet to pink at the narrow
  end, as the reference's stages;
- each generation's people on the front of their slice, in the river's
  order from left to right; the person at the fixed point at the front of
  theirs, partners beside;
- every parentage a line meshed along the cone's wall from one slice to the
  next, solid, broken or a trace as surely as it is known;
- at each slice's right, as a funnel's stages, how many people it holds and
  which generation it is.

Names give way to beads and to those figures; the pointer brings a dropped
name back, as everywhere.

Its living scene: light carried up the attested links, from the older end
to the newer, and a sheen crossing the glass now and then. It holds still on
the river's terms: reduced motion, "Cut", the pause, a hidden tab, a river
off screen, a machine too slow for it.

### 2. The first light theme

`[data-theme="clarity"]` sets `color-scheme: light`: a pale ground, white
cards, dark teal text, the orange accent of the reference. It carries the
kit (Canopée's furniture), its round (10 px) and tempo, and lighter
shadows: the kit's are drawn for a dark ground and would be bruises on a
white one.

The operator's second look found it a sheet of paper: white boxes on a
near-white ground (1.08:1), nothing to tell one kind of information from the
next. The ground is now blue-grey (1.2:1 against a box), and every box says
what it holds:

- a white card with a hairline and a shadow, a 3 px strip along its top in
  its category's ink, the same ink as the icon chip beside its heading (a
  square chip rather than the kit's disc), its heading over a rule;
- the figures' tiles in five inks in turn — the funnel's blue, violet and
  pink, the accent's orange, the dashboard's teal — each with its tint for
  the chip, each ink 5:1 or more on its tint and on white;
- a group of fields is a pack, a pale panel whose legend is a pill, and what
  sat sunk into the page inside a box (a name, a union, a fact) is a pack
  too; a notice is tinted by its kind;
- a finding of the console's checks is a white row with its level along its
  edge, each area a heading drawn out by a rule;
- the record's editor is a box per tab, its strip in the colour of what the
  tab writes, the save a white bar floating over the page.

The paper style keeps none of it: it stays flat.

It is measured as every theme is (`every_theme_reads`): the tightest pair is
a control's edge on the page's second wash, 3.7:1; the orange is dark enough
to carry white text (6.1:1) and to be a link on any surface.

A reader chooses it like any other theme; the operating system's light or
dark preference still has no say.

### 3. The logo

The same drawing (docs/adr/0006) in Clarity's colours: a white tile, its
edge a hairline so that it holds its shape on a white ground; the X in the
funnel's gradient, pink where the sources meet, violet, blue toward the
descendants — the funnel's own reading, oldest below, newest above; the
laurels blue. The operator's first look found the first attempt (a dark
teal tile, an orange X) still read as Canopée's green and gold.

### 4. Photographs

Two, made in Canva from the references and exported at native size (pages
12 and 13 of the theme visuals design): a glass funnel of stacked rings,
blue to pink, on white, behind the overview's headline ("Every generation
is a ring. Every family, a funnel."); translucent ribbons with orange beads
of light for the last band.

### 5. A prism under the page (the operator's review, 2026-10-27)

The operator found Clarity "very ugly": the family's navigation frame and
the page's ground white on white. They asked for stylised pictures and a
very beautiful theme with contrast, made freely in Canva.

- **The ground.** The page stands on a vivid prism of glass, violet, blue,
  pink and orange light (`clarity-prism.jpg`, made in Canva, 3,840 × 2,160
  as ADR 0017 says), fixed to the window, under a veil of the page's
  surface (52 %; 80 % in the paper style), so that it reads as light and
  colour, never as a picture behind the words.
- **Frosted glass.** The masthead and every card are frosted glass over it
  (the surface at 72 % and 84 %, blurred and saturated behind): the words
  stand on them, never on the picture. The paper style keeps its cards flat.
- **The funnel.** Its stage shows the prism itself; its slices are tinted
  in five pastel tokens from blue to pink (`--cl-slice-1` to `5`), edged in
  the rim's ink, lit along their rims; its links are a deep indigo
  (`--cl-link`) over a halo of the rim, so that they cross the glass and
  the light alike; its figures and its generations' numbers stand on a
  stroke of the surface. The dashboard's bars take the funnel's inks.
- **The pictures** behind the overview's headline, its last band and the
  notes are made again in Canva: a crystal funnel on deep violet,
  iridescent ribbons, calm deep-blue glass.

No colour is named outside the tokens (`tests/design_tokens.rs`), and the
theme is measured as every theme is.

## Consequences

- Five themes, one light. ADR 0008's "every theme is dark" no longer holds;
  its reasons (one ground to measure, no flash) are kept: the theme is
  still rendered server-side, and its palette is measured like the others.
- Any part of the interface that assumed a dark ground shows under
  Clarity first; the kit's shadows were the one found, and are overridden.
- Adding a view still takes ADR 0007's four things.

## Alternatives considered

- **A flat funnel chart, trapezoids without depth.** Rejected: the
  reference is glossy and the request a cone; a flat chart reads as a
  report, not as a place to travel.
- **The oldest at the mouth.** Rejected: the operator's words put the
  oldest below and the newest above, and the mouth above.
