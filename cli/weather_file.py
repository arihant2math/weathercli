import json
import os
import pathlib


class WeatherFile:
    def __init__(self, file_name):
        directory = pathlib.Path.home() / ".weathercli"
        if not directory.exists():
            os.mkdir(directory)
        self.file = directory / file_name
        if not self.file.exists():
            with open(self.file, 'w') as f:
                f.write('{}')
        with open(self.file, 'r') as f:
            self.data = json.load(f)

    def write(self):
        # Serializing json
        json_object = json.dumps(self.data, indent=4)
        # Writing to sample.json
        with open(self.file, "w") as f:
            f.write(json_object)
