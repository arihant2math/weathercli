use std::fs;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::PathBuf;

use crate::local::dirs::weathercli_dir;

#[derive(Clone)]
pub struct WeatherFile {
    pub path: PathBuf,
    pub data: Vec<u8>,
    pub exists: bool,
}

impl WeatherFile {
    pub fn new(file_name: &str) -> crate::Result<Self> {
        let mut path = weathercli_dir().expect("expect home dir");
        let mut exists = true;
        path.push(file_name);
        if !path.exists() {
            exists = false;
            let parent_dir = path.parent().ok_or("Parent dir not found")?;
            fs::create_dir_all(parent_dir)?;
            let mut file = File::create(path.display().to_string())?;
            file.write_all(b"{}")?;
        }
        let file = File::open(path.display().to_string())?;
        let mut buf_reader = BufReader::new(file);
        let mut data = Vec::new();
        buf_reader.read_to_end(&mut data)?;
        Ok(Self { path, data, exists })
    }

    /// Writes self.data to the file at self.path
    pub fn write(&self) -> crate::Result<()> {
        let f = File::options()
            .write(true)
            .truncate(true)
            .open(self.path.display().to_string())?;
        let mut f = BufWriter::new(f);
        f.write_all(&*self.data)?;
        f.flush()?;
        Ok(())
    }

    pub fn get_text(&self) -> crate::Result<String> {
        return Ok(String::from_utf8(self.data.clone()).unwrap()); // TODO: Fix
    }

    pub fn settings() -> crate::Result<Self> {
        Self::new("settings.json")
    }

    pub fn weather_codes() -> crate::Result<Self> {
        Self::new("resources/weather_codes.res")
    }

    pub fn weather_ascii_art() -> crate::Result<Self> {
        Self::new("resources/weather_ascii_images.res")
    }
}
