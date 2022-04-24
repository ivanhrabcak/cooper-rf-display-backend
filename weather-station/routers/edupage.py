from fastapi import APIRouter
from dataclasses import asdict
from datetime import datetime

from ..config import Config
from ..util import Util


router = APIRouter(
    prefix="/api/edupage",
    tags=["data", "edupage"]
)

class EdupageData:
    @router.get("/substitution/{date}/{format}")
    @Util.multi_format
    def get_missing_teachers(date: str, format: str):
        date = Util.parse_date_ymd(date)

        config = Config.parse_config()

        edupage = Util.create_edupage(config)
        
        return edupage.get_missing_teachers(date)
    
    @router.get("/test")
    def test():
        return "Ok"
    
    @router.get("/lunch/{date}/{format}")
    @Util.multi_format
    def get_lunch_for_date(date: str, format: str):
        date = Util.parse_date_ymd(date)

        config = Config.parse_config()

        edupage = Util.create_edupage(config)
        lunch = edupage.get_lunches(date)

        if isinstance(lunch, str):
            return lunch

        return lunch.menus
    
    @router.get("/nextlesson/{format}")
    @Util.multi_format
    def get_next_lesson_time(format: str, hours: int, minutes: int):
        config = Config.parse_config()

        edupage = Util.create_edupage(config)

        now = datetime.now()        
        
        date = datetime(now.year, now.month, now.day, hours, minutes)
        next_ringing_time = edupage.get_next_ringing_time(date)
        
        return asdict(next_ringing_time)
        

        
