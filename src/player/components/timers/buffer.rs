use bevy::prelude::*;

use crate::time::Timer;

use super::*;

#[derive(Component, Deref, DerefMut)]
pub struct PlayerJumpBuffer(Timer);

impl PlayerJumpBuffer {
    pub fn new() -> Self {
        Self(Timer::new(None, Some(PLAYER_JUMP_BUFFER)))
    }
}
