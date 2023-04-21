import json

import weather_core
from click import argument, command
from weather_core import WeatherFile


@command("config", help="prints or changes the settings")
@argument("key_name")
@argument("value", required=False)
def config(key_name: str, value):
    settings_s = weather_core.Settings()
    value = str(value)
    if value is None or value == "" or value == "None":
        v = getattr(settings_s.internal, key_name.lower())
        print(v)
    else:
        if value.isdigit():
            value = int(value)
        elif value.lower() in ["true", "t", "yes", "y"]:
            value = True
        elif value.lower() in ["false", "f", "no", "n"]:
            value = False
        print("Writing " + key_name.lower() + "=" + str(value) + " ...")
        f = WeatherFile("settings.json")
        data = json.loads(f.data)
        data[key_name.upper()] = value
        f.data = json.dumps(data)
        f.write()
