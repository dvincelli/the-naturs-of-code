extern crate nannou;

use nannou::prelude::*;

fn main() {
    nannou::app(model).size(640, 360).update(update).run();
}

struct Model {
    _window: window::Id,
    pub angle: f32
}


fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    Model { _window, angle: 0.0 }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.angle = (model.angle + 0.1) % 360.0;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    draw.rotate(model.angle).line().start(pt2(-50.0, 0.0)).end(pt2(50.0, 0.0)).color(DARKBLUE);
    draw.rotate(model.angle).ellipse().w_h(8.0, 8.0).x_y(50.0, 0.0).color(STEELBLUE);
    draw.rotate(model.angle).ellipse().w_h(8.0, 8.0).x_y(-50.0, 0.0).color(STEELBLUE);

    draw.to_frame(app, &frame).unwrap();
}

