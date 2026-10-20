# ADR 0008 — Three themes

**Status:** accepted, v0.5.0 (2026-10-19); amended by
[ADR 0010](0010-summit-the-family-as-an-ascent.md), which adds Summit as a
fourth theme · **Supersedes in part:**
[ADR 0002](0002-study-06-is-the-default-theme.md), for the themes it kept on
offer · **Applies to:** `src/theme.rs`, `static/css/tokens.css`,
`static/app.css`, `static/river-scene.js`, `templates/settings.html`,
`locales/*.ftl`, `tests/themes.rs`, `tests/design_tokens.rs`,
`tests/routes.rs`, `tests/views.rs`, `tests/river.rs`

## Context

Until 0.5.0 the site offered thirteen themes and "match my system":

- Canopée, the default, with Fusion and Galaxy beside it. They share
  Canopée's furniture (the kit, ADR 0004 and ADR 0007), and Fusion and
  Galaxy each draw the family their own way.
- Ten palettes laid over Study 06's furniture: Nocturne (Study 06 itself),
  Craie (light), Gravure (high contrast), Sépia, Crépuscule, Encre,
  Fleuve d'hiver, and one theme each for deuteranopia, protanopia and
  tritanopia.

On 19 October 2026, reviewing the 0.5.0 preview, the operator asked to keep
only "the three main themes we set up": Canopée, Fusion and Galaxy.

## Decision

### 1. Three themes

`theme::THEMES` lists `canopee`, `fusion` and `galaxy`, in that order.
Canopée stays the default.

### 2. No "match my system"

All three themes are dark, so the system has nothing to choose between.
No `prefers-color-scheme` query is left in the stylesheet.

### 3. A retired choice falls through to the default

An account or a cookie that names a retired theme is passed over like any
unknown value, and the reader gets Canopée. Nothing breaks, and the
reader's next choice is stored as usual. A form that posts a retired id is
refused with a 400, like any other unknown id.

A retired theme is not mapped to the "nearest" kit theme (Encre to Galaxy,
say). A reader who chose a palette did not choose a drawing, and Galaxy
changes the landing page as well as the colours. Canopée changes nothing
but the colours.

### 4. Study 06 is the floor

`:root` in `tokens.css` carries Study 06's values, so a page with no
`data-theme` gets Study 06 whole. The light palette that was on `:root` is
gone, and so are the ten theme blocks.

Before the change, each of the three themes was checked to define every
colour that `:root` defines. So none of them changes by a pixel.

### 5. The retired themes' words go

Twenty keys leave the eleven catalogues:

- the ten names and their notes;
- "Match my system";
- the two notes on the settings page. One said the default holds "whatever
  your system asks for", the other that the river stays dark under every
  theme. Both described choices that no longer exist.

## Consequences

### What readers lose

**High contrast.** Gravure was built for maximum luminance separation, and
no theme does that now. The three themes meet WCAG AA, and
`every_theme_reads` holds them to it on every ground, the page's wash under
its texture included:

- 4.5:1 for text;
- 3:1 for the edge of a control.

AAA is not claimed.

**Colour-blind palettes.** The three dedicated themes are gone. Confidence
never depended on telling hues apart. Under any theme it is carried by:

- the length of a bar;
- the fill of a pie;
- the printed percentage;
- the opacity and underline of inline text.

Crossing lines also carry a dash pattern beside their hue. What is lost is
the extra margin those themes gave: a confidence ramp with no hue in it at
all.

**A light page.** None of the three themes is light. The Paper style
changes the type and the spacing, never the colour.

### Follow-ups worth weighing

None of these is done.

- Honour `prefers-contrast: more` inside the three themes, with stronger
  edges and the secondary text raised to the primary.
- Add a fourth theme to the kit, a light one, if readers ask for it.

### What stays

**The theme's `wash` flag.** No theme refuses the wash now. The server
still resolves it from the theme and the reader together, and a theme built
for contrast would refuse it on itself.

### Tests

New:

- `three_themes_are_offered`;
- `a_reader_who_chose_a_retired_theme_gets_the_default`;
- `every_theme_on_offer_has_a_block_and_no_other_does`;
- `the_system_has_no_say_and_a_page_without_a_theme_is_dark`;
- `the_floor_is_study_06_and_so_is_the_river`.

Changed:

- `every_theme_reads` measures four palettes: Study 06 on `:root`, and the
  three themes.
- The completeness check now reads the colours on `:root` as well. It had
  been skipping them.

## Alternatives considered

**Keep Gravure and the colour-blind themes as an accessibility menu, away
from the masthead.** Not done. The operator asked for three themes. This
record states what that costs, so the question can be reopened.
