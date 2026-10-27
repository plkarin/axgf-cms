"""The life on the river's ground in one atlas (docs/adr/0004 §17): sheets of
animals cut out in Canva (a transparent PNG each, the animals well apart)
cut into their pieces, row by row and left to right, each scaled to one
height (or, for a bird seen from above, one width), and packed into a
1024-pixel-wide atlas in 256 colours. river-scene.js names each piece by
its place, in the order printed.

usage: sprites.py OUT.png SHEET.png[:width[:count]] ...

    sprites.py river-life.png deer.png ibex.png horse.png eagle.png:200:2
"""
import sys

import numpy as np
from PIL import Image
from scipy import ndimage

out, sheets = sys.argv[1], sys.argv[2:]
pieces = []
for arg in sheets:
    path, _, rest = arg.partition(":")
    wide, _, count = rest.partition(":")
    im = Image.open(path).convert("RGBA")
    alpha = np.asarray(im)[..., 3]
    mask = ndimage.binary_closing(alpha > 24, iterations=3)
    labels, n = ndimage.label(mask)
    boxes = ndimage.find_objects(labels)
    sizes = ndimage.sum(mask, labels, range(1, n + 1))
    keep = [i for i in range(n) if sizes[i] > 2000]
    # Row by row (a row is a third of a 16:9 sheet's height), then left to
    # right.
    keep.sort(key=lambda i: (round((boxes[i][0].start + boxes[i][0].stop) / 2 / (im.size[1] / 3)), boxes[i][1].start))
    if count:
        keep = keep[:int(count)]
    for i in keep:
        y, x = boxes[i]
        y0, y1 = max(0, y.start - 4), min(im.size[1], y.stop + 4)
        x0, x1 = max(0, x.start - 4), min(im.size[0], x.stop + 4)
        piece = np.asarray(im.crop((x0, y0, x1, y1))).copy()
        # Only this animal: a neighbour's edge never bleeds into its box.
        own = ndimage.binary_dilation(labels[y0:y1, x0:x1] == i + 1, iterations=3)
        piece[..., 3] = (piece[..., 3] * own).astype(np.uint8)
        p = Image.fromarray(piece)
        scale = int(wide) / p.size[0] if wide else 120 / p.size[1]
        pieces.append((path, p.resize((max(1, round(p.size[0] * scale)), max(1, round(p.size[1] * scale))), Image.LANCZOS)))
W, x, y, row = 1024, 2, 2, 0
places = []
for path, p in pieces:
    if x + p.size[0] + 2 > W:
        x, y, row = 2, y + row + 2, 0
    places.append((path, x, y) + p.size)
    x += p.size[0] + 2
    row = max(row, p.size[1])
atlas = Image.new("RGBA", (W, y + row + 2), (0, 0, 0, 0))
for (path, p), (_, px, py, pw, ph) in zip(pieces, places):
    atlas.alpha_composite(p, (px, py))
atlas.quantize(colors=256, method=Image.Quantize.FASTOCTREE, dither=Image.Dither.NONE).save(out, optimize=True)
for path, px, py, pw, ph in places:
    print(path, [px, py, pw, ph])
