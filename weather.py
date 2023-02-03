import sys

import colorama
from click import group, option, pass_context, argument

from cli import print_out, update_weather_codes
from cli.backend.meteo.meteo_forecast import MeteoForecast
from cli.backend.nws.nws_forecast import NationalWeatherServiceForecast
from cli.backend.openweathermap.openweathermap_forecast import OpenWeatherMapForecast
from cli.backend.theweatherchannel.the_weather_channel_forecast import (
    TheWeatherChannelForecast,
)
from cli.commands.util import update, clear_cache, setup, config
from cli.custom_multi_command import CustomMultiCommand
from cli.local.settings import METRIC_DEFAULT, DEFAULT_BACKEND
from cli.location import get_coordinates, get_location


def get_data_from_datasource(datasource, location, true_metric):
    update_weather_codes()
    if datasource == "NWS":
        data = NationalWeatherServiceForecast(location, true_metric)
    elif datasource == "THEWEATHERCHANNEL":
        data = TheWeatherChannelForecast(location, true_metric)
    elif datasource == "OPENWEATHERMAP":
        data = OpenWeatherMapForecast(location, true_metric)
    elif datasource == "METEO":
        data = MeteoForecast(location, true_metric)
    else:
        print(colorama.Fore.RED + "Invalid Data Source!")
        exit(1)
    sys.stdout.flush()
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


if __name__ == "__main__":
    main.add_command(config)
    main.add_command(update)
    main.add_command(clear_cache)
    main.add_command(setup)
    main(obj={})
