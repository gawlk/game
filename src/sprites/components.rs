use bevy::prelude::*;

#[derive(Component)]
pub struct AnimationIndexes {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);
