from edupage_api.people import EduAccountType
from flask import Blueprint, request
from dataclasses import asdict
from datetime import datetime

import json
import csv
import io

from config import Config
from util import Util

PUBLIC_ENUMS = [EduAccountType] # enums that don't extend string

class EnumEncoder(json.JSONEncoder):
    def default(self, obj):
        if type(obj) in PUBLIC_ENUMS.values():
            return {"__enum__": str(obj)}
        return json.JSONEncoder.default(self, obj)

edupage_data_text_blueprint = Blueprint("edupage_data", __name__)

class EdupageDataText:
    def __serialize_dataclass_list(menus: list[any]) -> list[dict]:
        output = []
        for menu in menus:
            output.append(asdict(menu))
        
        return output
    
    def __serialize_to_csv(d: dict) -> str:
        with io.StringIO() as csvfile:
            writer = csv.DictWriter(csvfile, list(d.keys()))
            writer.writeheader()

            writer.writerow(d)
            
            return csvfile.getvalue()

    @edupage_data_text_blueprint.route("/api/edupage/substitution/<date>/text", methods=["GET"])
    def get_missing_teachers(date: str):
        date = Util.parse_date_ymd(date)

        config = Config.parse_config()

        edupage = Util.create_edupage(config)
        
        return "\n".join([EdupageDataText.__serialize_to_csv(asdict(x)) for x in edupage.get_missing_teachers(date)])
    
    @edupage_data_text_blueprint.route("/api/edupage/lunch/<date>/text", methods=["GET"])
    def get_lunch_for_date(date: str):
        date = Util.parse_date_ymd(date)

        print(date)

        config = Config.parse_config()

        edupage = Util.create_edupage(config)

        return EdupageDataText.__serialize_to_csv(
            "\n".join([EdupageDataText.__serialize_to_csv(asdict(x)) for x in edupage.get_lunches(date).menus])
        )
    
    @edupage_data_text_blueprint.route("/api/edupage/nextlesson/text", methods=["GET"])
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
        
        return EdupageDataText.__serialize_to_csv(asdict(next_ringing_time))
        

        
