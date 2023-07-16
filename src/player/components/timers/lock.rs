use bevy::prelude::*;

use crate::time::Timer;

use super::*;

#[derive(Component, Deref, DerefMut)]
pub struct PlayerLockedMovement(Timer);

impl PlayerLockedMovement {
    pub fn new() -> Self {
        Self(Timer::new(None, Some(PLAYER_WALL_JUMP_LOCKED_TIME)))
    }
}
