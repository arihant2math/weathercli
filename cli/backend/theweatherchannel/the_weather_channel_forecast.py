from bs4 import BeautifulSoup

from cli import WeatherForecast
import core

from cli.backend.theweatherchannel.the_weather_channel_current import (
    TheWeatherChannelCurrent,
)


class TheWeatherChannelForecast(WeatherForecast):
    def __init__(self, loc):
        location = self.get_location(loc)
        country = location.raw["address"]["country"]
        region = location.raw["address"]["city"]
        r = core.networking.get_urls(
            [
                "https://weather.com/weather/today/l/" + loc[0] + "," + loc[1],
                "https://weather.com/weather/hourbyhour/l/" + loc[0] + "," + loc[1],
                "https://weather.com/forecast/air-quality/l/" + loc[0] + "," + loc[1],
            ]
        )
        weather_soup = BeautifulSoup(r[0], "html.parser")
        forecast_soup = BeautifulSoup(r[1], "html.parser")
        air_quality_soup = BeautifulSoup(r[2], "html.parser")
        forecast = [
            TheWeatherChannelCurrent(weather_soup, forecast_soup, air_quality_soup)
        ]
        super().__init__(0, region, country, forecast, "WIP", r)
