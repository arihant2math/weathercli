import json
import math

import core

from cli import WeatherForecast
from cli.backend.meteo.meteo_current import MeteoCurrent
from cli.backend.meteo.meteo_future import MeteoFuture


class MeteoForecast(WeatherForecast):
    def __init__(self, loc, metric):
        location = self.get_location(loc)
        country = location.raw["address"]["country"]
        region = location.raw["address"]["city"]
        if not metric:
            data = core.networking.get_urls(
                [
                    "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current_weather=true"
                    "&hourly=temperature_2m,rain,showers,snowfall,cloudcover,dewpoint_2m,apparent_temperature,"
                    "pressure_msl,visibility,windspeed_10m,winddirection_10m"
                    "&daily=temperature_2m_max,temperature_2m_min"
                    "&temperature_unit=fahrenheit&windspeed_unit=mph&precipitation_unit=inch"
                    "&timezone=auto".format(loc[0], loc[1]),
                    "https://air-quality-api.open-meteo.com/v1/air-quality?latitude={}&longitude={}&hourly=european_aqi"
                    "".format(loc[0], loc[1]),
                ]
            )
        else:
            data = core.networking.get_urls(
                [
                    "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current_weather=true"
                    "&hourly=temperature_2m,rain,showers,snowfall,cloudcover,dewpoint_2m,apparent_temperature,visibility"
                    ",windspeed_10m,winddirection_10m"
                    "&daily=temperature_2m_max,temperature_2m_min&timezone=auto".format(
                        loc[0], loc[1]
                    ),
                    "https://air-quality-api.open-meteo.com/v1/air-quality?latitude={}&longitude={}&hourly=european_aqi"
                    "".format(loc[0], loc[1]),
                ]
            )
        forecast_json = json.loads(data[0])
        aqi_json = json.loads(data[1])
        raw_data = [forecast_json, aqi_json]
        forecast = [MeteoCurrent(forecast_json, aqi_json, metric)]
        for i in range(forecast[0].now + 1, len(forecast_json["hourly"]["rain"])):
            forecast.append(MeteoFuture(forecast_json, aqi_json, metric, i))
        super().__init__(0, region, country, forecast, "", raw_data)
        self.forecast_sentence = self.get_forecast_sentence()

    def get_forecast_sentence(self):
        rain = [amount != 0 for amount in self.raw_data[0]["hourly"]["rain"]]
        snow = [amount != 0 for amount in self.raw_data[0]["hourly"]["snowfall"]]
        for i in range(self.forecast[0].now):
            rain.pop(0)
            snow.pop(0)
        if True in [
            condition.condition_id // 100 == 5
            for condition in self.forecast[0].conditions
        ]:
            t = 0
            for i in rain:
                if not i:
                    break
                t += 1
            return "It will continue raining for " + str(t) + " hours."
        if True in [
            condition.condition_id // 100 == 6
            for condition in self.forecast[0].conditions
        ]:
            t = 0
            for i in snow:
                if not i:
                    break
                t += 1
            return "It will continue snowing for " + str(t) + " hours."
        else:
            if True in rain:
                rain_start = rain.index(True)
            else:
                rain_start = math.inf
            if True in snow:
                snow_start = snow.index(True)
            else:
                snow_start = math.inf
            if rain_start == math.inf and snow_start == math.inf:
                return "Conditions are predicted to be clear for the next 7 days."
            rain.reverse()
            snow.reverse()
            if rain_start != math.inf:
                rain_end = rain.index(True)
            else:
                rain_end = math.inf
            if snow_start != math.inf:
                snow_end = snow.index(True)
            else:
                snow_end = math.inf
            if rain_start != math.inf:
                return (
                    "It will rain in "
                    + str(rain_start)
                    + " hours for "
                    + str(rain_end - rain_start)
                    + " hours"
                )
            if snow_start != math.inf:
                return (
                    "It will rain in "
                    + str(snow_start)
                    + " hours for "
                    + str(snow_end - snow_start)
                    + " hours"
                )
        return "Conditions are predicted to be clear for the next 7 days."
