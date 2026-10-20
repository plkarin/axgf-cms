# ADR 0010 — Summit: the family as an ascent at sunrise

**Status:** accepted, v0.5.0 (2026-10-20) · **Amends:**
[ADR 0008](0008-three-themes.md), which kept three themes; Summit is a
fourth · **Builds on:** [ADR 0007](0007-a-theme-may-bring-its-own-view.md)
· **Applies to:** `static/summit.js`, `static/css/tokens.css`,
`static/app.css`, `static/ui.js`, `static/img/summit-hero.jpg`,
`static/img/summit-cta.jpg`, `src/theme.rs`, `src/render.rs`,
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

Fusion and Galaxy already draw the river's layout their own way (ADR 0007).
Summit is the third view of that kind.

## Decision

### 1. Up the mountain to the ancestors, down into the valley to the living

`summit.js` gives river.js a view, under the same contract as Fusion's and
Galaxy's: `shape`, `edge`, `paint`, `rest`, `named`, and the scene.

**The camp.** The person at the fixed point stands at a camp: a flag, and
their name beneath it.

**The ancestors climb.** They go up the mountain, one contour line per
generation, numbered ▲1, ▲2… at the edge.

- Each contour is a little narrower than the one below it, as a mountain
  is. The narrowing is perspective only: the order of every row is the
  river's.
- This is the one view that turns the river's vertical over. "Going back"
  in a family is going up: *remonter* in French.

**The descendants go down.** They stand in the valley under the camp,
numbered ▼1, ▼2… Its forest edge and mist sit between the camp and them.

**The trails.** Every parentage is a trail from parent to union to child. It
is solid, broken or a trace, as surely as it is known. Each trail climbs as a
path does: up the slope, across, then up again.

**The summit is the farthest the record reaches.** With no ancestor above
them, the person at the camp stands on the summit, and the camp's flag is
the summit's.

**The mountain is drawn from where the frame put everyone.** Every contour
is as wide as the people on it need. From each one, the flanks spread as
they go down. So nobody ever stands in the sky, and the mountain rises with
its people as they travel. A crest of peaks crowns the highest contour, and
the summit is the tallest of them.

### 2. The snow comes down with the years

The snow line falls as the camp's birth year goes back in time:

| Birth year at the camp | Snow |
|---|---|
| around 2000 | the summit's tip only |
| 1800 | over half the mountain |
| about 1700 | down to the camp |

Tongues of snow run down the couloirs, so the line is never a ruled one.
Travelling up the family therefore makes the mountain snowier, as the
operator asked.

**The year is the one the river already rests on** (`meta.year`). It is
read only when this reader may see it (`meta.era`). A year the reader may
not see must not show through the weather. Without one, the snow keeps to
the heights, whoever is at the camp.

### 3. The sun rises with the climb

Each person on the itinerary brings the dawn on: the sky warms, the last
stars fade and the sun rises beside the mountain. A climb starts in the
night and reaches its summit at sunrise. River.js already keeps the
itinerary, one chip per person, so nothing new is stored.

### 4. The scene, and when it holds still

Two canvases, as Galaxy's:

- **under the drawing:** the sky, from night to dawn, the sun and its glow,
  and the last stars;
- **over it:** mist drifting in the valley, and light running down every
  attested trail, from the heights toward the living.

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
- **two photographs**, made in the operator's Canva and exported at their
  native size:
  - a summit at sunrise above a misty valley;
  - footprints climbing a snowy ridge above a sea of clouds;
- **its words in eleven languages:**
  - "Every life is a step. Every family, an ascent.";
  - "Climb the mountain";
  - "↑ ancestors up toward the summit · ↓ descendants in the valley".

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
    - its script and pictures are served and name no colour;
    - it gives river.js everything it asks for, and holds still on the
      river's terms;
  - `themes.rs` and `design_tokens.rs` measure five palettes: Study 06 and
    the four themes;
  - `controls.rs` holds the four themes' rounds;
  - `routes.rs` counts four themes in the settings.

## Alternatives considered

**Ancestors down, as in the river, Fusion and Galaxy.** Rejected. The
summit would then be the living, and "the closer to the summit, the
snowier" would mean the present. The operator's own words run the other
way: back in time is up.

**Snow measured by the length of the itinerary.** Rejected. A reader who
travelled down to their own children would have found the mountain
snowier. The year is what "further back" means, and it is the same for
every reader allowed to see it.
