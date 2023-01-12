"""Alternative Weather Backend"""
import json
import math
from datetime import datetime

import core
from core import WindData

from cli import WeatherData
from cli.backend.weather_condition import WeatherCondition


class Meteo(WeatherData):
    def __init__(self, loc, metric):
        location = self.get_location(loc)
        country = location.raw["address"]["country"]
        region = location.raw["address"]["city"]
        if not metric:
            forecast = core.networking.get_urls(
                [
                    "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current_weather=true&hourly=temperature_2m,rain,showers,"
                    "snowfall,cloudcover&temperature_unit=fahrenheit&windspeed_unit"
                    "=mph&precipitation_unit=inch&timezone=auto".format(loc[0], loc[1]),
                    "https://air-quality-api.open-meteo.com/v1/air-quality?latitude={}&longitude={}&hourly=european_aqi".format(
                        loc[0], loc[1]
                    ),
                ]
            )
        else:
            forecast = core.networking.get_urls(
                [
                    "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current_weather=true&hourly=temperature_2m,rain,"
                    "showers,snowfall,cloudcover,&timezone=auto"
                    "".format(loc[0], loc[1]),
                    "https://air-quality-api.open-meteo.com/v1/air-quality?latitude={}&longitude={}&hourly=european_aqi"
                    "".format(loc[0], loc[1]),
                ]
            )
        forecast_json = json.loads(forecast[0])
        aqi_json = json.loads(forecast[1])
        self.now = forecast_json["hourly"]["time"].index(
            forecast_json["current_weather"]["time"]
        )
        min_temp = self.get_min(forecast_json["hourly"]["temperature_2m"])
        max_temp = self.get_max(forecast_json["hourly"]["temperature_2m"])
        wind = WindData(
            forecast_json["current_weather"]["windspeed"],
            int(forecast_json["current_weather"]["winddirection"]),
        )
        cloud_cover = forecast_json["hourly"]["cloudcover"][self.now]
        super().__init__(
            status="200",
            time=datetime.now,
            temperature=forecast_json["current_weather"]["temperature"],
            min_temp=min_temp,
            max_temp=max_temp,
            region=region,
            wind=wind,
            raw_data=forecast_json,
            aqi=aqi_json["hourly"]["european_aqi"][self.now] // 20,
            forecast=[],
            country=country,
            cloud_cover=cloud_cover,
            conditions=[],
            condition_sentence="",
            forecast_sentence="",
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
        if (0 < forecast_json["hourly"]["rain"][self.now] < 0.098 and not metric) or (
            0 < forecast_json["hourly"]["rain"][self.now] < 2.5 and metric
        ):
            self.conditions.append(WeatherCondition(500))
        elif (forecast_json["hourly"]["rain"][self.now] < 0.39 and not metric) or (
            forecast_json["hourly"]["rain"][self.now] < 10 and metric
        ):
            self.conditions.append(WeatherCondition(501))
        elif (forecast_json["hourly"]["rain"][self.now] < 2 and not metric) or (
            forecast_json["hourly"]["rain"][self.now] < 50 and metric
        ):
            self.conditions.append(WeatherCondition(502))
        else:
            self.conditions.append(WeatherCondition(503))
        if forecast_json["hourly"]["snowfall"][self.now] != 0:
            self.conditions.append(WeatherCondition(601))
        self.condition_ids = self.get_condition_ids()
        self.condition_sentence = self.get_condition_sentence()
        self.forecast_sentence = self.get_forecast_sentence()

    def get_min(self, data):
        return min(data[0:24])

    def get_max(self, data):
        return max(data[0:24])

    def get_forecast_sentence(self):
        rain = [amount != 0 for amount in self.raw_data["hourly"]["rain"]]
        snow = [amount != 0 for amount in self.raw_data["hourly"]["snowfall"]]
        for i in range(self.now):
            rain.pop(0)
            snow.pop(0)
        if True in [condition // 100 == 5 for condition in self.condition_ids]:
            t = 0
            for i in rain:
                if not i:
                    break
                t += 1
            return "It will continue raining for " + str(t) + " hours."
        if True in [condition // 100 == 6 for condition in self.condition_ids]:
            t = 0
            for i in snow:
                if not i:
                    break
                t += 1
            return "It will continue snowing for " + str(t) + " hours."
        else:
            if True in rain:
                rain_start = rain.index(True)
            else:
                rain_start = math.inf
            if True in snow:
                snow_start = snow.index(True)
            else:
                snow_start = math.inf
            if rain_start == math.inf and snow_start == math.inf:
                return "Conditions are predicted to be clear for the next 7 days."
            rain.reverse()
            snow.reverse()
            if rain_start != math.inf:
                rain_end = rain.index(True)
            else:
                rain_end = math.inf
            if snow_start != math.inf:
                snow_end = snow.index(True)
            else:
                snow_end = math.inf
            if rain_start != math.inf:
                return (
                    "It will rain in "
                    + str(rain_start)
                    + " hours for "
                    + str(rain_end - rain_start)
                    + " hours"
                )
            if snow_start != math.inf:
                return (
                    "It will rain in "
                    + str(snow_start)
                    + " hours for "
                    + str(snow_end - snow_start)
                    + " hours"
                )
        return "Conditions are predicted to be clear for the next 7 days."
