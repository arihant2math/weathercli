import json

import colorama
import core
import requests
import rich
from core import WeatherFile
from core import hash_file, networking
from core.backend import WeatherForecast

from cli.layout.layout import Layout
from cli.local.settings import store_key, WEATHER_DATA_HASH, LAYOUT_FILE


def update_weather_codes():
    f = WeatherFile("weather_codes.json")
    file_hash = hash_file(f.path)
    try:
        web_hash = requests.get(
            "https://arihant2math.github.io/weathercli/docs/index.json"
        ).json()["weather-codes-hash"]
    except Exception:
        web_hash = file_hash
    if (WEATHER_DATA_HASH != file_hash) or (web_hash != WEATHER_DATA_HASH):
        print("Downloading weather_codes.json update")
        data = networking.get_url(
            "https://arihant2math.github.io/weathercli/weather_codes.json"
        )
        f.data = data
        f.write()
        new_file_hash = hash_file(f.path)
        store_key("WEATHER_DATA_HASH", new_file_hash)


def print_out(data: WeatherForecast, print_json: bool, metric: bool):
    color = colorama.Fore
    if print_json:
        try:
            if isinstance(data.raw_data, list):
                for i in data.raw_data:
                    print("============================================================")
                    rich.print_json(json.dumps(i))
            elif isinstance(data.raw_data, str):
                rich.print_json(data.raw_data)
            elif isinstance(data.raw_data, dict):
                rich.print_json(json.dumps(data.raw_data))
            elif isinstance(data.raw_data, core.FormattedData):
                rich.print_json(data.raw_data.raw_data[0])
                rich.print_json(data.raw_data.raw_data[1])
                rich.print_json(data.raw_data.raw_data[2])
            else:
                print(type(data.raw_data))
        except Exception as e:
            print(e)
            print(data.raw_data)
    elif data.status == 0:
        out = Layout(LAYOUT_FILE)
        print(out.to_string(data, metric))
    else:
        print(color.RED + data.raw_data["message"] + color.RESET, end="")
