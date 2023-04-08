import time

from weather_core import WeatherFile
from weather_core.backend import WeatherCondition, WeatherData, WindData


class OpenWeatherMapFuture(WeatherData):
    def __new__(cls, *args, **kwargs):
        return super().__new__(cls)

    def __init__(self, data):
        self.time = int(time.time())
        self.temperature = data.main.temp
        self.min_temp = data.main.temp_min
        self.max_temp = data.main.temp_max
        self.wind = WindData(data.wind.speed, data.wind.deg)
        self.dewpoint = data.main.humidity
        self.feels_like = data.main.feels_like
        self.aqi = 0
        self.cloud_cover = data.clouds.all
        conditions = []
        self.condition_sentence = ""
        weather_codes = WeatherFile.weather_codes().data
        for condition in data.weather:
            conditions.append(WeatherCondition(condition.id, weather_codes))
        self.conditions = conditions
        self.condition_sentence = self.get_conditions_sentence()
