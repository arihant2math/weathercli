"""Historical Weather Data"""
import core
from core import networking


class NOAA:
    def __init__(self, loc):
        get_point = networking.get_url(
            "https://www.ncei.noaa.gov/cdo-web/api/v2/datasets"
        ).text
        print(get_point)


noaa = NOAA(core.get_location(False))
