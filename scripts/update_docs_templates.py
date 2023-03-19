import datetime
import json
import os
import shutil
import threading
from zipfile import ZipFile

import click
import core.networking

from path_helper import weathercli_dir
from update_index_hashes import update_hash


def get_artifact_urls(s, run_id):
    artifact_request = s.get(
        "https://api.github.com/repos/arihant2math/weathercli/actions/runs/"
        + str(run_id)
        + "/artifacts"
    )
    return json.loads(artifact_request.text)["artifacts"]


def download_artifact(s, artifact_list, name, file):
    artifact_id = [a for a in artifact_list if a["name"] == name][0]["id"]
    download = s.get(
        "https://api.github.com/repos/arihant2math/weathercli/actions/artifacts/"
        + str(artifact_id)
        + "/zip"
    )
    tmp_zip_file = weathercli_dir / "tmp" / (file + ".zip")
    with tmp_zip_file.open("wb") as f:
        f.write(bytes(download.bytes))
    with ZipFile(str(tmp_zip_file)) as z:
        with z.open(file) as exe:
            with (weathercli_dir / "docs_templates" / file).open("wb") as out:
                out.write(exe.read())


def filter_by_file(runs, file):
    return [run for run in runs if (run["path"] == file)]


@click.command()
@click.argument("gh_token")
def main(gh_token):
    if not os.path.exists("./tmp"):
        os.makedirs("./tmp")
    headers = {"Authorization": "Bearer " + gh_token}
    s = core.networking.Session(headers=headers)
    get_run_id = s.get(
        "https://api.github.com/repos/arihant2math/weathercli/actions/runs?per_page=100&status=completed"
    )
    runs = json.loads(get_run_id.text)["workflow_runs"]
    ci = filter_by_file(runs, ".github/workflows/build.yml")
    updater_ci = filter_by_file(runs, ".github/workflows/build-updater.yml")
    daemon_ci = filter_by_file(runs, ".github/workflows/build-daemon.yml")
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
    u = None
    w = None
    uu = None
    wu = None
    ud = None
    wd = None
    if artifacts is not None:
        print("Starting Unix Download")
        u = threading.Thread(
            target=download_artifact, args=(s, artifacts, "weather (Unix)", "weather")
        )
        u.start()
        print("Starting Windows Download")
        w = threading.Thread(
            target=download_artifact,
            args=(s, artifacts, "weather (Windows)", "weather.exe"),
        )
        w.start()
    if updater_artifacts is not None:
        print("Starting Unix Download (Updater)")
        uu = threading.Thread(
            target=download_artifact,
            args=(s, updater_artifacts, "updater (Unix)", "updater"),
        )
        uu.start()
        print("Starting Windows Download (Updater)")
        wu = threading.Thread(
            target=download_artifact,
            args=(s, updater_artifacts, "updater (Windows)", "updater.exe"),
        )
        wu.start()
    if daemon_artifacts is not None:
        print("Starting Unix Download (Daemon)")
        ud = threading.Thread(
            target=download_artifact,
            args=(s, daemon_artifacts, "weatherd (Unix)", "weatherd"),
        )
        ud.start()
        print("Starting Windows Download (Daemon)")
        wd = threading.Thread(
            target=download_artifact,
            args=(s, daemon_artifacts, "weatherd (Windows)", "weatherd.exe"),
        )
        wd.start()
    if u is not None:
        u.join()
    if w is not None:
        w.join()
    if uu is not None:
        uu.join()
    if wu is not None:
        wu.join()
    if ud is not None:
        ud.join()
    if wd is not None:
        wd.join()
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
