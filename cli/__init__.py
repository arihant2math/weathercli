import colorama
from cli.dummy_fore import DummyFore
from cli.backend.weather_data import WeatherData
import rich

from cli.layout import Layout


def print_out(data: WeatherData, print_json: bool, metric: bool):
    color = colorama.Fore
    if print_json:
        try:
            rich.print_json(data.raw_data)
        except:
            print(data.raw_data)
    elif data.status:
        out = Layout()
        print(out.to_string(data, metric))
    else:
        print(color.RED + data.raw_data["message"] + color.RESET, end="")
