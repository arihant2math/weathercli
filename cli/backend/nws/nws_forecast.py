import json

import core

from cli import WeatherForecast
from cli.backend.nws.nws_current import NationalWeatherServiceCurrent


class NationalWeatherServiceForecast(WeatherForecast):
    def __init__(self, loc, metric):
        location = self.get_location(loc)
        country = location.raw["address"]["country"]
        region = location.raw["address"]["city"]
        get_point = core.networking.get_url(
            "https://api.weather.gov/points/" + loc[0] + "," + loc[1]
        )
        point_json = json.loads(get_point)
        office = point_json["properties"]["cwa"]
        grid_location = [
            point_json["properties"]["gridX"],
            point_json["properties"]["gridY"],
        ]
        raw_data = core.networking.get_url(
            "https://api.weather.gov/gridpoints/{}/{},{}/".format(
                office, grid_location[0], grid_location[1]
            )
        )
        data = json.loads(raw_data)
        forecast = [NationalWeatherServiceCurrent(data, metric)]
        super().__init__(0, region, country, forecast, "WIP", data)
