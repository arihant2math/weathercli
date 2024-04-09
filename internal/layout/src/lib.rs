use local::weather_file::WeatherFile;
use log::debug;
use serde::{Deserialize, Serialize};
use shared_deps::bincode;
use shared_deps::serde_json;
use tera::{Context, Tera};
use terminal::color;

use crate::layout_input::LayoutInput;
use crate::layout_serde::LayoutDefaultsSerde;
use crate::row::Row;
use crate::tera_functions::{Color, Units, TerminalInfo};

pub mod error;
mod image_to_text;
pub mod item;
pub mod layout_input;
pub mod layout_serde;
mod row;
mod tera_functions;
pub mod util;

pub use crate::error::Error;
pub use crate::error::LayoutErr;

pub type Result<T> = std::result::Result<T, error::Error>;

const TEMPLATE_PREFIX: &str = r#"{% set BOLD = color(color="BOLD") %}
{% set ITALIC = color(color="ITALIC") %}
{% set UNDERLINE = color(color="UNDERLINE") %}
{% set BLINK = color(color="BLINK") %}
{% set STRIKETHROUGH = color(color="STRIKETHROUGH") %}
{% set RESET = color(color="RESET") %}
{% set FORE_RESET = color(color="FORE_RESET") %}
{% set BACK_RESET = color(color="BACK_RESET") %}
{% set FORE_BLACK = color(color="FORE_BLACK") %}
{% set FORE_RED = color(color="FORE_RED") %}
{% set FORE_GREEN = color(color="FORE_GREEN") %}
{% set FORE_YELLOW = color(color="FORE_YELLOW") %}
{% set FORE_BLUE = color(color="FORE_BLUE") %}
{% set FORE_MAGENTA = color(color="FORE_MAGENTA") %}
{% set FORE_CYAN = color(color="FORE_CYAN") %}
{% set FORE_WHITE = color(color="FORE_WHITE") %}
{% set FORE_LIGHTBLACK = color(color="FORE_LIGHTBLACK") %}
{% set FORE_LIGHTRED = color(color="FORE_LIGHTRED") %}
{% set FORE_LIGHTGREEN = color(color="FORE_LIGHTGREEN") %}
{% set FORE_LIGHTYELLOW = color(color="FORE_LIGHTYELLOW") %}
{% set FORE_LIGHTBLUE = color(color="FORE_LIGHTBLUE") %}
{% set FORE_LIGHTMAGENTA = color(color="FORE_LIGHTMAGENTA") %}
{% set FORE_LIGHTCYAN = color(color="FORE_LIGHTCYAN") %}
{% set FORE_LIGHTWHITE = color(color="FORE_LIGHTWHITE") %}
{% set BACK_BLACK = color(color="BACK_BLACK") %}
{% set BACK_RED = color(color="BACK_RED") %}
{% set BACK_GREEN = color(color="BACK_GREEN") %}
{% set BACK_YELLOW = color(color="BACK_YELLOW") %}
{% set BACK_BLUE = color(color="BACK_BLUE") %}
{% set BACK_MAGENTA = color(color="BACK_MAGENTA") %}
{% set BACK_CYAN = color(color="BACK_CYAN") %}
{% set BACK_WHITE = color(color="BACK_WHITE") %}
{% set BACK_LIGHTBLACK = color(color="BACK_LIGHTBLACK") %}
{% set BACK_LIGHTRED = color(color="BACK_LIGHTRED") %}
{% set BACK_LIGHTGREEN = color(color="BACK_LIGHTGREEN") %}
{% set BACK_LIGHTYELLOW = color(color="BACK_LIGHTYELLOW") %}
{% set BACK_LIGHTBLUE = color(color="BACK_LIGHTBLUE") %}
{% set BACK_LIGHTMAGENTA = color(color="BACK_LIGHTMAGENTA") %}
{% set BACK_LIGHTCYAN = color(color="BACK_LIGHTCYAN") %}
{% set BACK_LIGHTWHITE = color(color="BACK_LIGHTWHITE") %}
{{ RESET }}"#;

pub const VERSION: u64 = 22;

