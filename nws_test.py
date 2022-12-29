"""Alternative Weather Backend weather.com, but has to be carefully scraped"""
import json

import core
from core import WindData
from cli import WeatherData, print_out


class NationalWeatherService(WeatherData):
    def __init__(self, loc):
        # ctx = ssl.create_default_context(cafile=certifi.where())
        # geopy.geocoders.options.default_ssl_context = ctx
        # geolocator = Nominatim(user_agent="weathercli/0", scheme='http')
        # location = geolocator.reverse(loc[0] + ", " + loc[1])
        # country = location.raw['address']['country']
        get_point = core.networking.get_url("https://api.weather.gov/points/" + loc[0] + "," + loc[1])
        point_json = json.loads(get_point)
        office = point_json["properties"]["cwa"]
        grid_location = [point_json["properties"]["gridX"], point_json["properties"]["gridY"]]
        forecast = core.networking.get_url("https://api.weather.gov/gridpoints/{}/{},{}/forecast".format(office,
                                                                                                         grid_location[0],
                                                                                                         grid_location[1]))
        forecast_json = json.loads(forecast)
        super().__init__(
            status=str(forecast_json["status"]),
            temperature=temp,
            min_temp=low,
            max_temp=high,
            region=region,
            wind=wind,
            raw_data=r,
            aqi=self.get_air_quality(air_quality_soup),
            forecast=[],
            country=country,
            cloud_cover=0,
            conditions=[],
            condition_sentence="WIP",
            forecast_sentence="WIP",
        )


w = NationalWeatherService(core.get_location(False))

print_out(w, False, False, False)
