import importlib
import shutil
import subprocess
from pathlib import Path

import click

from scripts.generate_stubs_v2 import Module, write


@click.group()
def main():
    pass


@main.command("docs", help="Builds the docs")
def docs():
    Path("./docs").mkdir(exist_ok=True)
    jc = str(Path("./jc.exe").absolute())
    subprocess.Popen(
        [
            jc,
            "index.html",
            "./docs/index.html",
            "--template-dir",
            "./docs_templates",
            "--no-minify",
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
            "--no-minify",
        ],
        stdout=subprocess.DEVNULL,
        stderr=subprocess.STDOUT,
    )
    subprocess.Popen(
        [
            jc,
            "index.json",
            "./docs/index.json",
            "--template-dir",
            "./docs_templates",
            "--no-minify",
        ],
        stdout=subprocess.DEVNULL,
        stderr=subprocess.STDOUT,
    )
    shutil.copyfile("./docs_templates/hero.png", "./docs/hero.png")
    shutil.copyfile("./docs_templates/logo.png", "./docs/logo.png")
    shutil.copyfile("./docs_templates/weather.exe", "./docs/weather.exe")
    shutil.copyfile("./docs_templates/weather", "./docs/weather")
    shutil.copyfile("./docs_templates/updater.exe", "./docs/updater.exe")
    shutil.copyfile("./docs_templates/updater", "./docs/updater")
    shutil.copyfile("./docs_templates/weatherd.exe", "./docs/weatherd.exe")
    shutil.copyfile("./docs_templates/weatherd", "./docs/weatherd")
    shutil.copyfile("./docs_templates/theme.js", "./docs/theme.js")
    print("Done!")


@main.command("stubs")
@click.argument("out")
def stubs(out):
    ast_gen: list[list] = Module(importlib.import_module("core")).get_ast()
    write(Path(out), ast_gen)
    print("Done!")


@main.command("build")
def rust():
    subprocess.Popen(["maturin", "develop" "-r"])
    subprocess.Popen(["pyinstaller", "-F", "weather.py", "-i", "./icon/icon.png"])


if __name__ == "__main__":
    main(obj={})