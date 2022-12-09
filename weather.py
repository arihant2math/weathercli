import asyncio

from click import group, option, pass_context, argument

from cli import OpenWeatherMapWeatherData, get_combined_data, print_out, CustomMultiCommand, get_device_location, \
    get_coordinates
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
        raw_data = asyncio.run(get_combined_data(get_device_location(no_sys_loc), true_metric))
        data = OpenWeatherMapWeatherData(raw_data)
        print_out(raw_data, data, json, true_no_color, true_metric)
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
    raw_data = asyncio.run(get_combined_data(get_coordinates(location), metric))
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

    print_out(raw_data, data, ctx.obj["JSON"] or json, true_no_color, true_metric)


@main.command(['config', 'setting', 'c'], help="prints or changes the settings")
@argument('key_name')
@option('--value', help='This sets the key')
@pass_context
def test(ctx, key_name: str, value):
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


if __name__ == '__main__':
    main(obj={})
