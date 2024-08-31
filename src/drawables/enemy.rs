use bevy_ecs::world::World;
use raylib::prelude::*;
use raylib::{color::Color, math::Vector2};

use crate::utils::generic::TextureDrawer;

pub const ENEMY_SHIP_HEIGHT: i32 = 30;
pub const ENEMY_SHIP_WIDTH: i32 = 80;

pub const ENEMY_SHOOTER_HEIGHT: i32 = 20;
pub const ENEMY_SHOOTER_WIDTH: i32 = 10;

pub fn enemy_canvas_size() -> Vector2 {
    return Vector2::new(
        ENEMY_SHIP_WIDTH as f32,
        (ENEMY_SHIP_HEIGHT + ENEMY_SHOOTER_HEIGHT) as f32,
    );
}

pub fn draw_enemy(_world: &mut World, d: &mut TextureDrawer) {
    // ship hull
    d.draw_rectangle(
        0,
        ENEMY_SHOOTER_HEIGHT,
        ENEMY_SHIP_WIDTH,
        ENEMY_SHIP_HEIGHT,
        Color::RED,
    );

    // shooter
    d.draw_rectangle(
        (ENEMY_SHIP_WIDTH / 2) - (ENEMY_SHOOTER_WIDTH / 2), // x position
        0,                                                  // y position
        ENEMY_SHOOTER_WIDTH,                                // width
        ENEMY_SHOOTER_HEIGHT,                               // height
        Color::RED,
    );
}
