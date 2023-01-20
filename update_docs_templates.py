from zipfile import ZipFile

import click
import requests


@click.command()
@click.argument("gh_token")
def main(gh_token):
    s = requests.Session()
    s.auth = ("token", gh_token)
    get_run_id = s.get(
        "https://api.github.com/repos/arihant2math/weathercli/actions/runs"
    )
    runs = get_run_id.json()["workflow_runs"]
    ci = [run for run in runs if (run["path"] == ".github/workflows/build.yml")]
    highest_ci = ci[0]
    r2 = s.get(
        "https://api.github.com/repos/arihant2math/weathercli/actions/runs/"
        + str(highest_ci["id"])
        + "/artifacts"
    )
    artifacts = r2.json()["artifacts"]
    unix = [a for a in artifacts if a["name"] == "weather (Unix)"][0]["id"]
    windows = [a for a in artifacts if a["name"] == "weather (Windows)"][0]["id"]
    print("Starting Unix Download")
    unix_download = s.get(
        "https://api.github.com/repos/arihant2math/weathercli/actions/artifacts/"
        + str(unix)
        + "/zip"
    )
    with open("./tmp/weather.zip", "wb") as f:
        f.write(unix_download.content)
    print("Starting Windows Download")
    windows_download = s.get(
        "https://api.github.com/repos/arihant2math/weathercli/actions/artifacts/"
        + str(windows)
        + "/zip"
    )
    with open("./tmp/weather.exe.zip", "wb") as f:
        f.write(windows_download.content)
    print("Extracting Zips ...")
    with ZipFile("./tmp/weather.exe.zip") as exezip:
        with exezip.open("weather.exe") as exe:
            with open("./docs_templates/weather.exe", "wb") as out:
                out.write(exe.read())
    with ZipFile("./tmp/weather.zip") as unixzip:
        with unixzip.open("weather") as exe:
            with open("./docs_templates/weather", "wb") as out:
                out.write(exe.read())


if __name__ == "__main__":
    main(obj={})
