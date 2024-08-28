use bevy_ecs::prelude::*;

use crate::{
    components::{velocity::Velocity, weight::Weight},
    constants::{DT, GRAVITY},
};

pub fn apply_gravity(mut query: Query<(&mut Velocity, &Weight)>) {
    for (mut velocity, weight) in query.iter_mut() {
        velocity.y += weight.0 * GRAVITY * DT;
    }
}
