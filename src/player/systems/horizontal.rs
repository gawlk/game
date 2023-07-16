use bevy::prelude::*;

use super::*;

pub fn player_horizontal(
    query: Query<(&PlayerActions, &PlayerVerticalState)>,
    mut query_mut: Query<&mut PlayerVelocityX>,
    mut query_locked_movement_opt: Query<&mut PlayerLockedMovement>,
) {
    if query.is_empty() || query_mut.is_empty() {
        return;
    }

    if query_locked_movement_opt.get_single_mut().is_ok() {
        return;
    }

    let mut velocity_x = query_mut.single_mut();

    let (&PlayerActions { left, right, .. }, vertical_state) = query.single();

    let grounded = matches!(vertical_state, PlayerVerticalState::Grounded);

    velocity_x.compute(left, right, grounded);
}
