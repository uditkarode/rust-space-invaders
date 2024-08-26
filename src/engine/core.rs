use std::collections::HashMap;

use raylib::prelude::*;

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
}

// internal utils
impl Engine {}

// main run function -- sets up the window and the game loop
impl Engine {
    pub fn run(&mut self, window_title: &str) -> Result<(), anyhow::Error> {
        let (mut rl, thread) = raylib::init()
            .size(
                self.window_size.width as i32,
                self.window_size.height as i32,
            )
            .title(window_title)
            .build();

        rl.set_target_fps(120);
        rl.set_exit_key(Some(KeyboardKey::KEY_ESCAPE));

        // game loop
        while !rl.window_should_close() {
            let mut d = rl.begin_drawing(&thread);

            for object in self.objects.iter_mut() {
                // re-calculate the velocities of the object
                Engine::calc_velocities(object);

                // apply the velocities to the coordinates
                Engine::apply_velocities(object);

                // perform collision checks with the window
                Engine::collision_checks(&self.window_size, object);

                // update the game object's info
                Engine::update_object_info(&self.window_size, object);

                // allow the object to react to interested keys
                let mut pressed_keys: HashMap<KeyboardKey, bool> = HashMap::new();
                for key in object.common().interested_keys.iter() {
                    pressed_keys.insert(*key, d.is_key_down(*key));
                }
                object.handle_input(pressed_keys);

                // draw the object on the buffer at it's coords
                object.draw(&mut d);
            }

            d.clear_background(Color::BLACK);
        }

        Ok(())
    }
}
