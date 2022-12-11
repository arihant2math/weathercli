import json
import os
import pathlib
from typing import Any
import datetime


def add_data(store: str, key: str, value: str):
    directory = pathlib.Path.home() / ".weathercli"
    if not directory.exists():
        os.mkdir(directory)
    file = directory / "cache.json"
    if not file.exists():
        with open(file, 'w') as f:
            f.write('{}')
    with open(file, 'r') as f:
        data = json.load(f)
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
    with open(file, "w") as f:
        f.write(json_object)


def get_key(store: str, key: str) -> Any:
    directory = pathlib.Path.home() / ".weathercli"
    if not directory.exists():
        os.mkdir(directory)
    file = directory / "cache.json"
    if not file.exists():
        with open(file, 'w') as f:
            f.write('{}')
            return None  # the store definitely does not exist
    with open(file, 'r') as f:
        data = json.load(f)
    if (store in data) and (key in data[store]):
        return data[store][key]['value']
    else:
        return None


# TODO: FIX
def prune_cache():
    directory = pathlib.Path.home() / ".weathercli"
    if not directory.exists():
        os.mkdir(directory)
    file = directory / "cache.json"
    if not file.exists():
        with open(file, 'w') as f:
            f.write('{}')
            return
    with open(file, 'r') as f:
        data = json.load(f)
    for store in data:
        for key in store:
            if key["count"] < 2:
                pass
