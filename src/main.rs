use raylib::prelude::*;
use resources::window_size::WindowSize;
use systems::*;

use legion::*;
use raylib::ffi::{KeyboardKey, TraceLogLevel};
use utils::generic::hex_to_color;

mod constants;
mod utils;

mod components;
mod resources;
mod systems;

fn main() -> Result<(), anyhow::Error> {
    let mut world = World::default();
    let mut resources = Resources::default();

    let window_size = WindowSize {
        width: 1280.0,
        height: 720.0,
    };
    resources.insert(window_size.clone());

    let (mut rl, thread) = raylib::init()
        .size(window_size.width as i32, window_size.height as i32)
        .title("Bouncy Ball")
        .build();

    rl.set_target_fps(120);
    rl.set_exit_key(Some(KeyboardKey::KEY_ESCAPE));
    rl.set_trace_log(TraceLogLevel::LOG_NONE);

    let mut schedule = Schedule::builder()
        .add_system(apply_gravity::apply_gravity_system())
        .add_system(apply_air_drag::apply_air_drag_system())
        .flush()
        .add_system(apply_velocity::apply_velocity_system())
        .flush()
        .add_system(process_collisions::process_collisions_system())
        .build();

    let mut post_update_schedule = Schedule::builder().build();

    // add our ball
    world.push((
        components::position::Position {
            x: window_size.width / 2.0,
            y: 680.0,
        },
        components::weight::Weight(2.0),
        components::bounciness::Bounciness(0.6),
        components::collision_shape::CollisionShape::Circle(20.0),
        components::drawable::Drawable::Circle {
            radius: 20.0,
            color: hex_to_color("#cf5353"),
        },
        components::velocity::Velocity { x: 0.0, y: 0.0 },
    ));

    while !rl.window_should_close() {
        move_ball::move_ball(&mut world, &window_size, &rl);

        let mut d = rl.begin_drawing(&thread);
        draw_drawables::draw_drawables(&mut world, &mut d);

        schedule.execute(&mut world, &mut resources);
        post_update_schedule.execute(&mut world, &mut resources);
        d.clear_background(Color::BLACK);
    }

    Ok(())
}
