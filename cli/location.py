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
        if attempt_cache is None:
            location = core.get_location(no_sys_loc)
            core.caching.write("current_location", ",".join(location))
            return location
        else:
            t = threading.Thread(target=core.caching.update_hits, args=["current_location"])
            t.start()
            return attempt_cache.split(",")
    return core.get_location(no_sys_loc)


def get_coordinates(l: str):
    attempt_cache = core.caching.read("location" + l)
    if attempt_cache is None:
        if settings.BING_MAPS_API_KEY != "":
            geolocator = Bing(
                api_key=settings.BING_MAPS_API_KEY, user_agent="weathercli"
            )
        else:
            geolocator = Nominatim(user_agent="weathercli")
        try:
            coordinates = geolocator.geocode(l, timeout=10000)
        except:
            geolocator = Nominatim(user_agent="weathercli")
            coordinates = geolocator.geocode(l, timeout=10000)
        if coordinates is None:
            raise LookupError("No such place exists")
        r_value = [str(coordinates.latitude), str(coordinates.longitude)]
        core.caching.write("location" + l.lower().strip(), ",".join(r_value))
        return r_value
    else:
        t = threading.Thread(target=core.caching.update_hits, args=["location" + l])
        t.start()
        return attempt_cache.split(",")


def reverse_location(latitude: float, longitude: float) -> tuple[str, str]:
    ctx = ssl.create_default_context(cafile=certifi.where())
    geopy.geocoders.options.default_ssl_context = ctx
    k = str(latitude) + "," + str(longitude)
    attempt_cache = core.caching.read("coordinates" + k)
    if attempt_cache is None:
        geolocator = Nominatim(user_agent="weathercli")
        place: Location = geolocator.reverse(k, timeout=10000)
        if place is None:
            raise LookupError("No such place exists")
        del place.raw["licence"]
        reversed_location = place.raw
        country = reversed_location["address"]["country"]
        if "city" in reversed_location["address"]:
            region = reversed_location["address"]["city"]
        elif "county" in reversed_location["address"]:
            region = reversed_location["address"]["county"]
        else:
            region = ""
        core.caching.write("coordinate" + k, region + ",?`|" + country)
        return region, country

    else:
        t = threading.Thread(target=core.caching.update_hits, args=["coordinates" + k])
        t.start()
        reversed_location = attempt_cache.split(",?`|")
        return reversed_location[0], reversed_location[1]
