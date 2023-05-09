use crate::arguments::BackendOpts;
use custom_backend::dynamic_library_loader::is_valid_ext;
use local::list_dir;
use local::settings::Settings;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use terminal::color;
use terminal::prompt;
use weather_dirs::custom_backends_dir;

fn install(path: String) -> crate::Result<()> {
    // TODO: Add validity checks (prompt user for permission first)
    let real_path = PathBuf::from_str(&path).unwrap();
    let file_name = real_path.file_name().ok_or("Not a file")?.to_str().unwrap();
    if !is_valid_ext(file_name) {
        return Err("Not a valid system extension, aborting")?;
    }
    fs::copy(&real_path, custom_backends_dir()?.join(file_name))?;
    Ok(())
}

fn list(settings: Settings) -> crate::Result<()> {
    let paths = list_dir(custom_backends_dir()?)?;
    for path in paths {
        // TODO: Check which ones are valid (hard to do)
        let file_name = &*path;
        if is_valid_ext(file_name) {
            let valid = settings.enable_custom_backends;
            if valid {
                println!("{}{file_name}", color::FORE_GREEN);
            } else {
                println!("{}{file_name}", color::FORE_RED);
            }
        }
    }
    Ok(())
}

fn select(settings: Settings) -> crate::Result<()> {
    let selected = settings.default_backend;
    let mut settings = Settings::new()?;
    let choices = ["openweathermap", "meteo", "nws", "openweathermap_onecall"];
    let selected_usize = choices.iter().position(|&i| i == selected).unwrap_or(0);
    let choice = prompt::radio(&choices, selected_usize, None)?;
    settings.default_backend = choices[choice].to_string();
    settings.write()?;
    Ok(())
}

fn open_weather_map_api_key(settings: Settings) -> crate::Result<()> {
    let original = settings.open_weather_map_api_key;
    let mut s = prompt::input(
        Some("Enter your openweathermap api key: ".to_string()),
        Some(original),
    )?;
    s = s.trim().to_string();
    if s.len() != 32 {
        return Err(
            "Length of api key is not 32, this likely means you have entered an invalid api key.",
        )?;
    }
    println!("{}Saving api key as {s}", color::FORE_BLUE); // TODO: Fix
    let mut settings = Settings::new()?; // TODO: Fix excess read
    settings.open_weather_map_api_key = s;
    settings.write()?;
    Ok(())
}

fn bing_maps_api_key(settings: Settings) -> crate::Result<()> {
    let original = settings.bing_maps_api_key;
    let mut s = prompt::input(
        Some("Enter your bing maps api key: ".to_string()),
        Some(original),
    )?;
    s = s.trim().to_string();
    println!("{}Saving api key as {s}", color::FORE_BLUE);
    let mut settings = Settings::new()?; // TODO: Fix excess read
    settings.bing_maps_api_key = s;
    settings.write()?;
    Ok(())
}

fn delete() -> crate::Result<()> {
    let paths = list_dir(custom_backends_dir()?)?;
    let choice = paths[prompt::radio(&paths, 0, None)?].to_string();
    fs::remove_file(custom_backends_dir()?.join(&*choice))?;
    Ok(())
}

pub fn subcommand(arg: BackendOpts, settings: Settings) -> crate::Result<()> {
    match arg {
        BackendOpts::Install(opts) => install(opts.path)?,
        BackendOpts::List => list(settings)?,
        BackendOpts::Select => select(settings)?,
        BackendOpts::OpenWeatherMapApiKey => open_weather_map_api_key(settings)?,
        BackendOpts::BingMapsApiKey => bing_maps_api_key(settings)?,
        BackendOpts::Delete => delete()?,
    }
    Ok(())
}
