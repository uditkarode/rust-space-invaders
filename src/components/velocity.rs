use bevy_ecs::component::Component;

#[derive(Component, Default)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}
