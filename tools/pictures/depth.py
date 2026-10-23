"""A depth map of a picture (docs/adr/0016 §3), with Depth Anything V2
small, on the CPU: near is light, far is dark. genealogy.js moves each pixel
of the tree by it as the camera moves. Needs the same environment as
upscale.py, plus `pip install transformers`; the model (safetensors) is
fetched from Hugging Face on first use.

usage: depth.py SOURCE OUT.jpg [WIDTH]

    depth.py tree.png genealogy-tree-depth.jpg 544
"""
import sys

import numpy as np
import torch
from PIL import Image, ImageFilter
from transformers import AutoImageProcessor, AutoModelForDepthEstimation

torch.set_num_threads(2)
src, out = sys.argv[1], sys.argv[2]
width = int(sys.argv[3]) if len(sys.argv) > 3 else 544
name = "depth-anything/Depth-Anything-V2-Small-hf"
proc = AutoImageProcessor.from_pretrained(name)
model = AutoModelForDepthEstimation.from_pretrained(name, use_safetensors=True).eval()
im = Image.open(src).convert("RGB")
with torch.inference_mode():
    pred = model(**proc(images=im, return_tensors="pt")).predicted_depth
    pred = torch.nn.functional.interpolate(pred.unsqueeze(1), size=im.size[::-1], mode="bicubic", align_corners=False).squeeze()
d = pred.numpy()
d = (d - d.min()) / (d.max() - d.min() + 1e-9)
# Smoothed a little, so that an edge moves as one piece, and made small: the
# shader reads it filtered.
dm = Image.fromarray((d * 255).astype(np.uint8)).filter(ImageFilter.GaussianBlur(2))
dm.resize((width, round(width * im.size[1] / im.size[0])), Image.LANCZOS).save(out, quality=90, optimize=True)
print(out, "written")
