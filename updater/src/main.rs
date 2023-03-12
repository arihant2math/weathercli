use std::cmp::min;
use std::fs::File;
use std::io::Write;
use std::thread;
use std::time::Duration;

use clap::Parser;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;

#[derive(Clone, Parser)]
struct Cli {
   #[arg(long, short, default_value_t = String::from("all"))]
    component: String,
   #[clap(long, short, action)]
    quiet: bool,
   #[clap(long, short, action)]
    version: bool
}

async fn update_component(url: &str, path: &str, progress_msg: String, finish_msg: String) -> Result<(), String> {
    let client = Client::new();
    // Reqwest setup
    let res = client
        .get(url)
        .send()
        .await
        .or(Err(format!("Failed to GET from '{}'", url)))?;
    let total_size = res
        .content_length()
        .ok_or(format!("Failed to get content length from '{}'", &url))?;


    let mut file_expect = File::create(path);
    let retries = 0;
    while file_expect.is_err() {
        if retries > 3 {
            return Err(format!("Failed to create/open file '{}'", path));
        }
        file_expect = File::create(path);
        thread::sleep(Duration::from_millis(1000));
    }
    let mut file = file_expect.unwrap();

    let progress_bar = ProgressBar::new(total_size);
    progress_bar.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
        .expect("Failed due to progress bar error")
        .progress_chars("—> "));
    progress_bar.set_message(progress_msg + &*url);

    // download chunks
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.or(Err("Error while downloading file".to_string()))?;
        file.write_all(&chunk).map_err(|_| "Error while writing to file".to_string())?;
        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
        progress_bar.set_position(new);
    }

    progress_bar.finish_with_message(finish_msg);
    Ok(())
}


#[tokio::main]
async fn main() -> Result<(), String> {
    let args = Cli::parse();
    if args.version {
        println!("3.11.2023");
        return Ok(());
    }
    let mut to_update: Vec<String> = Vec::new();
    if args.component == "all" {
        to_update.push("main".to_string());
        to_update.push("daemon".to_string());
    }
    if args.component == "daemon" {
        to_update.push("daemon".to_string());
    }
    if args.component == "main" {
        to_update.push("main".to_string());
    }
    if to_update.contains(&"main".to_string()) {
        let url;
        let path;
        if cfg!(windows) {
            url = "https://arihant2math.github.io/weathercli/docs/weather.exe";
            path = "weather.exe";
        }
        else if cfg!(unix) {
            url = "https://arihant2math.github.io/weathercli/docs/weather";
            path = "weather";
        }
        else {
            return Err("OS unsupported".to_string());
        }

        let r = update_component(url, path, "Downloading weathercli update from ".to_string(),
                         "Updated weathercli".to_string()).await;
        r?;
    }
    if to_update.contains(&String::from("daemon")) {
        let url;
        let path;
        if cfg!(windows) {
            url = "https://arihant2math.github.io/weathercli/docs/weatherd.exe";
            path = "daemon.exe";
        }
        else if cfg!(unix) {
            url = "https://arihant2math.github.io/weathercli/docs/weatherd";
            path = "daemon";
        }
        else {
            return Err("OS unsupported".to_string());
        }
        let r = update_component(url, path, "Downloading daemon update from ".to_string(),
                         "Updated daemon".to_string()).await;
        r?;
    }
    Ok(())
}
