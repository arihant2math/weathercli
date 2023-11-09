use shared_deps::serde_json::Value;

use crate::layout::layout_serde::ItemSerde;
use crate::layout::{util, LayoutSettings};
use terminal::color;
use weather_error::LayoutErr;

pub struct Item {
    data: ItemSerde,
}

fn round(f: f64) -> String {
    format!("{f:.1}")
}

impl Item {
    pub fn new(i: ItemSerde) -> Self {
        Self { data: i }
    }

    fn get_variable_value(&self, data: &Value) -> crate::Result<String> {
        if data.is_null() {
            return Ok(String::from("null"));
        }
        let mut split: Vec<&str> = self.data.value.split('.').collect();
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
                    return Err(weather_error::Error::LayoutError(LayoutErr {
                        message: "Variable not found in data".to_string(),
                        row: None,
                        item: None,
                    }));
                }
                current = &current[split[0]];
            }
            split.remove(0);
        }
        match current.as_str() {
            Some(t) => Ok(t.to_string()),
            None => match current.as_f64() {
                Some(t) => Ok(round(t)),
                None => Ok(current
                    .as_i64()
                    .ok_or_else(|| {
                        weather_error::Error::LayoutError(LayoutErr {
                            message: "Variable type not supported".to_string(),
                            row: None,
                            item: None,
                        })
                    })?
                    .to_string()),
            },
        }
    }

    fn get_function_value(&self, data: &Value) -> crate::Result<String> {
        let args = self.data.args.clone().unwrap_or_default();
        let _kwargs = self.data.kwargs.clone().unwrap_or_default();
        match &*self.data.value {
            "color_aqi" => util::color_aqi(
                Self::new(args[0].clone())
                    .get_value(data)?
                    .parse()
                    .unwrap_or(0),
            ),
            "image" => util::image(
                Self::new(args[0].clone()).get_value(data)?.parse().unwrap(),
                Self::new(args[1].clone())
                    .get_value(data)?
                    .parse()
                    .unwrap_or(1.),
            ),
            _ => Err(weather_error::Error::LayoutError(LayoutErr {
                message: "Function not found".to_string(),
                row: None,
                item: None,
            })), // TODO: add more functions
        }
    }

    pub fn get_value(&self, data: &Value) -> crate::Result<String> {
        if self.data.item_type == "variable" {
            return self.get_variable_value(data);
        } else if self.data.item_type == "function" {
            return self.get_function_value(data);
        }
        Ok(self.data.value.clone())
    }

    pub fn to_string(
        &self,
        data: &Value,
        settings: LayoutSettings,
        metric: bool,
    ) -> crate::Result<String> {
        let text_color = settings.text_color;
        let text_bg_color = settings.text_bg_color;
        let variable_color = settings.variable_color;
        let variable_bg_color = settings.variable_bg_color;
        let unit_color = settings.unit_color;
        let unit_bg_color = settings.unit_bg_color;
        let item_color =
            color::from_string(self.data.color.clone().unwrap_or_default()).unwrap_or_default();
        let item_bg_color =
            color::from_string(self.data.bg_color.clone().unwrap_or_default()).unwrap_or_default();
        let item_color_string = item_color + &item_bg_color;
        if self.data.item_type == "text" {
            return Ok(format!(
                "{text_color}{text_bg_color}{item_color_string}{}",
                &self.data.value
            ));
        } else if self.data.item_type == "variable" {
            let value = self.get_variable_value(data)?;
            let s = format!("{variable_color}{variable_bg_color}{item_color_string}{value}{unit_color}{unit_bg_color}");
            return if metric {
                Ok(s + &self.data.metric.clone().unwrap_or_default())
            } else {
                Ok(s + &self.data.imperial.clone().unwrap_or_default())
            };
        } else if self.data.item_type == "function" {
            let value = self.get_function_value(data)?;
            return Ok(format!("{item_color_string}{value}"));
        }
        Ok(String::new())
    }
}
