import core

from cli.settings import OPEN_WEATHER_MAP_API_URL, OPEN_WEATHER_MAP_API_KEY
from cli.backend.openweathermap_conditions import OpenWeatherMapWeatherCondition
from cli.backend.weather_data import WeatherData
from core import WindData


class OpenWeatherMap(WeatherData):
    def __init__(self, coordinates, metric):
        data = core.get_combined_data_formatted(
            OPEN_WEATHER_MAP_API_URL, OPEN_WEATHER_MAP_API_KEY, coordinates, metric
        )
        super().__init__(
            status=data.weather.cod,
            temperature=data.weather.main.temp,
            min_temp=data.weather.main.temp_min,
            max_temp=data.weather.main.temp_max,
            region=data.weather.name,
            wind=WindData(data.weather.wind.speed, data.weather.wind.deg),
            raw_data=data.raw_data,
            aqi=data.air_quality.list[0].main["aqi"],
            forecast=data.forecast.list,
            country=data.weather.sys.country,
            cloud_cover=data.weather.clouds.all,
            conditions=[],
            condition_sentence="",
            forecast_sentence="",
        )
        self.condition_ids = self.get_condition_ids()
        for condition in data.weather.weather:
            self.conditions.append(OpenWeatherMapWeatherCondition(condition))
        self.condition_sentence = self.get_condition_sentence()
        self.forecast_sentence = self.get_forecast_sentence()

    def get_condition_ids(self):
        ids = []
        for condition in self.conditions:
            ids.append(condition.condition_id)
        return ids

    def get_forecast_sentence(self):
        data = self.forecast.copy()
        rain = []
        snow = []
        for period in data:
            if period.weather[0].id // 100 == 5:
                rain.append(True)
                snow.append(False)
            elif period.weather[0].id // 100 == 6:
                snow.append(True)
                rain.append(False)
            else:
                rain.append(False)
                snow.append(False)
        if self.conditions[0].condition_id // 100 == 5:
            t = 0
            for i in rain:
                if not i:
                    break
                t += 1
            return "It will continue raining for " + str(t * 3) + " hours."
        elif self.conditions[0].condition_id // 100 == 6:
            t = 0
            for i in snow:
                if not i:
                    break
                t += 1
            return "It will continue snowing for " + str(t * 3) + " hours."
        else:
            combined = zip(rain, snow)
            t = 0
            for period in combined:
                t += 1
                if period[0]:
                    return "It will rain in " + str(t * 3) + " hours"
                elif period[1]:
                    return "It will snow in " + str(t * 3) + " hours"
        return "Conditions are predicted to be clear for the next 3 days."
