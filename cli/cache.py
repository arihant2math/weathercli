import json
from typing import Any
import datetime

from cli.weather_file import WeatherFile


def add_data(store: str, key: str, value: str):
    f = WeatherFile('cache.json')
    data = f.data
    if store in data:
        store_values = data[store]
    else:
        store_values = {}
    time = datetime.datetime.now()
    time_str = str(time.year) + ":" + str(time.month) + ":" + str(time.day)
    if key in store_values:
        value_data: dict = store_values[key]
        value_data["count"] = int(value_data["count"]) + 1
        value_data["last_queried"] = time_str
        value_data["value"] = value
    else:
        value_data = {"count": 1, "value": value, "last_queried": time_str}
        store_values[key] = value_data
    if store not in data:
        data[store] = store_values
    # Serializing json
    json_object = json.dumps(data, indent=4)
    # Writing to sample.json
    with open(f.file, "w") as f:
        f.write(json_object)


def get_key(store: str, key: str) -> Any:
    f = WeatherFile('cache.json')
    if (store in f.data) and (key in f.data[store]):
        time = datetime.datetime.now()
        f.data[store][key]['count'] += 1
        time_str = str(time.year) + ":" + str(time.month) + ":" + str(time.day)
        f.data[store][key]['last_queried'] = time_str
        f.write()
        return f.data[store][key]['value']
    else:
        return None


def prune_cache():
    time = datetime.datetime.now()
    f = WeatherFile('cache.json')
    data = f.data
    for store in data:
        for key in store:
            if len(store) > 100:
                query_date: list[str] = data[store][key]['last_queried'].split(":")
                if int(query_date[0]) != time.now():
                    store.pop(key, None)
