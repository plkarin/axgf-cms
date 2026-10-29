# ADR 0002 — Study 06 is the default look, and every theme speaks its names

**Status:** accepted · superseded in part by
[ADR 0008](0008-three-themes.md): since 0.5.0 three themes are offered, and
no "match my system" · **Applies to:** `static/css/tokens.css`,
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

## 4. Type in rem, from the reader's own setting

**Decision.** Every type size is in rem. The read-mode body is 1rem: 16px
unless the reader has changed their browser's text size. The compact style
is 0.9375rem and paper 1.0625rem. The river's own small sizes, 0.625rem to
0.78125rem, are rem too. Buttons and fields take a minimum height (36px, or
28px small), not a fixed one, so larger text grows the control instead of
spilling out of it.

**Deviation from Study 06.** The study specifies 13.5–15.5px type in px.

**Why.** Px overrides the reader's own text-size setting. This product's
most attentive readers are often the oldest people in a family, reading on a
tablet. The setting they chose is the size the page should be.

## 5. A finger, not a pointer

**Decision.** On a coarse pointer (`@media (pointer: coarse)`) every button,
field and disclosure reaches 44 CSS px, the target size WCAG 2.5.5 asks for.
On the river, a tap is resolved to the nearest person within 44 CSS px, so a
person's touch target is at least 88px across. For a mouse, each person's hit
circle is sized to 44 CSS px at the drawing's current scale, never below its
drawn 20 units.

**Deviation from Study 06.** The study's controls are 36px and 28px, and the
river's dots are about 40px across at desktop width.

## 6. Hover has a touch equivalent: "tap to look, tap again to go"

**Decision.** On a touch screen, the first tap on a person in the river does
what hovering does with a mouse: it lights the route from the centre and
shows the person's dropped label. A second tap on the same person travels
there, and a tap on open water clears the selection.

**Deviation from Study 06.** The study reveals labels and lights the route on
hover only, and a tablet has no hover.

**Amended 2026-10-27 (the operator's review, on a phone: "the mesh cannot
be clicked").** A lit route alone looked like nothing happening. The first
tap now also names the person under the drawing: their full name and
years, and two ways on, "Centre here" (travel) and "Open full page" (their
record, absent for a person this reader may not see). The bar stays at the
foot of the screen while the drawing is in view (`.rv-pick`, river.js). A
second tap on the same person still travels; a tap on open water, or any
travel, puts the bar away. It holds under every view, since `river.js`
owns every touch rule.

## 7. Reading and editing are two modes

These changes come from the minimalism pass. They are not deviations from
Study 06's palette, but they change what B4 (the record tabs) built.

- **Read mode draws only what is recorded.** There are no labels without
  values, no dashes, no "Date unknown" or "no score", no empty sections, and
  no column a whole table would leave empty. A radar is drawn only from three
  axes or more.
  `tests/read_mode.rs` holds this over every person of the sample, and over a
  real archive on demand (866 people, 0 empty fields).
- **The record has its tabs back (v0.4.1).** v0.4.0 drew the record as one
  page without tabs. That removed two things when only one was asked for:
  hiding empty fields was right, removing the tabs went too far. Tabs
  organise what exists; they do not display what is missing. The record page
  and the panel beside the river carry Study 06's six — Fiche · Famille ·
  Événements · Sources · Médias · Profil — drawn as B4 draws them, and only
  those that hold something: a tab with nothing in it is not drawn at all,
  rather than dimmed. (B4 dimmed a zero-count tab because an empty tab is
  where you add things; that is edit mode's case, not reading's.) A record
  with one tab draws no strip. Inside each tab, read mode is unchanged. A
  radar still needs three axes, and a Profil tab that would hold only radars
  below that line is not drawn. The tabs are content controls, not chrome —
  how many there are follows what is recorded — so the record page's chrome
  count did not rise (8 signed out, 9 signed in, before and after).
- **In edit mode, empty is the normal state (v0.4.1).** Hiding what is empty
  governs reading. v0.4.0 applied it to the edit form too, putting every empty
  field behind one "n other fields" lump. It also dropped the only link into
  the profile editor, so the 132 attributes of AXGF 1.1's fourteen groups
  could not be reached — and were empty because there was no longer a way to
  fill them. The person editor now draws a person's own fields (names, dates,
  places, parents) without a disclosure, and lists all fourteen groups by
  name, always, each with how many of its fields hold a value. A group is
  closed by default and open when it has content. Opening one shows every
  field, filled or empty. That is one control per group, never one per field.
  `tests/profile_editor.rs` holds every attribute reachable for a person with
  none of them filled.
