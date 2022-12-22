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
        url = "https://arihant2math.github.io/weathercli/weather.exe";
        path = "weather.exe";
    } else if cfg!(unix) {
        url = "https://arihant2math.github.io/weathercli/weather";
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

    // Indicatif setup
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})").expect("Failed due to Indicatif progress bar")
        .progress_chars("#>-"));
    pb.set_message("Downloading ".to_string() + url);

    // download chunks
    let mut file = File::create(path).or(Err(format!("Failed to create file '{}'", path)))?;
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.or(Err(format!("Error while downloading file")))?;
        file.write_all(&chunk)
            .or(Err(format!("Error while writing to file")))?;
        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
        pb.set_position(new);
    }

    pb.finish_with_message("Downloaded ".to_string() + url + " to " + path);
    return Ok(());
}
