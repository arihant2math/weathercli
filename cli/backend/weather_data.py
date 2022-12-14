import core


# TODO: Port to Rust


class WeatherData:  # TODO: Abstract more attributes
    def __init__(
        self,
        status: str,
        temperature: int,
        min_temp: float,
        max_temp: float,
        region: str,
        wind: core.WindData,
        raw_data,
        aqi: int,
        forecast,
        country,
        cloud_cover,
        conditions,
        condition_sentence,
        forecast_sentence,
    ):
        self.status = status
        self.temperature = temperature
        self.min_temp = min_temp
        self.max_temp = max_temp
        self.region = region
        self.wind = wind
        self.raw_data = raw_data
        self.aqi = aqi
        self.forecast = forecast
        self.country = country
        self.cloud_cover = cloud_cover
        self.conditions = conditions
        self.condition_sentence = condition_sentence
        self.forecast_sentence = forecast_sentence

    def get_condition_sentence(self):
        data = self.conditions.copy()
        condition_match = data[0].sentence
        out = condition_match
        data.pop(0)
        for condition in data:
            out += ". Also, "
            condition_match = condition.sentence
            out += condition_match.lower()
        out += "."
        return out
