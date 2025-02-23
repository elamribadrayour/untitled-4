use nannou::color::encoding::Srgb;
use nannou::color::rgb;

use crate::color::{Game, Solarized};

pub enum Palette {
    Game,
    Solarized,
}

impl Palette {
    pub fn new(palette: &str) -> Self {
        match palette {
            "game" => Palette::Game,
            "solarized" => Palette::Solarized,
            _ => panic!("Invalid palette"),
        }
    }

    pub fn color(&self, i: usize) -> rgb::Rgb<Srgb, u8> {
        match self {
            Palette::Game => Game::color(i),
            Palette::Solarized => Solarized::color(i),
        }
    }

    pub fn background(&self) -> rgb::Rgb<Srgb, u8> {
        match self {
            Palette::Game => Game::background(),
            Palette::Solarized => Solarized::background(),
        }
    }

    pub fn padding(&self) -> rgb::Rgb<Srgb, u8> {
        match self {
            Palette::Game => Game::padding(),
            Palette::Solarized => Solarized::padding(),
        }
    }

    pub fn history(&self) -> rgb::Rgba<Srgb, u8> {
        match self {
            Palette::Game => Game::history(),
            Palette::Solarized => Solarized::history(),
        }
    }
}
