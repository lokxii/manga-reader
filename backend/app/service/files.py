from io import BytesIO
import os
from os import path

from PIL import Image
import pillow_avif


def get_path() -> str:
    return "/home/files"


def list_dir(book: str | None = None) -> [str]:
    if book is None:
        return os.listdir(get_path())
    else:
        return os.listdir(path.join(get_path(), book))


def compress(file, thumbnail):
    img = Image.open(file).convert("RGB")
    img_io = BytesIO()
    max_width = 500 if thumbnail else 900
    quality = 40 if thumbnail else 50 if img.width > 900 else 75
    if img.width > max_width:
        ratio = max_width / img.width
        img.thumbnail((img.width * ratio, img.height*ratio), Image.LANCZOS)
    img.save(img_io, "AVIF", quality=quality)
    img_io.seek(0)
    return img_io, "image/avif"


def dir_thumbnail(dir: str) -> (str, str):
    p = path.join(get_path(), dir)
    files = os.listdir(p)
    files.sort()
    first_file = path.join(p, files[0])
    return compress(first_file, thumbnail=True)


def dir_entry(dir: str, file: str) -> (str, str):
    p = path.join(get_path(), dir, file)
    return compress(p, thumbnail=False)
