use bevy_ecs::component::Component;

#[derive(Component, Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}
