from flask import Flask
from flask import send_file

from app.service.files import list_dir
from app.service.files import dir_first_file


app = Flask(__name__)


@app.get("/books")
def get_books():
    r = {
        "names": list_dir()
    }
    return r


@app.get("/books/<string:name>/thumbnail")
def get_book_thumbnail(name):
    return send_file(*dir_first_file(name))
