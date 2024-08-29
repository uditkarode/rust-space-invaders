use bevy_ecs::component::Component;
use raylib::math::Vector2;

#[derive(Clone)]
pub enum DrawableKind {
    Player,
    Enemy,
}

#[derive(Component, Clone)]
pub struct Drawable {
    pub canvas_size: Vector2,
    pub kind: DrawableKind,
}
