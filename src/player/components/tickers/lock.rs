use bevy::prelude::*;

use crate::time::TickerComponent;

use super::*;

#[derive(Component)]
pub struct WallJumpLockMarker;

#[derive(Bundle)]
pub struct WallJumpLockBundle {
    pub marker: WallJumpLockMarker,
    pub ticker: TickerComponent,
}

impl WallJumpLockBundle {
    pub fn new() -> Self {
        Self {
            marker: WallJumpLockMarker,
            ticker: TickerComponent::new(None, Some(PLAYER_WALL_JUMP_LOCKED_TIME)),
        }
    }
}
