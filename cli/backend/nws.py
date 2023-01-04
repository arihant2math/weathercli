"""Alternative Weather Backend"""
import json
import ssl

import certifi
import core
import geopy
from core import WindData
from geopy import Nominatim

from cli import WeatherData, print_out


class NationalWeatherService(WeatherData):
    def __init__(self, loc):
        ctx = ssl.create_default_context(cafile=certifi.where())
        geopy.geocoders.options.default_ssl_context = ctx
        geolocator = Nominatim(user_agent="weathercli/0", scheme="http")
        location = geolocator.reverse(loc[0] + ", " + loc[1])
        country = location.raw["address"]["country"]
        region = location.raw["address"]["city"]
        get_point = core.networking.get_url(
            "https://api.weather.gov/points/" + loc[0] + "," + loc[1]
        )
        point_json = json.loads(get_point)
        office = point_json["properties"]["cwa"]
        grid_location = [
            point_json["properties"]["gridX"],
            point_json["properties"]["gridY"],
        ]
        forecast = core.networking.get_url(
            "https://api.weather.gov/gridpoints/{}/{},{}/forecast".format(
                office, grid_location[0], grid_location[1]
            )
        )
        forecast_json = json.loads(forecast)
        if "status" in forecast_json:
            status = str(forecast_json["status"])
        else:
            status = "200"
        now = forecast_json["properties"]["periods"][0]
        wind_speed = now["windSpeed"]
        wind_direction = now["windDirection"]
        compass = {
            "N": 0,
            "NE": 45,
            "E": 90,
            "SE": 125,
            "S": 180,
            "SW": 225,
            "W": 270,
            "NW": 315,
        }  # TODO: Add support for finer directions like SSW
        heading = 0
        for key in compass:
            if key in wind_direction:
                heading = compass[key]
        speed = ""
        for i in wind_speed:
            if i.isdigit():
                speed += i
        wind = WindData(int(speed), heading)
        super().__init__(
            status=status,
            temperature=now["temperature"],
            min_temp=0,
            max_temp=0,
            region=region,
            wind=wind,
            raw_data=forecast_json,
            aqi=1,
            forecast=[],
            country=country,
            cloud_cover=0,
            conditions=[],
            condition_sentence=now["detailedForecast"],
            forecast_sentence="WIP",
        )
