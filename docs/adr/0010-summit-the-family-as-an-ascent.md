# ADR 0010 — Summit: the family as an ascent at sunrise

**Status:** accepted, v0.5.0 (2026-10-20) · **Amends:**
[ADR 0008](0008-three-themes.md), which kept three themes; Summit is a
fourth · **Builds on:** [ADR 0007](0007-a-theme-may-bring-its-own-view.md)
· **Applies to:** `static/summit.js`, `static/css/tokens.css`,
`static/app.css`, `static/ui.js`, `static/img/summit-hero.jpg`,
`static/img/summit-cta.jpg`, `static/img/summit-panorama.jpg`,
`src/theme.rs`, `src/render.rs`,
`src/routes/mod.rs`, `src/routes/public.rs`, `templates/river.html`,
`locales/*.ftl`, `tests/views.rs`, `tests/themes.rs`,
`tests/design_tokens.rs`, `tests/controls.rs`, `tests/masthead.rs`,
`tests/brand.rs`, `tests/routes.rs`

## Context

On 20 October 2026 the operator asked for a fourth theme, *Summit*, from two
references:

- a dashboard over a mountain landscape at sunrise;
- a weather application: a night-navy interface, gold accents, a book-face
  headline, cards, and a temperature trend drawn as a gold line.

Their words:

> "Navigation through the family follows the ascent of a mountain, around
> sunrise in the mountains. The further we navigate through the genealogy
> and the closer we get to the summit, the snowier the mountain becomes."

The first mountain was drawn flat, in the theme's colours. Seeing it, the
operator asked the same day for more:

> "The mountain looks like a child's cartoon. I would like it more
> detailed, serious, and its surroundings too, the trees, the cliffs… an
> animated panorama, a bit like the river."

Then they corrected its direction, which had put the ancestors on the
heights:

> "You got the direction wrong: the oldest comes from the bottom and the
> youngest reaches the summit. Roughly, someone born in 1900 will be lower
> on the mountain than someone born in 2000."

Fusion and Galaxy already draw the river's layout their own way (ADR 0007).
Summit is the third view of that kind.

## Decision

### 1. Up the mountain to the descendants, down into the valley to the ancestors

`summit.js` gives river.js a view, under the same contract as Fusion's and
Galaxy's: `shape`, `edge`, `paint`, `rest`, `named`, and the scene.

**The camp.** The person at the fixed point stands at a camp: a flag, and
their name beneath it.

**The mountain keeps the river's vertical**, as the tree and the galaxy do:
the oldest at its foot, the youngest toward its summit. In the operator's
words, someone born in 1900 stands lower on the mountain than someone born
in 2000.

**The descendants climb.** They go up the mountain, one contour line per
generation, numbered ▲1, ▲2… at the edge. Each contour is a little narrower
than the one below it, as a mountain is. The narrowing is perspective only:
the order of every row is the river's.

**The ancestors go down.** They stand in the valley under the camp,
numbered ▼1, ▼2…, the furthest back the deepest in its mist. Its forest
edge sits between the camp and them. However short the steps of a long
range, the valley's first row stands far enough under the camp to leave
room for the camp's name and years.

**The trails.** Every parentage is a trail from parent to union to child. It
is solid, broken or a trace, as surely as it is known. Each trail climbs as a
path does: up the slope, across, then up again.

**The summit is the youngest generation the drawing reaches.** With no
descendant above them, the person at the camp stands on the summit, and the
camp's flag is the summit's.

**The mountain is a photograph, and everyone stands on its rock.** It is a
wall of peaks above a forest and a misty valley, at dawn (§6). It is laid
under the drawing so that:

- its summit is the drawing's, where the flag stands;
- the camp stands at the foot of its rock, at the top of the forest, unless
  the valley needs the room;
- it is never narrower than the window river.js opens on the drawing, and
  its valley floor reaches most of the way down the frame. Below it the
  valley goes on in mist.

Its crest runs the whole width, so the rows of descendants, which often widen
as they climb, have rock to stand on:

