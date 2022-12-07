import asyncio
import platform

from click import group, option, pass_context, argument
import colorama

import aiohttp
import api_keys
from custom_multi_command import CustomMultiCommand

from location import get_device_location, get_coordinates
from openweathermap_weather_data import OpenWeatherMapWeatherData

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


def weather(location, metric):
    out = api_keys.OPEN_WEATHER_MAP_API_URL + "weather?lat=" + str(location[0]) + "&lon=" + str(
        location[1]) + "&appid=" + api_keys.OPEN_WEATHER_MAP_API_KEY
    if not metric:
        return out + "&units=imperial"
    else:
        return out + "&units=metric"


def air_quality(location, metric):
    return (
            api_keys.OPEN_WEATHER_MAP_API_URL + "air_pollution?lat=" + str(location[0]) + "&lon=" + str(location[1]) +
            "&appid=" + api_keys.OPEN_WEATHER_MAP_API_KEY
            + "&units=imperial")


def forecast(location, metric):
    out = api_keys.OPEN_WEATHER_MAP_API_URL + "forecast?lat=" + str(location[0]) + "&lon=" + str(location[1]) + "&appid=" + api_keys.OPEN_WEATHER_MAP_API_KEY
    if not metric:
        return out + "&units=imperial"
    else:
        return out + "&units=metric"


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


def condition_sentence(data: list[dict]) -> str:
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
    LIGHTMAGENTA_EX = ""
    MAGENTA = ""
    YELLOW = ""


def forecast_sentence(data):
    forecast_data = data
    if forecast_data[0]['weather'][0]['main'] == "Rain":
        return "It will rain in the next 3 hours"
    forecast_data.pop(0)
    while len(forecast_data) != 0:
        if forecast_data[0]['weather'][0]['main']:
            return "It will rain in the next " + str((len(data)-len(forecast_data) + 1)*3) + "-" +\
                str((len(data)-len(forecast_data) + 2)*3) + " hours"
        forecast_data.pop(0)
    return "It is not predicted to rain for the next 2 days"


def print_out(raw_data, data, json, no_color, celsius):
    if not no_color:
        Fore = colorama.Fore
    else:
        Fore = DummyFore()
    if json:
        raw_data["forecast"] = "unavailable because of length"
        print(raw_data)
    elif data.status:
        print(Fore.BLUE + "Weather for " + Fore.GREEN + data.region + ", " + data.country)
        print(Fore.LIGHTMAGENTA_EX + condition_sentence(raw_data['weather']))
        print(Fore.LIGHTMAGENTA_EX + forecast_sentence(raw_data['forecast']))
        print(Fore.BLUE + "Temperature: " + Fore.GREEN + str(data.temperature) + Fore.MAGENTA, end="° ")
        if celsius:
            print("C")
        else:
            print("F")
        print(Fore.BLUE + "Forecast (3h intervals): " + Fore.GREEN, end="")
        forecast_temps = raw_data["forecast"]
        while len(forecast_temps) > 8:
            forecast_temps.pop()
        for temp in forecast_temps:
            print(str(int(temp['main']['temp']//1)), end=" ")
        print("")
        print(Fore.BLUE + "Wind: " + Fore.GREEN + str(
            data.wind.speed) + Fore.MAGENTA, end=" ")
        if celsius:
            print("km/h", end=" ")
        else:
            print("mph", end=" ")
        print(Fore.BLUE + "at " + Fore.GREEN +
              str(data.wind.heading) + Fore.MAGENTA + " deg")
        if data.cloud_cover != 0:
            print(Fore.BLUE + "Cloud Cover: " + Fore.GREEN + str(data.cloud_cover) + Fore.MAGENTA + "%")
        aqi = data.aqi
        aqi_color = Fore.YELLOW
        if aqi == 5:
            aqi_color = Fore.RED
        elif aqi < 3:
            aqi_color = Fore.GREEN
        print(Fore.BLUE + "AQI: " + aqi_color + str(aqi))
    else:
        print(Fore.RED + raw_data["message"])


async def get_combined_data(coordinates, celsius) -> dict:
    to_get = [weather(coordinates, celsius), air_quality(coordinates, celsius), forecast(coordinates, celsius)]
    async with aiohttp.ClientSession() as session:
        responses = await fetch_all(session, to_get)
        data = responses[0]
        data["air_quality"] = responses[1]['list'][0]
        data["forecast"] = responses[2]['list']
    return data


@group(invoke_without_command=True, cls=CustomMultiCommand)
@option('-j', '--json', is_flag=True, help='If used the raw json will be printed out')
@option('-n', '--no-color', is_flag=True, help='This will not use color when printing the data out')
@option('--no-sys-loc', is_flag=True, help='If used the location will be gotten from the web rather than the system'
                                           'even if system location is available')
@option('--metric', is_flag=True, help='This will switch the output to metric')
@pass_context
def main(ctx, json, no_color, no_sys_loc, metric):
    if ctx.invoked_subcommand is None:
        raw_data = asyncio.run(get_combined_data(get_device_location(no_sys_loc), metric))
        data = OpenWeatherMapWeatherData(raw_data)
        print_out(raw_data, data, json, no_color, metric)
    else:
        ctx.ensure_object(dict)
        ctx.obj["JSON"] = json
        ctx.obj["NO_COLOR"] = no_color
        ctx.obj["METRIC"] = metric


@main.command(['place', 'p', 'c'])
@argument("location")
@option('-j', '--json', is_flag=True, help='If used the raw json will be printed out')
@option('-n', '--no-color', is_flag=True, help='This will not use color when printing the data out')
@option('--metric', is_flag=True, help='This will switch the output to metric')
@pass_context
def place(ctx, location, json, no_color, metric):
    raw_data = asyncio.run(get_combined_data(get_coordinates(location), metric))
    data = OpenWeatherMapWeatherData(raw_data)
    print_out(raw_data, data, ctx.obj["JSON"] or json, ctx.obj["NO_COLOR"] or no_color, ctx.obj["METRIC"] or metric)


if __name__ == '__main__':
    main(obj={})
