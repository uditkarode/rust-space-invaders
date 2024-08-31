use bevy_ecs::system::Resource;

#[derive(Clone, Resource)]
pub struct Score(pub u8);
