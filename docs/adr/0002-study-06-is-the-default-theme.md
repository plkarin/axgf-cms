# ADR 0002 — Study 06 is the default look, and every theme speaks its names

**Status:** accepted · **Applies to:** `static/css/tokens.css`,
`src/theme.rs`, `templates/settings.html`, `tests/design_tokens.rs`,
`tests/themes.rs`

The site has had seven themes since before the river: light, dark, high
contrast, sepia, and one each for deuteranopia, protanopia and tritanopia,
plus "match my system". Study 06, "The Furniture", is a single dark palette.
Version 0.4.0 makes it the site's look. Three questions follow from that, and
this records how each was answered.

---

## 1. Study 06 is the default, and the other themes stay

**Decision.** Study 06 is the `dark` theme, and `dark` is what a reader gets
until they choose otherwise (`theme::DEFAULT`). Every other theme defines the
same token names — `--surface-page`, `--surface-raised`, `--surface-sunken`,
`--surface-hover`, `--border-hairline`, `--border-strong`, `--text-primary`,
`--text-secondary`, `--text-tertiary`, `--accent`, `--focus-ring`, `--danger`
— in its own colours. Components use only those names.

**Alternatives that were turned down.**

- *Study 06 replaces dark, light stays the default.* The default used to be
  "match my system", which on most machines is light: the signature look
  would be the one almost nobody sees.
- *Study 06 is the only look; the six other themes go.* See the reasons.

**Why.**

- The river's own design depends on the colour-blind themes surviving. The
  half-century bands exist as a second channel for time precisely because
  colour does not survive colour blindness or print (ADR 0001). Removing
  those themes would break a documented intent.
- High contrast is an accessibility feature, on a product whose most
  attentive readers are often the oldest people in a family.
- Only this way is the token layer a design system rather than a skin. The
  point of the names is that themes redefine them.

**What holds it.**

- `tests/themes.rs`: every theme defines every colour `:root` defines, so
  no theme inherits light's.
- `tests/design_tokens.rs`:
  - `the_dark_theme_is_study_06_and_so_is_the_river` pins the study's
    values in all three places they are written;
  - `every_theme_reads` measures WCAG AA for text and 3:1 for focus rings
    and control edges in every theme;
  - `no_colour_is_named_outside_the_tokens` keeps `tokens.css` the only
    file that names a colour.

"Match my system" is still on offer and still lets `prefers-color-scheme`
decide between light and Study 06. A reader who chose a theme keeps it: only
readers who never chose see the change.

## 2. The river is dark under every theme

**Decision.** The river keeps Study 06 whatever theme the page around it
has. `.river` sets the Study 06 values on itself, so its controls and the
record beside it sit on the river's ground under every theme. The theme
picker says so in every language (`theme-river-note`).

**What looks wrong.** A reader who chooses light, sepia or high contrast
gets a dark river in a light page.

**Why.** The five-stop water ramp (`river::colour_at`) is calibrated for a
dark ground: its stops were chosen for how they read on `#0b110e`. A light
ramp is its own project, not a by-product of a token sweep, and a ramp that
merely inverts would read wrongly. What the themes protect still holds on the
river: time is carried by the half-century bands as well as by colour, and
confidence by dash pattern, not hue.

**Not built:** a light water ramp. If one is ever wanted, it starts as its
own study.

## 3. One deliberate deviation from Study 06: the control edge

**Decision.** `--border-strong` is `#526a5d`, not the study's `#3a4b42`.

**Why.** Study 06 was produced without a contrast audit; the audit is newer
information, and it wins. The study's edge measured 2.06:1 on its own page
(1.91 on a raised surface, 2.00 in a field's well), and WCAG 1.4.11 asks 3:1
of the boundary a control is known by. `#526a5d` keeps the study's hue and
saturation and raises only its lightness, just far enough: 3.25 on the page,
3.02 raised, 3.15 in a well. The river's own `--rv-rule-3`, which is the same
edge, moves with it, so the river and the `dark` theme stay one palette.

**What holds it.** `every_theme_reads` measures every pair in every theme and
carries no exceptions; none may be added to it. The study's other eleven
values are held exactly by `the_dark_theme_is_study_06_and_so_is_the_river`.
