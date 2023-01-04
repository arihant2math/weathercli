import core

from cli.settings import WEATHER_DATA_HASH, store_key
from cli.weather_file import WeatherFile


class WeatherCondition:
    def __init__(self, condition_id):
        self.condition_id = condition_id
        self.sentence = self.get_sentence()

    def get_sentence(self):
        f = WeatherFile("weather_codes.json")
        file_hash = core.hash_file(str(f.path.absolute()))
        if WEATHER_DATA_HASH != file_hash:
            print(
                "Warning: weather_codes.json is out of date or has been modified, downloading replacement."
            )
            data = core.networking.get_url(
                "https://arihant2math.github.io/weathercli/weather_codes.json"
            )
            with open(f.path, "w") as out:
                out.write(data)
            new_file_hash = core.hash_file(str(f.path.absolute()))
            store_key("WEATHER_DATA_HASH", new_file_hash)
            f = WeatherFile("weather_codes.json")

        if str(self.condition_id) in f.data:
            return f.data[str(self.condition_id)][3]
        return "Unknown Conditions, condition id=" + str(self.condition_id)
