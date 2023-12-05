use self_replace;
use std::fs::File;
use std::io::Write;
use std::thread;
use std::time::Duration;

use log::{debug, trace};

use weather_error::Error;

pub fn update(
    url: &str,
    path: &str,
    quiet: bool,
) -> crate::Result<()> {
    let replace = std::env::current_exe()?.display().to_string() == path;
    let download_path = if replace {
        path.to_string() + ".tmp"
    } else {
        path.to_string()
    };
    debug!("Downloading to {download_path} from {url}");
    let res = reqwest::blocking::get(url).map_err(|_| Error::NetworkError(format!(
            "Failed to download file from {url}"
        )))?;
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
            return Err(Error::IoError(format!(
                "Failed to create/open file '{}'",
                &download_path
            )));
        }
        file_expect = File::create(&download_path);
        retries += 1;
        thread::sleep(Duration::from_millis(100));
    }
    let mut file = file_expect?;
    file.write_all(&res.bytes().map_err(|_| Error::NetworkError("Cannot get bytes".to_string()))?)?;
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
