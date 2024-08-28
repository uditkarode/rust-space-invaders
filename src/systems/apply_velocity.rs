use bevy_ecs::prelude::*;

use crate::components::{position::Position, velocity::Velocity};

pub fn apply_velocity(mut query: Query<(&Velocity, &mut Position)>) {
    for (velocity, mut position) in query.iter_mut() {
        position.x += velocity.x;
        position.y += velocity.y;
    }
}
