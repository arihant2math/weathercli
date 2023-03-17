import json
import threading

import core

from cli.local import settings


def bing_maps_location_query(query):
    r = core.networking.get_url(
        'http://dev.virtualearth.net/REST/v1/Locations?query="{}"&maxResults=5&key={}'
        "".format(query, settings.BING_MAPS_API_KEY)
    )
    j = json.loads(r.text)["resourceSets"][0]["resources"][0]["point"]["coordinates"]
    return j[0], j[1]


def nominatim_geocode(query):
    r = core.networking.get_url(
        'https://nominatim.openstreetmap.org/search?q="{}"&format=jsonv2'.format(query)
    )
    j = json.loads(r.text)
    return j[0]["lat"], j[0]["lon"]


def nominatim_reverse_geocode(lat, lon):
    r = core.networking.get_url(
        "https://nominatim.openstreetmap.org/reverse?lat={}&lon={}&format=jsonv2".format(
            lat, lon
        )
    )
    return json.loads(r.text)


def get_location(no_sys_loc):
    if settings.CONSTANT_LOCATION:
        attempt_cache = core.caching.read("current_location")
        if attempt_cache is None:
            location = core.get_location(no_sys_loc)
            core.caching.write("current_location", ",".join(location))
            return location
        else:
            t = threading.Thread(
                target=core.caching.update_hits, args=["current_location"]
            )
            t.start()
            return attempt_cache.split(",")
    return core.get_location(no_sys_loc)


def get_coordinates(location_string: str):
    attempt_cache = core.caching.read("location" + location_string)
    if attempt_cache is None:
        if settings.BING_MAPS_API_KEY != "":
            try:
                coordinates = bing_maps_location_query(location_string)
            except:
                coordinates = nominatim_geocode(location_string)
        else:
            coordinates = nominatim_geocode(location_string)
        if coordinates is None:
            raise LookupError("No such place exists")
        core.caching.write(
            "location" + location_string.lower().strip(), ",".join(coordinates)
        )
        return coordinates
    else:
        t = threading.Thread(
            target=core.caching.update_hits, args=["location" + location_string]
        )
        t.start()
        return attempt_cache.split(",")


def reverse_location(latitude: float, longitude: float) -> tuple[str, str]:
    k = str(latitude) + "," + str(longitude)
    attempt_cache = core.caching.read("coordinates" + k)
    if attempt_cache is None:
        place = nominatim_reverse_geocode(latitude, longitude)
        country = place["address"]["country"]
        if "city" in place["address"]:
            region = place["address"]["city"]
        elif "county" in place["address"]:
            region = place["address"]["county"]
        else:
            region = ""
        core.caching.write("coordinate" + k, region + ",?`|" + country)
        return region, country

    else:
        t = threading.Thread(target=core.caching.update_hits, args=["coordinates" + k])
        t.start()
        reversed_location = attempt_cache.split(",?`|")
        return reversed_location[0], reversed_location[1]
