import cython

import api_keys


@cython.compile
def weather(location, metric):
    out = api_keys.OPEN_WEATHER_MAP_API_URL + "weather?lat=" + str(location[0]) + "&lon=" + str(
        location[1]) + "&appid=" + api_keys.OPEN_WEATHER_MAP_API_KEY
    if not metric:
        return out + "&units=imperial"
    else:
        return out + "&units=metric"


@cython.compile
def air_quality(location, metric):
    return (
            api_keys.OPEN_WEATHER_MAP_API_URL + "air_pollution?lat=" + str(location[0]) + "&lon=" + str(location[1]) +
            "&appid=" + api_keys.OPEN_WEATHER_MAP_API_KEY
            + "&units=imperial")


@cython.compile
def forecast(location, metric):
    out = api_keys.OPEN_WEATHER_MAP_API_URL + "forecast?lat=" + str(location[0]) + "&lon=" + str(
        location[1]) + "&appid=" + api_keys.OPEN_WEATHER_MAP_API_KEY
    if not metric:
        return out + "&units=imperial"
    else:
        return out + "&units=metric"
