use nannou::prelude::*;

use crate::dot::Dot;
use crate::env::{NB_COLS, NB_ROWS};

pub struct Matrix {
    pub cells: [[Dot; NB_COLS]; NB_ROWS],
}

impl Matrix {
    pub fn new() -> Self {
        Matrix {
            cells: [[Dot::empty(); NB_COLS]; NB_ROWS],
        }
    }

    pub fn get(&self, i: usize, j: usize) -> &Dot {
        &self.cells[i][j]
    }

    pub fn get_x(&self, i: usize, j: usize) -> f32 {
        self.get(i, j).position.x
    }

    pub fn get_y(&self, i: usize, j: usize) -> f32 {
        self.get(i, j).position.y
    }

    pub fn za3za3(&mut self, app: &App) {
        for i in 0..NB_ROWS {
            for j in 0..NB_COLS {
                self.cells[i][j].za3za3(app);
            }
        }
    }
}
