from flask import Flask
from flask_restful import Api, Resource, reqparse
from flask_cors import CORS

from sensor.dongle import Dongle
from sensor.data_collection import DataCollection
from sensor.data_collection import Storage

from api.sensor_data import sensor_data_blueprint

import signal

app = Flask(__name__)
CORS(app)

app.register_blueprint(sensor_data_blueprint)

api = Api(app)

if __name__ == "__main__":
    dongle = Dongle("COM3")
    dongle.init()
    
    # start data collection in the background
    data_collection_process = DataCollection(dongle)
    data_collection_process.start()

    def kill_data_collection_thread(x, y):
        data_collection_process.should_exit = True
        raise KeyboardInterrupt()

    signal.signal(signal.SIGINT, kill_data_collection_thread)

    app.run("0.0.0.0", 8080)
