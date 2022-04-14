from dataclasses import dataclass
from typing import Optional
import dacite

def parse_reading(s: str) -> Optional[dict]:
    data_format = [
        ("rssi", int),
        ("id", str),
        ("header", int),
        ("sequence", int),
        ("uptime", int),
        ("altitude", int),
        ("co2_concentration", int),
        ("humidity", float),
        ("illuminance", int),
        ("motion_count", int),
        ("orientation", int),
        ("press_count", int),
        ("pressure", int),
        ("sound_level", int),
        ("temperature", float),
        ("voc_conc", int),
        ("voltage", float)
    ]

    raw_data = s.split(",")
    data_dict = {}
    try:
        for i, (key, val_type) in enumerate(data_format):
            data_dict[key] = val_type(raw_data[i])
    except ValueError:
        return None
    
    return data_dict
