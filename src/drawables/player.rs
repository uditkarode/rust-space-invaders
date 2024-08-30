use bevy_ecs::world::World;
use raylib::prelude::*;
use raylib::{color::Color, math::Vector2};

use crate::components::identifiers::Player;
use crate::components::velocity::Velocity;
use crate::utils::generic::TextureDrawer;

const SHIP_HEIGHT: i32 = 30;
const SHIP_WIDTH: i32 = 80;

const SHOOTER_HEIGHT: i32 = 20;
const SHOOTER_WIDTH: i32 = 10;

const ROTATION_SCALING_FACTOR: f32 = 3.0;
const ROTATION_BUFFER: i32 = 8;

pub fn player_canvas_size() -> Vector2 {
    return Vector2::new(SHIP_WIDTH as f32, (SHIP_HEIGHT + SHOOTER_HEIGHT) as f32);
}

pub fn draw_player(world: &mut World, d: &mut TextureDrawer) {
    // ship hull
    d.draw_rectangle(0, 0, SHIP_WIDTH, SHIP_HEIGHT, Color::STEELBLUE);

    let mut player_query = world.query::<(&Player, &Velocity)>();
    let player_fields = player_query.get_single(world);

    let mut rotation = 0.0;
    if let Ok((_, velocity)) = player_fields {
        rotation = velocity.x * ROTATION_SCALING_FACTOR;
    }

    // shooter
    d.draw_rectangle_pro(
        Rectangle::new(
            ((SHIP_WIDTH / 2) - (SHOOTER_WIDTH / 2)) as f32, // x position
            (SHIP_HEIGHT - ROTATION_BUFFER) as f32,          // y position
            SHOOTER_WIDTH as f32,                            // width
            (SHOOTER_HEIGHT + ROTATION_BUFFER) as f32,       // height
        ),
        Vector2::new(0.0, 0.0),      // origin
        rotation.clamp(-20.0, 20.0), // rotation
        Color::STEELBLUE,
    );
}
