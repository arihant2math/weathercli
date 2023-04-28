extern crate image;

use std::path::Path;

use image::{GenericImageView, imageops};

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

pub fn ascii_image(input_path: &str, scale: f64) -> String {
    let img = image::open(Path::new(input_path)).unwrap();
    let img_width = img.dimensions().0;
    let img_height = img.dimensions().1;
    let new_img = img.resize(
        (img_width as f64 * scale) as u32,
        (img_height as f64 * scale) as u32,
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
    text
}
