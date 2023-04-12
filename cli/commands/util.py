import json
import platform
import subprocess
import sys
import time
from pathlib import Path

import colorama
import weather_core
from click import argument, option, command
from weather_core import WeatherFile

from cli import version


@command("settings", help="opens a terminal gui for editing settings")
def settings():
    weather_core.open_settings_app()


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


@command(
    "update",
    help="updates the cli (standalone executable install only)",
)
@option("--force", is_flag=True, help="If true, application will force update")
def update(force):
    print("Checking for updates ...")
    latest_version = weather_core.updater.get_latest_version()
    if getattr(sys, "frozen", False):
        application_path = Path(sys.executable)
        print("Latest Version: " + latest_version)
        print("Current Version: " + version.__version__)
        if latest_version != version.__version__ or force:
            print("Updating weather.exe at " + str(application_path))
            if platform.system() == "Windows":
                updater_location = application_path.parent / "components" / "updater.exe"
            else:
                updater_location = application_path.parent / "components" / "update"
            if not updater_location.exists():
                print("Updater not found, downloading updater")
                weather_core.updater.get_updater(str(updater_location))
            resp = json.loads(
                weather_core.networking.get_url(
                    "https://arihant2math.github.io/weathercli/index.json"
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
                weather_core.hash_file(str(updater_location.absolute())) != web_hash
            ) or web_force:
                weather_core.updater.get_updater(str(updater_location))
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


@command("setup", help="setup prompt")
def setup():
    settings_s = weather_core.Settings()
    settings = settings_s.internal
    print(colorama.Fore.CYAN + "=== Weather CLI Setup ===")
    weather_core.updater.update_web_resources(settings.development)
    print(colorama.Fore.RED + "Choose the default weather backend: ")
    options = [
        "Meteo",
        "Open Weather Map",
        "National Weather Service",
        "The Weather Channel",
    ]
    default = ["METEO", "OPENWEATHERMAP", "NWS", "THEWEATHERCHANNEL"].index(
        settings.default_backend
    )
    current = weather_core.choice(options, default)
    weather_backend_setting = ["METEO", "OPENWEATHERMAP", "NWS", "THEWEATHERCHANNEL"][
        current
    ]
    settings_s.internal.default_backend = weather_backend_setting
    settings_s.write()
    time.sleep(0.1)
    print(
        colorama.Fore.RED
        + "Is your location constant (i.e. is this computer stationary at all times)?"
    )
    if settings.constant_location:
        default = 0
    else:
        default = 1
    constant_location_setting = [True, False][
        weather_core.choice(["yes", "no"], default)
    ]
    settings_s.internal.constant_location = constant_location_setting
    settings_s.write()
    time.sleep(0.1)
    print(
        colorama.Fore.RED
        + "Should static resources (ascii art, weather code sentences, etc.) be auto-updated?"
    )
    if settings.auto_update_internet_resources:
        default = 0
    else:
        default = 1
    auto_update_setting = [True, False][weather_core.choice(["yes", "no"], default)]
    settings_s.internal.auto_update_internet_resources = auto_update_setting
    settings_s.write()
