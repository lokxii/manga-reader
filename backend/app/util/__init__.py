from io import BytesIO
import os

from PIL import Image


def get_path() -> str:
    return "/home/files"


def list_dir(p: str) -> [str]:
    d = os.listdir(p)
    try:
        d.remove(".DS_Store")
    except ValueError:
        pass
    return d


def compress(file, thumbnail):
    img = Image.open(file).convert("RGB")
    img_io = BytesIO()
    max_width = 300 if thumbnail else 900
    quality = 40 if thumbnail else 50 if img.width > 900 else 75
    if img.width > max_width:
        ratio = max_width / img.width
        img.thumbnail((img.width * ratio, img.height*ratio), Image.LANCZOS)
    img.save(img_io, "webp", quality=quality)
    img_io.seek(0)
    return img_io, "image/webp"
