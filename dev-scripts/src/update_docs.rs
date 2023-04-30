use std::collections::HashMap;
use std::{env, fs};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Write};


use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use serde_json::Value;

use crate::update_hash::update_hash;

fn get_artifact_urls(h: HashMap<String, String>, run_id: &str) -> weather_core::Result<Value> {
    let artifact_request = weather_core::networking::get_url(
        format!("https://api.github.com/repos/arihant2math/weathercli/actions/runs/{}/artifacts", run_id),
        None, Some(h), None)?;
    let json: Value = serde_json::from_str(&artifact_request.text)?;
    return Ok(json["artifacts"].clone());
}

fn download_artifact(artifact_list: &Vec<Value>, h: HashMap<String, String>, name: &str, file: &str) -> weather_core::Result<()> {
    println!("Downloading {name} to {file}");
    let artifacts: Option<&Value> = artifact_list.iter().find(|a| a["name"].as_str().unwrap() == name);
    let artifact_id = artifacts.expect(&*format!("Could not find artifact {name}"))["id"].as_i64().expect(&*format!("Could not find artifact {name} key id"));
    let download =
        weather_core::networking::get_url(format!("https://api.github.com/repos/arihant2math/weathercli/actions/artifacts/{}/zip", artifact_id), None, Some(h), None)?;
    let mut tmp_zip_file = OpenOptions::new().create(true).write(true).open(format!("./tmp/{file}.zip"))?;
    tmp_zip_file.write_all(&*download.bytes)?;
    let reader = BufReader::new(File::open(format!("./tmp/{file}.zip"))?);
    let mut zip = zip::ZipArchive::new(reader).expect("zip read failed");
    let mut file_zip = zip.by_index(0).unwrap();
    println!("Extracting {}", file_zip.name());
    let mut writer = BufWriter::new(
        OpenOptions::new().write(true).truncate(true)
            .open(format!("./docs_templates/{file}"))?);
    std::io::copy(&mut file_zip, &mut writer)?;
    Ok(())
}

fn filter_by_file(runs: &Vec<Value>, file: &str) -> Vec<Value> {
    return runs.iter().filter(|&run| run["path"].as_str().unwrap() == file).map(|r| r.clone()).collect();
}

pub fn update_docs(gh_token: &str) -> weather_core::Result<()> {
    let working_dir = env::current_dir()?;
    fs::create_dir_all(working_dir.join("tmp"))?;
    let mut headers = HashMap::new();
    headers.insert("Authorization".to_string(), format!("Bearer {}", gh_token));
    let get_run_id = weather_core::networking::get_url("https://api.github.com/repos/arihant2math/weathercli/actions/runs?per_page=10&status=completed",
                                                       None, Some(headers.clone()), None)?;
    let runs_json: Value = serde_json::from_str(&get_run_id.text)?;
    let runs = runs_json["workflow_runs"].as_array()
        .ok_or("not an array")?;
    let rust_ci = filter_by_file(runs, ".github/workflows/build.yml");
    let latest_updater_run_id = rust_ci[0]["id"].as_i64().unwrap();
    let binding = get_artifact_urls(headers.clone(), &latest_updater_run_id.to_string())?;
    let rust_artifacts = binding.as_array().unwrap();
    let tasks = vec![
   ["weather (Windows)", "weather.exe"],
   ["updater (Windows)", "updater.exe"],
   ["weatherd (Windows)", "weatherd.exe"],
   ["weather (Linux)", "weather"],
   ["updater (Linux)", "updater"],
   ["weatherd (Linux)", "weatherd"]];
    tasks.par_iter().for_each(|&s| download_artifact(&rust_artifacts, headers.clone(), s[0], s[1]).unwrap());
    fs::remove_dir_all(working_dir.join("tmp"))?; // TODO: Implement index version updates
    update_hash("./docs_templates/weather.exe", "weather-exe-hash-windows")?;
    update_hash("./docs_templates/weather", "weather-exe-hash-unix")?;
    update_hash("./docs_templates/updater.exe", "updater-exe-hash-windows")?;
    update_hash("./docs_templates/updater", "updater-exe-hash-unix")?;
    update_hash("./docs_templates/weatherd.exe", "weatherd-exe-hash-windows")?;
    update_hash("./docs_templates/weatherd", "weatherd-exe-hash-unix")?;
    Ok(())
}
