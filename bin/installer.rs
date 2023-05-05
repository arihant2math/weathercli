use std::fs;
use std::path::Path;

use clap::Parser;
#[cfg(target_os = "windows")]
use winreg::enums::HKEY_LOCAL_MACHINE;
#[cfg(target_os = "windows")]
use winreg::RegKey;

use weather_core::CONFIG;
use weather_core::local::settings::Settings;
use weather_core::updater::component::update_component;
use weather_core::updater::resource::update_web_resources;

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
fn add_to_path(dir: String) -> weather_core::Result<()> {
    println!("Adding to Path ...");
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let environment =
        hklm.open_subkey(r#"SYSTEM\CurrentControlSet\Control\Session Manager\Environment"#)?;
    let mut path: String = environment.get_value("Path")?;
    let append = fs::canonicalize(dir)?.display().to_string();
    if path.chars().last().unwrap_or(';') != ';' {
        path += ";"
    }
    path += &append;
    environment.set_value("Path", &path)?;
    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn add_to_path(dir: String) -> weather_core::Result<()> {
    return Err("Add to path is unsupported for your system")?;
}

#[tokio::main]
async fn main() -> weather_core::Result<()> {
    let args = Cli::parse();
    let settings = Settings::new()?;
    if args.guided {
        println!("WeatherCLI installer");
        return Ok(());
    }
    if !args.quiet {
        println!("Installing ...")
    }
    let dir_path = Path::new(&args.install_dir);
    if dir_path.is_file() {
        return Err("Install path is a file".to_string())?;
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
        return Err("Directory is not empty".to_string())?;
    }
    let url = settings.internal.update_server.clone() + &CONFIG.weather_file_name;
    let path = dir_path.to_path_buf().join(CONFIG.weather_file_name);
    update_component(
        &url,
        &path.display().to_string(),
        "Downloading weathercli from ".to_string(),
        "Installed weathercli".to_string(),
        args.quiet,
    )
    .await?;
    let url = settings.internal.update_server.clone() + &CONFIG.weather_d_file_name;
    let path = dir_path
        .to_path_buf()
        .join("internal")
        .join(CONFIG.weather_d_file_name);
    update_component(
        &url,
        &path.display().to_string(),
        "Downloading daemon from ".to_string(),
        "Installed daemon".to_string(),
        args.quiet,
    )
    .await?;
    if args.add_to_path {
        add_to_path(dir_path.display().to_string())?;
    }
    update_web_resources(settings.internal.update_server, Some(false))?;
    Ok(())
}
