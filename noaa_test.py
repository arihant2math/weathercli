"""Historical Weather Data"""
import ssl
import certifi

import core
from core import networking
import geopy
from geopy import Nominatim


class NOAA:
    def __init__(self, loc):
        ctx = ssl.create_default_context(cafile=certifi.where())
        geopy.geocoders.options.default_ssl_context = ctx
        geolocator = Nominatim(user_agent="weathercli/0", scheme="http")
        location = geolocator.reverse(loc[0] + ", " + loc[1])
        country = location.raw["address"]["country"]
        region = location.raw["address"]["city"]
        get_point = networking.get_url(
            "https://www.ncei.noaa.gov/cdo-web/api/v2/datasets"
        )
        print(get_point)


noaa = NOAA(core.get_location(False))
