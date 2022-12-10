import platform
import core
import requests
from geopy import Bing, Nominatim
from cli import settings


def get_device_location_web():
    return requests.get("https://ipinfo.io").json()["loc"].split(",")


def get_device_location(no_sys_loc=False):
    if platform.system() == "Windows" and not no_sys_loc:
        try:
            return core.get_location_windows()
        except PermissionError:
            return get_device_location_web()
    return get_device_location_web()


def get_coordinates(location):
    if settings.BING_MAPS_API_KEY != "":
        geolocator = Bing(api_key=settings.BING_MAPS_API_KEY, user_agent="weathercli")
    else:
        geolocator = Nominatim(user_agent="weathercli")
    coordinates = geolocator.geocode(location, timeout=10000)
    return [coordinates.latitude, coordinates.longitude]
