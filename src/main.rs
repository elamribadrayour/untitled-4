use nannou::prelude::*;

mod color;
mod dot;
mod env;
mod matrix;
mod model;

use crate::env::*;
use crate::model::Model;

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .view(view)
        .size(WIDTH, HEIGHT)
        .build()
        .unwrap();
    Model::new(window_id, "solarized")
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.update(app)
}

fn view(app: &App, model: &Model, frame: Frame) {
    model.draw(app, &frame);
}
