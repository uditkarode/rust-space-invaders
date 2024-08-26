use std::collections::HashMap;

use raylib::prelude::*;
use raylib::{color::Color, ffi::KeyboardKey};

use crate::engine::{
    game_object::{CollisionShape, GameObject, GameObjectCommon},
    types::XYPair,
    utils::generic::{hex_to_color, TextureDrawer},
};

const KB_X_BOOST: f32 = 0.2;
const KB_Y_BOOST: f32 = 16.0;

pub struct Ball {
    radius: f32,
    diameter: f32,
    color: Color,

    common: GameObjectCommon,
}

impl Ball {
    pub fn new(coords: XYPair, radius: f32, color_hex: &str) -> Self {
        let diameter = radius * 2.0;
        let color = hex_to_color(color_hex);
        let interested_keys = vec![KeyboardKey::KEY_A, KeyboardKey::KEY_D, KeyboardKey::KEY_W];

        let common = GameObjectCommon {
            coords,
            interested_keys,
            ..GameObjectCommon::default()
        };

        Self {
            color,
            radius,
            diameter,

            common,
        }
    }
}

impl GameObject for Ball {
    fn canvas_size(&self) -> XYPair {
        XYPair {
            x: self.diameter,
            y: self.diameter,
        }
    }

    fn weight_factor(&self) -> f32 {
        0.8
    }

    fn bounciness(&self) -> f32 {
        0.6
    }

    fn collision_shape(&self) -> CollisionShape {
        CollisionShape::Circle(self.radius)
    }

    fn common(&mut self) -> &mut GameObjectCommon {
        &mut self.common
    }

    fn draw(&self, d: &mut TextureDrawer) {
        d.draw_circle(
            self.radius as i32,
            self.radius as i32,
            self.radius,
            self.color,
        );
    }

    fn handle_input(&mut self, keys: HashMap<KeyboardKey, bool>) {
        if let Some(true) = keys.get(&KeyboardKey::KEY_A) {
            self.common.velocities.x -= KB_X_BOOST;
        }

        if let Some(true) = keys.get(&KeyboardKey::KEY_D) {
            self.common.velocities.x += KB_X_BOOST;
        }

        // jump if we are on the ground AND have 0 or lesser y velocity
        if let Some(true) = keys.get(&KeyboardKey::KEY_W) {
            if let Some(info) = &self.common.object_info {
                if self.common.velocities.y < 0.0
                    && self.common.coords.y + self.diameter == info.window_size.height as f32
                {
                    self.common.velocities.y -= KB_Y_BOOST;
                }
            }
        }
    }
}
