use std::collections::HashMap;
use std::fs;
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
    let artifacts: Vec<&Value> = artifact_list.iter().filter(|a| a["name"].as_str().unwrap() == name).collect();
    let artifact_id = artifacts[0]["id"].as_str().unwrap();
    let download =
        weather_core::networking::get_url(format!("https://api.github.com/repos/arihant2math/weathercli/actions/artifacts/{}/zip", artifact_id), None, Some(h), None)?;
    let mut tmp_zip_file = fs::OpenOptions::new().create(true).write(true).open("./tmp".to_string() + file + ".zip")?;
    tmp_zip_file.write_all(&*download.bytes)?;
    let reader = BufReader::new(tmp_zip_file.try_clone()?);
    let mut zip = zip::ZipArchive::new(reader).unwrap();
    let mut file = zip.by_index(0).expect("");
    let mut writer = BufWriter::new(tmp_zip_file);
    std::io::copy(&mut file, &mut writer)?;
    Ok(())
}

fn filter_by_file(runs: &Vec<Value>, file: &str) -> Vec<Value> {
    return runs.iter().filter(|&run| run["path"].as_str().unwrap() == file).map(|r| r.clone()).collect();
}

pub fn update_docs(gh_token: &str) -> weather_core::Result<()> {
    fs::create_dir_all("./tmp")?;
    let mut headers = HashMap::new();
    headers.insert("Authorization".to_string(), format!("Bearer {}", gh_token));
    let get_run_id = weather_core::networking::get_url("https://api.github.com/repos/arihant2math/weathercli/actions/runs?per_page=10&status=completed",
                                                       None, Some(headers.clone()), None)?;
    let runs_json: Value = serde_json::from_str(&get_run_id.text)?;
    let runs = runs_json["workflow_runs"].as_array()
        .ok_or_else(|| "not an array")?;
    let rust_ci = filter_by_file(runs, ".github/workflows/build.yml");
    let latest_updater_run_id = rust_ci[0]["id"].as_i64().unwrap();
    let binding = get_artifact_urls(headers.clone(), &latest_updater_run_id.to_string())?;
    let rust_artifacts = binding.as_array().unwrap();
    let mut tasks = vec![];
    println!("Starting Unix Download");
    tasks.push(["weather (Linux)", "weather"]);
    println!("Starting Windows Download");
    tasks.push(["weather (Windows)", "weather.exe"]);
    println!("Starting Unix Download (Updater)");
    tasks.push(["updater (Linux)", "updater"]);
    println!("Starting Windows Download (Updater)");
    tasks.push(["updater (Windows)", "updater.exe"]);
    println!("Starting Unix Download (Daemon)");
    tasks.push(["weatherd (Linux)", "weatherd"]);
    println!("Starting Windows Download (Daemon)");
    tasks.push(["weatherd (Windows)", "weatherd.exe"]);
    let _ = tasks.par_iter().map(|s| download_artifact(&rust_artifacts, headers.clone(), s[0], s[1]).unwrap());
    fs::remove_dir_all("./tmp")?; // TODO: Implement index hash updates
    update_hash("./docs_templates/weather.exe", "weather-exe-hash-windows")?;
    update_hash("./docs_templates/weather", "weather-exe-hash-unix")?;
    update_hash("./docs_templates/updater.exe", "updater-exe-hash-windows")?;
    update_hash("./docs_templates/updater", "updater-exe-hash-unix")?;
    update_hash("./docs_templates/weatherd.exe", "weatherd-exe-hash-windows")?;
    update_hash("./docs_templates/weatherd", "weatherd-exe-hash-unix")?;
    Ok(())
}
