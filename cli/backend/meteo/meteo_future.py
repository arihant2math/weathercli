"""Alternative Weather Backend"""
from datetime import datetime

from core import WindData

from cli.backend.weather_condition import WeatherCondition
from cli.backend.weather_data import WeatherData


class MeteoFuture(WeatherData):
    def __init__(self, forecast_json, aqi_json, metric, index):
        self.index = index
        min_temp = forecast_json["daily"]["temperature_2m_min"][self.index // 24]
        max_temp = forecast_json["daily"]["temperature_2m_max"][self.index // 24]
        wind = WindData(
            forecast_json["current_weather"]["windspeed"],
            int(forecast_json["current_weather"]["winddirection"]),
        )
        cloud_cover = forecast_json["hourly"]["cloudcover"][self.index]
        aqi = -1
        if (
            len(aqi_json["hourly"]["european_aqi"]) > self.index
            and aqi_json["hourly"]["european_aqi"][self.index] is not None
        ):
            aqi = aqi_json["hourly"]["european_aqi"][self.index] // 20
        super().__init__(
            time=datetime.now,
            temperature=forecast_json["current_weather"]["temperature"],
            min_temp=min_temp,
            max_temp=max_temp,
            wind=wind,
            dewpoint=forecast_json["hourly"]["dewpoint_2m"][self.index],
            feels_like=forecast_json["hourly"]["apparent_temperature"][self.index],
            aqi=aqi,
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
        if (0 < forecast_json["hourly"]["rain"][self.index] < 0.098 and not metric) or (
            0 < forecast_json["hourly"]["rain"][self.index] < 2.5 and metric
        ):
            self.conditions.append(WeatherCondition(500))
        elif (forecast_json["hourly"]["rain"][self.index] < 0.39 and not metric) or (
            forecast_json["hourly"]["rain"][self.index] < 10 and metric
        ):
            self.conditions.append(WeatherCondition(501))
        elif (forecast_json["hourly"]["rain"][self.index] < 2 and not metric) or (
            forecast_json["hourly"]["rain"][self.index] < 50 and metric
        ):
            self.conditions.append(WeatherCondition(502))
        else:
            self.conditions.append(WeatherCondition(503))
        if forecast_json["hourly"]["snowfall"][self.index] != 0:
            self.conditions.append(WeatherCondition(601))
        self.condition_ids = self.get_condition_ids()
        self.condition_sentence = self.get_condition_sentence()
