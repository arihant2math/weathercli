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
    pub args: Option<Vec<ItemJSON>>,
    pub kwargs: Option<HashMap<String, ItemJSON>>,
    pub scale: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LayoutDefaultsJSON {
    pub variable_color: String,
    pub text_color: String,
    pub unit_color: String,
    pub variable_bg_color: String,
    pub text_bg_color: String,
    pub unit_bg_color: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LayoutJSON {
    pub version: u64,
    pub defaults: LayoutDefaultsJSON,
    pub layout: Vec<Vec<ItemJSON>>,
}
