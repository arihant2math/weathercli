use std::collections::HashMap;

use serde::{Deserialize, Serialize};

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
pub enum RowEnum {
    RowString(String),
    RowVec(Vec<ItemEnum>),
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
pub struct LayoutJSON {
    pub version: Option<u64>,
    pub defaults: Option<LayoutDefaultsJSON>,
    pub layout: Option<Vec<RowEnum>>,
}
