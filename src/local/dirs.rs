use std::path::PathBuf;

#[cfg(windows)]
pub fn weathercli_dir() -> crate::Result<PathBuf> {
    Ok(dirs_sys::known_folder_profile().ok_or("Home dir not found")?.join(".weathercli"))
}

#[cfg(not(windows))]
pub fn weathercli_dir() -> crate::Result<PathBuf> {
    Ok(dirs_sys::home_dir().ok_or("Home dir not found")?.join(".weathercli"))
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
