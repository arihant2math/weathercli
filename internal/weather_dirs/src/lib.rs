use std::path::PathBuf;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Copy, Clone, Debug, Error)]
pub enum Error {
    #[error("Home Directory not found")]
    HomeDirectoryNotFound,
}

pub fn weathercli_dir() -> crate::Result<PathBuf> {
    Ok(home::home_dir()
        .ok_or(Error::HomeDirectoryNotFound)?
        .join(".weathercli"))
}

pub fn cache_dir() -> crate::Result<PathBuf> {
    Ok(weathercli_dir()?.join("cache"))
}

pub fn layouts_dir() -> crate::Result<PathBuf> {
    Ok(weathercli_dir()?.join("layouts"))
}

pub fn custom_backends_dir() -> crate::Result<PathBuf> {
    Ok(weathercli_dir()?.join("custom_backends"))
}

pub fn resources_dir() -> crate::Result<PathBuf> {
    Ok(weathercli_dir()?.join("resources"))
}
