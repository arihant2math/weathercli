import json
import os.path

from core import hash_file


def update_hash(file, key):
    f = json.load(open(file))
    file_hash = hash_file(os.path.abspath(file))
    index_json = json.load(open("./docs_templates/index.json"))
    index_json[key] = file_hash
    json.dump(index_json, open("./docs_templates/index.json", "w"))


update_hash("./weather_codes.json", "weather-codes-hash")
update_hash("./weather_ascii_images.json", "weather-ascii-images-hash")
