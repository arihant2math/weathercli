from datetime import datetime

from core import WindData

from cli.backend.weather_condition import WeatherCondition
from cli.backend.weather_data import WeatherData


class OpenWeatherMapFuture(WeatherData):
    def __init__(self, data):
        d = datetime.fromtimestamp(data.dt)
        super().__init__(
            d,
            data.main.temp,
            data.main.temp_min,
            data.main.temp_max,
            WindData(data.wind.speed, data.wind.deg),
            data.main.humidity,
            data.main.feels_like,
            -1,
            data.clouds.all,
            [],
            "",
        )
        for condition in data.weather:
            self.conditions.append(WeatherCondition(condition))
        self.condition_sentence = self.get_condition_sentence()
