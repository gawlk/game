use bevy::prelude::*;

use crate::time::TickerComponent;

use super::*;

#[derive(Component)]
pub struct JumpBufferMarker;

#[derive(Bundle)]
pub struct JumpBufferBundle {
    pub marker: JumpBufferMarker,
    pub ticker: TickerComponent,
}

impl JumpBufferBundle {
    pub fn new() -> Self {
        Self {
            marker: JumpBufferMarker,
            ticker: TickerComponent::new(None, Some(PLAYER_JUMP_BUFFER)),
        }
    }
}
