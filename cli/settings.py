from typing import Any

from cli.weather_file import WeatherFile


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


OPEN_WEATHER_MAP_API_URL = get_key(
    "OPEN_WEATHER_MAP_API_URL", "https://api.openweathermap.org/data/2.5/"
)
OPEN_WEATHER_MAP_API_KEY = get_key("OPEN_WEATHER_MAP_API_KEY")
BING_MAPS_API_KEY = get_key("BING_MAPS_API_KEY")
NO_COLOR_DEFAULT = get_key("NO_COLOR", False)
METRIC_DEFAULT = get_key("METRIC", False)
WEATHER_DATA_HASH = get_key("WEATHER_DATA_HASH")
