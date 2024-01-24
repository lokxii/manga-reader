from flask import Flask
from flask import send_file
from flask_cors import CORS

from app.service.files import list_dir
from app.service.files import dir_first_file


app = Flask(__name__)
CORS(app)


@app.get("/books")
def get_books():
    return list_dir()


@app.get("/books/<string:name>/thumbnail")
def get_book_thumbnail(name):
    return send_file(*dir_first_file(name))
