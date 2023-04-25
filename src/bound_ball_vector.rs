use nannou::lyon::geom::euclid::Vector2D;
use nannou::prelude::*;

struct Ball {
    position: Vec2,
    speed: Vec2,
    radius: f32,
}

pub struct Model {
    _window: window::Id,
    ball: Ball,
    gravity: Vec2,
}

pub fn model(app: &App) -> Model {
    let _window = app.new_window().title("vector").view(view).build().unwrap();
    let ball = Ball {
        position: Vec2::new(100.0, 100.0),
        speed: Vec2::new(1.0, 1.0),
        radius: 50.0,
    };
    Model { _window, ball, gravity: Vec2::new(0.0, -0.8), }
}

pub fn update(_app: &App, _model: &mut Model, _update: Update) {
    _model.ball.speed += _model.gravity;
    _model.ball.position += _model.ball.speed;

    let width = _app.main_window().rect().w();
    let height = _app.main_window().rect().h();

    if -width / 2.0 + _model.ball.radius > _model.ball.position.x || _model.ball.position.x > width / 2.0 - _model.ball.radius {
        _model.ball.speed.x *= -0.8;
    }
    if -height / 2.0 + _model.ball.radius > _model.ball.position.y || _model.ball.position.y > height / 2.0 - _model.ball.radius {
        _model.ball.speed.y *= -0.8;
    }
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    draw.ellipse().color(WHITE)
        .radius(_model.ball.radius)
        .x(_model.ball.position.x)
        .y(_model.ball.position.y);
    draw.to_frame(app, &frame).unwrap();
}
