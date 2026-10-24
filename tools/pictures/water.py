"""Where a picture's water is (docs/adr/0004 §12): its ponds by their
turquoise, its streams by their white foam inside a box, as a soft mask;
written with the depth map into one picture, depth in red and water in
green, which river-scene.js reads.

usage: water.py PICTURE DEPTH.jpg OUT.jpg [BOX x0,x1,y0 as shares]

    water.py ground.png depth.jpg river-ground-maps.jpg 0.6,0.745,0.15
"""
import sys

import numpy as np
from PIL import Image, ImageFilter

src, depth, out = sys.argv[1], sys.argv[2], sys.argv[3]
bx = [float(v) for v in (sys.argv[4] if len(sys.argv) > 4 else "0.6,0.745,0.15").split(",")]
im = Image.open(src).convert("RGB")
W, H = im.size
hsv = np.asarray(im.convert("HSV"), dtype=np.float32)
h, s, v = hsv[..., 0] * 360 / 255, hsv[..., 1] / 255, hsv[..., 2] / 255
rgb = np.asarray(im, dtype=np.float32)
pond = (h > 160) & (h < 205) & (s > 0.28) & (v > 0.28) & (rgb[..., 2] > rgb[..., 0] + 25)
yy, xx = np.mgrid[0:H, 0:W]
box = (xx > W * bx[0]) & (xx < W * bx[1]) & (yy > H * bx[2])
foam = box & (s < 0.32) & (v > 0.5) & (np.abs(rgb[..., 0] - rgb[..., 2]) < 40)
m = Image.fromarray(((pond | foam) * 255).astype(np.uint8))
m = m.filter(ImageFilter.MedianFilter(3)).filter(ImageFilter.MaxFilter(3)).filter(ImageFilter.GaussianBlur(1.5))
w, hh = 1024, 576
d = Image.open(depth).convert("L").resize((w, hh), Image.LANCZOS)
m = m.resize((w, hh), Image.LANCZOS)
maps = np.stack([np.asarray(d), np.asarray(m), np.zeros((hh, w), np.uint8)], -1)
Image.fromarray(maps).save(out, quality=92, optimize=True, subsampling=0)
print(out, "written")
