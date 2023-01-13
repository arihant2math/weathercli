from datetime import datetime

from core import WindData

from cli.backend.weather_condition import WeatherCondition
from cli.backend.weather_data import WeatherData


class OpenWeatherMapCurrent(WeatherData):
    def __init__(self, data):
        super().__init__(
            time=datetime.now,
            temperature=data.weather.main.temp,
            min_temp=data.weather.main.temp_min,
            max_temp=data.weather.main.temp_max,
            feels_like=data.weather.main.feels_like,
            dewpoint=data.weather.main.humidity,
            wind=WindData(data.weather.wind.speed, data.weather.wind.deg),
            aqi=data.air_quality.list[0].main["aqi"],
            cloud_cover=data.weather.clouds.all,
            conditions=[],
            condition_sentence="",
        )
        self.condition_ids = self.get_condition_ids()

        for condition in data.weather.weather:
            self.conditions.append(WeatherCondition(condition))
        self.condition_sentence = self.get_condition_sentence()
