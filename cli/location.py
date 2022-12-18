from geopy import Bing, Nominatim
from cli import settings, cache


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
        r_value = [str(coordinates.latitude), str(coordinates.longitude)]
        cache.add_data("location", location.lower().strip(), ",".join(r_value))
        return r_value
    else:
        return attempt_cache.split(",")
