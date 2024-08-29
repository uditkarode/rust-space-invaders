use bevy_ecs::world::World;
use raylib::{color::Color, math::Vector2, prelude::RaylibDraw};

use crate::utils::generic::TextureDrawer;

const SHIP_HEIGHT: i32 = 30;
const SHIP_WIDTH: i32 = 80;

const SHOOTER_HEIGHT: i32 = 20;
const SHOOTER_WIDTH: i32 = 10;

pub fn player_canvas_size() -> Vector2 {
    return Vector2::new(SHIP_WIDTH as f32, (SHIP_HEIGHT * 2) as f32);
}

pub fn draw_player(_world: &mut World, d: &mut TextureDrawer) {
    // ship hull
    d.draw_rectangle(0, 0, SHIP_WIDTH, SHIP_HEIGHT, Color::STEELBLUE);

    // shooter
    d.draw_rectangle(
        (SHIP_WIDTH / 2) - (SHOOTER_WIDTH / 2),
        SHIP_HEIGHT,
        SHOOTER_WIDTH,
        SHOOTER_HEIGHT,
        Color::STEELBLUE,
    );
}
