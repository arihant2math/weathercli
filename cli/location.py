import json
import ssl
import threading

import certifi
import core
import geopy
from geopy import Bing, Nominatim, Location

from cli.local import settings


def get_location(no_sys_loc):
    if settings.CONSTANT_LOCATION:
        attempt_cache = core.caching.read("current_location")
        t = threading.Thread(target=core.caching.update_hits, args=["current_location"])
        t.start()
        if attempt_cache is None:
            location = core.get_location(no_sys_loc)
            core.caching.write("current_location", ",".join(location))
            return location
        else:
            return attempt_cache.split(",")
    return core.get_location(no_sys_loc)


def get_coordinates(l: str):
    attempt_cache = core.caching.read("location" + l)
    t = threading.Thread(target=core.caching.update_hits, args=["location" + l])
    t.start()
    if attempt_cache is None:
        if settings.BING_MAPS_API_KEY != "":
            geolocator = Bing(
                api_key=settings.BING_MAPS_API_KEY, user_agent="weathercli"
            )
        else:
            geolocator = Nominatim(user_agent="weathercli")
        coordinates = geolocator.geocode(l, timeout=10000)
        if coordinates is None:
            raise LookupError("No such place exists")
        r_value = [str(coordinates.latitude), str(coordinates.longitude)]
        core.caching.write("location" + l.lower().strip(), ",".join(r_value))
        return r_value
    else:
        return attempt_cache.split(",")


def reverse_location(latitude: float, longitude: float) -> dict:
    ctx = ssl.create_default_context(cafile=certifi.where())
    geopy.geocoders.options.default_ssl_context = ctx
    k = str(latitude) + "," + str(longitude)
    attempt_cache = core.caching.read("coordinates" + k)
    t = threading.Thread(target=core.caching.update_hits, args=["coordinates" + k])
    t.start()
    if attempt_cache is None:
        geolocator = Nominatim(user_agent="weathercli")
        place: Location = geolocator.reverse(k, timeout=10000)
        if place is None:
            raise LookupError("No such place exists")
        core.caching.write("coordinate" + k, json.dumps(place.raw))
        return place.raw
    else:
        return json.loads(attempt_cache)
