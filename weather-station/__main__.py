from flask import Flask
from flask_restful import Api, Resource, reqparse
from flask_cors import CORS

from sensor.dongle import Dongle
from sensor.data_collection import DataCollection
from sensor.data_collection import Storage

from api.sensor_data import sensor_data_blueprint
from api.edupage_json import edupage_data_blueprint
from api.edupage_text import edupage_data_text_blueprint
from api.error_handler import errors

import signal

app = Flask(__name__)
CORS(app)

app.register_blueprint(sensor_data_blueprint)
app.register_blueprint(edupage_data_blueprint)
app.register_blueprint(edupage_data_text_blueprint)
app.register_blueprint(errors)


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
