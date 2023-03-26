import logging
import os.path
import sys
from datetime import datetime
from os.path import expanduser
from pathlib import Path

import colorama
import weather_core
from click import group, option, pass_context, argument

from cli import print_out, get_data_from_datasource
from cli.commands.util import update, clear_cache, setup, config, settings
from cli.custom_click_group import CustomClickGroup


def get_log_file():
    now = datetime.now()
    log_folder = Path(expanduser("~")) / ".weathercli" / "logs"
    log_folder.mkdir(parents=True, exist_ok=True)
    return log_folder / "{}-{}-{}_{}-{}-{}-{}.log".format(
        now.year, now.month, now.day, now.hour, now.minute, now.second, now.microsecond
    )


@group(invoke_without_command=True, cls=CustomClickGroup)
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
    settings_s = weather_core.Settings()
    settings = settings_s.internal
    FORMAT = "[%(levelname)s] %(message)s"
    if not settings.debug:
        logging.basicConfig(format=FORMAT, level=logging.CRITICAL)
    else:
        logging.basicConfig(
            filename=get_log_file(), filemode="w", format=FORMAT, level=logging.DEBUG
        )
    d = {"component": "main"}
    logger = logging.getLogger("weathercli")
    if datasource is None:
        datasource = settings.default_backend
    else:
        datasource = datasource.upper()
    true_metric = settings.metric_default
    if metric:
        true_metric = True
    elif imperial:
        true_metric = False
    if ctx.invoked_subcommand is None:
        location = weather_core.location.get_location(
            no_sys_loc, settings.constant_location
        )
        if settings.daemon_enabled and not os.path.exists(os.path.expanduser("~/.weathercli/downloader_urls.list")):
            print("daemon enabled, creating downloader list")
        logger.debug("datasource=" + datasource, extra=d)
        logger.info("location=" + str(location), extra=d)
        logger.debug("metric=" + str(true_metric), extra=d)
        data = get_data_from_datasource(
            datasource, location, true_metric, settings, logger, datasource == settings.default_backend
        )
        print_out(settings.layout_file, data, json, true_metric, logger)
    else:
        ctx.ensure_object(dict)
        ctx.obj["d"] = d
        ctx.obj["JSON"] = json
        ctx.obj["METRIC"] = true_metric
        ctx.obj["LOGGER"] = logger


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
    settings_s = weather_core.Settings()
    settings = settings_s.internal
    logger = ctx.obj["LOGGER"]
    d = ctx.obj["d"]
    if datasource is None:
        datasource = settings.default_backend
    else:
        datasource = datasource.upper()
    true_metric = ctx.obj["METRIC"]
    if metric:
        true_metric = True
    elif imperial:
        true_metric = False
    try:
        location = weather_core.location.get_coordinates(
            location, settings.bing_maps_api_key
        )
    except LookupError:
        print(colorama.Fore.RED + "Place not Found")
        logger.critical("Place not Found")
        exit()
    logger.debug("datasource=" + datasource, extra=d)
    logger.info("location=" + str(location), extra=d)
    logger.debug("metric=" + str(true_metric), extra=d)
    data = get_data_from_datasource(datasource, location, true_metric, settings, logger, False)
    print_out(settings.layout_file, data, ctx.obj["JSON"] or json, true_metric, logger)


if __name__ == "__main__":
    settings_s = weather_core.Settings()
    if not settings_s.internal.debug:
        def exception_handler(exception_type, exception, traceback):
            # No traceback
            print(
                colorama.Fore.RED
                + "Internal Weather CLI Error\nSomething went wrong\nDetails:\n%s: %s\nSet debug "
                  "to true for a traceback by running weather config debug true or manually "
                  "editing the settings file at ~/.weathercli/settings.json and setting the key "
                  "'debug' to true." % (exception_type.__name__, exception)
            )


        sys.excepthook = exception_handler
    main.add_command(config)
    main.add_command(update)
    main.add_command(clear_cache)
    main.add_command(setup)
    main.add_command(settings)
    main(obj={})
