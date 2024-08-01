use self_replace;
use std::fs::File;
use std::io::Write;
use std::thread;
use std::time::Duration;

use log::{debug, trace};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum UpdateError {
    // TODO: Merge with the one in resource
    #[error("Network error: {0}")]
    NetworkError(#[from] networking::Error),
    #[error("Reqwuest error: {0}")]
    ReqwuestError(String), // TODO: Store actual error
    #[error("JSON Error: {0}")]
    JSONError(#[from] shared_deps::simd_json::Error),
    #[error("I/O Error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Weather file Error: {0}")]
    WeatherFileError(#[from] local::weather_file::Error),
    #[error("Server Error: {0}")]
    ServerError(String),
}

pub fn update(url: &str, path: &str, quiet: bool) -> Result<(), UpdateError> {
    let replace = std::env::current_exe()?.display().to_string() == path;
    let download_path = if replace {
        path.to_string() + ".tmp"
    } else {
        path.to_string()
    };
    debug!("Downloading to {download_path} from {url}");
    let res = reqwest::blocking::get(url)
        .map_err(|_| UpdateError::ServerError(format!("Failed to download file from {url}")))?;
    let status = res.status().as_u16();
    trace!("Status code: {status}");
    assert_eq!(
        status, 200,
        "Server returned a status code of {status} instead of 200,\
    the update was aborted because downloading this file would damage the installation,\
    this is likely a bug.\nURL: {url}"
    ); // Prevent a 404 page from blanking someone's exe
    let mut retries = 0;
    let mut file_expect = File::create(&download_path);
    while file_expect.is_err() {
        if retries > 30 {
            return Err(UpdateError::ServerError(format!(
                "Failed to create/open file '{}'",
                &download_path
            ))); // TODO: not a server error (use own enum type)
        }
        file_expect = File::create(&download_path);
        retries += 1;
        thread::sleep(Duration::from_millis(100));
    }
    let mut file = file_expect?;
    file.write_all(
        &res.bytes()
            .map_err(|_| UpdateError::ServerError("Cannot get bytes".to_string()))?,
    )?;
    if replace {
        if !quiet {
            println!("Replacing {path}");
        }
        debug!("Replacing {} with {}", path, download_path);
        self_replace::self_replace(&download_path)?;
        std::fs::remove_file(&download_path)?;
    }
    Ok(())
}
