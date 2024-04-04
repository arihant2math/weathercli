pub mod cache;
pub(crate) mod json;
pub mod location;
pub mod settings;
pub mod weather_file;

use std::io;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use sha2::Digest;


/// returns the sha-256 of the file
pub fn hash_file(filename: &str) -> io::Result<String> {
    let input = Path::new(filename);
    let bytes = fs::read(input)?;
    Ok(hex::encode(sha2::Sha256::digest(bytes)))
}

pub fn now() -> u128 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).expect(
        "Time went backwards :( or there is an overflow error of some sort and stuff broke",
    );
    since_the_epoch.as_millis()
}

pub fn list_dir(dir: PathBuf) -> io::Result<Vec<String>> {
    Ok(fs::read_dir(dir)?
        .map(|f| f.unwrap().file_name().into_string().unwrap())
        .collect())
}
