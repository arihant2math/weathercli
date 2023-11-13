use std::{thread, time};
use std::env::current_exe;
use std::fs;
use std::io::Write;

use auto_launch::{AutoLaunchBuilder, Error};
use clap::Parser;

use cli::Datasource;
use custom_backend::dynamic_library_loader::ExternalBackends;
use local::settings::Settings;
use local::weather_file::WeatherFile;

pub type CResult<T> = Result<T, weather_error::Error>;

#[derive(Clone, Parser)]
struct Cli {
    #[arg(long, short, default_value_t = String::from("start"))]
    action: String,
    #[arg(long, short, action)]
    quiet: bool,
    #[arg(long, short, action)]
    version: bool,
    #[arg(long, short, action)]
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

fn main() -> CResult<()> {
    let settings = Settings::new()?;
    let args = Cli::parse();

    if args.action == "unregister" || args.action == "uninstall" {
        unregister().expect("Unregistering failed");
    }
    if args.action == "register" || (args.action == "start" && (!args.no_register)) {
        register().expect("Registering failed");
    }
    if args.action == "start" {
        let mut enabled = settings.enable_daemon;
        while enabled {
            if !args.quiet {
                println!("Updating Data ...");
            }
            #[allow(clippy::cast_sign_loss)]
            let sleep_duration = time::Duration::from_secs(settings.daemon_update_interval as u64);
            enabled = settings.enable_daemon;
            let default_datasource = &*settings.default_backend.clone();
            if default_datasource.to_lowercase() == "openweathermap" {
                let data = cli::commands::get_data_from_datasource(
                    Datasource::from_str(default_datasource),
                    local::location::get(false, settings.constant_location)?,
                    settings.clone(),
                    ExternalBackends::default(),
                )?;
                let bytes = bincode::serialize(&data).expect("Serialization Failed");
                let out = WeatherFile::new("d.cache")?;
                let path = out.path;
                let mut file = fs::OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .create(true)
                    .open(path)?;
                file.write_all(&bytes)?;
            }
            thread::sleep(sleep_duration);
        }
    }
    Ok(())
}
