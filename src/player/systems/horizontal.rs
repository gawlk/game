use bevy::prelude::*;

use crate::time::TickerComponent;

use super::*;

pub fn player_horizontal(
    query: Query<(&PlayerActions, &PlayerVerticalState)>,
    mut query_mut: Query<&mut PlayerVelocityX>,
    query_locked_movement_opt: Query<&TickerComponent, With<WallJumpLockMarker>>,
) {
    if query.is_empty() || query_mut.is_empty() {
        return;
    }

    if query_locked_movement_opt.get_single().is_ok() {
        return;
    }

    let mut velocity_x = query_mut.single_mut();

    let (&PlayerActions { left, right, .. }, vertical_state) = query.single();

    let grounded = matches!(vertical_state, PlayerVerticalState::Grounded);

    velocity_x.compute(left, right, grounded);
}
