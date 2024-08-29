use bevy_ecs::prelude::*;
use raylib::{ffi::KeyboardKey, math::Vector2, RaylibHandle};

use crate::{
    components::{
        drawable::{Drawable, DrawableKind},
        identifiers::{self, Player},
        position::Position,
        velocity::Velocity,
    },
    drawables::player::player_canvas_size,
    resources::window_size::WindowSize,
};

const KB_X_BOOST: f32 = 0.1;

pub fn handle_input(world: &mut World, _window_size: &WindowSize, rl: &RaylibHandle) {
    let mut query = world.query::<(&mut Velocity, &Player)>();

    for (mut velocity, _) in query.iter_mut(world) {
        if rl.is_key_down(KeyboardKey::KEY_A) {
            velocity.x -= KB_X_BOOST;
        }

        if rl.is_key_down(KeyboardKey::KEY_D) {
            velocity.x += KB_X_BOOST;
        }
    }

    if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
        let mut player_query = world.query::<(&Player, &Velocity, &Position)>();
        if let Ok((_, velocity, position)) = player_query.get_single(world) {
            let pc = player_canvas_size();

            // spawn projectile
            world.spawn((
                identifiers::Projectile,
                Position {
                    x: position.x + pc.x / 2.0,
                    y: position.y,
                },
                Velocity {
                    x: velocity.x,
                    y: -2.0,
                },
                Drawable {
                    canvas_size: Vector2::new(10.0, 10.0),
                    kind: DrawableKind::Projectile,
                },
            ));
        }
    }
}
