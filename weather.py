import asyncio

from click import group, option, pass_context, argument

from cli import OpenWeatherMapWeatherData, get_combined_data, print_out, CustomMultiCommand, get_device_location, \
    get_coordinates
from cli.api_keys import store_key, get_key, METRIC_DEFAULT


@group(invoke_without_command=True, cls=CustomMultiCommand)
@option('-j', '--json', is_flag=True, help='If used the raw json will be printed out')
@option('-n', '--no-color', is_flag=True, help='This will not use color when printing the data out')
@option('--no-sys-loc', is_flag=True, help='If used the location will be gotten from the web rather than the system'
                                           'even if system location is available')
@option('--metric', is_flag=True, help='This will switch the output to metric')
@option('--imperial', is_flag=True, help='This will switch the output to imperial')
@pass_context
def main(ctx, json, no_color, no_sys_loc, metric, imperial):
    if not METRIC_DEFAULT and metric:
        true_metric = True
    elif METRIC_DEFAULT and imperial:
        true_metric = False
    else:
        true_metric = METRIC_DEFAULT
    if ctx.invoked_subcommand is None:
        raw_data = asyncio.run(get_combined_data(get_device_location(no_sys_loc), true_metric))
        data = OpenWeatherMapWeatherData(raw_data)
        print_out(raw_data, data, json, no_color, true_metric)
    else:
        ctx.ensure_object(dict)
        ctx.obj["JSON"] = json
        ctx.obj["NO_COLOR"] = no_color
        ctx.obj["METRIC"] = true_metric


@main.command(['place', 'p', 'c'], help="prints the weather for the specified location")
@argument('location')
@option('-j', '--json', is_flag=True, help='If used the raw json will be printed out')
@option('-n', '--no-color', is_flag=True, help='This will not use color when printing the data out')
@option('--metric', is_flag=True, help='This will switch the output to metric')
@pass_context
def place(ctx, location, json, no_color, metric):
    raw_data = asyncio.run(get_combined_data(get_coordinates(location), metric))
    data = OpenWeatherMapWeatherData(raw_data)
    print_out(raw_data, data, ctx.obj["JSON"] or json, ctx.obj["NO_COLOR"] or no_color, ctx.obj["METRIC"] or metric)


@main.command(['config'], help="prints or changes the settings")
@argument('key_name')
@option('--value', help='This sets the key')
@pass_context
def test(ctx, key_name: str, value):
    if value is None:
        print(get_key(key_name.upper()))
    else:
        store_key(key_name.upper(), value)


if __name__ == '__main__':
    main(obj={})
