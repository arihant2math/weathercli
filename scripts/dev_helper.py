import importlib
import shutil
import subprocess
import sys
from pathlib import Path

import click
import colorama

from generate_stubs_v2 import Module, write


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
    # Everything below is to not break the updater from old versions
    shutil.copyfile("./docs_templates/weather.exe", "./docs/docs/weather.exe")
    shutil.copyfile("./docs_templates/weather", "./docs/docs/weather")
    shutil.copyfile("./docs_templates/updater.exe", "./docs/docs/updater.exe")
    shutil.copyfile("./docs_templates/updater", "./docs/docs/updater")
    shutil.copyfile("./docs_templates/weatherd.exe", "./docs/docs/weatherd.exe")
    shutil.copyfile("./docs_templates/weatherd", "./docs/docs/weatherd")
    shutil.copyfile("./docs_templates/index.json", "./docs/docs/index.json")
    print(colorama.Fore.GREEN + "Done!")


@main.command("stubs")
def stubs():
    ast_gen: list[list] = Module(
        importlib.import_module("weather_core"), False
    ).get_ast()
    write(Path("./venv/Lib/site-packages/weather_core/"), ast_gen, True, False)
    print(colorama.Fore.GREEN + "Done!")


@main.command("rust")
def rust():
    subprocess.Popen(["maturin", "develop" "-r"])
    ast_gen: list[list] = Module(
        importlib.import_module("weather_core"), True
    ).get_ast()
    write(Path("./venv/Lib/site-packages/weather_core/"), ast_gen, True, False)
    # subprocess.Popen(["pyinstaller", "-F", "weather.py", "-i", "./icon/icon.png"])
    print(colorama.Fore.GREEN + "Done!")


@main.command("build")
def build():
    subprocess.Popen(["pyinstaller", "-F", "weather.py", "-i", "./icon/icon.png"])


if __name__ == "__main__":
    main(obj={})
