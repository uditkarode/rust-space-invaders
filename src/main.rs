use components::{drawable::Drawable, position::Position};
use raylib::prelude::*;
use resources::window_size::WindowSize;
use systems::*;

use bevy_ecs::prelude::*;
use raylib::ffi::{KeyboardKey, TraceLogLevel};
use utils::generic::{hex_to_color, TextureDrawer};

mod constants;
mod utils;

mod components;
mod resources;
mod systems;

fn main() -> Result<(), anyhow::Error> {
    let mut world = World::default();

    let window_size = WindowSize {
        width: 1280.0,
        height: 720.0,
    };
    world.insert_resource(window_size.clone());

    let (mut rl, thread) = raylib::init()
        .size(window_size.width as i32, window_size.height as i32)
        .title("Bouncy Ball")
        .build();

    rl.set_target_fps(120);
    rl.set_exit_key(Some(KeyboardKey::KEY_ESCAPE));
    rl.set_trace_log(TraceLogLevel::LOG_NONE);

    let mut schedule = Schedule::default();

    schedule.add_systems(
        (
            (apply_gravity::apply_gravity, apply_air_drag::apply_air_drag),
            apply_velocity::apply_velocity,
            process_collisions::process_collisions,
        )
            .chain(),
    );

    // add our ball
    world.spawn((
        components::position::Position {
            x: window_size.width / 4.0,
            y: 480.0,
        },
        components::weight::Weight(2.0),
        components::bounciness::Bounciness(0.6),
        components::collision_shape::CollisionShape::Circle(20.0),
        components::drawable::Drawable {
            canvas_size: Vector2::new(40.0, 40.0),
            draw: Box::new(|d: &mut TextureDrawer| {
                d.draw_circle(20, 20, 20.0, hex_to_color("#cf5353"));
            }),
        },
        components::velocity::Velocity { x: 0.0, y: 0.0 },
    ));

    while !rl.window_should_close() {
        move_ball::move_ball(&mut world, &window_size, &rl);
        schedule.run(&mut world);

        let mut textures = Vec::new();

        for (drawable, position) in world.query::<(&Drawable, &Position)>().iter(&world) {
            let cs = drawable.canvas_size;
            let mut render_texture = rl
                .load_render_texture(&thread, cs.x as u32, cs.y as u32)
                .unwrap();

            {
                let mut rl_ref = &mut rl;
                let mut d = rl_ref.begin_texture_mode(&thread, &mut render_texture);
                (drawable.draw)(&mut d);
            }

            textures.push((render_texture, position));
        }

        let mut d = rl.begin_drawing(&thread);
        for (_, (texture, pos)) in textures.iter().enumerate() {
            d.draw_texture(texture.texture(), pos.x as i32, pos.y as i32, Color::WHITE);
        }
        d.clear_background(Color::BLACK);
    }

    Ok(())
}
