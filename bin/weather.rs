use clap::{Args, Parser, Subcommand};

use weather_core::local::settings::Settings;
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
