use std::io::Cursor;

use iced;
use iced::{Font, Sandbox};
use iced::window::{Icon, icon, Level};
use image::io::Reader as ImageReader;

use app::App;

mod app;
mod datasource;
mod message;

#[cfg(target_os = "windows")]
fn font() -> Font {
    Font::with_name("Segoeui")
}

#[cfg(target_os = "macos")]
fn font() -> Font {
    Font::with_name("Helvetica")
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
fn font() -> Font {
    Font::default()
}

fn icon() -> Icon {
    let bytes = include_bytes!("../../../icon/icon.png").to_vec();
    let image_parsed = ImageReader::new(Cursor::new(bytes)).with_guessed_format().unwrap().decode().unwrap(); // TODO: Force png (also fix unwraps)
    let image = image_parsed.to_rgba8();
    icon::from_rgba(
        image.into_raw(),
        512,
        512,
    ).unwrap()
}

pub fn run_settings_app() -> iced::Result {
    App::run(iced::Settings {
        id: Default::default(),
        window: iced::window::Settings {
            size: (1000, 900),
            position: Default::default(),
            min_size: Some((750, 500)),
            max_size: Some((2250, 1750)),
            visible: true,
            resizable: true,
            decorations: true,
            transparent: false,
            icon: Some(icon()),
            platform_specific: Default::default(),
            level: Level::Normal,
        },
        flags: (),
        default_font: font(),
        default_text_size: 16.0,
        antialiasing: true,
        exit_on_close_request: true,
    })
}
