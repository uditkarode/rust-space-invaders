use bevy_ecs::prelude::*;
use rand::Rng;

use crate::{
    components::{
        collision_shape::CollisionShape,
        drawable::{Drawable, DrawableKind},
        identifiers::Enemy,
        position::Position,
        velocity::Velocity,
    },
    drawables::enemy::enemy_canvas_size,
    resources::window_size::WindowSize,
};

const EDGE_BUFFER: f32 = 4.0;

pub fn spawn_enemy(
    mut commands: Commands,
    enemy_query: Query<&Enemy>,
    window_size: Res<WindowSize>,
) {
    if enemy_query.is_empty() {
        let enemy_size = enemy_canvas_size();
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(EDGE_BUFFER..window_size.width - enemy_size.x - EDGE_BUFFER);

        commands.spawn((
            Enemy,
            Position { x, y: 50.0 },
            CollisionShape::Rectangle(enemy_size.x, enemy_size.y),
            Drawable {
                canvas_size: enemy_size,
                kind: DrawableKind::Enemy,
            },
            Velocity::default(),
        ));
    }
}
