# ADR 0017 — The views' pictures, sharp on a dense screen

**Status:** accepted, v0.5.0 (2026-10-23) · **Amends:**
[ADR 0004](0004-canopee-is-the-reference-theme.md) (the river's ground),
[ADR 0010](0010-summit-the-family-as-an-ascent.md) (the mountain),
[ADR 0015](0015-the-tree-in-its-glass-and-the-funnels-laboratory.md) (the
laboratory), [ADR 0016](0016-genealogy-the-family-on-its-scroll.md) (the
woods) · **Applies to:** `static/img/river-ground.jpg`,
`static/img/fusion-lab.jpg`, `static/img/summit-panorama.jpg`,
`static/img/genealogy-woods.jpg`, `static/app.css`, `static/fusion.js`,
`static/summit.js`, `static/genealogy.js`, `tools/pictures/`

## Context

On 23 October 2026 the operator asked for the pictures behind the views to
be "as clear and sharp as possible, in high quality".

Three views stand on a picture: Canopée's river on a forest seen from
above, Fusion's glass in a laboratory, Summit's ascent on a mountain.
Canva makes a picture between 1,680 and 1,776 pixels wide. The views' stage
on the operator's screen (about 1,470 pixels of page, the record beside the
stage) is about 1,000 pixels wide and 920 high. Each picture covers it, so
it is drawn to the stage's height or more. At 1.5 device pixels to the
pixel, that is 1.4 to 1.5 times the picture's own size. Summit's was
already enlarged once, from 1,776 × 896 to 2,048 × 1,024. On a screen of
two device pixels to the pixel it is twice the picture's size, and the
river's zoom goes further. Measured on screenshots at 1.5: the forest's
crowns, the rock and snow of the mountain, the laboratory's light were
soft, and the JPEGs' blocks showed.

The other three views — Galaxy, Clarity, and Genealogy's tree — are drawn,
on canvases sized to the screen's own pixels (up to two per pixel). They
are as sharp as the screen.

## Decision

1. **Served at about twice the largest stage they cover.** A view's picture
   is 3,840 × 2,160 (16:9). Summit's is 4,096 × 2,048: its 2:1 aspect, a
   power of two each way, as the graphics card wants for the mipmaps it
   filters the picture with. Summit still shows it at its original aspect
   (1,776 / 896), so its skyline, read on 1,024 columns, is unchanged.
2. **Enlarged from the lossless original**, never from a served JPEG:
   Canva's PNG export of the picture's page, at its own size. It is
   enlarged four times by Real-ESRGAN x4plus, a super-resolution model that
   draws the detail a plain enlargement blurs (the rock, the crowns). The
   weights' checksum is in `tools/pictures/upscale.py`. Plain enlargement
   (Lanczos) and the lighter `realesr-general-x4v3` were tried on a corner
   of the mountain; the first was soft, the second painted.
3. **Brought back to the original's tones.** The model darkens a picture a
   little, about four levels in 255. `tools/pictures/finalize.py` takes the
   broad tones (blurred over six pixels) from the original, enlarged
   plainly, and the fine detail from the model. It then saves the result
   at the served size, as a progressive JPEG at quality 88: 1.3 to 1.5 MiB
   for the forest and the mountain, a third of a MiB for the laboratory.
4. **A new cache version** for each changed picture's address (`?v=`),
   since the pictures are cached for a day.
5. **Fusion's laboratory is out of focus by design**, a depth of field
   behind the glass. Enlarging it removes the compression's blocks; it
   cannot bring into focus what was never in focus. A sharp laboratory
   would be a new picture, to be decided with the operator.

## Consequences

- The binary is about 4 MiB larger. A visitor fetches one of these
  pictures once a day, under the theme they read.
- Making a view's picture again means: export the Canva page as PNG, run
  `tools/pictures/upscale.py`, then `tools/pictures/finalize.py` at the
  sizes above, and change the address's version. The tools need a Python
  environment with PyTorch, and are not part of the build or the tests.
- The model draws plausible detail, not recorded detail. That is right for
  the landscapes behind the views, which show no one. It would be wrong for
  a family's own pictures, which are never enlarged this way (ADR 0009
  keeps reworked portraits apart from evidence for the same reason).
