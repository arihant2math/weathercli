use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use crate::color;
use crate::local::dirs::{custom_backends_dir, weathercli_dir};
use crate::local::settings::Settings;
use crate::util::list_dir;

pub fn install(path: String) -> crate::Result<()> { // TODO: Add validity checks
    let real_path = PathBuf::from_str(&*path).unwrap();
    let file_name = real_path.file_name().ok_or_else(|| "Not a file")?.to_str().unwrap();
    fs::copy(&real_path, weathercli_dir()?.join("custom_backends").join(file_name))?;
    Ok(())
}

pub fn list() -> crate::Result<()> {
    let paths = fs::read_dir(weathercli_dir()?.join("custom_backends"))?;
    let settings = Settings::new()?; // TODO: Optimize excess read
    for path in paths { // TODO: Check which ones are valid
        let tmp = path?.file_name();
        let file_name = tmp.to_str().unwrap();
        let valid = settings.internal.enable_custom_backends;
        if valid {
            println!("{}{file_name}", color::FORE_GREEN)
        }
        else {
            println!("{}{file_name}", color::FORE_RED)
        }
    }
    Ok(())
}

pub fn delete() -> crate::Result<()> {
    let paths = list_dir(custom_backends_dir()?)?;
    let choice = paths[crate::prompt::choice(&*paths, 0, None)?].to_string();
    fs::remove_file(custom_backends_dir()?.join(&*choice))?;
    Ok(())
}
