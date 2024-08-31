use bevy_ecs::world::World;
use raylib::{color::Color, math::Vector2, prelude::RaylibDraw};

use crate::utils::generic::TextureDrawer;

pub const PROJECTILE_RADIUS: f32 = 5.0;

pub fn projectile_canvas_size() -> Vector2 {
    return Vector2::new(PROJECTILE_RADIUS * 2.0, PROJECTILE_RADIUS * 2.0);
}

pub fn draw_projectile(_world: &mut World, d: &mut TextureDrawer, is_enemy: bool) {
    let color = if is_enemy { Color::ORANGE } else { Color::LIME };

    d.draw_circle(
        PROJECTILE_RADIUS as i32, // x pos (centered on the canvas)
        PROJECTILE_RADIUS as i32, // y pos (centered on the canvas)
        PROJECTILE_RADIUS,
        color,
    );
}
