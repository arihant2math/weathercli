import aiohttp
import colorama
import core

import weather_codes
from cli.custom_multi_command import CustomMultiCommand
from cli.openweathermap_weather_data import OpenWeatherMapWeatherData
from cli.settings import OPEN_WEATHER_MAP_API_URL, OPEN_WEATHER_MAP_API_KEY
from cli.url import fetch_all


def get_description(condition_id: int) -> str:
    reader = weather_codes.data
    for row in reader:
        if str(row[0]) == str(condition_id):
            return row[4]
    return "Unknown Conditions, condition id=" + str(condition_id)


def condition_sentence(data: list) -> str:
    condition_match = get_description(data[0]["id"])
    out = condition_match
    data.pop(0)
    for condition in data:
        out += ". Also, "
        condition_match = get_description(condition["id"])
        out += condition_match.lower()
    out += "."
    return out


class DummyFore:
    BLUE = ""
    GREEN = ""
    RED = ""
    LIGHTBLUE_EX = ""
    LIGHTGREEN_EX = ""
    LIGHTMAGENTA_EX = ""
    LIGHTYELLOW_EX = ""
    MAGENTA = ""
    YELLOW = ""
    RESET = ""


def forecast_sentence(data):
    forecast_data = data
    if forecast_data[0]['weather'][0]['main'] == "Rain":
        return "It will rain in the next 3 hours"
    forecast_data.pop(0)
    while len(forecast_data) != 0:
        if forecast_data[0]['weather'][0]['main']:
            return "It will rain in the next " + str((len(data) - len(forecast_data) + 1) * 3) + "-" + \
                str((len(data) - len(forecast_data) + 2) * 3) + " hours"
        forecast_data.pop(0)
    return "It is not predicted to rain for the next 2 days"


def print_out(raw_data, data, json, no_color, celsius):
    if not no_color:
        Fore = colorama.Fore
    else:
        Fore = DummyFore
    if json:
        raw_data["forecast"] = "unavailable because of length"
        print(raw_data)
    elif data.status:
        print(Fore.LIGHTBLUE_EX + "Weather for " + Fore.LIGHTGREEN_EX + data.region + ", " + data.country)
        print(Fore.LIGHTMAGENTA_EX + condition_sentence(raw_data['weather']))
        print(Fore.LIGHTMAGENTA_EX + forecast_sentence(raw_data['forecast']))
        print(Fore.LIGHTBLUE_EX + "Temperature: " + Fore.LIGHTGREEN_EX + str(data.temperature) + Fore.MAGENTA, end="° ")
        if celsius:
            print("C")
        else:
            print("F")
        print(Fore.LIGHTBLUE_EX + "Forecast (3h intervals): " + Fore.LIGHTGREEN_EX, end="")
        forecast_temps = raw_data["forecast"]
        while len(forecast_temps) > 8:
            forecast_temps.pop()
        for temp in forecast_temps:
            print(str(int(temp['main']['temp'] // 1)), end=" ")
        print("")
        print(Fore.LIGHTBLUE_EX + "Wind: " + Fore.LIGHTGREEN_EX + str(
            data.wind.speed) + Fore.MAGENTA, end=" ")
        if celsius:
            print("km/h", end=" ")
        else:
            print("mph", end=" ")
        print(Fore.LIGHTBLUE_EX + "at " + Fore.LIGHTGREEN_EX +
              str(data.wind.heading) + Fore.MAGENTA + "°")
        if data.cloud_cover != 0:
            print(Fore.LIGHTBLUE_EX + "Cloud Cover: " + Fore.LIGHTGREEN_EX + str(data.cloud_cover) + Fore.MAGENTA + "%")
        aqi = data.aqi
        aqi_color = Fore.LIGHTYELLOW_EX
        if aqi == 5:
            aqi_color = Fore.RED
        elif aqi < 3:
            aqi_color = Fore.LIGHTGREEN_EX
        print(Fore.LIGHTBLUE_EX + "AQI: " + aqi_color + str(aqi))
        print(Fore.RESET)
    else:
        print(Fore.RED + raw_data["message"] + Fore.RESET)


async def get_combined_data(coordinates, metric: bool) -> dict:
    to_get = core.get_urls(OPEN_WEATHER_MAP_API_URL, OPEN_WEATHER_MAP_API_KEY, str(coordinates[0]) + ',' +
                           str(coordinates[1]), metric)
    async with aiohttp.ClientSession() as session:
        responses = await fetch_all(session, to_get)
        data = responses[0]
        data["air_quality"] = responses[1]['list'][0]
        data["forecast"] = responses[2]['list']
    return data
