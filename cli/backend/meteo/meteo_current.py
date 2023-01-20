"""Alternative Weather Backend"""
from datetime import datetime

from core import WindData

from cli.backend.weather_condition import WeatherCondition
from cli.backend.weather_data import WeatherData


class MeteoCurrent(WeatherData):
    def __init__(self, forecast_json, aqi_json, metric):
        self.now = forecast_json["hourly"]["time"].index(
            forecast_json["current_weather"]["time"]
        )
        wind = WindData(
            forecast_json["current_weather"]["windspeed"],
            int(forecast_json["current_weather"]["winddirection"]),
        )
        cloud_cover = forecast_json["hourly"]["cloudcover"][self.now]
        time = datetime.fromisoformat(forecast_json["current_weather"]["time"])
        super().__init__(
            time=time,
            temperature=forecast_json["current_weather"]["temperature"],
            min_temp=forecast_json["daily"]["temperature_2m_min"][0],
            max_temp=forecast_json["daily"]["temperature_2m_max"][0],
            wind=wind,
            dewpoint=forecast_json["hourly"]["dewpoint_2m"][self.now],
            feels_like=forecast_json["hourly"]["apparent_temperature"][self.now],
            aqi=aqi_json["hourly"]["european_aqi"][self.now] // 20,
            cloud_cover=cloud_cover,
            conditions=[],
            condition_sentence="",
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
        if forecast_json["hourly"]["rain"][self.now] != 0:
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
            elif forecast_json["hourly"]["rain"][self.now] != 0:
                self.conditions.append(WeatherCondition(503))
        if forecast_json["hourly"]["snowfall"][self.now] != 0:
            self.conditions.append(WeatherCondition(601))
        self.condition_ids = self.get_condition_ids()
        self.condition_sentence = self.get_condition_sentence()
