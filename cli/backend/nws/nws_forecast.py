import json

import core

from cli.backend.nws.nws_current import NationalWeatherServiceCurrent
from cli.backend.weather_forecast import WeatherForecast


class NationalWeatherServiceForecast(WeatherForecast):
    def __init__(self, loc, metric):
        region, country = self.get_location(loc)
        get_point = core.networking.get_url(
            "https://api.weather.gov/points/" + loc[0] + "," + loc[1]
        ).text
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
        ).text
        data = json.loads(raw_data)
        forecast = [NationalWeatherServiceCurrent(data, metric)]
        super().__init__(0, region, country, forecast, "WIP", data)
