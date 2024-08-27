use raylib::color::Color;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Drawable {
    Circle { radius: f32, color: Color },
}
