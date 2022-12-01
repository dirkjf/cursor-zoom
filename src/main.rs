use std::thread::sleep;
use std::time::{Duration, Instant};
use enigo::{Enigo, MouseButton, MouseControllable};

use nannou::prelude::*;


fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
}


fn model(app: &App) -> Model {
    let window = app.new_window().decorations(false).fullscreen().transparent(true).always_on_top(true);
    let _window = window.view(view).build().unwrap();
    Model { _window }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(hsva(0.0, 0.0, 0.0, 0.0));
    let win = app.window_rect();

    let window = app.window(_model._window).unwrap();
    let scale_factor = window.scale_factor();

    draw.ellipse().radius(60.0).color(STEELBLUE).x_y(get_cursor_location().0 as f32 / scale_factor - (win.w() / 2.0),
                                                     get_cursor_location().1 as f32 / scale_factor - (2.0 * get_cursor_location().1 as f32 / scale_factor) + (win.h() / 2.0));
    draw.to_frame(app, &frame).unwrap();
}

fn get_cursor_location() -> (i32, i32) {
    let mut enigo = Enigo::new();
    return enigo.mouse_location();
}



