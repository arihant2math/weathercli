"""Alternative Weather Backend"""
import json
import ssl

import certifi
import core
import geopy
from core import WindData
from geopy import Nominatim

from cli import WeatherData


class NOAA(WeatherData):
    def __init__(self, loc):
        ctx = ssl.create_default_context(cafile=certifi.where())
        geopy.geocoders.options.default_ssl_context = ctx
        geolocator = Nominatim(user_agent="weathercli/0", scheme="http")
        location = geolocator.reverse(loc[0] + ", " + loc[1])
        country = location.raw["address"]["country"]
        region = location.raw["address"]["city"]
        get_point = core.networking.get_url(
            "https://www.ncei.noaa.gov/cdo-web/api/v2/datasets"
        )
        print(get_point)
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
