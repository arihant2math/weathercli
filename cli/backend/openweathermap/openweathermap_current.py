import time

from weather_core.backend import WeatherCondition, WeatherData, WindData


class OpenWeatherMapCurrent(WeatherData):
    def __new__(cls, *args, **kwargs):
        return super().__new__(cls)

    def __init__(self, data):
        self.time = int(time.time())
        self.temperature = int(data.weather.main.temp)
        self.min_temp = int(data.weather.main.temp_min)
        self.max_temp = int(data.weather.main.temp_max)
        self.feels_like = int(data.weather.main.feels_like)
        self.dewpoint = data.weather.main.humidity
        self.wind = WindData(data.weather.wind.speed, data.weather.wind.deg)
        self.aqi = data.air_quality.list[0].main["aqi"]
        self.cloud_cover = data.weather.clouds.all
        conditions = []
        self.condition_sentence = ""
        self.condition_ids = self.get_condition_ids()

        for condition in data.weather.weather:
            conditions.append(WeatherCondition(condition.id))
        self.conditions = conditions
        self.condition_sentence = self.get_conditions_sentence()
