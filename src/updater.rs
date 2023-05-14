use std::fs;
use std::path::Path;

use clap::Parser;
use serde::Deserialize;
use serde::Serialize;

use local::settings::Settings;
use updater::component::update_component;
use updater::CONFIG;
use updater::hash_file;

pub type Result<T> = std::result::Result<T, weather_error::Error>;

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
#[serde(rename_all = "kebab-case")]
struct IndexStruct {
    version: String,
    updater_version: String,
    weather_codes_hash: String,
    weather_ascii_images_hash: String,
    daemon_version: String,
    weather_exe_hash_windows: String,
    weather_exe_hash_unix: String,
    updater_exe_hash_windows: String,
    updater_exe_hash_unix: String,
    weatherd_exe_hash_windows: String,
    weatherd_exe_hash_unix: String,
}

fn is_update_needed_platform(file: &str, web_hash: String) -> Result<bool> {
    if Path::new(file).exists() {
        let file_hash = hash_file(file)?;
        Ok(file_hash != web_hash)
    } else {
        Ok(true)
    }
}

async fn is_update_needed(index: IndexStruct, component: Component) -> Result<bool> {
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
async fn main() -> Result<()> {
    print!("\x1b[0J");
    let args = Cli::parse();
    let settings = Settings::new()?;
    let resp = reqwest::get(settings.update_server.clone() + "index.json")
        .await
        .expect("Network request failed");
    let json: IndexStruct =
        unsafe { simd_json::from_str(&mut resp.text().await.expect("Failed to receive text")) }?;
    if args.version && !args.quiet {
        println!("{}", env!("CARGO_PKG_VERSION").to_string()); // TODO: Standardize version retrieval
        return Ok(());
    }
    let install_dir = std::env::current_dir()?;
    let parent = install_dir.parent().unwrap_or(&*install_dir);
    let install_type_folders = fs::read_dir(parent)?
        .any(|f| f.expect("Dir Entry failed").file_name() == CONFIG.weather_file_name);
    let d_install_path = install_dir.clone();
    let w_install_path = if install_type_folders {
        parent.to_path_buf()
    } else {
        install_dir
    };
    updater::resource::update_web_resources(settings.update_server, Some(args.quiet))?;
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
        let url =
            "https://arihant2math.github.io/weathercli/".to_string() + CONFIG.weather_file_name;
        let mut path = w_install_path.to_path_buf();
        path.push(CONFIG.weather_file_name);
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
        let url =
            "https://arihant2math.github.io/weathercli/".to_string() + CONFIG.weather_d_file_name;
        let path = d_install_path
            .to_path_buf()
            .join(CONFIG.weather_d_file_name);
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
