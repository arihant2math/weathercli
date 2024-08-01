use crate::LayoutErr;
use shared_deps::serde_json::Value;

use crate::layout_serde::ItemSerde;
use crate::LayoutSettings;

mod function;
mod text;
mod variable;

trait ItemType {
    fn get_value(&self, data: &Value) -> crate::Result<String>;
    fn to_string(
        &self,
        data: &Value,
        settings: LayoutSettings,
        metric: bool,
    ) -> crate::Result<String>;
}

pub enum Item {
    Text(text::Text),
    Variable(variable::Variable),
    Function(function::Function),
}

impl Item {
    pub fn new(i: ItemSerde) -> crate::Result<Self> {
        Ok(match &*i.item_type {
            "text" => Item::Text(text::Text {
                color: i.color,
                bg_color: i.bg_color,
                value: i.value,
            }),
            "variable" => Item::Variable(variable::Variable {
                color: i.color,
                bg_color: i.bg_color,
                metric: i.metric,
                imperial: i.imperial,
                unit_color: i.unit_color,
                value: i.value,
            }),
            "function" => Item::Function(function::Function {
                color: i.color,
                bg_color: i.bg_color,
                value: i.value,
                args: i.args,
                kwargs: i.kwargs,
            }),
            _ => Err(LayoutErr {
                message: "item_type unknown".to_string(),
                row: None,
                item: None,
            })?,
        })
    }

    pub fn get_value(&self, data: &Value) -> crate::Result<String> {
        match self {
            Item::Text(t) => t.get_value(data),
            Item::Variable(v) => v.get_value(data),
            Item::Function(f) => f.get_value(data),
        }
    }

    pub fn to_string(
        &self,
        data: &Value,
        settings: LayoutSettings,
        metric: bool,
    ) -> crate::Result<String> {
        match self {
            Item::Text(t) => t.to_string(data, settings, metric),
            Item::Variable(v) => v.to_string(data, settings, metric),
            Item::Function(f) => f.to_string(data, settings, metric),
        }
    }
}
