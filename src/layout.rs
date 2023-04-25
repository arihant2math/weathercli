use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::backend::weather_forecast::WeatherForecastRS;
use crate::color;
use crate::layout::layout_row::Row;
use crate::layout::RowEnum::{RowString, RowVec};
use crate::local::weather_file::WeatherFile;

mod image_to_text;
mod layout_item;
mod layout_row;
pub mod util;

pub const VERSION: u64 = 10;
pub const DEFAULT_LAYOUT_SETTINGS: LayoutDefaultsJSON = LayoutDefaultsJSON {
    variable_color: None,
    text_color: None,
    unit_color: None,
    variable_bg_color: None,
    text_bg_color: None,
    unit_bg_color: None,
};

#[derive(Clone, Serialize, Deserialize)]
pub struct ItemJSON {
    #[serde(rename = "type")]
    pub item_type: String,
    pub color: Option<String>,
    pub bg_color: Option<String>,
    pub metric: Option<String>,
    pub imperial: Option<String>,
    pub unit_color: Option<String>,
    pub value: String,
    pub args: Option<Vec<ItemEnum>>,
    pub kwargs: Option<HashMap<String, ItemEnum>>,
    pub scale: Option<f64>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ItemEnum {
    ItemString(String),
    ItemInt(i64),
    ItemFloat(f64),
    Item(ItemJSON),
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum RowEnum {
    RowString(String),
    RowVec(Vec<ItemEnum>),
}

pub struct LayoutFile {
    layout: Vec<Row>,
    variable_color: String,
    text_color: String,
    unit_color: String,
    variable_bg_color: String,
    text_bg_color: String,
    unit_bg_color: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LayoutDefaultsJSON {
    pub variable_color: Option<String>,
    pub text_color: Option<String>,
    pub unit_color: Option<String>,
    pub variable_bg_color: Option<String>,
    pub text_bg_color: Option<String>,
    pub unit_bg_color: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
struct LayoutJSON {
    pub version: Option<u64>,
    pub defaults: Option<LayoutDefaultsJSON>,
    pub layout: Option<Vec<RowEnum>>,
}

impl LayoutFile {
    pub fn new(path: String) -> crate::Result<Self> {
        let file = WeatherFile::new(&("layouts/".to_string() + &path))?;
        let file_data: LayoutJSON =
            serde_json::from_str(&file.data).expect("Invalid Layout, JSON parsing failed"); // TODO: Default instead
        if file_data.version.is_none() {
            panic!("Invalid Layout, missing key 'version'"); // TODO: Default instead
                                                             // println!("Invalid Layout, missing Key 'version', add it like this {\n\t... // Your json here\n\t"version": ' + str(self.version) + "\n}"")
        } else if file_data.version.expect("version not found") > VERSION {
            panic!(
                "Version of layout file, {}, is greater than the highest supported version {}",
                file_data.version.expect("version not found"),
                VERSION
            )
        } else if file_data.version.expect("version not found") < 1 {
            panic!("Layout Version too old (version 0 is not supported), defaulting");
        }
        let retrieved_settings = file_data
            .defaults
            .clone()
            .unwrap_or(DEFAULT_LAYOUT_SETTINGS);
        let variable_color = color::from_string(
            retrieved_settings
                .clone()
                .variable_bg_color
                .unwrap_or("FORE_LIGHTGREEN".to_string()),
        )
        .expect("Invalid color");
        let text_color = color::from_string(
            retrieved_settings
                .clone()
                .variable_bg_color
                .unwrap_or("FORE_LIGHTBLUE".to_string()),
        )
        .expect("Invalid color");
        let unit_color = color::from_string(
            retrieved_settings
                .clone()
                .variable_bg_color
                .unwrap_or("FORE_MAGENTA".to_string()),
        )
        .expect("Invalid color");
        let variable_bg_color = color::from_string(
            retrieved_settings
                .clone()
                .variable_bg_color
                .unwrap_or("".to_string()),
        )
        .unwrap_or("".to_string());
        let text_bg_color = color::from_string(
            retrieved_settings
                .clone()
                .variable_bg_color
                .unwrap_or("".to_string()),
        )
        .unwrap_or("".to_string());
        let unit_bg_color = color::from_string(
            retrieved_settings
                
                .variable_bg_color
                .unwrap_or("".to_string()),
        )
        .unwrap_or("".to_string());
        if file_data.layout.is_none() {
            panic!("Layout key not found"); // TODO: No panic
        }
        let layout = file_data.layout.unwrap();
        let mut _internal_layout: Vec<Row> = Vec::new();
        for (_count, row) in layout.iter().enumerate() {
            match row.clone() {
                RowString(payload) => _internal_layout.push(Row::from_str(&payload)),
                RowVec(payload) => _internal_layout.push(Row::from_vec(payload)),
            }
        }
        Ok(LayoutFile {
            layout: _internal_layout,
            variable_color,
            text_color,
            unit_color,
            variable_bg_color,
            text_bg_color,
            unit_bg_color,
        })
    }

    pub fn to_string(&self, data: WeatherForecastRS, metric: bool) -> crate::Result<String> {
        let mut s = Vec::new();
        let data_value = serde_json::to_value(data).expect("Serialization failed");
        for (_count, row) in self.layout.iter().enumerate() {
            s.push(row.to_string(
                &data_value,
                self.variable_color.clone(),
                self.text_color.clone(),
                self.unit_color.clone(),
                self.variable_bg_color.clone(),
                self.text_bg_color.clone(),
                self.unit_bg_color.clone(),
                metric,
            )?)
        }
        Ok(s.join("\n"))
    }
}
