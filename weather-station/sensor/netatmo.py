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
    def get_token(client_secret: str, refresh_token: str) -> str:
        url = "https://app.netatmo.net/oauth2/token"
        body = {
            "client_secret": client_secret,
            "refresh_token": refresh_token,
            "grant_type": "refresh_token",
            "client_id": "na_client_ios"
        }

        request = requests.post(url, body)        
        response = request.json()

        return response["access_token"], response["refresh_token"]

    @staticmethod
    def fetch_data(device_id: str, access_token: str) -> dict | None:
        request_data = {
            "devices_type" : [
                "NHC"
            ],
            "app_identifier" : "app_homecoach",
            "data_amount" : "app"
        }

        response = requests.post(
            url="https://app.netatmo.net/api/getstationsdata?", 
            headers={ "Authorization": f"Bearer {access_token}" },
            json=request_data
        )

        
        response = response.json()

        response_body = response.get("body")

        if not response_body:
            print(response)
        
        devices = response_body["devices"]

        for device in devices:
            if device["_id"] == device_id:
                data = Netatmo.__simplify_data(device["dashboard_data"])
                data["id"] = device_id.replace(":", "")
                
                return data

        return None