from cli.weather_data import WeatherData
from cli.wind_data import WindData


class OpenWeatherMapWeatherData(WeatherData):
    def __init__(self, data: dict):
        super().__init__(data['main']['temp'], data['name'])
        if 'cod' in data:
            self.status: int = data['cod']
        else:
            self.status: int = 200
        self.aqi: int = data['air_quality']['main']['aqi']
        self.country: str = data['sys']['country']
        self.wind: WindData = WindData(data['wind'])
        self.cloud_cover: int = data['clouds']['all']
        self.weather = data['weather']
