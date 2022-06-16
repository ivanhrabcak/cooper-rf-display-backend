from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware

import schedule
import threading

from .sensor.data_collection import hardwario_collect_data, netatmo_collect_data
from .routers import edupage, sensors

app = FastAPI()

app.add_middleware(
    CORSMiddleware, 
    allow_origins=["*"], 
    allow_methods=["*"],
    allow_headers=["*"],
    allow_credentials=True
)

app.include_router(edupage.router)
app.include_router(sensors.router)

@app.on_event("startup")
def start_data_collection():
    schedule.every(5).seconds.do(hardwario_collect_data)
    # schedule.every(30).seconds.do(netatmo_collect_data)
    
    def run_scheduled_jobs_forever():
        import time
        while True:
            schedule.run_pending()
            time.sleep(1)
    
    threading.Thread(target=run_scheduled_jobs_forever).start()
