use bevy::prelude::*;

use super::{AnimationIndexes, AnimationTimer};

#[derive(Bundle)]
pub struct AnimationBundle {
    pub animation_indexes: AnimationIndexes,
    pub animation_timer: AnimationTimer,
}
