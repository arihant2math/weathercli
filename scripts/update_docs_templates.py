import datetime
import json
import os
import shutil
from multiprocessing import Process
from zipfile import ZipFile

import click
import weather_core.networking

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
    s = weather_core.networking.Session(headers=headers)
    get_run_id = s.get(
        "https://api.github.com/repos/arihant2math/weathercli/actions/runs?per_page=100&status=completed"
    )
    runs = json.loads(get_run_id.text)["workflow_runs"]
    rust_ci = filter_by_file(runs, ".github/workflows/build.yml")
    rust_artifacts = None
    if len(rust_ci) > 0:
        latest_updater_run_id = rust_ci[0]["id"]
        rust_artifacts = get_artifact_urls(s, latest_updater_run_id)
    tasks = []
    if rust_artifacts is not None:
        print("Starting Unix Download")
        tasks.append((s, rust_artifacts, "weather (Linux)", "weather"))
        print("Starting Windows Download")
        tasks.append((s, rust_artifacts, "weather (Windows)", "weather.exe"))
        print("Starting Unix Download (Updater)")
        tasks.append((s, rust_artifacts, "updater (Linux)", "updater"))
        print("Starting Windows Download (Updater)")
        tasks.append((s, rust_artifacts, "updater (Windows)", "updater.exe"))
        print("Starting Unix Download (Daemon)")
        tasks.append((s, rust_artifacts, "weatherd (Linux)", "weatherd"))
        print("Starting Windows Download (Daemon)")
        tasks.append((s, rust_artifacts, "weatherd (Windows)", "weatherd.exe"))
    jobs = []
    for task in tasks:
        # print(task)
        p = Process(target=download_artifact, args=task)
        p.start()
        jobs.append(p)
    for job in jobs:
        job.join()
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
