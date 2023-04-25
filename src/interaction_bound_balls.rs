use nannou::lyon::geom::euclid::Vector2D;
use nannou::prelude::*;

struct Ball {
    position: Vec2,
    speed: Vec2,
    radius: f32,
}

pub struct Model {
    _window: window::Id,
    balls: Vec<Ball>,
    gravity: Vec2,
}

pub fn model(app: &App) -> Model {
    let _window = app.new_window().title("balls").view(view)
        .mouse_pressed(mouse_pressed)
        .mouse_moved(mouse_moved)
        .build().unwrap();
    let mut balls: Vec<Ball> = Vec::new();
    for _ in 0..10 {
        let ball = Ball {
            position: Vec2::new(100.0, 100.0),
            speed: Vec2::new(1.0, 1.0),
            radius: 50.0,
        };
        balls.push(ball);
    }
    Model { _window, balls, gravity: Vec2::new(0.0, -0.8), }
}

pub fn update(_app: &App, _model: &mut Model, _update: Update) {
    for ball in &mut _model.balls {
        ball.speed += _model.gravity;
        ball.position += ball.speed;

        let width = _app.main_window().rect().w();
        let height = _app.main_window().rect().h();

        if -width / 2.0 + ball.radius > ball.position.x || ball.position.x > width / 2.0 - ball.radius {
            ball.speed.x *= -0.8;
        }
        if -height / 2.0 + ball.radius > ball.position.y || ball.position.y > height / 2.0 - ball.radius {
            ball.speed.y *= -0.8;
        }
    }
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    for ball in &_model.balls {
        draw.ellipse().color(WHITE)
            .radius(ball.radius)
            .x(ball.position.x)
            .y(ball.position.y);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn mouse_pressed(app: &App, _model: &mut Model, mouse_button: MouseButton) {
    let ball = Ball {
        position: Vec2::new(app.mouse.x, app.mouse.y),
        speed: Vec2::new(random_f32(), random_f32()),
        radius: 50.0,
    };
    _model.balls.push(ball);
}

fn mouse_moved(app: &App, _model: &mut Model, point: Point2) {
    let angle = point.angle();
    let gravity = Vec2::new(angle.cos(), angle.sin()) * 0.8;
    _model.gravity = gravity;
}
