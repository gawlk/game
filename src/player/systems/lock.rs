use bevy::prelude::*;

use super::*;

pub fn player_lock(
    mut commands: Commands,
    query: Query<Entity, With<Player>>,
    mut query_locked_movement_opt: Query<&mut PlayerLockedMovement>,
) {
    if query.is_empty() {
        return;
    }

    let player = query.single();

    if let Ok(mut locked_movement) = query_locked_movement_opt.get_single_mut() {
        locked_movement.tick();

        if locked_movement.is_over() {
            commands.entity(player).remove::<PlayerLockedMovement>();
        }
    }
}
