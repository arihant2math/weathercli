use shared_deps::serde_json::Value;
use std::collections::HashMap;
use terminal::color;
use weather_error::LayoutErr;

use crate::{LayoutSettings, util};
use crate::item::{Item, ItemType};
use crate::layout_serde::ItemSerde;

pub struct Function {
    pub color: Option<String>,
    pub bg_color: Option<String>,
    pub value: String,
    pub args: Option<Vec<ItemSerde>>,
    pub kwargs: Option<HashMap<String, ItemSerde>>,
}

impl ItemType for Function {
    fn get_value(&self, data: &Value) -> crate::Result<String> {
        let args = self.args.clone().unwrap_or_default();
        let _kwargs = self.kwargs.clone().unwrap_or_default();
        match &*self.value {
            "color_aqi" => util::color_aqi(
                Item::new(args[0].clone())?
                    .get_value(data)?
                    .parse()
                    .unwrap_or(0),
            ),
            "image" => util::image(
                Item::new(args[0].clone())?.get_value(data)?.parse().unwrap(),
                Item::new(args[1].clone())?
                    .get_value(data)?
                    .parse()
                    .unwrap_or(1.),
            ),
            "location" => util::location(
                Item::new(args[0].clone())?.get_value(data)?,
                Item::new(args[1].clone())?.get_value(data)?,
                Item::new(args[2].clone())?.get_value(data)?,
            ),
            _ => Err(weather_error::Error::LayoutError(LayoutErr {
                message: "Function not found".to_string(),
                row: None,
                item: None,
            })), // TODO: add more functions
        }
    }

    fn to_string(&self, data: &Value, _settings: LayoutSettings, _metric: bool) -> crate::Result<String> {
        let item_color =
            color::from_string(&self.color.clone().unwrap_or_default()).unwrap_or_default();
        let item_bg_color =
            color::from_string(&self.bg_color.clone().unwrap_or_default()).unwrap_or_default();
        let item_color_string = item_color + &item_bg_color;
        let value = self.get_value(data)?;
        return Ok(format!("{item_color_string}{value}"));
    }
}