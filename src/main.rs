use components::{
    drawable::{self, Drawable},
    position::Position,
};
use draw::draw_drawable;
use drawables::player::player_canvas_size;
use raylib::prelude::*;
use resources::window_size::WindowSize;
use systems::*;

use bevy_ecs::prelude::*;
use raylib::ffi::{KeyboardKey, TraceLogLevel};

mod components;
mod drawables;
mod resources;
mod systems;
mod utils;

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
            apply_velocity::apply_velocity,
            process_collisions::process_collisions,
        )
            .chain(),
    );

    // spawn player
    world.spawn((
        components::position::Position {
            x: window_size.width / 4.0,
            y: 640.0,
        },
        components::collision_shape::CollisionShape::Rectangle(
            player_canvas_size().x,
            player_canvas_size().y,
        ),
        drawable::Drawable {
            canvas_size: player_canvas_size(),
            kind: drawable::DrawableKind::Player,
        },
        components::velocity::Velocity::default(),
    ));

    while !rl.window_should_close() {
        handle_input::handle_input(&mut world, &window_size, &rl);
        schedule.run(&mut world);

        let mut textures = Vec::new();
        let mut drawables = Vec::new();

        for (drawable, position) in world.query::<(&Drawable, &Position)>().iter(&world) {
            drawables.push((drawable.clone(), position.clone()));
        }

        for (drawable, position) in drawables {
            let mut render_texture = rl
                .load_render_texture(
                    &thread,
                    drawable.canvas_size.x as u32,
                    drawable.canvas_size.y as u32,
                )
                .unwrap();

            {
                let mut rl_ref = &mut rl;
                let mut d = rl_ref.begin_texture_mode(&thread, &mut render_texture);
                draw_drawable(&mut world, &drawable, &mut d);
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
