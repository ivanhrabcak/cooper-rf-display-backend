from crypt import methods
from edupage_api import Menu
from flask import Blueprint, request
from dataclasses import asdict

from config import Config
from util import Util


edupage_data_blueprint = Blueprint("edupage_data", __name__)

class EdupageData:
    def __serialize_menu_list(menus: list[Menu]) -> list[dict]:
        output = []
        for menu in menus:
            output.append(asdict(menu))
        
        return output

    @edupage_data_blueprint.route("/api/edupage/substituition/<date>", methods=["GET"])
    def get_missing_teachers(date: str):
        date = Util.parse_date_ymd(date)

        config = Config.parse_config()

        edupage = Util.create_edupage(config)
        
        return asdict(edupage.get_missing_teachers(date))
    
    @edupage_data_blueprint.route("/api/edupage/lunch/<date>", methods=["GET"])
    def get_lunch_for_date(date: str):
        date = Util.parse_date_ymd(date)

        config = Config.parse_config()

        edupage = Util.create_edupage(config)

        return EdupageData.__serialize_menu_list(
            edupage.get_lunches(date).menus
        )
    
    @edupage_data_blueprint.route("/api/edupage/nextlesson", methods=["GET"])
    def get_next_lesson_time():
        params = request.args

        hours = params.get("hours", type=int)
        minutes = params.get("minutes", type=int)

        if None in [hours, minutes]:
            return {"error": "Missing fields in request!"}
        

        config = Config.parse_config()

        edupage = Util.create_edupage(config)

        # TODO: implement ringing times API

        return "Not implemented!", 500
        

        
