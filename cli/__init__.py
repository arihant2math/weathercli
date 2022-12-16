import json

import colorama
import core

from cli.custom_multi_command import CustomMultiCommand
from cli.dummy_fore import DummyFore
from cli.openweathermap_weather_data import OpenWeatherMapWeatherData
from cli.settings import OPEN_WEATHER_MAP_API_URL, OPEN_WEATHER_MAP_API_KEY
from cli.weather_data import WeatherData


def color_value(value, units=None):
    if units is None:
        return Fore.LIGHTGREEN_EX + value + Fore.LIGHTBLUE_EX
    else:
        return Fore.LIGHTGREEN_EX + value + Fore.MAGENTA + units + Fore.LIGHTBLUE_EX


def print_out(data: OpenWeatherMapWeatherData, print_json: bool, no_color: bool,
              metric: bool):
    global Fore
    if not no_color:
        Fore = colorama.Fore
    else:
        Fore = DummyFore
    if print_json:
        print(data.raw_data)
    elif data.status:
        print(Fore.LIGHTBLUE_EX + "Weather for " + color_value(data.region + ", " + data.country))
        print(Fore.LIGHTMAGENTA_EX + data.condition_sentence)
        print(Fore.LIGHTMAGENTA_EX + data.forecast_sentence)
        if metric:
            degree_ending = "° C"
        else:
            degree_ending = "° F"
        print(Fore.LIGHTBLUE_EX + "Temperature: " + color_value(str(data.temperature), degree_ending), end="")
        print(" with a min of {} and a max of {}".format(color_value(str(data.min_temp), degree_ending),
                                                         color_value(str(data.max_temp), degree_ending)))
        print(Fore.LIGHTBLUE_EX + "Forecast (3h intervals): " + Fore.LIGHTGREEN_EX, end="")
        forecast_temps = data.raw_data["forecast"]
        while len(forecast_temps) > 8:
            forecast_temps.pop()
        for temp in forecast_temps:
            print(str(int(temp['main']['temp'] // 1)), end=" ")
        print("")
        print(Fore.LIGHTBLUE_EX + "Wind: " + Fore.LIGHTGREEN_EX + str(
            data.wind.speed) + Fore.MAGENTA, end=" ")
        if metric:
            print("km/h", end=" ")
        else:
            print("mph", end=" ")
        print(Fore.LIGHTBLUE_EX + "at " + color_value(str(data.wind.heading), "°"))
        if data.cloud_cover != 0:
            print(Fore.LIGHTBLUE_EX + "Cloud Cover: " + color_value(str(data.cloud_cover), "%"))
        aqi = data.aqi
        aqi_color = Fore.LIGHTYELLOW_EX
        if aqi == 5:
            aqi_color = Fore.RED
        elif aqi < 3:
            aqi_color = Fore.LIGHTGREEN_EX
        print(Fore.LIGHTBLUE_EX + "AQI: " + aqi_color + str(aqi) + Fore.RESET)
    else:
        print(Fore.RED + data.raw_data["message"] + Fore.RESET)


def get_combined_data(coordinates, metric: bool) -> dict:
    responses = core.get_combined_data_unformatted(OPEN_WEATHER_MAP_API_URL, OPEN_WEATHER_MAP_API_KEY, coordinates,
                                                   metric)
    json_responses = [json.loads(r) for r in responses]
    data = json_responses[0]
    data["air_quality"] = json_responses[1]['list'][0]
    data["forecast"] = json_responses[2]['list']
    return data
