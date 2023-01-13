import ssl

import certifi
import geopy
from geopy import Nominatim


# TODO: Port to Rust


class Status:
    OK = 0
    SERVER_ERROR = 1
    INVALID_API_KEY = 2


class WeatherForecast:
    def __init__(
        self,
        status: int,
        region: str,
        country,
        forecast: list,
        forecast_sentence: str,
        raw_data,
    ):
        self.status = status
        self.region = region
        self.country = country
        self.forecast = forecast
        self.current_weather = forecast[0]
        self.forecast_sentence = forecast_sentence
        self.raw_data = raw_data

    def get_location(self, loc):
        ctx = ssl.create_default_context(cafile=certifi.where())
        geopy.geocoders.options.default_ssl_context = ctx
        geolocator = Nominatim(user_agent="weathercli/0", scheme="http")
        location = geolocator.reverse(loc[0] + ", " + loc[1])
        return location
