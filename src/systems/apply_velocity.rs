use legion::system;

use crate::components::{position::Position, velocity::Velocity};

#[system(for_each)]
pub fn apply_velocity(velocity: &Velocity, position: &mut Position) {
    position.x += velocity.x;
    position.y += velocity.y;
}
