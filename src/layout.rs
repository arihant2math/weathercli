use serde_json::Value;

pub fn parse(s: &str) {
    let json_value: Value = serde_json::from_str(s).expect("json parsing failed");

}
