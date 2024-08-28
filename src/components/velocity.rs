use bevy_ecs::component::Component;

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}
