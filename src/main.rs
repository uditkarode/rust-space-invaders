use engine::{
    core::Engine,
    types::{WindowSize, XYPair},
};
use objects::ball::Ball;

mod engine;
mod objects;

fn main() -> Result<(), anyhow::Error> {
    let window_size = WindowSize {
        width: 1280,
        height: 720,
    };
    let mut engine = Engine::new(&window_size)?;

    let radius = 24.0;
    let ball_coords = XYPair {
        x: (&window_size.width / 2) as f64 - radius,
        y: (&window_size.height / 2) as f64 - radius,
    };

    let ball = Ball::new(ball_coords, radius, "#cf5353");

    engine.add_game_object(ball);

    engine.run("Bouncy Ball")
}
