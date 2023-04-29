extern crate image;

use std::path::Path;

use image::{imageops, GenericImageView};

fn rgb(red: u8, green: u8, blue: u8) -> String {
    let escape = "\x1b[";
    escape.to_string()
        + "38;2;"
        + &red.to_string()
        + ";"
        + &green.to_string()
        + ";"
        + &blue.to_string()
        + "m"
}

pub fn ascii_image(input_path: &str, scale: f64) -> crate::Result<String> {
    let img = image::open(Path::new(input_path)).map_err(|e| "Failed to open image")?;
    let img_width = img.dimensions().0;
    let img_height = img.dimensions().1;
    #[allow(clippy::cast_sign_loss)]
    let new_img = img.resize(
        (f64::from(img_width) * scale) as u32,
        (f64::from(img_height) * scale) as u32,
        imageops::FilterType::Nearest,
    );
    let mut text = String::new();
    for p in new_img.pixels() {
        if p.1 == 0 {
            text += "\n";
        }
        let pixel = p.2 .0;
        text += &rgb(pixel[0], pixel[1], pixel[2]);
        text += "█";
    }
    Ok(text)
}
