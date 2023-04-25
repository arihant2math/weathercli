use std::cmp::min;
use std::fs::File;
use std::io::Write;
use std::thread;
use std::time::Duration;

use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use indicatif::style::TemplateError;
use reqwest::Client;
use crate::util;

impl From<TemplateError> for util::Error {
    fn from(error: TemplateError) -> Self {
        util::Error::Other(error.to_string())
    }
}

pub async fn update_component(
    url: &str,
    path: &str,
    progress_msg: String,
    finish_msg: String,
    quiet: bool,
) -> crate::Result<()> {
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
    let status = res.status().as_u16();
    assert_eq!(
        status, 200,
        "Server returned a status code of {} instead of 200,\
    the update was aborted because downloading this file would damage the installation,\
    this is likely a bug.\nURL: {}",
        status, url
    ); // Prevent a 404 page from blanking someone's exe
    let retries = 0;
    let mut file_expect = File::create(path);
    while file_expect.is_err() {
        if retries > 30 {
            return Err(util::Error::IoError(format!("Failed to create/open file '{}'", path)));
        }
        file_expect = File::create(path);
        thread::sleep(Duration::from_millis(100));
    }
    let mut file = file_expect?;
    let mut progress_bar = ProgressBar::hidden();
    if !quiet {
        progress_bar = ProgressBar::new(total_size);
        progress_bar.set_style(ProgressStyle::default_bar()
            .template("{msg}\n[{elapsed}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")?
            .progress_chars("━ "));
        progress_bar.set_message(progress_msg + url);
    }
    // download chunks
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.or(Err("Error while downloading file".to_string()))?;
        file.write_all(&chunk)
            .map_err(|_| "Error while writing to file".to_string())?;
        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
        if !quiet {
            progress_bar.set_position(new);
        }
    }
    if !quiet {
        progress_bar.set_style(ProgressStyle::default_bar()
            .template("{msg}\n[{elapsed}] [{wide_bar:.green}] {bytes}/{total_bytes} ({bytes_per_sec})")?
            .progress_chars("━ "));
        progress_bar.finish_with_message(finish_msg);
    }
    Ok(())
}
