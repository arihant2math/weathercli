use std::fs;
use std::path::Path;

use clap::Parser;

use weather_core::bin_common::update_component;
use weather_core::component_updater::update_web_resources;

#[derive(Clone, Parser)]
struct Cli {
    #[arg(long, short)]
    install_dir: String,
    #[clap(long, short, action)]
    add_to_path: bool,
    #[clap(long, short, action)]
    guided: bool,
    #[clap(long, short, action)]
    quiet: bool,
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let args = Cli::parse();
    if args.guided {
        println!("WeatherCLI installer");
        return Ok(());
    }
    if !args.quiet {
        println!("Installing ...")
    }
    let dir_path = Path::new(&args.install_dir);
    if dir_path.is_file() {
        return Err("Install path is a file".to_string());
    }
    if !dir_path.exists() {
        fs::create_dir(&args.install_dir).expect("Directory Creation Failed");
    }
    let is_empty = dir_path.read_dir()
        .expect("Dir read failed, check if script has appropriate permissions").next().is_none();
    if !is_empty {
        return Err("Directory is not empty".to_string());
    }
    let url;
    let path;
    if cfg!(windows) {
        url = "https://arihant2math.github.io/weathercli/docs/weather.exe";
        path = "weather.exe";
    } else if cfg!(unix) {
        url = "https://arihant2math.github.io/weathercli/docs/weather";
        path = "weather";
    } else {
        return Err("OS unsupported".to_string());
    }
    update_component(
            url,
            path,
            "Downloading weathercli from ".to_string(),
            "Installed weathercli".to_string(),
            args.quiet,
    ).await?;
    let url;
    let path;
    if cfg!(windows) {
        url = "https://arihant2math.github.io/weathercli/docs/weatherd.exe";
        path = "weatherd.exe";
    } else if cfg!(unix) {
        url = "https://arihant2math.github.io/weathercli/docs/weatherd";
        path = "weatherd";
    } else {
        return Err("OS unsupported".to_string());
    }
    update_component(
        url,
        path,
        "Downloading daemon from ".to_string(),
        "Installed daemon".to_string(),
        args.quiet,
    ).await?;
    update_web_resources(false, Some(false));
    Ok(())
}