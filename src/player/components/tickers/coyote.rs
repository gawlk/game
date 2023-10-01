use bevy::prelude::*;

use crate::time::TickerComponent;

use super::*;

#[derive(Component)]
pub struct CoyoteTimeMarker;

#[derive(Bundle)]
pub struct CoyoteTimeBundle {
    pub marker: CoyoteTimeMarker,
    pub ticker: TickerComponent,
}

impl CoyoteTimeBundle {
    pub fn new() -> Self {
        Self {
            marker: CoyoteTimeMarker,
            ticker: TickerComponent::new(None, Some(PLAYER_COYOTE_TIME)),
        }
    }
}
