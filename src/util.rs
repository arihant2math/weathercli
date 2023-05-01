use std::fs;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};

use sha2::Digest;

/// returns the sha-256 of the file
pub fn hash_file(filename: &str) -> crate::Result<String> {
    let input = Path::new(filename);
    let bytes = fs::read(input)?;
    Ok(hex::encode(sha2::Sha256::digest(bytes)))
}

pub fn list_dir(dir: PathBuf) -> crate::Result<Vec<String>> { // TODO: clean up when i'm not insane trying to deal with rust's type system
    let mut paths_dir: Vec<DirEntry> = Vec::new();
    for path in fs::read_dir(dir)? {
        paths_dir.push(path?);
    }
    let mut paths: Vec<String> = Vec::new();
    for path in paths_dir {
        let tmp = path.file_name().into_string().unwrap();
        paths.push(tmp);
    }
    Ok(paths)
}

pub struct Config<'a> {
    pub weather_file_name: &'a str,
    pub weather_dfile_name: &'a str,
    pub updater_file_name: &'a str,
}
