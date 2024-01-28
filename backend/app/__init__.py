from flask import Flask
from flask import send_file
from flask_cors import CORS

from app.service.files import dir_entry
from app.service.files import dir_thumbnail
from app.service.files import list_dir


app = Flask(__name__)
CORS(app)


@app.get("/books")
def get_books():
    d = list_dir()
    try:
        d.remove(".DS_Store")
    except ValueError:
        pass
    return d


@app.get("/books/<string:name>/thumbnail")
def get_book_thumbnail(name):
    if name not in list_dir():
        return "Book not found", 404

    return send_file(*dir_thumbnail(name))


@app.get("/books/<string:name>/pages")
def get_book_pages(name):
    if name not in list_dir():
        return "Book not found", 404

    dirs = list_dir(name)
    dirs.sort()
    try:
        dirs.remove(".DS_Store")
    except ValueError:
        pass
    return dirs


@app.get("/books/<string:name>/pages/<string:page>")
def get_book_one_page(name, page):
    if name not in list_dir():
        return "Book not found", 404
    if page not in list_dir(name):
        return "Page not found", 404

    return send_file(*dir_entry(name, page))
