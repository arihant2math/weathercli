import platform
import subprocess
import sys
from pathlib import Path
import plotext

from click import group, option, pass_context, argument
import core

from cli import print_out
from cli.backend.meteo import Meteo
from cli.custom_multi_command import CustomMultiCommand
from cli.location import get_coordinates
from cli.backend.nws import NationalWeatherService
from cli.backend.openweathermap import OpenWeatherMap
from cli.settings import store_key, get_key, METRIC_DEFAULT, NO_COLOR_DEFAULT, DEFAULT_BACKEND
from cli.backend.the_weather_channel import TheWeatherChannel
from cli.weather_file import WeatherFile


@group(invoke_without_command=True, cls=CustomMultiCommand)
@option("-j", "--json", is_flag=True, help="If used the raw json will be printed out")
@option(
    "--no-sys-loc",
    is_flag=True,
    help="If used the location will be gotten from the web rather than the system"
         "even if system location is available",
)
@option(
    "-n",
    "--no-color",
    is_flag=True,
    help="This will not use color when printing the data out",
)
@option(
    "--color",
    is_flag=True,
    help="This will force the cli to use color when printing the data out",
)
@option("--metric", is_flag=True, help="This will switch the output to metric")
@option("--imperial", is_flag=True, help="This will switch the output to imperial")
@option(
    "--datasource",
    help="The data source to retrieve the data from, current options are openweathermap, theweatherchannel, and nws",
)
@pass_context
def main(ctx, json, no_sys_loc, no_color, color, metric, imperial, datasource):
    if datasource is None:
        datasource = DEFAULT_BACKEND
    else:
        datasource = datasource.upper()
    true_metric = METRIC_DEFAULT
    if metric:
        true_metric = True
    elif imperial:
        true_metric = False

    true_no_color = NO_COLOR_DEFAULT
    if no_color:
        true_no_color = True
    elif color:
        true_no_color = False

    if ctx.invoked_subcommand is None:
        if datasource == "NWS":
            data = NationalWeatherService(core.get_location(no_sys_loc))
        elif datasource == "THEWEATHERCHANNEL":
            data = TheWeatherChannel(core.get_location(no_sys_loc))
        elif datasource == "OPENWEATHERMAP":
            data = OpenWeatherMap(core.get_location(no_sys_loc), true_metric)
        elif datasource == "METEO":
            data = Meteo(core.get_location(no_sys_loc), true_metric)
        else:
            print("Invalid Data Source!")
            exit(1)
        print_out(data, json, true_no_color, true_metric)
    else:
        ctx.ensure_object(dict)
        ctx.obj["JSON"] = json
        ctx.obj["NO_COLOR"] = true_no_color
        ctx.obj["METRIC"] = true_metric


@main.command(["place", "p"], help="prints the weather for the specified location")
@argument("location")
@option("-j", "--json", is_flag=True, help="If used the raw json will be printed out")
@option("-n", "--no-color", is_flag=True, help="This will not use color when printing the data out")
@option("--color", is_flag=True, help="This will force the cli to use color when printing the data out")
@option("--metric", is_flag=True, help="This will switch the output to metric")
@option("--imperial", is_flag=True, help="This will switch the output to imperial")
@option("--datasource", help="The data source to retrieve the data from, current options are openweathermap, "
                             "theweatherchannel, meteo, and nws")
@pass_context
def place(ctx, location, json, no_color, color, metric, imperial, datasource):
    if datasource is None:
        datasource = DEFAULT_BACKEND
    else:
        datasource = datasource.upper()
    true_metric = ctx.obj["METRIC"]
    if metric:
        true_metric = True
    elif imperial:
        true_metric = False

    true_no_color = ctx.obj["NO_COLOR"]
    if no_color:
        true_no_color = True
    elif color:
        true_no_color = False
    if datasource == "NWS":
        data = NationalWeatherService(get_coordinates(location))
    elif datasource == "THEWEATHERCHANNEL":
        data = TheWeatherChannel(get_coordinates(location))
    elif datasource == "OPENWEATHERMAP":
        data = OpenWeatherMap(get_coordinates(location), true_metric)
    elif datasource == "METEO":
        data = Meteo(get_coordinates(location), true_metric)
    else:
        print("Invalid Data Source!")
        exit(1)
    print_out(data, ctx.obj["JSON"] or json, true_no_color, true_metric)


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
    latest_version = core.update.get_latest_version()
    if getattr(sys, "frozen", False):
        application_path = Path(sys.executable)
        print("Latest Version: " + latest_version)
        if latest_version != "0.0.0":
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
    data = Meteo(core.get_location(False), False)
    plotext.plot([i for i in range(0, len(data.raw_data["hourly"]['temperature_2m']))], data.raw_data["hourly"]['temperature_2m'])
    plotext.title("Temperature")
    plotext.show()


if __name__ == "__main__":
    main(obj={})
