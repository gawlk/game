use bevy::prelude::*;

use crate::{
    player::{PLAYER_FALL_HALF_TIME, PLAYER_RISE_HALF_TIME},
    time::Ticker,
};

#[derive(Component, Debug)]
pub enum PlayerVerticalState {
    Grounded,
    Rising(Ticker),
    Falling(Ticker),
}

impl PlayerVerticalState {
    pub fn default_rising() -> Self {
        Self::Rising(Ticker::new(None, Some(PLAYER_RISE_HALF_TIME)))
    }

    pub fn default_falling() -> Self {
        Self::new_falling(PLAYER_FALL_HALF_TIME)
    }

    pub fn new_falling(start: f32) -> Self {
        Self::Falling(Ticker::new(Some(start), Some(PLAYER_FALL_HALF_TIME * 2.0)))
    }
}
