use nannou::prelude::*;

struct Ball {
    x: f32, y: f32,
    x_speed: f32, y_speed: f32,
    radius: f32,
}

pub struct Model {
    _window: window::Id,
    ball: Ball,
    gravity_x: f32,
    gravity_y: f32,
}

pub fn model(app: &App) -> Model {
    let _window = app.new_window().title("ball").view(view).build().unwrap();
    let ball = Ball {
        x: 100.0, y: 100.0,
        x_speed: 1.0, y_speed: 1.0,
        radius: 50.0,
    };
    Model { _window, ball, gravity_x: 0.0, gravity_y: -0.8 }
}

pub fn update(_app: &App, _model: &mut Model, _update: Update) {
    _model.ball.x_speed += _model.gravity_x;
    _model.ball.y_speed += _model.gravity_y;
    _model.ball.x += _model.ball.x_speed;
    _model.ball.y += _model.ball.y_speed;

    let width = _app.main_window().rect().w();
    let height = _app.main_window().rect().h();

    if -width / 2.0 + _model.ball.radius > _model.ball.x || _model.ball.x > width / 2.0 - _model.ball.radius {
        _model.ball.x_speed *= -0.8;
    }
    if -height / 2.0 + _model.ball.radius > _model.ball.y || _model.ball.y > height / 2.0 - _model.ball.radius {
        _model.ball.y_speed *= -0.8;
    }
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    draw.ellipse().color(WHITE)
        .radius(_model.ball.radius)
        .x(_model.ball.x)
        .y(_model.ball.y);
    draw.to_frame(app, &frame).unwrap();
}
