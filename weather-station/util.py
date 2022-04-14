from datetime import date, datetime
from edupage_api import Edupage

from dataclasses import asdict, is_dataclass
from functools import wraps
import io, csv

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
    
    def serialize_dataclass_list(menus: list[any]) -> list[dict]:
        output = []
        for menu in menus:
            output.append(asdict(menu))
        
        return output
    
    def serialize_to_csv(d: dict) -> str:
        with io.StringIO() as csvfile:
            writer = csv.DictWriter(csvfile, list(d.keys()))
            writer.writeheader()


            writer.writerow(d)
            
            return csvfile.getvalue()

    @staticmethod
    def multi_format(method):
        @wraps(method)
        def __impl(*method_args, **method_kwargs):
            output = method(*method_args, **method_kwargs)
            
            format = method_kwargs.get("format")

            if format == "json":
                if isinstance(output, list):
                    if len(output) == 0:
                        return { "response": output }

                    if is_dataclass(output[0]):
                        return { "response": Util.serialize_dataclass_list(output) }
                    else:
                        return { "response": output }
                else:
                    return { "response": output }
            elif format == "text" or format == "csv":
                if isinstance(output, list):
                    if len(output) == 0:
                        return Util.serialize_to_csv(output)
                    
                    if is_dataclass(output[0]):
                        serialized_data = [Util.serialize_to_csv(asdict(x)) for x in output]
                    elif isinstance(output[0], str):
                        serialized_data = output
                    elif isinstance(output[0], dict):
                        serialized_data = [Util.serialize_to_csv(x) for x in output]
                    else:
                        serialized_data = [Util.serialize_to_csv(x) for x in output]

                    return "\n".join(serialized_data)
                elif isinstance(output, dict):
                    return Util.serialize_to_csv(output)
            else:
                return "Invalid Format!", 400

        
        return __impl