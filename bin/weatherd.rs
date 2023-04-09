use std::{thread, time};
use std::env::current_exe;
use std::fs;
use std::io::Write;

use clap::Parser;

use weather_core::autolaunch::{AutoLaunchBuilder, Error};
use weather_core::local::settings::Settings;
use weather_core::local::weather_file::WeatherFile;

#[derive(Clone, Parser)]
struct Cli {
    #[arg(long, short, default_value_t = String::from("start"))]
    action: String,
    #[clap(long, short, action)]
    quiet: bool,
    #[clap(long, short, action)]
    version: bool,
    #[clap(long, short, action)]
    no_register: bool,
}

fn register() -> Result<(), Error> {
    let path = current_exe()?.display().to_string();
    let auto = AutoLaunchBuilder::new()
        .set_app_name("weatherd")
        .set_app_path(&path)
        .set_use_launch_agent(true)
        .build()?;
    if !auto.is_enabled()? {
        auto.enable()?;
    }
    Ok(())
}

fn unregister() -> Result<(), Error> {
    let path = current_exe()?.display().to_string();
    let auto = AutoLaunchBuilder::new()
        .set_app_name("weatherd")
        .set_app_path(&path)
        .set_use_launch_agent(true)
        .build()?;
    if auto.is_enabled()? {
        auto.disable()?;
    }
    Ok(())
}

fn main() {
    let settings = Settings::new();
    let args = Cli::parse();
    if args.version && !args.quiet {
            println!("{}", weather_core::version());
        return;
    }
    if args.action == "unregister" || args.action == "uninstall" {
        unregister().expect("Unregistering failed");
    }
    if args.action == "register" || (args.action == "start" && (!args.no_register)) {
        register().expect("Registering failed");
    }
    if args.action == "start" {
        let mut enabled = settings.internal.enable_daemon.unwrap();
        while enabled {
            if !args.quiet {
                println!("Updating Data ...");
            }
            let sleep_duration =
                time::Duration::from_secs(settings.internal.daemon_update_interval.unwrap() as u64);
            enabled = settings.internal.enable_daemon.unwrap();
            let default_datasource = &*settings.internal.default_backend.clone().unwrap();
            if default_datasource.to_lowercase() == "openweathermap" {
                let data =
                    weather_core::backend::openweathermap::open_weather_map_get_combined_data_formatted(
                        "https://api.openweathermap.org/data/2.5/",
                        settings.internal.open_weather_map_api_key.clone().unwrap(),
                                    weather_core::location::get_location(
                                        false,
                                        settings.internal.constant_location.unwrap()).to_vec(),
                                    settings.internal.metric_default.unwrap());
                let bytes = bincode::serialize(&data).expect("Serialization Failed");
                let out = WeatherFile::new("d.cache");
                let path = out.path;
                let mut file = fs::OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .create(true)
                    .open(path)
                    .expect("file open failed");
                file.write_all(&bytes).expect("Write failed");
            }
            thread::sleep(sleep_duration);
        }
    }
}
