import core
from geopy import Bing, Nominatim

from cli.local import cache, settings


def get_location(no_sys_loc):
    if settings.CONSTANT_LOCATION:
        attempt_cache = cache.get_key("current_location", "now")
        if attempt_cache is None:
            location = core.get_location(no_sys_loc)
            cache.add_data("current_location", "now", ",".join(location))
            return location
        else:
            return attempt_cache.split(",")
    return core.get_location(no_sys_loc)


def get_coordinates(location: str):
    attempt_cache = cache.get_key("location", location)
    if attempt_cache is None:
        if settings.BING_MAPS_API_KEY != "":
            geolocator = Bing(
                api_key=settings.BING_MAPS_API_KEY, user_agent="weathercli"
            )
        else:
            geolocator = Nominatim(user_agent="weathercli")
        coordinates = geolocator.geocode(location, timeout=10000)
        if coordinates is None:
            raise LookupError("No such place exists")
        r_value = [str(coordinates.latitude), str(coordinates.longitude)]
        cache.add_data("location", location.lower().strip(), ",".join(r_value))
        return r_value
    else:
        return attempt_cache.split(",")
