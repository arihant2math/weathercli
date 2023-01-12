from datetime import datetime

from cli import WeatherData
from core import WindData

from cli.backend.openweathermap.openweathermap_condition import OpenWeatherMapWeatherCondition


class OpenWeatherMapForecast(WeatherData):
    def __init__(self, data):
        d = datetime.fromtimestamp(data.dt)
        super().__init__(
            "200",
            d,
            data.main.temp,
            data.main.temp_min,
            data.main.temp_max,
            "",
            WindData(data.wind.speed, data.wind.deg),
            data,
            -1,
            [],
            "",
            data.clouds,
            [],
            "",
            "N/A",
        )
        self.condition_ids = self.get_condition_ids()
        for condition in data.weather:
            self.conditions.append(OpenWeatherMapWeatherCondition(condition))
        self.condition_sentence = self.get_condition_sentence()

    def get_condition_ids(self):
        ids = []
        for condition in self.conditions:
            ids.append(condition.condition_id)
        return ids
