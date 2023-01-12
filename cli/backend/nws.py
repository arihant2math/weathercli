"""Alternative Weather Backend"""
import json
from datetime import datetime

import core
from core import WindData
from cli import WeatherData
from cli.backend.weather_condition import WeatherCondition


class NationalWeatherService(WeatherData):
    def __init__(self, loc, metric):
        location = self.get_location(loc)
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
            "https://api.weather.gov/gridpoints/{}/{},{}/".format(
                office, grid_location[0], grid_location[1]
            )
        )
        forecast_json = json.loads(forecast)
        if "status" in forecast_json:
            status = str(forecast_json["status"])
        else:
            status = "200"
        wind = WindData(
            forecast_json["properties"]["windSpeed"]["values"][0]["value"],
            forecast_json["properties"]["windDirection"]["values"][0]["value"],
        )
        self.metric = metric
        super().__init__(
            status=status,
            time=datetime.now,
            temperature=self.convert_temp(
                forecast_json["properties"]["temperature"]["values"][0]["value"]
            ),
            min_temp=self.convert_temp(
                forecast_json["properties"]["minTemperature"]["values"][0]["value"]
            ),
            max_temp=self.convert_temp(
                forecast_json["properties"]["maxTemperature"]["values"][0]["value"]
            ),
            region=region,
            wind=wind,
            raw_data=forecast_json,
            aqi=-1,
            forecast=[],
            country=country,
            cloud_cover=forecast_json["properties"]["skyCover"]["values"][0]["value"],
            conditions=[],
            condition_sentence="",
            forecast_sentence="WIP",
        )
        if self.cloud_cover == 0:
            self.conditions.append(WeatherCondition(800))
        elif self.cloud_cover < 25:
            self.conditions.append(WeatherCondition(801))
        elif self.cloud_cover < 50:
            self.conditions.append(WeatherCondition(802))
        elif self.cloud_cover < 85:
            self.conditions.append(WeatherCondition(803))
        else:
            self.conditions.append(WeatherCondition(804))
        if (0 < forecast_json["properties"]["quantitativePrecipitation"]["values"][0]["value"] < 0.098 and not metric) or (
            0 < forecast_json["properties"]["quantitativePrecipitation"]["values"][0]["value"] < 2.5 and metric
        ):
            self.conditions.append(WeatherCondition(500))
        elif (forecast_json["properties"]["quantitativePrecipitation"]["values"][0]["value"] < 0.39 and not metric) or (
            forecast_json["properties"]["quantitativePrecipitation"]["values"][0]["value"] < 10 and metric
        ):
            self.conditions.append(WeatherCondition(501))
        elif (forecast_json["properties"]["quantitativePrecipitation"]["values"][0]["value"] < 2 and not metric) or (
            forecast_json["properties"]["quantitativePrecipitation"]["values"][0]["value"] < 50 and metric
        ):
            self.conditions.append(WeatherCondition(502))
        else:
            self.conditions.append(WeatherCondition(503))
        if len(forecast_json["properties"]["snowfallAmount"]["values"]) != 0:
            if forecast_json["properties"]["snowfallAmount"]["values"][0]["value"] != 0:
                self.conditions.append(WeatherCondition(601))
        self.condition_ids = self.get_condition_ids()
        self.condition_sentence = self.get_condition_sentence()

    def convert_temp(self, value):
        if self.metric:
            return value
        else:
            return value * 9 / 5 + 32
