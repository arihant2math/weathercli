use std::cmp::min;
use std::fs::File;
use std::io::Write;

use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), String> {
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
    let client = Client::new();
    println!("===== Updater =====");
    // Reqwest setup
    let res = client
        .get(url)
        .send()
        .await
        .or(Err(format!("Failed to GET from '{}'", &url)))?;
    let total_size = res
        .content_length()
        .ok_or(format!("Failed to get content length from '{}'", &url))?;

    let progress_bar = ProgressBar::new(total_size);
    progress_bar.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})").expect("Failed due to Indicatif progress bar")
        .progress_chars("#>-"));
    progress_bar.set_message("Downloading ".to_string() + url);

    // download chunks
    let mut file = File::create(path).or(Err(format!("Failed to create file '{}'", path)))?;
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.or(Err("Error while downloading file".to_string()))?;
        file.write_all(&chunk).map_err(|_| "Error while writing to file".to_string())?;
        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
        progress_bar.set_position(new);
    }

    progress_bar.finish_with_message("Downloaded ".to_string() + url + " to " + path);
    Ok(())
}