- the rows begin no higher than where the crest leaves the whole wall to
  stand on (27 % of the photograph's height). Above that a row would have
  only the summit's peak for its rock. Laying the rows lower moves the
  photograph, so the two are settled together;
- each row up the mountain is drawn in about the summit's axis as far as it
  still needs to stand under the crest where it is, and never less than the
  row below it.

So nobody stands in the sky. The drawing in changes no order.

A travel moves the photograph with the people, in step with river.js's own
(the same 560 ms, the same easing), so everyone keeps their rock.

### 2. The snow comes down with the years

The nearer the present the person at the camp was born, the nearer the
summit they stand, and the lower the snow line falls:

| Birth year at the camp | Snow |
|---|---|
| about 1700 | the summit's tip only |
| 1850 | half the way down to the camp |
| around 2000 | down to the camp |

The snow is laid on the photograph's own rock:

- just under the line, on the ledges and couloirs the photograph already
  holds snow in, so the line is never a ruled one;
- higher up, on the faces too;
- never on the cliffs, which are too steep to keep it.

Its relief is the rock's. Where the first light is on the rock, the snow is
rose. Climbing toward the summit, from parents to children, therefore makes
the mountain snowier, as the operator asked.

**The year is the one the river already rests on** (`meta.year`). It is
read only when this reader may see it (`meta.era`). A year the reader may
not see must not show through the weather. Without one, the snow keeps to
the heights, whoever is at the camp.

### 3. The sun rises with the climb

The sun follows how high the reader has climbed since they came (amended
2026-10-23): each generation travelled up toward the summit brings the dawn
on a step, and each generation down toward the valley takes it back one.
The climb is counted from the drawing the reader leaves — the generation
the person travelled to stood at in it — or, for someone who was not in it,
from the years between the two, when this reader may see both. It is
kept within what the sky can show, so the sun answers the first step either
way. Until then each person on the itinerary brought the dawn on, so that
going down to the ancestors raised the sun as well and, after six, it stood
still. As the dawn comes on:

- the sky warms and the last stars fade;
- the sun rises beside the summit, from behind the crest to above it;
- the photograph comes out of the blue of the night. The first light on its
  summit, which the photograph was taken in, waits for the dawn;
- at sunrise the sun comes down the wall from the summit.

The dawn shown comes on toward the dawn reached over a second or so, rather
than at a stroke. A climb starts in the night and reaches its summit at
sunrise, and climbing down takes the reader back toward the night. Nothing
is stored: the climb lives in the page, as the itinerary does.

### 4. The scene, and when it holds still

Two canvases, as Galaxy's:

- **under the drawing** (WebGL):
  - the photograph, in the light of the dawn reached, under the snow of the
    years, with the contour of every generation;
  - the sky behind its crest, the sun and its glow;
  - every frame: the last stars, high cloud drifting, snow blown off the
    summit, and mist drifting through the forest and over the valley floor;
- **over it:** light climbing every attested trail, from the valley toward
  the summit.

What changes only with a layout, the dawn or the window is drawn once into a
texture; only what moves is drawn every frame.

Before the photograph arrives, and if it never does, its outline stands in
the theme's rock, from the skyline the script carries. Without WebGL the
drawing carries a mountain of its own, drawn as wide as its people need.

It holds still on the river's terms:

- reduced motion;
- "Cut";
- the pause, "Hold the mountain still";
- a hidden tab;
- a river off screen;
- a machine too slow for it.

### 5. The dashboard's profile

The five figures are every view's. Beside them, the bars give way to the
climb's profile: how many people stand at each height, from the valley to
the summit. It is a gold line with its ground under it, like the
reference's temperature trend, with the camp marked on it.

### 6. The kit, in Summit's colours

Summit carries Canopée's furniture, as Fusion and Galaxy do, with:

- **its palette:**
  - a night-navy ground;
  - the gold of first light for its accent;
  - the dawn's violets, roses and peaches for its sky;
- **its shape:** a 12 px round, and an unhurried tempo;
- **its type:** titles in the book face (`--serif`), as the reference's
  headline;
- **three photographs**, made in the operator's Canva and exported at their
  native size:
  - a summit at sunrise above a misty valley;
  - footprints climbing a snowy ridge above a sea of clouds;
  - the mountain of the ascent (§1): a wall of peaks above a forest and a
    misty valley at dawn, 1776 × 896. It is served at 2048 × 1024 so the
    graphics card can filter it, and shown at its own aspect. Its skyline
    is written in `summit.js`: in each of 1024 columns, the first row that
    is no longer sky. It was read from the photograph: the first row of each
    column that departs from the sky above it, and stays departed. A new
    photograph needs a new skyline, and its summit and camp read again;
- **its words in eleven languages:**
  - "Every life is a step. Every family, an ascent.";
  - "Climb the mountain";
  - "↓ ancestors in the valley · ↑ descendants up toward the summit".

**Contrast** is measured by the rules of `every_theme_reads`, under the
theme's stars at their strongest pixel. The tightest pair is a control's edge
(`--border-strong`) on the third wash: 3.5:1. The gold reads at 8.8:1 on a
card, and the dark ink on it at 9.5:1.

## Consequences

- **Four themes.** `theme::THEMES` lists Canopée, Fusion, Galaxy and Summit,
  and `view_of("summit")` is `summit`. The selectors list four.
- **The kit's selector names four themes**, everywhere it is written:
  `app.css`, `ui.js`, `tests/brand.rs` and `tests/masthead.rs`.
- **The river is unchanged.** The server sends the same SVG and the same
  payload under every theme (ADR 0001, ADR 0007).
- **Tests:**
  - `views.rs` covers the new view like the other two:
    - it loads only its own script;
    - its words name no river;
    - its script and pictures, the panorama with them, are served and name
      no colour;
    - it gives river.js everything it asks for, and holds still on the
      river's terms;
  - `themes.rs` and `design_tokens.rs` measure five palettes: Study 06 and
    the four themes;
  - `controls.rs` holds the four themes' rounds;
  - `routes.rs` counts four themes in the settings.

## Alternatives considered

**Ancestors up the mountain.** The first version, the same day: going back
in a family is going up, *remonter* in French, and the summit was the
farthest the record reaches. It turned the river's vertical over, alone
among the views. The operator corrected it: the oldest come from the
bottom, the youngest reach the summit.

**Snow measured by the length of the itinerary.** Rejected. A reader who
travelled to their ancestors, down into the valley, would have found the
mountain snowier on the way down. The year says how near the summit the
camp stands, and it is the same for every reader allowed to see it.

**A mountain computed for every pixel**, as the river's ground is. Tried
first, the same day: rock in strata and couloirs from noise, snow, conifers
drawn one by one. The operator rejected it on sight as "a child's cartoon".
A photograph carries the detail computed rock lacks. The scene keeps what
must change with the climb: the light, the snow, the weather.

**A single pyramid.** The first panorama made had one narrow peak for its
summit. A family's rows often widen as they climb, so the highest rows
would have been squeezed onto its point. The wall chosen has a crest
across its whole width.

**Snow taken away under the line**, so that a recent camp shows bare rock.
The photograph's own dusting would have had to be painted out, and a face
stripped of it reads as concrete. The dusting stays; the snow of the years
is laid over it.
