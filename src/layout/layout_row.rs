use serde_json::Value;

use crate::layout::ItemEnum;
use crate::layout::layout_item::Item;

pub struct Row {
    items: Vec<Item>,
}

impl Row {
    pub fn from_str(data: &str) -> Self {
        let mut item_list = Vec::new();
        let mut previous_char = '\0';
        let mut current = "".to_string();
        for c in data.to_string().chars() {
            if (c == '{' || c == '}') && previous_char != '\\' {
                item_list.push(Item::from_str(&current));
                current = "".to_string();
                previous_char = '\0';
            } else {
                current += &c.to_string();
                previous_char = c;
            }
        }
        if !current.is_empty() {
            item_list.push(Item::from_str(&current));
        }
        Row { items: item_list }
    }

    pub fn from_vec(data: Vec<ItemEnum>) -> Self {
        let mut items: Vec<Item> = Vec::new();
        for (_count, item) in data.iter().enumerate() {
            items.push(Item::new(item.clone()));
        }
        Row { items }
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
    ) -> String {
        let mut s = "".to_string();
        for (_count, i) in self.items.iter().enumerate() {
            s += &*i.to_string(
                data,
                variable_color.clone(),
                text_color.clone(),
                unit_color.clone(),
                variable_bg_color.clone(),
                text_bg_color.clone(),
                unit_bg_color.clone(),
                metric,
            );
        }
        s
    }
}
