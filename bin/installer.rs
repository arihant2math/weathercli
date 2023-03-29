use std::fs;
use std::path::Path;

use clap::Parser;
#[cfg(target_os = "windows")]
use winreg::enums::*;
#[cfg(target_os = "windows")]
use winreg::RegKey;

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

#[cfg(target_os = "windows")]
fn add_to_path(dir: String) {
    println!("Adding to Path ...");
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let environment = hklm
        .open_subkey(r#"SYSTEM\CurrentControlSet\Control\Session Manager\Environment"#)
        .expect("");
    let mut path: String = environment.get_value("Path").expect("");
    let append = fs::canonicalize(dir).unwrap().display().to_string();
    if path.chars().last().unwrap_or(';') != ';' {
        path += ";"
    }
    path += &append;
    environment
        .set_value("Path", &path)
        .expect("RegEdit write failed");
}

#[cfg(not(target_os = "windows"))]
fn add_to_path(dir: String) {
    println!("Add to path is unsupported for your system")
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
    let is_empty = dir_path
        .read_dir()
        .expect("Dir read failed, check if script has appropriate permissions")
        .next()
        .is_none();
    if !is_empty {
        return Err("Directory is not empty".to_string());
    }
    let url;
    let path;
    if cfg!(windows) {
        url = "https://arihant2math.github.io/weathercli/weather.exe";
        path = "weather.exe";
    } else if cfg!(unix) {
        url = "https://arihant2math.github.io/weathercli/weather";
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
    )
    .await?;
    let url;
    let path;
    if cfg!(windows) {
        url = "https://arihant2math.github.io/weathercli/weatherd.exe";
        path = "weatherd.exe";
    } else if cfg!(unix) {
        url = "https://arihant2math.github.io/weathercli/weatherd";
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
    )
    .await?;
    if args.add_to_path {
        add_to_path(dir_path.display().to_string());
    }
    update_web_resources(false, Some(false));
    Ok(())
}
