use std::str::FromStr;
use std::thread;
use std::time::Duration;

use clap::{Args, Parser, Subcommand};
use serde_json::Value;

use weather_core::{component_updater, Datasource, datasource_from_str, networking, version, weather};
use weather_core::component_updater::get_updater;
use weather_core::local::cache::prune_cache;
use weather_core::local::settings::Settings;
use weather_core::local::weather_file::WeatherFile;
use weather_core::location::{get_coordinates, get_location};

#[derive(Clone, Parser)]
#[command(version, author, about, name = "weathercli")]
pub struct App {
    #[command(flatten)]
    global_opts: GlobalOpts,
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Clone, Subcommand)]
enum Command {
    Place(PlaceOpts),
    Settings,
    Config(ConfigOpts),
    PruneCache,
    ClearCache,
    Setup,
    Update(UpdateOpts),
}

#[derive(Clone, Args)]
struct ConfigOpts {
    key: String,
    value: Option<String>,
}

#[derive(Clone, Args)]
struct PlaceOpts {
    query: String,
}

#[derive(Clone, Copy, Args)]
struct UpdateOpts {
    #[arg(long, short, action)]
    force: bool,
}

#[derive(Clone, Copy, Args)]
struct GlobalOpts {
    #[arg(long, short, action, global = true, help = "Print raw json output, useful for debugging")]
    json: bool,
    #[arg(long, short, global = true, value_enum, help = "Which datasource to use, possible options are meteo, nws, and openweathermap")]
    datasource: Option<Datasource>,
    #[arg(long, short, action, global = true, help = "the output will be in metric")]
    metric: bool,
    #[arg(long, short, action, global = true, help = "the output will be in imperial, overrides --metric")]
    imperial: bool,
    #[arg(long, short, action, global = true, help = "If used, the location will not be gotten from the win32 api, if applicable")]
    no_sys_loc: bool,
}

fn config(key_name: String, value: Option<String>) {
    if value.is_none() {
        let f = WeatherFile::settings();
        let data: Value = serde_json::from_str(&f.data).expect("Deserialization failed");
        println!("{}: {}", &key_name, data[&key_name]);
    } else {
        println!(
            "Writing {}={} ...",
            key_name.to_lowercase(),
            value.clone().unwrap()
        );
        let mut f = WeatherFile::settings();
        let mut data: Value = serde_json::from_str(&f.data).expect("Deserialization failed");
        data[key_name.to_uppercase()] =
            Value::from_str(&value.unwrap()).expect("Value conversion failed");
        f.data = serde_json::to_string(&data).expect("Serialization failed");
        f.write();
    }
}

fn setup(settings_s: Settings) {
    let mut settings = settings_s;
    println!(
        "{}===== Weather CLI Setup =====",
        weather_core::color::FORE_CYAN
    );
    weather_core::component_updater::update_web_resources(
        settings.internal.development.unwrap(),
        None,
    );
    println!(
        "{}Choose the default weather backend: ",
        weather_core::color::FORE_RED
    );
    let options = vec![
        "Meteo".to_string(),
        "Open Weather Map".to_string(),
        "National Weather Service".to_string(),
        "The Weather Channel".to_string(),
    ];
    let mut default = vec!["METEO", "OPENWEATHERMAP", "NWS", "THEWEATHERCHANNEL"]
        .iter()
        .position(|&x| x == settings.internal.default_backend.clone().unwrap())
        .unwrap_or(0);
    let current = weather_core::prompt::choice(options, default, None);
    let weather_backend_setting = ["METEO", "OPENWEATHERMAP", "NWS", "THEWEATHERCHANNEL"][current];
    settings.internal.default_backend = Some(weather_backend_setting.to_string());
    settings.write();
    thread::sleep(Duration::from_millis(100));
    println!(
        "{}Is your location constant (i.e. is this computer stationary at all times)?",
        weather_core::color::FORE_RED
    );
    if settings.internal.constant_location.unwrap() {
        default = 0;
    } else {
        default = 1;
    }
    let constant_location_setting = [true, false]
        [weather_core::prompt::choice(vec!["yes".to_string(), "no".to_string()], default, None)];
    settings.internal.constant_location = Some(constant_location_setting);
    settings.write();
    thread::sleep(Duration::from_millis(100));
    println!(
        "{}Should static resources (ascii art, weather code sentences, etc.) be auto-updated?",
        weather_core::color::FORE_RED
    );
    if settings.internal.auto_update_internet_resources.unwrap() {
        default = 0;
    } else {
        default = 1;
    }
    let auto_update_setting = [true, false]
        [weather_core::prompt::choice(vec!["yes".to_string(), "no".to_string()], default, None)];
    settings.internal.auto_update_internet_resources = Some(auto_update_setting);
    settings.write();
}

fn update(force: bool) {
    println!("Checking for updates ...");
    let latest_version = component_updater::get_latest_version();
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
            get_updater(updater_location.display().to_string());
            let resp: Value = serde_json::from_str(
                &networking::get_url(
                    "https://arihant2math.github.io/weathercli/index.json".to_string(),
                    None,
                    None,
                    None,
                )
                .text,
            )
            .expect("JSON deserialize failed");
            let mut web_hash = resp["updater-exe-hash-unix"]
                .as_str()
                .expect("updater-exe-hash-unix key not found in index.json");
            if cfg!(windows) {
                web_hash = resp["updater-exe-hash-windows"]
                    .as_str()
                    .expect("updater-exe-hash-windows key not found in index.json");
            }
            if weather_core::hash_file(&updater_location.display().to_string()) != web_hash || force
            {
                get_updater(updater_location.display().to_string());
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
}

fn main() {
    let args = App::parse();
    let settings = Settings::new();
    let mut true_metric = settings.internal.metric_default.unwrap();
    if args.global_opts.metric {
        true_metric = true;
    }
    if args.global_opts.imperial {
        true_metric = false;
    }
    let datasource = args
        .global_opts
        .datasource
        .unwrap_or(datasource_from_str(&settings.internal.default_backend.clone().unwrap()));
    match args.command {
        Some(command) => {
            match command {
                Command::Place(opts) => weather(
                    datasource,
                    get_coordinates(
                        opts.query,
                        settings
                            .internal
                            .bing_maps_api_key
                            .clone()
                            .unwrap_or(String::new()),
                    )
                    .expect("Location not found"),
                    settings,
                    true_metric,
                    args.global_opts.json,
                ),
                Command::Config(opts) => config(opts.key, opts.value),
                Command::Settings => weather_core::open_settings_app(),
                Command::ClearCache => {
                    let mut f = WeatherFile::new("d.cache");
                    f.data = String::new();
                    f.write();
                },
                Command::PruneCache => prune_cache(),
                Command::Setup => setup(settings),
                Command::Update(opts) => update(opts.force),
            };
        }
        None => weather(
            datasource,
            get_location(
                args.global_opts.no_sys_loc,
                settings.internal.constant_location.unwrap(),
            ),
            settings,
            true_metric,
            args.global_opts.json,
        ),
    };
}
