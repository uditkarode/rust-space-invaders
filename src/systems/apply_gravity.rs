use legion::system;

use crate::{
    components::{velocity::Velocity, weight::Weight},
    constants::{DT, GRAVITY},
};

#[system(for_each)]
pub fn apply_gravity(velocity: &mut Velocity, weight: &Weight) {
    velocity.y += weight.0 * GRAVITY * DT;
}
