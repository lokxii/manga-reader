import os
from os import path

from PIL import Image


def get_path() -> str:
    return "/home/files"


def list_dir() -> [str]:
    return os.listdir(get_path())


def dir_first_file(dir: str) -> (str, str):
    p = path.join(get_path(), dir)
    img = Image.open(p)
    return p, f"image/{img.format}".lower()
