use serde_json::Value;

use crate::error::{Error, LayoutErr};
use crate::layout::layout_item::Item;
use crate::layout::layout_serde::ItemJSON;
use crate::layout::LayoutSettings;

pub struct Row {
    items: Vec<Item>,
}

fn reemit_layout_error(e: Error, count: usize) -> Error {
    match e {
        Error::LayoutError(e) => Error::LayoutError(LayoutErr {
            message: e.message,
            row: None,
            item: Some(count as u64),
        }),
        _ => e,
    }
}

impl Row {
    pub fn new(data: &Vec<ItemJSON>) -> Self {
        let mut items: Vec<Item> = Vec::new();
        for (_count, item) in data.iter().enumerate() {
            items.push(Item::new(item.clone()));
        }
        Self { items }
    }

    pub fn to_string(
        &self,
        data: &Value,
        settings: LayoutSettings,
        metric: bool,
    ) -> crate::Result<String> {
        let mut s = String::new();
        for (count, i) in self.items.iter().enumerate() {
            s += &*i
                .to_string(
                    data,
                    settings.clone(),
                    metric,
                )
                .map_err(|e| reemit_layout_error(e, count))?;
        }
        Ok(s)
    }
}
