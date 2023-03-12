import json
import os.path

from core import hash_file


def update_hash(file, key):
    file_hash = hash_file(os.path.abspath(file))
    index_json = json.load(open("./docs_templates/index.json"))
    index_json[key] = file_hash
    json.dump(index_json, open("./docs_templates/index.json", "w"))


if __name__ == "__main__":
    update_hash("./weather_codes.json", "weather-codes-hash")
    update_hash("./weather_ascii_images.json", "weather-ascii-images-hash")
    update_hash("./docs_templates/weather.exe", "weather-exe-hash-windows")
    update_hash("./docs_templates/weather", "weather-exe-hash-unix")
    update_hash("./docs_templates/updater.exe", "updater-exe-hash-windows")
    update_hash("./docs_templates/updater", "updater-exe-hash-unix")
    update_hash("./docs_templates/weatherd.exe", "weatherd-exe-hash-windows")
    update_hash("./docs_templates/weatherd", "weatherd-exe-hash-unix")
