use legion::*;
use raylib::{ffi::KeyboardKey, RaylibHandle};

use crate::{
    components::{collision_shape::CollisionShape, position::Position, velocity::Velocity},
    resources::window_size::WindowSize,
};

const KB_X_BOOST: f32 = 0.2;
const KB_Y_BOOST: f32 = 32.0;

pub fn move_ball(world: &mut World, window_size: &WindowSize, rl: &RaylibHandle) {
    let mut query = <(&mut Position, &mut Velocity, &CollisionShape)>::query();

    for (position, velocity, collision_shape) in query.iter_mut(world) {
        match collision_shape {
            CollisionShape::Circle(radius) => {
                if rl.is_key_down(KeyboardKey::KEY_A) {
                    velocity.x -= KB_X_BOOST;
                }

                if rl.is_key_down(KeyboardKey::KEY_D) {
                    velocity.x += KB_X_BOOST;
                }

                // jump if we are on the ground AND have 0 or lesser y velocity
                if rl.is_key_down(KeyboardKey::KEY_W) {
                    if velocity.y < 0.0 && position.y + (radius * 2.0) == window_size.height {
                        velocity.y -= KB_Y_BOOST;
                    }
                }
            }
        }
    }
}
