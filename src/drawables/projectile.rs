use bevy_ecs::world::World;
use raylib::{color::Color, math::Vector2, prelude::RaylibDraw};

use crate::utils::generic::TextureDrawer;

const PROJECTILE_RADIUS: f32 = 5.0;

pub fn projectile_canvas_size() -> Vector2 {
    return Vector2::new(PROJECTILE_RADIUS * 2.0, PROJECTILE_RADIUS * 2.0);
}

pub fn draw_projectile(_world: &mut World, d: &mut TextureDrawer) {
    d.draw_circle(
        PROJECTILE_RADIUS as i32, // x pos (centered on the canvas)
        PROJECTILE_RADIUS as i32, // y pos (centered on the canvas)
        PROJECTILE_RADIUS,
        Color::ORANGE,
    );
}
