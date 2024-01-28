from os import path

from app.util import compress
from app.util import get_path
from app.util import list_dir


def list_book_shelf(book: str | None = None) -> [str]:
    if book is None:
        return list_dir(get_path())
    else:
        return list_dir(path.join(get_path(), book))


def dir_thumbnail(dir: str) -> (str, str):
    p = path.join(get_path(), dir)
    files = list_dir(p)
    files.sort()
    first_file = path.join(p, files[0])
    return compress(first_file, thumbnail=True)


def dir_entry(dir: str, file: str) -> (str, str):
    p = path.join(get_path(), dir, file)
    return compress(p, thumbnail=False)
