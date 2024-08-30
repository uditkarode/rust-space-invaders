use bevy_ecs::prelude::*;
use raylib::{ffi::KeyboardKey, math::Vector2, RaylibHandle};

use crate::{
    components::{
        collision_shape::CollisionShape,
        drawable::{Drawable, DrawableKind},
        identifiers::{self, Player},
        position::Position,
        velocity::Velocity,
    },
    drawables::{
        player::player_canvas_size,
        projectile::{projectile_canvas_size, PROJECTILE_RADIUS},
    },
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

            let projectile_x_velocity = (velocity.x * -1.0).clamp(-2.0, 2.0);

            // spawn projectile
            world.spawn((
                identifiers::Projectile,
                Position {
                    x: position.x + pc.x / 2.0,
                    y: position.y,
                },
                Velocity {
                    x: projectile_x_velocity,
                    y: -4.0,
                },
                Drawable {
                    canvas_size: projectile_canvas_size(),
                    kind: DrawableKind::Projectile,
                },
                CollisionShape::Circle(PROJECTILE_RADIUS),
            ));
        }
    }
}
