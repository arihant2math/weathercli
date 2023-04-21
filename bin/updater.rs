use std::fs;
use std::path::Path;

use clap::Parser;
use serde::Deserialize;
use serde::Serialize;

use weather_core::bin_common::update_component;
use weather_core::Config;
use weather_core::hash_file;

#[derive(Clone, Parser)]
struct Cli {
    #[arg(long, short, default_value_t = String::from("all"))]
    component: String,
    #[clap(long, short, action)]
    quiet: bool,
    #[clap(long, short, action)]
    version: bool,
    #[clap(long, short, action)]
    force: bool,
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

fn is_update_needed_platform(file: &str, web_hash: String) -> Result<bool, String> {
    if Path::new(file).exists() {
        let file_hash = hash_file(file);
        Ok(file_hash != web_hash)
    } else {
        Ok(true)
    }
}

async fn is_update_needed(index: IndexStruct, component: Component) -> Result<bool, String> {
    if component == Component::Main {
        if cfg!(windows) {
            return is_update_needed_platform("weather.exe", index.weather_exe_hash_windows);
        } else if cfg!(unix) {
            return is_update_needed_platform("weather", index.weather_exe_hash_unix);
        }
    } else if component == Component::Daemon {
        if cfg!(windows) {
            return is_update_needed_platform("weatherd.exe", index.weatherd_exe_hash_windows);
        } else if cfg!(unix) {
            return is_update_needed_platform("weatherd", index.weatherd_exe_hash_unix);
        }
    }
    Ok(true)
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let args = Cli::parse();
    let resp = reqwest::get("https://arihant2math.github.io/weathercli/index.json")
        .await
        .expect("Index get failed");
    let json: IndexStruct =
        serde_json::from_str(&resp.text().await.expect("Failed to receive text"))
            .expect("JSON parsing failed");
    if args.version && !args.quiet {
        println!("{}", weather_core::version());
        return Ok(());
    }
    let install_dir = std::env::current_dir().expect("Not running from directory");
    let parent = install_dir.parent().unwrap_or(&*install_dir);
    let install_type_folders = fs::read_dir(parent)
        .expect("read parent dir failed")
        .any(|f| {
            f.expect("read failed").file_name().to_str().unwrap_or("")
                == Config::new().weather_file_name
        });
    let d_install_path = install_dir.clone();
    let w_install_path = if install_type_folders {
         parent.to_path_buf()
    } else {
         install_dir
    };
    weather_core::component_updater::update_web_resources(false, Some(args.quiet));
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
        if args.force || is_update_needed(json.clone(), component).await? {
            to_update.push(component)
        }
    }
    if to_update.is_empty() {
        println!("Nothing to Update!");
        return Ok(());
    }
    if to_update.contains(&Component::Main) {
        let url = "https://arihant2math.github.io/weathercli/".to_string()
            + &Config::new().weather_file_name;
        let mut path = w_install_path.to_path_buf();
        path.push(Config::new().weather_file_name);
        update_component(
            &url,
            &path.display().to_string(),
            "Downloading weathercli update from ".to_string(),
            "Updated weathercli".to_string(),
            args.quiet,
        )
        .await?;
    }
    if to_update.contains(&Component::Daemon) {
        let url = "https://arihant2math.github.io/weathercli/".to_string()
            + &Config::new().weather_dfile_name;
        let mut path = d_install_path.to_path_buf();
        path.push(Config::new().weather_dfile_name);
        update_component(
            &url,
            &path.display().to_string(),
            "Downloading daemon update from ".to_string(),
            "Updated daemon".to_string(),
            args.quiet,
        )
        .await?;
    }
    Ok(())
}
