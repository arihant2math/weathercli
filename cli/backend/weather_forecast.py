# TODO: Port to Rust
import weather_core


class WeatherForecast:
    def __init__(
        self,
        status: int,
        region: str,
        country,
        forecast: list,
        forecast_sentence: str,
        raw_data,
    ):
        """
        :param status: 0 is success, 10 is invalid API key, 11 is invalid client request, 20 is server error,
        :param region:
        :param country:
        :param forecast:
        :param forecast_sentence:
        :param raw_data:
        """
        self.status = status
        self.region = region
        self.country = country
        self.forecast = forecast
        self.current_weather = forecast[0]
        self.forecast_sentence = forecast_sentence
        self.raw_data = raw_data

    @staticmethod
    def get_location(loc):
        return weather_core.location.reverse_location(loc[0], loc[1])
