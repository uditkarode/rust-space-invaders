use legion::system;

use crate::{
    components::velocity::Velocity,
    constants::{AIR_RESISTANCE_FACTOR, DT},
};

#[system(for_each)]
pub fn apply_air_drag(velocity: &mut Velocity) {
    velocity.x *= 1.0 - (AIR_RESISTANCE_FACTOR * DT);
    velocity.y *= 1.0 - (AIR_RESISTANCE_FACTOR * DT);
}
