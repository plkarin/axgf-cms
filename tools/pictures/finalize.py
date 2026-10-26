"""Bring an enlarged picture back to its source's tones and save it at the
size it is served at (docs/adr/0017): the detail of the model, the colours
of the original. The model darkens a little; this takes the broad tones
(blurred over six pixels) from the original, enlarged plainly, and the fine
detail from the model.

usage: finalize.py SOURCE ENLARGED OUT WIDTH HEIGHT [QUALITY]
"""
import os
import sys

import numpy as np
from PIL import Image, ImageFilter

Image.MAX_IMAGE_PIXELS = None
src, up, out, W, H = sys.argv[1], sys.argv[2], sys.argv[3], int(sys.argv[4]), int(sys.argv[5])
q = int(sys.argv[6]) if len(sys.argv) > 6 else 88
E = Image.open(up).convert("RGB").resize((W, H), Image.LANCZOS)
L = Image.open(src).convert("RGB").resize((W, H), Image.LANCZOS)
blur = ImageFilter.GaussianBlur(radius=6)
e, gl, ge = (np.asarray(x, dtype=np.float32) for x in (E, L.filter(blur), E.filter(blur)))
res = np.clip(e + (gl - ge), 0, 255).astype(np.uint8)
Image.fromarray(res).save(out, quality=q, optimize=True, progressive=True, subsampling=2)
print(os.path.basename(out), W, "x", H, os.path.getsize(out) // 1024, "KiB")
