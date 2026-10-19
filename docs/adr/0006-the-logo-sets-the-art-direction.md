# ADR 0006 — The logo sets the art direction

**Status:** accepted, v0.5.0 (2026-10-19) · **Applies to:**
`static/img/logo-mark.svg`, `static/img/favicon.svg`, `static/img/*.png`,
`static/img/favicon.ico`, `static/site.webmanifest`, `templates/_logo.html`,
`templates/base.html`, `templates/admin_login.html`,
`static/css/tokens.css`, `static/app.css`, `src/render.rs`,
`src/routes/public.rs`, `src/routes/mod.rs`, `tests/brand.rs`

## Context

On 19 October 2026 the operator gave ax·genealogy its logo, `logo-mark.svg`:

- a deep green tile (`#14322b`, rounded at a fifth of its side);
- a laurel wreath in pale gold (`#f7d99a`);
- inside it, an X of four strokes. The two lower ones, thick, in a gold that
  deepens toward the bottom (`#a06f24` → `#e6b254`), rise from the corners
  and meet at the centre. There they part into two lighter, thinner ones
  (`#e6b254` → `#f7d99a`).

It is the river as a mark: the sources below meet in a person, and the
descendants branch out above.

They asked for three things:

- the logo wherever the site shows one, the favicon included;
- the logo to set the art direction;
- the theme to stay: Canopée (ADR 0004).

Before this:

- Canopée's mark was a line-drawn leaf in a lime ring, under Canopée only;
  every other theme drew the wordmark alone.
- The tab's icon was the 🌳 emoji, in a data URI.
- There was no icon for a phone's home screen and no manifest.

## Decision

### 1. One drawing, kept whole

- **The file.** `static/img/logo-mark.svg` is the operator's file with its
  embedded content-credentials manifest removed (11.7 KB → 3.9 KB). The
  original stays with the operator. Every stroke, leaf and colour is theirs.
- **The inline copy.** `templates/_logo.html` draws the same paths and
  ellipses inline, with classes in place of colours, so that a theme can
  colour it. It takes an id prefix, so two logos on one page do not share a
  gradient's id.
- **The guard.** `tests/brand.rs` holds the two copies identical, path for
  path and leaf for leaf.

### 2. In the masthead under every theme, coloured under Canopée only

- **Under Canopée**, the logo wears its own colours from four tokens,
  `--logo-tile`, `--logo-deep`, `--logo-gold` and `--logo-light`. They are
  the logo's own values, defined in Canopée's block alone.
- **Under every other theme** it is drawn in ink, with no tile, in the text's
  own colour, like the wordmark beside it. Study 06's rule that nothing in
  the bar is coloured (`tests/masthead.rs`) holds, and the logo is still
  there.
- **On the sign-in page** it sits above the title: 76 px under Canopée,
  64 px elsewhere.
- **In markup** it is decorative (`aria-hidden`), since the name is written
  beside it.

### 3. The icons a browser and a phone ask for

- **The tab's icon.** `favicon.svg` is the tile and the X without the
  wreath. The wreath turns to noise under 48 pixels, so the X is drawn at
  0.88 of the tile, where the logo draws it at 0.78. `favicon.ico` carries
  the same drawing at 16, 32 and 48 pixels, for every client that reads no
  SVG or asks at the root.
- **The home-screen icons.** `apple-touch-icon.png` (180 pixels, where iOS
  looks even when the page does not say) and `icon-192.png` and
  `icon-512.png` are the whole logo, full bleed: the platform rounds them
  itself. The wreath lies within 38 % of the radius, so a circular mask
  leaves it whole.
- **The manifest.** `site.webmanifest` names both icons. Its display is
  `browser`: an icon on the home screen, and no change to how the site
  behaves.
- All of them were rendered from the SVG in Chromium and are embedded like
  every other asset. The content-security policy already allowed them.

### 4. Canopée's accent is the logo's gold

| token | was | is |
|---|---|---|
| `--accent` | `#a8d672`, lime | `#e6b254`, the logo's middle gold |
| `--focus-ring` | `#b4dc7c` | `#f7d99a`, its pale gold |
| `--accent-soft` | `#1c2b1b` | `#2a2515` |
| `--glow-rgb` | `168 214 114` | `230 178 84` |

Measured as `every_theme_reads` measures:

- the gold on Canopée's lightest ground, 7.14:1;
- the dark ink on a gold button, 9.54:1;
- the focus ring, at least 10:1.

This changes:

- the primary buttons and the sign-in;
- the section you are in;
- the links and the rule under every title;
- the second line of the overview's headline;
- the ring around a portrait;
- the glows.

This stays:

- the ground and its forest;
- the river;
- every component and its motion;
- every other theme.

The river already had a gold accent of its own (`--rv-accent`, `#dcae64`),
so the site and the river now agree. The tree's icon disc keeps its green,
which is a category colour and not the accent.

## Consequences

- A theme that wants the logo in colour defines the four `--logo-*` tokens
  and colours `.logo-mark` under its own selector, as Canopée does.
- Gold now names both the accent and the "medium" band of confidence
  (`--conf-medium`, `#e0b46c`). They do not compete: confidence is drawn on
  bars and dots that also carry its shape (ADR 0003), and the accent marks
  what can be pressed.
- The README's screenshots predate the gold. They are made again with the
  showcase scripts once the operator has seen it.

## How this was verified

- `tests/brand.rs`:
  - every icon is served with its type;
  - the page links them, and the emoji is gone;
  - every icon in the manifest exists;
  - the masthead carries the logo under each of the twelve theme choices;
  - only Canopée's rules colour it;
  - the inline copy is the file's drawing;
  - the sign-in page's two logos keep distinct ids.
- `tests/themes.rs`, `tests/design_tokens.rs`, `tests/masthead.rs` and
  `tests/controls.rs`, unchanged and passing with the new palette.
- By eye, at 1920 pixels:
  - the masthead under Canopée, Chalk, Nocturne, Engraving and Sepia;
  - the sign-in page under Canopée and Chalk;
  - the overview, a record, the editor and the river under Canopée.
