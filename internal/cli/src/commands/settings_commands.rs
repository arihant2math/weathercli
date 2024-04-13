use local::settings::Settings;
use local::weather_file::WeatherFile;
use shared_deps::serde_json::Value;
use shared_deps::simd_json;
use crate::arguments::{SettingsKeyOpts, SettingsKeyValueOpts, SettingsOpts};
use crate::commands::open_settings_app;
use std::str::FromStr;
use terminal::prompt;

fn view(settings: &Settings, args: SettingsKeyOpts) -> crate::Result<()> {
    match args.key {
        Some(key) => {
            println!("{}: {}", &key, settings.get(&key.to_uppercase())?);
        },
        None => {
            println!("Default Datasource (DEFAULT_BACKEND): {}", settings.default_backend);
            println!("Metric by Default (METRIC_DEFAULT): {}", settings.metric_default);
            println!("Show Alerts (SHOW_ALERTS): {}", settings.show_alerts);
            println!("Constant Location (CONSTANT_LOCATION): {}", settings.constant_location);
            println!("Auto Update Resources (AUTO_UPDATE_INTERNET_RESOURCES): {}", settings.auto_update_internet_resources);
            println!("Enable Custom Backends (ENABLE_CUSTOM_BACKENDS): {}", settings.enable_custom_backends);
            println!("Enable Wasm Backends (ENABLE_WASM_BACKENDS): {}", settings.enable_wasm_backends);
        }
    }
    Ok(())
}

pub fn edit_settings() -> crate::Result<()> {
    let mut settings = Settings::new()?;
    let result = prompt::multiselect(
        &[
            "Metric",
            "Show Alerts",
            "Constant Location",
            "Auto Update Resources",
        ],
        &[
            settings.metric_default,
            settings.show_alerts,
            settings.constant_location,
            settings.auto_update_internet_resources,
        ],
        None,
    )?;
    settings.metric_default = result[0];
    settings.show_alerts = result[1];
    settings.constant_location = result[2];
    settings.auto_update_internet_resources = result[3];
    settings.write()?;
    Ok(())
}


fn edit(settings: &mut Settings, arg: SettingsKeyValueOpts) -> crate::Result<()> {
    match arg.key {
        Some(key) => {
            if let Some(value) = arg.value {
                println!("Writing {}={} ...", key.to_lowercase(), &value);
                let mut f = WeatherFile::settings()?;
                unsafe {
                    let mut data: Value = simd_json::from_str(&mut f.get_text()?)?;
                    data[key.to_uppercase()] = Value::from_str(&value)?;
                    f.data = Vec::from(simd_json::to_string(&data)?);
                    f.write()?;
                }
            } else {
                match &*key.to_uppercase() {
                    "BACKEND" | "DEFAULT_BACKEND" => {
                        crate::commands::backend_commands::select(settings)?;
                    },
                    "LAYOUT" | "LAYOUT_FILE" => {
                        crate::commands::layout_commands::select(settings)?;
                    },
                    "METRIC_DEFAULT" | "UNITS" => {
                        println!("Use metric by default:");
                        terminal::prompt::yes_no(settings.metric_default, None).map(|b| settings.metric_default = b)?;
                        settings.write()?;
                    },
                    "SHOW_ALERTS" => {
                        println!("Show alerts:");
                        terminal::prompt::yes_no(settings.show_alerts, None).map(|b| settings.show_alerts = b)?;
                        settings.write()?;
                    },
                    "CONSTANT_LOCATION" => {
                        println!("Use constant location:");
                        terminal::prompt::yes_no(settings.constant_location, None).map(|b| settings.constant_location = b)?;
                        settings.write()?;
                    },
                    "AUTO_UPDATE_INTERNET_RESOURCES" => {
                        println!("Auto update internet resources:");
                        terminal::prompt::yes_no(settings.auto_update_internet_resources, None).map(|b| settings.auto_update_internet_resources = b)?;
                    },
                    "ENABLE_CUSTOM_BACKENDS" => {
                        println!("Enable custom backends:");
                        terminal::prompt::yes_no(settings.enable_custom_backends, None).map(|b| settings.enable_custom_backends = b)?;
                    }
                    "ENABLE_WASM_BACKENDS" => {
                        println!("Enable wasm backends (WARNING: THIS IS EXPERIMENTAL AND COULD AFFECT STABILITY):");
                        terminal::prompt::yes_no(settings.enable_wasm_backends, None).map(|b| settings.enable_wasm_backends = b)?;
                    }
                    _ => {
                        println!("Current Value: {}", settings.get(&key)?);
                    }
                }
            }
        }
        None => {
            edit_settings()?;
        }
    }
    Ok(())
}

pub fn subcommand(arg: SettingsOpts, settings: &mut Settings) -> crate::Result<()> {
    match arg {
        SettingsOpts::View(key_args) => view(settings, key_args),
        SettingsOpts::Edit(key_value_args) => edit(settings, key_value_args),
        SettingsOpts::GuiEdit => Ok(open_settings_app()),
    }
}