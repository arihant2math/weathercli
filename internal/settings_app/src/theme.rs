use iced::{Color, Theme};
use iced::theme::{Custom, Palette};
use iced::theme::palette::Pair;

pub fn get_light_theme() -> Theme {
     let mut custom = Custom::new(Palette {
            background: Color::from_rgb(
                251 as f32 / 255.0,
                251 as f32 / 255.0,
                251 as f32 / 255.0,
            ),
            text:  Color::from_rgb(
                26 as f32 / 255.0,
                26 as f32 / 255.0,
                26 as f32 / 255.0,
            ),
            primary: Color::from_rgb(
                0 as f32 / 255.0,
                103 as f32 / 255.0,
                192 as f32 / 255.0,
            ),
            success: Color::from_rgb(
                0x12 as f32 / 255.0,
                0x66 as f32 / 255.0,
                0x4F as f32 / 255.0,
            ),
            danger: Color::from_rgb(
                0xC3 as f32 / 255.0,
                0x42 as f32 / 255.0,
                0x3F as f32 / 255.0,
            )
    });
    custom.extended.primary.strong = Pair::new(
        Color::from_rgb(
            0 as f32 / 255.0,
            103 as f32 / 255.0,
            192 as f32 / 255.0,
        ),
        Color::from_rgb(
            255 as f32 / 255.0,
            255 as f32 / 255.0,
            255 as f32 / 255.0,
        ),
    );
    custom.extended.primary.base = Pair::new(
        Color::from_rgb(
            25 as f32 / 255.0,
            117 as f32 / 255.0,
            197 as f32 / 255.0,
        ),
        Color::from_rgb(
            255 as f32 / 255.0,
            255 as f32 / 255.0,
            255 as f32 / 255.0,
        ),
    );
    return Theme::Custom(Box::new(custom));
}

pub fn get_dark_theme() -> Theme {
    return Theme::custom(Palette {
            background: Color::from_rgb(
                32 as f32 / 255.0,
                32 as f32 / 255.0,
                32 as f32 / 255.0,
            ),
            text:  Color::from_rgb(
                253 as f32 / 255.0,
                253 as f32 / 255.0,
                253 as f32 / 255.0,
            ),
            primary: Color::from_rgb(
                76 as f32 / 255.0,
                194 as f32 / 255.0,
                255 as f32 / 255.0,
            ),
            success: Color::from_rgb(
                0x12 as f32 / 255.0,
                0x66 as f32 / 255.0,
                0x4F as f32 / 255.0,
            ),
            danger: Color::from_rgb(
                0xC3 as f32 / 255.0,
                0x42 as f32 / 255.0,
                0x3F as f32 / 255.0,
            )
    });
}
