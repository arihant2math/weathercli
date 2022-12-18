import core
from cli.settings import WEATHER_DATA_HASH, store_key
from cli.weather_file import WeatherFile
import hashlib


class OpenWeatherMapConditions:
    def __init__(self, data):
        self.id: int = data["id"]
        self.name = data["main"]
        self.description = data["description"]
        self.icon = data["icon"]
        self.sentence = self.get_sentence()

    def get_sentence(self):
        f = WeatherFile("weather_codes.json")
        file_hash = core.hash_file(str(f.path.absolute()))
        if WEATHER_DATA_HASH != file_hash:
            print(
                "Warning: weather_codes.json is out of date or has been modified, downloading replacement."
            )
            data = core.get_urls(
                ["https://arihant2math.github.io/weathercli/weather_codes.json"]
            )[0]
            with open(f.path, "w") as out:
                out.write(data)
            new_file_hash = core.hash_file(str(f.path.absolute()))
            store_key("WEATHER_DATA_HASH", new_file_hash)
            f = WeatherFile("weather_codes.json")

        if str(self.id) in f.data:
            return f.data[str(self.id)][3]
        return "Unknown Conditions, condition id=" + str(self.id)
