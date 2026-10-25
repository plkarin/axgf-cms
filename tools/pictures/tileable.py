"""Make a top-down water picture tile along its length (docs/adr/0004 §15):
its last rows crossfaded into its first, so that the river-scene can repeat
it down a watercourse without a seam, then sized to a power of two for the
graphics card.

usage: tileable.py SOURCE OUT.jpg [WIDTH HEIGHT] [OVERLAP_SHARE]

    tileable.py stream.png river-water-stream.jpg 512 1024 0.18
"""
import sys

import numpy as np
from PIL import Image

src, out = sys.argv[1], sys.argv[2]
W = int(sys.argv[3]) if len(sys.argv) > 3 else 512
H = int(sys.argv[4]) if len(sys.argv) > 4 else 1024
share = float(sys.argv[5]) if len(sys.argv) > 5 else 0.18
a = np.asarray(Image.open(src).convert("RGB"), dtype=np.float32)
h = a.shape[0]
k = int(h * share)
# The bottom k rows fade into the top k rows; the top k rows are then
# dropped, so the picture's end meets its beginning.
ramp = np.linspace(0, 1, k, dtype=np.float32)[:, None, None]
blend = a[h - k:] * (1 - ramp) + a[:k] * ramp
t = np.concatenate([a[k:h - k], blend], axis=0)
img = Image.fromarray(np.clip(t, 0, 255).astype(np.uint8)).resize((W, H), Image.LANCZOS)
img.save(out, quality=86, optimize=True, progressive=True)
print(out, img.size)
