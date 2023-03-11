import logging
import sys
from datetime import datetime
from os.path import expanduser
from pathlib import Path

import colorama
from click import group, option, pass_context, argument

from cli import print_out, get_data_from_datasource
from cli.commands.util import update, clear_cache, setup, config
from cli.custom_click_group import CustomClickGroup
from cli.local.settings import METRIC_DEFAULT, DEFAULT_BACKEND, DEBUG
from cli.location import get_coordinates, get_location


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
    FORMAT = "[%(levelname)s] %(message)s"
    if not DEBUG:
        logging.basicConfig(format=FORMAT, level=logging.CRITICAL)
    else:
        logging.basicConfig(filename=get_log_file(), filemode="w", format=FORMAT, level=logging.DEBUG)
    d = {"component": "main"}
    logger = logging.getLogger("weathercli")
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
        logger.debug("datasource=" + datasource, extra=d)
        logger.info("location=" + str(location), extra=d)
        logger.debug("metric=" + str(true_metric), extra=d)
        data = get_data_from_datasource(datasource, location, true_metric, logger)
        print_out(data, json, true_metric, logger)
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
    logger = ctx.obj["LOGGER"]
    d = ctx.obj["d"]
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
        logger.critical("Place not Found")
        exit()
    logger.debug("datasource=" + datasource, extra=d)
    logger.info("location=" + str(location), extra=d)
    logger.debug("metric=" + str(true_metric), extra=d)
    data = get_data_from_datasource(datasource, location, true_metric, logger)
    print_out(data, ctx.obj["JSON"] or json, true_metric, logger)


if __name__ == "__main__":
    if not DEBUG:
        def exception_handler(exception_type, exception, traceback):
            # No traceback
            print(colorama.Fore.RED + "%s: %s" % (exception_type.__name__, exception))

        sys.excepthook = exception_handler
    main.add_command(config)
    main.add_command(update)
    main.add_command(clear_cache)
    main.add_command(setup)
    main(obj={})
