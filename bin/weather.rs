use clap::Parser;

use weather_core::cli::{App, Command, datasource_from_str};
use weather_core::cli::commands::{config, credits, setup, update, weather};
use weather_core::local::cache::prune_cache;
use weather_core::local::dirs::home_dir;
use weather_core::local::settings::Settings;
use weather_core::local::weather_file::WeatherFile;
use weather_core::location::{get_coordinates, get_location};

fn main() {
    let args = App::parse();
    let settings = Settings::new();
    if settings.internal.enable_custom_backends {
        let mut path = home_dir().expect("expect home dir");
        path.push(".weathercli");
    }
    let mut true_metric = settings.internal.metric_default;
    if args.global_opts.metric {
        true_metric = true;
    }
    if args.global_opts.imperial {
        true_metric = false;
    }
    let datasource = args.global_opts.datasource.unwrap_or(datasource_from_str(
        &settings.internal.default_backend,
    ));
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
                ),
                Command::Config(opts) => config(opts.key, opts.value),
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
        ),
    };
}
