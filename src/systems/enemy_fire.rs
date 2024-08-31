use bevy_ecs::prelude::*;
use std::time::Duration;

use crate::{
    components::{
        collision_shape::CollisionShape,
        drawable::{Drawable, DrawableKind},
        identifiers::{Enemy, EnemyProjectile},
        position::Position,
        velocity::Velocity,
    },
    drawables::{
        enemy::*,
        projectile::{projectile_canvas_size, PROJECTILE_RADIUS},
    },
    resources::elapsed_time::ElapsedTime,
};

const FIRST_SHOT_DELAY: Duration = Duration::from_millis(250);
const SHOT_INTERVAL: Duration = Duration::from_millis(750);

pub fn enemy_fire(
    mut commands: Commands,
    time: Res<ElapsedTime>,
    enemy_query: Query<&Position, With<Enemy>>,
    mut last_shot: Local<Duration>,
) {
    if time.0.saturating_sub(*last_shot) >= SHOT_INTERVAL {
        if *last_shot == Duration::ZERO {
            if time.0 < FIRST_SHOT_DELAY {
                return;
            }
        }

        for enemy_pos in enemy_query.iter() {
            let projectile_size = projectile_canvas_size();

            commands.spawn((
                EnemyProjectile,
                Position {
                    x: enemy_pos.x + (ENEMY_SHIP_WIDTH as f32 / 2.0)
                        - (ENEMY_SHOOTER_WIDTH as f32 / 2.0),
                    y: enemy_pos.y + ENEMY_SHIP_HEIGHT as f32 + ENEMY_SHOOTER_HEIGHT as f32,
                },
                Velocity { x: 0.0, y: 5.0 },
                Drawable {
                    canvas_size: projectile_size,
                    kind: DrawableKind::EnemyProjectile,
                },
                CollisionShape::Circle(PROJECTILE_RADIUS),
            ));
        }

        *last_shot = time.0;
    }
}
