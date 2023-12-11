use log::debug;
use local::weather_file::WeatherFile;
use shared_deps::bincode;
use shared_deps::serde_json;
use terminal::color;
use weather_error::{Error, LayoutErr};

use crate::layout_input::LayoutInput;
use crate::layout_serde::LayoutDefaultsSerde;
use crate::row::Row;

pub type Result<T> = std::result::Result<T, weather_error::Error>;

mod image_to_text;
pub mod item;
pub mod layout_serde;
mod row;
pub mod util;
pub mod layout_input;

pub const VERSION: u64 = 22;

#[derive(Clone)]
pub struct LayoutSettings {
    variable_color: String,
    text_color: String,
    unit_color: String,
    variable_bg_color: String,
    text_bg_color: String,
    unit_bg_color: String,
}

pub struct LayoutFile {
    pub layout: Vec<Row>,
    pub version: u64,
    pub settings: LayoutSettings,
}

fn reemit_layout_error(e: Error, count: usize) -> Error {
    match e {
        Error::LayoutError(e) => Error::LayoutError(LayoutErr {
            message: e.message,
            row: Some(count as u64),
            item: e.item,
        }),
        _ => e,
    }
}

fn check_version(version: u64) -> crate::Result<()> {
    if version > VERSION {
        return Err(Error::LayoutError(LayoutErr {
            message: format!("Version of layout file, {version}, is greater than the highest supported version {VERSION}"),
            row: None,
            item: None,
        }));
    } else if version <= 10 {
        return Err(Error::LayoutError(LayoutErr {
            message: "Layout Version too old (version 10 or earlier is not supported), defaulting"
                .to_string(),
            row: None,
            item: None,
        }));
    }
    Ok(())
}

fn get_layout_settings(data: LayoutDefaultsSerde) -> LayoutSettings {
    let retrieved_settings = data;
    let variable_color =
        color::from_string(&retrieved_settings.clone().variable_color).unwrap_or_default();
    let text_color = color::from_string(&retrieved_settings.clone().text_color).unwrap_or_default();
    let unit_color = color::from_string(&retrieved_settings.clone().unit_color).unwrap_or_default();
    let variable_bg_color =
        color::from_string(&retrieved_settings.clone().variable_bg_color).unwrap_or_default();
    let text_bg_color =
        color::from_string(&retrieved_settings.clone().text_bg_color).unwrap_or_default();
    let unit_bg_color = color::from_string(&retrieved_settings.clone().unit_bg_color).unwrap_or_default();
    LayoutSettings {
        variable_color,
        text_color,
        unit_color,
        variable_bg_color,
        text_bg_color,
        unit_bg_color,
    }
}

impl LayoutFile {
    pub fn new(path: &str) -> crate::Result<Self> {
        LayoutFile::from_path(&format!("layouts/{path}"))
    }

    pub fn from_path(path: &str) -> crate::Result<Self> {
        debug!("Loading layout from {}", path);
        let file = WeatherFile::new(path)?;
        let ext = file
            .path
            .extension()
            .unwrap_or_else(|| "res".as_ref())
            .to_str()
            .unwrap();
        if ext != "res" {
            return Err("Layout file does not have an extension of .res")?;
        }
        let mut d = file.data;
        let magic_bytes = d[0..7].to_vec();
        if magic_bytes != [0x6C, 0x61, 0x79, 0x6F, 0x75, 0x74, 0x0A] {
            return Err("Layout file does not have the correct magic bytes")?;
        }
        d = d[7..].to_vec();
        let version = ((d[0] as u64) << 24) + ((d[1] as u64) << 16) + ((d[2] as u64) << 8) + d[3] as u64;
        d = d[4..].to_vec();
        return Self::from_serde(bincode::deserialize(&d)?, version);
    }

    fn from_serde(file_data: layout_serde::LayoutSerde, version: u64) -> crate::Result<Self> {
        check_version(version)?;
        let layout = file_data.layout;
        let mut internal_layout: Vec<Row> = Vec::new();
        for row in layout {
            internal_layout.push(Row::new(row)?);
        }
        Ok(Self {
            layout: internal_layout,
            version,
            settings: get_layout_settings(file_data.defaults),
        })
    }

    pub fn to_string(&self, data: LayoutInput, metric: bool) -> crate::Result<String> {
        let mut s = Vec::new();
        let data_value = serde_json::to_value(data)?;
        for (count, row) in self.layout.iter().enumerate() {
            s.push(
                row.to_string(&data_value, self.settings.clone(), metric)
                    .map_err(|e| reemit_layout_error(e, count))?,
            );
        }
        Ok(s.join("\n"))
    }
}
