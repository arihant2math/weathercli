import shutil
import subprocess
import sys
from pathlib import Path

import click
import colorama


@click.group()
def main():
    pass


@main.command("docs", help="Builds the docs")
def docs():
    Path("./docs").mkdir(exist_ok=True)
    Path("./docs/docs").mkdir(exist_ok=True)  # for backwards compatibility
    Path("./docs/index.html").touch(exist_ok=True)
    Path("./docs/config.html").touch(exist_ok=True)
    if sys.platform == "win32":
        jc = str(Path("./jc.exe").absolute())
    else:
        jc = str(Path("./jc").absolute())
    subprocess.Popen(
        [
            jc,
            "index.html",
            "./docs/index.html",
            "--template-dir",
            "./docs_templates",
        ],
        stdout=subprocess.DEVNULL,
        stderr=subprocess.STDOUT,
    )
    subprocess.Popen(
        [
            jc,
            "config.html",
            "./docs/config.html",
            "--template-dir",
            "./docs_templates",
        ],
        stdout=subprocess.DEVNULL,
        stderr=subprocess.STDOUT,
    )
    shutil.copyfile("./docs_templates/index.json", "./docs/index.json")
    shutil.copyfile("./docs_templates/hero.png", "./docs/hero.png")
    shutil.copyfile("./docs_templates/logo.png", "./docs/logo.png")
    shutil.copyfile("./docs_templates/weather.exe", "./docs/weather.exe")
    shutil.copyfile("./docs_templates/weather", "./docs/weather")
    shutil.copyfile("./docs_templates/updater.exe", "./docs/updater.exe")
    shutil.copyfile("./docs_templates/updater", "./docs/updater")
    shutil.copyfile("./docs_templates/weatherd.exe", "./docs/weatherd.exe")
    shutil.copyfile("./docs_templates/weatherd", "./docs/weatherd")
    shutil.copyfile("./docs_templates/theme.js", "./docs/theme.js")
    shutil.copyfile("./docs_templates/weather_codes.json", "./docs/weather_codes.json")
    shutil.copyfile("./docs_templates/weather_ascii_images.json", "./docs/weather_ascii_images.json")
    shutil.copyfile("./docs_templates/default_layout.json", "./docs/default_layout.json")
    print(colorama.Fore.GREEN + "Done!")


if __name__ == "__main__":
    main(obj={})
