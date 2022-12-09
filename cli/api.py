from cli import settings


def weather(location, metric):
    out = settings.OPEN_WEATHER_MAP_API_URL + "weather?lat=" + str(location[0]) + "&lon=" + str(
        location[1]) + "&appid=" + settings.OPEN_WEATHER_MAP_API_KEY
    if not metric:
        return out + "&units=imperial"
    else:
        return out + "&units=metric"


def air_quality(location, metric):
    return (
            settings.OPEN_WEATHER_MAP_API_URL + "air_pollution?lat=" + str(location[0]) + "&lon=" + str(location[1]) +
            "&appid=" + settings.OPEN_WEATHER_MAP_API_KEY
            + "&units=imperial")


def forecast(location, metric):
    out = settings.OPEN_WEATHER_MAP_API_URL + "forecast?lat=" + str(location[0]) + "&lon=" + str(
        location[1]) + "&appid=" + settings.OPEN_WEATHER_MAP_API_KEY
    if not metric:
        return out + "&units=imperial"
    else:
        return out + "&units=metric"


