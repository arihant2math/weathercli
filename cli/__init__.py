import json
import os.path
import sys
from logging import Logger
from pathlib import Path
from threading import Thread

import colorama
import rich
import weather_core
from weather_core.backend import WeatherForecast

from cli.backend.meteo.meteo_forecast import MeteoForecast
from cli.backend.nws.nws_forecast import NationalWeatherServiceForecast
from cli.backend.openweathermap.openweathermap_forecast import OpenWeatherMapForecast
from cli.backend.theweatherchannel.the_weather_channel_forecast import (
    TheWeatherChannelForecast,
)
from cli.layout.layout_file import LayoutFile


def print_out(
    layout_file, data: WeatherForecast, print_json: bool, metric: bool, logger: Logger
):
    color = colorama.Fore
    if print_json:
        try:
            if isinstance(data.raw_data, list):
                for i in data.raw_data:
                    print(
                        "============================================================"
                    )
                    rich.print_json(json.dumps(i))
            elif isinstance(data.raw_data, str):
                rich.print_json(data.raw_data)
            elif isinstance(data.raw_data, dict):
                rich.print_json(json.dumps(data.raw_data))
            elif isinstance(data.raw_data, weather_core.FormattedData):
                rich.print_json(data.raw_data.raw_data[0])
                rich.print_json(data.raw_data.raw_data[1])
                rich.print_json(data.raw_data.raw_data[2])
            else:
                print(type(data.raw_data))
        except Exception as e:
            print(e)
            print(data.raw_data)
    elif data.status == 0:
        out = LayoutFile(layout_file, logger=logger)
        print(out.to_string(data, metric))
    else:
        print(color.RED + data.raw_data["message"] + color.RESET, end="")


def get_data_from_datasource(
    datasource, location, true_metric, settings, logger: Logger
):
    if not os.path.exists(
        Path(os.path.expanduser("~/.weathercli/weather_codes.json"))
        or Path(os.path.expanduser("~/.weathercli/weather_ascii_images.json"))
    ):  # Hacky way to check if the resources don't exist
        weather_core.updater.update_web_resources(
            settings.development
        )  # Force download them so that it doesn't fail
    if settings.auto_update_internet_resources:
        logger.info("Updating web resources")
        thread = Thread(
            target=weather_core.updater.update_web_resources, args=[settings.development]
        )
        thread.start()
    if datasource == "NWS":
        data = NationalWeatherServiceForecast(location, true_metric, settings)
    elif datasource == "THEWEATHERCHANNEL":
        data = TheWeatherChannelForecast(location, true_metric, settings)
    elif datasource == "OPENWEATHERMAP":
        data = OpenWeatherMapForecast(location, true_metric, settings)
    elif datasource == "METEO":
        data = MeteoForecast(location, true_metric, settings)
    else:
        print(colorama.Fore.RED + "Invalid Data Source!")
        logger.critical("Invalid Data Source")
        exit(1)
    logger.info("Data Retrieved")
    if settings.auto_update_internet_resources:
        thread.join()
    sys.stdout.flush()
    return data


def get_alerts(location):
    r = weather_core.networking.get_url(
        "https://api.weather.gov/alerts/active?status=actual&point={}%2C{}&limit=500"
        "".format(location[0], location[1])
    )
    return r
