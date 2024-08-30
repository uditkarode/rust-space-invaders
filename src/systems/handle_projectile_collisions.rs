use crate::components::{
    collision_shape::CollisionShape,
    identifiers::{Enemy, Projectile},
    position::Position,
};
use bevy_ecs::prelude::*;

pub fn handle_projectile_collisions(
    mut commands: Commands,
    projectile_query: Query<(Entity, &Position, &CollisionShape), With<Projectile>>,
    enemy_query: Query<(Entity, &Position, &CollisionShape), With<Enemy>>,
) {
    for (projectile_entity, projectile_pos, projectile_shape) in projectile_query.iter() {
        for (enemy_entity, enemy_pos, enemy_shape) in enemy_query.iter() {
            if check_collision(projectile_pos, projectile_shape, enemy_pos, enemy_shape) {
                // Destroy both the projectile and the enemy
                commands.entity(projectile_entity).despawn();
                commands.entity(enemy_entity).despawn();
            }
        }
    }
}

fn check_collision(
    pos1: &Position,
    shape1: &CollisionShape,
    pos2: &Position,
    shape2: &CollisionShape,
) -> bool {
    match (shape1, shape2) {
        (CollisionShape::Rectangle(w1, h1), CollisionShape::Rectangle(w2, h2)) => {
            pos1.x < pos2.x + w2
                && pos1.x + w1 > pos2.x
                && pos1.y < pos2.y + h2
                && pos1.y + h1 > pos2.y
        }
        (CollisionShape::Circle(r1), CollisionShape::Circle(r2)) => {
            let dx = pos1.x - pos2.x;
            let dy = pos1.y - pos2.y;
            let distance_squared = dx * dx + dy * dy;
            let radius_sum = r1 + r2;
            distance_squared <= radius_sum * radius_sum
        }
        (CollisionShape::Rectangle(w, h), CollisionShape::Circle(r))
        | (CollisionShape::Circle(r), CollisionShape::Rectangle(w, h)) => {
            let (rect_pos, _, _, circle_pos, circle_r) =
                if let CollisionShape::Rectangle(_, _) = shape1 {
                    (pos1, w, h, pos2, r)
                } else {
                    (pos2, w, h, pos1, r)
                };

            let closest_x = rect_pos
                .x
                .clamp(circle_pos.x - circle_r, circle_pos.x + circle_r);
            let closest_y = rect_pos
                .y
                .clamp(circle_pos.y - circle_r, circle_pos.y + circle_r);

            let dx = circle_pos.x - closest_x;
            let dy = circle_pos.y - closest_y;
            let distance_squared = dx * dx + dy * dy;

            distance_squared <= circle_r * circle_r
        }
    }
}
