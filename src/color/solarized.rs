use nannou::color::encoding::Srgb;
use nannou::color::{rgb, rgba};

pub struct Solarized;

impl Solarized {
    pub fn color(i: usize) -> rgb::Rgb<Srgb, u8> {
        let i = i % 3;
        match i {
            0 => rgb(211, 54, 130),
            1 => rgb(42, 161, 152),
            2 => rgb(108, 113, 196),
            _ => rgb(0, 0, 0),
        }
    }

    pub fn background() -> rgb::Rgb<Srgb, u8> {
        rgb(7, 54, 66)
    }

    pub fn padding() -> rgb::Rgb<Srgb, u8> {
        rgb(253, 246, 227)
    }

    pub fn history() -> rgb::Rgba<Srgb, u8> {
        let background = Solarized::background();
        rgba(background.red, background.green, background.blue, 1)
    }
}
