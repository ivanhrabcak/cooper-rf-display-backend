from datetime import date

from fastapi import APIRouter
from fastapi.responses import PlainTextResponse

from ..sensor.data_collection import Storage, get_stations
from ..util import Util
from ..config import Config

router = APIRouter(
    prefix="/api/data",
    tags=["sensor_data"]
)

class SensorData:
    @staticmethod
    def __serialize_date_set(dates: set[date]) -> list[str]:
        dates = list(dates)

        output = []
        for date in dates:
            serialized_date = date.strftime("%Y-%m-%d")
            
            if serialized_date not in output:
                output.append(serialized_date)
        
        return output

    @router.get("/dates/{format}")
    @Util.multi_format
    def dates_with_data(format: str):
        config = Config.parse_config()

        measurements = Storage(config.get("data_path")).get_readings()

        dates_with_data = set()
        for station in measurements:
            for measurement in measurements[station]:
                measurement_date = measurement.get("timestamp")
                dates_with_data.add(measurement_date)
        
        return SensorData.__serialize_date_set(dates_with_data)
    
    @router.get("/points/{date}/{format}")
    @Util.multi_format
    def data_points(date: str, format: str):
        date = Util.parse_date_ymd(date)

        config = Config.parse_config()
        
        measurements = Storage(config.get("data_path")).get_readings()
        
        data = {}
        for station in measurements:
            for measurement in measurements[station]:
                if measurement["timestamp"].date() == date:
                    if data.get(station) is not None:
                        data[station] += [measurement["timestamp"].strftime("%Y-%m-%d %H:%M:%S")]
                    else:
                        data[station] = [measurement["timestamp"].strftime("%Y-%m-%d %H:%M:%S")]

        if format != "json":
            for station in data:
                data[station] = ";".join(data[station])


        return data
    
    @router.get("/readings/{date}/json")
    def readings_json(date: str):
        date = Util.parse_date_ymd(date)

        config = Config.parse_config()
        measurements = Storage(config.get("data_path")).get_readings()
        
        data = {}
        for station in measurements:
            for measurement in measurements[station]:
                if measurement["timestamp"].date() == date:
                    if data.get(station) is not None:
                        data[station] += [measurement]
                    else:
                        data[station] = [measurement]
        
        return data
    
    @router.get("/readings/{date}/text")
    def readings_text(date: str):
        date = Util.parse_date_ymd(date)

        config = Config.parse_config()
        measurements = Storage(config.get("data_path")).get_readings()
        
        data = ""
        for station in measurements:
            data += f"{station}:\n"
            for measurement in measurements[station]:
                if measurement["timestamp"].date() != date:
                    continue
                
                measurement_date = measurement["timestamp"].date().strftime("%Y-%m-%d")
                data += f"{measurement_date},"
                data += ",".join([str(x) for x in measurement.values()][:-1])
                data += "\n"
        
        return PlainTextResponse(data)

    @router.get("/stations")
    def get_stations():
        return get_stations()

        