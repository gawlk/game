use bevy::prelude::*;

#[derive(Component, Deref, DerefMut, Debug)]
pub struct RectCollider(pub Vec2);

#[derive(Component, Default)]
pub struct Velocity {
    pub x: f32,     // m/s * delta
    pub raw_x: f32, // m/s
    pub y: f32,     // m/s * delta
}
