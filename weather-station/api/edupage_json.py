from edupage_api.people import EduAccountType
from flask import Blueprint, request
from dataclasses import asdict
from datetime import datetime

from config import Config
from util import Util

edupage_data_blueprint = Blueprint("edupage_data_text", __name__)

class EdupageData:
    def __serialize_dataclass_list(menus: list[any]) -> list[dict]:
        output = []
        for menu in menus:
            output.append(asdict(menu))
        
        return output


    @edupage_data_blueprint.route("/api/edupage/substitution/<date>", methods=["GET"])
    def get_missing_teachers(date: str):
        date = Util.parse_date_ymd(date)

        config = Config.parse_config()

        edupage = Util.create_edupage(config)
        
        return { "response": EdupageData.__serialize_dataclass_list(edupage.get_missing_teachers(date)) }
    
    @edupage_data_blueprint.route("/api/edupage/lunch/<date>", methods=["GET"])
    def get_lunch_for_date(date: str):
        date = Util.parse_date_ymd(date)

        print(date)

        config = Config.parse_config()

        edupage = Util.create_edupage(config)

        return { "response": EdupageData.__serialize_dataclass_list(
            edupage.get_lunches(date).menus
        ) }
    
    @edupage_data_blueprint.route("/api/edupage/nextlesson", methods=["GET"])
    def get_next_lesson_time():
        params = request.args

        hours = params.get("hours", type=int)
        minutes = params.get("minutes", type=int)

        if None in [hours, minutes]:
            return {"error": "Missing fields in request!"}
        

        config = Config.parse_config()

        edupage = Util.create_edupage(config)

        now = datetime.now()        
        
        date = datetime(now.year, now.month, now.day, hours, minutes)
        next_ringing_time = edupage.get_next_ringing_time(date)
        
        return asdict(next_ringing_time)
        

        
