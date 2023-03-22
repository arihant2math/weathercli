"""Alternative Weather Backend"""
import time
from datetime import datetime

from weather_core.backend import WeatherCondition
from weather_core.backend import WeatherData
from weather_core.backend import WindData


class MeteoFuture(WeatherData):
    def __new__(cls, *args, **kwargs):
        return super().__new__(cls)

    def __init__(self, forecast_json, aqi_json, metric, index):
        self.index = index
        min_temp = forecast_json["daily"]["temperature_2m_min"][self.index // 24]
        max_temp = forecast_json["daily"]["temperature_2m_max"][self.index // 24]
        wind = WindData(
            forecast_json["current_weather"]["windspeed"],
            int(forecast_json["current_weather"]["winddirection"]),
        )
        cloud_cover = forecast_json["hourly"]["cloudcover"][self.index]
        aqi = 0
        if (
            len(aqi_json["hourly"]["european_aqi"]) > self.index
            and aqi_json["hourly"]["european_aqi"][self.index] is not None
        ):
            aqi = aqi_json["hourly"]["european_aqi"][self.index] // 20
        super().__init__()
        # self.time = int(time.mktime(datetime.strptime(forecast_json["hourly"]["time"][self.index], '%y-%m-%dT%H:%M').timetuple()))
        self.time = int(time.mktime(datetime.now().timetuple()))
        self.temperature = forecast_json["current_weather"]["temperature"]
        self.min_temp = min_temp
        self.max_temp = max_temp
        self.wind = wind
        self.dewpoint = forecast_json["hourly"]["dewpoint_2m"][self.index]
        self.feels_like = forecast_json["hourly"]["apparent_temperature"][self.index]
        self.aqi = aqi
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
        if (0 < forecast_json["hourly"]["rain"][self.index] < 0.098 and not metric) or (
            0 < forecast_json["hourly"]["rain"][self.index] < 2.5 and metric
        ):
            conditions.append(WeatherCondition(500))
        elif (forecast_json["hourly"]["rain"][self.index] < 0.39 and not metric) or (
            forecast_json["hourly"]["rain"][self.index] < 10 and metric
        ):
            conditions.append(WeatherCondition(501))
        elif (forecast_json["hourly"]["rain"][self.index] < 2 and not metric) or (
            forecast_json["hourly"]["rain"][self.index] < 50 and metric
        ):
            conditions.append(WeatherCondition(502))
        else:
            conditions.append(WeatherCondition(503))
        if forecast_json["hourly"]["snowfall"][self.index] != 0:
            conditions.append(WeatherCondition(601))
        self.conditions = conditions
        self.condition_ids = self.get_condition_ids()
        self.condition_sentence = self.get_conditions_sentence()
