use crate::color::{FORE_CYAN, FORE_RED};
use crate::local::settings::Settings;
use crate::{networking, updater, version};
use serde_json::Value;
use std::time::Duration;
use std::{fs, thread};

pub fn setup(settings_s: Settings) -> crate::Result<()> {
    let mut settings = settings_s;
    println!("{FORE_CYAN}===== Weather CLI Setup =====");
    updater::resource::update_web_resources(settings.internal.update_server.clone(), None)?;
    println!("{FORE_RED}Choose the default weather backend: ");
    let options = [
        "Meteo",
        "Open Weather Map",
        "National Weather Service",
        "The Weather Channel",
    ];
    let mut default = ["METEO", "OPENWEATHERMAP", "NWS", "THEWEATHERCHANNEL"]
        .iter()
        .position(|&x| x == settings.internal.default_backend.clone())
        .unwrap_or(0);
    let current = crate::prompt::choice(&options, default, None)?;
    let weather_backend_setting = ["METEO", "OPENWEATHERMAP", "NWS", "THEWEATHERCHANNEL"][current];
    settings.internal.default_backend = weather_backend_setting.to_string();
    settings.write()?;
    thread::sleep(Duration::from_millis(100));
    println!(
        "{FORE_RED}Is your location constant (i.e. is this computer stationary at all times)?"
    );
    if settings.internal.constant_location {
        default = 0;
    } else {
        default = 1;
    }
    let constant_location_setting =
        [true, false][crate::prompt::choice(&["yes", "no"], default, None)?];
    settings.internal.constant_location = constant_location_setting;
    settings.write()?;
    thread::sleep(Duration::from_millis(100));
    println!("{FORE_RED}Should static resources (ascii art, weather code sentences, etc.) be auto-updated?");
    if settings.internal.auto_update_internet_resources {
        default = 0;
    } else {
        default = 1;
    }
    let auto_update_setting = [true, false][crate::prompt::choice(&["yes", "no"], default, None)?];
    settings.internal.auto_update_internet_resources = auto_update_setting;
    settings.write()?;
    Ok(())
}

pub fn update(force: bool) -> crate::Result<()> {
    println!("Checking for updates ...");
    let latest_version = updater::get_latest_version()?;
    let application_path = std::env::current_exe()?;
    println!("Latest Version: {latest_version}");
    println!("Current Version: {}", version());
    if latest_version != version() || force {
        println!("Updating weather.exe at {}", application_path.display());
        let mut updater_location = application_path
            .parent()
            .expect("no parent dir")
            .to_path_buf()
            .join("components");
        fs::create_dir_all(&updater_location)?;
        if cfg!(windows) {
            updater_location.push("updater.exe");
        } else {
            updater_location.push("updater");
        }
        if !updater_location.exists() {
            println!("Updater not found, downloading updater");
            updater::get_updater(updater_location.display().to_string())?;
            let resp: Value = serde_json::from_str(
                &networking::get_url(
                    "https://arihant2math.github.io/weathercli/index.json",
                    None,
                    None,
                    None,
                )?
                .text,
            )?;
            let mut web_hash = resp["updater-exe-hash-unix"]
                .as_str()
                .expect("updater-exe-hash-unix key not found in index.json");
            if cfg!(windows) {
                web_hash = resp["updater-exe-hash-windows"]
                    .as_str()
                    .expect("updater-exe-hash-windows key not found in index.json");
            }
            if crate::util::hash_file(&updater_location.display().to_string())? != web_hash || force
            {
                updater::get_updater(updater_location.display().to_string())?;
            }
            drop(resp); // Dropping to reduce memory leakage
            println!("Starting updater and exiting");
            let mut command = std::process::Command::new(updater_location.as_os_str());
            if force {
                command.arg("--force").spawn()?;
            } else {
                command.spawn()?;
            }
            std::process::abort();
        }
    }
    Ok(())
}
