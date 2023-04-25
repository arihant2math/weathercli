use std::{fs, io};

use clap::Parser;
use log4rs::append::console::{ConsoleAppender, Target};
use log4rs::append::file::FileAppender;
use log4rs::Config;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::filter::threshold::ThresholdFilter;
use log::LevelFilter;

use weather_core::cli::{App, Command, datasource_from_str};
use weather_core::cli::commands::{credits, setup, update, weather};
use weather_core::dynamic_loader::ExternalBackends;
use weather_core::local::cache::prune_cache;
use weather_core::local::dirs::home_dir;
use weather_core::local::settings::Settings;
use weather_core::local::weather_file::WeatherFile;
use weather_core::location::{get_coordinates, get_location};

#[cfg(target_family = "windows")]
fn is_valid_ext(f: &str) -> bool {
    let len = f.len();
    &f[len-4 ..] == ".dll"
}

#[cfg(target_family = "unix")]
fn is_valid_ext(f: &String) -> bool {
    let len = f.len();
    &f[len-3 ..] == ".so"
}


fn is_ext(f: &io::Result<fs::DirEntry>) -> bool {
    match f {
        Err(_e) => false, // TODO: Re-emit error
        Ok(dir) => {
            if dir.metadata().is_ok() && dir.metadata().unwrap().is_file() && is_valid_ext(dir.file_name().to_str().unwrap()) {
                return true
            }
            return false;
        }
    }
}

fn main() {
    let level = LevelFilter::Info;
    let file_path = "/tmp/foo.log";

    // Build a stderr logger.
    let stderr = ConsoleAppender::builder().target(Target::Stderr).build();

    // Logging to log file.
    let logfile = FileAppender::builder()
        // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build(file_path)
        .unwrap();

    // Log Trace level output to file where trace is the default level
    // and the programmatically specified level to stderr.
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(level)))
                .build("stderr", Box::new(stderr)),
        )
        .build(
            Root::builder()
                .appender("logfile")
                .appender("stderr")
                .build(LevelFilter::Trace),
        )
        .unwrap();

    // Use this to change log levels at runtime.
    // This means you can change the default log level to trace
    // if you are trying to debug an issue and need more logs on then turn it off
    // once you are done.
    let _handle = log4rs::init_config(config).unwrap();

    let args = App::parse();
    let settings = Settings::new();
    let mut true_metric = settings.internal.metric_default;
    if args.global_opts.metric {
        true_metric = true;
    }
    if args.global_opts.imperial {
        true_metric = false;
    }
    let datasource = datasource_from_str(&args.global_opts.datasource.unwrap_or(
        settings.internal.default_backend.clone(),
    ));
    let mut custom_backends = ExternalBackends::default();
    if settings.internal.enable_custom_backends { // TODO: make sure its a custom backend
        let mut path = home_dir().expect("expect home dir");
        path.push(".weathercli");
        path.push("custom_backends");
        if path.exists() {
            let plugins: Vec<String> = path.read_dir().expect("Reading the custom_backends dir failed")
                .filter(|f| is_ext(f)) // We only care about files
                .map(|f| f.unwrap().path().display().to_string())
                .collect();
            custom_backends = weather_core::dynamic_loader::load(plugins);
        }
    }
    match args.command {
        Some(command) => {
            match command {
                Command::Place(opts) => weather(
                    datasource,
                    get_coordinates(opts.query, settings.internal.bing_maps_api_key.clone())
                        .expect("Location not found"),
                    settings,
                    true_metric,
                    args.global_opts.json,
                    custom_backends
                ),
                Command::Config(opts) => weather_core::cli::commands::config(opts.key, opts.value),
                Command::Settings => weather_core::open_settings_app(),
                Command::ClearCache => {
                    let mut f = WeatherFile::new("d.cache");
                    f.data = String::new();
                    f.write();
                }
                Command::PruneCache => prune_cache(),
                Command::Setup => setup(settings),
                Command::Update(opts) => update(opts.force),
                Command::Credits => credits(),
            };
        }
        None => weather(
            datasource,
            get_location(
                args.global_opts.no_sys_loc,
                settings.internal.constant_location,
            ),
            settings,
            true_metric,
            args.global_opts.json,
            custom_backends
        ),
    };
}
