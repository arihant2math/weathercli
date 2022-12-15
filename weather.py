import subprocess
import sys
from pathlib import WindowsPath

from click import group, option, pass_context, argument
import core

from cli import OpenWeatherMapWeatherData, get_combined_data, print_out, CustomMultiCommand
from cli.location import get_coordinates
from cli.settings import store_key, get_key, METRIC_DEFAULT, NO_COLOR_DEFAULT


@group(invoke_without_command=True, cls=CustomMultiCommand)
@option('-j', '--json', is_flag=True, help='If used the raw json will be printed out')
@option('--no-sys-loc', is_flag=True, help='If used the location will be gotten from the web rather than the system'
                                           'even if system location is available')
@option('-n', '--no-color', is_flag=True, help='This will not use color when printing the data out')
@option('--color', is_flag=True, help='This will force the cli to use color when printing the data out')
@option('--metric', is_flag=True, help='This will switch the output to metric')
@option('--imperial', is_flag=True, help='This will switch the output to imperial')
@pass_context
def main(ctx, json, no_sys_loc, no_color, color, metric, imperial):
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
        raw_data = get_combined_data(core.get_location(no_sys_loc), true_metric)
        data = OpenWeatherMapWeatherData(raw_data)
        print_out(data, json, true_no_color, true_metric)
    else:
        ctx.ensure_object(dict)
        ctx.obj["JSON"] = json
        ctx.obj["NO_COLOR"] = true_no_color
        ctx.obj["METRIC"] = true_metric


@main.command(['place', 'p', 'city'], help="prints the weather for the specified location")
@argument('location')
@option('-j', '--json', is_flag=True, help='If used the raw json will be printed out')
@option('-n', '--no-color', is_flag=True, help='This will not use color when printing the data out')
@option('--color', is_flag=True, help='This will force the cli to use color when printing the data out')
@option('--metric', is_flag=True, help='This will switch the output to metric')
@option('--imperial', is_flag=True, help='This will switch the output to imperial')
@pass_context
def place(ctx, location, json, no_color, color, metric, imperial):
    raw_data = get_combined_data(get_coordinates(location), metric)
    data = OpenWeatherMapWeatherData(raw_data)
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

    print_out(data, ctx.obj["JSON"] or json, true_no_color, true_metric)


@main.command(['config', 'setting', 'c'], help="prints or changes the settings")
@argument('key_name')
@option('--value', help='This sets the key')
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


@main.command(['update-windows'], help="updates the cli (windows only)")
@pass_context
def update_windows(ctx):
    print("Checking for updates ...")
    latest_version = core.is_update_available()
    if getattr(sys, 'frozen', False):
        application_path = WindowsPath(sys.executable)
        print("Latest Version: " + latest_version)
        if latest_version != "12/13/2022":
            print("Updating weather.exe at " + str(application_path))
            updater_location = application_path.parent / "updater.exe"
            if not updater_location.exists():
                print("Updater not found, downloading updater")
                core.get_updater(str(updater_location))
            print("Starting updater and exiting")
            subprocess.call([updater_location], cwd=str(application_path.parent))
            sys.exit(0)
    else:
        print("Not implemented for non executable installs")


if __name__ == '__main__':
    main(obj={})
