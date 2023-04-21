use std::thread;
use std::time::Duration;

use clap::{Args, Parser, Subcommand};

use weather_core::local::settings::Settings;
use weather_core::local::weather_file::WeatherFile;
use weather_core::location::{get_coordinates, get_location};
use weather_core::weather;

#[derive(Debug, Parser)]
#[clap(name = "weathercli")]
pub struct App {
    #[clap(flatten)]
    global_opts: GlobalOpts,
    #[clap(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    Place(PlaceOpts),
    Settings,
    ClearCache,
    Setup
}

#[derive(Debug, Args)]
struct PlaceOpts {
    query: String,
}

#[derive(Debug, Args)]
struct GlobalOpts {
    #[clap(long, short, action, global = true)]
    json: bool,
    #[clap(long, short, global = true)]
    datasource: Option<String>,
    #[clap(long, short, action, global = true)]
    metric: bool,
    #[clap(long, short, action, global = true)]
    imperial: bool,
    #[clap(long, short, action, global = true)]
    no_sys_loc: bool,
}

fn setup(settings_s: Settings) {
    let mut settings = settings_s;
    println!("{}===== Weather CLI Setup =====", weather_core::color::FORE_CYAN);
    weather_core::component_updater::update_web_resources(settings.internal.development.unwrap(), None);
    println!("{}Choose the default weather backend: ", weather_core::color::FORE_RED);
    let options = vec!["Meteo".to_string(), "Open Weather Map".to_string(), "National Weather Service".to_string(), "The Weather Channel".to_string()];
    let mut default = vec!["METEO", "OPENWEATHERMAP", "NWS", "THEWEATHERCHANNEL"].iter()
        .position(|&x| x == settings.internal.default_backend.clone().unwrap()).unwrap_or(0);
    let current = weather_core::prompt::choice(options, default, None);
    let weather_backend_setting = ["METEO", "OPENWEATHERMAP", "NWS", "THEWEATHERCHANNEL"][current];
    settings.internal.default_backend = Some(weather_backend_setting.to_string());
    settings.write();
    thread::sleep(Duration::from_millis(100));
    println!("{}Is your location constant (i.e. is this computer stationary at all times)?", weather_core::color::FORE_RED);
    if settings.internal.constant_location.unwrap() {
        default = 0;
    } else {
        default = 1;
    }
    let constant_location_setting = [true, false][weather_core::prompt::choice(vec!["yes".to_string(), "no".to_string()], default, None)];
    settings.internal.constant_location = Some(constant_location_setting);
    settings.write();
    thread::sleep(Duration::from_millis(100));
    println!("{}Should static resources (ascii art, weather code sentences, etc.) be auto-updated?", weather_core::color::FORE_RED);
    if settings.internal.auto_update_internet_resources.unwrap() {
        default = 0;
    } else {
        default = 1;
    }
    let auto_update_setting = [true, false][weather_core::prompt::choice(vec!["yes".to_string(), "no".to_string()], default, None)];
    settings.internal.auto_update_internet_resources = Some(auto_update_setting);
    settings.write();
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
        .unwrap_or(settings.internal.default_backend.clone().unwrap());
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
                Command::Settings => weather_core::open_settings_app(),
                Command::ClearCache => {
                    let mut f = WeatherFile::new("d.cache");
                    f.data = String::new();
                    f.write();
                }
                Command::Setup => setup(settings)
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
