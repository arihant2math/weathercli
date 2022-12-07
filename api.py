import api_keys


def weather(location, metric):
    out = api_keys.OPEN_WEATHER_MAP_API_URL + "weather?lat=" + str(location[0]) + "&lon=" + str(
        location[1]) + "&appid=" + api_keys.OPEN_WEATHER_MAP_API_KEY
    if not metric:
        return out + "&units=imperial"
    else:
        return out + "&units=metric"


def air_quality(location, metric):
    return (
            api_keys.OPEN_WEATHER_MAP_API_URL + "air_pollution?lat=" + str(location[0]) + "&lon=" + str(location[1]) +
            "&appid=" + api_keys.OPEN_WEATHER_MAP_API_KEY
            + "&units=imperial")


def forecast(location, metric):
    out = api_keys.OPEN_WEATHER_MAP_API_URL + "forecast?lat=" + str(location[0]) + "&lon=" + str(
        location[1]) + "&appid=" + api_keys.OPEN_WEATHER_MAP_API_KEY
    if not metric:
        return out + "&units=imperial"
    else:
        return out + "&units=metric"
