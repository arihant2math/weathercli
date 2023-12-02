use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ItemSerde {
    #[serde(rename = "type")]
    pub item_type: String,
    pub color: Option<String>,
    pub bg_color: Option<String>,
    pub metric: Option<String>,
    pub imperial: Option<String>,
    pub unit_color: Option<String>,
    pub value: String,
    pub args: Option<Vec<ItemSerde>>,
    pub kwargs: Option<HashMap<String, ItemSerde>>,
    pub scale: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LayoutDefaultsSerde {
    pub variable_color: String,
    pub text_color: String,
    pub unit_color: String,
    pub variable_bg_color: String,
    pub text_bg_color: String,
    pub unit_bg_color: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LayoutSerde {
    pub layout_version: u64, // layout version for debugging purposes
    pub name: String,
    pub author: Option<String>,
    pub description: Option<String>,
    pub defaults: LayoutDefaultsSerde,
    pub layout: Vec<Vec<ItemSerde>>,
}
