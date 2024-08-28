use bevy_ecs::component::Component;
use raylib::math::Vector2;

use crate::utils::generic::TextureDrawer;

#[derive(Component)]
pub struct Drawable {
    pub canvas_size: Vector2,
    pub draw: Box<dyn Fn(&mut TextureDrawer) + Send + Sync>,
}
