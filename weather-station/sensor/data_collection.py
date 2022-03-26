from threading import Thread
from sensor.dongle import Dongle
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

class DataCollection(Thread):
    def __init__(self, dongle: Dongle):
        self.dongle = dongle

        self.storage = Storage(".\\data")

        self.should_exit = False

        super().__init__()

    def run(self):
        while not self.should_exit:
            reading = self.dongle.wait_for_reading()
            if reading is None:
                continue

            self.storage.save_reading(reading)

