from threading import Thread
from typing import Optional
from datetime import datetime

from .hardwario import Dongle
from .netatmo import Netatmo
from ..config import Config
from ..util import Util

import json
import time
import os


class Storage:
    def __init__(self, data_directory: str):
        if not os.path.exists(data_directory):
            os.makedirs(data_directory)
        
        self.data_directory = data_directory
    
    @staticmethod
    def __clean_listdir(listdir: list[str]) -> list[str]:
        return list(filter(lambda x: x != ".DS_Store", listdir))
    
    def save_reading(self, reading: dict):
        station_path = os.path.join(self.data_directory, reading['id'])

        if not os.path.exists(station_path):
            os.makedirs(station_path)
        
        utc_now = int(time.time())

        text_directory = os.path.join(station_path, "text")
        if not os.path.exists(text_directory):
            os.makedirs(text_directory)
        
        text_file_path = os.path.join(text_directory, f"{utc_now}.csv")
        with open(text_file_path, "w+") as f:
            f.write(Util.serialize_reading(reading))

        new_file_path = os.path.join(station_path, f"{utc_now}.json")
        with open(new_file_path, "w+") as f:
            f.write(json.dumps(reading))
    
    def get_readings(self) -> Optional[dict[str, list[dict]]]:
        if not os.path.exists(self.data_directory):
            return None

        readings = {}
        for station in Storage.__clean_listdir(os.listdir(self.data_directory)):
            station_readings_path = os.path.join(self.data_directory, station)
            
            station_readings = []
            for reading in Storage.__clean_listdir(os.listdir(station_readings_path)):
                reading_path = os.path.join(station_readings_path, reading)
                reading_timestamp = int(reading.split(".json")[0])
                
                with open(reading_path, "r") as f:
                    reading = json.loads(f.read())
                    
                    reading["timestamp"] = datetime.fromtimestamp(reading_timestamp)
                    station_readings.append(reading)
                    
            
            readings[station] = station_readings
        
        return readings

def hardwario_collect_data(dongle: Dongle = Dongle(Config.parse_config()["serial_port"])):
    if not dongle.is_initialized:
        dongle.init()

    storage = Storage("./data")
    
    if dongle.serial_port.in_waiting == 0:
        return

    reading = dongle.wait_for_reading()
    if reading is None:
        return

    storage.save_reading(reading)


def netatmo_collect_data():
    config = Config.parse_config()

    storage = Storage("./data")

    netatmo_config = config["netatmo"]

    device_ids = netatmo_config["devices"]
    client_id = netatmo_config["client_id"]
    token = netatmo_config["token"]

    if not isinstance(device_ids, list):
        device_ids = [device_ids]
    
    for device in device_ids:
        reading = Netatmo.fetch_data(device, client_id, token)
        storage.save_reading(reading)
        