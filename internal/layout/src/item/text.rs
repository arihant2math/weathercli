use shared_deps::serde_json::Value;
use terminal::color;

use crate::item::ItemType;
use crate::LayoutSettings;

pub struct Text {
    pub color: Option<String>,
    pub bg_color: Option<String>,
    pub value: String
}

impl ItemType for Text {
    fn get_value(&self, _data: &Value) -> crate::Result<String> {
        Ok(self.value.clone())
    }

    fn to_string(&self, _data: &Value, settings: LayoutSettings, _metric: bool) -> crate::Result<String> {
        let text_color = settings.text_color;
        let text_bg_color = settings.text_bg_color;
        let item_color =
            color::from_string(&self.color.clone().unwrap_or_default()).unwrap_or_default();
        let item_bg_color =
            color::from_string(&self.bg_color.clone().unwrap_or_default()).unwrap_or_default();
        let item_color_string = item_color + &item_bg_color;
        return Ok(format!(
                "{text_color}{text_bg_color}{item_color_string}{}",
                &self.value
            ));
    }
}