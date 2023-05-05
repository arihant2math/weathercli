use std::path::PathBuf;

pub fn weathercli_dir() -> crate::Result<PathBuf> {
    Ok(home::home_dir()
        .ok_or("Home dir not found")?
        .join(".weathercli"))
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
