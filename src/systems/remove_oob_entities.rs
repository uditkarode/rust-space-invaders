use crate::{
    components::{collision_shape::CollisionShape, drawable::Drawable, position::Position},
    resources::window_size::WindowSize,
};
use bevy_ecs::prelude::*;

pub fn remove_oob_entities(
    mut commands: Commands,
    query: Query<(Entity, &Position, &Drawable), Without<CollisionShape>>,
    window_size: Res<WindowSize>,
) {
    for (entity, position, drawable) in query.iter() {
        let canvas_size = drawable.canvas_size;

        let outside_x = position.x < 0.0 || position.x + canvas_size.x > window_size.width;
        let outside_y = position.y < 0.0 || position.y + canvas_size.y > window_size.height;

        if outside_x || outside_y {
            commands.entity(entity).despawn();
        }
    }
}
