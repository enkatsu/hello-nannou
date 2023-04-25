mod bound_ball;
mod bound_ball_vector;
mod bound_balls;
mod interaction_bound_balls;

fn main() {
    nannou::app(bound_ball::model).update(bound_ball::update).run();
    // nannou::app(bound_ball_vector::model).update(bound_ball_vector::update).run();
    // nannou::app(bound_balls::model).update(bound_balls::update).run();
    // nannou::app(interaction_bound_balls::model).update(interaction_bound_balls::update).run();
}
