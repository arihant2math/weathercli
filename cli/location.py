import asyncio
import platform

import requests
from geopy import Bing, Nominatim
if platform.system() == "Windows":
    from winsdk.windows.devices import geolocation as wdg

from cli import api_keys


async def get_device_location_windows():
    locator = wdg.Geolocator()
    pos = await locator.get_geoposition_async()
    return [str(pos.coordinate.latitude), str(pos.coordinate.longitude)]


def get_device_location_web():
    return requests.get("https://ipinfo.io").json()["loc"].split(",")


def get_device_location(no_sys_loc=False):
    if platform.system() == "Windows" and not no_sys_loc:
        try:
            return asyncio.run(get_device_location_windows())
        except PermissionError:
            return get_device_location_web()
    return get_device_location_web()


def get_coordinates(location):
    if api_keys.BING_MAPS_API_KEY != "":
        geolocator = Bing(api_key=api_keys.BING_MAPS_API_KEY, user_agent="weathercli")
    else:
        geolocator = Nominatim(user_agent="weathercli")
    coordinates = geolocator.geocode(location, timeout=10000)
    return [coordinates.latitude, coordinates.longitude]
