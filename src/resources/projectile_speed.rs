use bevy_ecs::system::Resource;

#[derive(Resource)]
pub struct ProjectileSpeed(pub f32);

impl Default for ProjectileSpeed {
    fn default() -> Self {
        ProjectileSpeed(5.0) // Initial speed
    }
}
