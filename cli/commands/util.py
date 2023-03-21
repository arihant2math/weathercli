import json
import platform
import subprocess
import sys
import time
from pathlib import Path

import colorama
import core
import plotext
from click import argument, option, command
from core import WeatherFile

from cli import version
from cli.backend.meteo.meteo_forecast import MeteoForecast


@command("config", help="prints or changes the settings")
@argument("key_name")
@argument("value", required=False)
def config(key_name: str, value):
    settings_s = core.Settings()
    value = str(value)
    if value is None or value == "" or value == "None":
        v = getattr(settings_s.internal, key_name.upper())
        if v is not None:
            print(v)
        else:
            print("Key not found")
    else:
        print("Writing ...")
        if value.isdigit():
            value = int(value)
        elif value.lower() in ["true", "t", "yes", "y"]:
            value = True
        elif value.lower() in ["false", "f", "no", "n"]:
            value = False
        setattr(settings_s.internal, key_name.upper(), value)


@command(
    "update",
    help="updates the cli (standalone executable install only)",
)
@option("--force", is_flag=True, help="If true, application will force update")
def update(force):
    print("Checking for updates ...")
    latest_version = core.updater.get_latest_version()
    if getattr(sys, "frozen", False):
        application_path = Path(sys.executable)
        print("Latest Version: " + latest_version)
        print("Current Version: " + version.__version__)
        if latest_version != version.__version__ or force:
            print("Updating weather.exe at " + str(application_path))
            if platform.system() == "Windows":
                updater_location = application_path.parent / "updater.exe"
            else:
                updater_location = application_path.parent / "update"
            if not updater_location.exists():
                print("Updater not found, downloading updater")
                core.updater.get_updater(str(updater_location))
            resp = json.loads(
                core.networking.get_url(
                    "https://arihant2math.github.io/weathercli/docs/index.json"
                ).text
            )
            web_force = False
            if "force" in resp:
                force = True
                web_force = True
            if platform.system() == "Windows":
                web_hash = resp["updater-exe-hash-windows"]
            else:
                web_hash = resp["updater-exe-hash-unix"]
            if (
                core.hash_file(str(updater_location.absolute())) != web_hash
            ) or web_force:
                core.updater.get_updater(str(updater_location))
            print("Starting updater and exiting")
            if force:
                subprocess.Popen(
                    [str(updater_location), "--force"], cwd=str(application_path.parent)
                )
            else:
                subprocess.Popen(
                    [str(updater_location)], cwd=str(application_path.parent)
                )
            sys.exit(0)
    else:
        print("Not implemented for non executable installs")


@command("clear-cache", help="clears every cache")
def clear_cache():
    f = WeatherFile("d.cache")
    f.data = ""
    f.write()


@command("plot-temp", help="plots the temperature over time")
def plot_temp():
    data = MeteoForecast(core.get_location(False), False)
    plotext.plot(
        [i for i in range(0, len(data.raw_data["hourly"]["temperature_2m"]))],
        data.raw_data["hourly"]["temperature_2m"],
    )
    plotext.title("Temperature")
    plotext.show()


@command("setup", help="setup prompt")
def setup():
    settings_s = core.Settings()
    settings = settings_s.internal
    print(colorama.Fore.CYAN + "=== Weather CLI Setup ===")
    core.updater.update_web_resources(settings.DEVELOPMENT)
    print(colorama.Fore.RED + "Choose the default weather backend: ")
    options = [
        "Meteo",
        "Open Weather Map",
        "National Weather Service",
        "The Weather Channel",
    ]
    default = ["METEO", "OPENWEATHERMAP", "NWS", "THEWEATHERCHANNEL"].index(
        settings.DEFAULT_BACKEND
    )
    current = core.choice(options, default)
    weather_backend_setting = ["METEO", "OPENWEATHERMAP", "NWS", "THEWEATHERCHANNEL"][
        current
    ]
    settings_s.internal.DEFAULT_BACKEND = weather_backend_setting
    settings_s.write()
    time.sleep(0.1)
    print(
        colorama.Fore.RED
        + "Is your location constant (i.e. is this computer stationary at all times)?"
    )
    if settings.CONSTANT_LOCATION:
        default = 0
    else:
        default = 1
    constant_location_setting = [True, False][core.choice(["yes", "no"], default)]
    settings_s.internal.CONSTANT_LOCATION = constant_location_setting
    settings_s.write()
    time.sleep(0.1)
    print(
        colorama.Fore.RED
        + "Should static resources (ascii art, weather code sentences, etc.) be auto-updated?"
    )
    if settings.AUTO_UPDATE_INTERNET_RESOURCES:
        default = 0
    else:
        default = 1
    auto_update_setting = [True, False][core.choice(["yes", "no"], default)]
    settings_s.internal.AUTO_UPDATE_INTERNET_RESOURCES = auto_update_setting
    settings_s.write()
