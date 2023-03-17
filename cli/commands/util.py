import platform
import subprocess
import sys
import time
from pathlib import Path

import colorama
import core
import plotext
import requests
from click import argument, option, command
from core import WeatherFile

from cli import version
from cli.backend.meteo.meteo_forecast import MeteoForecast
from cli.local import settings
from cli.local.settings import get_key, store_key
from cli.prompt import multi_choice, yes_no


@command("config", help="prints or changes the settings")
@argument("key_name")
@argument("value", required=False)
def config(key_name: str, value):
    value = str(value)
    if value is None or value == "" or value == "None":
        v = get_key(key_name.upper())
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
        store_key(key_name.upper(), value)


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
            resp = requests.get(
                "https://arihant2math.github.io/weathercli/docs/index.json"
            ).json()
            if platform.system() == "Windows":
                web_hash = resp["updater-exe-hash-windows"]
            else:
                web_hash = resp["updater-exe-hash-unix"]
            if core.hash_file(updater_location.absolute()) != web_hash:
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
    print(colorama.Fore.CYAN + "=== Weather CLI Setup ===")
    core.updater.update_web_resources(settings.DEVELOPMENT)
    print(
        colorama.Fore.GREEN
        + "Choose the default weather backend: "
        + colorama.Fore.BLUE
    )
    options = [
        "Meteo",
        "Open Weather Map",
        "National Weather Service",
        "The Weather Channel",
    ]
    default = ["METEO", "OPENWEATHERMAP", "NWS", "THEWEATHERCHANNEL"].index(
        settings.DEFAULT_BACKEND
    )
    current = multi_choice(options, default)
    weather_backend_setting = ["METEO", "OPENWEATHERMAP", "NWS", "THEWEATHERCHANNEL"][
        current
    ]
    settings.store_key("DEFAULT_BACKEND", weather_backend_setting)
    time.sleep(0.1)
    s = (
        colorama.Fore.GREEN
        + "Is your location constant (i.e. is this computer stationary at all times)?"
        + colorama.Fore.BLUE
    )
    if settings.CONSTANT_LOCATION:
        default = 0
    else:
        default = 1
    constant_location_setting = yes_no(s, default)
    settings.store_key("CONSTANT_LOCATION", constant_location_setting)
    time.sleep(0.1)
    print()
    s = (
        colorama.Fore.GREEN
        + "Should static resources (ascii art, weather code sentences, etc.) be auto-updated?"
        + colorama.Fore.BLUE
    )
    if settings.AUTO_UPDATE_INTERNET_RESOURCES:
        default = 0
    else:
        default = 1
    auto_update_setting = yes_no(s, default)
    settings.store_key("AUTO_UPDATE_INTERNET_RESOURCES", auto_update_setting)
