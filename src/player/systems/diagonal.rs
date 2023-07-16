use bevy::prelude::*;

use super::*;

#[allow(dead_code)]
pub fn player_diagonal(
    query: Query<&PlayerActions>,
    mut query_mut: Query<(&mut PlayerVelocityX, &mut PlayerVelocityY)>,
) {
    if query.is_empty() || query_mut.is_empty() {
        return;
    }

    let (mut velocity_x, mut velocity_y) = query_mut.single_mut();

    let &PlayerActions {
        left,
        down,
        up,
        right,
        ..
    } = query.single();

    velocity_x.reset();

    if left != right {
        velocity_x.set_to_max(left);
    }

    velocity_y.reset();

    if down != up {
        let speed = PLAYER_RUN_MAX_SPEED;

        if down {
            velocity_y.set(-speed);
        } else {
            velocity_y.set(speed);
        }
    }
}
