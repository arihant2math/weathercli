use std::fs;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::PathBuf;

use local::dirs::home_dir;

use crate::local;

#[derive(Clone)]
pub struct WeatherFile {
    pub path: PathBuf,
    pub data: String,
    pub exists: bool,
}

impl WeatherFile {
    pub fn new(file_name: &str) -> crate::Result<Self> {
        let mut path = home_dir().expect("expect home dir");
        let mut exists = true;
        path.push(".weathercli");
        path.push(file_name);
        if !path.exists() {
            exists = false;
            let parent_dir = path.parent().ok_or_else(|| "Parent dir not found")?;
            fs::create_dir_all(parent_dir)?;
            let mut file = File::create(path.display().to_string())?;
            file.write_all(b"{}")?;
        }
        let file = File::open(path.display().to_string())?;
        let mut buf_reader = BufReader::new(file);
        let mut data = String::new();
        buf_reader.read_to_string(&mut data)?;
        Ok(WeatherFile { path, data, exists })
    }

    /// Writes self.data to the file at self.path
    pub fn write(&self) -> crate::Result<()> {
        let f = File::options()
            .write(true)
            .truncate(true)
            .open(self.path.display().to_string())?;
        let mut f = BufWriter::new(f);
        f.write_all(self.data.as_bytes())?;
        f.flush()?;
        Ok(())
    }

    pub fn get_path(&self) -> String {
        self.path.display().to_string()
    }

    pub fn weather_codes() -> crate::Result<Self> {
        WeatherFile::new("resources/weather_codes.json")
    }

    pub fn settings() -> crate::Result<Self> {
        WeatherFile::new("settings.json")
    }

    pub fn weather_ascii_art() -> crate::Result<Self> {
        WeatherFile::new("resources/weather_ascii_images.json")
    }
}
