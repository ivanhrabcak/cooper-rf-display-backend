from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from fastapi_utils.tasks import repeat_every

import logging
import schedule
import threading

from .sensor.dongle import Dongle
from .sensor.data_collection import Storage
from .routers import edupage, hardwario

app = FastAPI()
logger = logging.Logger(__name__)

app.add_middleware(
    CORSMiddleware, 
    allow_origins=["*"], 
    allow_methods=["*"],
    allow_headers=["*"],
    allow_credentials=True
)

app.include_router(edupage.router)
app.include_router(hardwario.router)

def collect_data(dongle: Dongle = Dongle("COM3")):
    if not dongle.is_initialized:
        dongle.init()

    storage = Storage("./data")
    
    if dongle.serial_port.in_waiting == 0:
        return

    reading = dongle.wait_for_reading()
    if reading is None:
        return

    storage.save_reading(reading)

@app.on_event("startup")
def start_data_collection():
    schedule.every(5).seconds.do(collect_data)
    
    def run_scheduled_jobs_forever():
        import time
        while True:
            schedule.run_pending()
            time.sleep(1)
    
    threading.Thread(target=run_scheduled_jobs_forever).start()
