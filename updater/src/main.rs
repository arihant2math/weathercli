use std::cmp::min;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::thread;
use std::time::Duration;

use clap::Parser;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use serde::Deserialize;
use serde::Serialize;
use sha256::try_digest;

fn hash_file(filename: &str) -> String {
    // let path = "./".to_string() + filename;
    let input = Path::new(filename);
    try_digest(input).unwrap()
}

#[derive(Clone, Parser)]
struct Cli {
    #[arg(long, short, default_value_t = String::from("all"))]
    component: String,
    #[clap(long, short, action)]
    quiet: bool,
    #[clap(long, short, action)]
    version: bool,
}

#[derive(Eq, PartialEq, Clone, Copy)]
enum Component {
    Main,
    Daemon,
}

#[derive(Serialize, Deserialize, Clone)]
struct IndexStruct {
    version: String,
    #[serde(rename = "updater-version")]
    updater_version: String,
    #[serde(rename = "weather-codes-hash")]
    weather_codes_hash: String,
    #[serde(rename = "weather-ascii-images-hash")]
    weather_ascii_images_hash: String,
    #[serde(rename = "daemon-version")]
    daemon_version: String,
    #[serde(rename = "weather-exe-hash-windows")]
    weather_exe_hash_windows: String,
    #[serde(rename = "weather-exe-hash-unix")]
    weather_exe_hash_unix: String,
    #[serde(rename = "updater-exe-hash-windows")]
    updater_exe_hash_windows: String,
    #[serde(rename = "updater-exe-hash-unix")]
    updater_exe_hash_unix: String,
    #[serde(rename = "weatherd-exe-hash-windows")]
    weatherd_exe_hash_windows: String,
    #[serde(rename = "weatherd-exe-hash-unix")]
    weatherd_exe_hash_unix: String,
}

async fn update_component(
    url: &str,
    path: &str,
    progress_msg: String,
    finish_msg: String,
) -> Result<(), String> {
    let client = Client::new();
    // Reqwest setup
    let res = client
        .get(url)
        .send()
        .await
        .or(Err(format!("Failed to GET from '{}'", url)))?;
    let total_size = res
        .content_length()
        .ok_or(format!("Failed to get content length from '{}'", &url))?;

    let mut file_expect = File::create(path);
    let retries = 0;
    while file_expect.is_err() {
        if retries > 3 {
            return Err(format!("Failed to create/open file '{}'", path));
        }
        file_expect = File::create(path);
        thread::sleep(Duration::from_millis(1000));
    }
    let mut file = file_expect.unwrap();

    let progress_bar = ProgressBar::new(total_size);
    progress_bar.set_style(ProgressStyle::default_bar()
        .template("{msg}\n[{elapsed}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
        .expect("Failed due to progress bar error")
        .progress_chars("—> "));
    progress_bar.set_message(progress_msg + url);

    // download chunks
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.or(Err("Error while downloading file".to_string()))?;
        file.write_all(&chunk)
            .map_err(|_| "Error while writing to file".to_string())?;
        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
        progress_bar.set_position(new);
    }
    progress_bar.set_style(ProgressStyle::default_bar()
        .template("{msg}\n[{elapsed}] [{wide_bar:.green}] {bytes}/{total_bytes} ({bytes_per_sec})")
        .expect("Failed due to progress bar error")
        .progress_chars("—> "));
    progress_bar.finish_with_message(finish_msg);
    Ok(())
}

fn update_needed_check(file: &str, web_hash: String) -> Result<bool, String> {
    if Path::new(&file).exists() {
        let file_hash = hash_file(file);
        Ok(file_hash != web_hash)
    } else {
        Ok(true)
    }
}

async fn update_needed(index: IndexStruct, component: Component) -> Result<bool, String> {
    if component == Component::Main {
        if cfg!(windows) {
            return update_needed_check("weather.exe", index.weather_exe_hash_windows);
        } else if cfg!(unix) {
            return update_needed_check("weather", index.weather_exe_hash_unix);
        }
    } else if component == Component::Daemon {
        if cfg!(windows) {
            return update_needed_check("weatherd.exe", index.weatherd_exe_hash_windows);
        } else if cfg!(unix) {
            return update_needed_check("weatherd", index.weatherd_exe_hash_unix);
        }
    }
    Ok(true)
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let args = Cli::parse();
    let resp = reqwest::get("https://arihant2math.github.io/weathercli/docs/index.json")
        .await
        .unwrap();
    let json: IndexStruct = serde_json::from_str(&resp.text().await.unwrap()).unwrap();
    if args.version {
        println!("3.11.2023");
        return Ok(());
    }
    let mut to_update: Vec<Component> = Vec::new();
    let mut update_requests: Vec<Component> = Vec::new();
    if args.component == "all" {
        update_requests.push(Component::Main);
        update_requests.push(Component::Daemon);
    }
    if args.component == "daemon" {
        update_requests.push(Component::Daemon);
    }
    if args.component == "main" {
        update_requests.push(Component::Main);
    }
    for component in update_requests {
        if update_needed(json.clone(), component).await? {
            to_update.push(component)
        }
    }
    if to_update.is_empty() {
        println!("Nothing to Update!");
        return Ok(());
    }
    if to_update.contains(&Component::Main) {
        let url;
        let path;
        if cfg!(windows) {
            url = "https://arihant2math.github.io/weathercli/docs/weather.exe";
            path = "weather.exe";
        } else if cfg!(unix) {
            url = "https://arihant2math.github.io/weathercli/docs/weather";
            path = "weather";
        } else {
            return Err("OS unsupported".to_string());
        }

        let r = update_component(
            url,
            path,
            "Downloading weathercli update from ".to_string(),
            "Updated weathercli".to_string(),
        )
        .await;
        r?;
    }
    if to_update.contains(&Component::Daemon) {
        let url;
        let path;
        if cfg!(windows) {
            url = "https://arihant2math.github.io/weathercli/docs/weatherd.exe";
            path = "daemon.exe";
        } else if cfg!(unix) {
            url = "https://arihant2math.github.io/weathercli/docs/weatherd";
            path = "daemon";
        } else {
            return Err("OS unsupported".to_string());
        }
        let r = update_component(
            url,
            path,
            "Downloading daemon update from ".to_string(),
            "Updated daemon".to_string(),
        )
        .await;
        r?;
    }
    Ok(())
}
