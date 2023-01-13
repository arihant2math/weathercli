from cli.local.weather_file import WeatherFile


class WeatherCondition:
    def __init__(self, condition_id):
        self.condition_id = condition_id
        self.sentence = self.get_sentence()

    def get_sentence(self):
        f = WeatherFile("weather_codes.json")
        if str(self.condition_id) in f.data:
            return f.data[str(self.condition_id)][3]
        return "Unknown Conditions, condition id=" + str(self.condition_id)
