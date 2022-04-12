from flask import Blueprint

from util import DateFormatException

errors = Blueprint("errors", __name__)

@errors.app_errorhandler(DateFormatException)
def handle_error(error):
    message = str(error)

    return {"error": message}, 500