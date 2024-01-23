import os
from os import path


def get_path() -> str:
    return "/home/files"


def list_dir() -> [str]:
    return os.listdir(get_path())


def dir_first_file(dir: str) -> (str, str):
    return path.join(get_path(), dir), dir
