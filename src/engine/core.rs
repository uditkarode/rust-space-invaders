use std::time::{Duration, Instant};

use anyhow::anyhow;
use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::{Canvas, Texture, TextureCreator},
    video::Window,
};

use super::{
    constants::*,
    game_object::{CollisionShape, GameObject},
    types::{ObjectInfo, WindowSize, XYPair},
};

pub struct Engine {
    window_size: WindowSize,
    objects: Vec<Box<dyn GameObject>>,
}

// public functions
impl Engine {
    pub fn new(window_size: &WindowSize) -> Result<Self, anyhow::Error> {
        Ok(Self {
            window_size: window_size.clone(),
            objects: Vec::new(),
        })
    }

    pub fn add_game_object(&mut self, game_object: impl GameObject + 'static) {
        self.objects.push(Box::new(game_object))
    }
}

// internal functions
impl Engine {
    fn calc_velocities(object: &mut Box<dyn GameObject>) {
        let mut velocities = object.common().velocities.clone();

        // apply gravity
        let gravity = GRAVITY * object.weight_factor() * DT;
        velocities.y += gravity;

        // apply air drag
        velocities.x *= 1.0 - (AIR_RESISTANCE_FACTOR * DT);
        velocities.y *= 1.0 - (AIR_RESISTANCE_FACTOR * DT);

        object.common().velocities = velocities;
    }

    fn apply_velocities(object: &mut Box<dyn GameObject>) {
        let common = object.common();
        let coords = common.coords.clone();
        let velocities = common.velocities.clone();

        object.common().coords = XYPair {
            x: coords.x + velocities.x,
            y: coords.y + velocities.y,
        };
    }

    fn collision_checks(window_size: &WindowSize, object: &mut Box<dyn GameObject>) {
        match object.collision_shape() {
            CollisionShape::Circle(radius) => {
                let mut coords = object.common().coords.clone();
                let mut velocities = object.common().velocities.clone();
                let diameter = 2.0 * radius;

                let on_ground = if coords.y + diameter >= window_size.height as f64 {
                    true
                } else {
                    false
                };

                let on_x_collision =
                    |velocities: &mut XYPair| velocities.x = -velocities.x * object.bounciness();

                let on_y_collision = |velocities: &mut XYPair| {
                    velocities.y = -velocities.y * object.bounciness();

                    // if we're just rolling on the ground, apply ground drag
                    if on_ground && velocities.y.abs() <= 1.0 {
                        velocities.x -= velocities.x * GROUND_DRAG_FACTOR
                    }
                };

                // x axis window collision
                if coords.x <= 0.0 {
                    coords.x = 0.0;
                    on_x_collision(&mut velocities);
                }
                if coords.x + diameter > window_size.width as f64 {
                    coords.x = window_size.width as f64 - diameter;
                    on_x_collision(&mut velocities);
                }

                // y axis window collision
                if coords.y - diameter < 0.0 {
                    coords.y = diameter;
                    on_y_collision(&mut velocities);
                }
                if coords.y + diameter > window_size.height as f64 {
                    coords.y = window_size.height as f64 - diameter;
                    on_y_collision(&mut velocities);
                }

                object.common().coords = coords;
                object.common().velocities = velocities;
            }
        }
    }

    fn update_object_info(window_size: &WindowSize, object: &mut Box<dyn GameObject>) {
        object.common().object_info = Some(ObjectInfo {
            window_size: window_size.clone(),
        });
    }

    fn draw(
        canvas: &mut Canvas<Window>,
        _window_size: &WindowSize,
        object: &mut Box<dyn GameObject>,
        texture_creator: &TextureCreator<sdl2::video::WindowContext>,
    ) {
        let common = object.common().clone();
        let coords = &common.coords;

        let texture = object.draw(texture_creator);

        Engine::draw_at(canvas, &texture, coords);
    }
}

// internal utils
impl Engine {
    fn draw_at(canvas: &mut Canvas<Window>, texture: &Texture, coords: &XYPair) {
        let x = coords.x;
        let y = coords.y;

        let destination_rect = Rect::new(
            x as i32,
            y as i32,
            texture.query().width as u32,
            texture.query().height as u32,
        );

        // copy the texture to the canvas at the specified position
        canvas.copy(texture, None, Some(destination_rect)).unwrap();

        canvas.present();
    }
}

// main run function -- sets up the window and the game loop
impl Engine {
    pub fn run(&mut self, window_title: &str) -> Result<(), anyhow::Error> {
        let sdl_context = sdl2::init().map_err(|_| anyhow!("Failed to create sdl context"))?;
        let video_subsystem = sdl_context
            .video()
            .map_err(|_| anyhow!("Failed to obtain video subsystem"))?;

        let window = video_subsystem
            .window(window_title, 1280, 720)
            .position_centered()
            .build()?;

        let mut event_pump = sdl_context.event_pump().unwrap();
        let timer = sdl_context.timer().unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        let texture_creator: &TextureCreator<_> = &canvas.texture_creator();

        // game loop
        'running: loop {
            let frame_start = Instant::now();

            // fill the canvas with black
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();
            canvas.present();

            // close the window if the user presses Esc
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }

            let keyboard_state = event_pump.keyboard_state();

            for object in self.objects.iter_mut() {
                // re-calculate the velocities of the object
                Engine::calc_velocities(object);

                // apply the velocities to the coordinates
                Engine::apply_velocities(object);

                // perform collision checks with the window
                Engine::collision_checks(&self.window_size, object);

                // update the game object's info
                Engine::update_object_info(&self.window_size, object);

                // allow the object to react to pressed keys
                object.handle_input(&keyboard_state);

                // draw the object on the buffer at it's coords
                Engine::draw(&mut canvas, &self.window_size, object, texture_creator);
            }

            // re-draw the new canvas
            canvas.present();

            // sleep to maintain 120fps if processing finished early
            let sleep_millis = Duration::from_secs_f64(DT)
                .saturating_sub(frame_start.elapsed())
                .as_millis() as u32;

            if sleep_millis > 0 {
                timer.delay(sleep_millis);
            }
        }
        Ok(())
    }
}
