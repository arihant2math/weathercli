from cli.backend.weather_condition import WeatherCondition


class OpenWeatherMapWeatherCondition(WeatherCondition):
    def __init__(self, data):
        super().__init__(data.id)
