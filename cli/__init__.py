import colorama
import core
import requests
import rich

from cli.backend.weather_forecast import WeatherForecast
from cli.dummy_fore import DummyFore
from cli.layout import Layout
from cli.local.settings import store_key, WEATHER_DATA_HASH
from cli.local.weather_file import WeatherFile


def update_weather_codes():
    f = WeatherFile("weather_codes.json")
    file_hash = core.hash_file(str(f.path.absolute()))
    web_hash = requests.get(
        "https://arihant2math.github.io/weathercli/docs/index.json"
    ).json()["weather-codes-hash"]
    if (WEATHER_DATA_HASH != file_hash) or (web_hash != WEATHER_DATA_HASH):
        print(
            colorama.Fore.YELLOW
            + "Warning: weather_codes.json is out of date or has been modified, downloading replacement."
            + colorama.Fore.RESET
        )
        data = core.networking.get_url(
            "https://arihant2math.github.io/weathercli/weather_codes.json"
        )
        with open(f.path, "w") as out:
            out.write(data)
        new_file_hash = core.hash_file(str(f.path.absolute()))
        store_key("WEATHER_DATA_HASH", new_file_hash)
        f = WeatherFile("weather_codes.json")


def print_out(data: WeatherForecast, print_json: bool, metric: bool):
    color = colorama.Fore
    if print_json:
        try:
            rich.print_json(data.raw_data)
        except:
            print(data.raw_data)
    elif data.status == 0:
        out = Layout()
        print(out.to_string(data, metric))
    else:
        print(color.RED + data.raw_data["message"] + color.RESET, end="")
