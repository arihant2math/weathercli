from bs4 import BeautifulSoup
from core import networking

from cli.backend.theweatherchannel.the_weather_channel_current import (
    TheWeatherChannelCurrent,
)
from cli.backend.weather_forecast import WeatherForecast


class TheWeatherChannelForecast(WeatherForecast):
    def __init__(self, loc, metric, settings):
        region, country = self.get_location(loc)
        if not metric:
            cookies = {"unitOfMeasurement": "e"}
        else:
            cookies = {"unitOfMeasurement", "m"}
        r1 = networking.get_url(
            "https://weather.com/weather/today/l/" + loc[0] + "," + loc[1],
            cookies=cookies,
        )
        r2 = networking.get_url(
            "https://weather.com/weather/hourbyhour/l/" + loc[0] + "," + loc[1],
            cookies=cookies,
        )
        r3 = networking.get_url(
            "https://weather.com/forecast/air-quality/l/" + loc[0] + "," + loc[1],
            cookies=cookies,
        )
        weather_soup = BeautifulSoup(r1.text, "html.parser")
        forecast_soup = BeautifulSoup(r2.text, "html.parser")
        air_quality_soup = BeautifulSoup(r3.text, "html.parser")
        forecast = [
            TheWeatherChannelCurrent(weather_soup, forecast_soup, air_quality_soup)
        ]
        super().__init__(0, region, country, forecast, "WIP", [r1, r2, r3])
