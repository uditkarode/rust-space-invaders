use sdl2::{
    keyboard::KeyboardState,
    render::{Texture, TextureCreator},
    video::WindowContext,
};

use super::{
    constants::DEFAULT_COLLISION_DAMPING_FACTOR,
    types::{ObjectInfo, XYPair},
};

// collision shape
pub enum CollisionShape {
    Circle(f64),
}

// game object common
#[derive(Default, Clone)]
pub struct GameObjectCommon {
    pub coords: XYPair,
    pub velocities: XYPair,
    pub object_info: Option<ObjectInfo>,
}

// game object
pub trait GameObject {
    fn common(&mut self) -> &mut GameObjectCommon;

    fn weight_factor(&self) -> f64;

    fn bounciness(&self) -> f64 {
        DEFAULT_COLLISION_DAMPING_FACTOR
    }

    fn collision_shape(&self) -> CollisionShape;

    fn draw<'a>(&'a self, texture_creator: &'a TextureCreator<WindowContext>) -> Texture;

    fn handle_input(&mut self, _keyboard_state: &KeyboardState) {}
}
