from dataclasses import dataclass
from datetime import datetime, timedelta
import requests
from ..exceptions import NetatmoAuthenticationException

@dataclass
class TokenSet:
    access_token: str
    refresh_token: str
    expires_in: int
    acquired: datetime

    def is_expired(self):
        if self.acquired == None:
            return False
        
        return datetime.now() > (self.acquired + timedelta(seconds=self.expires_in))

    def empty():
        return TokenSet(
            None, None, None, None
        )
    
    def is_empty(self):
        return self.access_token == None

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
    def login(client_secret: str, username: str, password: str) -> TokenSet:
        url = "https://app.netatmo.net/oauth2/token"
        body = {
            "client_secret": client_secret,
            "client_id": "na_client_ios",
            "username": username,
            "password": password,
            "grant_type": "password"
        }

        headers = {
            "Content-Type": "application/x-www-form-urlencoded",
            "User-Agent": "HomeCoach/1.5.2 (com.netatmo.homecoach; build:114; iOS 14.3.0) Alamofire/4.9.1"
        }

        request = requests.post(url, body, headers=headers)
        response = request.json()

        error = response.get("error")
        if error:
            raise NetatmoAuthenticationException(f"Failed to authenticate with password: {error}")

        token_set = TokenSet(
            response["access_token"],
            response["refresh_token"],
            response["expires_in"],
            datetime.now()
        )
        
        return token_set


    @staticmethod
    def get_access_token(client_secret: str, refresh_token: str) -> TokenSet:
        url = "https://app.netatmo.net/oauth2/token"
        body = {
            "client_secret": client_secret,
            "refresh_token": refresh_token,
            "grant_type": "refresh_token",
            "client_id": "na_client_ios"
        }

        headers = {
            "User-Agent": "HomeCoach/1.5.2 (com.netatmo.homecoach; build:114; iOS 14.3.0) Alamofire/4.9.1"
        }

        request = requests.post(url, body, headers=headers)        
        response = request.json()

        error = response.get("error")
        if error:
            raise NetatmoAuthenticationException(f"Failed to authenticate with refresh_token: {error}")

        token_set = TokenSet(
            response["access_token"], 
            response["refresh_token"], 
            response["expires_in"],
            datetime.now()
        )

        return token_set

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