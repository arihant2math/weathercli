use regex::Regex;
use std::fs;
use std::io::Write;
use terminal::color;
use weather_error::{Error, LayoutErr};

pub fn color_aqi(aqi: u8) -> crate::Result<String> {
    Ok(match aqi {
        5 => color::FORE_RED.to_string() + &aqi.to_string(),
        3 | 4 => color::FORE_LIGHTYELLOW.to_string() + &aqi.to_string(),
        _ => color::FORE_GREEN.to_string() + &aqi.to_string(),
    })
}

fn url_validator(u: &str) -> bool {
    let re: &str = r"https?://(www\d?\.)?\w+\.\w+";
    let r = Regex::new(re).expect("Regex failed to compile (bug)");
    r.is_match(u)
}

pub fn image(source: String, scale: f64) -> crate::Result<String> {
    let is_url = url_validator(&source);
    if is_url {
        let response = networking::get!(&source)?;
        let mut path = weather_dirs::weathercli_dir()?;
        let file_name = "temp.img";
        path.push(file_name);
        let mut f = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&path)?;
        f.write_all(&response.bytes)?;
        return crate::layout::image_to_text::ascii_image(&path.to_str().ok_or(Error::LayoutError(LayoutErr {
            message: "Temp image save path is not valid of unicode".to_string(),
            row: None,
            item: None,
        }))?, scale);
    }
    Err("source is not a url".to_string())? // TODO: Fix
}

// TODO: Implement to_ascii and rainbow
