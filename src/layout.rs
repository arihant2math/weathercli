use crate::backend::weather_forecast::WeatherForecastRS;
use crate::color;
use crate::layout::layout_row::Row;
use crate::layout::layout_json::{LayoutJSON, LayoutDefaultsJSON};
use crate::layout::layout_json::RowEnum::{RowString, RowVec};
use crate::local::weather_file::WeatherFile;
use crate::error::{Error, LayoutErr};

mod image_to_text;
mod layout_item;
mod layout_row;
mod layout_json;
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

pub struct LayoutFile {
    layout: Vec<Row>,
    variable_color: String,
    text_color: String,
    unit_color: String,
    variable_bg_color: String,
    text_bg_color: String,
    unit_bg_color: String,
}

fn reemit_layout_error(e: Error, count: usize) -> Error {
    match e {
        Error::LayoutError(e ) => Error::LayoutError(LayoutErr {
            message: e.message,
            row: Some(count as u64),
            item: e.item
        }),
        _ => e
    }
}

impl LayoutFile {
    pub fn new(path: String) -> crate::Result<Self> { // Error here means default unless its default.json
        let file = WeatherFile::new(&("layouts/".to_string() + &path))?;
        let file_data: LayoutJSON =
            serde_json::from_str(&file.data)?;
        if file_data.version.is_none() {
            return Err(Error::LayoutError(LayoutErr {
                message: "Invalid Layout, missing key 'version'".to_string(),
                row: None,
                item: None,
            }));
            // trace!("Invalid Layout, missing Key 'version', add it like this {\n\t... // Your json here\n\t"version": ' + str(self.version) + "\n}"")
        } else if file_data.version.expect("version not found") > VERSION {
            return Err(Error::LayoutError(LayoutErr {
                message: format!("Version of layout file, {}, is greater than the highest supported version {}",
                file_data.version.unwrap_or(0), VERSION),
                row: None,
                item: None,
            }));
        } else if file_data.version.expect("version not found") < 1 {
            return Err(Error::LayoutError(LayoutErr {
                message: "Layout Version too old (version 0 is not supported), defaulting".to_string(),
                row: None,
                item: None,
            }));
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
            return Err(LayoutErr {message: "Layout key not found".to_string(), row: None, item: None})?;
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
        let data_value = serde_json::to_value(data)?;
        for (count, row) in self.layout.iter().enumerate() {
            s.push(row.to_string(
                &data_value,
                self.variable_color.clone(),
                self.text_color.clone(),
                self.unit_color.clone(),
                self.variable_bg_color.clone(),
                self.text_bg_color.clone(),
                self.unit_bg_color.clone(),
                metric,
            ).map_err(|e| reemit_layout_error(e, count))?)
        }
        Ok(s.join("\n"))
    }
}
