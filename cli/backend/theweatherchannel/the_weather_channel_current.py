"""Alternative Weather Backend weather.com, but has to be carefully scraped"""
import math
import time
from datetime import datetime

from core.backend import WeatherData
from core.backend import WindData


class TheWeatherChannelCurrent(WeatherData):
    def __new__(cls, *args, **kwargs):
        return super().__new__(cls)

    def __init__(self, weather_soup, forecast_soup, air_quality_soup):
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
        self.time = int(time.mktime(datetime.now().timetuple()))
        self.temperature = self.get_temp(weather_soup)
        self.min_temp = low
        self.max_temp = high
        self.wind = wind
        self.dewpoint = 0
        self.feels_like = 0
        self.aqi = self.get_air_quality(air_quality_soup) // 20
        self.cloud_cover = 0
        self.conditions = []
        self.condition_sentence = "WIP"

    @staticmethod
    def get_air_quality(soup):
        return int(
            soup.find("text", attrs={"data-testid": "DonutChartValue"}).getText()
        )

    @staticmethod
    def get_temp(soup):
        return int(
            soup.find("div", attrs={"data-testid": "CurrentConditionsContainer"})
            .find("span", attrs={"data-testid": "TemperatureValue"})
            .getText()
            .replace("°", "")
        )

    @staticmethod
    def get_high_low(soup):
        data = soup.find("div", attrs={"data-testid": "wxData"}).text.replace("°", "")
        high_low = data.split("/")
        if high_low[0] == "--":
            high_low[0] = math.nan
        if high_low[1] == "--":
            high_low[1] = math.nan
        return float(high_low[0]), float(high_low[1])
