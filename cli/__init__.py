import colorama
from core import hash_file, networking
import requests
import rich

from cli.backend.weather_forecast import WeatherForecast
from cli.dummy_fore import DummyFore
from cli.layout.layout import Layout
from cli.local.settings import store_key, WEATHER_DATA_HASH, LAYOUT_FILE
from cli.local.weather_file import WeatherFile


def update_weather_codes():
    f = WeatherFile("weather_codes.json")
    file_hash = hash_file(str(f.path.absolute()))
    try:
        web_hash = requests.get(
            "https://arihant2math.github.io/weathercli/docs/index.json"
        ).json()["weather-codes-hash"]
    except Exception:
        web_hash = file_hash
    if (WEATHER_DATA_HASH != file_hash) or (web_hash != WEATHER_DATA_HASH):
        data = networking.get_url(
            "https://arihant2math.github.io/weathercli/weather_codes.json"
        )
        with open(f.path, "w") as out:
            out.write(data)
        new_file_hash = hash_file(str(f.path.absolute()))
        store_key("WEATHER_DATA_HASH", new_file_hash)


def print_out(data: WeatherForecast, print_json: bool, metric: bool):
    color = colorama.Fore
    if print_json:
        try:
            rich.print_json(data.raw_data)
        except Exception:
            print(data.raw_data)
    elif data.status == 0:
        out = Layout(LAYOUT_FILE)
        print(out.to_string(data, metric))
    else:
        print(color.RED + data.raw_data["message"] + color.RESET, end="")
