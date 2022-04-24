from threading import Thread
from typing import Optional
from datetime import datetime

from .dongle import Dongle

import json
import time
import os


class Storage:
    def __init__(self, data_directory: str):
        if not os.path.exists(data_directory):
            os.makedirs(data_directory)
        
        self.data_directory = data_directory
    
    def save_reading(self, reading: dict):
        station_path = os.path.join(self.data_directory, reading['id'])

        if not os.path.exists(station_path):
            os.makedirs(station_path)
        
        utc_now = int(time.time())

        new_file_path = os.path.join(station_path, f"{utc_now}.json")
        with open(new_file_path, "w+") as f:
            f.write(json.dumps(reading))
    
    def get_readings(self) -> Optional[dict[str, list[dict]]]:
        if not os.path.exists(self.data_directory):
            return None

        readings = {}
        for station in os.listdir(self.data_directory):
            station_readings_path = os.path.join(self.data_directory, station)
            
            station_readings = []
            for reading in os.listdir(station_readings_path):
                reading_path = os.path.join(station_readings_path, reading)
                reading_timestamp = int(reading.split(".json")[0])
                
                with open(reading_path, "r") as f:
                    reading = json.loads(f.read())
                    
                    reading["timestamp"] = datetime.fromtimestamp(reading_timestamp)
                    station_readings.append(reading)
                    
            
            readings[station] = station_readings
        
        return readings