- **The editor has the reading tabs, all six, always (v0.4.2).** Modifier
  opens the editor on the tab being read (`#famille` for Famille). In edit
  mode no tab is ever hidden, empty ones included: an empty tab is where
  things are added. That asymmetry with reading is deliberate. The tabs show
  and hide parts of **one form with one save**. A save rewrites the whole
  archive, 12 to 17 s on the operator's 434 MB bundle, so six pages with six
  saves would turn an editing session into six waits. One Enregistrer turns
  every change, on every tab, into a single commit (`AppState::commit`),
  written all or nothing. Every record a tab draws (a union, the family
  somebody is a child of, an event, a source, a file, a profile group) is
  written back only when the form drew it and something in it changed.
  Without scripting, the tab on screen is the URL fragment (`:target`).
  `editor.js` adds marks on tabs with unsaved changes and asks before the
  page is left. The editor's 1.1 MB of HTML could be cut by splitting it into
  six server pages, but only at the cost of six saves. That trade becomes
  worth making once incremental saves land in axgf-rs 0.6.
- **Editing is entered explicitly, and looks different.** Editors get one
  "Modifier" on the record and on the panel, and edit mode carries a band
  under the masthead saying so.
- **A reader without the right to edit receives no edit control.** None is
  rendered for them, in any form: not disabled, not hidden by CSS. The walk
  holds this as a viewer account.
- **Every screen has a chrome budget.** Read-only screens may have 12 chrome
  controls, at most 7 of them in the masthead; edit and administration
  screens may have 20. A list row may carry two controls, and signing in may
  add at most three controls to a read-only screen. On the person editor the
  record's own fields are its content, not chrome: in edit mode they are what
  the editor came to change. Since v0.4.2, each of the editor's tabs is a
  screen with its own budget: the controls outside every tab plus its own.
  A record on a tab (a union, an event, a file) is content, one per thing
  recorded. The fixed "add" actions are chrome. The Profil tab's fourteen
  groups add one control each to its budget, and the walk fails a group with
  more than one control at rest.
  `tests/interactive_walk.rs` fails on any excess.

## 8. Dark wins unless the reader chose otherwise (v0.4.3)

**How a theme is chosen**, in order:

1. the theme stored on the reader's account;
2. the `axgf_theme` cookie;
3. `theme::DEFAULT`, which is `dark`, written into the page.

`prefers-color-scheme` is not in that list. Since v0.4.0 the server has
written `data-theme="dark"` for a reader who chose nothing, so the media
query already reached only "match my system". v0.4.3 makes that explicit and
closes the last gap:

- "Match my system" renders `data-theme="system"`, and the one
  `prefers-color-scheme` query in `tokens.css` applies to that selector and
  to nothing else.
- A page with no `data-theme` at all now falls back to Study 06 in the
  stylesheet as well. It used to fall back to the light palette on `:root`.

So a reader who has not chosen gets the dark ground even when their system
asks for light. Their system decides only if they ask it to, by choosing
"match my system". The theme picker's help line says so in every language.
`prefers-reduced-motion` keeps its power: it is a medical setting, and no
choice made here overrides it.

**Themes are named, not described.** Nocturne (dark, the default), Craie
(light) and Gravure (high contrast) keep their function as a subtitle.
Crépuscule (dark, warm), Encre (dark, midnight blue) and Fleuve d'hiver
(light, cool) are new. There are three new ones and no more, because each
theme multiplies the screenshot and contrast matrix. The colour-blind themes
keep the names of what they are for: there, the name is the function.

**Every theme is measured against more grounds than before.**
`every_theme_reads` now also measures text, control edges, the focus ring,
danger and warning on each of the three wash stops. For a theme that carries
a texture, it measures them again with the grain's strongest pixel
composited in (5% white on a dark ground, 5% black on a light one). All ten
token blocks pass with no exception. The three new themes were tuned against
those rules and two more the stylesheet's comments state: tree-card outlines
at 3:1, and crossing hues at 3:1 when drawn at 75% opacity.

**A theme may carry a texture, on the page only.** `--wash-texture` is one
more layer of the root element's background, under the same switch as the
wash. That switch is now **off until the reader turns it on**; it used to
default on. The texture is an inline SVG grain of a few hundred bytes: drawn,
never fetched, so it cannot block first paint. Crépuscule, Encre and Fleuve
d'hiver carry one; no other theme does.

It never goes on a button, a field, a card or a frame. Study 06 builds every
control from the river's own ground: surfaces a shade lighter than the page,
never a colour, and hairline edges. A textured control would undo that.

## 9. The masthead holds ten controls: the accessibility controls are not in a menu (v0.4.3)

**Decision.** The masthead shows the person search, then three native
selects (language, theme and density), each showing a short label, then a
link to the rest of the settings. The account menu stays on the right. The
theme used to be reachable only through the language button's menu.

The masthead's chrome cap rises from 7 to 10:

- the four sections;
- the search;
- language, theme and density;
- the settings;
- the account, or signing in.

**Why.** Theme, density and language are the accessibility controls.
Burying them in a menu hides them from the person who most needs them: the
one who must switch to a larger density or to high contrast before they can
read anything at all. A control that helps someone read is not clutter.

**What did not move.** Every page's own budget is what it was: 5 controls of
its own on a read-only screen and 13 on an edit screen or editor tab. Those
are the old totals of 12 and 20 with the old masthead's 7 taken out.
`tests/interactive_walk.rs` counts the masthead and the page separately, and
fails either one over its cap.
