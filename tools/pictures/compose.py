"""Genealogy's tree on its stage (docs/adr/0016 §3): the enlarged picture,
brought back to the original's tones as finalize.py does, cut out along
the mask Canva's "remove background" made of it, and set on the stage's
green (--gn-stage) with a soft light behind the scroll (--gn-stage-lit)
and the scroll's shadow. Its last pixels are the stage's own colour, so
that genealogy.js can run that colour on past them.

usage: compose.py ORIGINAL ENLARGED CUTOUT OUT WIDTH HEIGHT STAGE_HEX LIT_HEX

    compose.py tree.png tree-x4.png tree-cut.png genealogy-tree.jpg 2392 3200 '#142519' '#2a3a26'
"""
import sys, os
import numpy as np
from PIL import Image, ImageFilter
Image.MAX_IMAGE_PIXELS = None
orig, up, cut, out, W, H, stage, lit = sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4], int(sys.argv[5]), int(sys.argv[6]), sys.argv[7], sys.argv[8]
hexrgb = lambda h: np.array([int(h.lstrip('#')[i:i + 2], 16) for i in (0, 2, 4)], dtype=np.float32)
E = Image.open(up).convert("RGB").resize((W, H), Image.LANCZOS)
L = Image.open(orig).convert("RGB").resize((W, H), Image.LANCZOS)
blur = ImageFilter.GaussianBlur(radius=6)
e, gl, ge = (np.asarray(x, dtype=np.float32) for x in (E, L.filter(blur), E.filter(blur)))
pic = np.clip(e + (gl - ge), 0, 255)
A = Image.open(cut).getchannel("A").resize((W, H), Image.LANCZOS).filter(ImageFilter.GaussianBlur(1.2))
a = np.asarray(A, dtype=np.float32)[..., None] / 255
# The stage: its green, lighter behind the scroll, falling back to the plain
# green well inside the picture's edges.
yy, xx = np.mgrid[0:H, 0:W].astype(np.float32)
r = np.sqrt(((xx - W * 0.52) / (W * 0.42)) ** 2 + ((yy - H * 0.5) / (H * 0.46)) ** 2)
glow = np.clip(1 - r, 0, 1)[..., None] ** 1.6
bg = hexrgb(stage) + (hexrgb(lit) - hexrgb(stage)) * glow
# The scroll's shadow on it, down and to the right of the light.
sh = Image.fromarray((a[..., 0] * 255).astype(np.uint8)).filter(ImageFilter.GaussianBlur(W * 0.018))
sh = np.roll(np.roll(np.asarray(sh, dtype=np.float32) / 255, int(H * 0.008), 0), int(W * 0.012), 1)[..., None]
bg = bg * (1 - 0.55 * sh)
res = pic * a + bg * (1 - a)
# The picture's last pixels are the stage's own, exactly.
edge = np.minimum.reduce([xx, W - 1 - xx, yy, H - 1 - yy])[..., None]
res = np.where(edge < 2, hexrgb(stage), res)
Image.fromarray(np.clip(res, 0, 255).astype(np.uint8)).save(out, quality=86, optimize=True, progressive=True, subsampling=2)
print(os.path.basename(out), W, "x", H, os.path.getsize(out) // 1024, "KiB")
