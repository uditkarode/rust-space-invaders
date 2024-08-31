use crate::resources::elapsed_time::ElapsedTime;
use crate::resources::score::Score;
use crate::{
    components::{
        collision_shape::CollisionShape,
        identifiers::{Enemy, EnemyProjectile, Player, Projectile},
        position::Position,
    },
    resources::projectile_speed::ProjectileSpeed,
};
use bevy_ecs::prelude::*;

pub fn handle_projectile_collisions(
    mut commands: Commands,
    projectile_query: Query<(Entity, &Position, &CollisionShape), With<Projectile>>,
    enemy_query: Query<(Entity, &Position, &CollisionShape), With<Enemy>>,
    player_query: Query<(Entity, &Position, &CollisionShape), With<Player>>,
    enemy_projectile_query: Query<(Entity, &Position, &CollisionShape), With<EnemyProjectile>>,
    elapsed_time: Res<ElapsedTime>,
    mut enemy_projectile_speed: ResMut<ProjectileSpeed>,
    mut score: ResMut<Score>,
) {
    // Handle player projectiles hitting enemies
    for (projectile_entity, projectile_pos, projectile_shape) in projectile_query.iter() {
        for (enemy_entity, enemy_pos, enemy_shape) in enemy_query.iter() {
            if check_collision(projectile_pos, projectile_shape, enemy_pos, enemy_shape) {
                // Destroy both the projectile and the enemy
                commands.entity(projectile_entity).despawn();
                commands.entity(enemy_entity).despawn();

                // Increase the score
                score.0 += 1;

                // Increase the projectile speed
                enemy_projectile_speed.0 += 2.0;
            }
        }
    }

    // Handle enemy projectiles hitting the player
    if let Ok((player_entity, player_pos, player_shape)) = player_query.get_single() {
        for (projectile_entity, projectile_pos, projectile_shape) in enemy_projectile_query.iter() {
            if check_collision(player_pos, player_shape, projectile_pos, projectile_shape) {
                commands.entity(player_entity).despawn();
                commands.entity(projectile_entity).despawn();
                println!("\n\n ------ Game Over ------");
                println!(" Survived: {:.2}s", elapsed_time.0.as_secs_f32());
                println!(" Score: {}", score.0);
                println!(" -----------------------\n");
                std::process::exit(0);
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
            let center1_x = pos1.x + r1;
            let center1_y = pos1.y + r1;
            let center2_x = pos2.x + r2;
            let center2_y = pos2.y + r2;
            let dx = center1_x - center2_x;
            let dy = center1_y - center2_y;
            let distance_squared = dx * dx + dy * dy;
            let radius_sum = r1 + r2;
            distance_squared <= radius_sum * radius_sum
        }
        (CollisionShape::Rectangle(w, h), CollisionShape::Circle(r))
        | (CollisionShape::Circle(r), CollisionShape::Rectangle(w, h)) => {
            let (rect_pos, rect_w, rect_h, circle_pos, circle_r) =
                if let CollisionShape::Rectangle(_, _) = shape1 {
                    (pos1, w, h, pos2, r)
                } else {
                    (pos2, w, h, pos1, r)
                };

            let circle_center_x = circle_pos.x + circle_r;
            let circle_center_y = circle_pos.y + circle_r;

            // Find the closest point on the rectangle to the circle's center
            let closest_x = circle_center_x.clamp(rect_pos.x, rect_pos.x + rect_w);
            let closest_y = circle_center_y.clamp(rect_pos.y, rect_pos.y + rect_h);

            // Calculate the distance between the closest point and the circle's center
            let dx = circle_center_x - closest_x;
            let dy = circle_center_y - closest_y;
            let distance_squared = dx * dx + dy * dy;

            // Check if the distance is less than or equal to the circle's radius
            distance_squared <= circle_r * circle_r
        }
    }
}
