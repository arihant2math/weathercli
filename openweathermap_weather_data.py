from weather_data import WeatherData
from wind_data import WindData


class OpenWeatherMapWeatherData(WeatherData):
    def __init__(self, data: dict):
        super().__init__(data['main']['temp'])
        if 'cod' in data:
            self.status = data['cod']
        else:
            self.status = 200
        self.aqi = data['air_quality']['main']['aqi']
        self.region = data['name']
        self.country = data['sys']['country']
        self.wind = WindData(data['wind'])
        self.cloud_cover = data['clouds']['all']
        self.weather = data['weather']
