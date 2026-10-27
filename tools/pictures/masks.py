"""The river's ground maps (docs/adr/0004 §17): red, how near each pixel is
(a depth map from depth.py); green, its trees, where the wind bends them;
blue, its meadows, where the wind runs in waves and the herds graze. Trees
are the dark pixels and the larches' gold where the picture has texture;
meadows the yellow-green ones where it has little; both are smoothed, so
that an edge moves as one piece.

usage: masks.py SOURCE DEPTH.jpg OUT.jpg [WIDTH HEIGHT]

    masks.py canopee.png canopee-depth.jpg river-ground-maps.jpg 1024 576
"""
import sys

import numpy as np
from PIL import Image, ImageFilter

src, depth, out = sys.argv[1], sys.argv[2], sys.argv[3]
W = int(sys.argv[4]) if len(sys.argv) > 4 else 1024
H = int(sys.argv[5]) if len(sys.argv) > 5 else 576
im = Image.open(src).convert("RGB").resize((W, H), Image.LANCZOS)
hsv = np.asarray(im.filter(ImageFilter.GaussianBlur(1)).convert("HSV"), dtype=np.float32) / 255
hue, sat, val = hsv[..., 0] * 360, hsv[..., 1], hsv[..., 2]
grey = np.asarray(im.convert("L"), dtype=np.float32) / 255


def blur(x, r):
    return np.asarray(Image.fromarray((np.clip(x, 0, 1) * 255).astype(np.uint8)).filter(ImageFilter.GaussianBlur(r)), dtype=np.float32) / 255


def up(x, lo, hi):
    return np.clip((x - lo) / (hi - lo), 0, 1)


def band(x, a, b, c, d):
    return up(x, a, b) * (1 - up(x, c, d))


# How much the picture varies around each pixel: a forest's crowns do, a
# meadow does not.
mean = blur(grey, 3)
texture = np.sqrt(np.clip(blur(grey * grey, 3) - mean * mean, 0, None))
trees = np.maximum(1 - up(val, 0.26, 0.36), band(hue, 22, 28, 42, 48) * up(sat, 0.72, 0.82) * up(texture, 0.06, 0.1))
meadow = up(val, 0.38, 0.46) * band(sat, 0.36, 0.44, 0.74, 0.8) * band(hue, 32, 37, 75, 90) * (1 - trees)
near = np.asarray(Image.open(depth).convert("L").resize((W, H), Image.LANCZOS), dtype=np.float32) / 255
maps = np.stack([near, blur(trees, 1.2), blur(meadow, 2)], -1)
Image.fromarray((np.clip(maps, 0, 1) * 255).astype(np.uint8)).save(out, quality=92, optimize=True)
print(out, "trees", round(float(maps[..., 1].mean()), 3), "meadows", round(float(maps[..., 2].mean()), 3))
