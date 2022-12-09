import asyncio
import platform

import colorama

import aiohttp
import core

from cli.api import weather, air_quality, forecast
from cli.settings import OPEN_WEATHER_MAP_API_URL, OPEN_WEATHER_MAP_API_KEY
from cli.custom_multi_command import CustomMultiCommand

from cli.location import get_device_location, get_coordinates
from cli.openweathermap_weather_data import OpenWeatherMapWeatherData

if platform.system() == "Windows":
    pass


async def fetch(session, url):
    async with session.get(url) as response:
        if response.status != 200:
            response.raise_for_status()
        return await response.json()


async def fetch_all(session, urls):
    tasks = []
    for url in urls:
        task = asyncio.create_task(fetch(session, url))
        tasks.append(task)
    results = await asyncio.gather(*tasks)
    return results


def get_description(condition_id: int) -> (str, int):
    if condition_id // 100 == 2:
        return "There is a thunderstorm", True
    elif condition_id // 100 == 3:
        return "It is drizzling", True
    elif condition_id // 100 == 5:
        return "It is raining", True
    elif condition_id == 615:
        return "There is light rain at it is snowing", True
    elif condition_id == 616:
        return "It is raining and snowing", True
    elif condition_id // 100 == 6:
        return "It is snowing", True
    condition = {701: "misty",
                 711: "smokey",
                 721: "There is a haze",
                 731: "It is dusty, expect to see dust whirls",
                 741: "foggy",
                 751: "sandy",
                 761: "dusty",
                 762: "There is ash in the air",
                 771: "There are squalls",
                 781: "There is a Tornado nearby",
                 800: "clear",
                 801: "slightly cloudy",
                 802: "moderately cloudy",
                 803: "very cloudy",
                 804: "overcast"}
    condition_intro_text_override = [721, 731, 762, 771, 781]
    return condition[condition_id], condition_id in condition_intro_text_override


def condition_sentence(data: list) -> str:
    condition_match, override_previous = get_description(data[0]["id"])
    if override_previous:
        out = condition_match
    else:
        out = "Conditions are " + condition_match
    data.pop(0)
    for condition in data:
        if override_previous:
            out += ". Also, "
        condition_match, override_previous = get_description(condition["id"])
        if override_previous:
            out += condition_match.lower()
        else:
            out += " and " + condition_match
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
    to_get = core.get_urls(OPEN_WEATHER_MAP_API_URL, OPEN_WEATHER_MAP_API_KEY, str(coordinates[0]) + ',' + str(coordinates[1]), metric)
    async with aiohttp.ClientSession() as session:
        responses = await fetch_all(session, to_get)
        data = responses[0]
        data["air_quality"] = responses[1]['list'][0]
        data["forecast"] = responses[2]['list']
    return data
