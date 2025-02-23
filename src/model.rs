use nannou::prelude::*;

use crate::color::Palette;
use crate::env::{NB_COLS, NB_ROWS, PADDING};
use crate::matrix::Matrix;

pub struct Model {
    pub palette: Palette,
    pub _window_id: window::Id,
    pub matrix: Matrix,
}

impl Model {
    pub fn new(window_id: window::Id, palette: &str) -> Self {
        let mut matrix = Matrix::new();
        for (i, row) in matrix.cells.iter_mut().enumerate() {
            let i_f = i as f32;
            for (j, cell) in row.iter_mut().enumerate() {
                let j_f = j as f32;
                let x = -0.5 + i_f / NB_COLS as f32 + 1.0 / NB_COLS as f32 / 2.0;
                let y = -0.5 + j_f / NB_ROWS as f32 + 1.0 / NB_ROWS as f32 / 2.0;
                cell.position = Vec2::new(x, y);
            }
        }

        Model {
            matrix,
            _window_id: window_id,
            palette: Palette::new(palette),
        }
    }

    pub fn update(&mut self, app: &App) {
        self.matrix.za3za3(app);
    }

    pub fn draw(&self, app: &App, frame: &Frame) {
        let draw = app.draw();
        let window = app.window_rect().pad(PADDING);

        if app.elapsed_frames() == 0 {
            draw.background().color(self.palette.padding());

            draw.rect()
                .xy(window.xy())
                .wh(window.wh())
                .color(self.palette.background());
        }

        draw.rect()
            .xy(window.xy())
            .wh(window.wh())
            .color(self.palette.history());

        let grid_dim = window.wh();
        for i in 0..NB_ROWS - 1 {
            for j in 0..NB_COLS - 1 {
                let color = self.palette.color(i + j);
                let cells = [
                    Vec2::new(
                        self.matrix.get_x(i, j) * grid_dim.x,
                        self.matrix.get_y(i, j) * grid_dim.y,
                    ),
                    Vec2::new(
                        self.matrix.get_x(i + 1, j) * grid_dim.x,
                        self.matrix.get_y(i + 1, j) * grid_dim.y,
                    ),
                    Vec2::new(
                        self.matrix.get_x(i + 1, j + 1) * grid_dim.x,
                        self.matrix.get_y(i + 1, j + 1) * grid_dim.y,
                    ),
                    Vec2::new(
                        self.matrix.get_x(i, j + 1) * grid_dim.x,
                        self.matrix.get_y(i, j + 1) * grid_dim.y,
                    ),
                    Vec2::new(
                        self.matrix.get_x(i, j) * grid_dim.x,
                        self.matrix.get_y(i, j) * grid_dim.y,
                    ),
                ];
                draw.polyline().weight(5.0).points(cells).color(color);
            }
        }

        draw.to_frame(app, frame).unwrap();

        app.main_window()
            .capture_frame(format!("output/frame-{}.png", app.elapsed_frames()));

        if app.elapsed_frames() == 1000 {
            app.quit();
        }
    }
}
