use log::trace;
use std::fs;
use std::fs::File;
use std::io;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::PathBuf;

use thiserror::Error;

use weather_dirs::weathercli_dir;

#[derive(Clone)]
pub struct WeatherFile {
    pub path: PathBuf,
    pub data: Vec<u8>,
    pub exists: bool,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("I/O Error: {0}")]
    IoError(#[from] io::Error),
    #[error("Weather Dir Error: {0}")]
    WeatherDirError(#[from] weather_dirs::Error),
    #[error("Failed to convert bytes to string")]
    UTF8ConversionFailed,
    #[error("Parent Dir not found")]
    ParentDirNotFound,
}

type Result<T> = std::result::Result<T, Error>;

impl WeatherFile {
    pub fn new<S: AsRef<str>>(file_name: S) -> Result<Self> {
        let path = weathercli_dir()?.join(file_name.as_ref());
        trace!("Opening {}", path.display());
        let exists = path.exists();
        if !exists {
            let parent_dir = path.parent().ok_or(Error::ParentDirNotFound)?;
            fs::create_dir_all(parent_dir)?;
            let mut file = File::create(path.display().to_string())?;
            if path.extension().unwrap_or_else(|| "".as_ref()) == "json" {
                file.write_all(b"{}")?;
            } else {
                file.write_all(b"")?;
            }
        }
        let file = File::open(path.display().to_string())?;
        let mut buf_reader = BufReader::new(file);
        let mut data = Vec::new();
        buf_reader.read_to_end(&mut data)?;
        Ok(Self { path, data, exists })
    }

    /// Writes self.data to the file at self.path
    pub fn write(&self) -> Result<()> {
        trace!("Writing to {}", self.path.display());
        let f = File::options()
            .write(true)
            .truncate(true)
            .open(self.path.display().to_string())?;
        let mut f = BufWriter::new(f);
        f.write_all(&self.data)?;
        f.flush()?;
        Ok(())
    }

    pub fn get_text(&self) -> Result<String> {
        Ok(String::from_utf8(self.data.clone()).map_err(|_e| Error::UTF8ConversionFailed)?)
    }

    pub fn settings() -> Result<Self> {
        Self::new("settings.json")
    }

    pub fn weather_codes() -> Result<Self> {
        Self::new("resources/weather_codes.res")
    }

    pub fn weather_ascii_art() -> Result<Self> {
        Self::new("resources/weather_ascii_images.res")
    }
}
