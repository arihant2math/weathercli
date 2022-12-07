import json
import os
import pathlib
from typing import Any


def store_key(key_name: str, value):
    directory = pathlib.Path.home() / ".weathercli"
    if not directory.exists():
        os.mkdir(directory)
    file = directory / "settings.json"
    if not file.exists():
        with open(file, 'w') as f:
            f.write('{}')
    with open(file, 'r') as f:
        data = json.load(f)
    data[key_name] = value
    # Serializing json
    json_object = json.dumps(data, indent=4)
    # Writing to sample.json
    with open(file, "w") as f:
        f.write(json_object)


def get_key(key: str, default=None) -> Any:
    if default is None:
        default = ""
    directory = pathlib.Path.home() / ".weathercli"
    if not directory.exists():
        os.mkdir(directory)
    file = directory / "settings.json"
    if not file.exists():
        with open(file, 'w') as f:
            f.write('{}')
    with open(file, 'r') as f:
        data = json.load(f)
    if key in data:
        return data[key]
    else:
        store_key(key, default)
        return default


OPEN_WEATHER_MAP_API_URL = get_key('OPEN_WEATHER_MAP_API_URL', 'https://api.openweathermap.org/data/2.5/')
OPEN_WEATHER_MAP_API_KEY = get_key('OPEN_WEATHER_MAP_API_KEY')
BING_MAPS_API_KEY = get_key('BING_MAPS_API_KEY')
NO_COLOR_DEFAULT = get_key('NO_COLOR', True)
METRIC_DEFAULT = get_key('METRIC', False)
