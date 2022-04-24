import requests

class Netatmo:
    @staticmethod
    def __simplify_data(reading: dict) -> dict:
        return {
            "timestamp": reading["time_utc"],
            "temperature": reading["Temperature"],
            "pressure": reading["Pressure"],
            "co2": reading["CO2"],
            "humidity": reading["Humidity"],
            "noise": reading["Noise"]
        }

    @staticmethod
    def fetch_data(device_id: str, client_id: str, token: str) -> dict | None:
        request_data = {
            "devices_type" : [
                "NHC"
            ],
            "app_identifier" : "app_homecoach",
            "data_amount" : "app"
        }

        response = requests.post(
            url="https://app.netatmo.net/api/getstationsdata?", 
            headers={ "Authorization": f"Bearer {client_id}|{token}" },
            json=request_data
        )

        devices = response.json()["body"]["devices"]

        for device in devices:
            if device["_id"] == device_id:
                data = Netatmo.__simplify_data(device["dashboard_data"])
                data["id"] = device_id.replace(":", "")
                
                return data

        return None