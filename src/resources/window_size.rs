use bevy_ecs::system::Resource;

#[derive(Clone, Resource)]
pub struct WindowSize {
    pub height: f32,
    pub width: f32,
}
