use bevy_ecs::prelude::*;

use crate::{
    components::{
        bounciness::Bounciness, collision_shape::CollisionShape, position::Position,
        velocity::Velocity,
    },
    constants::GROUND_DRAG_FACTOR,
    resources::window_size::WindowSize,
};

pub fn process_collisions(
    mut query: Query<(&mut Position, &mut Velocity, &CollisionShape, &Bounciness)>,
    window_size: Res<WindowSize>,
) {
    for (mut position, mut velocity, collision_shape, bounciness) in query.iter_mut() {
        match collision_shape {
            CollisionShape::Circle(radius) => {
                let diameter = 2.0 * radius;
                let on_ground = position.y + diameter >= window_size.height;

                let on_x_collision = |velocity: &mut Velocity| {
                    velocity.x = -velocity.x * bounciness.0;
                };

                let on_y_collision = |velocity: &mut Velocity| {
                    velocity.y = -velocity.y * bounciness.0;
                    if on_ground && velocity.y.abs() <= 1.0 {
                        velocity.x -= velocity.x * GROUND_DRAG_FACTOR;
                    }
                };

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
