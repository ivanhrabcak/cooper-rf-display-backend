from datetime import datetime
from datetime import date
from flask import Blueprint
from functools import reduce
from typing import Callable

import json

from sensor.data_collection import Storage

sensor_data_blueprint = Blueprint("sensor_data", __name__)

class SensorData:
    @staticmethod
    def __serialize_date_set(dates: set[date]) -> list[str]:
        dates = list(dates)

        output = []
        for date in dates:
            output.append(date.strftime("%Y-%m-%d"))
        
        return output
    
    def __parse_date_ymd(date: str) -> date:
        try:
            return datetime.strptime(date, "%Y-%m-%d").date()
        except ValueError:
            return {"error": "Bad date format (please use %Y-%m-%d)"}

    @sensor_data_blueprint.route("/api/data/dates", methods=["GET"])
    def dates_with_data():
        measurements = Storage("./data").get_readings()

        dates_with_data = set()
        for station in measurements:
            for measurement in measurements[station]:
                measurement_date = measurement.get("timestamp")
                dates_with_data.add(measurement_date)
        
        return {"dates": SensorData.__serialize_date_set(dates_with_data)}
    
    @sensor_data_blueprint.route("/api/data/points/<date>", methods=["GET"])
    def data_points(date: str):
        date = SensorData.__parse_date_ymd(date)
        
        measurements = Storage("./data").get_readings()
        
        data = {}
        for station in measurements:
            for measurement in measurements[station]:
                if measurement["timestamp"].date() == date:
                    if data.get(station) is not None:
                        data[station] += [measurement["timestamp"]]
                    else:
                        data[station] = [measurement["timestamp"]]
        
        return data
    
    @sensor_data_blueprint.route("/api/data/readings/<date>/json")
    def readings_json(date: str):
        date = SensorData.__parse_date_ymd(date)

        measurements = Storage("./data").get_readings()
        
        data = {}
        for station in measurements:
            for measurement in measurements[station]:
                if measurement["timestamp"].date() == date:
                    if data.get(station) is not None:
                        data[station] += [measurement]
                    else:
                        data[station] = [measurement]
        
        return data
    
    @sensor_data_blueprint.route("/api/data/readings/<date>/text")
    def readings_text(date: str):
        date = SensorData.__parse_date_ymd(date)

        measurements = Storage("./data").get_readings()
        
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
        
        return data
        