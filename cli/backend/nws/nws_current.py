"""Alternative Weather Backend"""
from datetime import datetime

from core import WindData

from cli.backend.weather_condition import WeatherCondition
from cli.backend.weather_data import WeatherData


class NationalWeatherServiceCurrent(WeatherData):
    def __init__(self, data, metric):
        wind = WindData(
            data["properties"]["windSpeed"]["values"][0]["value"],
            data["properties"]["windDirection"]["values"][0]["value"],
        )
        self.metric = metric
        # print(data["properties"]["updateTime"])
        super().__init__(
            time=datetime.now,
            temperature=self.convert_temp(
                data["properties"]["temperature"]["values"][0]["value"]
            ),
            min_temp=self.convert_temp(
                data["properties"]["minTemperature"]["values"][0]["value"]
            ),
            max_temp=self.convert_temp(
                data["properties"]["maxTemperature"]["values"][0]["value"]
            ),
            wind=wind,
            aqi=-1,
            feels_like=data["properties"]["apparentTemperature"]["values"][0]["value"],
            dewpoint=data["properties"]["dewpoint"]["values"][0]["value"],
            cloud_cover=data["properties"]["skyCover"]["values"][0]["value"],
            conditions=[],
            condition_sentence="",
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
            self.conditions.append(WeatherCondition(500))
        elif (
            data["properties"]["quantitativePrecipitation"]["values"][0]["value"] < 0.39
            and not metric
        ) or (
            data["properties"]["quantitativePrecipitation"]["values"][0]["value"] < 10
            and metric
        ):
            self.conditions.append(WeatherCondition(501))
        elif (
            data["properties"]["quantitativePrecipitation"]["values"][0]["value"] < 2
            and not metric
        ) or (
            data["properties"]["quantitativePrecipitation"]["values"][0]["value"] < 50
            and metric
        ):
            self.conditions.append(WeatherCondition(502))
        else:
            self.conditions.append(WeatherCondition(503))
        if len(data["properties"]["snowfallAmount"]["values"]) != 0:
            if data["properties"]["snowfallAmount"]["values"][0]["value"] != 0:
                self.conditions.append(WeatherCondition(601))
        self.condition_ids = self.get_condition_ids()
        self.condition_sentence = self.get_condition_sentence()

    def convert_temp(self, value):
        if self.metric:
            return value
        else:
            return value * 9 / 5 + 32
