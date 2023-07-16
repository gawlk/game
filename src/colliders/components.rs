use bevy::prelude::*;

#[derive(Component, Deref, DerefMut, Debug)]
pub struct RectCollider(pub Vec2);
