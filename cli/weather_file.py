import json
import os
import pathlib


class WeatherFile:
    def __init__(self, file_name):
        directory = pathlib.Path.home() / ".weathercli"
        if not directory.exists():
            os.mkdir(directory)
        self.path = directory / file_name
        if not self.path.exists():
            with open(self.path, "w") as f:
                f.write("{}")
        with open(self.path, "r") as f:
            self.data = json.load(f)

    def write(self):
        # Serializing json
        json_object = json.dumps(self.data, indent=4)
        # Writing to sample.json
        with open(self.path, "w") as f:
            f.write(json_object)
