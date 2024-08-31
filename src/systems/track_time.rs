use bevy_ecs::prelude::*;
use raylib::RaylibHandle;
use std::time::Duration;

use crate::resources::elapsed_time::ElapsedTime;

pub fn track_time(world: &mut World, rl: &RaylibHandle) {
    let frame_time = rl.get_frame_time();
    let frame_duration = Duration::from_secs_f32(frame_time);

    let mut elapsed_time = world.resource_mut::<ElapsedTime>();
    elapsed_time.0 += frame_duration;
}
