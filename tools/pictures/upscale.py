"""Enlarge a picture four times with Real-ESRGAN x4plus, on the CPU, by tiles
(docs/adr/0017). Not part of the build: run it by hand when a view's
picture is made again.

Needs Python 3 with numpy, Pillow, torch and spandrel (a virtual
environment: `python3 -m venv venv && venv/bin/pip install spandrel`), and
the model's weights, checked before use:

    curl -sSLO https://github.com/xinntao/Real-ESRGAN/releases/download/v0.1.0/RealESRGAN_x4plus.pth
    sha256sum RealESRGAN_x4plus.pth
    # 4fa0d38905f75ac06eb49a7951b426670021be3018265fd191d2125df9d682f1

usage: upscale.py WEIGHTS SOURCE OUT [--tile 192]

SOURCE is the lossless original (Canva's PNG export at the picture's own
size), never a served JPEG. About ten to fifteen minutes on two cores.
"""
import sys
import time

import numpy as np
import torch
from PIL import Image
from spandrel import ModelLoader

torch.set_num_threads(2)
w_path, src, dst = sys.argv[1:4]
args = sys.argv[4:]
tile = int(args[args.index("--tile") + 1]) if "--tile" in args else 192
sd = torch.load(w_path, map_location="cpu", weights_only=True)
for k in ("params_ema", "params"):
    if isinstance(sd, dict) and k in sd:
        sd = sd[k]
        break
model = ModelLoader().load_from_state_dict(sd).eval()
S = model.scale
im = Image.open(src).convert("RGB")
a = torch.from_numpy(np.asarray(im).astype(np.float32) / 255).permute(2, 0, 1)[None]
_, _, H, W = a.shape
out = torch.zeros(1, 3, H * S, W * S)
pad = 16
t0 = time.time()
n = 0
with torch.inference_mode():
    for y0 in range(0, H, tile):
        for x0 in range(0, W, tile):
            y1, x1 = min(y0 + tile, H), min(x0 + tile, W)
            ya, xa = max(0, y0 - pad), max(0, x0 - pad)
            yb, xb = min(H, y1 + pad), min(W, x1 + pad)
            r = model(a[:, :, ya:yb, xa:xb])
            out[:, :, y0 * S:y1 * S, x0 * S:x1 * S] = r[:, :, (y0 - ya) * S:(y1 - ya) * S, (x0 - xa) * S:(x1 - xa) * S]
            n += 1
print(f"{n} tiles in {time.time() - t0:.0f}s, scale {S}, {W}x{H} -> {W * S}x{H * S}")
o = (out[0].clamp(0, 1).permute(1, 2, 0).numpy() * 255 + 0.5).astype(np.uint8)
Image.fromarray(o).save(dst)
