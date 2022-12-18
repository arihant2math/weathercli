import core


# TODO: Port to Rust


class WeatherData:  # TODO: Abstract more attributes
    def __init__(
        self,
        temperature: int,
        min_temp: int,
        max_temp: int,
        region: str,
        wind: core.WindData,
        raw_data,
    ):
        self.temperature = temperature
        self.min_temp = min_temp
        self.max_temp = max_temp
        self.region = region
        self.wind = wind
        self.raw_data = raw_data
