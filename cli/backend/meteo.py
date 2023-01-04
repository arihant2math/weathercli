"""Alternative Weather Backend"""
import json
import ssl

import certifi
import core
import geopy
from core import WindData
from geopy import Nominatim

from cli import WeatherData
from cli.backend.weather_condition import WeatherCondition


class Meteo(WeatherData):
    def __init__(self, loc, metric):
        ctx = ssl.create_default_context(cafile=certifi.where())
        geopy.geocoders.options.default_ssl_context = ctx
        geolocator = Nominatim(user_agent="weathercli/0", scheme="http")
        location = geolocator.reverse(loc[0] + ", " + loc[1])
        country = location.raw["address"]["country"]
        region = location.raw["address"]["city"]
        if not metric:
            forecast = core.networking.get_urls(
                [
                    "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&hourly=temperature_2m,rain,showers,"
                    "snowfall,cloudcover,visibility,windspeed_10m,windspeed_1000hPa,"
                    "winddirection_1000hPa&daily=winddirection_10m_dominant&temperature_unit=fahrenheit&windspeed_unit"
                    "=mph&precipitation_unit=inch&timezone=auto".format(loc[0], loc[1]),
                    "https://air-quality-api.open-meteo.com/v1/air-quality?latitude={}&longitude={}&hourly=european_aqi".format(
                        loc[0], loc[1]
                    ),
                ]
            )
        else:
            forecast = core.networking.get_urls(
                [
                    "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&hourly=temperature_2m,rain,"
                    "showers,snowfall,cloudcover,visibility,windspeed_10m,windspeed_1000hPa,"
                    "winddirection_1000hPa&daily=winddirection_10m_dominant&precipitation_unit=inch&timezone=auto"
                    "".format(loc[0], loc[1]),
                    "https://air-quality-api.open-meteo.com/v1/air-quality?latitude={}&longitude={}&hourly=european_aqi"
                    "".format(loc[0], loc[1]),
                ]
            )
        forecast_json = json.loads(forecast[0])
        aqi_json = json.loads(forecast[1])
        min_temp = self.get_min(forecast_json["hourly"]["temperature_2m"])
        max_temp = self.get_max(forecast_json["hourly"]["temperature_2m"])
        wind = WindData(
            forecast_json["hourly"]["windspeed_10m"][0],
            forecast_json["hourly"]["winddirection_1000hPa"][0],
        )
        cloud_cover = forecast_json["hourly"]["cloudcover"][0]
        super().__init__(
            status="200",
            temperature=forecast_json["hourly"]["temperature_2m"][0],
            min_temp=min_temp,
            max_temp=max_temp,
            region=region,
            wind=wind,
            raw_data=forecast_json,
            aqi=aqi_json["hourly"]["european_aqi"][0] // 20,
            forecast=[],
            country=country,
            cloud_cover=cloud_cover,
            conditions=[],
            condition_sentence="",
            forecast_sentence="WIP",
        )
        if cloud_cover == 0:
            self.conditions.append(WeatherCondition(800))
        elif cloud_cover < 25:
            self.conditions.append(WeatherCondition(801))
        elif cloud_cover < 50:
            self.conditions.append(WeatherCondition(802))
        elif cloud_cover < 85:
            self.conditions.append(WeatherCondition(803))
        else:
            self.conditions.append(WeatherCondition(804))
        if forecast_json["hourly"]["rain"][0] != 0:
            self.conditions.append(WeatherCondition(501))
        if forecast_json["hourly"]["snowfall"][0] != 0:
            self.conditions.append(WeatherCondition(601))
        self.condition_sentence = self.get_condition_sentence()

    def get_min(self, data):
        return min(data[0:24])

    def get_max(self, data):
        return max(data[0:24])
