use local::settings::Settings;
use std::thread;
use std::time::Duration;
use terminal::color::{FORE_CYAN, FORE_LIGHTMAGENTA};
use terminal::prompt::{input, yes_no};
use updater::component::update_component;

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

pub fn update(force: bool, dry_run: bool, version: String) -> crate::Result<()> {
    println!("Checking for updates ...");
    let latest_version = updater::get_latest_version()?;
    let application_path = std::env::current_exe()?;
    println!("Latest Version: {latest_version}");
    println!("Current Version: {version}");
    if latest_version != version || force {
        println!("Updating weather.exe at {}", application_path.display());
        if !dry_run {
            update_component(
                &("https://arihant2math.github.io/weathercli/".to_string()
                    + updater::CONFIG.weather_file_name),
                &std::env::current_exe()?.display().to_string(),
                false,
            )?;
        }
    }
    Ok(())
}
