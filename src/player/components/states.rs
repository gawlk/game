use bevy::prelude::*;

use crate::{
    player::{PLAYER_FALL_HALF_TIME, PLAYER_RISE_HALF_TIME},
    time::Timer,
};

#[derive(Component)]
pub enum PlayerVerticalState {
    Grounded,
    Rising(Timer),
    Falling(Timer),
}

impl PlayerVerticalState {
    pub fn default_rise() -> Self {
        Self::Rising(Timer::new(None, Some(PLAYER_RISE_HALF_TIME)))
    }

    pub fn default_fall() -> Self {
        Self::new_fall(PLAYER_FALL_HALF_TIME)
    }

    pub fn new_fall(start: f32) -> Self {
        Self::Falling(Timer::new(Some(start), Some(PLAYER_FALL_HALF_TIME * 2.0)))
    }
}
