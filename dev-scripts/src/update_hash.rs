use std::fs;
use std::io::{Read, Write};
use serde_json::Value;

pub fn update_hash(file: &str, key: &str) -> weather_core::Result<()> {
    let file_hash = weather_core::util::hash_file(file)?;
    let mut index_json_data = String::new();
    fs::OpenOptions::new().read(true).open("./docs_templates/index.json")?.read_to_string(&mut index_json_data)?;
    let mut index_json: Value = serde_json::from_str(&index_json_data)?;
    index_json[key] = Value::String(file_hash);
    let data = serde_json::to_string_pretty(&index_json)?;
    fs::OpenOptions::new().write(true).truncate(true).open("./docs_templates/index.json")?.write_all(data.as_bytes())?;
    Ok(())
}