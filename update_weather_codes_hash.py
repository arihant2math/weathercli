import json
import os.path

from core import hash_file

f = json.load(open("./weather_codes.json"))
file_hash = hash_file(os.path.abspath("./weather_codes.json"))
index_json = json.load(open("./docs_templates/index.json"))
index_json["weather-codes-hash"] = file_hash
json.dump(index_json, open("./docs_templates/index.json", "w"))
