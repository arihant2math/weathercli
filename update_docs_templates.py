import os
import shutil
from zipfile import ZipFile

import click
import requests


@click.command()
@click.argument("gh_token")
def main(gh_token):
    if not os.path.exists("./tmp"):
        os.makedirs("./tmp")
    s = requests.Session()
    s.auth = ("token", gh_token)
    get_run_id = s.get(
        "https://api.github.com/repos/arihant2math/weathercli/actions/runs"
    )
    runs = get_run_id.json()["workflow_runs"]
    ci = [run for run in runs if (run["path"] == ".github/workflows/build.yml")]
    latest_run_id = ci[0]["id"]
    artifact_request = s.get(
        "https://api.github.com/repos/arihant2math/weathercli/actions/runs/"
        + str(latest_run_id)
        + "/artifacts"
    )
    artifacts = artifact_request.json()["artifacts"]
    try:
        unix_artifact_id = [a for a in artifacts if a["name"] == "weather (Unix)"][0]["id"]
    except IndexError:
        unix_artifact_id = None
    windows_artifact_id = [a for a in artifacts if a["name"] == "weather (Windows)"][0][
        "id"
    ]
    print("Starting Unix Download")
    if unix_artifact_id is not None:
        unix_download = s.get(
            "https://api.github.com/repos/arihant2math/weathercli/actions/artifacts/"
            + str(unix_artifact_id)
            + "/zip"
        )
        with open("./tmp/weather.zip", "wb") as f:
            f.write(unix_download.content)
        with ZipFile("./tmp/weather.zip") as unixzip:
            with unixzip.open("weather") as exe:
                with open("./docs_templates/weather", "wb") as out:
                    out.write(exe.read())
    print("Starting Windows Download")
    windows_download = s.get(
        "https://api.github.com/repos/arihant2math/weathercli/actions/artifacts/"
        + str(windows_artifact_id)
        + "/zip"
    )
    with open("./tmp/weather.exe.zip", "wb") as f:
        f.write(windows_download.content)
    with ZipFile("./tmp/weather.exe.zip") as exezip:
        with exezip.open("weather.exe") as exe:
            with open("./docs_templates/weather.exe", "wb") as out:
                out.write(exe.read())
    shutil.rmtree("./tmp")


if __name__ == "__main__":
    main(obj={})
