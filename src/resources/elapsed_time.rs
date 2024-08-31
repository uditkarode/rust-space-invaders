use bevy_ecs::system::Resource;
use std::time::Duration;

#[derive(Resource)]
pub struct ElapsedTime(pub Duration);

impl Default for ElapsedTime {
    fn default() -> Self {
        ElapsedTime(Duration::from_secs(0))
    }
}
