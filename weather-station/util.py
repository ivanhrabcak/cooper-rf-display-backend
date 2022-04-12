from datetime import date, datetime
from edupage_api import Edupage

class DateFormatException(Exception):
    pass

class Util:
    @staticmethod
    def parse_date_ymd(date: str) -> date:
        try:
            return datetime.strptime(date, "%Y-%m-%d").date()
        except ValueError:
            raise DateFormatException("Bad date format (please use %Y-%m-%d)")
    
    @staticmethod
    def create_edupage(config: dict) -> Edupage:
        config = config["edupage"]

        edupage = Edupage()
        edupage.login(config["username"], config["password"], config["subdomain"])

        return edupage
    
    @staticmethod
    def ensure_all_fields(config: dict, structure: list[tuple]):
        for key, expected_type in structure:
            if not isinstance(config.get(key), expected_type):
                raise TypeError(f"Invalid structure: the key {key} " + \
                                f"should have the {expected_type} (not {type(config.get(key))})")