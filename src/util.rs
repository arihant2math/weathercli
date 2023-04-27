use std::fs;
use std::path::Path;

use sha2::Digest;

/// returns the sha-256 of the file
pub fn hash_file(filename: &str) -> crate::Result<String> {
    let input = Path::new(filename);
    let bytes = fs::read(input)?;
    Ok(hex::encode(sha2::Sha256::digest(bytes)))
}

pub struct Config<'a> {
    pub weather_file_name: &'a str,
    pub weather_dfile_name: &'a str,
    pub updater_file_name: &'a str,
}
