use std::{thread, time};
use std::env::current_exe;

use auto_launch::{AutoLaunchBuilder, Error};
use clap::Parser;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use weather_core::local::settings::Settings;
use weather_core::local::weather_file::WeatherFile;

#[derive(Clone, Parser)]
struct Cli {
    #[arg(long, short, default_value_t = String::from("start"))]
    action: String,
}


fn register() -> Result<(), Error>  {
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
    if args.action == "unregister" || args.action == "uninstall" {
        unregister().expect("Unregistering failed");
    }
    if args.action == "register" || args.action == "start" {
        register().expect("Registering failed");
    }
    if args.action == "start" {
        let sleep_duration = time::Duration::from_secs(settings.internal.daemon_update_interval.unwrap() as u64);
        loop {
            println!("Updating Data ...");
            let w = WeatherFile::new("downloader_urls.list".to_string());
            let urls_split = w.data.split('\n');
            let urls = urls_split.collect::<Vec<&str>>();
            let data: Vec<_> = urls
                .par_iter()
                .map(|url| {
                    reqwest::blocking::get(url.to_string())
                        .expect("Url Get failed")
                        .text()
                        .unwrap()
                })
                .collect();
            let mut out = WeatherFile::new("d.cache".to_string());
            let joined =
                weather_core::local::cache::get_date_string() + "EOF\n\n\n\n\nBEGIN" + &*data.join("EOF\n\n\n\n\nBEGIN");
            out.data = joined;
            out.write();
            thread::sleep(sleep_duration);
        }
    }
}
