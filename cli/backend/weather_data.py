import core

# TODO: Port to Rust


class WeatherData:  # TODO: Abstract more attributes
    def __init__(
        self,
        time,
        temperature: int,
        min_temp: float,
        max_temp: float,
        wind: core.WindData,
        dewpoint,
        feels_like,
        aqi: int,
        cloud_cover,
        conditions: list,
        condition_sentence,
    ):
        self.time = time
        self.temperature = temperature
        self.min_temp = min_temp
        self.max_temp = max_temp
        self.wind = wind
        self.dewpoint = dewpoint
        self.feels_like = feels_like
        self.aqi = aqi
        self.cloud_cover = cloud_cover
        self.conditions = conditions
        self.condition_sentence = condition_sentence

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

    def get_condition_ids(self):
        ids = []
        for condition in self.conditions:
            ids.append(condition.condition_id)
        return ids
