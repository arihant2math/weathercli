use crate::color;

pub fn pretty_list(li: Vec<String>, delimiter: Option<&str>) -> String {
    li.join(delimiter.unwrap_or(", "))
}

pub fn color_aqi(aqi: u8) -> String {
    match aqi {
        5 => color::FORE_RED.to_string() + &aqi.to_string(),
        3|4 => color::FORE_LIGHTYELLOW.to_string() + &aqi.to_string(),
        _ => color::FORE_GREEN.to_string() + &aqi.to_string()
    }
}

// TODO: Implement
// def replace(string, target, substitution):
//     return string.replace(target, substitution)
//
// def to_ascii(string: str):
//     return "\n".join(temp_constants.WEATHER_SYMBOL_WEGO[string])
