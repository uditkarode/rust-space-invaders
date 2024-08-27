use legion::*;
use raylib::prelude::{RaylibDraw, RaylibDrawHandle};

use crate::components::{drawable::Drawable, position::Position};

pub fn draw_drawables(world: &mut World, d: &mut RaylibDrawHandle) {
    let mut query = <(&mut Position, &Drawable)>::query();

    for (position, drawable) in query.iter_mut(world) {
        match drawable {
            Drawable::Circle { radius, color } => {
                d.draw_circle(
                    (position.x + radius) as i32,
                    (position.y + radius) as i32,
                    *radius,
                    *color,
                );
            }
        }
    }
}
