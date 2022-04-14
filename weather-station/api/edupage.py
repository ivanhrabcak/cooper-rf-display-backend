from flask import Blueprint, request
from dataclasses import asdict
from datetime import datetime

from config import Config
from util import Util


edupage_data_blueprint = Blueprint("edupage_data_text", __name__)

class EdupageData:
    @edupage_data_blueprint.route("/api/edupage/substitution/<date>/<format>", methods=["GET"])
    @Util.multi_format
    def get_missing_teachers(date: str, format: str):
        date = Util.parse_date_ymd(date)

        config = Config.parse_config()

        edupage = Util.create_edupage(config)
        
        return edupage.get_missing_teachers(date)
    
    @edupage_data_blueprint.route("/api/edupage/lunch/<date>/<format>", methods=["GET"])
    @Util.multi_format
    def get_lunch_for_date(date: str, format: str):
        date = Util.parse_date_ymd(date)

        config = Config.parse_config()

        edupage = Util.create_edupage(config)
        lunch = edupage.get_lunches(date)

        if isinstance(lunch, str):
            return lunch

        return lunch.menus
    
    @edupage_data_blueprint.route("/api/edupage/nextlesson/<format>", methods=["GET"])
    @Util.multi_format
    def get_next_lesson_time(format: str):
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
        

        
