use bevy_ecs::prelude::*;
use raylib::{ffi::KeyboardKey, RaylibHandle};

use crate::{components::velocity::Velocity, resources::window_size::WindowSize};

const KB_X_BOOST: f32 = 0.1;

pub fn handle_input(world: &mut World, _window_size: &WindowSize, rl: &RaylibHandle) {
    let mut query = world.query::<&mut Velocity>();

    for mut velocity in query.iter_mut(world) {
        if rl.is_key_down(KeyboardKey::KEY_A) {
            velocity.x -= KB_X_BOOST;
        }

        if rl.is_key_down(KeyboardKey::KEY_D) {
            velocity.x += KB_X_BOOST;
        }
    }
}
