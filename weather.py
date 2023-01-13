import platform
import subprocess
import sys
import time
from pathlib import Path

import colorama
import plotext

from click import group, option, pass_context, argument
import core

from cli import print_out, update_weather_codes
from cli.backend.meteo.meteo_current import MeteoCurrent
from cli.backend.meteo.meteo_forecast import MeteoForecast
from cli.backend.nws.nws_forecast import NationalWeatherServiceForecast
from cli.backend.openweathermap.openweathermap_forecast import OpenWeatherMapForecast
from cli.backend.theweatherchannel.the_weather_channel_forecast import TheWeatherChannelForecast
from cli.custom_multi_command import CustomMultiCommand
from cli.getch import _Getch
from cli.local import settings
from cli.location import get_coordinates, get_location
from cli.local.settings import store_key, get_key, METRIC_DEFAULT, DEFAULT_BACKEND
from cli.local.weather_file import WeatherFile


def get_data_from_datasource(datasource, location, true_metric):
    update_weather_codes()
    if datasource == "NWS":
        data = NationalWeatherServiceForecast(location, true_metric)
    elif datasource == "THEWEATHERCHANNEL":
        data = TheWeatherChannelForecast(location)
    elif datasource == "OPENWEATHERMAP":
        data = OpenWeatherMapForecast(location, true_metric)
    elif datasource == "METEO":
        data = MeteoForecast(location, true_metric)
    else:
        print(colorama.Fore.RED + "Invalid Data Source!")
        exit(1)
    return data


@group(invoke_without_command=True, cls=CustomMultiCommand)
@option("-j", "--json", is_flag=True, help="If used the raw json will be printed out")
@option(
    "--no-sys-loc",
    is_flag=True,
    help="If used the location will be gotten from the web rather than the system"
    "even if system location is available",
)
@option("--metric", is_flag=True, help="This will switch the output to metric")
@option("--imperial", is_flag=True, help="This will switch the output to imperial")
@option(
    "--datasource",
    help="The data source to retrieve the data from, current options are openweathermap, "
    "theweatherchannel, meteo, and nws",
)
@pass_context
def main(ctx, json, no_sys_loc, metric, imperial, datasource):
    if datasource is None:
        datasource = DEFAULT_BACKEND
    else:
        datasource = datasource.upper()
    true_metric = METRIC_DEFAULT
    if metric:
        true_metric = True
    elif imperial:
        true_metric = False

    if ctx.invoked_subcommand is None:
        location = get_location(no_sys_loc)
        data = get_data_from_datasource(datasource, location, true_metric)
        print_out(data, json, true_metric)
    else:
        ctx.ensure_object(dict)
        ctx.obj["JSON"] = json
        ctx.obj["METRIC"] = true_metric


@main.command(["place", "p"], help="prints the weather for the specified location")
@argument("location")
@option("-j", "--json", is_flag=True, help="If used the raw json will be printed out")
@option("--metric", is_flag=True, help="This will switch the output to metric")
@option("--imperial", is_flag=True, help="This will switch the output to imperial")
@option(
    "--datasource",
    help="The data source to retrieve the data from, current options are openweathermap, "
    "theweatherchannel, meteo, and nws",
)
@pass_context
def place(ctx, location, json, metric, imperial, datasource):
    if datasource is None:
        datasource = DEFAULT_BACKEND
    else:
        datasource = datasource.upper()
    true_metric = ctx.obj["METRIC"]
    if metric:
        true_metric = True
    elif imperial:
        true_metric = False
    try:
        location = get_coordinates(location)
    except LookupError:
        print(colorama.Fore.RED + "Place not Found")
        exit()
    data = get_data_from_datasource(datasource, location, true_metric)
    print_out(data, ctx.obj["JSON"] or json, true_metric)


@main.command(["config", "c"], help="prints or changes the settings")
@argument("key_name")
@option("--value", help="This sets the key")
@pass_context
def config(ctx, key_name: str, value):
    value = str(value)
    if value is None:
        print(get_key(key_name.upper()))
    else:
        if value.isdigit():
            value = int(value)
        elif value.lower() in ["true", "t", "yes", "y"]:
            value = True
        elif value.lower() in ["false", "f", "no", "n"]:
            value = False
        store_key(key_name.upper(), value)


@main.command(
    "update",
    help="updates the cli (standalone executable install only)",
)
@pass_context
def update(ctx):
    print("Checking for updates ...")
    latest_version = core.updater.get_latest_version()
    if getattr(sys, "frozen", False):
        application_path = Path(sys.executable)
        print("Latest Version: " + latest_version)
        if latest_version != "0":
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


@main.command("clear-cache", help="clears every cache")
@pass_context
def clear_cache(ctx):
    f = WeatherFile("cache.json")
    f.data = {}
    f.write()


@main.command("plot-temp", help="plots the temperature over time")
@pass_context
def plot_temp(ctx):
    data = MeteoCurrent(core.get_location(False), False)
    plotext.plot(
        [i for i in range(0, len(data.raw_data["hourly"]["temperature_2m"]))],
        data.raw_data["hourly"]["temperature_2m"],
    )
    plotext.title("Temperature")
    plotext.show()


@main.command("setup", help="setup prompt")
@pass_context
def setup(ctx):
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


if __name__ == "__main__":
    main(obj={})
