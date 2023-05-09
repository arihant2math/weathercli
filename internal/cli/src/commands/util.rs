use local::settings::Settings;
use serde_json::Value;
use std::time::Duration;
use std::{fs, thread};
use terminal::color::{FORE_CYAN, FORE_LIGHTMAGENTA};
use terminal::prompt::{input, yes_no};

pub fn setup(settings_s: Settings) -> crate::Result<()> {
    let mut settings = settings_s;
    println!("{FORE_CYAN}===== Weather CLI Setup =====");
    updater::resource::update_web_resources(settings.update_server.clone(), None)?;
    println!("{FORE_LIGHTMAGENTA}Choose the default weather backend: ");
    let options = [
        "Meteo",
        "Open Weather Map OneCall",
        "Open Weather Map (API Key required)",
        "National Weather Service",
        "The Weather Channel",
    ];
    let default = [
        "METEO",
        "OPENWEATHERMAP_ONECALL",
        "OPENWEATHERMAP",
        "NWS",
        "THEWEATHERCHANNEL",
    ]
    .iter()
    .position(|&x| x == settings.default_backend.clone())
    .unwrap_or(0);
    let current = terminal::prompt::radio(&options, default, None)?;
    let weather_backend_setting = [
        "METEO",
        "OPENWEATHERMAP_ONECALL",
        "OPENWEATHERMAP",
        "NWS",
        "THEWEATHERCHANNEL",
    ][current];
    settings.default_backend = weather_backend_setting.to_string();
    settings.write()?;
    if settings.default_backend == "OPENWEATHERMAP" {
        println!("{FORE_LIGHTMAGENTA}Do you want to enter your openweathermap API key?");
        let cont = yes_no(true, None)?;
        if cont {
            let resp = input(Some("Enter your openweathermap key: ".to_string()), None)?;
            settings.open_weather_map_api_key = resp;
            settings.write()?;
        }
    }
    thread::sleep(Duration::from_millis(100));
    println!(
        "{FORE_LIGHTMAGENTA}Is your location constant (i.e. is this computer stationary at all times)?"
    );
    settings.constant_location = yes_no(settings.constant_location, None)?;
    settings.write()?;
    thread::sleep(Duration::from_millis(100));
    println!("{FORE_LIGHTMAGENTA}Should static resources (ascii art, weather code sentences, etc.) be auto-updated?");
    settings.auto_update_internet_resources =
        yes_no(settings.auto_update_internet_resources, None)?;
    settings.write()?;
    Ok(())
}

pub fn update(force: bool, version: String) -> crate::Result<()> {
    println!("Checking for updates ...");
    let latest_version = updater::get_latest_version()?;
    let application_path = std::env::current_exe()?;
    println!("Latest Version: {latest_version}");
    println!("Current Version: {version}");
    if latest_version != version || force {
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
        unsafe {
            if !updater_location.exists() {
                println!("Updater not found, downloading updater");
                updater::get_updater(updater_location.display().to_string())?;
                let resp: Value = simd_json::from_str(
                    &mut networking::get_url(
                        "https://arihant2math.github.io/weathercli/index.json",
                        None,
                        None,
                        None,
                    )?
                    .text,
                )?;
                let web_hash = if cfg!(windows) {
                    resp["updater-exe-hash-windows"]
                        .as_str()
                        .expect("updater-exe-hash-windows key not found in index.json")
                } else {
                    resp["updater-exe-hash-unix"]
                        .as_str()
                        .expect("updater-exe-hash-unix key not found in index.json")
                };
                if updater::hash_file(&updater_location.display().to_string())? != web_hash
                    || force
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
    }
    Ok(())
}
