import json
from json import JSONDecodeError
from typing import Any

from core import WeatherFile


def store_key(key_name: str, value):
    settings_file = WeatherFile("settings.json")
    try:
        d = json.loads(settings_file.data)
        d[key_name] = value
        settings_file.data = json.dumps(d)
        settings_file.write()
    except JSONDecodeError:
        print("Write Failed: Corrupt File")


def get_key(key: str, default=None) -> Any:
    settings_file = WeatherFile("settings.json")
    try:
        d = json.loads(settings_file.data)
        if key in d:
            return d[key]
        else:
            if default is not None:
                store_key(key, default)
            return default
    except JSONDecodeError:
        pass


def get_key_fast(settings_file_data, key: str, default=None) -> Any:
    if key in settings_file_data:
        return settings_file_data[key]
    else:
        if default is not None:
            store_key(key, default)
        return default


def delete_key(key: str):
    settings_file = WeatherFile("settings.json")
    try:
        d = json.loads(f.data)
        if key in d:
            del d[key]
        settings_file.data = json.dumps(d)
        settings_file.write()
    except JSONDecodeError:
        print("Delete Failed: Corrupt File")


f = WeatherFile("settings.json")
try:
    data = json.loads(f.data)
except JSONDecodeError:
    data = {}
OPEN_WEATHER_MAP_API_KEY = get_key_fast(data, "OPEN_WEATHER_MAP_API_KEY")
BING_MAPS_API_KEY = get_key_fast(data, "BING_MAPS_API_KEY")
NCDC_API_KEY = get_key_fast(data, "NCDC_API_KEY")
METRIC_DEFAULT = get_key_fast(data, "METRIC", False)
WEATHER_DATA_HASH = get_key_fast(data, "WEATHER_DATA_HASH")
DEFAULT_BACKEND = get_key_fast(data, "DEFAULT_BACKEND", "METEO")
CONSTANT_LOCATION = get_key_fast(data, "CONSTANT_LOCATION", False)
DEFAULT_LAYOUT = get_key_fast(data, "DEFAULT_LAYOUT")
AUTO_UPDATE_INTERNET_RESOURCES = get_key_fast(data, "AUTO_UPDATE_INTERNET_RESOURCES", True)
if type(DEFAULT_BACKEND) != str:
    print("Invalid Default Backend, defaulting to Meteo")
    DEFAULT_BACKEND = "METEO"
else:
    DEFAULT_BACKEND = DEFAULT_BACKEND.upper()
LAYOUT_FILE = get_key_fast(data, "LAYOUT_FILE", "none")
if str(LAYOUT_FILE).lower() == "none":
    LAYOUT_FILE = None
