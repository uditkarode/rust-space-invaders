use bevy_ecs::component::Component;

#[derive(Component)]
pub enum CollisionShape {
    Circle(f32),
    Rectangle(f32, f32),
}
