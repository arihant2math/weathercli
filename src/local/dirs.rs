use std::path::PathBuf;

#[cfg(windows)]
pub fn home_dir() -> crate::Result<PathBuf> {
    Ok(dirs_sys::known_folder_profile().ok_or("Home dir not found")?)
}

#[cfg(not(windows))]
pub fn home_dir() -> crate::Result<PathBuf> {
    Ok(dirs_sys::home_dir().ok_or("Home dir not found")?)
}
