use std::str::FromStr;
use std::thread;
use std::time::Duration;

use serde_json::Value;
use log::debug;

use crate::{component_updater, get_data_from_datasource, networking, version};
use crate::cli::{Datasource, print_out};
use crate::component_updater::get_updater;
use crate::dynamic_loader::ExternalBackends;
use crate::local::settings::Settings;
use crate::local::weather_file::WeatherFile;

pub fn weather(
    datasource: Datasource,
    coordinates: [String; 2],
    settings: Settings,
    true_metric: bool,
    json: bool,
    custom_backends: ExternalBackends
) -> crate::Result<()> {
    debug!("Coordinates: {:?}", coordinates.clone());
    debug!("Metric: {}", true_metric);
    debug!("json: {}", json);
    let mut s = settings.clone();
    s.internal.metric_default = true_metric;
    let data = get_data_from_datasource(datasource, coordinates, s, custom_backends)?;
    print_out(settings.internal.layout_file, data, json, true_metric)?;
    Ok(())
}

pub fn config(key_name: String, value: Option<String>) -> crate::Result<()> {
    match value {
        None => {
            let f = WeatherFile::settings()?;
            let data: Value = serde_json::from_str(&f.data)?;
            println!("{}: {}", &key_name, data[&key_name]);
        },
        Some(real_value) => {
            println!(
                "Writing {}={} ...",
                key_name.to_lowercase(),
                &real_value
            );
            let mut f = WeatherFile::settings()?;
            let mut data: Value = serde_json::from_str(&f.data)?;
            data[key_name.to_uppercase()] =
                Value::from_str(&real_value)?;
            f.data = serde_json::to_string(&data)?;
            f.write()?;
        }
    };
    Ok(())
}

pub fn setup(settings_s: Settings) -> crate::Result<()> {
    let mut settings = settings_s;
    println!(
        "{}===== Weather CLI Setup =====",
        crate::color::FORE_CYAN
    );
    component_updater::update_web_resources(None)?;
    println!(
        "{}Choose the default weather backend: ",
        crate::color::FORE_RED
    );
    let options = ["Meteo", "Open Weather Map", "National Weather Service", "The Weather Channel"];
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
        "{}Is your location constant (i.e. is this computer stationary at all times)?",
        crate::color::FORE_RED
    );
    if settings.internal.constant_location {
        default = 0;
    } else {
        default = 1;
    }
    let constant_location_setting = [true, false]
        [crate::prompt::choice(&["yes", "no"], default, None)?];
    settings.internal.constant_location = constant_location_setting;
    settings.write()?;
    thread::sleep(Duration::from_millis(100));
    println!(
        "{}Should static resources (ascii art, weather code sentences, etc.) be auto-updated?",
        crate::color::FORE_RED
    );
    if settings.internal.auto_update_internet_resources {
        default = 0;
    } else {
        default = 1;
    }
    let auto_update_setting = [true, false]
        [crate::prompt::choice(&["yes", "no"], default, None)?];
    settings.internal.auto_update_internet_resources = auto_update_setting;
    settings.write()?;
    Ok(())
}

pub fn update(force: bool) -> crate::Result<()> {
    println!("Checking for updates ...");
    let latest_version = component_updater::get_latest_version()?;
    let application_path = std::env::current_exe().expect("Current exe not found");
    println!("Latest Version: {}", latest_version);
    println!("Current Version: {}", version());
    if latest_version != version() || force {
        println!("Updating weather.exe at {}", application_path.display());
        let mut updater_location = application_path
            .parent()
            .expect("no parent dir")
            .to_path_buf();
        if cfg!(windows) {
            updater_location.push("components");
            updater_location.push("updater.exe");
        } else {
            updater_location.push("components");
            updater_location.push("updater");
        }
        if !updater_location.exists() {
            println!("Updater not found, downloading updater");
            get_updater(updater_location.display().to_string())?;
            let resp: Value = serde_json::from_str(
                &networking::get_url(
                    "https://arihant2math.github.io/weathercli/index.json",
                    None,
                    None,
                    None,
                )?.text,
            )?;
            let mut web_hash = resp["updater-exe-hash-unix"]
                .as_str()
                .expect("updater-exe-hash-unix key not found in index.json");
            if cfg!(windows) {
                web_hash = resp["updater-exe-hash-windows"]
                    .as_str()
                    .expect("updater-exe-hash-windows key not found in index.json");
            }
            if crate::util::hash_file(&updater_location.display().to_string())? != web_hash
                || force
            {
                get_updater(updater_location.display().to_string())?;
            }
            println!("Starting updater and exiting");
            if force {
                std::process::Command::new(updater_location.display().to_string())
                    .arg("--force")
                    .spawn()
                    .expect("spawn failed");
            } else {
                std::process::Command::new(updater_location.display().to_string())
                    .spawn()
                    .expect("spawn failed");
            }
        }
    }
    Ok(())
}

pub fn credits() {
    println!("Backends:\nMeteo - https://open-meteo.com\nOpen Weather Map - https://openweathermap.org/\nNWS - weather.gov");
    println!("Icons from Icons8: https://icons8.com/");
}
