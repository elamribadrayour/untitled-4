use nannou::color::encoding::Srgb;
use nannou::color::{rgb, rgba};

pub struct Game;

impl Game {
    pub fn color(i: usize) -> rgb::Rgb<Srgb, u8> {
        let i = i % 4;
        match i {
            0 => rgb(214, 50, 48),
            1 => rgb(247, 208, 2),
            2 => rgb(26, 83, 192),
            3 => rgb(13, 39, 75),
            _ => rgb(0, 0, 0),
        }
    }

    pub fn background() -> rgb::Rgb<Srgb, u8> {
        rgb(246, 238, 227)
    }

    pub fn padding() -> rgb::Rgb<Srgb, u8> {
        rgb(217, 189, 165)
    }

    pub fn history() -> rgb::Rgba<Srgb, u8> {
        let background = Game::background();
        rgba(background.red, background.green, background.blue, 1)
    }
}
