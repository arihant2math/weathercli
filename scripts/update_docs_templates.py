import datetime
import json
import os
import shutil
from zipfile import ZipFile

import click
import requests

from path_helper import weathercli_dir
from update_index_hashes import update_hash


def get_artifact_urls(s, run_id):
    artifact_request = s.get(
        "https://api.github.com/repos/arihant2math/weathercli/actions/runs/"
        + str(run_id)
        + "/artifacts"
    )
    return artifact_request.json()["artifacts"]


def download_artifact(s, artifact_list, name, file):
    artifact_id = [a for a in artifact_list if a["name"] == name][0]["id"]
    download = s.get(
        "https://api.github.com/repos/arihant2math/weathercli/actions/artifacts/"
        + str(artifact_id)
        + "/zip"
    )
    tmp_zip_file = weathercli_dir / ("tmp" + file + ".zip")
    with tmp_zip_file.open("wb") as f:
        f.write(download.content)
    with ZipFile(str(tmp_zip_file)) as z:
        with z.open(file) as exe:
            with (weathercli_dir / ("docs_templates" + file)).open("wb") as out:
                out.write(exe.read())


def filter_by_file(runs, file):
    return [run for run in runs if (run["path"] == file)]


@click.command()
@click.argument("gh_token")
def main(gh_token):
    if not os.path.exists("./tmp"):
        os.makedirs("./tmp")
    s = requests.Session()
    s.auth = ("token", gh_token)
    get_run_id = s.get(
        "https://api.github.com/repos/arihant2math/weathercli/actions/runs?per_page=100&status=completed"
    )
    runs = get_run_id.json()["workflow_runs"]
    ci = filter_by_file(runs, "../.github/workflows/build.yml")
    updater_ci = filter_by_file(runs, "../.github/workflows/build-updater.yml")
    daemon_ci = filter_by_file(runs, "../.github/workflows/build-daemon.yml")
    artifacts = None
    updater_artifacts = None
    daemon_artifacts = None
    if len(ci) > 0:
        latest_run_id = ci[0]["id"]
        artifacts = get_artifact_urls(s, latest_run_id)
    if len(updater_ci) > 0:
        latest_updater_run_id = updater_ci[0]["id"]
        updater_artifacts = get_artifact_urls(s, latest_updater_run_id)
    if len(daemon_ci) > 0:
        latest_daemon_run_id = daemon_ci[0]["id"]
        daemon_artifacts = get_artifact_urls(s, latest_daemon_run_id)

    if artifacts is not None:
        print("Starting Unix Download")
        download_artifact(s, artifacts, "weather (Unix)", "weather")
        print("Starting Windows Download")
        download_artifact(s, artifacts, "weather (Windows)", "weather.exe")
    if updater_artifacts is not None:
        print("Starting Unix Download (Updater)")
        download_artifact(s, updater_artifacts, "updater (Unix)", "../updater")
        print("Starting Windows Download (Updater)")
        download_artifact(s, updater_artifacts, "updater (Windows)", "updater.exe")
    if daemon_artifacts is not None:
        print("Starting Unix Download (Daemon)")
        download_artifact(s, daemon_artifacts, "weatherd (Unix)", "weatherd")
        print("Starting Windows Download (Daemon)")
        download_artifact(s, daemon_artifacts, "weatherd (Windows)", "weatherd.exe")
    shutil.rmtree("./tmp")
    d = json.load((weathercli_dir / "docs_templates" / "index.json").open())
    now = datetime.datetime.now()
    s = "{}.{}.{}".format(now.year, now.month, now.day)
    d["version"] = s
    d["updater-version"] = s
    d["daemon-version"] = s
    json.dump(d, (weathercli_dir / "docs_templates" / "index.json").open("w"))
    docs_templates_dir = weathercli_dir / "docs_templates"
    update_hash(docs_templates_dir / "weather.exe", "weather-exe-hash-windows")
    update_hash(docs_templates_dir / "weather", "weather-exe-hash-unix")
    update_hash(docs_templates_dir / "updater.exe", "updater-exe-hash-windows")
    update_hash(docs_templates_dir / "updater", "updater-exe-hash-unix")
    update_hash(docs_templates_dir / "weatherd.exe", "weatherd-exe-hash-windows")
    update_hash(docs_templates_dir / "weatherd", "weatherd-exe-hash-unix")


if __name__ == "__main__":
    main(obj={})
