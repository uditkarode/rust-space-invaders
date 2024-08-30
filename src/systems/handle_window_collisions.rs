use bevy_ecs::prelude::*;

use crate::{
    components::{
        collision_shape::CollisionShape,
        identifiers::{Player, Projectile},
        position::Position,
        velocity::Velocity,
    },
    resources::window_size::WindowSize,
};

const COLLISION_VELOCITY_FACTOR: f32 = 0.8;

pub fn handle_window_collisions(
    mut query: Query<
        (
            &mut Position,
            &mut Velocity,
            &CollisionShape,
            Option<&Player>,
        ),
        Without<Projectile>,
    >,
    window_size: Res<WindowSize>,
) {
    for (mut position, mut velocity, collision_shape, player) in query.iter_mut() {
        let on_x_collision = |velocity: &mut Velocity| {
            velocity.x = -velocity.x;

            if let Some(_) = player {
                velocity.x *= COLLISION_VELOCITY_FACTOR;
            }
        };

        let on_y_collision = |velocity: &mut Velocity| {
            velocity.y = -velocity.y;
        };

        match collision_shape {
            CollisionShape::Rectangle(width, height) => {
                // x axis window collision
                if position.x <= 0.0 {
                    position.x = 0.0;
                    on_x_collision(&mut velocity);
                }
                if position.x + width > window_size.width {
                    position.x = window_size.width - width;
                    on_x_collision(&mut velocity);
                }

                // y axis window collision
                if position.y < 0.0 {
                    position.y = 0.0;
                    on_y_collision(&mut velocity);
                }
                if position.y + height > window_size.height {
                    position.y = window_size.height - height;
                    on_y_collision(&mut velocity);
                }
            }

            CollisionShape::Circle(radius) => {
                let diameter = 2.0 * radius;

                // x axis window collision
                if position.x <= 0.0 {
                    position.x = 0.0;
                    on_x_collision(&mut velocity);
                }
                if position.x + diameter > window_size.width {
                    position.x = window_size.width - diameter;
                    on_x_collision(&mut velocity);
                }

                // y axis window collision
                if position.y < 0.0 {
                    position.y = 0.0;
                    on_y_collision(&mut velocity);
                }
                if position.y + diameter > window_size.height {
                    position.y = window_size.height - diameter;
                    on_y_collision(&mut velocity);
                }
            }
        }
    }
}
