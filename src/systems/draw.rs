use bevy_ecs::world::World;

use crate::{
    components::drawable::{Drawable, DrawableKind},
    drawables::player::draw_player,
    utils::generic::TextureDrawer,
};

pub fn draw_drawable(world: &mut World, drawable: &Drawable, d: &mut TextureDrawer) {
    match drawable.kind {
        DrawableKind::Enemy => {
            // draw enemy
        }

        DrawableKind::Player => {
            draw_player(world, d);
        }
    }
}
