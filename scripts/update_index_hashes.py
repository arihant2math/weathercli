import json
import os.path

from core import hash_file

from path_helper import weathercli_dir


def update_hash(file, key):
    file_hash = hash_file(os.path.abspath(str(file)))
    docs_index = weathercli_dir / "docs_templates" / "index.json"
    index_json = json.load(docs_index.open())
    index_json[key] = file_hash
    json.dump(index_json, docs_index.open("w"))


if __name__ == "__main__":
    update_hash(weathercli_dir / "weather_codes.json", "weather-codes-hash")
    update_hash(
        weathercli_dir / "weather_ascii_images.json", "weather-ascii-images-hash"
    )
    update_hash(
        weathercli_dir / "docs_templates" / "weather.exe", "weather-exe-hash-windows"
    )
    update_hash(weathercli_dir / "docs_templates" / "weather", "weather-exe-hash-unix")
    update_hash(
        weathercli_dir / "docs_templates" / "updater.exe", "updater-exe-hash-windows"
    )
    update_hash(weathercli_dir / "docs_templates" / "updater", "updater-exe-hash-unix")
    update_hash(
        weathercli_dir / "docs_templates" / "weatherd.exe", "weatherd-exe-hash-windows"
    )
    update_hash(
        weathercli_dir / "docs_templates" / "weatherd", "weatherd-exe-hash-unix"
    )
