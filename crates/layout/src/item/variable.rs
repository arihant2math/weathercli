use crate::LayoutErr;
use log::error;
use shared_deps::serde_json;
use shared_deps::serde_json::Value;
use terminal::color;

use crate::item::ItemType;
use crate::LayoutSettings;

fn round(f: f64) -> String {
    format!("{f:.1}")
}

pub struct Variable {
    pub color: Option<String>,
    pub bg_color: Option<String>,
    pub metric: Option<String>,
    pub imperial: Option<String>,
    pub unit_color: Option<String>,
    pub value: String,
}

impl ItemType for Variable {
    fn get_value(&self, data: &Value) -> crate::Result<String> {
        if data.is_null() {
            error!("data is null"); // TODO: More info
            return Ok(String::from("null"));
        }
        let mut split: Vec<&str> = self.value.split('.').collect();
        let mut current = data;
        while !split.is_empty() {
            if split[0]
                .chars()
                .next()
                .expect("0th element expected don't place two dots in a row, like: \"..\"")
                == '['
            {
                // list item
                let place = split[0][1..split.clone()[0].len() - 1]
                    .parse::<usize>()
                    .unwrap();
                current = &current[place];
            } else {
                // normal variable
                if current.is_null() {
                    error!("Variable not found in data");
                    return Ok("null".to_string());
                }
                current = &current[split[0]];
            }
            split.remove(0);
        }
        if let Some(c) = current.as_str() {
            return Ok(c.to_string());
        } else if let Some(c) = current.as_f64() {
            return Ok(round(c));
        } else if let Some(c) = current.as_i64() {
            return Ok(c.to_string());
        } else if let Some(c) = current.as_bool() {
            return Ok(c.to_string());
        } else if let Some(_) = current.as_array() {
            return Ok(serde_json::to_string_pretty(current).unwrap()); // TODO: Remove unwrap
        } else if current.is_null() {
            return Ok("null".to_string());
        }
        return Err(crate::Error::LayoutError(LayoutErr {
            message: format!("Variable {} has an unsupported type.", current.to_string()),
            row: None,
            item: None,
        }));
    }
    fn to_string(
        &self,
        data: &Value,
        settings: LayoutSettings,
        metric: bool,
    ) -> crate::Result<String> {
        let variable_color = settings.variable_color;
        let variable_bg_color = settings.variable_bg_color;
        let unit_color = settings.unit_color;
        let unit_bg_color = settings.unit_bg_color;
        let item_color =
            color::from_string(&self.color.clone().unwrap_or_default()).unwrap_or_default();
        let item_bg_color =
            color::from_string(&self.bg_color.clone().unwrap_or_default()).unwrap_or_default();
        let item_color_string = item_color + &item_bg_color;
        let value = self.get_value(data)?;
        let s = format!("{variable_color}{variable_bg_color}{item_color_string}{value}{unit_color}{unit_bg_color}");
        return if metric {
            Ok(s + &self.metric.clone().unwrap_or_default())
        } else {
            Ok(s + &self.imperial.clone().unwrap_or_default())
        };
    }
}
