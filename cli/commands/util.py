import platform
import subprocess
import sys
import time
from pathlib import Path

import colorama
import core
import plotext
from click import argument, option, command

from cli import WeatherFile, version
from cli.backend.meteo.meteo_forecast import MeteoForecast
from cli.getch import _Getch
from cli.local import settings
from cli.local.settings import get_key, store_key


@command("config", help="prints or changes the settings")
@argument("key_name")
@option("--value", help="This sets the key")
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
def update():
    print("Checking for updates ...")
    latest_version = core.updater.get_latest_version()
    if getattr(sys, "frozen", False):
        application_path = Path(sys.executable)
        print("Latest Version: " + latest_version)
        print("Current Version: " + version.__version__)
        if latest_version != version.__version__:
            print("Updating weather.exe at " + str(application_path))
            if platform.system() == "Windows":
                updater_location = application_path.parent / "updater.exe"
            else:
                updater_location = application_path.parent / "update"
            if not updater_location.exists():
                print("Updater not found, downloading updater")
                core.updater.get_updater(str(updater_location))
            print("Starting updater and exiting")
            subprocess.Popen([updater_location], cwd=str(application_path.parent))
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
    print(colorama.Fore.GREEN + "=== Weather CLI Setup ===")
    print(colorama.Fore.BLUE + "Choose the default weather backend: ")
    options = [
        "Meteo",
        "Open Weather Map",
        "National Weather Service",
        "The Weather Channel",
    ]
    current = 0
    cursor_move = False
    for i in range(0, 4):
        if current != i:
            print("  " + options[i])
        else:
            print("> " + options[i])
    while True:
        g = _Getch().__call__()
        if cursor_move:
            if (g == b"M") or (g == b"P"):
                if current != 3:
                    current += 1
            elif (g == b"K") or (g == b"H"):
                if current != 0:
                    current -= 1
            cursor_move = False
        else:
            if g == b"\x03":
                exit(0)
            elif g == g == b"\r":
                break
            elif (g == b"a") or (g == b"1"):
                current = 0
            elif g == b"b" or (g == b"2"):
                current = 1
            elif g == b"c" or (g == b"3"):
                current = 2
            elif g == b"d" or (g == b"4"):
                current = 3
            elif g == b"\xe0":
                cursor_move = True
        sys.stdout.write("\u001b[1000D")  # Move left
        sys.stdout.write("\u001b[" + str(4) + "A")  # Move up
        for i in range(0, 4):
            if current != i:
                print("  " + options[i])
            else:
                print("> " + options[i])
        sys.stdout.flush()
    weather_backend_setting = ["meteo", "openweathermap", "nws", "theweatherchannel"][
        current
    ]
    settings.store_key("DEFAULT_BACKEND", weather_backend_setting)
    time.sleep(0.1)
    print("Is your location constant? yes (no)", end="")
    sys.stdout.flush()
    cursor_move = False
    current = 1
    while True:
        g = _Getch().__call__()
        if cursor_move:
            if (g == b"M") or (g == b"P"):
                current = 1
            elif (g == b"K") or (g == b"H"):
                current = 0
            cursor_move = False
        else:
            if g == b"\x03":
                exit(0)
            elif g == g == b"\r":
                break
            elif (g == b"a") or (g == b"1"):
                current = 0
            elif g == b"b" or (g == b"2"):
                current = 1
            elif g == b"\xe0":
                cursor_move = True
        sys.stdout.write("\u001b[1000D")  # Move left
        if current == 0:
            print("Is your location constant? (yes) no", end="")
        else:
            print("Is your location constant? yes (no)", end="")
        sys.stdout.flush()
    constant_location_setting = [True, False][current]
    settings.store_key("CONSTANT_LOCATION", constant_location_setting)
    time.sleep(0.1)
