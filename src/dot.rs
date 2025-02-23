use nannou::prelude::*;
use nannou::rand::{thread_rng, Rng};

#[derive(Copy, Clone)]
pub struct Dot {
    pub position: Vec2,
}

impl Dot {
    pub fn empty() -> Self {
        Dot {
            position: Vec2::new(0.0, 0.0),
        }
    }

    pub fn za3za3(&mut self, _app: &App) {
        let mut rng = thread_rng();
        let x = rng.gen_range(-0.0005..=0.0005);
        let y = rng.gen_range(-0.0005..=0.0005);
        self.position = Vec2::new(self.position.x + x, self.position.y + y);
    }
}
