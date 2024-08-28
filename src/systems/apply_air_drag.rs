use bevy_ecs::prelude::*;

use crate::{
    components::velocity::Velocity,
    constants::{AIR_RESISTANCE_FACTOR, DT},
};

pub fn apply_air_drag(mut query: Query<&mut Velocity>) {
    for mut velocity in query.iter_mut() {
        velocity.x *= 1.0 - (AIR_RESISTANCE_FACTOR * DT);
        velocity.y *= 1.0 - (AIR_RESISTANCE_FACTOR * DT);
    }
}
