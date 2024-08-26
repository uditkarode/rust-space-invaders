use sdl2::{
    keyboard::{KeyboardState, Scancode},
    pixels::PixelFormatEnum,
    render::{Texture, TextureCreator},
    video::WindowContext,
};

use crate::engine::{
    game_object::{CollisionShape, GameObject, GameObjectCommon},
    types::XYPair,
};

const KB_X_BOOST: f64 = 0.2;
const KB_Y_BOOST: f64 = 16.0;

pub struct Ball {
    radius: f64,
    diameter: f64,
    color: u32,

    common: GameObjectCommon,
}

impl Ball {
    pub fn new(coords: XYPair, radius: f64, color_hex: &str) -> Self {
        let diameter = radius * 2.0;
        let color = u32::from_str_radix(&color_hex[1..], 16).unwrap_or(0xFFFFFF);

        let common = GameObjectCommon {
            coords,
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
    fn weight_factor(&self) -> f64 {
        0.8
    }

    fn bounciness(&self) -> f64 {
        0.6
    }

    fn collision_shape(&self) -> CollisionShape {
        CollisionShape::Circle(self.radius)
    }

    fn common(&mut self) -> &mut GameObjectCommon {
        &mut self.common
    }

    fn draw<'a>(&'a self, texture_creator: &'a TextureCreator<WindowContext>) -> Texture {
        let mut texture = texture_creator
            .create_texture_streaming(
                PixelFormatEnum::RGBA8888,
                self.diameter as u32,
                self.diameter as u32,
            )
            .unwrap();

        // Lock the texture to gain access to its pixel buffer
        texture
            .with_lock(None, |buffer: &mut [u8], pitch: usize| {
                let h = self.radius as f64;
                let k = self.radius as f64;

                for y in 0..self.diameter as usize {
                    for x in 0..self.diameter as usize {
                        let dx = (x as f64 - h).abs();
                        let dy = (y as f64 - k).abs();
                        if (dx * dx + dy * dy).sqrt() <= self.radius {
                            let offset = y * pitch + x * 4;
                            buffer[offset] = 255; // R
                            buffer[offset + 1] = 0; // G
                            buffer[offset + 2] = 0; // B
                            buffer[offset + 3] = 255; // A
                        }
                    }
                }
            })
            .unwrap();

        texture
    }

    fn handle_input(&mut self, keys: &KeyboardState) {
        if keys.is_scancode_pressed(Scancode::W) {
            self.common.velocities.x -= KB_X_BOOST;
        }

        if keys.is_scancode_pressed(Scancode::D) {
            self.common.velocities.x += KB_X_BOOST;
        }

        // jump if we are on the ground AND have 0 or lesser y velocity
        if keys.is_scancode_pressed(Scancode::W) {
            if let Some(info) = &self.common.object_info {
                if self.common.velocities.y < 0.0
                    && self.common.coords.y + self.diameter == info.window_size.height as f64
                {
                    self.common.velocities.y -= KB_Y_BOOST;
                }
            }
        }
    }
}
