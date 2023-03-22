"""Alternative Weather Backend"""
import time
from datetime import datetime

from weather_core.backend import WindData, WeatherCondition, WeatherData


class MeteoCurrent(WeatherData):
    def __new__(cls, *args, **kwargs):
        return super().__new__(cls)

    def __init__(self, forecast_json, aqi_json, metric):
        self.now = forecast_json["hourly"]["time"].index(
            forecast_json["current_weather"]["time"]
        )
        wind = WindData(
            forecast_json["current_weather"]["windspeed"],
            int(forecast_json["current_weather"]["winddirection"]),
        )
        cloud_cover = forecast_json["hourly"]["cloudcover"][self.now]
        date = datetime.fromisoformat(forecast_json["current_weather"]["time"])
        self.time = int(time.mktime(date.timetuple()))
        self.temperature = round(forecast_json["current_weather"]["temperature"], 2)
        self.min_temp = forecast_json["daily"]["temperature_2m_min"][0]
        self.max_temp = forecast_json["daily"]["temperature_2m_max"][0]
        self.wind = wind
        self.dewpoint = round(forecast_json["hourly"]["dewpoint_2m"][self.now], 2)
        self.feels_like = round(
            forecast_json["hourly"]["apparent_temperature"][self.now], 2
        )
        self.aqi = aqi_json["hourly"]["european_aqi"][self.now] // 20
        self.cloud_cover = cloud_cover
        conditions = []
        if cloud_cover == 0:
            conditions.append(WeatherCondition(800))
        elif cloud_cover < 25:
            conditions.append(WeatherCondition(801))
        elif cloud_cover < 50:
            conditions.append(WeatherCondition(802))
        elif cloud_cover < 85:
            conditions.append(WeatherCondition(803))
        else:
            conditions.append(WeatherCondition(804))
        if forecast_json["hourly"]["rain"][self.now] != 0:
            if (
                0 < forecast_json["hourly"]["rain"][self.now] < 0.098 and not metric
            ) or (0 < forecast_json["hourly"]["rain"][self.now] < 2.5 and metric):
                conditions.append(WeatherCondition(500))
            elif (forecast_json["hourly"]["rain"][self.now] < 0.39 and not metric) or (
                forecast_json["hourly"]["rain"][self.now] < 10 and metric
            ):
                conditions.append(WeatherCondition(501))
            elif (forecast_json["hourly"]["rain"][self.now] < 2 and not metric) or (
                forecast_json["hourly"]["rain"][self.now] < 50 and metric
            ):
                conditions.append(WeatherCondition(502))
            elif forecast_json["hourly"]["rain"][self.now] != 0:
                conditions.append(WeatherCondition(503))
        if forecast_json["hourly"]["snowfall"][self.now] != 0:
            conditions.append(WeatherCondition(601))
        self.conditions = conditions
        self.condition_ids = self.get_condition_ids()
        self.condition_sentence = self.get_conditions_sentence()
