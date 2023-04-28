use std::{fs, process};
use std::path::Path;

use clap::{Args, Parser, Subcommand};

use crate::update_hash::update_hash;

mod update_docs;
mod update_hash;

#[derive(Clone, Parser)]
#[command(version, author, about, name = "weathercli")]
pub struct App {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Clone, Subcommand)]
pub enum Command {
    #[command(about = "Build docs")]
    Docs,
    #[command(about = "Bump docs executable")]
    UpdateDocs(UpdateDocsOpts),
    #[command(about = "Update index hashes")]
    IndexHashes,
}

#[derive(Clone, Args)]
pub struct UpdateDocsOpts {
    github_api_token: String,
}

fn build_docs() -> weather_core::Result<()> {
    fs::create_dir_all(Path::new("./docs"))?;
    fs::OpenOptions::new().create(true).open(Path::new("./docs/index.html"))?;
    fs::OpenOptions::new().create(true).open(Path::new("./docs/config.html"))?;
    let mut jc = Path::new("./jc");
    if cfg!(windows) {
        jc = Path::new("./jc.exe");
    }
    let mut p1 = process::Command::new(jc.display().to_string()).arg("index.html").arg("./docs/index.html").arg("--template-dir").arg("./docs_templates").spawn().expect("spawn failed");
    p1.wait().expect("waiting failed");
    let mut p2 = process::Command::new(jc.display().to_string()).arg("config.html").arg("./docs/config.html").arg("--template-dir").arg("./docs_templates").spawn().expect("spawn failed");
    p2.wait().expect("waiting failed");
    fs::copy("./docs_templates/index.json", "./docs/index.json")?;
    fs::copy("./docs_templates/hero.png", "./docs/hero.png")?;
    fs::copy("./docs_templates/logo.png", "./docs/logo.png")?;
    fs::copy("./docs_templates/weather.exe", "./docs/weather.exe")?;
    fs::copy("./docs_templates/weather", "./docs/weather")?;
    fs::copy("./docs_templates/updater.exe", "./docs/updater.exe")?;
    fs::copy("./docs_templates/updater", "./docs/updater")?;
    fs::copy("./docs_templates/weatherd.exe", "./docs/weatherd.exe")?;
    fs::copy("./docs_templates/weatherd", "./docs/weatherd")?;
    fs::copy("./docs_templates/theme.js", "./docs/theme.js")?;
    println!("Done!");
    Ok(())
}

fn index_hashes() -> weather_core::Result<()> {
    update_hash("./docs_templates/weather_codes.json", "weather-codes-hash")?;
    update_hash("./docs_templates/weather_ascii_images.json", "weather-ascii-images-hash")?;
    update_hash("./docs_templates/default_layout.json", "default-layout-hash")?;
    update_hash("./docs_templates/weather.exe", "weather-exe-hash-windows")?;
    update_hash("./docs_templates/weather", "weather-exe-hash-unix")?;
    update_hash("./docs_templates/updater.exe", "updater-exe-hash-windows")?;
    update_hash("./docs_templates/updater", "updater-exe-hash-unix")?;
    update_hash("./docs_templates/weatherd.exe", "weatherd-exe-hash-windows")?;
    update_hash("./docs_templates/weatherd", "weatherd-exe-hash-unix")?;
    Ok(())
}


fn main() -> weather_core::Result<()> {
    let args = App::parse();
    match args.command {
        Command::Docs => build_docs(),
        Command::UpdateDocs(opts) => update_docs::update_docs(&*opts.github_api_token),
        Command::IndexHashes => index_hashes()
    }
}