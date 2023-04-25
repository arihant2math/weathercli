use std::collections::HashMap;
use std::fs;
use std::io::Write;

use regex::Regex;
use serde_json::Value;

use crate::color;
use crate::layout::{ItemEnum, ItemJSON, util};

pub struct Item {
    data: ItemJSON,
}

fn round(f: f64) -> String {
    format!("{:.1}", f)
}

fn url_validator(u: &str) -> bool {
    let r = Regex::new(r"https?://(www\d?\.)?\w+\.\w+").expect("Regex failed (bug)");
    r.is_match(u)
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
        let mut new_s: String = s.to_string();
        if !new_s.is_empty() {
            if new_s.chars().next().expect("Oth char expected") == '@' {
                new_s = new_s[1..].to_string();
                let splt: Vec<&str> = new_s.split('|').collect();
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
                    scale: None,
                });
            } else if new_s.chars().next().expect("Oth char expected") == '#' {
                new_s = new_s[1..].to_string();
                let mut split: Vec<&str> = new_s.split('|').collect();
                let value = split[0];
                split.remove(0);
                let mut args: Vec<ItemEnum> = Vec::new();
                let mut kwargs: HashMap<String, ItemEnum> = HashMap::new();
                for item in split {
                    if !item.contains('=') {
                        args.push(ItemEnum::ItemString(item.to_string()))
                    } else {
                        let temp_item = item.to_string();
                        let kwarg: Vec<&str> = temp_item.split('=').collect();
                        kwargs.insert(
                            kwarg[0].to_string(),
                            ItemEnum::ItemString(kwarg[1].to_string()),
                        );
                    }
                }
                let item: ItemJSON = ItemJSON {
                    item_type: "function".to_string(),
                    color: None,
                    bg_color: None,
                    metric: None,
                    imperial: None,
                    unit_color: None,
                    value: value.to_string(),
                    args: Some(args),
                    kwargs: Some(kwargs),
                    scale: None,
                };
                return Item::from_item_json(item);
            } else if new_s.chars().next().expect("Oth char expected") == '\\' {
                new_s = new_s[1..].to_string();
            }
        }
        Item::from_item_json(ItemJSON {
            item_type: "text".to_string(),
            color: None,
            bg_color: None,
            metric: None,
            imperial: None,
            unit_color: None,
            value: new_s,
            args: None,
            kwargs: None,
            scale: None,
        })
    }

    pub fn from_item_json(i: ItemJSON) -> Self {
        Item { data: i }
    }

    fn get_variable_value(&self, data: &Value) -> Option<String> {
        let mut split: Vec<&str> = self.data.value.split('.').collect();
        let mut current = data;
        while !split.is_empty() {
            if split[0]
                .chars().next()
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
                if !current.is_null() {
                    current = &current[split[0]];
                } else {
                    return None;
                }
            }
            split.remove(0);
        }
        match current.as_str() {
            Some(t) => Some(t.to_string()),
            None => match current.as_f64() {
                Some(t) => Some(round(t)),
                None => current.as_i64().map(|t| t.to_string()),
            },
        }
    }

    fn get_function_value(&self, data: &Value) -> Option<String> {
        let args = self.data.args.clone().unwrap_or(Vec::new());
        let _kwargs = self.data.kwargs.clone().unwrap_or(HashMap::new());
        match &*self.data.value {
            "color_aqi" => Some(util::color_aqi(
                Item::new(args[0].clone())
                    .get_value(data)
                    .expect("no aqi value")
                    .parse()
                    .unwrap_or(0),
            )),
            _ => None, // TODO: add more functions
        }
    }

    pub fn get_value(&self, data: &Value) -> Option<String> {
        if self.data.item_type == "variable" {
            return self.get_variable_value(data);
        } else if self.data.item_type == "function" {
            return self.get_function_value(data);
        }
        Some(self.data.value.clone())
    }

    pub fn to_string(
        &self,
        data: &Value,
        variable_color: String,
        text_color: String,
        unit_color: String,
        variable_bg_color: String,
        text_bg_color: String,
        unit_bg_color: String,
        metric: bool,
    ) -> crate::Result<String> {
        if self.data.item_type == "text" {
            return Ok(text_color
                + &text_bg_color
                + &self.data.color.clone().unwrap_or("".to_string())
                + &self.data.bg_color.clone().unwrap_or("".to_string())
                + &self.data.value);
        } else if self.data.item_type == "variable" {
            let value = self.get_variable_value(data);
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
                Ok(s + &self.data.metric.clone().unwrap_or("".to_string()))
            } else {
                Ok(s + &self.data.imperial.clone().unwrap_or("".to_string()))
            };
        } else if self.data.item_type == "function" {
            let value = self.get_function_value(data);
            return Ok(self.data.color.clone().unwrap_or("".to_string())
                + &self.data.bg_color.clone().unwrap_or("".to_string())
                + &value.unwrap_or("".to_string()));
        }
        else if self.data.item_type == "image" {
            let source = Item::from_str(&self.data.value).get_value(data);
            let is_url = url_validator(&source.clone().unwrap());
            if is_url {
                let response = crate::networking::get_url(&source.unwrap_or("".to_string()), None, None, None);
                let mut f = fs::OpenOptions::new().write(true).truncate(true).create(true).open("temp.img").expect("Open options failed");
                f.write(&response?.bytes).expect("Write Failed");
            }
            return Ok(crate::layout::image_to_text::ascii_image("temp.img", self.data.scale.unwrap()));
        }
        Ok("".to_string())
    }
}
