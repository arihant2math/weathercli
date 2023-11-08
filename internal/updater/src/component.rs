use self_replace;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

use weather_error::Error;

pub fn update_component(
    url: &str,
    path: &str,
    progress_msg: String,
    finish_msg: String,
    quiet: bool,
) -> crate::Result<()> {
    let replace = std::env::current_exe()? == PathBuf::from(path);
    let download_path = if replace {
        path.to_string() + ".tmp"
    } else {
        path.to_string()
    };
    let res = networking::get_url(url, None, None, None)?;
    let status = res.status;
    assert_eq!(
        status, 200,
        "Server returned a status code of {} instead of 200,\
    the update was aborted because downloading this file would damage the installation,\
    this is likely a bug.\nURL: {}",
        status, url
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
    file.write_all(&res.bytes)?;
    if replace {
        if !quiet {
            println!("Replacing ...");
        }
        self_replace::self_replace(&download_path)?;
        std::fs::remove_file(&download_path)?;
    }
    Ok(())
}