#[derive(Clone, Serialize, Deserialize)]
pub struct LayoutSettings {
    variable_color: String,
    text_color: String,
    unit_color: String,
    variable_bg_color: String,
    text_bg_color: String,
    unit_bg_color: String,
}

pub enum LayoutFileFormat {
    Bincode(Vec<Row>),
    Template(String),
}

pub struct LayoutFile {
    pub layout: LayoutFileFormat,
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
    let unit_bg_color =
        color::from_string(&retrieved_settings.clone().unit_bg_color).unwrap_or_default();
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

    pub fn from_template_path(file: WeatherFile) -> crate::Result<Self> {
        let mut text = file.get_text()?;
        text = TEMPLATE_PREFIX.to_string().replace("\n", "") + &text;
        Ok(Self {
            layout: LayoutFileFormat::Template(text),
            version: VERSION,
            settings: get_layout_settings(LayoutDefaultsSerde {
                // TODO: Make this configurable
                variable_color: "FORE_LIGHTGREEN".to_string(),
                text_color: "FORE_LIGHTBLUE".to_string(),
                unit_color: "FORE_MAGENTA".to_string(),
                variable_bg_color: "BACK_RESET".to_string(),
                text_bg_color: "BACK_RESET".to_string(),
                unit_bg_color: "BACK_RESET".to_string(),
            }),
        })
    }

    pub fn from_bincode_path(file: WeatherFile) -> crate::Result<Self> {
        let mut d = file.data;
        let magic_bytes = d[0..7].to_vec();
        if magic_bytes != [0x6C, 0x61, 0x79, 0x6F, 0x75, 0x74, 0x0A] {
            return Err("Layout file does not have the correct magic bytes".to_string())?;
        }
        d = d[7..].to_vec();
        let version =
            ((d[0] as u64) << 24) + ((d[1] as u64) << 16) + ((d[2] as u64) << 8) + d[3] as u64;
        d = d[4..].to_vec();
        return Self::from_serde(bincode::deserialize(&d)?, version);
    }

    pub fn from_path(path: &str) -> crate::Result<Self> {
        debug!("Loading layout from {}", path);
        let file = WeatherFile::new(path)?;
        let ext = file
            .path
            .extension()
            .unwrap_or_else(|| "layout".as_ref())
            .to_str()
            .unwrap();
        if ext == "layout" {
            return Self::from_template_path(file);
        } else if ext == "res" {
            return Self::from_bincode_path(file);
        } else {
            return Err("Layout file does not have an extension of .res or .layout".to_string())?;
        }
    }

    fn from_serde(file_data: layout_serde::LayoutSerde, version: u64) -> crate::Result<Self> {
        check_version(version)?;
        let layout = file_data.layout;
        let mut internal_layout: Vec<Row> = Vec::new();
        for row in layout {
            internal_layout.push(Row::new(row)?);
        }
        Ok(Self {
            layout: LayoutFileFormat::Bincode(internal_layout),
            version,
            settings: get_layout_settings(file_data.defaults),
        })
    }

    pub fn to_string(&self, data: LayoutInput, metric: bool) -> crate::Result<String> {
        match &self.layout {
            LayoutFileFormat::Bincode(rows) => {
                let mut s = Vec::new();
                let data_value = serde_json::to_value(data)?;
                for (count, row) in rows.iter().enumerate() {
                    s.push(
                        row.to_string(&data_value, self.settings.clone(), metric)
                            .map_err(|e| reemit_layout_error(e, count))?,
                    );
                }
                Ok(s.join("\n"))
            }
            LayoutFileFormat::Template(s) => {
                let mut tera = Tera::default();
                tera.register_function("color", Color::new());
                tera.register_function("units", Units::new());
                tera.register_function("terminal_info", TerminalInfo::new());
                tera.add_raw_template("macros", "").unwrap();
                tera.add_raw_template("layout", s).unwrap(); // TODO: no unwrap
                let mut context = Context::new();
                context.insert("data", &data);
                context.insert("settings", &self.settings);
                context.insert("metric", &metric);
                Ok(tera.render("layout", &context).unwrap())
            }
        }
    }
}
