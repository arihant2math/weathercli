use std::io::Cursor;

use iced;
use iced::{Font, Sandbox, Size};
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
            size: (Size {
                width: 1000f32,
                height: 900f32
            }),
            position: Default::default(),
            min_size: Some(Size {
                width: 750f32,
                height: 500f32
            }),
            max_size: Some(Size {
                width: 2250f32,
                height: 1750f32
            }),
            visible: true,
            resizable: true,
            decorations: true,
            transparent: false,
            icon: Some(icon()),
            platform_specific: Default::default(),
            level: Level::Normal,
            exit_on_close_request: true,
        },
        flags: (),
        default_font: font(),
        default_text_size: iced::Pixels(16.0),
        antialiasing: true,
        fonts: Default::default(),
    })
}
