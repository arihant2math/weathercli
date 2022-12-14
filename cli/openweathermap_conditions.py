import json

import core
from cli.weather_file import WeatherFile
import hashlib


def hash_file(filename):
    """"This function returns the SHA-1 hash
   of the file passed into it"""

    # make a hash object
    h = hashlib.sha1()

    # open file for reading in binary mode
    with open(filename, 'rb') as file:
        # loop till the end of the file
        chunk = 0
        while chunk != b'':
            # read only 1024 bytes at a time
            chunk = file.read(1024)
            h.update(chunk)

    # return the hex representation of digest
    return h.hexdigest()


class OpenWeatherMapConditions:
    def __init__(self, data):
        self.id: int = data["id"]
        self.name = data["main"]
        self.description = data["description"]
        self.icon = data["icon"]
        self.sentence = self.get_sentence()

    def get_sentence(self):
        f = WeatherFile("weather_codes.json")
        true_hash = "b5c4e4f4c9c210b77b1ef94a4d440cd057cc900a"
        file_hash = hash_file(f.path)
        if true_hash != file_hash:
            print("Warning: weather_codes.json is out of date or has been modified, downloading replacement.")
            data = core.get_urls(["https://raw.githubusercontent.com/arihant2math/weathercli/main/weather_codes.json"])[0]
            with open(f.path, 'w') as out:
                out.write(data)
            f = WeatherFile("weather_codes.json")

        if str(self.id) in f.data:
            return f.data[str(self.id)][3]
        return "Unknown Conditions, condition id=" + str(self.id)
