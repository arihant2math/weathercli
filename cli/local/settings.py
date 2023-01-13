from typing import Any

from cli.local.weather_file import WeatherFile


def store_key(key_name: str, value):
    f = WeatherFile("settings.json")
    f.data[key_name] = value
    f.write()


def get_key(key: str, default=None) -> Any:
    f = WeatherFile("settings.json")
    if key in f.data:
        return f.data[key]
    else:
        if default is not None:
            store_key(key, default)
        return default


def get_key_fast(data, key: str, default=None) -> Any:
    if key in data:
        return data[key]
    else:
        if default is not None:
            store_key(key, default)
        return default


f = WeatherFile("settings.json")
data = f.data
OPEN_WEATHER_MAP_API_KEY = get_key_fast(data, "OPEN_WEATHER_MAP_API_KEY")
BING_MAPS_API_KEY = get_key_fast(data, "BING_MAPS_API_KEY")
NCDC_API_KEY = get_key_fast(data, "NCDC_API_KEY")
METRIC_DEFAULT = get_key_fast(data, "METRIC", False)
WEATHER_DATA_HASH = get_key_fast(data, "WEATHER_DATA_HASH")
DEFAULT_BACKEND = get_key_fast(data, "DEFAULT_BACKEND", "METEO")
CONSTANT_LOCATION = get_key_fast(data, "CONSTANT_LOCATION", False)
DEFAULT_LAYOUT = get_key_fast(data, "DEFAULT_LAYOUT")
if type(DEFAULT_BACKEND) != str:
    print("Invalid Default Backend, defaulting")
    DEFAULT_BACKEND = "METEO"
else:
    DEFAULT_BACKEND = DEFAULT_BACKEND.upper()
