use bevy::prelude::*;

use crate::time::Timer;

use super::*;

#[derive(Component, Deref, DerefMut)]
pub struct PlayerCoyoteTimer(Timer);

impl PlayerCoyoteTimer {
    pub fn new() -> Self {
        Self(Timer::new(None, Some(PLAYER_COYOTE_TIME)))
    }
}
