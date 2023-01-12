"""Alternative Weather Backend weather.com, but has to be carefully scraped"""
import math
import ssl
from datetime import datetime

import certifi
import core
from bs4 import BeautifulSoup
from core import WindData

from cli import WeatherData


class TheWeatherChannel(WeatherData):
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
        high, low = self.get_high_low(weather_soup)
        wind_data = forecast_soup.find("span", attrs={"data-testid": "Wind"}).getText()
        compass = {
            "N": 0,
            "NE": 45,
            "E": 90,
            "SE": 125,
            "S": 180,
            "SW": 225,
            "W": 270,
            "NW": 315,
        }
        heading = 0
        for key in compass:
            if key in wind_data:
                heading = compass[key]
        speed = ""
        for i in wind_data:
            if i.isdigit():
                speed += i
        wind = WindData(int(speed), heading)
        super().__init__(
            status="200",
            time=datetime.now,
            temperature=self.get_temp(weather_soup),
            min_temp=low,
            max_temp=high,
            region=region,
            wind=wind,
            raw_data=r,
            aqi=self.get_air_quality(air_quality_soup) // 20,
            forecast=[],
            country=country,
            cloud_cover=0,
            conditions=[],
            condition_sentence="WIP",
            forecast_sentence="WIP",
        )

    def get_air_quality(self, soup):
        return int(
            soup.find("text", attrs={"data-testid": "DonutChartValue"}).getText()
        )

    def get_temp(self, soup):
        return int(
            soup.find("div", attrs={"data-testid": "CurrentConditionsContainer"})
            .find("span", attrs={"data-testid": "TemperatureValue"})
            .getText()
            .replace("°", "")
        )

    def get_high_low(self, soup):
        data = soup.find("div", attrs={"data-testid": "wxData"}).text.replace("°", "")
        high_low = data.split("/")
        if high_low[0] == "--":
            high_low[0] = math.nan
        if high_low[1] == "--":
            high_low[1] = math.nan
        return float(high_low[0]), float(high_low[1])
