from io import BytesIO
import os
from os import path

from PIL import Image


def get_path() -> str:
    return "/home/files"


def list_dir(book: str | None = None) -> [str]:
    if book is None:
        return os.listdir(get_path())
    else:
        return os.listdir(path.join(get_path(), book))


def compress(file):
    img = Image.open(file).convert("RGB")
    img_io = BytesIO()
    img.thumbnail(tuple(map(lambda x: x * 0.5, img.size)), Image.LANCZOS)
    img.save(img_io, "JPEG", quality=50)
    img_io.seek(0)
    return img_io, "image/jpeg"


def dir_thumbnail(dir: str) -> (str, str):
    p = path.join(get_path(), dir)
    files = os.listdir(p)
    files.sort()
    first_file = path.join(p, files[0])
    return compress(first_file)


def dir_entry(dir: str, file: str) -> (str, str):
    p = path.join(get_path(), dir, file)
    return compress(p)
