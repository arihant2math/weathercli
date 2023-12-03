mod app;
mod datasource;
mod message;
mod theme;

use iced;
use iced::Sandbox;
use iced::window::Level;
use app::App;



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
            icon: Default::default(), // TODO: icon
            platform_specific: Default::default(),
            level: Level::Normal,
        },
        flags: (),
        default_font: Default::default(),
        default_text_size: 16.0,
        antialiasing: true,
        exit_on_close_request: true,
    })
}
