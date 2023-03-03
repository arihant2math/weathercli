"""Alternative Weather Backend"""
import time
from datetime import datetime

from core.backend import WeatherCondition
from core.backend import WeatherData
from core.backend import WindData


class NationalWeatherServiceCurrent(WeatherData):
    def __new__(cls, *args, **kwargs):
        return super().__new__(cls)

    def __init__(self, data, metric):
        self.metric = metric
        wind = WindData(
            self.convert_speed(data["properties"]["windSpeed"]["values"][0]["value"]),
            data["properties"]["windDirection"]["values"][0]["value"],
        )
        # print(data["properties"]["updateTime"])
        super().__init__()
        self.time = int(time.mktime(datetime.now().timetuple()))
        self.temperature = self.convert_temp(
            data["properties"]["temperature"]["values"][0]["value"]
        )
        self.min_temp = self.convert_temp(
            data["properties"]["minTemperature"]["values"][0]["value"]
        )
        self.max_temp = self.convert_temp(
            data["properties"]["maxTemperature"]["values"][0]["value"]
        )
        self.wind = wind
        self.aqi = 0
        self.feels_like = self.convert_temp(
            data["properties"]["apparentTemperature"]["values"][0]["value"]
        )
        self.dewpoint = data["properties"]["dewpoint"]["values"][0]["value"]
        self.cloud_cover = data["properties"]["skyCover"]["values"][0]["value"]
        conditions = []
        if self.cloud_cover == 0:
            conditions.append(WeatherCondition(800))
        elif self.cloud_cover < 25:
            conditions.append(WeatherCondition(801))
        elif self.cloud_cover < 50:
            conditions.append(WeatherCondition(802))
        elif self.cloud_cover < 85:
            conditions.append(WeatherCondition(803))
        else:
            conditions.append(WeatherCondition(804))
        if (
            0
            < data["properties"]["quantitativePrecipitation"]["values"][0]["value"]
            < 0.098
            and not metric
        ) or (
            0
            < data["properties"]["quantitativePrecipitation"]["values"][0]["value"]
            < 2.5
            and metric
        ):
            conditions.append(WeatherCondition(500))
        elif (
            data["properties"]["quantitativePrecipitation"]["values"][0]["value"] < 0.39
            and not metric
        ) or (
            data["properties"]["quantitativePrecipitation"]["values"][0]["value"] < 10
            and metric
        ):
            conditions.append(WeatherCondition(501))
        elif (
            data["properties"]["quantitativePrecipitation"]["values"][0]["value"] < 2
            and not metric
        ) or (
            data["properties"]["quantitativePrecipitation"]["values"][0]["value"] < 50
            and metric
        ):
            conditions.append(WeatherCondition(502))
        else:
            conditions.append(WeatherCondition(503))
        if len(data["properties"]["snowfallAmount"]["values"]) != 0:
            if data["properties"]["snowfallAmount"]["values"][0]["value"] != 0:
                conditions.append(WeatherCondition(601))
        self.conditions = conditions
        self.condition_ids = self.get_condition_ids()
        self.condition_sentence = self.get_conditions_sentence()

    def convert_temp(self, value):
        if self.metric:
            return value
        else:
            return value * 9 / 5 + 32

    def convert_speed(self, value):
        if self.metric:
            return value
        else:
            return value / 1.609
