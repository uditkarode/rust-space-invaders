use bevy_ecs::world::World;
use raylib::prelude::*;
use raylib::{color::Color, math::Vector2};

use crate::utils::generic::TextureDrawer;

const SHIP_HEIGHT: i32 = 30;
const SHIP_WIDTH: i32 = 80;

const SHOOTER_HEIGHT: i32 = 20;
const SHOOTER_WIDTH: i32 = 10;

pub fn enemy_canvas_size() -> Vector2 {
    return Vector2::new(SHIP_WIDTH as f32, (SHIP_HEIGHT + SHOOTER_HEIGHT) as f32);
}

pub fn draw_enemy(_world: &mut World, d: &mut TextureDrawer) {
    // ship hull
    d.draw_rectangle(0, SHOOTER_HEIGHT, SHIP_WIDTH, SHIP_HEIGHT, Color::RED);

    // shooter
    d.draw_rectangle(
        (SHIP_WIDTH / 2) - (SHOOTER_WIDTH / 2), // x position
        0,                                      // y position
        SHOOTER_WIDTH,                          // width
        SHOOTER_HEIGHT,                         // height
        Color::RED,
    );
}
