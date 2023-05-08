// TODO: switch allocator to jebmalloc due to simd_json performance
use std::time::{SystemTime, UNIX_EPOCH};

use log::LevelFilter;
use log4rs::append::console::{ConsoleAppender, Target};
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::filter::threshold::ThresholdFilter;
use log4rs::Handle;

use crate::custom_backend::dynamic_library_loader::{is_valid_ext, ExternalBackends};
use crate::local::dirs::weathercli_dir;
#[cfg(feature = "gui")]
use crate::local::settings_app;
use crate::util::Config;

use crate::local::dirs::custom_backends_dir;
use log::debug;
use std::{fs, io};

pub mod backend;
pub mod cli;
pub mod custom_backend;
pub mod error;
pub mod layout;
pub mod local;
pub mod location;
pub mod prompt;
pub mod updater;
pub mod util;

pub type Result<T> = std::result::Result<T, error::Error>;

pub fn now() -> u128 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).expect(
        "Time went backwards :( or there is an overflow error of some sort and stuff broke",
    );
    since_the_epoch.as_millis()
}

pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[cfg(feature = "gui")]
pub fn open_settings_app() {
    settings_app::run_settings_app().expect("App Failed");
}

#[cfg(not(feature = "gui"))]
pub fn open_settings_app() {
    panic!("GUI support not enabled!");
}

#[cfg(target_os = "windows")]
pub const CONFIG: Config<'static> = Config {
    weather_file_name: "weather.exe",
    weather_d_file_name: "weatherd.exe",
    updater_file_name: "updater.exe",
};

#[cfg(not(target_os = "windows"))]
pub const CONFIG: Config<'static> = Config {
    weather_file_name: "weather",
    weather_d_file_name: "weatherd",
    updater_file_name: "updater",
};


pub fn init_logging() -> crate::Result<Handle> {
    let level = LevelFilter::Info;
    let mut file_path = weathercli_dir()?.join("logs");
    file_path.push(format!("{}.log", now()));
    // Build a stderr logger.
    let stderr = ConsoleAppender::builder()
        .target(Target::Stderr)
        .encoder(Box::new(PatternEncoder::new("{m}\n")))
        .build();
    // Logging to log file.
    let logfile = FileAppender::builder()
        // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
        .encoder(Box::new(PatternEncoder::new("[{l} {M} {d}] {m}\n")))
        .build(file_path.as_os_str().to_str().unwrap())
        .unwrap();

    // Log Trace level output to file where trace is the default level
    // and the programmatically specified level to stderr.
    let config = log4rs::Config::builder()
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
    Ok(log4rs::init_config(config).unwrap())
}

fn is_ext(f: &io::Result<fs::DirEntry>) -> bool {
    match f {
        Err(_e) => false,
        Ok(dir) => {
            if dir.metadata().is_ok()
                && dir.metadata().unwrap().is_file()
                && is_valid_ext(dir.file_name().to_str().unwrap())
            {
                return true;
            }
            false
        }
    }
}

pub fn load_custom_backends() -> crate::Result<ExternalBackends> {
    debug!("Detecting external dlls");
    let path = custom_backends_dir()?;
    let plugins: Vec<String> = path
        .read_dir()
        .expect("Reading the custom_backends dir failed")
        .filter(is_ext) // We only care about files
        .map(|f| f.unwrap().path().display().to_string())
        .collect();
    debug!("Loading: {plugins:?}");
    let custom_backends = crate::custom_backend::dynamic_library_loader::load(plugins);
    Ok(custom_backends)
}
