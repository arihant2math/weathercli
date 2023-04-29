use crate::color;

pub fn color_aqi(aqi: u8) -> crate::Result<String> {
    Ok(match aqi {
        5 => color::FORE_RED.to_string() + &aqi.to_string(),
        3 | 4 => color::FORE_LIGHTYELLOW.to_string() + &aqi.to_string(),
        _ => color::FORE_GREEN.to_string() + &aqi.to_string(),
    })
}
// TODO: Implement to_ascii and rainbow
