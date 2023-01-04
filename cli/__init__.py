import colorama
from core import color_value
from cli.dummy_fore import DummyFore
from cli.backend.weather_data import WeatherData


def print_out(data: WeatherData, print_json: bool, no_color: bool, metric: bool):
    if not no_color:
        color = colorama.Fore
    else:
        color = DummyFore
    if print_json:
        print(data.raw_data)
    elif data.status:
        print(
            color.LIGHTBLUE_EX
            + "Weather for "
            + color_value(data.region + ", " + data.country, None, not no_color)
        )
        print(color.LIGHTMAGENTA_EX + data.condition_sentence)
        print(color.LIGHTMAGENTA_EX + data.forecast_sentence)
        if metric:
            degree_ending = "° C"
        else:
            degree_ending = "° F"
        print(
            color.LIGHTBLUE_EX
            + "Temperature: "
            + color_value(str(data.temperature), degree_ending, not no_color),
            end="",
        )
        print(
            " with a min of {} and a max of {}".format(
                color_value(str(data.min_temp), degree_ending, not no_color),
                color_value(str(data.max_temp), degree_ending, not no_color),
            )
        )
        print(
            color.LIGHTBLUE_EX
            + "Wind: "
            + color.LIGHTGREEN_EX
            + str(data.wind.speed)
            + color.MAGENTA,
            end=" ",
        )
        if metric:
            print("km/h", end=" ")
        else:
            print("mph", end=" ")
        print(
            color.LIGHTBLUE_EX
            + "at "
            + color_value(str(data.wind.heading), "°", not no_color)
        )
        if data.cloud_cover != 0:
            print(
                color.LIGHTBLUE_EX
                + "Cloud Cover: "
                + color_value(str(data.cloud_cover), "%", not no_color)
            )
        aqi = data.aqi
        aqi_color = color.LIGHTYELLOW_EX
        if aqi == 5:
            aqi_color = color.RED
        elif aqi < 3:
            aqi_color = color.LIGHTGREEN_EX
        print(color.LIGHTBLUE_EX + "AQI: " + aqi_color + str(aqi) + color.RESET)
    else:
        print(color.RED + data.raw_data["message"] + color.RESET)
