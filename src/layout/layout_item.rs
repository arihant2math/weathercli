use std::collections::HashMap;

use serde_json::Value;

use crate::backend::weather_forecast::WeatherForecastRS;
use crate::color;
use crate::layout::{ItemEnum, ItemJSON};

pub struct Item {
    data: ItemJSON,
}

impl Item {
    pub fn new(i: ItemEnum) -> Self {
        match i {
            ItemEnum::ItemString(s) => Item::from_str(&s),
            ItemEnum::ItemFloat(f) => Item::from_str(&f.to_string()),
            ItemEnum::ItemInt(i) => Item::from_str(&i.to_string()),
            ItemEnum::Item(i) => Item::from_item_json(i),
        }
    }

    pub fn from_str(s: &str) -> Self {
        let mut new_s: String = s.to_string(); // TODO: Add check for empty string
        if new_s.chars().nth(0).expect("Oth char expected") == '@' {
            new_s = (&new_s[1..]).to_string();
            let splt: Vec<&str> = new_s.split("|").collect();
            let mut metric: Option<String> = None;
            let mut imperial: Option<String> = None;
            if splt.len() == 2 {
                metric = Some(splt[1].to_string());
                imperial = Some(splt[1].to_string());
            } else if splt.len() == 3 {
                imperial = Some(splt[1].to_string());
                metric = Some(splt[2].to_string());
            }
            return Item::from_item_json(ItemJSON {
                item_type: "variable".to_string(),
                color: None,
                bg_color: None,
                metric,
                imperial,
                unit_color: None,
                value: splt[0].to_string(),
                args: None,
                kwargs: None,
            });
        } else if new_s.chars().nth(0).expect("Oth char expected") == '#' {
            new_s = new_s[1..].to_string();
            let mut split: Vec<&str> = new_s.split("|").collect();
            split.remove(0);
            let mut args: Vec<String> = Vec::new();
            let mut kwargs: HashMap<String, String> = HashMap::new();
            for item in split {
                if !item.contains('=') {
                    args.push(item.to_string())
                } else {
                    let temp_item = item.to_string();
                    let kwarg: Vec<&str> = temp_item.split("=").collect();
                    kwargs.insert(kwarg[0].to_string(), kwarg[1].to_string());
                }
            }
            let item: ItemJSON = ItemJSON {
                item_type: "function".to_string(),
                color: None,
                bg_color: None,
                metric: None,
                imperial: None,
                unit_color: None,
                value: "".to_string(),
                args: Some(args),
                kwargs: Some(kwargs),
            };
            return Item::from_item_json(item);
        } else if new_s.chars().nth(0).expect("Oth char expected") == '\\' {
            new_s = (&new_s[1..]).to_string();
        }
        return Item::from_item_json(ItemJSON {
            item_type: "text".to_string(),
            color: None,
            bg_color: None,
            metric: None,
            imperial: None,
            unit_color: None,
            value: new_s,
            args: None,
            kwargs: None,
        });
    }

    pub fn from_item_json(i: ItemJSON) -> Self {
        Item { data: i }
    }
    fn get_variable_value(&self, data: Value) -> Option<String> {
        let mut split: Vec<&str> = self.data.value.split('.').collect();
        let mut current = data;
        while !split.is_empty() {
            if split[0]
                .chars()
                .nth(0)
                .expect("0th element expected don't place two dots in a row, like: \"..\"")
                == '['
            {
                // list item
                let place = split[0][1..split.clone()[0].len() - 1]
                    .parse::<usize>()
                    .unwrap();
                current = current.clone()[place].clone();
            } else {
                // normal variable
                if !current.is_null() {
                    current = current.clone()[split[0]].clone();
                } else {
                    return None;
                }
            }
            split.remove(0);
        }
        match current.as_str() {
            Some(t) => Some(t.to_string()),
            None => match current.as_f64() {
                Some(t) => Some(t.to_string()),
                None => match current.as_i64() {
                    Some(t) => Some(t.to_string()),
                    None => None,
                },
            },
        }
    }

    fn get_function_value(&self) -> Option<String> {
        let args = self.data.args.clone().unwrap_or(Vec::new());
        let kwargs = self.data.kwargs.clone().unwrap_or(HashMap::new());
        match &*self.data.value {
            "color_aqi" => Some(args.get(0).unwrap().to_string()),
            _ => None,
        }
    }

    fn get_value(&self, data: WeatherForecastRS) -> Option<String> {
        if self.data.item_type == "variable" {
            return self
                .get_variable_value(serde_json::to_value(data).expect("Serialization failed"));
        } else if self.data.item_type == "function" {
            return self.get_function_value();
        }
        Some(self.data.value.clone())
    }

    pub fn to_string(
        &self,
        data: WeatherForecastRS,
        variable_color: String,
        text_color: String,
        unit_color: String,
        variable_bg_color: String,
        text_bg_color: String,
        unit_bg_color: String,
        metric: bool,
    ) -> String {
        if self.data.item_type == "text" {
            return text_color
                + &text_bg_color
                + &self.data.color.clone().unwrap_or("".to_string())
                + &self.data.bg_color.clone().unwrap_or("".to_string())
                + &self.data.value;
        } else if self.data.item_type == "variable" {
            let value =
                self.get_variable_value(serde_json::to_value(data).expect("Serialization failed"));
            let s = variable_color
                + &variable_bg_color
                + &color::from_string(self.data.color.clone().unwrap_or("".to_string()))
                    .unwrap_or("".to_string())
                + &color::from_string(self.data.bg_color.clone().unwrap_or("".to_string()))
                    .unwrap_or("".to_string())
                + &value.unwrap_or("".to_string())
                + &unit_color
                + &unit_bg_color
                + &self.data.unit_color.clone().unwrap_or("".to_string());
            return if metric {
                // TODO: Fix color mess
                s + &self.data.metric.clone().unwrap_or("".to_string())
            } else {
                s + &self.data.imperial.clone().unwrap_or("".to_string())
            };
        } else if self.data.item_type == "function" {
            let value = self.get_function_value();
            return self.data.color.clone().unwrap_or("".to_string())
                + &self.data.bg_color.clone().unwrap_or("".to_string())
                + &value.unwrap_or("".to_string());
        }
        // else if self.data.item_type == "image" {
        //     let source = Item::from_str(&self.data.value).get_value(data);
        //     let is_uri = uri_validator(source);
        //     if is_uri {
        //         let response = networking::get_url(source.unwrap_or("".to_string()), None, None, None);
        //         let f = open("temp.img", "bw");
        //         f.write(bytes(response.bytes));
        //         f.close()
        //     }
        //     data = image_to_text("temp.img", self.item_data["scale"])
        //     return data;
        // }
        "".to_string()
    }
}
