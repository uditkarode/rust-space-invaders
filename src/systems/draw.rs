use bevy_ecs::world::World;

use crate::{
    components::drawable::{Drawable, DrawableKind},
    drawables::{enemy::draw_enemy, player::draw_player, projectile::draw_projectile},
    utils::generic::TextureDrawer,
};

pub fn draw_drawable(world: &mut World, drawable: &Drawable, d: &mut TextureDrawer) {
    match drawable.kind {
        DrawableKind::Projectile => {
            draw_projectile(world, d);
        }

        DrawableKind::Player => {
            draw_player(world, d);
        }

        DrawableKind::Enemy => {
            draw_enemy(world, d);
        }
    }
}
